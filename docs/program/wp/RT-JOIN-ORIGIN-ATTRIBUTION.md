# RT-JOIN-ORIGIN-ATTRIBUTION — the causal checkpoint and its authority

> **THE ANSWER IS IN §10: the first missing general authority is the GENERAL
> TRAVERSAL ROUTE.** §9 carries the four measurements it is argued from.
>
> **§§1-8 are retained CHRONOLOGY, and §4 in particular is SUPERSEDED.** It
> records that no authority could be named *at that time*, on the venues then
> available and before Kernel supplied the exact invocation and its projection
> snapshot. It is kept because the two negative results in it are load-bearing
> — they are what established that the defect is neither a `D3` regression nor
> reachable from a substitute program — **not because its headline claim is
> still current.** Read §4 as "what was true before §8", never as this
> document's finding.

> ## THE VENUE, FIRST, BECAUSE EVERY NUMBER BELOW DEPENDS ON IT
>
> **Synthetic integration tree `11edde073245d9495dacdaa74ac6d11afe613260`**,
> materialised as an **unreferenced detached commit `0c5be874`** carrying
> exactly that tree.
>
> | parent | what it is |
> |---|---|
> | `f0217c67e99c302ffa2e8d5a9f86bca54ddf9bb5` | current Runtime `main` at kickoff |
> | `dd3cd050e44cac6b63751b12802cf77ca59a5b82` | the exact Kernel witness, `wp/KERNEL-NESTED-IND` held tip |
>
> Common base `120b426bdd18bc9f5ff81467635d6aa307640113`.
>
> **Verified before measuring, not assumed:** `git rev-parse HEAD^{tree}` equals
> `11edde07`; `D3`'s `D3Event` is present in `lowering/units.rs`; the Kernel
> witness test is present in `nc14_data_match_lowering.rs`. Both halves are
> genuinely integrated.
>
> **Kernel's branch was never moved, edited, rebased or committed to.** No
> candidate-like ref exists. No instrumentation is committed. No production
> change.
>
> ⇒ **This is not `main`.** `main` cannot host the witness at all — see §1.
> Every statement here is about the integration venue; **any claim about
> `main`'s executable behaviour would be inference, not measurement**, and none
> is made.

## 1. `main` cannot host the witness, measured

The exact Kernel witness, taken verbatim from `dd3cd050`'s
`nested_recursive_field_elaborates_checks_and_runs_from_checked_artifact`:

```
data Rose = Tip | Node (List Rose)
fn depth (r : Rose) : Nat = match r { Tip |-> Zero ; Node xs |-> match xs {
  Nil |-> Zero ; Cons h t |-> Suc (depth h) } }
```

Elaborated at `f0217c67`:

```
KernelRejected { error: PositivityViolation(
  "non-strictly-positive occurrence of D in constructor g484 arg 0"),
  span: Span { start: 0, end: 33 } }
```

Span `0..33` is the `data` line. Nested strictly-positive admission is exactly
what `KERNEL-NESTED-IND` adds and it is unmerged, so no lowering, no plan and no
join population exist on `main` for this program. **This is why the synthetic
venue exists**, and it is the whole reason the node could not be discharged on a
Runtime base.

## 2. The three authority seats, located at the venue

| authority | seat |
|---|---|
| 1 — general traversal route | `lowering/mod.rs` `enter_source_occurrence_plan` |
| 2 — general selection/disposition seat | `lowering/mod.rs` `close_statically_unselected_match_cases` |
| 3 — planner owner/population | `planning/static_transition.rs` `required_join_origins` |

The **sole origin-to-expression table** is
`StaticTransitionPlan::source_occurrences`, a `Vec<Option<PlannedOccurrence>>`
indexed by origin id, where `PlannedOccurrence` is exactly
`{ static_origin, expr }`. Ownership comes from
`semantic.function_owner(origin)`. `required_join_origins` walks that vector
zipped with `join_results` and **refuses outright if the population is not keyed
by source origin**, so there is one authority to read and it self-checks its own
keying.

`None` in that vector is a real answer — a control node is a planned node with
no source term — so a lookup returning `None` is a planner failure, not a gap.

## 3. What was measured, and the result is negative

Instrument (temporary, env-gated on `KEN_RTJOA`, **reverted, never committed**):
every `enter_source_occurrence_plan` call recorded before the planned-join
filter, so *"was the route taken for this origin"* is separable from *"is this
origin a planned join"*; every match selection with its `final_reachable` case
indices and case-body roots; and, on the `required \ covered` branch, a full
dump of the four measurements.

### 3a. Kernel's own committed witness test PASSES

`nc14_data_match_lowering` — **9 passed, 0 failed**, including
`nested_recursive_field_elaborates_checks_and_runs_from_checked_artifact`. The
runtime-IR-evaluator lane is green at the venue.

### 3b. A native build of the same program SUCCEEDS

`data Rose`/`depth` wrapped in a `program capabilities FS APartial` boundary
with a `main`, through `ken_cli::build_native_program` — the Cranelift lane,
which is where `finalize_join_disposition` runs. It **compiles and links**.

**The positive control, because a green build proves nothing unless the seat
ran.** `finalize_join_disposition` was reached four times, each with a non-empty
required population, and every required origin was consumed:

| call | required | consumed | dispositioned | entered | required set |
|---|---|---|---|---|---|
| 1 | 1 | 1 | 0 | 5 | `{SOI(6)}` |
| 2 | 3 | 3 | 0 | 20 | `{SOI(12), SOI(18), SOI(23)}` |
| 3 | 3 | 3 | 0 | 35 | `{SOI(43), SOI(49), SOI(54)}` |
| 4 | 2 | 2 | 0 | 50 | `{SOI(58), SOI(59)}` |

**`required \ covered` is empty at every one. SOI(26) appears in no population.**

### 3c. The differential, and it rules out the obvious explanation

The identical program and instrument on **bare `dd3cd050`** — the pre-`D3`
held tip — produce **byte-identical results**: the same four closures, the same
origin sets, the same successful native build.

⇒ **`D3` is not the difference.** The attractive hypothesis — that something
landed on `main` since `120b426b` already repaired this — is **false**, and it
is ruled out by measurement rather than left as a possibility.

## 4. SUPERSEDED BY §10 — no authority could be named AT THIS POINT

> **This section is the first measurement, and its conclusion is spent.** It
> was written before Kernel supplied the exact invocation (§8) and the
> projection snapshot that made the defect reachable (§9). **§10 names the
> authority.** What survives here is the reasoning about why a wrong program's
> trace must not be used to name one — which is why the section is retained
> rather than deleted.

**I cannot attribute an authority for a failure I cannot reproduce, and I will
not name one from the node text.**

The origin ids my program produces are `6`, `12/18/23`, `43/49/54`, `58/59`.
**None is 26**, and the populations close cleanly. That is not evidence the
defect is absent — **it is evidence that my program is not the invocation that
produced it.** A different origin numbering is a different plan.

**Naming an authority here would be the exact failure the frame warns about**,
one level up from the one it names. The frame says a classification argued from
the set difference rather than from the trace is the symptom rather than the
cause; an authority argued from a trace of *a different program* is worse,
because it would carry a real measurement's credibility.

**And a green result is the most dangerous shape to over-read.** `required \
covered` empty passes for any reason, including "this configuration was never
built". The four-row positive control above establishes the seat ran with real
populations — that is the strongest statement available, and it still does not
reach SOI(26).

### What is missing, precisely

**The exact invocation that produced `SOI(26)`.** `SOI(26)` appears nowhere in
the repository except this node's own frame, and Kernel's committed witness test
drives the **runtime-IR evaluator** (`evaluate_runtime_ir_program_expr`), which
never enters the Cranelift lane where `finalize_join_disposition` lives. So the
error came from a native attempt that is not in their tree.

Needed, and any one of these is sufficient:

1. the exact source text and entry shape Kernel compiled (mine wraps `depth` in
   a `main` and a `program capabilities` boundary, which they may not have), or
2. the command / test name that produced the refusal, or
3. the verbatim error string with its surrounding populations.

**I am deliberately not searching for a program shape that fails.** Trying
variants until one reproduces an error is how a convenient shape gets
substituted for the real witness, and this campaign has paid for that three
times — most recently five mutations compiling `Ok` on a witness that could not
discriminate.

## 5. Boundaries held

- **No production change.** The instrument is reverted; `crates/` at
  `wp/RT-JOIN-ORIGIN-ATTRIBUTION` is byte-identical to `f0217c67`.
- **None of the four forbidden manipulations.** `SOI(26)` was not consumed, not
  inserted into the dead set, not deleted from `required`, and not
  special-cased. There is no production edit of any kind.
- **No repair attempted**, and no mechanism proposed.
- **Kernel's branch untouched**; the measurement venue is an unreferenced
  detached commit.

## 6. Deferred ownership, with the condition that releases it

| owed by | what | released when |
|---|---|---|
| the Runtime **correction candidate** | run the exact witness on this same synthetic integration venue before merge | immediately — the venue is reproducible from the two parent SHAs |
| the **first post-Kernel integration/closure candidate** | the committed **runnable** exact-witness control | nested-inductive admission has merged, so the witness can exist on `main` |

Neither is a bare deferral: both have an owner and a stated release condition.

## 7. Reproducing the venue

```sh
git commit-tree 11edde073245d9495dacdaa74ac6d11afe613260 \
  -p f0217c67e99c302ffa2e8d5a9f86bca54ddf9bb5 \
  -p dd3cd050e44cac6b63751b12802cf77ca59a5b82 \
  -m "RTJOA synthetic integration venue"
git worktree add --detach <path> <resulting-commit>
```

Confirm `git rev-parse HEAD^{tree}` is `11edde07` before trusting any result.

## 8. Kernel's exact invocation stops two stages before the join authority

The Kernel ring supplied the invocation §4 asked for: package
`nested_inductive_pkg`, source `src/main.ken`, selected executable `liftSize`,
first selected target closure, `RuntimeExample` `nested-size-uses-lift`, runtime
IR argument `LiftNode(Join(LiftLeaf, LiftNode(Empty)))`, through
`run_example_with_interpreter_observation` with an empty `NativeSeedEnvironment`
and a `Nat 3` interpreter oracle.

It differs from the Rose program materially, and the difference is real rather
than cosmetic: `Bag` carries `Empty`/`One`/`Join`, and the `Join` arm makes
**two** nested recursive calls combined by `liftAdd`.

**Reproduced exactly, on the synthetic venue. The verdict is:**

```
Unsupported {
  stage: BoundaryPreflight,
  construct: "RuntimeProgram",
  reason: "package carries trust metadata outside the supported native subset"
}
```

### The join authority is never reached, and that is measured, not inferred

**Zero `RTJOA-SEAT-OK` lines were emitted.** That instrument prints on every
successful `finalize_join_disposition`, and it printed four times for the Rose
program in §3b. Here it prints nothing at all: **the compile is refused at the
native boundary preflight, strictly before lowering, so `required`, `consumed`
and `dispositioned` are never computed for this program.**

⇒ **`SOI(26)` cannot be attributed on either venue, because the run that would
produce it does not happen.** There is no set difference to classify.

### The differential again, and again it rules out the interesting explanation

The identical harness on **bare `dd3cd050`** returns the **same
`BoundaryPreflight` refusal**. So the gate is **not** something Runtime's `main`
introduced since `120b426b`, and *"current integration tightened a preflight and
masked the join defect"* is **false by measurement**.

### What this places SOI(26) behind

The refusal is `artifact/api.rs`'s trust-metadata preflight. It is the same wall
the Architect was separately ruling on when authorizing an **erasure-time
reachability projection** — `erase_checked_core_package_for_target` emitting
beyond the checked declaration closure it is handed, so the program carries
trust metadata the native subset does not admit.

**No branch in the repository carries that projection** — searched, none exists.

⇒ **`SOI(26)` is downstream of an unlanded erasure correction**, and Kernel must
have observed it on a tree carrying that correction locally. **A third venue is
needed**: the synthetic integration tree **plus** the erasure-time reachability
projection.

### Why I stop here rather than reconstruct the projection

Writing the erasure correction myself to reach the join seat would be a
**production repair**, which `AC-3` forbids and the frame names twice. It would
also be *my* reconstruction of a mechanism the Architect has ruled on but not
yet seen landed — so any SOI(26) trace taken on it would be a trace of my guess
at their correction, which is the substitute-witness failure with an extra layer.

**The ask is now exact:** the tree, patch or branch on which Kernel observed
`BackendFailure` at `StaticOriginId(26)`. Given it, the four measurements in §2
run unchanged and the classification follows.

## 9. Venue 3 — SOI(26) reproduced, and the four measurements

Kernel supplied the complete uncommitted projection snapshot as commit
`a577f136454ed84113ea853ff91c59929fdd53bf`, tree
`648548abb650e55a5bf56d2f39ed3f4efb5aae71`, parent `dd3cd050`.

**Venue 3** is the clean object-level merge of the §venue synthetic commit
`0c5be874` with that snapshot: tree
`ccbb5ee519776c0ada29a09eb488c61299c80d68`, materialised as **unreferenced**
commit `26bf2961`. No conflict. Kernel's branch was not moved, edited, named or
committed to.

**The exact invocation from §8, unchanged, now returns the node's error
verbatim:**

```
BackendFailure { stage: NativeLoweringOrExecution, reason:
  "module operation failed: function left planned source join
   StaticOriginId(26) neither emitted nor statically unselected" }
```

### Measurement 1 — the sole origin-to-expression table

| field | value |
|---|---|
| origin | `StaticOriginId(26)` |
| `occurrence.keyed` | `Some(true)` — the table's own keying self-check passes |
| `RuntimeExpr` kind | `ComputationalMatch { scrutinee: Var(0), … }` over `LiftRose`, cases `LiftLeaf` / `LiftNode` |
| semantic function owner | `Some(PredeclaredFunctionId(2))` |

The scrutinee is `Var(0)` — the function's own parameter — so **SOI(26) is the
body of `liftSize` itself**, not a nested or conditional sub-expression.

### Measurement 2 — token, predecessor bit, and membership

| field | value |
|---|---|
| `join_result` | `PlannedJoinResult { representation: CarrierWord, has_continuing_predecessor: true }` |
| `join_token` | `JoinPlanToken { origin: SOI(26), representation: CarrierWord, has_continuing_predecessor: true }` |
| `required` (owner `PredeclaredFunctionId(2)`) | `{SOI(26), SOI(33), SOI(39), SOI(53)}` |
| `consumed` | `{}` |
| `dispositioned` | `{}` |
| `covered` | `{}` |

**The entire owner population is uncovered, not merely SOI(26).** SOI(26) is
simply the least element of a four-member set of which **none** was reached.

#### The closing function

**The closing function is `PredeclaredFunctionId(2)` — the same function that
owns SOI(26)**, and this is a derivation from two measured fields rather than a
third direct observation. Stated that way deliberately, because the required
measurement list names it explicitly and an asserted field would read as
observed:

1. `semantic.function_owner(SOI(26))` was **measured** as
   `Some(PredeclaredFunctionId(2))` (measurement 1);
2. `required_join_origins(function)` admits an origin **only** when
   `function_owner(origin) == function` — it iterates the occurrence table and
   inserts on exactly that equality, refusing outright if the population is not
   keyed by source origin.

⇒ Since SOI(26) is present in the `required` set the closeout raised on, the
`function` that closeout was called with is necessarily the owner of SOI(26),
namely `PredeclaredFunctionId(2)`. The call site is
`define_unit_body`'s `validate_join_plan_consumption(unit.function)`
(`lowering/units.rs:4696`), which passes the emitting unit's own id.

**There is no second closing function in play**: the two sibling closures in
the positive control below carry disjoint `required` sets, so no other function
could have raised this one.

### Measurement 3 — traversal, selection, and subtree

| observation | value |
|---|---|
| `enter_source_occurrence_plan` called for SOI(26) | **false** |
| entered for this function | `8, 7, 6, 5, 4, 3, 2, 20, 19, 17, 15, 14, 13, 12, 11, 58` — **none of 26, 33, 39, 53** |
| match selections recorded | **none** |
| owner-subtree joins rooted at SOI(26) | `{SOI(26), SOI(33), SOI(39), SOI(53)}` |

**Positive control, so "nothing was consumed" is not read as "the seat never
ran".** Two sibling functions closed correctly in the same compile:
`required=1 consumed=1 {SOI(8)}` and `required=2 consumed=2 {SOI(14), SOI(20)}`.
The traversal, the ledger and the closeout all work; **they are not reached for
this owner.**

## 10. THE FIRST MISSING GENERAL AUTHORITY IS THE GENERAL TRAVERSAL ROUTE

**Authority 1.** The lowering traversal that emits
`PredeclaredFunctionId(2)`'s body never routes through
`enter_source_occurrence_plan` for any join planned in that body.

**Argued from the trace, not from the set difference:**

- **It is not authority 3 (planning / ownership).** The occurrence is present
  and correctly keyed — `required_join_origins` refuses outright if the
  population is not keyed by source origin, and it did not. SOI(26) carries a
  well-formed `JoinPlanToken` with a coherent representation and predecessor
  bit, and `function_owner` returns a real function. Nothing about the planner's
  answer is malformed; lowering simply never asks.
- **It is not authority 2 (selection / disposition).** **No static selection was
  recorded at all** for this function, so there is no unselected branch whose
  subtree went undispositioned. And SOI(26) is the function's **own body**,
  scrutinising its **own parameter** — semantically reachable by construction. A
  body is not statically unselected, so dispositioning it would be **asserting a
  deadness that is false**, which is the forbidden "insert it into the dead set"
  wearing a subtree's clothes.
- **It is authority 1, and the whole-population shape is what says so.** All
  four planned joins of one owner are missing together, while two sibling
  functions in the same compile consume theirs correctly. That is not a
  per-origin omission; it is **one owner's body emission not passing through the
  general entry route at all.**

**The general statement, which is the deliverable:** an emission path exists
that produces a function body without routing its planned source occurrences
through `enter_source_occurrence_plan`. **The correction belongs at that general
traversal route** — at whatever body-emission path this owner takes — and not at
SOI(26), whose only distinction is being the least element of the set that path
failed to visit.

⇒ Per the frame: **reachable ⇒ the correction belongs at the general traversal
route.**

**Stopping here for the Architect's mechanism ruling.** Which body-emission path
`PredeclaredFunctionId(2)` takes, and how it should reach the general entry, is
the mechanism question and it is not mine.
