//! The interpreter and both native evaluators agree at every String ingress.
//! The runtime-IR examples deliberately carry the same code points/bytes as
//! the checked interpreter probes; neither an expected result nor an SSA
//! handle substitutes for an engine observation.

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, prim_reduce_elaborated, EvalStore, EvalVal, ListCharIds};
use ken_kernel::Term;
use ken_runtime::{
    evaluate_runtime_ir_expr, run_example_with_seed_observation, NativeSeedEnvironment,
    RuntimeExample, RuntimeExpr, RuntimeGroundValue, RuntimeIrSeedEnvironment, RuntimeObservation,
    RuntimePartiality, RuntimePrimitive, RuntimeValue,
};

fn empty_native_seed() -> NativeSeedEnvironment {
    NativeSeedEnvironment::empty(ken_runtime::boundary_resource_profile::starter_smoke_profile())
}

fn assert_both_native_engines(
    name: &str,
    expr: RuntimeExpr,
    expected: RuntimeGroundValue,
    native_seed: &NativeSeedEnvironment,
    ir_seed: &RuntimeIrSeedEnvironment,
) {
    let expected = RuntimeObservation::Returned(expected);
    let ir = evaluate_runtime_ir_expr(&expr, ir_seed);
    let example = RuntimeExample {
        name: name.to_string(),
        checked_core_shape: "String ingress witness".to_string(),
        ir: expr,
        observation: expected.clone(),
    };
    let native = run_example_with_seed_observation(&example, native_seed)
        .expect("Cranelift compilation and execution");
    assert!(native.verifier_passed, "Cranelift verifier {name}");
    assert_eq!(native.observation, expected, "Cranelift {name}");
    assert_eq!(
        ir.expect("runtime-IR evaluation"),
        expected,
        "runtime-IR {name}"
    );
}

fn primitive(symbol: &str, args: Vec<RuntimeExpr>, partiality: RuntimePartiality) -> RuntimeExpr {
    RuntimeExpr::PrimitiveCall {
        primitive: RuntimePrimitive {
            symbol: symbol.to_string(),
            partiality,
        },
        args,
    }
}

fn char_list(chars: &[i64]) -> RuntimeExpr {
    chars.iter().rev().fold(
        RuntimeExpr::Value(RuntimeValue::Constructor {
            constructor: "ctor:fixture::List::Nil".to_string(),
            args: vec![],
        }),
        |tail, code| {
            let RuntimeExpr::Value(tail) = tail else {
                unreachable!("constructed list tail remains a value")
            };
            RuntimeExpr::Value(RuntimeValue::Constructor {
                constructor: "ctor:fixture::List::Cons".to_string(),
                args: vec![RuntimeValue::Int((*code).into()), tail],
            })
        },
    )
}

fn interpreter_chars(chars: &[i64]) -> EvalVal {
    let mut env = ElabEnv::new().expect("interpreter prelude");
    let mut list = "(Nil Char)".to_string();
    for code in chars.iter().rev() {
        list = format!("(Cons Char ({code} : Int) {list})");
    }
    let id = env
        .elaborate_decl(&format!(
            "const rt_string_ingress : String = list_char_to_string {list}"
        ))
        .expect("the actual List Char expression checks");
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    eval(&[], &Term::const_(id, vec![]), &env.env, &mut store)
}

fn interpreter_bytes(bytes: &[u8]) -> EvalVal {
    let env = ElabEnv::new().expect("interpreter prelude");
    let mut store = EvalStore::new();
    let decoded = prim_reduce_elaborated(
        "bytes_decode",
        &[EvalVal::Bytes(bytes.to_vec())],
        &env,
        &mut store,
    );
    let EvalVal::Ctor { id, args, .. } = decoded else {
        panic!("interpreter bytes_decode did not return a Result constructor");
    };
    assert_eq!(id, env.globals["Ok"], "interpreter decode must return Ok");
    args.last().cloned().expect("Ok must carry its String")
}

fn assert_char_list_row(name: &str, chars: &[i64], expected: &str) {
    assert_eq!(
        interpreter_chars(chars),
        EvalVal::Str(expected.into()),
        "interpreter {name}"
    );
    assert_both_native_engines(
        name,
        primitive(
            "list_char_to_string",
            vec![char_list(chars)],
            RuntimePartiality::Total,
        ),
        RuntimeGroundValue::String(expected.into()),
        &empty_native_seed(),
        &RuntimeIrSeedEnvironment::empty(),
    );
}

#[test]
fn native_string_two_chars_are_not_two_utf8_bytes() {
    assert_char_list_row("byte-sequence-is-not-scalar-sequence", &[195, 169], "Ã©");
}

#[test]
fn native_string_single_non_ascii_scalar_is_not_a_utf8_byte() {
    assert_char_list_row("single-non-ascii-scalar", &[233], "é");
}

#[test]
fn native_string_scalar_above_u8_is_not_refused() {
    assert_char_list_row("scalar-outside-u8", &[0x100], "Ā");
}

#[test]
fn native_string_decomposed_char_list_normalizes() {
    assert_char_list_row("decomposed-two-scalars", &[0x65, 0x301], "é");
    // A later boundary serializer can normalize the returned text. Read its
    // byte length inside each evaluator, before any outbound observation.
    let string = primitive(
        "list_char_to_string",
        vec![char_list(&[0x65, 0x301])],
        RuntimePartiality::Total,
    );
    assert_both_native_engines(
        "decomposed-two-scalars-before-export",
        primitive("byte_length", vec![string], RuntimePartiality::Total),
        RuntimeGroundValue::Int(2.into()),
        &empty_native_seed(),
        &RuntimeIrSeedEnvironment::empty(),
    );
}

fn decode_result_partiality() -> RuntimePartiality {
    RuntimePartiality::SafeResult {
        err: "ctor:fixture::Result::Err".to_string(),
        ok: "ctor:fixture::Result::Ok".to_string(),
        error: "ctor:fixture::Utf8Error::InvalidUtf8".to_string(),
    }
}

fn ok_string(value: &str) -> RuntimeGroundValue {
    RuntimeGroundValue::Constructor {
        constructor: "ctor:fixture::Result::Ok".to_string(),
        args: vec![RuntimeGroundValue::String(value.into())],
    }
}

#[test]
fn native_string_bytes_decode_normalizes_like_the_interpreter() {
    let bytes = vec![0x65, 0xcc, 0x81];
    assert_eq!(interpreter_bytes(&bytes), EvalVal::Str("é".into()));
    assert_both_native_engines(
        "decomposed-utf8-bytes",
        primitive(
            "bytes_decode",
            vec![RuntimeExpr::Value(RuntimeValue::Bytes(bytes))],
            decode_result_partiality(),
        ),
        ok_string("é"),
        &empty_native_seed(),
        &RuntimeIrSeedEnvironment::empty(),
    );
}

#[test]
fn native_string_literal_ingress_normalizes() {
    let decomposed = "e\u{301}";
    assert_both_native_engines(
        "literal-string-ingress",
        primitive(
            "byte_length",
            vec![RuntimeExpr::Value(RuntimeValue::String(decomposed.into()))],
            RuntimePartiality::Total,
        ),
        RuntimeGroundValue::Int(2.into()),
        &empty_native_seed(),
        &RuntimeIrSeedEnvironment::empty(),
    );
}

#[test]
fn native_string_ground_ingress_normalizes() {
    let decomposed = "e\u{301}";
    let symbol = "decl:fixture::Local::string_seed";
    let mut ir_seed = RuntimeIrSeedEnvironment::empty();
    ir_seed.insert(symbol, RuntimeGroundValue::String(decomposed.into()));
    let expr = primitive(
        "byte_length",
        vec![RuntimeExpr::Call {
            callee: Box::new(RuntimeExpr::Closure {
                captures: vec![symbol.into()],
                params: vec![],
                body: Box::new(RuntimeExpr::Var(0)),
            }),
            args: vec![],
        }],
        RuntimePartiality::Total,
    );
    // The Cranelift ground ingress is pinned before serialization in
    // ground_string_ingress_normalizes_before_boundary_observation: the
    // closed carried-word path cannot specialize byte_length after a call.
    assert_eq!(
        evaluate_runtime_ir_expr(&expr, &ir_seed).expect("runtime-IR capture"),
        RuntimeObservation::Returned(RuntimeGroundValue::Int(2.into())),
    );
}

#[test]
fn native_string_bytes_f3_decode_encode_holds_in_both_native_engines() {
    // F3 is quantified over already-canonical Ken Strings. This test extends
    // the interpreter F3 check in bytes_primitive_contracts with both native
    // engines, including non-ASCII and a value built by the scalar ingress.
    for (name, expr, expected) in [
        (
            "ascii",
            RuntimeExpr::Value(RuntimeValue::String("Q7?".into())),
            "Q7?",
        ),
        (
            "non-ascii",
            RuntimeExpr::Value(RuntimeValue::String("é".into())),
            "é",
        ),
        (
            "scalar-built",
            primitive(
                "list_char_to_string",
                vec![char_list(&[0x65, 0x301])],
                RuntimePartiality::Total,
            ),
            "é",
        ),
    ] {
        let encoded = primitive("bytes_encode", vec![expr], RuntimePartiality::Total);
        assert_both_native_engines(
            name,
            primitive("bytes_decode", vec![encoded], decode_result_partiality()),
            ok_string(expected),
            &empty_native_seed(),
            &RuntimeIrSeedEnvironment::empty(),
        );
    }
}
