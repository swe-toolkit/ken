//! Causal controls for arbitrary-precision surface Decimal coefficients.

use ken_elaborator::{ElabEnv, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;
use num_bigint::BigInt;

const WIDE: &str = "9223372036854775808";

fn eval_view(src: &str) -> EvalVal {
    let mut env = ElabEnv::new().expect("base env");
    let result = env.elaborate_decl_v1(src).expect("declaration elaborates");
    let mut store = EvalStore::new();
    for (id, value) in &env.num_values {
        let evaluated = match value {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Float(f) => EvalVal::Float(*f),
            NumericLitVal::Float32(f) => EvalVal::Float32(*f),
            NumericLitVal::Decimal { coeff, exp } => {
                ken_interp::decimal_value(env.prelude_env.mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        };
        store.num_values.insert(*id, evaluated);
    }
    let Decl::Transparent { body, .. } = env.env.lookup(result.def_id).expect("transparent view")
    else {
        panic!("expected transparent view")
    };
    eval(&[], body, &env.env, &mut store)
}

fn decimal_parts(value: &EvalVal) -> (BigInt, i64) {
    let EvalVal::Ctor { args, .. } = value else {
        panic!("expected DecimalPair constructor, got {value:?}")
    };
    let coeff = match &args[0] {
        EvalVal::BigInt(n) => n.clone(),
        EvalVal::Int(n) => BigInt::from(*n),
        other => panic!("expected integer coefficient, got {other:?}"),
    };
    let exp = match &args[1] {
        EvalVal::Int(n) => *n,
        EvalVal::BigInt(n) => n.to_string().parse().expect("bounded exponent"),
        other => panic!("expected integer exponent, got {other:?}"),
    };
    (coeff, exp)
}

#[test]
fn wide_decimal_reaches_kernel_carrier_and_evaluation_exactly() {
    let mut env = ElabEnv::new().expect("base env");
    let result = env
        .elaborate_decl_v1(&format!("const wide = {WIDE}d"))
        .expect("wide coefficient must elaborate");
    let Decl::Transparent { body, .. } = env.env.lookup(result.def_id).expect("transparent")
    else {
        panic!("expected transparent wide decimal")
    };
    let literal_id = match body {
        ken_kernel::Term::Const { id, .. } => *id,
        other => panic!("expected literal primitive, got {other:?}"),
    };
    assert!(matches!(
        env.num_values.get(&literal_id),
        Some(NumericLitVal::Decimal { coeff, exp })
            if coeff == &WIDE.parse::<BigInt>().unwrap() && *exp == 0
    ));
    let evaluated = eval_view(&format!("const wide = {WIDE}d"));
    let (coeff, exp) = decimal_parts(&evaluated);
    assert_eq!(coeff.to_string(), WIDE);
    assert_eq!(exp, 0);
}

#[test]
fn decimal_eq_decides_wide_exactness_through_target() {
    let result = eval_view(
        "const equal = decimalEq \
         (MkDecimalPair 9223372036854775808 1) \
         (MkDecimalPair 92233720368547758080 0)",
    );
    assert_eq!(result, EvalVal::Bool(true));
}

#[test]
fn narrow_decimal_pairs_and_i32_exponents_remain_exact() {
    for (source, expected_coeff, expected_exp) in [
        ("0.1d", "1", -1),
        ("19.99d", "1999", -2),
        ("3.14d", "314", -2),
    ] {
        let evaluated = eval_view(&format!("const value = {source}"));
        let (coeff, exp) = decimal_parts(&evaluated);
        assert_eq!(coeff.to_string(), expected_coeff, "{source}");
        assert_eq!(exp, expected_exp, "{source}");
    }
    let evaluated = eval_view("const value = 1.000000000000000000000000000000d");
    let (coeff, exp) = decimal_parts(&evaluated);
    assert_eq!(coeff.to_string(), "1000000000000000000000000000000");
    assert_eq!(exp, -30);
}
