//! `V3-FO-CHECKER-SOUNDNESS`, `D0` -- the buildability probe.
//!
//! `23 §4.4` needs two Ken theorems, `embedding_adequacy` and
//! `checker_soundness`. `checker_soundness`'s statement (`23 §4.3`,
//! `:452-467`) needs three surface-level capabilities that nothing in this
//! tree has exercised yet:
//!
//! 1. an inductive **indexed by** `FokSequent` (`Derivation`'s Ken analogue,
//!    `FokDerivation : FokSequent -> Type`);
//! 2. propositional truncation `‖A‖`, so `Derives(s) : Omega := ‖ FokDerivation
//!    s ‖` is writable;
//! 3. a proof term that eliminates a `Equal Bool b True` hypothesis by cases
//!    on `b` -- the shape every step of `fok_check_rule`'s `fok_and`/`fok_or`
//!    unfolding will need at `D2`.
//!
//! This file establishes each of the three **independently, by actual `.ken`
//! elaboration** -- not by reading the kernel or the elaborator source -- and
//! reports each result on its own. A HARD STOP at any one of them is a
//! complete `D0` result (node doc, `AC-5`): this file does not patch the
//! language and does not begin authoring `FokDerivation`'s real rule set
//! (that is `D1`).
//!
//! No change to `fok_check_cert`/the checker, no FO `Proved` verdict, no new
//! primitive/postulate/axiom/trusted-base entry (`AC-1`, `AC-3`, `AC-4`).

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

fn load_fok(env: &mut ElabEnv) {
    env.elaborate_file(FOK_SOURCE).expect(
        "FoKripke.ken must still elaborate/kernel-check unmodified (D0 touches no D0-4 artifact)",
    );
}

// ---------------------------------------------------------------------
// Probe 1: an inductive indexed by `FokSequent`.
// ---------------------------------------------------------------------

/// `data FokDerivation : FokSequent -> Type where { ... }` -- a minimal,
/// deliberately-not-yet-real rule set (one constructor, no premises) whose
/// only job is to establish that the surface `data D : Idx -> Type where`
/// form (`34 §2`) accepts a *user-defined inductive* (`FokSequent`, not a
/// prelude type like `Nat`) as its index type, and that the kernel accepts
/// the resulting family. `D1` authors the real rule set; this is not it.
#[test]
fn probe1_fokderivation_indexed_by_foksequent_elaborates_and_kernel_checks() {
    let mut env = mk_env();
    load_fok(&mut env);

    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    let id = env
        .elaborate_decl(
            "data FokDerivation : FokSequent -> Type where { \
               FokDerivationPlaceholder : (s : FokSequent) -> FokDerivation s \
             }",
        )
        .expect("an inductive indexed by FokSequent must elaborate and kernel-check");

    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_trust, after_trust,
        "an ordinary indexed inductive must add no trusted_base entry"
    );

    let ind = env
        .env
        .inductive(id)
        .expect("FokDerivation must be registered as an inductive family");
    assert_eq!(
        ind.indices.len(),
        1,
        "FokDerivation must carry exactly one index (the FokSequent)"
    );
    assert_eq!(ind.constructors.len(), 1);
    assert_eq!(
        ind.constructors[0].target_indices.len(),
        1,
        "the constructor's target must record its FokSequent index"
    );
}

// ---------------------------------------------------------------------
// Probe 2: propositional truncation `‖A‖` from `.ken` surface syntax.
// ---------------------------------------------------------------------

/// The literal spec syntax: `Derives(s) : Omega := ‖ FokDerivation s ‖`
/// (`23 §4.3`, adapted to the Fok names). If this elaborates, `Derives`
/// itself must also be recorded and checked. If it does not, the exact
/// rejection is the `D0` result for this axis, and probe 2b establishes what
/// the surface *does* admit in its place.
#[test]
fn probe2_propositional_truncation_literal_spec_syntax() {
    let mut env = mk_env();
    load_fok(&mut env);
    env.elaborate_decl(
        "data FokDerivation : FokSequent -> Type where { \
           FokDerivationPlaceholder : (s : FokSequent) -> FokDerivation s \
         }",
    )
    .expect("probe 1 must hold for probe 2 to test the real Derives shape");

    let result =
        env.elaborate_decl("fn fok_derives (s : FokSequent) : Omega = ‖ FokDerivation s ‖");

    match result {
        Ok(_) => panic!(
            "propositional truncation unexpectedly elaborated from `.ken` surface syntax -- \
             the D0 report's answer to probe 2 must change from FAIL to PASS"
        ),
        Err(e) => {
            // Record the exact rejection so the D0 report cites it, not a
            // paraphrase. As of this probe: no lexer token exists for `‖`/`‖`
            // at all (`crates/ken-elaborator/src/lexer.rs` has no Trunc/‖
            // entry), so this must fail at LEXING, before any semantic
            // question about `FokDerivation` or `Omega` is even reached.
            let msg = e.to_string();
            eprintln!("probe2 (literal ‖A‖ syntax) rejected: {msg}");
        }
    }
}

/// Same statement, ASCII `|A|`-style spelling some Ken surface formers use
/// as an ASCII alternative to a Unicode delimiter -- checked independently
/// in case `‖ ‖` specifically has no token but a different spelling does.
#[test]
fn probe2b_propositional_truncation_ascii_spelling_probe() {
    let mut env = mk_env();
    load_fok(&mut env);
    env.elaborate_decl(
        "data FokDerivation : FokSequent -> Type where { \
           FokDerivationPlaceholder : (s : FokSequent) -> FokDerivation s \
         }",
    )
    .expect("probe 1 must hold for probe 2b to test the real Derives shape");

    for (label, src) in [
        (
            "double-pipe ||A||",
            "fn fok_derives_ascii (s : FokSequent) : Omega = ||FokDerivation s||",
        ),
        (
            "bare identifier Trunc",
            "fn fok_derives_trunc (s : FokSequent) : Omega = Trunc (FokDerivation s)",
        ),
    ] {
        let result = env.elaborate_decl(src);
        match result {
            Ok(_) => panic!(
                "ASCII truncation spelling '{label}' unexpectedly elaborated -- \
                 the D0 report's answer to probe 2 must change from FAIL to PASS"
            ),
            Err(e) => eprintln!("probe2b ({label}) rejected: {e}"),
        }
    }
}

// ---------------------------------------------------------------------
// Probe 3: eliminating a `Equal Bool b True` hypothesis by cases on `b`.
// ---------------------------------------------------------------------

/// The signature-restructured form: the case-split variable `b` is the
/// declared parameter, and the hypothesis is relegated to the RETURN type's
/// `Pi`-chain so it stays symbolic through the match and gets branch-refined
/// along with everything else in the goal (`34 §3.3`'s per-branch
/// definitional refinement covers the RESULT type, not other context
/// bindings -- this is the established restructuring technique, not the
/// naive shape).
#[test]
fn probe3_bool_true_hypothesis_eliminates_by_cases_restructured() {
    let mut env = mk_env();
    load_fok(&mut env);

    let before_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_decl(
        "theorem fok_bool_true_elim (b : Bool) : Equal Bool b True -> Equal Bool b True = \
           match b { \
             True ↦ λh. h; \
             False ↦ λh. absurd h \
           }",
    )
    .expect(
        "a proof term must be able to eliminate a `Equal Bool b True` hypothesis by cases on b \
         (False branch discharges via observational Equal-at-Bool reducing to Bottom)",
    );

    let after_trust: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_trust, after_trust,
        "the Bool-inversion proof must add no trusted_base entry"
    );
}

/// The naive pre-bound form: the hypothesis is a parameter declared BEFORE
/// the match on `b`, so a direct dependent match would need to refine an
/// already-bound context entry's type, not just the goal. Run independently
/// and reported separately -- whether this form also elaborates (full
/// context-dependent motive refinement) or requires the restructuring above
/// is itself part of the D0 result, not assumed either way.
#[test]
fn probe3b_bool_true_hypothesis_eliminates_by_cases_naive_prebound() {
    let mut env = mk_env();
    load_fok(&mut env);

    let result = env.elaborate_decl(
        "theorem fok_bool_true_elim_prebound (b : Bool) (h : Equal Bool b True) : Equal Bool b True = \
           match b { \
             True ↦ h; \
             False ↦ absurd h \
           }",
    );

    match result {
        Ok(_) => eprintln!(
            "probe3b: the naive pre-bound-hypothesis form ALSO elaborates -- \
             context-dependent motive refinement covers a prior binder, not only the goal"
        ),
        Err(e) => eprintln!(
            "probe3b: the naive pre-bound-hypothesis form is rejected ({e}) -- \
             the D2 proof will need the restructured form probe3 establishes"
        ),
    }
}
