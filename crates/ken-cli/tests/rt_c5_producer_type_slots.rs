//! Promise class: durable C5 regularity for checked, ground source and
//! interpreter-produced List indices (`spec/40-runtime/42-evaluation.md §3.6`).
//! Each cast and its reflexivity witness is independently kernel-checked.
use ken_interp::eval::{eval, EvalVal};
use ken_kernel::term::Term;

#[test]
fn checked_producer_lists_and_source_lists_cast_reflexively() {
    let mut elab = ken_elaborator::ElabEnv::new().expect("prelude");
    for decl in [
        "data CIInt (n : Int) : Type where { MkCIInt : CIInt n }",
        "data CI (xs : List Char) : Type where { MkCI : CI xs }",
        "data CIBytes (xs : List UInt8) : Type where { MkCIBytes : CIBytes xs }",
        "const intIdx : Int = 7",
        "const srcIdx : List Char = Cons Char 97 (Nil Char)",
        "const viewIdx : List Char = string_to_list_char \"a\"",
        "const bytesIdx : List UInt8 = bytes_to_list (bytes_encode \"a\")",
        "const srcBytesIdx : List UInt8 = Cons UInt8 97 (Nil UInt8)",
    ] {
        elab.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("{decl}: {error:?}"));
    }

    let mut store = ken_cli::build_eval_store(&elab);
    let cases = [
        ("int-same", "CIInt", "MkCIInt", "intIdx", "intIdx"),
        ("source-list-same", "CI", "MkCI", "srcIdx", "srcIdx"),
        ("view-list-same", "CI", "MkCI", "viewIdx", "viewIdx"),
        ("view-to-source", "CI", "MkCI", "viewIdx", "srcIdx"),
        (
            "bytes-view-same",
            "CIBytes",
            "MkCIBytes",
            "bytesIdx",
            "bytesIdx",
        ),
    ];
    for (name, family, ctor, left, right) in cases {
        // Refer to checked indices: 42 §3.2 erases subset proofs before
        // value-only evaluation, including when Const unfolds the body.
        let left_body = Term::const_(elab.globals[left], vec![]);
        let right_body = Term::const_(elab.globals[right], vec![]);
        let left_ty = Term::app(
            Term::indformer(elab.globals[family], vec![]),
            left_body.clone(),
        );
        let right_ty = Term::app(
            Term::indformer(elab.globals[family], vec![]),
            right_body.clone(),
        );
        let value = Term::app(
            Term::constructor(elab.globals[ctor], vec![]),
            left_body.clone(),
        );
        let cast = Term::Cast(
            Box::new(left_ty.clone()),
            Box::new(right_ty.clone()),
            Box::new(Term::Refl(Box::new(left_ty.clone()))),
            Box::new(value.clone()),
        );
        ken_kernel::check(&elab.env, &ken_kernel::Context::new(), &cast, &right_ty)
            .unwrap_or_else(|error| panic!("{name} not checked: {error:?}"));
        let left_eval = eval(&[], &left_ty, &elab.env, &mut store);
        let right_eval = eval(&[], &right_ty, &elab.env, &mut store);
        let expected = eval(&[], &value, &elab.env, &mut store);
        let actual = eval(&[], &cast, &elab.env, &mut store);
        assert!(
            matches!(&expected, EvalVal::Ctor { id, .. } if *id == elab.globals[ctor]),
            "{name}: constructor value is not a constructor: {expected:?}"
        );
        assert_eq!(
            actual, expected,
            "{name}: checked cast stuck: source={left_eval:?}, target={right_eval:?}"
        );
    }

    // `bytes_to_list` has no definitional reduction for the kernel to
    // discharge the view-to-source cast with Refl. Compare the values here;
    // an evaluator-level equality row in ken-interp pins the C5 relation.
    let (_, view) = elab.env.transparent_body(elab.globals["bytesIdx"]).unwrap();
    let (_, source) = elab
        .env
        .transparent_body(elab.globals["srcBytesIdx"])
        .unwrap();
    let view = eval(&[], &view, &elab.env, &mut store);
    let source = eval(&[], &source, &elab.env, &mut store);
    assert_eq!(
        view, source,
        "UInt8 view/source values differ: view={view:?} source={source:?}"
    );
}
