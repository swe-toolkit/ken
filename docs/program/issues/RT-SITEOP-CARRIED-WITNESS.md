---
id: RT-SITEOP-CARRIED-WITNESS
title: "Site-bound operand reader cannot witness a carried value — a synthesized SiteOperand demands a compile-time Lowered template from the same seat byte-span activation wants carried"
status: active
owner: runtime
size: L
gate: none
depends_on: [RT-CARRIER-BYTESPAN-OBSERVE]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: Hard stop returned by RT-CARRIER-BYTESPAN-OBSERVE D5, 2026-08-07, candidate 4244d082. The frame's own §1a recut clause fired — the 30 quarantined rows do not discharge from one mechanism. Steward-cut per that clause. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # DISPATCHED 2026-08-17 — THE FORK IS RULED AND THIS NODE IS ON THE CRITICAL PATH
>
> **Kicked to the runtime ring at `evt_gwrw3dkpt577`, base `origin/main`
> `02f255fc1`, after the full handoff gate** (ring quiescent, all three home
> branches confirmed carrying current `agent/COORDINATION.md`, all three
> compacted and verified per-pane). **`active` from release** so the node is not
> invisible to a frontier audit while it is being worked.
>
> **The dispatch is `D1b` ONLY.** See the start-here block below.
>
> **The one bar was §3 of its frame: an open Architect fork on the mechanism.
> The Architect ruled it at `evt_559gymspqap8w`, and the ruling is pasted
> verbatim into `§3b` of the frame** — read it there, not from the event. Its
> sole `depends_on`, [[RT-CARRIER-BYTESPAN-OBSERVE]], is `merged`.
>
> **The ruled mechanism:** project the carried word to runtime `(pointer, len)`
> through an **emitted helper** and admit that as the site operand's value —
> §2g's sanctioned route, not the banned `Carried -> Lowered` inverse.
>
> ### THIS NODE ACQUIRED A DEPENDENT IT DID NOT HAVE. `blocks` was `[]`.
>
> [[NATIVE-HANDLE-CARRIER]] hard-stopped on **this exact gap**
> (`evt_4eynen6drs79x`, 2026-08-17): its first native refusal is *"seat
> `Argument(0)` of `FsReadFile` needs `BytesPointerLength`, which it cannot
> observe in `CarriedWord`"*. The Architect ruled the fix **does not belong in
> that node** — the component that must change is synthesized error-value
> construction and site-operand provenance. ⇒ **This node is its successor**,
> and through it heads **19 transitive dependents**.
>
> ### START AT `D1b`, AND STOP WHEN IT REPORTS.
>
> **`D1b` answers the one premise the Architect deliberately did not walk:** is
> the synthesized `FileError`'s child read as a **template** anywhere downstream
> (erasure, checked-core body view)? **If it is, the ruled direction is wrong by
> the Architect's own terms** and this returns to the Architect.
>
> **`size: L` is the PRE-RULING provisional and is not evidence of anything.**
> The Architect held sizing until `D1b` reports, *"because a plumbing answer and
> a representational answer are not the same node."* The recut is the Steward's.
>
> **Do not read `L` and plan a long campaign; do not read `ready` and start at
> `D2`.**

## The gap

Each `Fs*` path seat is consumed **twice**:

1. as a **wire span** — which `RT-CARRIER-BYTESPAN-OBSERVE`'s `D4` observer
   satisfies at every measured seat; and
2. as **`SiteOperand(0)`** of the synthesized `FileError`'s
   `Option::Some(<site path>)`, which demands a **compile-time `Lowered`
   template**.

Supplying (2) from a boundary word is the `Carried -> Lowered` inverse that §5
bans. So the same seat cannot be both `EITHER_PHASE` and a site-bound operand.

```rust
// lowering/mod.rs:11354-11362 — the sole template projection
fn site_operand_argument(&self, seat: StaticOriginId, index: u32,
                         seats: &ClaimedEffectSeats<'_>) -> Result<..> {
    let value = seats.specialized(EffectSeatSlot::Argument(index))?.clone();
    //                 ^^^^^^^^^^^ requires the compile-time template
```

`mod.rs:11650-11654` states the consequence in its own voice: a declared
`SiteOperand` whose claimed operand is carried *"refuses at that exact seat,
propagated from `specialized`. It does not reconstruct a template, widen the
carrier, borrow a sibling, or fall back — reconciliation needs a compile-time
witness, and there is none."*

## How it was established

**Two independent routes, which is why it is stated as measured rather than
diagnosed.**

- **Runtime implementer, stepwise at `4244d082`:** baseline refuses at
  `FsWriteFile Argument(0)`; flipping `Argument(0)` moves the refusal to
  `Argument(2)`, proving seat 5 is real; flipping both returns it to
  `Argument(0)`, now from the template projection, past the claim gate. All 26
  lowering refusals across ten files reduce to this one cause, with **zero
  failures of any other kind.**
- **Steward, structurally:** the two source sites above, read directly. Not a
  re-run of the implementer's measurement — a different route to the same
  place.

## What it owns

- **29 of the 30 `#[ignore]` rows** quarantined under
  [[RT-CARRIER-BYTESPAN-OBSERVE]], across 10 files.
- **The four seats left `SPECIALIZED_ONLY`** by that node's `D5`:
  `(FsReadFile, 0)`, `(FsWriteFile, 0)`, `(FsChangeMode, 0)`, `(FsOpen, 0)`.
- **The `D6` activation-gate discharge pass**, moved here because its premise
  is "the activation", and this node is where the activation completes.

## Frame

`docs/program/wp/RT-SITEOP-CARRIED-WITNESS.md`.
