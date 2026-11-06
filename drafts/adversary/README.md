# Adversary drafts (134, for review)

Draft content for the new adversary model (Matt, 2026-11-06): the universal threat decks (tiers 1-3), the universal
project deck, and the Militarist's board and event deck. **These files are the source of truth**: the engine reads
`content/`, which `python scripts/content/load_adversary.py` writes from them (`--check`: exit 1 if `content/` is
stale); edit the drafts, never the loaded files. The format, vocabulary and value scale are in [the spec](../../experiments/results/134_militarist_drafts/spec.md), the readable listing (every card, the
ops the engine lacks, the harms ledger across the drafts) in
[listing.md](../../experiments/results/134_militarist_drafts/listing.md).

| File | Deck |
|---|---|
| `threats_t1.json`, `threats_t2.json`, `threats_t3.json` | the threat deck of each tier |
| `projects.json` | the project deck |
| `militarist.json` | the Militarist's board (`board`) and event deck (`events`) |

`python drafts/adversary/check.py` checks them: ids, deck sizes and tags, the tier rules, counters, the harms
ledger's rule, and the draw rule (a random-deal simulation at 1-4 countries and every difficulty's stage counts,
plus Hall's condition at 4 countries: no Militarist pull may need a second reshuffle). `--content` runs the same
checks on the loaded `content/` (no value or check lines there, so those are skipped). `--write` also rewrites the
listing and the picker (`picks/adversary_drafts.json`, images in `art-originals/picks/adversary_drafts/`).
