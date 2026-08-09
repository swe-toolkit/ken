---
id: RT-CALL-EDGE-EXECUTABILITY-AXIS
title: "executable_call_edges probes a body-axis set with an entry-axis key, so a template-only callee whose axes differ survives the filter and fails later as a forward-declaration error"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary report evt_1gtad2keqngcq (2026-08-09) on merged 1f706520, the RT-BODY-OCCURRENCE-PROVENANCE accepted partial at exact 876450ab. Steward-triaged as a confirmed latent defect and filed per COORDINATION §2. NOT folded into RT-CANDIDATE-LEDGER-RESIDUALS: that node's leading claim is "neither is a defect", and this one is.
---

> # LATENT, FAIL-CLOSED, AND CHEAP. DO NOT WIDEN IT.
>
> No witness exists today. The failure direction is a **spurious refusal**, not
> a fabricated `FuncId` and not unsoundness. The fix is about one line.
>
> **Do not repair this inside `RT-SPECIALIZED-MATCH-ATTRIBUTION`**, which is
> measurement-only and forbids production changes.

## The defect

`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs:11354`

```rust
let template_only = self.template_only_worker_bodies()?;   // 11350
...
.filter(|edge| !template_only.contains(&edge.callee_origin()))   // 11354
```

**The set is body-axis; the key is entry-axis.** `template_only_worker_bodies`
builds its candidates at `11248-11252` from `context.worker_body_origin()`, and
the sibling method's comment at `11334-11336` says so: *"`template_only` is a
set of worker BODY origins ... so the membership test names the body axis."*
`edge.callee_origin()` is the scheduling entry — `entry_origin()`'s own doc at
`9633`, `resolve_call_edges` at `units.rs:671-673`, and the `AC-4` control
`call_identity_stays_on_the_entry_axis_after_the_body_axis_moved` (`16796`) all
say so.

**The invariant it violates is stated in the same file by the same candidate**,
at `11861`: *"executability is a function of the body alone."*

All three production membership probes of that set:

| site | probe | axis | verdict |
|---|---|---|---|
| `11337` `executable_units` | `unit.body_occurrence()` | body | correct — fixed by `876450ab` |
| `11354` `executable_call_edges` | `edge.callee_origin()` | entry | **mismatch** |
| `11861` composed-selector refusal | `answer.body_origin` | body | correct |

The outlier is the sibling of the one the candidate fixed, reads the same set,
and sits seventeen lines away.

## Why it is a natural error, not a careless one

Call identity legitimately lives on the entry axis, and `EmittableCallEdge`
carries only that axis (`9557`). The site asks an **executability** question
with the only origin it has to hand. The two invariants are genuinely
different and this is the one place they meet. Do not write the repair as
though someone was sloppy.

## Failure direction, bounded

A template-only callee whose two axes differ does not match the set, so the
edge survives the filter, reaches `units.rs:679` `bundle.function(edge.callee())`,
and gets `None` — because `executable_units` correctly excluded that unit from
both the declaration and definition passes. The result is a hard refusal
reading *"a call edge names a unit that was never forward-declared"*: a
spurious compile failure blaming forward-declaration rather than the retarget.
`UnitBundle::function`'s `Option` catches it, which is what it is for.

## What is NOT established

**No witness where both conjuncts hold.** Split-axis units exist now — `16831`
asserts `fixtures_with_split_axes > 0` and `computational-nested` is one
(`origin_of(n18) != n5`). Template-only-ness needs the `D5a` full-retarget
population. **Nobody has constructed a unit that is both**, and the Adversary
said plainly that it did not try.

So this is latent by exactly the argument `core.rs:16713-16719` already
rejects as authority: *"agreement on the current population is not authority."*
That is the grounds for fixing it, and also the reason not to claim a
regression.

## `D0` — decide the axis, one line either way

1. Either map callee to unit and read `body_occurrence()`, mirroring `11337`;
2. **or**, if the axes provably coincide for template-only units specifically,
   say so in a comment at `11354` and name why.

Either discharges this. The point is to convert an accident into a decision.
**The mechanism is the owner's call** — the Adversary explicitly declined to
choose it, and so do I.

## Acceptance

- `AC-1` The probe at `11354` and the set it reads name the **same axis**, or a
  comment at that site states why they need not and grounds it.
- `AC-2` If option 1: a control exercising a template-only callee **whose two
  axes differ**. If no such fixture can be constructed today, say so in the
  handoff and state what population would be needed — **do not** substitute a
  same-axis fixture and report the AC discharged.
- `AC-3` No control keyed on the source text of the accessor. The only existing
  control touching `callee_origin` is the source-text oracle at
  `control.rs:4080`, which pins the accessor's **declaration string**, not the
  axis, and therefore pins nothing here.

## Forbidden

- Widening beyond the two options in `D0`.
- Touching `876450ab`'s seven paths for any other reason.
- Reporting `AC-2` discharged on a fixture whose axes coincide. That is the
  green-vs-green-adjacent shape: the control would pass for a reason unrelated
  to the property.
