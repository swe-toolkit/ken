//! Checked late byte contracts and an independent, non-Parsing ASCII client.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{
    apply, eval, prim_reduce, prim_reduce_elaborated, EvalStore, EvalVal, ListCharIds,
};
use ken_kernel::{KernelError, Term};

const MODULE: &str = "Data.Binary.BytesPrimitiveContracts";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn byte_list(env: &ElabEnv, value: &EvalVal) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut cursor = value;
    loop {
        match cursor {
            EvalVal::Ctor { id, .. } if *id == env.prelude_env.nil_id => return bytes,
            EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.cons_id => {
                let Some(EvalVal::Int(code)) = args.get(1) else {
                    panic!("byte view head must be a UInt8");
                };
                bytes.push(u8::try_from(*code).expect("UInt8 range"));
                cursor = &args[2];
            }
            other => panic!("byte view must be a List UInt8, got {other:?}"),
        }
    }
}

#[test]
fn four_late_contracts_and_fresh_ascii_client_check() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots(&[catalog_root()], "Data.Collections.Derived")
        .expect("checked Derived provider");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("byte contracts import their real Derived dependency");
    env.execute_loaded_entry_checked_fences(MODULE)
        .expect("all checked byte contract declarations elaborate");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let actual: BTreeSet<_> = after.difference(&before).copied().collect();
    let actual_names: BTreeSet<_> = actual
        .iter()
        .map(|id| match env.env.lookup(*id) {
            Some(ken_kernel::Decl::Opaque { name, .. }) => name.clone(),
            other => panic!("new trust item is not a named postulate: {other:?}"),
        })
        .collect();
    let expected_names: BTreeSet<_> = [
        "bytes_concat_list_view",
        "bytes_encode_ascii_octets",
        "bytes_decode_encode",
        "ascii_bytes_utf8",
    ]
    .into_iter()
    .map(|name| format!("{MODULE}.{name}"))
    .collect();
    assert_eq!(
        actual_names, expected_names,
        "exactly four new trusted contracts"
    );
    let client = r#"
import Core.Logic.Transport (sym)
import Data.Collections.Derived (list_append)
import Data.Binary.BytesPrimitiveContracts
  (AllAscii, AllAsciiCodes, AsciiBytes, IsUtf8, NoCodes, SomeCodes,
   Ascii81, Ascii55, Ascii63, MkAsciiCode, bytes_concat_list_view,
   bytes_encode_ascii_octets, bytes_decode_encode, ascii_bytes_utf8)

const fresh_bytes_client_text : String = "Q7?"
const fresh_bytes_client_ascii : AllAscii fresh_bytes_client_text =
  SomeCodes 81 (Cons Int 55 (Cons Int 63 (Nil Int))) (MkAsciiCode 81 Ascii81 Proved)
    (SomeCodes 55 (Cons Int 63 (Nil Int)) (MkAsciiCode 55 Ascii55 Proved)
      (SomeCodes 63 (Nil Int) (MkAsciiCode 63 Ascii63 Proved) NoCodes))

const fresh_bytes_expected_codes : List Int =
  Cons Int 81 (Cons Int 55 (Cons Int 63 (Nil Int)))

theorem fresh_bytes_client_codes
    : Equal (List Int)
        (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_bytes_client_text)))
        fresh_bytes_expected_codes =
  bytes_encode_ascii_octets fresh_bytes_client_text fresh_bytes_client_ascii

theorem fresh_bytes_client_decodes
    : Equal (Result Utf8Error String)
        (bytes_decode (bytes_encode fresh_bytes_client_text))
        (Ok Utf8Error String fresh_bytes_client_text) =
  bytes_decode_encode fresh_bytes_client_text

theorem fresh_bytes_client_concat (a : Bytes) (b : Bytes)
    : Equal (List UInt8)
        (bytes_to_list (bytes_concat a b))
        (list_append UInt8 (bytes_to_list a) (bytes_to_list b)) =
  bytes_concat_list_view a b

theorem fresh_bytes_client_utf8 : IsUtf8 (bytes_encode fresh_bytes_client_text) =
  ascii_bytes_utf8 (bytes_encode fresh_bytes_client_text)
    (J (λcodes _. AllAsciiCodes codes)
      fresh_bytes_client_ascii
      (sym (List Int)
        (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_bytes_client_text)))
        (map Char Int charToInt (string_to_list_char fresh_bytes_client_text))
        (bytes_encode_ascii_octets fresh_bytes_client_text fresh_bytes_client_ascii)))
"#;
    env.elaborate_file(client)
        .expect("fresh generic client checks");
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Parsing")
        .expect("existing Parsing module is available without modification");
    env.elaborate_file(
        "import Capability.Parsing.Parsing (IsUtf8 as parsing_is_utf8)\n\
         theorem ascii_contract_matches_parser (bs : Bytes) (w : AsciiBytes bs)\n\
             : parsing_is_utf8 bs = ascii_bytes_utf8 bs w",
    )
    .expect("contract predicate must convert to existing Parsing.IsUtf8");
    env.elaborate_decl("const outside_code : Int = 233")
        .expect("literal index is an ordinary checked Int");
    let outside_ascii = env
        .elaborate_decl(
            "const outside_ascii : AsciiCode outside_code = MkAsciiCode 233 Ascii81 Proved",
        )
        .expect_err("ASCII witness must reject code 233");
    assert!(
        matches!(
            outside_ascii,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "wrong rejection for out-of-range code: {outside_ascii:?}"
    );
}

#[test]
fn byte_primitives_satisfy_each_contract_on_independent_inputs() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("checked source contracts");
    let mut store = EvalStore::new();
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    let bytes_to_list = eval(
        &[],
        &Term::const_(env.bytes_env.bytes_to_list_id, vec![]),
        &env.env,
        &mut store,
    );

    // F1: concatenation preserves the order and boundaries of both inputs.
    let left = vec![0, 127, 255];
    let right = vec![42, 81];
    let concatenated = prim_reduce(
        "bytes_concat",
        &[EvalVal::Bytes(left.clone()), EvalVal::Bytes(right.clone())],
    );
    assert_eq!(concatenated, EvalVal::Bytes(vec![0, 127, 255, 42, 81]));
    let mut expected = left;
    expected.extend(right);
    let view = apply(bytes_to_list.clone(), concatenated, &env.env, &mut store);
    assert_eq!(byte_list(&env, &view), expected);

    // F2: a whole ASCII string's byte codes equal its independently named
    // character codes, including the edge codes and a fresh printer-free text.
    for (s, codes) in [("Q7?", vec![81, 55, 63]), ("\0\u{7f}", vec![0, 127])] {
        let encoded = prim_reduce("bytes_encode", &[EvalVal::Str(s.into())]);
        let view = apply(bytes_to_list.clone(), encoded, &env.env, &mut store);
        assert_eq!(byte_list(&env, &view), codes);
        assert_eq!(
            codes.into_iter().map(i32::from).collect::<Vec<_>>(),
            s.chars().map(|c| c as i32).collect::<Vec<_>>()
        );
    }

    // F3: decoding an encoded String returns that String, including a
    // non-ASCII scalar; the axiom is quantified over all Strings.
    for text in ["Q7?", "é"] {
        let encoded = prim_reduce("bytes_encode", &[EvalVal::Str(text.into())]);
        let result = prim_reduce_elaborated("bytes_decode", &[encoded], &env, &mut store);
        assert!(matches!(result, EvalVal::Ctor { id, ref args, .. }
            if id == env.globals["Ok"]
                && matches!(args.last(), Some(EvalVal::Str(decoded)) if decoded == text)));
    }

    // F4': for independently known ASCII bytes, decode then encode returns
    // identical bytes. This asserts only ASCII, not an inverse for all UTF-8.
    let ascii = EvalVal::Bytes(vec![0, 81, 127]);
    let decoded = prim_reduce_elaborated("bytes_decode", &[ascii.clone()], &env, &mut store);
    let EvalVal::Ctor { id, args, .. } = decoded else {
        panic!("ASCII must decode successfully");
    };
    assert_eq!(id, env.globals["Ok"]);
    let Some(EvalVal::Str(text)) = args.last() else {
        panic!("ASCII must decode to String");
    };
    assert_eq!(
        prim_reduce("bytes_encode", &[EvalVal::Str(text.clone())]),
        ascii
    );
    let non_nfc = EvalVal::Bytes(vec![0x65, 0xcc, 0x81]);
    let decoded = prim_reduce_elaborated("bytes_decode", &[non_nfc.clone()], &env, &mut store);
    let EvalVal::Ctor { id, args, .. } = decoded else {
        panic!("valid non-NFC UTF-8 must decode");
    };
    assert_eq!(id, env.globals["Ok"]);
    let Some(EvalVal::Str(normalized)) = args.last() else {
        panic!("valid UTF-8 must decode to String");
    };
    assert_ne!(
        prim_reduce("bytes_encode", &[EvalVal::Str(normalized.clone())]),
        non_nfc,
        "F4' must not be generalized to non-NFC UTF-8 bytes"
    );
}
