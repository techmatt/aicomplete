// 327: faces.html, every printed face beyond player cards on one sheet (web/card/print.js), from the card viewer's
// export. `?only=board,research,event,project,threat` narrows it; `window.faceRows` holds fitCards' rows once done.
import { fitCards, inset } from '../card/card.js';
import { printFace, renderResearch } from '../card/print.js';
import { esc } from '../card/card.js';

const res = await fetch('/api/cards');
const data = await res.json();
const layout = inset(data.layout);
const only = new URLSearchParams(location.search).get('only')?.split(',');
const want = (k) => !only || only.includes(k);
const frame = (f) => `/art/frames/r3/${f}.jpg`;
const face = (c) => {
  const html = printFace(c, layout, frame, data.fits, c.scene ? `/scene/${c.scene}.png` : undefined);
  return c.kind === 'ai' ? `<div data-id="${esc(c.id)}">${html}</div>` : `<div class="trim" data-id="${esc(c.id)}">${html}</div>`;
};
const group = (title, kind) => {
  const cs = data.cards.filter((c) => c.kind === kind && c.print);
  return cs.length ? `<h2>${title} (${cs.length})</h2><div class="row">${cs.map(face).join('')}</div>` : '';
};
document.getElementById('sheet').innerHTML = [
  want('board') ? group('AI boards', 'ai') : '',
  want('research') ? `<h2>Research strip</h2><div class="row">${renderResearch(data.research, 0)}${renderResearch(data.research, 3)}</div>` : '',
  want('event') ? group('Adversary event cards', 'adversary') : '',
  want('project') ? group('Adversary projects', 'project') : '',
  want('threat') ? group('Threats', 'threat') : '',
].join('');
window.faceRows = await fitCards(document);
document.getElementById('status').textContent = `${window.faceRows.length} cards fitted`;
