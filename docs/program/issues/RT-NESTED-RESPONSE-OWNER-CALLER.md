---
id: RT-NESTED-RESPONSE-OWNER-CALLER
title: "The ignored rt_span_prov_native:355 row refuses at ObjectEmission: a forward-declared response owner (the second BufferFreeze) has no verified selected incoming call, because its selected caller is planned inside the first BufferFreeze's specialization and lowering records no disposition for it. Measure where that caller site is lowered, then repair so the caller carries a recorded disposition and the row passes on both engines"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect fresh-D0 boundary ruling evt_v7p1c6t7kmw6 on main ff0a35f24 (Steward request evt_396hg4b5bbpc0): the span row is not bracket-tree work (9 Specialized rows, 0 Deferred). Re-homed from RT-COMPMATCH-TREE-SCRUTINEE inventory line 1; RT-IGNORED-ROWS-NEXT-GROUP left it with no successor. Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# A nested response owner's caller has a disposition

## Objective

`sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
(`crates/ken-cli/tests/rt_span_prov_native.rs:355`) is un-ignored and passes
on both engines.

## Settled inputs (Architect `evt_v7p1c6t7kmw6`, at `ff0a35f24`)

- **The refusal.** ObjectEmission: "a forward-declared response owner has no
  verified selected incoming call: owner=StaticResponseOwnerId(0),
  context=ContinuationContextId(2), preexisting=false". The caller's token
  has emission_owner Specialization(1), producer construct origin 1079,
  alternative 1, target Specialization(2), and `disposition=None`.
- **The population.** 9 Specialized rows, 0 Deferred. Owner 0 is the
  BufferFreeze response at Vis 1079 (k-spec 2, new context). Its selected
  caller is planned inside Specialization(1), the k-spec of the other
  BufferFreeze response (Vis 1147, preexisting context 0).
- **Not the bracket tree.** The bracket rows (px8ta) ride the Deferred
  forward-Ret route. This row has no Deferred rows, and its failing owner is
  a Specialized BufferFreeze, not a settlement.
- **The instrument.** The coverage gate at `units.rs:7831` reports the
  miss. A forward declaration never satisfies it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only, no repair).** Instrument the Specialization(1) body
   lowering. Report verbatim whether the producer construct at origin 1079
   is visited there, and under which `defining_owner`. If it is visited
   elsewhere, report the owner and whether the claim path or an inline path
   takes it. A visit under another owner is a planned-versus-lowering
   emission-owner mismatch on the Specialized route; no visit means the
   specialization body omits the site. The Architect rules the repair from
   the D0.
2. **The ruled repair.**

## Acceptance

- **AC-1.** Owner 0's selected caller carries a recorded DirectCall or
  ComposedCall disposition, and the row is un-ignored and passes on both
  engines with the exact freeze sequence it asserts.
- **AC-2 (controls).** The five sibling rows in `rt_span_prov_native.rs`,
  the SEQUENTIAL distinguishable witness and its plan-row pins, and
  `one_bracket_retains_native_parity` stay green. The `units.rs:7831`
  coverage gate is not relaxed.
- **AC-3 (mutation, QA).** Reverting the repair returns the row to the
  ObjectEmission refusal with `disposition=None`.

## Stop conditions

- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or widens the lookup at
  `core.rs:11695`.
- Any kernel, trust or spec change.
