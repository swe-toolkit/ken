# `RT-CARRIER-PRODUCER-OCCURRENCE` — frame

Owner: **runtime**. Size: **M**. Gate: none.
Depends on: `RT-SRCBODY-BIND-ORDER` (merged, `acfcc915`).
Origin record:
[`RT-CARRIER-PRODUCER-OCCURRENCE`](../issues/RT-CARRIER-PRODUCER-OCCURRENCE.md)

Ground: `origin/main` **`d18da5c6`** as originally written; **every source
coordinate was RE-GROUND to `origin/main`
`e75f1fe2768f1996c582bad6c6832e1e1716fc24` on 2026-09-18** — see the amendment
at the head of §1. Line numbers are hints, not anchors.

## 0. Posture

`c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload`
(`constructors.rs:3334`, `#[ignore]` at `:3333` -- hint, re-measured
2026-09-18) is `#[ignore]`d. It dies at
`.expect("the C2 carrier edge emits")` before its property is evaluated, so the
row currently measures nothing.

**Treat every anchor in this frame as perishable. If a fixed input turns out
false against the landed code, say so and escalate — do not quietly build
around it.** In particular §1f is a derivation I did not execute; §2 `D0` exists
to kill it cheaply if it is wrong.

**The refusal is a guard doing its job, and this node's default posture is that
the fixture is wrong, not the guard.** Read §4 before proposing any change to
PRODUCTION under `lowering/` outside `core/tests/`. **(This sentence used to say
`lowering/mod.rs`. The guard moved to `aggregates.rs`, which made it name the
one file the repair will never need — the same vacuity the amendment fixes in
§5's third hard-stop condition.)**

## 1. Fixed inputs

> # AMENDMENT 2026-09-18 (Steward) — EVERY SOURCE COORDINATE BELOW IS RE-ANCHORED
>
> **This frame was written at `368ff87e8` (2026-08-07). `c7f071bcb`
> (`RT-EMITTER-AGGREGATES-SPLIT D1`, a behaviour-preserving move to
> `lowering/aggregates.rs`) relocated every production site it names.** Nothing
> in the code is broken. Only the frame's aim was.
>
> Escalated by runtime-implementer (`evt_2n3wx07120cq6`) under §0's perishable-
> anchor rule, held by runtime-leader, independently confirmed by the Architect
> (`evt_24jpn6zngv87y`). **Every coordinate in this amendment was then
> re-measured by the Steward against `origin/main e75f1fe27`** — not taken from
> either report.
>
> **The implementer was right to stop.** Re-pointing a prohibition is a frame
> amendment and the frame is the Steward's. They located every site correctly
> and did not build around a fixed input they had measured false.
>
> ## WHY A DE-AIMED BAN IS WORSE THAN A DANGLING ONE
>
> **A line-anchored prohibition in a moving file does not fail by dangling. It
> fails by RE-AIMING onto a same-shaped neighbour, and the natural check returns
> YES.** Ban 1 read *"do not relax the refusal at `mod.rs:4994-5001`."*
> Measured at `e75f1fe27`, `mod.rs:4994-5001` **is a refusal** — a
> `StaticWorkerBinding` provenance/transition refusal. A reviewer asking *"is
> the refusal still there?"* gets **yes**. The ban verifies as satisfied while
> protecting nothing it was written to protect, and `mod.rs` contains **zero**
> occurrences of `source_aggregate_producer` or `reconcile_source_aggregate`.
>
> **The re-aim is structural, not bad luck.** A file is dense in the construct
> its bans are about; refusals cluster with refusals. A de-aimed refusal-ban
> lands on another refusal with near-certainty, so the new target is
> systematically the most confusing one available.
>
> **A PARTIALLY drifted anchor set is more dangerous than a wholly drifted one:
> the surviving anchor licenses the rest**, and the anchor a spot check picks is
> the cheap one.
>
> ## THE BAN FORM THIS FRAME NOW USES (Architect ruling, adopted)
>
>     ANCHOR      <symbol> :: <the property being protected>
>     file:line   a HINT, marked as one, never the anchor
>
> `AC-4` survived the split untouched because it is directory-scoped, and that
> is the property to copy. A symbol that moved is still found by name; a symbol
> deleted or renamed **errors**, which is correct for a prohibition whose
> subject is gone. Neither re-aims. **Treat every `file:line` in this frame as a
> hint that may already be stale, and resolve the symbol.**
>
> ## RE-ANCHOR TABLE — measured at `origin/main e75f1fe27`
>
> Paths are under `crates/ken-runtime/src/cranelift_backend/lowering/`.
>
> | frame says | symbol | actually, at `e75f1fe27` |
> |---|---|---|
> | `mod.rs:4994-5001` (§1a, ban 1) | `reconcile_source_aggregate`'s producer-occurrence refusal | `aggregates.rs:1198-1204`, in `reconcile_source_aggregate` (`:1191`) |
> | `mod.rs:9571-9580` (§1b) | `source_aggregate_producer` | `boundary.rs:1020` |
> | `mod.rs:4895-4980` (§1c) | `source_aggregate_preflight` | `aggregates.rs:831` |
> | `mod.rs:4866` (§1c) | the preflight-then-emit ordering | `aggregates.rs:1157`+`:1158` and `:1182`+`:1183` — **two sites, both preflight then `emit_carrier_transfer`** |
> | (`emit_carrier_transfer`) | `emit_carrier_transfer` | `aggregates.rs:1772` |
> | `mod.rs:11046` (§1f) | `synthesized_constructor` | `aggregates.rs:3459` |
> | `mod.rs:11064-11071` (§1f, ban 2) | its no-emission-owner early return | `aggregates.rs:3477-3488` — `let Some(owner) = self.defining_emission_owner else { return Ok(Lowered::Constructor { .. occurrence: None .. }) }` |
> | `mod.rs:4882-4887` (ban 1's citation) | the *"missing producer is a REFUSAL, never a fallback"* doc | `aggregates.rs:818` |
>
> **Fixture coordinates in `core/tests/constructors.rs` drifted about +780:**
>
> | frame says | symbol | actually |
> |---|---|---|
> | `:2548` | the `#[ignore]` attribute | `:3333` |
> | `:2549` | `c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload` | `:3334`, body ends `:3610` |
> | `:2654` | edge 1 `c2_host_result_producer` | `:3440` |
> | `:2721` | edge 2 `c2_ordinary_result_producer` | `:3507` |
> | `:2752` | edge 3 `c2_host_result_consumer` | `:3538` |
> | `:2626` | `ordinary_producer_origin` | `:3411` (used `:3494`, `:3529`) |
> | `:2730`, `:2734` | edge 2's hand-written `occurrence: None` | `:3515`, `:3519` |
> | `:2511` (ban 3) | `.expect("the C2 carrier edge emits")` | `:2564` — see the correction below |
>
> ## TWO CORRECTIONS TO THE ESCALATION REPORTS, both in the safe direction
>
> **1. Ban 3's subject is NOT lost, and `:2564` is not a coincidence.** The
> report reads `:2564` as an unrelated expect that happens to resolve. Resolved
> from the frame commit's own blob — `git show
> 368ff87e8:.../core/tests/constructors.rs | sed -n '2511p'` — **line 2511 at
> framing time WAS `.expect("the C2 carrier edge emits")`.** The same assertion
> is at `:2564` today. The ban's coordinate de-aimed; **its subject is intact
> and findable by text.** The warning still stands in full, because today's
> `:2511` is `&native,` — not an expect at all — so a literal reader of ban 3
> protects nothing.
>
> **Note for the repair: that assertion lives in the shared C2 rig near
> `ac_c7_run` (`:2483`), NOT inside `c2_ac4`'s body** (`:3334-3610` contains no
> such expect; `:3326` is a comment referring to it). Ban 3 protects a shared
> rig assertion, which is a stronger reason not to weaken it, not a weaker one.
>
> **2. The `occurrence: None` count is 11, not 14 and not 13.** The frame's 14
> and the report's 13 are both `grep` hit counts, and **two of the 13 hits are
> prose, not literals** (`:7848`, `:7948`). Literal count at `e75f1fe27`:
> **11** — `:302`, `:309`, `:468`, `:521`, `:3455`, `:3515`, `:3519`, `:7730`,
> `:7836`, `:7882`, `:7904`. A grep on text cannot tell a literal from a mention
> of one; if you need this number, re-derive it excluding comment lines and say
> how you excluded them.
>
> ## WHAT IS UNCHANGED
>
> **`AC-4` is untouched and still governs.** It is directory-scoped
> (`lowering/` other than `core/tests/`, at `:237-241`), so it never de-aimed
> and it fences the real guard at `aggregates.rs:1198-1204` today. **No
> acceptance criterion is relaxed by this amendment** — only coordinates are
> corrected and `§4` is re-anchored.
>
> ## CORRECTION TO THIS AMENDMENT — §5's THIRD CONDITION WAS NOT UNAFFECTED
>
> **I first wrote here that §5's third hard-stop condition was "likewise
> directory-scoped and stands as written." That was wrong,** and the Architect
> caught it (`evt_6m8xfrbddq0kn`). Only its EXCLUSION (`core/tests/`) is a
> directory. **Its SUBJECT was `lowering/mod.rs` — one file.** After the split
> the repair lands in `aggregates.rs`, so the repair *can* be written without
> touching `mod.rs`, so the condition **could not fire — not for this repair,
> not for any repair, ever again.** It is re-anchored in §5.
>
> **A DE-AIMED HARD STOP IS WORSE THAN A DE-AIMED BAN.** A de-aimed ban points
> at a decoy — a reader protects the wrong lines, but there is something there
> to look at. A de-aimed hard stop goes **vacuous**: the guard is not
> misdirected, it is dead, **and a dead guard reports clear on every input**
> while sitting in §5 reading as live protection.
>
> **And note how it survived the very pass that was auditing for this.** I
> checked `AC-4` and §5 by reading them for directory-scoping. `AC-4` had it;
> §5's third did not, **and it read as though it did because its exclusion
> clause is a directory.** ⇒ **A PARTIALLY directory-scoped condition passes a
> scan for directory-scoping** — the half that has the property vouches for the
> half that does not. That is the partial-drift warning above, one level up,
> and it defeated the audit rather than being caught by it. When you check a
> guard for a property, check its SUBJECT, not whichever clause is nearest.
>
> **The node is still genuinely undone.** This amendment fixes the frame's aim,
> not its verdict.

### 1a. The refusal site

`crates/ken-runtime/src/cranelift_backend/lowering/mod.rs:4994-5001`, in
`reconcile_source_aggregate`:

```rust
let Some(occurrence) = value.source_aggregate_producer() else {
    return Err(unsupported(
        lowered_value_kind(value),
        "a source aggregate reached the carrier with no planner-issued producer \
         occurrence, so it would name no ownership record and could only be given \
         the authority of wherever it happened to be transferred",
    ));
};
```

### 1b. Only two variants can carry an occurrence

`source_aggregate_producer` (`mod.rs:9571-9580`) returns the `occurrence` field
for `Lowered::Constructor` and `Lowered::Record`, and `None` for everything
else. `reconcile_source_aggregate` is called only from the `Constructor` and
`Record` arms of the preflight walker, so that `_ => None` arm is not reachable
from the refusal path. **The observed `construct: "Constructor"` therefore names
a `Lowered::Constructor` whose `occurrence` field is literally `None`** — not a
lookup that failed, and not a different variant misreported.

### 1c. The walker is whole-graph and runs before any allocation

`source_aggregate_preflight` (`mod.rs:4895-4980`) is called from
`transfer_into_carrier` (`mod.rs:4866`) *before* `emit_carrier_transfer`. It
recurses through `HostResult` (`:4933`) and `DynamicConstructor` (`:4937`),
which carry no ownership record of their own but are not leaves. It has **no
`_` arm** by construction (`:4959-4978`), so a new `Lowered` variant with a
child position is a compile error rather than a silently unreconciled subtree.

Consequence for sizing: a refusal anywhere in the tree is reported before
anything is allocated, so the first refusal is the only one you observe. **The
panic tells you nothing about how many more sit behind it.**

### 1d. The row builds THREE edges, and the panic reports only the first

`c2_ac4` compiles three separate functions, in this order:

| # | edge | built at | how its aggregate gets an occurrence |
|---|---|---|---|
| 1 | `c2_host_result_producer` | `:2654` | `error` comes from `synthesized_constructor` (`:2682`) |
| 2 | `c2_ordinary_result_producer` | `:2721` | hand-written `Lowered::Constructor`, `occurrence: None` at `:2730` and `:2734` |
| 3 | `c2_host_result_consumer` | `:2752` | consumes; does not transfer a source aggregate in |

Edge 1 is compiled first, so its `.expect` is the one that fires.

> **CORRECTION — `§0`, runtime-implementer, measured at `e75f1fe27` during
> `D3`. The table above assigns ONE defect per edge, and edge 1 carries TWO,
> of different kinds.**
>
> Edge 1's `ok` value is a `Lowered::DynamicConstructor` whose single
> `DynamicConstructorAlternativeV1` is written `occurrence: None` **by hand** —
> the same kind of defect `§1e` attributes to edge 2, sitting in edge 1. Edge
> 1's `error` value is the `synthesized_constructor` one the table names. The
> table lists only the second.
>
>     ANCHOR
>       c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload
>       :: its `ok` alternative carries a hand-written absent occurrence
>
> **The second defect is invisible from the panic, and that is structural
> rather than bad luck.** The walker reaches the `error` template first, so
> the `Lowered::Constructor` refusal is the only one reported. Measured: with
> the alternative's occurrence threaded, and then with both of edge 2's
> threaded as well, the panic was byte-identical to the baseline. It moved
> only when the ambient body authority was bound.
>
> ⇒ **A panic reporting the first refusal on an edge bounds how many defects
> the READER can see, never how many the edge carries.** `§1c` already states
> this across the tree; the table is where it stops being applied, because one
> row per edge reads as one defect per edge.

### 1e. Edge 2's `occurrence: None` is written into the fixture by hand

`:2727-2740` constructs `Lowered::Constructor { .. occurrence: None .. }`
nested inside another with `occurrence: None`, then transfers it at
`ordinary_producer_origin` (`:2742-2747`). **The plan already contains a real
occurrence for that node** — the fixture computes `ordinary_producer_origin` at
`:2626` and asserts against its identity at `:2707-2719`. The occurrence exists
in the plan and simply is not threaded into the value handed to the carrier.

### 1f. Edge 1's refusal is a DELIBERATE production branch

Reached because the rig has no emission owner. **This is the derivation to
confirm or kill in `D0`. I did not run it.**

`synthesized_constructor` (`mod.rs:11046`) early-returns
`Lowered::Constructor { occurrence: None, .. }` at `:11064-11071` when
`self.defining_emission_owner` is `None`. Its own comment states the intent:
absent means no context is being defined, which is not an emission this
population covers, so no occurrence is issued and *the loud refusal at the
allocation stands rather than a borrowed owner being invented*. On the other
branch (`:11089`) the occurrence is resolved from the planner.

The c2 rig reaches the first branch:
`c2_compile_edge_with_arg:2497` builds its compiler with
`bare_carrier_test_lowering` (`:1884`), whose struct literal sets
`defining_emission_owner: None` (`:1926`).

Chain, end to end:

```
c2_compile_edge_with_arg:2497 -> bare_carrier_test_lowering:1884
  -> defining_emission_owner: None                     (:1926)
edge 1 error = synthesized_constructor(...)            (:2682)
  -> mod.rs:11064 early return, occurrence: None
transfer_into_carrier                                  (mod.rs:4866)
  -> source_aggregate_preflight  HostResult arm        (mod.rs:4933)
  -> recurses into `error`, Constructor arm            (mod.rs:4898)
  -> reconcile_source_aggregate                        (mod.rs:4987)
  -> source_aggregate_producer() == None
  -> refusal, construct: "Constructor"                 (mod.rs:4994)
```

That reproduces the observed signature exactly, including the `construct`
field.

**The fixture already knows.** `:2677-2681` carries a `D7` note saying this
fixture has no `Effect` occurrence, so `match_origin` is not a producer seat,
the template gets no occurrence, and it refuses at the allocation — *"which is
where it already fails."* A prior deliverable observed this refusal and left it
in place deliberately.

### 1g. Both repair routes have working precedent in the same file

- **Thread a planned occurrence into a hand-built value:** ten sibling tests do
  it, e.g. `:2059`, `:2085`, `:3041`, `:4304`, `:8079`, each
  `occurrence: Some(...)`.
- **Give the rig a defining emission owner:** `:6151` does
  `compiler.defining_emission_owner = Some(owner)`, with `:6119` setting it back
  to `None`.

Neither route requires a new API, a new primitive, or a change under
`lowering/mod.rs`. **This is the constructibility audit's answer and it is
positive** — the repair is expressible in the vocabulary the fixture already
has.

### 1h. Provenance

Base-fail and candidate-fail with an identical signature, measured two-ended by
the `RT-SRCBODY-BIND-ORDER` all-eight-package census (`evt_ksrhrv82t5ae`), with
`px8-ds-test-support` both on and off. Pre-existing base debt, not a `D1`
regression. The reported line moved `:2509` to `:2511` only because the
candidate added two lines above it.

## 2. Deliverables

**`D0` — confirm or kill §1f, before building anything.**
Run the row un-ignored and capture which of the three edges refuses and at which
`Lowered` node. Report the first refusal's `construct` and the edge name.
If the first refusal is not edge 1's `error`, §1f is wrong: **stop, say so, and
re-derive** — do not adapt the repair to the observed panic without saying which
fixed input failed.

**`D1` — enumerate the population, do not stop at the first refusal.**
§1c means the panic hides its successors. Enumerate every source aggregate this
row transfers into the carrier and state, per node, whether it carries a
planner-issued occurrence. At minimum the three sites at `:2670`, `:2730`,
`:2734` and the `synthesized_constructor` result at `:2682`.
Then widen once: of the 14 `occurrence: None` literals in `constructors.rs`,
name which reach a `transfer_into_carrier` call and which do not. A count alone
does not discharge this; the deliverable is the reaching set.

**`D2` — rule rig versus real, in writing, with the evidence.**
Two outcomes, and they are not one deliverable:
- **Rig.** The fixture manufactures a state production never produces. The
  repair is in the fixture and `D3` proceeds.
- **Real.** A production path can reach the carrier with no producer
  occurrence. Then the refusal is protecting a live hole, the row is evidence
  rather than debt, and this becomes an Architect question — **route it, do not
  repair it.**

§1f is evidence for *rig*, and edge 2 (§1e) is plainly rig. Say which outcome
you are ruling and why. **A ruling of *rig* for edge 1 must engage the `D7` note
at `:2677-2681`**, which asserts the current refusal is correct.

**`D3` — repair the fixture so the carrier edge emits.**
Use one of the §1g routes. Every source aggregate the row transfers must carry
an occurrence the planner issued for that node — not one borrowed from a
sibling, and not a value minted to satisfy the check.

**`D4` — un-ignore the row and prove it measures its property.**
Remove the `#[ignore]` and the annotation block immediately above it.
**Re-measured on `origin/main` at `5899268451`** — the whole group sits about
785 lines below where this deliverable used to place it:

    :3315-3331   the `//` annotation block          (was written here as :2530-2546)
    :3332        #[test]
    :3333        #[ignore = "RT-CARRIER-PRODUCER-OCCURRENCE: ..."]   (was :2548)
    :3334        fn c2_ac4_runtime_host_result_selects_a_separately_generated_nested_payload

There is no separate leading doc comment above the block; `:3314` is blank and
`:3313` closes the preceding test. **Line numbers are hints** per §0 — anchor on
the attribute string and the function name, and if the group is not where this
says, that is a finding to report, not a discrepancy to reconcile silently.

> **`:3328-3330` of that block is the sentence `D5` should have caught.** It
> reads *"Un-ignoring the row is therefore NOT the repair and would only restore
> a refusal."* **That is true at this base and false the moment `D3` lands**, and
> it is a second instance of exactly what `D5` is for. `D4` deletes the block, so
> it is discharged either way — but if `D4` is ever split from `D3`, this
> sentence has to go with `D3`, not stay behind asserting a refusal that no
> longer happens.

**This deliverable is why this node is the lane's next kick.** It is the only
released runtime node whose completion removes an `#[ignore]` from the selected
population. `D4` is not optional trailing cleanup and does not defer to a
successor: **a `D3` that lands without `D4` leaves the count unmoved**, which is
the whole failure this lane has been reproducing.

**`D5` — currency.**
If `D2` rules *rig*, the `D7` note at `:2677-2681` says the refusal is correct
and expected, and that sentence becomes false the moment `D3` lands. Correct it
in place. If `D2` rules *real*, leave it and say so.

## 3. Acceptance criteria

### `AC-1` — the first refusal is identified by execution, not by reading

> **MEASURED:** `D0` reports the edge name and `construct` of the first refusal,
> from a run.
> **CLAIMED:** §1f's chain is the actual path.
> **THE GAP:** §1f was derived by reading. It reproduces the signature including
> the `construct` field, which is strong, but a second node that also refuses
> with `construct: "Constructor"` would be indistinguishable from it on the
> signature alone.

### `AC-2` — the reaching set is enumerated, not sampled

> **MEASURED:** `D1` names every source aggregate this row transfers and its
> occurrence status, plus the reaching subset of the file's 14 `occurrence:
> None` literals.
> **CLAIMED:** the repair is sized against the whole population.
> **THE GAP:** §1c guarantees the panic reports one refusal and hides the rest.
> **A repair sized on the observed panic is sized on a sample of one.**
> **Positive control:** after repairing only edge 1, the row must still fail —
> at edge 2. If it goes green, §1e is wrong and `D1` mis-enumerated. Record that
> intermediate result; do not skip straight to the full repair.

### `AC-3` — the restored row discriminates its property

> **MEASURED:** with the row un-ignored and passing, a **population-side**
> mutation reddens it — swap the two arm identities, or point the consumer at
> the other constructor, so the selection genuinely changes.
> **CLAIMED:** the nested payload is separately generated and correctly
> selected.
> **THE GAP:** a green row proves the carrier now emits. It does not prove
> selection happened, and emitting is exactly what `D3` changed. The row's
> existing `assert_ne!` at `:2646-2649` guards identity distinctness, not
> selection.
> Restore the mutation afterward and say so.

### `AC-4` — no production behaviour changed under a *rig* ruling

> **MEASURED:** if `D2` ruled *rig*, `git diff origin/main` touches no file
> under `crates/ken-runtime/src/cranelift_backend/lowering/` other than the
> `core/tests/` subtree.
> **CLAIMED:** the guard is intact.
> **THE GAP:** the cheapest false fix here is a one-line relaxation at
> `mod.rs:4994` that no test would catch, because every test that would catch it
> is the one being repaired. **This AC is the only thing standing on that line.**

### `AC-5` — no regression

Green in CI. Per `COORDINATION §12` this means CI, **not** a local
`--workspace` run. Locally, run only `-p ken-runtime`.

## 4. Banned scope

> **RE-ANCHORED 2026-09-18. Each ban below is anchored on a SYMBOL and the
> PROPERTY it protects. The `hint:` coordinate is a convenience that may already
> be stale — resolve the symbol, and if the symbol is gone, STOP and escalate
> rather than finding the nearest thing that looks like it.** See the §1
> amendment for why a line-anchored ban re-aims onto a same-shaped neighbour and
> then verifies as satisfied.

- **BAN 1 — `reconcile_source_aggregate` :: its producer-occurrence refusal must
  not be relaxed or deleted, and no fallback owner may be added.**
  *hint: `aggregates.rs:1198-1204`, fn at `:1191`.*
  The refusal text names precisely what a fallback would grant: the authority of
  wherever the value happened to be transferred. The doc at `aggregates.rs:818`
  (*hint*) states that a missing producer is a REFUSAL, never a fallback.
  If the proposed repair is to make the emit succeed by accepting an aggregate
  with no ownership record, **that is a mechanism question and it returns to the
  Architect** — it does not land here.
  **This is the ban `AC-4` calls the only thing standing on that line**, and it
  is the one that had de-aimed onto eight lines of `StaticWorkerBinding` code.
- **BAN 2 — `synthesized_constructor` :: its no-emission-owner early-return
  branch must not change without an Architect ruling.**
  *hint: `aggregates.rs:3477-3488`, fn at `:3459`.* Identify it by
  `let Some(owner) = self.defining_emission_owner else { .. occurrence: None .. }`,
  not by line. §1f says that branch is deliberate and the fixture's own `D7`
  note says the resulting refusal is correct. Changing it is a design change
  wearing a fixture repair's clothes.
- **BAN 4 (added 2026-09-18) — THE CLASS, not a list: no refusal whose reason
  is a MISSING PLANNER-ISSUED OCCURRENCE may be relaxed anywhere under
  `lowering/` outside `core/tests/`, whether or not this frame names its
  symbol.** Added after runtime-implementer's `D1` found a third such refusal
  in a symbol no ban reached (`evt_6ym3avaqwsb7r`).
  Known members at `e75f1fe27`, **as hints and explicitly NOT as the
  population**:

      aggregates.rs:1201   reconcile_source_aggregate      (BAN 1's site)
      aggregates.rs:1397   reconcile_source_aggregate      SECOND refusal in
                                                           the same fn -- BAN 1
                                                           reaches it ONLY
                                                           because BAN 1 is
                                                           symbol-anchored; its
                                                           line-anchored
                                                           predecessor did not
      aggregates.rs:3169   emit_carrier_dynamic_constructor (fn at :3113)
                           "the selected alternative ... carries no planned
                            occurrence, so its allocation has no lifetime meet"

  **This ban is deliberately written as a PROPERTY and must not be "completed"
  by enumerating its members.** An enumeration widens past the property in the
  admitting direction — the exact failure the language ring hit the same day —
  and a fifth site would then sit outside a list that looks exhaustive. **If you
  find a member not listed above, that is a finding to report, not a gap in the
  ban.**
  `AC-4` is what actually fences this today, and that is the second time in this
  frame that the directory-scoped criterion caught what a named one missed.
- **BAN 3 — the `.expect("the C2 carrier edge emits")` assertion :: must not be
  weakened to make the row pass.** Identify it by that exact string.
  *hint: `core/tests/constructors.rs:2564`.* It lives in the shared C2 rig near
  `ac_c7_run` (`:2483`), **not** inside `c2_ac4`'s body — so weakening it
  reaches rows beyond this node's.
- **Do not re-baseline or re-scope the row's assertions** to fit whatever the
  repaired edge produces. If the row cannot assert its stated property after the
  repair, that is a finding to report, not an assertion to adjust.
  **Replacing an assertion is TWO acts needing TWO authorizations, on different
  evidence — see the carve-out below. Removing the old value and installing a
  new one are never authorized together.**

  > **CARVE-OUT, 2026-09-18 (Steward). This ban as first written stopped `D4`
  > one line from green, and the ring was right to stop.**
  >
  > **CORRECTION, same day, Architect at `evt_7sz14dsfcz0a3`. The first version
  > of this carve-out — landed in `daa4c010c` and replaced here — asked WHERE
  > THE EXPECTED VALUE CAME FROM. That test fails on the very case it was
  > written for**, because replacing an assertion involves *two* values that can
  > point opposite ways: `D4` removes `ImmediateBool`, which came from the
  > defect, and installs `PersistentGround`, which was read off the repaired
  > run's own output. One edit, both cells, nothing discriminated. **Provenance
  > is also a claim about history made by the party it relieves, and it is not
  > checkable from the file.**
  >
  > **THE TEST:**
  >
  > > **Does the artifact state a derivation that would have PREDICTED this
  > > value BEFORE the run?**
  >
  >     the row can only say WHAT THE RUN EMITTED   -> re-baselining, whatever
  >                                                    the history was. BANNED.
  >     the row can say WHY THAT VALUE AND NOT       -> a repair, whatever the
  >       ANOTHER, independently of any run             history was. Permitted.
  >
  > **A reader holding the file and nothing else can apply this.** That is the
  > whole reason it replaces the provenance test.
  >
  > **THE TWO AUTHORIZATIONS, resting on different evidence:**
  >
  >     REMOVING the old expected value   authorized by the OLD value's
  >                                       provenance -- it is the defect's own
  >                                       fingerprint sitting in the oracle
  >     INSTALLING a new one              authorized ONLY by a derivation stated
  >                                       independently of any run, IN THIS FILE
  >                                       -- never in a thread, a post, or a
  >                                       commit message
  >
  > Written as one rule they collapse, and the installation borrows evidence
  > that reaches only the removal.
  >
  > **Measured instance, `D4`:** the error-path equality compared a projected
  > boundary tag against `ImmediateBool` — **the value the pre-`D3` rig itself
  > chose for `Wrote`.** The assertion compared the fixture's own input against
  > itself, so the defect's fingerprint sat inside the oracle. Leaving it there
  > is not rigour; it preserves the bug in the thing meant to catch the bug.
  >
  > **The premise does not reach the wider conclusion.** *"It was never an
  > independent oracle"* licenses **repairing** the oracle. It does not license
  > **retiring** the check — it says nothing about whether the property is worth
  > asserting. Both readings follow from the same true sentence and only the
  > weaker one is entailed; prefer it.
  >
  > **OBLIGATIONS ON ANY INSTALLATION, all three:**
  >
  > 1. **A derived property, never an observed literal.** `517` pins an arena
  >    index: sound as a measurement, unsound as a criterion.
  > 2. **The derivation is written INTO THIS ROW with its citations**, and every
  >    link is grounded in a producer rather than in a name that looks right. If
  >    a link cannot be grounded it goes to the Architect — an ungrounded
  >    replacement is wrong in exactly the way the original was.
  > 3. **The repaired assertion reddens under a mutation ON THE PATH IT
  >    GUARDS**, not merely on some other path the row also covers.
  >
  > **If no mutation on that path can redden it, retiring it is then honest —
  > but the row must SAY it does not cover that path.** An assertion that cannot
  > fail quietly retired, and one loudly retired, are the same coverage and very
  > different artifacts.
  >
  > **Why obligation 2 says IN THIS FILE.** Measured on candidate `d20afe1be` by
  > a grep of the whole file rather than the diff: the only derivation present
  > was *"517 is tag 5 = `PersistentGround`"*, with the reasoning that justifies
  > it living solely in a convo post. **A later reader opening the file finds a
  > value read off the fixed run, sitting beneath the rule forbidding exactly
  > that, and nothing to tell it apart from re-baselining.** A justification that
  > is not in the artifact does not travel with it.
- **Do not repair the other census row.** `two_same_shape_workers_are_distinguished`
  is `RT-WORKER-FIXTURE-DECODE`'s, and it is `ready`.

## 5. Hard stop

Stop and report, rather than proceeding, if any of these holds:

- `D0` contradicts §1f.
- `D2` rules *real* — a production path can reach the carrier with no producer
  occurrence. That is an Architect question and it is more important than this
  row.
- **RE-ANCHORED 2026-09-18 — this condition had gone VACUOUS.** It read *"the
  repair cannot be written without touching `lowering/mod.rs` outside
  `core/tests/`."* Only its exclusion was directory-scoped; its SUBJECT was one
  file. After `c7f071bcb` the repair lands in `aggregates.rs`, so the repair
  **can** be written without touching `mod.rs`, so the condition could no longer
  fire — for this repair or any other. It now reads:
  **the repair cannot be written without touching PRODUCTION under `lowering/`
  outside `core/tests/`.** §1g says it can; if that turns out false, the
  constructibility audit was wrong and the size is wrong with it.
- `D1`'s reaching set is materially larger than the four sites named — that is a
  re-sizing conversation with the Steward, not a longer turn.

Per the one-hour turn target, a genuine hard stop is a good outcome. Neither
finishing nor stopping is the bad one.

## 6. Contention

Touches `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/
constructors.rs`, which is also
[[RT-WORKER-FIXTURE-DECODE]]'s file. **Both nodes are runtime-owned and the
fleet is single-threaded, so they cannot run concurrently** — sequence them, do
not parallelize. Their target rows are far apart — `c2_ac4...` at `:3334`
here (hint),
`two_same_shape_workers_are_distinguished` at `:7404` there (hint,
re-measured 2026-09-18) — and their
deliverables are disjoint, so either order works.

`RT-CARRIER-BYTESPAN-OBSERVE` is `active` on the same crate. This node is
sequenced behind it.
