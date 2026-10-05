//! Decimal/Char DEMOTE→derived acceptance tests
//! (`conformance/surface/numbers/seed-decimal-char-demote.md`).
//!
//! Covers AC-D1/D2 (Decimal exact derivation, the F4 flip), AC-C1/C2/C3
//! (Char refinement, derived ops, Unicode scalar boundary behavior). AC-D3
//! (`Num`/`DecEq Decimal`
//! law instances) and `Ord Char` antisymmetry are re-homed to the
//! lawful-classes lane (Steward ruling) — not covered here. Pin-2 (`String`
//! → `Char` extraction computing the scalar proof) is deferred to the
//! extraction feature (Architect ruling, `evt_164tmsbevyk51`) — not covered
//! here either.
//!
//! Every case drives the REAL derived producer (`decimalAdd`/`decimalMul`/
//! `decimalEq`/`eqChar`/`leqChar`/`intToChar`, elaborated Ken definitions in
//! `ken-elaborator/src/decimal_char.rs`) via `elaborate_decl_v1` + `eval` —
//! never a hand-fed expected value
//! ([[conformance-hand-feeds-the-deliverable]]).

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{convert, convert_type, infer, whnf, Context, Decl, Term};

/// Elaborate and evaluate a single top-level `view` declaration, seeding the
/// literal side-table from the elaborator's own `num_values` (never a
/// hand-built literal value).
fn eval_view(src: &str) -> EvalVal {
    let mut env = ElabEnv::new().expect("prelude init");
    let r = env.elaborate_decl_v1(src).expect("elaborates");
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, v) in &env.num_values {
        let val = match v {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Float(f) => EvalVal::Float(*f),
            NumericLitVal::Float32(f) => EvalVal::Float32(*f),
            NumericLitVal::Decimal { coeff, exp } => {
                ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
            NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
        };
        store.num_values.insert(*id, val);
    }
    match env.env.lookup(r.def_id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, &mut store),
        other => panic!(
            "expected a checked Transparent def, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// Extract `(coeff, exp)` from a `Decimal` `Ctor` value — never constructs
/// one, only inspects the real producer's output.
fn as_decimal(v: &EvalVal) -> (i64, i64) {
    match v {
        EvalVal::Ctor { args, .. } => {
            let coeff = match &args[0] {
                EvalVal::Int(n) => *n,
                other => panic!("expected Int coeff, got {:?}", other),
            };
            let exp = match &args[1] {
                EvalVal::Int(n) => *n,
                other => panic!("expected Int exp, got {:?}", other),
            };
            (coeff, exp)
        }
        other => panic!("expected a Decimal Ctor, got {:?}", other),
    }
}

// ── AC-D1/D2 — Decimal derivation is exact; the F4 flip (soundness) ────────

/// surface/numbers/decimal-mul-exact-flips-vs-saturating (soundness)
#[test]
fn decimal_mul_exact_flips_vs_saturating() {
    // Decimal(coeff=10^10, exp=0) * Decimal(coeff=10^10, exp=0):
    // coeff product 10^20 > i64::MAX (~9.22e18) — the old native
    // `mul_decimal` used `saturating_mul`, clamping to i64::MAX. The
    // derived `decimalMul` runs bignum `mul_int`, exact.
    let result = eval_view(
        "const t = decimalMul (MkDecimalPair 10000000000 0) (MkDecimalPair 10000000000 0)",
    );
    match &result {
        EvalVal::Ctor { args, .. } => {
            match &args[0] {
                EvalVal::BigInt(n) => {
                    assert_eq!(
                        n.to_string(),
                        "100000000000000000000",
                        "exact 10^20, no saturation"
                    );
                }
                EvalVal::Int(n) => panic!("10^20 must widen to BigInt, got Int({})", n),
                other => panic!("expected Int/BigInt coeff, got {:?}", other),
            }
            match &args[1] {
                EvalVal::Int(0) => {}
                other => panic!("expected exp Int(0), got {:?}", other),
            }
        }
        other => panic!("expected a Decimal Ctor, got {:?}", other),
    }
}

/// surface/numbers/decimal-eq-distinct-flips-vs-false-true (soundness) —
/// the F4 closure: two DISTINCT decimals that the old saturating
/// `decimal_eq` compared (wrongly) `True`.
#[test]
fn decimal_eq_distinct_flips_vs_false_true() {
    // a = Decimal(coeff=i64::MAX, exp=0); b = Decimal(coeff=i64::MAX, exp=1)
    // (b = 10 * a exactly: 92233720368547758070 vs 9223372036854775807).
    // Old `decimal_eq` aligned b's coeff via `saturating_mul(10)`, which
    // saturates to i64::MAX, then compared `i64::MAX == i64::MAX` -> true
    // (the F4 hole: two distinct decimals comparing equal). The derived
    // `decimalEq` aligns exactly via `decimalPow10`/`mul_int`, so this must
    // reduce to `false`.
    let result = eval_view(
        "const t = decimalEq (MkDecimalPair 9223372036854775807 0) \
                             (MkDecimalPair 9223372036854775807 1)",
    );
    assert_eq!(
        result,
        EvalVal::Bool(false),
        "distinct decimals must compare unequal — the F4 saturating false-True hole must not recur"
    );
}

/// A same-value check on the same vector, aligned the other direction, to
/// confirm `decimalEq` isn't just accidentally always false.
#[test]
fn decimal_eq_same_value_both_alignments_true() {
    let result = eval_view(
        "const t = decimalEq (MkDecimalPair 9223372036854775807 1) \
                             (MkDecimalPair 92233720368547758070 0)",
    );
    assert_eq!(
        result,
        EvalVal::Bool(true),
        "9223372036854775807 * 10^1 (as coeff 9223372036854775807, exp 1) \
         must equal the same value spelled as (coeff 92233720368547758070, exp 0)"
    );
}

/// Sanity: `decimalAdd` at the same exponent needs no alignment.
#[test]
fn decimal_add_same_exponent() {
    let result = eval_view("const t = decimalAdd (MkDecimalPair 1 0) (MkDecimalPair 2 0)");
    assert_eq!(as_decimal(&result), (3, 0));
}

// ── AC-C1 — Char refinement (soundness) ─────────────────────────────────────

/// surface/numbers/char-excludes-surrogates (soundness)
///
/// Promise class: durable invariant.
/// MEASURED: a valid character literal elaborates, while the surrogate escape
/// is an `InvalidEscape` with the exact scalar-specific diagnostic.
/// CLAIMED: the character-literal route excludes surrogate code points.
/// THE GAP: this pins only the literal route; it does not establish named
/// refinement introductions.
#[test]
fn char_literal_rejects_surrogate_escape() {
    let mut env = ElabEnv::new().expect("prelude init");
    env.elaborate_decl_v1("const a : Char = 'a'")
        .expect("a valid scalar character literal elaborates as Char");

    let error = env
        .elaborate_decl_v1(r"const b : Char = '\u{D800}'")
        .expect_err("a surrogate character escape must be rejected");
    assert!(
        matches!(
            &error,
            ElabError::InvalidEscape { reason, .. }
                if reason == "unicode escape is not a valid scalar value"
        ),
        "surrogate rejection must identify the invalid scalar: {error:?}"
    );
    assert_eq!(
        error.to_string(),
        "invalid escape at 18-26: unicode escape is not a valid scalar value"
    );
}

/// surface/numbers/char-is-isscalar-refinement (soundness)
#[test]
fn char_is_isscalar_refinement() {
    // `Char` erases to `Int` (refinement-erasure) — a value of type `Char`
    // reduces to a plain `Int`, never a `List`/`u32`-tagged carrier.
    let result = eval_view("const t : Char = 65");
    assert_eq!(result, EvalVal::Int(65));
}

// ── AC-C2 — derived Char ops over the projection (soundness) ───────────────

/// surface/numbers/char-eq-and-ord-on-projection (soundness)
#[test]
fn char_eq_and_ord_on_projection() {
    assert_eq!(eval_view("const t = eqChar 65 65"), EvalVal::Bool(true));
    assert_eq!(eval_view("const t = eqChar 65 66"), EvalVal::Bool(false));
    // The order PAIR (accept a<=b while reject b<=a) — a single accept is
    // green-vs-green under an orientation flip.
    assert_eq!(eval_view("const t = leqChar 65 66"), EvalVal::Bool(true));
    assert_eq!(eval_view("const t = leqChar 66 65"), EvalVal::Bool(false));
    assert_eq!(eval_view("const t = charToInt 65"), EvalVal::Int(65));
}

// AC-C3 — surrogate/OOR reject, flips vs inRangeBool := True (soundness)

/// surface/numbers/int-to-char-rejects-surrogate-and-oor (soundness)
///
/// Replaces the source-text oracle in
/// `docs/program/issues/TEST-SOURCE-TEXT-ORACLE-RETIRE.md`, item 13.
/// Promise class: normative compatibility vector.
///
/// MEASURED: the real `intToChar` elaboration and evaluator produce the `Some`
/// or `None` constructor at each codepoint. CLAIMED: the row's surrogate and
/// out-of-range cases reject while U+0041 accepts. THE GAP: this vector covers
/// these edges, not every interior value or negative integer.
#[test]
fn int_to_char_unicode_scalar_boundaries() {
    let cases = [
        (0xD7FF_u32, true),
        (0xE000, true),
        (0xD800, false),
        (0xDFFF, false),
        (0x10FFFF, true),
        (0x110000, false),
        (0x41_u32, true),
    ];
    for (codepoint, accepted) in cases {
        let result = eval_view(&format!("const t = intToChar {codepoint}"));
        let is_some = match result {
            EvalVal::Ctor { args, .. } if args.len() == 2 => true,
            EvalVal::Ctor { args, .. } if args.len() == 1 => false,
            other => panic!("intToChar U+{codepoint:04X} must return Option, got {other:?}"),
        };
        assert_eq!(
            is_some, accepted,
            "intToChar U+{codepoint:04X} accept/reject boundary"
        );
    }
}

/// surface/numbers/char-deceq-collapses-on-codepoint (soundness, hard-AC)
///
/// Replaces the source-text oracle in
/// `docs/program/issues/TEST-SOURCE-TEXT-ORACLE-RETIRE.md`, item 13 (structural half).
/// Promise class: durable invariant. MEASURED: `isScalar 97` is Ω-sorted, and
/// two distinct neutral proofs of it are definitionally equal in the kernel.
/// CLAIMED: `isScalar` is proof-irrelevant, so no scalar proof can distinguish
/// two `Char`s with one codepoint. THE GAP: only codepoint 97 is instantiated;
/// `Char` itself erases to `Int`, so the value half is pinned by the eqChar test.
#[test]
fn is_scalar_proofs_are_irrelevant() {
    let mut env = ElabEnv::new().expect("prelude init");
    let empty = Context::new();
    let int97 = Term::IntLit(97u32.into());
    let is_scalar = Term::const_(env.globals["isScalar"], vec![]);
    let pred = Term::app(is_scalar, int97.clone());

    let pred_sort = infer(&env.env, &empty, &pred).expect("isScalar 97 is well typed");
    let reduced_pred_sort = whnf(&env.env, &empty, &pred_sort);
    let omega_sort = match &reduced_pred_sort {
        Term::Omega(level) => Term::Omega(level.clone()),
        other => panic!("isScalar 97 must reduce to an Ω sort, got {other:?}"),
    };
    assert!(convert_type(&env.env, &empty, &pred_sort, &omega_sort));

    let mut ctx = Context::new();
    ctx.push(pred.clone());
    ctx.push(pred.clone());
    let proof_left = Term::var(1);
    let proof_right = Term::var(0);
    let proof_left_ty = infer(&env.env, &ctx, &proof_left).expect("left proof variable");
    let proof_right_ty = infer(&env.env, &ctx, &proof_right).expect("right proof variable");
    assert!(convert_type(&env.env, &ctx, &proof_left_ty, &pred));
    assert!(convert_type(&env.env, &ctx, &proof_right_ty, &pred));
    assert!(
        convert(&env.env, &ctx, &pred, &proof_left, &proof_right),
        "distinct neutral proofs of an Ω predicate must convert"
    );

    env.elaborate_decl("fn tag97 (c : Int) : Type 0 = Bool")
        .expect("Type-sorted fixture predicate elaborates");
    let type_pred = Term::app(Term::const_(env.globals["tag97"], vec![]), int97);
    let type_sort =
        infer(&env.env, &empty, &type_pred).expect("Type-sorted fixture predicate is well typed");
    let reduced_type_sort = whnf(&env.env, &empty, &type_sort);
    assert!(
        matches!(&reduced_type_sort, Term::Type(_)),
        "fixture predicate must be Type-sorted, got {reduced_type_sort:?}"
    );

    let mut type_ctx = Context::new();
    type_ctx.push(type_pred.clone());
    type_ctx.push(type_pred.clone());
    assert!(
        !convert(
            &env.env,
            &type_ctx,
            &type_pred,
            &Term::var(1),
            &Term::var(0)
        ),
        "distinct neutral Bool values must remain proof-relevant"
    );
}

/// Value-consequence of pin 1: `eqChar` (which routes through `eq_int` on
/// the projection, per pin 1's payoff) agrees with direct `Int` equality —
/// observable only because `IsTrue` is proof-irrelevant so no separate
/// scalar-proof component can desync the comparison.
#[test]
fn char_deceq_pin1_value_consequence() {
    assert_eq!(eval_view("const t = eqChar 97 97"), EvalVal::Bool(true));
}
