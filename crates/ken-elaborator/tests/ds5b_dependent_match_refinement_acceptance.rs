//! DS-5b acceptance: dependent-match index refinement (constructor
//! injectivity + sibling convoy), `docs/program/wp/ds-5b-dependent-match-
//! refinement.md`.
//!
//! `check_match_dependent`'s motive recovery previously refined only the
//! scrutinee's own index (spec `34-data-match §3.2`); it could not (1)
//! re-type a branch's own peeled recursive field via constructor
//! injectivity, or (2) re-type an outer sibling binder sharing the same
//! index (the "convoy" case). Both are carried into the local context via
//! the kernel's own `Eq`/`J`/`Cast` (`16`) — never postulated.
//!
//! Coverage:
//! - AC-injectivity: `tail`-shaped peeled-recursive-field re-typing.
//! - AC-convoy: a sibling binder re-typed through a nested match.
//! - AC-goal: a branch that constructs a fresh family value against an
//!   index-dependent goal (needs its own goal refined, not a context
//!   variable — the third, narrower capability this WP's construction
//!   also required).
//! - AC8: an unlicensed equation is never fabricated — a genuinely
//!   ill-typed program stays kernel-rejected.
//! - Non-indexed inertness: `List`/`Bool` matches are unaffected (implicitly
//!   covered by the full pre-existing suite staying green; direct check
//!   here too).

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::EvalVal;
use ken_kernel::KernelError;
use std::collections::BTreeSet;

/// Structural equality on evaluated `Ctor` values, ignoring the K3 interning
/// `slot` (which is store-assignment-order-dependent, not content-derived
/// across two independently-evaluated bodies sharing one store). Two
/// evaluations of textually distinct but structurally identical terms are
/// expected to be `EvalVal`-equal in every field EXCEPT `slot`.
fn vec_nat_structurally_eq(a: &EvalVal, b: &EvalVal) -> bool {
    match (a, b) {
        (
            EvalVal::Ctor {
                id: id_a,
                args: args_a,
                ..
            },
            EvalVal::Ctor {
                id: id_b,
                args: args_b,
                ..
            },
        ) => {
            id_a == id_b
                && args_a.len() == args_b.len()
                && args_a
                    .iter()
                    .zip(args_b.iter())
                    .all(|(x, y)| vec_nat_structurally_eq(x, y))
        }
        (EvalVal::Int(x), EvalVal::Int(y)) => x == y,
        (EvalVal::BigInt(x), EvalVal::BigInt(y)) => x == y,
        _ => a == b,
    }
}

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

fn elab_ok(env: &mut ElabEnv, src: &str) {
    env.elaborate_decl(src)
        .unwrap_or_else(|e| panic!("elaboration failed: {}", e));
}

fn expect_err_val(env: &mut ElabEnv, src: &str) -> ElabError {
    env.elaborate_decl(src)
        .expect_err("declaration unexpectedly elaborated")
}

fn vec_env() -> ElabEnv {
    let mut env = mk_env();
    elab_ok(
        &mut env,
        "data Vec (A : Type) : Nat -> Type where { \
           VNil : Vec A 0; \
           VCons : (n : Nat) -> A -> Vec A n -> Vec A (n+1) \
         }",
    );
    env
}

/// AC-injectivity: `tail`'s `VCons` branch peels `Suc m = Suc n` (via the
/// kernel's own `eq_at_inductive` same-constructor case) to re-type the
/// recursive field `ys : Vec A m` up to the goal `Vec A n` — the exact
/// capability DS-5 named as blocked.
#[test]
fn tail_constructor_injectivity_retypes_peeled_recursive_field() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn tail (A : Type) (n : Nat) (xs : Vec A (Suc n)) : Vec A n = \
         match xs { VCons m y ys |-> ys }",
    );
}

/// AC-convoy: matching `v : Vec Nat n` refines `n`; the sibling `w : Vec
/// Nat n` (an outer, independently-bound function parameter, never
/// destructured by the outer match) must refine in lockstep so the nested
/// match on `w` stays exhaustive without an explicit (impossible) `VNil`
/// arm. Un-refined, this is `ExhaustivenessError` on the omitted `VNil`.
#[test]
fn sibling_convoy_retypes_outer_binder_through_nested_match() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn firstIsSecond (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Bool = \
         match v { \
           VNil |-> True; \
           VCons m a xs |-> match w { VCons _ b ys |-> True } \
         }",
    );
}

/// AC-goal: a branch that constructs a FRESH family value (`VNil Nat`, the
/// base case a real `zip`-shaped function needs) has no existing context
/// binding for capability 1/2 to redirect — its natural type uses the
/// constructor's own target index, not the caller's un-refined index
/// variable. The checking goal itself must be refined (then the result
/// cast back up), not just a context variable.
#[test]
fn base_case_construction_retypes_the_checking_goal() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn firstIsVNil (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { VNil |-> VNil Nat; VCons m a xs |-> v }",
    );
}

/// AC8 (over-refinement discriminator): a goal that requires an equation
/// the branch does NOT license must stay rejected — `ys`'s only provable
/// re-typing (via the `Suc m = Suc n` premise) is `Vec Nat n`, never `Vec
/// Nat (Suc n)`. No cast is ever fabricated from thin air (every `Cast`
/// this WP builds carries a real `J`-derived proof of a real premise), so
/// this must still be a genuine kernel rejection, not a silent accept.
#[test]
fn over_refinement_stays_kernel_rejected() {
    let mut env = vec_env();
    let err = expect_err_val(
        &mut env,
        "fn wrongGoal (n : Nat) (xs : Vec Nat (Suc n)) : Vec Nat (Suc n) = \
         match xs { VCons m y ys |-> ys }",
    );
    assert!(
        matches!(
            &err,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "expected a kernel TypeMismatch rejection (the only equation the \
         Suc m = Suc n premise licenses is m = n, never m = Suc n), got: \
         {err:?}"
    );
}

/// Zero-`Axiom` acceptance bar (DS-2-style executable before==after
/// `trusted_base()` set-diff, mirroring `ds2_ord_nat_acceptance.rs`):
/// injectivity discharges through the kernel's own `Eq`/`J`/`Cast` (`16`)
/// — never a postulate — so elaborating all three DS-5b capabilities
/// together must introduce ZERO new `trusted_base()` entries. A `Cast`/`J`
/// construction can in principle require a postulate if a proof
/// obligation doesn't discharge cleanly; this is the machine-checked
/// claim that it didn't, not just an absence of `Axiom` in the diff.
#[test]
fn trusted_base_delta_is_empty_across_all_three_capabilities() {
    let mut env = vec_env();
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    elab_ok(
        &mut env,
        "fn tail (A : Type) (n : Nat) (xs : Vec A (Suc n)) : Vec A n = \
         match xs { VCons m y ys |-> ys }",
    );
    elab_ok(
        &mut env,
        "fn firstIsSecond (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Bool = \
         match v { \
           VNil |-> True; \
           VCons m a xs |-> match w { VCons _ b ys |-> True } \
         }",
    );
    elab_ok(
        &mut env,
        "fn firstIsVNil (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { VNil |-> VNil Nat; VCons m a xs |-> v }",
    );
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "injectivity + convoy + goal-refinement must introduce ZERO new \
         trusted_base() entries (zero-Axiom acceptance bar)"
    );
}

/// Non-indexed inertness: a `List`/`Bool` match (no index to refine) must
/// still elaborate — the new equation-in-context machinery is gated on the
/// family actually having indices (`ind.indices.len() > 0` inside
/// `method_index_premise_pairs`) and must never fire on a non-indexed
/// family. This is the same guarantee the full pre-existing suite already
/// exercises broadly; pinned directly here too.
#[test]
fn non_indexed_match_stays_unaffected() {
    let mut env = mk_env();
    elab_ok(
        &mut env,
        "fn allTrue (xs : List Bool) : Prop = \
         match xs { Nil |-> Equal Bool True True ; \
                    Cons b bs |-> And (Equal Bool b True) (allTrue bs) }",
    );
}

/// `LANG-CONVOY-ENCLOSING-FIELD` D1 / `LANG-CONVOY-MATCH-FIELD-PROVENANCE`
/// D1 + D4 + `AC-1` -- the two-vector `zip` recursive step spec
/// `34-data-match.md §3.2`'s Boundary paragraph names as a known gap: the
/// inner match on the sibling `w` destructures `w` through its own nested
/// match, and the same branch's body re-uses `xs` -- a field the ENCLOSING
/// (outer, `v`) match already bound -- in the same expression (the
/// recursive call).
///
/// **Before this node's remedy (error shape, kernel-observed).** Measured
/// at `LANG-CONVOY-ENCLOSING-FIELD`'s base (`6275bbc35`): the declaration
/// failed a kernel `TypeMismatch` where `expected` and `found` were the
/// SAME HEAD, differing only in the trailing de Bruijn index
/// (`((Dg574 Dg67) @9)` vs `((Dg574 Dg67) @4)`) -- not just the bare error
/// variant, so a regression on an unrelated arm could not have kept a
/// looser assertion green while reading as "the known gap is unchanged."
///
/// **Error location was NOT established by the kernel error itself.** Its
/// `span` was `0..167`, the entire 167-character declaration; it names no
/// argument, no sub-expression, no position (Adversary finding, Steward
/// `evt_zb2660kkzh14`). The `xs`-argument claim below is therefore an
/// ELABORATOR-SIDE observation, not a kernel-side one: a temporary
/// `#[cfg(debug_assertions)]` probe at the `RVar` `var_refinements`
/// consultation site (`elab.rs`, the site `install_index_refinements`
/// feeds), run at the pre-remedy base and removed before commit, showed
/// the recursive call `zip m xs ys` forced exactly three bindings through
/// `var_refinements` -- `w` (bottom_pos 2, the OUTER match's legitimate
/// sibling-convoy refinement), `ys` (bottom_pos 8, the INNER match's own
/// legitimate constructor-injectivity field), and `xs` (bottom_pos 5, the
/// one this node's remedy must skip). Re-running the same probe against
/// the region-stack remedy showed `xs` alone stopped appearing while `w`
/// and `ys` continued to fire unchanged and the declaration elaborated --
/// evidence that `xs`'s retyping, specifically, was both wrong before and
/// corrected by the fix, though this remains a probe observation, not a
/// kernel-attributed span.
///
/// **The fixture is a conjunction, not a single property.** The inner
/// `match w { VCons _ b ys |-> … }` has one arm against `Vec`'s two
/// constructors; it elaborates only because `VNil` is index-impossible at
/// `Vec Nat (S m)` (`34 §4.3`). The fixture reaches the enclosing-match-
/// field gap only while that index-impossibility holds -- a regression
/// there surfaces as `ExhaustivenessError`, a different route than the
/// `TypeMismatch` this control exercises.
///
/// **`AC-1` (post-remedy): elaborates AND evaluates correctly.** "The
/// `TypeMismatch` is gone" is explicitly NOT this criterion -- an
/// over-wide skip also makes the error disappear, by refusing to refine
/// something it should have refined, and a wrong-but-successful
/// elaboration would pass a vanishing-error test. This fixture's `zip`
/// ignores `w`'s payload entirely (it only destructures `w` for
/// exhaustiveness) and reconstructs each level from `v`'s own `m`/`a`, so
/// its correct result is structurally `v` itself.
///
/// **`AC-1`'s HAZARD -- distinguishing a correct refinement from an
/// over-wide skip -- is WITHDRAWN, not discharged (Architect ruling
/// `evt_2hfhtcqk3fpn7`, refuting the prior `evt_b4hfddjceg8d` §3 claim
/// made without running the mutation).** `let_interleaved_outer_binder_
/// not_skipped_by_convoy`'s `let k = 0` gives `k` no index-dependent type
/// (`cur_ty` is bare `Nat`), so `try_reindex_cast`'s own no-spurious-
/// refinement guard (`elab.rs:2830`, `AC8`) makes the loop a no-op at `k`
/// whether or not the region guard skips it -- QA measured this directly
/// by replacing the guard with the prohibited positional floor
/// (`if abs_pos >= 3`) and re-running the file: exit 0, 7 passed, 1
/// ignored, 0 failed, identical to the region-set run. Reproduced
/// independently here. **No test in this file currently distinguishes the
/// ruled region set from the refuted floor.**
///
/// A genuinely discriminating fixture needs three constraints together:
/// (1) position above the enclosing field region (a `let`/`λ` push,
/// already satisfied by `k`); (2) a type that literally depends on the
/// index the nested match peels, so `try_reindex_cast` returns `Some`
/// rather than the AC8 no-op above; (3) consumption where the unrefined
/// type fails to check. A bounded attempt at (2)+(3) -- binding `k` to a
/// fresh `Vec Nat n` value (via a small `repl : (n : Nat) -> Vec Nat n`
/// helper, so `k`'s type is NOT inherited from an already-refined
/// reference) and consuming it either as a further nested match's
/// scrutinee or via the recursive call -- hit an apparently unrelated
/// internal elaborator error (`index refinement: could not classify the
/// branch goal: TypeMismatch { expected: Dg67, found: ((Dg574 Dg67)
/// @N) }`, from `refine_branch_goal`, `elab.rs:2913-2917`) in every
/// variant tried, and a diagnostic probe on `try_reindex_cast`'s own
/// operands showed `k`'s weakened raw type and the middle match's `b2`
/// disagreeing on which absolute position they name -- consistent with a
/// frame/weakening mismatch specific to an intervening `let` between an
/// outer match's premise computation and a nested match's own field push.
/// That is a plausible, DIFFERENT, and orthogonal gap from this node's own
/// remedy; it is reported, not fixed, here (out of scope -- `install_
/// index_refinements` consumers beyond this node's own fix are explicitly
/// banned scope). The region-set-vs-floor choice therefore remains
/// **design-justified but behaviourally unwitnessed at this node**: the
/// region set is the provenance-correct predicate (a field bound by an
/// enclosing match is not a genuine outer binder, independent of whether
/// a program can currently observe the difference), but no surface
/// program in this file currently exercises the divergence.
///
/// **This fixture's evaluation is nonetheless the literal `AC-1` ask, and
/// it is retained here, real, and IGNORED rather than weakened or
/// deleted.** It is blocked by an orthogonal, pre-existing `ken-interp`
/// gap, found while building this control: `zip`'s elaborated body embeds
/// real `Cast`/`J` terms (capability 1/2/3's proof machinery, `elab.rs`)
/// at EVERY arm, including the base case (capability 3's goal-cast for
/// `VNil Nat`). `ken_interp::eval`'s `Term::J` arm does not exist -- the
/// crate's only `Term::J` match arm is `term_var_free`'s free-variable
/// walk (`eval.rs:958`), not a reduction; `Term::J` in `eval()` itself
/// falls through the function's own final catch-all,
/// `crates/ken-interp/src/eval.rs:1916` (`_ => EvalVal::Neutral`, comment:
/// "Remaining K2 forms: not reduced in the G1 scope"). `cast_reduce`
/// (`eval.rs:1144-1156`) requires the equality proof to evaluate to
/// `EvalVal::ReflVal` to take its one reducing branch (C5 regularity); a
/// `Neutral` proof always falls to its "(oracle)" branch,
/// `EvalVal::Unknown` -- declared, not accidental, per that function's own
/// comment. So ANY dependent-match program that exercises DS-5b's
/// capability 1/2/3 evaluates to `Unknown` today, for a reason that has
/// nothing to do with this node's remedy and predates it entirely -- the
/// SAME machinery backs `sibling_convoy_retypes_outer_binder_through_
/// nested_match` and `tail_constructor_injectivity_retypes_peeled_
/// recursive_field` above, neither of which this file has ever evaluated
/// (both only assert `elab_ok`).
///
/// Per the ruling, this is an AUTHORIZED HARD STOP owned by a `ken-interp`
/// successor (Steward-filed, scope), not a defect in this node and not
/// something this node repairs. The assertion below is the REAL `AC-1`
/// expectation (a `vec_nat_structurally_eq` comparison against the
/// expected value, not an `Unknown` sentinel -- pinning `Unknown` would
/// freeze `ken-interp`'s declared G1 scope limit as an expectation, and
/// red the day the capability lands instead of passing). It is registered
/// in `.github/ignored-test-exemptions.toml` under `blocked-upstream-
/// relation`, readmission `TermJReduction`, following the
/// `RT-CLOSURE-BOUNDARY-LANE` row's contract.
#[test]
#[ignore = "TermJReduction: the convoy cast's proof is not ReflVal and ken-interp has no Term::J reduction arm, so cast_reduce yields Unknown for the G1 scope; fails at base 7aae5fcc6"]
fn two_vector_zip_recursive_step_convoy_fixture() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn zip (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { \
           VNil |-> VNil Nat; \
           VCons m a xs |-> match w { VCons _ b ys |-> VCons Nat m a (zip m xs ys) } \
         }",
    );
    // Numeral literals do not check against `Nat` here (a separate,
    // orthogonal parser/elaborator gap: bare `Token::Nat` numerals infer
    // to a default numeric type in synthesis mode but fail `check`-mode
    // against the `Nat` inductive -- `const n : Nat = 0` itself fails the
    // same way). `Zero`/`Suc` are the prelude's real Peano constructors and
    // sidestep it entirely.
    let result_id = env
        .elaborate_decl(
            "const zipResult = \
             zip (Suc (Suc Zero)) \
               (VCons Nat (Suc Zero) (Suc Zero) (VCons Nat Zero Zero (VNil Nat))) \
               (VCons Nat (Suc Zero) Zero (VCons Nat Zero (Suc Zero) (VNil Nat)))",
        )
        .expect("zip application must elaborate post-remedy");
    let expected_id = env
        .elaborate_decl(
            "const zipExpected = \
             VCons Nat (Suc Zero) (Suc Zero) (VCons Nat Zero Zero (VNil Nat))",
        )
        .expect("expected value must elaborate");

    let (_, result_body) = env
        .env
        .transparent_body(result_id)
        .expect("const binding must be transparent");
    let (_, expected_body) = env
        .env
        .transparent_body(expected_id)
        .expect("const binding must be transparent");

    let mut store = ken_interp::EvalStore::new();
    let result = ken_interp::eval(&[], &result_body, &env.env, &mut store);
    let expected = ken_interp::eval(&[], &expected_body, &env.env, &mut store);

    assert!(
        vec_nat_structurally_eq(&result, &expected),
        "zip 2 v w must evaluate to v itself (this fixture's zip ignores \
         w's payload and reconstructs each level from v's own m/a) -- got \
         {result:?}, expected {expected:?}"
    );
}

/// `LANG-CONVOY-MATCH-FIELD-PROVENANCE` D2 -- the let-interleaved
/// POSITION record (NOT a region-set-vs-floor discriminator -- that claim
/// was made, then measured false; see the correction below). The Architect
/// explicitly did NOT build this fixture: he established from
/// `elab.rs:1143`/`:1132` (`RLet`/`RLam` push `cx.ctx` and elaborate the
/// body inside that push) that a `let` between the outer match's arm and a
/// nested inner match strands a genuine outer binder (`k`) ABOVE the
/// enclosing arm's own field region -- but did not confirm the shape
/// reaches capability 2 at all. A temporary `#[cfg(debug_assertions)]`
/// probe at capability 2's loop (`elab.rs`, removed before commit) showed
/// the shape DOES reach it: `k` sits at `abs_pos=6` with `match_field_
/// regions=[3..6, 7..10]` (the enclosing arm's `m,a,xs` and the inner
/// arm's own `_,b,ys`) at the moment the loop reaches it -- `k` is outside
/// every active range, and the declaration elaborates.
///
/// **This does NOT, however, distinguish the region set from a floor --
/// measured, not merely unclaimed (Architect ruling
/// `evt_2hfhtcqk3fpn7` §2, refuting his own earlier `evt_b4hfddjceg8d`
/// §3 claim that "an over-wide skip reds this one").** `k`'s type here is
/// `Nat` (`let k = 0`), which mentions no index at all, so `try_reindex_
/// cast`'s own no-spurious-refinement guard (`elab.rs:2830`, `AC8`) makes
/// capability 2's loop a no-op at `k`'s position REGARDLESS of whether the
/// region guard skips it. QA confirmed this by replacing the guard with
/// the prohibited positional floor (`if abs_pos >= 3`) and running the
/// whole file: exit 0, 7 passed, 1 ignored, 0 failed -- identical to the
/// region-set run. Independently reproduced here (same result, both
/// directions). See `two_vector_zip_recursive_step_convoy_fixture`'s doc
/// comment for the bounded attempt at a genuinely discriminating fixture
/// and why it was not completed within this node's scope.
#[test]
fn let_interleaved_outer_binder_not_skipped_by_convoy() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn zipK (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { \
           VNil |-> VNil Nat; \
           VCons m a xs |-> \
             let k = 0 in \
             match w { VCons _ b ys |-> VCons Nat m a (zipK m xs ys) } \
         }",
    );
}

/// `LANG-INTERVENING-LET-FRAME-WEAKENING` `D1` -- the three-way
/// attribution the Architect required as a follow-up of his approval of
/// `LANG-CONVOY-MATCH-FIELD-PROVENANCE` (`evt_5b3c38r3xrqm6`), measuring
/// whether this fixture's failure is a regression on that merge or a
/// pre-existing gap. The program is a fresh `let k` bound to a genuine
/// `Vec Nat n` value (via `repl`, NOT an already-refined alias of `w` --
/// that would be "born correct" and prove nothing), interleaved between
/// the outer match's premise and a nested match, consumed by a further
/// nested match on `k` itself.
///
/// **Measured, three ways, run not read (the discipline the predecessor's
/// two recuts existed to enforce):**
///
/// 1. **Predecessor's merge-base `43bd0d597`** (pre-remedy `elab.rs`, no
///    `match_field_regions` at all): fails.
/// 2. **Shipped region-set `elab.rs`, capability 2's guard replaced by the
///    prohibited positional floor** (`if abs_pos >= 3`): fails, byte-for-
///    byte the same error.
/// 3. **Shipped region-set `elab.rs`, as landed**: fails, byte-for-byte
///    the same error.
///
/// All three: `ElabError::Internal("index refinement: could not classify
/// the branch goal: TypeMismatch { expected: Dg67, found: ((Dg574 Dg67)
/// @8) }")`, raised at `refine_branch_goal` (`elab.rs:2913-2917`).
///
/// **INVARIANT across all three ⇒ per the node's own branch condition,
/// this is a clean pre-existing gap, independent of the merged region-set
/// predicate -- NOT an acceptance regression on `LANG-CONVOY-MATCH-FIELD-
/// PROVENANCE`.** This pins that measurement so it survives being findable
/// by grep rather than living only in convo history. Diagnosing WHERE the
/// three-way-invariant failure originates (`D2`) and whether `RVar`
/// resolution is the actual route (`D3`) are this node's next
/// deliverables, not this commit's -- no repair lands here.
#[test]
fn intervening_let_fresh_binder_fails_invariantly_across_all_three_bases() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn repl (n : Nat) : Vec Nat n = \
         match n { Zero |-> VNil Nat; Suc m |-> VCons Nat m Zero (repl m) }",
    );
    let err = expect_err_val(
        &mut env,
        "fn zipK (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { \
           VNil |-> VNil Nat; \
           VCons m a xs |-> \
             let k = repl n in \
             match w { \
               VCons _ b ys |-> \
                 match k { VCons _ c ks |-> VCons Nat m a (zipK m xs ys) } \
             } \
         }",
    );
    match &err {
        ElabError::Internal(msg) => {
            assert!(
                msg.contains("could not classify the branch goal"),
                "expected the measured `refine_branch_goal` classification \
                 failure, got a different Internal message: {msg:?}"
            );
            assert!(
                msg.contains("expected: Dg67,"),
                "expected the measured TypeMismatch's `expected` operand \
                 (bare `Nat`, printed `Dg67`) -- a different expected \
                 operand means this is NOT the three-way-invariant \
                 failure this test pins, got: {msg:?}"
            );
            // Structural, not the literal `Dg574`/`@8` -- both are
            // context-shape-dependent (an unrelated prelude/`vec_env()`
            // edit renumbers them without changing the failure this test
            // pins). The measured instance (`Vec Nat @8`, printed
            // `((Dg574 Dg67) @8)`) lives in the doc comment above; the
            // check here is head-plus-Nat-id: `found` is a doubly-wrapped
            // application whose inner argument is the SAME `Dg67` as
            // `expected`, suffixed by a de Bruijn index -- differing from
            // `expected` only by that wrapper, whatever its head/index
            // numbers happen to be.
            assert!(
                msg.contains("found: ((") && msg.contains(" Dg67) @"),
                "expected the measured TypeMismatch's `found` operand to \
                 be a head applied to the same `Dg67` (Nat) id as \
                 `expected`, wrapped with a trailing de Bruijn index -- a \
                 different shape means this is NOT the three-way-invariant \
                 failure this test pins, got: {msg:?}"
            );
        }
        other => panic!(
            "expected an `ElabError::Internal` classification failure, \
             got: {other:?}"
        ),
    }
}

/// `LANG-INTERVENING-LET-FRAME-WEAKENING` reconciliation -- TWO DIFFERENT
/// interleaved-`let` failures, not one, independently re-measured here
/// (not taken from the Adversary's report).
///
/// **`intervening_let_fresh_binder_fails_invariantly_across_all_three_
/// bases` (above) dies in `refine_branch_goal`, invariantly, before
/// reaching the kernel at all.** Its `k` is a FRESH `Vec Nat n` value
/// (via `repl`, never an alias of an existing binder), consumed by a
/// further nested match on `k` itself.
///
/// **This fixture's `k` is a DIRECT ALIAS of the enclosing match's own
/// peeled field** (`let k : Vec Nat m = xs`, `m`/`xs` both already bound
/// by `VCons m a xs`), consumed by passing `k` -- not `xs` -- as the
/// recursive call's argument. Measured: it GETS PAST
/// `refine_branch_goal` cleanly and reaches the KERNEL, where the
/// rejection CLASS is GUARD-DEPENDENT:
///
/// - **shipped region set**: `KernelRejected
///   TypeMismatch`, `expected ((Dg574 Dg67) @9), found ((Dg574 Dg67) @4)`
///   -- the same convoy-class signature as the predecessor node's own `D1`
///   (`@9` vs `@4`).
/// - **prohibited positional floor** (`if abs_pos >= 3`, temporarily
///   applied and reverted, NOT committed -- a source mutation the shipped
///   tree cannot express as a runtime toggle): `KernelRejected
///   NotTerminating("SCT: idempotent self-loop has no strictly-decreasing
///   parameter")` -- a completely different rejection CLASS, from the SCT
///   gate rather than the type checker.
///
/// **Neither guard makes this program elaborate.** The claim this fixture
/// carries is narrower and different: the two guards reach DIFFERENT
/// failure classes on the identical program, which is itself evidence the
/// guards behave differently here (unlike the withdrawn `D2` pair from the
/// predecessor node, where the floor and the region set were
/// indistinguishable). This is NOT a discharge of that withdrawal -- it is
/// a separate, later-discovered discriminating shape, filed to this node
/// rather than reopening the predecessor's.
///
/// **These are two distinct pre-existing gaps, not one.** Diagnosing which
/// (if either) shares a root cause with the other is this node's `D2`,
/// not this commit's -- `D2`/`D3` remain out of scope here (Steward
/// release, merge-closeout only).
#[test]
fn interleaved_let_alias_of_enclosing_field_rejects_differently_under_region_set() {
    let mut env = vec_env();
    elab_ok(
        &mut env,
        "fn zip (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { \
           VNil |-> VNil Nat; \
           VCons m a xs |-> match w { VCons _ b ys |-> VCons Nat m a (zip m xs ys) } \
         }",
    );
    let err = expect_err_val(
        &mut env,
        "fn zipAdv (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Vec Nat n = \
         match v { \
           VNil |-> VNil Nat; \
           VCons m a xs |-> \
             let k : Vec Nat m = xs in \
             match w { VCons _ b ys |-> VCons Nat m a (zipAdv m k ys) } \
         }",
    );
    match &err {
        // The `@9`/`@4` positional literals asserted here previously added
        // no discriminating power beyond the error CLASS check below:
        // the prohibited positional floor rejects with `NotTerminating`,
        // which lands in the `other =>` arm, so a disjoint error class
        // already separates the two guards. The literals also matched too
        // loosely against neighbours (`@4` matches `@40`/`@43`) while
        // being too brittle against any unrelated binder-structure shift.
        // A one-armed control over two disjoint error classes is a
        // complete control -- deleted rather than "completed."
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        } => {}
        other => panic!(
            "expected the measured shipped-region-set rejection (a \
             kernel TypeMismatch) -- got a \
             different error, which means this fixture's guard-dependent \
             behaviour needs re-measuring: {other:?}"
        ),
    }
}
