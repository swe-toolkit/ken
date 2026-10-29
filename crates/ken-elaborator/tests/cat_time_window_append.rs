//! Time-window append distribution: spec/50-stdlib/69-time-and-events.md §8.
//! Source: conformance/stdlib/time-and-events/seed-window-append-distribution.md.
//! Promise class: durable invariant. Generic checked clients and an unordered
//! closed fixture distinguish append order from swapped retained windows.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-window-append-distribution.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("checked Derived and EventTrace packages roots-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (filter, list_append)
import Data.Time.EventTrace (Event, trace_window, trace_window_append)

theorem generic_filter_append
      (a : Type) (keep : a → Bool) (xs : List a) (ys : List a)
    : Equal (List a)
        (filter a keep (list_append a xs ys))
        (list_append a (filter a keep xs) (filter a keep ys)) =
  filter::append a keep xs ys

theorem generic_window_append
      (p : Type) (lo : Nat) (hi : Nat) (xs : List (Event p)) (ys : List (Event p))
    : Equal (List (Event p))
        (trace_window p lo hi (list_append (Event p) xs ys))
        (list_append (Event p)
          (trace_window p lo hi xs)
          (trace_window p lo hi ys)) =
  trace_window_append p lo hi xs ys
"#;

/// MEASURED: the exact trusted GlobalId sets agree before and after loading
/// Derived and EventTrace over their preloaded checked providers. Both new
/// laws are owned by their respective packages and Transparent; open clients
/// consume their full types without order or bound premises. The one new
/// Derived helper is checked yet refuses selective import. CLAIMED: both
/// public laws are reusable with no new local trust or exported scaffolding.
/// THE GAP: inherited provider trust remains and closed clients below do
/// not establish the generic quantified proofs independently of the kernel.
#[test]
fn generic_filter_and_window_append_are_checked_public_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.And",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived filter append proof root-loads");
    let after_derived = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let trace_owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace window append law root-loads");
    let after_trace = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§8 trusted GlobalIds: {} → {} → {}",
        before.len(),
        after_derived.len(),
        after_trace.len()
    );
    assert_eq!(after_derived, before, "Derived adds no trusted GlobalId");
    assert_eq!(after_trace, before, "EventTrace adds no trusted GlobalId");
    for (module, name, owned) in [
        (DERIVED, "filter::append", &derived_owned),
        (DERIVED, "filter_keep_step_append", &derived_owned),
        (TRACE, "trace_window_append", &trace_owned),
    ] {
        let id = env.globals[&format!("{module}.{name}")];
        assert!(owned.contains(&id), "{module} owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open clients consume both complete public law types");
    match trace_env().elaborate_file(&format!("import {DERIVED} (filter_keep_step_append)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{DERIVED}.filter_keep_step_append"));
        }
        other => panic!("filter_keep_step_append must remain private: {other:?}"),
    }
}

// Select each source program by a theorem identity, not Markdown fence order.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§8 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let has = parse_decls(source)
            .expect("§8 seed fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if has {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must name exactly one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §8 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §8 proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §8 proof form: {other:?}"),
    }
}

fn theorem_shape(source: &str, theorem_name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§8 theorem parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                theorem,
                body,
                ..
            } if name == theorem_name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem named {theorem_name}");
    };
    one.clone()
}

const EXPECTED_ACCEPTED: &str = r#"
theorem expected_unordered : Equal Bool (event_before Bool e5 e1) False = Proved
theorem expected_xs_window : Equal (List (Event Bool)) (trace_window Bool t2 t6 xs) xs_window = Refl
theorem expected_ys_window : Equal (List (Event Bool)) (trace_window Bool t2 t6 ys) ys_window = Refl
theorem expected_appended : Equal (List (Event Bool))
  (trace_window Bool t2 t6 (list_append (Event Bool) xs ys)) appended_window = Refl
theorem expected_law : Equal (List (Event Bool))
  (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
  (list_append (Event Bool) (trace_window Bool t2 t6 xs) (trace_window Bool t2 t6 ys)) =
  trace_window_append Bool t2 t6 xs ys
theorem expected_same_shape : Equal (List (Event Bool))
  (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
  (list_append (Event Bool) (trace_window Bool t2 t6 xs) (trace_window Bool t2 t6 ys)) = Refl
theorem expected_swapped_start : Equal Nat (first_event_time swapped_windows) t3 = Proved
theorem expected_swapped_not_five :
  Equal Bool (leq_nat t5 (first_event_time swapped_windows)) False = Proved
"#;
const EXPECTED_REFL_REFUSAL: &str = r#"
theorem expected_neg : Equal (List (Event Bool))
  (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
  (list_append (Event Bool) (trace_window Bool t2 t6 ys) (trace_window Bool t2 t6 xs)) = Refl
"#;
const EXPECTED_LAW_REFUSAL: &str = r#"
theorem expected_neg : Equal (List (Event Bool))
  (trace_window Bool t2 t6 (list_append (Event Bool) xs ys))
  (list_append (Event Bool) (trace_window Bool t2 t6 ys) (trace_window Bool t2 t6 xs)) =
  trace_window_append Bool t2 t6 xs ys
"#;

const FIXTURE_CONTROLS: &str = r#"
import Data.Time.EventTrace (event_in_window)

theorem window_keeps_five : Equal Bool (event_in_window Bool t2 t6 e5) True = Proved
theorem window_keeps_three : Equal Bool (event_in_window Bool t2 t6 e3) True = Proved
theorem window_drops_one : Equal Bool (event_in_window Bool t2 t6 e1) False = Proved
theorem window_drops_eight : Equal Bool (event_in_window Bool t2 t6 e8) False = Proved

theorem unswapped_starts_at_five :
  Equal Nat (first_event_time appended_window) t5 = Proved
"#;

/// MEASURED: the entire accepted seed checks the unordered xs witness,
/// the two separate one-event windows, their ordered append, the public-law
/// application, an unswapped same-shape Refl, and an independent first-event
/// observation of the swap. Four direct predicate controls run on the seed's
/// actual events; the accepted append starts at five, the swapped at three.
/// Each isolated refusal has the same swapped goal but a different proof
/// body, and elaborates only its parsed closed theorem span after a fresh
/// prefix. No diagnostic variant or message is required by the seed.
/// CLAIMED: windowing an append keeps the input order, not a swapped order.
/// THE GAP: finite Bool traces do not establish the generic law or official
/// conformance; the open checked law/client above supplies generic evidence.
#[test]
fn complete_unordered_seed_accepts_and_both_swapped_claims_refuse() {
    let accepted = seed_case("trace_window_append_instance");
    let refl_refusal = seed_case("reject_swapped_window_append_by_refl");
    let law_refusal = seed_case("reject_swapped_window_append_by_public_law");
    for (actual, expected) in [
        ("xs_head_comparison_is_false", "expected_unordered"),
        ("xs_window_is_exact", "expected_xs_window"),
        ("ys_window_is_exact", "expected_ys_window"),
        ("append_window_is_exact", "expected_appended"),
        ("trace_window_append_instance", "expected_law"),
        ("unswapped_same_shape_refl_control", "expected_same_shape"),
        ("swapped_windows_start_at_three", "expected_swapped_start"),
        (
            "swapped_windows_do_not_start_at_five",
            "expected_swapped_not_five",
        ),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_ACCEPTED, expected),
            "accepted §8 obligation {actual} retains its proposition and proof"
        );
    }
    let mut positive = trace_env();
    positive
        .elaborate_file(accepted)
        .expect("complete §8 accepted source and unswapped control check");
    positive
        .elaborate_file(FIXTURE_CONTROLS)
        .expect("actual seed events retain two distinct ordered window heads");
    for name in [
        "trace_window_append_instance",
        "unswapped_same_shape_refl_control",
        "swapped_windows_do_not_start_at_five",
        "unswapped_starts_at_five",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    for source in [refl_refusal, law_refusal] {
        let parsed = parse_decls(source).expect("isolated §8 refusal parses");
        let [SourceDecl::TheoremDecl {
            params,
            span: claim_span,
            ..
        }] = parsed.as_slice()
        else {
            panic!("each refusal must contain exactly one theorem");
        };
        assert!(params.is_empty(), "swapped equality is closed");
        let claim = source
            .get(claim_span.start..claim_span.end)
            .expect("parsed theorem span is inside the refusal source");
        let claim_decls = parse_decls(claim).expect("isolated claim span parses");
        assert!(matches!(
            claim_decls.as_slice(),
            [SourceDecl::TheoremDecl { .. }]
        ));
        let mut refusing = trace_env();
        refusing
            .elaborate_file(accepted)
            .expect("fresh complete accepted seed checks before refusal");
        refusing
            .elaborate_file(FIXTURE_CONTROLS)
            .expect("fresh seed's opposite first-event controls check");
        refusing
            .elaborate_file(claim)
            .expect_err("the sole closed swapped theorem must be refused");
    }
    // Check fixture shape only after each isolated false claim reaches the
    // rejection assertion, so a true-neighbor mutation cannot be masked here.
    for (source, actual, expected) in [
        (
            refl_refusal,
            "reject_swapped_window_append_by_refl",
            EXPECTED_REFL_REFUSAL,
        ),
        (
            law_refusal,
            "reject_swapped_window_append_by_public_law",
            EXPECTED_LAW_REFUSAL,
        ),
    ] {
        assert_eq!(
            theorem_shape(source, actual),
            theorem_shape(expected, "expected_neg"),
            "swapped refusal {actual} retains its exact proposition and proof"
        );
    }
}
