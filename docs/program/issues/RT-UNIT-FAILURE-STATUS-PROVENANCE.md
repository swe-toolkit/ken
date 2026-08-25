---
id: RT-UNIT-FAILURE-STATUS-PROVENANCE
title: "Structural closure over generated-unit failure-status provenance — a generated-unit failure that carries identity is collapsed into a globally-interpreted scalar at the root trap-exit authority, so the process reporter classifies a bare number instead of the failure's origin/kind. Two instances under ONE predicate: the TrapWord to -4 identity loss at calls.rs:2075-2090 (TrapExitAuthority::Root, identity_preserved:false) that turns the positioned InvalidOffset parity rows into RuntimeTrap(4)/explicit-entry-trap, and the earlier MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS == -3 reporter alias. Preserve the planned trap identity to the reporter via the existing typed trap/failure authority — never a new sentinel."
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]
blocks: []
github: null
origin: "Steward, 2026-08-25, recut from the Architect umbrella arm(b) ruling (evt_7jpt4hm2nm6hh, thr_ep9mqam5fs8b) on RT-NATIVE-CARRIED-VALUE at merged d9bc68db0. px8ta closed but the four native full-program witnesses do NOT: three semantic blockers remain. This node is the SemanticErrorV1 blocker and the Architect directed it recut from the narrow -3 honesty stub into the structural closure over generated-unit failure propagation, folding in the newly-bound TrapWord->-4 identity loss. Explicit Architect instruction: do NOT mint a third node; do NOT promise reporter honesty alone greens SemanticError. Prior draft origin: Architect hard-stop #3 evt_1vhmndq7fscd1. Steward framing call per COORDINATION section 2."
---

> # RECUT 2026-08-25 — structural closure (Architect umbrella arm(b))
>
> The Architect's umbrella object read at merged `d9bc68db0` ruled arm (b):
> RT-NATIVE-CARRIED-VALUE is NOT at its four-value closure, and the residual is
> semantic, not an un-ignore of the px8ds 256 MiB policy. Of the three remaining
> blockers, THIS node is the SemanticErrorV1 one, recut per the Architect's
> direction from the narrow `-3` reporter-honesty stub into the structural closure
> over generated-unit failure propagation — folding in the newly-bound
> TrapWord->`-4` identity loss at `calls.rs:2075-2090`. Do NOT mint a third node;
> the `-3` alias and the `-4` TrapWord loss are one predicate.
>
> FRAME FIRST in the lane-1 sequence (Architect recommendation): it restores the
> identity needed to name the positioned parity row's next causal object. The
> second remaining object [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]]
> (ReadSome/Wrote, `calls.rs:1631-1640`) is sequenced AFTER and MUST NOT co-run —
> both touch `calls.rs`. Its dependency
> [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]] is merged. Base the branch on
> `37af4636` or later main.

## Objective

A generated-unit failure that carries a distinct identity is collapsed into a
globally-interpreted scalar at the root trap-exit authority, so the process
reporter classifies the bare number rather than the failure's origin/kind. Carry
the failure's planned trap identity through to the reporter — via the existing
typed trap/failure authority — so it is classified by origin/kind. Cover both
instances under one mechanism.

## Fixed inputs (Architect arm(b), grounded at merged `d9bc68db0`)

- SemanticErrorV1 witness rows (the `-4` instance):
  `rt_parity_native::fs_read_at_malformed_offset_narrows_to_invalid_offset` and
  `fs_write_at_malformed_offset_narrows_to_invalid_offset`. Both build and run,
  settle their resources, but emit NO `FsReadAt`/`FsWriteAt` event and terminate
  `exit_status = 1`, `terminal_error = RuntimeTrap(4)`, stderr `explicit entry
  trap` — instead of the interpreter's exact `InvalidOffset` SemanticErrorV1.
- First authority (Architect-bound, not the wording): `calls.rs:2075-2090`. A
  nonzero generated-unit `TrapWord` reaches
  `TrapExitAuthority::Root { process_sentinel: true, ... }` with
  `identity_preserved: false` and `return -4`. Proof: changing ONLY that natural
  return from `-4` to `-44` changed the same row's terminal observation from
  `RuntimeTrap(4)` to `RuntimeTrap(44)` and the reporter to `unknown terminal
  sentinel`, effect prefix otherwise unchanged (log SHA-256
  `a294ee06dfa08cd97e6fd68d89df49dceb184287f93466b34b3123ceef5acf44`). Production
  restored to `calls.rs` blob
  `d4e056b330f4bf2d78010be613d6511c42ab8774`; object worktree clean.
- The `-3` instance (folded in): `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS == -3`,
  emitted by `emit_carrier_dynamic_constructor`'s residual, reaches the process
  reporter classified as "malformed ExitCode::Failure payload" — a sentinel alias.
  The selected path never produced an ExitCode failure; a generated unit returned
  an internal mismatch scalar directly, forwarded unchanged by
  `call_declared_unit_target`.

## The structural predicate (unifies both instances)

A generated-unit failure that carries identity (an origin/kind) is collapsed into
a globally-interpreted scalar at the root trap-exit authority
(`identity_preserved: false`), so the reporter classifies a bare scalar (`-4`,
`-3`) rather than the failure's actual origin/kind. This is the same predicate
that produced the three ExitCode hard stops — a downstream classification
standing in for upstream producer identity — so the fix is provenance-carrying,
not a new magic number.

## Deliverable

- D1 — at the root trap-exit authority (`calls.rs:2075-2090`) and the `-3`
  forwarding path, preserve the generated-unit failure's planned trap identity
  through to the process reporter so the reporter classifies by origin/kind, not
  by a globally-interpreted scalar. Use the EXISTING typed trap/failure authority
  (or one subsuming envelope) — do NOT allocate another uncoordinated sentinel.
  Both the `-4` TrapWord loss and the `-3` alias close under this one structural
  mechanism.

## Critical boundary (Architect — do not overpromise)

Preserving the planned trap identity MAY expose a distinct underlying semantic
producer for the `InvalidOffset` SemanticErrorV1 rows. If it does, that producer
gets an OBJECT READ before any repair — it is NOT repaired under this node. This
node does NOT promise that reporter honesty alone greens the SemanticErrorV1
rows. Its success is identity preservation to the reporter; a hard stop into an
Architect object read on an exposed distinct producer is a GOOD outcome.

## Acceptance criteria

- AC-1 — a generated-unit failure carrying a distinct planned trap identity is
  reported WITH that identity (origin/kind), not collapsed to a bare global label.
  Proven positively on a failure whose identity differs from the global `-4`/`-3`
  scalars.
- AC-2 (the `-4` instance) — at `calls.rs:2075-2090`, the nonzero generated-unit
  `TrapWord` no longer collapses to `identity_preserved: false` / `return -4`; the
  planned trap identity survives to the reporter.
- AC-3 (the `-3` instance) — the `MALFORMED_DYNAMIC_CONSTRUCTOR_STATUS == -3` path
  no longer reaches the reporter as a bare scalar; it carries origin/kind under
  the same mechanism (folded, not a separate sentinel).
- AC-4 — fail-closed backstop preserved: a genuinely unclassifiable failure still
  refuses; NO new uncoordinated sentinel is allocated (the fix reuses the typed
  trap/failure authority).
- AC-5 (the object-read arm) — IF preserving trap identity exposes a distinct
  underlying semantic producer for the `InvalidOffset` rows, the WP HARD-STOPS
  with that producer characterized (layer + first authority) for an Architect
  object read; the SemanticErrorV1 rows are NOT promised green by this node alone.
- AC-6 (mutation controls) — reintroducing the `identity_preserved: false` / `-4`
  collapse REDS AC-2; a mutation that re-forwards the bare `-3` scalar REDS AC-3.
- AC-7 — `calls.rs` is the production surface; do NOT un-ignore the witness rows
  here (that is the later closure fold); zero `trusted_base()` delta.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` / `-p ken-verify` only, never `--workspace`.

## Reviewers

Architect (the identity is preserved to the root reporter via the existing typed
authority, not a new sentinel; the `-4` and `-3` instances are one structural
mechanism; a distinct exposed producer hard-stops to an object read rather than
being repaired here) + runtime-qa (the identity-collapse-reintroduction mutation
reds AC-2/AC-3; the fail-closed backstop is intact; no new sentinel). No Decision
fork — the provenance contract determines the answer; the AC-5 hard stop routes to
an Architect object read, not a Decision.

## Capability tier

T1 — the review turns on the provenance / identity-preservation argument at the
root trap-exit authority, and on the judgment of whether an exposed producer is a
distinct object. Size M (widened from the narrow `-3` stub: two instances, crosses
the root trap-exit authority, one structural mechanism).

## Sequencing

Lane-1 (runtime, priority). FIRST of the two remaining semantic objects in the
RT-NATIVE-CARRIED-VALUE arm(b) critical path (Architect recommended order),
because it restores the identity needed to name the positioned parity row's next
causal object. The SECOND object [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]]
(ReadSome/Wrote, `calls.rs:1631-1640`) is sequenced AFTER and MUST NOT co-run —
both touch `calls.rs`. After both land plus a fresh four-value object read, the
umbrella owes a final ReadEof-witness / un-ignore / CI-rearm closure fold. Base
the branch on `37af4636` or later main.
