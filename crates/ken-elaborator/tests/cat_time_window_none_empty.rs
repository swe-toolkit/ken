//! Empty windows without in-range members: spec/50-stdlib/69-time-and-events.md
//! §14. Promise class: durable invariant. A generic checked law and a closed
//! accepted/refused pair isolate the time-1 event's miss premise.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const TRACE: &str = "Data.Time.EventTrace";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace with the checked no-member law roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, event_in_window, trace_window, trace_window_none_empty)

theorem none_client
      (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p))
    : ((e : Event p) → list_elem (Event p) e tr → Equal Bool (event_in_window p lo hi e) False)
      → Equal (List (Event p)) (trace_window p lo hi tr) (Nil (Event p)) =
  trace_window_none_empty p lo hi tr
"#;

fn closed_client(lo: &str) -> String {
    format!(
        r#"
import Core.Logic.Transport (cong, trans)
import Core.Logic.Or (Or, Inl, Inr)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, MkEvent, event_in_window, trace_window, trace_window_none_empty)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4

const e1 : Event Bool = MkEvent Bool t1 True
const e5 : Event Bool = MkEvent Bool t5 False
const tr : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e5 (Nil (Event Bool)))

theorem e1_misses : Equal Bool (event_in_window Bool {lo} t5 e1) False = Proved

theorem e5_misses : Equal Bool (event_in_window Bool {lo} t5 e5) False = Proved

theorem every_member_misses :
  (e : Event Bool) → list_elem (Event Bool) e tr →
    Equal Bool (event_in_window Bool {lo} t5 e) False =
  λe. λm0.
    elim_trunc
      (Equal Bool (event_in_window Bool {lo} t5 e) False)
      (λc0. match c0 {{
        Inl s0 ↦
          trans Bool (event_in_window Bool {lo} t5 e)
            (event_in_window Bool {lo} t5 e1) False
            (cong (Event Bool) Bool e e1
              (event_in_window Bool {lo} t5) s0)
            e1_misses;
        Inr m1 ↦
          elim_trunc
            (Equal Bool (event_in_window Bool {lo} t5 e) False)
            (λc1. match c1 {{
              Inl s1 ↦
                trans Bool (event_in_window Bool {lo} t5 e)
                  (event_in_window Bool {lo} t5 e5) False
                  (cong (Event Bool) Bool e e5
                    (event_in_window Bool {lo} t5) s1)
                  e5_misses;
              Inr m2 ↦ absurd m2
            }})
            m1
      }})
      m0

theorem empty_by_public_law :
  Equal (List (Event Bool)) (trace_window Bool {lo} t5 tr) (Nil (Event Bool)) =
  trace_window_none_empty Bool {lo} t5 tr every_member_misses
"#
    )
}

/// MEASURED: root-loading EventTrace preserves the exact trusted GlobalId
/// identity set and owns a Transparent public theorem; an open generic client
/// consumes precisely the membership-quantified miss and Nil conclusion.
/// CLAIMED: the generic window law adds no trusted assumption and can be used
/// without a bound-order or trace-order premise. THE GAP: inherited provider
/// trust remains; the finite witness below does not establish the general law.
#[test]
fn generic_none_empty_law_is_owned_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("Derived provider root-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace with the no-member law kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace §14 trusted GlobalIds {} -> {} equal {}",
        before.len(),
        after.len(),
        before == after
    );
    assert_eq!(
        after, before,
        "the no-member law introduces no trusted GlobalId"
    );
    let law = env.globals[&format!("{TRACE}.trace_window_none_empty")];
    assert!(owned.contains(&law), "EventTrace owns the no-member law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    let opaque = owned
        .iter()
        .filter(|id| !matches!(env.env.lookup(**id), Some(Decl::Transparent { .. })))
        .count();
    eprintln!(
        "EventTrace §14 owned {} non-transparent {}",
        owned.len(),
        opaque
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client consumes the exact public type");
    eprintln!("EventTrace §14 generic client accepted");
}

/// MEASURED: at [2,5), each member independently proves exclusion and the
/// public law proves Nil. At [1,5), the same-form client's time-1 miss lemma
/// rejects with a kernel type mismatch at that lemma; the time-5 proof has
/// the identical body in both variants. CLAIMED: a non-inverted empty window
/// depends on every member actually missing, not just on proof syntax or the
/// public-law application. THE GAP: this fixed fixture does not enumerate
/// every trace; the generic law supplies that checked generalization.
#[test]
fn excluded_trace_accepts_and_included_head_refuses_at_its_miss_lemma() {
    let mut env = trace_env();
    env.elaborate_file(&closed_client("t2"))
        .expect("[2,5) excludes both events and the public law proves Nil");
    env.elaborate_file(
        "theorem empty_value : Equal (List (Event Bool)) (trace_window Bool t2 t5 tr) (Nil (Event Bool)) = Proved",
    )
    .expect("[2,5) computes the empty list independently of the public law");
    env.elaborate_file(
        "theorem nonempty_value : Equal (List (Event Bool)) (trace_window Bool t1 t5 tr) (Cons (Event Bool) e1 (Nil (Event Bool))) = Refl",
    )
    .expect("[1,5) computes the included head independently of the failed lemma");
    let refused = closed_client("t1");
    let e1_span = parse_decls(&refused)
        .expect("closed [1,5) client parses")
        .into_iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == "e1_misses" => Some(span),
            _ => None,
        })
        .expect("e1 miss lemma exists");
    let mut env = trace_env();
    match env.elaborate_file(&refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if e1_span.start <= span.start && span.start < span.end && span.end <= e1_span.end => {}
        other => panic!("[1,5) must refuse at the time-1 miss lemma: {other:?}"),
    }
}
