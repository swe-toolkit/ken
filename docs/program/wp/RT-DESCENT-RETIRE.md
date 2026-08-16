# RT-DESCENT-RETIRE — delete the selector, the enum, the authority, and the lane

**Four residual classes were retired by porting and two were DISPOSITIONED —
`LexicalCallArgumentRecursor` as fixture-only and `MatchScrutineeRecursor` as
capability-inert — so the migration selector still exists, still evaluates on
every compilation, and the `RecursiveDescent` emission lane is still compiled
in and still selected by 28 fixture entries. This node deletes it. That residue
is the tech debt the directive names, so this is a required node, not a
tidy-up.**

> **The lede used to read *"with all five residual classes retired ... the lane
> is dead."* `D1` measured 28 live selections at `97b963ac4` and it is corrected
> above rather than left to mislead.** The lane is **not** dead; it is
> **dispositioned**, which is a different claim and the one this node acts on.
> Both dispositions defer the variant's actual retirement to this node. See
> `D1`, `D2b` and the re-aimed hard stop in section 7.

**Owner:** Team Runtime. **Branch:** `wp/RT-DESCENT-RETIRE`. **Size:** M.
**Risk:** medium — a wide deletion across five production files, with a
**one-shot** oracle.

**Read `docs/program/16-recursive-descent-retirement.md` first.**

**Gated on five nodes, not four.** Do not start until
[[RT-DECL-CLOSURE-PORT]], [[RT-SEED-CALL-PORT]], [[RT-PRODUCER-MATCH-PORT]],
[[RT-RECURSOR-TRANSPORT]] **and [[RT-FNUNIT-RESULT-TOKEN]]** have merged.

**The fifth is not a migration node and its gate is not "the class is
retired."** `RT-FNUNIT-RESULT-TOKEN` owns `nc22`, the only program exercising a
shape that **only the lane you are deleting supports**. It is `#[ignore]`d under
that node's quarantine — so if you delete the lane first, the capability
disappears and **the single row that would have caught it is already
suppressed**. Its gate is `nc22` running **green on the `FunctionizedUnits`
lane**, not the skip being tidied. Added 2026-08-08 by the Steward; that node
was filed after this frame was written.

---

## 1. Fixed inputs

| path | blob at `origin/main = 14c3c5f7` |
|---|---|
| `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` | `f7bc0d0354d8b8d6f7aa68176846b7b05e5a8514` |
| `crates/ken-runtime/src/cranelift_backend/lowering/mod.rs` | `b924db34df3be74421fa773132fe476a53503ecc` |
| `crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs` | `f9d7fc1025bfa80cb5eaf66284252d3bdd59c28c` |
| `crates/ken-runtime/src/object_linker_packaging.rs` | `59d2940576894f516494c28c5b8d66a8260337f8` |

**Every one of these is stale by pickup** — four nodes run first. **Re-pin at
pickup.** These are recorded to bound the *surface*, not to be trusted as
values.

## 2. The surface

At `origin/main = 14c3c5f7`, `BodyEmissionAuthority` / `RecursiveDescent`
occurrences span **five production files plus three test modules**:

| file | occurrences |
|---|---|
| `lowering/core.rs` | 22 |
| `lowering/core/tests/control.rs` | 16 |
| `lowering/mod.rs` | 4 |
| `planning/static_transition.rs` | 3 |
| `object_linker_packaging.rs` | 1 |

Plus `core/tests/constructors.rs` and `core/tests/effects.rs`.

**A deletion that misses a file leaves a dead branch that still compiles.**
The count above is the pre-campaign surface and will have moved; **re-derive
it, do not re-pin it.**

## 3. THE ORACLE IS SPENT BY THE COMMIT THAT CLEARS IT

Once the last residual class is retired, **nothing in the tree can distinguish
"the lane is unreachable" from "the lane was deleted."** The evidence that the
lane is genuinely dead exists **only before this node lands**, and this node
destroys it.

⇒ **`D1` captures that evidence first, while it is still capturable.**
Do not begin deleting and then attempt to prove the lane was dead — by then
the proof is unavailable and any argument for it is circular.

> ### THIS NODE ACTIVATES PORTED ARMS WHOSE EVIDENCE NOTHING WILL RE-READ
>
> **Added 2026-08-08 from an Adversary finding on merged `3061a645`. Carry it
> as an explicit checklist item rather than discovering it here.**
>
> The recursor arc lands repairs under **port-then-activate**: an arm is written,
> proven correct by reasoning plus a record, and is then **neither
> production-reachable nor test-exercised** until the variant retires. The
> `carried_join_arm` backedge representation is the current example — zero
> arrivals in an unhooked run, so nothing in `crates/` demonstrates it at all.
>
> **This node flips both properties at once.** The arm becomes live, and its
> reasoning — predecessor-free block, the word never read, mirror of the scalar
> lane — becomes **load-bearing for the first time**, with no control standing
> behind it.
>
> **Nothing about the arm changes at that moment, so nothing prompts anyone
> to re-read its evidence.** That is the whole hazard: a diff-driven review sees
> an untouched arm and moves on. It is the same **cost-moves-at-activation**
> shape recorded at [[RT-SEED-CALL-PORT]] `D3`, where unmutated `AC-6` controls
> went from guarding an inert path to guarding production without changing.
>
> ⇒ **Enumerate every arm this retirement activates, and re-read each one's
> evidence at activation** — not because it was wrong, but because it was never
> load-bearing before. An arm whose only demonstration lives in a `docs/`
> record is the priority: the code surface will not remind you it exists.

## 4. Deliverables

- **`D1` — Capture the spent-oracle evidence, BEFORE any deletion.** On the
  pre-deletion tree: run the full-residual enumeration over every measured
  program and the whole test corpus, and **record every residual that fires and
  every program that selects `BodyEmissionAuthority::RecursiveDescent`.** Post
  this before `D2`.

  > ### `D1`'s ZERO-SELECTION EXPECTATION WAS NEVER REACHABLE. Corrected 2026-08-16.
  >
  > **This deliverable used to require *"no residual fires anywhere"* and *"no
  > program selects `RecursiveDescent`"*, conditioned on *"all five classes
  > retired."*** It ran at `97b963ac4` and returned **28 selections across 18
  > tests — 27 lexical-call-argument, 1 match-scrutinee.**
  >
  > **Zero was unreachable by construction.** The campaign retired four variants
  > by porting and **dispositioned** two; "no port owed" was never "no selections
  > remain." [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]]'s ratified disposition says
  > the variant *"is re-described as fixture-only with the retained lane and
  > **retires when `RT-DESCENT-RETIRE` removes that lane.**"* ⇒ **The variant
  > cannot stop selecting until this node acts, and the old text forbade this
  > node from acting until it stopped selecting.**
  >
  > **The corrected expectation** (Architect `evt_98zg6sbqh7ej`): *the surviving
  > selections are the two dispositioned populations, pinned by test name and
  > rendering at `97b963ac4`.*
  >
  > **The warrant is a differential across two SHAs, not an arithmetic
  > partition.** An earlier `D1` found **31 selections across 20 tests = 27 L +
  > 4 M**; today's found **28 across 18 = 27 L + 1 M**. The lexical population
  > was predicted stable by its own node's `AC-5` and is stable, 27 to 27; the
  > match population was predicted to shrink under the
  > [[RT-MATCH-SCRUTINEE-DISPOSITION]] narrowing and shrank, 4 to 1. **Two
  > populations moved independently, each exactly as its own node predicted.**
  >
  > **Do NOT rest this on `27 + 1 = 28`.** `RecursiveDescentResidual` has
  > exactly two live variants at `97b963ac4` (`lowering/core.rs:2015-2025`;
  > `TransparentDeclarationClosure` was retired by `RT-DECL-CLOSURE-PORT` `D6`),
  > so every selection is necessarily one of the two. **"No third class" is true
  > by construction and carries no information** — it is a property of the enum,
  > not evidence about the sites.

- **`D2b` — THE PINNED POPULATION. TRANSCRIBED BELOW BY THE STEWARD; nothing is
  owed for it.** Architect `evt_98zg6sbqh7ej` required this before `D2c` is
  entered; `D1` produced the names (runtime-leader `evt_6tn48svtnnc4s`) and this
  is transcription, not measurement.

  **THE PIN — 28 selections across 18 tests, at exact `97b963ac4`.** Selection
  count in parentheses. **Every entry is `LexicalCallArgumentRecursor` except
  the one marked `M`.**

  | # | test | sel | rendering |
  |---|---|---|---|
  | 1 | `recursive_computational_aggregate_traverses_ordinary_frame` | 1 | L |
  | 2 | `recursive_computational_host_result_keeps_established_dynamic_lane` | 1 | L |
  | 3 | `d0_r3_fusion_gate_resolves_zero_for_the_seed_and_one_for_the_checked_twin` | 1 | L |
  | 4 | `d0_row2_functionized_lane_never_reaches_the_source_machine_mint` | 1 | L |
  | 5 | `d2e_ac9_layout_agrees_with_the_prefix_production_assembled` | 1 | L |
  | 6 | `d2f_0_the_applied_root_production_path_gate` | 1 | L |
  | 7 | `d2f_a_production_compile_builds_the_fusion_identity_plane` | 1 | L |
  | 8 | `d2k_0_control_reddens_when_the_wrong_consumer_condition_is_removed` | 5 | L |
  | 9 | `d8_every_required_join_plan_is_consumed_exactly_once` | 2 | L |
  | 10 | `msd_d2a_the_retention_and_routing_guards_have_a_concrete_difference` | 1 | **M** |
  | 11 | `px8j_all_three_producer_paths_reach_real_consumers` | 1 | L |
  | 12 | `px8j_one_two_three_scope_segments_reach_selection_hole_and_unwind` | 3 | L |
  | 13 | `px8j_owned_scope_deletion_fails_closed_before_another_frame_is_emitted` | 2 | L |
  | 14 | `px8j_selected_scope_partitions_differ_across_the_real_return_hole` | 2 | L |
  | 15 | `px8j_siblings_share_an_origin_and_nested_ih_gets_a_child_origin` | 1 | L |
  | 16 | `recursive_descent_recursors_compile_without_a_boundary_crossing` | 2 | L |
  | 17 | `row2_functionized_lane_installs_and_consumes_the_recursive_ih` | 1 | L |
  | 18 | `rt_d1_the_exact_position_b_witness_carries_without_a_port` | 1 | L |

  **27 L + 1 M = 28. Sum verified against the per-test counts, not assumed.**

  **The single `M` is row 10, and it is the natural one:** that test is
  [[RT-MATCH-SCRUTINEE-DISPOSITION]]'s own `D2a` control — the one asserting the
  retention and routing guards have a concrete difference. It is the surviving
  `M` selection because it is the test built to exhibit exactly that.

  > **This set is FROZEN. It is what makes the expected-red clause falsifiable.**
  > `D2c` licenses a red on these 18 as an expected shape, and **an expected set
  > defined after the run by whatever actually reds is a null oracle.** Do not
  > widen it after seeing CI. Widening it is the finding, not the fix.

  ### `D2b` HAS TWO CATEGORIES. The domain was under-specified; corrected 2026-08-16.

  **Category A above pinned 28 PROGRAM ARRIVALS and nothing else.** `D2c` reded
  three tests that **call `select_body_emission_authority` directly**, compile no
  program and assert no value — a disjoint category the pin never enumerated.
  **The Steward froze the set `D1` produced without asking what else could red.**

  > ### MEMBERS versus CATEGORIES. This is the rule, and it binds next time.
  >
  > **You may NOT add MEMBERS to an expected set because they reded.** That is
  > unfalsifiable and it is what `AC-8` forbids. **You MAY enumerate a CATEGORY
  > whose membership predicate is decidable without the experiment's result.**
  >
  > **The test is one question: could the set have been computed BEFORE the
  > run?** A grep at the base SHA — yes, and it returns the same answer today as
  > it would have yesterday. *"It reded"* — no, never. ⇒ **This was a PROCESS
  > failure, not an evidentiary one**, and that distinction is the whole content
  > of `AC-8`. Architect ruling `evt_38c0px3312y62`.

  **CATEGORY B — every direct caller of `select_body_emission_authority`,
  enumerated by grep at base `c98f72ba8`.** Architect census, **re-verified
  independently by the Steward**: 28 mentions in `control.rs`, of which
  `:17644` is a doc comment ⇒ **27 real call sites**, plus the definition
  (`core.rs:2406`) and the single production call (`core.rs:2650`). Every site
  classified by the authority asserted within four lines; **all 27 resolved,
  none unclassified.**

  | sites | asserts | `D2c` outcome |
  |---|---|---|
  | `:10153`, `:10171`, `:10213` (`the_body_authority_selector_narrows_only_completed_ports_and_stays_fail_closed`) | `RecursiveDescent` | **red** |
  | `:10290` (`retained_authority_residual_is_the_typed_selector_accounting`) | `RecursiveDescent` | **red** |
  | `:18216` (`d5_c3_a_second_residual_retains_recursive_descent`) | `RecursiveDescent` | **red** |
  | `:16428` (`msd_d2a_the_retention_and_routing_guards_have_a_concrete_difference`) | `RecursiveDescent` | **red**, and also in category A |
  | the remaining 21 | `FunctionizedUnits` | **green** |

  ⇒ **Every direct caller asserting `RecursiveDescent` reded. Not one stayed
  green. Zero unaccounted.** And **the 21 green ones are a real control**: the
  reroute did not blanket-break selector callers, it broke exactly those pinning
  the retiring return value and nothing else. **That is the non-degenerate pair
  this campaign needed, and it already existed.**

  **`:16428` sits in BOTH categories** — it calls the selector directly *and*
  compiles programs, and it reded as an arrival. **The categories overlap; they
  do not conflict.**

  > ### THREE THINGS THAT WOULD HAVE MADE THE STOP STAND — none occurred. Keep them.
  >
  > 1. **Any red from a program not in the 28.**
  > 2. **A red from a direct caller asserting `FunctionizedUnits`** — the reroute
  >    breaking something it should not.
  > 3. **A `RecursiveDescent`-asserting direct caller that stayed GREEN.** ⇒ the
  >    reroute did not take effect everywhere and **`D2c` is invalid as a
  >    measurement.** **Keep this one especially: it validates the INSTRUMENT
  >    rather than the result**, and it is the only one of the three that fails
  >    the run rather than the retirement.
- **`D2` — A positive control on `D1`'s instrument.** Reintroduce one residual
  temporarily and confirm the enumeration **reports it** and the authority
  **flips to `RecursiveDescent`**. Restore byte-identically.
  **Without this, `D1` is a negative check that passes for any reason** —
  including a broken instrument.
- **`D2c` — STEP 1 OF THE RETIREMENT: ROUTE TO NOTHING, DELETE NOTHING.**
  Architect `evt_5f4jvs4f6pbdt`, 2026-08-16. Make
  `select_body_emission_authority` **never return
  `BodyEmissionAuthority::RecursiveDescent`**, leaving the lane, the residual
  enum, the authority variant and the emission code **entirely in place**. One
  revertible change. Then the **full workspace suite plus conformance, in CI**
  (`COORDINATION §12` — CI, never the laptop).

  **This is the measurement, and it is the largest differential available
  anywhere in this campaign: every program in the corpus becomes a differential
  row.** [[RT-MATCH-SCRUTINEE-PORT]] settled the semantics over **five
  hand-built `RuntimeExpr` with no kernel `Term` preimage**; the retirement acts
  on **every program the narrowed guard retains.** Those are not the same set.
  **No census is owed** — green here settles the population question over the
  whole corpus instead of over five fixtures.

  **If step 1 reds anywhere, that red IS the capability the five rows could not
  see**, and it arrives as a concrete failing program rather than as an
  argument. Hand it back as a node; do not delete around it.

  **`D2c` LANDS AS ITS OWN CANDIDATE, AHEAD OF ANY DELETION.** Partial-WP merge
  is standing policy (`COORDINATION §10⁻`); here it is load-bearing rather than
  convenient, because a revert of step 1 must restore production routing in a
  single commit.

  > ### `D2c` RAN. NOT CAPABILITY-BLOCKED — the RECORD is the gate.
  >
  > **Final disposition, Architect `evt_3bkkjpps1bcpe`.** The nine refusing
  > programs are **zero source-reachable** (five hash-tagged lexical fixture
  > renderings, all within the twelve, hashes re-checked against the merged
  > node's own table), and `D3` is **negative** — no merged completeness claim
  > falsified, no erratum. ⇒ **No user-facing capability is lost.**
  >
  > **What gates `D3`-`D8` now is the RECORD**, not capability: after deletion
  > the record is all that survives of four representability facts, so the
  > successor's `D1` must land **one verdict per construct** and **each leaves a
  > PIN, not prose**. The earlier BLOCKED reading is kept below as the record of
  > what one adjudicated row supported before the other eight were mapped.
  >
  > ### SUPERSEDED: `D2c` RAN AND THE RETIREMENT IS BLOCKED. `evt_35hwm50tas8kp`.
  >
  > **The sentinel failed on assertion 1 verbatim — `must retain its compiling
  > RecursiveDescent baseline`.** `RecursiveDescent` compiled row 4 depth 2 at
  > base; **the functionized lane REFUSES it at `StaticWorkerBinding`.** Same
  > program, two worlds, different behaviour ⇒ **a regression established
  > differentially on a real production compile.**
  >
  > **No `D6` re-home is lawful. `D3`-`D8` gated. `D2c` stays UNPUBLISHED** —
  > one revertible commit that never landed, which is `AC-7` earning its keep.
  >
  > **AND IT IS NOT ONE CONSTRUCT.** Architect `evt_7qtgrtwv76vke` on the ring's
  > inventory `evt_6bvnv6t4teech`: **nine of the fourteen in-set reds are the
  > surviving lane refusing a program the retiring lane compiles, across FOUR
  > independent constructs** — `ComputationalMatch` in-flight non-transferable
  > activation (4), `StaticWorkerBinding` (2), backend `Module` missing
  > recursive-position-1 worker projection (2), backend `PlannerInvariant`
  > missing affine checked-root authority (1). **A pattern, not an omission.**
  >
  > **The artifact hypothesis is CLOSED:** the identical refusal reproduces at
  > untouched base `c98f72ba8` through the **pre-existing** exclusion mechanism,
  > touching no production code. **The finding is about the lane, not `D2c`'s
  > edit.**
  >
  > ⇒ **The successor is [[RT-DESCENT-LANE-COMPLETENESS]], framed as a
  > lane-completeness question rather than a port.** Whether this node is
  > BLOCKED or merely incurs recorded representability gaps is decided by that
  > node's `D2` — the source-reachability of the nine. **The full record is in
  > the successor's frame.**

  > ### HOW TO READ A RED. Three conditions, Architect `evt_98zg6sbqh7ej`.
  >
  > **1. A red from a program NOT in `D2b`'s pinned set is the HARD STOP, in
  > full.** That is exactly the surviving-class case section 7's carve-out was
  > written for, and `D2c` is what surfaces it as a concrete failing program
  > instead of an argument. **The stop is re-aimed, not retired.**
  >
  > **2. A red INSIDE the set is adjudicated PER TEST, never excused per set.**
  > Each one individually: a test asserting a property **of the retiring lane**
  > retires or is re-homed under `D6`; a test asserting a **semantic property
  > reachable on the surviving lane** means the lane is capability-load-bearing
  > after all and **the retirement is blocked.** ⇒ **"Expected shape" licenses
  > WHICH tests may red. It never licenses what a red MEANS.**
  >
  > **3. Fixture-only does not make a fixture safe to delete.** `0/12` source
  > reachability bounds what a red costs in **user-facing capability**; it does
  > not make the fixture worthless, because **a fixture can be the only witness
  > for a shape.** That is the `nc22` reasoning this node's own release gate
  > turned on — retiring the witness alongside the capability removes both in
  > one commit. Most of the twelve will legitimately retire with the lane they
  > exercise; **the one to watch is a fixture that incidentally witnesses
  > something on the SURVIVING lane.** `AC-5` and `D6` cover it, and it is
  > stated here because the pressure under a line reading *"expected shape,
  > route it"* runs toward deletion.

> ### `D3` THROUGH `D8` ARE STEP 2, GATED ON `D2c` GREEN IN CI.
>
> **DO NOT COLLAPSE THE TWO STEPS.** Architect `evt_5f4jvs4f6pbdt`: a single
> commit that reroutes **and** deletes cannot tell a routing regression from a
> compile error, and the evidence is destroyed either way it goes. **Step 1 is
> the measurement; step 2 is bookkeeping over provably dead code.**

- **`D3` — Delete the classifiers**: `recursive_descent_residual`,
  `declaration_recursive_descent_residual`, `RecursiveDescentResidual`, and
  `select_body_emission_authority`.
- **`D4` — Delete the authority**: `BodyEmissionAuthority::RecursiveDescent`
  and, if the enum is then a single variant, the enum itself and every branch on
  it across all five files.
- **`D5` — Delete the recursive-descent emission lane** it selected.
- **`D6` — Retire or re-home the lane's tests.** Tests that exercised the
  `RecursiveDescent` lane are testing deleted code. Do not delete a test that
  is actually asserting a *semantic* property reachable on the surviving lane —
  re-home those. Do not keep a test green by keeping dead code alive for it.
- **`D6b` — ANSWER THE COVERAGE QUESTION THE ADVERSARY LEFT OPEN.** Folded here
  2026-08-08 rather than filed as its own node: it is a coverage-accounting
  question, `AC-5` already forbids a silent net loss, and this node performs the
  last deletion that can change the answer.

  **The question, in the Adversary's words** (`evt_7fx8em9q24p8h`, on the merged
  `RT-PRODUCER-MATCH-PORT` `D3`): *"after this retirement, does any live row
  still exercise the ported shape, or does it now have zero live coverage in
  either direction?"*

  **It named this as unmeasured on purpose and that is the right disposition.**
  It had taken three instrument errors from rushing population measurements, and
  judged a wrong answer here worse than no answer. **Do not read the absence of
  a figure as evidence either way.**

  **What makes it live rather than academic.** That node's `D3` reverted one row
  to its original program, so the row no longer exercises the ported shape, and
  it re-homed two others that **nobody has verified**. If those three were the
  only coverage, the shape now has none — and the retirement would have removed
  the lane *and* its witnesses in two separate merges, neither of which could see
  the other.

  ⇒ **Answer it from the fixture set, in both directions:** which live rows
  exercise the producer-call-in-scrutinee shape on the surviving lane, and which
  exercise its refusal. **Zero in either direction is a finding to route, not a
  gap for you to fill here.** The Adversary states it is cheap for the ring to
  run; if it is not, say so rather than estimating.

- **`D6a` — SWEEP THE REACHABILITY-PREMISED "CANNOT OCCUR" ARGUMENTS.** Added
  2026-08-08 from a measured falsification, folded here rather than filed as its
  own node because **this node makes the largest reachability change in the
  campaign** and the sweep is worthless before it.

  **The measurement that demands it.** During `RT-PRODUCER-MATCH-PORT` `D2` an
  arm was found refusing with an argument that had become false:

  > *"a deforestable producer is by construction one whose shape was read at
  > compile time. So a carried scrutinee cannot arrive here from today's
  > corpus."*

  **`RT-SEED-CALL-PORT` `D3` falsified it** — `requires_heterogeneous_deforestation`
  classifies on the **source** shape while the callee is now lowered as a
  separately owned unit. Nobody was looking for it; the implementer hit it
  building the next node, and it was **the same implementer who had landed the
  commit that broke it.**

  ⇒ **The campaign's entire purpose is changing which lane a program takes, and
  an in-code argument premised on the old reachability goes false SILENTLY.** A
  stale "cannot occur" is not merely wrong prose — **it is the justification for
  an arm that may now be reachable and wrong.** No test reds.

  **THE 46 IS A CANDIDATE SET, NOT A BOUND — AND THE PHRASE LIST MISSES THE
  CLASS THAT WAS ACTUALLY FALSIFIED.** A grep for
  `cannot (arrive|occur|reach|happen)` / `never reaches` / `unreachable in
  practice` / `by construction` across `cranelift_backend/lowering/*.rs` and
  `planning/*.rs`, excluding tests, returned **46 hits at
  `origin/main = 1699e0a3`.** **Do not treat that as the population.** I
  selected it by phrase while scoping this deliverable by premise; those are not
  the same set, and the gap is not hypothetical.

  **Two verified counterexamples**, both production, both load-bearing
  impossibility premises, **neither matching any of the four patterns**
  (Adversary, on the `D2` merge; coordinates re-derived by me at
  `origin/main = 55d811b8`, since the reported ones had drifted):

  - `lowering/core.rs:11324` — *"planner proved impossible, and no switch could
    instantiate it."*
  - `lowering/core.rs:15447` — *"...already refused every other source, so this
    arm is **unreachable-by-validation** rather than a fallback."*

  ⇒ **The second is the falsified claim's exact structural shape:** an arm whose
  safety is **delegated to a named upstream stage's refusal.** The one that broke
  held precisely that, and broke because the upstream stage classified on the
  **source** shape while the world moved underneath it.

  **And note the trap in the wording:** *"unreachable-by-validation"* is one
  hyphen from *"unreachable in practice"* and means something different — it
  names **which stage is trusted**, which is exactly the premise that can go
  stale.

  ⇒ **Widen the selector before you enumerate.** The at-risk premise *"an
  earlier stage already refused this"* is expressible with none of the four
  phrases. A first cut — `already refused` / `refused every` / `proved
  impossible` / `validated upstream` / `guaranteed by the (planner|validator)` —
  finds **4 more in `core.rs` alone**. **Treat both lists as seeds and state the
  selector you actually ran.**

  **Neither counterexample is claimed to be false.** Their truth was not
  evaluated; they are evidence about the *enumeration's coverage*, nothing more.

  **Scope it by the premise, not by the phrase.** The at-risk class is any claim
  resting on *which values can reach a point* — carried versus compile-time
  shape, which authority a program selects, which lane an arm sits behind.
  A claim resting on a type or a structural invariant is not at risk.

  **Report the classification, not just the fixes.** For each hit: at-risk or
  not, and if at-risk, still-true or falsified. **A hit dismissed without a
  stated reason is not swept.** Re-run the grep at your own base and state its
  domain beside the result — the previous node's sweep failed by running a
  narrower domain than the claim it made.
- **`D7` — The closing measurement**: emitted function count and per-function
  code-size distribution across the measured programs, against
  `RT-DECL-CLOSURE-PORT.AC-6`'s opening figures.

- **`D8` — RE-DESCRIBE THE FIVE REFUSAL CONTROLS. Do not repair them and do not
  retire them.** Absorbed into this node 2026-08-16 when
  [[RT-RECURSOR-TRANSPORT]] closed at PR #2443/#2444.

  **The governing text is the `2026-08-16` banner at the head of
  `docs/program/issues/RT-DESCENT-RETIRE.md`.** Read it before opening this
  deliverable; it is not restated here, so that there is one authority and not
  two that can drift.

  What it fixes, in one sentence each:

  - **Repair is foreclosed** (Architect `evt_5h7vzc27mc11j`). None of the five
    failures is a capability gap — they are a conservation law, a planner
    invariant, a semantic impossibility, and a structural absence. **Row 1's
    refusal IS the invariant [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]] landed
    at PR #2440**, so repairing it would undo a ratified disposition.
  - **The new expected values are already measured** — the per-category first
    outcomes in `docs/program/wp/RT-RECURSOR-TRANSPORT.md` at PR #2443.
    **Do not re-measure them.** The re-description is specified by measurement,
    not by assertion.
  - **Write each pin as unobserved-by-construction, not as rejected-forever.**
    An expectation change is not a repair, and the five rows are
    internal-contract pins on the emitter's refusal — they cannot observe
    frontend reachability. Labelling one a reachability tripwire would be worse
    than leaving it unlabelled; that gap is filed separately as
    [[RT-FRONTEND-REACHABILITY-TRIPWIRE]] and is **not** yours here.
  - **Two dispositions are open and this node settles them**, rather than
    inheriting them silently: the two-sibling rows, and corrected row 2.
    **`d8d` is a COUNT DIVERGENCE, not a refusal** — different owner, and it is
    never partitioned into a refusal bucket.

## 5. Acceptance criteria

- **`AC-1`.** The whole test corpus **compiles and passes** with the lane
  deleted. Workspace green **in CI** — never a local `--workspace` run
  (`COORDINATION §12`).
- **`AC-2`.** `D1`'s evidence and `D2`'s positive control are both in the tree.
  `D1` without `D2` does not discharge this AC.
- **`AC-3` — the deletion is complete.** No `BodyEmissionAuthority`,
  `RecursiveDescentResidual`, or recursive-descent lane symbol survives in
  `crates/ken-runtime/src/`. This is a **review** obligation on the QA seat
  and a compile consequence — **not** a grep oracle committed as a test
  (operator: source-text oracles are an invitation for failure and delay).
- **`AC-4`.** `D7`'s closing figures are recorded next to
  `RT-DECL-CLOSURE-PORT.AC-6`'s opening figures. Report; do not tune, and
  do not pin a threshold — a size number rots at the next merge.
- **`AC-5`.** Every test removed under `D6` is accounted for: retired as
  lane-specific, or re-homed with its semantic property intact.
  A silent net loss of coverage fails this.

  **`D6b` is inside this AC, and it extends the accounting backwards.** The
  coverage that can go silently missing is not only what *this* node removes —
  `RT-PRODUCER-MATCH-PORT` `D3` already reverted one row and re-homed two
  unverified ones. **Answering `D6b` in both directions is part of discharging
  `AC-5`**; a coverage claim that only accounts for this node's own deletions
  does not discharge it. **Zero live coverage in either direction is a finding
  to route, not a failure of this AC** — the AC fails when the question is left
  unanswered, not when the answer is unwelcome.

- **`AC-6`.** Each of the five refusal controls carries a re-description whose
  expected value cites the measured first outcome in
  `docs/program/wp/RT-RECURSOR-TRANSPORT.md`, and the two open dispositions are
  settled explicitly in this node's record. **A control left with its old
  expectation, or re-described from reasoning rather than from that
  measurement, does not discharge this.**

- **`AC-10` — the sentinel is dispositioned on WHICH ASSERTION FAILED, never on
  reasoning.** `recursive_descent_recursors_compile_without_a_boundary_crossing`
  is the one red of the 14 whose own doc comment claims **in terms** that
  *"retiring that route before the live-domain lane covers these rows would
  remove a compiling capability."* **Its body never measures that** — the
  functionized leg captures `_excluded_result` and **discards it**, so the
  `CLAIMED` line asserts something the code does not execute.

  **Its unexcluded leg carries exactly two assertions, in order:** (1)
  `result.is_ok()` — *"must retain its compiling `RecursiveDescent` baseline"*;
  (2) `crossings.is_empty()` — *"must not gain a `RecursiveDescent` boundary
  crossing"*. **Assertion 1 precedes 2, so the FIRST failure message is fully
  discriminating**, and it is one line of output the run already produced.

  | first failure | disposition |
  |---|---|
  | *"must not gain a `RecursiveDescent` boundary crossing"* | **The sentinel fired as designed. No capability loss** — under `D2c` the unexcluded leg IS the functionized route, so the route comparison collapses, which is exactly what its declared promise class anticipates: *"transition sentinel. Retirement or an authorized boundary repair must rewrite this route comparison rather than preserve its current exact outcomes."* **Rewrite under `D6`.** And assertion 1 having passed is **affirmative evidence the functionized route compiles row 4 at depths 2 and 3** — the measurement its `CLAIMED` line asserted and its body never made. `D2c` supplies it free. |
  | *"must retain its compiling `RecursiveDescent` baseline"* | **The functionized route does not compile these rows.** Handed back as [[RT-DESCENT-LANE-COMPLETENESS]]; **not adjudicated inside `D6`.** **AMENDED 2026-08-16 (`evt_3bkkjpps1bcpe`): this row is NOT a capability loss.** All nine such programs are **zero source-reachable**, so the retirement incurs **recorded representability gaps**, not lost capability. The row still blocks `D6` — but on the RECORD (a pin per construct, after the successor's `D1`), never on capability. |

  **Neither branch may be taken by argument.** Architect `evt_38c0px3312y62`.

- **`AC-8` — the expected-red set was FROZEN BEFORE the run.** `D2b`'s pinned
  18 tests / 28 selections landed on `main` ahead of the `D2c` candidate, and
  the set used to classify CI results is that one, unwidened. **A set enlarged
  after seeing which tests actually red does not discharge this** — it is a null
  oracle, and the enlargement is itself the finding to route.

- **`AC-9` — every red inside the set is adjudicated INDIVIDUALLY**, each
  recorded as either lane-specific (retire or re-home under `D6`) or a semantic
  property the surviving lane refuses. A per-set disposition does not discharge
  this, and neither does a count.

  > **AMENDED 2026-08-16, `evt_3bkkjpps1bcpe`.** This AC used to say the second
  > arm means *"the retirement is blocked"*. **It does not, and that reading was
  > too strong.** A refused property blocks only if it is **source-reachable**;
  > the nine measured here are zero-reachable and therefore **recorded gaps**.
  > **The arm's real obligation is a PIN per construct after the successor's
  > `D1`** — correct semantics leaves an asserted-refusal pin, a missing port
  > leaves an owned obligation.

- **`AC-7` — the two steps are SEPARATE CANDIDATES.** `D2c` merged, with the
  full workspace suite and conformance green in CI, **before any deletion
  lands**. A single candidate that both reroutes and deletes **fails this AC
  however green it is** — the point is that its red, had it been red, would
  have been uninterpretable.

## 6. Banned scope

- **Starting before all five gating nodes merge** — the four migration nodes
  and [[RT-FNUNIT-RESULT-TOKEN]]. A partial deletion is strictly worse than
  none: it removes the fallback while a class can still select it, or while a
  shape still has no other lane to run on.
- **Keeping the lane "just in case."** That is the half-migrated state the
  directive rules out. If a case still needs it, **stop** — the campaign is
  not finished and the missing class is a node, not a retained fallback.
- **Deleting a test that asserts a property still reachable** on the
  surviving lane.
- **Repairing or retiring the five refusal controls.** `D8` re-describes them.
  Added 2026-08-16 — repair is foreclosed by Architect `evt_5h7vzc27mc11j`, and
  retiring them would delete the only pins on refusals that a ratified
  disposition rests on.

## 7. Hard stop

Stop and report if `D1` finds any residual still firing, if `D2`'s positive
control fails to flip the authority, **if `D2c` reds anywhere in CI**, or if the
deletion cannot complete without retaining a `RecursiveDescent` branch.
**Any of those means the campaign is
not done, and the honest outcome is to name the surviving class and hand it back
to the Steward as a node** — not to delete around it.

> **A CERTIFIED REFUSAL IS NOT A RESIDUAL STILL FIRING. Do not hard-stop on
> one.** Added 2026-08-16. The five rows [[RT-RECURSOR-TRANSPORT]] leaves behind
> are shapes the emitter **refuses**; a residual firing is a program that
> **selects** the `RecursiveDescent` lane. Those are different measurements, and
> `D1`'s enumeration is over the second. The five were dispositioned as internal
> compiler invariants at PR #2440 and their disposition is `D8`'s subject, not
> this gate's. **If `D1` does surface a program that selects the authority, the
> hard stop applies in full** — that is a surviving class and it is a node.

> ### THAT CARVE-OUT FIRED ON 2026-08-16 AND WAS RULED AGAINST ON SUBSTANCE.
> ### IT IS RE-AIMED AT DELETION, NOT RETIRED. Architect `evt_98zg6sbqh7ej`.
>
> **`D1` surfaced 28 selections. The clause above is literal, post-dates both
> dispositions, and fired.** It does not block `D2c`. Three legs, each
> independently sufficient:
>
> **1. The stop already fired on THIS population and the remedy was already
> performed — twice.** [[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]] opens:
> *"`RT-DESCENT-RETIRE`'s `D1` found this variant live only in fixtures.
> Production selects `BodyEmissionAuthority::RecursiveDescent` 31 times across 20
> tests, and 27 of those selections carry `LexicalCallArgumentRecursor`."*
> **That node exists BECAUSE this stop fired.** The campaign named the surviving
> class and handed it back as node work; both nodes were built and both merged.
> And that node's ratified disposition says the variant *"is re-described as
> fixture-only with the retained lane and **retires when `RT-DESCENT-RETIRE`
> removes that lane.**"* ⇒ **The 27 surviving lexical selections are the ratified
> expectation of a merged node, not a discovery. Firing the stop again is double
> jeopardy on a discharged stop.**
>
> **2. Gating on zero selections permits `D2c` only when it is VACUOUS.** With
> zero, the reroute is a no-op, CI is trivially green, and nothing is learned.
> `D2c` carries evidentiary weight in exact proportion to the live selection
> count. **The defect is not that the two requirements contradict** — in a
> zero-selection world `D1` passes and `D2c` is merely unnecessary. **It is that
> the stop routes the NON-zero world to "halt" when the campaign has already
> built the instrument for exactly that world.**
>
> **3. `D2c` cannot realize the harm the stop exists to prevent.** The stop
> guards against deleting a lane something still uses. **`D2c` deletes nothing**,
> is one revertible change, and surfaces any dependent program as a concrete
> failing CI row. Its premise is *"you are about to delete."* `D2c` is not
> deleting.
>
> **Leg 3 holds even if the 28 were wholly unattributed** — an unexplained
> population would make `D2c` MORE urgent, not less, since it is the only
> instrument that names such programs concretely rather than by argument. ⇒
> **The attribution recorded under `D1` is corroboration and must not be
> load-bearing.**
>
> ⇒ **The stop's live form is `D2c`'s condition 1: a red from a program OUTSIDE
> `D2b`'s pinned set.** A live selection is not a surviving class **while nothing
> is being deleted**; it becomes one the moment deletion is on the table without
> the differential — which is why `D3`-`D8` stay gated on `D2c` green.
