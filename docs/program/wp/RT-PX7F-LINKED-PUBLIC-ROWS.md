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

### The mechanism — CORRECTED. The check is right; the carrier is wrong.

> **The reading first published in this section was WRONG and is replaced
> here rather than annotated, because a superseded claim left in place is the
> copy a later reader obeys.** It said equal `len` with differing `start`
> meant "the same constructor interned at a different position", and therefore
> that the check compared interning position where it needed constructor
> equality. **`len` is the byte-length of the name span, not a shape.**
> Refuted by @architect (`evt_3p21wmxc54ytk`) before any repair was proposed.

The failing comparison is over a packed `ConstructorIdentity`
(`pack_identity`: `((start << 32) | len) + 1`). Decoding the spans against
`names` — which is the measurement the previous reading skipped:

    row              expected                           actual
    right_denial     "ctor:right-denial::ITree::Ret"    "ctor:right-denial::ITree::Vis"
    double-release   "ctor:double-release::ITree::Ret"  "ctor:double-release::ITree::Vis"

**`Ret` and `Vis` are both three bytes.** That is the whole reason `len`
matched, and it is why equal-`len` was never evidence of anything: it is a
*value*, and this node's parent census states the rule that disqualifies it —
fold on a mechanism, never a symptom, a value, or a label. I applied that rule
correctly to reject folding rows 1 and 2 this morning and then broke it one
layer down on `len`.

⇒ **Outcome `(a)`: the bytes DIFFER, so the check is CORRECT and there is no
interning defect.** The planner demanded `ITree::Ret` and emission carried
`ITree::Vis`. `names.len()` also differs per row (5613 vs 5721), confirming
each row decodes against its own arena and that the cross-decodes are garbage,
as expected.

**`D1` is therefore a different question than this node assumed:** why does
this row's response carry the `Vis` constructor where the plan says `Ret`?
Both prior arms are dead — arm 2 on design grounds (emission only reads the
planner's value and never mints), and arm 1 because its premise required the
bytes to be equal, which they are not.

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

### THE QUESTION THIS LEAVES

Not "which side is authoritative" — that fork is dead, both arms with it. The
live question is **why the response carries `Vis` where the plan says `Ret`**,
which is a question about what the response-K context returns, not about
identity encoding.

`AC-2` is **NOT** discharged: the rows have not moved and this candidate lands
no `crates/` file. What is landed is the measurement `D0` asked for, which was
step one and is now durable rather than living in a thread.

## Two cheap reads, ahead of `D1` proper

### Read 1 — NO COLLISION. Neither composed-return node owns this.

    RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION  ready  D0-only feasibility:
      can a fresh declared-call result be CO-LOCATED with a predeclared Ret
      sink at the true StaticWorker producer. Lands no production.
    RT-COMPOSED-RETURN-RUNTIME-CLOSURE           draft  HELD as FALLBACK,
      explicitly NOT the selected mechanism; a runtime-closure repair route.

Neither witness condition is *"the response carries `Vis` where the plan
demands `Ret`"*, and **neither node names `px7f` or either of these two rows.**

**The one near-miss, checked rather than assumed:** the runtime-closure node's
title does contain `Vis` — three times — but it is quoting normative spec
`42 §6.2`, `Vis e k -> apply k (H e)`. That is the effect-tree constructor in a
semantics formula, not a witness condition about an identity mismatch. A
grep on the token alone would have read as a collision.

⇒ `D1` is this node's, as re-posed.

### Read 2 — it genuinely IS a `Vis`, and the fixtures localize WHICH one

The spec formula above is the discriminator: `Vis` **is** "performs an effect,
with a continuation"; `Ret` is "returns". So the question is whether the
response continuation performs a second effect. Read from the two fixtures:

    fixture          INNER continuation (resource body)   OUTER (after main's bracket)
    RIGHT_NOT_HELD   metadata_after -> Ret, both arms     after_right_outer -> host_exit
    DOUBLE_RELEASE   double_release_after_first -> bind   double_release_done -> host_exit
                     a SECOND effect

**The two fixtures DISAGREE on the inner shape and AGREE on the outer**, and
both rows produce the identical `Ret`-versus-`Vis` mismatch. A site whose shape
differs between the fixtures cannot explain a mismatch that is the same on
both; the outer continuation can. ⇒ **The site is main's continuation after
the bracket**, in both rows, and in both it performs `host_exit` — an ambient
operation, hence a `Vis`.

⇒ **Outcome (1) on the bytes: the carried constructor really is a `Vis`.**

**THE LOCALIZATION ABOVE IS FALSIFIED. The site is NOT main's continuation.**
The hardening @runtime-leader called for was authored and it refuted the
argument it was built to test.

### The falsifying control, and what it cost to trust it

`RET_ONLY_OUTER` differs from `RIGHT_NOT_HELD` on exactly one axis:
`ret_only_after` returns an `ExitCode` via `Ret` in both arms instead of
calling `host_exit`. Inner continuation already `Ret` in both, so this is the
`Ret`/`Ret` case the residual named as the falsifier.

**It still fires `units.rs:3430`** — the same `require_i64` K-ret check as the
two rows — verified by decoding the tag against that run's own registry, with
0 tags mapping to two sites. So changing the outer continuation from a `Vis`
to a `Ret` does **not** avoid the mismatch, and the cross-fixture argument
that the outer site explained it is **wrong**.

> **A near-miss worth recording, because it would have destroyed a valid
> result.** An intermediate run reported plain `-1` with `require_i64`
> supposedly tagged, which reads as *"the control never reaches the K-ret
> check"* and would have voided the control as failing for an unrelated
> reason. That reading was an artifact: the patch's anchor matched a
> **different function first**, so `require_i64` was never actually
> instrumented in that build. Caught only by re-running with uniform
> instrumentation over every `-1` emitter. **A probe that silently did not
> apply is indistinguishable from a probe that applied and found nothing** —
> the tag registry is what separates them, and a bare `-1` says only that no
> INSTRUMENTED site fired.

### Where `D1` actually stands

All three programs — both ignored rows and a `Ret`/`Ret` control — reach the
same check and fail it. The mismatch is therefore **not** explained by the
outer continuation's shape, and the live hypothesis is that it is general to
this `withResource`-plus-`bind` family rather than specific to a continuation
that performs a second effect. **That is a hypothesis and nothing here tests
it.**

**No label may be written yet.** `AC-2`'s second branch requires stating *why*
the refusal is terminal, and the mechanism statement that would have gone into
it is the one just refuted.

### Hard stop reached, reported rather than worked around

`RET_ONLY_OUTER` is a **failing** test. Committing it green is impossible and
`#[ignore]`ing it is this frame's first hard stop — *a row that needs a new
`#[ignore]` anywhere to make progress*. It is therefore **recorded here and
removed from the suite**, not landed. The program text is above; it reproduces
in one run.

⇒ Disposition is **undetermined**. It may still be `AC-2`'s second branch, but
that cannot be asserted until the site is known, and the previous text
asserting it is withdrawn.
