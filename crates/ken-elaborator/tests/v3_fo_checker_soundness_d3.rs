//! `V3-FO-CHECKER-SOUNDNESS` D3: the accepted propositional certificate
//! fragment produces the indexed derivation promised by `fok_checker_soundness`.
//!
//! Promise class: durable invariant. D4 replaced the former four-argument
//! transition sentinel with the full theorem application below; full-tree
//! fragment classification remains a stable description of the D3 partition.

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

/// Durable invariant: a propositional accepted tree remains covered by the
/// full theorem after D4 removes the fragment hypothesis.
#[test]
fn accepted_imp_right_init_tree_instantiates_full_checker_soundness() {
    let mut env = elaborate_fok();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_decl("const d3_q : FokForm = FokImp FokBottom FokBottom")
        .expect("propositional target");
    env.elaborate_decl(
        "const d3_init_child : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Cons FokForm FokBottom (Nil FokForm)) \
             (Cons FokForm FokBottom (Nil FokForm))) \
           (FokInit Zero Zero) \
           (Nil FokCert)",
    )
    .expect("accepted Init child");
    env.elaborate_decl(
        "const d3_cert : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm d3_q (Nil FokForm))) \
           (FokImpRight Zero) \
           (Cons FokCert d3_init_child (Nil FokCert))",
    )
    .expect("accepted ImpRight root");
    env.elaborate_decl(
        "theorem d3_fragment_ok \
           : Equal Bool (fok_cert_no_forall_right d3_cert) True = Proved",
    )
    .expect("the durable classifier still recognizes the D3 fragment");
    env.elaborate_decl(
        "theorem d3_checker_ok \
           : Equal Bool (fok_check_cert d3_q d3_cert) True = Proved",
    )
    .expect("the checker accepts the concrete Init/ImpRight tree");
    env.elaborate_decl(
        "theorem d3_soundness_live : fok_classically_valid d3_q = \
         fok_checker_soundness d3_q d3_cert d3_checker_ok",
    )
    .expect("the full theorem must preserve the accepted propositional case");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D3 must not add trust");
}

/// Durable invariant: a ForallRight below a propositional root is outside the
/// fragment, independently of whether D4 later proves that excluded rule.
fn assert_nested_forall_is_excluded(name: &str, quantified: &str) {
    let mut env = elaborate_fok();
    let child_name = format!("{name}_child");
    let root_name = format!("{name}_root");

    env.elaborate_decl(&format!(
        "const {child_name} : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm {quantified} (Nil FokForm))) \
           (FokForallRight Zero (FokQParameter Zero)) \
           (Nil FokCert)"
    ))
    .expect("nested ForallRight child");
    env.elaborate_decl(&format!(
        "const {root_name} : FokCert = \
         FokMkCert \
           (FokMkSequent \
             (Nil FokForm) \
             (Cons FokForm (FokImp FokBottom FokBottom) (Nil FokForm))) \
           (FokImpRight Zero) \
           (Cons FokCert {child_name} (Nil FokCert))"
    ))
    .expect("propositional root containing a quantified child");
    env.elaborate_decl(&format!(
        "theorem {name}_excluded \
           : Equal Bool (fok_cert_no_forall_right {root_name}) False = Proved"
    ))
    .expect("the fragment witness must inspect every child");
}

#[test]
fn fragment_witness_excludes_nested_forall_world_right() {
    assert_nested_forall_is_excluded(
        "d3_nested_world",
        "(FokForallWorld (FokImp FokBottom FokBottom))",
    );
}

#[test]
fn fragment_witness_excludes_nested_forall_object_right() {
    assert_nested_forall_is_excluded(
        "d3_nested_object",
        "(FokForallObj (FokImp FokBottom FokBottom))",
    );
}
