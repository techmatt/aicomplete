// 242: a player card as it prints (web/card/), for the player view's hover and draft. The printed card's
// stylesheet would clash with the viewer's `.card` chips, so each face lives in its own shadow root.
// Content rows come from content/cards.json and the face words from web/card/faces.json, as the card
// viewer's export joins them (`render: { ...row, ...face }`). 325: a card's scene when the dev server serves it
// downsized (`loadScenes`); else the frame alone.

import { fitCards, frameOf, inset, renderCard } from '../../card/card.js';
import cardCss from '../../card/card.css?inline';

const rows = Object.values(import.meta.glob('../../../content/cards.json', { eager: true, import: 'default' }))[0] as { id: string; kind: string; tier: number }[];
const faces = Object.values(import.meta.glob('../../card/faces.json', { eager: true, import: 'default' }))[0] as Record<string, object>;
const layoutRaw = Object.values(import.meta.glob('../../card/layout.json', { eager: true, import: 'default' }))[0];
const fits = Object.values(import.meta.glob('../../../art/frames/r3/fit.json', { eager: true, import: 'default' }))[0] as Record<string, unknown>;
const frames = Object.fromEntries(
  Object.entries(import.meta.glob('../../../art/frames/r3/*.jpg', { eager: true, query: '?url', import: 'default' }))
    .map(([p, url]) => [p.split('/').pop()!.replace('.jpg', ''), url as string]),
);
const layout = inset(layoutRaw);
const byId = new Map(rows.map((r) => [r.id, r]));

// @font-face does not work inside a shadow root: the page carries the faces once.
let fontsAdded = false;
function addFonts() {
  if (fontsAdded) return;
  fontsAdded = true;
  const st = document.createElement('style');
  // (A built bundle's `.match(...) ?? []` came out as `.match(...).join`, which threw on a page's first card.)
  const rules: string[] = [];
  for (const m of String(cardCss).matchAll(/@font-face\s*\{[^}]*\}/g)) rules.push(m[0]);
  st.textContent = rules.join('\n');
  document.head.append(st);
}

/** 325: card id -> its scene's URL, from the dev server (vite.config.ts's `sceneArt`); empty on a built site. */
let scenes: Record<string, string> = {};

/** Asks the dev server which cards have a scene drawn; before the first card prints (a host is cached with its art). */
export async function loadScenes() {
  try {
    const r = await fetch('./__scenes.json');
    if (r.ok && /json/.test(r.headers.get('content-type') ?? '')) scenes = await r.json();
  } catch {
    scenes = {};
  }
}

/** Whether `id` is a player card this view can print. */
export const printable = (id: string) => byId.has(id) && frames[frameOf(byId.get(id)!)] !== undefined;

/** Card size in px at scale 1 (69 x 94 mm). */
export const CARD_PX = { w: (69 * 96) / 25.4, h: (94 * 96) / 25.4 };

/**
 * A host element holding the printed card `id` at `scale`, fitted (the card viewer's text ladder) once it is
 * in the page. Null for a card this view has no face for.
 */
export function printedCard(id: string, scale: number): HTMLElement | null {
  const row = byId.get(id);
  if (!row) return null;
  const frame = frameOf(row);
  const url = frames[frame];
  if (!url) return null;
  addFonts();
  const host = document.createElement('div');
  host.className = 'printed-host';
  host.style.width = `${CARD_PX.w * scale}px`;
  host.style.height = `${CARD_PX.h * scale}px`;
  const root = host.attachShadow({ mode: 'open' });
  const html = renderCard({ ...row, ...(faces[id] ?? {}) }, layout, url, fits[frame], scenes[id]);
  root.innerHTML = `<style>${cardCss}\n.scaled{transform:scale(${scale});transform-origin:0 0;width:69mm;height:94mm}</style><div class="scaled">${html}</div>`;
  // Fit after the host is attached (measuring needs layout); a detached host fits on its next frame.
  requestAnimationFrame(() => fitHost(host));
  return host;
}

/** 324: fits a printed host's text once it is in the page (`data-fitted` marks it done); a host still detached waits
 *  for a later call (the player view's cache puts a host back in the page and calls this again). */
export function fitHost(host: HTMLElement) {
  if (!host.isConnected || host.dataset.fitted || !host.shadowRoot) return;
  host.dataset.fitted = '1';
  void fitCards(host.shadowRoot);
}
