---
id: RT-MAPPING-DISPATCH-CONTROL-COVERAGE
title: "Add the three negative/boundary controls the RT-MAPPING-MULTIOP-DISPATCH merge landed without: an S4 positive control that an unbounded/multi-shot effect chain is REFUSED (not silently looped), an S7 cross-class negative control on the Ok(Vec::new()) empty-census guard, and extension of the SuppressLocalContinuationDrive / owner-identity discriminator to the repeated-same-producer class"
status: draft
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Steward-filed 2026-09-10 on the landing of [[RT-MAPPING-MULTIOP-DISPATCH]] (merged 1c48b6c5c). Captures the Adversary's two non-blocking missing-negative-test notes (evt_fsd2bf8e4hrv, carried unchanged through the respin review evt_5qgbdffsq56k0) plus the Architect's S4 positive-control observation. These were explicitly NON-BLOCKING for that merge — the capability is sound and its S1-S7 gate held on the exact SHA — so the coverage is filed as a separate queued node rather than held against a merged candidate. QUEUED behind the active lanes; the runtime ring picks it up when the L1 runtime priority allows."
---

**This node is FILED and QUEUED. No ring is released on it yet.** It records
owed test coverage for a capability that has already merged; the merge was
correct and the notes below were non-blocking. It is here so the coverage is not
lost, per COORDINATION section 2.

## What landed, and what it landed without

[[RT-MAPPING-MULTIOP-DISPATCH]] built the general repeated-N>=2 same-producer
dispatch capability at the static response owner and merged at `1c48b6c5c`. Its
S1-S7 soundness gate held on the exact SHA (Runtime QA + Architect + Adversary
NO-DEFECT). Three controls named during review were judged non-blocking and are
owed as follow-up coverage. Re-measure every coordinate at pickup — the merge
moved lines.

## The three owed controls

**C1 — S4 positive control: an unbounded / multi-shot chain is REFUSED, not
looped (Architect).** S4 is the node's HARD BOUNDARY: statically-bounded chains
ONLY; effectful-recursion / unbounded / multi-shot effect trees are out of scope
and must fail-closed. The landed suite proves the bounded case executes; it does
NOT positively witness the refusal of the out-of-scope case. Add a control that
either (a) constructs such a sequence at the D5a surface and shows it REFUSED
(fail-closed, never silently unrolled or looped), or (b) if it is not
constructible at this surface, states that explicitly so a future increment does
not lean on this loop for the unbounded case (which carries its own totality
obligation). The danger the control guards is a silent unroll masquerading as
success.

**C2 — S7 cross-class negative control on the `Ok(Vec::new())` empty-census
guard (Adversary obs1).** S7 requires the generalization to be ADDITIVE and
INERT on the existing corpus — the widened gating must fire ONLY on the
previously-excluded repeated-stage case. The empty-census fast path
(`Ok(Vec::new())`) is the seam where a cross-class program could slip through
the new gating without being exercised. Add a negative control that a program in
a DIFFERENT effect class does not spuriously enter the repeated-producer path
via the empty-census guard — i.e. the guard's zero-case is proven not to be an
accidental accept for a class the capability does not own.

**C3 — extend the `SuppressLocalContinuationDrive` / owner-identity
discriminator to the repeated-same-producer class (Adversary obs2).** The
existing discriminator distinguishes the drive/no-drive decision by owner
identity for the single-op and heterogeneous cases. The repeated-same-producer
class introduced by this node is a new population for that discriminator; extend
its coverage so a mutation to the owner-identity decision reds a
repeated-producer witness (not only a single-op or heterogeneous one). A
discriminator that is blind to exactly the class the node added does not certify
that class.

## Acceptance criteria

- **AC-C1.** The S4 boundary is positively witnessed — a control that shows the
  refusal (or documents non-constructibility at this surface), with the failure
  attributable to the fail-closed boundary and not to an unrelated trap.
- **AC-C2.** A cross-class program is shown NOT to enter the repeated-producer
  path through the `Ok(Vec::new())` guard; the control reds if the guard is
  mutated to admit the foreign class.
- **AC-C3.** A repeated-same-producer witness reds under a mutation to the
  `SuppressLocalContinuationDrive` / owner-identity decision — the discriminator
  is proven to cover the new class.
- **AC-NO-REGRESSION.** Whole-suite green in CI (COORDINATION section 12);
  targeted local `-p ken-runtime` / the affected `--test` only.

## Why this is coverage, not a re-open

The capability is sound and merged; nothing here changes runtime behaviour or
the TCB. These are controls that make the existing invariants FALSIFIABLE at the
new class boundary — the difference between a suite that would catch a future
regression in this capability and one that would not. Sizing is S; the runtime
ring confirms reachability of each control at pickup and hard-stops to the
Steward if any control cannot be constructed at the current surface.
