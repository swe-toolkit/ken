# RT-JOIN-ORIGIN-ATTRIBUTION — the causal checkpoint, and the input it is missing

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

## 4. THE FIRST MISSING GENERAL AUTHORITY IS NOT NAMED, AND THIS IS THE FINDING

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
