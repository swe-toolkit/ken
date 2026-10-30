//! LANG-LITERAL-PLAN-CHECKED-IDS: source takeovers must not redirect checked
//! literal-comparison plans on either the CLI or single-declaration route.
//!
//! Promise class: durable invariant. MEASURED: each of five source declarations
//! takes over one flat global spelling, yet a false literal-match theorem is
//! rejected with and without takeover, and the true theorem checks with it.
//! CLAIMED: plans use their carriers' registered comparison/view identities.
//! THE GAP: the false and true propositions differ only in the selected
//! literal value; other carriers and names remain CI/review territory.

use std::fs;
use std::process::Command;

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval_checked, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, KernelError};

struct Case {
    name: &'static str,
    takeover: &'static str,
    value: &'static str,
    plan: &'static str,
    matching_value: &'static str,
    false_theorem: &'static str,
    true_theorem: &'static str,
    miss_call: &'static str,
    hit_call: &'static str,
}

const EQ_INT: Case = Case {
    name: "eq_int",
    takeover: "fn eq_int (_x : Int) (_y : Int) : Bool = True",
    value: "const five_u8 : UInt8 = 5",
    plan: "fn u8_plan (x : UInt8) : Nat = match x { 3 |-> Suc Zero ; _ |-> Zero }",
    matching_value: "const three_u8 : UInt8 = 3",
    false_theorem: "theorem u8_five_is_one : Equal Nat (u8_plan five_u8) (Suc Zero) = Proved",
    true_theorem:
        "theorem u8_three_is_reflexive : Equal Nat (u8_plan three_u8) (u8_plan three_u8) = Refl",
    miss_call: "const missed : Nat = u8_plan five_u8",
    hit_call: "const hit : Nat = u8_plan three_u8",
};
const STRING_VIEW: Case = Case {
    name: "string_to_list_char",
    takeover: "fn string_to_list_char (_s : String) : List Char = Nil Char",
    value: "const xy : String = \"xy\"",
    plan: "fn s_plan (x : String) : Nat = match x { \"\" |-> Suc Zero ; _ |-> Zero }",
    matching_value: "const empty_string : String = \"\"",
    false_theorem: "theorem s_xy_is_one : Equal Nat (s_plan xy) (Suc Zero) = Proved",
    true_theorem: "theorem s_empty_is_one : Equal Nat (s_plan empty_string) (Suc Zero) = Proved",
    miss_call: "const missed : Nat = s_plan xy",
    hit_call: "const hit : Nat = s_plan empty_string",
};
const BYTES_VIEW: Case = Case {
    name: "bytes_to_list",
    takeover: "fn bytes_to_list (_bs : Bytes) : List UInt8 = Nil UInt8",
    value: "const one_byte : Bytes = 0x[01]",
    plan: "fn b_plan (x : Bytes) : Nat = match x { 0x[] |-> Suc Zero ; _ |-> Zero }",
    matching_value: "const empty_bytes : Bytes = 0x[]",
    false_theorem: "theorem b_one_is_one : Equal Nat (b_plan one_byte) (Suc Zero) = Proved",
    true_theorem:
        "theorem b_empty_is_reflexive : Equal Nat (b_plan empty_bytes) (b_plan empty_bytes) = Refl",
    miss_call: "const missed : Nat = b_plan one_byte",
    hit_call: "const hit : Nat = b_plan empty_bytes",
};
const UINT8_VIEW: Case = Case {
    name: "uint8_to_int",
    takeover: "fn uint8_to_int (_x : UInt8) : Int = 0",
    value: "const one_byte : Bytes = 0x[01]",
    plan: "fn b_plan (x : Bytes) : Nat = match x { 0x[00] |-> Suc Zero ; _ |-> Zero }",
    matching_value: "const zero_byte : Bytes = 0x[00]",
    false_theorem: "theorem b_one_is_one : Equal Nat (b_plan one_byte) (Suc Zero) = Proved",
    true_theorem:
        "theorem b_zero_is_reflexive : Equal Nat (b_plan zero_byte) (b_plan zero_byte) = Refl",
    miss_call: "const missed : Nat = b_plan one_byte",
    hit_call: "const hit : Nat = b_plan zero_byte",
};
const INT8_VIEW: Case = Case {
    name: "int8_to_int",
    takeover: "fn int8_to_int (_x : Int8) : Int = 0",
    value: "const five_i8 : Int8 = 5",
    plan: "fn i8_plan (x : Int8) : Nat = match x { 3 |-> Suc Zero ; _ |-> Zero }",
    matching_value: "const three_i8 : Int8 = 3",
    false_theorem: "theorem i8_five_is_one : Equal Nat (i8_plan five_i8) (Suc Zero) = Proved",
    true_theorem:
        "theorem i8_three_is_reflexive : Equal Nat (i8_plan three_i8) (i8_plan three_i8) = Refl",
    miss_call: "const missed : Nat = i8_plan five_i8",
    hit_call: "const hit : Nat = i8_plan three_i8",
};

fn source(case: &Case, takeover: bool, true_theorem: bool) -> String {
    let theorem = if true_theorem {
        case.true_theorem
    } else {
        case.false_theorem
    };
    let mut declarations = Vec::new();
    if takeover {
        declarations.push(case.takeover);
    }
    declarations.extend([case.value, case.plan]);
    if true_theorem {
        declarations.push(case.matching_value);
    }
    declarations.push(theorem);
    format!("{}\n", declarations.join("\n"))
}

fn check_cli(case: &Case, takeover: bool, true_theorem: bool) {
    let dir = tempfile::tempdir().expect("unique checked-source directory");
    let path = dir.path().join("entry.ken");
    fs::write(&path, source(case, takeover, true_theorem)).expect("write Ken fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("check")
        .arg(&path)
        .output()
        .expect("run ken check");
    let stderr = String::from_utf8_lossy(&output.stderr);
    if true_theorem {
        assert!(
            output.status.success(),
            "{}: true theorem rejected: {stderr}",
            case.name
        );
    } else {
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}: false theorem accepted: {stderr}",
            case.name
        );
        assert!(
            stderr.contains("KernelRejected { error: TypeMismatch"),
            "{}: false theorem must fail at kernel equality: {stderr}",
            case.name
        );
    }
}

fn check_declarations(case: &Case, takeover: bool, true_theorem: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    let trust_before = env.env.trusted_base();
    if takeover {
        let original = env.globals[case.name];
        env.elaborate_decl(case.takeover)
            .expect("source-written takeover");
        assert_ne!(
            env.globals[case.name], original,
            "{} must really take over the flat map",
            case.name
        );
    }
    env.elaborate_decl(case.value).expect("source value");
    env.elaborate_decl(case.plan).expect("source literal match");
    if true_theorem {
        env.elaborate_decl(case.matching_value)
            .expect("source matching value");
    }
    let result = env.elaborate_decl(if true_theorem {
        case.true_theorem
    } else {
        case.false_theorem
    });
    if true_theorem {
        result.unwrap_or_else(|error| panic!("{}: true theorem rejected: {error:?}", case.name));
        // Primitive-backed views may leave a closed proposition neutral under
        // kernel conversion. Observe both arms through the typed evaluator,
        // rather than treating reflexivity as proof of a branch selection.
        assert_eq!(
            evaluate_nat(&mut env, case.miss_call),
            0,
            "{}: wrong arm selected",
            case.name
        );
        assert_eq!(
            evaluate_nat(&mut env, case.hit_call),
            1,
            "{}: matching arm missed",
            case.name
        );
    } else {
        match result {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }) => {}
            other => panic!(
                "{}: false theorem must fail at kernel equality: {other:?}",
                case.name
            ),
        }
    }
    assert_eq!(
        env.env.trusted_base(),
        trust_before,
        "{}: takeover must not add trust",
        case.name
    );
}

fn evaluate_nat(env: &mut ElabEnv, source: &str) -> usize {
    let id = env.elaborate_decl(source).expect("source observation");
    let Some(Decl::Transparent { body, ty, .. }) = env.env.lookup(id) else {
        panic!("expected transparent checked observation")
    };
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
    let mut value =
        eval_checked(body, ty, &env.env, &mut store).expect("typed literal match must evaluate");
    let mut count = 0;
    loop {
        match value {
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.zero_id && args.is_empty() => {
                return count;
            }
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.suc_id && args.len() == 1 => {
                count += 1;
                value = args[0].clone();
            }
            other => panic!("expected Nat observation, got {other:?}"),
        }
    }
}

fn check_case(case: &Case) {
    // The unshadowed false theorem must already be false; source takeover
    // changes only its flat spelling, never the checked comparison relation.
    for (takeover, true_theorem) in [(false, false), (true, false), (true, true)] {
        check_cli(case, takeover, true_theorem);
        check_declarations(case, takeover, true_theorem);
    }
}

#[test]
fn eq_int_source_takeover_does_not_redirect_fixed_width_match() {
    check_case(&EQ_INT);
}

#[test]
fn string_view_source_takeover_does_not_redirect_string_match() {
    check_case(&STRING_VIEW);
}

#[test]
fn bytes_view_source_takeover_does_not_redirect_bytes_match() {
    check_case(&BYTES_VIEW);
}

#[test]
fn uint8_view_source_takeover_does_not_redirect_bytes_match() {
    check_case(&UINT8_VIEW);
}

#[test]
fn int8_view_source_takeover_does_not_redirect_fixed_width_match() {
    check_case(&INT8_VIEW);
}
