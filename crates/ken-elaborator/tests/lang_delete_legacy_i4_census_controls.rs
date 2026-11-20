//! Measurement-only I4 census controls, never part of the product merge.
//! Promise class: transition sentinel; retire with the scratch census.

use ken_elaborator::{census_i4, ElabEnv};
use serde_json::Value;
use std::path::PathBuf;

/// MEASURED: this census test always panics with the unique safety marker.
/// CLAIMED: no measurement-only commit can be published by a merge-on-green
/// path. THE GAP: the CI planner must include this test in its exact-once shard
/// union and the publisher must stop on a red result; the named local run
/// validates the refusal, not the future CI selection.
#[test]
fn census_commit_never_lands() {
    panic!("LEGACY_CENSUS_SENTINEL measurement-only commit; close, never land");
}

fn standalone_record<'a>(records: &'a [Value], name: &str) -> &'a Value {
    let matches: Vec<_> = records
        .iter()
        .filter(|record| record["kind"] == "STANDALONE" && record["payload"]["name"] == name)
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "one actual standalone arrival: {records:?}"
    );
    matches[0]
}

fn catalog() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

/// MEASURED: a bare pre-source, non-floor Not is flat-routed by the original
/// standalone resolver while the Strict-root shadow reports REFUSED.
/// CLAIMED: I4 cannot overlook a standalone expression reference.
/// THE GAP: this production call is one row; CI counts the other callers.
#[test]
fn bare_not_elaborate_expr_registers_a_standalone_arrival() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    let flat = env.globals["Not"].0;
    let _ = env.elaborate_expr("not-owner", "Not");
    let records = census_i4::current_thread_records();
    let hit = standalone_record(&records, "Not");
    assert_eq!(hit["payload"]["status"], "REFUSED", "{hit}");
    assert_eq!(hit["payload"]["flat"], flat, "{hit}");
    assert_eq!(hit["payload"]["owner"], "not-owner", "{hit}");
    assert_eq!(
        hit["test"], "bare_not_elaborate_expr_registers_a_standalone_arrival",
        "{hit}"
    );
    assert!(
        records.iter().all(|record| record["kind"] != "S"),
        "a shadow-only lookup manufactured an S-* arrival: {records:?}"
    );
}

/// MEASURED: a type-only bare Not in a standalone ascription is separately
/// recorded as REFUSED, without relying on an expression-level RCon.
/// CLAIMED: both value and type lookup positions reach the shadow census.
/// THE GAP: later type-checking can reject the ascription; only its route is pinned.
#[test]
fn bare_not_in_standalone_ascription_registers_a_type_arrival() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    let _ = env.elaborate_expr("not-type-owner", "0 : Not");
    let records = census_i4::current_thread_records();
    let hit = standalone_record(&records, "Not");
    assert_eq!(hit["payload"]["status"], "REFUSED", "{hit}");
    assert_eq!(hit["payload"]["owner"], "not-type-owner", "{hit}");
    assert_eq!(
        hit["test"], "bare_not_in_standalone_ascription_registers_a_type_arrival",
        "{hit}"
    );
    assert!(
        records.iter().all(|record| record["kind"] != "S"),
        "a shadow-only type lookup manufactured an S-* arrival: {records:?}"
    );
}

/// MEASURED: True is a pre-source constructor with a flat ID but the Strict
/// root admits its spelling without selecting a checked scope ID.
/// CLAIMED: builtin successes are neither silently filtered nor called SAME.
/// THE GAP: this is one floor-former constructor; the CI census sees the rest.
#[test]
fn bare_true_reports_unselected_builtin_with_flat_id() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    let flat = env.globals["True"].0;
    let _ = env.elaborate_expr("true-owner", "True");
    let records = census_i4::current_thread_records();
    let hit = standalone_record(&records, "True");
    assert_eq!(hit["payload"]["status"], "UNSELECTED-BUILTIN", "{hit}");
    assert!(hit["payload"]["checked"].is_null(), "{hit}");
    assert_eq!(hit["payload"]["flat"], flat, "{hit}");
}

/// MEASURED: an imported alias selects the provider's checked ID but has no
/// matching flat-global spelling; then an explicitly rebound flat spelling
/// produces a genuine distinct-ID observation at the same root binding.
/// CLAIMED: ROOT-ONLY and DIVERGED are different instrument outcomes.
/// THE GAP: these are controlled fixture states, not population frequencies.
#[test]
fn renamed_not_import_distinguishes_root_only_from_diverged() {
    let mut env = ElabEnv::new().expect("pre-source environment");
    env.set_catalog_roots_v1(&[catalog()])
        .expect("catalog root");
    env.elaborate_file("import Core.Logic.Not (Not as N)")
        .expect("checked renamed provider");
    assert!(!env.globals.contains_key("N"));
    let _ = env.elaborate_expr("alias-owner", "N");
    let first = standalone_record(&census_i4::current_thread_records(), "N").clone();
    assert_eq!(first["payload"]["status"], "ROOT-ONLY", "{first}");
    assert!(first["payload"]["flat"].is_null(), "{first}");
    assert!(first["payload"]["checked"].is_number(), "{first}");

    let rival = env.globals["True"];
    env.globals.insert("N".to_string(), rival);
    let _ = env.elaborate_expr("alias-rival-owner", "N");
    let records = census_i4::current_thread_records();
    let hits: Vec<_> = records
        .iter()
        .filter(|record| {
            record["kind"] == "STANDALONE"
                && record["payload"]["name"] == "N"
                && record["payload"]["owner"] == "alias-rival-owner"
        })
        .collect();
    assert_eq!(hits.len(), 1, "one genuine divergent arrival: {records:?}");
    let second = hits[0];
    assert_eq!(second["payload"]["status"], "DIVERGED", "{second}");
    assert_eq!(second["payload"]["flat"], rival.0, "{second}");
    assert_eq!(
        second["payload"]["checked"], first["payload"]["checked"],
        "{second}"
    );
}
