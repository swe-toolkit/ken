//! `V3-FO-CHECKER-SOUNDNESS`, `D2` -- the Bool-inversion infrastructure.
//!
//! **Polarity census, corrected from this file's first draft (Architect
//! `evt_6dhccr5qrrav`).** Neither `fok_and` nor `fok_or` is called directly
//! by `fok_check_rule` (`catalog/packages/Tooling/Verification/
//! FoKripke.ken:572-633`) -- the routes are:
//!
//! - `fok_and` reaches checker acceptance through `fok_form_eq`/
//!   `fok_qterm_eq`, which `FokInit`'s guard (`:583`) consumes AT `= True`:
//!   `fok_form_eq g d`.
//! - `fok_or` reaches the checker only through the `*_mentions_parameter`
//!   freshness chain, whose sole external consumer is
//!   `fok_check_forall_right`'s guard (`:554`):
//!   `match fok_sequent_mentions_parameter conclusion eigen { True -> False;
//!   False -> ... }`. **On every path where the checker ACCEPTS a
//!   `ForallRight` step, that `or`-built chain is `False`, not `True`.**
//!
//! | hypothesis | consequence | shape | delivered here |
//! |---|---|---|---|
//! | `fok_and a b = True` | `a = True` and `b = True` | two projections | **yes, and needed** (`FokInit`) |
//! | `fok_or a b = False` | `a = False` and `b = False` | two projections | **yes, and needed** (`ForallRight` acceptance) |
//! | `fok_or a b = True` | `a = True` or `b = True` | CPS eliminator | yes, kept for completeness -- **no checker consumer** |
//! | `fok_and a b = False` | `a = False` or `b = False` | CPS eliminator | **not delivered** -- would descend into `fok_qterm_eq`/`fok_form_eq` under a freshness mismatch; whether it is needed is a claim about a proof not yet written |
//!
//! This increment authors, in `FoKripke.ken`:
//!
//! - `fok_and_left`/`fok_and_right`   : `Equal Bool (fok_and a b) True -> Equal Bool <side> True`
//! - `fok_or_left_false`/`fok_or_right_false` : `Equal Bool (fok_or a b) False -> Equal Bool <side> False`
//! - `fok_or_elim` : the `fok_or` `= True` disjunction eliminator (retained
//!   from the first draft -- a true, kernel-checked lemma, costs nothing,
//!   just not consumed by the checker)
//!
//! Every lemma places its equality hypothesis (and, for `fok_or_elim`, both
//! continuations) in the RETURN type's `Pi`-chain, case-splitting on exactly
//! the variable `fok_and`/`fok_or` themselves scrutinize (`a`) -- the only
//! signature shape that elaborates (`D0` part (3): `34 §3.3`'s per-branch
//! definitional refinement covers a branch's *result type*, not a
//! hypothesis pre-bound before the match).
//!
//! Each lemma is proved TWICE here: once in the working, restructured shape
//! (asserted `Ok`, and this is the real `FoKripke.ken` deliverable), and once
//! in the naive pre-bound shape (asserted `Err`) -- a failable control per
//! lemma, not merely re-citing `D0`'s one demonstration.
//!
//! No Ω-sorted eliminator is built or relied on here: both new `= False`
//! lemmas are direct `theorem` implications (their own conclusion is already
//! `Omega`-classified `Equal`), not continuation-passing eliminators, so the
//! `Type`-vs-`Omega` target-sort question the Architect flagged for
//! `fok_or_elim` (unmeasured, explicitly not required) does not arise for
//! either new lemma.
//!
//! Ordinary `fn`/`theorem` work only: no `data`, no `FokDerivation`, no
//! truncation spelling, no touch to `fok_check_cert`/its callees/the Rust
//! reference/`attempt_fo`, no FO `Proved`, no primitive/postulate/axiom/
//! trusted-base addition, no `embedding_adequacy`/`denote`/`Carriers`/
//! `AtomEnv`, no slice widening, no sort validation.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken (with D2's Bool-inversion lemmas) must elaborate/kernel-check");
    env
}

// ---------------------------------------------------------------------
// The five lemmas already elaborated as part of `FOK_SOURCE` above (that IS
// the D2 deliverable). This test additionally pins the trusted_base delta
// and that all five names are registered.
// ---------------------------------------------------------------------

#[test]
fn all_five_lemmas_elaborate_with_zero_trusted_base_delta() {
    let mut env = ElabEnv::new().expect("base env");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken with D2's lemmas must elaborate/kernel-check");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "ordinary theorem/fn lemmas over existing definitions must add no trusted_base entry"
    );
    for name in [
        "fok_and_left",
        "fok_and_right",
        "fok_or_left_false",
        "fok_or_right_false",
        "fok_or_elim",
    ] {
        assert!(
            env.globals.contains_key(name),
            "{name} must be registered in globals"
        );
    }
}

// ---------------------------------------------------------------------
// Concrete-instance sanity checks -- secondary evidence only. The PRIMARY
// evidence that each lemma is correct is that its own fully general,
// abstract-`a`/`b` statement above elaborates and kernel-checks; a ground
// instantiation could pass "by incidental normalization" even if the
// general statement were subtly wrong in a way these small cases don't
// probe, so these are not a substitute for that general statement.
// ---------------------------------------------------------------------

#[test]
fn fok_and_left_applies_to_a_concrete_true_true_instance() {
    let mut env = mk_env();
    env.elaborate_decl(
        "theorem and_left_instance : Equal Bool True True = fok_and_left True True Proved",
    )
    .expect("fok_and_left applied to a concrete True/True instance must kernel-check");
}

#[test]
fn fok_and_right_applies_to_a_concrete_true_true_instance() {
    let mut env = mk_env();
    env.elaborate_decl(
        "theorem and_right_instance : Equal Bool True True = fok_and_right True True Proved",
    )
    .expect("fok_and_right applied to a concrete True/True instance must kernel-check");
}

#[test]
fn fok_or_left_false_applies_to_a_concrete_false_false_instance() {
    let mut env = mk_env();
    // a = False, b = False: fok_or False False = False.
    env.elaborate_decl(
        "theorem or_left_false_instance : Equal Bool False False = \
           fok_or_left_false False False Proved",
    )
    .expect("fok_or_left_false applied to a concrete False/False instance must kernel-check");
}

#[test]
fn fok_or_right_false_applies_to_a_concrete_false_false_instance() {
    let mut env = mk_env();
    // a = False, b = False: fok_or False False = False.
    env.elaborate_decl(
        "theorem or_right_false_instance : Equal Bool False False = \
           fok_or_right_false False False Proved",
    )
    .expect("fok_or_right_false applied to a concrete False/False instance must kernel-check");
}

#[test]
fn fok_or_elim_left_branch_applies_to_a_concrete_instance() {
    let mut env = mk_env();
    // a = True, b = False: fok_or True False = True. `target` is Type-sorted
    // (fok_or_elim is proof-relevant, unlike the four Omega-classified
    // theorems above), so instantiate it at Nat, not at an Equal-typed
    // (Omega-sorted) goal -- feed the True proof to the left continuation,
    // which returns Zero.
    env.elaborate_decl(
        "const or_elim_left_instance : Nat = \
           fok_or_elim True False Nat Proved \
             (λh. Zero) (λh. Suc Zero)",
    )
    .expect("fok_or_elim's left branch on a concrete instance must kernel-check");
}

#[test]
fn fok_or_elim_right_branch_applies_to_a_concrete_instance() {
    let mut env = mk_env();
    // a = False, b = True: fok_or False True = True (reduces to b). Feed the
    // True proof to the right continuation, which returns Suc Zero.
    env.elaborate_decl(
        "const or_elim_right_instance : Nat = \
           fok_or_elim False True Nat Proved \
             (λh. Zero) (λh. Suc Zero)",
    )
    .expect("fok_or_elim's right branch on a concrete instance must kernel-check");
}

// ---------------------------------------------------------------------
// FAILABLE CONTROL: the naive pre-bound-hypothesis form is rejected for
// EVERY lemma this increment adds -- not merely re-citing D0's one
// demonstration. Each hypothesis/continuation is declared as an ordinary
// parameter BEFORE the match on `a`, so the branch match cannot refine its
// type; the branch bodies below are exactly what would elaborate if
// refinement reached pre-bound context entries (it does not).
// ---------------------------------------------------------------------

#[test]
fn fok_and_left_naive_prebound_form_is_kernel_rejected() {
    let mut env = mk_env();
    let result = env.elaborate_decl(
        "theorem fok_and_left_naive (a : Bool) (b : Bool) (h : Equal Bool (fok_and a b) True) \
           : Equal Bool a True = \
           match a { \
             True ↦ Proved; \
             False ↦ h \
           }",
    );
    assert!(
        result.is_err(),
        "the naive pre-bound-hypothesis form of fok_and_left must be rejected, not silently accepted"
    );
    eprintln!(
        "fok_and_left naive pre-bound form rejected: {}",
        result.unwrap_err()
    );
}

#[test]
fn fok_and_right_naive_prebound_form_is_kernel_rejected() {
    let mut env = mk_env();
    let result = env.elaborate_decl(
        "theorem fok_and_right_naive (a : Bool) (b : Bool) (h : Equal Bool (fok_and a b) True) \
           : Equal Bool b True = \
           match a { \
             True ↦ h; \
             False ↦ absurd h \
           }",
    );
    assert!(
        result.is_err(),
        "the naive pre-bound-hypothesis form of fok_and_right must be rejected, not silently accepted"
    );
    eprintln!(
        "fok_and_right naive pre-bound form rejected: {}",
        result.unwrap_err()
    );
}

#[test]
fn fok_or_left_false_naive_prebound_form_is_kernel_rejected() {
    let mut env = mk_env();
    let result = env.elaborate_decl(
        "theorem fok_or_left_false_naive (a : Bool) (b : Bool) \
           (h : Equal Bool (fok_or a b) False) : Equal Bool a False = \
           match a { \
             True ↦ absurd h; \
             False ↦ Proved \
           }",
    );
    assert!(
        result.is_err(),
        "the naive pre-bound-hypothesis form of fok_or_left_false must be rejected, not silently accepted"
    );
    eprintln!(
        "fok_or_left_false naive pre-bound form rejected: {}",
        result.unwrap_err()
    );
}

#[test]
fn fok_or_right_false_naive_prebound_form_is_kernel_rejected() {
    let mut env = mk_env();
    let result = env.elaborate_decl(
        "theorem fok_or_right_false_naive (a : Bool) (b : Bool) \
           (h : Equal Bool (fok_or a b) False) : Equal Bool b False = \
           match a { \
             True ↦ absurd h; \
             False ↦ h \
           }",
    );
    assert!(
        result.is_err(),
        "the naive pre-bound-hypothesis form of fok_or_right_false must be rejected, not silently accepted"
    );
    eprintln!(
        "fok_or_right_false naive pre-bound form rejected: {}",
        result.unwrap_err()
    );
}

#[test]
fn fok_or_elim_naive_prebound_form_is_kernel_rejected() {
    let mut env = mk_env();
    let result = env.elaborate_decl(
        "fn fok_or_elim_naive (a : Bool) (b : Bool) (target : Type) \
           (h : Equal Bool (fok_or a b) True) \
           (ka : Equal Bool a True -> target) (kb : Equal Bool b True -> target) : target = \
           match a { \
             True ↦ ka h; \
             False ↦ kb h \
           }",
    );
    assert!(
        result.is_err(),
        "the naive pre-bound-hypothesis/continuation form of fok_or_elim must be rejected, not silently accepted"
    );
    eprintln!(
        "fok_or_elim naive pre-bound form rejected: {}",
        result.unwrap_err()
    );
}
