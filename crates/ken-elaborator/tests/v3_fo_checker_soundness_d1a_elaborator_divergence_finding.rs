//! `V3-FO-CHECKER-SOUNDNESS`, `D1a` finding and
//! `LANG-CTOR-PREMISE-ELABORATION-DIVERGES` D2 regression.
//!
//! `D1a`'s dispatch: author `FokDerivation : FokSequent -> Type`, generated
//! by exactly the rule variants `fok_check_rule`/`fok_check_forall_right`
//! validate -- same premises, same conclusion sequent shape, same
//! eigenparameter freshness condition (`AC-2`).
//!
//! Before D2, every required constructor premise of the shape
//! `Equal <T> (<recursive fn> <free vars>) <value>` diverged during positivity
//! admission. The process exceeded 10 GiB RSS and a larger test-thread stack
//! did not help. Two single-factor controls isolated the conjunction: the same
//! recursive application in an ordinary `fn` parameter was instant, as was a
//! constructor premise using a non-recursive function.
//!
//! The minimal failing declaration was non-indexed and non-self-referential,
//! so neither indexing nor recursive-family occurrence caused the runaway.
//! Full normalization of the computed premise did: it descended into every
//! child of a stuck eliminator and attempted to materialize an irrelevant full
//! normal form before positivity inspected the type.
//!
//! D2 replaces full normalization in strict positivity with WHNF-on-demand
//! plus delta-aware no-occurrence guards. The former hazardous rows now run in
//! ordinary CI and must terminate; the two original controls remain beside
//! them so disabling reduction entirely cannot masquerade as the repair.
//!
//! No touch to `fok_check_cert`/the checker/`attempt_fo`, no FO `Proved`, no
//! primitive/postulate/axiom/trusted-base addition, no
//! `embedding_adequacy`/`denote`/`Carriers`/`AtomEnv`, no slice widening, no
//! sort validation. `FoKripke.ken` remains unchanged.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken must still elaborate/kernel-check unmodified");
    env
}

// ---------------------------------------------------------------------
// CONTROLS -- fast and safe, run normally.
// ---------------------------------------------------------------------

/// The exact premise `FokDerivation`'s `FokDerivInit` needs
/// (`Equal (Option FokForm) (fok_nth_form gamma left) (Some FokForm g)`), as
/// an ordinary `fn` parameter rather than a `data` constructor argument.
/// Elaborates instantly -- the recursive-function-applied-to-an-abstract-var
/// shape is not itself the problem outside a constructor telescope.
#[test]
fn control_fn_parameter_with_recursive_fn_equality_elaborates() {
    let mut env = mk_env();
    env.elaborate_decl(
        "fn probe_fn (gamma : List FokForm) (left : Nat) (g : FokForm) \
           (h : Equal (Option FokForm) (fok_nth_form gamma left) (Some FokForm g)) : Bool = True",
    )
    .expect("fn parameter with the exact FokDerivInit premise must elaborate");
}

/// The same constructor-telescope SHAPE (a `data` ctor premise applying a
/// user-defined function to an abstract var), but with a NON-recursive
/// function. Elaborates instantly -- recursion in the applied function is
/// the load-bearing factor, not merely "a function application in a data
/// ctor premise."
#[test]
fn control_data_ctor_with_nonrecursive_fn_equality_elaborates() {
    let mut env = mk_env();
    env.elaborate_decl("fn identity_nat (a : Nat) : Nat = a")
        .expect("identity_nat must elaborate");
    env.elaborate_decl(
        "data DummyId : Type where { \
           DummyIdMk : (a : Nat) -> Equal Nat (identity_nat a) a -> DummyId \
         }",
    )
    .expect("data ctor with a non-recursive fn equality must elaborate");
}

// ---------------------------------------------------------------------
// REPAIRED CASES -- these were resource-hazard regressions before D2 and now
// run in ordinary CI.
// ---------------------------------------------------------------------

/// `FokDerivation`'s actual required `FokDerivInit` shape, verbatim. D2 makes
/// the former resource-hazard case elaborate and kernel-check without trust
/// growth.
#[test]
fn data_ctor_with_recursive_nth_form_equality_elaborates() {
    let mut env = mk_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_decl(
        "data DummyNthForm : Type where { \
           DummyNthFormMk : \
             (gamma : List FokForm) -> (left : Nat) -> (g : FokForm) -> \
             Equal (Option FokForm) (fok_nth_form gamma left) (Some FokForm g) -> \
             DummyNthForm \
         }",
    )
    .expect("the exact FokDerivInit premise must elaborate after D2");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D2 must add no trusted-base entry");
}

/// The minimal former repro: the smallest recursive function in the file
/// (`fok_nat_eq`, two non-nested match arms, no `List`/`Option` involved),
/// applied to abstract variables inside a non-indexed, non-self-referential
/// `data` constructor. Before D2 it diverged identically.
#[test]
fn minimal_data_ctor_with_recursive_nat_eq_equality_elaborates() {
    let mut env = mk_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_decl(
        "data DummyNat : Type where { \
           DummyNatMk : \
             (a : Nat) -> (b : Nat) -> \
             Equal Bool (fok_nat_eq a b) True -> \
             DummyNat \
         }",
    )
    .expect("minimal recursive constructor premise must elaborate after D2");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "D2 must add no trusted-base entry");
}
