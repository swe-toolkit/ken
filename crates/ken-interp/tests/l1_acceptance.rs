//! `l1_acceptance.rs` holds L1 numeric tower acceptance tests.
//!
//! `CI-ASSERTIONLESS-L1` establishes only that AC-2 now has its production-fed
//! witness and that the three named ignored placeholders are honestly marked
//! as non-cover. The machine-checked exemption artifact and its readmission
//! conditions live in `.github/ignored-test-exemptions.toml`. This header makes
//! no claim about any other row's conformance-cover status.

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal, ObligationKind};
use ken_elaborator::extract::{v2_extract, ProvKind};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::env::Context;
use ken_kernel::{convert, Decl, GlobalId, KernelError, Term};

// ── test infrastructure ──────────────────────────────────────────────────────

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, v) in &env.num_values {
        store.num_values.insert(*id, lit_to_eval(v, mkdecimalpair_id));
    }
    store
}

fn lit_to_eval(v: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match v {
        NumericLitVal::Int(n) => EvalVal::from(n.clone()),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
    }
}

fn eval_def(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        _ => EvalVal::Unknown,
    }
}

/// Assert that the type of a definition's constant is the expected type term.
fn assert_def_type(env: &ElabEnv, id: GlobalId, expected_ty: &Term) {
    let (_, ty) = env.env.const_type(id).expect("def has no type");
    assert_eq!(&ty, expected_ty, "definition has wrong type");
}

// ── AC1: Int exactness above 2⁵³ ────────────────────────────────────────────

/// surface/numbers/int-arbitrary-precision-above-2^53 (soundness)
/// `100000000000000000000 + 1 : Int` reduces to exact `100000000000000000001`.
/// The off-grid witness: 10²⁰ is f64-exact; 10²⁰+1 rounds to 10²⁰ under f64.
#[test]
fn ac1_int_exact_above_2_53() {
    let mut env = ElabEnv::new().unwrap();
    // Elaborate addition of two large Int literals via the + op.
    // 10^20 in Int: 100000000000000000000
    // 10^20 + 1:    100000000000000000001
    let result = env.elaborate_decl_v1(
        "const big_sum = (100000000000000000000 : Int) + (1 : Int)"
    ).unwrap();
    let mut store = make_store(&env);
    let val = eval_def(&env, &mut store, result.def_id);

    // The f64-rounded value is 10^20 (10^20 is exactly representable in f64).
    // The exact Int value is 10^20+1.
    let f64_rounded: i128 = 100000000000000000000_i128; // 10^20
    let exact: i128 = 100000000000000000001_i128;       // 10^20 + 1

    // Structural value assertion: value must be exact, NOT f64-rounded.
    assert_ne!(val, EvalVal::from(f64_rounded), "value must not be f64-rounded");
    assert_eq!(val, EvalVal::from(exact), "value must be exact (AC1 off-grid witness)");
}

// ── AC2: literal types are distinct ─────────────────────────────────────────

/// surface/numbers/literal-defaulting-distinct-types
/// `2`, `2.0`, `2.0d` have three distinct types: `Int`, `Float`, `Decimal`.
#[test]
fn ac2_literal_types_distinct() {
    let mut env = ElabEnv::new().unwrap();

    // `2` defaults to Int
    let int_result = env.elaborate_decl_v1("const n = 2").unwrap();
    let int_ty = Term::const_(env.numeric_env.int_id, vec![]);
    assert_def_type(&env, int_result.def_id, &int_ty);

    // `2.0` defaults to Float
    let float_result = env.elaborate_decl_v1("const f = 2.0").unwrap();
    let float_ty = Term::const_(env.numeric_env.float_id, vec![]);
    assert_def_type(&env, float_result.def_id, &float_ty);

    // `2.0d` defaults to Decimal
    let dec_result = env.elaborate_decl_v1("const d = 2.0d").unwrap();
    let dec_ty = Term::const_(env.numeric_env.decimal_id, vec![]);
    assert_def_type(&env, dec_result.def_id, &dec_ty);

    // `1.5f32` defaults to Float32
    let f32_result = env.elaborate_decl_v1("const g = 1.5f32").unwrap();
    let f32_ty = Term::const_(env.numeric_env.float32_id, vec![]);
    assert_def_type(&env, f32_result.def_id, &f32_ty);
}

/// surface/numbers/expected-type-overrides-default
/// `1` in `Int64` position elaborates at `Int64`, not `Int`.
#[test]
fn ac2_expected_type_overrides_default() {
    let mut env = ElabEnv::new().unwrap();
    // Preserve the old compile-only probe as the baseline: it witnesses only
    // elaboration success and does not expose the literal's own type.
    let _legacy_compile_only = env
        .elaborate_decl_v1("fn f (x : Int64) : Int64 = x + 1")
        .unwrap();

    let defaulted = env.elaborate_decl_v1("const defaulted = 1").unwrap();
    let expected = env
        .elaborate_decl_v1("const expected : Int64 = 1")
        .unwrap();

    // The same literal takes two observably different elaboration paths. The
    // unconstrained form fires the default table and becomes a kernel IntLit.
    let defaulted_body = env
        .env
        .transparent_body(defaulted.def_id)
        .expect("defaulted literal definition must be transparent")
        .1;
    assert!(
        matches!(&defaulted_body, Term::IntLit(_)),
        "unconstrained integer literal must take the Int default path"
    );

    // The expected-type form must instead emit a typed literal constant. If
    // it defaulted to Int and only unified afterwards, this body would also be
    // IntLit even though elaboration still succeeds.
    let expected_body = env
        .env
        .transparent_body(expected.def_id)
        .expect("expected-type literal definition must be transparent")
        .1;
    let expected_literal_id = match expected_body {
        Term::Const { id, .. } => id,
        other => panic!(
            "expected-type literal must bypass the Int default path; got {other:?}"
        ),
    };
    let (_, expected_literal_ty) = env
        .env
        .const_type(expected_literal_id)
        .expect("expected-type literal constant must have a type");
    let int64_ty = Term::const_(env.numeric_env.int64_id, vec![]);
    assert_eq!(
        expected_literal_ty, int64_ty,
        "literal in an Int64 position must itself elaborate at Int64"
    );
}

// ── AC3: overflow obligation emitted ────────────────────────────────────────

/// surface/numbers/fixed-width-overflow-emits-obligation (soundness)
/// `a + b : Int32` on unconstrained operands emits a PartialPrim obligation.
#[test]
fn ac3_overflow_obligation_emitted_int32() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "fn f (a : Int32) (b : Int32) : Int32 = a + b"
    ).unwrap();

    // Structural obligation observation (AC3 requirement: NOT "it compiles").
    assert_eq!(
        result.obligations.len(), 1,
        "bare + on Int32 must emit exactly 1 no-overflow obligation"
    );
    assert!(
        matches!(result.obligations[0].kind, ObligationKind::PartialPrim),
        "obligation must be PartialPrim (overflow side-condition)"
    );

    // V2 extraction: verify ProvKind
    let extracted = v2_extract(&result);
    assert_eq!(extracted.obligations.len(), 1);
    assert!(
        matches!(extracted.obligations[0].provenance.kind, ProvKind::PartialPrim),
        "V2 triple must carry PartialPrim provenance"
    );
}

/// surface/numbers/in-range-overflow-obligation-discharged
/// Same bare `a + b : Int32` emits the obligation; discharging via refinement requires V3.
/// This tests that the obligation structure is present for future V3 discharge.
#[test]
fn ac3_overflow_obligation_dischargeable_structure() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "fn g (a : Int32) (b : Int32) : Int32 = a + b"
    ).unwrap();

    // The obligation is emitted — the V3 prover would discharge it for in-range operands.
    assert_eq!(result.obligations.len(), 1);
    let obl = &result.obligations[0];
    // goal_closed should be a Pi(Int32, Pi(Int32, phi)) structure
    assert!(
        matches!(&obl.goal_closed, Term::Pi(_, _)),
        "goal_closed must be Pi-abstracted over the context"
    );
    // The hole is open (in trusted_base — undischarged)
    assert!(env.is_open_hole(obl.hole_id), "obligation hole must be open");
}

// ── AC4: no silent wrap ──────────────────────────────────────────────────────

/// surface/numbers/bare-overflow-never-silently-wraps (soundness)
/// `(100 : Int8) + (100 : Int8)` emits a no-overflow obligation and does NOT
/// silently produce the wrapped value -56.
#[test]
fn ac4_bare_overflow_never_wraps_silently() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "const ovf = (100 : Int8) + (100 : Int8)"
    ).unwrap();

    // Must emit an obligation (the overflowing obligation is unsatisfiable here,
    // but it must be EMITTED — not silently wrapped).
    assert_eq!(
        result.obligations.len(), 1,
        "bare + on Int8 must emit the no-overflow obligation, never silent-wrap"
    );
    assert!(matches!(result.obligations[0].kind, ObligationKind::PartialPrim));

    // The obligation hole is open (not discharged — 200 ∉ [-128,127]).
    assert!(env.is_open_hole(result.obligations[0].hole_id));

    // Evaluate: the op runs (wrapping at the interpreter level for now), but the
    // OBLIGATION signals the overflow. The obligation is what guards correctness.
    // We do NOT assert the evaluated value is -56 (that would be silent-wrap behavior).
}

/// surface/numbers/explicit-wrapping-op-is-modular
/// `(100 : Int8) +% (100 : Int8)` reduces to -56 with no obligation.
#[test]
fn ac4_explicit_wrapping_is_modular() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "const wrap_ovf = (100 : Int8) +% (100 : Int8)"
    ).unwrap();

    // No obligation for explicit wrapping.
    assert_eq!(
        result.obligations.len(), 0,
        "+% (wrapping add) must NOT emit a no-overflow obligation"
    );

    // Evaluates to the modular result: 100 + 100 = 200; 200 - 256 = -56 as i8.
    let mut store = make_store(&env);
    let val = eval_def(&env, &mut store, result.def_id);
    assert_eq!(val, EvalVal::Int(-56), "+% (Int8) 100 100 must give -56 (modular)");
}

// ── AC5: no implicit coercion ────────────────────────────────────────────────

/// surface/numbers/no-implicit-cross-type-coercion
/// `(x : Int) + (y : Int64)` without an explicit conversion is a type error.
#[test]
fn ac5_no_implicit_cross_type_coercion() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl_v1("fn same (x : Int) (y : Int) : Int = x + y")
        .expect("matching Int operands must accept");

    let error = env
        .elaborate_decl_v1("fn mixed (x : Int) (y : Int64) : Int = x + y")
        .expect_err("mixed Int and Int64 operands must reject without an explicit conversion");
    let ElabError::KernelRejected {
        error: KernelError::TypeMismatch { expected, found },
        ..
    } = error
    else {
        panic!("mixed operands must reach a kernel operand type mismatch, got {error:?}")
    };
    assert_eq!(*expected, Term::const_(env.numeric_env.int_id, vec![]));
    assert_eq!(*found, Term::const_(env.numeric_env.int64_id, vec![]));
}

/// Not conformance cover; waits on L-classes to expose `Int.toInt64`.
#[test]
#[ignore = "explicit conversions require L-classes or a separate conversion WP"]
fn ac5_explicit_conversion_is_partial_option() {
    let mut env = ElabEnv::new().unwrap();
    // Int.toInt64 : Int → Option Int64  (total, may fail if out of range)
    let _result = env.elaborate_decl_v1(
        "fn f (x : Int) = Int.toInt64 x"
    ).unwrap();
}

// ── AC6: Decimal exact / Float honest ────────────────────────────────────────

/// surface/numbers/decimal-exact-while-float-honest
/// `0.1d + 0.2d == 0.3d` is `true`; `0.1 + 0.2 == 0.3` (Float) is `false`.
#[test]
fn ac6_decimal_exact() {
    let mut env = ElabEnv::new().unwrap();
    let dec_eq = env.elaborate_decl_v1(
        "const decimal_ok = (0.1d + 0.2d) == 0.3d"
    ).unwrap();
    let mut store = make_store(&env);
    let val = eval_def(&env, &mut store, dec_eq.def_id);
    assert_eq!(val, EvalVal::Bool(true), "0.1d + 0.2d == 0.3d must be true (exact)");
}

#[test]
fn ac6_float_not_exact() {
    let mut env = ElabEnv::new().unwrap();
    let float_eq = env.elaborate_decl_v1(
        "const float_not_ok = (0.1 + 0.2) == 0.3"
    ).unwrap();
    let mut store = make_store(&env);
    let val = eval_def(&env, &mut store, float_eq.def_id);
    assert_eq!(
        val,
        EvalVal::Bool(false),
        "0.1 + 0.2 == 0.3 (Float) must be false (IEEE binary64 rounding)"
    );
}

// ── §3.1: Int div-by-zero obligation ────────────────────────────────────────

/// Not conformance cover; waits on integer division op registration.
#[test]
#[ignore = "integer division not yet in scope for L1; requires div op registration"]
fn sec31_int_div_zero_emits_obligation() {
    let mut env = ElabEnv::new().unwrap();
    let _result = env.elaborate_decl_v1(
        "fn f (a : Int) (b : Int) = a / b"
    ).unwrap();
}

// ── §6.1: primitive op runtime value and neutral conversion ──────────────────

/// surface/numbers/primitive-op-runtime-value-k3-conversion-deferred
/// The interpreter evaluates `2 + 3 : Int` to `5`, while kernel conversion
/// keeps the primitive operation application distinct from the literal `5`.
#[test]
fn sec61_literal_reduces_in_kernel() {
    let mut env = ElabEnv::new().unwrap();
    let sum = env.elaborate_decl_v1(
        "const five = (2 : Int) + (3 : Int)"
    ).unwrap();
    let literal = env.elaborate_decl_v1(
        "const literal_five = (5 : Int)"
    ).unwrap();
    let mut store = make_store(&env);
    let val = eval_def(&env, &mut store, sum.def_id);
    assert_eq!(val, EvalVal::Int(5), "the interpreter must evaluate 2 + 3 : Int to 5");

    let sum_body = env
        .env
        .transparent_body(sum.def_id)
        .expect("five must be a transparent definition")
        .1;
    let literal_body = env
        .env
        .transparent_body(literal.def_id)
        .expect("literal_five must be a transparent definition")
        .1;
    let int_ty = Term::const_(env.numeric_env.int_id, vec![]);
    assert!(
        !convert(&env.env, &Context::new(), &int_ty, &sum_body, &literal_body),
        "kernel conversion must keep primitive addition neutral against literal 5"
    );
}

// ── §6.2: algebraic law is a proposition, not a kernel rule ─────────────────

/// surface/numbers/algebraic-law-is-proposition-not-reduction (soundness)
/// On abstract operands, `a + b` and `b + a` are NOT definitionally equal
/// (commutativity holds only propositionally).
/// This test queries kernel conversion directly over the production-elaborated
/// abstract operation bodies.
#[test]
fn sec62_abstract_add_is_neutral() {
    let mut env = ElabEnv::new().unwrap();
    let result_ab = env.elaborate_decl_v1(
        "fn add_ab (a : Int) (b : Int) : Int = a + b"
    ).unwrap();
    let result_ba = env.elaborate_decl_v1(
        "fn add_ba (a : Int) (b : Int) : Int = b + a"
    ).unwrap();

    let body_ab = env
        .env
        .transparent_body(result_ab.def_id)
        .expect("add_ab must be a transparent definition")
        .1;
    let body_ba = env
        .env
        .transparent_body(result_ba.def_id)
        .expect("add_ba must be a transparent definition")
        .1;
    let Term::Lam(_, body_ab) = body_ab else {
        panic!("add_ab must bind abstract operand a")
    };
    let Term::Lam(_, body_ab) = *body_ab else {
        panic!("add_ab must bind abstract operand b")
    };
    let Term::Lam(_, body_ba) = body_ba else {
        panic!("add_ba must bind abstract operand a")
    };
    let Term::Lam(_, body_ba) = *body_ba else {
        panic!("add_ba must bind abstract operand b")
    };

    let int_ty = Term::const_(env.numeric_env.int_id, vec![]);
    let mut ctx = Context::new();
    ctx.push(int_ty.clone()); // a : Int (Var 1 after the next push)
    ctx.push(int_ty.clone()); // b : Int (Var 0)
    assert!(
        !convert(&env.env, &ctx, &int_ty, &body_ab, &body_ba),
        "kernel conversion must reject abstract a + b ≟ b + a"
    );
}

// ── §2.4: Char excludes surrogates ──────────────────────────────────────────

/// Not conformance cover; waits on Char literal syntax.
#[test]
#[ignore = "Char literal syntax not yet in scope for L1"]
fn sec24_char_excludes_surrogates() {
    // Char literals + surrogate validation deferred to the surface syntax WP.
}

// ── Int8 obligation cross-case sweep ─────────────────────────────────────────

/// Cross-case sweep: `+` on Int8 also emits PartialPrim obligation.
/// Confirms the overflow class is uniform across widths.
#[test]
fn sweep_int8_overflow_emits_obligation() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "fn h (a : Int8) (b : Int8) : Int8 = a + b"
    ).unwrap();
    assert_eq!(result.obligations.len(), 1);
    assert!(matches!(result.obligations[0].kind, ObligationKind::PartialPrim));
}

/// Int (arbitrary-precision) `+` is TOTAL — no obligation emitted.
#[test]
fn sweep_int_total_no_obligation() {
    let mut env = ElabEnv::new().unwrap();
    let result = env.elaborate_decl_v1(
        "fn total_add (a : Int) (b : Int) : Int = a + b"
    ).unwrap();
    assert_eq!(
        result.obligations.len(), 0,
        "Int + is total (arbitrary-precision); must not emit overflow obligation"
    );
}
