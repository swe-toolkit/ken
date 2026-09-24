---
id: LANG-KENFMT-AXIOM-CLOSING-PAREN
title: "Formatter repair: ken fmt must print a valid axiom declaration whose type ends in a parenthesized application without dropping the final closing parenthesis, so formatting is parse-preserving and idempotent on every axiom shape"
status: ready
owner: language
size: S
gate: architect
tier: T2
depends_on: [LANG-IMPORT-LOAD-ORDER-INDEPENDENCE]
blocks: [BYTES-CONCAT-AND-ENCODE-CONTRACTS]
github: null
origin: "BYTES-CONCAT-AND-ENCODE-CONTRACTS D2 full-CI red (run 36012146849) and its frame-authorized formatter STOP (spec-leader evt_8hpd13pfaetv). The language ring owns kenfmt (crates/ken-elaborator/src/layout.rs). Serves L3; sequenced after the L2 landing because both touch ken-elaborator. Steward-filed per COORDINATION section 2."
---

# `ken fmt` drops an axiom's last parenthesis

## Fixed inputs -- measured by the Spec ring at `b2c2cfd0e`

- This valid declaration:

  ```
  axiom lost_closing_paren : (a : Bytes) → Equal Bytes a (bytes_concat a a)
  ```

  After `ken-cli fmt`, the output is missing its final `)`. A second
  `fmt --check` rejects it with `expected RParen, found Eof`.
- In CI, BYTES D2 `84c88e4a0` fails three gates on its new
  `catalog/packages/Data/Binary/BytesPrimitiveContracts.ken.md`, all with
  the same parse error (`expected RParen, found KwAxiom`):
  - `ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded`;
  - the frozen-corpus canonical check;
  - the Z3 emission control's whole-corpus kenfmt checks.
- `layout.rs` prints `Decl::AxiomDecl` through `print_decl_signature` over
  the declaration's span. Whether the dropped token comes from the printer or
  from the parser's recorded `AxiomDecl` span is unmeasured. Find it.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

`ken fmt` is parse-preserving and idempotent on every `axiom` declaration,
including one whose type ends in a parenthesized application. Repair the
cause at its owner, whether that is the printer or the span, and correct any
other declaration kind that shares the same defect. Change no formatting
output for a declaration that already round-trips.

## Acceptance

- **AC-1 (red first).** The reproducer above, and the unchanged BYTES D2 file
  from `84c88e4a0` used as a fixture, fail the round trip at base. On the
  candidate, both round-trip: the formatted output parses to the same
  declarations, and a second format is byte-identical.
- **AC-2 (mutation).** Reverting only the repair reddens AC-1.
- **AC-3 (no collateral).** The whole-catalog kenfmt gate and the
  frozen-corpus canonical check stay green, with no catalog file reformatted.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

Stop if the fix changes the output for any declaration that already
round-trips, or if it needs a grammar change.
