//! ADR 0013 Layer 2, spec 16 §2.2: the elaborator-registered kernel
//! `leq_int` and interpreter `leq_int` decide the same arbitrary-precision
//! order. Promise class: durable cross-layer behavioral invariant.

use ken_elaborator::ElabEnv;
use ken_interp::eval::{prim_reduce, EvalVal};
use ken_kernel::env::Context;
use ken_kernel::term::Term;
use ken_kernel::whnf;
use num_bigint::BigInt;

#[test]
fn registered_kernel_and_interpreter_agree_at_signed_and_bignum_boundaries() {
    let elab = ElabEnv::new().expect("register the real Int and Bool primitives");
    let leq = elab.globals["leq_int"];
    let nums = &elab.numeric_env;
    let above_i64: BigInt = "9223372036854775808".parse().unwrap();
    let below_i64: BigInt = "-9223372036854775809".parse().unwrap();
    let huge: BigInt = "170141183460469231731687303715884105728".parse().unwrap();
    let cases = [
        ("0 <= 5", BigInt::from(0), BigInt::from(5), true),
        ("1 > 0", BigInt::from(1), BigInt::from(0), false),
        ("negative order", BigInt::from(-5), BigInt::from(-3), true),
        (
            "negative reverse",
            BigInt::from(-3),
            BigInt::from(-5),
            false,
        ),
        ("equal negative", BigInt::from(-5), BigInt::from(-5), true),
        (
            "beyond i64",
            BigInt::from(i64::MAX),
            above_i64.clone(),
            true,
        ),
        (
            "beyond i64 reverse",
            above_i64.clone(),
            BigInt::from(i64::MAX),
            false,
        ),
        ("below i64", below_i64, BigInt::from(i64::MIN), true),
        ("equal huge", huge.clone(), huge.clone(), true),
        ("huge reverse", huge, above_i64, false),
    ];
    let ctx = Context::new();
    let before = elab.env.trusted_base();
    for (label, m, n, expected) in cases {
        let call = Term::app(
            Term::app(Term::const_(leq, vec![]), Term::IntLit(m.clone())),
            Term::IntLit(n.clone()),
        );
        let kernel = whnf(&elab.env, &ctx, &call);
        let expected_constructor = if expected {
            nums.bool_true_id
        } else {
            nums.bool_false_id
        };
        assert_eq!(
            kernel,
            Term::constructor(expected_constructor, vec![]),
            "kernel {label}: {m} <= {n}"
        );
        assert_eq!(
            prim_reduce(
                "leq_int",
                &[EvalVal::BigInt(m.clone()), EvalVal::BigInt(n.clone())]
            ),
            EvalVal::Bool(expected),
            "interpreter {label}: {m} <= {n}"
        );
    }
    assert_eq!(
        elab.env.trusted_base(),
        before,
        "kernel computation adds no trust"
    );
}
