// The card viewer's export: everything `npm run cards` shows, read from the checkout it runs
// in, with no build and no cargo. Content is content/*.json, player faces are the generated
// web/card/faces.json (the fast lane keeps it current), pricing notes are docs/card-pricing-notes.json
// and stats are the newest screen in experiments/results. Written to out/cards-export.json.
//   node scripts/cards-export.mjs [--root DIR]
// Nothing here writes card text: a face or a name that isn't in content isn't on the page.

import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
export const REPO = resolve(here, '..', '..');

const readJson = (f) => JSON.parse(readFileSync(f, 'utf8').replace(/^\uFEFF/, ''));

/** Key order is not content: a reformatted file must not badge every card. */
function stable(v) {
  if (Array.isArray(v)) return `[${v.map(stable).join(',')}]`;
  if (v && typeof v === 'object')
    return `{${Object.keys(v)
      .sort()
      .map((k) => `${JSON.stringify(k)}:${stable(v[k])}`)
      .join(',')}}`;
  return JSON.stringify(v);
}
export const contentHash = (...parts) => createHash('sha256').update(stable(parts)).digest('hex').slice(0, 16);

// 118: in words, not the raw key ('3 research_cubes').
// 118: one of a resource is singular ("1 credit", not "1 credits").
const costs = (list = []) => list.map(([r, n]) => { const w = r.replaceAll('_', ' '); return `${n} ${n === 1 ? w.replace(/s$/, '') : w}`; }).join(', ');

// ---------- docs/card-pricing-notes.json ----------

/** The one current pricing line per card id, where one matters: `{ "<card id>": "<line>" }`. */
function cardNotes(root, warnings) {
  const f = join(root, 'docs', 'card-pricing-notes.json');
  if (!existsSync(f)) return {};
  const notes = readJson(f);
  for (const [id, line] of Object.entries(notes)) if (typeof line !== 'string') warnings.push(`docs/card-pricing-notes.json: ${id} is not one line`);
  return notes;
}

// ---------- stats ----------

function walk(dir, out = []) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    if (e.isDirectory()) walk(join(dir, e.name), out);
    else out.push(join(dir, e.name));
  }
  return out;
}

/** Rows of quoted or bare fields: a quoted cell may hold commas, quotes and line ends. */
function csvRows(text) {
  const rows = [];
  let row = [];
  let cur = '';
  let quoted = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quoted) {
      if (c !== '"') cur += c;
      else if (text[i + 1] === '"') (cur += '"'), i++;
      else quoted = false;
    } else if (c === '"') quoted = true;
    else if (c === ',') row.push(cur), (cur = '');
    else if (c === '\n') {
      row.push(cur.replace(/\r$/, ''));
      rows.push(row);
      row = [];
      cur = '';
    } else cur += c;
  }
  if (cur || row.length) rows.push([...row, cur]);
  return rows.filter((r) => r.some(Boolean));
}

function readCsv(file) {
  const [head, ...rows] = csvRows(readFileSync(file, 'utf8').replace(/^\uFEFF/, ''));
  return rows.map((row) => Object.fromEntries((head ?? []).map((h, i) => [h, row[i]])));
}

const LADDER = ['planner', 'rollout', 'beam', 'engine', 'greedy', 'random'];
const plain = (a) => !a.includes(':');
/** The ladder's highest agent, a profile (`planner:deals=1`) counting as its base; the shortest name first. */
const defaultAgent = (agents) => {
  const base = (a) => a.split(':')[0];
  const tier = (a) => (LADDER.includes(base(a)) ? LADDER.indexOf(base(a)) : LADDER.length);
  return [...agents].sort((a, b) => tier(a) - tier(b) || a.length - b.length)[0];
};
const MIN_GAMES = 100;

/**
 * The newest screen with per-card counts, committed or a local run in out/experiments:
 * revision, then plain agents, then games. Under MIN_GAMES is a smoke run, not a screen.
 */
function latestScreen(folders) {
  let best = null;
  for (const f of folders.filter((d) => existsSync(d)).flatMap((d) => walk(d))) {
    if (!f.endsWith('results.json')) continue;
    let d;
    try {
      d = readJson(f);
    } catch {
      continue;
    }
    if (!d.cards || !Object.keys(d.cards).length || (d.games_played ?? 0) < MIN_GAMES) continue;
    const rank = [d.rules_revision ?? 0, Object.keys(d.cards).filter(plain).length, d.games_played ?? 0];
    const first = best ? rank.findIndex((v, i) => v !== best.rank[i]) : 0;
    if (!best || (first >= 0 && rank[first] > best.rank[first])) best = { file: f, d, rank };
  }
  return best;
}

/**
 * Win % with a building standing at the end minus without, per agent and card, weighted
 * across variant and difficulty by the games with it. games.csv names only final buildings,
 * so programs and events have none, and it is a correlation: strong seats build more.
 */
function winDeltas(file) {
  if (!existsSync(file)) return { wins: {}, games: {} };
  const cells = new Map();
  for (const g of readCsv(file)) {
    const key = `${g.agent}\0${g.variant}\0${g.difficulty}`;
    if (!cells.has(key)) cells.set(key, { agent: g.agent, games: [] });
    const built = new Set(String(g.build ?? '').split('|').flatMap((s) => (s.split(':')[1] ?? '').split('+')).filter(Boolean));
    cells.get(key).games.push({ won: Number(g.won) ? 1 : 0, built });
  }
  const sum = {};
  const played = {};
  for (const { agent, games } of cells.values()) {
    played[agent] = (played[agent] ?? 0) + games.length;
    const ids = new Set(games.flatMap((g) => [...g.built]));
    for (const id of ids) {
      const yes = games.filter((g) => g.built.has(id));
      const no = games.filter((g) => !g.built.has(id));
      if (!no.length) continue;
      const rate = (l) => l.reduce((n, g) => n + g.won, 0) / l.length;
      const s = ((sum[agent] ??= {})[id] ??= { d: 0, n: 0 });
      s.d += (rate(yes) - rate(no)) * yes.length;
      s.n += yes.length;
    }
  }
  const wins = Object.fromEntries(
    Object.entries(sum).map(([a, m]) => [a, Object.fromEntries(Object.entries(m).filter(([, s]) => s.n).map(([id, s]) => [id, (100 * s.d) / s.n]))]),
  );
  return { wins, games: played };
}

/**
 * Reasoned prices: any CSV under experiments/results with a `card` or `id` column and a
 * numeric column whose name holds "reason" (card-prices.csv by convention). A rationale
 * column of words beside it is passed over. The last file in path order wins a card. A
 * `revision` column says which rules' costs the prices were reasoned at; a `note` on the
 * first row says what else to know, and both column titles print it. A file whose
 * `revision` is older than the rules' is passed over (084): its prices were reasoned at
 * costs since changed and name cards since cut, so a fresh read brings the column back.
 */
function reasonedPrices(results, rules) {
  const out = {};
  const seen = [];
  const stale = [];
  let revision = null;
  let note = null;
  if (!existsSync(results)) return { prices: out, files: seen, stale, revision, note };
  for (const f of walk(results).sort()) {
    if (!f.endsWith('.csv') || /[\\/](games|cards)\.csv$/.test(f)) continue;
    const rows = readCsv(f);
    const head = Object.keys(rows[0] ?? {});
    const idCol = head.find((h) => h === 'card' || h === 'id');
    const number = (v) => v !== undefined && v !== '' && Number.isFinite(Number(v));
    const col = head.find((h) => /reason/i.test(h) && rows.some((r) => number(r[h])) && rows.every((r) => !r[h] || number(r[h])));
    if (!idCol || !col) continue;
    // 099: a price file has one row per card: the effect ledger's instances.csv (a row per
    // effect) qualified, and each card showed its last effect's value as its price.
    if (new Set(rows.map((r) => r[idCol])).size !== rows.length) continue;
    if (rules && rows[0].revision && Number(rows[0].revision) < rules) {
      stale.push(f);
      continue;
    }
    seen.push(f);
    revision = rows[0].revision || revision;
    note = rows[0].note || note;
    for (const row of rows) if (number(row[col])) out[row[idCol]] = Number(row[col]);
  }
  return { prices: out, files: seen, stale, revision, note };
}

function buildStats(root, rules) {
  const results = join(root, 'experiments', 'results');
  const best = latestScreen([results, join(root, 'out', 'experiments')]);
  const reasoned = reasonedPrices(results, rules);
  const columns = [];
  const byAgent = {};
  let source = null;
  if (best) {
    const dir = dirname(best.file);
    const { wins, games } = winDeltas(join(dir, 'games.csv'));
    for (const [agent, cards] of Object.entries(best.d.cards)) {
      byAgent[agent] = {};
      for (const [id, c] of Object.entries(cards)) {
        byAgent[agent][id] = {
          kept_pct: games[agent] ? (100 * c.kept) / games[agent] : null,
          played_pct: c.kept + c.drawn ? (100 * c.played) / (c.kept + c.drawn) : null,
          win_delta: wins[agent]?.[id] ?? null,
          offered: c.offered,
          kept: c.kept,
          played: c.played,
          drawn: c.drawn,
        };
      }
    }
    const agents = Object.keys(byAgent);
    source = {
      path: relative(root, dir).replace(/\\/g, '/'),
      revision: best.d.rules_revision ?? null,
      games: best.d.games_played ?? null,
      agents,
      agent: defaultAgent(agents),
    };
    // Every stat label carries the rules revision it was measured at.
    const r = source.revision ? ` (r${source.revision})` : '';
    columns.push(
      // Not kept / offered: `offered` counts draft packets only, and the opening deal is kept
      // undrafted, so that ratio reads 1000% on a card offered four times.
      { key: 'kept_pct', label: `Kept %${r}`, title: `keeps per 100 games of this agent: each card is one copy, so about the share of games it was kept in; ${source.path}`, unit: '%' },
      { key: 'played_pct', label: `Played %${r}`, title: `played ÷ (kept + drawn), as the screen report has it; ${source.path}`, unit: '%' },
      { key: 'win_delta', label: `Win Δ${r}`, title: 'win % with this building standing at game end minus without, by variant and difficulty: a correlation, and buildings only (games.jsonl has other per-card outcomes, but committed results dirs do not keep it)', unit: ' pts' },
    );
  }
  const at = reasoned.revision ? ` (r${reasoned.revision})` : '';
  const caveat = reasoned.note ? `. ${reasoned.note}` : '';
  const stale = reasoned.revision ? `; reasoned at r${reasoned.revision}'s costs, so a card repriced since reads wrong${caveat}` : caveat;
  if (Object.keys(reasoned.prices).length)
    columns.push(
      { key: 'reasoned', label: `Reasoned${at}`, title: `reasoned price${stale}`, unit: '' },
      { key: 'price_gap', label: `Printed − reasoned${at}`, title: `printed credits minus the reasoned price: above 0 is dear, below 0 cheap${stale}`, unit: '' },
    );
  return {
    source,
    columns,
    byAgent,
    reasoned: reasoned.prices,
    reasoned_from: reasoned.files.map((f) => relative(root, f).replace(/\\/g, '/')),
    reasoned_stale: reasoned.stale.map((f) => relative(root, f).replace(/\\/g, '/')),
  };
}

// ---------- cards ----------

const TIER = (t) => (t === 0 ? 'Unique' : `Tier ${t}`);

function playerCard(c, face, keep) {
  const keepCost = c.tier === 2 ? `${keep.keep_tier2} insight (${keep.keep_tier2_upgraded} upgraded)` : c.tier === 1 ? `${keep.keep_tier1} insight` : 'in hand at the start';
  const price = [
    ['Credits', c.cost],
    ['Insight', c.insight ?? 0],
    c.energy && ['Energy', c.energy],
    c.chips && ['Chips', c.chips],
    c.antimatter && ['Antimatter', c.antimatter],
    c.research_cubes && ['Research cubes', c.research_cubes],
    c.lowers?.length && ['Lowers income', costs(c.lowers)],
    c.needs?.length && ['Needs', c.needs.map(([t, n]) => `${n} ${t}`).join(', ')],
    c.replaces && ['Replaces', c.replaces],
    ['Keep', keepCost],
    c.ability?.cost?.length && ['Action cost', costs(c.ability.cost)],
    c.ability?.tokens && ['Action tokens', c.ability.tokens],
    c.tokens && ['Tokens', c.tokens.kind],
  ].filter(Boolean);
  const ops = { effects: c.effects, passives: c.passives, ability: c.ability?.effects };
  return {
    id: c.id,
    kind: c.kind,
    name: c.name,
    tier: c.tier,
    tierLabel: TIER(c.tier),
    // 158: a drafted player card Matt has not picked yet (the viewer's draft badge, as 156's Unity drafts).
    draft: !!c.draft,
    cost: c.cost,
    costLabel: `${c.cost} credits`,
    tags: c.tags ?? [],
    keywords: c.keywords ?? [],
    text: [face?.top_text, face?.text].filter(Boolean).join(' '),
    sections: [],
    price,
    ops,
    hashParts: [c, face ?? null],
    render: { ...c, ...face },
  };
}

function frameless(kind, row, fields) {
  return {
    id: row.id,
    kind,
    name: row.name,
    tags: [],
    keywords: [],
    text: row.text ?? '',
    sections: [],
    price: [],
    ops: {},
    cost: null,
    costLabel: '',
    hashParts: [row],
    ...fields,
  };
}

// 135: a threat prints a counter menu (`counters`: options, each a list of items) and tags, not one
// `counter` list and a source (139: this read the old fields and threw).
function counterWords(option) {
  return option
    .map((i) => (i.resource ? `${i.amount} ${i.resource}` : i.sacrifice ? `${i.amount} ${i.sacrifice} tags` : `${i.nodes_near} nodes near`))
    .join(' + ');
}

function threat(t) {
  const options = t.counters ?? [];
  const first = options[0] ?? [];
  const credits = first.filter((i) => i.resource === 'credits').reduce((n, i) => n + i.amount, 0);
  const menu = options.map(counterWords).join(' or ');
  const tags = (t.tags ?? []).join(', ');
  return frameless('threat', t, {
    tier: t.tier,
    tierLabel: TIER(t.tier),
    keywords: (t.tags ?? []).map((g) => `tag: ${g}`),
    cost: credits || first.reduce((n, i) => n + (i.amount ?? i.nodes_near ?? 0), 0),
    costLabel: `counter: ${menu}`,
    price: [['Counter', menu], ['Tags', tags]],
    ops: { effects: t.effects },
  });
}

function adversary(a) {
  return frameless('adversary', a, {
    tier: a.stage,
    tierLabel: `Stage ${a.stage}`,
    keywords: [],
    ops: { standing: a.standing, twists: a.twists },
  });
}

// 156: every Alignment card prints tiers I, II and III (its side's tips pick one); `draft` marks a
// drafted card Matt has not picked yet.
function alignment(a) {
  const numerals = ['I', 'II', 'III'];
  const tiers = a.tiers ?? [];
  return frameless(a.side, a, {
    tier: 0,
    tierLabel: a.draft ? 'Three tiers (draft)' : 'Three tiers',
    text: tiers.map((t, i) => `${numerals[i]}: ${t.text}`).join(' '),
    draft: !!a.draft,
    ops: Object.fromEntries(tiers.map((t, i) => [`tier ${numerals[i]}`, t.effects])),
  });
}

function project(p) {
  return frameless('project', p, {
    tier: p.stage,
    tierLabel: `Stage ${p.stage}`,
    cost: p.cost,
    costLabel: `${p.cost} compute per player`,
    keywords: p.ongoing ? ['ongoing'] : [],
    price: [['Compute per player', p.cost]],
    ops: { effects: p.effects },
  });
}

/** Content gives a megaproject no id: country `i` always uses piece `i`, so a rename is a change. */
function megaproject(country, i) {
  const m = country.project;
  return frameless('megaproject', { ...m, id: `megaproject_${i}` }, {
    tier: null,
    tierLabel: country.name,
    cost: m.cubes,
    costLabel: `${m.cubes} cubes`,
    price: [['Cubes', m.cubes]],
  });
}

function aiBoard(b, cardName) {
  // 118: a step's effects were dropped, so the Researcher's free-station space read '—'.
  const effect = (e) => (e.op === 'build_station' ? 'builds a free research station in reach' : e.op.replaceAll('_', ' '));
  // 118: a space's income reads +N, apart from the energy that places it.
  const step = (u) => [(u.income ?? []).map((x) => `+${costs([x])}`).join(", "), ...(u.effects ?? []).map(effect), u.cost && `${u.cost} energy to place`, u.keep2 && 'tier-2 keep upgrade', ...(u.tags ?? []).map((t) => `${t} tag`), u.capstone && `capstone: ${u.capstone.name}`];
  const track = (l) => l.map((u) => step(u).filter(Boolean).join('; ') || '—').join(' · ');
  const capstones = [...b.economy_track, ...(b.play_track ?? [])].flatMap((u) => u.capstone ?? []);
  return frameless('ai', b, {
    tier: null,
    tierLabel: '', // 118: the kind already says 'AI board' (it printed twice)
    tags: b.tags ?? [],
    price: [
      ['Credits a round', b.credits],
      ...(b.income?.length ? [['Income', costs(b.income)]] : []),
      // 118: a power with no cost and no tags (Synthesis) priced as an empty string.
      ...(b.powers ?? []).map((p) => [`Power: ${p.name}`, [(p.tags ?? []).map(([t, n]) => `sacrifice ${n} ${t}`).join(', '), costs(p.cost)].filter(Boolean).join('; ') || 'free']),
    ],
    sections: [
      { head: `Innate: ${b.innate.name}`, text: b.innate.text ?? '' },
      ...(b.powers ?? []).map((p) => ({ head: `Power: ${p.name}`, text: p.text ?? '' })),
      { head: 'Economy track', text: track(b.economy_track) },
      ...(b.play_track?.length ? [{ head: 'Card-play track', text: track(b.play_track) }] : []),
      ...capstones.map((c) => ({ head: `Capstone: ${c.name}`, text: c.text })),
      ...(b.start ? [{ head: 'Starts with', text: cardName(b.start) }] : []),
      ...(b.cards?.length ? [{ head: 'Unique cards', text: b.cards.map(cardName).join(', ') }] : []),
    ],
    ops: {
      innate: b.innate.passives,
      ...Object.fromEntries((b.powers ?? []).map((p) => [`power: ${p.name}`, p.effects])),
      ...Object.fromEntries(capstones.map((c) => [`capstone: ${c.name}`, c.passives])),
    },
  });
}

/** A scene's `card` is a suggestion, but it is the only link from a card to its art. */
function sceneMap(root, warnings) {
  const f = join(root, 'tools', 'card-art', 'scenes.json');
  if (!existsSync(f)) return {};
  const out = {};
  for (const group of Object.values(readJson(f).scenes ?? {}))
    for (const [id, s] of Object.entries(group)) if (s.card) out[s.card] = id;
  if (!Object.keys(out).length) warnings.push('tools/card-art/scenes.json names no scenes');
  return out;
}

export function exportCards(root = REPO) {
  const warnings = [];
  const content = (name) => readJson(join(root, 'content', `${name}.json`));
  const faces = readJson(join(root, 'web', 'card', 'faces.json'));
  const baseline = readJson(join(root, 'configs', 'baseline.json'));
  const mod = baseline.modules.find((m) => m.module === 'cards');
  const deck = new Set(mod?.cards ?? []);
  const keep = mod?.params ?? {};
  const players = content('cards');
  const map = content('map');
  const cardName = (id) => players.find((c) => c.id === id)?.name ?? id;

  const cards = [
    ...players.map((c) => playerCard(c, faces[c.id], keep)),
    ...content('threats').map(threat),
    ...content('alignment').map(alignment),
    ...content('adversary').events.map(adversary), // 135: {board, events}
    ...content('projects').map(project),
    ...map.countries.map(megaproject),
    ...content('ais').map((b) => aiBoard(b, cardName)),
  ];

  for (const c of players) if (!faces[c.id]) warnings.push(`${c.id} has no face in web/card/faces.json: run aic-sim docs`);
  for (const id of Object.keys(faces)) if (!players.some((c) => c.id === id)) warnings.push(`faces.json has ${id}, which is no card: run aic-sim docs`);

  const notes = cardNotes(root, warnings);
  const scenes = sceneMap(root, warnings);
  const lib = readFileSync(join(root, 'crates', 'aic-core', 'src', 'lib.rs'), 'utf8');
  const revision = Number(/RULES_REVISION: u32 = (\d+)/.exec(lib)?.[1]) || null;
  const stats = buildStats(root, revision);

  const out = cards.map(({ hashParts, ...c }) => {
    const isPlayer = !!c.render;
    const reasoned = stats.reasoned[c.id];
    return {
      ...c,
      hash: contentHash(...hashParts),
      scene: scenes[c.id] ?? null,
      inDeck: isPlayer ? deck.has(c.id) : null,
      pricing: isPlayer ? notes[c.id] ?? null : null,
      reasoned: reasoned ?? null,
      priceGap: reasoned == null || c.cost == null ? null : c.cost - reasoned,
    };
  });
  for (const id of Object.keys(notes)) if (!out.some((c) => c.id === id)) warnings.push(`docs/card-pricing-notes.json names ${id}, which is no card`);
  for (const id of Object.keys(stats.reasoned)) if (!out.some((c) => c.id === id)) warnings.push(`a reasoned price names ${id}, which is no card`);
  for (const [id, scene] of Object.entries(scenes)) if (!out.some((c) => c.id === id)) warnings.push(`tools/card-art/scenes.json: scene ${scene} names ${id}, which is no card`);
  for (const c of out) if (c.inDeck === false) c.keywords.push('not in baseline');

  const git = spawnSync('git', ['rev-parse', '--short', 'HEAD'], { cwd: root, encoding: 'utf8' });
  const data = {
    format: 1,
    revision,
    commit: git.status === 0 ? git.stdout.trim() : null,
    layout: readJson(join(root, 'web', 'card', 'layout.json')),
    fits: readJson(join(root, 'art', 'frames', 'r3', 'fit.json')),
    cards: out,
    stats: { source: stats.source, columns: stats.columns, byAgent: stats.byAgent, reasoned_from: stats.reasoned_from, reasoned_stale: stats.reasoned_stale },
    warnings,
  };
  const file = join(root, 'out', 'cards-export.json');
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, JSON.stringify(data));
  return data;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const at = process.argv.indexOf('--root');
  const t0 = performance.now();
  const data = exportCards(at > 0 ? resolve(process.argv[at + 1]) : REPO);
  const kinds = {};
  for (const c of data.cards) kinds[c.kind] = (kinds[c.kind] ?? 0) + 1;
  console.log(`cards-export: ${data.cards.length} cards in ${Math.round(performance.now() - t0)} ms`, kinds);
  console.log(`stats: ${data.stats.source ? `${data.stats.source.path}, r${data.stats.source.revision}` : 'none'}`);
  for (const f of data.stats.reasoned_stale) console.log(`reasoned prices passed over, older than r${data.revision}: ${f}`);
  for (const w of data.warnings) console.warn(`warning: ${w}`);
}
