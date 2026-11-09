//! Countering a threat (135): its menu after difficulty and reductions,
//! pooled resource payments, and the options one seat pays whole.

use super::*;

/// What [`cheapest_resource_need`] says of a threat no option of which is
/// resources only: more than any table pays.
pub const UNPAYABLE: i64 = 99;

/// Whether every item of `option` is a resource (pooled with Pay).
pub fn resource_only(option: &[CounterItem]) -> bool {
    option.iter().all(|i| matches!(i, CounterItem::Pay { .. }))
}

/// What `option` asks of each resource, summed, in first-seen order.
pub(super) fn resource_needs(option: &[CounterItem]) -> Vec<(Resource, i64)> {
    let mut out: Vec<(Resource, i64)> = Vec::new();
    for item in option {
        if let CounterItem::Pay { resource, amount } = item {
            match out.iter_mut().find(|(r, _)| r == resource) {
                Some((_, n)) => *n += amount,
                None => out.push((*resource, *amount)),
            }
        }
    }
    out
}

/// The resources `threat`'s menu names in any of its options, first-seen order.
pub(super) fn menu_resources(w: &World, threat: &str) -> Vec<Resource> {
    let mut out = Vec::new();
    for item in def(w, threat).counters.iter().flatten() {
        if let CounterItem::Pay { resource, .. } = item {
            if !out.contains(resource) {
                out.push(*resource);
            }
        }
    }
    out
}

/// What countering `threat` asks now: its menu scaled by `counter_pct` (and
/// credits by `counter_credit_pct`), plus the difficulty steps from
/// `counter_bonus_at` and `counter_more_at`, less Jam Signals and standing
/// Auto Counters on every resource item.
pub fn requirement(w: &World, g: &GameState, threat: &str) -> Vec<Vec<CounterItem>> {
    requirement_less(w, threat, g.var(&jam_key(threat)) + standing(w, g, threat))
}

/// [`requirement`] less `cut` of each resource item (floor 0).
pub(super) fn requirement_less(w: &World, threat: &str, cut: i64) -> Vec<Vec<CounterItem>> {
    // 142: the counter step from `counter_bonus_at` (d2), and a second one from `counter_more_at` (d3).
    let (mut bonus, mut all, mut credits) = (0, 0, 0);
    if adversary::level(w, w.int("adversary.counter_bonus_at")) {
        bonus = w.int("adversary.counter_bonus");
        all = w.int("adversary.counter_bonus_all");
        credits = w.int("adversary.counter_bonus_credits");
    }
    if adversary::level(w, w.int("adversary.counter_more_at")) {
        all += w.int("adversary.counter_more_all");
        credits += w.int("adversary.counter_more_credits");
    }
    // 138: every resource counter item of a tier-N threat costs `counter_pct[N-1]` percent of its printed amount
    // (rounded up), at every difficulty.
    let d = def(w, threat);
    let pct = w.list("adversary.counter_pct").get(d.tier as usize - 1).copied().unwrap_or(100);
    let credit_pct = w.int("adversary.counter_credit_pct");
    d.counters
        .iter()
        .enumerate()
        .map(|(k, option)| {
            option
                .iter()
                .map(|item| match *item {
                    CounterItem::Pay { resource, amount } => {
                        // 142: the first option's every resource item (was its credits only).
                        let first = if k == 0 { bonus } else { 0 };
                        let mut scaled = (amount * pct + 99) / 100;
                        if resource == Resource::Credits {
                            scaled = (scaled * credit_pct + 99) / 100;
                        }
                        // 142: a credits item takes `counter_bonus_credits` in place of `counter_bonus_all`.
                        let step = if resource == Resource::Credits { credits } else { all };
                        CounterItem::Pay { resource, amount: (scaled + first + step - cut).max(0) }
                    }
                    other => other,
                })
                .collect()
        })
        .collect()
}

/// Paid so far toward `threat` in `resource`, by all seats.
pub(super) fn paid(g: &GameState, threat: &str, resource: Resource) -> i64 {
    (0..g.seats.len()).map(|s| g.var(&paid_key(threat, s, resource))).sum()
}

/// The smallest resource total still owed over `threat`'s resource-only
/// options (pooled payments taken off), for the agents; [`UNPAYABLE`] if
/// every option asks for more than resources.
pub fn cheapest_resource_need(w: &World, g: &GameState, threat: &str) -> i64 {
    requirement(w, g, threat)
        .iter()
        .filter(|o| resource_only(o))
        .map(|o| resource_needs(o).iter().map(|(r, n)| (n - paid(g, threat, *r)).max(0)).sum::<i64>())
        .min()
        .unwrap_or(UNPAYABLE)
}

/// 145: the share of `threat`'s counter paid, by its best resource-only
/// option: what is paid in that option's own currencies over what it asks
/// (1 credit toward a 15-credit option is 1/15, not a third of 3 energy). 0
/// with no resource option or nothing paid.
pub fn paid_share(w: &World, g: &GameState, threat: &str) -> f64 {
    need_and_share(w, g, threat).1
}

/// [`cheapest_resource_need`] and [`paid_share`] from one read of the menu
/// (the agents' eval asks both of every threat).
pub fn need_and_share(w: &World, g: &GameState, threat: &str) -> (i64, f64) {
    let (mut need, mut share) = (UNPAYABLE, 0.0f64);
    for o in requirement(w, g, threat).iter().filter(|o| resource_only(o)) {
        let needs = resource_needs(o);
        let (mut ask, mut got, mut gap) = (0, 0, 0);
        for (r, n) in needs {
            let p = paid(g, threat, r);
            ask += n.max(0);
            got += p.min(n.max(0));
            gap += (n - p).max(0);
        }
        need = need.min(gap);
        share = share.max(if ask > 0 { got as f64 / ask as f64 } else { 1.0 });
    }
    (need, share)
}

/// The most of `resource` a pooled payment toward `threat` may still add:
/// the largest gap among its resource-only options naming it.
pub(super) fn room(req: &[Vec<CounterItem>], g: &GameState, threat: &str, resource: Resource) -> i64 {
    let have = paid(g, threat, resource);
    req.iter()
        .filter(|o| resource_only(o))
        .flat_map(|o| resource_needs(o))
        .filter(|(r, _)| *r == resource)
        .map(|(_, n)| n - have)
        .max()
        .unwrap_or(0)
}

/// What standing counters take off `threat` (070): the Auto Counters of its
/// type (its tag, or Universal) held by every seat, wherever the threat is
/// (148: r61's menus are one scarce currency or credits, so the old "you can
/// pay toward" cut seldom fired). Read, not stored, so a delayed threat isn't
/// cut twice and an extra deal is cut too.
pub(super) fn standing(w: &World, g: &GameState, threat: &str) -> i64 {
    if scope_of(g, threat).is_none() {
        return 0;
    }
    let mut cut = 0;
    for s in 0..g.seats.len() {
        // A walk, not a list: the heuristic asks this of every threat a position (103).
        cards::each_passive(w, g, s, |p| {
            if let Op::AutoCounter { source, amount } = p {
                if of_type(w, threat, *source) {
                    cut += *amount;
                }
            }
        });
    }
    cut
}

/// Returns every pooled contribution on `threat` to its payers.
pub(super) fn refund(g: &mut GameState, cx: &mut Cx, threat: &str) {
    let w = cx.world;
    for r in menu_resources(&w, threat) {
        for s in 0..g.seats.len() {
            let k = paid_key(threat, s, r);
            let n = g.var(&k);
            if n > 0 {
                cx.add_resource(g, s, r, n);
                cx.set_key(g, &k, 0);
            }
        }
    }
}

/// The tally name of what countered by `option`: a resource option its first
/// item's resource, else `tags` or `nodes`.
fn kind(option: &[CounterItem]) -> &'static str {
    if option.iter().any(|i| matches!(i, CounterItem::NodesNear { .. })) {
        return "nodes";
    }
    if option.iter().any(|i| matches!(i, CounterItem::Sacrifice { .. })) {
        return "tags";
    }
    match option.first() {
        Some(CounterItem::Pay { resource, .. }) => resource.name(),
        _ => "free",
    }
}

/// Counted in all, by how, and per round; and by tier and how (142).
pub(super) fn tally_countered(g: &mut GameState, cx: &mut Cx, threat: &str, how: &str) {
    let tier = def(&cx.world, threat).tier;
    cx.tally(g, "threats.countered", 1);
    cx.per_round(g, "threats.countered", 1);
    cx.tally(g, &format!("threats.countered_by.{how}"), 1);
    cx.tally(g, &format!("threats.countered.t{tier}"), 1);
    cx.tally(g, &format!("threats.countered_by.t{tier}.{how}"), 1);
    // 147: by the adversary's stage (I-III) too, so a read sees the tempo shift.
    let stage = g.var(adversary::STAGE);
    cx.tally(g, &format!("threats.countered.s{stage}.t{tier}"), 1);
}

/// Counters `threat` if the pooled payments cover one of its resource-only
/// options (the first such, menu order): that option is spent, everything
/// else goes back to the payers (later seats first), payers' Logistics AI
/// pays out, and the threat is discarded. Returns whether it was countered.
pub(super) fn try_complete(g: &mut GameState, cx: &mut Cx, threat: &str) -> bool {
    let w = cx.world;
    if scope_of(g, threat).is_none() {
        return false;
    }
    let req = requirement(&w, g, threat);
    let Some(k) = req
        .iter()
        .position(|o| resource_only(o) && resource_needs(o).iter().all(|(r, n)| paid(g, threat, *r) >= *n))
    else {
        return false;
    };
    let needs = resource_needs(&req[k]);
    let payers: Vec<usize> = (0..g.seats.len())
        .filter(|&p| needs.iter().any(|(r, n)| *n > 0 && g.var(&paid_key(threat, p, *r)) > 0))
        .collect();
    // 090: what counters cost beyond credits, the measure's (energy counters).
    for (r, need) in &needs {
        if *r != Resource::Credits && *need > 0 {
            cx.tally(g, &format!("threats.counter_paid.{r}"), *need);
        }
    }
    for r in menu_resources(&w, threat) {
        let need = needs.iter().find(|(x, _)| *x == r).map_or(0, |(_, n)| *n);
        let mut excess = paid(g, threat, r) - need;
        for p in (0..g.seats.len()).rev() {
            let key = paid_key(threat, p, r);
            let back = excess.min(g.var(&key));
            if back > 0 {
                cx.add_resource(g, p, r, back);
                excess -= back;
            }
            cx.set_key(g, &key, 0);
        }
    }
    // 099: what the Auto Counter took off (each item's need, floored at 0).
    if standing(&w, g, threat) > 0 {
        let unaided = resource_needs(&requirement_less(&w, threat, g.var(&jam_key(threat)))[k]);
        let cut: i64 = unaided.iter().zip(needs.iter()).map(|((_, a), (_, b))| a - b).sum();
        cx.tally(g, "threats.auto_countered", cut);
    }
    // An option a jam or standing counter cut to nothing was paid by no one.
    // 145: "jam" when the jam alone cut it (a jam read as the standing counter).
    let how = if needs.iter().all(|(_, n)| *n <= 0) {
        let jam_only = resource_needs(&requirement_less(&w, threat, g.var(&jam_key(threat)))[k]);
        if jam_only.iter().all(|(_, n)| *n <= 0) { "jam" } else { "standing" }
    } else {
        kind(&req[k])
    };
    tally_countered(g, cx, threat, how);
    clear(g, cx, threat);
    cx.note(None, || format!("{} is countered", def(&w, threat).name));
    for p in payers {
        for pa in cards::passives(&w, g, p) {
            if let Op::Logistics { resource, amount } = pa {
                cx.add_resource(g, p, *resource, *amount);
            }
        }
    }
    true
}

/// Whether seat `s` alone can pay `option` of `threat` now.
pub(super) fn can_pay_whole(w: &World, g: &GameState, s: usize, threat: &str, option: &[CounterItem]) -> bool {
    if !resource_needs(option).iter().all(|(r, n)| g.resource(s, *r) >= *n) {
        return false;
    }
    let tags = tag_cost(option);
    if !tags.is_empty() && !cards::TagNeed::new(&tags).payable(&cards::sources(w, g, s)) {
        return false;
    }
    let nodes = nodes_asked(option);
    // A counter never gives up the seat's last node.
    nodes == 0 || (nodes_near(w, g, s, threat).len() as i64 >= nodes && node_count(g, s) > nodes)
}

pub(super) fn tag_cost(option: &[CounterItem]) -> Vec<(TagOrAny, i64)> {
    option
        .iter()
        .filter_map(|i| match i {
            CounterItem::Sacrifice { sacrifice, amount } if *amount > 0 => Some((TagOrAny::Is(*sacrifice), *amount)),
            _ => None,
        })
        .collect()
}

pub(super) fn nodes_asked(option: &[CounterItem]) -> i64 {
    option
        .iter()
        .map(|i| match i {
            CounterItem::NodesNear { nodes_near } => (*nodes_near).max(0),
            _ => 0,
        })
        .sum()
}

/// Seat `s` counters `threat` with `option` of its menu, alone and in
/// full: tags sacrificed, nodes near it removed (its choice), resources
/// paid. Every pooled payment on it goes back.
pub(super) fn counter_with(g: &mut GameState, cx: &mut Cx, s: usize, threat: &Key, option: usize) -> Res {
    let w = cx.world;
    if scope_of(g, threat).is_none() {
        return Err(Halt::Illegal(format!("no active threat {threat}")));
    }
    let req = requirement(&w, g, threat);
    let Some(items) = req.get(option).cloned() else {
        return Err(Halt::Illegal(format!("{threat} has no counter option {option}")));
    };
    if !cx.replaying() {
        if resource_only(&items) {
            return Err("that option is paid by pooling: use Pay".into());
        }
        if !near(&w, g, s, threat) {
            return Err("only a player near the threat can counter it".into());
        }
        if !can_pay_whole(&w, g, s, threat, &items) {
            return Err("can't pay that option alone".into());
        }
    }
    let tags = tag_cost(&items);
    if !tags.is_empty() {
        cards::sacrifice_for(g, cx, s, &tags)?;
    }
    for k in 0..nodes_asked(&items) {
        let mine = nodes_near(&w, g, s, threat);
        let opts = mine.iter().map(|&h| labeled_hex_opt(&w, "remove node", h)).collect();
        let name = &def(&w, threat).name;
        let prompt = || format!("{name}: seat {s} removes which node near it ({} of {})?", k + 1, nodes_asked(&items));
        let Some(i) = cx.choose_loss(s, prompt, opts)? else {
            return Err(Halt::Illegal("no node near the threat to remove".into()));
        };
        board::lose_node(g, cx, mine[i], s, "counter");
    }
    for (r, n) in resource_needs(&items) {
        cx.add_resource(g, s, r, -n);
    }
    refund(g, cx, threat);
    let how = kind(&items);
    tally_countered(g, cx, threat, how);
    clear(g, cx, threat);
    cx.note(None, || format!("{} is countered", def(&w, threat).name));
    Ok(())
}
