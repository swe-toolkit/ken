//! `V3-FO-CHECKER-SOUNDNESS` D4: both accepted ForallRight target shapes
//! reach the full checker-soundness theorem, and the shared freshness guard
//! rejects an otherwise-valid recursive child.
//!
//! Promise class: durable invariant. Intended extensions may add rules, but
//! these accepted derivations and the eigenparameter discriminator remain part
//! of the checker contract.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn elaborate_fok() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(FOK_SOURCE)
        .expect("the real FoKripke source must pass full admission");
    env
}

fn assert_forall_right_reaches_full_soundness(name: &str, quantifier: &str) {
    let mut env = elaborate_fok();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let bound_atom = format!("{name}_bound_atom");
    let instantiated_atom = format!("{name}_instantiated_atom");
    let body = format!("{name}_body");
    let instantiated_body = format!("{name}_instantiated_body");
    let target = format!("{name}_target");
    let leaf = format!("{name}_leaf");
    let child = format!("{name}_child");
    let root = format!("{name}_root");

    env.elaborate_decl(&format!(
        "const {bound_atom} : FokForm = \
         FokAccess (FokQBound Zero) (FokQBound Zero)"
    ))
    .expect("bound atom in the quantified body");
    env.elaborate_decl(&format!(
        "const {instantiated_atom} : FokForm = \
         FokAccess (FokQParameter Zero) (FokQParameter Zero)"
    ))
    .expect("atom after eigenparameter substitution");
    env.elaborate_decl(&format!(
        "const {body} : FokForm = FokImp {bound_atom} {bound_atom}"
    ))
    .expect("tautological quantified body");
    env.elaborate_decl(&format!(
        "const {instantiated_body} : FokForm = \
         FokImp {instantiated_atom} {instantiated_atom}"
    ))
    .expect("tautological instantiated body");
    env.elaborate_decl(&format!("const {target} : FokForm = {quantifier} {body}"))
        .expect("quantified target");
    env.elaborate_decl(&format!(
        "const {leaf} : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Cons FokForm {instantiated_atom} (Nil FokForm)) \
             (Cons FokForm {instantiated_atom} (Nil FokForm))) \
           (FokInit Zero Zero) \
           (Nil FokCert)"
    ))
    .expect("accepted Init leaf");
    env.elaborate_decl(&format!(
        "const {child} : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm {instantiated_body} (Nil FokForm))) \
           (FokImpRight Zero) \
           (Cons FokCert {leaf} (Nil FokCert))"
    ))
    .expect("accepted substituted recursive child");
    env.elaborate_decl(&format!(
        "const {root} : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm {target} (Nil FokForm))) \
           (FokForallRight Zero (FokQParameter Zero)) \
           (Cons FokCert {child} (Nil FokCert))"
    ))
    .expect("ForallRight root");
    env.elaborate_decl(&format!(
        "theorem {name}_checker_ok \
           : Equal Bool (fok_check_cert {target} {root}) True = Proved"
    ))
    .expect("fresh eigenparameter and recursive child must be accepted");
    env.elaborate_decl(&format!(
        "theorem {name}_soundness_live : fok_classically_valid {target} = \
         fok_checker_soundness {target} {root} {name}_checker_ok"
    ))
    .expect("the full theorem must cover this ForallRight target shape");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D4 must not add trust");
}

#[test]
fn accepted_forall_world_right_reaches_full_checker_soundness() {
    assert_forall_right_reaches_full_soundness("d4_world", "FokForallWorld");
}

#[test]
fn accepted_forall_object_right_reaches_full_checker_soundness() {
    assert_forall_right_reaches_full_soundness("d4_object", "FokForallObj");
}

#[test]
fn eigenparameter_in_conclusion_rejects_an_otherwise_valid_child() {
    let mut env = elaborate_fok();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_decl(
        "const d4_fresh_atom : FokForm = \
         FokAccess (FokQParameter Zero) (FokQParameter Zero)",
    )
    .expect("atom mentions the chosen eigenparameter");
    env.elaborate_decl(
        "const d4_fresh_body : FokForm = \
         FokImp d4_fresh_atom d4_fresh_atom",
    )
    .expect("otherwise-provable quantified body");
    env.elaborate_decl(
        "const d4_fresh_target : FokForm = \
         FokForallWorld d4_fresh_body",
    )
    .expect("freshness-violating target");
    env.elaborate_decl(
        "const d4_fresh_leaf : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Cons FokForm d4_fresh_atom (Nil FokForm)) \
             (Cons FokForm d4_fresh_atom (Nil FokForm))) \
           (FokInit Zero Zero) \
           (Nil FokCert)",
    )
    .expect("accepted Init leaf");
    env.elaborate_decl(
        "const d4_fresh_child : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm d4_fresh_body (Nil FokForm))) \
           (FokImpRight Zero) \
           (Cons FokCert d4_fresh_leaf (Nil FokCert))",
    )
    .expect("accepted recursive child");
    env.elaborate_decl(
        "theorem d4_fresh_child_ok \
           : Equal Bool \
             (fok_check_tree \
               (FokMkSequent \
                 (Nil FokForm) \
                 (Cons FokForm d4_fresh_body (Nil FokForm))) \
               d4_fresh_child) \
             True = Proved",
    )
    .expect("the child must not be the reason the root rejects");
    env.elaborate_decl(
        "const d4_fresh_root : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm d4_fresh_target (Nil FokForm))) \
           (FokForallRight Zero (FokQParameter Zero)) \
           (Cons FokCert d4_fresh_child (Nil FokCert))",
    )
    .expect("freshness-violating ForallRight root");
    env.elaborate_decl(
        "theorem d4_freshness_rejects \
           : Equal Bool \
             (fok_check_cert d4_fresh_target d4_fresh_root) False = Proved",
    )
    .expect("the conclusion occurrence must reject the eigenparameter");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "the freshness control must add no trust");
}
