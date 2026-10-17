//! Checked entrypoint value shapes; the cast below is deliberately evaluated
//! as a value-level C5 probe, not presented to the kernel as a proof between
//! a host value and a source expression.
use super::*;
use ken_interp::eval::eval;
use ken_interp::EvalVal;
use ken_kernel::Term;

fn source(
    elab: &ken_elaborator::ElabEnv,
    store: &mut ken_interp::EvalStore,
    name: &str,
) -> (Term, EvalVal) {
    let (_, term) = elab
        .env
        .transparent_body(elab.globals[name])
        .expect("checked source body");
    let val = eval(&[], &term, &elab.env, store);
    (term, val)
}

fn cast_index(
    elab: &ken_elaborator::ElabEnv,
    store: &mut ken_interp::EvalStore,
    family: &str,
    ctor: &str,
    produced: EvalVal,
    source: Term,
) -> EvalVal {
    let index = Term::var(0);
    let from = Term::app(Term::indformer(elab.globals[family], vec![]), index.clone());
    let to = Term::app(Term::indformer(elab.globals[family], vec![]), source);
    let value = Term::app(Term::constructor(elab.globals[ctor], vec![]), index);
    let cast = Term::Cast(
        Box::new(from.clone()),
        Box::new(to),
        Box::new(Term::Refl(Box::new(from))),
        Box::new(value),
    );
    eval(&[produced], &cast, &elab.env, store)
}

/// Promise class: durable entrypoint C5 value agreement. MEASURED: the actual
/// host-built arguments and environment agree with separately checked source
/// lists, including their Nil, Cons and MkProd type slots; changing content
/// makes the evaluator-level cast stick. CLAIMED: no CLI host constructor
/// silently carries Unknown instead of its checked ProcessInput field type.
/// THE GAP: the test uses the same production value builder and interp eval,
/// but does not assert a kernel proof for a host-only term.
#[test]
fn process_input_lists_and_product_use_checked_field_types() {
    let mut elab = ken_elaborator::ElabEnv::new().expect("prelude");
    for decl in [
        "data CIArgs (xs : List Bytes) : Type where { MkCIArgs : CIArgs xs }",
        "data CIEnv (xs : List (Prod Bytes Bytes)) : Type where { MkCIEnv : CIEnv xs }",
        "const srcArgs : List Bytes = Cons Bytes (bytes_encode \"a\") (Nil Bytes)",
        "const srcOtherArgs : List Bytes = Cons Bytes (bytes_encode \"b\") (Nil Bytes)",
        "const srcEmptyArgs : List Bytes = Nil Bytes",
        "const srcEnv : List (Prod Bytes Bytes) = Cons (Prod Bytes Bytes) (MkProd Bytes Bytes (bytes_encode \"k\") (bytes_encode \"v\")) (Nil (Prod Bytes Bytes))",
        "const srcOtherEnv : List (Prod Bytes Bytes) = Cons (Prod Bytes Bytes) (MkProd Bytes Bytes (bytes_encode \"k\") (bytes_encode \"w\")) (Nil (Prod Bytes Bytes))",
        "const srcEmptyEnv : List (Prod Bytes Bytes) = Nil (Prod Bytes Bytes)",
    ] {
        elab.elaborate_decl(decl).unwrap_or_else(|error| panic!("{decl}: {error:?}"));
    }
    let mut store = build_eval_store(&elab);
    let input = process_input_value(
        &elab,
        &[b"a".to_vec()],
        &[(b"k".to_vec(), b"v".to_vec())],
        b".",
        &mut store,
    );
    let EvalVal::Ctor { args, .. } = input else {
        panic!("expected checked ProcessInput")
    };
    let produced_args = args[0].clone();
    let produced_env = args[1].clone();
    let other_input = process_input_value(
        &elab,
        &[b"b".to_vec()],
        &[(b"k".to_vec(), b"w".to_vec())],
        b".",
        &mut store,
    );
    let EvalVal::Ctor { args: other, .. } = other_input else {
        panic!("expected other ProcessInput")
    };
    for (name, family, ctor, produced, other_produced, good, bad) in [
        (
            "arguments",
            "CIArgs",
            "MkCIArgs",
            produced_args,
            other[0].clone(),
            "srcArgs",
            "srcOtherArgs",
        ),
        (
            "environment",
            "CIEnv",
            "MkCIEnv",
            produced_env,
            other[1].clone(),
            "srcEnv",
            "srcOtherEnv",
        ),
    ] {
        let (term, source_value) = source(&elab, &mut store, good);
        let (_, different) = source(&elab, &mut store, bad);
        assert_eq!(
            produced, source_value,
            "{name}: producer/source slot or payload mismatch"
        );
        assert_eq!(
            other_produced, different,
            "{name}: negative producer/source must agree"
        );
        assert_ne!(
            produced, other_produced,
            "{name}: negative must vary content"
        );
        let result = cast_index(
            &elab,
            &mut store,
            family,
            ctor,
            produced.clone(),
            term.clone(),
        );
        assert!(
            matches!(result, EvalVal::Ctor { id, .. } if id == elab.globals[ctor]),
            "{name}: evaluator cast did not reach its constructor: {result:?}"
        );
        assert_eq!(
            cast_index(&elab, &mut store, family, ctor, other_produced, term),
            EvalVal::Unknown,
            "{name}: different content must stay stuck"
        );
    }

    let empty = process_input_value(&elab, &[], &[], b".", &mut store);
    let EvalVal::Ctor { args, .. } = empty else {
        panic!("expected empty ProcessInput")
    };
    for (produced, good) in [
        (args[0].clone(), "srcEmptyArgs"),
        (args[1].clone(), "srcEmptyEnv"),
    ] {
        let (_, source_value) = source(&elab, &mut store, good);
        assert_eq!(
            produced, source_value,
            "empty List type slot must also agree"
        );
    }
}
