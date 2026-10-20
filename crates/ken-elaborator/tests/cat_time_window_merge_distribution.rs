//! Ordered window/merge distribution: spec/50-stdlib/69-time-and-events.md §3.
//! Source: conformance/stdlib/time-and-events/seed-window-merge-distribution.md.
//! Promise class: durable invariant. Closed value controls supplement the checked
//! universal law; each false one-sided-order claim is tied to its own controls.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole, Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-window-merge-distribution.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded_trace() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §3 roots-loads from the checked Derived provider");
    env
}

// This client restates the full §3.1 type at arbitrary payload and bounds,
// with both independent ordering premises. It also imports the one newly
// public generic Derived proof without preloading a facaded alias.
const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (filter, filter_starts_after, starts_after)
import Data.Time.EventTrace
  (Event, time_ordered, trace_merge, trace_window, trace_window_merge)

theorem generic_filter_starts_after
  (a : Type) (le : a → a → Bool)
  (le_trans : (x : a) → (y : a) → (z : a) →
    Equal Bool (le x y) True → Equal Bool (le y z) True →
    Equal Bool (le x z) True)
  (keep : a → Bool) (x : a) (xs : List a) :
  is_sorted a le (Cons a x xs) →
  starts_after a le x (filter a keep xs) =
  filter_starts_after a le le_trans keep x xs

theorem generic_window_merge
  (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p)) :
  (ys : List (Event p)) →
  time_ordered p xs → time_ordered p ys →
  Equal (List (Event p))
    (trace_window p lo hi (trace_merge p xs ys))
    (trace_merge p (trace_window p lo hi xs) (trace_window p lo hi ys)) =
  trace_window_merge p lo hi xs
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §§3.1, 3.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: both owner modules roots-load; the exact trusted GlobalId set
/// equals the providers' set after each load. The two public proofs belong to
/// their modules and are transparent; an importing arbitrary-payload client
/// restates both full types. Every new EventTrace helper refuses import.
/// CLAIMED: the distribution law and generic provider are checked, public,
/// and add no new trust, while their proof scaffolding stays private.
/// THE GAP: inherited provider trust is unchanged, not zero; the checked
/// universal bodies and closed seed below each establish different properties.
#[test]
fn generic_window_merge_is_package_owned_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Function.Combinators",
        "Core.Logic.Transport",
        "Core.Logic.Compare",
        "Core.Logic.And",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Classes.LawfulClasses",
        "Data.Collections.List",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived's public filter_starts_after proof checks");
    let after_derived = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_derived, before, "Derived adds no trusted GlobalId");
    let filter_id = env.globals[&format!("{DERIVED}.filter_starts_after")];
    assert!(
        derived_owned.contains(&filter_id),
        "Derived owns the filter proof"
    );
    assert!(matches!(
        env.env.lookup(filter_id),
        Some(Decl::Transparent { .. })
    ));

    let trace_owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace's checked distribution proof roots-loads");
    let after_trace = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§3 trusted_base() count: providers={}, Derived={}, EventTrace={}",
        before.len(),
        after_derived.len(),
        after_trace.len()
    );
    assert_eq!(after_trace, before, "EventTrace adds no trusted GlobalId");
    let law_id = env.globals[&format!("{TRACE}.trace_window_merge")];
    assert!(
        trace_owned.contains(&law_id),
        "EventTrace owns the §3 proof"
    );
    assert!(matches!(
        env.env.lookup(law_id),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("both generic public theorem types check in a selective client");

    // Import a public name first, so a broken import path cannot masquerade
    // as privacy. Check each helper separately for its exact qualified name.
    env.elaborate_file(&format!(
        "import {TRACE} (trace_window_merge as public_control)"
    ))
    .expect("§3 public law imports from the package");
    for name in [
        "trace_window_merge_into",
        "trace_window_merge_into_cons",
        "trace_window_merge_left_head",
        "trace_window_merge_right_head",
        "trace_head_after",
        "trace_merge_nil_right",
        "trace_merge_cons_left",
        "trace_merge_cons_right",
        "trace_window_head_after",
        "trace_window_head_after_decide",
        "trace_head_after_from_start",
        "trace_head_after_decide",
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(trace_owned.contains(&id), "{name} belongs to EventTrace");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {TRACE} ({name})")) {
            Err(ElabError::UnboundName { name: missing, .. }) => {
                assert_eq!(missing, format!("{TRACE}.{name}"));
            }
            other => panic!("private EventTrace {name} must refuse import: {other:?}"),
        }
    }
}

fn seed_sources() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("distribution seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

fn accepted_seed_prefix(sources: &[&str]) -> String {
    let [ordered, controls, _, _] = sources else {
        panic!("seed needs two checked control fences and two separate refusals");
    };
    let mut env = loaded_trace();
    env.elaborate_file(ordered)
        .expect("ordered law instance and both exact tie outputs check");
    env.elaborate_file(controls)
        .expect("both unordered inputs and four exact output controls check");
    format!("{ordered}\n{controls}")
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-merge-distribution.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the ordered positive fence and the independent exact-output
/// unordered control fence both elaborate through the root-loaded package.
/// CLAIMED: windowing and merging produce the ordered left-tie list, while
/// unordered inputs remain accepted with the two specified differing outputs.
/// THE GAP: these Bool fixtures do not establish the quantified theorem; its
/// checked body and generic client are checked separately above.
#[test]
fn shipped_ordered_and_unordered_value_controls_check() {
    let sources = seed_sources();
    accepted_seed_prefix(&sources);
}

// Compare parsed Ken propositions, ignoring only source spans and formatting.
// The controlled expressions themselves are values of the checked seed, not
// facts about the Markdown file's wording or line positions.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) => format!("con:{name}"),
        SourceType::TVar(name, _) => format!("var:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed seed type expression: {other:?}"),
    }
}

fn applied<'a>(ty: &'a SourceType, name: &str) -> Vec<&'a SourceType> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    match head {
        SourceType::TCon(found, _) | SourceType::TVar(found, _) if found == name => args,
        other => panic!("expected {name} head, got {other:?}"),
    }
}

fn equality(ty: &SourceType) -> Option<[&SourceType; 3]> {
    let mut head = ty;
    while let SourceType::TApp(function, _, _) = head {
        head = function;
    }
    if !matches!(head, SourceType::TCon(name, _) | SourceType::TVar(name, _) if name == "Equal") {
        return None;
    }
    Some(
        applied(ty, "Equal")
            .try_into()
            .expect("seed equality has carrier and two endpoints"),
    )
}

// Require a window-of-merge and a merge-of-windows on the very same two
// inputs, with the same payload and bounds. A false equality about arbitrary
// other lists is not a counterexample to this distribution law.
fn assert_distribution_operands(
    windowed_merge: &SourceType,
    merged_windows: &SourceType,
    expected_left: &str,
    expected_right: &str,
) {
    let [p, lo, hi, merged]: [&SourceType; 4] = applied(windowed_merge, "trace_window")
        .try_into()
        .expect("window of merge has four arguments");
    let [merge_p, xs, ys]: [&SourceType; 3] = applied(merged, "trace_merge")
        .try_into()
        .expect("inner merge has three arguments");
    let [output_p, wx, wy]: [&SourceType; 3] = applied(merged_windows, "trace_merge")
        .try_into()
        .expect("merge of windows has three arguments");
    let [left_p, left_lo, left_hi, left_xs]: [&SourceType; 4] = applied(wx, "trace_window")
        .try_into()
        .expect("left window has four arguments");
    let [right_p, right_lo, right_hi, right_ys]: [&SourceType; 4] = applied(wy, "trace_window")
        .try_into()
        .expect("right window has four arguments");
    let shape = type_shape;
    for comparable in [merge_p, output_p, left_p, right_p] {
        assert_eq!(shape(p), shape(comparable), "all payloads must agree");
    }
    for comparable in [left_lo, right_lo] {
        assert_eq!(shape(lo), shape(comparable), "all lower bounds must agree");
    }
    for comparable in [left_hi, right_hi] {
        assert_eq!(shape(hi), shape(comparable), "all upper bounds must agree");
    }
    assert_eq!(
        shape(xs),
        shape(left_xs),
        "left input is windowed unchanged"
    );
    assert_eq!(
        shape(ys),
        shape(right_ys),
        "right input is windowed unchanged"
    );
    assert_eq!(shape(xs), format!("var:{expected_left}"));
    assert_eq!(shape(ys), format!("var:{expected_right}"));
}

fn controlled_actual<'a>(
    decls: &'a [SourceDecl],
    actual: &SourceType,
) -> (&'a SourceType, &'a SourceType) {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { theorem, .. } => {
                equality(theorem).and_then(|[carrier, observed, expected]| {
                    (type_shape(observed) == type_shape(actual)).then_some((carrier, expected))
                })
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let [control] = found.as_slice() else {
        panic!("each distribution operand needs its own checked exact-output control");
    };
    *control
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §3.1; shipped seed refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the shared ordered and unordered value controls check first;
/// each refusal's parsed equality compares exactly the checked actual
/// window-of-merge and merge-of-windows expressions for its own input pair,
/// whose independently checked expected outputs differ. Each refusal is then
/// kernel-rejected at its own theorem in a fresh environment.
/// CLAIMED: neither one-sided ordering premise suffices for the structural
/// list equality; an unrelated false equality cannot substitute for the case.
/// THE GAP: finite counterexamples refute weakenings, not prove the general
/// law. The arbitrary-payload checked proof is exercised separately above.
#[test]
fn each_one_sided_order_refusal_matches_its_checked_false_value_pair() {
    let sources = seed_sources();
    let positive = accepted_seed_prefix(&sources);
    let positives = parse_decls(&positive).expect("checked seed controls parse");
    for (negative, expected_left, expected_right) in [
        (sources[2], "unordered_left", "ordered_right"),
        (sources[3], "ordered_left", "unordered_right"),
    ] {
        let decls = parse_decls(negative).expect("negative seed fence parses");
        let [SourceDecl::TheoremDecl {
            params,
            theorem,
            body,
            span: theorem_span,
            ..
        }] = decls.as_slice()
        else {
            panic!("each refused seed fence must contain one theorem");
        };
        assert!(params.is_empty(), "seed refusals are closed claims");
        assert!(
            matches!(body, ken_elaborator::Expr::ECon(name, _) if name == "Proved"),
            "negative must attempt a checked proof"
        );
        let [carrier, windowed_merge, merged_windows] =
            equality(theorem).expect("negative claims a structural equality");
        assert_distribution_operands(
            windowed_merge,
            merged_windows,
            expected_left,
            expected_right,
        );
        let (left_carrier, left_control) = controlled_actual(&positives, windowed_merge);
        let (right_carrier, right_control) = controlled_actual(&positives, merged_windows);
        assert_eq!(type_shape(carrier), type_shape(left_carrier));
        assert_eq!(type_shape(carrier), type_shape(right_carrier));
        assert_ne!(
            type_shape(left_control),
            type_shape(right_control),
            "checked exact outputs must disagree on this pair"
        );
        let mut env = loaded_trace();
        env.elaborate_file(&positive)
            .expect("all ordered and unordered seed controls check first");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start >= theorem_span.start
                && span.end <= theorem_span.end
                && span.start < span.end => {}
            other => panic!(
                "false structural claim on {expected_left}/{expected_right} must refuse at its theorem, not setup: {other:?}"
            ),
        }
    }
}
