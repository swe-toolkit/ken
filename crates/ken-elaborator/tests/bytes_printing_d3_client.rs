//! A printer-independent client derives byte, decode, and UTF-8 properties
//! for its own ASCII literal using the four primitive contracts and K3.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant.
/// MEASURED: an external checked client inhabits byte, concatenation,
/// decode, and UTF-8 properties for its own ASCII literal without new trust.
/// CLAIMED: all four primitive contracts apply beyond the printer tokens.
/// THE GAP: this client checks the independent facts, not Parsing's separate
/// recursive-decoder round-trip proof.
#[test]
fn fresh_ascii_literal_outside_parsing_derives_bytes_identity() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots(&[catalog_root()], "Data.Binary.BytesPrimitiveContracts")
        .expect("four checked contracts load");
    let before_parsing: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Parsing")
        .expect("Parsing loads through its real dependencies");
    let before_client: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before_client, before_parsing,
        "Parsing may inherit but not add trust"
    );

    // "Y9$" is independent of the printer's true/false/not/and/punctuation
    // literals. K3 checks its finite witness instead of decoding an opaque op.
    env.elaborate_file(
        r#"
import Data.Binary.BytesPrimitiveContracts
  (AllAscii, AllAsciiCodes, AsciiBytes, IsUtf8, Ascii89, Ascii57, Ascii36,
    MkAsciiCode, NoCodes, SomeCodes, ascii_bytes_utf8,
    bytes_concat_list_view, bytes_decode_encode, bytes_encode_ascii_octets)
import Data.Collections.Derived (list_append, map)
import Core.Classes.LawfulClasses (bytes_to_list_injective)
import Core.Logic.Transport (cong, sym, trans)

const fresh_text : String = "Y9$"
const fresh_ascii : AllAscii fresh_text =
  SomeCodes 89 (Cons Int 57 (Cons Int 36 (Nil Int))) (MkAsciiCode 89 Ascii89 Proved)
    (SomeCodes 57 (Cons Int 36 (Nil Int)) (MkAsciiCode 57 Ascii57 Proved)
      (SomeCodes 36 (Nil Int) (MkAsciiCode 36 Ascii36 Proved) NoCodes))
const expected_codes : List Int = Cons Int 89 (Cons Int 57 (Cons Int 36 (Nil Int)))
const expected_bytes : Bytes =
  list_to_bytes (map Int UInt8 int_to_uint8_raw expected_codes)

theorem fresh_codes : Equal (List Int)
    (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_text)))
    expected_codes =
  bytes_encode_ascii_octets fresh_text fresh_ascii

theorem map_uint8_codes_retract (xs : List UInt8)
    : Equal (List UInt8)
        (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int xs)) xs =
  match xs {
    Nil ↦ Proved;
    Cons h t ↦
      trans (List UInt8)
        (Cons UInt8 (int_to_uint8_raw (uint8_to_int h))
          (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
        (Cons UInt8 h (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
        (Cons UInt8 h t)
        (cong UInt8 (List UInt8) (int_to_uint8_raw (uint8_to_int h)) h
          (λhead. Cons UInt8 head
            (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t)))
          (uint8_int_retract h))
        (cong (List UInt8) (List UInt8)
          (map Int UInt8 int_to_uint8_raw (map UInt8 Int uint8_to_int t))
          t (Cons UInt8 h) (map_uint8_codes_retract t))
  }

theorem fresh_bytes_identity : Equal Bytes (bytes_encode fresh_text) expected_bytes =
  bytes_to_list_injective (bytes_encode fresh_text) expected_bytes
    (trans (List UInt8)
      (bytes_to_list (bytes_encode fresh_text))
      (map Int UInt8 int_to_uint8_raw expected_codes)
      (bytes_to_list expected_bytes)
      (trans (List UInt8)
        (bytes_to_list (bytes_encode fresh_text))
        (map Int UInt8 int_to_uint8_raw
          (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_text))))
        (map Int UInt8 int_to_uint8_raw expected_codes)
        (sym (List UInt8)
          (map Int UInt8 int_to_uint8_raw
            (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_text))))
          (bytes_to_list (bytes_encode fresh_text))
          (map_uint8_codes_retract (bytes_to_list (bytes_encode fresh_text))))
        (cong (List Int) (List UInt8)
          (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_text)))
          expected_codes (map Int UInt8 int_to_uint8_raw) fresh_codes))
      (sym (List UInt8)
        (bytes_to_list expected_bytes)
        (map Int UInt8 int_to_uint8_raw expected_codes)
        (list_bytes_roundtrip (map Int UInt8 int_to_uint8_raw expected_codes))))

const fresh_ascii_bytes : AsciiBytes (bytes_encode fresh_text) =
  J (λcodes _. AllAsciiCodes codes) fresh_ascii
    (sym (List Int)
      (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode fresh_text)))
      expected_codes fresh_codes)

theorem fresh_utf8 : IsUtf8 (bytes_encode fresh_text) =
  ascii_bytes_utf8 (bytes_encode fresh_text) fresh_ascii_bytes

theorem fresh_decode : Equal (Result Utf8Error String)
    (bytes_decode (bytes_encode fresh_text)) (Ok Utf8Error String fresh_text) =
  bytes_decode_encode fresh_text

theorem fresh_concat : Equal (List UInt8)
    (bytes_to_list (bytes_concat (bytes_encode fresh_text) expected_bytes))
    (list_append UInt8
      (bytes_to_list (bytes_encode fresh_text)) (bytes_to_list expected_bytes)) =
  bytes_concat_list_view (bytes_encode fresh_text) expected_bytes
"#,
    )
    .expect("fresh checked client derives an actual Bytes equality from F2 and K3");

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        after, before_client,
        "client proof must introduce no additional assumption"
    );
    for law in [
        "fresh_bytes_identity",
        "fresh_concat",
        "fresh_decode",
        "fresh_utf8",
    ] {
        assert!(env.globals.contains_key(law), "client law {law} checked");
    }
}
