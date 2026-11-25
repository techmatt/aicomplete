// The small policy troops-play.mjs and ambassadors-play.mjs click a game to the win by, and what they read off
// the player view on the way. `POLICY` is evaluated in the page: it defines `window.__tp` (`decide` the next move,
// `click` the control the player view shows for it, `turn` a step of the adversary's turn, `fast` the whole game
// straight through the engine for `--search`, and the lookers each script's checks use).
//
// The policy (r142): draft from the first tier deck and keep nothing. Each round a seat uses every node power its
// board offers, a costly one before the free one, placing each node next to its corrupted city. Its one walking
// ambassador arrives from the supply on its node nearest that city (never one on the city itself, where it could
// act the round it arrives: the demos show the plain route), then moves a hex a round toward it (or hops to a
// nearer node of its own) and onto it once no agent is there; it liberates the city in the first later round the
// Liberate bill is in hand (the walker and the supply) and no troop guards it: the round after, with the bill ready. It
// buys ambassadors toward the bill; with the bill in hand, or when a Strike is shown ahead against its node, it
// removes the troop guarding the city; it clears agents off it with Remove agents.
export const POLICY = `(() => {
  const vis = (el) => { const r = el.getBoundingClientRect(); return r.width > 0 && r.height > 0 && getComputedStyle(el).visibility !== 'hidden'; };
  const isHex = (t) => Array.isArray(t) && t.length === 2;
  const count = (h, kind) => h.tokens.find((t) => t.kind === kind)?.count ?? 0;
  const val = (s, k) => aic.view.seats[s].values.find((x) => x.key === k)?.value ?? 0;
  const glob = (k) => aic.view.globals.find((x) => x.key === k)?.value ?? 0;
  const reach = (s, h) => [h.index, ...h.neighbors].some((i) => count(aic.view.hexes[i], 'node:' + s) > 0);
  const guardsOf = (h) => aic.view.arrows.filter((a) => a.kind === 'guard' && a.to === h.index).map((a) => a.from);
  const name = (h) => aic.view.countries[h.country].name + ' #' + h.number;
  /** A seat's ambassadors on a hex, and those of them still ready (the view's tipped:<seat> counts the used ones). */
  const mineOn = (s, h) => count(h, 'ambassador:' + s);
  const readyOn = (s, h) => Math.max(0, mineOn(s, h) - count(h, 'tipped:' + s));
  /** Steps from hex index \`from\` to every hex, over the map's neighbors; \`walk\`: over land holding no agent. */
  const steps = (from, walk = false) => {
    const d = new Map([[from, 0]]);
    const open = (i) => !walk || (aic.view.hexes[i].kind !== 'ocean' && count(aic.view.hexes[i], 'ambassador:adversary') === 0);
    for (const q = [from]; q.length;) { const i = q.shift(); for (const n of aic.view.hexes[i].neighbors) if (!d.has(n) && open(n)) { d.set(n, d.get(i) + 1); q.push(n); } }
    return d;
  };
  /** Each seat's corrupted city: the one its ambassador stands on, else the pairs taken nearest first, a seat and a
   *  city once each while both last. */
  const targets = () => {
    const v = aic.view;
    const cities = v.hexes.filter((h) => h.kind === 'city' && count(h, 'cube:adversary') > 0);
    const pairs = [];
    for (const c of cities) {
      const d = steps(c.index);
      for (const s of v.seats) {
        const nodes = v.hexes.filter((h) => count(h, 'node:' + s.index) > 0).map((h) => d.get(h.index) ?? 99);
        pairs.push({ seat: s.index, city: c, d: Math.min(99, ...nodes) });
      }
    }
    pairs.sort((a, b) => a.d - b.d || a.city.index - b.city.index || a.seat - b.seat);
    const out = new Map(), taken = new Set();
    // A seat whose ambassador stands on a corrupted city stays with that city.
    for (const s of v.seats) {
      const c = cities.find((h) => mineOn(s.index, h) > 0 && !taken.has(h.index));
      if (c) { out.set(s.index, c); taken.add(c.index); }
    }
    for (const p of pairs) if (!out.has(p.seat) && !taken.has(p.city.index)) { out.set(p.seat, p.city); taken.add(p.city.index); }
    for (const p of pairs) if (!out.has(p.seat)) out.set(p.seat, p.city);
    return out;
  };
  const hexIndex = (t) => aic.view.hexes.findIndex((h) => h.hex[0] === t[0] && h.hex[1] === t[1]);
  /** A board's node powers: its powers that place a node (their index is the id's last part). */
  const nodePowers = (s) => (aic.view.seats[s].board?.cards ?? []).filter((c) => c.id.includes('.power.') && /place 1 Node/i.test(c.text ?? ''))
    .map((c) => ({ k: Number(c.id.split('.power.')[1]), name: c.name, text: c.text ?? '', status: c.status, pay: /^Pay (\\d+) ([a-z_ ]+?)s?:/.exec(c.text ?? '') }));
  window.__tp = {
    /** What the last action taken was for: it picks the answer to the question that follows. */
    intent: null,
    /** What one seat wants to do now, or null (it passes). */
    want(s, mine) {
      const v = aic.view;
      const first = mine.find((a) => a.type === 'tier') ?? mine.find((a) => a.type === 'draft');
      if (first) return { action: first, why: 'draft' };
      if (!mine.some((a) => a.type === 'pass')) return { action: mine[0], why: 'the only kind of move' };
      if (!/board\\.act/.test(v.phase?.name ?? '')) return null;
      const std = (act) => mine.find((a) => a.type === 'standard' && a.act === act);
      const city = targets().get(s) ?? null;
      const supply = val(s, 'resource.ambassadors'), credits = val(s, 'resource.credits');
      const guards = city ? guardsOf(city) : [];
      const agents = city ? count(city, 'ambassador:adversary') : 0;
      // The Liberate bill for a city, from the action's own rule; the seat's ambassadors on the city pay first.
      const bill = Number(/for (\\d+) ambassadors \\(a city\\)/.exec(v.standard.find((x) => x.act === 'liberate')?.doc ?? '')?.[1] ?? 5);
      const have = supply + (city ? mineOn(s, city) : 0);
      const walkers = v.hexes.filter((h) => mineOn(s, h) > 0);
      // What the bill can count once the walker stands on the city.
      const total = supply + (walkers.length ? 1 : 0);
      if (city) {
        if (readyOn(s, city) > 0 && !guards.length && !agents && have >= bill && std('liberate')) return { action: std('liberate'), why: 'liberate', intent: { kind: 'site', city: city.index } };
        if (!guards.length && agents && reach(s, city) && total >= bill + 2 * Math.ceil(agents / 3) && std('remove')) return { action: std('remove'), why: 'clear the agents', intent: { kind: 'site', city: city.index } };
        const struck = v.arrows.some((a) => a.kind === 'troop' && a.rounds === 1 && guards.includes(a.from) && count(v.hexes[a.to], 'node:' + s) > 0);
        if (guards.length && std('troop') && (total >= bill || struck)) return { action: std('troop'), why: 'remove the guard', intent: { kind: 'troop', city: city.index } };
      }
      // Every node power the board offers this round: the one that gains an ambassador, then a costly one, then the free one.
      const offered = nodePowers(s).map((p) => ({ ...p, a: mine.find((a) => a.type === 'power' && Number(a.power) === p.k) })).filter((p) => p.a);
      const power = offered.find((p) => /gain 1 ambassador/.test(p.text)) ?? offered.find((p) => p.pay) ?? offered[0];
      if (power) return { action: power.a, why: 'node power', power: power.name, intent: { kind: 'node', city: city?.index ?? null } };
      if (city && std('move')) {
        const d = steps(city.index, true);
        const far = (h) => d.get(h.index) ?? 99;
        const own = v.hexes.filter((h) => count(h, 'node:' + s) > 0).sort((a, b) => far(a) - far(b));
        const nodes = own.some((h) => h.index !== city.index) ? own.filter((h) => h.index !== city.index) : own;
        if (!walkers.length && supply > 0 && nodes.length) return { action: std('move'), why: 'arrive', intent: { kind: 'move', to: nodes[0].index, from: 'supply' } };
        const w = walkers.filter((h) => readyOn(s, h) > 0 && h.index !== city.index).sort((a, b) => far(a) - far(b))[0];
        if (w && nodes.length && far(nodes[0]) < far(w) - 1) return { action: std('move'), why: 'move to a nearer node', intent: { kind: 'move', to: nodes[0].index, from: w.index } };
        const next = w && w.neighbors.map((i) => v.hexes[i]).filter((h) => far(h) < far(w)).sort((a, b) => far(a) - far(b))[0];
        if (next && next.index !== city.index) return { action: std('move'), why: 'move', intent: { kind: 'move', to: next.index, from: w.index } };
        if (next && !agents) return { action: std('move'), why: 'move onto the city', intent: { kind: 'move', to: next.index, from: w.index, city: city.index } };
      }
      if (city && total < bill && std('ambassador') && credits >= 16 + (guards.length ? 7 : 0)) return { action: std('ambassador'), why: 'gain an ambassador' };
      return null;
    },
    /** The next move: { action, why }, or null when no action is legal. */
    decide() {
      const v = aic.view, legal = aic.legal;
      if (!legal.length) return null;
      if (v.choice) {
        const answers = legal.filter((a) => a.type === 'choose');
        const hexes = answers.filter((a) => isHex(a.target));
        const it = this.intent;
        let best = hexes[0] ?? answers[0] ?? legal[0];
        if (hexes.length && it?.kind === 'node' && it.city !== null) {
          // Next to the city before on it: the walker starts from a node beside it.
          const d = steps(it.city);
          const far = (a) => { const n = d.get(hexIndex(a.target)) ?? 99; return n === 0 ? 1.5 : n; };
          best = hexes.reduce((b, a) => (far(a) < far(b) ? a : b));
        } else if (hexes.length && it?.kind === 'troop') {
          const from = (a) => v.arrows.filter((x) => x.kind === 'guard' && x.from === hexIndex(a.target));
          best = hexes.find((a) => from(a).some((x) => x.to === it.city)) ?? hexes.find((a) => from(a).length) ?? best;
        } else if (hexes.length && it?.kind === 'site') {
          best = hexes.find((a) => hexIndex(a.target) === it.city) ?? best;
        } else if (it?.kind === 'move') {
          // Move asks where to, then (with more than one that could come) which ambassador: a hex, or the supply.
          if (!/which ambassador/.test(v.choice.prompt)) best = hexes.find((a) => hexIndex(a.target) === it.to) ?? best;
          else best = (it.from === 'supply' ? answers.find((a) => !isHex(a.target)) : hexes.find((a) => hexIndex(a.target) === it.from)) ?? best;
        }
        return { action: best, why: 'answer' + (it ? ' (' + it.kind + ')' : '') };
      }
      this.intent = null;
      for (const s of [...new Set(legal.map((a) => a.seat))].sort()) {
        const w = this.want(s, legal.filter((a) => a.seat === s));
        if (w) { this.intent = w.intent ?? null; return w; }
      }
      return { action: legal.find((a) => a.type === 'pass') ?? legal[0], why: 'pass' };
    },
    /** Makes \`a\` by the player view's own control for it; returns what it clicked, or null when there is none. */
    click(a) {
      const key = JSON.stringify(a);
      const button = () => [...document.querySelectorAll('#player button[data-action]')].filter(vis).find((b) => b.dataset.action === key);
      const hex = () => isHex(a.target) && [...document.querySelectorAll('#player .hex-pick[data-hexes]')].filter(vis).some((e) => e.dataset.hexes.split(' ').includes(a.target.join(',')));
      let switched = '';
      if (!button() && !hex() && aic.focus !== a.seat) {
        const row = [...document.querySelectorAll('#player [data-seat-focus="' + a.seat + '"]')].find(vis);
        if (row) { row.click(); switched = 'seat ' + a.seat + ', '; }
      }
      const b = button();
      if (b) { const t = b.textContent; b.click(); return switched + t; }
      if (hex()) {
        document.querySelector('#board .hex[data-hex="' + a.target.join(',') + '"]').dispatchEvent(new MouseEvent('click', { bubbles: true }));
        return switched + 'hex ' + a.target.join(',');
      }
      return null;
    },
    /** One click of the open "Since your last move" panel (the adversary's turn), or null when it is closed. */
    turn() {
      const next = document.querySelector('#p-turn [data-turn="1"]:not([disabled])');
      if (next) { next.click(); return 'turn step'; }
      const done = document.querySelector('#p-turn [data-turn="close"]');
      if (done) { done.click(); return 'turn done'; }
      return null;
    },
    /** What the engine's view holds of the garrison rules now. */
    state() {
      const v = aic.view;
      const guard = v.arrows.filter((a) => a.kind === 'guard');
      const refused = v.seats.map((s) => val(s.index, 'board.liberate_refused'));
      return {
        over: !!v.outcome, round: v.round, phase: v.phase?.name ?? null, choice: !!v.choice, acting: /board\\.act/.test(v.phase?.name ?? '') && !v.choice,
        troops: v.hexes.reduce((n, h) => n + count(h, 'troop'), 0), guards: guard.length, strikeArrows: v.arrows.filter((a) => a.kind === 'troop').length,
        strikeAhead: glob('board.strike_ahead'), redeployAhead: glob('board.redeploy_ahead'), corrupted: glob('alignment.corrupted_cities'), refused,
      };
    },
    /** (a) on screen: a troop glyph on a hex whose guard link is drawn with its shield. */
    garrison() {
      for (const a of aic.view.arrows.filter((x) => x.kind === 'guard')) {
        const from = aic.view.hexes[a.from], to = aic.view.hexes[a.to];
        const troop = [...document.querySelectorAll('#board .token[data-token="troop"][data-hex="' + from.hex.join(',') + '"]')].find(vis);
        const link = document.querySelector('#board .arrows .arrow.guard[data-from="' + a.from + '"][data-to="' + a.to + '"]');
        if (troop && link?.querySelector('path.shield') && vis(link)) return { troop: name(from), guards: name(to), doc: a.doc };
      }
      return null;
    },
    /** (b) on screen: the Strike-ahead notice in the adversary's next-round box. */
    strikeNotice() {
      const e = [...document.querySelectorAll('#player .slot-box.next .ahead.strike')].find(vis);
      const next = aic.view.piles.find((p) => p.key === 'adversary.next')?.visible[0];
      return e ? { text: e.textContent, hover: e.title, next: next?.name ?? null, solidArrows: document.querySelectorAll('#board .arrows .arrow.troop.soon').length } : null;
    },
    /** (c) on screen for the shown seat: the Liberate-refused note, and a corrupted site in its reach wearing the
     *  guard's shield; the hex to hover for the reason. */
    refusedNote() {
      const s = aic.focus;
      const e = [...document.querySelectorAll('#p-act .prefused')].find(vis);
      if (!e || !val(s, 'board.liberate_refused')) return null;
      const a = aic.view.arrows.find((x) => x.kind === 'guard' && reach(s, aic.view.hexes[x.to]) && count(aic.view.hexes[x.to], 'ambassador:adversary') === 0);
      const mark = a && document.querySelector('#board .arrows .arrow.guard[data-to="' + a.to + '"] path.shield');
      if (!a || !mark) return null;
      const to = aic.view.hexes[a.to];
      const r = document.querySelector('#board .hex[data-hex="' + to.hex.join(',') + '"] .cell').getBoundingClientRect();
      return { seat: s, text: e.textContent, hover: e.title, site: name(to), x: r.left + r.width / 2, y: r.top + r.height * 0.75 };
    },
    /** 295: a seat's side of the table now: what it holds, its nodes, the stations on the map, and its node powers
     *  as the engine has them and as its panel shows them (the Use button and the status beside each name). */
    seat(s) {
      const v = aic.view;
      const res = Object.fromEntries(v.seats[s].values.filter((x) => x.key.startsWith('resource.')).map((x) => [x.key.slice(9), x.value]));
      const shown = aic.focus === s ? [...document.querySelectorAll('#player .ppowers li')].filter(vis).map((li) => ({ name: li.querySelector('b')?.textContent ?? '', status: li.querySelector('small')?.textContent ?? '', used: li.classList.contains('used'), use: !!li.querySelector('button[data-action]') })) : null;
      return { round: v.round, res, nodes: val(s, 'board.node'), stations: v.hexes.reduce((n, h) => n + count(h, 'station'), 0), powers: nodePowers(s), shown };
    },
    /** 295: a seat's ambassadors on hex index \`i\`: the engine's counts, and the standing and tipped stacks the map draws. */
    ambassadors(s, i) {
      const h = aic.view.hexes[i];
      const drawn = (kind) => [...document.querySelectorAll('#board .token[data-token="' + kind + ':' + s + '"][data-hex="' + h.hex.join(',') + '"]')].find(vis);
      const tipped = drawn('tipped');
      const r = tipped?.getBoundingClientRect();
      return { hex: name(h), corrupted: count(h, 'cube:adversary') > 0, kind: h.kind, on: mineOn(s, h), tipped: count(h, 'tipped:' + s), standingDrawn: !!drawn('ambassador'),
        tippedDrawn: !!tipped?.querySelector('.tipped'), fallback: !!tipped?.querySelector('[data-glyph="fallback"]'), x: r ? r.left + r.width / 2 : 0, y: r ? r.top + r.height / 2 : 0 };
    },
    /** \`--search\`: the whole game by the policy, straight through the engine. */
    fast(max) {
      const seen = {};
      const mark = (k, ok) => { if (ok && seen[k] === undefined) seen[k] = aic.view.round; };
      // 295: a seat's move onto its city (the round), its liberation the round after, and two node powers in a round.
      const movedOn = new Map(), used = new Map();
      let i = 0;
      for (; i < max && !aic.view.outcome; i++) {
        const st = this.state();
        if (st.acting) { mark('a', st.guards > 0); mark('b', st.strikeAhead === 1); mark('c', st.refused.some((n) => n > 0)); mark('strikeArrow', st.strikeArrows > 0); mark('redeploy', st.redeployAhead === 1); }
        const d = this.decide();
        if (!d) break;
        const it = this.intent;
        if (d.why === 'move onto the city') movedOn.set(d.action.seat + ':' + it.city, st.round);
        if (d.why === 'liberate') mark('moveThenLiberate', movedOn.get(d.action.seat + ':' + it.city) === st.round - 1);
        if (d.why === 'node power') {
          const k = d.action.seat + ':' + st.round;
          used.set(k, [...(used.get(k) ?? []), /^Pay /.test(nodePowers(d.action.seat).find((p) => p.name === d.power)?.text ?? '')]);
          mark('twoNodePowers', used.get(k).length >= 2 && used.get(k).some(Boolean));
        }
        aic.act(d.action);
      }
      const o = aic.view.outcome;
      return { seen, actions: i, end: o ? (o.won ? 'WON' : 'LOST') + ' in round ' + o.round + ': ' + o.detail : 'no outcome', won: !!o?.won };
    },
  };
  return true;
})()`;
