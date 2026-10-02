//! Consumer of Decoder's success-only recursion theorem on encoded input.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// Promise class: durable invariant.
/// MEASURED: a checked client applies decoder_recursive_succeeds to its own
/// recursive list-of-octets decoder with honest remaining length and derives
/// Decoded for bytes_encode "Y".
/// CLAIMED: the public theorem allows a client to derive success on printed
/// input without naming Decoder's private fuel worker.
/// THE GAP: this is a small client, not Parsing's full printer/grammar theorem;
/// the layer tests a structural input case, not the value of the encoded byte.
#[test]
fn printed_input_decodes_via_public_recursive_success_theorem() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Cursor")
        .expect("Cursor provider loads");
    let before_decoder: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Decoder")
        .expect("Decoder provider loads");
    let before_client: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before_decoder, before_client, "Decoder adds no new trust");

    env.elaborate_file(
        r#"
import Capability.Parsing.Cursor (CursorOps, MkCursorOps, cursor_remaining)
import Data.Numeric.Nat.Order (lt_nat)
import Capability.Parsing.Decoder
  (Decoder, DecoderResult, Decoded, decoder_recursive, decoder_recursive_succeeds)
import Data.Collections.Derived (length)
import Core.Logic.Transport (sym, trans)

fn sample_remaining (cur : List UInt8) : Nat = length UInt8 cur
fn sample_peek (cur : List UInt8) : Option UInt8 = None UInt8
fn sample_advance (cur : List UInt8) : List UInt8 = cur
fn sample_locate (cur : List UInt8) : Nat = Zero
const sample_ops : CursorOps (List UInt8) UInt8 Nat =
  MkCursorOps (List UInt8) UInt8 Nat
    sample_remaining sample_peek sample_advance sample_locate

fn sample_layer
    (recur : Decoder (List UInt8) Nat Bool)
    : Decoder (List UInt8) Nat Bool =
  λcur. match cur {
    Nil ↦ Decoded (List UInt8) Nat Bool True (Nil UInt8);
    Cons head tail ↦ recur (Nil UInt8)
  }

fn sample_spec (cur : List UInt8) (v : Bool) (next : List UInt8) : Prop =
  Equal (DecoderResult (List UInt8) Nat Bool)
    (Decoded (List UInt8) Nat Bool v next)
    (Decoded (List UInt8) Nat Bool True (Nil UInt8))

theorem sample_nil_lt_cons (head : UInt8) (tail : List UInt8)
    : Equal Bool
        (lt_nat
          (cursor_remaining (List UInt8) UInt8 Nat sample_ops (Nil UInt8))
          (cursor_remaining (List UInt8) UInt8 Nat sample_ops (Cons UInt8 head tail))) True =
  Proved

theorem sample_spec_empty : sample_spec (Nil UInt8) True (Nil UInt8) =
  and_intro Top Top Proved Proved

theorem sample_step
    (recur : Decoder (List UInt8) Nat Bool)
    : (cur : List UInt8)
      → ((inner : List UInt8) → (v : Bool) → (next : List UInt8)
        → Equal Bool
            (lt_nat
              (cursor_remaining (List UInt8) UInt8 Nat sample_ops inner)
              (cursor_remaining (List UInt8) UInt8 Nat sample_ops cur)) True
        → sample_spec inner v next
        → Equal (DecoderResult (List UInt8) Nat Bool)
            (recur inner) (Decoded (List UInt8) Nat Bool v next))
      → (v : Bool) → (next : List UInt8) → sample_spec cur v next
      → Equal (DecoderResult (List UInt8) Nat Bool)
          (sample_layer recur cur) (Decoded (List UInt8) Nat Bool v next) =
  λcur. match cur {
    Nil ↦ λih. λv. λnext. λholds.
      sym (DecoderResult (List UInt8) Nat Bool)
        (Decoded (List UInt8) Nat Bool v next)
        (Decoded (List UInt8) Nat Bool True (Nil UInt8)) holds;
    Cons head tail ↦ λih. λv. λnext. λholds.
      trans (DecoderResult (List UInt8) Nat Bool)
        (recur (Nil UInt8))
        (Decoded (List UInt8) Nat Bool True (Nil UInt8))
        (Decoded (List UInt8) Nat Bool v next)
        (ih (Nil UInt8) True (Nil UInt8) (sample_nil_lt_cons head tail) sample_spec_empty)
        (sym (DecoderResult (List UInt8) Nat Bool)
          (Decoded (List UInt8) Nat Bool v next)
          (Decoded (List UInt8) Nat Bool True (Nil UInt8)) holds)
  }

const printed_input : Bytes = bytes_encode "Y"
const printed_cursor : List UInt8 = bytes_to_list printed_input

theorem printed_input_decodes : Equal (DecoderResult (List UInt8) Nat Bool)
    (decoder_recursive (List UInt8) UInt8 Nat Bool sample_ops sample_layer printed_cursor)
    (Decoded (List UInt8) Nat Bool True (Nil UInt8)) =
  decoder_recursive_succeeds (List UInt8) UInt8 Nat Bool sample_ops sample_layer
    sample_spec sample_step
    printed_cursor True (Nil UInt8) sample_spec_empty
"#,
    )
    .expect("checked client derives Decoded using the public theorem");

    let after_client: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before_client, after_client, "client adds no new trust");
    assert!(env.globals.contains_key("printed_input_decodes"));
}
