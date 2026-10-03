---
id: KERNEL-UNIT-ETA-SPEC-SCOPE
title: "Conversion gives definitional eta to every data type with one no-field constructor, including indexed families (definitional K), while spec 14 §4 gives data no eta and 17 §2 limits it to Unit and records. Count the rule's firings, then narrow it to the spec"
status: ready
owner: kernel
size: S
tier: T1
gate: architect
depends_on: [KERNEL-OBS-PI-CAST-GATE]
blocks: []
github: null
origin: "Research sweep evt_7daqm6ydmbnsr S7. Architect ruling evt_24070d0c9zgmw item 3. Kernel change; operator 2026-10-03: 'approve C'. Steward-filed per COORDINATION section 2."
---

# Unit-eta fires only where the spec gives eta

## Objective

Conversion's unit-eta arm fires for `Unit` (and records, through Σ-η) and
for no other `data` type, as spec 14 §4 (`OQ-η-records`, DECIDED) and 17
§2 state.

## Settled inputs (Architect `evt_24070d0c9zgmw`, read at `80afb1b10`)

- The arm: `conv.rs:1052-1062`, the `_` fallback of the type-directed
  conversion. It returns `true` when the type's head is an `IndFormer` whose
  inductive has one constructor with no `args`. It ignores the type's
  arguments, so an indexed family with one nullary constructor gets
  definitional K.
- Consistent under UIP, so this is a spec-conformance narrowing, not a
  soundness repair. Narrowing makes no new commitment.
- No record marker or `Unit` id constant was found in `ken-kernel/src` by
  grep. How the narrowed arm identifies `Unit` by checked identity (check
  10) is named at AC-0, not assumed.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. AC-0 census, then the arm narrowed to `Unit`, keyed on its checked
   identity, never on constructor shape or spelling.

## Acceptance

- **AC-0 (measure; no edit).** Instrument the arm and record its firings
  over the kernel suite, catalog parity and the elaborator tests, keyed by
  family `GlobalId`. Name how `Unit` reaches the kernel and the identity the
  narrowed arm keys on.
- **AC-1.** With zero firings outside `Unit` and records: two distinct
  neutral values of a non-indexed one-nullary-constructor `data` are not
  convertible; neither are two neutral proofs of an indexed one-nullary
  family (`Same a b`). `Unit`-eta still holds. The kernel suite and catalog
  parity stay green.
- **AC-2 (falsifier).** Restore the shape test, and both AC-1 refusal pins
  redden.

## Stop conditions

- Any proof relies on eta at another family: stop to the Architect with the
  AC-0 list. A spec amendment for the non-indexed case is then a separate
  spec decision; the indexed case is narrowed either way.
