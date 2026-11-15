#!/usr/bin/env python3
"""Exact Z3 feature-on lib-test selector and realized-inventory controls."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("ci-z3-lib-scope.py")
spec = importlib.util.spec_from_file_location("ci_z3_lib_scope", SCRIPT)
scope = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scope)


def listing(selected):
    names = sorted(scope.EXCLUDED) + [
        sorted(scope.EXCLUDED)[0] + "_suffix_sibling", "another_lib_test"
    ]
    return {
        "test-count": len(names),
        "rust-suites": {
            "lib": {
                "binary-id": scope.BINARY_ID,
                "binary-name": "ken_elaborator",
                "testcases": {
                    name: {"filter-match": {"status": "matches" if name in selected else "mismatch"}}
                    for name in names
                },
            },
        },
    }


class Z3LibScopeTests(unittest.TestCase):
    def test_exact_exclusions_keep_prefix_sibling_and_new_tests(self):
        full = listing(set(scope.EXCLUDED) | {sorted(scope.EXCLUDED)[0] + "_suffix_sibling", "another_lib_test"})
        selected = listing({sorted(scope.EXCLUDED)[0] + "_suffix_sibling", "another_lib_test"})
        self.assertEqual(scope.check(full, selected), 2)
        filter_expression = scope.filter_expression()
        for name in scope.EXCLUDED:
            self.assertIn(f"test(={name})", filter_expression)
        self.assertIn("binary_id(=ken-elaborator)", filter_expression)
        with tempfile.TemporaryDirectory() as directory:
            first = Path(directory) / "full.json"
            second = Path(directory) / "selected.json"
            first.write_text(json.dumps(full))
            second.write_text(json.dumps(selected))
            result = subprocess.run([sys.executable, str(SCRIPT), "check", str(first), str(second)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("2 selected, 4 exact exclusions", result.stdout)

    def test_substring_skip_sibling_loss_reddens_realized_check(self):
        sibling = sorted(scope.EXCLUDED)[0] + "_suffix_sibling"
        full = listing(set(scope.EXCLUDED) | {sibling, "another_lib_test"})
        # Model libtest's substring --skip: it incorrectly omits the sibling.
        selected = listing({"another_lib_test"})
        with self.assertRaisesRegex(ValueError, "missing=.*suffix_sibling"):
            scope.check(full, selected)
        with self.assertRaisesRegex(ValueError, "unexpected="):
            scope.check(full, listing({sibling, "another_lib_test", sorted(scope.EXCLUDED)[0]}))

    def test_missing_exclusion_or_malformed_inventory_fails_closed(self):
        sibling = sorted(scope.EXCLUDED)[0] + "_suffix_sibling"
        full = listing(set(scope.EXCLUDED) | {sibling, "another_lib_test"})
        selected = listing({sibling, "another_lib_test"})
        full["rust-suites"]["lib"]["testcases"].pop(next(iter(scope.EXCLUDED)))
        full["test-count"] -= 1
        with self.assertRaisesRegex(ValueError, "inventories differ"):
            scope.check(full, selected)
        with self.assertRaisesRegex(Exception, "test-count"):
            scope.check({"test-count": 0, "rust-suites": {}}, selected)


if __name__ == "__main__":
    unittest.main()
