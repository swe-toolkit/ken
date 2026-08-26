---
id: RT-ITREE-CHECKED-IH-RESULT-SUCCESSOR
title: "RT-ITREE checked-IH result-successor — the UPSTREAM planner-only predecessor that derives and validates the missing checked-IH result-successor relation: a SUCCESSOR PROJECTION of one existing CheckedIhEnvironmentTransport, keyed by the existing transport/call endpoints plus the reached destination owner and checked invocation coordinate, pairing (forward walk) the D3A application result through ConstructArgument -> TerminalResumeOuter active frame -> declared recursive child -> exact checked self-resumption invocation -> eventual Ret case -> Ret constructor-child binder -> exact lexical-closure capture occurrence and its existing BoundaryClosureEnvironment record. No lowering consumer, no emitted call/ABI/artifact/result/runtime change; behaviorally inert. RT-RESULT's atomic D3A+D3B consumer builds on top AFTER this lands. HS5 (evt_494k61s04fnv9): lowering reached the end of its authority; the structural closure belongs in this planner relation, not a sixth lowering exception."
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-RESULT-CONTINUATION-BINDING-PROVENANCE]
github: null
origin: "Architect hard-stop-5 ruling evt_494k61s04fnv9, 2026-08-26 (thr_3wd9rz5wqpdz7), on RT-RESULT-CONTINUATION-BINDING-PROVENANCE D3B. The D3B localization is VALID and independently closed the fixed-input correction (read paired via frame 301/closure 460/capture 459/body 452; write independently via frame 314/closure 473/capture 472/body 465); both reach an ordinary zero-argument checked invocation for which no planner-issued successor exists. Lowering has reached the end of its authority; the next component is this upstream planner-owned relation. Steward-owned frame per the ruling; Steward owns the final node name. Inventory fold 244b2468afd4f0cd06837fd3079f291d7d330af5 (entry 5) into the RT-RESULT recut."
---

> # UPSTREAM PLANNER-ONLY PREDECESSOR — HS5 structural closure (Architect evt_494k61s04fnv9)
>
> HS5 accepted. Evidence WIP `4e516e54712a47cf14c47b7abf2840f943071af9`
> (tree `9f7ac95f038bfb69bd6a881ec14133957e569078`, corrected base `14040ecae`)
> stays EVIDENCE ONLY on the consumer node; frozen HS4 `7199330550` unchanged.
> Hard-stop count is 5; Research is not triggered until hard stop 6.
>
> The D3B localization proved the missing component is NOT a lowering fallback
> and NOT another continuation-identity lane. On BOTH admitted programs the graph
> already pairs the D3A result through construction, `TerminalResumeOuter`, the
> active-frame header, the declared recursive child, and exact checked
> self-resumption — then both reach an ordinary ZERO-argument checked invocation
> (`checked_ih_environment_transport_for_invocation` returns `None` at
> `(Spec(2), cont 301, pos 1)` and independently `(Spec(5), cont 314, pos 1)`;
> `recursive_unit_body=None`), while the separately existing predeclared
> boundary-closure records (241 read / 352 write) establish the closures'
> code/capture layouts but do NOT prove a dynamic-environment crossing and do NOT
> turn the zero-argument invocation into the later one-parameter closure call.
> The missing relation is a genuine UPSTREAM planner predecessor.
>
> This node derives and validates that relation ONLY. It has NO lowering
> consumer, changes NO emitted call, ABI, artifact, result, or runtime behavior,
> and is INDEPENDENTLY LANDABLE. RT-RESULT's D3A stays frozen/non-landable while
> this is built; after this lands, the RT-RESULT D3 branch rebases and builds the
> ATOMIC D3A+D3B consumer, whose D3B may consume ONLY this exact successor
> projection through the existing shared D3A call lane and ordinary
> active-continuation semantics.

## Objective

Derive and validate the missing planner-owned checked-IH result-successor
relation: a SUCCESSOR PROJECTION of one existing `CheckedIhEnvironmentTransport`
that pairs the D3A application result, by a forward planner walk carrying the
active computational-frame lineage, to the eventual `ITree::Ret`
constructor-child binder and its exact lexical-closure capture occurrence. The
node produces the relation, one exact accessor, and its validator controls. It
emits nothing and changes no runtime behavior; its correctness is the pairing,
not any product.

## Authorized component shape (Architect evt_494k61s04fnv9)

1. The relation is a SUCCESSOR PROJECTION of one existing
   `CheckedIhEnvironmentTransport`. It does NOT mint another environment
   transport, call identity, binder catalog, aggregate record, or numeric
   identity. Its KEY is the existing exact transport/call endpoints PLUS the
   reached destination owner and checked invocation coordinate. Numeric origins
   remain report coordinates only.
2. Derive it by a FORWARD planner walk carrying the active computational-frame
   lineage from the transport's destination construct. Do NOT add a reverse
   parent/body/environment search. The derivation must pair, IN ORDER:
   exact transport result -> destination construct field -> `TerminalResumeOuter`
   active frame -> declared recursive child -> exact checked self-resumption
   invocation -> eventual Ret case -> Ret constructor-child binder -> exact
   lexical-closure capture occurrence and its existing `BoundaryClosureEnvironment`
   record.
3. Extend the EXISTING binder-provenance walk rather than adding a second binder
   catalog: retain `CheckedCaseBinderLayout` as the SOLE binder-order authority,
   but PRESERVE the exact ordinary `ConstructorChild` provenance needed at the
   Ret capture instead of collapsing every non-IH binder to an undifferentiated
   `Ordinary`. `CheckedIhBinding(None)` remains a required NEGATIVE control (the
   Ret occurrence is an ordinary constructor-child binder, not an IH).
4. Distinguish two operations the evidence proves are NOT interchangeable. The
   reached checked-IH invocation takes ZERO source arguments; the later ordinary
   boundary closure declares ONE parameter. Do NOT call that closure at the
   zero-argument site, invent its parameter, or reinterpret the zero-argument
   invocation as the later closure call. The successor relation authorizes REUSE
   of the existing D3A transport/capture/envelope/call morphism at the
   recursively exposed checked-IH invocation; the ordinary active-frame Ret
   binder and later closure call remain ordinary source semantics.
5. The predeclared closure environment record stays OWNED by its canonical
   predeclared source. The successor may REFERENCE that exact record ONLY after
   proving the active lineage and binder/capture pairing; it must NOT copy,
   re-own, or treat record presence alone as crossing authority
   (`boundary_closure_crossing_environment` deliberately refuses a
   descriptor-only crossing).
6. Expose ONE exact accessor keyed by the reached destination owner PLUS the full
   checked invocation coordinate; it returns the paired existing transport and
   successor proof, or `None`. NO "only candidate", first-match,
   family/spelling/tag/field-count, runtime-word, `Var(0)`, capture-ordinal,
   body-identity, or environment search.

## Deliverables

- The planner-owned checked-IH result-successor relation and its one exact
  accessor (item 6), derived by the forward walk (item 2), extending the existing
  `CheckedCaseBinderLayout` provenance walk (item 3), on the planner side only.
- The validator controls below. No lowering edit, no emitted-behavior change.

## Acceptance criteria

- AC-SUCC-DERIVE (one complete successor per governed arrival, read/write
  separate) — on BOTH unchanged admitted programs, derive EXACTLY ONE complete
  successor for each governed recursively exposed arrival: read and write
  SEPARATELY, each with its OWN active frame, Ret binder, closure record, and
  capture occurrence (read frame 301 / closure 460 / capture 459 / body 452;
  write frame 314 / closure 473 / capture 472 / body 465 — derived, not asserted,
  and never cross-used).
- AC-SUCC-REDERIVE (validator equality) — re-derive the closed relation in the
  validator and require EXACT equality with the planner-issued relation.
- AC-SUCC-MUTATIONS (each bites at its intended arm, byte-clean restore) —
  duplicate a successor; omit one; SWAP the read/write endpoints; perturb the
  active frame; perturb the checked invocation coordinate; change the Ret binder
  role; substitute descriptor-only authority for the proven crossing. EACH must
  FAIL at its intended arm and restore byte-identically.
- AC-SUCC-EXCLUSION (positive exclusion controls) — keep an ordinary NON-governed
  checked invocation AND a descriptor-only closure present; NEITHER receives a
  successor merely because it resembles the admitted paths. The accessor returns
  `None` for both.
- AC-SUCC-INERT (behaviorally inert) — establish that the planner-only
  predecessor changes no behavior: D3A is still ABSENT on current main, both
  read/write products stay at their existing defaults, the emitted ABI / call /
  artifact surfaces are BYTE-IDENTICAL, and D1 plus the erasure blob
  `8532ced2...` are unchanged.
- AC-SUCC-NEGATIVE (`CheckedIhBinding(None)` preserved) — the Ret occurrence is
  proven an ordinary constructor-child binder, not an IH; the negative control
  reddens if the derivation reclassifies it as an IH.
- AC-SUCC-HS6 (forward-derivability or hard stop 6) — if the COMPLETE relation
  cannot be derived by the forward planner facts of item 2, that is HARD STOP 6:
  HOLD before ruling and TRIGGER Research. Do NOT weaken the relation, add a
  reverse search, or move discovery into lowering.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the relation is a successor PROJECTION of one existing
`CheckedIhEnvironmentTransport` keyed by existing endpoints + reached
owner/invocation coordinate — no new transport/call/binder/aggregate/numeric
identity; derived by the FORWARD walk with no reverse parent/body/environment
search; `CheckedCaseBinderLayout` remains the sole binder-order authority with
the ordinary `ConstructorChild` provenance preserved and `CheckedIhBinding(None)`
a live negative control; the zero-argument checked invocation is NOT conflated
with the later one-parameter closure call; the predeclared closure record is
referenced, never re-owned; one exact keyed accessor with no candidate/first-
match/tag/ordinal/body/environment search) + runtime-qa (AC-SUCC-DERIVE derives
one complete read AND write successor from their own facts; AC-SUCC-MUTATIONS
each bite at the intended arm with byte-clean restore; AC-SUCC-EXCLUSION denies a
successor to the ordinary/descriptor-only controls; AC-SUCC-INERT holds — no
emitted-behavior change, byte-identical ABI/call/artifact, D1 + erasure blob
intact). A failure to derive the complete relation by the forward facts
HARD-STOPS to the Architect as hard stop 6 (Research triggered).

## Capability tier

T1 — a soundness-bearing UPSTREAM planner relation reviewed on the provenance
argument (a forward successor projection of one existing transport, keyed and
derived exactly, distinguishing the zero-argument checked invocation from the
later one-parameter closure call), not a differential diff. Size M.

## Sequencing

Lane-1 (runtime, priority). This is the UPSTREAM planner-only predecessor for
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]]: it lands FIRST (independently
landable, behaviorally inert), then the RT-RESULT D3 branch rebases and builds
the ATOMIC D3A+D3B consumer whose D3B consumes ONLY this successor projection
through the existing shared D3A call lane. Runtime is HELD on RT-RESULT until
this predecessor is framed, landed, and the successor D3A+D3B work is explicitly
re-released. If this node hard-stops (AC-SUCC-HS6), that is hard stop 6 and
Research is triggered. Single Runtime lane object at a time; PX8 stays blocked
until the whole native carried-value program lands.
