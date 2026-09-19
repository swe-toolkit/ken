---
id: RT-PX7F-LINKED-PUBLIC-ROWS
title: "Clear the two px7f linked-public rows. Both trap at runtime with UnclassifiedRuntimeTrap { terminal_value: -1 }; AC-3 of the census established that a process-mode compile emits four require_nonzero checks into the entry adapter via define_root_adapter, gated on the compile lane and not the body -- a property shared with every process-mode row in the workspace, so it does not individuate these two. WHICH of the four checks fires on each row is unmeasured, and that measurement is step one of this node, not its deliverable. DELIVERABLE IS THE ROWS, per AC-2: each row is either un-ignored and green in CI, or still #[ignore]d with a reason string that OPENS with the ID of whoever owns the row NEXT -- the successor's ID when a live successor exists, otherwise this node's own ID stating which check fires and why it is terminal. The relabel branch is first-class and is NOT a terminal refusal. If the two rows fire different checks the grouping is wrong, and saying so is a result, not a failure."
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-19, at origin/main 38b4ee59853e5405e97c94cdd2661393a891cc8a. Cut from RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS's AC-6 proposal, grouping G1 (landed at 38b4ee598), which proposed a node scoped to discriminating which adapter check fires. FILED AS A REPAIR NODE RATHER THAN A DISCRIMINATOR ON THE OPERATOR'S RULING, 2026-09-19, verbatim: 'It is too slow. You need to streamline the process and focus on forward movement, not management.' The discrimination is the first step inside this node; the node does not close on a report. Filed by the Steward because COORDINATION §2 reserves tracked-work creation to this seat -- raised by runtime-leader at evt_7nw102t7nc7z8 and confirmed."
---

> ## FRAMED 2026-09-19 — `ready`, size M, tier T1
>
> Frame: [`wp/RT-PX7F-LINKED-PUBLIC-ROWS.md`](../wp/RT-PX7F-LINKED-PUBLIC-ROWS.md)

**The rows**, both in `crates/ken-cli/tests/px7f_resource_native.rs` at
`38b4ee598`:

    :314  linked_public_right_denial_preserves_exact_masks
    :348  linked_public_second_release_is_closed_and_the_handle_closes_once

Both currently open `#[ignore = "RT-SITEOP-CARRIED-WITNESS …"`, which is
`merged`. This node is their live owner and clearing them is its product.

**What the census established and what it did not.** `AC-3` measured the
`define_root_adapter` emitter: four `require_nonzero` checks in the entry
adapter, two of them arena checks (`native_int_arena`, `boundary_arena`)
unconditional and two (`process_input`, `host_dispatch_context`) behind
`process_mode`; ten `iconst(I64, -1)` producers in the lowering tree. The
Architect reproduced this independently. **`define_root_adapter` has exactly one
call site**, which is what makes "and so does every process-mode row in the
workspace" hold rather than being a guess.

⇒ **The `-1` does not attribute a row.** Ten producers, and the terminal value
carries no mechanism. Any argument that reasons from the value alone is
refuted before it starts.

**The grouping is falsifiable by this node's own work.** The two rows are
grouped as candidates, not folded — their shared phase, shared terminal value
and byte-identical `#[ignore]` strings are a symptom, a value and a label, and
the census disqualified all three as fold criteria. If the discrimination in
`D0` returns a different check for each row, **the pairing is wrong and this
node reports that and splits.** That outcome is a result and closes the node's
first deliverable honestly; it is not a failed turn.
