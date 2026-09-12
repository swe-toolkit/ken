//! `LANG-MATCH-LITERAL-PATTERN`: value-level literal pattern selection.
//!
//! Promise class: durable invariants. The positive/negative pairs drive every
//! admitted carrier through its real comparator plan; the rejection and
//! coverage controls pin the fail-closed and open-column boundaries.

use ken_elaborator::{error::ElabError, ArmDeadCause, ElabEnv, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId, Term};

fn elaborate(env: &mut ElabEnv, source: &str) -> GlobalId {
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("elaboration failed for `{source}`: {error}"))
}

fn body(env: &ElabEnv, id: GlobalId) -> &Term {
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("expected transparent declaration, got {other:?}"),
    }
}

fn store_for(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    for (id, value) in &env.num_values {
        let value = match value {
            NumericLitVal::Int(value) => EvalVal::from(value.clone()),
            NumericLitVal::Float(value) => EvalVal::Float(*value),
            NumericLitVal::Float32(value) => EvalVal::Float32(*value),
            NumericLitVal::Decimal { coeff, exp } => {
                ken_interp::decimal_value(env.prelude_env.mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(value) => EvalVal::Str(value.clone()),
            NumericLitVal::Bytes(value) => EvalVal::Bytes(value.clone()),
        };
        store.num_values.insert(*id, value);
    }
    store
}

fn eval_value(env: &ElabEnv, id: GlobalId) -> EvalVal {
    eval(&[], body(env, id), &env.env, &mut store_for(env))
}

fn eval_nat(env: &ElabEnv, id: GlobalId) -> usize {
    fn count(value: EvalVal, zero: GlobalId, suc: GlobalId) -> usize {
        match value {
            EvalVal::Ctor { id, args, .. } if id == zero && args.is_empty() => 0,
            EvalVal::Ctor { id, args, .. } if id == suc && args.len() == 1 => {
                1 + count(args[0].clone(), zero, suc)
            }
            other => panic!("expected Nat result, got {other:?}"),
        }
    }

    count(eval_value(env, id), env.globals["Zero"], env.globals["Suc"])
}

fn assert_selects(env: &mut ElabEnv, function: &str, matching: &str, nonmatching: &str) {
    elaborate(env, function);
    let selected = elaborate(env, matching);
    let fallback = elaborate(env, nonmatching);
    assert_eq!(eval_nat(env, selected), 1, "matching source: {matching}");
    assert_eq!(eval_nat(env, fallback), 0, "fallback source: {nonmatching}");
}

fn contains_proof_equality(term: &Term) -> bool {
    matches!(
        term,
        Term::Eq(_, _, _) | Term::J(_, _, _) | Term::Cast(_, _, _, _) | Term::Refl(_)
    ) || term.children().into_iter().any(contains_proof_equality)
}

#[test]
fn every_supported_carrier_selects_and_falls_through_by_value() {
    // MEASURED: each row returns distinct Nat payloads for equal and unequal
    // values through an elaborated function and the real interpreter.
    // CLAIMED: every admitted carrier selects through its normative value plan.
    // THE GAP: the all-row differential distinguishes a registered spelling
    // from a reaching comparator and independently exercises every table arm.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();

    assert_selects(
        &mut env,
        "fn lit_int (x : Int) : Nat = match x { 100000000000000000001 |-> Suc Zero ; _ |-> Zero }",
        "const lit_int_yes : Nat = lit_int 100000000000000000001",
        "const lit_int_no : Nat = lit_int 100000000000000000002",
    );
    assert_selects(
        &mut env,
        "fn lit_i8 (x : Int8) : Nat = match x { 127 |-> Suc Zero ; _ |-> Zero }",
        "const lit_i8_yes : Nat = lit_i8 127",
        "const lit_i8_no : Nat = lit_i8 (int_to_int8_raw (sub_int 0 128))",
    );
    assert_selects(
        &mut env,
        "fn lit_i16 (x : Int16) : Nat = match x { 32767 |-> Suc Zero ; _ |-> Zero }",
        "const lit_i16_yes : Nat = lit_i16 32767",
        "const lit_i16_no : Nat = lit_i16 (int_to_int16_raw (sub_int 0 32768))",
    );
    assert_selects(
        &mut env,
        "fn lit_i32 (x : Int32) : Nat = match x { 2147483647 |-> Suc Zero ; _ |-> Zero }",
        "const lit_i32_yes : Nat = lit_i32 2147483647",
        "const lit_i32_no : Nat = lit_i32 (int_to_int32_raw (sub_int 0 2147483648))",
    );
    assert_selects(
        &mut env,
        "fn lit_i64 (x : Int64) : Nat = match x { 9223372036854775807 |-> Suc Zero ; _ |-> Zero }",
        "const lit_i64_yes : Nat = lit_i64 9223372036854775807",
        "const lit_i64_no : Nat = lit_i64 (int_to_int64_raw (sub_int 0 9223372036854775808))",
    );
    assert_selects(
        &mut env,
        "fn lit_u8 (x : UInt8) : Nat = match x { 255 |-> Suc Zero ; _ |-> Zero }",
        "const lit_u8_yes : Nat = lit_u8 255",
        "const lit_u8_no : Nat = lit_u8 0",
    );
    assert_selects(
        &mut env,
        "fn lit_u16 (x : UInt16) : Nat = match x { 65535 |-> Suc Zero ; _ |-> Zero }",
        "const lit_u16_yes : Nat = lit_u16 65535",
        "const lit_u16_no : Nat = lit_u16 0",
    );
    assert_selects(
        &mut env,
        "fn lit_u32 (x : UInt32) : Nat = match x { 4294967295 |-> Suc Zero ; _ |-> Zero }",
        "const lit_u32_yes : Nat = lit_u32 4294967295",
        "const lit_u32_no : Nat = lit_u32 0",
    );
    assert_selects(
        &mut env,
        "fn lit_u64 (x : UInt64) : Nat = match x { 18446744073709551615 |-> Suc Zero ; _ |-> Zero }",
        "const lit_u64_yes : Nat = lit_u64 18446744073709551615",
        "const lit_u64_no : Nat = lit_u64 0",
    );
    assert_selects(
        &mut env,
        "fn lit_float (x : Float) : Nat = match x { 0.0 |-> Suc Zero ; _ |-> Zero }",
        "const lit_float_yes : Nat = lit_float (div_float 0.0 (sub_float 0.0 1.0))",
        "const lit_float_no : Nat = lit_float (div_float 0.0 0.0)",
    );
    assert_selects(
        &mut env,
        "fn lit_float32 (x : Float32) : Nat = match x { 1.5f32 |-> Suc Zero ; _ |-> Zero }",
        "const lit_float32_yes : Nat = lit_float32 1.5f32",
        "const lit_float32_no : Nat = lit_float32 2.5f32",
    );
    assert_selects(
        &mut env,
        "fn lit_char (x : Char) : Nat = match x { 'λ' |-> Suc Zero ; _ |-> Zero }",
        "const lit_char_yes : Nat = lit_char 'λ'",
        "const lit_char_no : Nat = lit_char 'x'",
    );
    assert_selects(
        &mut env,
        "fn lit_string (x : String) : Nat = match x { \"é\" |-> Suc Zero ; _ |-> Zero }",
        r#"const lit_string_yes : Nat = lit_string "e\u{301}""#,
        "const lit_string_no : Nat = lit_string \"éx\"",
    );
    let string_same_length_miss = elaborate(
        &mut env,
        "const lit_string_same_length_no : Nat = lit_string \"x\"",
    );
    assert_eq!(eval_nat(&env, string_same_length_miss), 0);
    assert_selects(
        &mut env,
        r#"fn lit_bytes (x : Bytes) : Nat = match x { b"\x01\x02" |-> Suc Zero ; _ |-> Zero }"#,
        "const lit_bytes_yes : Nat = lit_bytes 0x[0102]",
        "const lit_bytes_no : Nat = lit_bytes 0x[0201]",
    );
    let bytes_length_miss = elaborate(
        &mut env,
        "const lit_bytes_length_no : Nat = lit_bytes 0x[01]",
    );
    assert_eq!(eval_nat(&env, bytes_length_miss), 0);
    assert_selects(
        &mut env,
        "fn lit_empty_string (x : String) : Nat = match x { \"\" |-> Suc Zero ; _ |-> Zero }",
        "const lit_empty_string_yes : Nat = lit_empty_string \"\"",
        "const lit_empty_string_no : Nat = lit_empty_string \"x\"",
    );
    assert_selects(
        &mut env,
        "fn lit_empty_bytes (x : Bytes) : Nat = match x { 0x[] |-> Suc Zero ; _ |-> Zero }",
        "const lit_empty_bytes_yes : Nat = lit_empty_bytes 0x[]",
        "const lit_empty_bytes_no : Nat = lit_empty_bytes 0x[00]",
    );

    assert_eq!(env.env.trusted_base(), trusted_before);
    for name in [
        "lit_i8",
        "lit_float",
        "lit_float32",
        "lit_char",
        "lit_string",
        "lit_bytes",
    ] {
        assert!(
            !contains_proof_equality(body(&env, env.globals[name])),
            "literal match `{name}` must contain no proof equality, J, or Cast"
        );
    }
}

#[test]
fn unsupported_carriers_fail_closed_with_the_named_row_and_carrier() {
    // MEASURED: Decimal, a user-defined carrier, and an unadmitted native
    // carrier each return TypeMismatch from a real literal match.
    // CLAIMED: comparator-plan selection has no guessing/default success arm.
    // THE GAP: the supported Int control proves the parser/matcher is reached.
    let mut decimal = ElabEnv::new().expect("base environment");
    match decimal.elaborate_decl(
        "fn bad_decimal (x : Decimal) : Nat = match x { 0.0d |-> Suc Zero ; _ |-> Zero }",
    ) {
        Err(ElabError::TypeMismatch { reason, .. }) => {
            assert!(reason.contains("Decimal"));
            assert!(reason.contains("unsupported"));
        }
        other => panic!("Decimal literal pattern must fail closed, got {other:?}"),
    }

    let mut user = ElabEnv::new().expect("base environment");
    elaborate(&mut user, "data WrappedInt = WrapInt Int");
    match user.elaborate_decl(
        "fn bad_user (x : WrappedInt) : Nat = match x { 0 |-> Suc Zero ; _ |-> Zero }",
    ) {
        Err(ElabError::TypeMismatch { reason, .. }) => {
            assert!(reason.contains("WrappedInt"));
            assert!(reason.contains("numeric"));
        }
        other => panic!("user carrier literal pattern must fail closed, got {other:?}"),
    }

    let mut native = ElabEnv::new().expect("base environment");
    match native
        .elaborate_decl("fn bad_usize (x : USize) : Nat = match x { 0 |-> Suc Zero ; _ |-> Zero }")
    {
        Err(ElabError::TypeMismatch { reason, .. }) => {
            assert!(reason.contains("USize"));
            assert!(reason.contains("unsupported"));
        }
        other => panic!("unadmitted carrier literal pattern must fail closed, got {other:?}"),
    }

    let mut wrong_form = ElabEnv::new().expect("base environment");
    match wrong_form.elaborate_decl(
        "fn wrong_numeric_form (x : Int) : Nat = match x { 1.0 |-> Suc Zero ; _ |-> Zero }",
    ) {
        Err(ElabError::TypeMismatch { reason, .. }) => {
            assert!(reason.contains("Int"));
            assert!(reason.contains("numeric"));
        }
        other => panic!("wrong literal form must reject at its expected carrier, got {other:?}"),
    }

    let mut control = ElabEnv::new().expect("base environment");
    elaborate(
        &mut control,
        "fn admitted_int (x : Int) : Nat = match x { 0 |-> Suc Zero ; _ |-> Zero }",
    );
}

#[test]
fn comparator_semantic_duplicates_are_subsumed_and_finite_sets_stay_open() {
    // MEASURED: source-distinct equal Float and NFC String values report the
    // first arm as the winner; one finite Int literal without a residual is
    // non-exhaustive. CLAIMED: coverage uses comparator values and stays open.
    // THE GAP: checking the exact dead cause separates coverage from emptiness.
    let mut float = ElabEnv::new().expect("base environment");
    let float_source = "fn duplicate_float (x : Float) : Nat = match x { \
        0.0 |-> Zero ; 0e0 |-> Suc Zero ; _ |-> Zero }";
    match float.elaborate_decl(float_source) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { first, rest },
            ..
        }) => {
            assert!(rest.is_empty());
            assert_eq!(&float_source[first.start..first.end], "0.0 |-> Zero");
        }
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("comparator-equal Float arm must not be NoInhabitants"),
        other => panic!("duplicate Float literal must be subsumed, got {other:?}"),
    }

    let mut string = ElabEnv::new().expect("base environment");
    let string_source = r#"fn duplicate_string (x : String) : Nat = match x { "é" |-> Zero ; "e\u{301}" |-> Suc Zero ; _ |-> Zero }"#;
    match string.elaborate_decl(string_source) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { .. },
            ..
        }) => {}
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("NFC-equal String arm must not be NoInhabitants"),
        other => panic!("duplicate String literal must be subsumed, got {other:?}"),
    }

    let mut broad = ElabEnv::new().expect("base environment");
    let broad_source =
        "fn broad_first (x : Int) : Nat = match x { _ |-> Zero ; 0 |-> Suc Zero }";
    match broad.elaborate_decl(broad_source) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { first, rest },
            ..
        }) => {
            assert!(rest.is_empty());
            assert_eq!(&broad_source[first.start..first.end], "_ |-> Zero");
        }
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("a literal covered by a wildcard must not be NoInhabitants"),
        other => panic!("earlier wildcard must subsume the literal, got {other:?}"),
    }

    let mut incomplete = ElabEnv::new().expect("base environment");
    match incomplete.elaborate_decl(
        "fn incomplete_literals (x : Int) : Nat = match x { 0 |-> Zero ; 1 |-> Suc Zero }",
    ) {
        Err(ElabError::ExhaustivenessError { .. }) => {}
        other => panic!("finite literal set must stay non-exhaustive, got {other:?}"),
    }
}

#[test]
fn literals_compose_inside_constructor_columns() {
    // MEASURED: a constructor-contained literal reaches and falls through in
    // the existing nested matrix. CLAIMED: the value-test column composes with
    // structural splitting. THE GAP: distinct results prove the nested literal
    // is neither ignored nor mistaken for a constructor split.
    let mut nested = ElabEnv::new().expect("base environment");
    elaborate(
        &mut nested,
        "data LiteralWrapper = MkLiteralWrap Int | LiteralOther",
    );
    assert_selects(
        &mut nested,
        "fn nested_literal (x : LiteralWrapper) : Nat = match x { MkLiteralWrap 3 |-> Suc Zero ; MkLiteralWrap _ |-> Zero ; LiteralOther |-> Zero }",
        "const nested_literal_yes : Nat = nested_literal (MkLiteralWrap 3)",
        "const nested_literal_no : Nat = nested_literal (MkLiteralWrap 4)",
    );
}

#[test]
fn literals_compose_with_as_or_tuple_record_and_guard_slices() {
    // MEASURED: as/or/tuple/guard forms carry a literal through unequal runtime
    // outcomes, and the record form kernel-checks both applications. CLAIMED:
    // the new value column composes with all five prior pattern slices. THE GAP:
    // alias use and unequal payloads expose dropped or reordered columns; record
    // execution remains covered by the record slice because its opaque terminal
    // makes the interpreter's strict pair value Unknown.
    let mut env = ElabEnv::new().expect("base environment");

    elaborate(
        &mut env,
        "fn literal_as (x : Int) : Int = match x { 3 as matched |-> matched ; _ |-> x }",
    );
    let as_yes = elaborate(&mut env, "const literal_as_yes : Int = literal_as 3");
    let as_no = elaborate(&mut env, "const literal_as_no : Int = literal_as 4");
    assert!(matches!(eval_value(&env, as_yes), EvalVal::Int(3)));
    assert!(matches!(eval_value(&env, as_no), EvalVal::Int(4)));

    assert_selects(
        &mut env,
        "fn literal_or (x : Int) : Nat = match x { 1 | 2 |-> Suc Zero ; _ |-> Zero }",
        "const literal_or_yes : Nat = literal_or 2",
        "const literal_or_no : Nat = literal_or 3",
    );
    assert_selects(
        &mut env,
        "fn literal_tuple (x : (left : Int) × Int) : Nat = match x { (3, _) |-> Suc Zero ; (_, _) |-> Zero }",
        "const literal_tuple_yes : Nat = literal_tuple (3, 4)",
        "const literal_tuple_no : Nat = literal_tuple (2, 4)",
    );

    elaborate(&mut env, "record LiteralFields { left : Int, right : Int }");
    elaborate(
        &mut env,
        "fn literal_record (x : LiteralFields) : Nat = match x { { left = 3 } |-> Suc Zero ; { left = _ } |-> Zero }",
    );
    elaborate(
        &mut env,
        "const literal_record_yes : Nat = literal_record { left = 3, right = 4 }",
    );
    elaborate(
        &mut env,
        "const literal_record_no : Nat = literal_record { left = 2, right = 4 }",
    );
    assert_selects(
        &mut env,
        "fn literal_guard (x : Int) : Nat = match x { 0 if False |-> Suc (Suc Zero) ; 0 |-> Suc Zero ; _ |-> Zero }",
        "const literal_guard_yes : Nat = literal_guard 0",
        "const literal_guard_no : Nat = literal_guard 1",
    );
}

#[test]
fn boolean_constructor_patterns_and_general_top_catchall_refusal_are_unchanged() {
    // MEASURED: lexical booleans still take the constructor compiler, while a
    // match containing only a top wildcard retains the pre-slice refusal.
    // CLAIMED: literal syntax does not capture Bool or lift the general catchall
    // boundary. THE GAP: the paired accept/reject distinguishes both dispatches.
    let mut booleans = ElabEnv::new().expect("base environment");
    let selected = elaborate(
        &mut booleans,
        "const bool_constructor_pattern : Nat = match true { true |-> Suc Zero ; false |-> Zero }",
    );
    assert_eq!(eval_nat(&booleans, selected), 1);

    let mut wildcard = ElabEnv::new().expect("base environment");
    match wildcard.elaborate_decl("const top_wildcard_still_refused : Nat = match true { _ |-> Zero }") {
        Err(ElabError::Internal(reason)) => {
            assert!(reason.contains("wildcard/var not yet supported"));
        }
        other => panic!("general top wildcard refusal must remain unchanged, got {other:?}"),
    }
}
