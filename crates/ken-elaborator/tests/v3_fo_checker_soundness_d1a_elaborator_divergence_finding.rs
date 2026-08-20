//! `V3-FO-CHECKER-SOUNDNESS`, `D1a` finding and
//! `LANG-CTOR-PREMISE-ELABORATION-DIVERGES` D2 regression.
//!
//! `D1a`'s dispatch: author `FokDerivation : FokSequent -> Type`, generated
//! by exactly the rule variants `fok_check_rule`/`fok_check_forall_right`
//! validate -- same premises, same conclusion sequent shape, same
//! eigenparameter freshness condition (`AC-2`).
//!
//! **Before D2, this could not be completed.** Every one of `FokDerivation`'s required
//! constructors needs a premise of the shape `Equal <T> (<recursive fn> <free
//! vars>) <value>` -- e.g. `Equal (Option FokForm) (fok_nth_form gamma left)
//! (Some FokForm g)`, transcribing `fok_check_rule`'s own guards. Attempting
//! to elaborate `FokDerivation` with any such constructor causes the
//! elaborator to diverge: unbounded, still-growing memory use (observed
//! >10 GiB RSS and climbing after ~100s before being killed to protect the
//! shared box -- COORDINATION §12), not a legitimate deep-but-finite
//! computation, and not fixed by a larger test-thread stack (the
//! `run_with_big_stack` remedy this node family has used successfully for
//! every prior deep-computation issue does NOT help here).
//!
//! **Root cause, bisected empirically (this file's own method, not read off
//! the elaborator source) down to one precise axis:**
//!
//! | shape | data ctor telescope | `fn`/`theorem` parameter |
//! |---|---|---|
//! | non-recursive fn applied to abstract var | elaborates instantly | (not tested; irrelevant) |
//! | RECURSIVE fn applied to abstract var (even the smallest one, `fok_nat_eq`, two non-nested match arms) | **DIVERGES** | elaborates instantly |
//!
//! Indexing and self-reference are NOT implicated -- a trivial, non-indexed,
//! non-self-referential `data DummyNat : Type where { ... }` with a single
//! `Equal Bool (fok_nat_eq a b) True` premise diverges identically to
//! `FokDerivation` itself. The IDENTICAL premise, as an ordinary `fn`
//! parameter (not inside any `data` constructor), elaborates immediately.
//!
//! ⇒ **`data ... where` constructor-telescope elaboration takes a
//! qualitatively different code path than ordinary Pi-binder (`fn`/
//! `theorem`) parameter checking, and that path diverges on ANY premise
//! whose type applies a recursive user-defined function to a non-concrete
//! (abstract, constructor-telescope-bound) argument.** No inductive family
//! in this repository's existing test corpus (`explicit_data_elaboration.rs`'s
//! `Vector`/`CheckedSource` etc.) has a proof-carrying constructor of this
//! shape -- every existing example either has no premise argument at all, or
//! a nullary "proof marker" constructor (`SourceLengthOk : SourceLength bs
//! len`) rather than a computed `Equal`-typed hypothesis. This node is the
//! first to need it.
//!
//! **This is a hard stop for `D1a`, not a licence to patch the language or
//! restructure around it.** `AC-2` requires `FokDerivation`'s premises to be
//! the SAME checks `fok_check_rule` performs; there is no way to express
//! "`gamma[left] = Some g`" or "`fok_form_eq g d = True`" without applying a
//! recursive function to the constructor-bound `gamma`/`left`/`g`/`d`, so no
//! restructuring of `FokDerivation`'s signature avoids the pathological
//! shape (unlike the `D0`-derisked Bool-inversion restructuring, which
//! genuinely had an alternative signature).
//!
//! D2 replaces full normalization in strict positivity with WHNF-on-demand
//! plus delta-aware no-occurrence guards. The former hazardous rows now run in
//! ordinary CI and must terminate; the two original controls remain beside
//! them so disabling reduction entirely cannot masquerade as the repair.
//!
//! No touch to `fok_check_cert`/the checker/`attempt_fo`, no FO `Proved`, no
//! primitive/postulate/axiom/trusted-base addition, no
//! `embedding_adequacy`/`denote`/`Carriers`/`AtomEnv`, no slice widening, no
//! sort validation. `FoKripke.ken` is unchanged by this file (confirmed by
//! `git diff --stat` showing only this new test file).

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
