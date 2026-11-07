//! 082: the value table. One table (`configs/agents/values.json`) gives a value in
//! credits to each resource, tag, token and effect, transcribed from
//! `docs/card-pricing.md`'s *Current rates* as they stood at r34 (the table is
//! their home since 082; 090 corrected energy, antimatter, troop removal,
//! liberation and the station flip against play, and gave income a horizon by
//! tier); a card is valued by its ops.
//!
//! [`op_worth`] matches every [`Op`] with no catch-all arm, so a new op does not
//! compile until it has an arm here, and a test fails until it has its row in the
//! table. A card's value is split by what it depends on ([`Worth`]): what it does
//! now, what it pays a round (and whether the heuristic's board terms already see
//! it), per node, per tag, and what needs a corrupted city. The caller supplies
//! the table's context ([`Ctx`]: rounds left, nodes, tags, a corrupted city).
//!
//! These are agent numbers, never rules: they change how agents value a card, not
//! what any card does.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

use aic_core::content::{Ability, CardDef, CardKind, Event, Facility, Op, Power, Resource, RoundFlag, Side, TokenKind, TAGS};
use serde::Deserialize;

/// The table as loaded.
#[derive(Clone, Debug, Deserialize)]
pub struct Table {
    #[allow(dead_code)]
    note: String,
    resources: BTreeMap<String, f64>,
    tags: TagRates,
    tokens: BTreeMap<String, f64>,
    rates: BTreeMap<String, f64>,
    pub(crate) ops: BTreeMap<String, Row>,
    /// 102 (D26): troops a "remove up to N troops" removes in play, by N.
    troops_removed: TroopsRemoved,
    /// Per-card adjustments, each with its reason (kept minimal).
    pub overrides: Vec<Override>,
    /// What a position reads, looked up once (the heuristic asks per card).
    #[serde(skip)]
    fast: Fast,
    /// A [`Table::repriced`] table's own: the agent's table before repricing.
    #[serde(skip)]
    unpriced: Option<Arc<Table>>,
}

#[derive(Clone, Debug, Default)]
struct Fast {
    /// By `Resource as usize`.
    res: [f64; 7],
    node: f64,
    node_late: f64,
    late_rounds: f64,
    no_city_share: f64,
    uses_share: f64,
    /// 107: `rates.tag_share`, `rates.own_mix` and `rates.mix_prior`.
    tag_share: f64,
    own_mix: f64,
    mix_prior: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct TroopsRemoved {
    #[allow(dead_code)]
    note: String,
    #[serde(flatten)]
    by: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct TagRates {
    first: f64,
    more: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Row {
    pub(crate) credits: f64,
    #[allow(dead_code)]
    pub(crate) per: String,
}

/// One per-card adjustment: credits added to the card's value, and why.
#[derive(Clone, Debug, Deserialize)]
pub struct Override {
    pub card: String,
    pub credits: f64,
    #[allow(dead_code)]
    pub why: String,
}

/// Rows of `ops` that are parts of another op's arm, not ops of their own.
#[cfg(test)]
pub(crate) const PART_ROWS: &[&str] =
    &["remove_ambassador", "remove_troop", "typed_jam", "liberate_align", "liberate_troops", "liberate_botnet", "cleanse_station"];

const SOURCE: &str = include_str!("../../../../configs/agents/values.json");

/// The table, parsed once.
pub fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let mut t: Table = serde_json::from_str(SOURCE).expect("configs/agents/values.json parses");
        t.refresh();
        t
    })
}

/// 131: the table with `sets` (an agent spec's `t.<key>=v`, see [`Table::set`])
/// applied, made once a process for each distinct set and shared by every agent
/// naming it; none is the table itself. A screen sets a rate per arm with no build.
pub fn table_with(sets: &BTreeMap<String, f64>) -> Result<&'static Table, String> {
    if sets.is_empty() {
        return Ok(table());
    }
    static MADE: Mutex<Vec<(String, &'static Table)>> = Mutex::new(Vec::new());
    let key: String = sets.iter().map(|(k, v)| format!("{k}={v};")).collect();
    let mut made = MADE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, t)) = made.iter().find(|(k, _)| *k == key) {
        return Ok(t);
    }
    let mut t = table().clone();
    for (k, v) in sets {
        t.set(k, *v)?;
    }
    t.refresh();
    let t: &'static Table = Box::leak(Box::new(t));
    made.push((key, t));
    Ok(t)
}

impl Table {
    /// 131: sets one number of the table by name: a resource (`chips`), a rate
    /// (`tag_count`), an op row's credits (`auto_counter`), a token (`feed_agents`),
    /// `troops_removed.<n>` or `tags.first` / `tags.more`. A name in two sections
    /// takes its section (`resources.chips`); an unknown one is an error.
    /// Call [`Table::refresh`] after.
    pub fn set(&mut self, key: &str, v: f64) -> Result<(), String> {
        if !v.is_finite() {
            return Err(format!("t.{key}: not a finite number"));
        }
        let (section, name) = match key.split_once('.') {
            Some((s, n)) => (Some(s), n),
            None => (None, key),
        };
        let mut hits: Vec<&'static str> = Vec::new();
        let want = |s: &str| section.is_none_or(|x| x == s);
        if want("resources") && self.resources.contains_key(name) {
            hits.push("resources");
        }
        if want("rates") && self.rates.contains_key(name) {
            hits.push("rates");
        }
        if want("ops") && self.ops.contains_key(name) {
            hits.push("ops");
        }
        if want("tokens") && self.tokens.contains_key(name) {
            hits.push("tokens");
        }
        if section == Some("troops_removed") {
            hits.push("troops_removed");
        }
        if section == Some("tags") && (name == "first" || name == "more") {
            hits.push("tags");
        }
        match hits.as_slice() {
            ["resources"] => _ = self.resources.insert(name.to_string(), v),
            ["rates"] => _ = self.rates.insert(name.to_string(), v),
            ["ops"] => self.ops.get_mut(name).expect("found").credits = v,
            ["tokens"] => _ = self.tokens.insert(name.to_string(), v),
            ["troops_removed"] => _ = self.troops_removed.by.insert(name.to_string(), v),
            ["tags"] if name == "first" => self.tags.first = v,
            ["tags"] => self.tags.more = v,
            [] => return Err(format!("t.{key}: no such number in configs/agents/values.json")),
            many => return Err(format!("t.{key}: in {} (name the section: t.{}.{name})", many.join(" and "), many[0])),
        }
        Ok(())
    }

    /// The lookups [`Fast`] keeps, read again from the table's maps.
    fn refresh(&mut self) {
        let t = self;
        let all = [
            Resource::Ambassadors,
            Resource::Antimatter,
            Resource::Chips,
            Resource::Credits,
            Resource::Energy,
            Resource::Insight,
            Resource::ResearchCubes,
        ];
        let mut res = [0.0; 7];
        for r in all {
            res[r as usize] = *t.resources.get(r.name()).unwrap_or_else(|| panic!("values.json: no resource {r}"));
        }
        t.fast = Fast {
            res,
            node: t.op("place_node") * t.rate("node"),
            node_late: t.op("place_node") * t.rate("node_late"),
            late_rounds: t.rate("late_rounds"),
            no_city_share: t.rate("no_city_share"),
            uses_share: t.rate("uses_share"),
            tag_share: t.rate("tag_share"),
            own_mix: t.rate("own_mix"),
            mix_prior: t.rate("mix_prior"),
        };
    }

    /// 102 (D23): this table with energy and antimatter at other rates (the
    /// keep plan's, priced as the heuristic prices them).
    pub fn repriced(&self, energy: f64, antimatter: f64) -> Table {
        let own = Arc::new(self.own().clone());
        let mut t = self.clone();
        t.fast.res[Resource::Energy as usize] = energy;
        t.fast.res[Resource::Antimatter as usize] = antimatter;
        t.resources.insert(Resource::Energy.name().to_string(), energy);
        t.resources.insert(Resource::Antimatter.name().to_string(), antimatter);
        t.unpriced = Some(own);
        t
    }

    /// The agent's table at its own rates: under [`Table::repriced`], the table
    /// it was repriced from; else this one.
    pub fn own(&self) -> &Table {
        self.unpriced.as_deref().unwrap_or(self)
    }

    pub fn rate(&self, k: &str) -> f64 {
        *self.rates.get(k).unwrap_or_else(|| panic!("values.json: no rate {k:?}"))
    }

    pub fn op(&self, k: &str) -> f64 {
        self.ops.get(k).unwrap_or_else(|| panic!("values.json: no op row {k:?}")).credits
    }

    /// A resource's rate in credits.
    pub fn res(&self, r: Resource) -> f64 {
        self.fast.res[r as usize]
    }

    /// What a token added to a card of `kind` is worth (a feeder's rate).
    pub fn feed(&self, kind: TokenKind) -> f64 {
        let k = format!("feed_{}", kind.name());
        *self.tokens.get(&k).unwrap_or_else(|| panic!("values.json: no token {k:?}"))
    }

    /// A token held with nothing better to say for it.
    pub fn token_held(&self) -> f64 {
        self.tokens["held"]
    }

    /// The tags a card leaves in play: the first, then each more.
    pub fn tags_value(&self, d: &CardDef) -> f64 {
        if d.kind == CardKind::Event || d.tags.is_empty() {
            return 0.0;
        }
        self.tags.first + self.tags.more * (d.tags.len() - 1) as f64
    }

    /// 102 (D26): the troops "remove up to `n` troops" removes in play, by the
    /// table's `troops_removed` (measured; the effect ledger reads the same
    /// rows); an `n` not measured counts as `n`.
    pub fn troops_removed(&self, n: i64) -> f64 {
        self.troops_removed.by.get(&n.to_string()).copied().unwrap_or(n as f64)
    }

    /// A keep's price in credits (the table's keep rows).
    pub fn keep(&self, tier: u8) -> f64 {
        if tier == 2 {
            self.rate("keep_tier2")
        } else {
            self.rate("keep_tier1")
        }
    }

    /// A card's printed cost at the table's rates, with `credits` as its credit
    /// price (printed, or after a seat's discounts) and `insight` its insight;
    /// lowered income counts each round left.
    pub fn cost(&self, d: &CardDef, credits: i64, insight: i64, left: f64) -> f64 {
        credits as f64
            + self.res(Resource::Insight) * insight as f64
            + self.res(Resource::Energy) * d.energy as f64
            + self.res(Resource::Chips) * d.chips as f64
            + self.res(Resource::Antimatter) * d.antimatter as f64
            + self.res(Resource::ResearchCubes) * d.research_cubes as f64
            + d.lowers.iter().map(|(r, n)| self.res(*r) * *n as f64 * left.max(1.0)).sum::<f64>()
    }
}

/// What the table's values depend on, for one seat now.
#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    /// Rounds of income still to come.
    pub left: f64,
    /// Rounds an income or an Action is counted for (r35: the horizon, with a
    /// tempo discount, at the typical purchase; `left` in a game).
    pub horizon: f64,
    /// The seat's nodes on the board.
    pub node: f64,
    /// The seat's tags on its cards in play.
    pub tags: [i64; TAGS.len()],
    /// Whether a city is corrupted (a liberation has a target).
    pub corrupted: bool,
    /// 107: the share of the seat's cards carrying each tag (`rates.tag_share`
    /// each, unless the caller reads the seat's own: [`Ctx::with_mix`]).
    pub share: [f64; TAGS.len()],
    /// 124 (D47, `w.team_share`): the teammates' share of cards carrying each
    /// tag, which a trigger on a teammate's plays fires at; `None`: the seat's own
    /// `share`, as before.
    pub team: Option<[f64; TAGS.len()]>,
    /// 128: the share of the table's energy price an energy costs this seat: 1,
    /// or less for a seat whose energy piles up ("energy I can't spend is cheap:
    /// an Action that spends it is worth its whole yield"; eval.rs `energy_cap`).
    pub energy_rate: f64,
    /// 128: whether a megaproject is still open to the seat's project cubes (an
    /// Action that places them has no target once none is: Particle Accelerator).
    pub cubes_open: bool,
}

impl Ctx {
    /// The context the table's own defaults stand for.
    pub fn typical(t: &Table) -> Ctx {
        Ctx {
            left: t.rate("income_rounds"),
            horizon: t.rate("income_rounds"),
            node: t.rate("node_count"),
            tags: [t.rate("tag_count") as i64; TAGS.len()],
            corrupted: true,
            share: [t.fast.tag_share; TAGS.len()],
            team: None,
            energy_rate: 1.0,
            cubes_open: true,
        }
    }

    /// 107 (`rates.own_mix`): the context with each tag's share read from the
    /// seat's own cards, `counts` cards carrying each tag of `cards` in all, with
    /// `rates.mix_prior` cards at the table's `tag_share` added so a thin hand
    /// doesn't swing it ("a Science trigger fires as often as your cards are Science").
    pub fn with_mix(mut self, t: &Table, counts: &[i64; TAGS.len()], cards: i64) -> Ctx {
        if t.fast.own_mix <= 0.0 {
            return self;
        }
        let k = t.fast.mix_prior;
        for (i, s) in self.share.iter_mut().enumerate() {
            *s = (counts[i] as f64 + k * t.fast.tag_share) / (cards as f64 + k);
        }
        self
    }

    /// The typical context for a card of `tier` (r35): its income counted over
    /// the tier's horizon, rounds left at a typical purchase less a tempo
    /// discount (`horizon_t1`, `horizon_t2`).
    pub fn typical_for(t: &Table, tier: u8) -> Ctx {
        let horizon = if tier >= 2 { t.rate("horizon_t2") } else { t.rate("horizon_t1") };
        Ctx { horizon, ..Ctx::typical(t) }
    }

    /// A context in a game: income counts every round left.
    pub fn at(t: &Table, left: f64, node: f64, tags: [i64; TAGS.len()], corrupted: bool) -> Ctx {
        Ctx {
            left,
            horizon: left,
            node,
            tags,
            corrupted,
            share: [t.fast.tag_share; TAGS.len()],
            team: None,
            energy_rate: 1.0,
            cubes_open: true,
        }
    }
}

/// An op's (or a list's) value in credits, split by what it depends on.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Worth {
    /// Now, whatever the table.
    pub now: f64,
    /// The part of `now` the heuristic's board terms don't see after the op
    /// resolves (a shelter, a cordon, a peek): credited when the action is taken.
    pub unseen: f64,
    /// Each node placed (at `node`, or `node_late` near the end).
    pub node: f64,
    /// Now, per node the seat has.
    pub per_node: f64,
    /// Now, per tag the seat has.
    pub per_tag: [f64; TAGS.len()],
    /// Now, only with a corrupted city (else `no_city_share` of it).
    pub city: f64,
    /// A round, standing, that the heuristic's income already counts (income,
    /// income per tag, production).
    pub income: f64,
    /// A round per tag, as `income`.
    pub income_per_tag: [f64; TAGS.len()],
    /// A round, standing, that nothing else counts (a discount, a shelter rule).
    pub standing: f64,
    /// 107: a round, standing, per share of the seat's cards carrying a tag (a
    /// trigger or discount on one tag's plays), times [`Ctx::share`].
    pub standing_tag: [f64; TAGS.len()],
    /// 124 (D47): as `standing_tag`, on a teammate's plays: times the teammates'
    /// share ([`Ctx::team`]; the seat's own without one).
    pub team_tag: [f64; TAGS.len()],
}

impl std::ops::AddAssign for Worth {
    fn add_assign(&mut self, o: Worth) {
        self.now += o.now;
        self.unseen += o.unseen;
        self.node += o.node;
        self.per_node += o.per_node;
        self.city += o.city;
        self.income += o.income;
        self.standing += o.standing;
        for t in 0..TAGS.len() {
            self.per_tag[t] += o.per_tag[t];
            self.income_per_tag[t] += o.income_per_tag[t];
            self.standing_tag[t] += o.standing_tag[t];
            self.team_tag[t] += o.team_tag[t];
        }
    }
}

impl Worth {
    fn now(v: f64) -> Worth {
        Worth { now: v, ..Worth::default() }
    }

    /// Now and unseen by the heuristic's board terms.
    fn unseen(v: f64) -> Worth {
        Worth { now: v, unseen: v, ..Worth::default() }
    }

    fn standing(v: f64) -> Worth {
        Worth { standing: v, ..Worth::default() }
    }

    fn income(v: f64) -> Worth {
        Worth { income: v, ..Worth::default() }
    }

    pub fn scaled(mut self, k: f64) -> Worth {
        self.now *= k;
        self.unseen *= k;
        self.node *= k;
        self.per_node *= k;
        self.city *= k;
        self.income *= k;
        self.standing *= k;
        for t in 0..TAGS.len() {
            self.per_tag[t] *= k;
            self.income_per_tag[t] *= k;
            self.standing_tag[t] *= k;
            self.team_tag[t] *= k;
        }
        self
    }

    /// 107: a standing rate on one tag's plays (`rates.fires_play` plays a round
    /// times `per_play`), its share of the seat's cards read from the context.
    fn on_tag(tag: aic_core::content::Tag, per_play: f64) -> Worth {
        let mut w = Worth::default();
        w.standing_tag[tag as usize] = per_play;
        w
    }

    /// What resolves now, in this context.
    pub fn instant(&self, t: &Table, c: &Ctx) -> f64 {
        let node = if c.left <= t.fast.late_rounds { t.fast.node_late } else { t.fast.node };
        let city = if c.corrupted { 1.0 } else { t.fast.no_city_share };
        let tags: f64 = (0..TAGS.len()).map(|i| self.per_tag[i] * c.tags[i] as f64).sum();
        self.now + self.node * node + self.per_node * c.node + self.city * city + tags
    }

    /// What it pays a round, in this context: income and standing.
    pub fn per_round(&self, c: &Ctx) -> f64 {
        self.income + self.unseen_round(c) + self.income_round_tags(c)
    }

    fn income_round_tags(&self, c: &Ctx) -> f64 {
        (0..TAGS.len()).map(|i| self.income_per_tag[i] * c.tags[i] as f64).sum()
    }

    /// The standing part the heuristic's income doesn't count, a round.
    pub fn unseen_round(&self, c: &Ctx) -> f64 {
        let team = c.team.as_ref().unwrap_or(&c.share);
        self.standing
            + (0..TAGS.len()).map(|i| self.standing_tag[i] * c.share[i] + self.team_tag[i] * team[i]).sum::<f64>()
    }
}

/// A resource's rate as income (093): energy income is discounted by
/// `rates.energy_income_share`, since it arrives a drop at a time rather than
/// when it is needed.
fn income_rate(t: &Table, res: Resource) -> f64 {
    let share = if res == Resource::Energy { t.rate("energy_income_share") } else { 1.0 };
    t.res(res) * share
}

/// What one op is worth when the seat that resolves it is a player.
pub fn op_worth(t: &Table, op: &Op) -> Worth {
    let r = |res: Resource| t.res(res);
    let row = |k: &str| t.op(k);
    let tag_i = |tag: aic_core::content::Tag| tag as usize;
    match op {
        Op::Gain { resource, amount } => Worth::now(row("gain") * r(*resource) * *amount as f64),
        Op::GainPerNode { resource } => Worth { per_node: row("gain_per_node") * r(*resource), ..Worth::default() },
        Op::ChoosePlayer => Worth::now(row("choose_player")),
        Op::OtherGain { resource, amount } => Worth::now(row("other_gain") * r(*resource) * *amount as f64),
        Op::Transfer { resource, max } => Worth::now(row("transfer") * r(*resource) * *max as f64),
        Op::PlaceNode { count, free } => Worth {
            node: *count as f64,
            now: if *free { t.rate("node_free") * *count as f64 } else { 0.0 },
            ..Worth::default()
        },
        Op::ClearMarkers { count } => Worth::now(row("clear_markers") * t.rate("target_share") * *count as f64),
        Op::ShieldNode { count } => Worth::now(row("shield_node") * t.rate("target_share") * *count as f64),
        Op::DelayThreat { count } => Worth::now(row("delay_threat") * *count as f64),
        Op::Damage { ambassadors, troops } => Worth::now(row("damage") * damage(t, *ambassadors, *troops)),
        Op::DamageCountry { ambassadors, troops } => Worth::now(row("damage_country") * damage(t, *ambassadors, *troops)),
        Op::DestroyAll => Worth::now(row("destroy_all")),
        Op::RemoveBotnet => Worth::now(row("remove_botnet")),
        Op::RemoveAmbassadorsCountry { unity_per, max, .. } => {
            let n = if *max == 0 { t.rate("token_pile") } else { *max as f64 };
            let sway = if *unity_per > 0 { (n / *unity_per as f64).floor() * t.rate("sway") } else { 0.0 };
            Worth::now(row("remove_ambassadors_country") * n + sway)
        }
        Op::RemoveTroops { count, .. } => Worth::now(row("remove_troops") * t.troops_removed(*count)),
        Op::BuildFactory => Worth::now(row("build_factory")),
        Op::Cleanse { amount, one_country, cities, only } => {
            let share = if *one_country { 0.8 } else { 1.0 };
            if *cities {
                Worth { city: row("liberate") * share * *amount as f64, ..Worth::default() }
            } else {
                // r35: a station is a flip, a factory stays cheap; either kind
                // reads the station's row, the dearer one it may take.
                let each = if *only == Some(Facility::Factory) { row("cleanse") } else { row("cleanse_station") };
                Worth::now(each * share * *amount as f64)
            }
        }
        Op::Liberate { align, ambassadors, troops, botnet } => Worth {
            city: row("liberate")
                + if *align { row("liberate_align") } else { 0.0 }
                + row("gain_ambassadors") * r(Resource::Ambassadors) * *ambassadors as f64
                + row("liberate_troops") * *troops as f64
                + if *botnet { row("liberate_botnet") } else { 0.0 },
            ..Worth::default()
        },
        Op::ClearCity { max } => {
            let n = if *max == 0 { t.rate("token_pile") } else { *max as f64 };
            Worth::now(row("clear_city") * n)
        }
        Op::Harden { count } => Worth::now(row("harden") * *count as f64),
        Op::MoveAmbassadors { max } => Worth::now(row("move_ambassadors") * *max as f64),
        Op::GainAmbassadors { count } => Worth::now(row("gain_ambassadors") * r(Resource::Ambassadors) * *count as f64),
        Op::Shelter => Worth::unseen(row("shelter") * t.rate("target_share")),
        Op::Cordon => Worth::unseen(row("cordon")),
        Op::FreeBuilding => Worth::now(row("free_building")),
        Op::BuildStation => Worth::now(row("build_station")),
        Op::ProjectCube { count, .. } => Worth::now(row("project_cube") * *count as f64),
        Op::Peek { count } => Worth::unseen(row("peek") * *count as f64),
        Op::Jam { amount, source } => {
            let typed = if source.is_some() { row("typed_jam") } else { 1.0 };
            Worth::now(row("jam") * typed * *amount as f64)
        }
        Op::CounterThreat => Worth::now(row("counter_threat")),
        Op::DrainProject { amount } => Worth::now(row("drain_project") * *amount as f64),
        Op::DiscardProject => Worth::now(row("discard_project")),
        Op::Production { resource, amount } => Worth::income(row("production") * income_rate(t, *resource) * *amount as f64),
        // 106: an income per tag fixed at play, valued as one at the seat's count.
        Op::ProductionPerTag { resource, tag, per } => {
            let mut w = Worth::default();
            w.income_per_tag[tag_i(*tag)] = row("production_per_tag") * income_rate(t, *resource) / (*per).max(1) as f64;
            w
        }
        Op::AddTokens { kind, amount, .. } => Worth::now(row("add_tokens") * t.feed(*kind) * *amount as f64),
        Op::CutBotnet => Worth::unseen(row("cut_botnet")),
        Op::MoveOne { .. } => Worth::now(row("move_one")),
        Op::SpreadAmbassadors => Worth::now(row("spread_ambassadors")),
        Op::Consolidate { max } => Worth::now(row("consolidate") * *max as f64),
        Op::Draw { count } => Worth::now(row("draw") * t.rate("drawn_card") * *count as f64),
        Op::DrainAll { amount } => Worth::now(row("drain_all") * *amount as f64),
        Op::GainPerTokens { resource, per, .. } => {
            Worth::now(row("gain_per_tokens") * r(*resource) * (t.rate("token_pile") / (*per).max(1) as f64).floor())
        }
        Op::CashOut { resource, rate, .. } => Worth::now(row("cash_out") * r(*resource) * *rate as f64 * t.rate("token_pile")),
        Op::GainPerTag { resource, tag, per } => {
            let mut w = Worth::default();
            w.per_tag[tag_i(*tag)] = row("gain_per_tag") * r(*resource) / (*per).max(1) as f64;
            w
        }
        Op::Sway { side, amount } => {
            let sign = if *side == Side::Unity { 1.0 } else { -1.0 };
            Worth::now(row("sway") * sign * t.rate("sway") * *amount as f64)
        }
        Op::Troops { count, each_country, destroy, .. } => {
            let n = *count as f64 * if *each_country { t.rate("countries") } else { 1.0 };
            let lost = if *destroy { row("destroy_node").abs() } else { 0.0 };
            Worth::now(row("troops") * n - lost * n)
        }
        Op::DestroyNode => Worth::now(row("destroy_node")),
        Op::EachDestroyNode { count } => Worth::now(row("each_destroy_node") * *count as f64),
        Op::PullUnity { amount } => Worth::now(row("pull_unity") * *amount as f64),
        Op::LoseInsight { amount } => Worth::unseen(row("lose_insight") * *amount as f64),
        Op::FlipStation { each_country, .. } => {
            Worth::now(row("flip_station") * if *each_country { t.rate("countries") } else { 1.0 })
        }
        Op::CorruptCity { .. } => Worth::now(row("corrupt_city")),
        Op::PlayersLose { resource, amount, .. } => Worth::now(row("players_lose") * r(*resource) * *amount as f64),
        Op::AdversaryBuild => Worth::now(row("adversary_build")),
        Op::Create { count, sites, .. } => Worth::now(row("create") * t.rate("countries") * (*count * *sites) as f64),
        Op::CreateSite { .. } => Worth::now(row("create_site") * t.rate("countries")),
        Op::Corrupt { .. } => Worth::now(row("corrupt")),
        Op::LoseNodeOr { resource, amount, count } => Worth::now(
            row("lose_node_or") * (r(*resource) * *amount as f64).min(t.rate("node") * *count as f64),
        ),
        Op::Flag { name, .. } => Worth::unseen(match name {
            RoundFlag::Shelter => row("shelter") * t.rate("target_share"),
            RoundFlag::Cordon => row("cordon"),
            _ => -row("flag"),
        }),
        Op::Requisition { resource, amount, cards, .. } => Worth::now(
            row("requisition") * (r(*resource) * *amount as f64).min(t.rate("drawn_card") * *cards as f64),
        ),
        Op::LoseAmbassador => Worth::now(row("lose_ambassador")),
        Op::DiscardMostExpensive => Worth::now(row("discard_most_expensive")),
        Op::Compute { amount } => Worth::now(row("compute") * *amount as f64),
        Op::ComputeEvery { amount } => Worth::now(row("compute_every") * *amount as f64),
        Op::LoseTokens { amount } => Worth::now(row("lose_tokens") * *amount as f64),
        Op::TeamIncome { resource, amount } => Worth::income(row("team_income") * r(*resource) * *amount as f64),
        Op::FreePurchases { count } => Worth::now(row("free_purchases") * t.rate("keep_tier1") * *count as f64),
        Op::CounterAll => Worth::now(row("counter_all")),
        Op::Pushback { ambassadors } => Worth::now(row("pushback") * t.rate("countries") * *ambassadors as f64),
        Op::DealThreats { count, .. } => Worth::now(row("deal_threats") * *count as f64),
        Op::Sacrifice => Worth::now(row("sacrifice")),
        Op::RemoveBotnets { count } => Worth::now(row("remove_botnets") * *count as f64),
        Op::Income { resource, amount } => Worth::income(row("income") * income_rate(t, *resource) * *amount as f64),
        Op::IncomePerTag { resource, tag, per, amount } => {
            let mut w = Worth::default();
            w.income_per_tag[tag_i(*tag)] =
                row("income_per_tag") * income_rate(t, *resource) * *amount as f64 / (*per).max(1) as f64;
            w
        }
        Op::BuildingDiscount { amount } => Worth::standing(row("building_discount") * *amount as f64),
        Op::HardenAction { chips } => {
            Worth::standing((row("harden_action") - 0.5 * r(Resource::Chips) * *chips as f64 / 2.0).max(0.0))
        }
        Op::Slots { amount } => Worth::standing(row("slots") * *amount as f64),
        Op::Logistics { resource, amount } => Worth::standing(row("logistics") * r(*resource) * *amount as f64),
        Op::ProjectDrain { amount } => Worth::standing(row("project_drain") * *amount as f64),
        Op::AutoCounter { amount, .. } => Worth::standing(row("auto_counter") * *amount as f64),
        Op::UnityBonus { amount } => Worth::standing(row("unity_bonus") * *amount as f64),
        Op::CorruptionShield => Worth::standing(row("corruption_shield")),
        Op::CivicShelter => Worth::standing(row("civic_shelter")),
        Op::Wall => Worth::standing(row("wall")),
        Op::Patience => Worth::standing(row("patience")),
        Op::SupplyInsight { amount } => Worth::standing(row("supply_insight") * *amount as f64),
        Op::TagsFromHand { .. } => Worth::standing(row("tags_from_hand")),
        Op::ConvertUpkeep { from, amount, to, gain } => {
            Worth::standing((row("convert_upkeep") * (r(*to) * *gain as f64 - r(*from) * *amount as f64)).max(0.0))
        }
        // 105: the engine wording. A trigger is worth what it resolves (tokens
        // here aside: a threshold prices those) times how often it fires a round.
        Op::CardDiscount { tag, resource, amount } => {
            // 107: insight off every card cuts only the plays the discount floor
            // leaves above it (Hyperscale Botnet was kept at a full rate, never played).
            let each = row("card_discount") * t.rate("fires_play") * r(*resource) * *amount as f64;
            match (tag, resource) {
                // 107: at the seat's share of the tag's cards (`Ctx::share`).
                (Some(tag), _) => Worth::on_tag(*tag, each),
                (None, Resource::Insight) => Worth::standing(each * t.rate("insight_discount_share")),
                (None, _) => Worth::standing(each),
            }
        }
        Op::When { on, then } => {
            let both = if matches!(on, Event::TeamPlay { both: true, .. }) { 1.0 + row("other_gain") } else { 1.0 };
            let each = row("when") * triggered(t, then) * both;
            match on_tag(on) {
                // 124 (D47): one on a teammate's plays of a tag fires at the
                // teammates' share of them (`Ctx::team`), not the owner's.
                Some(tag) if matches!(on, Event::TeamPlay { .. }) => {
                    let mut w = Worth::default();
                    w.team_tag[tag as usize] = each * fires(t, on) / t.rate("tag_share");
                    w
                }
                // 107: a trigger on one tag's plays fires at the seat's share of them.
                Some(tag) => Worth::on_tag(tag, each * fires(t, on) / t.rate("tag_share")),
                None => Worth::standing(each * fires(t, on)),
            }
        }
        Op::Threshold { at, then } => {
            Worth::standing(row("threshold") * t.rate("threshold_fill") / (*at).max(1) as f64 * triggered(t, then))
        }
        Op::CopyIncome => Worth::income(row("copy_income")),
        Op::Reuse => Worth::now(row("reuse")),
        Op::DiscardHand { resource, amount } => {
            Worth::now(row("discard_hand") * t.rate("hand_discards") * (r(*resource) * *amount as f64 - t.rate("dead_card")))
        }
        Op::War => Worth::now(row("war")),
        // 135: the new adversary model's threat and team ops (no player card prints them).
        Op::TroopSteps { .. } => Worth::now(row("troop_steps")),
        Op::StandingTroopsBonus { .. } => Worth::now(row("standing_troops_bonus")),
        Op::AdversaryFactory { count, .. } => Worth::now(row("adversary_factory") * *count as f64),
        Op::ComputePerMissing { amount, .. } => Worth::now(row("compute_per_missing") * *amount as f64),
        Op::CorpTroops { count, .. } => Worth::now(row("corp_troops") * (*count).max(1) as f64),
        Op::DestroyFacility { .. } => Worth::now(row("destroy_facility")),
        Op::DisableBuilding { count, .. } => Worth::now(row("disable_building") * *count as f64),
        Op::Discard { count } => Worth::now(row("discard") * t.rate("drawn_card") * *count as f64),
        Op::ExtraWar { .. } => Worth::now(row("extra_war")),
        Op::LoseAmbassadorsOr { count, resource, amount } => Worth::now(
            row("lose_ambassadors_or") * (r(*resource) * *amount as f64).min(r(Resource::Ambassadors) * *count as f64),
        ),
        Op::LoseIncome { resource, amount } => Worth::unseen(row("lose_income") * r(*resource) * *amount as f64),
        Op::LoseOr { options } => Worth::now(
            options.iter().map(|o| op_worth(t, o).now).fold(f64::NEG_INFINITY, f64::max).max(-1e9) * row("lose_or"),
        ),
        Op::MarchNow { .. } => Worth::now(row("march_now")),
        Op::OccupyFactory { .. } => Worth::now(row("occupy_factory")),
        Op::PlaceBotnet { count } => Worth::now(row("place_botnet") * *count as f64),
        Op::SeizeFactory => Worth::now(row("seize_factory")),
        Op::Theft { resource, amount, .. } => Worth::now(row("theft") * r(*resource) * *amount as f64),
    }
}

/// 107: the tag a trigger's plays must carry, if one: its firing rate is
/// `rates.tag_share` of the plays at the table, the seat's own share in a game.
fn on_tag(on: &Event) -> Option<aic_core::content::Tag> {
    match *on {
        Event::Play { tag } | Event::PlayTag { tag } | Event::TeamPlay { tag, .. } => tag,
        _ => None,
    }
}

/// 105: how often a trigger fires a round, at the table's rates.
fn fires(t: &Table, on: &Event) -> f64 {
    let tagged = |tag: Option<aic_core::content::Tag>| if tag.is_some() { t.rate("tag_share") } else { 1.0 };
    match *on {
        Event::Play { tag } => t.rate("fires_play") * tagged(tag),
        Event::PlayTag { tag: Some(_) } => t.rate("fires_play") * t.rate("tag_share"),
        // 106: every tag on the played card.
        Event::PlayTag { tag: None } => t.rate("fires_play") * t.rate("tags_per_card"),
        Event::TeamPlay { tag, .. } => t.rate("fires_play") * tagged(tag) * (t.rate("countries") - 1.0).max(1.0),
        Event::PlayCost { at } => t.rate("fires_play") * (1.0 - at as f64 / t.rate("rebate_span")).clamp(0.05, 1.0),
        Event::RemoveTroop => t.rate("fires_troop"),
        Event::RemoveCompute => t.rate("fires_compute"),
        Event::RemoveAmbassadors => t.rate("fires_ambassadors"),
        Event::PlaceTokens { .. } => t.rate("fires_tokens"),
        Event::WallStops => t.rate("fires_wall"),
    }
}

/// 105: what one firing of a trigger's ops is worth: now, and its nodes at the
/// table's node rate; tokens added here are a threshold's to price.
fn triggered(t: &Table, then: &[Op]) -> f64 {
    let mut w = Worth::default();
    for op in then.iter().filter(|o| !matches!(o, Op::AddTokens { here: true, .. })) {
        w += op_worth(t, op);
    }
    // 107: an income a firing raises counts `raised_rounds` rounds (Retained
    // Earnings' raise read 0, so its Capital feeders had no pile to feed).
    w.now + w.node * t.fast.node + w.income * t.rate("raised_rounds")
}

/// 107: a card's standing effects. A threshold whose tokens the card's own
/// trigger adds fills at that trigger's rate (`rates.own_fill`), not the flat
/// `threshold_fill`: Conglomerate Headquarters adds one a tag played.
pub fn passives_worth(t: &Table, ops: &[Op]) -> Worth {
    // Tokens a round the card's own triggers add here: untagged, and per share of
    // one tag's plays (107: read at the seat's share, as the trigger itself).
    let mut own = 0.0;
    let mut own_tag = [0.0; TAGS.len()];
    for o in ops {
        if let Op::When { on, then } = o {
            let added: i64 = then
                .iter()
                .map(|e| match e {
                    Op::AddTokens { amount, here: true, .. } => *amount,
                    _ => 0,
                })
                .sum();
            match on_tag(on) {
                Some(tag) => own_tag[tag as usize] += fires(t, on) / t.rate("tag_share") * added as f64,
                None => own += fires(t, on) * added as f64,
            }
        }
    }
    let fed = own > 0.0 || own_tag.iter().any(|x| *x > 0.0);
    let mut w = Worth::default();
    for op in ops {
        w += match op {
            Op::Threshold { at, then } if fed && t.rate("own_fill") > 0.0 => {
                let each = t.op("threshold") / (*at).max(1) as f64 * triggered(t, then);
                let mut x = Worth::standing(each * own);
                for (i, f) in own_tag.iter().enumerate() {
                    x.standing_tag[i] = each * f;
                }
                x
            }
            _ => op_worth(t, op),
        };
    }
    w
}

/// A damage's worth: the better of its two kinds.
fn damage(t: &Table, ambassadors: i64, troops: i64) -> f64 {
    (t.op("remove_ambassador") * ambassadors as f64).max(t.op("remove_troop") * troops as f64)
}

/// The sum of a list's ops.
pub fn list_worth(t: &Table, ops: &[Op]) -> Worth {
    let mut w = Worth::default();
    for op in ops {
        w += op_worth(t, op);
    }
    w
}

/// A building's Action, a use: the best branch less what it costs. A branch that
/// spends tokens from its own card (or counts or cashes them) while another branch
/// adds them is worth its payoff over the uses a cycle takes (076: progressive
/// token cards alternate adds and spends).
pub fn ability_worth(t: &Table, a: &Ability) -> Worth {
    let branches = a.branches();
    let add_here: i64 = branches
        .iter()
        .flat_map(|b| &b.effects)
        .filter_map(|e| match e {
            Op::AddTokens { amount, here: true, .. } => Some(*amount),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    let mut best: Option<Worth> = None;
    for b in &branches {
        let adds = b.effects.iter().all(|e| matches!(e, Op::AddTokens { here: true, .. }));
        if adds && add_here > 0 && branches.len() > 1 {
            continue;
        }
        let mut w = list_worth(t, &b.effects);
        let cost: f64 = b.cost.iter().map(|(r, n)| t.res(*r) * *n as f64).sum();
        w.now -= cost;
        let counts_here = b.effects.iter().any(|e| {
            matches!(e, Op::CashOut { here: true, .. } | Op::GainPerTokens { here: true, .. })
        });
        if add_here > 0 && (b.tokens > 0 || counts_here) {
            // A pile of `need` takes need / add uses to fill, then one to spend.
            let need = if b.tokens > 0 { b.tokens as f64 } else { t.rate("token_pile") };
            let cycle = (need / add_here as f64).ceil() + 1.0;
            w = w.scaled(1.0 / cycle);
        }
        let better = match &best {
            None => true,
            Some(o) => w.now + w.city + w.node * 3.0 > o.now + o.city + o.node * 3.0,
        };
        if better {
            best = Some(w);
        }
    }
    best.unwrap_or_default()
}

/// A token on card `d` is worth this: what its spend pays per token, or the
/// table's held rate.
pub fn token_worth(t: &Table, d: &CardDef) -> Worth {
    let spend = d.ability.as_ref().and_then(|a| {
        a.branches().into_iter().filter(|b| b.tokens > 0).map(|b| list_worth(t, &b.effects).scaled(1.0 / b.tokens as f64)).next()
    });
    spend.unwrap_or(Worth::now(t.token_held()))
}

/// 124 (D47, `w.threshold_tokens`): `base`, or on a card with a threshold (At
/// N) what the threshold pays per token when that is more ("a token toward an
/// At 4 that pays 12 is worth 3").
pub fn token_worth_threshold(t: &Table, d: &CardDef, base: Worth) -> Worth {
    let at = d
        .passives
        .iter()
        .filter_map(|p| match p {
            Op::Threshold { at, then } => Some(t.op("threshold") * triggered(t, then) / (*at).max(1) as f64),
            _ => None,
        })
        .reduce(f64::max);
    match at {
        Some(v) if v > base.now => Worth::now(v),
        _ => base,
    }
}

/// 102 (D24): [`token_worth`], but a token on a card whose Action cashes its own
/// pile out (Exabyte Archive, Bonded Warehouse, Sovereign Capital Fund) is worth
/// what that cash-out pays for it, not the held rate.
pub fn token_worth_cash(t: &Table, d: &CardDef) -> Worth {
    let cash = d.ability.as_ref().and_then(|a| {
        a.branches()
            .into_iter()
            .flat_map(|b| &b.effects)
            .filter_map(|e| match e {
                Op::CashOut { resource, rate, here: true, .. } => Some(t.op("cash_out") * t.res(*resource) * *rate as f64),
                Op::GainPerTokens { resource, per, here: true, .. } => {
                    Some(t.op("gain_per_tokens") * t.res(*resource) / (*per).max(1) as f64)
                }
                _ => None,
            })
            .reduce(f64::max)
    });
    match cash {
        Some(v) if v > t.token_held() => Worth::now(v),
        _ => token_worth(t, d),
    }
}

/// A card's value in pieces: what playing it does (its effects and, for a
/// building, its standing effects), a use of its Action, the tags it leaves in
/// play and any override.
#[derive(Clone, Copy, Debug, Default)]
pub struct CardValue {
    pub play: Worth,
    pub use_: Worth,
    /// Whether it has an Action.
    pub has_use: bool,
    /// Whether that Action is one-shot (r37): used once, not each round.
    pub once: bool,
    pub tags: f64,
    pub extra: f64,
    /// 128: the energy one use of its Action costs (a seat with spare energy
    /// pays less than the table's price for it: [`Ctx::energy_rate`]).
    pub use_energy: f64,
    /// 128: its Action only places project cubes, so has no use once no
    /// megaproject is open ([`Ctx::cubes_open`]).
    pub cube_use: bool,
}

/// 128: a card left in play counts its own tags for its per-tag ops, as the
/// engine does (a program "1 per 2 Science tags" counts its own Science tag).
fn own_tags(w: &mut Worth, d: &CardDef) {
    if d.is_event() {
        return;
    }
    for tag in &d.tags {
        let i = *tag as usize;
        w.now += w.per_tag[i];
        w.income += w.income_per_tag[i];
    }
}

impl CardValue {
    pub fn of(t: &Table, d: &CardDef) -> CardValue {
        // r37: a one-shot Action (a single branch that leaves play) is used once, and
        // the tokens the card arrives with are its fuel, counted in that use.
        let once = d.ability.as_ref().is_some_and(|a| a.once && a.or.is_none());
        let mut play = if once {
            let rest: Vec<Op> =
                d.effects.iter().filter(|e| !matches!(e, Op::AddTokens { here: true, .. })).cloned().collect();
            list_worth(t, &rest)
        } else {
            list_worth(t, &d.effects)
        };
        play += passives_worth(t, &d.passives);
        // 102: an Action at the value table's own rates, even under a repriced
        // table (`Table::repriced`), as the heuristic values a building's Action.
        let mut use_ = d.ability.as_ref().map(|a| ability_worth(t.own(), a)).unwrap_or_default();
        own_tags(&mut play, d);
        own_tags(&mut use_, d);
        let use_energy = d.ability.as_ref().map_or(0.0, |a| {
            a.cost.iter().filter(|(r, _)| *r == Resource::Energy).map(|(_, n)| *n as f64).sum()
        });
        let cube_use = d.ability.as_ref().is_some_and(|a| {
            let cubes = |b: &Ability| b.effects.iter().any(|e| matches!(e, Op::ProjectCube { .. }));
            cubes(a) && a.or.as_deref().is_none_or(cubes)
        });
        let extra = t.overrides.iter().filter(|o| o.card == d.id).map(|o| o.credits).sum();
        CardValue { play, use_, has_use: d.ability.is_some(), once, tags: t.tags_value(d), extra, use_energy, cube_use }
    }

    /// 128: one use of its Action in this context, its energy at the seat's rate.
    fn one_use(&self, t: &Table, c: &Ctx) -> f64 {
        if self.cube_use && !c.cubes_open {
            return 0.0;
        }
        let rebate = self.use_energy * t.own().res(Resource::Energy) * (1.0 - c.energy_rate);
        (self.use_.instant(t, c) + self.use_.per_round(c) + rebate).max(0.0)
    }

    /// The card's whole value in this context: now, plus a round's worth for each
    /// round left, plus its Action's uses.
    pub fn total(&self, t: &Table, c: &Ctx) -> f64 {
        let uses = if self.once {
            1.0
        } else if self.has_use {
            c.horizon * t.fast.uses_share
        } else {
            0.0
        };
        self.play.instant(t, c)
            + c.horizon * self.play.per_round(c)
            + uses * self.one_use(t, c)
            + self.tags
            + self.extra
    }

    /// What a building in play still earns a round that the heuristic's income
    /// doesn't count: its standing rules and a share of its Action.
    pub fn built_round(&self, t: &Table, c: &Ctx) -> f64 {
        // 099: a one-shot Action earns nothing a round: its one use is its fuel's
        // worth (the token term), so using it no longer reads as giving up a stream.
        let uses = if self.has_use && !self.once { t.fast.uses_share } else { 0.0 };
        // 130: a `values.json` override is part of the card's worth in play too, spread
        // over the rounds left (the heuristic multiplies this by them); without it a card
        // read higher in hand than built, and an overridden building was never played.
        self.play.unseen_round(c) + uses * self.one_use(t, c) + self.extra / c.horizon.max(1.0)
    }
}

/// A board power's use, less its cost in resources (its tag sacrifices aside).
pub fn power_worth(t: &Table, p: &Power) -> Worth {
    let mut w = list_worth(t, &p.effects);
    w.now -= p.cost.iter().map(|(r, n)| t.res(*r) * *n as f64).sum::<f64>();
    w
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every op has its row, and every row is an op's or a named part of one.
    #[test]
    fn every_op_has_a_row() {
        let t = table();
        let ops: Vec<String> =
            crate::docs::op_names(include_str!("../../../aic-core/src/op.rs"));
        assert!(ops.len() > 90, "{} ops read", ops.len());
        let missing: Vec<&String> = ops.iter().filter(|o| !t.ops.contains_key(*o)).collect();
        assert!(missing.is_empty(), "ops with no row in configs/agents/values.json: {missing:?}");
        let unused: Vec<&String> =
            t.ops.keys().filter(|k| !ops.contains(k) && !PART_ROWS.contains(&k.as_str())).collect();
        assert!(unused.is_empty(), "rows that are no op: {unused:?}");
    }

    /// Every card, board power and override reads: finite values, an override names
    /// a card and says why.
    #[test]
    fn every_card_has_a_value() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        let t = table();
        let c = Ctx::typical(t);
        for d in &w.content.cards {
            let v = CardValue::of(t, d).total(t, &c);
            assert!(v.is_finite(), "{}", d.id);
        }
        for b in &w.content.ais {
            for p in &b.powers {
                assert!(power_worth(t, p).instant(t, &c).is_finite(), "{}", p.name);
            }
        }
        for o in &t.overrides {
            assert!(w.content.card_index(&o.card).is_some(), "override for no card {:?}", o.card);
            assert!(!o.why.trim().is_empty(), "override {:?} says no reason", o.card);
        }
    }

    /// 100: the effect ledger (`scripts/cards/effect_ledger.py`) mirrors this file
    /// by hand; its committed `cards.csv` must read each card as `CardValue::total`
    /// does at `Ctx::typical_for` the card's tier. A failure means value.rs, the
    /// table or a card moved: fix the ledger's mirror if it no longer matches, then
    /// rerun the ledger.
    #[test]
    fn effect_ledger_mirrors_the_table() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        let t = table();
        let mut rows = csv_records(include_str!("../../../../experiments/results/effect-ledger/cards.csv")).into_iter();
        let head = rows.next().expect("a header");
        let col = |name: &str| head.iter().position(|h| h == name).expect(name);
        let (id, reasoned) = (col("card"), col("reasoned"));
        let mut read = 0;
        let mut off = Vec::new();
        for cells in rows {
            let d = w.content.card(&cells[id]);
            read += 1;
            // 102 (D26): a troop removal too: both read `troops_removed`.
            let ledger: f64 = cells[reasoned].parse().expect("a number");
            let rust = CardValue::of(t, d).total(t, &Ctx::typical_for(t, d.tier));
            if (ledger - rust).abs() > 0.011 {
                off.push(format!("{} ledger {ledger} value.rs {rust:.2}", d.id));
            }
        }
        assert_eq!(read, w.content.cards.len(), "cards.csv is stale: rerun scripts/cards/effect_ledger.py");
        assert!(off.is_empty(), "the ledger's mirror of value.rs disagrees:\n{}", off.join("\n"));
    }

    /// The records of a CSV as Python's `csv` writes it: a quoted field may hold
    /// commas, newlines and doubled quotes.
    fn csv_records(text: &str) -> Vec<Vec<String>> {
        let (mut rows, mut row, mut cell) = (Vec::new(), Vec::new(), String::new());
        let mut quoted = false;
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            match (quoted, ch) {
                (true, '"') if chars.peek() == Some(&'"') => {
                    chars.next();
                    cell.push('"');
                }
                (_, '"') => quoted = !quoted,
                (false, ',') => row.push(std::mem::take(&mut cell)),
                (false, '\n') => {
                    row.push(std::mem::take(&mut cell));
                    rows.push(std::mem::take(&mut row));
                }
                (_, c) => cell.push(c),
            }
        }
        if !cell.is_empty() || !row.is_empty() {
            row.push(cell);
            rows.push(row);
        }
        rows
    }

    /// A liberate capstone counts its city only while one is corrupted, and a
    /// one-shot liberate Action is a whole city, used once (r37).
    #[test]
    fn liberation_reads_its_city() {
        let r = crate::agents::tests::rules(2);
        let w = r.world();
        let t = table();
        let mut c = Ctx::typical(t);
        let d = w.content.card("hostile_buyout");
        let with = CardValue::of(t, d).total(t, &c);
        c.corrupted = false;
        let without = CardValue::of(t, d).total(t, &c);
        assert!(with > without + 10.0, "{with} {without}");
        // No card prints a one-shot Action since r42: Popular Uprising's liberate
        // half, made its only Action and one-shot (its r41 shape).
        let mut one_shot = w.content.card("popular_uprising").clone();
        let mut half = *one_shot.ability.as_mut().and_then(|a| a.or.take()).expect("its liberate half");
        half.once = true;
        one_shot.ability = Some(half);
        let pu = CardValue::of(t, &one_shot);
        // r37: one Action, one-shot: a whole liberation, used once; the Followers
        // it arrives with are that use's fuel, not value of their own.
        assert!(pu.once && (pu.use_.city - t.op("liberate")).abs() < 1e-9, "{:?}", pu.use_);
        assert_eq!(pu.play.now, 0.0, "{:?}", pu.play);
        c.corrupted = true;
        let whole = pu.total(t, &c);
        assert!((whole - t.op("liberate") - pu.tags).abs() < 1.0, "{whole}");
    }
}
