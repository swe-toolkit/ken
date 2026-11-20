//! Measurement-only I4 census controls, never part of the product merge.
//! Promise class: transition sentinel; retire with the scratch census.

use std::panic::{catch_unwind, AssertUnwindSafe};

use ken_elaborator::ElabEnv;

/// MEASURED: this census test always panics with the unique safety marker.
/// CLAIMED: no measurement-only commit can be published by a merge-on-green
/// path. THE GAP: the CI planner must include this test in its exact-once shard
/// union and the publisher must stop on a red result; the named local run
/// validates the refusal, not the future CI selection.
#[test]
fn census_commit_never_lands() {
    panic!("LEGACY_CENSUS_SENTINEL measurement-only commit; close, never land");
}

/// MEASURED: a bare pre-source, non-floor `Not` goes through
/// resolve_expr_standalone and registers one matched flat-global arrival.
/// CLAIMED: an I4 deletion cannot overlook the standalone expression route.
/// THE GAP: this row checks a real production call rather than manufacturing
/// a counter entry; the other standalone callers are counted by scratch CI.
#[test]
fn bare_not_elaborate_expr_registers_a_standalone_arrival() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    assert!(
        env.globals.contains_key("Not"),
        "Not is a live pre-source global"
    );
    let caught = catch_unwind(AssertUnwindSafe(|| env.elaborate_expr("not-owner", "Not")))
        .expect_err("bare Not must register at the standalone resolver");
    let report = caught
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| caught.downcast_ref::<&str>().copied())
        .expect("census panic is a string");
    assert!(
        report.contains("LEGACY_CENSUS_STANDALONE")
            && report.contains("row=bare_not_elaborate_expr_registers_a_standalone_arrival")
            && report.contains("entry=elaborate_expr")
            && report.contains("owner=not-owner")
            && report.contains("arrivals=1")
            && report.contains("\"Not\": 1")
            && report.contains("matched=global"),
        "wrong census route: {report}"
    );
}

/// MEASURED: a type-only bare name in a standalone ascription reaches the
/// same flat-global counter, without an expression-level RCon for that name.
/// CLAIMED: the census covers both value and type lookup positions within an
/// elaborate_expr call. THE GAP: later type-checking is deliberately bypassed
/// by the measurement panic; no type correctness is inferred from this row.
#[test]
fn bare_not_in_standalone_ascription_registers_a_type_arrival() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    let caught = catch_unwind(AssertUnwindSafe(|| {
        env.elaborate_expr("not-type-owner", "0 : Not")
    }))
    .expect_err("type-only Not must reach standalone resolution");
    let report = caught
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| caught.downcast_ref::<&str>().copied())
        .expect("census panic is a string");
    assert!(
        report.contains("LEGACY_CENSUS_STANDALONE")
            && report.contains("row=bare_not_in_standalone_ascription_registers_a_type_arrival")
            && report.contains("owner=not-type-owner")
            && report.contains("arrivals=1")
            && report.contains("\"Not\": 1"),
        "type-only name did not reach the matched standalone counter: {report}"
    );
}
