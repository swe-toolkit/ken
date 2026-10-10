//! The bounded Cursor element-list view and the §6 DFA lexer input bridge.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §6. Runtime oracles:
//! conformance/stdlib/formal-languages/seed-lexer-bridge.md (seven cases).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError, NumericLitVal};
use ken_interp::eval::{eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::Decl;

const CURSOR: &str = "Capability.Parsing.Cursor";
const LEXER: &str = "Capability.Parsing.Lexer";

fn roots() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant (§6's two provider surfaces and laws).
/// MEASURED: distinct owned GlobalIds for each public declaration, generic
/// selected clients applying both bridge laws and both Cursor laws, rejection
/// of private selective imports, and identity-set trust equality before/after
/// each provider. CLAIMED: the eight names are checked, reusable and add zero
/// local trust. THE GAP: these assertions exercise generic law signatures;
/// seven separate interpreter cases exercise their closed behavior below.
#[test]
fn generic_clients_owner_ids_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    for provider in [
        "Algorithm.FormalLanguages.Dfa",
        "Capability.Diagnostics.Core",
        "Data.Collections.Derived",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[roots()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let cursor_owned = env
        .elaborate_module_from_roots(&[roots()], CURSOR)
        .expect("Cursor must roots-load with newly proved element laws");
    let after_cursor = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_cursor, before, "Cursor must add no trust");
    for name in [
        "cursor_take",
        "cursor_elements",
        "cursor_elements_peek_none",
        "cursor_elements_peek_some",
    ] {
        let id = env.globals[&format!("{CURSOR}.{name}")];
        assert!(cursor_owned.contains(&id), "Cursor must own {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "Cursor {name} must be checked and transparent"
        );
    }
    let lexer_owned = env
        .elaborate_module_from_roots(&[roots()], LEXER)
        .expect("Lexer must roots-load independently after providers");
    let after_lexer = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_lexer, after_cursor, "Lexer must add no trust");
    for name in [
        "dfa_cursor_run",
        "dfa_cursor_accepts",
        "dfa_cursor_run_elements",
        "dfa_cursor_accepts_elements",
    ] {
        let id = env.globals[&format!("{LEXER}.{name}")];
        assert!(lexer_owned.contains(&id), "Lexer must own {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "Lexer {name} must be checked and transparent"
        );
    }
    env.elaborate_file(
        r#"import Algorithm.FormalLanguages.Dfa (Dfa, accepts, run)
           import Capability.Parsing.Cursor
             (CursorOps, CursorLaws, cursor_advance, cursor_elements, cursor_peek,
              cursor_take, cursor_elements_peek_none, cursor_elements_peek_some)
           import Capability.Parsing.Lexer
             (dfa_cursor_run, dfa_cursor_accepts,
              dfa_cursor_run_elements, dfa_cursor_accepts_elements)
           fn client_take
             (c : Type) (el : Type) (loc : Type) (ops : CursorOps c el loc)
             (fuel : Nat) (cur : c) : List el =
             cursor_take c el loc ops fuel cur
           theorem client_run
             (q : Type) (c : Type) (el : Type) (loc : Type)
             (d : Dfa q el) (ops : CursorOps c el loc) (s : q) (cur : c) :
             Equal q
               (dfa_cursor_run q c el loc d ops s cur)
               (run q el d s (cursor_elements c el loc ops cur)) =
             dfa_cursor_run_elements q c el loc d ops s cur
           theorem client_accepts
             (q : Type) (c : Type) (el : Type) (loc : Type)
             (d : Dfa q el) (ops : CursorOps c el loc) (cur : c) :
             Equal Bool
               (dfa_cursor_accepts q c el loc d ops cur)
               (accepts q el d (cursor_elements c el loc ops cur)) =
             dfa_cursor_accepts_elements q c el loc d ops cur
           theorem client_end
             (c : Type) (el : Type) (loc : Type)
             (ops : CursorOps c el loc) (cur : c) :
             Equal (Option el) (cursor_peek c el loc ops cur) (None el) →
             Equal (List el) (cursor_elements c el loc ops cur) (Nil el) =
             cursor_elements_peek_none c el loc ops cur
           theorem client_step
             (c : Type) (el : Type) (loc : Type)
             (ops : CursorOps c el loc) (laws : CursorLaws c el loc ops)
             (cur : c) (x : el) :
             Equal (Option el) (cursor_peek c el loc ops cur) (Some el x) →
             Equal (List el)
               (cursor_elements c el loc ops cur)
               (Cons el x (cursor_elements c el loc ops
                 (cursor_advance c el loc ops cur))) =
             cursor_elements_peek_some c el loc ops laws cur x"#,
    )
    .expect("generic selectively imported §6 laws and operands must elaborate");
    for (owner, name) in [
        (CURSOR, "cursor_take_step"),
        (LEXER, "dfa_cursor_run_fuel"),
        (LEXER, "dfa_cursor_take_step"),
    ] {
        match env.elaborate_file(&format!("import {owner} ({name})")) {
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{owner}.{name}"));
            }
            other => panic!("private {owner}.{name} import result: {other:?}"),
        }
    }
}

const FIXTURE: &str = r#"
import Algorithm.FormalLanguages.Dfa (Dfa, MkDfa, accepts, run)
import Capability.Parsing.Cursor
  (ArgCursor, ArgLocation, CursorOps, MkCursorOps, arg_cursor_ops,
   arg_cursor_start, arg_cursor_laws, cursor_advance, cursor_elements,
   cursor_elements_peek_none, cursor_elements_peek_some, cursor_peek)
import Capability.Parsing.Lexer
  (dfa_cursor_run, dfa_cursor_accepts, dfa_cursor_run_elements,
   dfa_cursor_accepts_elements)
import Data.Collections.Derived (list_append)
import Core.Classes.LawfulClasses (bool_not)

const even_length : Dfa Bool UInt8 =
  MkDfa Bool UInt8 (λs. λx. bool_not s) True (λs. s)
const last_is_b : Dfa Bool UInt8 =
  MkDfa Bool UInt8
    (λs. λx. eq_int (uint8_to_int x) (98 : Int)) False (λs. s)
const aa : Bytes = bytes_encode "aa"
const a : Bytes = bytes_encode "a"
const ab : Bytes = bytes_encode "ab"
const empty_bytes : Bytes = bytes_encode ""
const cur_aa : ArgCursor = arg_cursor_start (Cons Bytes aa (Nil Bytes))
const cur_a : ArgCursor = arg_cursor_start (Cons Bytes a (Nil Bytes))
const cur_empty : ArgCursor = arg_cursor_start (Nil Bytes)
const cur_aaa : ArgCursor = arg_cursor_start
  (Cons Bytes aa (Cons Bytes a (Nil Bytes)))
const cur_gap : ArgCursor = arg_cursor_start
  (Cons Bytes a (Cons Bytes empty_bytes (Cons Bytes a (Nil Bytes))))
const cur_ab : ArgCursor = arg_cursor_start (Cons Bytes ab (Nil Bytes))
const cur_aa_last : ArgCursor = arg_cursor_start (Cons Bytes aa (Nil Bytes))
const independent_aaa : List UInt8 =
  list_append UInt8 (bytes_to_list aa) (bytes_to_list a)
const stuck_ops : CursorOps Unit Bool Unit =
  MkCursorOps Unit Bool Unit (λu. Suc Zero) (λu. Some Bool True) (λu. u) (λu. u)
const toggle : Dfa Bool Bool =
  MkDfa Bool Bool (λs. λx. bool_not s) True (λs. s)
"#;

fn fixture() -> (ElabEnv, EvalStore) {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[roots()], LEXER)
        .expect("real Lexer and provider closure roots-load");
    env.elaborate_file(FIXTURE)
        .expect("§6 runtime fixtures must elaborate without byte Proved assertions");
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, value) in &env.num_values {
        let value = match value {
            NumericLitVal::Int(n) => EvalVal::from(n.clone()),
            NumericLitVal::Float(f) => EvalVal::Float(*f),
            NumericLitVal::Float32(f) => EvalVal::Float32(*f),
            NumericLitVal::Decimal { coeff, exp } => {
                ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
            }
            NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
            NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
        };
        store.num_values.insert(*id, value);
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    (env, store)
}

fn observe(env: &mut ElabEnv, store: &mut EvalStore, name: &str, ty: &str, expr: &str) -> EvalVal {
    let id = env
        .elaborate_decl(&format!("const {name} : {ty} = {expr}"))
        .unwrap_or_else(|error| panic!("{name} expression must elaborate: {error:?}"));
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        other => panic!("{name} must elaborate transparently, got {other:?}"),
    }
}

fn bytes(env: &ElabEnv, value: &EvalVal) -> Vec<u8> {
    let mut output = Vec::new();
    let mut rest = value;
    loop {
        match rest {
            EvalVal::Ctor { id, .. } if *id == env.prelude_env.nil_id => return output,
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.cons_id => {
                match args.get(1) {
                    Some(EvalVal::Int(byte)) => output.push(u8::try_from(*byte).expect("UInt8")),
                    other => panic!("expected UInt8 head, got {other:?}"),
                }
                rest = args.get(2).expect("Cons tail");
            }
            other => panic!("expected List UInt8, got {other:?}"),
        }
    }
}

fn bool_value(env: &ElabEnv, value: &EvalVal) -> bool {
    match value {
        EvalVal::Ctor { id, args, .. }
            if *id == env.numeric_env.bool_true_id && args.is_empty() =>
        {
            true
        }
        EvalVal::Ctor { id, args, .. }
            if *id == env.numeric_env.bool_false_id && args.is_empty() =>
        {
            false
        }
        EvalVal::Bool(value) => *value,
        other => panic!("expected Bool constructor, got {other:?}"),
    }
}

fn bools(env: &ElabEnv, value: &EvalVal) -> Vec<bool> {
    let mut output = Vec::new();
    let mut rest = value;
    loop {
        match rest {
            EvalVal::Ctor { id, .. } if *id == env.prelude_env.nil_id => return output,
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.cons_id => {
                output.push(bool_value(env, args.get(1).expect("Cons Bool head")));
                rest = args.get(2).expect("Cons tail");
            }
            other => panic!("expected List Bool, got {other:?}"),
        }
    }
}

fn octets(env: &mut ElabEnv, store: &mut EvalStore, name: &str, expr: &str, want: &[u8]) {
    let actual = observe(env, store, name, "List UInt8", expr);
    assert_eq!(bytes(env, &actual), want, "{name}: exact runtime octets");
}

fn truth(env: &mut ElabEnv, store: &mut EvalStore, name: &str, expr: &str, want: bool) {
    let actual = observe(env, store, name, "Bool", expr);
    assert_eq!(bool_value(env, &actual), want, "{name}: runtime Bool");
}

fn state(env: &mut ElabEnv, store: &mut EvalStore, name: &str, expr: &str, want: bool) {
    truth(env, store, name, expr, want);
}

fn peek(env: &mut ElabEnv, store: &mut EvalStore, name: &str, expr: &str, want: Option<i64>) {
    let actual = observe(env, store, name, "Option UInt8", expr);
    match want {
        None => assert!(
            matches!(actual, EvalVal::Ctor { id, .. } if id == env.globals["None"]),
            "{name}: expected None, got {actual:?}"
        ),
        Some(byte) => match actual {
            EvalVal::Ctor { id, args, .. } if id == env.globals["Some"] => {
                assert_eq!(
                    args.last(),
                    Some(&EvalVal::Int(byte)),
                    "{name}: expected byte"
                );
            }
            other => panic!("{name}: expected Some {byte}, got {other:?}"),
        },
    }
}

fn lex_even(cur: &str) -> String {
    format!("dfa_cursor_accepts Bool ArgCursor UInt8 ArgLocation even_length arg_cursor_ops {cur}")
}
fn lex_run(cur: &str) -> String {
    format!("dfa_cursor_run Bool ArgCursor UInt8 ArgLocation even_length arg_cursor_ops True {cur}")
}
fn elements(cur: &str) -> String {
    format!("cursor_elements ArgCursor UInt8 ArgLocation arg_cursor_ops {cur}")
}

/// Promise class: durable invariant. MEASURED: two 97 octets, Some 97, the
/// one-octet suffix after advance, and True after two DFA steps. CLAIMED:
/// successful peek contributes one element and processing two bytes accepts.
/// THE GAP: the generic law is checked separately; this is a runtime instance.
#[test]
fn seed_accepts_two_bytes_and_peek_some() {
    let (mut env, mut store) = fixture();
    octets(
        &mut env,
        &mut store,
        "aa_elements",
        &elements("cur_aa"),
        &[97, 97],
    );
    peek(
        &mut env,
        &mut store,
        "aa_first",
        "cursor_peek ArgCursor UInt8 ArgLocation arg_cursor_ops cur_aa",
        Some(97),
    );
    octets(
        &mut env,
        &mut store,
        "aa_tail",
        &elements("(cursor_advance ArgCursor UInt8 ArgLocation arg_cursor_ops cur_aa)"),
        &[97],
    );
    truth(&mut env, &mut store, "aa_accept", &lex_even("cur_aa"), true);
}

/// Promise class: durable invariant. MEASURED: one octet, False final state,
/// False acceptance for the same even-length machine. CLAIMED: rejection
/// remains distinct from empty and two-byte acceptance. THE GAP: finite seed.
#[test]
fn seed_rejects_one_byte() {
    let (mut env, mut store) = fixture();
    octets(
        &mut env,
        &mut store,
        "a_elements",
        &elements("cur_a"),
        &[97],
    );
    state(&mut env, &mut store, "a_state", &lex_run("cur_a"), false);
    truth(&mut env, &mut store, "a_reject", &lex_even("cur_a"), false);
}

/// Promise class: durable invariant. MEASURED: None, Nil, unchanged True
/// state, True acceptance on empty input, and list-run True. CLAIMED: an end
/// cursor has no initial step. THE GAP: this is a closed instance of end law.
#[test]
fn seed_accepts_empty_input_and_peek_none() {
    let (mut env, mut store) = fixture();
    peek(
        &mut env,
        &mut store,
        "empty_peek",
        "cursor_peek ArgCursor UInt8 ArgLocation arg_cursor_ops cur_empty",
        None,
    );
    octets(
        &mut env,
        &mut store,
        "empty_elements",
        &elements("cur_empty"),
        &[],
    );
    state(
        &mut env,
        &mut store,
        "empty_state",
        &lex_run("cur_empty"),
        true,
    );
    truth(
        &mut env,
        &mut store,
        "empty_accept",
        &lex_even("cur_empty"),
        true,
    );
    state(
        &mut env,
        &mut store,
        "empty_list_state",
        "run Bool UInt8 even_length True (Nil UInt8)",
        true,
    );
}

/// Promise class: durable invariant. MEASURED: three actual runtime bytes
/// across two arguments agree with an independently constructed concatenation
/// of their byte views; streaming/list runs and acceptance are all False.
/// CLAIMED: neither shared implementation may silently drop the last byte.
/// THE GAP: the independent list is itself interpreter-evaluated; no generic
/// byte-list equality proof is claimed.
#[test]
fn seed_run_agrees_with_independent_list() {
    let (mut env, mut store) = fixture();
    octets(
        &mut env,
        &mut store,
        "across_arguments",
        &elements("cur_aaa"),
        &[97, 97, 97],
    );
    octets(
        &mut env,
        &mut store,
        "independent_word",
        "independent_aaa",
        &[97, 97, 97],
    );
    state(
        &mut env,
        &mut store,
        "stream_three",
        &lex_run("cur_aaa"),
        false,
    );
    state(
        &mut env,
        &mut store,
        "independent_three",
        "run Bool UInt8 even_length True independent_aaa",
        false,
    );
    state(
        &mut env,
        &mut store,
        "elements_three",
        &format!("run Bool UInt8 even_length True ({})", elements("cur_aaa")),
        false,
    );
    truth(
        &mut env,
        &mut store,
        "three_reject",
        &lex_even("cur_aaa"),
        false,
    );
}

/// Promise class: durable invariant. MEASURED: [97,97] and the second Some
/// byte across a normalised empty argument; both stream and list accept.
/// CLAIMED: an empty middle argument is not an end-of-input signal.
/// THE GAP: no location or general argument concatenation law is inferred.
#[test]
fn seed_crosses_empty_argument() {
    let (mut env, mut store) = fixture();
    octets(
        &mut env,
        &mut store,
        "gap_elements",
        &elements("cur_gap"),
        &[97, 97],
    );
    peek(&mut env, &mut store, "gap_next", "cursor_peek ArgCursor UInt8 ArgLocation arg_cursor_ops (cursor_advance ArgCursor UInt8 ArgLocation arg_cursor_ops cur_gap)", Some(97));
    truth(
        &mut env,
        &mut store,
        "gap_accept",
        &lex_even("cur_gap"),
        true,
    );
    truth(
        &mut env,
        &mut store,
        "gap_list_accept",
        &format!("accepts Bool UInt8 even_length ({})", elements("cur_gap")),
        true,
    );
}

/// Promise class: durable invariant. MEASURED: same two-byte length and first
/// octet, different second octet; last-byte DFA accepts ab but rejects aa,
/// matching independent byte-view list decisions. CLAIMED: the runner reads
/// from the advanced cursor, not a repeated first byte. THE GAP: two words
/// do not constitute the generic bridge law.
#[test]
fn seed_uses_advanced_byte_not_first_twice() {
    let (mut env, mut store) = fixture();
    octets(
        &mut env,
        &mut store,
        "ab_elements",
        &elements("cur_ab"),
        &[97, 98],
    );
    octets(
        &mut env,
        &mut store,
        "aa_last_elements",
        &elements("cur_aa_last"),
        &[97, 97],
    );
    truth(
        &mut env,
        &mut store,
        "ab_accept",
        "dfa_cursor_accepts Bool ArgCursor UInt8 ArgLocation last_is_b arg_cursor_ops cur_ab",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "aa_last_reject",
        "dfa_cursor_accepts Bool ArgCursor UInt8 ArgLocation last_is_b arg_cursor_ops cur_aa_last",
        false,
    );
    truth(
        &mut env,
        &mut store,
        "independent_ab",
        "accepts Bool UInt8 last_is_b (bytes_to_list ab)",
        true,
    );
    truth(
        &mut env,
        &mut store,
        "independent_aa",
        "accepts Bool UInt8 last_is_b (bytes_to_list aa)",
        false,
    );
}

/// Promise class: durable invariant. MEASURED: without progress laws,
/// `cursor_elements` stops after one unit of fuel, the unfold equation's right
/// side has two elements, and streaming/list runners both step once to False.
/// CLAIMED: the Some law needs progress, while the bridges need no laws.
/// THE GAP: a closed stuck control refutes the unsafe generalization but does
/// not claim to construct `CursorLaws` for this dictionary.
#[test]
fn seed_stuck_advance_law_boundary() {
    let (mut env, mut store) = fixture();
    let scan = observe(
        &mut env,
        &mut store,
        "stuck_scan",
        "List Bool",
        "cursor_elements Unit Bool Unit stuck_ops MkUnit",
    );
    assert_eq!(bools(&env, &scan), [true], "bounded scan takes one element");
    let double = observe(&mut env, &mut store, "stuck_unfold", "List Bool", "Cons Bool True (cursor_elements Unit Bool Unit stuck_ops (cursor_advance Unit Bool Unit stuck_ops MkUnit))");
    assert_eq!(
        bools(&env, &double),
        [true, true],
        "unlawful unfold would repeat"
    );
    state(
        &mut env,
        &mut store,
        "stuck_run",
        "dfa_cursor_run Bool Unit Bool Unit toggle stuck_ops True MkUnit",
        false,
    );
    state(
        &mut env,
        &mut store,
        "stuck_list_run",
        "run Bool Bool toggle True (cursor_elements Unit Bool Unit stuck_ops MkUnit)",
        false,
    );
}
