//! K3: a fresh literal closes a generic checked ASCII-code witness.
//! The finite table is checked Ken source; its bounds are 0..127.

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{check, convert, normalize, whnf, Context, KernelError, Term};

fn checked_ascii_client(env: &mut ElabEnv) -> Result<(), (String, ken_elaborator::ElabError)> {
    let tags = (0..=127)
        .map(|n| format!("Ascii{n}"))
        .collect::<Vec<_>>()
        .join("; ");
    let tag_decl = format!("data AsciiTag : Type where {{ {tags} }}");
    env.elaborate_decl(&tag_decl)
        .map_err(|e| (tag_decl.clone(), e))?;
    let arms = (0..=127)
        .map(|n| format!("Ascii{n} |-> {n}"))
        .collect::<Vec<_>>()
        .join("; ");
    let decode = format!("fn ascii_value (tag : AsciiTag) : Int = match tag {{ {arms} }}");
    env.elaborate_decl(&decode)
        .map_err(|e| (decode.clone(), e))?;
    let rest = [
        "data AsciiCode (n : Int) : Type where { MkAsciiCode : (tag : AsciiTag) -> Equal Int n (ascii_value tag) -> AsciiCode n }",
        "data AllAsciiCodes : List Int -> Type where { NoCodes : AllAsciiCodes (Nil Int); SomeCodes : (n : Int) -> (tail : List Int) -> AsciiCode n -> AllAsciiCodes tail -> AllAsciiCodes (Cons Int n tail) }",
        "const fresh_k3 : String = \"Az\"",
        "const fresh_k3_ascii : AllAsciiCodes (map Char Int charToInt (string_to_list_char fresh_k3)) = SomeCodes 65 (Cons Int 122 (Nil Int)) (MkAsciiCode 65 Ascii65 Proved) (SomeCodes 122 (Nil Int) (MkAsciiCode 122 Ascii122 Proved) NoCodes)",
    ];
    for src in rest {
        env.elaborate_decl(src).map_err(|e| (src.into(), e))?;
    }
    Ok(())
}

#[test]
fn fresh_ascii_literal_closes_generic_checked_finite_code_witness() {
    let mut env = ElabEnv::new().expect("prelude");
    if let Err((src, error)) = checked_ascii_client(&mut env) {
        panic!("checked ASCII client failed at {src}: {error:?}");
    }
    // Independent concrete oracle: no constructor code is inferred from the
    // reducer being tested. Promise: durable semantic invariant on a fresh
    // source literal, not a snapshot of a generated declaration count.
    let body = env.env.transparent_body(env.globals["fresh_k3"]).unwrap().1;
    assert_eq!(scalars(&env, &body), vec![65, 122]);
    let checked_long = format!("const checked_long_view : String = \"{}\"", "x".repeat(64));
    env.elaborate_decl(&checked_long).unwrap();
    let checked_body = env
        .env
        .transparent_body(env.globals["checked_long_view"])
        .unwrap()
        .1;
    assert_eq!(scalars(&env, &checked_body), vec![120; 64]);

    // Stress the new reducer's finite construction separately from the
    // pre-existing general checker recursion limit on deeply nested Lists.
    let long = format!("const long_view : String = \"{}\"", "x".repeat(256));
    env.elaborate_decl(&long).unwrap();
    let body = env
        .env
        .transparent_body(env.globals["long_view"])
        .unwrap()
        .1;
    let reduced = whnf(&env.env, &Context::new(), &string_view(&env, body));
    assert_eq!(list_int_heads(&env, reduced), Some(vec![120; 256]));
}

fn string_view(env: &ElabEnv, literal: Term) -> Term {
    Term::app(
        Term::const_(env.prelude_env.string_to_list_char_id, vec![]),
        literal,
    )
}

fn list_int_heads(env: &ElabEnv, mut view: Term) -> Option<Vec<u32>> {
    let mut codes = Vec::new();
    loop {
        let (head, args) = ken_kernel::inductive::peel_app(&view);
        if matches!(head, Term::Constructor { id, .. } if id == env.prelude_env.nil_id)
            && args.len() == 1
        {
            return Some(codes);
        }
        if !matches!(head, Term::Constructor { id, .. } if id == env.prelude_env.cons_id)
            || args.len() != 3
        {
            return None;
        }
        let Term::IntLit(code) = &args[1] else {
            return None;
        };
        codes.push(code.to_string().parse::<u32>().ok()?);
        view = args[2].clone();
    }
}

fn scalars(env: &ElabEnv, literal: &Term) -> Vec<u32> {
    let next_id = env.env.next_global_id();
    let reduced = normalize(
        &env.env,
        &Context::new(),
        &string_view(env, literal.clone()),
    );
    assert_eq!(
        env.env.next_global_id(),
        next_id,
        "reduction minted a global"
    );
    let char_ty = Term::const_(env.globals["Char"], vec![]);
    let list_char = Term::app(Term::indformer(env.prelude_env.list_id, vec![]), char_ty);
    check(&env.env, &Context::new(), &reduced, &list_char)
        .expect("the produced List Char must independently kernel-typecheck");
    list_int_heads(env, reduced)
        .expect("literal view must be a typed Nil/Cons spine of IntLit scalars")
}

#[test]
fn neutral_bound_string_and_char_do_not_compute_or_fabricate_a_witness() {
    let mut env = ElabEnv::new().unwrap();
    let ctx = Context::new();
    let string_var = string_view(&env, Term::var(0));
    assert_eq!(whnf(&env.env, &ctx, &string_var), string_var);
    let char_to_int = Term::app(Term::const_(env.globals["charToInt"], vec![]), Term::var(0));
    assert_eq!(whnf(&env.env, &ctx, &char_to_int), Term::var(0));
    checked_ascii_client(&mut env).unwrap();
    let cases = [
        "fn no_ascii_from_neutral_string (s : String) : AllAsciiCodes (map Char Int charToInt (string_to_list_char s)) = fresh_k3_ascii",
        "fn no_ascii_from_neutral_char (c : Char) : AsciiCode (charToInt c) = MkAsciiCode 65 Ascii65 Proved",
    ];
    for source in cases {
        let failure = env
            .elaborate_decl(source)
            .expect_err("neutral argument must not acquire a finite witness");
        assert!(
            matches!(
                failure,
                ken_elaborator::ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "neutral boundary rejected for another reason: {failure:?}"
        );
    }
}

#[test]
fn checked_unicode_scalar_nfc_and_non_ascii_witness_refusal() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl("const composed : String = \"é🙂\"")
        .unwrap();
    env.elaborate_decl("const decomposed : String = \"e\u{301}🙂\"")
        .unwrap();
    let composed = env
        .env
        .transparent_body(env.globals["composed"])
        .unwrap()
        .1
        .clone();
    let decomposed = env
        .env
        .transparent_body(env.globals["decomposed"])
        .unwrap()
        .1
        .clone();
    assert_eq!(scalars(&env, &composed), vec![233, 128578]);
    assert_eq!(scalars(&env, &decomposed), vec![233, 128578]);
    env.elaborate_decl("const empty_view : String = \"\"")
        .unwrap();
    let empty = env
        .env
        .transparent_body(env.globals["empty_view"])
        .unwrap()
        .1
        .clone();
    assert_eq!(scalars(&env, &empty), Vec::<u32>::new());
    checked_ascii_client(&mut env).unwrap();
    let failure = env.elaborate_decl("const not_ascii : AllAsciiCodes (map Char Int charToInt (string_to_list_char composed)) = SomeCodes 233 (Cons Int 128578 (Nil Int)) (MkAsciiCode 233 Ascii65 Proved) (SomeCodes 128578 (Nil Int) (MkAsciiCode 128578 Ascii122 Proved) NoCodes)")
        .expect_err("non-ASCII scalar cannot equal an ASCII decoder result");
    assert!(
        matches!(
            failure,
            ken_elaborator::ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "unexpected non-ASCII refusal: {failure:?}"
    );
    let before = env.env.next_global_id();
    assert!(ken_kernel::check::checked_char_literal(&env.env, 0xD800).is_err());
    assert!(ken_kernel::check::checked_char_literal(&env.env, 0x110000).is_err());
    assert_eq!(
        env.env.next_global_id(),
        before,
        "invalid scalar validation allocated globals"
    );
}

#[test]
fn occurrences_convert_from_values_not_literal_ids_and_other_ops_remain_neutral() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl("const first : String = \"--\"").unwrap();
    env.elaborate_decl("const second : String = \"--\"")
        .unwrap();
    env.elaborate_decl("const different : String = \"-.\"")
        .unwrap();
    let get = |name: &str| {
        env.env
            .transparent_body(env.globals[name])
            .unwrap()
            .1
            .clone()
    };
    let a = get("first");
    let b = get("second");
    let c = get("different");
    assert_ne!(
        a, b,
        "independent literals have distinct source occurrences"
    );
    let ty = Term::app(
        Term::indformer(env.prelude_env.list_id, vec![]),
        Term::const_(env.globals["Char"], vec![]),
    );
    let va = string_view(&env, a);
    let vb = string_view(&env, b);
    let vc = string_view(&env, c);
    assert!(convert(&env.env, &Context::new(), &ty, &va, &vb));
    assert!(!convert(&env.env, &Context::new(), &ty, &va, &vc));
    let first_head = |view: &Term| {
        let reduced = whnf(&env.env, &Context::new(), view);
        ken_kernel::inductive::peel_app(&reduced).1[1].clone()
    };
    let ha = first_head(&va);
    let hb = first_head(&vb);
    let hc = first_head(&vc);
    let char_ty = Term::const_(env.globals["Char"], vec![]);
    assert!(convert(&env.env, &Context::new(), &char_ty, &ha, &hb));
    assert!(
        convert(&env.env, &Context::new(), &char_ty, &ha, &hc),
        "first scalar is shared across -- and -. even though whole views differ"
    );
    let second_head = |view: &Term| {
        let reduced = whnf(&env.env, &Context::new(), view);
        let tail = ken_kernel::inductive::peel_app(&reduced).1[2].clone();
        first_head(&tail)
    };
    assert!(convert(
        &env.env,
        &Context::new(),
        &char_ty,
        &second_head(&va),
        &second_head(&vb)
    ));
    assert!(!convert(
        &env.env,
        &Context::new(),
        &char_ty,
        &second_head(&va),
        &second_head(&vc)
    ));
    assert_eq!(
        list_int_heads(&env, normalize(&env.env, &Context::new(), &va)),
        Some(vec![45, 45])
    );
    // No implicit whole-String equality or unrelated primitive evaluation.
    assert_eq!(whnf(&env.env, &Context::new(), &get("first")), get("first"));
    let string_ty = Term::const_(env.globals["String"], vec![]);
    let whole_string_eq = Term::Eq(
        Box::new(string_ty),
        Box::new(get("first")),
        Box::new(get("second")),
    );
    assert!(matches!(
        whnf(&env.env, &Context::new(), &whole_string_eq),
        Term::Eq(..)
    ));
    let eq_int = Term::app(
        Term::app(
            Term::const_(env.globals["eq_int"], vec![]),
            Term::IntLit(45u32.into()),
        ),
        Term::IntLit(45u32.into()),
    );
    assert_eq!(whnf(&env.env, &Context::new(), &eq_int), eq_int);
    let length = env.globals["char_length"];
    let app = Term::app(Term::const_(length, vec![]), get("first"));
    assert_eq!(whnf(&env.env, &Context::new(), &app), app);
    let encode = Term::app(
        Term::const_(env.globals["bytes_encode"], vec![]),
        get("first"),
    );
    assert_eq!(whnf(&env.env, &Context::new(), &encode), encode);

    // Even a well-typed primitive Literal with an elaborator-side value is
    // neutral unless the kernel admitted that exact payload itself.
    let ghost = ken_kernel::declare_primitive(
        &mut env.env,
        vec![],
        Term::const_(env.globals["String"], vec![]),
        ken_kernel::PrimReduction::Literal,
    )
    .unwrap();
    env.num_values.insert(
        ghost,
        ken_elaborator::NumericLitVal::Str(ken_elaborator::NfcString::new("FORGED")),
    );
    let ghost_view = string_view(&env, Term::const_(ghost, vec![]));
    assert_eq!(whnf(&env.env, &Context::new(), &ghost_view), ghost_view);
    let before = env.env.next_global_id();
    assert!(
        ken_kernel::check::register_literal_char_view(
            &mut env.env,
            env.globals["String"],
            env.globals["Char"],
            env.prelude_env.string_to_list_char_id,
            env.prelude_env.list_id,
            env.prelude_env.nil_id,
            env.prelude_env.cons_id,
        )
        .is_err(),
        "registration must not replace the audited operation"
    );
    assert_eq!(env.env.next_global_id(), before);
}

#[test]
fn checked_char_literal_and_view_head_reduce_via_derived_char_to_int() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl("const direct_c : Char = 'β'").unwrap();
    env.elaborate_decl("const via_view : String = \"β\"")
        .unwrap();
    let direct = env
        .env
        .transparent_body(env.globals["direct_c"])
        .unwrap()
        .1
        .clone();
    let from_string = env
        .env
        .transparent_body(env.globals["via_view"])
        .unwrap()
        .1
        .clone();
    assert_eq!(
        direct,
        Term::IntLit(946u32.into()),
        "a checked Char literal is directly a type-preserving core IntLit"
    );
    let direct_code = Term::app(Term::const_(env.globals["charToInt"], vec![]), direct);
    assert_eq!(
        whnf(&env.env, &Context::new(), &direct_code),
        Term::IntLit(946u32.into())
    );
    let view = normalize(&env.env, &Context::new(), &string_view(&env, from_string));
    let (_, parts) = ken_kernel::inductive::peel_app(&view);
    let from_head = Term::app(
        Term::const_(env.globals["charToInt"], vec![]),
        parts[1].clone(),
    );
    assert_eq!(
        whnf(&env.env, &Context::new(), &from_head),
        Term::IntLit(946u32.into())
    );
}

#[test]
fn interpreter_view_and_codes_use_the_kernel_checked_payload() {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_decl("const parity : String = \"Az🙂\"")
        .unwrap();
    let literal = env
        .env
        .transparent_body(env.globals["parity"])
        .unwrap()
        .1
        .clone();
    let expected = scalars(&env, &literal);
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    let Term::Const { id, .. } = literal else {
        panic!("expected checked literal")
    };
    // An independent, corrupted evaluator-side cache must not change either
    // the view or the scalar: the immutable kernel payload wins.
    store
        .num_values
        .insert(id, EvalVal::Str(ken_elaborator::NfcString::new("WRONG")));
    let observed = eval(
        &[],
        &string_view(&env, Term::const_(id, vec![])),
        &env.env,
        &mut store,
    );
    let mut codes = Vec::new();
    let mut cursor = &observed;
    loop {
        match cursor {
            EvalVal::Ctor { id: ctor, args, .. } if *ctor == env.prelude_env.cons_id => {
                assert_eq!(args.len(), 3);
                let EvalVal::Int(code) = &args[1] else {
                    panic!("expected Char code: {:?}", args[1])
                };
                codes.push(*code as u32);
                cursor = &args[2];
            }
            EvalVal::Ctor { id: ctor, .. } if *ctor == env.prelude_env.nil_id => break,
            other => panic!("expected native List Char ctor, got {other:?}"),
        }
    }
    assert_eq!(expected, vec![65, 122, 128578]);
    assert_eq!(codes, expected);
}
