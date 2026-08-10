---
id: LANG-STRUCTURAL-RESULT-ELAB
title: "Implement the structural-result selector in the elaborator -- derive the field/evidence/result association from the kernel method telescope and elaborate `structural result of x` to the hidden recursive method result"
status: ready
owner: language
size: L
gate: none
depends_on: [KERNEL-RECURSIVE-RESULT-SURFACE]
blocks: [KERNEL-NESTED-IND, DS-9]
github: null
origin: The implementation successor promised by KERNEL-RECURSIVE-RESULT-SURFACE, whose frame states "Implementation is an uncreated successor owned by Language/elaborator; the Steward creates it when D0 lands." D0 and D1 landed together as f9572c27 (PR #1800), so the contract is now concrete enough to frame against. Steward-filed per COORDINATION §2. Architect ruling evt_2s6gmzqvaj5mr fixes the semantic shape and the two prohibitions; the landed spec fixes the spelling.
---

> # THE CONTRACT IS LANDED SPEC. DO NOT RE-DERIVE IT, AND DO NOT AMEND IT HERE.
>
> `spec/30-surface/34-data-match.md §3.1.1` and `39-elaboration.md §2.3`/`§4`
> are **normative and merged**. This node implements them. If you believe the
> spec is wrong, that is an escalation to the Steward and then the spec
> enclave — **not** a local reinterpretation, and not a deliverable here.
>
> The predecessor node `KERNEL-RECURSIVE-RESULT-SURFACE` carries the measured
> obstruction, the Architect's approved semantic shape, and the two
> prohibitions. **Read it.** This node does not restate them.

## Why this node exists

The spelling and semantics are fixed; nothing consumes them yet. Until an
elaborator implements the association and the selector, two
`KERNEL-NESTED-IND` seed rows stay gated and DS-9 `D3`+ cannot express an
unbounded fold over `JsonArray : List Json`.

## The two prohibitions are load-bearing and are restated ONLY because they
## are the tempting implementations

Architect `evt_2s6gmzqvaj5mr`, in force:

1. Do not make hidden binders visible through `surface_var`, and do not add
   them as ordinary constructor-pattern fields. That changes the source
   constructor's arity and exposes kernel-internal support topology.
2. Do not coerce an ordinary field reference or an owner self-call into the
   IH. The former changes the field's source type contextually; the latter
   admits a call that is neither a direct guest-motive instance nor
   SCT-justified recursion.

**Either one makes the discriminating controls pass while delivering the wrong
capability.** That is why they are prohibitions and not preferences.

## Deliverables

- **`D0` — the association.** Extend the elaborator's per-field association
  with an optional recursive-result position, derived **only** from the kernel
  method telescope, with the one-to-one / in-range / same-support validation
  failing closed. Carries `StructuralResultAssociationMissing`, `...Duplicate`,
  `...Swapped`, and `...Foreign`. No surface form yet.
- **`D1` — the selector.** The `structural_result` production, name resolution
  to a single surface binding, and elaboration to the associated hidden result
  term. Carries `StructuralResultOutOfScope`.
- **`D2` — the discriminators.** `AC-1` … `AC-5` below. Separated deliberately:
  the Architect's *"depth-three alone is not the closure argument"* is the
  standard three candidates already failed, and a control set appended to an
  implementation deliverable is the shape that produced those failures.

⛔ **Binding the two `seed-nested.md` rows is NOT in this node.** It is a
`KERNEL-NESTED-IND` `D6` successor with fresh QA, Architect, and frontier-class
conformance-validator review. **No verdict from the four spent `D6` candidates
transfers to anything.**

## Acceptance — the discriminators must prove MECHANISM, not depth

Inherited verbatim from `KERNEL-RECURSIVE-RESULT-SURFACE`, whose frame states
these are the capability's acceptance and are discharged by the implementation
successor.

| AC | criterion |
|---|---|
| `AC-1` | emitted method bodies reference **the exact trailing recursive-result binder** for each correlated residual field |
| `AC-2` | **deleting that association reds an arbitrarily deeper value control** -- the association is load-bearing, not incidentally correct at the depths tested |
| `AC-3` | swapped and foreign associations **reject**, and so does use outside a lifted recursive field |
| `AC-4` | generated support identifiers **remain unresolvable** from source |
| `AC-5` | direct and W-style matching stay **byte-for-behaviour unchanged** |

Plus one row the landed spec adds, which the predecessor's ACs predate:

| `AC-6` | the positive validity rule decides an association the spec never names -- an arbitrary branch binding `u` with a validated `u -> r` accepts, and removing that association rejects, **with no new case added to either the spec or the implementation** |

`AC-6` is `AC-D0b`'s control carried into code. An implementation that
recognizes a carrier, constructor, field name, or motive **by name** fails it
while passing every other row.

## What this unblocks

Two `seed-nested.md` rows -- `nested-size-uses-lift` and
`nested-dependent-motive-uses-lift` -- and, via Architect `evt_6ysrp62e4zayg`
extending the obstruction to `List`-carried recursion, **DS-9 `D3`+**: the
unbounded Json fold over `JsonArray : List Json` and
`JsonObject : List (Pair String Json)`. Foundation is idle directly behind that;
DS-9 `D1`, `D2`, and `D3a` have all merged.

`D2`'s standalone `List Char` recursion does not share the blocker and is not
reopened.
