"""Checks of `trace_lib.py`, `same_play.py tracediff` and `scripts/edit/replace.py` (281):

    python scripts/bench/test_trace_lib.py

No build and no binary: the traces and files are written here, in temporary folders. Standard library only.
"""
import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, '..', 'edit'))
import replace  # noqa: E402
import same_play  # noqa: E402
import trace_lib  # noqa: E402


def ev(**e):
    return json.dumps({'event': e})


def act(options=1, **a):
    return json.dumps({'action': a, 'options': options})


TRACE = [
    json.dumps({'config_hash': 'abc', 'seed': '0', 'trace': 'baseline'}),
    ev(type='tokens_changed', hex=[0, 0], token='troop', delta=1, total=1),
    ev(type='round_started', round=1),
    ev(type='phase_started', phase='cards.play'),
    ev(type='asked', prompt='Seat 0: which hex?', seat=0),
    act(3, type='choose', seat=0, option=1),
    ev(type='var_changed', key='adversary.compute_waiting', delta=1, total=1),
    ev(type='resource_changed', resource='credits', delta=-2, total=5),
    act(2, type='play', seat=1, card='x'),
    'seeds 0..=0, agent random (max 100000 actions), content 0, engine 0.1.0 rules r1',
    'run overrides: countries=3 adversary.difficulty=1',
    '',
    '                                     run',
    'config                     configs/baseline.json',
    'win %                      0.0 ±0.0',
    'ai.offered.builder /game                  7.00',
    'adversary.ambassadors_r01 /game reaching it   3.00',
]


class TraceLib(unittest.TestCase):
    def test_walk(self):
        steps = list(trace_lib.walk(TRACE))
        self.assertEqual([s.kind for s in steps], ['tokens_changed', 'round_started', 'phase_started', 'asked',
                                                   'action', 'var_changed', 'resource_changed', 'action'])
        self.assertEqual((steps[0].round, steps[0].phase), (0, ''))
        choose = steps[4]
        self.assertEqual((choose.round, choose.phase, choose.seat, choose.options, choose.ask, choose.line),
                         (1, 'cards.play', 0, 3, 'Seat 0: which hex?', 6))
        self.assertIsNone(steps[7].ask)
        self.assertEqual((steps[6].seat, steps[6].actor), (None, 0))  # an event with no seat: the last actor's

    def test_summary(self):
        s = trace_lib.summary(TRACE)
        self.assertEqual(s['overrides'], ['countries=3', 'adversary.difficulty=1'])
        self.assertEqual(s['counters'], {'ai.offered.builder': 7.0, 'adversary.ambassadors_r01': 3.0})
        self.assertEqual(s['rows']['win %'], '0.0 ±0.0')
        self.assertTrue(s['header'].startswith('seeds '))

    def test_content(self):
        self.assertTrue(trace_lib.card_names())
        hexes = trace_lib.hexes(2)
        self.assertEqual(len(hexes), 32)
        self.assertEqual(len(set(hexes.values())), 32)

    def test_first_diff(self):
        other = list(TRACE)
        other[0] = other[0].replace('abc', 'def')
        other[6] = other[6].replace('"total": 1', '"total": 2')
        self.assertEqual(trace_lib.first_diff(TRACE, other)[0][0], 1)
        a, b, ctx = trace_lib.first_diff(TRACE, other, ['config_hash'])
        self.assertEqual((a[0], b[0]), (7, 7))
        self.assertEqual(ctx, [TRACE[2], TRACE[3]])
        self.assertIsNone(trace_lib.first_diff(TRACE, other, ['config_hash', 'adversary.compute_waiting']))
        self.assertIsNone(trace_lib.first_diff(TRACE, other, ['config_hash', 'adversary.']))
        a, b, _ = trace_lib.first_diff(TRACE, TRACE[:6], [])
        self.assertEqual((a[0], b[1]), (7, '(end of trace)'))

    def test_tracediff_cli(self):
        with tempfile.TemporaryDirectory() as d:
            p, q = os.path.join(d, 'a'), os.path.join(d, 'b')
            for f, lines in ((p, TRACE), (q, TRACE[:5] + TRACE[6:])):
                with open(f, 'w', encoding='utf-8', newline='\n') as h:
                    h.write('\n'.join(lines) + '\n')
            out = io.StringIO()
            with contextlib.redirect_stdout(out):
                self.assertEqual(same_play.tracediff(p, p, []), 0)
                self.assertEqual(same_play.tracediff(p, q, []), 1)
            self.assertIn('phase_started', out.getvalue())


class Replace(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = self.tmp.name
        self.f = os.path.join(self.d, 'a.txt')
        with open(self.f, 'w', encoding='utf-8', newline='') as h:
            h.write('alpha beta\nalpha_x alpha\n')

    def tearDown(self):
        self.tmp.cleanup()

    def text(self):
        with open(self.f, encoding='utf-8', newline='') as h:
            return h.read()

    def run_spec(self, spec, *extra):
        s = os.path.join(self.d, 'spec.json')
        with open(s, 'w', encoding='utf-8', newline='\n') as h:
            json.dump(spec, h)
        with contextlib.redirect_stdout(io.StringIO()):
            return replace.main([s, *extra])

    def test_spec(self):
        self.assertEqual(self.run_spec([{'file': self.f, 'old': 'beta', 'new': 'gamma'},
                                        {'file': self.f, 'old': 'alpha', 'new': 'a', 'count': 3}]), 0)
        self.assertEqual(self.text(), 'a gamma\na_x a\n')

    def test_all_or_nothing(self):
        before = self.text()
        self.assertEqual(self.run_spec([{'file': self.f, 'old': 'beta', 'new': 'gamma'},
                                        {'file': self.f, 'old': 'alpha', 'new': 'a'}]), 1)  # 3 found, 1 wanted
        self.assertEqual(self.run_spec([{'file': self.f, 'old': 'beta', 'new': 'gamma'},
                                        {'file': os.path.join(self.d, 'none'), 'old': 'x', 'new': 'y'}]), 1)
        self.assertEqual(self.run_spec([{'file': self.f, 'old': 'alpha', 'new': 'a', 'count': 'all'}],
                                       '--dry-run'), 0)
        self.assertEqual(self.text(), before)

    def test_word(self):
        git = lambda *a: subprocess.run(['git', *a], cwd=self.d, check=True, capture_output=True)
        git('init', '-q')
        git('add', 'a.txt')
        here = os.getcwd()
        os.chdir(self.d)
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(replace.main(['--word', 'alpha', 'omega', '.']), 0)
        finally:
            os.chdir(here)
        self.assertEqual(self.text(), 'omega beta\nalpha_x omega\n')


if __name__ == '__main__':
    unittest.main()
