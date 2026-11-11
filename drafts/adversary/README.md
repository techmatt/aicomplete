# Adversary drafts (the source of content/)

Draft content for the new adversary model (Matt, 2026-11-06): the universal threat decks (tiers 1-2; 146: no tier 3), the universal
project deck, and the Militarist's board and event deck. **These files are the source of truth**: the engine reads
`content/`, which `python scripts/content/load_adversary.py` writes from them (`--check`: exit 1 if `content/` is
stale); edit the drafts, never the loaded files. The format, vocabulary and value scale are in [the spec](../../experiments/results/134_militarist_drafts/spec.md);
the cards as printed are in [docs/adversary-cards.md](../../docs/adversary-cards.md) (134's listing.md is a write-up, no
longer regenerated).

| File | Deck |
|---|---|
| `threats_t1.json`, `threats_t2.json` | the threat deck of each tier (146 wrote both to Matt's threat card standard with [`threat_cards.py`](../../experiments/results/146_threat_standard/threat_cards.py), which also prints each card's give / get) |
| `projects.json` | the project deck (151: written with [`adversary_cards.py`](../../experiments/results/151_adversary_income_r65/adversary_cards.py)) |
| `militarist.json` | the Militarist's board (`board`) and event deck (`events`), first written by the same generator |

The 146 and 151 generators are records of their day: 166-178 edited these drafts by hand (rulings since), so rerunning
a generator would revert them. Edit the JSON here.

`python drafts/adversary/check.py` checks them: ids, deck sizes and tags, the tier rules, counters, the threat card
standard (two consequences and two counter options at most, no shared currency, no each-player harm, no special
pieces), the event standard (151: one Rally X, the fixed order threat, rally, deploy, other lines, corrupt, at most
two other lines, "each board" never "each country"), a project's two consequences at most, the event pull ladder, the harms
ledger's rule, and the draw rule (a random-deal simulation at 1-4 countries and every difficulty's stage counts,
plus Hall's condition at 4 countries: no Militarist pull may need a second reshuffle). `--content` runs the same
checks on the loaded `content/` (no value or check lines there, so those are skipped).
