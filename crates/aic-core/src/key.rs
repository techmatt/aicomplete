//! What keeps a state cheap to copy (030): [`Key`], an interned name, and
//! [`FlatMap`], a sorted vector with `BTreeMap`'s order. A search copies
//! states by the million, and a state is mostly names (`"<module>.<name>"`
//! tracks, card ids in piles) with a number beside them: as `String`s in
//! `BTreeMap`s, one copy was thousands of allocations.
//!
//! Neither shows in the state's JSON: a key writes as its string and a
//! `FlatMap` as a map in key order, so fingerprints and records are the same
//! as with the plain types, and order is the strings' own, never an address.
//!
//! The interner is global but not state: it only says which `&'static str`
//! stands for a string, it is never iterated, and what it holds changes
//! nothing a game can see. It never frees, and the names a game can make are
//! bounded by the content (ids times seats); a state from outside adds at
//! most the names it carries.

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ops::Deref;
use std::sync::{OnceLock, RwLock};

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::hash::{FNV_OFFSET, FNV_PRIME};

/// An interned name. Copying one copies a pointer; two keys are equal exactly
/// when their pointers are. Orders, hashes, prints and serializes as its text.
#[derive(Clone, Copy)]
pub struct Key(&'static str);

/// FNV-1a by words for the thread's interner cache and every [`FastMap`]: short keys,
/// and no DoS to fear.
#[derive(Default)]
pub struct Fnv(u64);

impl Hasher for Fnv {
    /// Eight bytes at a time (069): a lookup by card id hashed some twenty bytes
    /// one by one, tens of times a position. No [`FastMap`] is ever iterated,
    /// so the hash's value shows nowhere.
    fn write(&mut self, bytes: &[u8]) {
        let mut h = if self.0 == 0 { FNV_OFFSET } else { self.0 };
        let (words, rest) = bytes.as_chunks::<8>();
        for w in words {
            h = (h ^ u64::from_le_bytes(*w)).wrapping_mul(FNV_PRIME);
        }
        for b in rest {
            h = (h ^ *b as u64).wrapping_mul(FNV_PRIME);
        }
        self.0 = h;
    }
    /// Rotated: a multiply leaves its low bits weak, and a table buckets on them.
    fn finish(&self) -> u64 {
        self.0.rotate_left(26)
    }
}

type Cache = HashMap<&'static str, &'static str, BuildHasherDefault<Fnv>>;

thread_local! {
    /// Each thread's copy of what it has seen, so workers don't queue on the lock.
    static CACHE: RefCell<Cache> = RefCell::new(Cache::default());
}

fn table() -> &'static RwLock<HashSet<&'static str>> {
    static TABLE: OnceLock<RwLock<HashSet<&'static str>>> = OnceLock::new();
    TABLE.get_or_init(Default::default)
}

fn intern(s: &str) -> &'static str {
    CACHE.with(|c| {
        if let Some(k) = c.borrow().get(s) {
            return *k;
        }
        let found = table().read().expect("interner lock").get(s).copied();
        let k = found.unwrap_or_else(|| {
            let mut t = table().write().expect("interner lock");
            match t.get(s) {
                Some(k) => *k,
                None => {
                    let k: &'static str = Box::leak(s.to_owned().into_boxed_str());
                    t.insert(k);
                    k
                }
            }
        });
        c.borrow_mut().insert(k, k);
        k
    })
}

impl Key {
    pub fn new(s: &str) -> Key {
        Key(intern(s))
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl Deref for Key {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        self.0
    }
}

impl Borrow<str> for Key {
    #[inline]
    fn borrow(&self) -> &str {
        self.0
    }
}

impl PartialEq for Key {
    #[inline]
    fn eq(&self, other: &Key) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for Key {}

impl Ord for Key {
    #[inline]
    fn cmp(&self, other: &Key) -> Ordering {
        if std::ptr::eq(self.0, other.0) {
            Ordering::Equal
        } else {
            cmp_bytes(self.0.as_bytes(), other.0.as_bytes())
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.0, f)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.0, f)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        Key::new(s)
    }
}

impl From<&String> for Key {
    fn from(s: &String) -> Key {
        Key::new(s)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Key {
        Key::new(&s)
    }
}

impl From<Key> for String {
    fn from(k: Key) -> String {
        k.0.to_owned()
    }
}

impl PartialEq<str> for Key {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Key {
    fn eq(&self, other: &String) -> bool {
        self.0 == other
    }
}

impl PartialEq<Key> for String {
    fn eq(&self, other: &Key) -> bool {
        self == other.0
    }
}

impl PartialEq<Key> for &str {
    fn eq(&self, other: &Key) -> bool {
        *self == other.0
    }
}

impl Serialize for Key {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.0)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Key, D::Error> {
        Ok(Key::new(&String::deserialize(d)?))
    }
}

/// A hash map for lookups by name only (parameters, content ids): quicker than
/// a tree over strings that share their prefixes. Never iterate one into a
/// state or a result: its order is the hash's.
pub type FastMap<V> = HashMap<String, V, BuildHasherDefault<Fnv>>;

/// A name built on the stack, for a lookup that shouldn't allocate: rules and
/// heuristics ask for `"cards.hand.<seat>"` and the like thousands of times a
/// position. Reads as a `str`.
#[derive(Clone, Copy)]
pub struct KeyBuf {
    len: u8,
    buf: [u8; 71],
}

impl KeyBuf {
    pub fn new(s: &str) -> KeyBuf {
        KeyBuf { len: 0, buf: [0; 71] }.text(s)
    }

    /// Adds text. A name past the buffer is a bug in the content's ids (a test
    /// builds every key of every card) and panics.
    pub fn text(mut self, s: &str) -> KeyBuf {
        let at = self.len as usize;
        self.buf[at..at + s.len()].copy_from_slice(s.as_bytes());
        self.len = (at + s.len()) as u8;
        self
    }

    /// Adds a number in decimal, its digits written in place (103: through a
    /// `str` check and a copy, a seat's or a hex's number was 4% of a search).
    pub fn num(mut self, n: usize) -> KeyBuf {
        let mut width = 1;
        let mut rest = n / 10;
        while rest > 0 {
            width += 1;
            rest /= 10;
        }
        let at = self.len as usize;
        let mut rest = n;
        for d in self.buf[at..at + width].iter_mut().rev() {
            *d = b'0' + (rest % 10) as u8;
            rest /= 10;
        }
        self.len = (at + width) as u8;
        self
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: `buf[..len]` only ever holds whole `str`s, copied in `text`.
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len as usize]) }
    }
}

impl Deref for KeyBuf {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for KeyBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for KeyBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<KeyBuf> for String {
    fn from(k: KeyBuf) -> String {
        k.as_str().to_owned()
    }
}

impl From<KeyBuf> for Key {
    fn from(k: KeyBuf) -> Key {
        Key::new(k.as_str())
    }
}

/// A map kept as a vector sorted by key: one allocation to copy, however many
/// entries. It takes the `BTreeMap` calls the state uses, and a key by
/// anything that makes one. Writing a key into a large map moves what follows
/// it, which at a state's size (a few hundred entries) is cheaper than the
/// tree's allocations.
#[derive(Clone, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

/// The state's maps: `"<module>.<name>"` to a value.
pub type Table<V> = FlatMap<Key, V>;

impl<K, V> Default for FlatMap<K, V> {
    fn default() -> Self {
        FlatMap(Vec::new())
    }
}

/// How a [`FlatMap`]'s search compares a key with what it looks for: always as
/// `Ord` does, faster for names. `str`'s own compare calls the C runtime's
/// `memcmp`, and with a map of a few hundred names and lookups by the million
/// that call alone was a tenth of the strategist's time (103).
pub trait Probe: Ord {
    fn probe(&self, q: &Self) -> Ordering {
        self.cmp(q)
    }
}

impl Probe for str {
    #[inline]
    fn probe(&self, q: &str) -> Ordering {
        cmp_bytes(self.as_bytes(), q.as_bytes())
    }
}

impl Probe for Key {
    #[inline]
    fn probe(&self, q: &Key) -> Ordering {
        self.cmp(q)
    }
}

/// `a.cmp(b)` for byte strings, eight bytes a step and inline: names are short
/// and share prefixes (`"cards.picked.0"`), where a call to `memcmp` costs more
/// than the compare.
#[inline]
pub fn cmp_bytes(a: &[u8], b: &[u8]) -> Ordering {
    let n = a.len().min(b.len());
    let (mut x, mut y) = (&a[..n], &b[..n]);
    while x.len() >= 8 {
        let (p, xs) = x.split_at(8);
        let (q, ys) = y.split_at(8);
        let p = u64::from_be_bytes(p.try_into().expect("eight bytes"));
        let q = u64::from_be_bytes(q.try_into().expect("eight bytes"));
        if p != q {
            return p.cmp(&q);
        }
        (x, y) = (xs, ys);
    }
    for (p, q) in x.iter().zip(y) {
        if p != q {
            return p.cmp(q);
        }
    }
    a.len().cmp(&b.len())
}

impl<K: Probe + Copy, V> FlatMap<K, V> {
    #[inline]
    fn find<Q: Probe + ?Sized>(&self, q: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
    {
        self.0.binary_search_by(|(k, _)| k.borrow().probe(q))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn get<Q: Probe + ?Sized>(&self, q: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.find(q).ok().map(|i| &self.0[i].1)
    }

    pub fn get_mut<Q: Probe + ?Sized>(&mut self, q: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        self.find(q).ok().map(|i| &mut self.0[i].1)
    }

    pub fn contains_key<Q: Probe + ?Sized>(&self, q: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        self.find(q).is_ok()
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self.find::<K>(&k) {
            Ok(i) => Some(std::mem::replace(&mut self.0[i].1, v)),
            Err(i) => {
                self.0.insert(i, (k, v));
                None
            }
        }
    }

    pub fn remove<Q: Probe + ?Sized>(&mut self, q: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        self.find(q).ok().map(|i| self.0.remove(i).1)
    }

    pub fn entry(&mut self, k: impl Into<K>) -> Entry<'_, K, V> {
        let k = k.into();
        let at = self.find::<K>(&k);
        Entry { map: self, key: k, at }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&K, &V)> + Clone {
        self.0.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (&K, &mut V)> {
        self.0.iter_mut().map(|(k, v)| (&*k, v))
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &K> + Clone {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl DoubleEndedIterator<Item = &V> + Clone {
        self.0.iter().map(|(_, v)| v)
    }

    pub fn retain(&mut self, mut f: impl FnMut(&K, &mut V) -> bool) {
        self.0.retain_mut(|(k, v)| f(k, v))
    }

    /// The entries from `q` on, as `range(q..)`.
    pub fn from_key<Q: Probe + ?Sized>(&self, q: &Q) -> impl Iterator<Item = (&K, &V)>
    where
        K: Borrow<Q>,
    {
        let at = self.find(q).unwrap_or_else(|i| i);
        self.0[at..].iter().map(|(k, v)| (k, v))
    }
}

impl<V> FlatMap<Key, V> {
    /// [`FlatMap::entry`] by name, interning it only for a new entry: the
    /// interner's thread-local lookup was most of a counter's update (103).
    #[inline]
    pub fn entry_str(&mut self, k: &str) -> Entry<'_, Key, V> {
        let at = self.find(k);
        let key = match at {
            Ok(i) => self.0[i].0,
            Err(_) => Key::new(k),
        };
        Entry { map: self, key, at }
    }

    /// [`FlatMap::insert`] by name, interning it only for a new entry.
    #[inline]
    pub fn insert_str(&mut self, k: &str, v: V) -> Option<V> {
        match self.find(k) {
            Ok(i) => Some(std::mem::replace(&mut self.0[i].1, v)),
            Err(i) => {
                self.0.insert(i, (Key::new(k), v));
                None
            }
        }
    }
}

/// [`FlatMap::entry`]: a place in the map, found or to be filled.
pub struct Entry<'a, K, V> {
    map: &'a mut FlatMap<K, V>,
    key: K,
    at: Result<usize, usize>,
}

impl<'a, K: Ord + Copy, V> Entry<'a, K, V> {
    pub fn or_insert(self, v: V) -> &'a mut V {
        self.or_insert_with(|| v)
    }

    pub fn or_insert_with(self, v: impl FnOnce() -> V) -> &'a mut V {
        let i = match self.at {
            Ok(i) => i,
            Err(i) => {
                self.map.0.insert(i, (self.key, v()));
                i
            }
        };
        &mut self.map.0[i].1
    }

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(V::default)
    }
}

impl<V> std::ops::Index<&str> for FlatMap<Key, V> {
    type Output = V;
    fn index(&self, k: &str) -> &V {
        self.get(k).unwrap_or_else(|| panic!("no entry {k:?}"))
    }
}

impl<K: Probe + Copy, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(it: I) -> Self {
        let mut m = FlatMap::default();
        for (k, v) in it {
            m.insert(k, v);
        }
        m
    }
}

impl<K, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a FlatMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().map(|(k, v)| (k, v))
    }
}

impl<'a, K, V> IntoIterator for &'a mut FlatMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = std::iter::Map<std::slice::IterMut<'a, (K, V)>, fn(&'a mut (K, V)) -> (&'a K, &'a mut V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for FlatMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.0.iter().map(|(k, v)| (k, v))).finish()
    }
}

impl<K: Serialize, V: Serialize> Serialize for FlatMap<K, V> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_map(self.0.iter().map(|(k, v)| (k, v)))
    }
}

impl<'de, K, V> Deserialize<'de> for FlatMap<K, V>
where
    K: Deserialize<'de> + Probe + Copy,
    V: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(BTreeMap::<K, V>::deserialize(d)?.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_their_text() {
        let a = Key::new("board.track");
        let b = Key::from("board.track".to_string());
        assert_eq!(a, b);
        assert!(std::ptr::eq(a.as_str(), b.as_str()));
        assert_ne!(a, Key::new("board.tracks"));
        assert!(Key::new("a") < Key::new("b"));
        assert_eq!(a, "board.track");
        assert_eq!(format!("{a:?} {a}"), "\"board.track\" board.track");
        assert_eq!(serde_json::to_string(&a).unwrap(), "\"board.track\"");
        let back: Key = serde_json::from_str("\"board.track\"").unwrap();
        assert_eq!(a, back);
    }

    /// A key hashes as a `String` of its text: the beam's order-free sequence
    /// keys are built from these hashes.
    #[test]
    fn keys_hash_as_strings() {
        use std::collections::hash_map::DefaultHasher;
        let h = |v: &dyn Fn(&mut DefaultHasher)| {
            let mut s = DefaultHasher::new();
            v(&mut s);
            s.finish()
        };
        assert_eq!(h(&|s| Key::new("x.y").hash(s)), h(&|s| "x.y".to_string().hash(s)));
    }

    #[test]
    fn stack_keys_read_as_their_text() {
        let k = KeyBuf::new("cards.under.").num(3).text(".").text("x");
        assert_eq!(k.as_str(), "cards.under.3.x");
        assert_eq!(&*k, "cards.under.3.x");
        assert_eq!(KeyBuf::new("a").num(0).num(1200).as_str(), "a01200");
        assert_eq!(String::from(k), "cards.under.3.x");
        assert_eq!(Key::from(k), Key::new("cards.under.3.x"));
    }

    /// The longest name the content can make fits the stack key: a card or
    /// threat id in the longest pattern, with the longest resource name.
    #[test]
    fn every_content_id_fits_a_stack_key() {
        let c = crate::content::content();
        let longest = crate::vocab::RESOURCES.iter().map(|r| r.name()).max_by_key(|n| n.len()).unwrap();
        let ids = c.cards.iter().map(|x| &x.id).chain(c.threats.iter().map(|x| &x.id));
        for id in ids.chain(c.projects.iter().map(|x| &x.id)) {
            KeyBuf::new("cards.tokens.").num(3).text(".").text(id);
            KeyBuf::new("threats.paid.").text(id).text(".").num(3).text(".").text(longest);
        }
    }

    /// Everything a `BTreeMap` of the same calls would do, entry for entry
    /// and in JSON.
    #[test]
    fn flat_maps_behave_as_btree_maps() {
        let mut flat: Table<i64> = Table::default();
        let mut tree: BTreeMap<String, i64> = BTreeMap::new();
        let mut x = 7u64;
        for i in 0..2000 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let k = format!("k{}", (x >> 33) % 97);
            match i % 4 {
                0 => assert_eq!(flat.insert(k.as_str().into(), i), tree.insert(k, i)),
                1 => assert_eq!(flat.remove(k.as_str()), tree.remove(&k)),
                2 => *flat.entry(k.as_str()).or_insert(0) += i,
                _ => *tree.entry(k).or_insert(0) += 0,
            }
            if i % 4 == 2 {
                let k = format!("k{}", (x >> 33) % 97);
                *tree.entry(k).or_insert(0) += i;
            }
            if i % 4 == 3 {
                let k = format!("k{}", (x >> 33) % 97);
                flat.entry(k.as_str()).or_insert(0);
            }
        }
        assert_eq!(serde_json::to_string(&flat).unwrap(), serde_json::to_string(&tree).unwrap());
        let back: Table<i64> = serde_json::from_str(&serde_json::to_string(&tree).unwrap()).unwrap();
        assert!(back == flat);
        let from: Vec<_> = flat.from_key("k5").map(|(k, v)| (k.to_string(), *v)).collect();
        let want: Vec<_> = tree.range("k5".to_string()..).map(|(k, v)| (k.clone(), *v)).collect();
        assert_eq!(from, want);
    }
}
