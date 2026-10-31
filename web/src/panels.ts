// HTML for the panels around the board: the Alignment track and loss conditions, the
// adversary row (deck, this round's card, the face-up next card, discard), the countries
// strip (megaprojects, threats), and the seat panels' contents. Everything comes from
// the generic view; known keys (adversary.*, alignment.*, board.*, threats.*, ai.*,
// economy.income.*, cards.*) get a dedicated drawing, every other value, card group or
// pile falls back to a generic chip so nothing goes unshown.

import { content, param, type CardView, type Config, type PileView, type Value, type View } from './engine';
import { ADVERSARY_COLOUR, escapeHtml, extraCosts, KINDS, strong } from './format';

const esc = escapeHtml;

/** Global values drawn by the status strip or the adversary row; the rest are generic chips. */
const DRAWN_GLOBALS = new Set([
  'adversary.stage', 'adversary.difficulty', 'adversary.deck', 'adversary.compute',
  'threats.tier', 'alignment.undecided', 'alignment.unity', 'alignment.discord', 'alignment.unity_tips',
  'alignment.discord_tips', 'alignment.bar',
  'board.war', 'board.fronts',
]);
/** Piles drawn by the status strip, the adversary row or the countries strip. */
const DRAWN_PILES = new Set([
  'adversary.deck', 'adversary.next', 'adversary.current', 'adversary.discard', 'threats.row',
  'alignment.deck.unity', 'alignment.deck.discord', 'alignment.resolved.unity', 'alignment.resolved.discord',
  'adversary.projects', 'adversary.projects_ongoing', 'adversary.projects_done', 'adversary.project_deck',
]);

const findValue = (vs: Value[], key: string) => vs.find((v) => v.key === key);
const pile = (v: View, key: string) => v.piles.find((p) => p.key === key);

/** A value chip; `near` marks a value close to its max (where that is a warning). */
function valueChip(v: Value, scope: string, label = v.label, near = true): string {
  const val = v.max !== undefined ? `${v.value}<small>/${v.max}</small>` : `${v.value}`;
  // A countdown ("antimatter actions left 1/1") is full when it is safe: no warning
  // near its max (020: drawn red all round).
  const countdown = /left/.test(label);
  const warn = near && !countdown && v.max !== undefined && v.max > 0 && v.value >= v.max * 0.75 ? ' near' : '';
  return `<span class="value${warn}" data-value="${esc(v.key)}" data-scope="${scope}"><span class="label">${esc(label)}</span> <b>${val}</b></span>`;
}

function cardChip(c: CardView, where: string, printed = false): string {
  // 044: costs beyond credits sit in the bracket the printed chip drops, so "unaffordable"
  // could have no visible reason. A card with no price showing (built, played) owes nothing.
  const extra = c.cost !== undefined ? extraCosts(c.text).map((x) => ` <small title="also costs">+${esc(x)}</small>`).join('') : '';
  const cost = (c.cost !== undefined ? ` <small title="cost">${c.cost}</small>` : '') + extra;
  const status = c.status && !['draft', 'built', 'resolved', 'done', 'ongoing'].includes(c.status) ? `<em>${esc(c.status)}</em>` : '';
  // 024: a card repeats its rules text under its name, as Terraforming Mars' do.
  // The view's text starts with a bracketed line ("[building, tier 1, industry, +1 insight]
  // ..."): kind, tags and every cost but credits. Printed chips show the rules text only.
  const rules = (c.text ?? '').replace(/^\[[^\]]*\]\s*/, '');
  const text = printed && rules ? `<span class="ctext">${esc(rules)}</span>` : '';
  return `<span class="card${printed ? ' printed' : ''} ${esc(c.status.replace(/\s+/g, '-'))}" data-card="${esc(c.id)}" data-where="${where}">${esc(c.name)}${cost}${status}${text}</span>`;
}

/** Adversary pieces on the board, by kind. */
function onBoard(v: View, kinds: string[]): Record<string, number> {
  const n: Record<string, number> = Object.fromEntries(kinds.map((k) => [k, 0]));
  for (const h of v.hexes) for (const t of h.tokens) if (t.kind in n) n[t.kind] += t.count;
  return n;
}

// ---------- status: alignment and losses ----------

/** A bucket of the tug of war; Unity fills toward the left, Discord toward the right. */
function bucketBar(b: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!b) return '';
  const pct = b.max ? Math.min(100, (100 * b.value) / b.max) : 0;
  return `<span class="bar ${side}" data-value="${esc(b.key)}" data-scope="global"><span class="fill" style="width:${pct}%"></span>
    <span class="btext">${name} ${b.value}/${b.max ?? '?'}</span></span>`;
}

/** "Unity tips 1 (next at 2)". */
function tipsChip(t: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!t) return '';
  return `<span class="tips ${side}" data-value="${esc(t.key)}" data-scope="global">${name} tips <b>${t.value}</b>${t.max !== undefined ? `<small> (next at ${t.max})</small>` : ''}</span>`;
}

/** The Alignment line, and its resolved cards and decks for the line under it; nothing without the module. */
function alignmentHtml(v: View): { track: string; cards: string[] } {
  const g = v.globals;
  const und = findValue(g, 'alignment.undecided');
  const unity = findValue(g, 'alignment.unity');
  const discord = findValue(g, 'alignment.discord');
  const bar = findValue(g, 'alignment.bar');
  if (!und && !unity && !discord && !bar) return { track: '', cards: [] };
  const level = bar?.value ?? 0;
  const max = bar?.max ?? 0;
  const undHtml = und
    ? `<span class="undecided" data-value="${esc(und.key)}" data-scope="global"><b>${und.value}</b><small>/${und.max ?? '?'}</small> undecided</span>`
    : '';
  const track = `<div class="alignment">
      <span class="alabel">Alignment</span>
      <span class="tug">${bucketBar(unity, 'unity', 'Unity')}${undHtml}${bucketBar(discord, 'discord', 'Discord')}</span>
      ${tipsChip(findValue(g, 'alignment.unity_tips'), 'unity', 'Unity')}${tipsChip(findValue(g, 'alignment.discord_tips'), 'discord', 'Discord')}
      ${bar ? `<span class="level" data-value="${esc(bar.key)}" data-scope="global">bar <b>${level}</b>/${max}</span><span class="win">${esc(bar.doc ?? '')}</span>` : ''}
    </div>`;
  const groups = ([['alignment.resolved.unity', 'unity'], ['alignment.resolved.discord', 'discord']] as const)
    .map(([key, cls]) => [pile(v, key), cls] as const)
    .filter(([p]) => p && p.visible.length)
    .map(([p, cls]) => `<span class="agroup ${cls}" data-pile="${esc(p!.key)}"><span class="what">${esc(p!.label)}</span>${p!.visible.map((c) => cardChip(c, `pile:${p!.key}`)).join('')}</span>`);
  const decks = ['unity', 'discord'].map((s) => pile(v, `alignment.deck.${s}`)).filter((p): p is PileView => !!p)
    .map((p) => `<span class="adeck" data-pile="${esc(p.key)}">${esc(p.label)} <b>${p.count}</b></span>`);
  return { track, cards: [...groups, ...decks] };
}

/** The "Lose if" group and, beside it, the other tracks; each only if it has something. */
function lossesHtml(v: View, config: Config): string[] {
  const g = v.globals;
  const loss: string[] = [];
  const dd = pile(v, 'alignment.deck.discord');
  if (dd) {
    const danger = dd.count <= 1 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-pile="${esc(dd.key)}" title="a Discord tip that finds no card loses the game"><span class="label">Discord deck runs out</span> <b>${dd.count}</b></span>`);
  }
  const deck = findValue(g, 'adversary.deck');
  if (deck) {
    const danger = deck.value <= 1 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-value="${deck.key}" data-scope="global"><span class="label">adversary cards run out</span> <b>${deck.value}</b><small> left</small></span>`);
  }
  // 044: the third locked loss: a seat's last presence, if this config has it.
  const counts = v.seats.map((s) => s.values.find((x) => x.key === 'board.presence')?.value);
  if (param(config, 'alignment', 'presence_loss') && counts.every((n) => n !== undefined) && counts.length) {
    const least = Math.min(...(counts as number[]));
    loss.push(`<span class="value${least <= 1 ? ' danger' : ''}" title="a seat with no presence on the board loses the game (early on, nothing destroys a last presence)"><span class="label">presence gone</span> <b>${least}</b></span>`);
  }
  // Unlabelled `vars` (source "engine") are folded under the piles: see `varsHtml`.
  // The clock's step is a name, not a count: "clock: war" (020: it read "war 2").
  const clockChip = (x: Value) => `<span class="value" data-value="${esc(x.key)}" data-scope="global"><span class="label">clock</span> <b>${esc(x.label)}</b></span>`;
  const other = g.filter((x) => !DRAWN_GLOBALS.has(x.key) && x.source !== 'engine').map((x) => (x.key === 'board.clock' ? clockChip(x) : valueChip(x, 'global')));
  // The other tracks (the clock's step, Unity bonuses) beside the losses, not in them
  // (020: drawn inside the red "Lose if" group).
  return [
    ...(loss.length ? [`<span class="losses"><span class="what">Lose if</span>${loss.join('')}</span>`] : []),
    ...(other.length ? [`<span class="tracks">${other.join('')}</span>`] : []),
  ];
}

export function statusHtml(v: View, config: Config): string {
  const { track, cards } = alignmentHtml(v);
  // The Alignment cards and the loss conditions share one wrapping line.
  const acards = [...cards, ...lossesHtml(v, config)];
  return track + (acards.length ? `<div class="acards">${acards.join('')}</div>` : '');
}

// ---------- the adversary row ----------

/** An adversary card face under the box's title: name, stage, escalation mark, then its printed text. */
function adversaryCard(c: CardView, where: string, title: string): string {
  const def = content.adversary?.find((a) => a.id === c.id);
  // The view's text starts "stage n: "; the stage is drawn as a chip instead.
  const text = def?.text ?? (c.text ?? '').replace(/^stage \d+:\s*/, '');
  const stage = def ? `<span class="stage">stage ${def.stage}</span>` : '';
  const war = def?.twists.some((t) => t.op === 'war') ? '<span class="war-tag">war</span>' : '';
  return `<div class="acard" data-card="${esc(c.id)}" data-where="${where}"><div class="sline"><span class="stitle">${title}</span><span class="sname">${esc(c.name)}</span>${stage}${war}${def?.escalation ? '<span class="escal">escalation</span>' : ''}</div><div class="atext">${esc(text)}</div></div>`;
}

function cardBox(p: PileView | undefined, title: string, empty: string, cls: string): string {
  if (!p) return '';
  const cards = p.visible.length
    ? p.visible.map((c) => adversaryCard(c, `pile:${p.key}`, title)).join('')
    : `<div class="sline"><span class="stitle">${title}</span></div><span class="empty">${esc(empty)}</span>`;
  return `<div class="slot-box ${cls}" data-pile="${esc(p.key)}">${cards}</div>`;
}

/** The war-round badge, drawn when `board.war` is 1. */
function warBadge(v: View): string {
  const war = findValue(v.globals, 'board.war');
  if (!war?.value) return '';
  return `<span class="war-round" data-value="${esc(war.key)}" data-scope="global">⚔ war round</span>`;
}

export function pipelineHtml(v: View): string {
  const deck = pile(v, 'adversary.deck');
  const next = pile(v, 'adversary.next');
  const cur = pile(v, 'adversary.current');
  const di = pile(v, 'adversary.discard');
  if (!deck && !next && !cur) return '';
  const g = v.globals;
  const short: Record<string, string> = { 'board.fronts': 'fronts' };
  const head = ['adversary.stage', 'adversary.difficulty', 'adversary.compute', 'threats.tier', 'board.fronts']
    .map((k) => findValue(g, k)).filter((x): x is Value => !!x)
    .map((x) => valueChip(x, 'global', short[x.key] ?? x.label.replace('adversary ', ''), false))
    .join('');
  const peek = deck?.visible.length
    ? `<div class="sline peek" title="${esc(`top of the deck: ${deck.visible.map((c) => c.name).join(', ')}`)}">top: ${deck.visible.map((c) => `<span data-card="${esc(c.id)}" data-where="pile:adversary.deck">${esc(c.name)}</span>`).join(', ')}</div>`
    : '';
  const deckBox = deck
    ? `<div class="slot-box deck" data-pile="adversary.deck"><div class="sline"><span class="stitle">Deck</span><span class="big">${deck.count}</span><small>left</small></div>${peek}<div class="hint">the clock: lose when no card is left for the next round</div></div>`
    : '';
  const last = di?.visible[0];
  const discBox = di
    ? `<div class="slot-box discard" data-pile="adversary.discard"><div class="sline"><span class="stitle">Discard</span><span class="big">${di.count}</span></div>${last ? `<div class="hint">last: <span data-card="${esc(last.id)}" data-where="pile:adversary.discard">${esc(last.name)}</span></div>` : ''}</div>`
    : '';
  const arrow = '<span class="parrow">→</span>';
  const boxes = [
    deckBox,
    cardBox(next, 'Next round (face up)', 'not revealed yet', 'next'),
    cardBox(cur, 'This round', 'no card this round', 'current'),
    discBox,
  ].filter(Boolean);
  const left = onBoard(v, ['ambassador:adversary', 'troop']);
  const pieces = `<span class="left" title="adversary pieces on the board">on board: ${left['ambassador:adversary']} ambassador${left['ambassador:adversary'] === 1 ? '' : 's'} · ${left.troop} troop${left.troop === 1 ? '' : 's'}</span>`;
  return `<div class="phead"><b>Adversary</b>${head}${pieces}${warBadge(v)}</div><div class="prow">${boxes.join(arrow)}</div>${projectsHtml(v)}`;
}

/**
 * The adversary's projects (017): the visible row, each with its compute bar (the view's
 * status is "n/cost compute"), then ongoing projects in play, the resolved count and
 * the deck.
 */
function projectsHtml(v: View): string {
  const row = pile(v, 'adversary.projects');
  const on = pile(v, 'adversary.projects_ongoing');
  const done = pile(v, 'adversary.projects_done');
  const deck = pile(v, 'adversary.project_deck');
  if (!row && !on && !done && !deck) return '';
  const cards = (row?.visible ?? []).map((c) =>
    `<div class="project" data-card="${esc(c.id)}" data-where="pile:adversary.projects"><span class="tname"><b>${esc(c.name)}</b>${progressBars(c.status)}</span><span class="ttext">${esc(c.text ?? '')}</span></div>`);
  const ongoing = on?.visible.length
    ? `<span class="agroup ongoing" data-pile="adversary.projects_ongoing"><span class="what">ongoing</span>${on.visible.map((c) => cardChip(c, 'pile:adversary.projects_ongoing')).join('')}</span>`
    : '';
  const last = done?.visible[done.visible.length - 1];
  const doneText = done ? `<span class="adeck" data-pile="adversary.projects_done" title="resolved or discarded">done <b>${done.count}</b>${last ? ` <small>last: <span data-card="${esc(last.id)}" data-where="pile:adversary.projects_done">${esc(last.name)}</span></small>` : ''}</span>` : '';
  const deckText = deck ? `<span class="adeck" data-pile="adversary.project_deck">deck <b>${deck.count}</b></span>` : '';
  return `<div class="projects-row" data-pile="adversary.projects"><span class="stitle">Projects</span>${cards.join('') || '<span class="empty">none visible</span>'}${ongoing}${doneText}${deckText}</div>`;
}

// ---------- countries strip: threats ----------

/** "1/2 energy, 0/3 credits" → bars. */
function progressBars(status: string): string {
  return status.split(',').map((part) => {
    const m = /(\d+)\s*\/\s*(\d+)\s*(.*)/.exec(part.trim());
    if (!m) return part.trim() ? `<span class="prog">${esc(part.trim())}</span>` : '';
    const [have, need] = [Number(m[1]), Number(m[2])];
    const pct = need ? Math.min(100, (100 * have) / need) : 100;
    return `<span class="prog${have >= need ? ' done' : ''}"><span class="fill" style="width:${pct}%"></span><span class="ptext">${have}/${need} ${esc(m[3])}</span></span>`;
  }).join('');
}

function threatCard(c: CardView, where: string): string {
  const effect = (c.text ?? '').replace(/\s*Counter:.*$/, '');
  // 044: a waiting or jammed threat must look it, not only in the hover.
  const waits = /Waits: /.test(c.text ?? '') ? '<span class="waits">waits</span>' : '';
  const jam = (c.text ?? '').match(/Jammed: needs (\d+) less/);
  const jammed = jam ? `<span class="jam">-${esc(jam[1])}</span>` : '';
  return `<div class="threat${waits ? ' waiting' : ''}" data-card="${esc(c.id)}" data-where="${where}"><span class="tname"><b>${esc(c.name)}</b>${waits}${jammed}${progressBars(c.status)}</span><span class="ttext">${esc(effect)}</span></div>`;
}

/**
 * A country's megaproject card: its name and a row of cube places, filled left to right
 * by owner (the adversary's, then each seat's), from `board.project*`.
 */
function megaprojectHtml(v: View, c: View['countries'][number]): string {
  const total = findValue(c.values, 'board.project');
  if (!total) return '';
  const name = total.label.replace(/^megaproject:\s*/, '');
  const max = total.max ?? 0;
  const done = !!findValue(c.values, 'board.project.done');
  const owners: { label: string; n: number; colour: string; key: string }[] = [];
  const adv = findValue(c.values, 'board.project.adversary');
  if (adv) owners.push({ label: 'adversary', n: adv.value, colour: ADVERSARY_COLOUR, key: adv.key });
  for (const s of v.seats) {
    const x = findValue(c.values, `board.project.${s.index}`);
    if (x) owners.push({ label: `seat ${s.index}`, n: x.value, colour: strong(v.countries[s.country]?.colour ?? '#888888'), key: x.key });
  }
  const cells: string[] = [];
  for (const o of owners) for (let i = 0; i < o.n && cells.length < max; i++) cells.push(`<span class="pc" style="background:${o.colour}" title="${esc(o.label)}"></span>`);
  while (cells.length < max) cells.push(`<span class="pc${done ? ' done' : ''}"></span>`);
  const by = done ? '<b class="pdone">completed</b>' : owners.filter((o) => o.n).map((o) => `<span data-value="${esc(o.key)}" data-scope="country:${c.index}">${esc(o.label)} ${o.n}</span>`).join(' · ');
  return `<div class="project${done ? ' done' : ''}" data-value="${esc(total.key)}" data-scope="country:${c.index}"><b title="megaproject">◆ ${esc(name)}</b><span class="cubes">${cells.join('')}</span><small>${total.value}/${max}${by ? ` · ${by}` : ''}</small></div>`;
}

export function countriesHtml(v: View): string {
  const boxes = v.countries.map((c) => {
    const seat = v.seats.find((s) => s.country === c.index);
    const ai = seat?.board?.name;
    const tok = (k: string) => findValue(c.values, `tokens.${k}`)?.value ?? 0;
    // Cities by owner cube: the adversary's (corrupted), a seat's (aligned), none (clean).
    const cities = v.hexes.filter((h) => h.country === c.index && h.kind === 'city');
    const corrupted = cities.filter((h) => h.tokens.some((t) => t.kind === 'cube:adversary')).length;
    const aligned = cities.filter((h) => h.tokens.some((t) => /^cube:\d+$/.test(t.kind))).length;
    // Only what is there: a clean country reads "cities 3 clean".
    const some = (n: number, what: string, cls = '') => (n ? [`<span class="${cls}">${n} ${what}</span>`] : []);
    const parts = [
      `cities ${cities.length - corrupted - aligned} clean`, ...some(aligned, 'aligned'), ...some(corrupted, 'corrupted', 'adv'),
      ...some(tok('ambassador:adversary'), 'amb.', 'adv'), ...some(tok('node'), `node${tok('node') === 1 ? '' : 's'}`, 'adv'),
      ...some(tok('troop'), `troop${tok('troop') === 1 ? '' : 's'}`, 'adv'),
      ...some(tok('unit'), `unit${tok('unit') === 1 ? '' : 's'}`),
    ];
    const counts = `<span class="counts" title="cities: clean, aligned, corrupted; adversary ambassadors, nodes and troops; corp units at bases">${parts.join(' · ')}</span>`;
    const flags = c.values.filter((x) => !x.key.startsWith('tokens.') && !x.key.startsWith('board.project')).map((x) => valueChip(x, `country:${c.index}`)).join('');
    const groups = c.cards.map((g) => g.key === 'threats.on'
      ? g.cards.map((x) => threatCard(x, `country:${c.index}:${g.key}`)).join('')
      : `<div class="row"><span class="what">${esc(g.label)}</span>${g.cards.map((x) => cardChip(x, `country:${c.index}:${g.key}`)).join('')}</div>`).join('');
    const project = megaprojectHtml(v, c);
    return `<div class="country" data-country="${c.index}" style="--tint:${strong(c.colour)};--soft:${c.colour}">
      <div class="chead"><b title="country ${c.index}">${esc(countryName(v, c.index))}</b>${seat ? ` seat ${seat.index}${ai ? ` · ${esc(ai)}` : ''}` : ''} ${counts}${flags}</div>
      ${project}
      ${groups || (project ? '' : '<div class="none">no threat</div>')}</div>`;
  });
  const row = pile(v, 'threats.row');
  if (row) {
    boxes.push(`<div class="country shared" style="--tint:#555;--soft:#eee"><div class="chead"><b>${esc(row.label)}</b></div>${row.visible.map((x) => threatCard(x, `pile:${row.key}`)).join('') || '<div class="none">none</div>'}</div>`);
  }
  return boxes.join('');
}

/** The board's key: hex kinds, the base pennant, fronts, and whose colour is whose. */
export function legendHtml(): string {
  const t = Object.values(KINDS).map((x) => `<span class="kind" style="background:${x.fill}">${x.label}</span>`).join('');
  const flag = `<svg viewBox="-0.05 -0.2 0.3 0.4" width="0.9em" height="1.2em" style="vertical-align:middle"><g class="base-mark">
    <path d="M0,0.17 V-0.17" class="pole"/><path d="M0,-0.17 L0.2,-0.1 L0,-0.03 Z" class="flag" fill="#888"/></g></svg>`;
  const front = `<svg viewBox="0 -0.1 0.6 0.2" width="1.8em" height="0.8em" style="vertical-align:middle"><g class="fronts"><line x1="0" y1="0" x2="0.6" y2="0" class="front-link"/></g></svg>`;
  return `${t}<span class="lg">${flag} base: flag in the colour of the country it faces</span><span class="lg">${front} front between two bases</span><span class="lg"><span class="sw" style="background:${ADVERSARY_COLOUR}"></span> adversary · seat colour = its country's</span>`;
}

// ---------- seat panels ----------

const SEAT_DRAWN = (k: string) =>
  k.startsWith('resource.') || k.startsWith('economy.income.') || k === 'cards.slots';

/** The seat's resources with income, its board's tracks, then other values. */
export function seatValuesHtml(v: View, seatIndex: number): string {
  const s = v.seats[seatIndex];
  const scope = `seat:${seatIndex}`;
  // Every resource any seat has, so seats show the same set (020: the Builder's
  // energy, 0 with no income, was missing while the Protector's showed).
  const names = new Set(v.seats.flatMap((x) => x.values).filter((x) => x.key.startsWith('resource.')).map((x) => x.key.slice('resource.'.length)));
  for (const x of s.values) {
    const r = x.key.startsWith('economy.income.') ? x.key.slice('economy.income.'.length) : null;
    if (r && x.value !== 0) names.add(r);
  }
  const order = ['credits', 'energy', 'insight', 'chips', 'antimatter', 'ambassadors'];
  const sorted = [...names].sort((a, b) => (order.indexOf(a) + 99) % 99 - (order.indexOf(b) + 99) % 99 || a.localeCompare(b));
  const resHtml = sorted.map((r) => {
    const have = findValue(s.values, `resource.${r}`);
    const inc = findValue(s.values, `economy.income.${r}`);
    // Insight is set to its income each round, not added to (020: "+4" read as more).
    const sign = r === 'insight' ? '→' : inc && inc.value >= 0 ? '+' : '';
    const incHtml = inc ? `<span class="inc" data-value="${inc.key}" data-scope="${scope}" title="${r === 'insight' ? 'next round: insight is set to this, plus any insight-next-round bonus' : 'income in upkeep'}">${sign}${inc.value}</span>` : '';
    return `<span class="res r-${esc(r)}" data-value="resource.${esc(r)}" data-scope="${scope}"><span class="label">${esc(have?.label ?? r.replace(/_/g, ' '))}</span> <b>${have?.value ?? 0}</b>${incHtml}</span>`;
  }).join('');

  const ai: string[] = [];
  const slots = findValue(s.values, 'cards.slots');
  const built = s.cards.find((g) => g.key === 'cards.built');
  if (slots) {
    const used = /(\d+)\s*\/\s*\d+/.exec(built?.label ?? '')?.[1] ?? String(built?.cards.length ?? 0);
    ai.push(`<span class="value" data-value="cards.slots" data-scope="${scope}"><span class="label">slots</span> <b>${used}<small>/${slots.value}</small></b></span>`);
  }
  const other = s.values.filter((x) => !SEAT_DRAWN(x.key)).map((x) => valueChip(x, scope)).join('');
  return `<div class="resources">${resHtml}</div>${boardHtml(v, seatIndex)}<div class="values ai">${ai.join('')}${other}</div>`;
}

/**
 * The seat's player board (`SeatView.board`): one row per track, every printed step
 * left to right. Uncovered steps are solid, covered ones dashed, the step in effect
 * is boxed; marks (`tier2`, `antimatter`, any other) sit under the step's value.
 */
function boardHtml(v: View, seatIndex: number): string {
  const b = v.seats[seatIndex].board;
  if (!b) return '';
  const tracks = b.tracks.map((t, ti) => {
    const steps = t.steps.map((st, i) => {
      // 044: with nothing uncovered no step is in effect (the second track at the start).
      const cls = ['step', i < t.uncovered ? 'open' : 'covered', i === t.current && t.uncovered > 0 ? 'current' : ''].filter(Boolean).join(' ');
      const marks = (st.marks ?? []).map((m) => `<span class="mark m-${esc(m.key)}" title="${esc(m.doc ?? m.key)}">${esc(m.label)}</span>`).join('');
      return `<span class="${cls}" data-track="${seatIndex}:${ti}:${i}"><b>${st.value}</b>${marks}</span>`;
    }).join('');
    return `<div class="track-row"><span class="label" title="${esc(t.label)}">${esc(t.resource || t.label)}</span>${steps}</div>`;
  }).join('');
  return `<div class="board" data-board="${seatIndex}">${tracks}</div>`;
}

/** Hover text for a track step drawn by `boardHtml` (`data-track="seat:track:step"`). */
export function trackStepDetails(v: View, ref: string): string | null {
  const [s, t, i] = ref.split(':').map(Number);
  const b = v.seats[s]?.board;
  const tr = b?.tracks[t];
  const st = tr?.steps[i];
  if (!b || !tr || !st) return null;
  const state = i === tr.current && tr.uncovered > 0 ? 'in effect' : i < tr.uncovered ? 'uncovered' : 'still covered';
  const marks = (st.marks ?? []).map((m) => `<br>${esc(m.label)}: ${esc(m.doc ?? m.key)}`).join('');
  return `<b>${esc(tr.label)}</b>, step ${i + 1} of ${tr.steps.length}: <b>${st.value}</b> (${state})${marks}<br>${tr.uncovered} uncovered · <code>${esc(b.key)}</code> · from ${esc(b.source)}`;
}

/** Hover text for a seat's board name. */
export function boardDetails(v: View, seatIndex: number): string | null {
  const b = v.seats[seatIndex]?.board;
  if (!b) return null;
  const cards = b.cards.map((c) => `${esc(c.name)} <small>(${esc(c.status)})</small>`).join('<br>');
  return `<b>${esc(b.name)}</b> <code>${esc(b.id)}</code>${b.text ? `<br>${esc(b.text)}` : ''}${b.innate ? `<hr><b>${esc(b.innate.name)}</b>${b.innate.text ? `: ${esc(b.innate.text)}` : ''}` : ''}${b.slots !== undefined ? `<br>${b.slots} slots printed` : ''}${cards ? `<hr>board cards:<br>${cards}` : ''}<br><small><code>${esc(b.key)}</code> · from ${esc(b.source)}</small>`;
}

/** "Coastal Republic", or `C<i>` if the view has no name. */
export const countryName = (v: View, i: number) => v.countries[i]?.name || `C${i}`;

export function seatCardsHtml(v: View, seatIndex: number): string {
  const s = v.seats[seatIndex];
  const order = ['cards.packet', 'cards.drafted', 'cards.hand', 'cards.built'];
  const groups = [...s.cards].sort((a, b) => ((order.indexOf(a.key) + 99) % 99) - ((order.indexOf(b.key) + 99) % 99));
  return groups.map((g) => {
    const cls = g.key.replace(/\W/g, '-');
    const body = g.cards.length ? g.cards.map((c) => cardChip(c, `seat:${seatIndex}:${g.key}`, true)).join('') : '<span class="slot">—</span>';
    return `<div class="row ${cls}"><span class="what" title="${esc(g.key)}">${esc(g.label)}</span>${body}</div>`;
  }).join('');
}

/**
 * Game-wide `vars` tracks that no module labels (source "engine"), folded away under
 * the piles. Modules keep their per-seat and per-card bookkeeping out of the globals
 * (`ViewOut::bookkeeping` in view.rs), so in the baseline this is normally empty; a new module's
 * unlabelled track shows here.
 */
export function varsHtml(v: View, open: boolean): string {
  const raw = v.globals.filter((x) => x.source === 'engine' && !DRAWN_GLOBALS.has(x.key));
  if (!raw.length) return '';
  return `<details id="vars"${open ? ' open' : ''}><summary>Unlabelled tracks (${raw.length} raw vars)</summary><div class="values">${raw.map((x) => valueChip(x, 'global', x.label, false)).join('')}</div></details>`;
}

// ---------- other piles ----------

export function pilesHtml(v: View): string {
  const rest = v.piles.filter((p) => !DRAWN_PILES.has(p.key));
  if (!rest.length) return '';
  return `<h3>Piles</h3><div class="pile-list">${rest.map((p) => {
    const shown = p.visible.slice(-3).map((c) => cardChip(c, `pile:${p.key}`)).join('');
    const more = p.visible.length > 3 ? `<small>+${p.visible.length - 3}</small>` : '';
    return `<div class="row pile" data-pile="${esc(p.key)}"><span class="what" title="${esc(p.key)}">${esc(p.label)}${p.owner !== null ? ` (seat ${p.owner})` : ''}: <b>${p.count}</b></span>${more}${shown}</div>`;
  }).join('')}</div>`;
}
