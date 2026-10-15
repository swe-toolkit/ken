# JSON token codec — seed cases

Format: `../../README.md`.
Spec: `spec/50-stdlib/71-serialization.md` §1.

These authored source-shaped cases target the prospective
`Data.Serialization.JsonCodec` module. They cover all six `Json`
constructors, ordered array and object token output, duplicate object
members, the four generic laws, and typed malformed-input results. A
closed law application exercises a public theorem; it does not replace
its generic proof.

Architect D0 `evt_yttamsdpp9nh` reports that its development and seed
controls checked in scratch. That is Architect-reported evidence, not
execution of this seed against a shipped module. The codec is not
claimed as a catalog delivery, and no Cargo test, CI result, publication,
or `trusted_base()` measurement is claimed here.

## Shared fixtures and positive assertions

The number and string leaves are bound as constants. The ordered object
has two differently named keys with distinct values; the duplicate-key
object has two same-key members with distinct values. The nested value
places an object and string inside a nonempty array. Opaque payloads are
copied, not parsed, formatted, normalized or compared by the codec.

```ken
import Data.Serialization.Json
  (Json, JsonNull, JsonBool, JsonNumber, JsonString, JsonArray, JsonObject)
import Data.Serialization.JsonCodec
  (JsonToken, TokNull, TokBool, TokNumber, TokString, TokArrayOpen,
   TokArrayClose, TokObjectOpen, TokObjectClose, TokKey, JsonError,
   UnexpectedEnd, UnexpectedToken, MissingKey, TrailingToken,
   json_encode, json_decode, json_decode_encode, json_decode_trailing,
   json_decode_empty, json_decode_stray_close)
import Data.Collections.Derived (list_append)

const number_payload : Int = 7
const text_payload : String = "a"
const number_value : Json = JsonNumber number_payload
const string_value : Json = JsonString text_payload
const first_key : String = "first"
const second_key : String = "second"
const repeated_key : String = "same"

const ordered_array : Json =
  JsonArray (Cons Json JsonNull (Cons Json (JsonBool False) (Nil Json)))
const reversed_array : Json =
  JsonArray (Cons Json (JsonBool False) (Cons Json JsonNull (Nil Json)))
const ordered_object : Json =
  JsonObject
    (Cons (Pair String Json)
      (mk_pair String Json first_key (JsonBool True))
      (Cons (Pair String Json)
        (mk_pair String Json second_key (JsonBool False))
        (Nil (Pair String Json))))
const duplicate_object : Json =
  JsonObject
    (Cons (Pair String Json)
      (mk_pair String Json repeated_key (JsonBool True))
      (Cons (Pair String Json)
        (mk_pair String Json repeated_key (JsonBool False))
        (Nil (Pair String Json))))
const nested_value : Json =
  JsonArray (Cons Json ordered_object (Cons Json string_value (Nil Json)))

const expected_number_tokens : List JsonToken =
  Cons JsonToken (TokNumber number_payload) (Nil JsonToken)
const expected_string_tokens : List JsonToken =
  Cons JsonToken (TokString text_payload) (Nil JsonToken)
const expected_array_tokens : List JsonToken =
  Cons JsonToken TokArrayOpen
    (Cons JsonToken TokNull
      (Cons JsonToken (TokBool False)
        (Cons JsonToken TokArrayClose (Nil JsonToken))))
const expected_object_tokens : List JsonToken =
  Cons JsonToken TokObjectOpen
    (Cons JsonToken (TokKey first_key)
      (Cons JsonToken (TokBool True)
        (Cons JsonToken (TokKey second_key)
          (Cons JsonToken (TokBool False)
            (Cons JsonToken TokObjectClose (Nil JsonToken))))))
const expected_duplicate_tokens : List JsonToken =
  Cons JsonToken TokObjectOpen
    (Cons JsonToken (TokKey repeated_key)
      (Cons JsonToken (TokBool True)
        (Cons JsonToken (TokKey repeated_key)
          (Cons JsonToken (TokBool False)
            (Cons JsonToken TokObjectClose (Nil JsonToken))))))

theorem round_trip_null :
  Equal (Result JsonError Json) (json_decode (json_encode JsonNull))
    (Ok JsonError Json JsonNull) = Proved

theorem round_trip_bool :
  Equal (Result JsonError Json) (json_decode (json_encode (JsonBool True)))
    (Ok JsonError Json (JsonBool True)) = Proved

theorem round_trip_number :
  Equal (Result JsonError Json) (json_decode (json_encode number_value))
    (Ok JsonError Json number_value) = json_decode_encode number_value

theorem round_trip_string :
  Equal (Result JsonError Json) (json_decode (json_encode string_value))
    (Ok JsonError Json string_value) = json_decode_encode string_value

theorem round_trip_array :
  Equal (Result JsonError Json) (json_decode (json_encode ordered_array))
    (Ok JsonError Json ordered_array) = json_decode_encode ordered_array

theorem round_trip_ordered_object :
  Equal (Result JsonError Json) (json_decode (json_encode ordered_object))
    (Ok JsonError Json ordered_object) = json_decode_encode ordered_object

theorem round_trip_duplicate_object :
  Equal (Result JsonError Json) (json_decode (json_encode duplicate_object))
    (Ok JsonError Json duplicate_object) = json_decode_encode duplicate_object

theorem round_trip_nested :
  Equal (Result JsonError Json) (json_decode (json_encode nested_value))
    (Ok JsonError Json nested_value) = json_decode_encode nested_value

theorem encode_array_order :
  Equal (List JsonToken) (json_encode ordered_array) expected_array_tokens = Proved

theorem law_trailing_first_extra :
  Equal (Result JsonError Json)
    (json_decode
      (list_append JsonToken (json_encode JsonNull)
        (Cons JsonToken TokArrayOpen
          (Cons JsonToken (TokBool False) (Nil JsonToken)))))
    (Err JsonError Json (TrailingToken TokArrayOpen)) =
  json_decode_trailing JsonNull TokArrayOpen
    (Cons JsonToken (TokBool False) (Nil JsonToken))

theorem law_empty :
  Equal (Result JsonError Json) (json_decode (Nil JsonToken))
    (Err JsonError Json UnexpectedEnd) = json_decode_empty

theorem law_stray_close :
  Equal (Result JsonError Json)
    (json_decode (Cons JsonToken TokArrayClose
      (Cons JsonToken TokNull (Nil JsonToken))))
    (Err JsonError Json (UnexpectedToken TokArrayClose)) =
  json_decode_stray_close (Cons JsonToken TokNull (Nil JsonToken))

theorem malformed_missing_key :
  Equal (Result JsonError Json)
    (json_decode
      (Cons JsonToken TokObjectOpen
        (Cons JsonToken TokNull
          (Cons JsonToken TokObjectClose (Nil JsonToken)))))
    (Err JsonError Json MissingKey) = Proved

theorem malformed_truncated_array :
  Equal (Result JsonError Json)
    (json_decode (Cons JsonToken TokArrayOpen (Nil JsonToken)))
    (Err JsonError Json UnexpectedEnd) = Proved

theorem malformed_mismatched_close :
  Equal (Result JsonError Json)
    (json_decode
      (Cons JsonToken TokArrayOpen
        (Cons JsonToken TokObjectClose (Nil JsonToken))))
    (Err JsonError Json (UnexpectedToken TokObjectClose)) = Proved
```

### Round trips cover every constructor

- spec: `spec/50-stdlib/71-serialization.md` §§1.1–1.3.
- promise class: durable invariant.
- given: a value for each scalar and collection constructor, plus a
  nested array/object value and an object with duplicate keys.
- expect: each `json_decode_encode` use site has its stated `Equal`
  result; null and Boolean cases also reduce directly.
- measured: six constructor forms, nonempty nested collections, two
  ordered members, and duplicate member retention.
- claimed: the public generic theorem applies to representative values
  of the full carrier; order and duplicates remain represented in the
  round-trip input.
- gap: these closed applications do not prove the universal theorem or
  independently validate its proof body.
- why: each application retains a nonempty structural case, and the two
  object values make order and duplicate behavior part of the result.

### Token output preserves array and object order

- spec: `spec/50-stdlib/71-serialization.md` §1.2.
- promise class: durable invariant.
- given: `ordered_array`, `ordered_object`, and `duplicate_object`.
- expect: scalar outputs equal their one-token
  `expected_number_tokens` and `expected_string_tokens`; the array
  output equals `expected_array_tokens`. Object outputs equal
  `expected_object_tokens` and `expected_duplicate_tokens`.
- measured: exact scalar and array token lists, plus object token
  vectors with distinct member values and a repeated key.
- claimed: arrays and object members are emitted in list order, each
  key precedes its value, and duplicate keys are not removed.
- gap: the number, string and object vectors are source-shaped
  black-box expectations; this seed does not add a String comparator
  or claim an independent key-equality oracle. The closed array equation
  has no opaque payloads.
- why: reversing either object member moves its distinct Boolean value;
  deduplicating the repeated key changes the token count.

### Typed malformed-input results

- spec: `spec/50-stdlib/71-serialization.md` §§1.2–1.3.
- promise class: durable invariant.
- given: empty input, an unfinished array, an object value without a
  preceding key, a stray array close, and an array closed by an object
  token.
- expect: `UnexpectedEnd`, `MissingKey`, and
  `UnexpectedToken TokObjectClose` at the named boundaries; the generic
  empty and stray-close theorem instances return their specified errors.
- measured: distinct malformed classes with typed `JsonError` values.
- claimed: these cases do not collapse missing keys, truncation, or a
  mismatched close into success or one undifferentiated error.
- gap: the cases do not classify every malformed token sequence or pin
  error positions, which §1 defers.
- why: each false-success or wrong-error result differs from a positive
  or law-backed expected constructor.

## False claims, appended one at a time

Each following theorem is intended to be checked alone as an addition to
the unchanged positive client. The expected result is refusal of the false
proposition, not a runtime rejection of its input.

### A reversed array is not the round-trip result

- spec: `spec/50-stdlib/71-serialization.md` §§1.2–1.3.
- promise class: durable invariant.
- given: `ordered_array` and the distinct `reversed_array`.
- expect: the following false result claim is refused.
- measured: the claim swaps the two existing elements and changes no
  other fixture.
- claimed: array element order is preserved.
- gap: this instance does not prove the generic theorem.
- why: the positive round-trip application uses the same input and the
  direct encoder oracle fixes its token order.

```ken
-- Refused: the expected array reverses the two input elements.
theorem reject_reversed_array :
  Equal (Result JsonError Json) (json_decode (json_encode ordered_array))
    (Ok JsonError Json reversed_array) = Proved
```

### Trailing input is not accepted as a prefix

- spec: `spec/50-stdlib/71-serialization.md` §1.2.
- promise class: durable invariant.
- given: `[TokNull, TokNull]`, whose first token completes a root.
- expect: the following claim that decoding returns `Ok JsonNull` is
  refused; the positive `law_trailing_first_extra` instead yields a
  typed `TrailingToken` for the first extra token.
- measured: one complete root followed by one extra token.
- claimed: the decoder does not silently ignore a suffix.
- gap: this case does not classify the remaining malformed-input space.
- why: a prefix-accepting decoder contradicts the positive typed error.

```ken
-- Refused: decoding must report the first token after the completed root.
theorem reject_trailing_prefix :
  Equal (Result JsonError Json)
    (json_decode
      (Cons JsonToken TokNull (Cons JsonToken TokNull (Nil JsonToken))))
    (Ok JsonError Json JsonNull) = Proved
```

### A keyless object value is not accepted

- spec: `spec/50-stdlib/71-serialization.md` §1.2.
- promise class: durable invariant.
- given: `[TokObjectOpen, TokNull, TokObjectClose]`.
- expect: the following claim that this decodes to an empty object is
  refused; the positive `malformed_missing_key` reports `MissingKey`.
- measured: one completed value inside an object without a preceding
  `TokKey`.
- claimed: object values require a pending key.
- gap: this case does not classify every malformed object sequence.
- why: accepting the empty object would discard the supplied value.

```ken
-- Refused: the object value has no preceding key.
theorem reject_keyless_object_value :
  Equal (Result JsonError Json)
    (json_decode
      (Cons JsonToken TokObjectOpen
        (Cons JsonToken TokNull
          (Cons JsonToken TokObjectClose (Nil JsonToken)))))
    (Ok JsonError Json (JsonObject (Nil (Pair String Json)))) = Proved
```

## Coverage boundary

The seed exercises each constructor, one round-trip assertion per
constructor, a nested value, ordered and repeated object members,
the four public law interfaces, and representative typed refusals.
The scalar, array and ordered-object vectors state exact expected
tokens and key placement without introducing trusted String comparison.
The closed array equation has no opaque payloads. These cases do not
establish the D0 proof bodies, a shipped
module or seed, JSON text/byte behavior, complete malformed-input
classification, a measured trusted-base delta, CI, or publication.
