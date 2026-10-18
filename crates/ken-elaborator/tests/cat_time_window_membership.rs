//! Window membership: spec/50-stdlib/69-time-and-events.md §2.
//! Source: conformance/stdlib/time-and-events/seed-window-membership.md.
//! Promise class: durable invariant. The real seed and independent generic
//! client check inclusion, strict exclusion, and typed false-premise refusal.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr, KenMdFenceRole,
};
use ken_kernel::{Decl, KernelError};

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-window-membership.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §2 roots-loads from Derived's public filter laws");
    env
}

// An independent public client restates both §2.3 types at an arbitrary
// payload type and list; neither type has a payload comparison or ordering
// premise. The And is the prelude's Ω-valued conjunction, not Core.Logic.And.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, event_time, trace_window,
   trace_window_member_sound, trace_window_member_complete)

theorem generic_window_sound
  (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) (e : Event p) :
  list_elem (Event p) e (trace_window p lo hi tr) →
  And (list_elem (Event p) e tr)
      (And (Equal Bool (leq_nat lo (event_time p e)) True)
           (Equal Bool (lt_nat (event_time p e) hi) True)) =
  trace_window_member_sound p lo hi tr e

theorem generic_window_complete
  (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) (e : Event p) :
  list_elem (Event p) e tr →
  Equal Bool (leq_nat lo (event_time p e)) True →
  Equal Bool (lt_nat (event_time p e) hi) True →
  list_elem (Event p) e (trace_window p lo hi tr) =
  trace_window_member_complete p lo hi tr e
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §§2.2–2.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: roots-load explicit providers, Derived and EventTrace; exact
/// trusted-GlobalId sets stay equal across each owner load. Both Derived
/// filter laws and both EventTrace window laws are owner-local transparent
/// checked bodies; a separate client restates the two generic public types.
/// CLAIMED: the §2 universal proofs are public and add no new trust.
/// THE GAP: inherited provider assumptions remain; closed boundary behavior
/// and a false-premise refusal are independently exercised by the seed below.
#[test]
fn generic_window_membership_laws_check_without_new_trust() {
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
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived's generic filter-membership proofs check");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        before,
        "Derived may add no trusted GlobalId"
    );
    for law in ["list_elem_filter_sound", "list_elem_filter_complete"] {
        let id = env.globals[&format!("{DERIVED}.{law}")];
        assert!(derived_owned.contains(&id), "{law} belongs to Derived");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked transparent proof body"
        );
    }
    let trace_owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §2 checks against the public filter laws");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "EventTrace may add no trusted GlobalId");
    for law in ["trace_window_member_sound", "trace_window_member_complete"] {
        let id = env.globals[&format!("{TRACE}.{law}")];
        assert!(trace_owned.contains(&id), "{law} belongs to EventTrace");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must have a checked transparent proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent arbitrary-payload §2.3 client types check");
}

// A separate, test-owned upper-bound pair isolates the last argument of
// completeness. Supplying a hypothetical true bound checks the whole call
// with identical source-member and lower-bound witnesses; replacing only
// that upper hypothesis with Proved must kernel-reject. No axiom is added.
const UPPER_BOUND_CONTROL: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Collections.Derived (list_elem, list_elem_head)
import Data.Time.EventTrace
  (Event, MkEvent, event_time, trace_window, trace_window_member_complete)
const boundary_lo : Nat = Zero
const boundary_hi : Nat = Suc Zero
const boundary_event : Event Bool = MkEvent Bool boundary_hi True
const boundary_input : List (Event Bool) =
  Cons (Event Bool) boundary_event (Nil (Event Bool))
theorem input_contains_boundary :
  list_elem (Event Bool) boundary_event boundary_input =
  list_elem_head (Event Bool) boundary_event (Nil (Event Bool))
theorem lower_bound_holds :
  Equal Bool (leq_nat boundary_lo (event_time Bool boundary_event)) True = Proved
theorem strict_upper_bound_fails :
  Equal Bool (lt_nat (event_time Bool boundary_event) boundary_hi) False = Proved
theorem conditional_upper_bound
  (upper : Equal Bool (lt_nat (event_time Bool boundary_event) boundary_hi) True) :
  list_elem (Event Bool) boundary_event
    (trace_window Bool boundary_lo boundary_hi boundary_input) =
  trace_window_member_complete Bool boundary_lo boundary_hi boundary_input
    boundary_event input_contains_boundary lower_bound_holds upper
"#;

const FALSE_UPPER_BOUND: &str = r#"
theorem impossible_upper_bound_membership :
  list_elem (Event Bool) boundary_event
    (trace_window Bool boundary_lo boundary_hi boundary_input) =
  trace_window_member_complete Bool boundary_lo boundary_hi boundary_input
    boundary_event input_contains_boundary lower_bound_holds Proved
"#;

fn source_fences() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("conformance Ken fences are well-formed")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

// The false proof is the last argument of completeness, after its source
// member and lower-bound arguments. K0004 spans the theorem's stated goal
// through its proof application, not just the last token. Inspect the proof
// structurally rather than pinning a seed line or incidental theorem name.
fn assert_false_upper_premise(source: &str, actual: ken_elaborator::Span) {
    let decls = parse_decls(source).expect("negative Ken fence parses");
    let application = decls.into_iter().find_map(|decl| {
        let SourceDecl::TheoremDecl {
            body,
            span: theorem_span,
            ..
        } = decl
        else {
            return None;
        };
        let Expr::EApp(_, argument, application_span) = body else {
            return None;
        };
        let Expr::ECon(name, final_span) = *argument else {
            return None;
        };
        (name == "Proved" && final_span.end == application_span.end).then_some(theorem_span)
    });
    let span = application.expect("false upper-bound premise ends a theorem proof");
    assert_eq!(
        actual, span,
        "K0004 must identify the false theorem ending in its strict bound"
    );
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-membership.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: every positive Ken source fence is elaborated against the
/// shipped package, including the independently spelled Bool-payload exact
/// window, function-payload lower/upper controls and both law instances.
/// A false source fence is individually submitted after its positive prefix;
/// KernelRejected(TypeMismatch) spans the false theorem goal and application
/// ending in its strict-bound proof, not the child argument alone. A separate
/// positive control checks the same call under a hypothetical upper bound;
/// its paired last-argument Proved substitution is kernel-rejected.
/// CLAIMED: windows include `lo`, exclude `hi`, and filter membership is
/// equivalent to source membership with both bounds, without payload DecEq.
/// THE GAP: K0004 spans the whole theorem, not its final child premise; the
/// paired hypothetical-bound control isolates that premise, but finite
/// clients do not prove generic laws. Generic proof bodies are checked above.
#[test]
fn seed_checks_half_open_membership_and_rejects_false_upper_bound() {
    let mut positive = String::new();
    let mut refused = 0usize;
    for source in source_fences() {
        let mut env = trace_env();
        env.elaborate_file(&positive)
            .expect("all prior seed positive fences must check before this case");
        match env.elaborate_file(source) {
            Ok(_) => {
                positive.push_str(source);
                positive.push('\n');
            }
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                assert_false_upper_premise(source, span);
                refused += 1;
            }
            other => panic!("seed fence must check or reject at its false proof: {other:?}"),
        }
    }
    assert!(
        !positive.is_empty(),
        "seed must exercise a checked positive client"
    );
    assert!(
        refused > 0,
        "seed must exercise its false upper-bound premise"
    );

    let mut independent = trace_env();
    independent
        .elaborate_file(UPPER_BOUND_CONTROL)
        .expect("source member and lower bound check under assumed upper fact");
    match independent.elaborate_file(FALSE_UPPER_BOUND) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => assert_false_upper_premise(FALSE_UPPER_BOUND, span),
        other => panic!("false upper-bound substitution must kernel-reject: {other:?}"),
    }
}
