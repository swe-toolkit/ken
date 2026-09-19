---
id: RT-CARRIED-RESOURCE-SCALAR
title: "RE-MEASURE BEFORE FRAMING 2026-09-19 — all three named rows are live and un-ignored; the FsHandleMetadata seat is refuted outright by the readmission of linked_public_escape_is_exact_closed, and the two FsWriteAt rows are NOT covered by that (this node's own per-seat argument forbids carrying one seat's result to another). WAS: A carried word cannot satisfy a ResourceScalar effect seat -- same Need-not-in-Avail shape as the byte-span gap, different need, different seats"
status: draft
owner: runtime
size: TBD
gate: none
depends_on: [RT-SRCBODY-BIND-ORDER]
blocks: []
github: null
origin: Measured at frozen base 21fd46dc by the RT-SRCBODY-BIND-ORDER D10 differential (evt_2jc88hbzfskpm). All 16 CI failures at aa032cc2 fail at the base too -- ZERO bind-order flips -- so this is pre-existing base debt, not a regression. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # RE-MEASURE BEFORE FRAMING 2026-09-19 (Steward). All three named rows are LIVE.
>
> Measured at `ba67e549f`: **none of the three rows under "Rows it owns" carries
> an `#[ignore]` attribute.** The node was filed so a skipped CI row would have
> an owner; there is no skipped row left to own under these names.
>
> **The `FsHandleMetadata` row is refuted outright.**
> `px7f_resource_native::linked_public_escape_is_exact_closed` was readmitted by
> `RT-IGNORED-PASSING-ROWS-DISPOSITION` (merged), and its readmission comment
> addresses this node's exact signature and disposes it:
>
>     The prior label read: "RT-CARRIED-RESOURCE-SCALAR: the FsHandleMetadata
>     seat cannot observe a carried word as a resource scalar", on the signature
>     "Effect: seat Argument(0) of FsHandleMetadata needs ResourceScalar, which
>     it cannot observe in CarriedWord", and asserted that the row "refuses at
>     object emission, so the program never executes". None of that still holds:
>     the program emits, executes, and the row passes un-ignored.
>
> That readmission rests on a mutation, not on the green, and took three attempts
> to find one that reached — so the row is a live control, not a vacuous pass.
>
> **The two `FsWriteAt` rows are NOT covered by that finding, and this node's own
> argument is why.** `px8f_buffer_native` and the `ken-verify`
> `px8f_write_partition` row cover a **different seat**. This file states the
> case against folding seats together — *"a per-seat capability gap, not a
> class-wide one"*, with `BufferFreeze` already carrying `ResourceScalar` at
> `EITHER_PHASE` — and that argument bites in this direction too. **One seat's
> refutation does not carry the other's**, so this node is not closed.
>
> ⇒ **Status stays `draft` and the frame is still owed, but the frame's first job
> changed.** It is no longer "repair the `ResourceScalar`/`CarriedWord` gap"; it
> is "measure whether an `FsWriteAt` gap still exists at all, given the
> `FsHandleMetadata` one does not." Do not inherit the signature block below as a
> current fact — it is the 2026 filing's measurement at frozen base `21fd46dc`,
> which is thousands of commits behind.
>
> ## THE FRAME IS OWED. This node is `draft` and NOT startable.
>
> It exists so that a **skipped CI row has an owner**. A skipped row measures
> nothing; the node that owns it owns **un-skipping** it. Size is `TBD`
> deliberately -- nothing measured bounds the repair, and a guessed size on this
> campaign has been wrong every time it was guessed.

## Exact signature

```text
Effect: seat Argument(0) of FsHandleMetadata needs ResourceScalar, which it cannot observe in CarriedWord
Effect: seat Argument(0) of FsWriteAt needs ResourceScalar, which it cannot observe in CarriedWord
```

## Rows it owns

- \`px7f_resource_native\` \`linked_public_escape_is_exact_closed\` (FsHandleMetadata)
- \`px8f_buffer_native\` \`linked_checked_write_all_observes_short_progress_and_matches_interpreter\` (FsWriteAt)
- \`px8f_write_partition\` (**ken-verify**) \`checked_write_all_reaches_full_short_zero_progress_flip_and_error_prefixes\` (FsWriteAt)

## Why this is NOT [[RT-CARRIER-BYTESPAN-OBSERVE]]

**The refusal has the identical SHAPE and a different NEED.** Byte-span is
scoped to \`BytesPointerLength\` seats and its frame states explicitly that
availability is **per seat, never a blanket phase relaxation**. These seats need
\`ResourceScalar\`. Folding them into byte-span on the strength of the matching
sentence would be the same-shape-different-population inference this campaign
has paid for repeatedly -- and it would violate that node's own scope.

Note the landed precedent cuts both ways: \`BufferFreeze\` \`Argument(0)\`/\`(3)\`
already carry \`ResourceScalar\` at \`EITHER_PHASE\` because their route emits the
helper. So this is a **per-seat capability gap, not a class-wide one**.

## Provenance

**Fails at frozen base `21fd46dc`, so it is not caused by the de Bruijn
binding repair.** Measured per row with `--no-fail-fast`; see the hazard note
in the D10 handback -- `cargo test` with several `--test` flags is fail-fast
**per binary**, and a partial run reads as a complete one.
