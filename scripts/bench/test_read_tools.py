"""Checks of the read tools on a pair of identical reads (163), one all native and one half wasm, as a farmed read
of the same seeds gives:

    python scripts/bench/test_read_tools.py

- `ladder.py`: both win % columns equal at every difficulty, the gap +0.0, no seed left out (across engines, the
  default), and the win % columns still equal under `--same-engine`, which leaves the mixed seeds out of the gap only.
- `paired.py`: the gap is +0.0 on every seed by default, and on the same-engine seeds only under `--same-engine`.

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


if __name__ == '__main__':
    unittest.main()
