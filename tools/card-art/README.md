# Card art

Paths are from the repo root unless a recipe says otherwise.

The card renderer is `web/card/` (`card.js`, `card.css`, `icons.js`, the generated
`faces.json`, `layout.json`, `containers/`, bundled `fonts/`): `renderCard(card, layout, frameUrl, fit, artUrl)`
turns card content into HTML, in mm, so one template serves print and, later, the web
client. Fonts are fixed: Saira Condensed titles and figures, Source Serif 4 body (Matt's pick,
`picks/card_fonts_r1.json`). **Sizes are measured, not guessed:** `renderCard` leaves 7.5 pt text
and a 15 pt title, and `await fitCards(root)` sizes the cards in `root` once they are in the
page. Each box (a building's top box, the effect box below the art) walks the ladder alone,
largest first, 7.5 / 6.9 / 6.3 / 5.8 pt then `dense` 5.4 (which shrinks its icons too), until
its icon rows and text leave 1.6 mm of its height. A title stays on one line and shrinks in
0.25 pt steps until it clears its plate by 1 mm a side, floor 10.5 pt. A caller that forgets gets
the default sizes, so a long card overflows and a long title runs wide. The viewer calls it
after drawing the grid or the large view; `show` in `tools/card-art/lib.mjs` calls it, so
`render`, `scene-picker` and every print get it. In Playwright, `page.evaluate(fitCards)`; it
loads the fonts itself, and measures a copy at true size, so zoom and device scale don't change
a card. About 110 ms for 159 cards. **Cards are composed live, never pre-baked:** composed cards and print sheets go
to `out/cards/` (git-ignored). Committed: frame templates (`art/frames/`), guides
(`art/guides/`), sliced containers, fonts. Changing a card's text or numbers changes no
asset. Picker previews are never committed: they go to the shared
`art-originals/picks/<picker>/`. Card art prompts live in
`tools/card-art/scenes.json`, apart from content, so changing a card's mechanics changes
neither its art nor its prompt.

**One master.** Every frame derives from `art/frames/r3/master.jpg` (`steel-event-3`), so borders, bevels and plates match. `npm run derive` makes the six
variants in `art/frames/r3/`, numbers in `tools/card-art/derive.json`: buildings restack
the frame's inside from master rows, adding a divider above the art; tier 2 tints the
steel gold; the enamel, found by hue, goes through the kind's color ramp (event keeps the
master's). The frame holds the fixed structure; the renderer draws what varies: the
insight hexagon, requirement plates and 0–3 tags.

**Where it sits is what it is** (locked by Matt, 2026-10-29). Top strip: cost octagon,
insight hexagon, requirements, tags, nothing else. Above the art (buildings): what lasts,
income, *Effect:*, *Action:*. Below the art: what happens once, extra costs first, as
minus-icons. No box: gain now. Brown box: income. Dark frame: the adversary's pieces or
projects. Dashed outline: another player; thin solid outline: counted across your cards. A square tile is a resource, a disc a
token on a card, a bare silhouette a board piece, a ringed circle a tag. In the dark frame a
clock is a delay, a minus on the jam mark a jam. All marks: the legend in [docs/card-guidelines.md](../../docs/card-guidelines.md). The cost group
follows Race for the Galaxy: a hexagon at about 70% of the octagon, in front of its lower
right, with a halo in the strip color; at 0 insight it is omitted.

**Icons.** Containers are one painted sheet in the master's steel (`npm run containers --
--draw`, 65 s; without the flag it reslices the kept sheet into `web/card/containers/`).
Glyphs are vector, in `icons.js`. Tiles and discs are one blank each: `card.css` multiplies
the face to the resource's color.

**Faces.** Generated, never written: `crates/aic-core/src/face.rs` turns a card's ops into
its icon lines and rules text, and `aic-sim docs` writes them to `web/card/faces.json`, one
card a line. A card's `text` in `content/cards.json` is refused at load, and the fast lane
fails if `faces.json` is stale. The grammar is in docs/rules/card-grammar.md. The
renderer fails on an icon word it doesn't know. `tools/card-art/samples.json` says which
card shows each variant and which buildings show the splay.

**Trim and splay.** `layout.json` is the frame as painted. `derive` scales every frame in,
so the outer steel edge lands 0.5 mm inside the trim (`inset_mm` 3.5, 90% wide, 92.6% tall),
and fills the bleed by mirroring the steel band. `inset()` in `card.js` carries the zones
with it. The splay offset is the distance from the trim to the banner: only a covered
card's strip shows, and the newer card covers its name.

Run from `tools/card-art/` (Node 24, system Chrome through `playwright-core`, one
browser; `npm ci` first):

- `npm run derive`: the six frames from the master, about 5 s.
- `npm run fit`: measures each frame's zones into `art/frames/r3/fit.json`, which the
  renderer uses instead of the guide, and prints any that differ from the master. Some
  octagons measure one pixel (0.08 mm) off it: flood-fill noise, the geometry is the same.
- `npm run render -- [variants] [--pdf]`: composes each variant's real card at 300 dpi
  (815×1110). With no variants named, also the splay, the icon sheet and a full check of
  every card in content (about 1.5 min: background it); with `--pdf`, `all-cards.pdf`
  (every real card at true size, 9 to an A4 page, 18 pages) and `splay.pdf`. Fails if a font fell back,
  anything printed leaves the safe zone (x 6–63, y 6–88 mm), a panel or title overflows or a
  requirement plate reaches the tags.
- `node sheet.mjs <out.png> <images…>`: a one-row inspection sheet.
- `npm run picker`: writes `picks/card_frames_r3.json` from `out/cards/`, with its images
  in `art-originals/picks/card_frames_r3/`. Stops if that folder isn't reachable (see
  *Originals*).
- `npm run guides`: six labeled guides plus unlabeled `-blocks` twins, from
  `layout.json`. The twins are what the image tool sees: labels invite text.
- `npm run gen -- steel-program-1 … --pool=3`: raw frame draws for a new direction: Codex CLI's `$imagegen` on Matt's ChatGPT Plus login, the wordwarriors route
  (`codex exec --json -s read-only`, the prompt on stdin, the guide on `-i`, the PNG
  collected from `~/.codex/generated_images/<thread>/`). No API key. Prompts are in
  `prompts.json`. It stops at the first usage-limit message. Its frames go to
  `art/frames/r3/`, which `fit` and `render` read by variant name only, so a draw is
  ignored until it is copied over `master.jpg`.

- `npm run scene-gen -- [scene_id …] [--pool=N]`: draws card scenes, the same Codex route,
  into `art-originals/scenes/<round>/raw/`, the whole of `batch` with no ids named. The pool
  (one scene per real card, grouped by tag) and the one preamble are in
  `tools/card-art/scenes.json`: a scene's `card` is a suggestion, not
  a binding. Draws run 42–85 s each at 3 at once: 36 took about 11.5 min, about 10% of the
  window. A style
  line alone comes back as detailed concept art; the `emphasis` line (never a
  photograph or 3D render, and what to do instead) is what pushes a draw toward illustration.
  "Near-future setting" alone gives a historical foundry and village: the preamble
  says clean modern machinery, clothes and buildings, never historical.
- `npm run scene-picker`: from the raw draws, each scene cut to the program window's shape
  (`<id>.png`), its two cards from `batch` at 300 dpi (`cards/`), the same at true size
  (`cards.pdf`, about 260 MB for 80 cards), and `picks/<picker>.json` with previews, about
  45 s for 24, or about 2 min when writing through the Drive folder, about 3 min for 40
  or 60 (cards.pdf about 400 MB for 120 cards):
  background it. `npm ci` first in a fresh worktree. Runs at below-normal priority, one
  image at a time. Each new batch replaces `batch` in `scenes.json`, so the scenes already
  drawn are the row ids in `picks/scene_batch_<n>.json`. Passes the scene to
  `renderCard`'s `artUrl`, which center-crops it to the window `fit` measured: 53.3×33.2 mm
  for program and event, 53.3×25.3 mm for buildings, so a building shows the middle 76%.
- Every one of the 154 deck cards has a suggested scene, drawn. A scene whose card was cut or
  redrawn has `was` in place of `card`: drawn, in the pool, suggested for nothing. A scene with
  both was drawn for `was` and reused on `card`: an unused scene that broadly fits a new card's
  name goes there before a new draw (Matt, 110). `npm run cards:check` in `web/` fails on a deck
  card with no scene or no served image. The viewer
  serves a scene id from the newest `scenes/b<n>/` that has it. Three ids (`armored_conduit_trench`,
  `grant_check_racks`, `data_license_handover`) were respelled by copying the drawn files to the
  new name; the old files stay for the old pickers.
- **Originals:** `art-originals/` is a git-ignored junction to
  `C:\Code\gdrive-sync\aicomplete\art-originals` (`mklink /J art-originals <that path>`).
  The scripts other than the pickers fall back to that path. The pickers stop without the
  junction, because their manifests link `../art-originals`. `AIC_ART_ORIGINALS` overrides
  the folder for all of them: a picker then reads and writes images there, warns, and its
  manifest still links `../art-originals`, so the images show only where the junction
  points at that folder. `npm run scene-picker` stops first, naming the ids, if a raw draw
  is missing. Each round has its folder (`card-frames-r3/` holds the
  container sheet): full-size originals, `frames/` (815×1110 PNG) and `log.jsonl`.
  Retired rounds (`card-frames-r1`, `-r2`, `scenes/r1`) live in
  `C:\Code\gdrive-sync\aicomplete\archive\art-originals\`, same relative paths, outside the
  junction. `scenes/r1` there is the raw draws only: to rebuild the rest, export commit
  `06ef387`, copy `raw/` into its `art-originals/scenes/r1/`, and run `npm run scene-picker`.
- **The master's outer steel band lies in its bleed;** `derive` is what brings it inside
  the trim. Composed singles show the full bleed; the splay is trimmed.
