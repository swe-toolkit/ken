//! Inferred matches on indexed families share the dependent method path,
//! whether or not a root constructor was omitted.
//! Spec: `spec/30-surface/34-data-match.md §3.1–3.2, §4.1–4.3`.
//! Promise class: durable invariant. All fixtures exercise the checked
//! source-to-eliminator path through `ElabEnv::elaborate_file`.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{whnf, Context, Decl, Term};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

const FIN: &str = r#"
data Fin : Nat → Type where {
  FZ : (n : Nat) → Fin (Suc n);
  FS : (n : Nat) → Fin n → Fin (Suc n)
}
"#;

/// A complete match has no nested eliminators: its indexed Elim is the root.
/// Accepting the source alone cannot distinguish the dependent path from a
/// constant indexed motive, because the shared Elim builder types both.
fn assert_complete_match_has_index_premise(env: &ElabEnv, name: &str) {
    fn indexed_elim(term: &Term) -> Option<&Term> {
        if matches!(term, Term::Elim { indices, .. } if !indices.is_empty()) {
            return Some(term);
        }
        term.children().into_iter().find_map(indexed_elim)
    }
    let id = *env.globals.get(name).expect("named checked definition");
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("named function has a kernel-checked body")
    };
    let Some(Term::Elim { motive, .. }) = indexed_elim(body) else {
        panic!("complete match must emit an indexed root eliminator")
    };
    let Term::Ascript(expr, _) = motive.as_ref() else {
        panic!("indexed root motive must be ascribed")
    };
    let mut body = expr.as_ref();
    while let Term::Lam(_, rest) = body {
        body = rest;
    }
    assert!(
        matches!(body, Term::Pi(premise, _) if matches!(premise.as_ref(), Term::Eq(..))),
        "indexed inferred root motive lacks its equality premise: {motive:?}"
    );
}

#[test]
fn complete_vec_at_variable_index_checks() {
    // MEASURED: both root methods are supplied and the indexed eliminator
    // kernel-checks. CLAIMED: completeness must not change its motive path.
    // THE GAP: a surface acceptance alone need not discriminate constant
    // motives; reverting the indexed dispatch makes this exact row red.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn complete (a : Type) (n : Nat) (xs : Vec a n) : Nat = \
         let r = match xs {{ VNil ↦ Zero; VCons m _ _ ↦ m }} in r"
    ))
    .expect("complete indexed Vec at n must check");
    assert_complete_match_has_index_premise(&env, "complete");
}

#[test]
fn complete_vec_at_successor_index_checks() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn complete_suc (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VNil ↦ Zero; VCons m _ _ ↦ m }} in r"
    ))
    .expect("adding a VNil arm to a well-typed omitted-index match must check");
    assert_complete_match_has_index_premise(&env, "complete_suc");
}

#[test]
fn complete_fin_with_both_arms_checks() {
    // Fin has no family parameters: the old constant-motive path erroneously
    // passed its one index as a parameter to the kernel eliminator.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{FIN}\nfn complete_fin (n : Nat) (i : Fin (Suc n)) : Nat = \
         let r = match i {{ FZ m ↦ Zero; FS m _ ↦ Suc m }} in r"
    ))
    .expect("complete indexed Fin with no parameters must check");
    assert_complete_match_has_index_premise(&env, "complete_fin");
}

#[test]
fn nested_root_split_closes_the_remaining_method_telescope() {
    // A nested tail split can yield an eliminator rather than a lambda for
    // the outer constructor's pending IH. The method is applied to that IH.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn nested (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VCons m x VNil ↦ Zero; VCons m x _ ↦ m }} in r"
    ))
    .expect("nested split under omitted root VNil must close its method");
}

#[test]
fn nested_vnil_and_vcons_specialize_the_carried_root_ih() {
    // Both nested constructor methods carry the outer VCons IH. Its type
    // specializes to the nested constructor through the nested motive.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn nested_both (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ \
           VCons m _ VNil ↦ Zero; \
           VCons m _ (VCons k _ _) ↦ Suc k \
         }} in r"
    ))
    .expect("both specialized nested methods kernel-check");
}

#[test]
fn nested_split_threads_two_root_ih_columns() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data PairIx : Nat → Type where { \
           PZero : PairIx Zero; \
           PStep : (m : Nat) → PairIx m → PairIx m → PairIx (Suc m) \
         }\nfn nested_pair (n : Nat) (xs : PairIx (Suc n)) : Nat = \
         let r = match xs { \
           PStep m PZero _ ↦ Zero; PStep m _ _ ↦ m \
         } in r",
    )
    .expect("both dependent root IHs survive the nested split");
}

#[test]
fn constant_nested_motive_keeps_the_nonindexed_control() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(
        "data NatBox : Type where { BoxNat : Nat → NatBox }\n\
         fn boxed (b : NatBox) : Nat = let r = match b { \
           BoxNat Zero ↦ Zero; BoxNat (Suc n) ↦ n \
         } in r",
    )
    .expect("independent nested motive remains the constant method path");
}

#[test]
fn nested_variable_index_without_a_dependent_tail_checks() {
    // The near-neighbour of a concrete index: a constructor-bound variable
    // is eligible even when this nested motive has no dependent convoy.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\ndata HolderVar (a : Type) : Type where {{ \
           HoldVar : (n : Nat) → Vec a n → HolderVar a \
         }}\nfn variable_nested (a : Type) (h : HolderVar a) : Nat = \
         let r = match h {{ \
           HoldVar n VNil ↦ Zero; HoldVar n (VCons m _ _) ↦ m \
         }} in r"
    ))
    .expect("distinct variable index without a dependent tail checks");
}

#[test]
fn nested_concrete_index_advises_an_annotation() {
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\ndata Holder (a : Type) : Type where {{ Hold : Vec a Zero → Holder a }}\n\
         fn no_equational_generalization (a : Type) (h : Holder a) : Nat = \
         let r = match h {{ Hold VNil ↦ Zero; Hold (VCons m _ _) ↦ m }} in r"
        ))
        .expect_err("concrete nested index needs an explicit separate match");
    assert!(
        matches!(&error, ElabError::TypeMismatch { reason, .. }
        if reason.contains("nested indexed split") && reason.contains("annotate")),
        "{error:?}"
    );
}

#[test]
fn unlowerable_pattern_field_result_has_surface_diagnostic() {
    // MEASURED: an inferred result type `Vec a m` uses the VCons-local `m`.
    // CLAIMED: a leaf-local type cannot escape into the indexed motive.
    // THE GAP: the checked twin below pins a lawful field-dependent branch.
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn field_result (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Vec a n = \
             let r = match xs {{ VCons m x tl ↦ tl }} in r"
        ))
        .expect_err("inferred motive cannot contain the constructor-local m");
    assert!(
        matches!(error, ElabError::InferredMatchResultEscapesPattern {
            ref escaping_binder, ..
        } if escaping_binder.as_deref() == Some("m")),
        "{error:?}"
    );
    let rendered = error.to_string();
    assert!(
        rendered.contains("m") && rendered.contains("annotate"),
        "{rendered}"
    );
}

#[test]
fn checked_field_result_remains_supported() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn field_result_checked (a : Type) (n : Nat) \
         (xs : Vec a (Suc n)) : Vec a n = \
         match xs {{ VCons m x tl ↦ tl }}"
    ))
    .expect("checked field-dependent match uses its expected motive");
}

#[test]
fn lowerable_inferred_match_remains_accepted() {
    // Controls the new refusal: the same inferred path with a closed Nat
    // result must lower. A blanket refusal would make this test red.
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(&format!(
        "{VEC}\nfn lowerable (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
         let r = match xs {{ VCons m _ _ ↦ Suc m }} in r"
    ))
    .expect("inferred Nat result lowers outside every constructor field");
}

#[test]
fn reachable_missing_root_still_reports_unmatched_constructor() {
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn missing_root (a : Type) (n : Nat) (xs : Vec a n) : Nat = \
             let r = match xs {{ VCons m _ _ ↦ m }} in r"
        ))
        .expect_err("VNil is possible at an unconstrained index");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { ref missing, .. }
            if missing.constructor == "VNil"),
        "{error:?}"
    );
}

#[test]
fn nested_column_omission_stays_refused() {
    // The root VNil is impossible at Suc n, but the nested tail may be VCons.
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(&format!(
            "{VEC}\nfn missing_nested (a : Type) (n : Nat) (xs : Vec a (Suc n)) : Nat = \
             let r = match xs {{ VCons m x VNil ↦ Zero }} in r"
        ))
        .expect_err("an indexed root cannot license nested-column omission");
    assert!(
        matches!(error, ElabError::ExhaustivenessError { ref missing, .. }
            if missing.constructor == "VCons"),
        "{error:?}"
    );
}

fn assert_split_column_names(source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(source)
        .expect("split-column names and later fields must check");
    let value = |name: &str| {
        let id = *env.globals.get(name).expect("named checked constant");
        let body = env
            .env
            .transparent_body(id)
            .expect("transparent constant")
            .1;
        whnf(&env.env, &Context::new(), &body)
    };
    assert_eq!(value("selected"), value("expected_selected"));
    assert_eq!(value("selected_zero"), value("expected_zero"));
}

#[test]
fn nonindexed_split_variable_and_later_field_resolve_per_row() {
    // Promise class: durable invariant. MEASURED: a constructor row's later
    // field and a variable row's first and later fields normalize to distinct
    // constructor arguments. CLAIMED: a column push stays uniform while each
    // leaf hides only binders its own source row did not introduce. THE GAP:
    // the Suc/Zero values differ, so a swapped or hidden source index changes
    // the checked result, rather than merely preserving its type.
    assert_split_column_names(
        "data PairOut : Type where { Out : Nat → Nat → PairOut }\n\
         data PlainPair : Type where { MkPlain : Nat → Nat → PlainPair }\n\
         fn rebuild (p : PlainPair) : PairOut = let r = match p { \
           MkPlain Zero later ↦ Out Zero later; \
           MkPlain first later ↦ Out first later \
         } in r\n\
         const selected : PairOut = rebuild (MkPlain (Suc Zero) (Suc (Suc Zero)))\n\
         const selected_zero : PairOut = rebuild (MkPlain Zero (Suc (Suc Zero)))\n\
         const expected_selected : PairOut = Out (Suc Zero) (Suc (Suc Zero))\n\
         const expected_zero : PairOut = Out Zero (Suc (Suc Zero))",
    );
}

#[test]
fn indexed_split_variable_and_later_field_resolve_per_row() {
    // Same row-local source boundary under a dependent indexed root motive.
    assert_split_column_names(
        "data PairOut : Type where { Out : Nat → Nat → PairOut }\n\
         data IndexedPair : Nat → Type where { \
           IEmpty : IndexedPair Zero; \
           MkIndexed : (n : Nat) → Nat → Nat → IndexedPair (Suc n) \
         }\n\
         fn rebuild (n : Nat) (p : IndexedPair (Suc n)) : PairOut = \
         let r = match p { \
           MkIndexed m Zero later ↦ Out Zero later; \
           MkIndexed m first later ↦ Out first later \
         } in r\n\
         const selected : PairOut = rebuild Zero \
           (MkIndexed Zero (Suc Zero) (Suc (Suc Zero)))\n\
         const selected_zero : PairOut = rebuild Zero \
           (MkIndexed Zero Zero (Suc (Suc Zero)))\n\
         const expected_selected : PairOut = Out (Suc Zero) (Suc (Suc Zero))\n\
         const expected_zero : PairOut = Out Zero (Suc (Suc Zero))",
    );
}
