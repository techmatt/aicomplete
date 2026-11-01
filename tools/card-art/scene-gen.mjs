// `npm run scene-gen -- [scene_id …] [--pool=N]`: draw card scenes (scenes.json), the whole
// batch with no ids named. A redraw replaces it; the earlier original stays as
// <id>.try<n>.png. Originals and log.jsonl go to art-originals/scenes/<round>/raw/, never
// committed. Stops at the first usage-limit message. Exits 1 if a draw failed or the limit
// left some undrawn.
import { mkdir } from 'node:fs/promises';
import path from 'node:path';

import { IMAGE_PREAMBLE, ORIGINALS, TOOL, drawAll, parsePool, readJson, runCodex } from './lib.mjs';

const argv = process.argv.slice(2);
const pool = parsePool(argv, 'scene-gen');
const named = argv.filter((a) => !a.startsWith('--'));
const s = await readJson(path.join(TOOL, 'scenes.json'));
const ids = named.length ? named : Object.keys(s.batch);
const scenes = Object.fromEntries(Object.entries(s.scenes).flatMap(([tag, group]) => Object.entries(group).map(([id, scene]) => [id, { ...scene, tag }])));
const unknown = ids.filter((id) => !scenes[id]);
if (unknown.length) throw new Error(`scene-gen: no scene ${unknown.join(', ')}: nothing drawn`);
const dir = path.join(ORIGINALS, 'scenes', s.round, 'raw');
await mkdir(dir, { recursive: true });

const scenePrompt = (scene) =>
  [
    ...IMAGE_PREAMBLE,
    'Image shape: landscape, 3:2.',
    '',
    `${s.preamble} ${s.emphasis} ${s.lighting} ${s.framing} ${scene.subject} The tag color is ${s.tags[scene.tag].accent}.`,
  ].join('\n');

await drawAll(ids, pool, 'scene-gen', {
  dir,
  draw: async (id) => ({ run: await runCodex(scenePrompt(scenes[id])) }),
  kept: (id, dest, run) => console.log(`scene-gen: ${id}: ${run.seconds}s, usage ${JSON.stringify(run.usage)}`),
});
