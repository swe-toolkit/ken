//! Spec 69 §21: merging singleton traces is insertion and insertion sort.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const TRACE: &str = "Data.Time.EventTrace";
const SINGLE: &str = "event_singleton";
const INSERT: &str = "trace_merge_single_insert";
const SORT: &str = "trace_merge_all_singletons_sort";
const HELPER: &str = "trace_merge_single_insert_cons";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads");
    env
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN_CLIENT: &str = r#"import Core.Logic.Transport (sym)
import Core.Classes.LawfulClasses (IsTrue, bool_or, leq_nat)
import Data.Collections.Derived (map, insert, sort, is_sorted)
import Data.Time.EventTrace
  (Event, event_time, event_before, time_ordered, trace_merge, trace_merge_all,
    event_singleton, trace_merge_single_insert, trace_merge_all_singletons_sort)

fn singles (p : Type) (tr : List (Event p)) : List (List (Event p)) =
  map (Event p) (List (Event p)) (event_singleton p) tr

theorem client_total (p : Type) (x : Event p) (y : Event p)
    : IsTrue (bool_or (event_before p x y) (event_before p y x)) =
  (proof total for leq_nat) (event_time p x) (event_time p y)

theorem client_sorted (p : Type) (tr : List (Event p))
    : time_ordered p (trace_merge_all p (singles p tr)) =
  J
    (λzs _. is_sorted (Event p) (event_before p) zs)
    ((proof sorted for sort) (Event p) (event_before p) (client_total p) tr)
    (sym
      (List (Event p))
      (trace_merge_all p (singles p tr))
      (sort (Event p) (event_before p) tr)
      (trace_merge_all_singletons_sort p tr))

theorem client_insert_sorted (p : Type) (x : Event p) (ys : List (Event p))
    : time_ordered p ys → time_ordered p (trace_merge p (Cons (Event p) x (Nil (Event p))) ys) =
  λsorted_ys.
    J
      (λzs _. is_sorted (Event p) (event_before p) zs)
      ((proof sorted for insert) (Event p) (event_before p) (client_total p) x ys sorted_ys)
      (sym
        (List (Event p))
        (trace_merge p (Cons (Event p) x (Nil (Event p))) ys)
        (insert (Event p) (event_before p) x ys)
        (trace_merge_single_insert p x ys))
"#;

const TIE_PREFIX: &str = r#"import Data.Collections.Derived (insert, map, sort)
import Data.Time.EventTrace
  (Event, MkEvent, event_before, event_singleton, trace_merge, trace_merge_all,
    trace_merge_single_insert, trace_merge_all_singletons_sort)

const t1 : Nat = Suc Zero
const first : Event Bool = MkEvent Bool t1 True
const second : Event Bool = MkEvent Bool t1 False
const early : Event Bool = MkEvent Bool Zero True
const left : List (Event Bool) = Cons (Event Bool) first (Nil (Event Bool))
const right : List (Event Bool) = Cons (Event Bool) second (Nil (Event Bool))
const tie_order : List (Event Bool) = Cons (Event Bool) first right

theorem merge_tie_value : Equal (List (Event Bool)) (trace_merge Bool left right) tie_order = Refl
theorem insert_tie_value
    : Equal (List (Event Bool)) (insert (Event Bool) (event_before Bool) first right) tie_order = Refl

theorem tie_keeps_first
    : Equal
        (List (Event Bool))
        (trace_merge Bool left right)
        (insert (Event Bool) (event_before Bool) first right) =
  trace_merge_single_insert Bool first right
"#;

const BATCH: &str = r#"const input : List (Event Bool) = Cons (Event Bool) first (Cons (Event Bool) early right)
const ordered : List (Event Bool) = Cons (Event Bool) early tie_order
const batch : List (List (Event Bool)) =
  map (Event Bool) (List (Event Bool)) (event_singleton Bool) input

theorem batch_value : Equal (List (Event Bool)) (trace_merge_all Bool batch) ordered = Refl

theorem batch_sort
    : Equal (List (Event Bool)) (trace_merge_all Bool batch) (sort (Event Bool) (event_before Bool) input) =
  trace_merge_all_singletons_sort Bool input
"#;

const SWAPPED: &str = r#"theorem tie_swapped
    : Equal
        (List (Event Bool))
        (trace_merge Bool left right)
        (insert (Event Bool) (event_before Bool) second left) =
  trace_merge_single_insert Bool first right
"#;

/// SOURCE: Spec 69 §21 and its checked design ruling.
/// MEASURED: exact provider-first trust identities, owning root's checked
/// Transparent declarations, and open clients that cite their actual Consts.
/// THE GAP: provider trust is inherited; this is not a provider trust audit.
#[test]
fn public_open_clients_are_owned_transparent_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads with the new laws");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "EventTrace adds no trusted GlobalId");

    env.elaborate_file(OPEN_CLIENT)
        .expect("public singleton and both open transports check");
    for (client, name) in [
        ("singles", SINGLE),
        ("client_insert_sorted", INSERT),
        ("client_sorted", SORT),
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(owned.contains(&id), "EventTrace must own {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked Transparent body"
        );
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .unwrap_or_else(|| panic!("{client} must have a checked body"));
        assert!(body_contains_const(&body, id), "{client} must use {name}");
    }
}

/// MEASURED: independent closed values for the tie and for an initially
/// unordered batch; both equations apply without a premise.
#[test]
fn closed_singleton_merge_and_batch_sort_keep_ties_in_order() {
    let mut env = loaded();
    env.elaborate_file(TIE_PREFIX).expect("closed tie controls");
    env.elaborate_file(BATCH).expect("closed batch controls");
}

/// MEASURED: the same well-typed law application rejects only the swapped
/// outcome, with a kernel type mismatch inside the isolated theorem.
#[test]
fn swapping_tied_events_refuses_after_fresh_accepted_prefix() {
    let mut env = loaded();
    env.elaborate_file(TIE_PREFIX)
        .expect("fresh prefix with independent tie values and accepted law application");
    let parsed = parse_decls(SWAPPED).expect("swapped theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated swapped equality");
    };
    assert_eq!(name, "tie_swapped");
    let proof_start = SWAPPED.rfind(INSERT).expect("law application in the body");
    let proof_end = SWAPPED.trim_end().len();
    match env.elaborate_file(SWAPPED) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start
            && span.start <= proof_start
            && proof_end <= span.end
            && span.end <= claim.end =>
        {
            eprintln!("swapped tie: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("swapped tie must refuse at its own law application: {other:?}"),
    }
}

/// MEASURED: the checked proof helper does not enter the importable API.
#[test]
fn cons_case_helper_is_not_importable() {
    let mut env = loaded();
    match env.elaborate_file(&format!("import {TRACE} ({HELPER})\n")) {
        Err(ElabError::UnboundName { name, .. }) if name == format!("{TRACE}.{HELPER}") => {}
        other => panic!("private cons helper must refuse selective import: {other:?}"),
    }
}
