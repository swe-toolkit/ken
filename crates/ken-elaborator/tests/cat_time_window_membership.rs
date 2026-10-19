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

struct CompletenessCase {
    theorem_span: ken_elaborator::Span,
    upper_true: String,
    upper_false: String,
    event_at_lo: String,
    event_at_hi: String,
    assumed_upper: String,
}

// Inspect the application AST, not fence positions or declaration spellings.
// The last completeness argument is a claimed proof of the strict upper
// predicate. Form closed obligations from its actual payload, event and bound
// arguments; the Ken checker, not Rust text comparison, decides each fact.
fn completeness_cases(source: &str) -> Vec<CompletenessCase> {
    let decls = parse_decls(source).expect("seed Ken fence parses");
    let mut cases = Vec::new();
    for decl in decls {
        let SourceDecl::TheoremDecl {
            params,
            theorem,
            body,
            span: theorem_span,
            ..
        } = decl
        else {
            continue;
        };
        let mut args = Vec::new();
        let mut head = &body;
        while let Expr::EApp(function, argument, _) = head {
            args.push(argument.as_ref());
            head = function;
        }
        args.reverse();
        let Expr::EVar(name, _) = head else {
            continue;
        };
        if name != "trace_window_member_complete" {
            continue;
        }
        assert!(params.is_empty(), "seed completeness cases must be closed");
        let [p, lo, hi, _, event, _, _, last] = args.as_slice() else {
            panic!("seed completeness call must supply its eight arguments");
        };
        let Expr::ECon(name, final_span) = last else {
            panic!("seed completeness instance must close its final proof");
        };
        assert_eq!(name, "Proved", "seed last premise must be explicit");
        let snippet = |expr: &Expr| &source[expr.span().start..expr.span().end];
        let event_time = format!("event_time ({}) ({})", snippet(p), snippet(event));
        let upper = format!("lt_nat ({event_time}) ({})", snippet(hi));
        let bound = format!("Equal Bool ({upper})");
        let upper_true = format!("{bound} True");
        let assumed_upper = format!(
            "theorem seed_upper_hypothesis (upper_fact : {upper_true}) : {} = {}upper_fact",
            &source[theorem.span().start..theorem.span().end],
            &source[body.span().start..final_span.start],
        );
        cases.push(CompletenessCase {
            theorem_span,
            upper_true,
            upper_false: format!("{bound} False"),
            event_at_lo: format!("Equal Nat ({event_time}) ({})", snippet(lo)),
            event_at_hi: format!("Equal Nat ({event_time}) ({})", snippet(hi)),
            assumed_upper,
        });
    }
    cases
}

fn closed_fact_checks(prefix: &str, proposition: &str, witness: &str) -> bool {
    let mut env = trace_env();
    env.elaborate_file(prefix)
        .expect("the prefix of positive seed cases checks");
    let probe = format!("theorem seed_closed_fact : {proposition} = {witness}");
    match env.elaborate_file(&probe) {
        Ok(_) => true,
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => false,
        other => panic!("closed seed fact `{probe}` must check or refute: {other:?}"),
    }
}

// K0004 spans the theorem rather than the failing child. Match the AST's
// theorem span; the checked true/false bound and assumed-upper control below
// isolate the final argument as the cause instead of trusting a broad span.
fn assert_false_upper_premise(source: &str, actual: ken_elaborator::Span) {
    let decls = parse_decls(source).expect("negative Ken fence parses");
    assert_eq!(
        decls.len(),
        1,
        "negative fence must contain only its refusal"
    );
    let mut cases = completeness_cases(source);
    assert_eq!(cases.len(), 1, "negative theorem uses completeness");
    let proof = cases.pop().expect("one negative completeness case");
    assert_eq!(actual, proof.theorem_span, "K0004 must be this theorem");
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-membership.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: each source fence is classified by checking its closed strict
/// upper-bound fact and endpoint equality, built from parsed application
/// arguments. Every non-refusal fence must elaborate successfully; a checked
/// completeness instance must exercise the inclusive lower endpoint. For
/// an upper-endpoint refusal, the same seed application checks when its final
/// false `Proved` is replaced by a hypothetical upper-bound witness; the
/// original is kernel-rejected at its own theorem. An independent paired
/// upper-bound control reinforces the last-argument discrimination.
/// CLAIMED: windows include `lo`, exclude `hi`, and filter membership is
/// equivalent to source membership with both bounds, without payload DecEq.
/// THE GAP: K0004 spans the whole theorem, not its final child premise;
/// only the checked seed-specific hypothetical-bound contrast isolates that
/// child. Finite clients do not prove generic laws; checked bodies are above.
#[test]
fn seed_checks_half_open_membership_and_rejects_false_upper_bound() {
    let mut positive = String::new();
    let mut refused = 0usize;
    let mut inclusive_lower_complete = false;
    for source in source_fences() {
        let instances = completeness_cases(source);
        let upper_endpoints = instances
            .iter()
            .filter(|case| {
                closed_fact_checks(&positive, &case.event_at_hi, "Proved")
                    && closed_fact_checks(&positive, &case.upper_false, "Proved")
            })
            .collect::<Vec<_>>();
        if !upper_endpoints.is_empty() {
            assert_eq!(
                upper_endpoints.len(),
                1,
                "each refused seed application needs its own diagnostic"
            );
            let case = upper_endpoints[0];
            assert!(
                !closed_fact_checks(&positive, &case.upper_true, "Proved"),
                "strict upper bound at the endpoint must not be true"
            );
            let mut env = trace_env();
            env.elaborate_file(&positive)
                .expect("all earlier positive seed fences check");
            env.elaborate_file(&case.assumed_upper)
                .expect("the seed source and lower witnesses check under upper hypothesis");
            let mut env = trace_env();
            env.elaborate_file(&positive)
                .expect("all earlier positive seed fences check");
            match env.elaborate_file(source) {
                Err(ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    span,
                }) => assert_false_upper_premise(source, span),
                other => panic!("false seed strict bound must kernel-reject: {other:?}"),
            }
            refused += 1;
        } else {
            let mut env = trace_env();
            env.elaborate_file(&positive)
                .expect("all earlier positive seed fences check");
            env.elaborate_file(source)
                .expect("every seed case not proved false at its upper endpoint must check");
            inclusive_lower_complete |= instances.iter().any(|case| {
                closed_fact_checks(&positive, &case.event_at_lo, "Proved")
                    && closed_fact_checks(&positive, &case.upper_true, "Proved")
            });
            positive.push_str(source);
            positive.push('\n');
        }
    }
    assert!(
        inclusive_lower_complete,
        "seed must check a completeness client at the inclusive lower endpoint"
    );
    assert!(
        refused > 0,
        "seed must refuse a false strict upper-bound proof"
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
