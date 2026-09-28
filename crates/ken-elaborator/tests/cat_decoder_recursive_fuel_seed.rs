//! Recursive Decoder fuel on a cursor whose remaining count is the actual list length.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn eval_global(env: &ElabEnv, store: &mut EvalStore, name: &str) -> EvalVal {
    let id = env.globals[name];
    let body = match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => body,
        other => panic!("{name} must be transparent, got {other:?}"),
    };
    eval(&[], body, &env.env, store)
}

fn nat_count(env: &ElabEnv, value: &EvalVal) -> usize {
    match value {
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.zero_id && args.is_empty() => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id && args.len() == 1 => {
            1 + nat_count(env, &args[0])
        }
        other => panic!("expected Nat, got {other:?}"),
    }
}

/// Promise class: durable invariant.
/// MEASURED: the checked honest-length cursor satisfies all three Cursor laws;
/// the reference evaluator returns Decoded True at Nil for three depths of a
/// productive recursive layer and for a layer that never recurses.
/// CLAIMED: decoder_recursive gives the terminal layer one call beyond the
/// strictly decreasing recursive calls, including on exhausted input.
/// THE GAP: these four closed inputs do not prove the property for every cursor
/// or layer; the exported success theorem supplies the checked general proof.
#[test]
fn recursive_seed_reaches_terminal_layer_with_honest_remaining() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Cursor")
        .expect("Cursor provider loads");
    let before_decoder: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], "Capability.Parsing.Decoder")
        .expect("Decoder provider loads");
    let after_decoder: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before_decoder, after_decoder, "Decoder adds no trust");

    env.elaborate_file(
        r#"
import Capability.Parsing.Cursor
  (CursorOps, MkCursorOps, CursorLaws, CursorPeekHasRemaining,
    CursorAdvanceProgress, CursorEndValid, cursor_nat_lt, cursor_remaining)
import Capability.Parsing.Decoder
  (Decoder, DecoderResult, decoder_recursive, decoder_alt, decoder_seq,
    decoder_satisfy, decoder_pure)
import Data.Collections.Derived (length)

fn honest_remaining (cur : List Bool) : Nat = length Bool cur
fn list_peek (cur : List Bool) : Option Bool =
  match cur {
    Nil ↦ None Bool;
    Cons head tail ↦ Some Bool head
  }
fn list_advance (cur : List Bool) : List Bool =
  match cur {
    Nil ↦ Nil Bool;
    Cons head tail ↦ tail
  }
fn list_locate (cur : List Bool) : Nat = length Bool cur
const honest_ops : CursorOps (List Bool) Bool Nat =
  MkCursorOps (List Bool) Bool Nat
    honest_remaining list_peek list_advance list_locate

theorem lt_suc (n : Nat) : Equal Bool (cursor_nat_lt n (Suc n)) True =
  match n { Zero ↦ Proved; Suc rest ↦ lt_suc rest }

theorem honest_peek_has_remaining
    : CursorPeekHasRemaining (List Bool) Bool Nat honest_ops =
  λcur. match cur {
    Nil ↦ λvalue. λpeeked. absurd peeked;
    Cons head tail ↦ λvalue. λpeeked. Proved
  }
theorem honest_advance_progress
    : CursorAdvanceProgress (List Bool) Bool Nat honest_ops =
  λcur. match cur {
    Nil ↦ λvalue. λpeeked. absurd peeked;
    Cons head tail ↦ λvalue. λpeeked. lt_suc (length Bool tail)
  }
theorem honest_end_valid : CursorEndValid (List Bool) Bool Nat honest_ops =
  λcur. match cur {
    Nil ↦ λempty. Proved;
    Cons head tail ↦ λempty. absurd empty
  }
theorem honest_laws : CursorLaws (List Bool) Bool Nat honest_ops =
  and_intro
    (CursorPeekHasRemaining (List Bool) Bool Nat honest_ops)
    (And (CursorAdvanceProgress (List Bool) Bool Nat honest_ops)
      (CursorEndValid (List Bool) Bool Nat honest_ops))
    honest_peek_has_remaining
    (and_intro
      (CursorAdvanceProgress (List Bool) Bool Nat honest_ops)
      (CursorEndValid (List Bool) Bool Nat honest_ops)
      honest_advance_progress honest_end_valid)

fn repeat_layer (recur : Decoder (List Bool) Nat Bool) : Decoder (List Bool) Nat Bool =
  decoder_alt (List Bool) Nat Bool
    (decoder_seq (List Bool) Nat Bool Bool
      (decoder_satisfy (List Bool) Bool Nat honest_ops (λignored. True))
      recur)
    (decoder_pure (List Bool) Nat Bool True)

fn pure_layer (recur : Decoder (List Bool) Nat Bool) : Decoder (List Bool) Nat Bool =
  decoder_pure (List Bool) Nat Bool True

const zero_input : List Bool = Nil Bool
const one_input : List Bool = Cons Bool True (Nil Bool)
const two_input : List Bool = Cons Bool True (Cons Bool False (Nil Bool))
const remaining_zero : Nat = cursor_remaining (List Bool) Bool Nat honest_ops zero_input
const remaining_one : Nat = cursor_remaining (List Bool) Bool Nat honest_ops one_input
const remaining_two : Nat = cursor_remaining (List Bool) Bool Nat honest_ops two_input
const honest_zero : DecoderResult (List Bool) Nat Bool =
  decoder_recursive (List Bool) Bool Nat Bool honest_ops repeat_layer zero_input
const honest_one : DecoderResult (List Bool) Nat Bool =
  decoder_recursive (List Bool) Bool Nat Bool honest_ops repeat_layer one_input
const honest_two : DecoderResult (List Bool) Nat Bool =
  decoder_recursive (List Bool) Bool Nat Bool honest_ops repeat_layer two_input
const honest_pure_empty : DecoderResult (List Bool) Nat Bool =
  decoder_recursive (List Bool) Bool Nat Bool honest_ops pure_layer zero_input
"#,
    )
    .expect("honest cursor laws and four recursive results are kernel-checked");

    let laws = env.globals["honest_laws"];
    assert!(
        matches!(env.env.lookup(laws), Some(Decl::Transparent { .. })),
        "the honest cursor's three laws must be checked rather than assumed"
    );
    let after_fixture: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after_decoder, after_fixture, "fixture adds no trust");

    let mut store = EvalStore::new();
    for (name, expected) in [
        ("remaining_zero", 0),
        ("remaining_one", 1),
        ("remaining_two", 2),
    ] {
        let value = eval_global(&env, &mut store, name);
        assert_eq!(
            nat_count(&env, &value),
            expected,
            "{name}: remaining must equal input length"
        );
    }
    let decoded_id = env.globals["Capability.Parsing.Decoder.Decoded"];
    let failed_id = env.globals["Capability.Parsing.Decoder.DecoderFailed"];
    let exhausted_id = env.globals["Capability.Parsing.Decoder.DecoderFuelExhausted"];
    let true_id = env.globals["True"];
    let nil_id = env.prelude_env.nil_id;
    for name in [
        "honest_zero",
        "honest_one",
        "honest_two",
        "honest_pure_empty",
    ] {
        let result = eval_global(&env, &mut store, name);
        let EvalVal::Ctor { id, args, .. } = &result else {
            panic!("{name}: expected a decoder result, got {result:?}");
        };
        if *id == failed_id {
            let fuel_exhausted =
                matches!(&args[3], EvalVal::Ctor { id, .. } if *id == exhausted_id);
            panic!("{name}: unexpected decoder failure (fuel exhausted: {fuel_exhausted})");
        }
        assert_eq!(*id, decoded_id, "{name}: expected Decoded, got {result:?}");
        assert!(
            matches!(&args[3], EvalVal::Ctor { id, .. } if *id == true_id),
            "{name}: result is not True: {result:?}"
        );
        assert!(
            matches!(&args[4], EvalVal::Ctor { id, .. } if *id == nil_id),
            "{name}: input was not fully consumed: {result:?}"
        );
    }
}
