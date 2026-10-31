#!/usr/bin/env python3
"""In-memory controls for the environment-pin CI source gate."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("check-test-environment-pins.py")
SPEC = importlib.util.spec_from_file_location("environment_pin_lint", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
lint = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = lint
SPEC.loader.exec_module(lint)


class EnvironmentPinControls(unittest.TestCase):
    def test_pre_px10_forced_owner_source_detects_both_ids(self) -> None:
        # From 42ba094f3:rt_performing_underived.rs, before the PX10 re-key.
        text = '''fn forced_underived_owner_either_refuses_or_preserves_performed_operations() {
    for forced_owner in [None, Some(901_u32), Some(778_u32)] {
        let build = || ken_cli::build_native_program(source);
        let rows = plan.all_static_response_rows.iter()
            .map(|row| (row.vis_origin, row.operation.as_str()))
            .collect::<Vec<_>>();
        assert!(rows.contains(&(778, "FsReadAt"))
            && rows.contains(&(901, "BufferAllocate")));
    }
}'''
        hits = lint.scan_text("crates/ken-cli/tests/rt_performing_underived.rs", text)
        self.assertEqual(sorted(hit.value for hit in hits if hit.shape == "origin-id"),
                         ["778", "778", "901", "901"])

    def test_pre_px10_baseline_semantic_hash_detected(self) -> None:
        # From 42ba094f3:verify_package_duplicate_top_level_refusal.rs:130.
        text = '''#[test]
fn unrelated_example_keeps_the_baseline_semantics_and_hash() {
    let baseline = compile_literate(ADMITTED).expect("package");
    assert_eq!(baseline.core_semantic_hash, 0xbe86_689a_0345_7405);
}'''
        hits = lint.scan_text(
            "crates/ken-elaborator/tests/verify_package_duplicate_top_level_refusal.rs", text
        )
        self.assertEqual([(hit.value, hit.shape) for hit in hits],
                         [("0xbe86_689a_0345_7405", "hash")])

    def test_bare_tuple_hits_without_keyword_on_its_line(self) -> None:
        # From the PX10-refreshed px7f_resource_native.rs:566.
        text = '''fn fixture() {
    let plans = ken_cli::build_native_program(source);
    let owner = plan.returned_vis_protocols;
    let rows = [("double-release", DOUBLE_RELEASE, 695, &[(0, &[603])])];
}'''
        hits = lint.scan_text("crates/ken-cli/tests/px7f_resource_native.rs", text)
        self.assertIn((4, "695", "origin-id"),
                      [(hit.line, hit.value, hit.shape) for hit in hits])

    def test_hand_built_global_id_and_production_are_not_environment_reads(self) -> None:
        text = '''fn production() { let _ = GlobalId(901); }
#[cfg(test)]
pub(crate) mod tests {
    fn fixture() {
        let _ = ken_cli::compile_program(source);
        let _ = plan.returned_vis_protocols;
        let id = GlobalId(503);
    }
}'''
        self.assertEqual(lint.scan_text("crates/ken-kernel/src/lib.rs", text), [])
        self.assertEqual(lint.scan_text(
            "crates/ken-kernel/tests/fixture.rs", "fn fixture() { let id = GlobalId(503); }"
        ), [])

    def test_global_reads_are_distinct_from_fixture_constructors(self) -> None:
        text = '''fn fixture() {
    let a = env.globals[503];
    let b = env.globals.get(504);
    let fixture = GlobalId(505);
}'''
        hits = lint.scan_text("crates/ken-kernel/tests/fixture.rs", text)
        self.assertEqual(hits, [
            lint.Hit("crates/ken-kernel/tests/fixture.rs", 2, "503", "global-read"),
            lint.Hit("crates/ken-kernel/tests/fixture.rs", 3, "504", "global-read"),
        ])

    def test_split_count_is_statement_scoped(self) -> None:
        text = '''assert_eq!(
    x
        .iter()
        .count(),
    12
);'''
        self.assertEqual(lint.scan_text("crates/ken-kernel/tests/fixture.rs", text),
                         [lint.Hit("crates/ken-kernel/tests/fixture.rs", 1, "12", "count")])

    def test_split_hash_is_statement_scoped(self) -> None:
        text = '''assert_eq!(
    digest,
    0x0123_4567_89ab_cdef
);'''
        self.assertEqual(lint.scan_text("crates/ken-kernel/tests/fixture.rs", text),
                         [lint.Hit("crates/ken-kernel/tests/fixture.rs", 1,
                                   "0x0123_4567_89ab_cdef", "hash")])

    def test_one_digit_count_is_not_a_pin(self) -> None:
        self.assertEqual(lint.scan_text(
            "crates/ken-kernel/tests/fixture.rs", "assert_eq!(xs.len(), 9);"
        ), [])

    def test_hash_requires_twelve_digits_not_padding_underscores(self) -> None:
        self.assertEqual(lint.scan_text(
            "crates/ken-kernel/tests/fixture.rs", "assert_eq!(digest, 0x1234_5678_abc);"
        ), [])

    def test_rust_macro_delimiters_and_inline_attributes_do_not_evoke_blindness(self) -> None:
        text = '''fn production() { assert_eq!(xs.len(), 44); }
#[cfg(test)]
#[allow(dead_code)]
pub(crate) mod tests {
    fn fixture() { assert_eq! { xs.len(), 27 }; }
    fn sibling() { assert_eq! [ xs.len(), 28 ]; }
}'''
        self.assertEqual(lint.scan_text("crates/ken-kernel/src/lib.rs", text), [
            lint.Hit("crates/ken-kernel/src/lib.rs", 5, "27", "count"),
            lint.Hit("crates/ken-kernel/src/lib.rs", 6, "28", "count"),
        ])

    def test_stale_inventory_entry_fails_even_when_there_are_no_hits(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            test = root / "crates" / "ken-kernel" / "tests" / "empty.rs"
            test.parent.mkdir(parents=True)
            test.write_text("#[test] fn empty() {}\n", encoding="utf-8")
            old = lint.ALLOWED
            try:
                lint.ALLOWED = (("crates/ken-kernel/tests/empty.rs", "123", "fixture input value",
                                 "no longer present"),)
                self.assertEqual(lint.inspect(root), ([], set(lint.ALLOWED)))
            finally:
                lint.ALLOWED = old


if __name__ == "__main__":
    unittest.main()
