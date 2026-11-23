// 321: plays named cards in the player view, the way a card pass checks "does it do what its text says" in the
// viewer. The cards are staged in a rigged three-seat game (configs/three_seat.json): each named card's tier deck
// holds the named cards only (the other tier keeps its whole deck), their `needs` are cleared by a content overlay,
// and every seat starts with 100 credits and 20 energy. Each round the seats draft from a named card's tier and keep
// named cards; in the action phase each named card a seat can play is played by clicking its Play button (the seat's
// row first), each named building's Action by its Use button, every question answered by its first answer (a hex:
// the one holding the most adversary pieces on or next to it). Nothing is judged: the report lists, per card, what
// each play and use did (the engine's notes, each seat's resource changes, pieces added or taken off the map, cards
// moved), the questions it asked and a screenshot after it; a card never played says why (its status in hand).
//
//   node scripts/cards-play.mjs ID[,ID...] [--seed 3] [--rounds 6] [--out DIR] [--size 1366x768]
//
// Needs a served viewer (AIC_URL). Writes OUT/summary.json, OUT/report.md and OUT/<card>-<n>.png; fails (exit 1) on
// a console error or a named card that never played. Reading the report against each card's face is the check.
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openPage, viewerUrl } from './cdp.mjs';

const arg = (k, d) => {
  const i = process.argv.indexOf(`--${k}`);
  return i > 0 ? process.argv[i + 1] : d;
};
const web = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const repo = join(web, '..');
const ids = (process.argv[2] ?? '').startsWith('--') ? [] : (process.argv[2] ?? '').split(',').filter(Boolean);
if (!ids.length) throw new Error('usage: node scripts/cards-play.mjs ID[,ID...] [--seed N] [--rounds N] [--out DIR]');
const seed = arg('seed', '3');
const rounds = Number(arg('rounds', '6'));
const out = arg('out', join(repo, 'out', 'cards-play'));
const [width, height] = arg('size', '1366x768').split('x').map(Number);

const content = JSON.parse(readFileSync(join(repo, 'content', 'cards.json'), 'utf8'));
const defs = new Map((content.cards ?? content).map((c) => [c.id, c]));
for (const id of ids) if (!defs.has(id)) throw new Error(`no card ${id} in content/cards.json`);
const config = JSON.parse(readFileSync(join(repo, 'configs', 'three_seat.json'), 'utf8'));
const named = new Set(ids);
const tiers = new Set(ids.map((id) => defs.get(id).tier ?? 1));
for (const m of config.modules) {
  if (m.module === 'cards') m.cards = m.cards.filter((c) => !tiers.has(defs.get(c)?.tier ?? 1) || named.has(c)).concat(ids.filter((id) => !m.cards.includes(id)));
  if (m.module === 'ai') Object.assign(m.params, { start_credits: m.params.start_credits.map(() => 100), start_energy: 20 });
}
config.content = [...(config.content ?? []), ...ids.filter((id) => defs.get(id).needs?.length).map((id) => `cards.${id}.needs=[]`)];
config.name = 'cards-play';

/** In the page: the moves and what each did. */
const LIB = `(() => {
  const named = new Set(${JSON.stringify(ids)});
  const tierOf = ${JSON.stringify(Object.fromEntries(ids.map((id) => [id, defs.get(id).tier ?? 1])))};
  const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0; };
  const count = (h, k) => h.tokens.find((t) => t.kind === k)?.count ?? 0;
  const near = (h) => [h.index, ...h.neighbors].reduce((n, i) => n + aic.view.hexes[i].tokens.filter((t) => /adversary|troop|botnet/.test(t.kind)).reduce((a, t) => a + t.count, 0), 0);
  const isHex = (t) => Array.isArray(t) && t.length === 2;
  const button = (a) => [...document.querySelectorAll('#player button[data-action]')].filter(vis).find((b) => b.dataset.action === JSON.stringify(a));
  window.__cp = {
    /** Draft and keep through the engine: a named card's tier, its card, keep it; everything else passes. */
    toActions(n = 300) {
      for (let i = 0; i < n && !aic.view.outcome; i++) {
        if (/board\\.act/.test(aic.view.phase?.name ?? '') && !aic.view.choice) return true;
        const L = aic.legal;
        const want = new Set(Object.values(tierOf).map(String));
        // A keep costs insight the plays need: a seat keeps a named card while it holds fewer than two.
        const held = (s) => (aic.view.seats[s].cards.find((g) => g.key === 'cards.hand')?.cards ?? []).filter((c) => named.has(c.id)).length;
        const a = L.find((x) => x.type === 'tier' && want.has(String(x.tier))) ?? L.find((x) => x.type === 'draft' && named.has(x.card))
          ?? L.find((x) => x.type === 'draft') ?? L.find((x) => x.type === 'keep' && named.has(x.card) && held(x.seat) < 2) ?? L.find((x) => x.type === 'pass')
          ?? L.find((x) => x.type === 'choose') ?? L[0];
        if (!a) return false;
        aic.act(a);
      }
      return false;
    },
    /** The next named card to play or use: { action, card, kind } or null. */
    next(done) {
      return aic.legal.filter((a) => (a.type === 'play' || a.type === 'use') && named.has(a.card) && !done.has(a.type + ':' + a.seat + ':' + a.card))
        .map((a) => ({ action: a, card: a.card, kind: a.type }))[0] ?? null;
    },
    /** Clicks the player view's control for \`a\` (its seat's row first); true when it clicked one. */
    click(a) {
      if (!button(a) && aic.focus !== a.seat) [...document.querySelectorAll('#player [data-seat-focus="' + a.seat + '"]')].find(vis)?.click();
      const b = button(a);
      if (!b) return false;
      b.click();
      return true;
    },
    /** Answers the open question by its control: a hex on the map, else its button. */
    answer() {
      const ch = aic.legal.filter((a) => a.type === 'choose' && a.seat === aic.view.choice.seat);
      const hexes = ch.filter((a) => isHex(a.target));
      const of = (a) => aic.view.hexes.find((h) => h.hex[0] === a.target[0] && h.hex[1] === a.target[1]);
      const a = hexes.length ? hexes.reduce((b, x) => (near(of(x)) > near(of(b)) ? x : b)) : ch[0];
      const prompt = aic.view.choice.prompt;
      if (isHex(a.target)) document.querySelector('#board .hex[data-hex="' + a.target.join(',') + '"]').dispatchEvent(new MouseEvent('click', { bubbles: true }));
      else if (button(a)) button(a).click();
      else aic.act(a);
      return prompt + ' -> ' + (a.option ?? (isHex(a.target) ? 'hex ' + a.target.join(',') : JSON.stringify(a)));
    },
    /** What the frames from \`from\` on did, in words. */
    did(from) {
      const tl = aic.timeline, notes = [], res = new Map(), tok = new Map(), moved = [];
      for (let i = from; i <= tl.last; i++) {
        for (const e of tl.frames[i].events) {
          if (e.type === 'note') notes.push(String(e.text));
          else if (e.type === 'resource_changed') res.set('seat ' + e.seat + ' ' + e.resource, (res.get('seat ' + e.seat + ' ' + e.resource) ?? 0) + Number(e.delta));
          else if (e.type === 'tokens_changed') tok.set(String(e.token), (tok.get(String(e.token)) ?? 0) + Number(e.delta));
          else if (e.type === 'card_moved') moved.push(e.card + ': ' + e.from + ' -> ' + e.to);
        }
      }
      const fmt = (m) => [...m].filter(([, d]) => d).map(([k, d]) => k + ' ' + (d > 0 ? '+' : '') + d);
      return { notes, resources: fmt(res), map: fmt(tok), cards: moved };
    },
    /** Each named card's status wherever a seat holds it. */
    statuses() {
      const out = {};
      for (const s of aic.view.seats) for (const g of s.cards) for (const c of g.cards) if (named.has(c.id)) out[c.id] = 'seat ' + s.index + ' ' + g.key.replace('cards.', '') + ': ' + c.status;
      return out;
    },
    /** Every seat uses its node powers, each node toward the most adversary pieces (a target card needs one in reach). */
    nodes() {
      for (let k = 0; k < 12; k++) {
        const p = aic.legal.find((a) => a.type === 'power' && aic.view.seats[a.seat].board?.cards.find((c) => c.id.endsWith('.power.' + a.power))?.node);
        if (!p) return k;
        aic.act(p);
        for (let q = 0; q < 6 && aic.view.choice; q++) this.answer();
      }
      return 12;
    },
    passAll() { while (/board\\.act/.test(aic.view.phase?.name ?? '') && !aic.view.choice) { const p = aic.legal.find((a) => a.type === 'pass'); if (!p) break; aic.act(p); } },
  };
  return true;
})()`;

mkdirSync(out, { recursive: true });
const page = await openPage({ width, height });
let ok = false;
try {
  await page.send('Page.navigate', { url: viewerUrl });
  await page.until('window.aic && window.aic.view', 'the viewer to start', 30000);
  await page.js(`window.aic.setView('player')`);
  await page.js(`window.aic.play(${JSON.stringify(JSON.stringify(config))}, '${seed}', 'cards-play.json')`);
  await page.until(`window.aic.timeline.seed === '${seed}'`, `the seed-${seed} game`, 30000);
  await page.js(LIB);
  const report = Object.fromEntries(ids.map((id) => [id, { plays: [], lastStatus: null }]));
  const done = new Set();
  for (let r = 0; r < rounds; r++) {
    if (!(await page.js('window.__cp.toActions()'))) break;
    const round = await page.js('aic.view.round');
    await page.js('window.__cp.nodes()');
    for (let k = 0; k < 40; k++) {
      const n = await page.js(`window.__cp.next(new Set(${JSON.stringify([...done])}))`);
      if (!n) break;
      done.add(`${n.kind}:${n.action.seat}:${n.card}`);
      const from = await page.js('aic.timeline.last + 1');
      if (!(await page.js(`window.__cp.click(${JSON.stringify(n.action)})`))) {
        report[n.card].plays.push({ round, seat: n.action.seat, kind: n.kind, problem: 'no control in the player view' });
        continue;
      }
      const asked = [];
      for (let q = 0; q < 12 && (await page.js('!!aic.view.choice')); q++) asked.push(await page.js('window.__cp.answer()'));
      const name = `${n.card}-${report[n.card].plays.length + 1}`;
      await page.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: 1, y: 1 });
      writeFileSync(join(out, `${name}.png`), await page.shot());
      report[n.card].plays.push({ round, seat: n.action.seat, kind: n.kind, asked, ...(await page.js(`window.__cp.did(${from})`)), shot: `${name}.png` });
    }
    for (const [id, st] of Object.entries(await page.js('window.__cp.statuses()'))) report[id].lastStatus = `round ${round}: ${st}`;
    // Every seat's uses again next round.
    for (const k of [...done]) if (k.startsWith('use:')) done.delete(k);
    await page.js('window.__cp.passAll()');
  }
  const never = ids.filter((id) => !report[id].plays.some((p) => p.kind === 'play' && !p.problem));
  const lines = [`# cards-play: ${ids.length} cards, seed ${seed}`, ''];
  for (const id of ids) {
    const d = defs.get(id);
    lines.push(`## ${d.name} (\`${id}\`, tier ${d.tier ?? 1} ${d.kind})`, '');
    if (!report[id].plays.length) lines.push(`- never played; last seen: ${report[id].lastStatus ?? 'never drawn'}`);
    for (const p of report[id].plays) {
      if (p.problem) { lines.push(`- round ${p.round}, seat ${p.seat}, ${p.kind}: **${p.problem}**`); continue; }
      lines.push(`- round ${p.round}, seat ${p.seat}, ${p.kind}${p.asked.length ? `; asked: ${p.asked.join('; ')}` : ''}`);
      if (p.notes.length) lines.push(`  - notes: ${p.notes.join(' | ')}`);
      if (p.resources.length) lines.push(`  - resources: ${p.resources.join(', ')}`);
      if (p.map.length) lines.push(`  - map: ${p.map.join(', ')}`);
      lines.push(`  - shot: ${p.shot}`);
    }
    lines.push('');
  }
  writeFileSync(join(out, 'report.md'), lines.join('\n'));
  writeFileSync(join(out, 'summary.json'), `${JSON.stringify({ seed, rounds, config: config.content, report, never, consoleErrors: page.consoleErrors }, null, 1)}\n`);
  console.log(lines.join('\n'));
  if (never.length) console.log(`never played: ${never.join(', ')}`);
  if (page.consoleErrors.length) console.log(`console errors: ${page.consoleErrors.join(' | ')}`);
  ok = !never.length && !page.consoleErrors.length;
  console.log(ok ? 'PASS' : 'FAIL');
} finally {
  await page.close();
}
process.exit(ok ? 0 : 1);
