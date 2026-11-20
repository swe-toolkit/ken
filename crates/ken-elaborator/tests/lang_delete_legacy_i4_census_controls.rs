//! Measurement-only I4 census controls, never part of the product merge.
//! Promise class: transition sentinel; retire with the scratch census.

use std::panic::{catch_unwind, AssertUnwindSafe};

use ken_elaborator::ElabEnv;

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
