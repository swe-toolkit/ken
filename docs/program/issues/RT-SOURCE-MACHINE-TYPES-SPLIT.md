---
id: RT-SOURCE-MACHINE-TYPES-SPLIT
title: "Relocate the existing source-machine types and control into their own child -- relocation ONLY, and explicitly not the transition IR, which is deferred to its own semantic node after this phase closes"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-LOWERING-VALUES-BOUNDARY-SPLIT]
blocks: [RT-EMITTER-CALLS-RETURNS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 12; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.


> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 12 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/core.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The source machine.** Its **existing** types and control, relocated.

> # RELOCATION ONLY. THE SOURCE MACHINE IS NOT MADE INTO A TRANSITION IR HERE.
>
> **Standing amendment, and this slice is where it binds hardest.** The IR
> recommendation — canonical planned/generated term identities, a closed source
> machine, typed rule results, a hybrid checked transducer — was **triaged and
> adopted as a target OUTSIDE this phase**, deferred to its own semantic nodes
> after closure, each with its own mutation and publication boundary.
>
> ⇒ **A "while we are in here" improvement to the source machine is out of
> scope and is not a fallback if the move is awkward.** If the relocation is
> genuinely blocked by the current representation, that is an exposed
> behavioural dependency: **stop and return it for a semantic ruling.**

> **Modules own semantic lifecycles.** The durable direction of the whole phase
> is *plan construction -> validated read-only views -> lowering state and source
> machine -> concrete backend mutation -> independent evidence -> closure ->
> publication*. **Do not name a permanent module after a temporary campaign
> node**, and do not size modules to be equal.


# `D0` — THE LEDGER. No code moves in this deliverable.

**Produce the exact old/new symbol ledger and test-property ledger for this
owner**, derived from the Stage A inventories and **re-measured at a named SHA**
on the current tree:

| inventory | what it supplies here |
|---|---|
| [type ownership](../backend-split-census-type-ownership.md) | declaring owner, visibility, external mint-shape files and all external reference files, per type |
| [lifecycles](../backend-split-census-lifecycles.md) | authority and ledger mint / transition / close / terminal sites |
| [re-exports](../backend-split-census-reexports.md) | the 57 re-export statements by build profile |
| [tests](../backend-split-census-tests.md) | `#[test]` functions, mutation surfaces and fixtures |
| [co-change](../backend-split-census-cochange.md) | which files historically move together |

**The census is a starting point, not an authority on today's tree.** It was
taken at a pinned SHA and the tree has moved. **Re-measure every count you rely
on and name the SHA you measured at** — a census row is a claim about when it
was written, not about what the tree now contains.

**State the blind spots you inherit.** The type-ownership selector cannot see
private types, macro-generated declarations, declarations whose visibility and
type keyword are split across lines, traits, constants, functions, or fields.
A ledger that does not say what its selector missed is not a ledger.

## `D0` ledger — committed, Architect-endorsed (`evt_2fmyp9r7wvm5y`)

Measured at `840baf6cd` (origin/main at pickup, 0 behind). No code moved in
this deliverable; this section is the ledger `D1`/`D2` execute against.

### Scope correction (endorsed, not a fork)

The frame's own CONTENTION section already reads "Bound files: `core.rs` and
`mod.rs`" (quoting the durable rule in Architect `evt_14x1bqgrj4yze`); the
perishable singular top-line above is superseded by the frame's own design.
`mod.rs` holds not only the four core state types but real control: an
11-method cluster in its own `impl<'a> Lowering<'a>` block. Verified: all four
core state types (`SourceMachineState`, `SourceContinuation`,
`SourceContinuationTerminal`, `SourceCallOutcome`) are touched by no file
outside `cranelift_backend/lowering/` (`core.rs`, `mod.rs`, the lowering test
tree only) — genuinely lowering-domain, movable to a lowering child with the
item-11 descendant-visibility zero-widening property.

### AC-1 — the move population, by production-injection-point tracing

Every item below is grounded by tracing its actual non-test callers, not by
name or textual adjacency.

**MOVE (`core.rs`, `impl<'a> Lowering<'a>` at 3097-20335, contiguous
6649-10091 span):** `lower_source_machine`,
`lower_source_machine_with_continuation`(`_inner`),
`lower_source_bounded_nat_match`, `lower_forked_branch`,
`lower_source_dynamic_bool_match`, `lower_source_dynamic_host_result_match`,
`source_carried_descriptors`, `source_carried_case_is_emitted`,
`lower_source_carried_leaf`, `lower_source_carried_match`,
`lower_source_{dynamic,nested_dynamic,planned_dynamic}_constructor_match`,
`source_call_state`, `machine_body_occurrence`, `owned_child_occurrence`,
`owned_case_body_occurrence` (18 methods). Pre-impl vocabulary at 3000-3096:
`SourceCarriedControlMutation`(+`Guard`), `with_source_carried_control_mutation`,
`source_carried_control_refusal`, `CARRIED_REPRESENTATION_MISMATCH_STATUS`,
`SourceCarriedCase`. Free functions `rt_continuation_kinds`,
`rt_operand_desc` (called only from the cluster). Mutation-control bundles
whose true injection point (traced to the underlying `thread_local` set/read,
not just the accessor name) lands inside the cluster: `LrcD2bLetDisposition`
+ accessors, `LRC_D2A_BACKEDGE_{ARRIVALS,FORWARDS}` + accessors +
`LRC_D2A_SUPPRESS_FORWARD`, `D8F_DECLINED_CALL_CLAIMS` + accessors
(`RT_D2_BACKEDGE_PROPAGATIONS` by contrast injects into the SHARED
`resume_active_continuation` — retained, see below).

**MOVE (`mod.rs`):** the 4 core types above; the `SourcePrefixTemplate`/
`SourcePrefixTerminal`/`SourcePredecessorEdge`/`SourceJoinTarget`-manipulating
cluster in the 15522-19852 `impl` block — `source_terminal_join`,
`discard_source_prefix`, `replace_source_terminal_with_unwind`,
`split_source_prefix`, `instantiate_source_prefix_template`,
`mint_source_predecessor`, `planned_active_scalar_cut`,
`finish_source_constructor`, `install_recursor_invocation` (every production
call site of each lands exclusively inside the `core.rs` cluster above; sole
production def of `install_recursor_invocation` @`mod.rs:17562`); plus
`record_source_machine_computational_match_selection` and
`carry_source_call_inputs` from the 6507-10786 block (same evidence).
Confirmed anchors: `lower_source_machine` @`core.rs:6649`, `source_call_state`
@`core.rs:9812`.

**EXPLICITLY RETAINED, named domain, reason** (property-over-tag/adjacency,
matching item 11's precedent):

- `lower_computational_{match_expr,producer_expr,match_value_composed}`,
  `lower_bounded_nat_computational`, `materialize_eliminator_frame_env`,
  `lower_recursor_residual_call`, `take_fused_region_at`,
  `lower_fused_producer_through_suffix`, `resume_active_continuation`,
  `reject_carried_residual_arguments` — "computational-match / eliminator-frame
  descent" (a different, not-yet-split mechanism the source machine's case
  bodies call OUT of, not INTO; textually adjacent, tagged with 8+ distinct
  other-WP citations, own doc explicitly contrasts itself with "the machine").
- The continuation/fusion cluster (`settle_continuation_candidate` through
  `dispatch_fused_consuming_call`, 10157-13354) — `RT-LEXICAL-R3-FUSION-EMITTER`
  / `RT-CONTSRC-PRODUCER-LOCAL` tagged, zero source-type touches.
- The "carried_match"/static-worker/recursor-position-unit cluster
  (14426-16086) — function-state domain (`RT-MATCH-RECURSOR-CONSUMERS`/
  `RT-DECL-CLOSURE-PORT`/`RT-RECURSOR-TRANSPORT` tagged), a likely-future item.
- `lower_expr` (16536-17907) — the lowering-wide top dispatcher every domain
  routes through, including source-machine (`ComputationalMatch` is its one
  documented special case); moving it recreates the monolith (banned scope).
- Generic occurrence helpers (`retained_body_occurrence`, `child_occurrence`,
  `case_body_occurrence`, `OwnedSourceOccurrence`,
  `disposition_statically_unselected_source_subtree`,
  `enter_source_occurrence_plan`, `SourceComputationalAnswerRoute`,
  `SourceSelectedContinuation`) — used by both the moving cluster and retained
  domains; the standing amendment ("source/child correspondence stay
  occurrence-owned") plus direct shared-call-site evidence confirm these stay
  at the LCA, not moved.
- `seal_source_trap_branch`/`emit_current_trap` — shared trap-sealing helper,
  call sites in 6+ different domains including source-machine; stays.
- `mod.rs` blocks 4822-6492 (constructor-field/static-worker-disposition) and
  12376-13334 (declared-children reconciliation) — zero source-machine touch,
  fully other-domain.
- `mod.rs` block 6507-10786 minus the two MOVE methods above — the
  carrier/aggregate emission and join-disposition domain
  (`RT-CARRIER-BYTESPAN-OBSERVE`/`RT-CONTINUATION-EDGE-DISPOSITION`/
  `RT-DECL-CLOSURE-PORT` tagged).
- `compile_expr_into_module` and its 4 siblings, plus the `CheckedFrame*`/
  `AmbientBodyAuthority`/D2f-D8n mutation-control families whose traced
  injection points land in retained methods (Ccr/Coc/Sar →
  `lower_computational_match_value_composed`; D8m →
  `lower_computational_producer_expr`; D8n → `CheckedFrameFunctionScope`;
  `RT_D2_BACKEDGE_PROPAGATIONS` → `resume_active_continuation`).

**Frozen stage predicate** (items 7/15, 8/16, 9/14): not implicated. Nothing
in the moving population is an aggregate/effect/join-trap symbol; the moving
type names appear nowhere in the planner `static_transition` tree (verified
empty); the aggregate/carrier/join-disposition code traced through is
retained, unmoved, unrenamed.

**Blind spots** (stated, not closed): macro-generated items (none found
manually, not exhaustively swept by a macro-expansion tool); split-line
declarations (manual read only, no separate selector run); cfg/attr/derive/
repr inventory not yet separately tabulated per moved item (owed at `D1` per
the frame — preservation across the move is `AC-3`'s job, recorded here as
still open); `mod.rs`'s own top-of-file items before line 4822 not yet
censused (types-only region by spot check, not fully swept — `D1` also owes
closing this).

### AC-2 — test ledger, closed by exhaustive read (231/231, not marker-sampled)

231 `#[test]` in `control.rs` at `840baf6cd`. The leader's push-back was
correct: "N tests show zero marker hits" is evidence about the markers tried,
not proof the population is empty ("an enumeration needs a proven closure, not
a better grep") — so every one of the 231 was individually read, not sampled.

**Class 3 (mutation controls at production injection point) — 10 confirmed
MOVE:**
`d8f_the_declined_call_does_not_answer_for_the_checked_identity`;
`lrc_d2a_forwards_each_arrival_and_excludes_projection_owned_early_refusals`;
`d2b_the_abandoned_let_body_joins_are_dispositioned_at_the_arm_that_abandons_it`;
`d2b_capability_gate_the_two_position_shape_refuses_before_its_case_body`;
`d2b_row_b_a_live_nonbackedge_let_runs_its_body_and_consumes_its_join`;
`px8j_source_machine_install_rejects_repeated_scope_identity`;
`px8j_source_machine_install_rejects_wrong_control_roles_and_origins`;
`px8j_source_machine_install_accepts_valid_unchecked_segment`;
`oriented_edge_mutations_reject_in_the_source_machine_consumer` (control.rs:822
— its helper `run_px8ds_source_consumer` calls `install_recursor_invocation`
directly; surfaced only by the full read, not the original marker pass);
`d6b_calling_the_selected_recursive_argument_in_the_ordinary_unit_copy_fails_closed_at_the_carrier`
(control.rs:25768 — its refusal string `"a source-machine call's callee"`
matches `core.rs:9824` verbatim, inside `source_call_state`; also surfaced
only by the full read).

**Class 1 (domain tests, source-machine's own dispatch/recognition) —
confirmed EMPTY**, by exhaustive read rather than absence-of-marker. No test's
own discriminated property is source-machine's internal Eval/Value state
stepping, backedge propagation, or arm-selection logic outside the class-3
population above.

**Class 2 (shared fixtures) / Class 4 (end-to-end, legitimately residual) —
the remaining 221, RETAIN**, each grounded in a read production call or type
traced to a specific other domain: computational-match/eliminator-frame
descent, continuation/fusion (`RT-LEXICAL-R3-FUSION-EMITTER`,
`RT-CONTSRC-PRODUCER-LOCAL`), checked-invocation/oriented-subcontinuation-plan
(PX8-DS, root-authority), function-state (carried-match, static-worker,
recursor-position-unit, `RT-DECL-CLOSURE-PORT`), planner/join-disposition
(`NativeJoinPlanV1`, `ContinuationClaimLedger`,
`RT-CONTINUATION-EDGE-DISPOSITION`), declared-unit/seed-material/B2F,
trap-exit/`RT-FNSPLIT-B2O`, and source-text census infrastructure
(`RT-FNSPLIT-B2A-C`/`B2A-S`) unrelated to any production domain.

Three clusters deliberately flagged as heavy-but-not-owned — they touch
source-machine's own state deeply without being ITS property (item-11
property-over-tag precedent, verified by tracing each producer to its actual
injection site):

- Px8jProducerPath tests (`Composed`/`DeferredConstructor`/`SourceMachine`
  variants) — 3 distinct production sites in 3 different domains; only
  `SourceMachine`'s is in-cluster.
- The D6a-upstream 8-test cluster (`D6aConsumerSeat::{Composed,
  SourceMachine}`) — a carried-match consumer's join tested against TWO
  producers, one of which is source-machine's own recursor-layer marker; the
  test's subject is the consumer's join law, not either producer alone.
- The 3 `d8e_*` tests — D8e's static-worker-binding consumer, exercised (and
  in one case deliberately routed OFF) via source-machine's `Call` arm;
  subject is the consumer, confirmed by the test that removes the
  source-machine path and shows the consumer's behavior changes for a
  different, D8d-owned reason.

**Final AC-2 population for `D1`/`D2`:** 10 tests move with the production
cluster; 221 stay in the residual `control.rs`.

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (source-machine dispatch, recognition and
control tests). Place multi-leaf fixtures **once**,
at their lowest common ancestor — `tests/mod.rs` or a narrowly named
`support.rs` — and never duplicate them. **Leave genuinely lowering-wide
controls in the residual `control.rs`.**

> ### DO NOT DECOMPOSE `control.rs` ON PRODUCTION FILE BOUNDARIES OR BY LINE RANGES
>
> **Architect `evt_6r403ez3m2m69`.** `control.rs` holds several independent
> populations — planner/occurrence, continuation/fusion, function-state and
> source-machine, emitter and join/trap controls, plus cross-cutting census and
> closure tests. **That is not one production owner**, and partitioning it by
> where the code under test happens to live today re-homes tests twice.
>
> **There is no upfront "split all the tests" phase**, deliberately: it would
> choose owners before their production boundaries exist and churn the same
> imports and fixtures a second time.

**`D1` and `D2` are separate accepted partials by default.** Combine them into
one candidate **only** when an exact compile or mutation-restoration dependency
makes the pair semantically atomic — and say which it was.


# ACCEPTANCE

**Amended on the Architect's whole-plan verdict `evt_14x1bqgrj4yze`.** The
first-cut acceptance did not prove the completeness or the preservation it
claimed; what follows is the corrected bar.

- **`AC-1` — an EXACT move ledger, closed over every Rust item class.**
  > **"Record the blind spots" is honest and it does not close them.** Stage A's
  > type selector sees 278 non-private types and does **not** see 694 `pub fn`,
  > 25 `pub const`, 7 `pub static`, 5 `pub mod`, private items, traits, impl
  > methods, macros, split-line declarations, or fields. A ledger built on it
  > alone is not exact, whatever it says about its own limits.

  Enumerate **every** moved item class: modules and re-exports; types with their
  fields and variants; traits, impls and methods; functions; consts and statics;
  cfg, attributes, derive, repr and visibility; and macro-produced owned items.
  **Each class needs its own fresh selector or syntax inventory, plus an explicit
  manual closure for what that selector cannot see.**

  **For the cfg / attribute / derive / repr / visibility class specifically:
  this records what the moved population carries TODAY. Preservation across the
  move is `AC-3`'s job at `D1`** — do not fold the two questions together.

  **A group label is not a ledger entry.** "ABI preflight helpers" names a set
  without enumerating it and does not discharge "exact".

  **PARTITION every declaration in the bound file(s).** Each one is either
  **moved to exactly one named owner**, or **EXPLICITLY RETAINED with its owning
  domain named**. A declaration that is neither is a **gap, not a non-event**.
  > **A moved-set universal is not the property that discharges "exact".** A
  > ledger can name four moved items perfectly and remain silent on the other
  > hundred-odd in the same files. Item 4's first candidate did exactly that —
  > 25 reconciled against 142 the selector returned — and it read as complete.

  Research `evt_1pwq0rssre6d8`: *"A selector count plus a blind-spot paragraph
  cannot discharge a universal."* **Declare the selector population for each
  class AND close its blind classes.** A declared population **bounds** the
  claim; it does not **discharge** it. Do not claim the universal on the strength
  of the count.
  > **The clause above is conjunctive, and the word "either" was the defect**
  > (Architect `evt_1dh3mj0janmfp`, revising its own correction on item 4's
  > evidence). Declaring the population is what makes "exact" a **well-formed**
  > universal rather than an unbounded one — so it is required *as well as* the
  > closure, never *instead of* it.

  **Source-text oracles and `include_str!` paths belong in the ledger** — Stage A
  found **49** such lines, and relocation can change what they mean without
  changing production behaviour.

- **`AC-2` — test identity and DISCOVERY, before the mutation proof.**
  > **Mutation restoration proves the discriminating tests that have mutations.
  > It does not prove that every moved test is still DISCOVERED** under the same
  > cfg and profile. A test that silently stops being collected passes every
  > mutation check that remains.

  Produce a **before/after test identity and discovery ledger for each relevant
  build profile**; execute directly and record a **nonzero selected-test count**;
  **then** the mutation proof — each moved mutation reds the **same reached
  property**, with the same **nonzero** denominator, restored. **Enumerate any
  source-oracle path or text rewrite as a non-move hunk.**

  **Each test-ledger row carries its CLASS and its exact old/new production
  INJECTION POINT.** Research `evt_1pwq0rssre6d8`, from the program report's
  four-way partition: **domain tests, shared fixtures, mutation controls at their
  production injection point, and end-to-end controls crossing planning through
  execution.**
  > **Class 4 legitimately REMAINS in the residual integration module.** A
  > ledger row without a class invites an end-to-end control to be converted into
  > a domain test, or moved by size — which is exactly what the report forbids.

- **`AC-3` — a TRANSPORT MANIFEST, not a line-pairing review aid.**
  > **Pairing removed lines with added lines is not a behaviour-preservation
  > control.** Attributes, cfg, visibility, field and variant order, derives,
  > imports and name resolution, re-export surfaces and diagnostics can all
  > change while every line still pairs.

  For **every** moved item record **old path, new path**, and an item comparison
  preserving **body, attributes, cfg, repr/derive, field and variant order,
  visibility, diagnostic text, hashes and serialization, and public/export
  profile**.

  **Permitted normalization, and nothing else:** module declarations, imports and
  path qualification, and **explicitly ledgered** adapter/re-export scaffolding.
  **Enumerate every other hunk as a non-move. A semantic hunk hard-stops the
  slice.** `git diff --color-moved` may support the review; **it cannot be the
  gate.**

- **`AC-4`** — the affected library configuration **and** the targeted test
  configurations both compile. **Control:** scoped `scripts/ken-cargo` runs only;
  the workspace gate is **CI's**, never a local run.

- **`AC-4b` — the TARGET CHILD's size is constrained, not just the root's.**
  Record the resulting line count of **every file this slice creates or
  enlarges**. **No move may CREATE OR ENLARGE any file past 10k**, and a move
  that would is a finding to route rather than a transfer to complete.
  > **"Create" alone did not match this criterion's own recording obligation**,
  > which already covers every file the slice *creates or enlarges*. The gap sat
  > on the most likely path in the plan: `lowering/core/tests/constructors.rs`
  > is **9,727** lines — 273 under the ceiling, in the very directory the fifteen
  > `D2` companion-test moves deposit into, and already **+436** with no test
  > moved yet.

  **Where a slice moves nothing this criterion is INAPPLICABLE, not satisfied**
  — `RT-PLANNER-ROOT-CLOSURE-SPLIT` under outcome 1, and the closure node, which
  deletes rather than moves. Restate it as inapplicable; do not tick it.
  > Research `evt_1pwq0rssre6d8`: none of the fifteen move frames constrained the
  > target child's size, so the phase could shrink every root while producing a
  > fresh violation.
- **`AC-5` — the ADAPTER AND FACADE DEBT LEDGER.** Any `D1` that introduces
  transitional scaffolding **appends an exact ledger** naming the symbol, why it
  is temporarily required, and **the final-closure deletion obligation**.
  > **[[RT-BACKEND-SPLIT-CLOSURE]] cannot prove it deleted "every adapter" if the
  > earlier slices never closed the population.** This criterion is what makes
  > that closure checkable, and it is owed by every slice that leaves scaffolding
  > behind.

- **`AC-6`** — this slice's own transfer is stated as complete, and **phase
  closure is explicitly NOT claimed.** Reporting a bound file's new line count as
  evidence the phase is done fails this criterion.

> ### LABEL THE THREE EVIDENCE SEATS IN THE LEDGER. Guardrail 7.
>
> **Research `evt_1pwq0rssre6d8`.** The common gate already says plans and
> commands never count as emitted evidence. The ledger must additionally label,
> per moved item, the **intention producer**, the **independent artifact
> observer / evidence decoder**, and the **closeout / publication seat** —
> **so a convenient emitter-family move cannot silently collapse them into one.**

# THE FROZEN STAGE PREDICATE — so `D0` cannot choose the boundary opportunistically

**Architect `evt_14x1bqgrj4yze`.** The per-domain symbol sets are deliberately
**not** pre-enumerated here — that would duplicate `D0` and go stale. What is
frozen is the total predicate:

- **The planner owns** plan identities, minting, relation and seat construction,
  validation and closure, and read-only projections.
- **The emitter owns** concrete CLIF/backend mutation that consumes a validated
  plan, and **may not mint or reshape planner identity**.
- **Aggregate, effect, and join/trap symbols are assigned EXACTLY ONCE across
  their planner/emitter pair.** The later `D0` **reconciles against the earlier
  LANDED ledger, not against its frame.**

That settles items 7/15, 8/16 and 9/14 as a boundary question. **The exact names
remain `D0`'s job.**

# BANNED SCOPE

- **No semantic change of any kind.** An exposed behavioural dependency **stops
  the move** and returns for a ruling; it is not repaired inside a pure move.
- **No grouping with another slice to reduce node count**, and no planner or
  lowering mega-diff. A census merge permits one frame with independently
  reviewable commits — it permits nothing else.
- **No facade that recreates the monolith**, and no widened visibility to make a
  move compile. If a symbol must widen, that is a finding.
- **No renaming for tidiness.** A move that also renames cannot be reviewed as a
  move.
- **No line-count-driven extraction.** The constraint is architectural soundness
  with a 10k ceiling, not equal-sized files.

# CONTENTION

**Bound files: `cranelift_backend/lowering/core.rs` and
`cranelift_backend/lowering/mod.rs`.**

> ### CHECK CONTENTION BY FILE INTERSECTION AT PICKUP, NOT BY THIS NODE LIST
>
> **Architect `evt_14x1bqgrj4yze`.** A frame that names today's live semantic
> nodes is **deliberately perishable** — the claim was true when written and
> decays silently.
>
> **The durable rule:** a **planner** slice checks active semantic candidates
> against `static_transition.rs` and `control.rs`; a **lowering or emitter**
> slice checks `core.rs`, `mod.rs` and `control.rs`. **A non-empty intersection
> holds the slice.**
>
> The sequencing preference stands — planner work first, lowering and emitter
> work only after semantic work has left those files.

> ### THE CHAIN'S WARRANT IS ARTIFACT DEPENDENCY, NOT SEAT COUNT
>
> **Corrected on the Architect's verdict.** This frame first justified the strict
> chain partly by there being one implementer seat. **Seat count is scheduling
> state, not architecture, and it must not be encoded as a dependency.**
>
> **The chain is nevertheless honest, for a real reason:** every `D2` reads and
> edits the same `lowering/core/tests/control.rs`, and each later `D0` must
> **remeasure the tree after the preceding production and test relocation**.
> Within the planner and the lowering/emitter groups the production roots also
> collide.
>
> ⇒ **If production and test moves were ever split into independent nodes**, the
> planner-production and lowering-production chains could **fork**, with final
> closure joining them. **With the current frames they cannot.**

**Re-derive every symbol by name at pickup**, never by line offset. `core.rs` was
20,413 lines and `mod.rs` 21,200 at `7509c77a7`; both are under active
pressure from this
phase itself.

# GATES BINDING EVERY STRUCTURAL FRAME IN THIS PHASE

These are not this slice's invention. They bind every child of
[[RT-BACKEND-MODULE-SPLIT]] and are reproduced here so a pickup does not have to
open the phase record to learn them.

- **Exact old/new symbol and test-property ledgers.**
- **No representation, diagnostic, hash, serialization, behaviour or trust
  change.** This phase is behaviour-preserving.
- **No widened production API, and no facade that recreates the monolith.**
- **Affected library and targeted test configurations both compile.**
- **Each moved mutation reds the same reached property**, with the same
  **nonzero** denominator, and is restored.
- **Plans and commands never count as emitted evidence.**
- **Source text is a census aid, not the only semantic oracle.**
- **Scoped local checks plus CI's workspace gate — never a local workspace run**
  (`COORDINATION section 12`).

> ### AN EXPOSED BEHAVIOURAL DEPENDENCY STOPS THE MOVE. It is not repaired here.
>
> If the move reveals that two regions are coupled by behaviour rather than by
> namespace, **return it for a semantic ruling.** Repairing it inside a "pure
> move" is what makes a structural slice unreviewable, because the diff then
> contains both a relocation and a change and neither can be checked against the
> other.

> ### THE THREE STANDING AMENDMENTS
>
> - **The graph foundation is not an `ids.rs` drawer.** `PredeclaredFunctionId`
>   stays unit-owned; `StaticOriginId` and source/child correspondence stay
>   occurrence-owned.
> - **`boundary_value_clif.rs` is not absorbed merely for size.** Its lifecycle
>   and consumers must be proven first.
> - **The source machine is relocation only in this phase**, never a transition
>   IR. Generated traps receive **no fabricated source origin**.

