---
id: RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL
title: "A vestigial specialized-only gate on FsReadAt's Argument(2) buffer reply/ok-construction path (effects.rs:3226) re-refuses a carried buffer that the request path already admits; the destructured span_origin is UNUSED (the span is projected from the operand list at 3233), so the fix is REMOVAL of the dead gate, not a reroute -- the ResourceScalar-family leftover reader RT-EXACTINT moved onto the cap41_* critical path"
status: ready
owner: runtime
size: S
gate: none
depends_on: [RT-EXACTINT-CARRIED-OBSERVE]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Adversary M8 completeness flag on the landed [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] route (evt_5wx3bax63yak); Architect removal-not-reroute ruling (evt_2qdpkfvtqrxzy: 3226's destructured span_origin is unused, the span is projected from site_operand_argument(.., 2, ..) at 3233, so the specialized(SEAT_2)? match is a vestigial gate whose only post-D1 effect is the spurious carried-buffer refusal); runtime-implementer critical-path re-disposition (evt_6vxb4f1rxh3jk: with ExactIntU64 closed the witness terminal is now this Arg(2) reply-path refusal, so the deferral's off-critical-path ground is invalidated and it must be re-dispositioned from a carry to a cut). Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

The removal of the vestigial specialized-only gate at `effects.rs:3226` on
`FsReadAt`'s `Argument(2)` buffer reply/ok-construction arm. After
[[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] made the REQUEST-arm resource-token
readers carried-capable and [[RT-EXACTINT-CARRIED-OBSERVE]] closed the
`ExactIntU64` terminal, the `cap41_*` rows advance to this gate: it refuses a
carried buffer on the reply arm -- `seat Argument(2) of FsReadAt needs
ResourceScalar, which this release can observe only in a specialized template,
but this visit holds a carried word`.

This is a `ResourceScalar`-family item (RT-RESOURCE-RELEASE's leftover reader),
NOT `ExactIntU64` work. It is kept distinct in the accounting from
[[RT-EXACTINT-CARRIED-OBSERVE]] even though it shares the `effects.rs` edit
region for contention.

# WHY REMOVAL, NOT REROUTE (Architect ruling `evt_2qdpkfvtqrxzy`)

The Architect read `3226`/`3233` directly: the destructured `span_origin` is
UNUSED. The constructor binds the span from `site_operand_argument(builder,
static_origin, 2, &seats)` (the operand-list projection) at `3233`, with the
code's own comment "projected from the operand list rather than rebuilt from its
destructured payload." So the `specialized(SEAT_2)?` match is a VESTIGIAL GATE:
its value is discarded; its only post-D1 effect is the spurious carried-buffer
refusal.

Routing `3226` through `lower_resource_token_seat` (the reroute an earlier note
named) would be WRONG: it adds a guarded observation whose scalar result is
thrown away -- a dead read wearing the shared-observation costume. The correct
fix is to REMOVE the vestigial gate (delete the specialized-only destructure
whose value nothing consumes), so a carried buffer is no longer spuriously
re-refused on the reply path.

# HOW THIS REACHED THE CRITICAL PATH

The runtime-implementer originally dispositioned this DEFERRED on three grounds
(evt_6vxb4f1rxh3jk): (a) closing it is a mechanism change, not a fold; (b) the
direction is safe (a clean refusal, no miscompile); (c) it was off the `cap41_*`
critical path, because the rows hit the `ExactIntU64` terminal at `Arg(1)`
first. Closing `ExactIntU64` is precisely what moves it onto the path -- the
witness terminal is now this `Arg(2)` reply-path refusal. Grounds (a) and (b)
stand unchanged; ground (c) is invalidated, which is why the disposition is now
a CUT (this node) rather than a carry. NHC blocks on it.

# `D0` -- CLASSIFY (first deliverable; the Architect's 1-3)

Measure and report, so the removal is confirmed clean before it lands:

1. Confirm `site_operand_argument(.., 2, ..)` projects the buffer argument
   correctly when `Arg(2)` arrives CARRIED -- that operand-list projection (at
   `3233`), NOT the destructured payload, is the live path that binds the span.
2. Confirm `Arg(2)` is already validated as a resource token on the REQUEST path
   (`2477` via `lower_resource_token_seat`), so the reply-path gate is a
   redundant re-validation whose removal drops only a spurious refusal, not a
   real check.
3. Direction: confirm removal enables no scalar misread -- the destructured
   value was already discarded, so nothing downstream reads it.

If (1)-(3) hold it is a clean removal; proceed to `D1`. If any of (1)-(3) needs
real design work, HARD-STOP and report with that argument -- the node re-scopes
rather than landing a removal that turns out to consume something.

# `D1` -- THE REMOVAL

Delete the vestigial specialized-only `specialized(SEAT_2)?` destructure whose
`span_origin` nothing consumes. The span continues to bind from the operand-list
projection at `3233`. No new route, no `Avail` change, no reader added -- this is
a removal, not a widening.

# ACCEPTANCE

- **AC-1 (the reply path admits carried).** The `FsReadAt` `Argument(2)`
  reply/ok-construction path no longer refuses a carried buffer; the `cap41_*`
  rows advance past this blocker. Report the full per-row disposition. A further
  distinct blocker exposed behind this one is a measurement to report and cut
  (or, if the rows go green, hand back to [[NATIVE-HANDLE-CARRIER]]'s
  `D-final`), not a failure of this node.
- **AC-2 (removal drops only a spurious refusal).** The request-path
  resource-token validation (`2477`) is unchanged and remains the real check;
  the removed gate consumed nothing (`span_origin` unused). State that removal
  enables no scalar misread and no relaxation of any real resource check.
- **AC-3 (no regression).** All currently-compiling lowering preserved;
  workspace-green in CI. (Local: targeted `-p` only, never `--workspace`; the
  respin gate is `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`,
  the coverage the predecessors ran.)
- **Required reviewer:** the Architect is the required reviewer on this node's
  merge Decision (soundness-adjacent completeness removal) and confirms the D0
  classification. Adversary hunts the landed code.

# EXPLICITLY NOT IN SCOPE

- **The `ExactIntU64` need** -- closed by [[RT-EXACTINT-CARRIED-OBSERVE]]. This
  node touches only the `ResourceScalar` reply-path gate.
- **Any `Avail` partition change, new route, or new reader.** This is a removal
  of a dead gate, not a widening.
- **The REQUEST-path resource-token validation (`2477`).** It stays; only the
  redundant reply-path re-validation is removed.
- **Any kernel / TCB edit.**

# CONTENTION

`ken-runtime` cranelift backend lowering (`effects.rs`), the same file region as
[[RT-EXACTINT-CARRIED-OBSERVE]] (predecessor). Single ring, single lane;
released to the runtime ring only after RT-EXACTINT merges (its D1 candidate is
in review). [[NATIVE-HANDLE-CARRIER]] is held on this node.

# CAPABILITY TIER

T2-leaning: a bounded removal of a vestigial gate on the Architect's direct read
of `3226`/`3233`, with a three-point D0 classification confirming nothing is
consumed. Escalates only if D0 finds any of (1)-(3) needs real design. Size S.
