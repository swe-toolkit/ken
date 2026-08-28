//! `V3-FO-CHECKER-SOUNDNESS` D1a artifact and correspondence controls
//! (`AC-1`/`AC-2`), carried forward under
//! `V3-FO-SORTED-EIGENPARAMETER-DERIVATION`.
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
//!
//! `AC-FRESHNESS-ISOLATED` (`V3-FO-SORTED-EIGENPARAMETER-DERIVATION`). The rule
//! is now parameter-only sorted: `FokForallRight`'s eigen is a parameter INDEX
//! (`Nat`), and the derivation constructors take a `Nat` eigen too. The
//! ForallRight freshness controls inject the stale-eigen fault as `FokQParameter
//! 0` (the parameter already present in the body). Crucially, each body is
//! chosen so the stale eigen is WELL-SORTED for its binder and so `subst0` is a
//! no-op on it (the body names no `FokQBound 0`), which makes the reused child
//! certificate match under BOTH the fresh and the stale eigen. The stale
//! certificate is therefore rejected by the FRESHNESS guard ALONE, not by a
//! sort clash or a child-shape mismatch: the object body puts `FokQParameter 0`
//! in `FokForcingP`'s OBJECT slot (previously it sat in the WORLD slot, so the
//! object control rejected on a sort conflict independent of freshness and could
//! not fail on the property it named). Mutation evidence that the guard is what
//! bites lives in `..._freshness_guard_mutation_proof.rs`.

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

/// Builds a ForallRight correspondence pair for one binder kind.
///
/// `body` must satisfy two properties for the freshness assertion to be
/// ISOLATED (`AC-FRESHNESS-ISOLATED`):
///   1. it names `FokQParameter 0` in a slot WELL-SORTED for the binder's eigen
///      (a world slot under `FokForallWorld`, an object slot under
///      `FokForallObj`), so reusing parameter `0` as the eigen is sort-valid and
///      cannot reject on a sort clash; and
///   2. it names no `FokQBound 0`, so `subst0` is a no-op and the ONE reused
///      child certificate matches under both the fresh eigen and the stale
///      eigen `0` -- leaving the freshness guard as the sole discriminator.
/// `fresh_ix` is a `Nat` expression naming a parameter index absent from `body`.
fn check_forall_right_shape(
    prefix: &str,
    forall_ctor: &str,
    deriv_ctor: &str,
    body: &str,
    fresh_ix: &str,
) {
    let mut env = mk_env();
    let declarations = [
        format!("const {prefix}_gamma : List FokForm = Cons FokForm FokBottom (Nil FokForm)"),
        format!("const {prefix}_body : FokForm = {body}"),
        format!(
            "const {prefix}_delta : List FokForm = \
             Cons FokForm ({forall_ctor} {prefix}_body) \
               (Cons FokForm FokBottom (Nil FokForm))"
        ),
        format!(
            "const {prefix}_sequent : FokSequent = \
             FokMkSequent {prefix}_gamma {prefix}_delta"
        ),
        format!(
            "const {prefix}_child_delta : List FokForm = \
             fok_list_form_set_nth {prefix}_delta Zero \
               (fok_subst0_form {prefix}_body (FokQParameter ({fresh_ix})))"
        ),
        format!(
            "const {prefix}_child_sequent : FokSequent = \
             FokMkSequent {prefix}_gamma {prefix}_child_delta"
        ),
        format!(
            "fn {prefix}_constructor_probe \
                 (gamma : List FokForm) (delta : List FokForm) \
                 (right : Nat) (eigen : Nat) (body : FokForm) \
                 (lookup : Equal (Option FokForm) (fok_nth_form delta right) \
                   (Some FokForm ({forall_ctor} body))) \
                 (freshness : Equal Bool \
                   (fok_sequent_mentions_parameter (FokMkSequent gamma delta) \
                     (FokQParameter eigen)) False) \
                 (child : FokDerivation \
                   (FokMkSequent gamma \
                     (fok_list_form_set_nth delta right \
                       (fok_subst0_form body (FokQParameter eigen))))) \
               : FokDerivation (FokMkSequent gamma delta) = \
               {deriv_ctor} gamma delta right eigen body lookup freshness child"
        ),
        format!(
            "const {prefix}_child_cert : FokCert = \
             FokMkCert {prefix}_child_sequent (FokInit Zero (Suc Zero)) (Nil FokCert)"
        ),
        format!(
            "const {prefix}_cert : FokCert = \
             FokMkCert {prefix}_sequent (FokForallRight Zero ({fresh_ix})) \
               (Cons FokCert {prefix}_child_cert (Nil FokCert))"
        ),
        format!(
            "theorem {prefix}_checker_accepts : \
             Equal Bool (fok_check_tree {prefix}_sequent {prefix}_cert) True = Proved"
        ),
        format!(
            "const {prefix}_stale_eigen_cert : FokCert = \
             FokMkCert {prefix}_sequent (FokForallRight Zero Zero) \
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
    // World body: `Access` is World/World, so `FokQParameter 0` in either slot
    // is a well-sorted world eigen. Both slots are parameters (no `FokQBound 0`)
    // so `subst0` is a no-op and the stale eigen `0` is rejected on FRESHNESS
    // alone. Parameters `0` and `1` fill the body, so the fresh eigen is
    // parameter `2`, absent from the body.
    check_forall_right_shape(
        "d1a_world",
        "FokForallWorld",
        "FokDerivForallWorldRight",
        "FokAccess (FokQParameter (Suc Zero)) (FokQParameter Zero)",
        "Suc (Suc Zero)",
    );
}

#[test]
fn forall_obj_right_constructor_and_checker_reject_a_nonfresh_eigenparameter() {
    // Object body: `ForcingP` is World/Object. `FokQParameter 0` sits in the
    // OBJECT slot (previously the WORLD slot, which made the stale-eigen cert
    // reject on a World-vs-Object sort clash INDEPENDENT of freshness --
    // `AC-FRESHNESS-ISOLATED`). Parameter `1` fills the world slot; neither is
    // `FokQBound 0`, so `subst0` is a no-op. The stale eigen `0` is now
    // well-sorted for the object binder and rejected on FRESHNESS alone. Fresh
    // eigen is parameter `2`, absent from the body.
    check_forall_right_shape(
        "d1a_obj",
        "FokForallObj",
        "FokDerivForallObjRight",
        "FokForcingP (FokQParameter (Suc Zero)) (FokQParameter Zero)",
        "Suc (Suc Zero)",
    );
}
