// Smoke test for the wasm package: plays one game through the JSON API.
// Usage (after `wasm-pack build crates/aic-wasm --target web`):
//   node scripts/wasm-smoke.mjs [config.json]
import { readFileSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pkg = path.join(root, 'crates', 'aic-wasm', 'pkg');
const m = await import(pathToFileURL(path.join(pkg, 'aic_wasm.js')).href);
await m.default({ module_or_path: readFileSync(path.join(pkg, 'aic_wasm_bg.wasm')) });

const cfg = readFileSync(process.argv[2] ?? path.join(root, 'configs', 'baseline.json'), 'utf8');
const resolved = JSON.parse(m.resolve_config(cfg));
const board = JSON.parse(m.board(cfg));
let state = m.new_game(cfg, '18446744073709551615');
const started = m.start(cfg, '18446744073709551615');
const sameStart = started.state === state && JSON.parse(started.events).length > 0;
const taken = [];
// The golden policy: action k is legal action (k * 7919) % count (see golden_game).
for (let k = 0; ; k++) {
  const legal = JSON.parse(m.legal_actions(state));
  if (legal.length === 0) break;
  const a = legal[(k * 7919) % legal.length];
  state = m.apply(state, JSON.stringify(a)).state;
  taken.push(a);
}
const actions = taken.length;
const s = JSON.parse(state);

// The generic view and the game record (what web/ uses).
const view = JSON.parse(m.view(state));
const record = m.record_text(state, JSON.stringify(taken), 'smoke');
const parsed = JSON.parse(m.parse_record(record));
const checked = JSON.parse(m.check_record(record));
const viewOk = view.hexes.length === board.hexes.length && view.seats.length === s.seats.length
  && JSON.stringify(view.outcome) === JSON.stringify(s.outcome);
const recordOk = parsed.seed === s.seed && parsed.actions.length === actions && checked.ok
  && checked.warnings.length === 0 && !parsed.config_error;
console.log(`view ${viewOk ? 'ok' : 'BAD'}, record ${recordOk ? 'ok' : 'BAD'} (${record.length} bytes), start ${sameStart ? 'ok' : 'BAD'}`);

// Must equal GOLDEN_FINGERPRINT in crates/aic-core/src/tests/mod.rs (same config,
// seed and policy), proving native and wasm builds play the same game.
const GOLDEN = 'f0ea402cf0d5fc6a';
// Must equal GOLDEN_CONTENT_HASH there: this game may never pay for a repriced card.
const GOLDEN_CONTENT = 'cfdd2bbc39ded04b';
// 148: the state's play less its measures (`stats`), computed in the wasm build.
const fingerprint = m.fingerprint(state);
const golden = process.argv[2] ? true : fingerprint === GOLDEN && parsed.content_hash === GOLDEN_CONTENT;
console.log(`final state fingerprint ${fingerprint}${process.argv[2] ? '' : ` (golden ${GOLDEN})`}`);
if (!process.argv[2]) console.log(`content hash ${parsed.content_hash} (golden ${GOLDEN_CONTENT})`);

let threw = false;
try {
  m.apply(state, JSON.stringify({ type: 'pass', seat: 0 }));
} catch (e) {
  threw = typeof e === 'string';
}
console.log(`engine ${m.version()}, config ${resolved.hash}, ${board.hexes.length} hexes`);
console.log(`seed ${s.seed}: ${actions} actions, outcome ${JSON.stringify(s.outcome)}`);
if (!s.outcome || s.seed !== '18446744073709551615' || !threw || !golden || !viewOk || !recordOk || !sameStart) {
  console.error('wasm smoke test FAILED');
  process.exit(1);
}
console.log('wasm smoke test ok');
