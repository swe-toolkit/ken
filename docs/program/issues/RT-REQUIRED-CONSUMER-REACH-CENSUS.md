---
id: RT-REQUIRED-CONSUMER-REACH-CENSUS
title: "The projection mints an entry only where required differs from source, so row 4 depth 1 is excluded from the new surface BY CONSTRUCTION -- census which rows the surface reaches, and attribute by SENTENCE the Closure refusal depths 2 and 3 now sit at"
status: merged
owner: runtime
size: S
gate: none
depends_on: [RT-REQUIRED-OCCURRENCE-PROJECTION]
blocks: [RT-CROSSING-CALL-SITE-ATTRIBUTION]
github: https://github.com/swe-toolkit/ken/pull/2305
origin: Steward, 2026-08-15, on the RT-REQUIRED-OCCURRENCE-PROJECTION merge (66715f9fb, PR #2293). Its D4 recorded that row 4 depths 2 and 3 advance to Closure and that rows 1, 4-depth-1 and 5 are unchanged -- the boundary NAME but not the refusal SENTENCE, which is exactly the shape RT-CONSUMING-OCCURRENCE-ROUTE-WIRE's D4 discipline requires of a residual. Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

> # WHAT THE PREDECESSOR LEFT, AND WHY IT IS NOT "THE ROUTE IS UNBLOCKED"
>
> [[RT-REQUIRED-OCCURRENCE-PROJECTION]] merged at `66715f9fb` and its `D4`
> advanced **row 4 depths 2 and 3** to a `Closure` refusal, pinned by a landed
> control. The tracker no longer lists anything on this chain as blocked.
>
> **"Unblocked" is not the same as "the surface reaches the remaining rows",
> and at least one row it does not.** The projection is minted under an
> equality guard (`static_transition.rs:11683`):
>
> ```rust
> if required != source {
>     let projection = RequiredConsumerProjection { source, required };
>     ...insert...
> }
> ```
>
> ⇒ **Where the required occurrence coincides with the source-level one, no
> projection is minted at all** — there is nothing for the lowering funnel to
> read, and `required_consumer_projection_for` returns `None`. That is a
> deliberate property of the design, not a defect: the projection exists to
> carry a value the key cannot already express.
>
> **[[RT-CONSUMING-OCCURRENCE-ROUTE-WIRE]] measured that at row 4 depth 1 the
> two values coincide** — *"Same-level required and source values coincide"*.
> Combined with `D4`'s measured *"row 4 depth 1 unchanged"*, three consistent
> facts predict that **depth 1 is outside the new surface by construction.**
> `D2` below is the measurement that turns that prediction into a fact, because
> a prediction from a code reading is not a fixed input.
>
> ### THE PRACTICAL CONSEQUENCE, and it is why this node exists
>
> **A repair cut on the assumption that "the projection now serves rows 4 and 5"
> would be cut wrong**, in the same way `D2k-1c` was: the surface it names does
> not reach part of its population. Row 4 depth 1's advance was measured by the
> route-wire probe **through the existing key field**, and that probe was fully
> reverted. So depth 1's route forward is a **different edge** from depths 2/3's,
> and nothing in the tree says so.

> # `D1` IS DISCHARGED, MEASURED 2026-08-15 AT `a737d8c9b`. THE STOP FIRED.
>
> Report `evt_6qc0vkzj43c0e`. Measured through the existing five-row
> `d2k_0_the_five_no_longer_reach_a_static_worker_value_read` control; temporary
> diagnostic fully reverted, worktree clean at the measured base, **no source
> diff, commit, or candidate**, and no ownership proposal made.
>
> | row | construct | site | sentence |
> |---|---|---|---|
> | row 4 depth 2 | `Closure` | `lowering/mod.rs:11550-11552` | *a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane* |
> | row 4 depth 3 | `Closure` | same | identical |
>
> **It is the first of the four candidate refusals — the durable-lane sentence
> [[RT-CLOSURE-BOUNDARY-LANE]] owns for a different population.** The stop
> condition below therefore fired as written.
>
> ### THE STOP WAS SCOPED TOO WIDE, AND THAT WAS THE FRAME'S DEFECT
>
> **`D2` and `D4` have no dependency on which `Closure` sentence fires.** `D2` is
> decided by `if required != source` in the planner, upstream of the refusal
> entirely; `D4` is control hygiene in another file. Reading the stop as covering
> them was the correct reading of what was written. **`D2`, `D3` and `D4` were
> released at `evt_1eq64yvqc2b96`** with one narrowing: `D3` records the residual
> as a measured fact and **stops before the ownership and sequencing half.**
>
> ### THE CONVERGENCE IS NOT EVIDENCE OF A SHARED ROOT
>
> The site is the `Lowered::Closure | Lowered::DeclarationClosure` arm of
> **`boundary_transfer_admissibility`** (`RT-FNSPLIT-C1` `D5`) — a **total,
> wildcard-free walk over the whole value graph**, run before any allocation,
> which exists precisely because *"the root variant table is not sufficient"*.
>
> ⇒ **Every closure-carrying graph that attempts the crossing refuses here, by
> construction.** A shared site is evidence that **the gate is total**, not that
> two populations share a production root. Folding on a matching sentence is the
> move this campaign's own banner forbids: *shared syntax is not a proven shared
> root, and a subsumption is routed before coding rather than inferred.*
>
> ### ROUTED TO THE ARCHITECT, `evt_7rpkfc7awktmb` — the UPSTREAM fork
>
> **For rows 4 depths 2 and 3, is a closure in the crossing graph correct or
> incorrect?**
>
> - **Correct** ⇒ the lane genuinely needs a closure to cross; the repair is a
>   **durable lane** — a representation feature, and the two populations are one
>   defect.
> - **Incorrect** ⇒ `realize_required_consumer_locally` produces a closure-shaped
>   value where it should not; the repair is a **lowering fix inside this chain**
>   that never reaches the admissibility gate, and the convergence is a
>   coincidence of totality.
>
> **These are not variants of one repair** — one grows a representation surface,
> the other removes a value that should not exist. **Ownership follows the
> ruling; it is not the question being asked.**

## What has landed, so you do not re-derive it

| node | what it established | what it did NOT do |
|---|---|---|
| [[RT-CONTKEY-CONSUMING-OCCURRENCE]] (`a998d3f6`) | the source-keyed relation, complete at depth 1 | nothing below depth 1 |
| [[RT-CONTKEY-CONSUMER-DESCENT-CARRY]] (`b0f9c2ff2`) | `required(N)` = the consumer established at `N-1` | no row closed |
| [[RT-CONSUMING-OCCURRENCE-ROUTE-WIRE]] (`46a8ba199`, no candidate) | the depth-2+ boundary is a **representation** problem; depth 1 advances with a same-level consumer | the probe was **fully reverted** — none of it is in the tree |
| [[RT-REQUIRED-OCCURRENCE-PROJECTION]] (`66715f9fb`) | the lawful depth-2+ surface, validated by re-derivation; depths 2/3 advance to `Closure` | did not name the refusal **sentence**, and did not census which rows the surface reaches |

## What this node owns

**Two measurements and a residual record. It changes no production behaviour.**

The landed control `required_consumer_projection_reaches_the_depth_two_funnel`
(`control.rs:5976`) asserts the refusing **construct** — `"Closure"` — and the
binder-install count, and suppressing the route returns depth 2 to
`StaticWorkerBinding` with zero installs. **That is a strong control and it is
not an attribution.** `"Closure"` is the construct label on **four distinct
refusals** with four different sentences:

| site | sentence |
|---|---|
| `lowering/mod.rs:11550` | *a closure cannot cross the boundary: it is runtime-local and live-domain only, and it has no durable lane* |
| `lowering/mod.rs:20247` | *capture `{symbol}` has no runtime value in the seed environment* |
| `lowering/mod.rs:20312` | *seed capture `{symbol}` has no artifact-static material minted for it* |
| `lowering/mod.rs:21272` | *closures are callable but not observable ground values in native lowering* |

**Which one fires decides what the next cut is**, and they are not close
together: one is a durable-lane representation gap, two are seed-material gaps,
one is an observability rule. **`mod.rs:11550`'s sentence is also the exact
signature [[RT-CLOSURE-BOUNDARY-LANE]] owns** — if that is the one, this chain
has landed its rows onto a boundary another node already owns, and the
sequencing question is real rather than hypothetical. **Do not assume it is
that one.**

## Deliverables

**`D1` — attribute the `Closure` refusal by SENTENCE and by SITE.** For row 4
depth 2 and again for depth 3, record which of the four refusals fires, its
`file:line`, and its message verbatim. **Assert the sentence in this node's
control** — the boundary is this chain's and its controls own it, which is the
positive form of the Architect's `evt_prwxvqcq17cj` §5 ban on *other* nodes
pinning it.

**`D2` — census the surface's reach, per row.** For each of the five rows
(row 1 owned-scope, row 4 depths 1/2/3, row 5 after-hole), record **whether a
`RequiredConsumerProjection` is minted at all**, and when it is not, which of
the two reasons applies:

1. `required == source` — the equality guard skips it; **the row is outside
   this surface by construction**, and a repair aimed at it must use a
   different edge; or
2. no `required_consuming_occurrence` was carried to the call in the first
   place — the row never entered `pending_required_consumer_projections`.

**These are different residuals with different owners and the frame that
confuses them will be cut wrong.** The observation seat already exists —
`CONTINUATION_REQUIRED_CONSUMER_OBSERVATIONS` (`static_transition.rs:11660`)
records `(continuation_origin, result_root, required, derived_at_consumer)` per
pending projection under `cfg(test)`. Prefer extending what a row reports over
adding a second seat.

**`D3` — record the residual in the next framer's shape.** Per row: the
boundary, the refusal sentence, and whether the row sits **behind** that
boundary or **outside** the surface. That distinction is the deliverable.

**`D4` — the carried Adversary finding, folded because it is two lines in the
control this node reads.** `evt_62attjpj3esa`, re-checked against the tree
before framing: `aggregate_arrivals` and `aggregate_forwards` are incremented
**inside** the `if let Some(established_arrivals)` block, so both sum only over
the qualified subset and are entailed by the per-case assertions
(`aggregate_arrivals >= qualified_cases` is a sum of N positives being `>= N`;
`aggregate_forwards == aggregate_arrivals` is already forced elementwise). **Move
the two increments outside the `if let`** so they sum over every case including
the complement. **The values are unchanged today** — the complement contributes
zero — so this is behaviour-preserving; what changes is that a complement case
which started arriving would then red. Same four assertions, two of them newly
able to fail.

**`D5` — ATTRIBUTE THE CROSSING. Added 2026-08-15 by Architect ruling
`evt_3q0742egf06dg`, which pre-commits every outcome.** `D1` established *which
boundary*; `D5` establishes *why the row is at it*. **All three outcomes name
their own repair, so this needs no further Architect pass.**

**Step 0 — free, do it before building anything.** `D1`'s diagnostic already
printed `lowered_value_kind(self)` at the refusal (`mod.rs:11553-11556`), so
**`Closure` vs `DeclarationClosure` may already be in `evt_6qc0vkzj43c0e`'s raw
trace.** `DeclarationClosure` points straight at the `RT-DECL-CLOSURE-PORT`
`D2a` precedent recorded at `semantic_ir.rs:950-958`.

**Step 1 — locate the crossing.** At the refusal, record which production site
entered the walk — `transfer_into_carrier` (`mod.rs:6480`) or the
carried-constructor preflight (`core.rs:15696`) — the `StaticOriginId` handed to
it, and **the path from the transferred root to the offending child** (root
variant plus the arg/field index chain). The existing trace says *"first closure
child"* and not where it sits; **the path is what separates "the projection's
own result is a closure" from "the projection's result is a field of someone
else's constructor."**

**Step 2 — the differential, on an existing seat.** Re-compile each row under
`with_required_consumer_route_suppressed` (`core.rs:27-43`, already test-only,
already used by `D2a`'s complement) and record **two facts, not one**:

- **(a)** does a closure child appear in the graph at the predecessor boundary?
- **(b)** is `transfer_into_carrier` / the constructor preflight **reached at
  all**?

**(a) alone cannot see the third branch.** Only (a) and (b) together separate
all three.

### The three branches, pre-committed — do NOT return to the Architect to choose

| measurement | branch | repair |
|---|---|---|
| closure **ABSENT** under suppression | **incorrect** | the projection manufactures it. A lowering fix **inside this chain**; [[RT-CLOSURE-BOUNDARY-LANE]] untouched; the convergence is a coincidence of totality |
| closure **PRESENT**, crossing **ALSO reached** under suppression | **correct** | the graph genuinely needs a durable closure. **Only here may the two populations be one defect** — and still subject to the shared-root rule before any subsumption is **coded**, never inferred from the ruling |
| closure **PRESENT**, crossing **NOT reached** under suppression | **the third branch** (Architect's stated PREDICTION, not a finding) | the projection advanced the row into a transfer it never attempted. The defect is the **routing**; the repair is keeping the realization local, and **neither** candidate mechanism is the right cut |

**The third branch is why a two-way measurement is not enough.** It would be
attributed to whichever of the other two it superficially resembles — the
`D2k-1c` failure mode arriving through the measurement instead of the filing.

**Why it is live rather than hypothetical:** `realize_required_consumer_locally`
is contracted to realize the projection *"without exporting a compiler-only
static worker through a function ABI"*, handing the result *"straight back to
the caller's existing exact eliminator."* **A value realized locally to avoid a
boundary should not be reaching `transfer_into_carrier` at all.**

> #### `D5` MEASURED, AND THE TABLE ABOVE IS NOW HISTORY. Steward, 2026-08-15.
>
> **Row 2 is ELIMINATED. Rows 1 and 3 are NOT SEPARATED and may not be.**
> Measured: enabled rows are `(present, reached) = (true, true)`; both suppressed
> legs are `(false, false)` and return to `StaticWorkerBinding`.
>
> **Row 2's antecedent required `reached` to be TRUE under suppression. It is
> false.** ⇒ **the durable-closure branch is dead, and with it the only branch
> under which these rows could ever have been one defect with
> [[RT-CLOSURE-BOUNDARY-LANE]].** That is my derivation from the Architect's own
> pre-committed dispositions, not a sentence he wrote — but rows 1 and 3 both
> leave that node untouched by their own terms, so it holds either way.
>
> **Rows 1 and 3 are not separated, because `closure_path` is computed ONLY at
> the crossing** (`dec_35e0tfng528d`, `evt_38p42gjq12br`). On the suppressed
> rows, `closure_child_present: false` is an artifact of there being no
> observation point — **both branches predict it exactly.** `D5`'s `CLAIMED`
> line was amended to stop claiming otherwise. **What `D5` DID establish:** the
> required-consumer route manufactures the **closure-bearing crossing** at
> `StaticOriginId(5)` / `Constructor.arg[0].Closure`.
>
> **The separator is the CALL SITE, and it is
> [[RT-CROSSING-CALL-SITE-ATTRIBUTION]]** — not another differential.
> Suppression may not settle it at all: without the projection these rows never
> build the subgraph, so *"does the closure pre-exist"* may be **ill-posed**.

**One more reason not to fold, stronger than the totality argument:**
`boundary_transfer_admissibility` carries **two** closure arms, and
`Lowered::ComputationalRecursorClosure` has its own — *"a computational recursor
closure names an in-flight activation, not a transferable value"*. **The
recursor lane already has an arm and these rows are not hitting it.** The
offending child is a **general closure value**, which the function itself
distinguishes one arm away.

## Acceptance criteria

**`AC-1` — no closure is assumed, and none is asserted.** Same rule the two
predecessors carried and the same reason. **A measured refusal with the refusal
attributed fully discharges this node.** No AC, control, or commit message may
claim a row closed.

**`AC-2` — the attribution is by sentence, not by construct.** A control
asserting `"Closure"` alone does not discharge `D1`; four refusals share that
label.

**`AC-3` — `D2` distinguishes the two not-minted reasons.** A census that
reports only "no projection" collapses the distinction the next cut turns on
and does not discharge `D2`.

**`AC-4` — no production behaviour changes.** No new consumer, no new
production edge, no guard weakening, no change to the equality guard at
`static_transition.rs:11683`. **This node measures the surface that landed; it
does not extend it.**

**`AC-5` — the key and the projection stay untouched.**
`ContinuationSpecializationKey`, `RequiredConsumerProjection`,
`derive_required_consumer_occurrence` and
`validate_required_consumer_projections` are unchanged. The injectivity premise
and the re-derivation property both stay true.

**`AC-6` — `D4` is proved behaviour-preserving.** Show the four assertions'
values are identical before and after the move on the current tree. If any
value changes, **that is a finding, not a fix to absorb** — stop and report it.

**`AC-8` — `D5` separates all THREE branches, not two.** A measurement that
records only whether a closure child appears under suppression does not
discharge `D5`; **whether the crossing is reached must be recorded
independently.** A two-way result attributes the third branch to whichever of
the other two it resembles, which is the exact failure this deliverable exists
to avoid.

**`AC-9` — `D5`'s diagnostics are reverted, as `D1`'s were.** Any temporary
instrumentation at the refusal or the crossing site is removed before release,
and the released tree carries no source diff from it. `AC-4` still binds.

**`AC-7` — no-regression, in CI** (`COORDINATION §12`). Local runs stay
targeted. **The predecessor's red was a cross-node population collision that
only CI could see** — before publishing, grep the control corpus for tests whose
population names the rows this node touches.

## Fixed inputs, measured at `23d873ce1`

- **The projection guard is `if required != source`**, `static_transition.rs:11683`.
- **The landed lowering consumer** is `required_consumer_projection_for` at
  `lowering/core.rs:11976`, ahead of the `DirectCall` settlement, gated behind
  the fusion-owned and fusion-composed early returns.
- **`realize_required_consumer_locally`** is `lowering/core.rs:12961`.
- **The depth-2 control** is `control.rs:5976`; it asserts
  `("Closure".to_string(), 2)` enabled and `("StaticWorkerBinding".to_string(), 0)`
  suppressed, with `applications == 1`.
- **Four `unsupported("Closure", ...)` sites**, tabulated above. Re-count them
  before relying on the table — it was taken at this SHA.
- **The five row fixtures exist and are named**: `row1`, `row4_d1`, `row4_d2`,
  `row4_d3`, `row5` (`control.rs:5716-5730`), each with its own capture label.
- **Row 5 sits at the `StaticWorkerBinding` wall** and row 1 is a separate
  `NativeJoinPlanV1` class.

## Excluded

- **Repairing anything this node measures.** The residual is the deliverable.
  A repair is the next cut and it is the Steward's to size.
- **Row 1** — a different class. Its `None` split is `H4` of
  [[RT-CONTKEY-REFUSAL-PROFILE-SPLIT]].
- **Row 6** — never in this population; it is [[RT-MATCH-RECURSOR-CONSUMERS]].
- **Reopening `D2k-1c`** in [[RT-LEXICAL-RECURSOR-CONSUMERS]] — a wrong cut, and
  that node has zero dispatchable increments.
- **Retiring a residual class** — [[RT-RECURSOR-TRANSPORT]] owns
  `enum RecursiveDescentResidual` (`lowering/core.rs:1979`, two live variants).
- **The tri-state convention** the Adversary raised — the empty-scan fallback
  and `validator_admitted` on `D2k-1e` owe the same conversion. **That is a
  convention question for two nodes and it is the Steward's to route.** Do not
  design it here.

## Stop condition

> **SUPERSEDED IN PART, 2026-08-15. `D1`'s stop has FIRED and is DISCHARGED.**
> It read *"if `D1` finds the refusal is `mod.rs:11550`, report and stop"*, and
> **it was scoped too wide** — `D2` and `D4` never depended on which sentence
> fired. That was a defect in this frame, not in the execution. The ownership
> question was routed and **ruled** (`evt_3q0742egf06dg`), and `D5` now carries
> pre-committed dispositions for every outcome. **Do not stop again on the
> convergence.**

**The one live stop, and it is `D5`'s:** if Step 2 shows the row **reaches no
transfer under either setting**, then `D1`'s attribution is to something other
than the projection and **the whole fork is misaddressed.** Say so and stop —
**do not force a branch.** That outcome is the Architect's explicit escape and
it is worth more than a forced fit.

**If `D4`'s move changes any assertion's value**, stop and report — see `AC-6`.

**Still not the ring's, even now:** whether the two populations are one defect.
The `correct` branch makes a subsumption *possible*, not *established* —
[[RT-LEXICAL-RECURSOR-CONSUMERS]]' shared-root rule requires it to be **coded
against a proven shared root**, never inferred from a ruling that permits it.

Everything else — sizing, sequencing, whether an increment is releasable — is
the Steward's.

## Why this earns a slot

**It is the operator's priority lane, and it is the cheapest thing on it.** The
predecessor delivered a real surface and advanced two rows; what it did not do
is say **where they landed** or **which rows the surface can reach**. Both are
reads, not repairs.

**The specific waste it prevents is a wrong cut.** This chain has already
produced one — `D2k-1c`, whose two ways forward both crossed its own banned
scope — and the tell in hindsight was that the surface it named did not reach
its population. **The same tell is present right now**, in the equality guard
that skips depth 1, and no artifact records it.
