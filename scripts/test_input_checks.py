"""266 (D117): the scripts refuse input they used to take silently.

    python scripts/test_input_checks.py

- `cards/apply_overlay.py`: `null` on a field no row of the table has (a misspelling) is refused; on a real field it
  drops it, and a rerun after the drop changes nothing.
- `design_check.py`: an unknown flag, a second DIR, a flag with no value and a DIR with no games file each stop it;
  `--agent` and `--variant` keep only their games (281).
- `cards/apply_overlay.py --from REV` (281): a bad REV stops it, an unchanged value changes nothing, and its warning
  names only the working-tree edits in fields the overlay doesn't set.
- `cards/effect_ledger.py --reasoned-delta BASE` (281): it needs `--stdout`, refuses `--hold-ratio` beside it and a
  bad BASE, and lists exactly the card whose reasoned value moved.
- `drafts/adversary/check.py`'s deal (304): its stage counts match the engine's generated `round_cap` table.
- `design_check_jobs.json` (307): every row names a card or power in content, stamped (`seen`) at its content now.
- `docs_params.py` (307): the docs' "`param` N" quotes match the baseline.

The read scripts' checks (`read_lib --base`, `paired_gap` on one seed, `play_read` / `use_read` pooling a native
folder with a farmed read) are in `bench/test_read_tools.py`. No build and no binary. Python's standard library only.
"""
import os
import subprocess
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, 'cards'))
sys.path.insert(0, HERE)  # design_check, when this file runs alone
import apply_overlay  # noqa: E402


def cards():
    return {'cards.json': [{'id': 'a', 'name': 'A', 'note': 'x', 'effects': [{'op': 'gain', 'amount': 1}]},
                           {'id': 'b', 'name': 'B'}]}


class ApplyOverlay(unittest.TestCase):
    def test_null_on_a_misspelled_field_is_refused(self):
        files = cards()
        with self.assertRaisesRegex(ValueError, "no field 'noet' to drop"):
            apply_overlay.apply_entry(files, 'cards.a.noet=null')
        with self.assertRaisesRegex(ValueError, "no field 'amont' to drop"):
            apply_overlay.apply_entry(files, 'cards.a.effects.0.amont=null')

    def test_null_drops_a_real_field_and_reruns(self):
        files = cards()
        self.assertTrue(apply_overlay.apply_entry(files, 'cards.a.note=null'))
        self.assertNotIn('note', files['cards.json'][0])
        # a field the row lacks but another row has: nothing to drop, no error (a rerun while any row still has it)
        files = cards()
        self.assertFalse(apply_overlay.apply_entry(files, 'cards.b.note=null'))
        self.assertFalse(apply_overlay.apply_entry(files, 'cards.*.amount=null'))
        # a field content.rs declares that no row holds now (an earlier run dropped it): the rerun changes nothing
        self.assertFalse(apply_overlay.apply_entry(files, 'cards.a.replaces=null'))

    def test_alignment_layout_round_trips(self):
        # 286: alignment.json is hand-laid; loading it and dumping it gives it back byte for byte, so an
        # `alignment.` entry applies and changes only its own lines
        import json
        path = os.path.join(HERE, '..', 'content', 'alignment.json')
        with open(path, encoding='utf-8', newline='') as f:
            text = f.read()
        data = json.loads(text)
        self.assertEqual(apply_overlay.dump(data, 'alignment.json'), text)
        row = data[0]
        files = {'alignment.json': data}
        self.assertTrue(apply_overlay.apply_entry(files, f"alignment.{row['id']}.tiers.1.text=New text"))
        new = apply_overlay.dump(data, 'alignment.json').split('\n')
        changed = [(a, b) for a, b in zip(text.split('\n'), new) if a != b]
        self.assertEqual(len(new), len(text.split('\n')))
        self.assertEqual(len(changed), 1)
        self.assertTrue(changed[0][1].startswith('   {"text": "New text",'), changed)
        code, out, err = run_script('apply_overlay.py', '--set', f"alignment.{row['id']}.name={row['name']}", '--check')
        self.assertEqual((code, err), (0, ''))
        self.assertIn('nothing to change', out)


def run_script(name, *args):
    p = subprocess.run([sys.executable, os.path.join(HERE, 'cards', name), *args],
                       capture_output=True, text=True, encoding='utf-8')
    return p.returncode, p.stdout, p.stderr


class ApplyOverlayFrom(unittest.TestCase):
    """281: `--from REV` applies over REV's files and warns of working-tree edits in fields it doesn't set."""

    def test_drift_names_only_fields_the_overlay_leaves(self):
        base = [{'id': 'a', 'cost': 5, 'note': 'x'}, {'id': 'b', 'cost': 2}]
        work = [{'id': 'a', 'cost': 4, 'note': 'y'}, {'id': 'b', 'cost': 2, 'tier': 2}, {'id': 'c', 'cost': 1}]
        drift = apply_overlay.unset_drift('cards', work, base, ['cards.a.cost=6'])
        self.assertEqual(drift, ['cards.a.note', 'cards.b.tier', 'cards.c (a row only in the working tree)'])
        # `*` and a whole row cover their fields
        self.assertEqual(apply_overlay.unset_drift('cards', work, base, ['cards.*.note=z', 'cards.b={"cost":2}',
                                                                          'cards.c={"cost":1}', 'cards.a.cost=1']), [])

    def test_bad_revision_stops_it(self):
        code, out, err = run_script('apply_overlay.py', '--set', 'cards.x.cost=1', '--from', 'no_such_rev', '--check')
        self.assertEqual(code, 2)
        self.assertIn('--from no_such_rev: no content/cards.json there', err)

    def test_from_head_on_a_clean_value_changes_nothing(self):
        import json
        with open(os.path.join(HERE, '..', 'content', 'cards.json'), encoding='utf-8') as f:
            card = json.load(f)[0]
        if subprocess.run(['git', 'diff', '--quiet', 'HEAD', '--', 'content/cards.json'],
                          cwd=os.path.join(HERE, '..')).returncode != 0:
            self.skipTest('content/cards.json differs from HEAD')
        code, out, err = run_script('apply_overlay.py', '--set', f"cards.{card['id']}.cost={card.get('cost', 0)}",
                                    '--from', 'HEAD', '--check')
        self.assertEqual((code, err), (0, ''))
        self.assertIn('1 entries from HEAD, nothing to change', out)


class ReasonedDelta(unittest.TestCase):
    """281: `effect_ledger.py --stdout --reasoned-delta BASE`."""

    def test_needs_stdout_and_not_hold_ratio(self):
        code, _, err = run_script('effect_ledger.py', '--reasoned-delta', 'HEAD')
        self.assertEqual(code, 2)
        self.assertIn('--reasoned-delta prints: pass --stdout', err)
        code, _, err = run_script('effect_ledger.py', '--stdout', '--reasoned-delta', 'HEAD', '--hold-ratio', 'x.csv')
        self.assertEqual(code, 2)
        self.assertIn('two different tables', err)

    def test_bad_base_stops_it(self):
        code, _, err = run_script('effect_ledger.py', '--stdout', '--reasoned-delta', 'no_such_rev')
        self.assertNotEqual(code, 0)
        self.assertIn('--reasoned-delta no_such_rev: neither a file nor a revision', err)

    def test_lists_the_card_whose_value_moved(self):
        import json
        import tempfile
        base = os.path.join(HERE, '..', 'content', 'cards.json')
        with open(base, encoding='utf-8') as f:
            cards = json.load(f)
        card = next(c for c in cards if any(e.get('op') == 'gain' and 'amount' in e for e in c.get('effects', [])))
        next(e for e in card['effects'] if e.get('op') == 'gain' and 'amount' in e)['amount'] += 3
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, 'cards.json')
            with open(path, 'w', encoding='utf-8', newline='\n') as f:
                json.dump(cards, f)
            code, out, err = run_script('effect_ledger.py', '--stdout', '--cards', path, '--reasoned-delta', base)
        self.assertEqual(code, 0, err)
        lines = out.strip().split('\n')
        self.assertEqual(lines[0], 'card,name,cost_base,cost_now,reasoned_before,reasoned_after,delta,moved_cost')
        self.assertEqual([ln.split(',')[0] for ln in lines[1:]], [card['id']])
        row = lines[1].split(',')
        self.assertGreater(float(row[6]), 0)
        self.assertEqual(int(row[7]), int(float(row[2]) + float(row[6]) + 0.5))


class DesignCheckArgs(unittest.TestCase):
    def run_it(self, *args):
        p = subprocess.run([sys.executable, os.path.join(HERE, 'design_check.py'), *args],
                           capture_output=True, text=True, encoding='utf-8')
        return p.returncode, p.stdout + p.stderr

    def test_unknown_flag(self):
        code, out = self.run_it('--quite')
        self.assertNotEqual(code, 0)
        self.assertIn('unknown flag --quite', out)

    def test_second_dir(self):
        code, out = self.run_it('out/a', 'out/b')
        self.assertNotEqual(code, 0)
        self.assertIn('one DIR only', out)

    def test_flag_without_value(self):
        code, out = self.run_it('--old')
        self.assertNotEqual(code, 0)
        self.assertIn('--old needs a value', out)

    def test_flag_twice(self):
        code, out = self.run_it('--old', 'a', '--old', 'b')
        self.assertNotEqual(code, 0)
        self.assertIn('--old given twice', out)

    def test_agent_and_variant_filters(self):
        # 281: --agent beside --variant, each agent as the spec writes it (its options' commas kept)
        import json
        import tempfile
        import design_check
        spec = {'configs': ['c'], 'variants': [{'name': 'bp'}, {'name': 'bp-on'}], 'difficulties': [2],
                'agents': ['strategist', 'strategist:w.a=1,w.b=2']}
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, 'games.jsonl')
            with open(path, 'w', encoding='utf-8', newline='\n') as f:
                f.write(json.dumps({'format': 1, 'spec': spec}) + '\n')
                for c in range(4):
                    f.write(json.dumps({'cell': c}) + '\n')
                f.write('{"cell": 4, "se')  # 284: a live run's half-written last line is dropped
            cells = lambda gs: [g['cell'] for g in gs]  # noqa: E731
            self.assertEqual(cells(design_check.load_games(path)), [0, 1, 2, 3])
            self.assertEqual(cells(design_check.load_games(path, agents={'strategist'})), [0, 2])
            opts = design_check.agent_list('greedy,strategist:w.a=1,w.b=2')
            self.assertEqual(opts, {'greedy', 'strategist:w.a=1,w.b=2'})
            self.assertEqual(cells(design_check.load_games(path, agents=opts)), [1, 3])
            self.assertEqual(cells(design_check.load_games(path, {'on'}, opts)), [3])
            with self.assertRaisesRegex(SystemExit, '--agent greedy: no game'):
                design_check.load_games(path, agents={'greedy'})
            # 286: --agent filters --old too: an old read without that agent stops it
            old = os.path.join(tmp, 'old.jsonl')
            with open(old, 'w', encoding='utf-8', newline='\n') as f:
                f.write(json.dumps({'format': 1, 'spec': dict(spec, agents=['greedy'])}) + '\n')
                f.write(json.dumps({'cell': 0}) + '\n')
            code, out = self.run_it(path, '--old', old, '--agent', 'strategist')
            self.assertNotEqual(code, 0)
            self.assertIn(f'--agent strategist: no game in {old}', out)
        code, out = self.run_it('--agent', 'strategist')
        self.assertNotEqual(code, 0)
        self.assertIn('--agent needs a read', out)

    def test_missing_games_file(self):
        code, out = self.run_it(os.path.join(HERE, 'no_such_read'))
        self.assertNotEqual(code, 0)
        self.assertIn('no games file', out)


class CheckDeal(unittest.TestCase):
    """304: `drafts/adversary/check.py`'s `stage_deals` mirrors the engine's deal (299 printed d4's deck a card short
    and wrote it into three docs). The engine's own numbers are the generated `round_cap` table in
    `docs/rules/adversary.md`, which `aic-sim docs --check` keeps current from the baseline."""

    def test_stage_deals_match_the_engines_round_cap_table(self):
        import importlib.util
        root = os.path.dirname(HERE)
        spec = importlib.util.spec_from_file_location('adversary_check', os.path.join(root, 'drafts', 'adversary', 'check.py'))
        check = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(check)
        with open(os.path.join(root, 'docs', 'rules', 'adversary.md'), encoding='utf-8') as f:
            text = f.read()
        block = text.split('<!-- generated: round_cap -->', 1)[1].split('<!-- /generated -->', 1)[0]
        engine = {}
        for line in block.strip().splitlines()[2:]:
            cells = [c.strip() for c in line.strip('|').split('|')]
            engine[int(cells[0])] = tuple(int(c) for c in cells[1:4])
        self.assertEqual(sorted(engine), [0, 1, 2, 3, 4])
        for d, stages, _ in check.stage_deals():
            self.assertEqual(stages, engine[d], f'difficulty {d}: check.py deals {stages}, the engine {engine[d]}')


class JobRows(unittest.TestCase):
    """307: `scripts/design_check_jobs.json` against content (302's and 306's diff reviews found rows priced against
    a card's old effects). Red here: recheck the named rows, then `python scripts/design_check.py --stamp-jobs NAME`."""

    def setUp(self):
        import design_check
        self.dc = design_check
        self.cfg = design_check.jload('scripts/design_check.json')
        self.cards = design_check.jload('content/cards.json')
        self.boards = design_check.jload('content/ais.json')

    def warnings(self, data=None):
        o = self.dc.Out()
        self.dc.check_job_rows(self.cfg, self.cards, self.boards, o, data)
        return [(c, n) for c, n, _ in o.warn]

    def test_the_table_matches_content(self):
        self.assertEqual(self.warnings(), [])

    def test_a_changed_card_a_gone_card_and_a_stray_job_are_flagged(self):
        import copy
        data = copy.deepcopy(self.dc.jload(self.dc.JOBS_FILE))
        job, rows = next(iter(data['jobs'].items()))
        card = next(c for c in rows['cards'] if not c.get('power'))
        live = next(c for c in self.cards if c['name'] == card['name'])
        live['cost'] += 1  # read live: no recheck
        self.assertEqual(self.warnings(data), [])
        live.setdefault('effects', []).append({'op': 'draw', 'amount': 1})
        self.assertEqual(self.warnings(data), [('job-stale', card['name'])])
        card['name'] = 'Gone Card'
        data['jobs']['Not a standard action'] = {'unit': 'x', 'cards': []}
        self.assertEqual(sorted(self.warnings(data)), [('job-table', 'Gone Card'), ('job-table', 'Not a standard action')])


class DocsParams(unittest.TestCase):
    """307: hand-written docs quote each tunable at its baseline (`docs_params.py`; r144 left three stale). Red
    here: fix the quote, or add an on-purpose one (an off arm) to its `ALLOW`."""

    def test_docs_quote_the_baseline(self):
        import docs_params
        self.assertEqual(docs_params.stale(), [])

    def test_a_stale_quote_is_found(self):
        import docs_params
        self.assertTrue(docs_params.QUOTE.search('spend `remove_ambassadors` (2) ambassadors'))
        self.assertIsNone(docs_params.QUOTE.search('`remove_ambassadors` 2 to 3'))


if __name__ == '__main__':
    unittest.main()
