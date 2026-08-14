//! L2 acceptance tests: `data` / `match` / exhaustiveness / refinements.
//!
//! Pins: `conformance/surface/data-match/seed-data-match.md` AC1–AC8.
//! Spec: `spec/30-surface/34-data-match.md`.
//!
//! AC5 (indexed families) and AC6 (dependent motive) are deferred — they
//! require `indices` support and dependent motives respectively.

use ken_elaborator::{error::ElabError, ArmDeadCause, ElabEnv, Span};
use ken_kernel::{whnf, Context, GlobalId, Level, Term};

// ----- helpers -----

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

fn elab(env: &mut ElabEnv, src: &str) -> Result<GlobalId, ElabError> {
    env.elaborate_decl(src)
}

fn elab_ok(env: &mut ElabEnv, src: &str) -> GlobalId {
    elab(env, src).unwrap_or_else(|e| panic!("elab_ok failed: {}", e))
}

fn body_of(env: &ElabEnv, id: GlobalId) -> Term {
    env.env.transparent_body(id).expect("not a transparent def").1
}

fn term_mentions_const(t: &Term, target: GlobalId) -> bool {
    match t {
        Term::Const { id, .. } if *id == target => true,
        _ => t
            .children()
            .into_iter()
            .any(|child| term_mentions_const(child, target)),
    }
}

fn term_mentions_var(t: &Term, target: usize) -> bool {
    match t {
        Term::Var(i) if *i == target => true,
        _ => t
            .children()
            .into_iter()
            .any(|child| term_mentions_var(child, target)),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AC1  construct-then-eliminate  (`34 §1`, `14 §3`)
//
// Property: `data` declares a real inductive + computing `elim_D`; `match`
// ι-reduces on the constructor.
//
// Scrutinee given inline: `SomeInt 3` in the match expression.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ac1_data_decl_registers_former_and_ctors() {
    let mut env = mk_env();
    elab_ok(&mut env, "data MaybeInt = NoInt | SomeInt Int");

    assert!(env.globals.contains_key("MaybeInt"), "type former registered");
    assert!(env.globals.contains_key("NoInt"), "NoInt ctor registered");
    assert!(env.globals.contains_key("SomeInt"), "SomeInt ctor registered");

    let d_id = env.globals["MaybeInt"];
    assert!(env.env.inductive(d_id).is_some(), "MaybeInt is an inductive family");

    let c_id = env.globals["SomeInt"];
    assert!(env.env.constructor(c_id).is_some(), "SomeInt is a constructor");
}

#[test]
fn ac1_construct_then_eliminate_reduces() {
    let mut env = mk_env();
    elab_ok(&mut env, "data MaybeInt = NoInt | SomeInt Int");

    // `SomeInt 3` is the scrutinee inline — no separate binding needed.
    let id = elab_ok(
        &mut env,
        "let answer : Int = match SomeInt 3 { SomeInt x |-> x ; NoInt |-> 0 }",
    );
    let body = body_of(&env, id);
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);

    // ι-rule: elim_MaybeInt M [0; λx.x] (SomeInt v) ⇝ (λx.x) v ⇝ v
    // The result should be the opaque literal (the SomeInt arg), not still an Elim.
    assert!(
        !matches!(reduced, Term::Elim { .. }),
        "AC1: elim_MaybeInt did not ι-reduce; still got Elim node"
    );
}

#[test]
fn ac1_nil_arm_reduces_to_default() {
    let mut env = mk_env();
    elab_ok(&mut env, "data MaybeInt = NoInt | SomeInt Int");

    let id = elab_ok(
        &mut env,
        "let answer : Int = match NoInt { SomeInt x |-> x ; NoInt |-> 0 }",
    );
    let body = body_of(&env, id);
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);

    // ι-rule: elim_MaybeInt M [0; λx.x] NoInt ⇝ 0
    assert!(
        !matches!(reduced, Term::Elim { .. }),
        "AC1: elim_MaybeInt on NoInt did not ι-reduce"
    );
    // The result is a kernel-native IntLit (DS-6c: `0`'s literal emission
    // no longer produces an opaque Const postulate reference).
    assert!(
        matches!(reduced, Term::IntLit(_)),
        "AC1: expected a kernel-native IntLit after ι-reduction, got {:?}",
        reduced
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// AC2  match-elaborates-to-elim  (`34 §3`, `39 §2.6`)
//
// Structural: body head is Term::Elim; nested match ⇒ nested Elim.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ac2_match_head_is_term_elim() {
    let mut env = mk_env();
    elab_ok(&mut env, "data Shape = Circle Int | Rect Int Int");

    let id = elab_ok(
        &mut env,
        "let r : Int = match Circle 2 { Circle x |-> x ; Rect w h |-> w }",
    );
    let body = body_of(&env, id);

    assert!(
        matches!(body, Term::Elim { .. }),
        "AC2: match did not compile to Term::Elim; got {:?}",
        body
    );
}

#[test]
fn ac2_elim_computes_on_circle() {
    let mut env = mk_env();
    elab_ok(&mut env, "data Shape = Circle Int | Rect Int Int");

    let id = elab_ok(
        &mut env,
        "let r : Int = match Circle 2 { Circle x |-> x ; Rect w h |-> w }",
    );
    let body = body_of(&env, id);
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);

    // elim_Shape M [λx.x; λw.λh.w] (Circle 2) ⇝ (λx.x) 2 ⇝ 2  (opaque literal)
    assert!(
        !matches!(reduced, Term::Elim { .. }),
        "AC2: elim_Shape on Circle 2 did not ι-reduce"
    );
}

#[test]
fn ac2_nested_match_produces_nested_elim() {
    let mut env = mk_env();
    elab_ok(&mut env, "data AB = A | B");

    let id = elab_ok(
        &mut env,
        "let v : Int = match A { A |-> match B { A |-> 1 ; B |-> 2 } ; B |-> 0 }",
    );
    let body = body_of(&env, id);

    // Outer Elim: method for A should itself be an Elim (the nested match).
    let a_method = match &body {
        Term::Elim { methods, .. } => methods[0].clone(), // A is constructor 0
        other => panic!("expected outer Elim, got {:?}", other),
    };
    assert!(
        matches!(a_method, Term::Elim { .. }),
        "AC2 nested: A-arm method is not an Elim (nested match not lowered); got {:?}",
        a_method
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// AC3  exhaustiveness-required  (`34 §4.1`, `§4.4`)  (soundness — TR3)
//
// The discriminating signal is the NAMED witness (not just "rejects"):
//  (a) missing Blue → rejects naming "Blue"
//  (b) all three arms → accepts
// ─────────────────────────────────────────────────────────────────────────────

fn setup_color(env: &mut ElabEnv) {
    elab_ok(env, "data Color = Red | Green | Blue");
}

#[test]
fn ac3_missing_arm_names_blue() {
    let mut env = mk_env();
    setup_color(&mut env);

    // Match on a constructor directly to avoid binding issues.
    let result = elab(
        &mut env,
        "let bad : Int = match Red { Red |-> 0 ; Green |-> 1 }",
    );

    match result {
        Err(ElabError::ExhaustivenessError { missing, .. }) => {
            assert_eq!(
                missing.constructor, "Blue",
                "AC3: named witness should be 'Blue', got '{}'",
                missing.constructor
            );
            assert_eq!(
                missing.arity, 0,
                "AC3: 'Blue' is zero-arity, so its witness pattern IS its name"
            );
            assert_eq!(
                missing.to_string(),
                "Blue",
                "AC3: a zero-arity witness renders with no trailing wildcards"
            );
        }
        Ok(_) => panic!("AC3: non-exhaustive match accepted (should have been rejected)"),
        Err(other) => panic!("AC3: expected ExhaustivenessError naming 'Blue', got: {}", other),
    }
}

#[test]
fn ac3_exhaustive_match_accepts() {
    let mut env = mk_env();
    setup_color(&mut env);

    let id = elab_ok(
        &mut env,
        "let result : Int = match Blue { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 }",
    );
    let body = body_of(&env, id);
    assert!(matches!(body, Term::Elim { .. }), "AC3: accepted match should produce Elim");
}

#[test]
fn ac3_exhaustive_elim_reduces_on_blue() {
    let mut env = mk_env();
    setup_color(&mut env);

    let id = elab_ok(
        &mut env,
        "let result : Int = match Blue { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 }",
    );
    let body = body_of(&env, id);
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);

    // elim_Color M [0; 1; 2] Blue ⇝ 2 (ι on Blue ctor) — a kernel-native
    // IntLit (DS-6c), not an opaque Const postulate reference.
    assert_eq!(
        reduced,
        Term::IntLit(num_bigint::BigInt::from(2)),
        "AC3: elim_Color on Blue should reduce to IntLit(2), got {:?}",
        reduced
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// AC4  reachability-redundant-arm  (`34 §4.2`)
//
//  (a) duplicate Red → ReachabilityError
//  (b) three distinct arms → accepts
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ac4_duplicate_arm_is_reachability_error() {
    let mut env = mk_env();
    setup_color(&mut env);

    let result = elab(
        &mut env,
        "let bad : Int = match Red { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 ; Red |-> 9 }",
    );

    match result {
        Err(ElabError::ReachabilityError { .. }) => { /* ✓ AC4 */ }
        Ok(_) => panic!("AC4: redundant arm should have been flagged"),
        Err(other) => panic!("AC4: expected ReachabilityError, got: {}", other),
    }
}

#[test]
fn ac4_all_distinct_arms_accept() {
    let mut env = mk_env();
    setup_color(&mut env);

    let id = elab_ok(
        &mut env,
        "let result : Int = match Green { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 }",
    );
    let body = body_of(&env, id);
    assert!(matches!(body, Term::Elim { .. }), "AC4: all-distinct match should produce Elim");
}

// `LANG-REACHABILITY-SUBSUMING-ARMS` control 1: `Subsumed` with EXACTLY ONE
// winner, via the single-column `check_match_dependent` path (a flat,
// checking-mode match against a known expected type -- same dispatch shape
// as `ac4_duplicate_arm_is_reachability_error` above, now asserting the
// structured `cause` rather than only the bare variant).
#[test]
fn duplicate_flat_arm_is_subsumed_by_exactly_one_earlier_arm() {
    let mut env = mk_env();
    setup_color(&mut env);

    let src = "let bad : Int = match Red { Red |-> 0 ; Green |-> 1 ; Blue |-> 2 ; Red |-> 9 }";
    let result = elab(&mut env, src);

    match &result {
        Err(
            e @ ElabError::ReachabilityError {
                span,
                cause: ArmDeadCause::Subsumed { first, rest },
            },
        ) => {
            assert!(
                rest.is_empty(),
                "a single-column duplicate constructor has exactly one claimant, got \
                 rest={rest:?}"
            );
            assert_eq!(
                &src[first.start..first.end],
                "Red |-> 0",
                "the winner should be the FIRST Red arm"
            );
            assert_eq!(
                &src[span.start..span.end],
                "Red |-> 9",
                "the dead arm should be the SECOND Red arm"
            );
            let rendered = e.to_string();
            println!("rendered diagnostic: {rendered}");
            assert_eq!(
                rendered,
                format!(
                    "redundant match arm at {}-{}: pattern already covered by the earlier \
                     arm(s) at {}-{}",
                    span.start, span.end, first.start, first.end
                ),
                "the rendered diagnostic must name exactly the one claimant"
            );
        }
        Ok(_) => panic!("duplicate constructor arm should have been flagged"),
        Err(other) => panic!("expected ReachabilityError(Subsumed), got: {other}"),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Nested constructor patterns (`34 §3.1` pattern-matrix compilation).
//
// Regression for GAP-nested-patterns: `infer_match` used to track coverage
// by top-level constructor only, so two arms sharing a head constructor
// (`Succ Zero` / `Succ (Succ m)`) tripped a false ReachabilityError even
// though the nested patterns are disjoint. The fix compiles the standard
// column-by-column pattern matrix, splitting further on a field whose own
// sub-pattern is itself a constructor.
// ─────────────────────────────────────────────────────────────────────────────

fn setup_natl(env: &mut ElabEnv) {
    elab_ok(env, "data NatL = Zero | Succ NatL");
}

#[test]
fn nested_ctor_pattern_accepted_and_reduces() {
    let mut env = mk_env();
    setup_natl(&mut env);

    let id = elab_ok(
        &mut env,
        "let result : Int = match Succ (Succ Zero) { \
         Zero |-> 0 ; Succ Zero |-> 1 ; Succ (Succ m) |-> 2 }",
    );
    let body = body_of(&env, id);
    assert!(matches!(body, Term::Elim { .. }), "nested match should produce Elim");

    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);
    assert_eq!(
        reduced,
        Term::IntLit(num_bigint::BigInt::from(2)),
        "Succ (Succ Zero) should select the Succ (Succ m) arm and ι-reduce to IntLit(2), \
         got {:?}",
        reduced
    );
}

#[test]
fn nested_ctor_pattern_selects_the_right_arm_at_succ_zero() {
    let mut env = mk_env();
    setup_natl(&mut env);

    // Discriminating: swap which literal each arm returns, and scrutinize
    // `Succ Zero` — only reduces to the FIRST Const if the `Succ Zero`
    // (not `Succ (Succ m)`) arm actually fired.
    let id = elab_ok(
        &mut env,
        "let result : Int = match Succ Zero { \
         Zero |-> 0 ; Succ Zero |-> 7 ; Succ (Succ m) |-> 9 }",
    );
    let body = body_of(&env, id);
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);
    // Kernel-native IntLit (DS-6c); assert the VALUE too (7, not 9) so
    // this stays the discriminating test its own comment describes.
    assert_eq!(
        reduced,
        Term::IntLit(num_bigint::BigInt::from(7)),
        "expected the Succ Zero arm (7) to ι-reduce to IntLit(7), got {:?}",
        reduced
    );
}

#[test]
fn nested_ctor_pattern_missing_case_is_exhaustiveness_error() {
    let mut env = mk_env();
    setup_natl(&mut env);

    // `Succ Zero` is uncovered: the outer `Succ` bucket only handles the
    // nested-`Succ` sub-case, leaving the nested-`Zero` sub-case missing.
    let result = elab(
        &mut env,
        "let bad : Int = match Zero { Zero |-> 0 ; Succ (Succ m) |-> 2 }",
    );

    match result {
        Err(ElabError::ExhaustivenessError { .. }) => { /* ✓ */ }
        Ok(_) => panic!("non-exhaustive nested match accepted (should have been rejected)"),
        Err(other) => panic!("expected ExhaustivenessError, got: {}", other),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// `LANG-WITNESS-ARITY-DERIVED` D2: `missing_pattern_witness` now derives its
// arity from the constructor `id` alone (`crates/ken-kernel/src/env.rs`'s
// `constructor(id) -> Option<(&InductiveDecl, usize)>`) instead of taking a
// caller-supplied `arity: usize` that could disagree with the name. This is
// the single control the node calls for: an arity >= 1 constructor's witness
// must render exactly its own declared arity's worth of wildcards. `Succ`
// (arity 1) also reaches `infer_match`'s general non-dependent path -- the
// same emission site previously witnessed only at arity 0 (`Zero`).
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn non_dependent_arity_one_constructor_witness_derives_its_own_arity() {
    let mut env = mk_env();
    setup_natl(&mut env);

    let result = elab(&mut env, "let bad : Int = match Zero { Zero |-> 0 }");

    match result {
        Err(ElabError::ExhaustivenessError { missing, .. }) => {
            assert_eq!(
                missing.constructor, "Succ",
                "derived witness should name 'Succ', got '{}'",
                missing.constructor
            );
            assert_eq!(
                missing.arity, 1,
                "'Succ' is declared with exactly one argument (NatL); the \
                 derived arity must equal that declaration"
            );
            assert_eq!(
                missing.to_string(),
                "Succ _",
                "an arity-1 witness renders exactly one trailing wildcard"
            );
        }
        Ok(_) => panic!("non-exhaustive match accepted (should have been rejected)"),
        Err(other) => panic!("expected ExhaustivenessError naming 'Succ', got: {}", other),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// `LANG-REACHABILITY-SUBSUMING-ARMS` AC-1/AC-2 -- control 2 (matrix path,
// >=2 winners): `ReachabilityError` carries `cause: ArmDeadCause`, total by
// construction (`ArmDeadCause::Subsumed { first, rest }` /
// `ArmDeadCause::NoInhabitants`), naming the earlier arm(s) whose union
// already covers the dead arm's pattern as DATA rather than only folded
// into rendered prose. A single subsuming arm degenerates "which arms" the
// same way a zero-arity constructor degenerated "name vs applied pattern"
// on the exhaustiveness side, so the discriminating case here needs at
// least two -- a wildcard sub-pattern shadowed at three distinct leaves
// (one per `T3` constructor), each already won by a different earlier arm.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn redundant_wildcard_arm_shadowed_at_three_leaves_names_every_subsuming_arm() {
    let mut env = mk_env();
    elab_ok(&mut env, "data T3 = A | B | C");
    elab_ok(&mut env, "data Wrap = MkWrap T3");

    let src = "let bad : Int = match MkWrap A { \
               MkWrap A |-> 0 ; MkWrap B |-> 1 ; MkWrap C |-> 2 ; MkWrap y |-> 3 }";
    let result = elab(&mut env, src);

    match &result {
        Err(
            e @ ElabError::ReachabilityError {
                span,
                cause: ArmDeadCause::Subsumed { first, rest },
            },
        ) => {
            // AC-1: at least two distinct earlier subsuming arms total.
            assert!(
                !rest.is_empty(),
                "AC-1: expected at least two subsuming arms (first + rest), got just \
                 first={first:?}"
            );

            // AC-2: read the arms as DATA, not rendered text -- each
            // subsuming span slices back to one of the three earlier,
            // distinct arm sources.
            let mut spans: Vec<&Span> = vec![first];
            spans.extend(rest.iter());
            let texts: Vec<&str> = spans.iter().map(|s| &src[s.start..s.end]).collect();
            for expected in ["MkWrap A |-> 0", "MkWrap B |-> 1", "MkWrap C |-> 2"] {
                assert!(
                    texts.iter().any(|t| *t == expected),
                    "AC-2: expected a subsuming span covering '{expected}', got {texts:?}"
                );
            }
            assert_eq!(
                texts.len(),
                3,
                "the wildcard arm is shadowed at all three leaves (A, B, C), so it \
                 should name all three earlier arms, got {texts:?}"
            );

            // The reported span is the dead wildcard arm itself.
            assert_eq!(
                &src[span.start..span.end],
                "MkWrap y |-> 3",
                "the reported span should be the dead wildcard arm itself"
            );

            // Report the diagnostic verbatim, per the frame.
            let rendered = e.to_string();
            println!("rendered diagnostic: {rendered}");
            assert_eq!(
                rendered,
                "redundant match arm at 84-98: pattern already covered by the earlier \
                 arm(s) at 33-47, 50-64, 67-81",
                "the rendered diagnostic must include each structured subsuming span"
            );
        }
        Ok(_) => panic!("wildcard arm shadowed at every leaf should have been flagged"),
        Err(other) => panic!("expected ReachabilityError(Subsumed), got: {other}"),
    }
}

// `LANG-REACHABILITY-SUBSUMING-ARMS` controls 3/4: `ArmDeadCause::
// NoInhabitants` for a foreign/typo constructor arm -- one that resolves to
// a REAL global constructor (so name resolution does not reject it first),
// but not one of the scrutinee's own inductive's constructors, so no earlier
// arm ever claims it and it wins at no leaf. Before this repair this shape
// reached the new single-column `.expect()` and panicked (Architect finding,
// evt_5hwsbae6kqj5w; independently reproduced by QA, evt_69dkk9q8hn3ye); the
// matrix-path variant previously produced a representable-but-false empty
// `Subsumed` payload. Both must now return `Err(NoInhabitants)`, never panic
// and never claim false coverage.

#[test]
fn foreign_constructor_arm_is_no_inhabitants_not_a_panic_single_column() {
    let mut env = mk_env();
    elab_ok(&mut env, "data NatProbe = ZProbe | SProbe NatProbe");
    elab_ok(&mut env, "data Foreign = NProbe");

    // Flat arms (`ZProbe`, `SProbe n`, `NProbe`) route to the single-column
    // `check_match_dependent` path (control 3).
    let src = "let bad : Int = match ZProbe { ZProbe |-> 0 ; SProbe n |-> 1 ; NProbe |-> 2 }";
    let result = elab(&mut env, src);

    match &result {
        Err(e @ ElabError::ReachabilityError { span, cause: ArmDeadCause::NoInhabitants }) => {
            assert_eq!(
                &src[span.start..span.end],
                "NProbe |-> 2",
                "the reported span should be the foreign-constructor arm itself"
            );
            let rendered = e.to_string();
            println!("rendered diagnostic: {rendered}");
            assert_eq!(
                rendered,
                format!(
                    "unreachable match arm at {}-{}: no value of the scrutinee type can \
                     match this pattern",
                    span.start, span.end
                ),
                "NoInhabitants must not claim any covering arm"
            );
        }
        Ok(_) => panic!(
            "a foreign-constructor arm (NProbe, not a NatProbe constructor) elaborated \
             successfully"
        ),
        Err(other) => panic!(
            "expected a clean ReachabilityError(NoInhabitants), NOT a panic; got: {other}"
        ),
    }
}

#[test]
fn foreign_constructor_arm_is_no_inhabitants_not_empty_subsumed_matrix() {
    let mut env = mk_env();
    elab_ok(&mut env, "data NatProbe = ZProbe | SProbe NatProbe");
    elab_ok(&mut env, "data Foreign = NProbe");

    // Nested sub-patterns (`SProbe ZProbe`, `SProbe (SProbe n)`) are NOT
    // flat, so this routes to `infer_match`'s general matrix path (control
    // 4) instead of the single-column path above. `NProbe`'s row never
    // matches either of NatProbe's own constructor ids and is not a
    // Wild/Var row either, so it is filtered out of every bucket and never
    // reaches a leaf -- `subsumed_by` for it stays empty.
    let src = "let bad : Int = match ZProbe { \
               ZProbe |-> 0 ; SProbe ZProbe |-> 1 ; SProbe (SProbe n) |-> 2 ; NProbe |-> 3 }";
    let result = elab(&mut env, src);

    match &result {
        Err(e @ ElabError::ReachabilityError { span, cause: ArmDeadCause::NoInhabitants }) => {
            assert_eq!(
                &src[span.start..span.end],
                "NProbe |-> 3",
                "the reported span should be the foreign-constructor arm itself"
            );
            let rendered = e.to_string();
            println!("rendered diagnostic: {rendered}");
            assert_eq!(
                rendered,
                format!(
                    "unreachable match arm at {}-{}: no value of the scrutinee type can \
                     match this pattern",
                    span.start, span.end
                ),
                "NoInhabitants must not claim any covering arm, and must not be an empty \
                 Subsumed payload"
            );
        }
        Ok(_) => panic!(
            "a foreign-constructor arm (NProbe, not a NatProbe constructor) elaborated \
             successfully"
        ),
        Err(other) => panic!("expected ReachabilityError(NoInhabitants), got: {other}"),
    }
}

#[test]
fn nested_ctor_pattern_shadowed_by_earlier_flat_arm_is_reachability_error() {
    let mut env = mk_env();
    setup_natl(&mut env);

    // `Succ n` (flat) already covers every Succ-headed value, so the later
    // `Succ (Succ m)` arm is dead code — must still be caught even though it
    // shares no top-level ambiguity with `Succ n` at the FIRST split.
    let result = elab(
        &mut env,
        "let bad : Int = match Zero { \
         Zero |-> 0 ; Succ n |-> 1 ; Succ (Succ m) |-> 2 }",
    );

    match result {
        Err(ElabError::ReachabilityError { .. }) => { /* ✓ */ }
        Ok(_) => panic!("arm shadowed by an earlier flat arm should have been flagged"),
        Err(other) => panic!("expected ReachabilityError, got: {}", other),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// AC5  indexed-impossible-pair  — DEFERRED
//
// Requires indexed family support (non-empty `indices` in InductiveSpec) and
// "absurdity fill" for index-impossible arms (`34 §4.3`).
// Tracked as follow-on: L2-indexed.
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// AC6  branch-refinement-is-hypothesis  — DEFERRED
//
// Requires a *dependent* motive (motive mentioning the scrutinee/index).
// Current `infer_match` emits only the constant motive `λx. R`.
// Tracked as follow-on: L2-dep-motive.
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// AC7  refinement-obligation  (`34 §5`, `21 §2`, `22 §2.1`)  (soundness — TR7)
//
// Pre-declare a proposition `P : Int → Ω` so the refinement predicate has
// type Ω (required by `elaborate_view_with_spec`).
//
//  (a) intro side: `{ n : Int | P n }` return type → obligation emitted
//  (b) forget side: `{ n : Int | P n }` value used as Int → no obligation
// ─────────────────────────────────────────────────────────────────────────────

fn setup_prop(env: &mut ElabEnv) {
    // Declare P : Int → Ω (a predicate postulate).
    let int_id = env.globals["Int"];
    let omega = Term::omega(Level::Zero);
    let prop_ty = Term::pi(Term::const_(int_id, vec![]), omega);
    env.declare_postulate_raw("P", prop_ty).expect("declare P");
}

#[test]
fn ac7_refinement_intro_emits_obligation() {
    let mut env = mk_env();
    setup_prop(&mut env);

    // The return type `{ n : Int | P n }` forces an obligation `P 3` at the
    // introduction site.
    let result = env.elaborate_decl_v1("const nonneg_val : { n : Int | P n } = 3");

    let elab_result =
        result.unwrap_or_else(|e| panic!("AC7a: expected success, got: {}", e));

    // AC7 (soundness, TR7): at least one obligation is emitted.
    assert!(
        !elab_result.obligations.is_empty(),
        "AC7a: no obligation emitted for refinement introduction (TR7 regression)"
    );
}

#[test]
fn ac7_plain_carrier_no_obligation() {
    let mut env = mk_env();
    setup_prop(&mut env);

    // A plain Int annotation (no refinement predicate) must emit zero obligations —
    // the obligation gate is the refinement *introduction*, not mere Int typing.
    // This is the negative discriminant complementing AC7a's positive discriminant:
    //   AC7a: `{ n : Int | P n }` annotation → obligation emitted
    //   AC7b: `Int`               annotation → no obligation
    let result = env.elaborate_decl_v1("const plain_val : Int = 5");
    let elab_result = result.unwrap_or_else(|e| {
        panic!("AC7b: plain-Int const should accept, got: {}", e)
    });

    assert!(
        elab_result.obligations.is_empty(),
        "AC7b: spurious obligation emitted for plain Int annotation (no refinement)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// AC8  proof-returning-dependent-motive  (`34 §3.5`, `39 §2.1`, `14 §3`)
//
// Positive red-to-green for KM-dependent-match-proof-motive-build D1. On the
// pre-fix `927dd34` head this literal source rejects at the whole body with
// `KernelRejected { TypeMismatch { expected: Type 0, found: Ω0 } }`, because
// `match km_scrutinee b` missed checked dependent-motive recovery and fell
// through to `infer_match`'s constant `D -> Type 0` motive path.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn ac8_proof_returning_match_through_transparent_scrutinee_elaborates() {
    let mut env = mk_env();
    let scrutinee_id = elab_ok(&mut env, "fn km_scrutinee (b : Bool) : Bool = b");

    let id = elab_ok(
        &mut env,
        "theorem km_proof_motive_positive (b : Bool) \
           : Equal Bool (km_scrutinee b) (km_scrutinee b) = \
           match km_scrutinee b { True |-> Proved ; False |-> Proved }",
    );

    let body = body_of(&env, id);
    let mut inner = &body;
    while let Term::Lam(_, body) = inner {
        inner = body;
    }

    let (motive, scrut) = match inner {
        Term::Elim { fam, motive, scrut, .. } => {
            assert_eq!(*fam, env.globals["Bool"], "AC8 must eliminate over Bool");
            (motive.as_ref(), scrut.as_ref())
        }
        other => panic!(
            "AC8 proof-returning match must lower to Term::Elim, got {other:?}"
        ),
    };
    assert!(
        term_mentions_const(scrut, scrutinee_id),
        "the scrutinee should preserve the transparent helper call"
    );

    let (motive_body, motive_ty) = match motive {
        Term::Ascript(body, ty) => (body.as_ref(), ty.as_ref()),
        other => panic!(
            "AC8 motive must be ascribed so the kernel sees its Ω codomain, got {other:?}"
        ),
    };
    match motive_ty {
        Term::Pi(_, codomain) => {
            assert_eq!(
                **codomain,
                Term::Omega(Level::Zero),
                "AC8 motive codomain must be Ω0"
            );
        }
        other => panic!("AC8 motive type must be a Pi over Bool, got {other:?}"),
    }
    match motive_body {
        Term::Lam(_, body) => {
            assert!(
                term_mentions_var(body, 0),
                "AC8 motive body must mention the generalized scrutinee binder"
            );
            assert!(
                !term_mentions_const(body, scrutinee_id),
                "AC8 motive body must abstract over `km_scrutinee b`, not keep a constant motive"
            );
        }
        other => panic!("AC8 motive body must be a lambda, got {other:?}"),
    }
}

#[test]
fn ac8_wrong_specialized_branch_still_rejects() {
    let mut env = mk_env();
    elab_ok(&mut env, "fn km_scrutinee (b : Bool) : Bool = b");

    let err = elab(
        &mut env,
        "theorem km_proof_motive_negative (b : Bool) \
           : Equal Bool (km_scrutinee b) True -> Equal Bool (km_scrutinee b) True = \
           match km_scrutinee b { True |-> \\p. p ; False |-> \\p. Proved }",
    )
    .expect_err("AC8 negative must reject");

    match err {
        ElabError::KernelRejected {
            error: ken_kernel::KernelError::TypeMismatch { .. },
            ..
        } => {}
        other => panic!(
            "AC8 negative must reject through branch-obligation kernel TypeMismatch, got {other:?}"
        ),
    }
}

#[test]
fn ac8_option_table_branch_motive_elaborates() {
    let mut env = mk_env();
    elab_ok(
        &mut env,
        "fn km_intersection_table (left : Option Unit) (keep : Bool) (prior : Option Unit) \
           : Option Unit = \
           match left { \
             None |-> prior ; \
             Some x |-> match keep { True |-> Some Unit x ; False |-> prior } \
           }",
    );
    elab_ok(
        &mut env,
        "fn km_member_from_lookup (left : Option Unit) : Bool = \
           match left { None |-> False ; Some x |-> True }",
    );
    elab_ok(
        &mut env,
        "fn km_lookup (b : Bool) : Option Unit = \
           match b { True |-> Some Unit MkUnit ; False |-> None Unit }",
    );
    elab_ok(
        &mut env,
        "theorem km_option_refl (o : Option Unit) : Equal (Option Unit) o o = Refl",
    );

    // Mechanical CAT-4 D3 reconstruction from the Ken-owned trigger: a nested
    // proof-returning `match km_lookup ...` whose option-table target mentions
    // the lookup scrutinee and a reducible membership test.
    elab_ok(
        &mut env,
        "theorem intersectionLookupMemberCharacterization (b : Bool) \
           : Equal (Option Unit) \
               (km_intersection_table (km_lookup b) \
                 (km_member_from_lookup (km_lookup b)) (None Unit)) \
               (km_intersection_table (km_lookup b) \
                 (km_member_from_lookup (km_lookup b)) (None Unit)) = \
           match km_lookup b { \
             None |-> km_option_refl \
               (km_intersection_table (None Unit) \
                 (km_member_from_lookup (None Unit)) (None Unit)) ; \
             Some x |-> match km_member_from_lookup (Some Unit x) { \
               True |-> km_option_refl \
                 (km_intersection_table (Some Unit x) \
                   (km_member_from_lookup (Some Unit x)) (None Unit)) ; \
               False |-> km_option_refl \
                 (km_intersection_table (Some Unit x) \
                   (km_member_from_lookup (Some Unit x)) (None Unit)) \
             } \
           }",
    );
}

#[test]
fn ac8_option_table_wrong_constructor_argument_still_rejects() {
    let mut env = mk_env();
    elab_ok(
        &mut env,
        "fn km_intersection_table (left : Option Unit) (keep : Bool) (prior : Option Unit) \
           : Option Unit = \
           match left { \
             None |-> prior ; \
             Some x |-> match keep { True |-> Some Unit x ; False |-> prior } \
           }",
    );
    elab_ok(
        &mut env,
        "fn km_member_from_lookup (left : Option Unit) : Bool = \
           match left { None |-> False ; Some x |-> True }",
    );
    elab_ok(
        &mut env,
        "fn km_lookup (b : Bool) : Option Unit = \
           match b { True |-> Some Unit MkUnit ; False |-> None Unit }",
    );
    elab_ok(
        &mut env,
        "theorem km_option_refl (o : Option Unit) : Equal (Option Unit) o o = Refl",
    );

    let err = elab(
        &mut env,
        "theorem intersectionLookupMemberCharacterizationBad (b : Bool) \
           : Equal (Option Unit) \
               (km_intersection_table (km_lookup b) \
                 (km_member_from_lookup (km_lookup b)) (None Unit)) \
               (km_intersection_table (km_lookup b) \
                 (km_member_from_lookup (km_lookup b)) (None Unit)) = \
           match km_lookup b { \
             None |-> km_option_refl \
               (km_intersection_table (None Unit) \
                 (km_member_from_lookup (None Unit)) (None Unit)) ; \
             Some x |-> match km_member_from_lookup (Some Unit x) { \
               True |-> km_option_refl (None Unit) ; \
               False |-> km_option_refl \
                 (km_intersection_table (Some Unit x) \
                   (km_member_from_lookup (Some Unit x)) (None Unit)) \
             } \
           }",
    )
    .expect_err("wrong constructor-argument proof must reject");

    match err {
        ElabError::KernelRejected {
            error:
                ken_kernel::KernelError::TypeMismatch {
                    expected,
                    found,
                },
            ..
        } => {
            assert!(
                !matches!((&*expected, &*found), (Term::Type(Level::Zero), Term::Omega(Level::Zero))),
                "wrong-argument rejection must not be the pre-D1 proof-motive sort failure"
            );
        }
        other => panic!(
            "wrong-argument sibling must reject through kernel TypeMismatch, got {other:?}"
        ),
    }
}

#[test]
fn ac8_direct_lookup_member_reflection_helper_elaborates() {
    let mut env = mk_env();
    elab_ok(
        &mut env,
        "data MiniTree k v = MiniLeaf | MiniNode (MiniTree k v) k v (MiniTree k v)",
    );
    elab_ok(
        &mut env,
        "fn mini_lookup (k : Type) (v : Type) (leq : k -> k -> Bool) \
           (key : k) (m : MiniTree k v) : Option v = \
           match m { \
             MiniLeaf |-> None v ; \
             MiniNode l k2 v2 r |-> match leq key k2 { \
               True |-> match leq k2 key { \
                 True |-> Some v v2 ; \
                 False |-> mini_lookup k v leq key l \
               } ; \
               False |-> mini_lookup k v leq key r \
             } \
           }",
    );
    elab_ok(
        &mut env,
        "fn mini_member (k : Type) (v : Type) (leq : k -> k -> Bool) \
           (key : k) (m : MiniTree k v) : Bool = \
           match mini_lookup k v leq key m { None |-> False ; Some x |-> True }",
    );
    elab_ok(
        &mut env,
        "theorem mini_lookup_none_from_member_false_hit (v : Type) (val : v) \
           (h : Equal Bool True False) \
           : Equal (Option v) (Some v val) (None v) = \
           absurd h",
    );
    elab_ok(
        &mut env,
        "theorem mini_lookup_none_from_member_false \
           (k : Type) (v : Type) (leq : k -> k -> Bool) \
           (key : k) (m : MiniTree k v) \
           : Equal Bool (mini_member k v leq key m) False -> \
             Equal (Option v) (mini_lookup k v leq key m) (None v) = \
           match m { \
             MiniLeaf |-> \\h. Proved ; \
             MiniNode l k2 v2 r |-> match leq key k2 { \
               True |-> match leq k2 key { \
                 True |-> mini_lookup_none_from_member_false_hit v v2 ; \
                 False |-> mini_lookup_none_from_member_false k v leq key l \
               } ; \
               False |-> mini_lookup_none_from_member_false k v leq key r \
             } \
           }",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Additional robustness tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn data_recursive_type_accepts() {
    let mut env = mk_env();
    elab_ok(&mut env, "data NatL = Zero | Succ NatL");

    assert!(env.globals.contains_key("Zero"));
    assert!(env.globals.contains_key("Succ"));

    // Simple match on Zero inline.
    let id = elab_ok(
        &mut env,
        "let is_zero : Int = match Zero { Zero |-> 1 ; Succ n |-> 0 }",
    );
    let body = body_of(&env, id);
    assert!(matches!(body, Term::Elim { .. }));

    // ι-reduces on Zero.
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);
    assert_eq!(
        reduced,
        Term::IntLit(num_bigint::BigInt::from(1)),
        "recursive type match on Zero should ι-reduce to IntLit(1), got {:?}",
        reduced
    );
}

#[test]
fn unknown_ctor_in_pattern_is_error() {
    let mut env = mk_env();
    setup_color(&mut env);

    let result = elab(
        &mut env,
        "let bad : Int = match Red { Red |-> 0 ; Green |-> 1 ; Purple |-> 2 }",
    );
    assert!(result.is_err(), "unknown ctor 'Purple' should produce an error");
}

#[test]
fn data_two_arg_ctor_match_accepted() {
    let mut env = mk_env();
    elab_ok(&mut env, "data Pair = P Int Int");

    let id = elab_ok(
        &mut env,
        "let fst : Int = match P 1 2 { P x y |-> x }",
    );
    let body = body_of(&env, id);
    assert!(matches!(body, Term::Elim { .. }));

    // ι-reduces: elim_Pair M [λx.λy.x] (P 1 2) ⇝ (λx.λy.x) 1 2 ⇝ 1
    let ctx = Context::new();
    let reduced = whnf(&env.env, &ctx, &body);
    assert!(
        !matches!(reduced, Term::Elim { .. }),
        "Pair match should ι-reduce"
    );
}
