//! JSON token codec clients: spec/50-stdlib/71-serialization.md §1.
//! Source: conformance/stdlib/serialization/seed-json-token-codec.md.
//! Promise class: durable invariant. Exact seed token/error vectors pin the
//! contract's order and refusal behavior; generic client laws pin the types.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const CODEC: &str = "Data.Serialization.JsonCodec";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/serialization/seed-json-token-codec.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn codec_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], CODEC)
        .expect("JsonCodec roots-loads from its checked providers");
    env
}

// Checked Ken fences are executable seed programs, not repository prose
// being counted or asserted. The first is the positive client; each later
// fence contains one independent false theorem.
fn seed_programs() -> Vec<&'static str> {
    SEED.split("\n```ken\n")
        .skip(1)
        .map(|fence| fence.split_once("\n```").expect("closed Ken fence").0)
        .collect()
}

/// SOURCE: spec/50-stdlib/71-serialization.md §§1.1–1.3.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact trusted-GlobalId inventories after loading the explicit
/// provider closure and then the codec coincide; all four owned theorem
/// globals have transparent bodies;
/// four independent generic client statements type-check with public imports.
/// CLAIMED: the complete six-constructor Json carrier has the four proved
/// generic laws at zero additional trust.
/// THE GAP: inherited provider trust remains, and exact operational tokens
/// and errors are separately checked against the seed client below.
#[test]
fn generic_laws_check_and_loading_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Data.Serialization.Json",
        "Data.Collections.Derived",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], CODEC)
        .expect("codec and generic proof bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(
        after, before,
        "codec adds no trusted GlobalId beyond its provider closure"
    );
    for law in [
        "json_decode_encode",
        "json_decode_trailing",
        "json_decode_empty",
        "json_decode_stray_close",
    ] {
        let id = env.globals[&format!("{CODEC}.{law}")];
        assert!(owned.contains(&id), "{law} must be owned by the codec");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} must carry a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("four independently stated law types check from client imports");
}

const GENERIC_CLIENT: &str = r#"
import Data.Serialization.Json (Json)
import Data.Serialization.JsonCodec
  (JsonToken, JsonError, TokArrayClose, TrailingToken, UnexpectedEnd,
   UnexpectedToken, json_encode, json_decode, json_decode_encode,
   json_decode_trailing, json_decode_empty, json_decode_stray_close)
import Data.Collections.Derived (list_append)

theorem client_round_trip (value : Json) :
  Equal (Result JsonError Json) (json_decode (json_encode value))
    (Ok JsonError Json value) = json_decode_encode value

theorem client_trailing (value : Json) (extra : JsonToken) (more : List JsonToken) :
  Equal (Result JsonError Json)
    (json_decode
      (list_append JsonToken (json_encode value) (Cons JsonToken extra more)))
    (Err JsonError Json (TrailingToken extra)) =
  json_decode_trailing value extra more

theorem client_empty :
  Equal (Result JsonError Json) (json_decode (Nil JsonToken))
    (Err JsonError Json UnexpectedEnd) = json_decode_empty

theorem client_stray_close (more : List JsonToken) :
  Equal (Result JsonError Json)
    (json_decode (Cons JsonToken TokArrayClose more))
    (Err JsonError Json (UnexpectedToken TokArrayClose)) =
  json_decode_stray_close more
"#;

/// SOURCE: conformance/stdlib/serialization/seed-json-token-codec.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the seed's complete positive Ken fence elaborates through a
/// client import of the public token constructors, errors, operations and
/// laws; the array-order assertion has the proof terminal required by its
/// nonempty-list equality, while scalar and object fixtures remain distinct.
/// CLAIMED: all six constructors, order, repeated keys, nested values and
/// representative typed malformed inputs behave as specified.
/// THE GAP: these finite vectors do not replace generic checked proofs;
/// their imports and bodies are independently checked by the compiler.
#[test]
fn complete_seed_positive_client_checks() {
    let programs = seed_programs();
    let mut env = codec_env();
    env.elaborate_file(programs[0])
        .expect("complete source-shaped positive seed client checks");
    for name in [
        "round_trip_null",
        "round_trip_bool",
        "round_trip_number",
        "round_trip_string",
        "round_trip_array",
        "round_trip_ordered_object",
        "round_trip_duplicate_object",
        "round_trip_nested",
        "encode_array_order",
        "law_trailing_first_extra",
        "law_empty",
        "law_stray_close",
        "malformed_missing_key",
        "malformed_truncated_array",
        "malformed_mismatched_close",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked positive seed theorem"
        );
    }
}

/// SOURCE: conformance/stdlib/serialization/seed-json-token-codec.md refusals.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the same full positive client passes, each seed negative
/// checks alone and kernel-rejects with TypeMismatch inside its own theorem.
/// CLAIMED: the false reversed-array result, accepted trailing prefix and
/// discarded keyless object value cannot be justified by the codec.
/// THE GAP: these three counterexamples do not classify every malformed input.
#[test]
fn each_false_seed_result_is_refused_at_its_own_theorem() {
    let programs = seed_programs();
    for (negative, name) in [
        (programs[1], "reject_reversed_array"),
        (programs[2], "reject_trailing_prefix"),
        (programs[3], "reject_keyless_object_value"),
    ] {
        let mut env = codec_env();
        env.elaborate_file(programs[0])
            .expect("positive seed must check before a negative is submitted");
        match env.elaborate_file(negative) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) => {
                let start = negative.find("theorem ").expect("false theorem");
                assert!(
                    negative[start..].starts_with(&format!("theorem {name}"))
                        && span.start >= start
                        && span.end <= negative.len()
                        && span.start < span.end,
                    "{name}: refusal must target its own false theorem, got {span:?}"
                );
            }
            other => panic!("{name}: expected typed refusal at its theorem, got {other:?}"),
        }
    }
}
