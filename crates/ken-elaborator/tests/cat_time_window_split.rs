//! Adjacent half-open time windows: spec/50-stdlib/69-time-and-events.md §7.
//! Source: conformance/stdlib/time-and-events/seed-window-adjacent-split.md.
//! Promise class: durable invariant. A generic checked proof and a closed
//! nonempty two-window control distinguish a true split from swapped append.

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
    "/../../conformance/stdlib/time-and-events/seed-window-adjacent-split.md"
));
const PRIVATE_HELPERS: [&str; 9] = [
    "trace_window_split_cons",
    "trace_window_split_below",
    "trace_window_split_above",
    "trace_window_past",
    "trace_window_raise",
    "event_in_window_past",
    "event_in_window_raise",
    "lt_nat_reverse_leq_false",
    "window_keep_append",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §7 checked package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (list_append)
import Data.Time.EventTrace
  (Event, time_ordered, trace_window, trace_window_split)

theorem split_client
      (p : Type) (lo : Nat) (mid : Nat) (hi : Nat) (tr : List (Event p))
    : time_ordered p tr
      → Equal Bool (leq_nat lo mid) True
      → Equal Bool (leq_nat mid hi) True
      → Equal
          (List (Event p))
          (trace_window p lo hi tr)
          (list_append (Event p) (trace_window p lo mid tr) (trace_window p mid hi tr)) =
  trace_window_split p lo mid hi tr
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §7.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact trusted GlobalId identities before and after EventTrace
/// roots-load, a package-owned Transparent public law, nine package-owned
/// Transparent private helpers that each refuse selective import, and an
/// independent generic client consuming all three premises and the list
/// equality. CLAIMED: the generic checked split uses no new assumption and
/// exports no proof scaffolding. THE GAP: imported providers retain inherited
/// trust; the seed's closed values alone cannot establish this general law.
#[test]
fn generic_split_is_owned_checked_private_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("checked list_append and filter provider root-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("§7 split and private proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§7 EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§7 introduces no trusted GlobalId");
    let law = env.globals[&format!("{TRACE}.trace_window_split")];
    assert!(owned.contains(&law), "EventTrace owns the public §7 law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes the exact ordered and bounded type");
    for helper in PRIVATE_HELPERS {
        let id = env.globals[&format!("{TRACE}.{helper}")];
        assert!(owned.contains(&id), "EventTrace owns {helper}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{helper} is checked, not an Axiom"
        );
        match env.elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{TRACE}.{helper}"));
            }
            other => panic!("{helper} must stay unimportable: {other:?}"),
        }
    }
}

fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§7 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§7 seed source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §7 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §7 seed proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §7 seed proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("Ken §7 obligation parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem {name} required");
    };
    one.clone()
}

const EXPECTED_CONTROLS: &str = r#"
theorem accepted_instance :
  Equal (List (Event Bool)) (trace_window Bool t1 t9 tr)
    (list_append (Event Bool) (trace_window Bool t1 t3 tr) (trace_window Bool t3 t9 tr)) =
  trace_window_split Bool t1 t3 t9 tr tr_ordered Proved Proved

theorem accepted_same_shape :
  Equal (List (Event Bool)) (trace_window Bool t1 t9 tr)
    (list_append (Event Bool) (trace_window Bool t1 t3 tr) (trace_window Bool t3 t9 tr)) = Refl

theorem accepted_whole : Equal (List (Event Bool)) (trace_window Bool t1 t9 tr) tr = Refl

theorem accepted_left : Equal (List (Event Bool)) (trace_window Bool t1 t3 tr) left_value = Refl

theorem accepted_right : Equal (List (Event Bool)) (trace_window Bool t3 t9 tr) right_value = Refl

theorem accepted_midpoint : Equal Bool (event_in_window Bool t1 t3 e3) False = Proved
"#;
const EXPECTED_REFUSAL: &str = r#"
theorem refused_swap :
  Equal (List (Event Bool)) (trace_window Bool t1 t9 tr)
    (list_append (Event Bool) (trace_window Bool t3 t9 tr) (trace_window Bool t1 t3 tr)) = Refl
"#;

/// SOURCE: conformance/stdlib/time-and-events/seed-window-adjacent-split.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the whole accepted source executes against the root-loaded
/// package, including the public-law instance, three independent Refl window
/// values, same-shape Refl split and midpoint-exclusion Proved. The two
/// subwindow values are observably different; the exact swapped AST with Refl
/// then fails at its own Refl after a fresh accepted prefix. CLAIMED: the
/// adjacent windows append in order, not in reverse. THE GAP: one finite
/// ordered trace does not establish every generic case or premise necessity.
#[test]
fn complete_split_seed_accepts_and_swapped_append_refuses() {
    let accepted = seed_case("split_instance");
    let refused = seed_case("refuse_swapped_split");
    for (actual, expected) in [
        ("split_instance", "accepted_instance"),
        ("same_shape_control", "accepted_same_shape"),
        ("whole_value", "accepted_whole"),
        ("left_window_value", "accepted_left"),
        ("right_window_value", "accepted_right"),
        ("mid_event_excluded_left", "accepted_midpoint"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_CONTROLS, expected),
            "the accepted control {actual} must keep its exact §7 obligation"
        );
    }
    assert_eq!(
        theorem_shape(refused, "refuse_swapped_split"),
        theorem_shape(EXPECTED_REFUSAL, "refused_swap"),
        "refusal must reverse the two nonempty windows and attempt Refl"
    );
    let decls = parse_decls(refused).expect("isolated swap parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal is exactly one theorem");
    };
    assert_eq!(name, "refuse_swapped_split");
    assert!(params.is_empty(), "swapped claim is closed");
    let mut control = trace_env();
    control
        .elaborate_file(accepted)
        .expect("the entire §7 seed, its law instance and values check");
    match control.elaborate_file(
        "theorem unequal_split_values : Equal (List (Event Bool)) left_value right_value = Refl",
    ) {
        Err(ElabError::TypeMismatch { reason, .. })
            if reason == "Refl expects an `Eq`-shaped goal" => {}
        other => panic!("nonempty subwindows must be distinct: {other:?}"),
    }
    let mut refusing = trace_env();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks before refusal");
    match refusing.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= claim_span.start
                && span.end <= claim_span.end
                && refused.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("swapped window must fail at its own Refl: {other:?}"),
    }
}
