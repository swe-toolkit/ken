//! Package-local clients for `spec/50-stdlib/69-time-and-events.md` §23.
//! The closed fixtures are from Architect AC-0, not the CV-owned seed runner.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const TRACE: &str = "Data.Time.EventTrace";
const LAW: &str = "Data.Time.EventTrace.trace_merge_all_ordered_sort";
const HELPER: &str = "trace_sort_append_sorted_suffix";

const OPEN: &str = r#"
import Data.Collections.Derived (list_elem, sort, list_concat)
import Data.Time.EventTrace
  (Event, event_before, time_ordered, trace_merge_all, trace_merge_all_ordered_sort)

theorem open_ordered_batches_sort
      (p : Type) (traces : List (List (Event p)))
    : ((t : List (Event p)) → list_elem (List (Event p)) t traces → time_ordered p t)
      → Equal
        (List (Event p))
        (trace_merge_all p traces)
        (sort (Event p) (event_before p) (list_concat (Event p) traces)) =
  λall. trace_merge_all_ordered_sort p traces all
"#;

fn load(env: &mut ElabEnv, module: &str) -> Vec<GlobalId> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    env.elaborate_module_from_roots(&[root], module)
        .unwrap_or_else(|error| panic!("{module} roots-loads: {error:?}"))
}

fn providers() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        load(&mut env, provider);
    }
    env
}

fn loaded() -> ElabEnv {
    let mut env = providers();
    load(&mut env, TRACE);
    env
}

fn closed_prefix() -> ElabEnv {
    let mut env = loaded();
    env.elaborate_file(PREFIX)
        .expect("AC-0 imports and batch ordering witnesses check");
    env
}

fn cites_law(env: &ElabEnv, name: &str) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .expect("checked client body");
    let owner = env.globals[LAW];
    let mut pending = vec![&body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return;
        }
        pending.extend(term.children());
    }
    panic!("{name} must cite the exact owned law Const");
}

fn refuses_closing_refl(source: &str, label: &str) {
    let refl = source.trim_end().len() - "Refl".len();
    assert_eq!(source[refl..].trim_end(), "Refl");
    match closed_prefix().elaborate_file(source) {
        Err(ElabError::TypeMismatch { span, .. })
            if span.start == refl && span.end == refl + "Refl".len() =>
        {
            eprintln!("{label}: TypeMismatch at closing Refl {span:?}");
        }
        other => panic!("{label} must refuse at its closing Refl: {other:?}"),
    }
}

/// Source: §23's generic law and AC-T. Promise class: durable invariant.
/// MEASURED: exact trust identity equality across the package load, plus an
/// open client citing the EventTrace-owned Transparent theorem.
/// CLAIMED: EventTrace provides the all-ordered equation without new trust.
/// THE GAP: this does not fix provider trust or an export count. Checked
/// proof refactors are compatible; extra trusted IDs, loss of the checked
/// owner, or a change that breaks the generic client are breaking.
#[test]
fn exact_trust_and_owned_open_ordered_batches_sort() {
    let mut env = providers();
    let trusted = |env: &ElabEnv| env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let before = trusted(&env);
    let owned = load(&mut env, TRACE);
    let after = trusted(&env);
    eprintln!(
        "EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "EventTrace adds no trusted GlobalIds");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "EventTrace owns the theorem");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(OPEN)
        .expect("open ordered batches check");
    cites_law(&env, "open_ordered_batches_sort");
}

/// Source: §23's private suffix proof, scoped by AC-0.
/// Promise class: durable invariant. MEASURED: a selective import of the
/// helper refuses at its qualified name after EventTrace roots-loads.
/// CLAIMED: the helper is not available to clients. THE GAP: this checks
/// that name, not the entire export surface. Private proof refactors are
/// compatible; publishing the helper is breaking.
#[test]
fn sorted_suffix_helper_is_not_importable() {
    match loaded().elaborate_file(&format!("import {TRACE} ({HELPER})")) {
        Err(ElabError::UnboundName { name, .. }) if name == format!("{TRACE}.{HELPER}") => {}
        other => panic!("private helper must refuse selective import: {other:?}"),
    }
}

/// Source: §23's interleaved batches and AC-2. Promise class: durable invariant.
/// MEASURED: the owned law and closing Refl check the result [1, 2, 3];
/// a fresh copy of the prefix refuses [1, 3, 2] at the same proof step.
/// CLAIMED: the merge agrees with the sort here, not the concatenation.
/// THE GAP: these closed conversions do not check the generic signature or
/// general tie handling. Equivalent encodings are compatible; accepting
/// plain concatenation here is breaking.
#[test]
fn ordered_batches_sort_and_refuse_plain_concatenation() {
    let mut env = closed_prefix();
    env.elaborate_file(SORTED_RESULT)
        .expect("sorted concatenation checks through the law");
    cites_law(&env, "merge_all_result");
    refuses_closing_refl(PLAIN_CONCATENATION, "plain concatenation");
}

/// Source: §23's unordered-batch counterexample and AC-2.
/// Promise class: durable invariant. MEASURED: independently, the merge
/// returns [5, 1] and sort returns [1, 5]. Their direct equation refuses
/// at its closing Refl on a fresh prefix, without applying the law.
/// CLAIMED: the equation does not hold without an ordering premise.
/// THE GAP: this is a closed counterexample, not a check of the exact premise
/// in the generic declaration. Equivalent timestamps are compatible;
/// identifying these distinct results is breaking.
#[test]
fn descending_batch_breaks_premise_free_equation() {
    closed_prefix()
        .elaborate_file(DESCENDING_VALUES)
        .expect("merge and sort have different closed values");
    refuses_closing_refl(PREMISE_FREE_EQUATION, "premise-free equation");
}

const PREFIX: &str = r#"import Core.Logic.Conjunction (and_intro)

import Core.Logic.Or (Or, Inl, Inr)

import Core.Logic.Transport (sym, trans)

import Data.Collections.Derived (list_elem, sort, list_concat)

import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_merge_all, trace_merge_all_ordered_sort)

const e1 : Event Bool = MkEvent Bool (Suc Zero) True

const e2 : Event Bool = MkEvent Bool (Suc (Suc Zero)) True

const e3 : Event Bool = MkEvent Bool (Suc (Suc (Suc Zero))) True

const xs : List (Event Bool) = Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))

const ys : List (Event Bool) = Cons (Event Bool) e2 (Nil (Event Bool))

const batches : List (List (Event Bool)) =
  Cons (List (Event Bool)) xs (Cons (List (Event Bool)) ys (Nil (List (Event Bool))))

theorem xs_ordered : time_ordered Bool xs =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool (Cons (Event Bool) e3 (Nil (Event Bool))))
    Proved
    Proved

theorem ys_ordered : time_ordered Bool ys = Proved

theorem batches_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t batches → time_ordered Bool t =
  λmember.
    elim_trunc
      (time_ordered Bool t)
      (λcase.
        match case {
          Inl same_xs ↦ J (λu _. time_ordered Bool u) xs_ordered (sym (List (Event Bool)) t xs same_xs);
          Inr later ↦
            elim_trunc
              (time_ordered Bool t)
              (λcase_ys.
                match case_ys {
                  Inl same_ys ↦
                    J (λu _. time_ordered Bool u) ys_ordered (sym (List (Event Bool)) t ys same_ys);
                  Inr none ↦ absurd none
                })
              later
        })
      member
"#;

const SORTED_RESULT: &str = r#"const expected : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e2 (Cons (Event Bool) e3 (Nil (Event Bool))))

theorem merge_all_result : Equal (List (Event Bool)) (trace_merge_all Bool batches) expected =
  trans
    (List (Event Bool))
    (trace_merge_all Bool batches)
    (sort (Event Bool) (event_before Bool) (list_concat (Event Bool) batches))
    expected
    (trace_merge_all_ordered_sort Bool batches batches_ordered)
    Refl
"#;

const PLAIN_CONCATENATION: &str = r#"const expected : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Cons (Event Bool) e2 (Nil (Event Bool))))

theorem merge_all_result : Equal (List (Event Bool)) (trace_merge_all Bool batches) expected =
  trans
    (List (Event Bool))
    (trace_merge_all Bool batches)
    (sort (Event Bool) (event_before Bool) (list_concat (Event Bool) batches))
    expected
    (trace_merge_all_ordered_sort Bool batches batches_ordered)
    Refl
"#;

const DESCENDING_VALUES: &str = r#"const e5 : Event Bool = MkEvent Bool (Suc (Suc (Suc (Suc (Suc Zero))))) True

const descending : List (Event Bool) = Cons (Event Bool) e5 (Cons (Event Bool) e1 (Nil (Event Bool)))

const one_batch : List (List (Event Bool)) =
  Cons (List (Event Bool)) descending (Nil (List (Event Bool)))

theorem one_batch_merge : Equal (List (Event Bool)) (trace_merge_all Bool one_batch) descending = Refl

theorem one_batch_sort
    : Equal
        (List (Event Bool))
        (sort (Event Bool) (event_before Bool) (list_concat (Event Bool) one_batch))
        (Cons (Event Bool) e1 (Cons (Event Bool) e5 (Nil (Event Bool)))) =
  Refl
"#;

const PREMISE_FREE_EQUATION: &str = r#"const e5 : Event Bool = MkEvent Bool (Suc (Suc (Suc (Suc (Suc Zero))))) True

const descending : List (Event Bool) = Cons (Event Bool) e5 (Cons (Event Bool) e1 (Nil (Event Bool)))

const one_batch : List (List (Event Bool)) =
  Cons (List (Event Bool)) descending (Nil (List (Event Bool)))

theorem premise_free_merge_all_sort
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool one_batch)
        (sort (Event Bool) (event_before Bool) (list_concat (Event Bool) one_batch)) =
  Refl
"#;
