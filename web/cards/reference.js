// 267: the adversary reference card's face (Spirit Island's adversary card), at print size (100 x 140 mm):
// the board's rules, its research rule, its escalation, and the difficulty ladder, a row a level with its setup
// and its one named mid-game rule. Every word is the export's (`reference`, from web/card/adversary.json).
// Needs viewer.css.
import { esc } from '../card/card.js';

const NUMERALS = ['0', 'I', 'II', 'III', 'IV', 'V'];

export function referenceFace(c) {
  const a = c.reference;
  const rules = a.rules.map(([name, text]) => `<p><b>${esc(name)}.</b> ${esc(text)}</p>`).join('');
  const research = a.research ? `<p><b>${esc(a.research[0])}</b> <i>(research)</i>. ${esc(a.research[1])}</p>` : '';
  const escalation = `<p><b>Escalation</b> <i>(the event deck)</i>. ${a.escalation.map(esc).join('. ')}.</p>`;
  const base = a.levels[0];
  const rows = a.levels
    .slice(1)
    .map(
      (l) => `<tr>
        <td class="lv">${NUMERALS[l.level] ?? l.level}</td>
        <td><div class="nm">${esc(l.name)}</div><div class="fl">${esc(l.flavor)}</div></td>
        <td>${l.setup.map(esc).join('<br>') || '—'}</td>
        <td>${l.rule ? esc(l.rule) : '<span class="none">Setup only</span>'}</td>
        <td class="ac">${l.active_rules}</td>
      </tr>`,
    )
    .join('');
  return `<div class="reference tone-adversary">
    <div class="strip"><span>Adversary reference</span><span>Levels 0-${a.levels.length - 1}, cumulative</span></div>
    <h3>${esc(a.name)}</h3>
    <div class="body">
      ${rules}${research}${escalation}
      <p><b>Setup (level 0).</b> ${base.setup.map(esc).join('; ')} (incomes and threshold a country; deck: stages I / II / III).</p>
      <table>
        <colgroup><col class="c-lv"><col class="c-nm"><col class="c-su"><col><col class="c-ac"></colgroup>
        <thead><tr><th>Lv</th><th>Name</th><th>Setup (a country)</th><th>Mid-game rule it adds</th><th>Rules</th></tr></thead>
        <tbody>${rows}</tbody>
      </table>
      <p class="foot">Each level keeps every level below it. A mid-game rule fires on an event you already resolve.</p>
    </div>
  </div>`;
}
