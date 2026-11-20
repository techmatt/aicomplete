// The pieces cut from the game that must not show in the viewer, shared by player-play.mjs and troops-play.mjs:
// `installCut(page)` defines `window.__cut.removed()` in the page, which lists every cut piece on screen now.
//
// 292: a base hex, pennant, front or Drone drawn on the map, an Envoy or Buy energy button, a fronts or host value,
// a War or drones phase in the round bar, and their words in the page's visible text and hovers (`CUT_WORDS`), a
// card's own name aside ("War Rooms", "Survey Drone Fleet": read from content/).
// 293: Spill, Surge, Grow and seeding.
// 294: the march and its beat, Muster and its token, the purge's steps and the clear-board bar.
import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/** The cut pieces' words. "in front of you" is no front; a capital War is the step and its log lines. */
export const CUT_WORDS = String.raw`\b[Dd]rones?\b|\b[Mm]ilitary bases?\b|\bEnvoy\b|Buy a city's energy|\b[Hh]ost(?:s|ed|ing)?\b|\b[Ff]ronts?\b(?! of)|\bWar\b|\bwar (?:step|phase)\b|Warmongering|Twin City Compact|nearest players`
  // 293: the adversary's Spill, Surge and Grow lines and a botnet's seeding (the players' Growth powers stay; a
  // singular "seed" alone is the game's: "Seed", "Random seed").
  + String.raw`|\b[Ss]pill(?:s|ed)?\b|\b[Ss]urg(?:e|es|ed)\b|\b[Ss]eed(?:s|ed|ing)\b|[Bb]otnet(?:'s|s)? seed|\bseed (?:a |another )?[Bb]otnet|\bGrow\b`
  // 294: the march and its beat, Muster and its token, the purge's steps and the clear-board bar (the win's end
  // reason code is "liberated").
  + String.raw`|\b[Mm]arch(?:es|ed|ing)?\b|\b[Mm]uster(?:s|ed|ing)?\b|\b[Pp]urg(?:e|es|ed|ing)\b|clear-board|\bbar \d+ ?/ ?\d+`;

/** Content names that hold one of those words: taken out of the text before it is read. */
export function cutWordNames() {
  const dir = join(dirname(fileURLToPath(import.meta.url)), '..', '..', 'content');
  const re = new RegExp(CUT_WORDS);
  const names = new Set();
  const walk = (v) => {
    if (Array.isArray(v)) v.forEach(walk);
    else if (v && typeof v === 'object') {
      if (typeof v.name === 'string' && re.test(v.name)) names.add(v.name);
      Object.values(v).forEach(walk);
    }
  };
  for (const f of readdirSync(dir).filter((x) => x.endsWith('.json'))) walk(JSON.parse(readFileSync(join(dir, f), 'utf8')));
  return [...names].sort((a, b) => b.length - a.length);
}

/** Defines `window.__cut.removed()`: the cut pieces on screen now (drawn elements, view tokens and values, buttons,
 *  round-bar phases, then words in visible text and hovers), each a line saying what and where. */
export async function installCut(page) {
  await page.js(`(() => {
    const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden'; };
    const CUT = new RegExp(${JSON.stringify(CUT_WORDS)});
    const NAMES = ${JSON.stringify(cutWordNames())};
    const CUT_DOM = '#board .k-base, #board .base-mark, #board .fronts, #board .front-link, #board .front-edge, #board .token[data-token^="unit"], '
      + '#board .token[data-token="no_muster"], [data-value="board.fronts"], [data-value*="host_cap"], [data-value*="host_more"], [data-value="alignment.bar"]';
    window.__cut = {
      removed() {
        const found = [];
        for (const e of document.querySelectorAll(CUT_DOM)) if (vis(e)) found.push('element ' + (e.getAttribute('class') || e.dataset.value || e.tagName) + (e.dataset.token ? ' ' + e.dataset.token : ''));
        for (const h of aic.view.hexes) {
          if (h.kind === 'base' || h.facing !== undefined) found.push('view hex ' + h.kind + ' ' + h.hex.join(','));
          for (const t of h.tokens) if (t.kind.split(':')[0] === 'unit' || t.kind === 'no_muster') found.push('view token ' + t.kind);
        }
        for (const g of aic.view.globals) if (/^(alignment\\.bar|board\\.troop_(steps|spent))$/.test(g.key)) found.push('view global ' + g.key);
        for (const a of aic.legal) if (a.type === 'standard' && /^(envoy|buy_energy)$/.test(String(a.act))) found.push('legal action ' + a.act);
        for (const p of aic.view.round_order) if (/^board\\.(war|bases|march)$/.test(p.name)) found.push('phase ' + p.name);
        const strip = (t) => NAMES.reduce((s, n) => s.split(n).join(' '), t);
        const read = (where, raw) => {
          for (const line of strip(raw ?? '').split('\\n')) {
            const m = CUT.exec(line);
            if (m) found.push(where + ' "' + m[0] + '": ' + line.trim().slice(0, 120));
          }
        };
        read('text', document.body.innerText);
        for (const e of document.querySelectorAll('[title]')) if (vis(e)) read('hover', e.getAttribute('title'));
        const tip = document.getElementById('tip');
        if (tip && !tip.hidden) read('tip', tip.innerText);
        return [...new Set(found)];
      },
    };
    return true;
  })()`);
}
