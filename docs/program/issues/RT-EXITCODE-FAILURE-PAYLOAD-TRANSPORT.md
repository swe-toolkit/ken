---
id: RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT
title: "M-series successor (M3+1) — close the process-exit consumers over the existing exact-Int carrier forms: a persistent/dynamic ExitCode::Failure payload is admitted only as ImmediateInt, so both the carried-phase (core.rs:11523) and specialized-phase (calls.rs:2301) exit-status producers force it to native sentinel -3 instead of mapping it to an exit code"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-IH-DISPATCH-SITEOP]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect object-distinctness ruling (evt_317adj9ebfw86) and the landed-SHA re-measure + mechanism correction (evt_4kkspzs62gtn6, thr_7m88b1hsemj9c). Sequenced FIRST of M3's two successors (Architect: ExitCode before RT-RETAINED-UNIT-CALL-TARGET-DERIVATION, no fold — the ExitCode cut is smaller because the exact-Int representation and observer already exist). Re-anchored to landed origin/main 5fff430db (Architect re-ran the witness at that exact commit; object unchanged). Steward framing call per COORDINATION section 2. CLOSED 2026-08-25 — FALSIFIED as a product object by the Architect hard-stop #3 ruling (evt_1vhmndq7fscd1); recut as RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE."
---

> # CLOSED / FALSIFIED 2026-08-25 — READ THIS BEFORE ANYTHING BELOW
>
> The Architect FALSIFIED this WP as a product object after the hard-stop #3
> research advisory (ruling evt_1vhmndq7fscd1, thr_305pn5gzx37h). The durable
> finding: the exact-Int carrier already admits every valid exit code under the
> old policy, and the two named process-exit consumers are NOT missing transport;
> every final/named exit-representation marker was bypassed. The causal defect is
> a dynamic-constructor dispatch residual — `emit_carrier_dynamic_constructor`'s
> direct `return_(-3)` at `StaticOriginId(34)` — not an ExitCode payload gap. The
> three consecutive hard stops shared one predicate (a downstream semantic
> classification used as upstream producer/provenance authority), which this
> frame embeds in its objective, sites, controls, and diagnostic — so it is
> REPLACED, not amended.
>
> - Do NOT resume D1. Do NOT ship the production refactor in `34ab178ac` (kept
>   READ-ONLY as the load-bearing probe checkpoint only).
> - Replacement: [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]] (ready, fresh WP
>   thread; hard-stop count reset to zero — different design question).
> - The `-3` reporter alias (an independently proven honesty defect) is tracked
>   SEPARATELY as [[RT-UNIT-FAILURE-STATUS-PROVENANCE]]; it was NOT folded in.
> - [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] remains distinct — no dependency
>   or sequencing change.
>
> Everything below is retained for provenance and is superseded.

> # M3 successor, ready to frame — re-anchored to landed 5fff430db
>
> Sequenced first of M3's two distinct successors (Architect evt_4kkspzs62gtn6).
> The pre-landing stub's mechanism statement ("no durable native transport") was
> too broad; corrected below to the consumer-not-closed-over-existing-forms
> defect the Architect measured at landed 5fff430db. Full frame:
> `docs/program/wp/RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT.md`.
>
> AMENDED 2026-08-25 (Architect hard-stop #1, evt_3kprh7knmxa3w): the
> single-observer reading below ("close both consumers over the landed exact-`Int`
> observers" read as one helper) is refined — surface B (specialized) stays on
> `narrow_native_int_u64`; do NOT convert it to a carrier. The carried decoder
> outcome is factored below its effect-seat policy wrapper, and one shared exit
> mapper serves both surfaces; the carried process wrapper yields `-3` (not `-1`)
> on malformed input. Authoritative component shape + controls are in the frame's
> Deliverables/ACs. Only "`narrow_carried_int_u64` sufficient at each phase" is
> withdrawn; the two-surface census stands.

## Objective (Architect-corrected mechanism, measured at 5fff430db)

A checked program that crosses M3's effect seat runs to process exit and its
`ExitCode::Failure` payload is forced to the native sentinel `-3`
(`ken native trap: malformed ExitCode::Failure payload`). The defect is NOT a
missing representation: exact `Int` already has both immediate and persistent
carrier forms, and `effects.rs:1589-1700` already provides the
representation-blind observer `narrow_carried_int_u64`. The defect is that the
process-exit CONSUMERS are not closed over those existing forms — a persistent /
dynamic exact `Int` is admitted only as `BoundaryTag::ImmediateInt` and every
other form falls straight through to `-3`.

Two producer surfaces force `-3`, in two phases (both must be reconciled — the
Architect's probe replacing only the carried immediate-only arm did NOT green
the witness, so a one-site patch is insufficient):

- `core.rs:11523-11577` `transfer_carried_failure_exit_status` (carried phase):
  admits only `ImmediateInt`; every persistent exact `Int` goes directly to `-3`.
- `calls.rs:2301-2370` `emit_process_exit_status` (specialized phase): the
  sibling; also produces `-3` on an un-narrowable dynamic `Int`.
- `object_linker_packaging.rs:2223` only REPORTS the sentinel; it is NOT the
  defect site (the pre-landing stub misnamed it as the site).

Close both consumers over the landed exact-`Int` observers, then apply one
canonical exit mapping (`0 -> 1`, `1..=255 -> value`, out-of-range/malformed ->
`-3`). Do not add a third `Int` representation or duplicate the persistent
decoder.

## Scope boundary — this WP does NOT promise px8ta goes green

The fresh trigger trace (`px8ds_real_same_depth_path_runs_exact_edges`, HALF B)
is still exactly `BufferAllocate`, `ResourceRelease`, with ZERO
`ConsoleIsTerminal`. So faithful transport may expose `Failure 91/92` rather than
`Success`. Object-level completion is: the process-exit boundary FAITHFULLY
TRANSPORTS or HONESTLY REJECTS the payload — not that px8ta ends green. If px8ta
becomes genuinely end-to-end green, un-ignore it; if it advances to a distinct
nonzero outcome with the missing effect still causal, STOP and re-point that
successor rather than widening this cut (Architect ruling).

## Sequencing

First of M3's two successors. [[RT-RETAINED-UNIT-CALL-TARGET-DERIVATION]] stays
second and separate (it crosses planner ownership + function-local call-target
derivation — a larger cut). No technical dependency between them; this is
simplest-first sequencing.
