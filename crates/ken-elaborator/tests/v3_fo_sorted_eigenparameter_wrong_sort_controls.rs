//! `V3-FO-SORTED-EIGENPARAMETER-DERIVATION` `AC-3`/`AC-6`: wrong-sort refusal
//! controls, each paired with a near-identical accepted certificate.
//!
//! Promise class: durable invariant. Each row asserts the Ken checker
//! `fok_check_tree` REFUSES a certificate whose only defect is a sort error at
//! one named INJECTION POINT, and ACCEPTS a certificate that differs only in
//! that one coordinate (`AC-6`, so the refusal is not an artefact of an
//! unrelated malformed tree). The injection points are distinct source
//! coordinates even where their English effect overlaps ("a world term where an
//! object term belongs"): the eigen slot of a `FokForallObj`/`FokForallWorld`
//! node, an atom's argument role, and a bound index beyond its binder depth.
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

fn sort_collapsed_source() -> String {
    assert_eq!(
        FOK_SOURCE.matches(SORT_EQ_ORIG).count(),
        1,
        "fok_derived_sort_eq anchor must occur exactly once -- re-measure if the source moved"
    );
    FOK_SOURCE.replace(SORT_EQ_ORIG, SORT_EQ_COLLAPSED)
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
// closes by `Init` on the trailing `FokBottom`. The body names no `FokQBound 0`,
// so `subst0` is a no-op and the child matches any eigen; the reject variants
// therefore fail on SORT (the eigen slot), not on a child-shape mismatch.
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

#[test]
fn world_eigen_into_object_binder_is_refused() {
    // Object body: ForcingP(World=param1, Object=bound0). The sort-correct eigen
    // is a fresh OBJECT parameter (2). Injection point: the eigen slot of the
    // FokForallObj node -- reusing parameter 1 there asks the OBJECT binder to
    // take the parameter the world slot fixed as WORLD.
    let body = "FokForcingP (FokQParameter (Suc Zero)) (FokQBound Zero)";
    let (accept_seq, accept_cert) = forall_cert("FokForallObj", body, "Suc (Suc Zero)");
    let (reject_seq, reject_cert) = forall_cert("FokForallObj", body, "Suc Zero");
    assert_wrong_sort_pair(
        "world_eigen_into_object_binder",
        &accept_seq,
        &accept_cert,
        &reject_seq,
        &reject_cert,
    );
}

#[test]
fn object_eigen_into_world_binder_is_refused() {
    // World body: ForcingP(World=bound0, Object=param1). The sort-correct eigen
    // is a fresh WORLD parameter (2). Injection point: the eigen slot of the
    // FokForallWorld node -- reusing parameter 1 asks the WORLD binder to take
    // the parameter the object slot fixed as OBJECT.
    let body = "FokForcingP (FokQBound Zero) (FokQParameter (Suc Zero))";
    let (accept_seq, accept_cert) = forall_cert("FokForallWorld", body, "Suc (Suc Zero)");
    let (reject_seq, reject_cert) = forall_cert("FokForallWorld", body, "Suc Zero");
    assert_wrong_sort_pair(
        "object_eigen_into_world_binder",
        &accept_seq,
        &accept_cert,
        &reject_seq,
        &reject_cert,
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

    // The malformed-atom certificate is the clean target here: its `Init` closes
    // structurally, so the ONLY reason the checker refuses it is the sort clash.
    // The wrong-sort EIGEN controls are deliberately NOT reused for this
    // mutation -- reusing a parameter as an eigen makes it non-fresh as well as
    // wrong-sorted, so the freshness guard still refuses them after the sort
    // check is collapsed (that entanglement is exactly why AC-FRESHNESS-ISOLATED
    // uses a Bound-0-free, well-sorted body instead).
}
