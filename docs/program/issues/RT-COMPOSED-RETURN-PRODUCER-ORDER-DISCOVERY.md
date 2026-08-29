---
id: RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY
title: "DISCOVERY (viability verdict only): can source-specific validation authority be established BEFORE the existing producer — shape (a) — so the tail-resumptive composed-return fresh-R2 transfer becomes realizable without a store, captured continuation, runtime tag, or recovery. Scoped to shape (a); a refutation returns the shape (b) de-quotienting cost decision to the operator. Queued after the native-carried-value campaign."
status: draft
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator decision 2026-08-29, concurring with the Steward recommendation after Research advisory evt_774v5fjnxcfcw and Architect disposition evt_5te99temrdcty. The composed-return wall (closed node RT-COMPOSED-RETURN-PRODUCED-TRANSFER, D0b=NO) is a partial-order contradiction on the Tail lowering route. Research confirmed Q2 POSITIVE: the shape is reached by well-formed Ken SOURCE (two complete SourceFormat::Ken programs — fs-read-at-offset and fs-write-at-offset — through the real build_native_program pipeline; the 48 Tail / 3 Direct arrivals are compiler arrivals within them, not fixtures), so the wall is NOT moot. Research Q1: no surveyed family (Interaction Trees, Koka, CPS/SSA) preserves Ken's current order (emit R2 -> collapse -> quotient away source identity -> validate later) and still delivers R2 without a store/capture/tag/recovery; the Architect's two shapes are the only known families and there is no local patch. Operator: fund shape (a), QUEUE after the native-carried-value campaign; the language has no users so a wrong-value-vs-refuse failure-mode check is unnecessary."
---

> # DRAFT — OPERATOR-FUNDED, QUEUED. NOT RELEASED, NOT YET FULLY FRAMED.
>
> This node captures a durable operator funding decision (2026-08-29). It is
> **sequenced after the native-carried-value campaign** (`RT-NATIVE-CARRIED-VALUE`,
> active) on the single runtime ring — the ordering is ring contention and
> priority, not a logical dependency, so `depends_on` is empty. **Do not release
> it while the campaign is live.** The full frame (exact inputs measured at a
> named SHA, ACs with controls) is cut when the campaign nears drain and
> **re-measured then** — the cited coordinates below decay.

## The decision, and why it is real work and not fixture-cleanup

The composed-return wall is closed (`RT-COMPOSED-RETURN-PRODUCED-TRANSFER`,
D0b=NO): on the Tail lowering route the fresh result `R2` cannot be delivered
into the handler's `Ret` capture because the producer has already emitted before
the validated `Selected` owner exists — a partial-order contradiction, not a
missing carrier. Research settled the two questions that gate funding:

- **Reachable from Ken source (Q2 POSITIVE).** The D0b fixture compiles two
  complete `SourceFormat::Ken` programs (fs-read-at-offset, fs-write-at-offset)
  through the real parser/elaborator/native-build pipeline; "Tail" is derived by
  the compiler from checked source, not assigned by the test. One source witness
  (semantic core `bind t (\x. Ret (f x))` under resource bracketing) suffices to
  refute source-unreachable. This is bread-and-butter effectful I/O, so the gap
  sits on the ABI/native-completeness critical path.
- **No cheap or local fix (Q1).** Interaction Trees, Koka, and CPS/SSA all
  establish continuation/handler identity BEFORE producing the response, or make
  identity the selected function/block. None preserve Ken's current order without
  a store, captured continuation, runtime tag, or recovery — all forbidden here.
  The Architect's two shapes are the only known families; there is no third.

## Scope — shape (a) ONLY

Prove or refute whether **source-specific validation authority can be established
before the existing producer**, so the mint occurs only after it. Research's
grounding (re-measure before use): Ken already derives a fresh route per exact
pre-quotient inheritance and computes `checked_ih_fresh_result_route` per
inheritance BEFORE folding equal projections into the generated-entry confluence
class (`aggregates.rs:5932-6145`), so source-specific route authority exists
pre-quotient. **What is unproven — and is exactly this node's question — is
whether that authority can be exposed AT the producer under the current
confluence and caller-closure guarantees without becoming a second catalog or a
positional guess.**

Shape (b) (relocate the producer to a post-validation boundary via a static
de-quotienting / function-identity design) is **NOT in scope.** A shape-(a)
refutation is a clean, valuable outcome: it returns the shape-(b) cost decision
(static de-quotienting plus closure/code-size/environment reconciliation) to the
operator as a distinct, larger call.

## Deliverable — a VIABILITY VERDICT, not a landed mechanism

The deliverable is a discovery verdict: **shape (a) is at least probably viable**
(with the constructive sketch that makes it so), or **shape (a) is refuted** (with
the exact obstruction). It is not a build. If probably-viable, a follow-on build
WP is framed from the verdict; if refuted, the operator decides on shape (b).

## Prohibitions (carried from the wall — a candidate that needs any of these has refuted shape (a))

No store, captured continuation, runtime tag/discriminator, or recovery; no
backward token move across the emit-then-validate order; SSA permits only a
forward producer-to-resumption edge. Manufacturing the missing edge by any of
these is the forbidden route, not a solution.

## Inputs (decay — re-measure at frame-cut time)

Research advisory `evt_774v5fjnxcfcw` (source-coordinate ledger SHA-256
`9c4f7d45f7353ec2f9bcd28977ec88d62577cd758b902164e23bc64474900bb3`); Architect
disposition `evt_5te99temrdcty` at `origin/main 6c2b6a18f`; closed node
`RT-COMPOSED-RETURN-PRODUCED-TRANSFER` (D0b report SHA-256 `fdd5e859...`); spec
§6.2. No Research advisory is owed on release — this one discharges it; HS15 stays
unspent.
