use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::Decl;
use std::collections::BTreeSet;

const NEED: &str = "fn need (b : Bool) : Bool requires Equal Bool b True = b\n";
const NEEDI: &str = "fn needi (x : Int) : Int requires Equal Int x 5 = x\n";
const SUBSET: &str = "fn h (x : {v : Bool | Equal Bool v True}) : Bool = x\n";

#[derive(Clone, Copy)]
enum Expected {
    NoHoles,
    RequiresHole,
    PartialPrimHole,
    SubsetObligation,
}

// MEASURED: a fresh environment accepts the exact declaration, reports the
// expected obligation provenance, and (for Requires) registers an Opaque hole
// whose checked type equals its reported closed goal. CLAIMED: the inferred
// branch answers its own split demand without silently trusting a Requires
// proof. THE GAP: these individual source rows do not exhaust arbitrary
// nested ifs; a4 exercises the enclosing checked split and controls exercise
// the other call paths.
fn accepted(source: &str, function: &str, expected: Expected) {
    let mut env = ElabEnv::new().expect("prelude");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let results = env
        .elaborate_file_v1(source)
        .expect("source must elaborate");
    let result = results
        .iter()
        .find(|result| result.name == function)
        .expect("target result");
    let expected_count = usize::from(!matches!(expected, Expected::NoHoles));
    assert_eq!(
        result.obligations.len(),
        expected_count,
        "{function} obligation count"
    );
    if let Some(obligation) = result.obligations.first() {
        assert!(
            matches!(
                (&expected, &obligation.kind),
                (Expected::RequiresHole, ObligationKind::Requires)
                    | (Expected::PartialPrimHole, ObligationKind::PartialPrim)
                    | (
                        Expected::SubsetObligation,
                        ObligationKind::RefinementIntroduction
                    )
            ),
            "{function} obligation provenance: {:?}",
            obligation.kind
        );
        if matches!(expected, Expected::RequiresHole | Expected::PartialPrimHole) {
            assert!(
                matches!(env.env.lookup(obligation.hole_id),
                Some(Decl::Opaque { ty, .. }) if *ty == obligation.goal_closed),
                "{function} Requires hole must retain its checked closed goal"
            );
        }
    }
    let reported: BTreeSet<_> = results
        .iter()
        .flat_map(|result| {
            result
                .obligations
                .iter()
                .map(|obligation| obligation.hole_id)
        })
        .collect();
    let delta: BTreeSet<_> = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect();
    if !matches!(expected, Expected::SubsetObligation) {
        assert_eq!(delta, reported, "{function} trust delta must be reported");
    }
}

macro_rules! accept_row {
    ($test:ident, $fixture:expr, $name:literal, $body:literal, $holes:ident) => {
        #[test]
        fn $test() {
            accepted(&format!("{}{}", $fixture, $body), $name, Expected::$holes);
        }
    };
}

accept_row!(
    a1_inferred_let_then_requires,
    NEED,
    "a1",
    "fn a1 (b : Bool) : Bool = let y = if b then need b else b in y",
    NoHoles
);
accept_row!(
    a2_inferred_match_scrutinee_requires,
    NEED,
    "a2",
    "fn a2 (b : Bool) : Bool = match (if b then need b else b) { True ↦ True ; False ↦ False }",
    NoHoles
);
accept_row!(
    a3_independent_int_requires_stays_open,
    NEEDI,
    "a3",
    "fn a3 (x : Int) (b : Bool) : Int = let y = if b then needi x else x in y",
    RequiresHole
);
accept_row!(
    a4_nested_checked_if,
    NEED,
    "a4",
    "fn a4 (b : Bool) (c : Bool) : Bool = if c then (let y = if b then need b else b in y) else b",
    NoHoles
);
accept_row!(
    s1_subset_introduction,
    SUBSET,
    "s1",
    "fn s1 (b : Bool) : Bool = let y = if b then h b else b in y",
    SubsetObligation
);

accept_row!(
    control_checked_if,
    NEED,
    "c1",
    "fn c1 (b : Bool) : Bool = if b then need b else b",
    NoHoles
);
accept_row!(
    control_checked_if_subset,
    SUBSET,
    "c1s",
    "fn c1s (b : Bool) : Bool = if b then h b else b",
    SubsetObligation
);
accept_row!(
    control_inferred_let_body,
    NEED,
    "c2",
    "fn c2 (b : Bool) : Bool = let y = need b in y",
    RequiresHole
);
accept_row!(
    control_lambda_in_checked_if,
    NEED,
    "c3",
    "fn c3 (b : Bool) : Bool -> Bool = if b then (λx. need b) else (λx. x)",
    NoHoles
);
accept_row!(control_polymorphic_argument, NEED, "c5", "fn idp (a : Type) (x : a) : a = x\nfn c5 (b : Bool) : Bool = idp Bool (if b then need b else b)", NoHoles);
accept_row!(
    control_division_in_inferred_if,
    "",
    "c6",
    "fn c6 (x : Int) (d : Int) (b : Bool) : Int = let y = if b then x / d else x in y",
    PartialPrimHole
);
accept_row!(
    control_plain_inferred_if,
    "",
    "c7",
    "fn c7 (x : Int) (b : Bool) : Int = let y = if b then x else x in y",
    NoHoles
);
accept_row!(
    else_branch_unprovable_requires_stays_open,
    NEED,
    "c8",
    "fn c8 (b : Bool) : Bool = let y = if b then b else need b in y",
    RequiresHole
);

// The pre-existing literal-pattern refusal is a typed diagnostic on both
// trees, not a successful inferred-if path and not an E0082 split escape.
#[test]
fn control_literal_pattern_stays_a_typed_refusal() {
    let mut env = ElabEnv::new().expect("prelude");
    let source =
        format!("{NEED}fn c4 (n : Nat) (b : Bool) : Bool = match n {{ 0 ↦ need b ; _ ↦ b }}");
    let error = env
        .elaborate_file_v1(&source)
        .expect_err("unsupported Nat literal pattern");
    assert!(
        matches!(error, ElabError::TypeMismatch { reason, .. }
        if reason.contains("numeric literal-pattern row is unsupported for carrier 'Nat'")),
        "refusal must come from the literal-pattern boundary, not a split demand"
    );
}
