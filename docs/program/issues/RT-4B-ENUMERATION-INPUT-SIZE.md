---
id: RT-4B-ENUMERATION-INPUT-SIZE
title: "Gate 4b's observer records whether an oriented plan ARRIVED but not how large the population it walks is, so `keys = []` cannot be read as either lawful absence or a missing producer relation -- and the scalar that would settle it is the admitted-discovery ledger's length, not the oriented plan's, because that ledger is what candidate enumeration actually iterates"
status: ready
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Architect ruling evt_7b6d8jf1jd4qy authorizing the size field inside 4b's observation gate, plus the Steward's follow-up measurement at evt_ky5f547e6hjz establishing that enumeration does not distinguish any of its thirteen exits. Scope, ownership and sequencing taken by the Steward 2026-08-13.
---

## What this is

Gate 4b measured one arrival on the real C2 source: `oriented_present = true`,
`keys = []`, `descriptors = []`, `fusion_definitions = 0`. **Three of those four
values do not mean what the handback read them as**, and the fourth is
undetermined:

- **`fusion_definitions = 0` is the pinned expected value.** The field's own doc
  says *"Zero until the emitter exists; the gate pins that zero beside a
  resolved plane so the later `0 -> 1` is a statement about emission."* Gates 5
  and 6 are held and production is unarmed, so the emitter does not exist.
- **`oriented_present` is a boolean over an `Option`** —
  `oriented_subcontinuation_plan.is_some()` at `lowering/core.rs:2188`. It
  establishes that a plan object arrived. It does not establish that any
  population arrived with it.
- **`keys = []` is exactly `candidates.len() == 0`.** The Architect read the
  interning loop (`static_transition.rs:10030-10053`) and established that it
  has no decline path: every enumerated candidate is interned or the whole build
  errors. So `keys.len() == candidates.len()` identically.

⇒ **The key plane faithfully reports a number that was already decided upstream,
and nothing records what decided it.**

## Why the obvious scalar is the wrong one

The Architect authorized recording *"the oriented plan's size"*. **Measurement
says that is not the population candidate enumeration walks.**

`enumerate_live_fusion_candidates` (`static_transition.rs:10242`) iterates

```rust
for admitted in fusion_root_source_for_future_enumerator(plan)? {
```

— the **admitted-continuation-discovery ledger, derived from `plan`**, not from
`oriented`. The oriented plan is used for the presence check (`:10254`), for
`build_checked_transport`, and for key re-derivation — but it is not the thing
being walked.

⇒ **If the admitted ledger is empty, candidates are empty regardless of how
large the oriented plan is.** Recording only the oriented plan's size would
therefore fail to distinguish *nothing to fuse* from *something to fuse* in
precisely the case that matters, which is the same defect shape this gate keeps
producing: measuring a proxy rather than the route.

`OrientedSubcontinuationPlanV1` also has **no single size** — it carries four
vectors (`frames`, `recursive_calls`, `computational_ih_slots`,
`computational_ih_calls`). Any single scalar for "the oriented plan's size"
would be a choice, and an unstated choice is how a number gets read as more
than it measures.

## Not this node

- **A per-elimination-point census inside `enumerate_live_fusion_candidates`.**
  The Steward measured all thirteen exits (`:10254`, `:10265`, `:10272`,
  `:10279`, `:10288`, `:10296`, `:10301`, `:10313`, `:10321`, `:10324`,
  `:10327`, `:10332`, `:10343`) and **none of them is distinguished** — every
  one is a bare `continue` or bare early return. By the Architect's own stated
  criterion, teaching enumeration to distinguish them is a change to the builder
  and is **out of the observation gate**. Do not start it here.
- **Any enumeration, classifier, checker, marker, fusion-candidate,
  representation, ledger or closure-boundary repair.** Gates 5 and 6 stay held;
  production stays unarmed.
- **Any change to what a plan or an artifact contains.** This is a read.
