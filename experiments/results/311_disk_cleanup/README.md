# 311: disk cleanup

Measured at 1:55am and 2:05am on Nov 21 by a Python `os.scandir` walk that never follows a link
(a junction or symlink counts once, at its real home). MiB throughout.

| Location | Before | After |
|---|---:|---:|
| Main checkout `C:\Code\aicomplete` (links not followed) | 119.6 | 83.6 |
| · `.git` (one pack after gc; loose objects 33 MiB → 0) | 70.5 | 34.5 |
| · `experiments/` (tracked) | 41.7 | 41.7 |
| · everything else (crates, art, web, docs, scripts, `out/` 0.4) | 7.4 | 7.4 |
| Drive home `C:\Code\gdrive-sync\aicomplete` | 4347.0 | 4347.0 |
| · `art-originals/` (the junction's target) | 4188.9 | 1916.9 |
| · `archive/` | 158.1 | 2430.1 |
| Worktrees `C:\Code\aicomplete-worktrees\aicomplete` | 2854.6 | 2854.7 |
| · `112_price_correction` (red, kept; `target/` 2.5 GB, `out/` 226 MiB) | 2805.9 | 2805.9 |
| · `311_disk_cleanup` (this item) | 48.7 | 48.8 |

Moved into `archive/art-originals/`, same relative paths: every scene round's `cards.pdf`
(11 files, 1687.9 MiB) and `cards/` render folder (11 folders, 584.1 MiB), rounds r2 and b1–b10.
Both are scene-picker output (`npm run scene-picker` writes them for the round in `scenes.json`'s
`batch`) with the card faces of their day; nothing in the repo reads them back. The raw draws,
the cut scene PNGs the viewer serves and the picker previews stay in place.

History: 13,549 blobs, 43 MiB compressed; the largest is 0.9 MiB (a 289 screenshot). A history
rewrite would save almost nothing.

Found on the way: six scenes `scenes.json` gives a card have no PNG in any round, here or in the
archive (`antiproton_foundry_line`, `dark_fiber_collider`, `treaty_isotope_bank`,
`penning_trap_racks`, `cryo_sample_dewars`, `field_assay_tent`): never drawn, not moved.
