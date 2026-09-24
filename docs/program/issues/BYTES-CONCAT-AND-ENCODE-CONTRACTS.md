---
id: BYTES-CONCAT-AND-ENCODE-CONTRACTS
title: "register four trusted byte contracts -- F1 bytes_concat list view, F2 one whole-ASCII-string encoding equation, F3 bytes_decode (bytes_encode s) = Ok s, F4' AsciiBytes bs -> IsUtf8 bs -- over a checked AllAscii witness built on KERNEL-LITERAL-CHAR-VIEW, so any client can reason from explicit ASCII literals; then finish CAT-PARSING-LAWS' printer round trip with them"
status: draft
owner: foundation
size: M
gate: architect
tier: T1
depends_on: [CAT-PARSING-LAWS, KERNEL-LITERAL-CHAR-VIEW]
blocks: []
github: null
origin: "Operator ruling 2026-09-23, concurring with Steward recommendation evt_2tn395xd481ar: yes to the concatenation fact and to the narrowest encoding fact. Operator added: 'Ultimately, we will need proofs about UTF-8. Whether or not those are stated as facts depends on whether or not the properties of UTF-8 are taken from Rust in the implementation or in ken source.' TCB growth admitted by that ruling. Need raised by foundation-leader evt_1bkphsn9w1x3q (CAT-PARSING-LAWS hard-stop row 1). Steward-filed per COORDINATION section 2. AMENDED by operator 2026-09-24 ~03:40Z, 'concur with rec.': the Architect general design evt_1jyhkq1pfdnmc -- K3 (filed as KERNEL-LITERAL-CHAR-VIEW), F4' replacing the false fact 4 (evt_1jn0htjgamn7b), and four trusted postulates. Boundary and sequencing: Architect evt_15ce7ened9hxz."
---

# Clients need general ASCII byte facts

## Objective

Any client can prove byte-level facts about an explicit ASCII literal it
writes, and the Parsing printer round trip closes on those facts. Operator
2026-09-23: the facts must serve "the general case of language users needing
to reason from explicit ascii strings", not name particular tokens.

## Fixed inputs

- `spec/30-surface/37-strings-collections.md §2.6` registers two byte-view
  postulates, `bytes_list_roundtrip` and `list_bytes_roundtrip`. Primitives
  compute in the interpreter but are opaque to kernel conversion.
- `spec/30-surface/38-ffi-io.md §1.4` makes `bytes_encode` total and UTF-8 by
  contract. `Source.IsUtf8` requires decode/re-encode byte identity, so UTF-8
  validity is not closed under concatenation (Architect `evt_1jn0htjgamn7b`).
- `CAT-PARSING-LAWS` stopped its printer round trip on these facts (hard-stop
  row 1).
- Placement (Architect `evt_5radqzwbmcazf`): all facts are named source
  axioms in one late catalog module (for example
  `Data.Binary.BytesPrimitiveContracts`) that imports `Derived`
  `list_append`. `bytes.rs` is untouched. No Parsing import.
- `KERNEL-LITERAL-CHAR-VIEW` supplies the literal view and the checked
  finite `AllAsciiCodes` witness. **This node starts after it lands**
  (Architect `evt_15ce7ened9hxz`). Spec may draft prose in parallel, but the
  normative F2/F4' types and the ledger are committed against the landed
  mechanism.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverables

- **D2, Spec enclave.** Spec text and registration for exactly four trusted
  postulates, each one equation or implication:
  - **F1.** `bytes_to_list (bytes_concat a b) = list_append UInt8
    (bytes_to_list a) (bytes_to_list b)`.
  - **F2.** For `s` with an `AllAscii (string_to_list_char s)` witness,
    `map uint8_to_int (bytes_to_list (bytes_encode s)) =
    map charToInt (string_to_list_char s)`. One whole-string equation; it
    subsumes per-character facts and encode-over-append.
  - **F3.** `bytes_decode (bytes_encode s) = Ok s`, with a real inhabitant.
  - **F4'.** `AsciiBytes bs -> IsUtf8 bs`, where `AsciiBytes bs` is
    `AllAsciiCodes (map uint8_to_int (bytes_to_list bs))`.

  The shared `AllAscii` / `AsciiBytes` predicates have one home below
  Parsing. The ledger lists exactly these four.
- **D3, Foundation.** Finish the printer round trip in `Parsing.ken.md` with
  them.

## Acceptance

- **AC-1.** Each fact is stated in the spec and exercised against its
  primitive by a test. The handback lists every trusted item added; nothing
  else is added.
- **AC-2 (delivery test).** A generic client outside Parsing derives the
  byte-level theorem for a fresh ASCII literal absent from the printer, using
  only these facts and the K3 witness.
- **AC-3.** The round trip is unconditional on anything outside the named
  facts. Removing any one fact makes a proof that uses it fail to check.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- Spec text belongs to the Spec enclave. If no spec seat is available when
  D2 starts, STOP and return to the Steward.
- A fifth trusted fact, a general UTF-8 model, a new primitive or a change to
  the `Source` contract is a STOP back to the operator.
