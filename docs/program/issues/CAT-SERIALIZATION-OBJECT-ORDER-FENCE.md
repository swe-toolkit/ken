---
id: CAT-SERIALIZATION-OBJECT-ORDER-FENCE
title: "Nothing pins the JSON token encoder's object member order, key placement or duplicate retention (spec 71 section 1.2): a consistent reversal passes every law and seed row. Add the closed Refl theorems the codec WP wrongly said could not be written, and correct the Json.ken.md sentence that still puts duplicate-key rejection in decoding"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [CAT-SERIALIZATION-JSON-TOKEN-CODEC]
blocks: []
github: null
origin: "Adversary report on 608e4f922 (evt_3w8g4qwjhg69f), F1 and F2. Architect ruling evt_4f66kxfcpvxch: both real, one S T2 WP, conformance in the diff. Steward-filed per COORDINATION section 2."
---

# The JSON token fence pins object order

## Objective

The conformance fence for `Data.Serialization.Json` tells an encoder that
emits object members in list order from one that reorders them, and the
module's prose agrees with spec 71 on repeated keys.

## Settled inputs (Architect `evt_4f66kxfcpvxch`, on `608e4f922`)

- **The gap.** Only `encode_array_order` pins order. Object cases go
  through the round trip, which a consistent reversal (encoder reverses
  members, decoder prepends) also satisfies. The seed's
  `expected_number_tokens`, `expected_string_tokens`,
  `expected_object_tokens` and `expected_duplicate_tokens` have no
  consumer.
- **The excuse was false.** The codec WP's limit, that closed goals with
  String or Int leaves cannot be discharged in-language, does not hold on
  `608e4f922`. With the seed's full positive fence, these check (rc 0):
  `encode_number_tokens`, `encode_string_tokens`, `encode_object_order`
  and `encode_duplicate_members`, each
  `Equal (List JsonToken) (json_encode <value>) expected_<name> = Refl`.
  The ruling gives the exact text.
- **Controls refused** (rc 1, TypeMismatch at the row's own `Refl`):
  members reversed, the duplicate dropped to its last member, a wrong key,
  a wrong number, and a string value emitted as `TokKey`.
- **Parse limit.** A String or Int literal cannot appear inline as a
  constructor argument in these type-position vectors. Bind it as a
  `const`, as the seed already does.
- **The module stays at four laws.** Spec 71 §1.3 says "exactly these four
  laws", so the pins are conformance theorems, not module exports.
- **F2.** Json.ken.md:26-27 says duplicate-key rejection belongs to
  decoding, contradicting spec 71 §1.2 and §1.4. The ruling gives the
  replacement sentence.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. `conformance/stdlib/serialization/seed-json-token-codec.md`: the four
   theorems in the positive fence after `encode_array_order`; one negative
   `reject_reversed_members` with its spec/claimed/gap block; the order gap
   sentence and the Coverage-boundary sentence rewritten from black-box
   expectations to checked closed theorems. The "no String comparator"
   remark stays.
2. `Json.ken.md` :26-27, the ruling's replacement sentence.
3. `cat_serialization_json_token_codec.rs`: the four names in
   `complete_seed_positive_client_checks`, `reject_reversed_members` in
   the negative table, and the CLAIMED text at :128-129 true as written.

## Acceptance

- **AC-1.** The four theorems check and `reject_reversed_members` is
  refused with TypeMismatch, in the package's test, with
  `trusted_base()` unchanged.
- **AC-2.** The module still exports exactly the four spec 71 §1.3 laws.
- **AC-3 (mutation, QA).** Changing the seed's `expected_object_tokens` to
  the reversed member order turns `encode_object_order` red with
  TypeMismatch.

## Stop conditions

- A positive needs a new export, a postulate or a comparator.
- Any `trusted_base()` change.
- Spec 71 text changes. Its §1.4 sentence about the present design note
  is a carry for the next spec-71 touch, not this WP.
