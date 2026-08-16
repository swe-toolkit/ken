//! `V3-FO-CHECKER-SOUNDNESS`, `D1a` -- bounded finding, hard stop.
//!
//! `D1a`'s dispatch: author `FokDerivation : FokSequent -> Type`, generated
//! by exactly the rule variants `fok_check_rule`/`fok_check_forall_right`
//! validate -- same premises, same conclusion sequent shape, same
//! eigenparameter freshness condition (`AC-2`).
//!
//! **This could not be completed.** Every one of `FokDerivation`'s required
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
//! **Every reproduction below that actually triggers the divergence is
//! `#[ignore]`d, with an explicit resource-hazard warning in its reason
//! string.** They exist so this finding is independently, deliberately
//! re-verifiable (`cargo test -- --ignored <name>`, run only under an
//! external wall-clock/memory bound) rather than merely asserted -- but they
//! must never run in an ordinary `cargo test` invocation, scoped or not:
//! doing so would abort the whole test binary and, before that, consume the
//! shared box's memory (COORDINATION §12). The CONTROL cases (the two rows
//! that elaborate instantly) run normally, since they are fast and safe, and
//! are what makes the divergent rows a genuine isolation rather than an
//! unexplained crash.
//!
//! No touch to `fok_check_cert`/the checker/`attempt_fo`, no FO `Proved`, no
//! primitive/postulate/axiom/trusted-base addition, no
//! `embedding_adequacy`/`denote`/`Carriers`/`AtomEnv`, no slice widening, no
//! sort validation. `FoKripke.ken` is unchanged by this file (confirmed by
//! `git diff --stat` showing only this new test file).

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
// DIVERGENT CASES -- DO NOT RUN except deliberately, wall-clock/memory
// bounded (`timeout -k 2 20 <bin> <test_name> --exact --ignored`). Each
// consumed >10 GiB RSS and was still growing after ~100s when killed.
// ---------------------------------------------------------------------

/// `FokDerivation`'s ACTUAL required `FokDerivInit` shape, verbatim. This is
/// the real `D1a` deliverable, and it diverges.
#[test]
#[ignore = "RESOURCE HAZARD: diverges (>10 GiB RSS, still growing, no \
            termination observed within ~100s on a 1 GiB test-thread stack). \
            Run ONLY with an external timeout+memory bound. See this file's \
            module doc for the isolation. Do not remove #[ignore]."]
fn divergent_data_ctor_with_recursive_nth_form_equality() {
    let mut env = mk_env();
    let _ = env.elaborate_decl(
        "data DummyNthForm : Type where { \
           DummyNthFormMk : \
             (gamma : List FokForm) -> (left : Nat) -> (g : FokForm) -> \
             Equal (Option FokForm) (fok_nth_form gamma left) (Some FokForm g) -> \
             DummyNthForm \
         }",
    );
}

/// The MINIMAL possible repro: the smallest recursive function in the file
/// (`fok_nat_eq`, two non-nested match arms, no `List`/`Option` involved),
/// applied to abstract vars inside a trivial non-indexed, non-self-
/// referential `data` constructor. Diverges identically -- establishes the
/// pathology is general to "recursive fn applied to an abstract constructor-
/// telescope-bound var," not specific to `fok_nth_form`'s complexity, to
/// `List`/`Option`, or to indexing/self-reference.
#[test]
#[ignore = "RESOURCE HAZARD: diverges (>10 GiB RSS, still growing, no \
            termination observed within ~100s on a 1 GiB test-thread stack). \
            Run ONLY with an external timeout+memory bound. See this file's \
            module doc for the isolation. Do not remove #[ignore]."]
fn divergent_minimal_data_ctor_with_recursive_nat_eq_equality() {
    let mut env = mk_env();
    let _ = env.elaborate_decl(
        "data DummyNat : Type where { \
           DummyNatMk : \
             (a : Nat) -> (b : Nat) -> \
             Equal Bool (fok_nat_eq a b) True -> \
             DummyNat \
         }",
    );
}
