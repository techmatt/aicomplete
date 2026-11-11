// HTML for the panels around the board: the Alignment track and loss conditions, the
// adversary row (deck, this round's card, the face-up next card, discard), the countries
// strip (megaprojects, threats), and the seat panels' contents. Everything comes from
// the generic view; known keys (adversary.*, alignment.*, board.*, threats.*, ai.*,
// economy.income.*, cards.*) get a dedicated drawing, every other value, card group or
// pile falls back to a generic chip so nothing goes unshown.

import { content, param, type CardView, type Config, type PileView, type Value, type View } from './engine';
import { ADVERSARY_COLOR, NEUTRAL_COLOR, countryName, escapeHtml, extraCosts, hexKey, KINDS, strong } from './format';

const esc = escapeHtml;

/** Global values drawn by the status strip or the adversary row; the rest are generic chips. */
const DRAWN_GLOBALS = new Set([
  'adversary.stage', 'adversary.difficulty', 'adversary.deck', 'adversary.compute',
  'threats.tier', 'alignment.unity', 'alignment.discord', 'alignment.unity_tips',
  'alignment.discord_tips', 'alignment.bar',
  'board.fronts',
]);
/** Piles drawn by the status strip, the adversary row or the countries strip. */
const DRAWN_PILES = new Set([
  'adversary.deck', 'adversary.next', 'adversary.current', 'adversary.discard', 'threats.row',
  'alignment.deck.unity', 'alignment.deck.discord', 'alignment.resolved.unity', 'alignment.resolved.discord',
  'adversary.projects', 'adversary.projects_ongoing', 'adversary.projects_done', 'adversary.project_deck',
]);

const findValue = (vs: Value[], key: string) => vs.find((v) => v.key === key);
/** A key's place in `order`; a key not in it sorts after every one that is. */
const orderRank = (order: string[]) => (key: string) => (order.indexOf(key) + 99) % 99;
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
  // 118: a draft or keep tile says what the card needs (keeps were picked blind to it).
  const need = /^(draft|keep)/.test(c.status) ? /needs ([^\]]*?)(?:, built over|\])/.exec(c.text ?? '')?.[1] : undefined;
  const needs = need ? ` <small class="needs" title="tags it needs on your cards to play">needs ${esc(need)}</small>` : '';
  // 024: a card repeats its rules text under its name, as Terraforming Mars' do.
  // The view's text starts with a bracketed line ("[building, tier 1, industry, +1 insight]
  // ..."): kind, tags and every cost but credits. Printed chips show the rules text only.
  const rules = (c.text ?? '').replace(/^\[[^\]]*\]\s*/, '');
  const text = printed && rules ? `<span class="ctext">${esc(rules)}</span>` : '';
  return `<span class="card${printed ? ' printed' : ''} ${esc(c.status.replace(/\s+/g, '-'))}" data-card="${esc(c.id)}" data-where="${where}">${esc(c.name)}${cost}${needs}${status}${text}</span>`;
}

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

/** Adversary pieces on the board, by kind. */
function onBoard(v: View, kinds: string[]): Record<string, number> {
  const n: Record<string, number> = Object.fromEntries(kinds.map((k) => [k, 0]));
  for (const h of v.hexes) for (const t of h.tokens) if (t.kind in n) n[t.kind] += t.count;
  return n;
}

// ---------- status: alignment and losses ----------

/** One of the two Alignment tracks (136: independent, no Undecided pool between them). */
function bucketBar(b: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!b) return '';
  const pct = b.max ? Math.min(100, (100 * b.value) / b.max) : 0;
  return `<span class="bar ${side}" data-value="${esc(b.key)}" data-scope="global"><span class="fill" style="width:${pct}%"></span>
    <span class="btext">${name} ${b.value}/${b.max ?? '?'}</span></span>`;
}

/** "Unity tips 1 (next step at 2 tips)". */
function tipsChip(t: Value | undefined, side: 'unity' | 'discord', name: string): string {
  if (!t) return '';
  return `<span class="tips ${side}" data-value="${esc(t.key)}" data-scope="global">${name} tips <b>${t.value}</b>${t.max !== undefined ? `<small> (next step at ${t.max} tips)</small>` : ''}</span>`;
}

/** The Alignment line, and its resolved cards and decks for the line under it; nothing without the module. */
function alignmentHtml(v: View): { track: string; cards: string[] } {
  const g = v.globals;
  const unity = findValue(g, 'alignment.unity');
  const discord = findValue(g, 'alignment.discord');
  const bar = findValue(g, 'alignment.bar');
  if (!unity && !discord && !bar) return { track: '', cards: [] };
  const level = bar?.value ?? 0;
  const max = bar?.max ?? 0;
  const track = `<div class="alignment">
      <span class="alabel">Alignment</span>
      <span class="tug">${bucketBar(unity, 'unity', 'Unity')}${bucketBar(discord, 'discord', 'Discord')}</span>
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
  // 184: a game that is over stays red under its WON banner: no danger once there is an outcome.
  const live = !v.outcome;
  const dd = pile(v, 'alignment.deck.discord');
  if (dd) {
    const danger = live && dd.count <= 1 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-pile="${esc(dd.key)}" title="a Discord tip that finds no card loses the game"><span class="label">Discord deck runs out</span> <b>${dd.count}</b></span>`);
  }
  const deck = findValue(g, 'adversary.deck');
  if (deck) {
    const danger = live && deck.value <= 1 ? ' danger' : '';
    loss.push(`<span class="value${danger}" data-value="${deck.key}" data-scope="global"><span class="label">adversary cards run out</span> <b>${deck.value}</b><small> left</small></span>`);
  }
  // 044: the third locked loss: a seat's last node, if this config has it.
  const counts = v.seats.map((s) => s.values.find((x) => x.key === 'board.node')?.value);
  if (param(config, 'alignment', 'node_loss') && counts.every((n) => n !== undefined) && counts.length) {
    const least = Math.min(...(counts as number[]));
    loss.push(`<span class="value${live && least <= 1 ? ' danger' : ''}" title="a seat with no nodes on the board loses the game (no stage protects a last node: 175)"><span class="label">a seat's last node gone</span> <b>${least}</b><small> fewest held</small></span>`);
  }
  // Unlabeled `vars` (source "engine") are folded under the piles: see `varsHtml`.
  const other = g.filter((x) => !DRAWN_GLOBALS.has(x.key) && x.source !== 'engine').map((x) => valueChip(x, 'global'));
  // The other tracks (Unity bonuses) beside the losses, not in them
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

/** An adversary card face under the box's title: name, stage, then its printed text. */
function adversaryCard(c: CardView, where: string, title: string): string {
  const def = content.adversary.find((a) => a.id === c.id);
  // The view's text starts "stage n: "; the stage is drawn as a chip instead.
  const text = def?.text ?? (c.text ?? '').replace(/^stage \d+:\s*/, '');
  const stage = def ? `<span class="stage">stage ${def.stage}</span>` : '';
  return `<div class="acard" data-card="${esc(c.id)}" data-where="${where}"><div class="sline"><span class="stitle">${title}</span><span class="sname">${esc(c.name)}</span>${stage}</div><div class="atext">${esc(text)}</div></div>`;
}

function cardBox(p: PileView | undefined, title: string, empty: string, cls: string): string {
  if (!p) return '';
  const cards = p.visible.length
    ? p.visible.map((c) => adversaryCard(c, `pile:${p.key}`, title)).join('')
    : `<div class="sline"><span class="stitle">${title}</span></div><span class="empty">${esc(empty)}</span>`;
  return `<div class="slot-box ${cls}" data-pile="${esc(p.key)}">${cards}</div>`;
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
    ? `<div class="slot-box deck" data-pile="adversary.deck"><div class="sline"${next?.visible.length ? ` title="${deck.count} in the deck and ${next.visible.length} face up for next round: the lose-if count counts both"` : ''}><span class="stitle">Deck</span><span class="big">${deck.count}</span><small>left${next?.visible.length ? ` (+${next.visible.length})` : ''}</small></div>${peek}<div class="hint">the clock: lose when no card is left for the next round</div></div>`
    : '';
  const last = di?.visible[0];
  const discBox = di
    ? `<div class="slot-box discard" data-pile="adversary.discard"><div class="sline"><span class="stitle">Discard</span><span class="big">${di.count}</span></div>${last ? `<div class="hint">last: <span data-card="${esc(last.id)}" data-where="pile:adversary.discard">${esc(last.name)}</span></div>` : ''}</div>`
    : '';
  const arrow = '<span class="parrow">→</span>';
  const boxes = [
    deckBox,
    // 184: "(face up)" headed an empty slot reading "not revealed yet".
    cardBox(next, next?.visible.length ? 'Next round (face up)' : 'Next round', 'not revealed yet', 'next'),
    cardBox(cur, 'This round', 'no card this round', 'current'),
    discBox,
  ].filter(Boolean);
  const left = onBoard(v, ['ambassador:adversary', 'troop']);
  const pieces = `<span class="left" title="adversary pieces on the board">on board: ${left['ambassador:adversary']} agent${left['ambassador:adversary'] === 1 ? '' : 's'} · ${left.troop} troop${left.troop === 1 ? '' : 's'}</span>`;
  return `<div class="phead"><b>Adversary</b>${head}${pieces}</div><div class="prow">${boxes.join(arrow)}</div>${projectsHtml(v)}`;
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

/** 070: a threat's source type, its hexes (hover marks them) and who is refused. */
function sourceBadge(v: View, id: string): { badge: string; hexes: string } {
  const s = v.threat_sources.find((x) => x.threat === id);
  if (!s) return { badge: '', hexes: '' };
  const hexes = s.hexes.map((i) => hexKey(v.hexes[i].hex)).join(' ');
  const refused = v.seats.filter((x) => !s.seats.includes(x.index)).map((x) => x.index);
  const none = s.seats.length === 0;
  // 145: the r54+ rule (threats.md, *Near*): a node or an ambassador on or next to a location token.
  const says = none ? 'no one near it: no one can counter'
    : refused.length ? `seat ${refused.join(', ')}: not near it, can't counter` : '';
  const n = s.hexes.length;
  const title = `${s.source.replace(/\+/g, ', ')}: ${n} location token${n === 1 ? '' : 's'} on the map. You are near it (you may pay toward it; its harm hits you) with a node or ambassador on or next to a token.`;
  return {
    badge: `<span class="tsource ${esc(s.source)}" title="${esc(title)}">${esc(s.source)}</span>${says ? `<span class="trefused">${esc(says)}</span>` : ''}`,
    hexes,
  };
}

function threatCard(c: CardView, where: string, v: View): string {
  // 184: the frame every threat prints since r77 (payers, target) filled the 3-line clamp before the effect showed;
  // the hover keeps it.
  const effect = (c.text ?? '').replace(/\s*Counter:.*$/, '').replace(/^Payers:.*?Target:[^.]*\.\s*/, '');
  // 044: a waiting or jammed threat must look it, not only in the hover (184: r77 prints "Waits token:").
  const waits = /Waits( token)?: /.test(c.text ?? '') ? '<span class="waits">waits</span>' : '';
  const jam = (c.text ?? '').match(/Jammed: needs (\d+) less/);
  const jammed = jam ? `<span class="jam">-${esc(jam[1])}</span>` : '';
  const src = sourceBadge(v, c.id);
  return `<div class="threat${waits ? ' waiting' : ''}" data-card="${esc(c.id)}" data-where="${where}"${src.hexes ? ` data-hexes="${src.hexes}"` : ''}><span class="tname"><b>${esc(c.name)}</b>${src.badge}${waits}${jammed}${progressBars(c.status)}</span><span class="ttext">${esc(effect)}</span></div>`;
}

/**
 * A country's megaproject card: its name and a row of cube places, filled left to right
 * by owner (the adversary's, the neutral stations', then each seat's), from `board.project*`.
 * A completed card says how many cubes paid, overflow included.
 */
function megaprojectHtml(v: View, c: View['countries'][number]): string {
  const total = findValue(c.values, 'board.project');
  if (!total) return '';
  const name = total.label.replace(/^megaproject:\s*/, '');
  const max = total.max ?? 0;
  const done = !!findValue(c.values, 'board.project.done');
  const owners: { label: string; n: number; color: string; key: string }[] = [];
  const adv = findValue(c.values, 'board.project.adversary');
  if (adv) owners.push({ label: 'adversary', n: adv.value, color: ADVERSARY_COLOR, key: adv.key });
  const neutral = findValue(c.values, 'board.project.neutral');
  if (neutral) owners.push({ label: 'neutral', n: neutral.value, color: NEUTRAL_COLOR, key: neutral.key });
  for (const s of v.seats) {
    const x = findValue(c.values, `board.project.${s.index}`);
    if (x) owners.push({ label: `seat ${s.index}`, n: x.value, color: strong(v.countries[s.country]?.color ?? '#888888'), key: x.key });
  }
  const cells: string[] = [];
  for (const o of owners) for (let i = 0; i < o.n; i++) cells.push(`<span class="pc" data-color="${esc(o.label)}" style="background:${o.color}" title="${esc(o.label)}"></span>`);
  while (cells.length < max) cells.push(`<span class="pc${done ? ' done' : ''}"></span>`);
  const paid = findValue(c.values, 'board.project.paid');
  const by = done ? `<b class="pdone">completed</b>${paid ? ` · <span class="ppaid" data-value="${esc(paid.key)}" data-scope="country:${c.index}">${paid.value} cubes paid (${esc(paid.label)})</span>` : ''}` : owners.filter((o) => o.n).map((o) => `<span data-value="${esc(o.key)}" data-scope="country:${c.index}">${esc(o.label)} ${o.n}</span>`).join(' · ');
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
      ...some(tok('ambassador:adversary'), `agent${tok('ambassador:adversary') === 1 ? '' : 's'}`, 'adv'), ...some(tok('botnet'), `botnet${tok('botnet') === 1 ? '' : 's'}`, 'adv'),
      ...some(tok('troop'), `troop${tok('troop') === 1 ? '' : 's'}`, 'adv'),
      ...some(tok('unit'), `drone${tok('unit') === 1 ? '' : 's'}`),
    ];
    const counts = `<span class="counts" title="cities: clean, aligned, corrupted; agents, botnets and troops; drones at bases">${parts.join(' · ')}</span>`;
    // 184: an on/off round flag (board.view's FLAGS) printed a constant "1" ("Sheltered 1"): its label alone.
    const onOff = /^board\.(blackout|sabotage|shelter|embargo|cordon)$/;
    const flags = c.values.filter((x) => !x.key.startsWith('tokens.') && !x.key.startsWith('board.project'))
      .map((x) => (onOff.test(x.key)
        ? `<span class="value" data-value="${esc(x.key)}" data-scope="country:${c.index}"><span class="label">${esc(x.label)}</span></span>`
        : valueChip(x, `country:${c.index}`))).join('');
    const groups = c.cards.map((g) => g.key === 'threats.on'
      ? g.cards.map((x) => threatCard(x, `country:${c.index}:${g.key}`, v)).join('')
      : `<div class="row"><span class="what">${esc(g.label)}</span>${g.cards.map((x) => cardChip(x, `country:${c.index}:${g.key}`)).join('')}</div>`).join('');
    const project = megaprojectHtml(v, c);
    return `<div class="country" data-country="${c.index}" style="--tint:${strong(c.color)};--soft:${c.color}">
      <div class="chead"><b title="country ${c.index}">${esc(countryName(v, c.index))}</b>${seat ? ` seat ${seat.index}${ai ? ` · ${esc(ai)}` : ''}` : ''} ${counts}${flags}</div>
      ${project}
      ${groups || (project ? '' : '<div class="none">no threat</div>')}</div>`;
  });
  const row = pile(v, 'threats.row');
  if (row) {
    boxes.push(`<div class="country shared" style="--tint:#555;--soft:#eee"><div class="chead"><b>${esc(row.label)}</b></div>${row.visible.map((x) => threatCard(x, `pile:${row.key}`, v)).join('') || '<div class="none">none</div>'}</div>`);
  }
  return boxes.join('');
}

/** The board's key: hex kinds, the base pennant, fronts, and whose color is whose. */
export function legendHtml(): string {
  const t = Object.values(KINDS).map((x) => `<span class="kind" style="background:${x.fill}">${x.label}</span>`).join('');
  const flag = `<svg viewBox="-0.05 -0.2 0.3 0.4" width="0.9em" height="1.2em" style="vertical-align:middle"><g class="base-mark">
    <path d="M0,0.17 V-0.17" class="pole"/><path d="M0,-0.17 L0.2,-0.1 L0,-0.03 Z" class="flag" fill="#888"/></g></svg>`;
  const front = `<svg viewBox="0 -0.1 0.6 0.2" width="1.8em" height="0.8em" style="vertical-align:middle"><g class="fronts war"><line x1="0" y1="0" x2="0.6" y2="0" class="front-link"/></g></svg>`;
  return `${t}<span class="lg">${flag} base: flag in the color of the country it faces</span><span class="lg">${front} front between two bases</span><span class="lg"><span class="sw" style="background:${ADVERSARY_COLOR}"></span> adversary · seat color = its country's</span>`;
}

// ---------- seat panels ----------

const SEAT_DRAWN = (k: string) =>
  k.startsWith('resource.') || k.startsWith('economy.income.') || k === 'cards.slots' || k.startsWith('board.converter.');

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
  const rank = orderRank(['credits', 'energy', 'insight', 'chips', 'antimatter', 'research_cubes', 'ambassadors']);
  const sorted = [...names].sort((a, b) => rank(a) - rank(b) || a.localeCompare(b));
  const resHtml = sorted.map((r) => {
    const have = findValue(s.values, `resource.${r}`);
    const inc = findValue(s.values, `economy.income.${r}`);
    // Insight is set to its income each round, not added to (020: "+4" read as more).
    // 075: under Patience it adds, unspent insight carrying over.
    const patient = !!findValue(s.values, 'economy.insight_carried');
    // 184: the badge promised its income beside "insight lost next round 3": what next round brings is the income
    // plus the gift less the debt (economy.rs's round start; under Patience only the total with what is carried is
    // floored, so the change itself may be negative).
    const debt = r === 'insight' ? findValue(s.values, 'economy.insight_debt')?.value ?? 0 : 0;
    const gift = r === 'insight' ? findValue(s.values, 'economy.insight_gift')?.value ?? 0 : 0;
    const next = inc ? (r === 'insight' ? (patient ? inc.value + gift - debt : Math.max(0, inc.value + gift - debt)) : inc.value) : 0;
    const sign = r === 'insight' && !patient ? '→' : next >= 0 ? '+' : '';
    const incHtml = inc ? `<span class="inc" data-value="${inc.key}" data-scope="${scope}" title="${r === 'insight' ? (patient ? 'next round: this is added to what you hold (Patience): the income, plus any insight next round, less any insight lost next round' : 'next round: insight is set to this: the income, plus any insight next round, less any insight lost next round') : 'income in upkeep'}">${sign}${next}</span>` : '';
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
  // 074: every completed card's converter; one this seat can't use says why.
  const converters = s.values.filter((x) => x.key.startsWith('board.converter.')).map((x) =>
    `<span class="converter ${x.value ? 'usable' : 'unusable'}" data-value="${esc(x.key)}" data-scope="${scope}">${esc(x.label)}</span>`).join('');
  return `<div class="resources">${resHtml}</div>${converters ? `<div class="converters">${converters}</div>` : ''}${boardHtml(v, seatIndex)}<div class="values ai">${ai.join('')}${other}</div>`;
}

/**
 * The seat's player board (`SeatView.board`): one row per track, every printed step
 * left to right. Uncovered steps are solid, covered ones dashed, the step in effect
 * is boxed; marks (`TrackMark`) sit under the step's value.
 */
function boardHtml(v: View, seatIndex: number): string {
  const b = v.seats[seatIndex].board;
  if (!b) return '';
  const tracks = b.tracks.map((t, ti) => {
    const steps = t.steps.map((st, i) => {
      // 044: with nothing uncovered no step is in effect (the second track at the start).
      const cls = ['step', i < t.uncovered ? 'open' : 'covered', i === t.current && t.uncovered > 0 ? 'current' : ''].filter(Boolean).join(' ');
      // 179 (Matt, friction #16): the space's cost prints as a cost, first, in the cost style.
      const shown = [...(st.marks ?? [])].sort((a, b) => Number(b.key === 'cost') - Number(a.key === 'cost'));
      const marks = shown.map((m) => `<span class="mark m-${esc(m.key)}" title="${esc(m.doc ?? m.key)}">${esc(m.label)}</span>`).join('');
      return `<span class="${cls}" data-track="${seatIndex}:${ti}:${i}"><b>${st.value}</b>${marks}</span>`;
    }).join('');
    return `<div class="track-row"><span class="label" title="${esc(t.label)}">${esc(t.label)}</span>${steps}</div>`;
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
  return `<b>${esc(b.name)}</b> <code>${esc(b.id)}</code>${b.text ? `<br>${esc(b.text)}` : ''}${b.innate ? `<hr><b>${esc(b.innate.name)}</b>${b.innate.text ? `: ${esc(b.innate.text)}` : ''}` : ''}${b.slots !== undefined ? `<br>${b.slots} slots printed` : ''}${b.tags?.length ? `<br>printed tags: ${esc(b.tags.join(', '))}` : ''}${cards ? `<hr>board cards:<br>${cards}` : ''}<br><small><code>${esc(b.key)}</code> · from ${esc(b.source)}</small>`;
}

export function seatCardsHtml(v: View, seatIndex: number): string {
  const s = v.seats[seatIndex];
  const rank = orderRank(['cards.packet', 'cards.drafted', 'cards.hand', 'cards.built']);
  const groups = [...s.cards].sort((a, b) => rank(a.key) - rank(b.key));
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
 * unlabeled track shows here.
 */
export function varsHtml(v: View, open: boolean): string {
  const raw = v.globals.filter((x) => x.source === 'engine' && !DRAWN_GLOBALS.has(x.key));
  if (!raw.length) return '';
  return `<details id="vars"${open ? ' open' : ''}><summary>Unlabeled tracks (${raw.length} raw vars)</summary><div class="values">${raw.map((x) => valueChip(x, 'global', x.label, false)).join('')}</div></details>`;
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
