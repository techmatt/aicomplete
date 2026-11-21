// 312: the in-page screen checks sweep.mjs and playthrough.mjs share (they had drifted: playthrough had no TODO/FIXME,
// raw-id or stale-word check). SCREEN_LIB is page code: evaluated in the viewer (on its own, or spliced into a script's
// own page code), it installs `window.__aicScreen`, whose `screen(sel)` returns `{ kind, what }` issues for the
// elements `sel` matches. HARD names the kinds that fail a run; the rest are reported for a look.

/** Kinds that fail a run (sweep.mjs adds its own: console, stuck, hidden-target, token-unknown, ...). */
export const SCREEN_HARD = ['page-scroll', 'error-banner', 'placeholder', 'offscreen', 'clipped', 'stale-word', 'below-fold'];

/** Page code (no backticks in it: it sits inside a template string). */
export const SCREEN_LIB = String.raw`(() => {
  const aic = window.aic;
  const vis = (el) => { const s = getComputedStyle(el); const r = el.getBoundingClientRect();
    return s.visibility !== 'hidden' && s.display !== 'none' && r.width > 0 && r.height > 0; };
  const label = (el) => { const id = el.id ? '#' + el.id : ''; const c = typeof el.className === 'string' && el.className ? '.' + el.className.trim().split(/\s+/).join('.') : '';
    return el.tagName.toLowerCase() + id + c + ' "' + (el.textContent || '').trim().replace(/\s+/g, ' ').slice(0, 50) + '"'; };
  const scrollParent = (el) => { for (let p = el.parentElement; p; p = p.parentElement) { const o = getComputedStyle(p); if (/(auto|scroll)/.test(o.overflow + o.overflowX + o.overflowY)) return p; } return null; };
  const inside = (r, b) => r.left >= b.left - 1 && r.right <= b.right + 1 && r.top >= b.top - 1 && r.bottom <= b.bottom + 1;
  /** Words on screen that are code, not text: unfilled values and raw ids. */
  const PLACEHOLDER = /\b(undefined|NaN|null|\[object Object\]|TODO|FIXME)\b|\?\?/;
  const RAW_ID = /\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+\b/;
  // The words crates/aic-sim/src/docs/spelling.rs refuses in source (STALE there), plus the engine's labels it can't read.
  const STALE = /adversary(?:'s)? ambassadors?|enemy ambassadors?|adv\. ambassadors?|agents token|Matt to confirm|Security agents?\b|on board: \d+ ambassadors?|corp troops?|\bDeploy(s|ed)?\b|each board|\btier (I|II|III)\b|Movement \+\d|\bDispatch(es|ed)?\b(?! Control Room)|\bGrowth (option|\()|pick one a round/i;

  /** The screen checks over the elements sel matches: page scroll, the error banner, an element off screen with
   *  nothing to scroll it, clipped or truncated text, placeholder, raw-id and stale words, and (player view) a Draft /
   *  Keep button or the turn panel below its fold. */
  function screen(sel) {
    const issues = [];
    const add = (kind, what) => issues.push({ kind, what });
    const de = document.documentElement;
    if (de.scrollHeight > innerHeight || de.scrollWidth > innerWidth || document.body.scrollHeight > innerHeight || document.body.scrollWidth > innerWidth)
      add('page-scroll', 'page scrolls: ' + de.scrollWidth + 'x' + de.scrollHeight + ' in ' + innerWidth + 'x' + innerHeight);
    if (aic.error) add('error-banner', aic.error.slice(0, 200));
    const view = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    for (const el of document.querySelectorAll(sel)) {
      if (!(el instanceof HTMLElement) || !vis(el)) continue;
      const r = el.getBoundingClientRect();
      const sp = scrollParent(el);
      // Off the screen with nothing to scroll it into view.
      if (!sp && !inside(r, view) && el.closest('#data') === null) add('offscreen', label(el) + ' at ' + Math.round(r.left) + ',' + Math.round(r.top) + '-' + Math.round(r.right) + ',' + Math.round(r.bottom));
      const s = getComputedStyle(el);
      const own = [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim());
      if (/(hidden|clip)/.test(s.overflow + s.overflowX + s.overflowY) && (el.scrollWidth > el.clientWidth + 2 || el.scrollHeight > el.clientHeight + 2)) {
        const ell = s.textOverflow === 'ellipsis' || s.webkitLineClamp !== 'none';
        const hover = !!el.closest('[title],[data-card],[data-value],[data-pile],[data-track],[data-board-seat],[data-action]');
        add(ell ? (hover ? 'truncated' : 'truncated-no-hover') : 'clipped', label(el) + ' ' + el.scrollWidth + 'x' + el.scrollHeight + ' in ' + el.clientWidth + 'x' + el.clientHeight);
      }
      if (own) {
        const t = [...el.childNodes].filter((n) => n.nodeType === 3).map((n) => n.textContent).join(' ');
        if (PLACEHOLDER.test(t)) add('placeholder', label(el));
        // #source names the config file and the config's own name (baseline): not a raw id.
        if (!el.closest('code, pre, #data, #source') && RAW_ID.test(t)) add('raw-id', label(el) + ' (' + t.match(RAW_ID)[0] + ')');
        // The engine's hex names (C1 #13) are shown with the country's name (020).
        if (!el.closest('code, pre, #data') && /\bC\d+ #\d+/.test(t)) add('raw-id', label(el) + ' (engine hex name)');
        // 169: the house words (r70, amendment 7657): the adversary's pieces are agents, the
        // Security token Operatives; an old name or an open naming note is a stale word.
        if (!el.closest('code, pre, #data') && STALE.test(t)) add('stale-word', label(el) + ' (' + t.match(STALE)[0] + ')');
      }
    }
    // 289 (D127, D133): the player view's draft and keep buttons and the open turn panel show without scrolling: each
    // inside its scroll pane's visible area and the window, as drawn (a player could pass not seeing Keep).
    if (aic.mode === 'player') {
      const fold = [...document.querySelectorAll('#p-seat .pdcard .pact, #p-turn')].filter((el) => !el.hidden && vis(el));
      for (const el of fold) {
        const r = el.getBoundingClientRect();
        const sp = scrollParent(el);
        const pr = sp && sp.getBoundingClientRect();
        const box = sp ? { left: pr.left + sp.clientLeft, top: pr.top + sp.clientTop, right: Math.min(innerWidth, pr.left + sp.clientLeft + sp.clientWidth), bottom: Math.min(innerHeight, pr.top + sp.clientTop + sp.clientHeight) } : view;
        if (!inside(r, box)) add('below-fold', label(el) + ' ends at ' + Math.round(r.bottom) + ', its pane shows to ' + Math.round(box.bottom));
      }
    }
    return issues;
  }
  window.__aicScreen = { vis, label, scrollParent, inside, screen };
  return true;
})()`;
