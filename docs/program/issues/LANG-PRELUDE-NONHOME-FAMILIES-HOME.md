---
id: LANG-PRELUDE-NONHOME-FAMILIES-HOME
title: "Deleting Legacy resolution would strand pre-source names that tests and the spec treat as public but no package exports: 48 checked and saturating fixed-width operations, RangeError and three ABI-scalar narrowings, Decimal and its operators, isScalar and inRangeBool, and zip. Home each family at its existing identity, or route it through the surface it already has, as the Architect ruled by registration site, so I4 can delete Legacy"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_13j52ac43jrw8 and evt_3pxvhj0yngpxm on LANG-DELETE-LEGACY-RESOLUTION I1's 85-spelling census (implementer evt_bj4em6fyrmz3): one homing node, landing before I4. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# Every public prelude name has a route

## Objective

Every pre-source name the I1 census found positively used without a
package home has a Strict route at its existing identity: an
explicit-import home (H), a prelude floor member (B), or an existing
surface form (R). After this node, I4 can delete Legacy without
stranding a positive use.

## Settled inputs (main `68d4a6ab4`)

The scope is the Architect's ruling `evt_3pxvhj0yngpxm` ("THE HOMING
NODE" and sites 1 to 5), which is the contract for this node. In short:

- **H in `Data.Numeric.FixedWidth`.** The 48 `checked`/`saturating`
  `{Add,Sub,Mul}` operations over the eight widths (`conversions.rs`
  :244-310), and `RangeError`, `MkRangeError` and
  `intTo{USize,ISize,CInt}` (`:301`-`:365`). FixedWidth exports only the
  eight `intTo{W}` today. The raw narrowings stay unexported.
- **H in `Core.Classes.LawfulClasses`.** `isScalar` and `inRangeBool`
  (`decimal_char.rs` :238-257), beside `charToInt` and `intToChar`. The
  lower and upper scalar helpers stay unexported.
- **B, `Decimal`.** A floor member at `decimal_id`. Literal typing
  (`elab.rs` :16166) and `+`/`==` dispatch (`decimal_char.rs` :207,
  :217) key on its exact id; spec 35 §2.3 names it. The pair
  representation stays internal.
- **R, Decimal arithmetic.** `decimalAdd` and `decimalEq` through `+`
  and `==`. `-` and `*` at Decimal have no dispatch entry today
  (`numbers.rs` :752-755); this node registers them to `decimalSub` and
  `decimalMul`.
- **R, `zip`.** `Data.Collections.Derived`'s private List zip
  (`Derived.ken.md` :1564, with three proved laws) becomes public, and
  spec 37 §4 and §9 settle it there.
- **Consumers.** Five user sources declare a same-spelled `Decimal`
  (listed in the ruling) and rename their model type, keeping their
  verdicts. Importers of the three packages are swept for local spellings
  of the new exports.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec taxonomy rows and the 35 §2.3, 35 §5 and 37 §4/§9 sentences (Spec
enclave); the three catalog export lists and their prose; in
`crates/ken-elaborator`, `PRELUDE_PACKAGE_HOMES` (+55 pairs), the Strict
floor entry for Decimal and the two Decimal dispatch entries; the
consumer renames; CV seed rows. No new law, identity or kernel change.
The I1 rows that wait on this node migrate in I4, not here.

## Acceptance

- **AC-0 (D0, before code).** A scratch Strict probe shows that `USize`,
  `ISize`, `CInt` and their `*_to_int` conversions resolve without an
  import, and that `-` and `*` at Decimal refuse on main. A failing probe
  is a stop to the Architect.
- **AC-1 (H rows).** Each homed name, imported from its home, is
  accepted at its pre-source id; the same use without the import refuses
  `UnboundName`.
- **AC-2 (B row).** `const c : Decimal = 2.0d` checks without an import,
  and a same-spelled source `Decimal` is refused.
- **AC-3 (R rows).** `x + y`, `x - y`, `x * y` and `x == y` at Decimal
  each elaborate to the exact pre-source operation id.
- **AC-4 (seed).** The CV's rows from the ruling: an imported checked
  operation overflowing to `None`; `intToUSize` at manifest max + 1
  giving `Err`; Decimal without an import with exact `*`; one imported
  `isScalar` use.
- **AC-5.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A home needs a new identity, a new law or a kernel change.
- An operator R that does not elaborate to the exact id.
- A renamed Decimal user whose verdict changes arm.

Carried, not framed: after I4, the pre-source `zip` has no source route
and no keyed mechanism; retire it with its `ac5` bracket (Architect
`evt_3pxvhj0yngpxm` site 5). Sibling: `LANG-PROCESS-CHILD-PIPE-WRITE-HOME`
homes three pipe operations by the same H route.

Checks: 6 fired: the population is the implementer's 85-spelling census
(`/workspaces/ken/tmp/l2-i1-provider-audit.tsv`), disposed by
registration site, so each generated family is homed whole. 4 fired: the
Decimal `-`/`*` R rows and the zip R cannot be written on main; their
enablers (dispatch entries, the public zip) are in this node, and AC-0
probes them. 8 fired: each H row pairs the import with its absence on
one source, so a re-declared copy would fail AC-1's id check.
