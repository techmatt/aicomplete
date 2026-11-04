// `npm run probe -- DIR card_id...` (099): is a card the agents keep and never play ever legal?
// Replays every record in DIR (`aic-sim run ... --save DIR --save-when all`) through the wasm
// engine and, for each card id given, counts the steps a seat held it in hand, the steps a
// play of it was legal, and how often that seat then played it; the first legal step is shown
// with the seat's token piles. Legal and never played: the agent's valuation, not the engine.
// "Played" counts Play actions: a building Rush Build plays for free is not one.
// Needs web/src/wasm/ (npm run wasm). No browser, no server.
import { readFileSync, readdirSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const pkg = join(web, 'src', 'wasm');
const m = await import(pathToFileURL(join(pkg, 'aic_wasm.js')).href);
await m.default({ module_or_path: readFileSync(join(pkg, 'aic_wasm_bg.wasm')) });

const [dir, ...ids] = process.argv.slice(2);
if (!dir || !ids.length) {
  console.error('usage: npm run probe -- DIR card_id...');
  process.exit(2);
}
const pileOf = (st, key) => {
  const p = st.piles;
  if (Array.isArray(p)) return p.find((x) => x[0] === key)?.[1] ?? [];
  return p?.[key] ?? [];
};
const varsOf = (st) => (Array.isArray(st.vars) ? Object.fromEntries(st.vars) : st.vars ?? {});
const tally = Object.fromEntries(ids.map((id) => [id, { games: 0, held: 0, legal: 0, played: 0, first: null }]));
for (const f of readdirSync(dir).filter((x) => x.endsWith('.json')).sort()) {
  const rec = JSON.parse(m.parse_record(readFileSync(join(dir, f), 'utf8')));
  // A record from other rules plays other games here: say so (its counts are then this engine's).
  if (rec.rules_revision !== undefined && rec.rules_revision !== m.rules_revision())
    console.warn(`${f}: recorded at r${rec.rules_revision}, this engine plays r${m.rules_revision()}`);
  if (rec.config_error) { console.warn(`${f}: skipped, ${rec.config_error}`); continue; }
  // wasm-bindgen objects hold wasm memory until freed: take the state and free each.
  const take = (o) => { const s = o.state; o.free?.(); return s; };
  let state = take(m.start(JSON.stringify(rec.config), String(rec.seed)));
  const seen = new Set();
  for (const a of rec.actions) {
    const st = JSON.parse(state);
    const legal = JSON.parse(m.legal_actions(state));
    for (const id of ids) {
      for (let s = 0; s < st.seats.length; s++) {
        if (!pileOf(st, `cards.hand.${s}`).includes(id)) continue;
        const t = tally[id];
        t.held++;
        if (!seen.has(id)) { seen.add(id); t.games++; }
        if (legal.some((x) => x.type === 'play' && x.seat === s && x.card === id)) {
          t.legal++;
          t.first ??= { file: f, round: st.round, seat: s,
            tokens: Object.fromEntries(Object.entries(varsOf(st)).filter(([k]) => k.startsWith(`cards.tokens.${s}.`))) };
        }
        if (a.type === 'play' && a.seat === s && a.card === id) t.played++;
      }
    }
    try {
      state = take(m.apply(state, JSON.stringify(a)));
    } catch (e) {
      console.warn(`${f}: stopped replaying at ${JSON.stringify(a)}: ${e}`);
      break;
    }
  }
}
for (const [id, t] of Object.entries(tally)) {
  console.log(`${id}: held in ${t.games} games (${t.held} steps), legal ${t.legal} steps, played ${t.played}`);
  if (t.first) console.log(`  first legal: ${t.first.file} round ${t.first.round} seat ${t.first.seat}, piles ${JSON.stringify(t.first.tokens)}`);
}
