//! KERNEL-INT-DIV-MOD-NATIVE, independent Ops/evaluation slice.
//! Promise class: durable runtime and kernel-neutrality invariants (18a §5.2).
//! Raw `/` and `%` obligations are a separately gated surface deliverable.

use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, prim_reduce, EvalStore, EvalVal};
use ken_kernel::check::infer;
use ken_kernel::env::{Context, Decl, PrimReduction};
use ken_kernel::term::Term;
use ken_kernel::whnf;
use num_bigint::BigInt;

fn call(id: ken_kernel::GlobalId, a: BigInt, b: BigInt) -> Term {
    Term::app(
        Term::app(Term::const_(id, vec![]), Term::IntLit(a)),
        Term::IntLit(b),
    )
}

fn integer(value: EvalVal) -> BigInt {
    match value {
        EvalVal::Int(n) => n.into(),
        EvalVal::BigInt(n) => n,
        other => panic!("expected an Int value, got {other:?}"),
    }
}

fn absolute(value: &BigInt) -> BigInt {
    if value < &BigInt::from(0) {
        -value
    } else {
        value.clone()
    }
}

#[test]
fn registered_div_mod_are_binary_int_ops_and_kernel_neutral() {
    let elab = ElabEnv::new().expect("numeric prelude");
    let int = Term::const_(elab.globals["Int"], vec![]);
    let expected_ty = Term::pi(int.clone(), Term::pi(int.clone(), int.clone()));
    let trusted: BTreeSet<_> = elab.env.trusted_base().into_iter().collect();
    let context = Context::new();
    for symbol in ["div_int", "mod_int"] {
        let id = elab.globals[symbol];
        assert!(trusted.contains(&id), "registered Op {symbol} is trusted");
        assert!(
            elab.env.declarations().iter().any(|decl| matches!(
                decl,
                Decl::Primitive {
                    id: actual,
                    ty,
                    reduction: PrimReduction::Op { symbol: actual_symbol },
                    ..
                } if *actual == id && ty == &expected_ty && *actual_symbol == symbol
            )),
            "{symbol} has exactly the registered Int → Int → Int shape"
        );
        let applied = call(id, 7.into(), 2.into());
        assert_eq!(infer(&elab.env, &context, &applied).unwrap(), int);
        assert_eq!(
            whnf(&elab.env, &context, &applied),
            applied,
            "{symbol} must not execute in kernel conversion even on literals"
        );
    }
    assert_eq!(
        elab.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trusted,
        "neither neutral conversion nor inference adds trust"
    );
}

#[test]
fn truncated_division_remainder_and_independent_identity_hold_across_signs_and_i128() {
    for (a, b, q, r) in [
        (7, 2, 3, 1),
        (-7, 2, -3, -1),
        (7, -2, -3, 1),
        (-7, -2, 3, -1),
        (7, 3, 2, 1),
        (-7, 3, -2, -1),
        (0, 3, 0, 0),
        (7, 9, 0, 7),
    ] {
        assert_eq!(
            integer(prim_reduce("div_int", &[EvalVal::Int(a), EvalVal::Int(b)])),
            BigInt::from(q),
            "truncated quotient for {a}/{b}"
        );
        assert_eq!(
            integer(prim_reduce("mod_int", &[EvalVal::Int(a), EvalVal::Int(b)])),
            BigInt::from(r),
            "truncated remainder for {a}%{b}"
        );
    }

    // The identity oracle uses only multiplication/addition on the observed
    // quotient and remainder, never the interpreter's division/remainder path.
    // Both operands exceed 2^127 in magnitude and cover every sign pairing.
    let magnitude: BigInt = (BigInt::from(1) << 132u32) + BigInt::from(17);
    let divisor: BigInt = (BigInt::from(1) << 128u32) + BigInt::from(3);
    for a in [magnitude.clone(), -magnitude] {
        for b in [divisor.clone(), -divisor.clone()] {
            let q = integer(prim_reduce(
                "div_int",
                &[EvalVal::BigInt(a.clone()), EvalVal::BigInt(b.clone())],
            ));
            let r = integer(prim_reduce(
                "mod_int",
                &[EvalVal::BigInt(a.clone()), EvalVal::BigInt(b.clone())],
            ));
            assert_eq!(&q * &b + &r, a, "a = (a div b)*b + (a mod b)");
            assert!(
                absolute(&r) < absolute(&b),
                "remainder magnitude must be bounded"
            );
            if r != BigInt::from(0) {
                assert_eq!(r.sign(), a.sign(), "remainder follows dividend sign");
            }
        }
    }
}

fn assert_zero_fault(symbol: &'static str, a: EvalVal, zero: EvalVal) {
    let result = catch_unwind(|| prim_reduce(symbol, &[a, zero]));
    let panic = result.expect_err("a zero divisor must fault, never return a value");
    let message = panic
        .downcast_ref::<String>()
        .expect("explicit zero-divisor panic");
    assert_eq!(message, &format!("{symbol} has zero divisor"));
}

#[test]
fn zero_divisors_fault_in_both_primitive_and_registered_core_paths() {
    let elab = ElabEnv::new().expect("numeric prelude");
    let mut store = EvalStore::new();
    for symbol in ["div_int", "mod_int"] {
        assert_zero_fault(symbol, EvalVal::Int(7), EvalVal::Int(0));
        assert_zero_fault(
            symbol,
            EvalVal::BigInt(BigInt::from(1) << 128),
            EvalVal::BigInt(BigInt::from(0)),
        );
        let term = call(elab.globals[symbol], 7.into(), 0.into());
        let outcome = catch_unwind(AssertUnwindSafe(|| eval(&[], &term, &elab.env, &mut store)));
        let panic = outcome.expect_err("a saturated core Op on zero must fault");
        assert_eq!(
            panic.downcast_ref::<String>().map(String::as_str),
            Some(format!("{symbol} has zero divisor").as_str())
        );
    }
}

#[test]
fn nonzero_core_calls_evaluate_and_wrong_operand_shapes_do_not_fabricate_ints() {
    let elab = ElabEnv::new().expect("numeric prelude");
    let mut store = EvalStore::new();
    for (symbol, a, b, expected) in [("div_int", -7, 2, -3), ("mod_int", -7, 3, -1)] {
        let term = call(elab.globals[symbol], a.into(), b.into());
        assert_eq!(
            integer(eval(&[], &term, &elab.env, &mut store)),
            BigInt::from(expected)
        );
        assert_eq!(
            prim_reduce(symbol, &[EvalVal::Bool(true), EvalVal::Int(2)]),
            EvalVal::Neutral,
            "wrongly typed raw primitive operands stay stuck"
        );
    }
}
