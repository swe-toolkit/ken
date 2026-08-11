//! Causal controls for arbitrary-precision surface integer literals.

use ken_elaborator::{int_lit_val, ElabEnv, NumericLitVal};
use ken_kernel::{env::Context, whnf, Term};
use num_bigint::BigInt;

const HUGE: &str = "340282366920938463463374607431768211456";

fn huge() -> BigInt {
    HUGE.parse().expect("test literal is a BigInt")
}

/// Promise: a written integer larger than `i128::MAX` reaches both semantic
/// exits unchanged. MEASURED: the transparent kernel body and interpreter
/// value. CLAIMED: neither surface elaboration nor evaluation narrows `Int`.
/// GAP: native/fixed-width integer behavior is deliberately not generalized.
#[test]
fn huge_surface_int_reaches_kernel_and_evaluation_exactly() {
    let mut env = ElabEnv::new().expect("base env");
    let before: std::collections::BTreeSet<_> =
        env.env.trusted_base().into_iter().collect();
    let id = env
        .elaborate_decl(&format!("const huge : Int = {HUGE}"))
        .expect("arbitrary-precision surface Int must elaborate");
    let (_, body) = env
        .env
        .transparent_body(id)
        .expect("huge must be transparent");

    assert_eq!(body, Term::IntLit(huge()));

    let mut store = ken_interp::EvalStore::new();
    assert_eq!(
        ken_interp::eval(&[], &body, &env.env, &mut store),
        ken_interp::EvalVal::BigInt(huge())
    );

    let after: std::collections::BTreeSet<_> =
        env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "surface Int must not change trusted_base()");
}

/// Promise: the downstream arbitrary-precision literal carrier is exact too.
/// MEASURED: `int_lit_val` at `Int`, followed by the public evaluator bridge.
/// CLAIMED: a consumer cannot silently reintroduce the former `i128` carrier.
/// GAP: ordinary `Int` terms bypass this side table and are covered above.
#[test]
fn huge_downstream_numeric_carrier_reaches_eval_exactly() {
    let value = int_lit_val(&huge());
    let NumericLitVal::Int(value) = value else {
        panic!("Int carrier changed variant")
    };
    assert_eq!(value, huge());
    assert_eq!(
        ken_interp::EvalVal::from(value),
        ken_interp::EvalVal::BigInt(huge())
    );
}

/// Promise: equality for large literals is decided by the existing kernel
/// rule. MEASURED: two different decimal spellings elaborate independently,
/// then their kernel `Eq` reduces to `Top`. CLAIMED: no surface comparator is
/// involved. GAP: this does not add or change kernel equality machinery.
#[test]
fn kernel_equality_decides_equal_large_surface_spellings() {
    let mut env = ElabEnv::new().expect("base env");
    let plain = env
        .elaborate_decl(&format!("const huge_plain : Int = {HUGE}"))
        .expect("plain spelling");
    let padded = env
        .elaborate_decl(&format!("const huge_padded : Int = 000{HUGE}"))
        .expect("padded spelling");
    let (_, plain_body) = env.env.transparent_body(plain).expect("plain body");
    let (_, padded_body) = env.env.transparent_body(padded).expect("padded body");
    let goal = Term::Eq(
        Box::new(Term::const_(env.numeric_env.int_id, vec![])),
        Box::new(plain_body),
        Box::new(padded_body),
    );

    assert!(matches!(
        whnf(&env.env, &Context::new(), &goal),
        Term::Const { id, .. } if id == env.env.top_id()
    ));
}

/// Promise: widening the integer payload does not disturb the special
/// constructor-result spellings `0` and `n + 1`. MEASURED: the existing
/// indexed-family surface elaborates and records both constructor indices.
/// CLAIMED: BigInt equality replaces only the former literal pattern match.
/// GAP: other numeric expressions in constructor results remain unsupported.
#[test]
fn zero_and_succ_constructor_result_indices_still_elaborate() {
    let mut env = ElabEnv::new().expect("base env");
    let id = env
        .elaborate_decl(
            "data PrecisionVec (A : Type) : Nat -> Type where { \
             PrecisionNil : PrecisionVec A 0; \
             PrecisionCons : (n : Nat) -> A -> PrecisionVec A n -> \
             PrecisionVec A (n + 1) }",
        )
        .expect("0 and n + 1 result indices must retain their special meaning");
    let family = env.env.inductive(id).expect("PrecisionVec family");
    assert_eq!(family.constructors[0].target_indices.len(), 1);
    assert_eq!(family.constructors[1].target_indices.len(), 1);
}

fn assert_fixed_accepts(target: &str, literal: &str) {
    let mut env = ElabEnv::new().expect("base env");
    let source = format!("const accepted : {target} = {literal}");
    let id = env.elaborate_decl(&source).expect("in-range literal accepts");
    let (_, body) = env.env.transparent_body(id).expect("accepted body");
    let Term::Const { id: literal_id, .. } = body else {
        panic!("fixed-width literal must use its checked literal primitive")
    };
    let expected = literal.parse::<BigInt>().unwrap();
    let Some(NumericLitVal::Int(value)) = env.num_values.get(&literal_id) else {
        panic!("fixed-width literal must retain an integer side-table value")
    };
    assert_eq!(value, &expected);
    assert_eq!(
        ken_interp::EvalVal::from(value.clone()),
        ken_interp::EvalVal::from(expected)
    );
}

fn assert_fixed_rejects(target: &'static str, literal: &str, min: &str, max: &str) {
    let mut env = ElabEnv::new().expect("base env");
    let source = format!("const rejected : {target} = {literal}");
    let literal_start = source.rfind(literal).expect("literal in source");
    let before_decls = env.env.decls().count();
    let before_values = env.num_values.len();
    let err = env
        .elaborate_decl(&source)
        .expect_err("out-of-range literal must reject");
    assert!(matches!(
        err,
        ken_elaborator::ElabError::FixedWidthLiteralOutOfRange {
            literal: found,
            target: found_target,
            min: found_min,
            max: found_max,
            span,
        } if found == literal.parse::<BigInt>().unwrap()
            && found_target == target
            && found_min == min.parse::<BigInt>().unwrap()
            && found_max == max.parse::<BigInt>().unwrap()
            && span.start == literal_start
            && span.end == literal_start + literal.len()
    ));
    assert_eq!(env.env.decls().count(), before_decls);
    assert_eq!(env.num_values.len(), before_values);
}

/// Promise: expected fixed-width literals are admitted only when their exact
/// mathematical value is representable. MEASURED: both sides of Int8, UInt8,
/// and UInt64's upper boundaries plus the original 2^128 probe. CLAIMED: the
/// guard runs before literal primitive creation and reports its owned range.
/// GAP: arithmetic overflow and explicit conversions are separate mechanisms.
#[test]
fn fixed_width_literal_representability_is_decided_before_emission() {
    assert_fixed_accepts("UInt8", "255");
    assert_fixed_rejects("UInt8", "256", "0", "255");
    assert_fixed_accepts("Int8", "127");
    assert_fixed_rejects("Int8", "128", "-128", "127");
    assert_fixed_accepts("UInt64", "18446744073709551615");
    assert_fixed_rejects(
        "UInt64",
        "18446744073709551616",
        "0",
        "18446744073709551615",
    );
    assert_fixed_rejects("UInt8", HUGE, "0", "255");
}
