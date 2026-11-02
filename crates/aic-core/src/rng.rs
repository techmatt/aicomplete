//! Seeded, per-purpose random streams held in the game state.
//!
//! Every purpose (a name such as `"adversary"`) gets its own ChaCha8 stream,
//! keyed by the game seed and a hash of the purpose name. Adding, removing or
//! reordering modules therefore never shifts the numbers another module sees.

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::hash::fnv1a64;

/// Serde helpers that write integers as decimal strings, so they survive
/// JavaScript's 53-bit numbers.
pub(crate) mod as_string {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use std::fmt::Display;
    use std::str::FromStr;

    pub fn serialize<T: Display, S: Serializer>(v: &T, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(v)
    }

    pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(d: D) -> Result<T, D::Error>
    where
        T::Err: Display,
    {
        String::deserialize(d)?.parse().map_err(D::Error::custom)
    }
}

/// A ChaCha8 stream that serializes as
/// `{"seed": "<64 hex>", "stream": "<u64>", "word_pos": "<u128>"}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream(ChaCha8Rng);

impl Stream {
    pub fn new(seed: u64, purpose: &str) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        rng.set_stream(fnv1a64(purpose.as_bytes()));
        Stream(rng)
    }

    /// Uniform in `0..n`; `n` must be in `1..=u32::MAX`. Written out on
    /// `u32` draws, never `usize`: rand samples `usize` from a `u64` on
    /// 64-bit targets and a `u32` on wasm32, which would make native and
    /// browser games differ. It also keeps results independent of rand's
    /// sampling code.
    pub fn below(&mut self, n: usize) -> usize {
        let n = u32::try_from(n)
            .ok()
            .filter(|&n| n > 0)
            .expect("below(n): n in 1..=u32::MAX");
        // Reject the low `2^32 mod n` values so every residue is equally likely.
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.0.next_u32();
            if x >= threshold {
                return (x % n) as usize;
            }
        }
    }

    /// Stream id (the purpose hash), for `Rules::check_state`.
    pub fn stream_id(&self) -> u64 {
        self.0.get_stream()
    }
}

#[derive(Serialize, Deserialize)]
struct StreamJson {
    seed: String,
    #[serde(with = "as_string")]
    stream: u64,
    #[serde(with = "as_string")]
    word_pos: u128,
}

impl Serialize for Stream {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let seed = self
            .0
            .get_seed()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        StreamJson {
            seed,
            stream: self.0.get_stream(),
            word_pos: self.0.get_word_pos(),
        }
        .serialize(s)
    }
}

impl<'de> Deserialize<'de> for Stream {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let j = StreamJson::deserialize(d)?;
        let hex = j.seed.as_bytes();
        if hex.len() != 64 {
            return Err(D::Error::custom("stream seed must be 64 hex chars"));
        }
        let mut seed = [0u8; 32];
        for (i, b) in seed.iter_mut().enumerate() {
            let pair = std::str::from_utf8(&hex[2 * i..2 * i + 2]).map_err(D::Error::custom)?;
            *b = u8::from_str_radix(pair, 16).map_err(D::Error::custom)?;
        }
        let mut rng = ChaCha8Rng::from_seed(seed);
        rng.set_stream(j.stream);
        rng.set_word_pos(j.word_pos);
        Ok(Stream(rng))
    }
}

/// All streams of a game, by purpose. Created at `new_game` for every
/// purpose the configured module variants declare; nothing else may draw.
pub type RngStreams = crate::key::Table<std::sync::Arc<Stream>>;

/// Fisher-Yates, written out so the sequence never depends on a library's
/// shuffle implementation.
pub fn shuffle<T>(items: &mut [T], rng: &mut Stream) {
    for i in (1..items.len()).rev() {
        let j = rng.below(i + 1);
        items.swap(i, j);
    }
}
