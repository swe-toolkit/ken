# `RT-PX7F-LINKED-PUBLIC-ROWS` — frame

**Owner:** runtime. **Size:** M. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `38b4ee59853e5405e97c94cdd2661393a891cc8a`.

> ## THE DELIVERABLE IS THE ROWS, NOT A REPORT.
>
> This node closes when `px7f_resource_native.rs:314` and `:348` are
> un-ignored and green, or when one of them carries a grounded terminal
> refusal. **Finding which adapter check fires is step one inside this node,
> not its product.** A candidate whose roster contains no change under
> `crates/` has not advanced it.

## 1. Fixed inputs, measured at `38b4ee598`

    crates/ken-cli/tests/px7f_resource_native.rs
      :314  linked_public_right_denial_preserves_exact_masks
      :348  linked_public_second_release_is_closed_and_the_handle_closes_once

Both fail as `UnclassifiedRuntimeTrap { terminal_value: -1 }`, phase `RUNTIME`.
Both open `#[ignore = "RT-SITEOP-CARRIED-WITNESS …"` — `merged`; this node is
their live owner.

**From the census's `AC-3`, reproduced independently by the Architect.** A
process-mode compile emits four `require_nonzero` checks into the entry adapter
via `define_root_adapter`, **gated on the compile lane, not on the body**:

    native_int_arena          unconditional
    boundary_arena            unconditional
    process_input             behind process_mode
    host_dispatch_context     behind process_mode

`define_root_adapter` has exactly **one call site**. Ten `iconst(I64, -1)`
producers exist in the lowering tree.

⇒ **`-1` does not attribute a row**, and the emitter is shared with every
process-mode row in the workspace, so neither the value nor the emitter
individuates these two. Coordinates are perishable — re-measure at your build
SHA and re-derive by symbol, not by line.

## 2. Deliverables

- **`D0`** — which of the four checks fires, **per row**, named from the tree.
- **`D1`** — the repair, or a grounded ruling that a row's refusal is correct.
- **`D2`** — the rows: un-ignored and green, or relabelled with the mechanism.

## 3. Acceptance criteria

- **`AC-1` — `D0` ANSWERED PER ROW, NEVER SUMMED.** Two rows, two named checks.
  A single aggregate answer fails this AC even if correct.
  *(Control: the instrument names a check on a program whose check is known,
  **and** reports "no check fired" rather than silence when none does. An
  instrument that cannot report the negative cannot report an answer.)*
- **`AC-2` — THE ROWS MOVED.** Each row is either un-ignored and green in CI, or
  still `#[ignore]`d with a reason string that **opens with this node's ID**,
  **states which check fires and why it is terminal**, and then **either names
  the live successor that owns the next step, or states in those words that no
  live node owns it and that this node established that**. A string that opens
  with this node's ID and says only which check fires and why it is terminal
  **does not satisfy this AC**: at the flip this node is `merged`, and `M7a` is
  keyed on exactly that.
  *(Control: the candidate's roster **contains** a change to
  `crates/ken-cli/tests/px7f_resource_native.rs`. Because it touches `crates/`
  this is **`full` CI, never doc-only**.)*
- **`AC-3` — NO REGRESSION.** Green in CI, never a local `--workspace` run
  (`COORDINATION §12`). Local work is `scripts/ken-cargo -p ken-cli --test
  px7f_resource_native`, nothing wider.

## 4. If the two rows differ, say so and split

The pairing rests on symptom, value and label — all three disqualified as fold
criteria by the census's `D2`. **If `D0` returns a different check per row, the
grouping is wrong.** Report it, split the node, and hand the second half back to
the Steward. That is a result, not a failed turn, and it is the property that
made this grouping worth running rather than assuming.

## 5. Hard stops — report, do not work around

- A row that needs a **new** `#[ignore]` anywhere to make progress.
- A repair that reaches outside `ken-cli`/`ken-runtime` lowering.
- `D0` cannot distinguish the four checks at all — that is a finding about the
  instrument and it outranks the repair.

## 6. Contention

Touches `crates/ken-cli/tests/px7f_resource_native.rs` for the label or the
un-ignore. **`RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` names the same two rows in
its `AC-11` closeout roster.** Whichever lands first, the other re-derives —
coordinate through the runtime-leader rather than both editing the file.
