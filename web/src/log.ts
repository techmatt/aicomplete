// One entry of the log tab: a step's action, then its events, folded by phase when the
// step runs through several. `renderLog` in main.ts keeps the list.

import type { GameEvent } from './engine';
import { actionLabel, escapeHtml, eventText, hexKey, hexesIn, shortPhase } from './format';
import type { Frame } from './session';

const evItem = (e: GameEvent) => {
  const hs = hexesIn(e).map(hexKey).join(' ');
  return `<li class="ev ${escapeHtml(e.type)}"${hs ? ` data-hexes="${hs}"` : ''}>${escapeHtml(eventText(e))}</li>`;
};

/**
 * A step's events by phase (022, D7): the engine marks each phase it enters with a
 * `phase_started` event and each new round with `round_started`; the events before the
 * first mark belong to the phase the action was taken in (`before`, the step's phase).
 */
function phaseGroups(events: GameEvent[], before: string): { label: string; round: boolean; events: GameEvent[] }[] {
  const groups = [{ label: before, round: false, events: [] as GameEvent[] }];
  for (const e of events) {
    if (e.type === 'phase_started') groups.push({ label: shortPhase(String(e.phase)), round: false, events: [] });
    else if (e.type === 'round_started') groups.push({ label: `Round ${e.round}`, round: true, events: [] });
    else groups[groups.length - 1].events.push(e);
  }
  if (!groups[0].events.length) groups.shift();
  return groups;
}

export function logEntry(f: Frame, i: number, prev: Frame | undefined, countries?: string[]): string {
  const head = f.action ? `#${i} ${escapeHtml(actionLabel(f.action, true, true, { config: f.snap.state.config, countries }))}` : '#0 setup';
  const hexes = [...(f.action ? hexesIn(f.action) : []), ...f.events.flatMap(hexesIn)].map(hexKey);
  const ps = prev?.snap.state;
  const before = ps ? shortPhase(ps.config.round[ps.phase] ?? 'setup') : 'setup';
  const groups = phaseGroups(f.events, before);
  let body: string;
  // A step that stays in one phase lists its events as they are; one that runs through
  // phases (the adversary's turn after the last pass) gets a folded group per phase,
  // whose heading outlines the hexes that phase changed when hovered.
  if (groups.length <= 1) body = f.events.length ? `<ul>${f.events.map(evItem).join('')}</ul>` : '';
  else {
    body = groups.map((g) => {
      if (!g.events.length) return `<div class="pgroup empty${g.round ? ' round' : ''}">${escapeHtml(g.label)}</div>`;
      const hs = [...new Set(g.events.flatMap(hexesIn).map(hexKey))].join(' ');
      const n = g.events.length;
      return `<details class="pgroup${g.round ? ' round' : ''}"${hs ? ` data-hexes="${hs}"` : ''}><summary>${escapeHtml(g.label)} <small>${n} event${n === 1 ? '' : 's'}${hs ? ` · ${hs.split(' ').length} hex${hs.split(' ').length === 1 ? '' : 'es'}` : ''}</small></summary><ul>${g.events.map(evItem).join('')}</ul></details>`;
    }).join('');
  }
  return `<div class="entry" data-step="${i}" data-hexes="${[...new Set(hexes)].join(' ')}"><div class="head">${head}</div>${body}</div>`;
}
