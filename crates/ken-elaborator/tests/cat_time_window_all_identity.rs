//! All-in-window trace identity: spec/50-stdlib/69-time-and-events.md §10.
//! Promise class: durable invariant. An open law client and independent closed
//! fixture distinguish an all-included trace from one losing its middle event.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::Decl;

const TRACE: &str = "Data.Time.EventTrace";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("§10 EventTrace package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace (Event, event_in_window, trace_window, trace_window_all_identity)

theorem generic_window_identity
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
      (all : (e : Event p) → list_elem (Event p) e tr → Equal Bool (event_in_window p lo hi e) True)
    : Equal (List (Event p)) (trace_window p lo hi tr) tr =
  trace_window_all_identity p lo hi tr all
"#;

/// MEASURED: the package owns a Transparent public theorem, and an open client
/// applies its full membership-to-actual-predicate premise to structural list
/// equality without an order or bound premise. Root-loading the new law leaves
/// the trusted GlobalId identity set unchanged; its proof scaffolding refuses
/// selective import. CLAIMED: the generic all-in-window identity is reusable
/// without new trust or publicly exposing its private helper. THE GAP:
/// inherited provider trust remains; the closed fixture below alone cannot
/// establish the quantified property without the checked generic proof.
#[test]
fn generic_all_in_window_identity_is_checked_public_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Core.Logic.Or",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("§10 generic law kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§10 EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§10 adds no trusted GlobalId");
    let id = env.globals[&format!("{TRACE}.trace_window_all_identity")];
    assert!(owned.contains(&id), "EventTrace owns the §10 public law");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open client consumes exact membership premise and list identity");
    for helper in ["window_keep", "list_elem_head"] {
        match trace_env().elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{TRACE}.{helper}"));
            }
            other => panic!("{helper} must not be re-exported: {other:?}"),
        }
    }
}

// Package-owned [2,6) fixture: the shorter [2,5) window drops e5 but keeps
// the later e3, so retention is not merely a prefix of the source list.
const CLOSED_CLIENT: &str = r#"
import Core.Logic.Transport (cong, trans)
import Core.Logic.Or (Or, Inl, Inr)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace (Event, MkEvent, event_in_window, trace_window, trace_window_all_identity)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const e2 : Event Bool = MkEvent Bool t2 True
const e5 : Event Bool = MkEvent Bool t5 False
const e3 : Event Bool = MkEvent Bool t3 True
const tr : List (Event Bool) =
  Cons (Event Bool) e2 (Cons (Event Bool) e5 (Cons (Event Bool) e3 (Nil (Event Bool))))

theorem all_in_2_6 :
  (e : Event Bool) → list_elem (Event Bool) e tr → Equal Bool (event_in_window Bool t2 t6 e) True =
  λe. λm0.
    elim_trunc
      (Equal Bool (event_in_window Bool t2 t6 e) True)
      (λc0. match c0 {
        Inl s0 ↦
          trans Bool (event_in_window Bool t2 t6 e) (event_in_window Bool t2 t6 e2) True
            (cong (Event Bool) Bool e e2 (event_in_window Bool t2 t6) s0) Proved;
        Inr m1 ↦
          elim_trunc
            (Equal Bool (event_in_window Bool t2 t6 e) True)
            (λc1. match c1 {
              Inl s1 ↦
                trans Bool (event_in_window Bool t2 t6 e) (event_in_window Bool t2 t6 e5) True
                  (cong (Event Bool) Bool e e5 (event_in_window Bool t2 t6) s1) Proved;
              Inr m2 ↦
                elim_trunc
                  (Equal Bool (event_in_window Bool t2 t6 e) True)
                  (λc2. match c2 {
                    Inl s2 ↦
                      trans Bool (event_in_window Bool t2 t6 e) (event_in_window Bool t2 t6 e3) True
                        (cong (Event Bool) Bool e e3 (event_in_window Bool t2 t6) s2) Proved;
                    Inr m3 ↦ absurd m3
                  })
                  m2
            })
            m1
      })
      m0

theorem identity_by_public_law :
  Equal (List (Event Bool)) (trace_window Bool t2 t6 tr) tr =
  trace_window_all_identity Bool t2 t6 tr all_in_2_6

theorem identity_by_refl :
  Equal (List (Event Bool)) (trace_window Bool t2 t6 tr) tr = Refl

theorem shorter_window_value :
  Equal (List (Event Bool)) (trace_window Bool t2 t5 tr)
    (Cons (Event Bool) e2 (Cons (Event Bool) e3 (Nil (Event Bool)))) = Refl
"#;

const FALSE_SHORTER_IDENTITY: &str = r#"
theorem refuse_shorter_identity :
  Equal (List (Event Bool)) (trace_window Bool t2 t5 tr) tr = Refl
"#;

/// MEASURED: a closed premise proves every original member satisfies [2,6),
/// both the public law and a same-form `Refl` accept its unchanged list, and
/// [2,5) keeps the first and last events while excluding the middle one. A
/// false `Refl` identity for that same trace and shorter bound refuses at its
/// own theorem after a fresh accepted prefix. CLAIMED: the whole-trace
/// identity depends on inclusion of every member, including a middle one;
/// the boundary is not an invalid proof form. THE GAP: this finite fixture
/// cannot replace the generic proof or execute the separately owned CV seed.
#[test]
fn covered_trace_accepts_and_shorter_window_identity_refuses() {
    trace_env()
        .elaborate_file(CLOSED_CLIENT)
        .expect("closed premise, true law, true Refl and shorter value check");
    let parsed = parse_decls(FALSE_SHORTER_IDENTITY).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        span: claim_span,
        ..
    }] = parsed.as_slice()
    else {
        panic!("exactly one isolated refused theorem");
    };
    assert_eq!(name, "refuse_shorter_identity");
    assert!(params.is_empty(), "refusal is closed over the fixture");
    let mut rejecting = trace_env();
    rejecting
        .elaborate_file(CLOSED_CLIENT)
        .expect("fresh accepted prefix checks before isolated refusal");
    match rejecting.elaborate_file(FALSE_SHORTER_IDENTITY) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim_span.start <= span.start
                && span.start < span.end
                && span.end <= claim_span.end => {}
        other => panic!("false [2,5) trace identity must refuse at its Refl: {other:?}"),
    }
}
