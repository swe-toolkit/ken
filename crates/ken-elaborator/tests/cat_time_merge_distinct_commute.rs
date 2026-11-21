//! Spec 69 §20: merge commutes for pairwise distinct cross-trace timestamps.
//! Promise class: durable checked law and independent closed-value controls.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const TRACE: &str = "Data.Time.EventTrace";
const LAW: &str = "trace_merge_distinct_commute";

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

const ACCEPTED: &str = r#"import Core.Logic.Transport (sym)
import Core.Logic.Or (Or, Inl, Inr)
import Core.Classes.LawfulClasses (bool_or)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace (Event, MkEvent, event_time, trace_merge, trace_merge_distinct_commute)

fn distinct_times (x : Event Bool) (y : Event Bool) : Bool =
  bool_or (lt_nat (event_time Bool x) (event_time Bool y)) (lt_nat (event_time Bool y) (event_time Bool x))

theorem distinct_at
      (x : Event Bool) (a : Event Bool) (y : Event Bool) (b : Event Bool)
    : Equal (Event Bool) x a
      → Equal (Event Bool) y b
      → Equal Bool (distinct_times a b) True
      → Equal Bool (distinct_times x y) True =
  λx_is_a. λy_is_b. λbase.
    J (λz _. Equal Bool (distinct_times z y) True)
      (J (λw _. Equal Bool (distinct_times a w) True) base (sym (Event Bool) y b y_is_b))
      (sym (Event Bool) x a x_is_a)

const t1 : Nat = Suc Zero
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2

const a1 : Event Bool = MkEvent Bool t1 True
const c3 : Event Bool = MkEvent Bool t3 False
const b2 : Event Bool = MkEvent Bool t2 True

const left : List (Event Bool) = Cons (Event Bool) a1 (Cons (Event Bool) c3 (Nil (Event Bool)))
const right : List (Event Bool) = Cons (Event Bool) b2 (Nil (Event Bool))

theorem left_right_distinct
    : (x : Event Bool) → list_elem (Event Bool) x left
      → (y : Event Bool) → list_elem (Event Bool) y right
      → Equal Bool (distinct_times x y) True =
  λx. λin_left. λy. λin_right.
    elim_trunc
      (Equal Bool (distinct_times x y) True)
      (λhead_case.
        match head_case {
          Inl x_is_a ↦
            elim_trunc
              (Equal Bool (distinct_times x y) True)
              (λright_case.
                match right_case {
                  Inl y_is_b ↦ distinct_at x a1 y b2 x_is_a y_is_b Proved;
                  Inr none ↦ absurd none
                })
              in_right;
          Inr later ↦
            elim_trunc
              (Equal Bool (distinct_times x y) True)
              (λtail_case.
                match tail_case {
                  Inl x_is_c ↦
                    elim_trunc
                      (Equal Bool (distinct_times x y) True)
                      (λright_case.
                        match right_case {
                          Inl y_is_b ↦ distinct_at x c3 y b2 x_is_c y_is_b Proved;
                          Inr none ↦ absurd none
                        })
                      in_right;
                  Inr none ↦ absurd none
                })
              later
        })
      in_left

theorem distinct_commute
    : Equal (List (Event Bool)) (trace_merge Bool left right) (trace_merge Bool right left) =
  trace_merge_distinct_commute Bool left right left_right_distinct

theorem distinct_value_lr
    : Equal (List (Event Bool)) (trace_merge Bool left right)
        (Cons (Event Bool) a1 (Cons (Event Bool) b2 (Cons (Event Bool) c3 (Nil (Event Bool))))) = Refl

theorem distinct_value_rl
    : Equal (List (Event Bool)) (trace_merge Bool right left)
        (Cons (Event Bool) a1 (Cons (Event Bool) b2 (Cons (Event Bool) c3 (Nil (Event Bool))))) = Refl

const b3 : Event Bool = MkEvent Bool t3 True
const right_tied : List (Event Bool) = Cons (Event Bool) b3 (Nil (Event Bool))

theorem tied_value_lr
    : Equal (List (Event Bool)) (trace_merge Bool left right_tied)
        (Cons (Event Bool) a1 (Cons (Event Bool) c3 (Cons (Event Bool) b3 (Nil (Event Bool))))) = Refl

theorem tied_value_rl
    : Equal (List (Event Bool)) (trace_merge Bool right_tied left)
        (Cons (Event Bool) a1 (Cons (Event Bool) b3 (Cons (Event Bool) c3 (Nil (Event Bool))))) = Refl

const unordered : List (Event Bool) = Cons (Event Bool) c3 (Cons (Event Bool) a1 (Nil (Event Bool)))

theorem unordered_value_lr
    : Equal (List (Event Bool)) (trace_merge Bool unordered right) (trace_merge Bool right unordered) = Refl

theorem open_use (p : Type) (xs : List (Event p)) (ys : List (Event p))
    : ((x : Event p) → list_elem (Event p) x xs → (y : Event p) → list_elem (Event p) y ys
        → Equal Bool (bool_or (lt_nat (event_time p x) (event_time p y)) (lt_nat (event_time p y) (event_time p x))) True)
      → Equal (List (Event p)) (trace_merge p xs ys) (trace_merge p ys xs) =
  trace_merge_distinct_commute p xs ys
"#;

const REFUSED: &str = r#"theorem left_tied_distinct
    : (x : Event Bool) → list_elem (Event Bool) x left
      → (y : Event Bool) → list_elem (Event Bool) y right_tied
      → Equal Bool (distinct_times x y) True =
  λx. λin_left. λy. λin_right.
    elim_trunc
      (Equal Bool (distinct_times x y) True)
      (λhead_case.
        match head_case {
          Inl x_is_a ↦
            elim_trunc
              (Equal Bool (distinct_times x y) True)
              (λright_case.
                match right_case {
                  Inl y_is_b ↦ distinct_at x a1 y b3 x_is_a y_is_b Proved;
                  Inr none ↦ absurd none
                })
              in_right;
          Inr later ↦
            elim_trunc
              (Equal Bool (distinct_times x y) True)
              (λtail_case.
                match tail_case {
                  Inl x_is_c ↦
                    elim_trunc
                      (Equal Bool (distinct_times x y) True)
                      (λright_case.
                        match right_case {
                          Inl y_is_b ↦ distinct_at x c3 y b3 x_is_c y_is_b Proved;
                          Inr none ↦ absurd none
                        })
                      in_right;
                  Inr none ↦ absurd none
                })
              later
        })
      in_left

theorem tied_commute
    : Equal (List (Event Bool)) (trace_merge Bool left right_tied) (trace_merge Bool right_tied left) =
  trace_merge_distinct_commute Bool left right_tied left_tied_distinct
"#;

const NO_PREMISE: &str = r#"theorem tied_commute_bare
    : Equal (List (Event Bool)) (trace_merge Bool left right_tied) (trace_merge Bool right_tied left) =
  Refl
"#;

fn accepted_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace root-loads");
    env.elaborate_file(ACCEPTED)
        .expect("complete accepted client checks on a fresh prefix");
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

/// SOURCE: spec/50-stdlib/69-time-and-events.md §20 and Architect AC-0.
/// PROMISE CLASS: durable checked law and trust identity.
/// MEASURED: seven providers load first, then the owning EventTrace root adds
/// no new trusted GlobalId identity; its law is owned and Transparent, and the
/// open client body contains its exact checked owner Const.
/// CLAIMED: the generic law is imported, proved and consumed at zero new trust.
/// THE GAP: provider trust persists; provider-first is not a root-only load.
#[test]
fn public_open_law_is_owned_transparent_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace root-loads checked law");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        before, after,
        "EventTrace adds no trusted GlobalId identity"
    );
    let id = *env
        .globals
        .get(&format!("{TRACE}.{LAW}"))
        .expect("owner law identity");
    assert!(owned.contains(&id), "the public law belongs to EventTrace");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public law has a checked body"
    );
    env.elaborate_file(ACCEPTED)
        .expect("open generic signature and complete client check");
    let (_, body) = env
        .env
        .transparent_body(env.globals["open_use"])
        .expect("open law client has checked body");
    assert!(
        body_contains_const(&body, id),
        "open client must apply the exact checked owner law"
    );
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §20, closed controls.
/// PROMISE CLASS: durable behavior discriminating ordering and left-biased ties.
/// MEASURED: the whole client applies the law to [a@1,c@3] and [b@2],
/// independently checks both [a,b,c] values, checks two different tie orders
/// at time 3, and checks an unsorted cross-distinct merge without an ordering
/// premise. These finite rows supplement, not replace, the generic law.
#[test]
fn complete_distinct_tie_and_unsorted_client_checks() {
    accepted_env();
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §20, shared-time refusal.
/// PROMISE CLASS: durable premise discriminator.
/// MEASURED: the accepted prefix checks afresh; the same pairwise discharge
/// form with b@3 instead of b@2 rejects at its own false c@3/b@3 Proved arm,
/// before the appended law claim. Both tied merge values already check.
#[test]
fn shared_time_refuses_at_false_premise_after_fresh_accepted_prefix() {
    let mut env = accepted_env();
    let parsed = parse_decls(REFUSED).expect("tied premise and application parse");
    let [SourceDecl::TheoremDecl {
        name,
        span: premise,
        ..
    }, SourceDecl::TheoremDecl { name: law, .. }] = parsed.as_slice()
    else {
        panic!("isolated premise followed by same-form law application");
    };
    assert_eq!(name, "left_tied_distinct");
    assert_eq!(law, "tied_commute");
    let arm = "Inl y_is_b ↦ distinct_at x c3 y b3 x_is_c y_is_b Proved";
    let proof = REFUSED.find(arm).expect("false c@3/b@3 proof arm") + arm.len() - "Proved".len();
    match env
        .elaborate_file(REFUSED)
        .expect_err("the shared-time premise must not check")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } if premise.start <= span.start
            && span.start <= proof
            && proof + "Proved".len() <= span.end
            && span.end <= premise.end => {}
        other => panic!("shared-time refusal must cover its false premise proof: {other:?}"),
    }

    let parsed_bare = parse_decls(NO_PREMISE).expect("bare tied claim parses");
    let [SourceDecl::TheoremDecl {
        name,
        span: bare_claim,
        ..
    }] = parsed_bare.as_slice()
    else {
        panic!("one isolated bare tied equality");
    };
    assert_eq!(name, "tied_commute_bare");
    let refl = NO_PREMISE.rfind("Refl").expect("own Refl body");
    let mut bare_env = accepted_env();
    match bare_env
        .elaborate_file(NO_PREMISE)
        .expect_err("the tied merges differ without the premise")
    {
        ElabError::TypeMismatch { reason, span }
            if reason == "Refl expects an `Eq`-shaped goal"
                && bare_claim.start <= span.start
                && span.start <= refl
                && refl + "Refl".len() <= span.end
                && span.end <= bare_claim.end => {}
        other => panic!("the bare tied equality must refuse at its Refl: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §20, helper privacy.
/// PROMISE CLASS: durable visibility invariant.
/// MEASURED: all three private proof helpers refuse outside selective import
/// by qualified UnboundName. The checked root load separately checks bodies.
#[test]
fn recurrence_and_timestamp_helpers_remain_private() {
    let mut env = accepted_env();
    for helper in [
        "trace_merge_distinct_commute_cons",
        "trace_merge_distinct_commute_step",
        "distinct_times_reverse_false",
    ] {
        let qualified = format!("{TRACE}.{helper}");
        match env.elaborate_file(&format!("import {TRACE} ({helper})\n")) {
            Err(ElabError::UnboundName { name, .. }) if name == qualified => {}
            other => panic!("{helper} must refuse selective import: {other:?}"),
        }
    }
}
