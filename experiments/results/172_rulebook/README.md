# 172: rulebook, first draft, and its friction table (r73)

No rules change: no engine, content or `docs/rules/` edit, no `RULES_REVISION` bump.

- [docs/rulebook.md](../../../docs/rulebook.md): the player-facing rulebook, 19 sections, each with a Sources line.
  It states the engine at r73 where the engine and `docs/rules/` disagree.
- [docs/rulebook-friction.md](../../../docs/rulebook-friction.md): 47 ranked friction entries, the length line, and
  46 engine-against-docs mismatches (M1-M46).
- `aic-sim docs --check` exempts both pages from the 15 KB budget (`run::BUDGET_EXEMPT`).

## How it was made

1. Nine read-only verifiers, one per rules area, each read its `docs/rules/` file and checked every claim in the code.
   Their outputs are the `verify-*.md` papers here: the area's rules as the engine plays them, mismatches with
   file:line, hard-to-state rules, and player-count and difficulty differences.
2. The rulebook was drafted from the rules files and corrected from the papers.
3. An independent review checked the draft: three accuracy reviewers (sections 1-6, 7-11, 12-18), one new-player and
   Spirit Island reader, and one completeness critic. Their wrong or misleading accuracy claims went to an adversarial
   verifier, one batch per reviewer. The findings are in [review-findings.md](review-findings.md).
   - 14 of 15 accuracy claims held and were fixed. The one refuted: Full Capacity "on your cards" already includes board
     tags by the rulebook's own definition.
   - The reader and completeness findings were applied, except those noted below.

## Not applied

- Capitalizing Node and Botnet in prose. The rulebook keeps the house style (lowercase in prose, as in the rules docs).
- Renaming Gather and Push. They are the house terms (r70). The overlap with Spirit Island's verbs is friction row 23's
  concern, with the other doubled words.

## Numbers

Words: core rules (sections 1-17, without Sources lines) 7,975; legend and glossary 1,001; whole page 9,170. Counted
with a word regex over the Markdown (`[A-Za-z0-9][\w'-]*`). The Spirit Island comparison (6,000-8,000 words) is an
estimate from the rulebook's page count, not a count.
