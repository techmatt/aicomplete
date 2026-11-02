// The card viewer's page: browse, filter, sort and annotate what cards-server.mjs exports.
// Player cards go through the one renderer; every other kind is a plain face drawn from the
// same export. No card words live here: only labels of the page itself.
import { fitCards, frameOf, inset, renderCard } from '../card/card.js';
import { esc, plainFace } from './plain.js';

const $ = (id) => document.getElementById(id);
const KINDS = ['program', 'building', 'event', 'threat', 'unity', 'discord', 'adversary', 'project', 'megaproject', 'ai'];
const LABEL = { ai: 'AI board', noted: 'has a note', unnoted: 'no note', changed: 'changed since note' };
const label = (k) => LABEL[k] ?? k;

// The grid's sizes in CSS px, for fitting two rows: a printed card's 88 mm at zoom 1, the
// stat line under it, and main's top padding and row gap (viewer.css).
const CARD_HEIGHT = (88 * 96) / 25.4;
const STAT_LINE = 20;
const ABOVE_ROWS = 12;
const ROW_GAP = 14;
const ROWS = 2;
const SIZE_KEY = 'aic-cards-size';
const FILTERS_KEY = 'aic-cards-filters-hidden';

let data = null;
let layout = null;
const ui = { kinds: new Set(), tags: new Set(), tiers: new Set(), keywords: new Set(), reviews: new Set(), sort: 'kind', desc: false, shown: [], open: null, index: 0 };

// Storage can be refused (private windows, a locked-down profile): the page works without it.
const stored = (key) => {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
};
const store = (key, value) => {
  try {
    if (value === null) localStorage.removeItem(key);
    else localStorage.setItem(key, value);
  } catch {
    // Remembered for this page only.
  }
};

function load(next) {
  data = next;
  layout = inset(data.layout);
  for (const f of data.flags) REVIEW[f] = (c) => noteOf(c)?.flag === f;
}

async function call(path, body) {
  const res = await fetch(path, body === undefined ? undefined : { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body) });
  const out = await res.json().catch(() => ({ error: `${res.status}` }));
  if (!res.ok) throw new Error(out.error ?? res.statusText);
  return out;
}

const status = (msg, bad = false) => {
  $('status').textContent = msg;
  $('status').title = msg;
  $('status').className = bad ? 'bad' : '';
};

// ---------- review state ----------

const noteOf = (c) => data.notes[c.id];
const isChanged = (c) => noteOf(c) && noteOf(c).hash !== c.hash;
const isNew = (c) => !data.seen[c.id] && !noteOf(c);
const REVIEW = {
  noted: (c) => !!noteOf(c),
  unnoted: (c) => !noteOf(c),
  changed: isChanged,
  new: isNew,
  // Then one per note flag: load() adds them from the server's list.
};

function badges(c) {
  const n = noteOf(c);
  return [
    isNew(c) ? '<span class="badge new">new</span>' : '',
    isChanged(c) ? '<span class="badge changed">changed since your note</span>' : '',
    n?.flag ? `<span class="badge ${n.flag.replace(' ', '-')}">${esc(n.flag)}</span>` : '',
    n && !n.flag ? '<span class="badge">note</span>' : '',
  ].join('');
}

// ---------- stats ----------

function stat(c, key) {
  if (key === 'reasoned') return c.reasoned;
  if (key === 'price_gap') return c.priceGap;
  return data.stats.byAgent[$('agent').value]?.[c.id]?.[key] ?? null;
}
const fmt = (v, col) => (v == null ? '' : `${col.key === 'win_delta' && v > 0 ? '+' : ''}${Number.isInteger(v) ? v : v.toFixed(col.unit === '' ? 1 : 0)}${col.unit}`);

// ---------- faces ----------

function printed(c) {
  const f = frameOf(c);
  const art = c.scene ? `/scene/${c.scene}.png` : undefined;
  const html = renderCard(c.render, layout, `/art/frames/r3/${f}.jpg`, data.fits[f], art);
  return `<div class="trim">${html.replaceAll('<img ', '<img loading="lazy" decoding="async" ')}</div>`;
}

const plain = (c) => plainFace(c, label(c.kind), c.scene ? `/scene/${c.scene}.png` : undefined);

function face(c) {
  if (!c.render) return plain(c);
  try {
    return printed(c);
  } catch (e) {
    return plain({ ...c, text: `${c.text} [renderer: ${e.message}]`, sections: [] });
  }
}

// ---------- filter and sort ----------

function matches(c) {
  if (ui.kinds.size && !ui.kinds.has(c.kind)) return false;
  if (ui.tags.size && !c.tags.some((t) => ui.tags.has(t))) return false;
  if (ui.tiers.size && !ui.tiers.has(c.tierLabel)) return false;
  const lo = $('cost-min').value;
  const hi = $('cost-max').value;
  if ((lo !== '' || hi !== '') && (c.cost == null || (lo !== '' && c.cost < +lo) || (hi !== '' && c.cost > +hi))) return false;
  if (ui.keywords.size && !c.keywords.some((k) => ui.keywords.has(k))) return false;
  if (ui.reviews.size && ![...ui.reviews].some((r) => REVIEW[r](c))) return false;
  const q = $('search').value.trim().toLowerCase();
  if (q) {
    const hay = [c.name, c.id, c.text, c.kind, c.tags.join(' '), c.keywords.join(' '), noteOf(c)?.comment, ...c.sections.map((s) => `${s.head} ${s.text}`)].join(' ').toLowerCase();
    if (!q.split(/\s+/).every((w) => hay.includes(w))) return false;
  }
  return true;
}

function sortValue(c, key) {
  if (key === 'name') return c.name.toLowerCase();
  if (key === 'cost') return c.cost;
  if (key === 'tier') return c.tier;
  if (key === 'kind') return KINDS.indexOf(c.kind);
  return stat(c, key);
}

/** Cards with no value for the key stay last whichever way it runs. */
function sorted(list) {
  const way = ui.desc ? -1 : 1;
  return list
    .map((c, i) => [c, sortValue(c, ui.sort), i])
    .sort(([a, x, i], [b, y, j]) => {
      if (x == null || y == null) return (x == null) - (y == null) || i - j;
      return (x < y ? -way : x > y ? way : 0) || a.name.localeCompare(b.name);
    })
    .map(([c]) => c);
}

// ---------- browse ----------

const statLine = (c) =>
  data.stats.columns
    .map((col) => [col, stat(c, col.key)])
    .filter(([, v]) => v != null)
    .map(([col, v]) => `${col.label.replace(' %', '')} ${fmt(v, col)}`)
    .join(' · ');

function grid(list) {
  $('cards').className = 'grid';
  $('cards').innerHTML = list.length
    ? list.map((c) => `<button class="tile" data-id="${esc(c.id)}"><div class="badges">${badges(c)}</div><div class="scaled">${face(c)}</div><div class="tile-stats" style="width:calc(63mm * var(--zoom, .6))">${esc(statLine(c))}</div></button>`).join('')
    : '<div class="empty">No card matches these filters.</div>';
  fitCards($('cards'));
}

function table(list) {
  const cols = [['name', 'Name'], ['kind', 'Kind'], ['tier', 'Tier'], ['cost', 'Cost', 'num']];
  const head = [...cols, ...data.stats.columns.map((c) => [c.key, c.label, 'num', c.title])];
  $('cards').className = 'list';
  $('cards').innerHTML = `<table><thead><tr>${head.map(([k, l, cls, title]) => `<th data-sort="${k}" class="${cls ?? ''} ${ui.sort === k ? 'on' : ''}" title="${esc(title ?? '')}">${esc(l)}${ui.sort === k ? (ui.desc ? ' ↓' : ' ↑') : ''}</th>`).join('')}<th>Review</th><th>Text</th></tr></thead><tbody>${list
    .map(
      (c) => `<tr data-id="${esc(c.id)}"><td>${esc(c.name)}</td><td>${esc(label(c.kind))}</td><td>${esc(c.tierLabel)}</td><td class="num" title="${esc(c.costLabel)}">${c.cost ?? ''}</td>${data.stats.columns.map((col) => `<td class="num">${esc(fmt(stat(c, col.key), col))}</td>`).join('')}<td>${badges(c)}</td><td class="text">${esc(noteOf(c)?.comment ? `${c.text} — note: ${noteOf(c).comment}` : c.text)}</td></tr>`,
    )
    .join('')}</tbody></table>`;
}

/** The size that fits two rows and their stat lines under the header, on the slider's steps. */
function fittedSize() {
  const z = $('zoom');
  const room = window.innerHeight - $('header').offsetHeight - ABOVE_ROWS - ROW_GAP - ROWS * STAT_LINE;
  const fit = Math.floor((room / (ROWS * CARD_HEIGHT)) * 20) / 20;
  return Math.min(Number(z.max), Math.max(Number(z.min), fit));
}

/** A size the slider chose stands, here and on the next visit; otherwise the size follows the window. */
function applySize() {
  const chosen = Number(stored(SIZE_KEY));
  const z = $('zoom');
  const own = chosen >= Number(z.min) && chosen <= Number(z.max);
  z.value = own ? chosen : fittedSize();
  $('fit').hidden = !own;
  document.documentElement.style.setProperty('--zoom', z.value);
}

function render() {
  const list = sorted(data.cards.filter(matches));
  ui.shown = list.map((c) => c.id);
  $('count').textContent = `${list.length} of ${data.cards.length}`;
  $('dir').textContent = ui.desc ? '↓' : '↑';
  $('zoom-box').hidden = $('view').value !== 'grid';
  reviewChips();
  filterBar();
  ($('view').value === 'grid' ? grid : table)(list);
  applySize();
}

/** The collapsed bar still says how many filters are on. */
function filterBar() {
  const on = ui.kinds.size + ui.tags.size + ui.tiers.size + ui.keywords.size + ui.reviews.size + ($('cost-min').value !== '' || $('cost-max').value !== '' ? 1 : 0);
  const hidden = $('filters').hidden;
  $('filters-toggle').textContent = `${hidden ? '▸' : '▾'} Filters${on ? ` (${on})` : ''}`;
  $('filters-toggle').setAttribute('aria-expanded', String(!hidden));
}

// Its counts move with every note, so it is drawn with the cards, not with the controls.
function reviewChips() {
  chips($('reviews'), Object.keys(REVIEW), ui.reviews, (v) => data.cards.filter(REVIEW[v]).length);
}

function chips(el, values, set, count) {
  for (const v of set) if (!values.includes(v)) set.delete(v);
  el.innerHTML = values.map((v) => `<button class="chip ${set.has(v) ? 'on' : ''}" data-v="${esc(v)}">${esc(label(v))} <small>${count(v)}</small></button>`).join('');
}

function setSelect(el, options) {
  const was = el.value;
  el.innerHTML = options.map(([v, l]) => `<option value="${esc(v)}">${esc(l)}</option>`).join('');
  if (options.some(([v]) => v === was)) el.value = was;
}

function controls() {
  const of = (k) => data.cards.filter(k).length;
  const present = KINDS.filter((k) => data.cards.some((c) => c.kind === k));
  const unknown = [...new Set(data.cards.map((c) => c.kind))].filter((k) => !KINDS.includes(k));
  chips($('kinds'), [...present, ...unknown], ui.kinds, (v) => of((c) => c.kind === v));
  const tags = [...new Set(data.cards.flatMap((c) => c.tags))].sort();
  chips($('tags'), tags, ui.tags, (v) => of((c) => c.tags.includes(v)));
  const tiers = [...new Set(data.cards.map((c) => c.tierLabel))].filter((t) => /^(Unique|Tier|Stage)/.test(t)).sort();
  chips($('tiers'), tiers, ui.tiers, (v) => of((c) => c.tierLabel === v));
  const keywords = [...new Set(data.cards.flatMap((c) => c.keywords))].sort();
  chips($('keywords'), keywords, ui.keywords, (v) => of((c) => c.keywords.includes(v)));
  setSelect($('sort'), [['kind', 'Kind'], ['cost', 'Cost'], ['tier', 'Tier'], ['name', 'Name'], ...data.stats.columns.map((c) => [c.key, c.label])]);
  $('sort').value = [...$('sort').options].some((o) => o.value === ui.sort) ? ui.sort : 'kind';
  ui.sort = $('sort').value;
  const src = data.stats.source;
  $('agent-box').hidden = !src;
  if (src) {
    const was = $('agent').value;
    setSelect($('agent'), src.agents.map((a) => [a, a]));
    if (!was || was !== $('agent').value) $('agent').value = src.agent;
  }
  const notes = [...data.warnings];
  if (src && src.revision !== data.revision) notes.unshift(`Stats are from ${src.path} at r${src.revision} (${src.games} games), the newest screen with per-card counts; the rules are r${data.revision}, so they describe older cards.`);
  if (!src) notes.unshift('No screen results with per-card counts in this checkout: the stat columns are off.');
  $('notice').innerHTML = notes.map((n) => `<div>${esc(n)}</div>`).join('');
  document.title = `AI Complete: cards, r${data.revision}`;
  status(`r${data.revision}${data.commit ? ` · ${data.commit}` : ''} · exported ${data.exported}`);
}

// ---------- the large view ----------

const prettyOp = ({ op, ...args }) => `${op}${Object.keys(args).length ? ` ${JSON.stringify(args)}` : ''}`;

function opsBlock(c) {
  const lists = Object.entries(c.ops).filter(([, l]) => l?.length);
  if (!lists.length) return '';
  return `<h4>Ops</h4><pre class="ops">${lists.map(([k, l]) => `<b>${esc(k)}</b>\n${l.map((o) => `  ${esc(prettyOp(o))}`).join('\n')}`).join('\n')}</pre>`;
}

function pricingBlock(c) {
  if (!c.pricing.length) return '';
  const line = (p) =>
    p.cells
      ? `<div class="pricing"><small>${esc(p.section)}</small>${p.cells.map((v, i) => `${i ? `${esc(p.columns[i] ?? '')}: ` : ''}${esc(v)}`).join(' · ')}</div>`
      : `<div class="pricing"><small>${esc(p.section)}${p.mention ? ', in passing' : ''}</small>${esc(p.text)}</div>`;
  return `<h4>From docs/card-pricing.md</h4>${c.pricing.map(line).join('')}`;
}

function statsBlock(c) {
  const rows = data.stats.columns.map((col) => [col, stat(c, col.key)]).filter(([, v]) => v != null);
  if (!rows.length) return '';
  const src = data.stats.source;
  const raw = data.stats.byAgent[$('agent').value]?.[c.id];
  return `<h4>Measured${src ? `: ${esc($('agent').value)}, r${src.revision}` : ''}</h4><dl>${rows.map(([col, v]) => `<dt title="${esc(col.title)}">${esc(col.label)}</dt><dd>${esc(fmt(v, col))}</dd>`).join('')}${raw ? `<dt>Offered / kept / drawn / played</dt><dd>${raw.offered} / ${raw.kept} / ${raw.drawn} / ${raw.played}</dd>` : ''}</dl>`;
}

function detail(id) {
  const c = data.cards.find((x) => x.id === id);
  if (!c) return closeDetail();
  ui.open = id;
  ui.index = Math.max(ui.shown.indexOf(id), 0);
  const n = noteOf(c);
  const dlg = $('detail');
  dlg.innerHTML = `<div class="detail" tabindex="-1">
    <div class="left"><div class="scaled">${face(c)}</div></div>
    <div>
      <div class="top"><div><h2>${esc(c.name)}</h2><div class="sub">${esc(label(c.kind))} · ${esc(c.tierLabel)}${c.costLabel ? ` · ${esc(c.costLabel)}` : ''} · <code>${esc(c.id)}</code></div></div>
        <div class="nav"><button data-go="-1" title="Previous (←)">←</button><button data-go="1" title="Next (→)">→</button><button data-close title="Close (Esc)">✕</button></div></div>
      ${c.text ? `<h4>Text</h4><div>${esc(c.text)}</div>` : ''}
      ${c.sections.map((s) => `<h4>${esc(s.head)}</h4><div>${esc(s.text)}</div>`).join('')}
      ${c.tags.length || c.keywords.length ? `<h4>Tags and keywords</h4><div>${esc([...c.tags, ...c.keywords].join(', '))}</div>` : ''}
      ${c.price.length ? `<h4>Price inputs</h4><dl>${c.price.map(([k, v]) => `<dt>${esc(k)}</dt><dd>${esc(v)}</dd>`).join('')}</dl>` : ''}
      ${statsBlock(c)}
      ${pricingBlock(c)}
      ${opsBlock(c)}
      <form class="note" id="note">
        <h4>Your note</h4>
        ${isChanged(c) ? `<div class="was"><b>Changed since your note.</b> The text then: ${esc(n.text || '(none)')}</div>` : ''}
        <div class="flags">${data.flags.map((f) => `<button type="button" class="chip ${n?.flag === f ? 'on' : ''}" data-flag="${esc(f)}">${esc(f)}</button>`).join('')}</div>
        <textarea id="comment" placeholder="Comment">${esc(n?.comment)}</textarea>
        <div class="row"><button type="submit">Save</button><span>Ctrl+Enter saves. A note with no flag and no comment is removed.</span><span id="saved"></span></div>
      </form>
    </div>
  </div>`;
  fitCards(dlg);
  if (!dlg.open) dlg.showModal();
  // On the panel, not the comment: the arrow keys step between cards until you click in.
  dlg.querySelector('.detail').focus({ preventScroll: true });
  if (!data.seen[id]) {
    data.seen[id] = c.hash;
    document.querySelector(`[data-id="${CSS.escape(id)}"] .badge.new`)?.remove();
    call('/api/seen', { ids: [id] }).catch((e) => status(e.message, true));
  }
}

/** What the open card's note form holds. */
const formNote = () => ({ flag: $('detail').querySelector('[data-flag].on')?.dataset.flag ?? '', comment: $('comment').value.trim() });

/** What the form holds, when it differs from the saved note. */
function unsaved() {
  if (!ui.open || !$('comment')) return null;
  const { flag, comment } = formNote();
  const n = data.notes[ui.open];
  return flag === (n?.flag ?? '') && comment === (n?.comment ?? '') ? null : { flag, comment };
}

/** Leaving a card keeps what was typed. False if it could not be saved: stay on the card. */
async function keepNote() {
  const note = unsaved();
  if (!note) return true;
  try {
    load(await call('/api/notes', { id: ui.open, ...note }));
    render();
    return true;
  } catch (e) {
    $('saved').textContent = `Not saved: ${e.message}`;
    return false;
  }
}

async function closeDetail() {
  if (!(await keepNote())) return;
  ui.open = null;
  if ($('detail').open) $('detail').close();
}

async function step(by) {
  if (!(await keepNote())) return;
  // A save can drop the card from a review filter: its neighbors are then either side of
  // where it stood, and the next one has moved into its place.
  const here = ui.shown.indexOf(ui.open);
  const n = ui.shown.length;
  const i = here >= 0 ? here + by : by > 0 ? ui.index : ui.index - 1;
  if (n) detail(ui.shown[((i % n) + n) % n]);
  else closeDetail();
}

async function saveNote() {
  const id = ui.open;
  const top = $('detail').querySelector('.detail').scrollTop;
  try {
    load(await call('/api/notes', { id, ...formNote() }));
    render();
    if (ui.open === id) {
      detail(id);
      $('detail').querySelector('.detail').scrollTop = top;
      $('saved').textContent = 'Saved.';
    }
  } catch (e) {
    $('saved').textContent = `Not saved: ${e.message}`;
  }
}

// ---------- wiring ----------

function toggleChip(e, set) {
  const v = e.target.closest('.chip')?.dataset.v;
  if (v === undefined) return;
  if (!set.delete(v)) set.add(v);
  controls();
  render();
}

function wire() {
  $('kinds').onclick = (e) => toggleChip(e, ui.kinds);
  $('tags').onclick = (e) => toggleChip(e, ui.tags);
  $('tiers').onclick = (e) => toggleChip(e, ui.tiers);
  $('keywords').onclick = (e) => toggleChip(e, ui.keywords);
  $('reviews').onclick = (e) => toggleChip(e, ui.reviews);
  for (const id of ['search', 'cost-min', 'cost-max']) $(id).oninput = render;
  for (const id of ['view', 'agent']) $(id).onchange = render;
  $('zoom').oninput = () => {
    store(SIZE_KEY, $('zoom').value);
    applySize();
  };
  $('fit').onclick = () => {
    store(SIZE_KEY, null);
    applySize();
  };
  $('filters').hidden = stored(FILTERS_KEY) === '1';
  $('filters-toggle').onclick = () => {
    $('filters').hidden = !$('filters').hidden;
    store(FILTERS_KEY, $('filters').hidden ? '1' : null);
    filterBar();
  };
  new ResizeObserver(applySize).observe($('header'));
  window.addEventListener('resize', applySize);
  $('sort').onchange = () => {
    ui.sort = $('sort').value;
    render();
  };
  $('dir').onclick = () => {
    ui.desc = !ui.desc;
    render();
  };
  $('clear').onclick = () => {
    for (const s of [ui.kinds, ui.tags, ui.tiers, ui.keywords, ui.reviews]) s.clear();
    for (const id of ['search', 'cost-min', 'cost-max']) $(id).value = '';
    controls();
    render();
  };
  $('seen-all').onclick = async () => {
    const { seen } = await call('/api/seen', { all: true });
    data.seen = seen;
    render();
  };
  $('rebuild').onclick = async () => {
    $('rebuild').disabled = true;
    status('Exporting…');
    try {
      const out = await call('/api/rebuild', {});
      load(out);
      controls();
      render();
      status(`${$('status').textContent} · rebuilt in ${out.ms} ms`);
    } catch (e) {
      status(`Rebuild failed, the last export stands: ${e.message}`, true);
    }
    $('rebuild').disabled = false;
  };
  $('cards').onclick = (e) => {
    const th = e.target.closest('th[data-sort]');
    if (th) {
      if (ui.sort === th.dataset.sort) ui.desc = !ui.desc;
      else ui.sort = th.dataset.sort;
      $('sort').value = ui.sort;
      return render();
    }
    const id = e.target.closest('[data-id]')?.dataset.id;
    if (id) detail(id);
  };
  const dlg = $('detail');
  dlg.onclick = (e) => {
    if (e.target === dlg || e.target.closest('[data-close]')) return closeDetail();
    const go = e.target.closest('[data-go]');
    if (go) return step(Number(go.dataset.go));
    const flag = e.target.closest('[data-flag]');
    if (flag) {
      const was = flag.classList.contains('on');
      dlg.querySelectorAll('[data-flag]').forEach((b) => b.classList.remove('on'));
      if (!was) flag.classList.add('on');
    }
  };
  dlg.onsubmit = (e) => {
    e.preventDefault();
    saveNote();
  };
  dlg.onkeydown = (e) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      e.preventDefault();
      saveNote();
    } else if (e.target.tagName !== 'TEXTAREA' && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) {
      step(e.key === 'ArrowLeft' ? -1 : 1);
    }
  };
  dlg.oncancel = (e) => {
    e.preventDefault();
    closeDetail();
  };
  window.addEventListener('beforeunload', (e) => {
    if (unsaved()) e.preventDefault();
  });
}

async function start() {
  wire();
  try {
    load(await call('/api/cards'));
    controls();
    render();
    window.cardViewer = { get data() { return data; } };
  } catch (e) {
    status(`Could not load the cards: ${e.message}`, true);
  }
}

start();
