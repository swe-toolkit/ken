//! `V3-FO-CHECKER-SOUNDNESS` D1a artifact and correspondence controls
//! (`AC-1`/`AC-2`).
//!
//! Promise class: durable invariant. The artifact control pins the registered
//! family's closed four-constructor inventory, index shape, and ordered argument
//! arities. The correspondence controls stay green for changes that preserve the
//! four accepted checker branches and turn red when either `FokDerivation` or
//! `fok_check_rule` changes a conclusion, recursive-premise arity, or the
//! ForallRight freshness condition.
//!
//! Each row constructs the checked `FokDerivation` constructor term and a
//! matching accepted certificate, then changes the certificate at the rule's
//! natural input and requires the unchanged checker to reject it. No row
//! constructs or applies the `FokDerivation` eliminator/recursor.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_or(&mut env);
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken with FokDerivation must elaborate and kernel-check");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D1a must add no trusted-base entry");
    env
}

fn elaborate_all(env: &mut ElabEnv, decls: &[&str]) {
    for decl in decls {
        env.elaborate_decl(decl)
            .unwrap_or_else(|err| panic!("D1a control declaration rejected:\n{decl}\n{err}"));
    }
}

#[test]
fn fok_derivation_kernel_artifact_has_closed_rule_inventory() {
    let env = mk_env();
    let family_id = *env
        .globals
        .get("FokDerivation")
        .expect("FokDerivation must be registered globally");
    let family = env
        .env
        .inductive(family_id)
        .expect("FokDerivation must be registered as an inductive family");

    assert_eq!(
        family.indices.len(),
        1,
        "FokDerivation must have exactly the FokSequent index"
    );
    assert_eq!(
        family.constructors.len(),
        4,
        "FokDerivation must have exactly the four accepted checker branches"
    );
    assert_eq!(
        family
            .constructors
            .iter()
            .map(|constructor| constructor.target_indices.len())
            .collect::<Vec<_>>(),
        vec![1, 1, 1, 1],
        "every FokDerivation constructor must target exactly one sequent index"
    );
    assert_eq!(
        family
            .constructors
            .iter()
            .map(|constructor| constructor.args.len())
            .collect::<Vec<_>>(),
        vec![9, 7, 8, 8],
        "constructor argument arities must stay ordered as Init, ImpRight, World, Obj"
    );
}

#[test]
fn init_constructor_and_checker_reject_a_changed_principal_index() {
    let mut env = mk_env();
    elaborate_all(
        &mut env,
        &[
            "fn d1a_init_constructor_probe \
                 (gamma : List FokForm) (delta : List FokForm) \
                 (left : Nat) (right : Nat) (g : FokForm) (d : FokForm) \
                 (left_lookup : Equal (Option FokForm) (fok_nth_form gamma left) \
                   (Some FokForm g)) \
                 (right_lookup : Equal (Option FokForm) (fok_nth_form delta right) \
                   (Some FokForm d)) \
                 (same : Equal Bool (fok_form_eq g d) True) \
               : FokDerivation (FokMkSequent gamma delta) = \
               FokDerivInit gamma delta left right g d left_lookup right_lookup same",
            "const d1a_init_forms : List FokForm = Cons FokForm FokBottom (Nil FokForm)",
            "const d1a_init_sequent : FokSequent = FokMkSequent d1a_init_forms d1a_init_forms",
            "const d1a_init_derivation : FokDerivation d1a_init_sequent = \
               FokDerivInit d1a_init_forms d1a_init_forms Zero Zero FokBottom FokBottom \
                 Proved Proved Proved",
            "const d1a_init_cert : FokCert = \
               FokMkCert d1a_init_sequent (FokInit Zero Zero) (Nil FokCert)",
            "theorem d1a_init_checker_accepts : \
               Equal Bool (fok_check_tree d1a_init_sequent d1a_init_cert) True = Proved",
            "const d1a_init_changed_index_cert : FokCert = \
               FokMkCert d1a_init_sequent (FokInit Zero (Suc Zero)) (Nil FokCert)",
            "theorem d1a_init_changed_index_rejected : \
               Equal Bool (fok_check_tree d1a_init_sequent d1a_init_changed_index_cert) False = Proved",
        ],
    );
}

#[test]
fn imp_right_constructor_and_checker_reject_a_missing_recursive_premise() {
    let mut env = mk_env();
    elaborate_all(
        &mut env,
        &[
            "const d1a_imp_gamma : List FokForm = Cons FokForm FokBottom (Nil FokForm)",
            "const d1a_imp_delta : List FokForm = \
               Cons FokForm (FokImp FokBottom FokBottom) (Nil FokForm)",
            "const d1a_imp_sequent : FokSequent = FokMkSequent d1a_imp_gamma d1a_imp_delta",
            "const d1a_imp_child_gamma : List FokForm = \
               fok_list_form_append_one d1a_imp_gamma FokBottom",
            "const d1a_imp_child_delta : List FokForm = \
               fok_list_form_set_nth d1a_imp_delta Zero FokBottom",
            "const d1a_imp_child_sequent : FokSequent = \
               FokMkSequent d1a_imp_child_gamma d1a_imp_child_delta",
            "fn d1a_imp_constructor_probe \
                 (gamma : List FokForm) (delta : List FokForm) \
                 (right : Nat) (p : FokForm) (q : FokForm) \
                 (lookup : Equal (Option FokForm) (fok_nth_form delta right) \
                   (Some FokForm (FokImp p q))) \
                 (child : FokDerivation \
                   (FokMkSequent (fok_list_form_append_one gamma p) \
                     (fok_list_form_set_nth delta right q))) \
               : FokDerivation (FokMkSequent gamma delta) = \
               FokDerivImpRight gamma delta right p q lookup child",
            "const d1a_imp_child_cert : FokCert = \
               FokMkCert d1a_imp_child_sequent (FokInit Zero Zero) (Nil FokCert)",
            "const d1a_imp_cert : FokCert = \
               FokMkCert d1a_imp_sequent (FokImpRight Zero) \
                 (Cons FokCert d1a_imp_child_cert (Nil FokCert))",
            "theorem d1a_imp_checker_accepts : \
               Equal Bool (fok_check_tree d1a_imp_sequent d1a_imp_cert) True = Proved",
            "const d1a_imp_missing_child_cert : FokCert = \
               FokMkCert d1a_imp_sequent (FokImpRight Zero) (Nil FokCert)",
            "theorem d1a_imp_missing_child_rejected : \
               Equal Bool (fok_check_tree d1a_imp_sequent d1a_imp_missing_child_cert) False = Proved",
        ],
    );
}

fn check_forall_right_shape(prefix: &str, forall_ctor: &str, deriv_ctor: &str) {
    let mut env = mk_env();
    let declarations = [
        format!("const {prefix}_gamma : List FokForm = Cons FokForm FokBottom (Nil FokForm)"),
        format!(
            "const {prefix}_body : FokForm = \
             FokForcingP (FokQParameter Zero) (FokQBound Zero)"
        ),
        format!(
            "const {prefix}_delta : List FokForm = \
             Cons FokForm ({forall_ctor} {prefix}_body) \
               (Cons FokForm FokBottom (Nil FokForm))"
        ),
        format!(
            "const {prefix}_sequent : FokSequent = \
             FokMkSequent {prefix}_gamma {prefix}_delta"
        ),
        format!("const {prefix}_fresh : FokQTerm = FokQParameter (Suc Zero)"),
        format!(
            "const {prefix}_child_delta : List FokForm = \
             fok_list_form_set_nth {prefix}_delta Zero \
               (fok_subst0_form {prefix}_body {prefix}_fresh)"
        ),
        format!(
            "const {prefix}_child_sequent : FokSequent = \
             FokMkSequent {prefix}_gamma {prefix}_child_delta"
        ),
        format!(
            "fn {prefix}_constructor_probe \
                 (gamma : List FokForm) (delta : List FokForm) \
                 (right : Nat) (eigen : FokQTerm) (body : FokForm) \
                 (lookup : Equal (Option FokForm) (fok_nth_form delta right) \
                   (Some FokForm ({forall_ctor} body))) \
                 (freshness : Equal Bool \
                   (fok_sequent_mentions_parameter (FokMkSequent gamma delta) eigen) False) \
                 (child : FokDerivation \
                   (FokMkSequent gamma \
                     (fok_list_form_set_nth delta right (fok_subst0_form body eigen)))) \
               : FokDerivation (FokMkSequent gamma delta) = \
               {deriv_ctor} gamma delta right eigen body lookup freshness child"
        ),
        format!(
            "const {prefix}_child_cert : FokCert = \
             FokMkCert {prefix}_child_sequent (FokInit Zero (Suc Zero)) (Nil FokCert)"
        ),
        format!(
            "const {prefix}_cert : FokCert = \
             FokMkCert {prefix}_sequent (FokForallRight Zero {prefix}_fresh) \
               (Cons FokCert {prefix}_child_cert (Nil FokCert))"
        ),
        format!(
            "theorem {prefix}_checker_accepts : \
             Equal Bool (fok_check_tree {prefix}_sequent {prefix}_cert) True = Proved"
        ),
        format!(
            "const {prefix}_stale_eigen_cert : FokCert = \
             FokMkCert {prefix}_sequent (FokForallRight Zero (FokQParameter Zero)) \
               (Cons FokCert {prefix}_child_cert (Nil FokCert))"
        ),
        format!(
            "theorem {prefix}_stale_eigen_rejected : \
             Equal Bool (fok_check_tree {prefix}_sequent {prefix}_stale_eigen_cert) False = Proved"
        ),
    ];
    let refs = declarations.iter().map(String::as_str).collect::<Vec<_>>();
    elaborate_all(&mut env, &refs);
}

#[test]
fn forall_world_right_constructor_and_checker_reject_a_nonfresh_eigenparameter() {
    check_forall_right_shape("d1a_world", "FokForallWorld", "FokDerivForallWorldRight");
}

#[test]
fn forall_obj_right_constructor_and_checker_reject_a_nonfresh_eigenparameter() {
    check_forall_right_shape("d1a_obj", "FokForallObj", "FokDerivForallObjRight");
}
