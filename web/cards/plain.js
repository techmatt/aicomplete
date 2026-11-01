// The plain face: a card kind with no frame yet. Name, type and text, all from the export.
// Needs viewer.css.

export const esc = (s) => String(s ?? '').replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);

/**
 * @param c         a card from the export
 * @param kindLabel the kind as the strip shows it
 * @param sceneUrl  optional scene over the text
 */
export function plainFace(c, kindLabel, sceneUrl) {
  const paras = [c.text, ...c.sections.map((s) => [s.head, s.text])]
    .filter((p) => p && (Array.isArray(p) ? p[1] : true))
    .map((p) => (Array.isArray(p) ? `<p><b>${esc(p[0])}.</b> ${esc(p[1])}</p>` : `<p>${esc(p)}</p>`));
  return `<div class="plain tone-${esc(c.kind)}">
    <div class="strip"><span>${esc(kindLabel)}</span><span>${esc(c.tierLabel)}</span></div>
    <h3>${esc(c.name)}</h3>
    ${sceneUrl ? `<img loading="lazy" decoding="async" alt="" src="${esc(sceneUrl)}">` : ''}
    <div class="body">${paras.join('')}</div>
    ${c.costLabel ? `<div class="foot">${esc(c.costLabel)}</div>` : ''}
  </div>`;
}
