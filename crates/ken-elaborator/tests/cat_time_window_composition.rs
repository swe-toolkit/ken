//! Window composition and merge units: spec/50-stdlib/69-time-and-events.md §5.
//! Source: conformance/stdlib/time-and-events/seed-window-composition.md.
//! Promise class: durable invariant. Checked generic proofs and closed controls
//! are independent; the union-bounds refusal shares the accepted fixtures.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-window-composition.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §5 roots-loads from the checked providers");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Numeric.Nat.Order (max, min)
import Data.Time.EventTrace
  (Event, trace_window, trace_merge, trace_window_window,
   trace_merge_nil_left, trace_merge_nil_right)

theorem generic_window_window
  (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat)
  (tr : List (Event p)) :
  Equal (List (Event p))
    (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
    (trace_window p (max lo1 lo2) (min hi1 hi2) tr) =
  trace_window_window p lo1 hi1 lo2 hi2 tr

theorem generic_nil_left (p : Type) (ys : List (Event p)) :
  Equal (List (Event p)) (trace_merge p (Nil (Event p)) ys) ys =
  trace_merge_nil_left p ys

theorem generic_nil_right (p : Type) (xs : List (Event p)) :
  Equal (List (Event p)) (trace_merge p xs (Nil (Event p))) xs =
  trace_merge_nil_right p xs
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §5.
/// PROMISE CLASS: durable invariant.
/// MEASURED: roots-load the real Derived provider, compare exact trusted
/// GlobalId sets before and after EventTrace loads, assert ownership and
/// Transparent bodies of all three exported laws, import and consume their
/// exact quantified types with arbitrary payload/bounds, and reject selective
/// imports of all seven new private proof helpers.
/// CLAIMED: the three §5 laws are publicly reusable checked proofs with no
/// additional trust or public proof scaffolding.
/// THE GAP: providers' inherited trust remains; a weakened statement could
/// retain a proof body, so the arbitrary-argument client checks the full types.
#[test]
fn generic_composition_and_units_are_public_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("generic filter provider roots-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §5 proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§5 trust identities: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "no new trusted GlobalId in EventTrace");
    for name in [
        "trace_window_window",
        "trace_merge_nil_left",
        "trace_merge_nil_right",
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(owned.contains(&id), "EventTrace owns {name}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all three exact §5 types work on arbitrary inputs");
    for helper in [
        "leq_nat_max_split",
        "lt_nat_min_split",
        "bool_and_interchange",
        "event_in_window_inter",
        "window_keep",
        "window_filter_fuse_cons",
        "window_filter_fuse",
    ] {
        let id = env.globals[&format!("{TRACE}.{helper}")];
        assert!(owned.contains(&id), "EventTrace owns {helper}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{TRACE}.{helper}"))
            }
            other => panic!("private {helper} must refuse selective import: {other:?}"),
        }
    }
}

fn seed_fences() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("§5 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-composition.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the complete positive seed checks against the roots-loaded
/// package, including its eight law-instance and independent exact-value
/// controls for nested, intersected, union, and both merge-unit expressions.
/// CLAIMED: the same three-event input observes the intersected one-event
/// list on both sides of §5, and the two empty merge units preserve the input.
/// THE GAP: one closed Bool-payload trace cannot prove any generic law;
/// the arbitrary-argument checked body/client are measured above.
#[test]
fn shipped_window_and_merge_unit_client_checks() {
    let fences = seed_fences();
    let [positive, _] = fences.as_slice() else {
        panic!("§5 seed has one accepted and one isolated refused Ken fence");
    };
    trace_env()
        .elaborate_file(positive)
        .expect("all shipped §5 law instances and exact-value controls check");
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected closed §5 seed proposition: {other:?}"),
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
    assert!(
        matches!(head, SourceType::TCon(found, _) | SourceType::TVar(found, _) if found == name),
        "expected {name}, found {head:?}"
    );
    args
}

fn equality(ty: &SourceType) -> [&SourceType; 3] {
    applied(ty, "Equal")
        .try_into()
        .expect("closed control or refusal is a three-argument equality")
}

fn control<'a>(decls: &'a [SourceDecl], name: &str) -> [&'a SourceType; 3] {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                ..
            } if actual == name => Some(equality(theorem)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [case] = found.as_slice() else {
        panic!("exactly one checked seed control {name} required");
    };
    *case
}

fn assert_union_is_the_wrong_bounds(
    nested: &SourceType,
    intersected: &SourceType,
    union: &SourceType,
) {
    let [p, lo2, hi2, inner]: [&SourceType; 4] = applied(nested, "trace_window")
        .try_into()
        .expect("nested outer window has four arguments");
    let [inner_p, lo1, hi1, tr]: [&SourceType; 4] = applied(inner, "trace_window")
        .try_into()
        .expect("nested inner window has four arguments");
    let [intersection_p, max_lo, min_hi, intersection_tr]: [&SourceType; 4] =
        applied(intersected, "trace_window")
            .try_into()
            .expect("intersection window has four arguments");
    let [union_p, min_lo, max_hi, union_tr]: [&SourceType; 4] = applied(union, "trace_window")
        .try_into()
        .expect("union window has four arguments");
    for other in [inner_p, intersection_p, union_p] {
        assert_eq!(shape(p), shape(other), "all payload types must agree");
    }
    for other in [intersection_tr, union_tr] {
        assert_eq!(shape(tr), shape(other), "all windows share the same trace");
    }
    let [max1, max2]: [&SourceType; 2] = applied(max_lo, "max")
        .try_into()
        .expect("intersected lower bound takes max");
    let [min1, min2]: [&SourceType; 2] = applied(min_hi, "min")
        .try_into()
        .expect("intersected upper bound takes min");
    let [union1, union2]: [&SourceType; 2] = applied(min_lo, "min")
        .try_into()
        .expect("union lower bound takes min");
    let [union3, union4]: [&SourceType; 2] = applied(max_hi, "max")
        .try_into()
        .expect("union upper bound takes max");
    for (actual, expected) in [
        (max1, lo1),
        (max2, lo2),
        (min1, hi1),
        (min2, hi2),
        (union1, lo1),
        (union2, lo2),
        (union3, hi1),
        (union4, hi2),
    ] {
        assert_eq!(
            shape(actual),
            shape(expected),
            "same bounds, opposite extrema"
        );
    }
}

/// SOURCE: §5 seed's union-bounds refusal and accepted exact-value controls.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the accepted prefix kernel-checks on a fresh root-loaded
/// environment. The refused AST compares its *same nested expression* to
/// min-lower/max-upper, while the independently checked accepted controls
/// bind nested and max-lower/min-upper to one output, and min-lower/max-upper
/// to a different output. A separate rejected equal-output probe establishes
/// that the two expected closed lists do not reduce to the same list. The
/// appended refusal TypeMismatch-rejects at its own Refl, not in setup.
/// CLAIMED: union bounds cannot replace intersection bounds on this trace.
/// THE GAP: a finite counterexample excludes this union formula, not every
/// possible incorrect composition. The checked generic proof is above.
#[test]
fn shipped_union_bounds_refusal_matches_its_checked_value_controls() {
    let fences = seed_fences();
    let [positive, negative] = fences.as_slice() else {
        panic!("§5 seed has one accepted and one isolated refused Ken fence");
    };
    let positives = parse_decls(positive).expect("accepted §5 client parses");
    let negatives = parse_decls(negative).expect("refused §5 client parses");
    let [SourceDecl::TheoremDecl {
        params,
        theorem,
        body,
        span: theorem_span,
        ..
    }] = negatives.as_slice()
    else {
        panic!("refused §5 fence has exactly one theorem");
    };
    assert!(params.is_empty(), "refusal must be closed");
    assert!(
        matches!(body, SourceExpr::ECon(name, _) if name == "Refl"),
        "refusal attempts a structural equality proof"
    );
    let [carrier, nested, union] = equality(theorem);
    let [nested_carrier, checked_nested, one] = control(&positives, "nested_window_exact_value");
    let [inter_carrier, checked_inter, inter_one] =
        control(&positives, "intersection_window_exact_value");
    let [union_carrier, checked_union, three] = control(&positives, "union_window_exact_value");
    for control_carrier in [nested_carrier, inter_carrier, union_carrier] {
        assert_eq!(shape(carrier), shape(control_carrier));
    }
    assert_eq!(shape(nested), shape(checked_nested));
    assert_eq!(shape(union), shape(checked_union));
    assert_eq!(
        shape(one),
        shape(inter_one),
        "both true sides select one list"
    );
    assert_union_is_the_wrong_bounds(nested, checked_inter, union);

    let mut env = trace_env();
    env.elaborate_file(positive)
        .expect("entire accepted §5 seed and same-input controls check first");
    // These names are the two independently checked exact results above.
    // A direct mismatch probe rules out an aliased/vacuous fixture pair.
    let distinct_outputs =
        "theorem unequal_closed_outputs : Equal (List (Event Bool)) only4 tr = Refl";
    match env.elaborate_file(distinct_outputs) {
        Err(ElabError::TypeMismatch { reason, .. })
            if reason == "Refl expects an `Eq`-shaped goal" => {}
        other => panic!("controlled one- and three-event lists must differ: {other:?}"),
    }
    assert_ne!(
        shape(one),
        shape(three),
        "control endpoints must be distinct"
    );
    let mut refusing = trace_env();
    refusing
        .elaborate_file(positive)
        .expect("all same-input positive controls check on the refusal path");
    match refusing.elaborate_file(negative) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= theorem_span.start
                && span.end <= theorem_span.end
                && negative.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("false same-input union equality must reject at Refl: {other:?}"),
    }
}
