---
id: CAT-CONFIGURATION-DECODER-PRESENCE-CARRIER
title: "replace the decoder's lossy List Bytes result with List (Option Bytes) -- empty Bytes is an ordinary value in this package, not a reserved sentinel, so the current map sends two distinct inputs (no entry, and an entry whose value is empty) to one indistinguishable result element; carry the validated payload instead of recomputing it, and delete env_config_values with its placeholder branch so the agreement obligation disappears at its source rather than being proved"
status: draft
owner: foundation
size: M
gate: none
tier: T1
depends_on: [CAT-CONFIGURATION-DECODER-LAWS]
blocks: []
github: null
origin: "Architect ruling evt_43cf1x0808egr, 2026-09-21, answering a design question the Steward routed at evt_1xaz1d285wsz5. The conflation was found by foundation-leader (evt_28sx2xdgsb0nz, reproduction evt_7wgfandr6fd2e) while discharging CAT-CONFIGURATION-DECODER-LAWS, whose AC-1 asserted a universal agreement that is false. The Steward's frame had named the wrong lane -- it predicted a missing REQUIRED field reaching the placeholder, but schema_check_presence rejects SchemaRequired outright, so the reachable case is an absent OPTIONAL field. Filed as a separate node because a representation change to a shipped package is not a rider on a proof-backfill WP."
---

# Decoder presence carrier: make absence distinguishable from an empty value

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## 1. Objective

Change `catalog/packages/Application/Configuration/Decoder.ken.md` so a decoded
result distinguishes "the field was absent" from "the field was present and its
value was empty", by making each aligned element `Option Bytes`. Remove the
second traversal that made the two inconstructible from each other.

## 2. Settled inputs

**The defect.** Empty `Bytes` is an ordinary, valid raw value here, not a
reserved sentinel, and no law or schema declaration says absence defaults to
empty. The package promises raw-`Bytes` preservation, so a carrier that maps
`no entry` and `entry with empty value` to the same element is lossy in
public. Measured at `d5d7e6299`: `env_config_values` (`Decoder.ken.md:118-128`)
emits `Cons Bytes (list_to_bytes (Nil UInt8))` on `None`, and
`schema_check_presence` (`Schema.ken.md:90-100`) accepts `SchemaOptional` with
`True` while rejecting `SchemaRequired`, so the placeholder is reachable
exactly on the optional lane.

**The consumer census, and why it decides the shape.** Measured independently
by the Steward at `d5d7e6299`, agreeing with the Architect's own: the two
exported decoders have **no catalog, example, conformance, or production
consumers.** The only external references are two elaborator test files,
`crates/ken-elaborator/tests/cc8_env_config_decoder_acceptance.rs` and
`crates/ken-elaborator/tests/cat_tier_e_decoder_import.rs`. Ken is under
PRINCIPLES transient T, so there is no compatibility beneficiary. That is what
authorizes replacing the type IN PLACE rather than adding a parallel decoder.

**The ruled component boundary** (Architect, `evt_43cf1x0808egr`) -- five
requirements, all operative:

1. The exported result type becomes
   `Validation (NonEmpty Diagnostic) (List (Option Bytes))`, changed in place.
   No parallel legacy decoder, no package-local wrapper type.
2. The client checker's accepted value changes from `Bool` to `Option Bytes`:
   present lookup accepts `Some Bytes value`; absent optional accepts
   `None Bytes`; absent required still rejects with the existing issue and
   origin.
3. `env_config_validation` preserves the `Valid values` payload from
   `schema_validate` instead of discarding it. On `Valid` it returns the
   aligned payload unchanged; it only translates the `Invalid` issue carrier
   to `Diagnostic`.
4. `env_config_values` and its empty-placeholder branch are DELETED. Do not
   retain a second traversal once the first carries the value.
5. Key lookup order and shadowing, raw-byte identity, field order, issue
   accumulation, and the two entry-point provenance policies are unchanged.
   This needs no primitive, postulate, `Axiom`, new data type, or trust
   movement.

**Why `List (Option Bytes)` and not the alternatives**, so they are not
re-proposed: a parallel presence bitmap reintroduces an alignment obligation;
skipping absent fields loses schema-position alignment; an `Absent | Present
Bytes` datatype duplicates `Option`; and encoding a default into empty bytes
repeats the defect. Defaults, if ever wanted, belong explicitly in `Schema`.

## 3. Deliverable

The representation change above, plus the atomic retirement described in AC-3.
This node changes function bodies and an exported type; it is not a
proof-backfill node.

## 4. Acceptance criteria

**AC-1 -- absence and emptiness are distinguishable.** One identical optional
`SchemaBytes` field under two entry lists: absent yields `None`; present with
`list_to_bytes (Nil UInt8)` yields `Some` of empty `Bytes`. These are the
discriminating pair -- a change that does not separate them has not fixed the
defect.

**AC-2 -- nothing else moved.** Required-present yields `Some` with byte
identity; required-absent remains `Invalid` with its existing issue; the
`EnvVariableOrigin` / `ConfigEntryOrigin` pair stays distinct across the two
entry points. Field order, lookup order and shadowing, and issue accumulation
are unchanged.

**AC-3 -- the predecessor law retires in the same commit.** The optional-lane
law published by `CAT-CONFIGURATION-DECODER-LAWS` (its AC-2) is a truthful
characterization of the PREDECESSOR, not a permanent semantic promise. It must
be replaced or retired atomically with this change -- a landing that leaves it
asserting that an absent optional field yields empty `Bytes` is self-
contradictory. The required-field agreement law (that node's AC-1) survives
and must still hold under the new carrier.

## 5. Stop condition

Stop and report if the change cannot be made without a new primitive,
postulate, `Axiom`, new data type, or trusted-base entry; if `Option` is not
available to this package without widening an import beyond what `Schema`
already permits; or if any consumer outside the two named elaborator test
files turns out to exist on the landed base, since that would refute the
census that authorizes an in-place type change.

## 6. Not this node

Do not add defaults to `Schema`. Do not change `env_config_help`'s delegation
to `schema_help`. Do not alter duplicate-key shadowing or total ordering. Do
not re-prove the required-field agreement from scratch; it is
`CAT-CONFIGURATION-DECODER-LAWS`' deliverable and this node inherits it.

## 7. Release condition

`status: draft` deliberately. The Architect ruled that this node begins only
from a landed current main and does not authorize work ahead of
`CAT-CONFIGURATION-DECODER-LAWS`. Flip to `ready` and release only after that
node's candidate has landed, and re-measure section 2's coordinates against
the landed base first -- this node deletes a function that node proves laws
about.
