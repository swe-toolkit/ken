//! `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` `AC-3`/`AC-6`: wrong-sort refusal
//! controls, each paired with a near-identical accepted certificate.
//!
//! Promise class: durable invariant. Two distinct guarantees, kept separate.
//!
//! PUBLIC DIRECT REFUSAL (unmodified `fok_check_tree`, every row). The checker
//! REFUSES the bad certificate and ACCEPTS a near-identical one differing only
//! at the named injection point (`AC-6`, so the refusal is not an inert-checker
//! artefact). The injection points are distinct source coordinates even where
//! their English effect overlaps ("a world term where an object term belongs"):
//! the eigen slot of a `FokForallObj`/`FokForallWorld` node, an atom's argument
//! role, and a bound index beyond its binder depth. This public assertion does
//! NOT claim the bad tree has ONLY a sort defect.
//!
//! The malformed-atom and out-of-scope rows ARE single-guard (a sort/scope
//! error, no eigen). The wrong-sort EIGEN rows are OVER-DETERMINED: reusing a
//! parameter as the eigen makes it non-fresh as well as wrong-sorted, so the
//! public refusal alone cannot say which guard fired.
//!
//! TWO-STAGE SORT ISOLATION (eigen rows only, by mutation, after the public
//! assertion). To attribute the eigen refusal to the sort guard specifically:
//! (1) neutralize the freshness predicate but keep the sort check LIVE -- the
//! cert STILL rejects, so the sort guard is an independent cause; (2) neutralize
//! freshness AND collapse `fok_derived_sort_eq` -- the same cert ACCEPTS, so the
//! sort guard was the sole surviving cause. Freshness is neutralized only for
//! this isolation proof.
//!
//! These pair with the Rust-surface refusals exercised by the in-crate
//! `fo_kripke` unit tests; `AC-1`'s embedded refuting certificate is exercised
//! by `v3_fo_sorted_eigenparameter_refuting_cert.rs`.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    env_for(FOK_SOURCE)
}

fn env_for(source: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    catalog_or::load_core_logic_or(&mut env);
    env.elaborate_file(source)
        .expect("FoKripke.ken (possibly with a collapsed sort check) must elaborate");
    env
}

/// `AC-5` mutation: collapse the sort check by making `fok_derived_sort_eq`
/// declare every pair of sorts equal. World-vs-Object conflicts then compare
/// as agreement, so `fok_validate_qterm_sort` never rejects. The mutation
/// touches only the validation path; the structural checker and the reflection
/// theorems do not call it, so the file still elaborates.
const SORT_EQ_ORIG: &str = "\
fn fok_derived_sort_eq (a : FokDerivedSort) (b : FokDerivedSort) : Bool =
  match a {
    FokWorldSort ↦
      match b {
        FokWorldSort ↦ True;
        FokObjectSort ↦ False
      };
    FokObjectSort ↦
      match b {
        FokWorldSort ↦ False;
        FokObjectSort ↦ True
      }
  }";
const SORT_EQ_COLLAPSED: &str =
    "fn fok_derived_sort_eq (a : FokDerivedSort) (b : FokDerivedSort) : Bool = True";

/// Mutation: neutralize the EXACT freshness predicate by making
/// `fok_sequent_mentions_parameter` return the constant `False`, so no eigen is
/// ever "mentioned" and the freshness guard always passes. Independent of the
/// sort check, so the two can be applied together or separately.
const FRESHNESS_PREDICATE_BODY: &str =
    "      fok_or (fok_list_form_any_mentions gamma target) (fok_list_form_any_mentions delta target)";
const FRESHNESS_PREDICATE_NEUTERED: &str = "      False";

fn with_sort_collapsed(source: &str) -> String {
    assert_eq!(
        source.matches(SORT_EQ_ORIG).count(),
        1,
        "fok_derived_sort_eq anchor must occur exactly once -- re-measure if the source moved"
    );
    source.replace(SORT_EQ_ORIG, SORT_EQ_COLLAPSED)
}

fn with_freshness_neutralized(source: &str) -> String {
    assert_eq!(
        source.matches(FRESHNESS_PREDICATE_BODY).count(),
        1,
        "freshness predicate anchor must occur exactly once -- re-measure if the source moved"
    );
    source.replace(FRESHNESS_PREDICATE_BODY, FRESHNESS_PREDICATE_NEUTERED)
}

fn sort_collapsed_source() -> String {
    with_sort_collapsed(FOK_SOURCE)
}

/// True iff the checker verdict `Equal Bool (fok_check_tree sequent cert)
/// expected` is provable by `Proved` -- i.e. the checker actually returns
/// `expected` on this certificate.
fn verdict_is(env: &mut ElabEnv, name: &str, sequent: &str, cert: &str, expected: &str) -> bool {
    env.elaborate_decl(&format!(
        "theorem {name} : Equal Bool (fok_check_tree ({sequent}) ({cert})) {expected} = Proved"
    ))
    .is_ok()
}

/// One `AC-3`/`AC-6` pair: `accept_cert` is accepted, `reject_cert` (differing
/// only at the named injection point) is refused, on the same `sequent`.
fn assert_wrong_sort_pair(
    label: &str,
    accept_sequent: &str,
    accept_cert: &str,
    reject_sequent: &str,
    reject_cert: &str,
) {
    let mut env = mk_env();
    assert!(
        verdict_is(&mut env, &format!("{label}_accepts"), accept_sequent, accept_cert, "True"),
        "{label}: the sort-correct near-miss certificate must be ACCEPTED"
    );
    let mut env2 = mk_env();
    assert!(
        verdict_is(&mut env2, &format!("{label}_rejects"), reject_sequent, reject_cert, "False"),
        "{label}: the wrong-sort certificate must be REFUSED"
    );
    // AC-6 guard: the refusal is not a checker that refuses everything --
    // prove the reject cert is NOT also accepted.
    let mut env3 = mk_env();
    assert!(
        !verdict_is(&mut env3, &format!("{label}_not_accepted"), reject_sequent, reject_cert, "True"),
        "{label}: the wrong-sort certificate must not also be accepted"
    );
}

// Shared pieces. A ForallRight certificate over one binder, whose single child
// closes by `Init` on the trailing `FokBottom`. The single child is built with
// the SAME `eigen_ix` the rule carries, so the COHERENT child substitution
// belongs to the eigen coordinate: `child_delta = subst0(body, FokQParameter
// eigen_ix)` is exactly what the checker recomputes for that eigen, so the child
// matches by construction. A reject variant therefore fails on the eigen's SORT
// or on FRESHNESS -- never on a child-shape mismatch -- which is what lets the
// two-stage isolation below attribute the refusal to a specific guard.
fn forall_cert(forall_ctor: &str, body: &str, eigen_ix: &str) -> (String, String) {
    let gamma = "Cons FokForm FokBottom (Nil FokForm)";
    let delta = format!("Cons FokForm ({forall_ctor} ({body})) (Cons FokForm FokBottom (Nil FokForm))");
    let sequent = format!("FokMkSequent ({gamma}) ({delta})");
    let child_delta =
        format!("fok_list_form_set_nth ({delta}) Zero (fok_subst0_form ({body}) (FokQParameter ({eigen_ix})))");
    let child_cert =
        format!("FokMkCert (FokMkSequent ({gamma}) ({child_delta})) (FokInit Zero (Suc Zero)) (Nil FokCert)");
    let cert = format!(
        "FokMkCert ({sequent}) (FokForallRight Zero ({eigen_ix})) (Cons FokCert ({child_cert}) (Nil FokCert))"
    );
    (sequent, cert)
}

/// Public direct refusal PLUS two-stage SORT-isolation for a wrong-sort EIGEN
/// certificate.
///
/// The unmodified checker's public verdict is asserted first: the bad
/// certificate is REFUSED and not accepted. That is a refusal claim only -- the
/// tree is OVER-DETERMINED (reusing parameter `1`, which the body fixes to the
/// wrong sort, as the eigen makes it both wrong-sorted AND non-fresh), so the
/// public `= False` cannot say WHICH guard rejected it. The two stages then
/// isolate the sort guard by measurement:
///   - Stage 1: neutralize the freshness predicate but keep the sort check LIVE.
///     The certificate STILL rejects -- so the sort guard is an independent
///     cause, not merely masked by freshness. Freshness is neutralized here
///     ONLY for the isolation proof.
///   - Stage 2: neutralize freshness AND collapse `fok_derived_sort_eq`. The
///     SAME certificate now ACCEPTS -- confirming the sort guard was the sole
///     surviving discriminator (no third guard rejects it).
/// The lawful control (a fresh, correctly-sorted parameter `2` eigen) is kept
/// and accepted unmutated, so the refusals are not an inert-checker artefact.
fn assert_wrong_sort_eigen_isolated(label: &str, forall_ctor: &str, body: &str) {
    // Lawful control: fresh, correctly-sorted eigen (parameter 2), accepted.
    let (ok_seq, ok_cert) = forall_cert(forall_ctor, body, "Suc (Suc Zero)");
    let mut env = mk_env();
    assert!(
        verdict_is(&mut env, &format!("{label}_lawful_accepts"), &ok_seq, &ok_cert, "True"),
        "{label}: the lawful fresh correctly-sorted eigen certificate must be ACCEPTED"
    );

    // The wrong-sort eigen certificate: parameter 1, fixed to the wrong sort by
    // the body and non-fresh because it occurs there.
    let (bad_seq, bad_cert) = forall_cert(forall_ctor, body, "Suc Zero");

    // Public direct refusal on the UNMODIFIED checker, in separate environments.
    // The unmodified `fok_check_tree` refuses the bad certificate and does not
    // accept it. This asserts refusal only -- NOT that the sole cause is sort
    // (the tree is over-determined); the two stages below make that attribution.
    let mut env_pub_reject = mk_env();
    assert!(
        verdict_is(&mut env_pub_reject, &format!("{label}_public_rejects"), &bad_seq, &bad_cert, "False"),
        "{label}: the unmodified checker must REFUSE the wrong-sort eigen certificate"
    );
    let mut env_pub_not_accept = mk_env();
    assert!(
        !verdict_is(&mut env_pub_not_accept, &format!("{label}_public_not_accepted"), &bad_seq, &bad_cert, "True"),
        "{label}: the unmodified checker must not also accept it"
    );

    // Stage 1: freshness neutralized, sort LIVE -> still rejects.
    let mut env1 = env_for(&with_freshness_neutralized(FOK_SOURCE));
    assert!(
        verdict_is(&mut env1, &format!("{label}_stage1_sort_rejects"), &bad_seq, &bad_cert, "False"),
        "{label}: with freshness neutralized and the sort check live, the wrong-sort eigen \
         certificate must STILL REJECT -- the sort guard rejects it independently of freshness"
    );

    // Stage 2: freshness neutralized AND sort collapsed -> same cert accepts.
    let mut env2 = env_for(&with_sort_collapsed(&with_freshness_neutralized(FOK_SOURCE)));
    assert!(
        verdict_is(&mut env2, &format!("{label}_stage2_accepts"), &bad_seq, &bad_cert, "True"),
        "{label}: with BOTH freshness and the sort check neutralized, the SAME certificate must \
         ACCEPT -- confirming the sort guard was the sole surviving cause of the refusal"
    );
}

#[test]
fn world_eigen_into_object_binder_is_refused_by_the_sort_guard() {
    // Object body: ForcingP(World=param1, Object=bound0). Injection point: the
    // eigen slot of the FokForallObj node -- reusing parameter 1 asks the OBJECT
    // binder to take the parameter the world slot fixed as WORLD.
    assert_wrong_sort_eigen_isolated(
        "world_eigen_into_object_binder",
        "FokForallObj",
        "FokForcingP (FokQParameter (Suc Zero)) (FokQBound Zero)",
    );
}

#[test]
fn object_eigen_into_world_binder_is_refused_by_the_sort_guard() {
    // World body: ForcingP(World=bound0, Object=param1). Injection point: the
    // eigen slot of the FokForallWorld node -- reusing parameter 1 asks the
    // WORLD binder to take the parameter the object slot fixed as OBJECT.
    assert_wrong_sort_eigen_isolated(
        "object_eigen_into_world_binder",
        "FokForallWorld",
        "FokForcingP (FokQBound Zero) (FokQParameter (Suc Zero))",
    );
}

#[test]
fn malformed_atomic_argument_role_is_refused() {
    // Injection point: a `FokForcingP` atom's argument roles. Well-sorted uses a
    // world parameter and a distinct object parameter; the malformed variant
    // puts ONE parameter in both the world and the object slot, so it cannot be
    // consistently sorted. Carried by an `Init` cert that closes on the atom.
    let ok_atom = "FokForcingP (FokQParameter Zero) (FokQParameter (Suc Zero))";
    let bad_atom = "FokForcingP (FokQParameter Zero) (FokQParameter Zero)";
    let ok_forms = format!("Cons FokForm ({ok_atom}) (Nil FokForm)");
    let bad_forms = format!("Cons FokForm ({bad_atom}) (Nil FokForm)");
    let ok_seq = format!("FokMkSequent ({ok_forms}) ({ok_forms})");
    let bad_seq = format!("FokMkSequent ({bad_forms}) ({bad_forms})");
    let ok_cert = format!("FokMkCert ({ok_seq}) (FokInit Zero Zero) (Nil FokCert)");
    let bad_cert = format!("FokMkCert ({bad_seq}) (FokInit Zero Zero) (Nil FokCert)");
    assert_wrong_sort_pair(
        "malformed_atomic_argument_role",
        &ok_seq,
        &ok_cert,
        &bad_seq,
        &bad_cert,
    );
}

#[test]
fn out_of_scope_bound_reference_is_refused() {
    // Injection point: a bound index in an atom under a single binder. `Bound 0`
    // is in scope; `Bound 1` names a binder that does not exist, so the derived
    // sort lookup fails closed. Carried by an `Init` cert closing on the
    // quantified form.
    let ok_form = "FokForallObj (FokForcingP (FokQParameter Zero) (FokQBound Zero))";
    let bad_form = "FokForallObj (FokForcingP (FokQParameter Zero) (FokQBound (Suc Zero)))";
    let ok_forms = format!("Cons FokForm ({ok_form}) (Nil FokForm)");
    let bad_forms = format!("Cons FokForm ({bad_form}) (Nil FokForm)");
    let ok_seq = format!("FokMkSequent ({ok_forms}) ({ok_forms})");
    let bad_seq = format!("FokMkSequent ({bad_forms}) ({bad_forms})");
    let ok_cert = format!("FokMkCert ({ok_seq}) (FokInit Zero Zero) (Nil FokCert)");
    let bad_cert = format!("FokMkCert ({bad_seq}) (FokInit Zero Zero) (Nil FokCert)");
    assert_wrong_sort_pair(
        "out_of_scope_bound_reference",
        &ok_seq,
        &ok_cert,
        &bad_seq,
        &bad_cert,
    );
}

#[test]
fn collapsing_the_sort_check_reddens_the_wrong_sort_controls() {
    // AC-5: the sort refusals above are CAUSED by the sort check. Collapse it
    // and the malformed-atom certificate (which fails only on sort -- its Init
    // closes structurally) must flip from refused to accepted.
    let bad_atom = "FokForcingP (FokQParameter Zero) (FokQParameter Zero)";
    let bad_forms = format!("Cons FokForm ({bad_atom}) (Nil FokForm)");
    let bad_seq = format!("FokMkSequent ({bad_forms}) ({bad_forms})");
    let bad_cert = format!("FokMkCert ({bad_seq}) (FokInit Zero Zero) (Nil FokCert)");

    let mut base = mk_env();
    assert!(
        verdict_is(&mut base, "collapse_base_reject", &bad_seq, &bad_cert, "False"),
        "unmutated: the malformed-atom certificate is refused on sort"
    );

    let collapsed = sort_collapsed_source();
    let mut mutated = env_for(&collapsed);
    assert!(
        verdict_is(&mut mutated, "collapse_mut_accept", &bad_seq, &bad_cert, "True"),
        "with the sort check collapsed the same certificate must be ACCEPTED -- \
         proving the refusal was caused by the sort check"
    );

    // The malformed-atom certificate is the clean SINGLE-guard target here: its
    // `Init` closes structurally and no eigen is involved, so the ONLY reason the
    // checker refuses it is the sort clash, and collapsing the sort check alone
    // flips it. The wrong-sort EIGEN certificates are over-determined (wrong-sort
    // AND non-fresh), so they get the TWO-stage isolation above
    // (`assert_wrong_sort_eigen_isolated`): neutralize freshness to expose that
    // the sort guard rejects them independently, then collapse the sort check to
    // confirm it was the sole surviving cause.
}
