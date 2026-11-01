// The card viewer's export: everything `npm run cards` shows, read from the checkout it runs
// in, with no build and no cargo. Content is content/*.json, player faces are the generated
// web/card/faces.json (the fast lane keeps it current), pricing lines are docs/card-pricing.md
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

const readJson = (f) => JSON.parse(readFileSync(f, 'utf8').replace(/^﻿/, ''));
const has = (o, k) => o != null && Object.hasOwn(o, k);

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

const costs = (list = []) => list.map(([r, n]) => `${n} ${r}`).join(', ');

// ---------- docs/card-pricing.md ----------

/** Every table row, bullet and paragraph of the doc, with its section. */
function pricingEntries(md) {
  const entries = [];
  let section = '';
  let columns = null;
  let bullet = null;
  const flush = () => {
    if (bullet) entries.push(bullet);
    bullet = null;
  };
  const cells = (l) => l.trim().replace(/^\||\|$/g, '').split('|').map((c) => c.trim());
  for (const line of md.split(/\r?\n/)) {
    if (line.startsWith('#')) {
      flush();
      columns = null;
      section = line.replace(/^#+\s*/, '');
    } else if (line.startsWith('|')) {
      flush();
      if (/^\|[\s:|-]+\|?\s*$/.test(line)) continue; // the rule under a header
      if (columns) entries.push({ section, columns, cells: cells(line) });
      else columns = cells(line);
    } else if (/^- /.test(line)) {
      flush();
      columns = null;
      bullet = { section, text: line.slice(2).trim() };
    } else if (line.trim()) {
      // A bullet's next line, or a paragraph: both read as one entry.
      columns = null;
      if (bullet) bullet.text += ` ${line.trim()}`;
      else bullet = { section, text: line.trim() };
    } else {
      flush();
      columns = null;
    }
  }
  flush();
  return entries;
}

/** The lines about a card, in doc order (the doc puts the newest change first). */
function pricingFor(name, entries) {
  const quoted = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const named = new RegExp(`(^|[^\\p{L}\\p{N}])${quoted}($|[^\\p{L}\\p{N}])`, 'u');
  return entries
    .filter((e) => (e.cells ? named.test(e.cells[0]) : named.test(e.text)))
    .map((e) => (e.cells ? e : { section: e.section, text: e.text.replace(/\*\*/g, ''), mention: !e.text.replace(/\*\*/g, '').startsWith(name) }))
    .sort((a, b) => Boolean(a.mention) - Boolean(b.mention));
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
  return (rows ?? []).map((row) => Object.fromEntries((head ?? []).map((h, i) => [h, row[i]])));
}

const LADDER = ['planner', 'rollout', 'beam', 'engine', 'greedy', 'random'];
const plain = (a) => !a.includes(':');
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
 * first row says what else to know, and both column titles print it.
 */
function reasonedPrices(results) {
  const out = {};
  const seen = [];
  let revision = null;
  let note = null;
  if (!existsSync(results)) return { prices: out, files: seen, revision, note };
  for (const f of walk(results).sort()) {
    if (!f.endsWith('.csv') || /[\\/](games|cards)\.csv$/.test(f)) continue;
    const rows = readCsv(f);
    const head = Object.keys(rows[0] ?? {});
    const idCol = head.find((h) => h === 'card' || h === 'id');
    const number = (v) => v !== undefined && v !== '' && Number.isFinite(Number(v));
    const col = head.find((h) => /reason/i.test(h) && rows.some((r) => number(r[h])) && rows.every((r) => !r[h] || number(r[h])));
    if (!idCol || !col) continue;
    seen.push(f);
    revision = rows[0].revision || revision;
    note = rows[0].note || note;
    for (const row of rows) if (number(row[col])) out[row[idCol]] = Number(row[col]);
  }
  return { prices: out, files: seen, revision, note };
}

function buildStats(root) {
  const results = join(root, 'experiments', 'results');
  const best = latestScreen([results, join(root, 'out', 'experiments')]);
  const reasoned = reasonedPrices(results);
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
      agent: LADDER.find((a) => agents.includes(a)) ?? agents[0],
    };
    columns.push(
      // Not kept / offered: `offered` counts draft packets only, and the opening deal is kept
      // undrafted, so that ratio reads 1000% on a card offered four times.
      { key: 'kept_pct', label: 'Kept %', title: 'keeps per 100 games of this agent: each card is one copy, so about the share of games it was kept in', unit: '%' },
      { key: 'played_pct', label: 'Played %', title: 'played ÷ (kept + drawn), as the screen report has it', unit: '%' },
      { key: 'win_delta', label: 'Win Δ', title: 'win % with this building standing at game end minus without, by variant and difficulty: a correlation, and buildings only (games.jsonl has other per-card outcomes, but committed results dirs do not keep it)', unit: ' pts' },
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
    c.lowers?.length && ['Lowers income', costs(c.lowers)],
    c.needs?.length && ['Needs', c.needs.map(([t, n]) => `${n} ${t}`).join(', ')],
    c.replaces && ['Replaces', c.replaces],
    ['Keep', keepCost],
    c.ability?.cost?.length && ['Action cost', costs(c.ability.cost)],
    c.ability?.tokens && ['Action tokens', c.ability.tokens],
    c.tokens && ['Tokens', `${c.tokens.kind}${c.tokens.per_round ? `, +${c.tokens.per_round} a round` : ''}`],
  ].filter(Boolean);
  const ops = { effects: c.effects, passives: c.passives, ability: c.ability?.effects };
  return {
    id: c.id,
    kind: c.kind,
    name: c.name,
    tier: c.tier,
    tierLabel: TIER(c.tier),
    cost: c.cost,
    costLabel: `${c.cost} credits`,
    tags: c.tags ?? [],
    keywords: [...(c.keywords ?? []), ...(c.unique ? [`unique: ${c.unique}`] : [])],
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

function threat(t) {
  const credits = t.counter.filter(([r]) => r === 'credits').reduce((n, [, a]) => n + a, 0);
  return frameless('threat', t, {
    tier: t.tier,
    tierLabel: TIER(t.tier),
    cost: credits || t.counter.reduce((n, [, a]) => n + a, 0),
    costLabel: `counter: ${costs(t.counter)}`,
    price: [['Counter', costs(t.counter)]],
    ops: { effects: t.effects },
  });
}

function adversary(a) {
  return frameless('adversary', a, {
    tier: a.stage,
    tierLabel: `Stage ${a.stage}`,
    keywords: a.escalation ? ['escalation'] : [],
    ops: { twists: a.twists },
  });
}

function alignment(a) {
  return frameless(a.side, a, {
    tier: a.tier,
    tierLabel: TIER(a.tier),
    ops: { effects: a.effects, ...Object.fromEntries((a.steps ?? []).map((s) => [`from ${s.at} tips`, s.effects])) },
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
    price: [['Cubes', m.cubes], ['Benefit a round', costs(m.benefit)]],
  });
}

function aiBoard(b) {
  const track = (l) => l.map((u) => [costs(u.income), u.cost && `${u.cost} energy to place`, u.keep2 && 'tier-2 keep upgrade'].filter(Boolean).join('; ') || '—').join(' · ');
  return frameless('ai', b, {
    tier: null,
    tierLabel: 'AI board',
    price: [
      ['Credits a round', b.credits],
      ...(b.income?.length ? [['Income', costs(b.income)]] : []),
      ...(b.discounts ?? []).map((d) => ['Discount', `${d.resource} for ${d.tag}, ${d.rate} credits each`]),
      ...(b.powers ?? []).map((p) => [`Power: ${p.name}`, [p.tags.map(([t, n]) => `sacrifice ${n} ${t}`).join(', '), costs(p.cost)].filter(Boolean).join('; ')]),
    ],
    sections: [
      { head: `Innate: ${b.innate.name}`, text: b.innate.text ?? '' },
      ...(b.powers ?? []).map((p) => ({ head: `Power: ${p.name}`, text: p.text ?? '' })),
      { head: 'Growth', text: b.growth.map((g) => g.name).join(' · ') },
      { head: 'Presence track', text: track(b.presence) },
      ...(b.presence2?.length ? [{ head: 'Second presence track', text: track(b.presence2) }] : []),
      ...(b.start ? [{ head: 'Starts with', text: b.start }] : []),
      ...(b.cards?.length ? [{ head: 'Unique cards', text: b.cards.join(', ') }] : []),
    ],
    ops: {
      innate: b.innate.passives,
      ...Object.fromEntries((b.powers ?? []).map((p) => [`power: ${p.name}`, p.effects])),
      ...Object.fromEntries(b.growth.map((g) => [`growth: ${g.name}`, g.effects])),
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

  const cards = [
    ...players.map((c) => playerCard(c, faces[c.id], keep)),
    ...content('threats').map(threat),
    ...content('alignment').map(alignment),
    ...content('adversary').map(adversary),
    ...content('projects').map(project),
    ...map.countries.map(megaproject),
    ...content('ais').map(aiBoard),
  ];

  for (const c of players) if (!faces[c.id]) warnings.push(`${c.id} has no face in web/card/faces.json: run aic-sim docs`);
  for (const id of Object.keys(faces)) if (!players.some((c) => c.id === id)) warnings.push(`faces.json has ${id}, which is no card: run aic-sim docs`);

  const pricingFile = join(root, 'docs', 'card-pricing.md');
  const entries = existsSync(pricingFile) ? pricingEntries(readFileSync(pricingFile, 'utf8')) : [];
  const scenes = sceneMap(root, warnings);
  const stats = buildStats(root);

  const out = cards.map(({ hashParts, ...c }) => {
    const isPlayer = has(c, 'render');
    const reasoned = stats.reasoned[c.id];
    return {
      ...c,
      hash: contentHash(...hashParts),
      scene: scenes[c.id] ?? null,
      inDeck: isPlayer ? deck.has(c.id) || c.tier === 0 : null,
      pricing: isPlayer ? pricingFor(c.name, entries) : [],
      reasoned: reasoned ?? null,
      priceGap: reasoned == null || c.cost == null ? null : c.cost - reasoned,
    };
  });
  for (const id of Object.keys(stats.reasoned)) if (!out.some((c) => c.id === id)) warnings.push(`a reasoned price names ${id}, which is no card`);
  for (const c of out) if (c.inDeck === false) c.keywords.push('not in baseline');

  const lib = readFileSync(join(root, 'crates', 'aic-core', 'src', 'lib.rs'), 'utf8');
  const git = spawnSync('git', ['rev-parse', '--short', 'HEAD'], { cwd: root, encoding: 'utf8' });
  const data = {
    format: 1,
    revision: Number(/RULES_REVISION: u32 = (\d+)/.exec(lib)?.[1]) || null,
    commit: git.status === 0 ? git.stdout.trim() : null,
    layout: readJson(join(root, 'web', 'card', 'layout.json')),
    fits: readJson(join(root, 'art', 'frames', 'r3', 'fit.json')),
    cards: out,
    stats: { source: stats.source, columns: stats.columns, byAgent: stats.byAgent, reasoned_from: stats.reasoned_from },
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
  for (const w of data.warnings) console.warn(`warning: ${w}`);
}
