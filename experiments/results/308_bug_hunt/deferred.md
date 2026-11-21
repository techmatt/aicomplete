# 308 bug hunt: deferred to Matt (D136 on)

Most important first. docs/qa-findings.md keeps the open list.

- **D136** (rules, Matt's call) A cleanse frees a corrupted station or factory but leaves its agents
  (`removal.rs` cleanse: a city goes through `liberate_site`, a facility only loses its cube; `cleansable` checks
  troops, not agents). The Nuclear Option's "claim a site next to it" rider then can't take the freed factory beside
  its city, which usually keeps 1 agent (a factory whose city fell keeps a guard). Every cleanse card (Great
  Liberation, Reclamation Crew ...) does the same; it settles D80's undefined "free". Options: (a) treat a facility as
  a city (agents off, its free_facility Unity), a bump; (b) leave and word the faces "its agents stay"; (c) refuse a
  facility with an agent as a cleanse target. **Pick (a)**: one meaning of "free" for every site, as Liberate's.
- **D137** (cards, Matt's call) Siegebreaker Reactor (2 troops) and Peacekeeping Mission (3) may target a city with
  more guards than they remove: the antimatter is spent and nothing is rescued (the words now say "rescues only if no
  guard is left", F440). Options: offer only cities the removal clears (a bump, the agents' choices move); leave.
  **Pick:** offer only cities it clears: a player never pays a sink for nothing.
- **D138** (rules, Matt's call) At a research loss the Discord track can read 26/16: the corrupted cities' Discord
  lands in the compute step and the research check ends the game in the same phase, before any tip. Rules-correct
  (alignment.md: the loss is found in its research check). Options: a line on the end banner; a ruling that a full
  track tips first. **Pick:** leave; the banner's reason names the research loss.
- **D139** (viewer, Pick) In the cap's last round the map still draws the Escalate "+1" badges and projects read
  "completes next compute step", though no adversary turn follows the next flip (F430 fixed the Next box). Options:
  suppress the forecasts when deck and next are both empty; leave. **Pick:** suppress.
- **D140** (viewer, Pick) The turn panel sums a Redeploy as "+2 adversary troops, -2 adversary troops" (netted per
  hex); two troops moved. And two troops on one hex ask the same Strike question twice with no "(2nd of 2)".
  Options: name a move ("2 troops moved"), number the repeated question; leave. **Pick:** both, a viewer pass.
- **D141** (agents, Pick) `agents/moves.rs`' push and strike heuristics (`push_from`, the strike-gather's `finds`)
  rank a hex by the agents on and around it; since F424-F425 a hex reached only through an ambassador gives up troops
  only, so they can aim Kettle or Patrol there over a richer in-reach hex. Options: mirror `pushable` and the per-hex
  limit in the heuristics (a 16-seed smoke on and off); leave (rare). **Pick:** mirror, in an agents item.
