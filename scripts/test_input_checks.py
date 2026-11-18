"""266 (D117): the scripts refuse input they used to take silently.

    python scripts/test_input_checks.py

- `cards/apply_overlay.py`: `null` on a field no row of the table has (a misspelling) is refused; on a real field it
  drops it, and a rerun after the drop changes nothing.
- `design_check.py`: an unknown flag, a second DIR, a flag with no value and a DIR with no games file each stop it.

The read scripts' checks (`read_lib --base`, `paired_gap` on one seed, `play_read` / `use_read` pooling a native
folder with a farmed read) are in `bench/test_read_tools.py`. No build and no binary. Python's standard library only.
"""
import os
import subprocess
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, 'cards'))
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

    def test_missing_games_file(self):
        code, out = self.run_it(os.path.join(HERE, 'no_such_read'))
        self.assertNotEqual(code, 0)
        self.assertIn('no games file', out)


if __name__ == '__main__':
    unittest.main()
