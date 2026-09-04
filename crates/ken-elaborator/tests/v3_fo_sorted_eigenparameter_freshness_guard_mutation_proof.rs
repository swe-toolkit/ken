//! `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` `AC-FRESHNESS-ISOLATED` mutation
//! proof.
//!
//! Promise class: durable invariant (a mutation control). It proves that the
//! `d1a` ForallRight stale-eigen controls reject their non-fresh eigenparameter
//! because of the FRESHNESS guard ALONE, not because of a sort clash or a
//! child-shape mismatch.
//!
//! The pins in `v3_fo_checker_soundness_d1a_rule_correspondence.rs` assert that
//! a stale eigen `FokQParameter 0` is REJECTED by both binder surfaces. On its
//! own, a `= False` assertion could hold for an unrelated reason -- exactly the
//! defect `AC-FRESHNESS-ISOLATED` names, where the object body once put
//! `FokQParameter 0` in a WORLD slot so the reject came from a sort conflict.
//! This test discriminates by MUTATION: it neuters the freshness predicate
//! `fok_sequent_mentions_parameter` to the constant `False` and shows the very
//! same stale certificate flips from REJECTED to ACCEPTED. Nothing else about
//! the certificate, the sorts, or the child changes -- so the freshness guard
//! is the sole cause of the reject.
//!
//! The mutation targets the freshness PREDICATE, not `fok_check_forall_right`'s
//! guard: the checker and the reflection theorem
//! `fok_check_forall_right_result_eq` both recompute the predicate, so mutating
//! it keeps the two sides of that `= Refl` convertible through the
//! checker-through-reflection prefix. The later semantic adequacy proof is
//! intentionally outside this mutation fixture: target soundness intrinsically
//! consumes freshness to justify its level-environment update, so a collapsed
//! freshness premise must invalidate that theorem rather than be ignored.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

/// The single freshness predicate line, and its neutered form. Every other
/// reference to `fok_sequent_mentions_parameter` in the
/// checker-through-reflection prefix -- the checker guard, the derivation
/// constructors' premise, and the reflection theorem -- recomputes through this
/// one definition, so replacing its body is the smallest compile-preserving
/// mutation of "the freshness check" for that behavioral surface.
const FRESHNESS_PREDICATE_BODY: &str =
    "      fok_or (fok_list_form_any_mentions gamma target) (fok_list_form_any_mentions delta target)";
const FRESHNESS_PREDICATE_NEUTERED: &str = "      False";

fn mutated_source() -> String {
    assert_eq!(
        FOK_SOURCE.matches(FRESHNESS_PREDICATE_BODY).count(),
        1,
        "freshness predicate anchor must occur exactly once -- re-measure if the source moved"
    );
    FOK_SOURCE.replace(FRESHNESS_PREDICATE_BODY, FRESHNESS_PREDICATE_NEUTERED)
}

/// The setup declarations that build a stale-eigen certificate for one binder
/// kind, mirroring the `d1a` correspondence controls: `FokQParameter 0` is
/// well-sorted for the binder and appears in the body, and the body names no
/// `FokQBound 0`, so `subst0` is a no-op and the single child certificate
/// matches the stale eigen. The only reason the checker rejects `stale_cert`
/// is that `0` occurs in the conclusion (freshness).
fn stale_cert_setup(prefix: &str, forall_ctor: &str, body: &str) -> Vec<String> {
    vec![
        format!("const {prefix}_gamma : List FokForm = Cons FokForm FokBottom (Nil FokForm)"),
        format!("const {prefix}_body : FokForm = {body}"),
        format!(
            "const {prefix}_delta : List FokForm = \
             Cons FokForm ({forall_ctor} {prefix}_body) (Cons FokForm FokBottom (Nil FokForm))"
        ),
        format!("const {prefix}_sequent : FokSequent = FokMkSequent {prefix}_gamma {prefix}_delta"),
        format!(
            "const {prefix}_child_delta : List FokForm = \
             fok_list_form_set_nth {prefix}_delta Zero \
               (fok_subst0_form {prefix}_body (FokQParameter Zero))"
        ),
        format!(
            "const {prefix}_child_sequent : FokSequent = \
             FokMkSequent {prefix}_gamma {prefix}_child_delta"
        ),
        format!(
            "const {prefix}_child_cert : FokCert = \
             FokMkCert {prefix}_child_sequent (FokInit Zero (Suc Zero)) (Nil FokCert)"
        ),
        format!(
            "const {prefix}_stale_cert : FokCert = \
             FokMkCert {prefix}_sequent (FokForallRight Zero Zero) \
               (Cons FokCert {prefix}_child_cert (Nil FokCert))"
        ),
    ]
}

fn env_for(source: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(source)
        .expect("FoKripke.ken (possibly mutated freshness predicate) must still elaborate");
    env
}

fn checker_prefix(source: &str) -> &str {
    source
        .split_once("-- === D2b: embedding adequacy proof (23 section 4.4) ===")
        .expect("the semantic adequacy section must follow checker soundness")
        .0
}

fn env_for_checker_mutation(source: &str) -> ElabEnv {
    env_for(checker_prefix(source))
}

/// Elaborate the setup, then report whether `fok_check_tree sequent stale_cert`
/// equals `expected` (`True` accepted / `False` rejected) by attempting the
/// corresponding `Proved` theorem. Returns `true` iff that verdict holds.
fn stale_verdict_is(env: &mut ElabEnv, prefix: &str, setup: &[String], expected: &str) -> bool {
    for decl in setup {
        env.elaborate_decl(decl)
            .unwrap_or_else(|err| panic!("setup declaration rejected:\n{decl}\n{err}"));
    }
    env.elaborate_decl(&format!(
        "theorem {prefix}_verdict_{expected} : \
         Equal Bool (fok_check_tree {prefix}_sequent {prefix}_stale_cert) {expected} = Proved"
    ))
    .is_ok()
}

fn assert_freshness_is_sole_cause(forall_ctor: &str, body: &str) {
    // Unmutated: the stale certificate is REJECTED (and NOT accepted).
    let mut base = env_for(FOK_SOURCE);
    let setup = stale_cert_setup("base", forall_ctor, body);
    assert!(
        stale_verdict_is(&mut base, "base", &setup, "False"),
        "unmutated checker must REJECT the stale eigen certificate"
    );
    let mut base2 = env_for(FOK_SOURCE);
    let setup2 = stale_cert_setup("base2", forall_ctor, body);
    assert!(
        !stale_verdict_is(&mut base2, "base2", &setup2, "True"),
        "unmutated checker must not also accept it"
    );

    // Freshness neutered: the very same certificate is now ACCEPTED (and NOT
    // rejected). Only the freshness predicate changed, so freshness is the sole
    // cause of the reject above.
    let mutated = mutated_source();
    let mut mut_env = env_for_checker_mutation(&mutated);
    let setup3 = stale_cert_setup("mut", forall_ctor, body);
    assert!(
        stale_verdict_is(&mut mut_env, "mut", &setup3, "True"),
        "with freshness neutered the stale eigen certificate must be ACCEPTED -- \
         proving the reject was caused by freshness, not sort or child shape"
    );
    let mut mut_env2 = env_for_checker_mutation(&mutated);
    let setup4 = stale_cert_setup("mut2", forall_ctor, body);
    assert!(
        !stale_verdict_is(&mut mut_env2, "mut2", &setup4, "False"),
        "with freshness neutered it must no longer be rejected"
    );
}

#[test]
fn object_stale_eigen_reject_is_caused_by_freshness_alone() {
    // Object body: `FokQParameter 0` in `ForcingP`'s OBJECT slot (the de-masked
    // shape). Well-sorted for the object binder, so no sort clash pre-empts the
    // freshness guard.
    assert_freshness_is_sole_cause(
        "FokForallObj",
        "FokForcingP (FokQParameter (Suc Zero)) (FokQParameter Zero)",
    );
}

#[test]
fn world_stale_eigen_reject_is_caused_by_freshness_alone() {
    // World body: `FokQParameter 0` in an `Access` (World/World) slot.
    assert_freshness_is_sole_cause(
        "FokForallWorld",
        "FokAccess (FokQParameter (Suc Zero)) (FokQParameter Zero)",
    );
}
