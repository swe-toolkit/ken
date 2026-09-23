---
id: BYTES-CONCAT-AND-ENCODE-CONTRACTS
title: "state the byte-list view of bytes_concat as a registered contract beside the two existing Bytes/List UInt8 round trips, settle where Ken's UTF-8 encoding properties come from (a Rust primitive with stated contracts, or Ken source with proofs), supply the narrowest encoding fact the Boolean printer needs on that basis, and finish CAT-PARSING-LAWS' printer round trip with them"
status: ready
owner: foundation
size: M
gate: architect
tier: T1
depends_on: [CAT-PARSING-LAWS]
blocks: []
github: null
origin: "Operator ruling 2026-09-23, concurring with Steward recommendation evt_2tn395xd481ar: yes to the concatenation fact and to the narrowest encoding fact. Operator added: 'Ultimately, we will need proofs about UTF-8. Whether or not those are stated as facts depends on whether or not the properties of UTF-8 are taken from Rust in the implementation or in ken source.' TCB growth admitted by that ruling. Need raised by foundation-leader evt_1bkphsn9w1x3q (CAT-PARSING-LAWS hard-stop row 1). Steward-filed per COORDINATION section 2."
---

# The printer round trip needs two byte facts

## Settled inputs

- `CAT-PARSING-LAWS` stopped its printer round trip on two facts that no
  contract states (hard-stop row 1):
  1. `bytes_to_list (bytes_concat a b) = list_append UInt8 (bytes_to_list a)
     (bytes_to_list b)`;
  2. the byte list that `bytes_encode s` produces.
- `spec/30-surface/37-strings-collections.md §2.6` registers exactly two
  byte-view postulates, `bytes_list_roundtrip` and `list_bytes_roundtrip`, as
  "a deliberate fixed trust cost": primitives compute in the interpreter but
  are opaque to kernel conversion. They are registered in
  `crates/ken-elaborator/src/bytes.rs` (`declare_postulate`, `~:265`).
  `bytes_concat` is a registered primitive in the same file (`~:89`).
- `spec/30-surface/38-ffi-io.md §1.4` makes `bytes_encode` total and UTF-8 by
  contract, but it states only `bytes_decode (bytes_encode s) = Ok s`.
- **Operator:** UTF-8 proofs will be needed in the end. Whether they are
  stated as facts depends on whether UTF-8's properties come from Rust in
  the implementation or from Ken source.

## Deliverables

- **D1, Architect ruling before code: where UTF-8 lives.** Choose one:
  - **(a)** `bytes_encode` stays a Rust primitive, and its encoding
    properties are stated contracts.
  - **(b)** Ken source defines the encoder over `List Char` / `List UInt8`,
    so its properties are proved, and only the existing view postulates
    stay trusted.

  Give the minimal trust each choice leaves, and the effect on the
  interpreter and native backends. The concatenation fact stands in either
  case unless D1 also moves `bytes_concat` into Ken source; say which.
- **D2.** Spec text in `37 §2.6` (and `38 §1.4` if (a)) for each new fact,
  and its registration beside the existing two, or the Ken-source encoder
  and its proofs if (b). For the encoding fact, take **the narrowest form
  the printer needs** (likely ASCII-only), not a full UTF-8 model.
- **D3.** Finish `CAT-PARSING-LAWS`' printer round trip in `Parsing.ken.md`
  with these facts.

## Acceptance criteria

- **AC-1.** Each new trusted fact is one equation, quantified correctly and
  stated in the spec. A test exercises the primitive against each one. The
  handback lists every trusted item added; nothing else is added.
- **AC-2.** The round trip is unconditional on anything outside the named
  facts. Removing either fact makes its proof fail to check.
- **AC-3.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- **Spec text belongs to the Spec enclave.** If no spec seat is available
  when D2 starts, STOP and return to the Steward.
- Under (a), anything beyond the two named facts, such as a general UTF-8
  model or a new primitive, is a STOP back to the operator.
