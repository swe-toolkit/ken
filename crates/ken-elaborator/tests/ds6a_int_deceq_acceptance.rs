//! DS-6a (ADR 0013 Layer 1) acceptance at the elaborator layer — `Int`'s
//! REAL certificate registration (`crates/ken-elaborator/src/numbers.rs`)
//! and the catalog rewiring's conformance bar (`docs/program/wp/
//! DS-6a-int-deceq-certificate.md`), against the REAL
//! `catalog/packages/Core/Classes/LawfulClasses.ken.md` source. The general
//! mechanism + its registration-time hardening (negative arms) are covered
//! kernel-side in `ken-kernel/tests/ds6a_int_deceq_certificate.rs`; the
//! zero-Axiom-delta / real-proof discriminators for `Eq Int`/`DecEq Int`
//! themselves live in `es4_classes_acceptance.rs` (the existing home for
//! this file's law-field producer-grep gates).

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval_checked, EvalStore, EvalVal};
use ken_kernel::env::{Context, Decl as KernelDecl};
use ken_kernel::term::Term;
use ken_kernel::{check, whnf, KernelError};

fn mk_env_with_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env construction failed");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    catalog_or::load_derived_fixture(&mut env);
    env
}

// ─────────────────────────────────────────────────────────────────────────
// `DecEq Char` — checks a subset equality lift from the projected Int
// carriers, reusing the named Int certificate without adding new trust.
// ─────────────────────────────────────────────────────────────────────────

/// Promise class: durable checked behavior. Owner-resolved DecEq Char
/// compares checked pairs and adds no fresh trust.
#[test]
fn deceq_char_transports_from_deceq_int_not_a_fresh_postulate() {
    let mut env = mk_env_with_package();
    let lawful_owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Core.Classes.LawfulClasses")
        .expect("the LawfulClasses owner must roots-load in this environment");
    let class_id = env.globals.get("Core.Classes.LawfulClasses.DecEq").and_then(|id| env.class_env.class_by_id(*id))
        .expect("DecEq must remain a checked class")
        .projection
        .type_id;
    assert!(lawful_owned.contains(&class_id), "DecEq must be owned by LawfulClasses");
    let instance_id = |head: &str| {
        let id = env
            .class_env
            .instance_search("DecEq", head)
            .unwrap_or_else(|| panic!("DecEq {head} must remain registered"));
        assert!(lawful_owned.contains(&id), "DecEq {head} must belong to LawfulClasses");
        id
    };
    let id = instance_id("Char");
    assert!(lawful_owned.contains(&instance_id("Int")));
    assert!(matches!(env.env.lookup(id), Some(KernelDecl::Transparent { .. })));
    let eq = Term::proj1(Term::const_(id, vec![]));
    let bool_ty = Term::indformer(env.globals["Bool"], vec![]);
    let a = ken_kernel::check::checked_char_literal(&env.env, 65).unwrap();
    let b = ken_kernel::check::checked_char_literal(&env.env, 66).unwrap();
    for (x, y, expected) in [(&a, &a, true), (&a, &b, false)] {
        let term = Term::app(Term::app(eq.clone(), (*x).clone()), (*y).clone());
        check(&env.env, &Context::new(), &term, &bool_ty)
            .expect("DecEq Char operates on checked subset pairs");
        assert_eq!(
            eval_checked(&term, &bool_ty, &env.env, &mut EvalStore::new()).unwrap(),
            EvalVal::Bool(expected),
        );
    }

    // The instance is admitted only after kernel-checking its sound and
    // complete fields at Equal Char, not merely at Equal Int.
    let mut delta = ken_elaborator::trusted_base_delta(&env.env, id);
    delta.remove(&env.class_env.record_nil_val_id);
    assert!(
        delta.is_empty(),
        "DecEq Char must be zero-NEW-delta by transport — got {:?}",
        delta
    );
}

// ─────────────────────────────────────────────────────────────────────────
// Neutral preserved (ADR 0013's discriminating arm): DS-6a registers a
// certificate but adds NO reduction rule — `obs.rs::eq_reduce` is
// untouched, so `Eq`/`Equal` at an abstract operand pair stays exactly as
// unreachable as before, for both the registered `Int` and an unregistered
// primitive.
// ─────────────────────────────────────────────────────────────────────────

fn assert_abstract_eq_stays_neutral(env: &ElabEnv, prim_id: ken_kernel::GlobalId) {
    let prim_const = Term::const_(prim_id, vec![]);
    let mut ctx = Context::new();
    ctx.push(prim_const.clone());
    ctx.push(prim_const.clone());
    // x = Var(1), y = Var(0): both bound, abstract, unconstrained.
    let eq_ty = Term::Eq(
        Box::new(prim_const),
        Box::new(Term::var(1)),
        Box::new(Term::var(0)),
    );
    let reduced = whnf(&env.env, &ctx, &eq_ty);
    assert!(
        matches!(reduced, Term::Eq(..)),
        "Eq at abstract operands must stay neutral (Term::Eq), got {:?}",
        reduced
    );
}

#[test]
fn abstract_int_equality_stays_neutral() {
    let env = mk_env_with_package();
    assert_abstract_eq_stays_neutral(&env, env.numeric_env.int_id);
}

#[test]
fn unregistered_primitive_equality_stays_neutral() {
    let env = mk_env_with_package();
    // `Float` never registers a certificate (DS-6a scopes `Int` only) —
    // confirms the mechanism is genuinely opt-in, not a blanket change to
    // primitive-`Eq` behavior.
    assert!(env.env.deceq_cert(env.numeric_env.float_id).is_none());
    assert_abstract_eq_stays_neutral(&env, env.numeric_env.float_id);
}

// ─────────────────────────────────────────────────────────────────────────
// Over-equate regression pin (soundness). Per the Architect's conformance
// clarification: under DS-6a ALONE (no native `IntLit` reduction — that is
// DS-6b, separate and later), a proof of a CONCRETE false equality like
// `Equal Int 5 6` cannot even be phrased as a reducible goal (`eq_int`
// never fires on literals in K1 either way) — so the discriminating,
// DS-6a-scoped regression pin is: the certificate opens no new path to
// equating two genuinely DISTINCT `Int`s at all, concrete or abstract.
// Exercised at the most basic level `Refl` already covers: rejection at
// conversion is `TypeMismatch` with the two Eq endpoints identified, while
// the same certificate at the self-equality goal is accepted. DS-6a's
// registration must not loosen `check`'s `Refl` rule or `conv`.
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn distinct_int_variables_refl_still_rejected() {
    let env = mk_env_with_package();
    let int_const = Term::const_(env.numeric_env.int_id, vec![]);
    let mut ctx = Context::new();
    ctx.push(int_const.clone());
    ctx.push(int_const.clone());
    let eq_ty = Term::Eq(
        Box::new(int_const.clone()),
        Box::new(Term::var(1)),
        Box::new(Term::var(0)),
    );
    // `Refl (Var(1))` witnesses `x = x`, not the goal `x = y`.
    let err = check(&env.env, &ctx, &Term::Refl(Box::new(Term::var(1))), &eq_ty)
        .expect_err("Refl must not prove two distinct Int variables equal");
    match err {
        KernelError::TypeMismatch { expected, found } => {
            assert_eq!(*expected, eq_ty, "the rejected goal is x = y as offered");
            assert_eq!(
                *found,
                Term::Eq(
                    Box::new(int_const.clone()),
                    Box::new(Term::var(1)),
                    Box::new(Term::var(1)),
                ),
                "Refl x has type Eq Int x x; only the right index cannot convert",
            );
        }
        other => panic!("expected TypeMismatch at Refl conversion, got {other:?}"),
    }

    let self_eq = Term::Eq(
        Box::new(int_const),
        Box::new(Term::var(1)),
        Box::new(Term::var(1)),
    );
    check(
        &env.env,
        &ctx,
        &Term::Refl(Box::new(Term::var(1))),
        &self_eq,
    )
    .expect("Refl x proves x = x under the DS-6a package");
}
