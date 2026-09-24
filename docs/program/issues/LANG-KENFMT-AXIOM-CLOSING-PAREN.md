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
- **AC-1a (sibling).** The type alias `def Wrapped = List (Option Nat)`
  loses its final `)` under `ken fmt` at base, like the axiom (Architect
  measurement). It is red first and round-trips on the candidate. The
  handback states which repair shape was taken (see Design) and, for the
  declaration-level shape only, names every parser production whose span
  end is taken from a sub-term's span, with its disposition.
- **Design (Architect).** The cause is the parser: the `TypeAtomForm::Paren`
  arm of `parse_atom_type` returns the inner type with the inner span, and
  unlike `ExprAtomForm::Paren` it never re-spans over the parens. Preferred
  repair: re-span that arm from `(` through `)`, which closes every
  consumer (axiom, alias, constructor arguments, prop intros) in one place.
  Fallback, if and only if the preferred repair changes the formatted output
  of an already-round-tripping declaration (the frame's STOP): end
  `AxiomDecl` and `TypeAlias` at the last consumed token, as `PropDecl`
  already does. Do not add a printer-side patch, and leave `print_sum`'s
  adjacent-`)` extension in place. It becomes redundant under the preferred
  repair; say so in the handback. A diagnostic-span test that moves because
  a type span now covers its parens is not a STOP, but list every such pin
  in the handback.
- **AC-2 (mutation).** Reverting only the repair reddens AC-1.
- **AC-3 (no collateral).** The whole-catalog kenfmt gate and the
  frozen-corpus canonical check stay green, with no catalog file reformatted.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

Stop if the fix changes the output for any declaration that already
round-trips, or if it needs a grammar change.
