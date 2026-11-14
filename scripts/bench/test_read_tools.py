"""Checks of the read tools on a pair of identical reads (163), one all native and one half wasm, as a farmed read
of the same seeds gives:

    python scripts/bench/test_read_tools.py

- `ladder.py`: both win % columns equal at every difficulty, the gap +0.0, no seed left out (across engines, the
  default), and the win % columns still equal under `--same-engine`, which leaves the mixed seeds out of the gap only.
- `paired.py`: the gap is +0.0 on every seed by default, and on the same-engine seeds only under `--same-engine`.
- `ladder.py --screen` over a native base and a farmed lever in another folder (FILE=SPEC): one agent, a real gap.

No build and no binary: the reads are written here. Python's standard library only.
"""
import contextlib
import io
import json
import os
import re
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, '..'))
import ladder  # noqa: E402
import paired  # noqa: E402
import screens  # noqa: E402

SPEC = {
    'name': 'read', 'configs': ['configs/baseline.json'],
    'variants': [{'name': 'bp', 'set': []}, {'name': 'br', 'set': ['ai.boards=0,2']}],
    'difficulties': [0, 2], 'agents': ['strategist'],
}
SEEDS = range(1000, 1040)


def outcome(cell, seed):
    """A fixed win pattern whose odd seeds (the wasm ones below) win at another rate than its even ones, so a column
    over the same-engine seeds alone reads apart from the whole (161's d0 95.1 against 94.7)."""
    return (seed + cell) % 4 != 0


def write_read(d, wasm_seeds, spec=SPEC, won=None):
    """DIR/games.jsonl: a farm header, then one line a cell a seed; seeds in WASM_SEEDS played on wasm."""
    os.makedirs(d)
    won = won or outcome
    cells = len(spec['configs']) * len(spec['variants']) * len(spec['difficulties']) * len(spec['agents'])
    with open(os.path.join(d, 'games.jsonl'), 'w', encoding='utf-8', newline='\n') as f:
        f.write(json.dumps({'format': 1, 'spec': spec, 'rules_revision': 69}) + '\n')
        for s in SEEDS:
            for c in range(cells):
                f.write(json.dumps({'cell': c, 'seed': s, 'won': won(c, s), 'rounds': 8, 'reason': 'end',
                                    'counters': {}, 'engine': 'wasm' if s in wasm_seeds else 'native'}) + '\n')


def run(fn, *args):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        fn(*args)
    return out.getvalue()


def ladder_rows(text):
    """{difficulty: (new win %, old win %, gap, games)} from ladder's table."""
    rows = {}
    for line in text.splitlines():
        m = re.match(r'\| d(\d) \| \*\*([\d.]+)\*\* \| [\d.]+ \| ([\d.]+) \| ([+-][\d.]+) \|.*\((\d+) games\)', line)
        if m:
            rows[int(m[1])] = (float(m[2]), float(m[3]), float(m[4]), int(m[5]))
    return rows


class IdenticalReads(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.a = os.path.join(self.tmp.name, 'A')
        self.b = os.path.join(self.tmp.name, 'B')
        write_read(self.a, set())
        write_read(self.b, {s for s in SEEDS if s % 2})  # half the seeds on wasm, as a farm mixes them

    def tearDown(self):
        self.tmp.cleanup()
        ladder.SAME_ENGINE = paired.SAME_ENGINE = False

    def ladder(self, same_engine):
        ladder.SAME_ENGINE = same_engine
        return run(ladder.ladder, [self.b], [self.a], ladder.parse_bands(ladder.BANDS), False)

    def test_ladder_across_engines(self):
        text = self.ladder(False)
        rows = ladder_rows(text)
        self.assertEqual(sorted(rows), [0, 2])
        for d, (new, old, g, n) in rows.items():
            self.assertEqual(new, old, f'd{d}: the win % columns differ on identical reads')
            self.assertEqual(g, 0.0)
            self.assertEqual(n, 2 * len(SEEDS))
        self.assertIn('paired', text)
        self.assertNotIn('left out', text)

    def test_ladder_same_engine(self):
        text = self.ladder(True)
        for d, (new, old, g, _) in ladder_rows(text).items():
            self.assertEqual(new, old, f'd{d}: --same-engine moved the old win % column')
            self.assertEqual(g, 0.0)
        self.assertIn('left out', text)

    def pair(self, *flags):
        argv = sys.argv
        sys.argv = ['paired.py', self.a, self.b, '--pair', 'A/d2/bp/strategist', 'B/d2/bp/strategist', *flags]
        try:
            return run(paired.main)
        finally:
            sys.argv = argv

    def test_paired(self):
        n = len(SEEDS)
        self.assertIn(f'minus A/d2/bp/strategist: +0.0 ±0.0 on {n} seeds ({n // 2} seeds on mixed engines paired)',
                      self.pair())
        self.assertIn(f'on {n // 2} seeds ({n // 2} seeds on mixed engines left out)', self.pair('--same-engine'))


class TwoAgentsTwoConfigs(unittest.TestCase):
    """169: a read of two agents gets a row each (they were pooled into one win %), and a screen's second config is a
    lever of its own (its games overwrote the base config's)."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()

    def tearDown(self):
        self.tmp.cleanup()

    def test_ladder_rows_per_agent(self):
        spec = dict(SPEC, agents=['strategist', 'greedy'])
        d = os.path.join(self.tmp.name, 'A')
        write_read(d, set(), spec, lambda c, s: c % 2 == 1)  # cells run agents fastest: greedy wins, strategist not
        text = run(ladder.ladder, [d], [d], ladder.parse_bands(ladder.BANDS), False)
        self.assertIn('| strategist | d0 | **0.0**', text)
        self.assertIn('| greedy | d0 | **100.0**', text)

    def test_screen_config_lever(self):
        spec = dict(SPEC, configs=['configs/baseline.json', 'out/x_off.json'])
        d = os.path.join(self.tmp.name, 'S')
        per_config = len(spec['variants']) * len(spec['difficulties']) * len(spec['agents'])
        write_read(d, set(), spec, lambda c, s: c < per_config)  # the base config wins, x_off never
        text = run(ladder.screen, [d], False)
        self.assertRegex(text, r'\| base \| strategist \| d0 \| 100\.0 ')
        self.assertRegex(text, r'\| x_off \| strategist \| d0 \| 0\.0 \| [\d.]+ \| -100\.0 ')
        # 229: --measures adds rounds, paired on the same pair and seed, split by win and loss
        text = run(ladder.screen, [d], True)
        self.assertRegex(text, rf'\| x_off \| strategist \| d0 \| 8\.00 \| \+0\.00 ± 0\.00 \| nan \| 8\.00 \| {2 * len(SEEDS)} \|')

    def test_pairs_pool_seat_orders(self):
        # 229: --pairs pools bp and pb (lever variants `bp-A` split on `-` too); identical sides gap +0.0
        spec = dict(SPEC, variants=[{'name': 'bp', 'set': []}, {'name': 'pb', 'set': []}, {'name': 'br', 'set': []}])
        d = os.path.join(self.tmp.name, 'P')
        write_read(d, set(), spec, lambda c, s: c < 2)  # bp wins at d0 and d2, pb and br never
        text = run(ladder.ladder, [d], [d], ladder.parse_bands(ladder.BANDS), False, ladder.parse_pools('0-2'))
        self.assertIn('| strategist | r69 new | 50.0 (+25.0) | 0.0 (-25.0) | 25.0 | +25.0 | 25.0 | 50.0 | 0.0 | 33.3 | +0.0 ± 0.0 |', text)
        self.assertIn('| strategist | r69 new | 100.0 | 0.0 | 0.0 |', text)
        self.assertEqual(ladder.split_variant('bp-A'), ('bp', 'A'))


def write_bare(path, spec, won, seeds=SEEDS):
    """A farmed JSONL as `farm-seeds ... OUT` writes it: no header, one line a cell a seed."""
    cells = len(spec['configs']) * len(spec['variants']) * len(spec['difficulties']) * len(spec['agents'])
    with open(path, 'w', encoding='utf-8', newline='\n') as f:
        for s in seeds:
            for c in range(cells):
                f.write(json.dumps({'cell': c, 'seed': s, 'won': won(c, s), 'rounds': 8, 'reason': 'end',
                                    'counters': {}, 'engine': 'native'}) + '\n')


def write_results(d, spec, label):
    """DIR/results.json as a native experiment writes it: each cell's agent by its resolved spec, LABEL(written)."""
    cells = [dict(c, cell=i, agent=label(c['agent'])) for i, c in enumerate(screens.cells_of(spec))]
    with open(os.path.join(d, 'results.json'), 'w', encoding='utf-8', newline='\n') as f:
        json.dump({'cells': cells}, f)


class MixedScreens(unittest.TestCase):
    """198: a screen half native and half farmed pools one agent (a native run's results.json names it by its
    resolved spec), a screen over two folders pairs base and lever across them, and a bare farmed JSONL reads as
    FILE=SPEC."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.spec = dict(SPEC, variants=[{'name': 'bp', 'set': []}], difficulties=[2])
        self.lever = dict(self.spec, variants=[{'name': 'bp_x', 'set': []}])

    def tearDown(self):
        self.tmp.cleanup()

    def path(self, name):
        return os.path.join(self.tmp.name, name)

    def test_named_agents(self):
        resolved = {'strategist': 'engine:strategy=1,w.built=2'}
        cells = {i: dict(c, agent=resolved[c['agent']]) for i, c in enumerate(screens.cells_of(self.spec))}
        named = screens.named_agents(cells, self.spec)
        self.assertEqual([c['agent'] for c in named.values()], ['strategist'])
        self.assertEqual(named[0]['label'], 'engine:strategy=1,w.built=2')
        # a spec whose cells don't line up leaves the cells as they are
        self.assertIs(screens.named_agents(cells, SPEC), cells)

    def test_file_spec_stream(self):
        f, s = self.path('farm.jsonl'), self.path('spec.json')
        write_bare(f, self.spec, lambda c, s: True)
        with open(s, 'w', encoding='utf-8', newline='\n') as out:
            json.dump(self.spec, out)
        lines = list(screens.stream(f'{f}={s}'))
        self.assertEqual(lines[0]['spec'], self.spec)
        self.assertEqual(len(lines), 1 + len(SEEDS))

    def test_screen_native_base_farmed_lever(self):
        # base: a native run (results.json names its agent by the resolved spec), its games all won
        native = self.path('native')
        write_read(native, set(), self.spec, lambda c, s: True)
        write_results(native, self.spec, lambda a: 'engine:strategy=1,w.built=2')
        # lever: a bare farmed JSONL in another folder, read as FILE=SPEC, its games all lost
        farm, spec = self.path('farm.jsonl'), self.path('spec.json')
        write_bare(farm, self.lever, lambda c, s: False)
        with open(spec, 'w', encoding='utf-8', newline='\n') as out:
            json.dump(self.lever, out)
        text = run(ladder.screen, [native, f'{farm}={spec}'], False)
        self.assertNotIn('engine:', text)
        self.assertRegex(text, r'\| base \| strategist \| d2 \| 100\.0 ')
        self.assertRegex(text, rf'\| x \| strategist \| d2 \| 0\.0 \| [\d.]+ \| -100\.0 \| 0\.0 \| {len(SEEDS)} \|')

    def test_across_prefers_own_folder(self):
        base = {(0, 'bp', 1): 'a0', (1, 'bp', 1): 'a1', (0, 'bp', 2): 'b0'}
        self.assertEqual(ladder.across({(1, 'bp', 1): 'x', (1, 'bp', 2): 'y', (1, 'bp', 3): 'z'}, base),
                         {(1, 'bp', 1): 'a1', (1, 'bp', 2): 'b0'})


if __name__ == '__main__':
    unittest.main()
