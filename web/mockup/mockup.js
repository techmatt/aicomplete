// 327: the solo screen's static mockup (mockup.html), for Matt to react to before any view work: a real mid-game
// solo state (state.json, from scripts/mockup-state.mjs) drawn from printed faces only: the card renderer for the
// hand, buildings and adversary cards, print.js for the board and research strip, the map as the client drew it.
// Rules text is on hover (each face's title); no derived or internal numbers on screen. Not wired to play.
import { esc, fitCards, frameOf, inset, renderCard } from '../card/card.js';
import { icon, tagMedallion } from '../card/icons.js';
import { renderBoard, renderEvent, renderProject, renderResearch, renderThreat, PRINT_FRAME } from '../card/print.js';

const small = new URLSearchParams(location.search).get('size') === '1366';
document.body.classList.add(small ? 's1366' : 's1920');
const [data, state] = await Promise.all([fetch('/api/cards').then((r) => r.json()), fetch('/mockup/state.json').then((r) => r.json())]);
const layout = inset(data.layout);
const byId = new Map(data.cards.map((c) => [c.id, c]));
const frame = (f) => `/art/frames/r3/${f}.jpg`;
const scene = (c) => (c.scene ? `/scene/${c.scene}.png` : undefined);
const v = state.view;
const seat = v.seats[0];
const val = (k) => seat.values.find((x) => x.key === k)?.value ?? 0;
const glob = (k) => v.globals.find((x) => x.key === k) ?? { value: 0 };
const pile = (k) => v.piles.find((p) => p.key === k) ?? { count: 0, visible: [] };
const group = (k) => seat.cards.find((g) => g.key === k)?.cards ?? [];

// Card scales: a printed card is 63 × 88 mm at zoom 1.
const Z = small ? { hand: 0.44, top: 0.4, board: 0.42, research: 0.6, threat: 0.4 } : { hand: 0.74, top: 0.56, board: 0.62, research: 0.86, threat: 0.56 };

const wrap = (z, html, title = '', cls = '') => `<div class="sc ${cls}" style="--z:${z}"${title ? ` title="${esc(title)}"` : ''}>${html}</div>`;
const trim = (html) => `<div class="trim">${html}</div>`;

/** A player card as it prints, its words on hover. */
function playerCard(id, z) {
  const c = byId.get(id);
  const f = frameOf(c.render);
  return wrap(z, trim(renderCard(c.render, layout, frame(f), data.fits[f], scene(c))), `${c.name}: ${c.text}`);
}

/** An adversary card (event, project, threat) as it prints; its words are on the face's own title. */
function advCard(id, z, filled = 0) {
  const c = byId.get(id);
  const kind = c.kind === 'adversary' ? 'event' : c.kind;
  const f = PRINT_FRAME[kind];
  const html = kind === 'event'
    ? renderEvent(c.print, layout, frame(f), data.fits[f], scene(c))
    : kind === 'project'
      ? renderProject(c.print, layout, frame(f), data.fits[f], scene(c), filled)
      : renderThreat(c.print, layout, frame(f), data.fits[f], scene(c));
  return wrap(z, trim(html));
}

// ---------- the adversary strip ----------

const slot = (label, html) => `<div class="slot"><div class="slot-label">${label}</div>${html ?? '<div class="empty-card"></div>'}</div>`;
const eventIn = (k) => pile(k).visible[0]?.id;
const events = [['Last', 'adversary.last'], ['This', 'adversary.current'], ['Next', 'adversary.next']]
  .map(([label, k]) => slot(label, eventIn(k) ? advCard(eventIn(k), Z.top) : null)).join('');
const projects = pile('adversary.projects').visible.map((p) => {
  const filled = Number(/^(\d+)\//.exec(p.status ?? '')?.[1] ?? 0);
  return advCard(p.id, Z.top, filled);
}).join('');
const research = glob('adversary.research');
const cubes = Array.from({ length: research.max ?? 3 }, (_, i) => `<span class="cube-socket${i < research.value ? ' on' : ''}"></span>`).join('');
const researchStrip = `<div class="research-box"><div class="sc" style="--z:${Z.research}">${renderResearch(data.research, glob('adversary.research_slot').value, 'War Research')}</div>
  <div class="research-cubes" title="Research cubes on the track: every ${research.max ?? 3} advance the marker a slot">${icon('research_track')}${cubes}</div></div>`;

// ---------- the status column ----------

const RESOURCES = ['credits', 'energy', 'chips', 'antimatter'];
const resources = RESOURCES.map((r) => `<div class="res" title="${r}: ${val(`resource.${r}`)} now, +${val(`economy.income.${r}`)} each round">
  <span class="ico-l">${icon(r)}</span><b>${val(`resource.${r}`)}</b><span class="inc">+${val(`economy.income.${r}`)}</span></div>`).join('');
const thisRound = `<div class="res small" title="Insight to spend this round (+${val('economy.income.insight')} each round, from your card-play track)"><span class="ico-l">${icon('insight')}</span><b>${val('resource.insight')}</b><span class="inc">+${val('economy.income.insight')}</span></div>
  <div class="res small" title="Ambassadors in your supply; ready on the map: ${val('board.ambassadors_ready')}"><span class="ico-l">${icon('ambassador')}</span><b>${val('resource.ambassadors')}</b><span class="inc">${val('board.ambassadors_ready')} ready</span></div>`;
const tags = seat.values.filter((x) => x.key.startsWith('cards.tag.') && x.value > 0)
  .map((x) => `<span class="tagc" title="${x.value} ${x.key.slice(10)} tag${x.value === 1 ? '' : 's'} on your cards and board">${tagMedallion(x.key.slice(10))}<b>${x.value}</b></span>`).join('');
const corrupted = glob('alignment.corrupted_cities');
const win = `<div class="win" title="You win when no city is corrupted: ${corrupted.value} still are">${Array.from({ length: corrupted.value }, () => icon('city')).join('')}<span>→ 0</span></div>`;
const unity = glob('alignment.unity');
const discord = glob('alignment.discord');
const pips = (n, max, cls) => Array.from({ length: max }, (_, i) => `<span class="pip ${cls}${i < n ? ' on' : ''}"></span>`).join('');
const alignment = `<div class="align" title="Unity ${unity.value} of ${unity.max}: tips into a Unity card when full. Discord ${discord.value} of ${discord.max}: tips into a Discord card when full.">
  <span class="side u">Unity</span><span class="bar"><span class="half u">${pips(unity.value, unity.max, 'u')}</span><span class="mid"></span><span class="half">${pips(discord.value, discord.max, 'd')}</span></span><span class="side d">Discord</span></div>`;
/**
 * A small ring gauge: the count a component shows (`left`), the arc its share of the whole that component holds
 * (`of`, null: no whole to measure against, a plain ring). Nothing derived: each number is on the table.
 */
const gauge = (name, glyph, left, of, why, low = of ? left / of <= 0.3 : false) => {
  const f = of ? Math.max(0, Math.min(1, left / of)) : 1;
  const a = 2 * Math.PI * 15;
  return `<div class="gauge${low ? ' low' : ''}${of ? '' : ' plain'}" title="${esc(why)}"><svg viewBox="0 0 36 36"><circle cx="18" cy="18" r="15" class="g-back"/><circle cx="18" cy="18" r="15" class="g-arc" stroke-dasharray="${(a * f).toFixed(1)} ${a.toFixed(1)}"/></svg>
    <span class="g-ico">${icon(glyph)}</span><b>${left}</b><span class="g-name">${name}</span></div>`;
};
// The cards left on the clock, as the phase bar counts them (the deck and the next card, face up).
const deckLeft = glob('adversary.deck').value;
const slotAt = glob('adversary.research_slot');
const discordLeft = pile('alignment.deck.discord').count;
const discordAll = discordLeft + pile('alignment.resolved.discord').count;
// The deck's whole: the cards left on the clock and the rounds already played (one card a round).
const clocks = [
  gauge('Event deck', 'deck', deckLeft, deckLeft + v.round, `Adversary cards left: ${deckLeft}. You lose when a round has no card.`),
  // The marker's slot, as the strip shows it: the arc runs down as it nears the last.
  gauge('Research slot', 'research_track', slotAt.value, null, `The research marker is on slot ${slotAt.value} of ${slotAt.max}: you lose when it reaches the last.`, (slotAt.max ?? 7) - slotAt.value <= 2),
  gauge('Discord deck', 'discord', discordLeft, discordAll, `Discord cards left: ${discordLeft}. You lose when Discord tips with none left.`),
  gauge('Nodes', 'node', val('board.node'), null, `Your nodes on the map: ${val('board.node')}. You lose with none left.`, val('board.node') <= 2),
].join('');

// ---------- the prompt and phase bar ----------

const promptText = (state.words.prompt[0] ?? '').split(' — ')[0];
const answers = state.words.buttons.filter((b) => !/Replay|▸/.test(b.label));
const prompt = `<div class="prompt"><span class="ask">${esc(promptText)}</span>${answers.map((b, i) => `<button class="${i === 0 ? 'primary' : ''}">${esc(b.label)}</button>`).join('')}</div>`;
const steps = (state.words.phases.find((p) => p.includes('›')) ?? 'Adversary › Cards › Actions › Income › Threats').split(' › ');
const round = state.words.phases[0] ?? `Round ${v.round}`;
const NOW = 'Actions';
const phaseBar = `<div class="phases"><span class="round">${esc(round)}</span>${steps.map((s, i) => `<span class="step${s === NOW ? ' now' : ''}${steps.indexOf(NOW) > i ? ' done' : ''}"><i>${i + 1}</i>${esc(s)}</span>`).join('')}</div>`;

// ---------- the board, hand and buildings ----------

const boardView = seat.board;
const powers = byId.get(boardView.id).print;
const usedNames = boardView.cards.filter((p) => p.status === 'used').map((p) => p.name);
const usedGroups = powers.powers.filter((p) => p.group && usedNames.includes(p.name)).map((p) => p.group);
const [eco, play] = boardView.tracks;
const board = wrap(Z.board, renderBoard(powers, { economy: eco.uncovered, play: play.uncovered, used: [...usedNames, ...usedGroups] }));
const hand = group('cards.hand').map((c) => playerCard(c.id, Z.hand)).join('');
const built = group('cards.built');
// The built cards, then the board's empty slots as narrow dashed sockets (a building goes in one).
const slots = built.map((c) => playerCard(c.id, Z.hand)).join('')
  + Array.from({ length: boardView.slots - built.length }, () => wrap(Z.hand, '<div class="trim slot-empty" title="An empty building slot"></div>', '', 'empty-slot')).join('');

// ---------- threats ----------

const threats = v.threat_sources.map((t) => advCard(t.threat, Z.threat)).join('');

document.getElementById('screen').innerHTML = `
  <header class="top">${phaseBar}</header>
  <section class="adversary">
    <div class="group"><div class="group-label">Adversary cards</div><div class="row">${events}</div></div>
    <div class="group"><div class="group-label">Projects</div><div class="row">${projects}</div></div>
    <div class="group"><div class="group-label">Research</div>${researchStrip}</div>
  </section>
  <aside class="status">
    <div class="block res-block"><div class="block-label">Resources</div>${resources}${thisRound}</div>
    <div class="block"><div class="block-label">Tags</div><div class="tags">${tags}</div></div>
    <div class="block"><div class="block-label">To win</div>${win}</div>
    <div class="block"><div class="block-label">Alignment</div>${alignment}</div>
    <div class="block"><div class="block-label">Loss clocks</div><div class="clocks">${clocks}</div></div>
  </aside>
  <main class="map-area">${prompt}<div class="map">${state.map}</div></main>
  <aside class="threats"><div class="block-label">Threats</div>${threats}</aside>
  <footer class="tableau">
    <div class="group"><div class="group-label">${esc(boardView.name)}</div>${board}</div>
    <div class="group grow"><div class="group-label">Hand</div><div class="row hand">${hand}</div></div>
    <div class="group"><div class="group-label">Buildings</div><div class="row">${slots}</div></div>
  </footer>`;

await fitCards(document);
window.mockupReady = true;
