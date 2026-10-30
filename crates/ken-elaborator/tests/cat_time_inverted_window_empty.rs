//! Empty equal and inverted windows: spec/50-stdlib/69-time-and-events.md §9.
//! Promise class: durable invariant. An open, premise-bearing public-law
//! client and a closed trace distinguish `[4,4)`/`[7,4)` from `[4,5)`.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const TRACE: &str = "Data.Time.EventTrace";
const PRIVATE_HELPERS: [&str; 3] = [
    "event_in_window_inverted",
    "event_in_window_past",
    "window_keep",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("§9 EventTrace package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Time.EventTrace (Event, trace_window, trace_window_inverted_empty)

theorem generic_empty_window
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : Equal Bool (leq_nat hi lo) True
      → Equal (List (Event p)) (trace_window p lo hi tr) (Nil (Event p)) =
  trace_window_inverted_empty p lo hi tr
"#;

/// MEASURED: EventTrace owns the public law and private helper as Transparent
/// kernel-checked declarations, and a selective open client consumes the
/// exact `hi ≤ lo` premise and structural Nil result without trace order.
/// The exact inherited trusted GlobalId set is unchanged on root-load; all
/// three named helpers refuse selective import. CLAIMED: arbitrary finite
/// traces, including unordered ones, have a checked empty-window proof when
/// `hi ≤ lo`, without added local trust or exported scaffolding. THE GAP:
/// imported providers retain their existing trust; a closed fixture below
/// tests values but cannot establish the generic proposition by itself.
#[test]
fn generic_inverted_window_law_is_checked_public_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("checked filter provider roots-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("§9 EventTrace proof and helper kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§9 EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§9 introduces no trusted GlobalId");
    let law = env.globals[&format!("{TRACE}.trace_window_inverted_empty")];
    assert!(owned.contains(&law), "EventTrace owns the public §9 law");
    assert!(
        matches!(env.env.lookup(law), Some(Decl::Transparent { .. })),
        "the public law has a checked body"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client consumes the exact sole premise and result type");
    for helper in PRIVATE_HELPERS {
        let id = env.globals[&format!("{TRACE}.{helper}")];
        assert!(owned.contains(&id), "EventTrace owns {helper}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{helper} remains a checked declaration"
        );
        match env.elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{TRACE}.{helper}"));
            }
            other => panic!("{helper} must stay private: {other:?}"),
        }
    }
}

// Package-owned fixture, distinct from the separately owned §9 seed.
// tr=[e1,e4,e7] with times 1,4,7; unordered=[e7,e1,e4].
const CLOSED_CLIENT: &str = r#"
import Data.Time.EventTrace
  (Event, MkEvent, event_before, event_in_window, trace_window,
   trace_window_inverted_empty)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6

const e1 : Event Bool = MkEvent Bool t1 True
const e4 : Event Bool = MkEvent Bool t4 False
const e7 : Event Bool = MkEvent Bool t7 True
const tr : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e4 (Cons (Event Bool) e7 (Nil (Event Bool))))
const unordered : List (Event Bool) =
  Cons (Event Bool) e7
    (Cons (Event Bool) e1 (Cons (Event Bool) e4 (Nil (Event Bool))))
const only4 : List (Event Bool) = Cons (Event Bool) e4 (Nil (Event Bool))

theorem equal_bounds_law :
  Equal (List (Event Bool)) (trace_window Bool t4 t4 tr) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t4 t4 tr Proved

theorem inverted_bounds_law :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 tr) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t7 t4 tr Proved

theorem unordered_inverted_law :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 unordered) (Nil (Event Bool)) =
  trace_window_inverted_empty Bool t7 t4 unordered Proved

theorem equal_bounds_value :
  Equal (List (Event Bool)) (trace_window Bool t4 t4 tr) (Nil (Event Bool)) = Proved

theorem inverted_bounds_value :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 tr) (Nil (Event Bool)) = Proved

theorem unordered_inverted_value :
  Equal (List (Event Bool)) (trace_window Bool t7 t4 unordered) (Nil (Event Bool)) = Proved

theorem unordered_head_descends : Equal Bool (event_before Bool e7 e1) False = Proved

theorem keeps_four_predicate : Equal Bool (event_in_window Bool t4 t5 e4) True = Proved

theorem nonempty_window_value :
  Equal (List (Event Bool)) (trace_window Bool t4 t5 tr) only4 = Refl
"#;

const FALSE_NONEMPTY_NIL: &str = r#"
theorem refuse_nonempty_nil :
  Equal (List (Event Bool)) (trace_window Bool t4 t5 tr) (Nil (Event Bool)) = Proved
"#;

/// MEASURED: the complete accepted client applies the public law to an
/// equal and an inverted interval on the three-event trace and to an
/// inverted interval on its demonstrably unordered permutation. The same
/// `Proved` form checks both true Nil values; an independent `[4,5)` value
/// and predicate check show that e4 remains, and an isolated false Nil claim
/// with `Proved` then rejects at its own kernel type mismatch after a fresh
/// accepted prefix. CLAIMED: `hi ≤ lo`, including equality, is the exact
/// emptiness boundary, not merely an ordering or proof-terminal accident.
/// THE GAP: these Bool-payload witnesses do not independently prove the
/// generic theorem or constitute execution of the CV-owned conformance seed.
#[test]
fn equal_and_inverted_values_accept_but_nonempty_nil_refuses() {
    let mut env = trace_env();
    env.elaborate_file(CLOSED_CLIENT)
        .expect("closed laws, same-form Nil controls, unordered and nonempty values check");
    let parsed = parse_decls(FALSE_NONEMPTY_NIL).expect("isolated §9 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("exactly one isolated refused theorem");
    };
    assert_eq!(name, "refuse_nonempty_nil");
    assert!(params.is_empty(), "refusal is closed over the fixture");
    let mut rejecting = trace_env();
    rejecting
        .elaborate_file(CLOSED_CLIENT)
        .expect("fresh accepted fixture checks before refusal");
    match rejecting.elaborate_file(FALSE_NONEMPTY_NIL) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("false [4,5) Nil must reject in own theorem: {other:?}"),
    }
}
