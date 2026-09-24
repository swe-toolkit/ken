---
id: KERNEL-LITERAL-CHAR-VIEW
title: "K3: the kernel reduces string_to_list_char on a checked String literal to its List Char and charToInt on a checked Char literal to its IntLit codepoint, so a generic client can build a checked ASCII witness for a fresh literal; no other primitive gains reduction"
status: ready
owner: kernel
size: M
gate: architect
tier: T1
depends_on: []
blocks: [BYTES-CONCAT-AND-ENCODE-CONTRACTS, CAT-ARGPARSE-LAWS]
github: null
origin: "Operator 2026-09-24 ~03:40Z, 'concur with rec.': authorize K3, the kernel literal bridge, for BYTES (not the reflector alternative). TCB code growth admitted by that ruling. Design boundary and sequencing: Architect evt_15ce7ened9hxz. Steward-filed per COORDINATION section 2."
---

# The kernel cannot see a literal's characters

## Objective

A checked client proves a statement about the characters and codes of any
ASCII literal it writes, including one the catalog has never seen, by kernel
conversion rather than by a trusted fact.

## Fixed inputs -- measured at `1a4495376`

- `elab.rs::elab_str_lit` and `elab_char_lit` mint a fresh
  `PrimReduction::Literal` id per occurrence; the payload lives only in the
  elaborator's `num_values`. The kernel cannot see it.
- `string_to_list_char : String → List Char` is a registered primitive
  (`prelude.rs`, `string_to_list_char_id`). `charToInt (c : Char) : Int = c`
  is derived (`decimal_char.rs`). `conv.rs::whnf_progress` has no reduction
  for either on a literal.
- An elaborator-only rewrite is not proof. The payload the kernel converts
  with must be kernel-checked.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

Two positive kernel conversions, and nothing else:

1. `string_to_list_char` applied to a checked **String literal** reduces to a
   well-typed `List Char` in source scalar order from that literal's
   NFC-normalized content.
2. `charToInt` applied to a checked **Char literal**, including each head
   produced by (1), yields the kernel-visible `IntLit` codepoint.

Constraints from the Architect's boundary (`evt_15ce7ened9hxz`):

- Dispatch by declaration identity or typed registration, never by symbol
  string or an unverified side table. At registration the kernel holds an
  immutable, type-checked literal payload that validates the carrier and
  scalar range. One payload drives kernel conversion and the interpreter and
  native values.
- The reducer mints no unregistered globals during conversion. Pre-register
  the literal-head terms or use a type-preserving core representation; the
  kernel design review picks which.
- If `charToInt` unfolds to the same Char literal, both reduction orders
  agree. No non-confluent shortcut.
- Nonliteral applications stay neutral. No other primitive gains reduction:
  not `bytes_encode`, `bytes_concat`, `bytes_to_list`, `uint8_to_int`,
  `leq_int` or `eq_int`. No new postulate, reflector or primitive, and no
  implicit whole-String equality.
- Record the TCB code growth even if the `trusted_base()` id delta is zero.

The ASCII witness is ordinary checked Ken below Parsing: a finite ASCII tag
type for codes `0..127`, a checked decoder to `Int` literals,
`AsciiCode n := Σ (tag : AsciiTag). Equal Int n (ascii_value tag)` (or a
definitionally equivalent finite family), and a structural
`AllAsciiCodes : List Int → Type`. It is not 128 trusted equations and does
not use integer comparison.

## Acceptance

- **AC-1 (first measurement, fresh literal).** A generic checked client
  derives `AllAsciiCodes (map charToInt (string_to_list_char "Az"))` for the
  literal `"Az"`, which appears nowhere in the catalog. K3 reduces the view to
  `Cons 65 (Cons 122 Nil)`; each `Equal Int` closes by kernel literal equality
  (`obs.rs::eq_at_registered_literal`).
- **AC-2 (neutral controls).** On a bound `String` or `Char` variable the
  application stays neutral and the same witness fails to check. A non-ASCII
  literal gets its correct codepoint view, and its `AllAsciiCodes` witness
  cannot be built.
- **AC-3 (consistency).** For a paired literal, the interpreter and native
  values of the view and codes equal the kernel's converted terms.
- **AC-4 (occurrence identity).** Two separate occurrences of the same
  literal, in different declarations, have whole views and heads that are
  definitionally convertible from their checked equal scalar payloads:
  `string_to_list_char "--"` in one converts with the other. Syntactic
  identity or interning of literal ids is not required. A genuinely
  different literal's view does not convert. `CAT-ARGPARSE-LAWS` laws 1 and 2 resume
  on this (operator 2026-09-24).
- **AC-5.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If the finite witness cannot elaborate or close on a fresh literal, STOP
  with the exact goal. Do not widen K3 to integer comparison or add an ASCII
  axiom.
- If the operation needs any further kernel reducer, STOP for design review.
- BYTES spec D2 and Foundation D3 wait for this node to land.
