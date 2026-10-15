# JSON token serialization with a proved round trip

> **Status: DRAFT v0 (SPEC-SERIALIZATION-ROUND-TRIP-CONTRACT).** Section 1
> normatively specifies a pure JSON token codec for a prospective ordinary
> Ken package, with four laws proved in Architect D0. It does not
> claim that the codec, a conformance seed or a JSON character/byte codec
> has landed, that CI ran, or that a `trusted_base()` delta was measured.

## 1. JSON token codec

### 1.1 Carrier and package boundary

The catalog Section is `Data.Serialization`. Its module
`Data.Serialization.JsonCodec`, at
`catalog/packages/Data/Serialization/JsonCodec.ken.md`, operates on the
**existing** `Data.Serialization.Json.Json` value carrier. It does not
redeclare that carrier or introduce a smaller variant. All six of its
constructors are in scope:

| Existing `Json` constructor | Stored payload |
|---|---|
| `JsonNull` | none |
| `JsonBool` | `Bool` |
| `JsonNumber` | `Int` |
| `JsonString` | `String` |
| `JsonArray` | `List Json` |
| `JsonObject` | `List (Pair String Json)` |

`JsonNumber` covers the carrier's **integral-number** JSON subset, not
fractional or exponent number spellings. An object is an ordered list of
key/value pairs, not a map. Its keys, and the number and string leaf
payloads, remain **opaque** to this codec: encoding copies them into
tokens and decoding copies them back without comparing, parsing,
normalizing or converting them to characters. The existing `Json`
constructors are already exported from their owning package. The
codec reuses the canonical `List`, `Pair`, `Result`, `String` and `Int`
identities rather than introducing replacement carriers.

### 1.2 Tokens, typed errors and decoding

The public token alphabet and public error type are ordinary Ken
inductives. A package client must be able to construct tokens and
inspect typed errors, so the constructors require **explicit exports**
in addition to the types (`../30-surface/33 §4.1`):

```ken
pub data JsonToken : Type where {
  TokNull : JsonToken;
  TokBool : Bool → JsonToken;
  TokNumber : Int → JsonToken;
  TokString : String → JsonToken;
  TokArrayOpen : JsonToken;
  TokArrayClose : JsonToken;
  TokObjectOpen : JsonToken;
  TokObjectClose : JsonToken;
  TokKey : String → JsonToken
}
export TokNull, TokBool, TokNumber, TokString, TokArrayOpen,
  TokArrayClose, TokObjectOpen, TokObjectClose, TokKey

pub data JsonError : Type where {
  UnexpectedEnd : JsonError;
  UnexpectedToken : JsonToken → JsonError;
  MissingKey : JsonError;
  TrailingToken : JsonToken → JsonError
}
export UnexpectedEnd, UnexpectedToken, MissingKey, TrailingToken

pub fn json_encode (value : Json) : List JsonToken
pub fn json_decode (tokens : List JsonToken) : Result JsonError Json
```

`json_encode` is a nested structural fold over `Json` and its child
lists. A null, Boolean, number or string value encodes as its one
corresponding token. An array encodes as `TokArrayOpen`, the
encodings of its elements **in list order**, then `TokArrayClose`.
An object encodes as `TokObjectOpen`, then for each member in list
order a `TokKey key` immediately followed by the value's encoding,
then `TokObjectClose`. A key is a `TokKey`, not a `TokString` value.
`TokArrayOpen`, `TokArrayClose`, `TokObjectOpen` and `TokObjectClose`
make comma and colon tokens unnecessary: structure determines where
separators would occur. Whitespace, separators, quoting and escaping
belong to the deferred character layer, not this token alphabet.

`json_decode` reads the token list **structurally**, maintaining a
private stack of array, object, and object-key-pending frames. An
array frame collects completed values; an object frame collects
completed key/value pairs; a key-pending frame holds the key until
its value completes. Collection appends to the accumulator rather
than prepending, so the decoder preserves **element order, member
order and duplicate object keys**. It does not silently discard,
merge or reject a repeated key. No map or string-equality operation
is part of the token codec.

A `TokKey` is accepted only where an object expects a member key;
before a root completes, a key token anywhere else returns
`Err (UnexpectedToken (TokKey key))`. An object value completed
without a preceding key returns `Err MissingKey`. The corresponding
close token completes an array or object frame; a close token with
no matching frame, the wrong kind of frame, or an object key still
awaiting its value returns `Err (UnexpectedToken token)`.
If the input ends before a complete root value **and no earlier error
has been reported**, decoding returns `Err UnexpectedEnd`; empty input
and an unfinished array are examples. Once a root value completes,
any unconsumed next token produces `Err (TrailingToken token)` rather
than accepting a prefix and ignoring the suffix. Error positions are
**not** part of `JsonError`.
The internal frame and step carriers are private implementation
machinery, not additional public interface types or trust entries.

### 1.3 Four proved laws

`Data.Serialization.JsonCodec` must export **checked proof bodies**, not
postulates or proposed goals, for exactly these four laws. The
empty-input theorem is closed; the other three quantify over their
stated values or suffixes. `Equal` compares the displayed
`Result JsonError Json` values. The round trip holds for every value
of the full six-constructor carrier. The trailing-input law holds
for **every** first extra token and suffix:

```ken
pub theorem json_decode_encode (value : Json) :
  Equal (Result JsonError Json)
    (json_decode (json_encode value))
    (Ok JsonError Json value)

pub theorem json_decode_trailing
  (value : Json) (extra : JsonToken) (more : List JsonToken) :
  Equal (Result JsonError Json)
    (json_decode
      (list_append JsonToken (json_encode value)
        (Cons JsonToken extra more)))
    (Err JsonError Json (TrailingToken extra))

pub theorem json_decode_empty :
  Equal (Result JsonError Json)
    (json_decode (Nil JsonToken))
    (Err JsonError Json UnexpectedEnd)

pub theorem json_decode_stray_close (more : List JsonToken) :
  Equal (Result JsonError Json)
    (json_decode (Cons JsonToken TokArrayClose more))
    (Err JsonError Json (UnexpectedToken TokArrayClose))
```

`json_decode_trailing` is the proved **named malformed class**: the
codec refuses trailing input with its first extra token carried in
the typed error, regardless of the remaining suffix. The empty and
stray-close theorems prove two further, distinct typed refusals.
None of these statements asserts the correct error and position for
every malformed token sequence. Closed examples of each constructor
and malformed input test the exported operations, but do not replace
the generic proof obligations. In particular, checking only arrays
and objects with empty contents would not pin order or preservation
of nested values.

The D0 proof uses **bounded induction on the encoded token list's
length**, not an axiom about `String` or `Int` and not a decoder
runtime fuel. Its main internal lemma relates running an encoded
value followed by any suffix on any stack to completing that value
and resuming the suffix. The strict structural premise bounds
`TokNull :: json_encode value`; array and object item lemmas recurse
on their respective member lists with a smaller bound. The top-level
law chooses `Suc (token_count (json_encode value))`, using the
existing `Data.Collections.Derived` proofs of `list_append`
associativity and right identity, and `Core.Logic.Transport`'s
`cong`, `sym` and `trans`. These are proof dependencies and a
feasibility account, **not additional public laws**. The token
decoder's runtime recursion consumes one token per step. Unlike the
location-carrying `Capability.Parsing.Decoder`, token decoding uses no
cursor, `DecoderFuelExhausted`, `DecoderZeroProgress`, or location
premise. The existing `Json` carrier's `char_cursor_ops` export is
unchanged; `Capability.Parsing.Cursor` and `Decoder` remain appropriate
for the deferred text-to-token layer.

### 1.4 Trust, delivery and deferred scope

Architect's D0 development on base `d61591c10` checked the four
generic proofs and positive/negative scratch seeds, with no
`axiom`, `primitive`, `postulate` or `opaque` declarations in that
development; the four targeted encoder/decoder mutations and three
individually enabled negative seed goals were reported to reject.
These are **Architect-reported scratch results**, not a published
`JsonCodec` catalog module, an executed shipped conformance suite,
CI, or a measured zero `trusted_base()` delta. The codec adds no
kernel rule or primitive, and its proof does not inspect the opaque
`String` or `Int` fields. Future catalog delivery must retain the
four generic proof bodies instead of replacing them with `Axiom`s.

Derivation of generic `Encode`/`Decode` classes is deferred; §1 is an
explicit `Json` token codec, not a derived class instance. The
JSON character and byte layer is deferred, including text lexing,
separators and whitespace, quoted-string escaping, UTF-8,
number-text conversion, and any character/byte round trip. The
separate `String`/`Bytes` primitive contract remains in
`../30-surface/38 §1.5`; §1 does not replace it. Those facts are not
smuggled into a zero-trust token law: the existing
`string_to_list_char_retraction` is an axiom, the `ord_int_*` order
laws are axioms, and `bytes_decode_encode` is an axiom. This section
does not re-prove or import those facts as an unstated dependency.
The existing `Json` carrier itself covers only integral numbers.

Duplicate-key **rejection** is deferred. The token decoder instead
preserves ordered members, even repeated keys. There is no native
String-equality primitive here. A `DecEq String` instance exists, but
its comparison goes through `string_to_list_char` and its proof of
soundness depends on the axiom-backed string bijection; using it would
not prove this codec's zero-assumption claim. D0 also observed that
`eq_int x x` does not reduce for an open variable, so an Int-based
shortcut cannot supply a transparent comparison either. The present
`Data.Serialization.Json` design note says a decoder will reject
duplicate keys; that prospective sentence is **not** §1's behavior.
Its correction to match this contract is an explicit obligation for
the catalog follow-on and is outside this spec-only author slice.

Merkle hashing, protocol and supply-chain work coordinated with
Lane B, source/error positions, and a truncation-prefix law are
also deferred. Nothing here claims a JSON text parser, complete
malformed-input classification, canonical encoding over byte
strings, or a second equality on `Json` values.
