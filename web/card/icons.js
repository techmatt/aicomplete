// The rendered icon set. Containers are painted once, in the master frame's steel
// (containers/*.png, sliced by tools/card-art/containers.mjs); glyphs are vector, drawn on
// top here. The shape says the category, by the locked rules: resources are square tiles,
// tokens on cards are discs, board pieces are bare silhouettes, tags are ringed circles.

const svg = (body, cls = '', box = 24) => `<svg class="glyph ${cls}" viewBox="0 0 ${box} ${box}" aria-hidden="true">${body}</svg>`;
const line = (d, w = 2.2) => `<path d="${d}" fill="none" stroke="currentColor" stroke-width="${w}" stroke-linecap="round" stroke-linejoin="round"/>`;
const fill = (d) => `<path d="${d}" fill="currentColor"/>`;

const EYE = `${line('M2.5 12 Q12 2.5 21.5 12 Q12 21.5 2.5 12Z')}<circle cx="12" cy="12" r="3.6" fill="currentColor"/>`;

// A square tile: its face colour, its glyph.
const RESOURCES = {
  credits: ['#d8a31a', `<circle cx="12" cy="12" r="7.5" fill="none" stroke="currentColor" stroke-width="2.4"/>${line('M14.8 9.2 Q12 7.6 10 9.2 Q8.6 10.9 11 12 Q15.4 13.2 13.8 15.2 Q11.6 16.8 9 15', 1.9)}`],
  insight: ['#7a48b8', EYE],
  chips: ['#2f8f6e', `<rect x="7" y="7" width="10" height="10" rx="1.2" fill="currentColor"/>${line('M9.5 3.5v3.5M14.5 3.5v3.5M9.5 17v3.5M14.5 17v3.5M3.5 9.5H7M3.5 14.5H7M17 9.5h3.5M17 14.5h3.5', 2)}`],
  energy: ['#e0761a', fill('M13.8 2 5.5 13.4h5.2L9.2 22l9.3-12.6h-5.6z')],
  antimatter: ['#c22f6d', `${line('M12 12m-2.6 0a2.6 2.6 0 1 0 5.2 0a2.6 2.6 0 1 0-5.2 0')}<g fill="none" stroke="currentColor" stroke-width="1.7"><ellipse cx="12" cy="12" rx="10" ry="3.9"/><ellipse cx="12" cy="12" rx="10" ry="3.9" transform="rotate(60 12 12)"/><ellipse cx="12" cy="12" rx="10" ry="3.9" transform="rotate(120 12 12)"/></g>`],
  compute: ['#3a6fc4', `<rect x="4.5" y="4.5" width="15" height="15" rx="2" fill="none" stroke="currentColor" stroke-width="2.2"/>${fill('M8 8h3.4v3.4H8zM12.6 12.6H16V16h-3.4z')}`],
  cards: ['#5d6875', `<rect x="4" y="5.5" width="10.5" height="15" rx="1.6" fill="none" stroke="currentColor" stroke-width="2"/><rect x="9.5" y="3" width="10.5" height="15" rx="1.6" fill="currentColor"/>`],
};

// A disc: a token that lives on a card.
const TOKENS = {
  datasets: ['#2379ab', `<g fill="none" stroke="currentColor" stroke-width="2"><ellipse cx="12" cy="6.5" rx="7" ry="3"/><path d="M5 6.5v11c0 1.7 3 3 7 3s7-1.3 7-3v-11M5 12c0 1.7 3 3 7 3s7-1.3 7-3"/></g>`],
  capital: ['#a9812b', fill('M3 20h18v-2H3zM5 17h3v-6H5zM10.5 17h3v-6h-3zM16 17h3v-6h-3zM3 10h18L12 3.5z')],
  followers: ['#a24f8d', `<circle cx="8" cy="8" r="3.2" fill="currentColor"/><circle cx="16" cy="8" r="3.2" fill="currentColor"/>${fill('M2.5 20c0-5 2.2-8 5.5-8 1.5 0 2.8.6 3.8 1.8-1.1 1.6-1.7 3.8-1.8 6.2zM12 20c0-5 1.200-8 4-8s5.500 3 5.500 8z')}`],
  agents: ['#465160', `<circle cx="12" cy="12" r="7" fill="none" stroke="currentColor" stroke-width="2"/><circle cx="12" cy="12" r="2.2" fill="currentColor"/>${line('M12 2v6M12 16v6M2 12h6M16 12h6', 2)}`],
};

// A bare silhouette: a board piece, or moving one.
const AMBASSADOR = '<circle cx="12" cy="6" r="4.2" fill="currentColor"/><path d="M4 22.500c0-8 3-11.500 8-11.500s8 3.500 8 11.500z" fill="currentColor"/>';
const PIECES = {
  ambassador: AMBASSADOR,
  presence: fill('M12 2 4 8.500V22h16V8.500zM9.500 13h5v9h-5z'),
  factory: fill('M3 22V10l6 4v-4l6 4V3h6v19z'),
  // Moving a piece: the silhouette with an arrow.
  move: `<g transform="translate(-3.500 2) scale(.84)">${AMBASSADOR}</g>${line('M14.500 12h8M19 8l3.500 4-3.500 4', 2.4)}`,
  // Harden and shelter act on a country's board, so they are bare too.
  harden: fill('M12 1.500 3.500 4.500v7c0 5.500 3.500 9 8.500 11 5-2 8.500-5.500 8.500-11v-7z'),
  shelter: `${fill('M2 13 12 3l10 10h-3v0z')}${line('M5.500 14v7.500h13V14', 2.4)}`,
  // A country, for "in one country": the map's hex cluster.
  country: fill('M8 2.500 12 4.800v4.600L8 11.700 4 9.400V4.800zM16.500 2.500l4 2.300v4.600l-4 2.300-4-2.300V4.800zM12.200 12.300l4 2.300v4.600l-4 2.300-4-2.300v-4.600z'),
  project: `<rect x="3.500" y="4" width="17" height="16" rx="1" fill="none" stroke="currentColor" stroke-width="2.200"/>${fill('M7 8h10v2H7zM7 12h6v2H7z')}`,
  hex: line('M12 2 20.700 7v10L12 22 3.300 17V7z', 2.4),
  troop: fill('M12 2.500 15 7.500h-1.800v2.300l6.800 3.700v3l-6.800-1.800V19l2.300 1.700V22L12 21l-3.500 1v-1.300L10.800 19v-4.300L4 16.500v-3l6.800-3.700V7.500H9z'),
  node: `<circle cx="12" cy="12" r="4.200" fill="currentColor"/>${line('M12 2.500v5M12 16.500v5M2.500 12h5M16.500 12h5M5.300 5.300l3.500 3.500M15.200 15.200l3.500 3.500M18.700 5.300l-3.500 3.500M8.800 15.200l-3.500 3.500', 1.900)}`,
  cube: fill('M12 2 21 7v10l-9 5-9-5V7zm0 2.300L6 7.600l6 3.300 6-3.300zM5 9.300v6.500l6 3.300v-6.500z'),
  city: fill('M3 22V11h5V6h3V2h3v8h7v12z'),
  building: `<rect x="4" y="3" width="16" height="19" rx="1" fill="currentColor"/><path d="M8 7h3v3H8zM13 7h3v3h-3zM8 12h3v3H8zM13 12h3v3h-3z" fill="#efeadc"/>`,
  slot: `<rect x="4" y="3" width="16" height="19" rx="1.500" fill="none" stroke="currentColor" stroke-width="2.200" stroke-dasharray="3.500 2.500"/>`,
  // The Alignment track: a marker moving toward Unity.
  sway: `${line('M3 12h18M3 8v8M21 8v8', 2.200)}${fill('M9 6.500 16 12l-7 5.500z')}`,
  marker: fill('M12 2 22 12 12 22 2 12zM12 7l-5 5 5 5 5-5z'),
  threat: fill('M12 2 23 21H1zM10.800 9v6.500h2.400V9zm0 7.800v2.400h2.400v-2.400z'),
  // A delay: the threat waits.
  clock: `<circle cx="12" cy="12" r="9.500" fill="none" stroke="currentColor" stroke-width="2.400"/>${line('M12 6.500V12l4 2.500', 2.400)}`,
  // A threat struck through: countered.
  counter: `${fill('M12 2 23 21H1z')}${line('M3 3l18 18', 3)}`,
  // The adversary deck, looked at.
  peek: `<rect x="5" y="2.500" width="14" height="19" rx="1.600" fill="none" stroke="currentColor" stroke-width="2"/><g transform="translate(5.400 5.400) scale(.55)">${EYE}</g>`,
};

const TAGS = {
  science: ['#49a9e6', `${line('M9.500 3h5M10.500 3v7L5 19.500c-.5 1 .2 1.800 1.200 1.800h11.600c1 0 1.700-.8 1.200-1.800L13.500 10V3')}${fill('M7.500 16h9l1.800 3.800H5.700z')}`],
  power: ['#efb92f', fill('M13.800 2 5.500 13.400h5.200L9.200 22l9.300-12.600h-5.600z')],
  cyber: ['#52c985', `<rect x="6.500" y="6.500" width="11" height="11" rx="1.500" fill="none" stroke="currentColor" stroke-width="2.200"/>${line('M9.500 2.500v4M14.500 2.500v4M9.500 17.500v4M14.500 17.500v4M2.500 9.500h4M2.500 14.500h4M17.500 9.500h4M17.500 14.500h4', 2)}`],
  diplomacy: ['#e8a35c', `${line('M2 10.500 7 7l5 2 5-2 5 3.500-10 9.500z')}${line('M8 13l4 3.500M12 9.500l-3 3', 1.800)}`],
  industry: ['#c98d62', fill('M3 22V10l6 4v-4l6 4V3h6v19z')],
  security: ['#ec6a5c', fill('M12 1.500 3.500 4.500v7c0 5.500 3.500 9 8.500 11 5-2 8.500-5.500 8.500-11v-7z')],
};

/** Is this word in a card's icon line an icon, rather than a number or connector? */
export const isIcon = (name) => name in RESOURCES || name in TOKENS || name in PIECES;
/** A tag in a count, written `#science` in an icon line. */
export const isTag = (name) => name.startsWith('#') && name.slice(1) in TAGS;

export function icon(name) {
  if (RESOURCES[name]) return `<span class="ico tile" style="--c:${RESOURCES[name][0]}" aria-label="${name}">${svg(RESOURCES[name][1])}</span>`;
  if (TOKENS[name]) return `<span class="ico disc" style="--c:${TOKENS[name][0]}" aria-label="${name}">${svg(TOKENS[name][1])}</span>`;
  if (PIECES[name]) return `<span class="ico piece" aria-label="${name}">${svg(PIECES[name])}</span>`;
  throw new Error(`no icon ${name}`);
}

export function tagMedallion(tag) {
  const t = TAGS[tag];
  if (!t) throw new Error(`no tag ${tag}`);
  return `<span class="medallion" style="--c:${t[0]}" aria-label="${tag}">${svg(t[1])}</span>`;
}

/** Connectors, vector like the glyphs. */
export const arrow = () => svg(line('M2.500 12h17M13 5.500l7 6.500-7 6.500', 3), 'arrow');
/** The stacked cards of "replaces". */
export const stacked = () => svg('<rect x="3" y="7.500" width="12" height="14" rx="1.600" fill="#efeadc" stroke="#23262a" stroke-width="1.600"/><rect x="6" y="5" width="12" height="14" rx="1.600" fill="#efeadc" stroke="#23262a" stroke-width="1.600"/><rect x="9" y="2.500" width="12" height="14" rx="1.600" fill="#efeadc" stroke="#23262a" stroke-width="1.600"/>', 'stack');
/** The tiny eye of the insight hexagon. */
export const eye = () => svg(EYE, 'eye');
