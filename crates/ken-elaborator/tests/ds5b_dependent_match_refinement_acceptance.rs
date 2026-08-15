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
///
/// Currently unused by any assertion: `two_vector_zip_recursive_step_
/// convoy_fixture`'s evaluation half is blocked by an unrelated, pre-
/// existing `ken-interp` gap (see that test's doc comment) and pins
/// `Unknown` instead. Kept for the day that gap closes, at which point the
/// sentinel assertion there becomes a real call to this function.
#[allow(dead_code)]
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
/// **The evaluated-value half of `AC-1` is BLOCKED by an orthogonal,
/// pre-existing `ken-interp` gap, found while building this control --
/// escalated, not silently routed around.** `zip`'s elaborated body embeds
/// real `Cast`/`J` terms (capability 1/2/3's proof machinery, `elab.rs`) at
/// EVERY arm, including the base case (capability 3's goal-cast for `VNil
/// Nat`). `ken_interp::eval`'s `Term::J` arm does not exist -- it falls
/// through the function's own final catch-all, `crates/ken-interp/src/
/// eval.rs:1916` (`_ => EvalVal::Neutral`, comment: "Remaining K2 forms:
/// not reduced in the G1 scope"). `cast_reduce` (`eval.rs:1144-1156`)
/// requires the equality proof to evaluate to `EvalVal::ReflVal` to take
/// its one reducing branch (C5 regularity); a `Neutral` proof always falls
/// to its "(oracle)" branch, `EvalVal::Unknown`, which then propagates
/// strictly (`eval.rs:21`, `:1145`). So ANY dependent-match program that
/// exercises DS-5b's capability 1/2/3 evaluates to `Unknown` today, for a
/// reason that has nothing to do with this node's remedy -- the SAME
/// machinery backs `sibling_convoy_retypes_outer_binder_through_nested_
/// match` and `tail_constructor_injectivity_retypes_peeled_recursive_
/// field` above, neither of which this file has ever evaluated (both only
/// assert `elab_ok`). This predates `LANG-CONVOY-MATCH-FIELD-PROVENANCE`
/// entirely.
///
/// The assertion below is therefore a NAMED SENTINEL, not the value
/// comparison `AC-1` asked for: it pins `Unknown` for the documented
/// reason above and will legitimately go red the day `Term::J` gets a
/// reduction rule -- at which point replace it with a real
/// `vec_nat_structurally_eq` comparison against the expected value (also
/// constructed below, and already known-correct: it elaborates and its own
/// evaluation is exercised nowhere else in this fixture only because
/// `zipResult`'s `Unknown` makes the comparison moot today).
#[test]
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

    // `expected` alone (no `zip` call, no Cast/J) DOES reduce to a concrete
    // value -- confirming the `Unknown` below is specific to the Cast/J
    // path `zip` embeds, not a general evaluator failure on this fixture.
    assert!(
        !matches!(expected, EvalVal::Unknown),
        "sanity: the expected value alone (no Cast/J involved) must reduce \
         to a concrete Ctor, got {expected:?} -- if this fails, the gap \
         documented above has moved and this whole control needs \
         re-deriving"
    );
    assert!(
        matches!(result, EvalVal::Unknown),
        "sentinel: zip's evaluated result is expected to be Unknown today \
         (unreduced Term::J, see the doc comment above) -- got {result:?} \
         instead, which means either the ken-interp gap closed (replace \
         this assertion with vec_nat_structurally_eq(&result, &expected)) \
         or something else changed and needs re-deriving"
    );
}

/// `LANG-CONVOY-MATCH-FIELD-PROVENANCE` D2 -- the let-interleaved
/// discriminator. The Architect explicitly did NOT build this fixture: he
/// established from `elab.rs:1143`/`:1132` (`RLet`/`RLam` push `cx.ctx` and
/// elaborate the body inside that push) that a `let` between the outer
/// match's arm and a nested inner match strands a genuine outer binder
/// (`k`) ABOVE the enclosing arm's own field region -- but did not confirm
/// the shape reaches capability 2 at all. `AC-3` measured that it does:
/// a temporary `#[cfg(debug_assertions)]` probe at capability 2's loop
/// (`elab.rs`, removed before commit) showed `k` sits at `abs_pos=6` with
/// `match_field_regions=[3..6, 7..10]` (the enclosing arm's `m,a,xs` and
/// the inner arm's own `_,b,ys`) at the moment the loop reaches it --
/// `skipped=false`, `k` is outside every active range, and the
/// declaration elaborates. This is the only fixture that separates the
/// ruled region-set remedy from a floor: a floor keyed on the enclosing
/// match's entry depth (3) would treat `k`'s position (6, above that
/// depth) as ineligible for sibling convoy, identically to how it
/// (wrongly) treats the enclosing match's own fields -- the region set
/// does not, because `k`'s position was never recorded in
/// `match_field_regions` (that push closed at `6`, the enclosing arm's own
/// field count, before the `let` ran).
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
