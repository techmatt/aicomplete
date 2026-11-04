# 099 bug hunt: fixed (r38 → r39)

The findings the 099 hunt fixed, numbered on from the 044 hunt's F100. The open and won't-fix ones are in
[docs/qa-findings.md](../../../docs/qa-findings.md); the changes are in git.

| # | Fix |
|---|---|
| F101 | Rush Build isn't offered when no building stays playable once its energy and a sacrifice are paid, and offers only such sacrifices (r39). |
| F102 | The Harden action a card grants shows its card's chips (`seat_chips`), not "costs nothing". |
| F103 | The four megaproject texts say the converter needs a node in a city (r37's gate). |
| F104 | Free a station has its row in rules.md §6's standard actions (a docs test checks every action has one). |
| F105 | A player's station that holds isn't a cyber source made (`threats.sources_held`; the attempt still uses the round's source). |
| F106 | `board.wall_strikes` counts once a troop a march, not every step beside the Wall. |
| F107 | Agents: a one-shot Action (Popular Uprising) earns nothing a round standing or buried, so using it isn't a loss. |
| F108 | Agents: the flat liberate-progress term reads either half of an Action (dead since r37). |
| F109 | Buildings played by Rush Build count as played (`cards.played.<id>`, games.jsonl), not `ai.played.`. |
| F110 | The dear-power measure (`ai.dear.*`, use_read, economy_read) counts a tag-and-energy power that places no node (silent since r35). |
| F111 | energy_read and r37_read read every seat and skip a round the game ended in; r38_read names agent and difficulty. |
| F112 | Viewer: an Action's second half in words (not `CashOut`), "+4 research cubes" whole, a use button with its Action. |
| F113 | Stale words: rules.md §2's lead, the produce phase doc, the gift loop, the cost curve, README on `configs/agents/`, agents.md's tiers (r38), experiments.md's per-round keys. |
| F114 | e2e:corps places a node on a city before its converter checks (r37). |
| F115 | A report on one difficulty says its game length once, not "7.3 at difficulty 2, 7.3 at 2". |
| F116 | At one country the energy gift Actions (Substation Hub, Treaty Grid Exchange) aren't offered: they took an energy and gave nothing (r39). |
| F117 | Fieldwork (or any lone draw) isn't offered with both tier-1 piles empty (r39). |
| F118 | The dear-power measure counts a missing tag when energy is short too (`unpaid` hid `no_tags`). |
| F119 | `threats.auto_countered` tallies what the Auto Counter took off, not its printed 5 (a 1 + 1 energy threat). |
| F120 | A player's station that holds logs what happened: a landing, a cancel against a player ambassador, or an empty supply. |
| F121 | Viewer: under Patience the insight income's hover says it adds, not "is set to this". |
| F122 | rules.md: the fallback needs one more ambassador for each adversary ambassador on the city (§2, §8, the new-player summary); a corrupted city diverts whatever stands on it. |
| F123 | The 099 review's: Rush Build's sacrifice prompt, Accord as a dear power, a country name lowercased by the op-name rewrite, the probe's wasm leak, the read gates, the cost curve. |
| F124 | Viewer: a session or record that stops replaying isn't saved back truncated (the next load showed the cut game as whole); the record dialog says the copy holds only what replayed, and stops autoplay. |
| F125 | Viewer: a session saved under another rules revision says so on restore (sessions now carry `rules`). |
| F126 | Card viewer: a reasoned-price CSV needs a row a card (the ledger's `instances.csv`, a row an effect, set 159 cards' "Reasoned" to one effect's value). |
| F127 | `aic-sim replay` names a revision mismatch beside a replay error. `npm run probe` warns on other-revision records and names a file that stops replaying. |
| F128 | The sweep's end capture keeps that step's new key events in its label. `run_sim.ps1` keeps PowerShell comma lists (`--variants a,b`). Interceptor Battery's pricing note: a troop in one country. |
| F129 | Generated docs: the standard actions price the ambassador in credits and say what Align does to a dark city; the Sway table adds Clear and Delta Seawall and says `ambassador_discord` is per unit; the harms ledger marks uncountered threats' Discord and a player station that holds; ops.md's `flip_station`; rules.md §3 names every botnet remover. |
| F130 | The 099 review of round 4's fix: a session that stopped replaying is not saved by a seek, a key or autoplay either (`Timeline.save` holds until an action or undo). rules.md §6-7: a tag and an energy buy the dear powers; Overcharge and Particle Run among the sources; Tidal Barrage grows. |
| F131 | Viewer: a token pile of one reads "1 follower" ("1 followers", "1 datasets" before). |
