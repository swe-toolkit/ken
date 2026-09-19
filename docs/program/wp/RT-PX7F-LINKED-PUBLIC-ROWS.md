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
  still `#[ignore]`d with a reason string obeying the **START-TOKEN RULE: the
  token the string OPENS with is whoever owns the row NEXT.**
  - **A live successor exists** — the string **opens with the SUCCESSOR's ID**,
    then states which check fires and why, then the provenance: established by
    this node, and what this node measured.
  - **No live successor** — the string **opens with THIS node's ID**, states
    which check fires and why it is terminal, and then says **in those words**
    that no live node owns the next step and that this node established that.

  A string that opens with this node's ID **while a live successor exists** does
  not satisfy this AC, and neither does one that opens with this node's ID and
  says only which check fires. At the flip this node is `merged`, and **`M7a`
  arm 1 matches on the START token, treating anything later as a mention that
  does not own the row** — so successor-mid-string mints exactly the unroutable
  row arm 1 exists to catch. The census's `AC-11` independently states the same
  rule — *"carries its successor owner at the START of its own `#[ignore]`
  string"* — so START-token ordering is what **both** consumers already read,
  and this AC was the outlier.
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

## 5b. Symptom inventory — ARMED

**Seeded late.** `§1b` makes seeding this the Steward's duty *at framing*, and
this node reached two hard stops without it. The entries below were reconstructed
from the thread by the Architect, which is exactly the recovery `§1b` exists to
make unnecessary: **the count is re-derivable from a thread and a pattern across
stops is not, and the thread is the first thing a compaction discards.**

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...
1. mismatch attributed to interning position — keyed on span LENGTH standing in
   for constructor identity. Refuted at D0: bytes differ (Ret vs Vis, both
   three bytes).
2. mismatch attributed to the outer continuation being a Vis — keyed on the
   OUTER CONTINUATION'S SHAPE. Refuted by RET_ONLY_OUTER: the Ret/Ret control
   still fires units.rs:3430.
```

**The predicate question is NOT answered at two, deliberately** (Architect,
`evt_3c4jfa2wjtby4`). Both entries share the shape *"localized by a property
that turned out not to discriminate"*, and that is cheap to see and usually
wrong at two. The trigger is mechanical at the third entry precisely so nobody
folds on a symptom — the error this node's parent census exists to name.

**Hard-stop count on this WP: 2.** The `§1a` research pull fires at 3.

## 6. Contention

Touches `crates/ken-cli/tests/px7f_resource_native.rs` for the label or the
un-ignore. **`RT-SITEOP-ORPHANED-ROWS-REFUSAL-CENSUS` names the same two rows in
its `AC-11` closeout roster.** Whichever lands first, the other re-derives —
coordinate through the runtime-leader rather than both editing the file.

## `D0` ANSWERED — both rows, same check, and the mechanism is decoded

Measured at `1f0402ba40faba71065a44948c3ccb791cad9138`, which is `origin/main`
and this candidate's merge-base, re-derived here. Probe reverted; the four
touched files are blob-identical to `HEAD` and `grep -rc RTPROBE crates/` is 0.

### The answer, per row as `AC-1` requires

    linked_public_right_denial_preserves_exact_masks              units.rs:3430
    linked_public_second_release_is_closed_and_the_handle_closes_once
                                                                  units.rs:3430

Both rows fire **the same** check, cited by symbol:
`Lowering::require_i64(ret_tag, expected_ret)` inside
`define_static_response_owner_bodies`, on the response-K context's returned
carrier. **The grouping is CONFIRMED, not refuted** — the discriminator was
worth running and it came back agreeing.

**Candidate (a) is ELIMINATED for these rows.** All four `require_nonzero`
checks in the entry adapter **pass**. The census's `AC-3` established that the
borrowed-ingress check is *emitted*; this establishes that it does not *fail*.
Both are true and they are different claims.

### The mechanism, decoded rather than described

The failing comparison is over a packed `ConstructorIdentity`
(`pack_identity`: `((start << 32) | len) + 1`). Decoded:

    row                expected                        actual
    right_denial       DenseRange{start=3323, len=29}  DenseRange{start=3055, len=29}
    second_release     DenseRange{start=3254, len=31}  DenseRange{start=2318, len=31}

> **`len` matches exactly on both rows. Only `start` differs.**

Same-length ranges at different offsets: the two sides denote a constructor of
identical shape interned at a **different position**. So the check compares
**interning position** where the property it needs is **constructor
equality** — the same identity-versus-equality shape as the `px7l`/`px7m`
rows' node-identity-versus-body-equality, one layer over. That neighbouring
node reached its version of this independently; recorded here because the
recurrence is evidence about the class, not about either row.

### Controls, so the reading is a measurement

- **Baseline** reproduces `-1` on both rows; build exit 0; 2 ran, 1 filtered.
- **Inertness** — probe compiled in, env unset: `-1` on both. Off changes nothing.
- **Forcing positive control** — force one check's invalid branch and the tagged
  value reaches `terminal_value`. Without this, a `-1` reading says only that
  nothing was observed.
- **Liveness** — 13 distinct `require_nonzero` sites registered per row,
  including all four adapter sites, so the instrumented path demonstrably ran.
- **Uniqueness** — 0 tags map to two sites. The first registry was
  `thread_local` and **libtest spawns a thread per test even at
  `--test-threads=1`**, so numbering restarted per test and one tag denoted two
  files. Caught by the registry's own redundancy; fixed by making it
  process-global.

**Instrument shape, for whoever repeats it:** `#[track_caller]` on
`require_nonzero` and `require_i64`, forwarding `Location::caller()` into a
process-global site registry. **26 `require_nonzero` and 101 `require_i64`
call sites covered without editing a single one.**

### THE FORK THIS OPENS, AND IT IS NOT MINE TO PICK

Which side is authoritative? Either the planner's `k_ret_identity()` carries a
**stale** `start` for a re-interned constructor, or the runtime carrier's tag
is interned against a **different table** than the planner's. The two repairs
are opposite — re-derive the planner's expectation, versus re-intern at the
emission site — and the measurement above does not choose between them.

`AC-2` is **NOT** discharged: the rows have not moved and this candidate lands
no `crates/` file. What is landed is the measurement `D0` asked for, which was
step one and is now durable rather than living in a thread.
