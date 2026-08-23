use std::collections::BTreeSet;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert_type, normalize, Context, KernelError};

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn env_with_fok() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke including the D1 apparatus");
    env
}

/// Rust-side schema for the quotation-preservation judgment. For a
/// constructed accepted-shape quotation, compare the intrinsic denotation
/// directly with the original Pi-closed obligation by kernel conversion.
fn assert_quotation_preservation_conversion(
    env: &mut ElabEnv,
    fixture_name: &str,
    denotation: &str,
    pi_closed_obligation: &str,
) {
    let (denotation_term, denotation_sort) = env
        .elaborate_expr(format!("{fixture_name}-denotation"), denotation)
        .unwrap_or_else(|error| panic!("failed to elaborate denotation `{denotation}`: {error:?}"));
    let (obligation_term, obligation_sort) = env
        .elaborate_expr(format!("{fixture_name}-obligation"), pi_closed_obligation)
        .unwrap_or_else(|error| {
            panic!("failed to elaborate obligation `{pi_closed_obligation}`: {error:?}")
        });
    let context = Context::new();
    assert!(
        convert_type(&env.env, &context, &denotation_sort, &obligation_sort),
        "quotation sides inhabit different sorts for {fixture_name}"
    );
    assert!(
        convert_type(&env.env, &context, &denotation_term, &obligation_term),
        "intrinsic denotation does not convert to the Pi-closed obligation for {fixture_name}"
    );
}

fn add_bool_model(env: &mut ElabEnv) {
    let decls = [
        "const fok_d1_bool_carriers : FokCarriers FokSliceSignature = \
         FokMkCarriers FokSliceSignature Bool",
        "const fok_d1_bool_pred : FokAtomPred Bool = { \
         pred_p = λx. match x { True ↦ Top; False ↦ Bottom } }",
        "const fok_d1_bool_atoms : FokAtomEnv FokSliceSignature fok_d1_bool_carriers = \
         FokMkAtomEnv FokSliceSignature fok_d1_bool_carriers fok_d1_bool_pred",
        "const fok_d1_outer_atom : FokScopedIForm FokSliceSignature Zero = \
         FokScopedForall FokSliceSignature Zero \
           (FokScopedForall FokSliceSignature (Suc Zero) \
             (FokScopedAtom FokSliceSignature (Suc (Suc Zero)) \
               (FokFinSuc (Suc Zero) (FokFinZero Zero))))",
        "const fok_d1_inner_atom : FokScopedIForm FokSliceSignature Zero = \
         FokScopedForall FokSliceSignature Zero \
           (FokScopedForall FokSliceSignature (Suc Zero) \
             (FokScopedAtom FokSliceSignature (Suc (Suc Zero)) \
               (FokFinZero (Suc Zero))))",
        "const fok_d1_same_atom_imp : FokScopedIForm FokSliceSignature Zero = \
         FokScopedForall FokSliceSignature Zero \
           (FokScopedImp FokSliceSignature (Suc Zero) \
             (FokScopedAtom FokSliceSignature (Suc Zero) (FokFinZero Zero)) \
             (FokScopedAtom FokSliceSignature (Suc Zero) (FokFinZero Zero)))",
    ];
    for decl in decls {
        env.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("fixture failed for `{decl}`: {error:?}"));
    }
}

#[test]
fn intrinsic_apparatus_passes_full_admission_with_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("D1 source must elaborate, kernel-check, and pass SCT");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D1 declarations add no trusted authority");
}

#[test]
fn empty_carrier_and_total_scoped_atom_environment_are_writable() {
    let mut env = env_with_fok();
    let decls = [
        "data FokD1Empty : Type where { }",
        "const fok_d1_empty_carriers : FokCarriers FokSliceSignature = \
         FokMkCarriers FokSliceSignature FokD1Empty",
        "const fok_d1_empty_pred : FokAtomPred FokD1Empty = { \
         pred_p = λx. match x { } }",
        "const fok_d1_empty_atoms : FokAtomEnv FokSliceSignature fok_d1_empty_carriers = \
         FokMkAtomEnv FokSliceSignature fok_d1_empty_carriers fok_d1_empty_pred",
    ];
    for decl in decls {
        env.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("empty-carrier declaration failed: {error:?}"));
    }

    let unscoped = env.elaborate_decl(
        "const fok_d1_unscoped_atom : FokScopedIForm FokSliceSignature Zero = \
         FokScopedAtom FokSliceSignature Zero (FokFinZero Zero)",
    );
    assert!(
        matches!(
            unscoped,
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            })
        ),
        "a closed atom cannot name a missing binder; got {unscoped:?}"
    );
}

#[test]
fn fin_lookup_preserves_binder_order_and_atom_identity() {
    let mut env = env_with_fok();
    add_bool_model(&mut env);
    let decls = [
        "theorem fok_d1_outer_true \
         (h : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_outer_atom) : Top = h True False",
        "theorem fok_d1_outer_false \
         (h : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_outer_atom) : Bottom = h False True",
        "theorem fok_d1_inner_true \
         (h : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_inner_atom) : Top = h False True",
        "theorem fok_d1_inner_false \
         (h : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_inner_atom) : Bottom = h True False",
        "theorem fok_d1_outer_erasure : \
         Equal FokIForm \
           (fok_erase_n FokSliceSignature Zero fok_d1_outer_atom) \
           (FokIForall (FokIForall (FokIAtom (FokMkIVar (Suc Zero))))) = Proved",
        "theorem fok_d1_inner_erasure : \
         Equal FokIForm \
           (fok_erase_n FokSliceSignature Zero fok_d1_inner_atom) \
           (FokIForall (FokIForall (FokIAtom (FokMkIVar Zero)))) = Proved",
        "theorem fok_d1_embed_is_existing_path : \
         Equal Bool \
           (fok_form_eq (fok_scoped_embed FokSliceSignature fok_d1_outer_atom) \
             (fok_embed (fok_erase_n FokSliceSignature Zero fok_d1_outer_atom))) \
           True = Proved",
    ];
    for decl in decls {
        env.elaborate_decl(decl).unwrap_or_else(|error| {
            panic!("binder/erasure control failed for `{decl}`: {error:?}")
        });
    }

    assert_quotation_preservation_conversion(
        &mut env,
        "fok_d1_outer_quote_conversion",
        "fok_denote FokSliceSignature fok_d1_bool_carriers fok_d1_bool_atoms \
         fok_d1_outer_atom",
        "(x : Bool) -> (y : Bool) -> match x { True ↦ Top; False ↦ Bottom }",
    );
    assert_quotation_preservation_conversion(
        &mut env,
        "fok_d1_inner_quote_conversion",
        "fok_denote FokSliceSignature fok_d1_bool_carriers fok_d1_bool_atoms \
         fok_d1_inner_atom",
        "(x : Bool) -> (y : Bool) -> match y { True ↦ Top; False ↦ Bottom }",
    );
    assert_quotation_preservation_conversion(
        &mut env,
        "fok_d1_imp_quote_conversion",
        "fok_denote FokSliceSignature fok_d1_bool_carriers fok_d1_bool_atoms \
         fok_d1_same_atom_imp",
        "(x : Bool) -> \
         (match x { True ↦ Top; False ↦ Bottom }) -> \
         match x { True ↦ Top; False ↦ Bottom }",
    );
}

#[test]
fn intrinsic_connectives_and_adequacy_statement_are_live() {
    let mut env = env_with_fok();
    add_bool_model(&mut env);
    let decls = [
        "const fok_d1_bottom : FokScopedIForm FokSliceSignature Zero = \
         FokScopedBottom FokSliceSignature Zero",
        "const fok_d1_imp : FokScopedIForm FokSliceSignature Zero = \
         FokScopedImp FokSliceSignature Zero fok_d1_bottom fok_d1_bottom",
        "const fok_d1_or : FokScopedIForm FokSliceSignature Zero = \
         FokScopedOr FokSliceSignature Zero fok_d1_bottom fok_d1_imp",
        "theorem fok_d1_or_right : \
         fok_denote FokSliceSignature fok_d1_bool_carriers fok_d1_bool_atoms fok_d1_or = \
         trunc_intro (Inr Bottom \
           (fok_denote FokSliceSignature fok_d1_bool_carriers \
             fok_d1_bool_atoms fok_d1_imp) (λh. h))",
        "theorem fok_d1_imp_apply \
         (h : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_imp) \
         (hp : fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_bottom) : \
         fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_bottom = h hp",
        "theorem fok_d1_adequacy_statement_apply \
         (valid : fok_classically_valid \
           (fok_scoped_embed FokSliceSignature fok_d1_outer_atom)) \
         (h : fok_embedding_adequacy_statement FokSliceSignature \
           fok_d1_bool_carriers fok_d1_bool_atoms fok_d1_outer_atom) : \
         fok_denote FokSliceSignature fok_d1_bool_carriers \
           fok_d1_bool_atoms fok_d1_outer_atom = h valid",
    ];
    for decl in decls {
        env.elaborate_decl(decl).unwrap_or_else(|error| {
            panic!("connective/statement control failed for `{decl}`: {error:?}")
        });
    }
}

#[test]
fn truncation_conversion_preserves_quotation() {
    let mut env = env_with_fok();
    add_bool_model(&mut env);
    let denotation_source = "fok_denote FokSliceSignature fok_d1_bool_carriers fok_d1_bool_atoms \
         (FokScopedOr FokSliceSignature Zero \
           (FokScopedBottom FokSliceSignature Zero) \
           (FokScopedImp FokSliceSignature Zero \
             (FokScopedBottom FokSliceSignature Zero) \
             (FokScopedBottom FokSliceSignature Zero)))";
    let obligation_source = "‖Or Bottom (Bottom -> Bottom)‖";
    let (denotation, denotation_sort) = env
        .elaborate_expr("fok-d1-or-denotation", denotation_source)
        .expect("the intrinsic Or denotation");
    let (obligation, obligation_sort) = env
        .elaborate_expr("fok-d1-or-obligation", obligation_source)
        .expect("the original truncated-Or obligation");
    let context = Context::new();

    assert!(
        convert_type(&env.env, &context, &denotation_sort, &obligation_sort),
        "both sides must inhabit the same Omega sort"
    );
    assert!(
        convert_type(&env.env, &context, &obligation, &obligation),
        "positive control: the kernel conversion harness must accept identity"
    );
    assert_eq!(
        normalize(&env.env, &context, &denotation),
        normalize(&env.env, &context, &obligation),
        "full normalization must expose the same truncated Or"
    );
    assert!(
        convert_type(&env.env, &context, &denotation, &obligation),
        "ordinary kernel conversion must recurse through Trunc and compare \
         the recursively unfolded denotation with the quoted Or obligation"
    );
}

#[test]
fn recursive_parent_call_is_rejected_by_unchanged_sct() {
    let mut env = env_with_fok();
    let bad = env.elaborate_decl(
        "fn fok_d1_bad_erase (sigma : FokSignature) (n : Nat) \
         (f : FokScopedIForm sigma n) : FokIForm = \
         match f { \
           FokScopedBottom ↦ FokIBottom; \
           FokScopedAtom i ↦ FokIAtom (FokMkIVar (fok_fin_to_nat n i)); \
           FokScopedOr p q ↦ FokIOr (fok_d1_bad_erase sigma n f) \
             (fok_d1_bad_erase sigma n q); \
           FokScopedImp p q ↦ FokIImp (fok_d1_bad_erase sigma n p) \
             (fok_d1_bad_erase sigma n q); \
           FokScopedForall p ↦ FokIForall (fok_d1_bad_erase sigma (Suc n) p) \
         }",
    );
    assert!(
        matches!(
            bad,
            Err(ElabError::KernelRejected {
                error: KernelError::NotTerminating(_),
                ..
            })
        ),
        "replacing a direct subterm with the parent must red SCT; got {bad:?}"
    );
}
