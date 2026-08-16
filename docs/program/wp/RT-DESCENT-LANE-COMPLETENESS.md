# RT-DESCENT-LANE-COMPLETENESS

**Is the functionized lane a complete replacement for `RecursiveDescent`, or has
it been carrying only the ported subset?**

Frame. Steward-authored 2026-08-16, on the Architect's ruling
`evt_7qtgrtwv76vke`. Successor to [[RT-DESCENT-RETIRE]], which is `active` and
**blocked** on the outcome of this node.

**Treat every anchor here as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around
it.** Every line number below is an anchor to re-find at the named SHA, never a
value to check.

## 1. Objective

`D2c` of [[RT-DESCENT-RETIRE]] rerouted `select_body_emission_authority` to
never return `BodyEmissionAuthority::RecursiveDescent`, leaving every lane,
enum, variant and emission path in place. It reded, and the reds are **not** one
missing case.

**Nine programs that `RecursiveDescent` compiles are refused by the
functionized lane, across four independent constructs.**

⇒ **The question this node answers is not "add the missing case."** It is
whether the functionized-units lane is a complete replacement for
`RecursiveDescent`, or whether it has been carrying only the ported subset all
along. **Frame every deliverable against that question**; scoped as a single
port it will be answered and come back.

## 2. Fixed inputs

All measured. Cite them; do not re-derive them.

| input | value |
|---|---|
| **base SHA** | `c98f72ba8489741b2ff31c4da7a1922f6926d0bf` |
| **`D2c` candidate** | `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`, on `wp/RT-DESCENT-RETIRE` |
| **`D2c` disposition** | **UNPUBLISHED and untouched. DO NOT REBASE IT** — the base is what the pin is measured against |
| **`D2c` result** | 926 passed / 17 failed / 4 ignored, from 943 / 0 / 4 |
| **the 17** | 14 inside `D2b`'s frozen program-arrival set; 3 are direct callers of the rewritten function (`D2b` category B) |
| **`D3`-`D8` of the predecessor** | gated. No `D6` re-home is lawful while this node is open |
| **authorized Runtime implementation** | **none.** This node is measurement and adjudication only |

**The single file everything below lives in:**
`crates/ken-runtime/src/cranelift_backend/lowering/core/tests/control.rs`.

### 2a. The artifact hypothesis is CLOSED. Do not re-open it.

The one way this finding could have been about `D2c`'s edit rather than about
the lane was that the always-`FunctionizedUnits` rewrite might not be
behaviour-equivalent to the pre-existing exclusion mechanism.

**It was checked and excluded.** At untouched base `c98f72ba8`, asserting that
the pre-existing excluded `FunctionizedUnits` result is `Ok` inside
`recursive_descent_recursors_compile_without_a_boundary_crossing` fails at row
4 depth 2 with the **identical** `UnsupportedLowering` / `StaticWorkerBinding`
— same constructor origin 36, same static worker field 0, same origin 35, same
recognition 2 (runtime-leader, `evt_6bvnv6t4teech`; disposable patch removed,
tree clean).

⇒ **Two independent instruments, one of which does not involve `D2c`'s edit at
all. The finding is about the lane.**

**Corollary, and it is the uncomfortable half: the evidence predates `D2c`
entirely.** The exclusion mechanism was a complete differential instrument the
whole time. The sentinel ran the functionized route, held the answer, and
discarded it. That is what `D4` below exists to bound.

## 3. The nine refusing programs, by construct

Classified by the runtime ring (`evt_6bvnv6t4teech`), arithmetic verified by the
Architect against the 14. **Classification only — no adjudication was performed
and none was authorized.** Each name below resolves to a `fn` at base
`c98f72ba8`; find it by name, not by line.

| construct | n | tests |
|---|---|---|
| `ComputationalMatch` / in-flight non-transferable activation | 4 | `d0_r3_fusion_gate_resolves_zero_for_the_seed_and_one_for_the_checked_twin`, `d2f_0_the_applied_root_production_path_gate`, `d2f_a_production_compile_builds_the_fusion_identity_plane`, `px8j_selected_scope_partitions_differ_across_the_real_return_hole` |
| `StaticWorkerBinding` | 2 | `px8j_one_two_three_scope_segments_reach_selection_hole_and_unwind`, `recursive_descent_recursors_compile_without_a_boundary_crossing` (the sentinel) |
| Backend `Module` / missing recursive-position-1 worker projection | 2 | `d2e_ac9_layout_agrees_with_the_prefix_production_assembled`, `px8j_siblings_share_an_origin_and_nested_ih_gets_a_child_origin` |
| Backend `PlannerInvariant` / missing affine checked-root authority | 1 | `px8j_owned_scope_deletion_fails_closed_before_another_frame_is_emitted` |

**Four separate representability gaps is a pattern, not an omission.** That is
the whole reason this node is not a port node.

### 3a. The five without a refusing construct are NOT part of this node

`d0_row2_functionized_lane_never_reaches_the_source_machine_mint`, `d2k_0_...`,
`msd_d2a_the_retention_and_routing_guards_have_a_concrete_difference`,
`px8j_all_three_producer_paths_reach_real_consumers`, and
`row2_functionized_lane_installs_and_consumes_the_recursive_ih` assert the
**retiring lane's own control, lifecycle or route state**. No program refuses.
`msd_d2a` is correctly among them: it pins that the selector returns
`RecursiveDescent`, which `D2c` rewrites by design.

**They are `D6` rewrites in the predecessor and they stay gated behind the
nine. None may be touched while this node is open.**

## 4. Deliverables

> ### ALL FIVE DELIVERABLES ARE NOW DELIVERED. NOTHING IS OPEN ON THE RING.
>
> **`D2`, `D3`, `D4` pre-frame; `D1` by the Architect (`evt_5cxzxp4b6q31v`,
> four verdicts, recorded in the node); `D5` by the runtime ring
> (`evt_6tveatdhcz72y`). The heading here previously read *"`D1` IS THE ONLY
> OPEN ONE"* and was stale from the moment `D1` landed.**
>
> **runtime-leader `evt_2fmjv69z5bg2g`, measured at exact
> `3c9b8bbd5fae09859d6e330f8ac0a17b40fe1f68`** — note that is a **different SHA
> from this frame's base `c98f72ba8`**; no candidate or instrumentation remains
> and `D2c` is untouched. Results are in section 4a. **The ring awaits explicit
> release and no Runtime implementation is authorized.**
>
> ### AND MY `D3`-FIRST SEQUENCING WAS WRONG. Corrected here.
>
> I sequenced `D3` first behind a hard stop, on the assumption that an
> overlap with a merged node's claimed population is decidable **independently
> of `D1`**. It is not, and the evidence shows it twice over:
>
> - **The hit is universal.** All nine overlap an explicitly claimed merged-node
>   population. A hard stop on *any* hit would have fired on everything and
>   stalled the node.
> - **Whether an overlap is an ERRATUM is exactly `D1`'s verdict.** Those
>   records' dispositions are *source-unreachable compiler asserts/invariants*
>   or *a preserved refusal*. A preserved-refusal disposition is **accurate** if
>   that construct's refusal is correct semantics, and **false** if it is a
>   missing port. Same fact, opposite readings, decided by `D1`.
>
> ⇒ **`D3` is not an independent gate. It is a CONSEQUENCE of `D1`, per
> construct.** `AC-5`'s hard stop is withdrawn accordingly and `AC-4` is
> discharged as input. The ring was right to supply the input and decline the
> ownership call.

### 4a. What `D2`, `D3` and `D4` returned

**All three at `3c9b8bbd5`, runtime-leader `evt_2fmjv69z5bg2g`.**

**`D2` — every one of the nine maps BYTE-FOR-BYTE to a hash-tagged lexical
fixture rendering, and all are fixture-only** under merged
[[RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT]]:

| rendering | hash | tests |
|---|---|---|
| row 5 before-hole | `25c3d81c8054e552` | 4 |
| row 4 depth 1 | `a26749baed91331f` | 1 |
| row 4 depth 2 | `de31e8ed184a5754` | 1 (the sentinel) |
| row 3 two-sibling | `23fad2ab9d295856` | 2 |
| row 1 owned-scope | `7433055269044ce8` | 1 |

⇒ **Zero source-reachable programs**, by that node's definition-admission
argument. **This is RECORDED-GAP INPUT, not a soundness verdict** — the ring
said so explicitly and that is the correct boundary. The mapping caveat this
frame raised is **discharged**: it was established for all nine, not inherited
from the sentinel.

**`D3` — every one of the nine overlaps an explicitly claimed merged-node
population**: four R3 fusion-emitter row-5 claims, then lexical-recursors row 4
depth 1, row 4 depth 2, row 3, and row 1. **Those records do not claim complete
`FunctionizedUnits` emission**; their present dispositions are
source-unreachable compiler asserts/invariants, or a preserved refusal.
**Erratum input established, ownership undecided** — see the correction above
for why that is the right stopping point.

**`D4` — both helpers are INVALID as completed-compile evidence, and it is not
marginal.** `owner` and `multiplicity` each run five expressions and **every
functionized compile aborts** — row 1 `PlannerInvariant`, rows 4 and 5
`StaticWorkerBinding` — **while their trace-event assertions stay green. Zero
completed functionized runs.** Their evidence is partial trace harvested from
aborted compilations.

### 4b. The Architect ADJUDICATED all three. `evt_3bkkjpps1bcpe`.

**`D2` = RECORDED GAP, not blocked — verified, not accepted.** All five hashes
check against the merged node's own table: `7433055269044ce8`,
`23fad2ab9d295856`, `a26749baed91331f`, `de31e8ed184a5754`, `25c3d81c8054e552`.
Nine tests, five renderings, all within the twelve, **zero source-reachable.**
⇒ **The retirement loses no user-facing capability. It is no longer
capability-blocked.** The finding settles **cost, not correctness**.

**`D3` = NEGATIVE, independently corroborated.** The Architect spot-checked the
node carrying the largest exposure, `RT-LEXICAL-R3-FUSION-EMITTER` (four of the
nine), *"because this campaign has form"* — two merged titles in this arc had
already asserted bars their bodies no longer supported. **It records refusals as
refusals**: *"Preserve it as a refusal; never relabel it plane `0`"*, and
elsewhere that something **"still refuses"** naming `ComputationalMatch` /
in-flight — **the same construct as the largest class in the inventory.**

⇒ **No merged completeness claim is falsified. No erratum. Ownership of this
node does not move.**

**A `D1` input, explicitly not a verdict:** at least the `ComputationalMatch`
construct was **already a documented, ratified refusal in a merged node**.
`D2c` **rediscovered** it; it did not surface it. That is evidence toward the
correct-semantics arm **for that construct only** — the other three are not
shortcut from it.

### 4c. `D1` PRECEDES THE RECORD, not the deletion. And the record needs PINS.

After `D3`-`D8` the lane is gone and these tests are retired or rewritten.
**The record is all that remains of four known representability facts.**

**And *"a gap the lane must someday close"* is the WRONG record if the refusal
is correct semantics** — it would misdirect every future reader into porting
something that should not be representable. Hence the ordering, and hence
`AC-9`.

> **This is the `nc22` reasoning applied FORWARD: do not retire a fact and its
> only detector in the same commit.** That was this node's own release gate on
> the predecessor two hours earlier, and **it binds the exit as much as the
> entry.**

### D1 — ARCHITECT, soundness. FOUR verdicts, not one. DELIVERED.

**Delivered `evt_5cxzxp4b6q31v`: two CORRECT SEMANTICS, two MISSING PORT. The
verdict table is in the node. Nothing below is outstanding.**

**For each of the four constructs in section 3: is the functionized lane's
refusal CORRECT SEMANTICS, or a MISSING PORT?**

| verdict | consequence for that construct |
|---|---|
| **correct semantics** | `RecursiveDescent` compiled a shape with no runtime denotation. Retirement **removes a latent representability hole**; nothing is owed but the gap is recorded |
| **missing port** | The lane owes the case, and the retirement waits on it |

**They may not answer alike.** A principled representability refusal and an
unported case can sit side by side across four constructs. **One verdict per
construct.**

**This is a soundness question and it routes to the Architect. The ring does not
decide it as engineering.** The error text settles neither reading — see the two
foreclosed shortcuts in section 6.

### D2 — RUNTIME RING, measurement. This decides BLOCKED versus RECORDED GAP.

**Establish the source-reachability of each of the nine refusing programs.**

**Do not inherit `0/12`.** That figure was measured over twelve **renderings**;
the nine are **test names**, and the mapping between the two is established for
**the sentinel only** (rendering 5, hash `de31e8ed184a5754`). Verify each
mapping or report it as unestablished. **Do not extrapolate from the one row
that is known.**

| outcome | consequence |
|---|---|
| **every refusing program fixture-only** | The retirement incurs **four recorded representability gaps** rather than a capability loss, and may proceed once they are written where a future kernel-admission change will meet them |
| **any one source-reachable** | **Hard-blocked**, and the port is owed |

**The methodology already exists** — `RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT`
executed exactly this. Reuse it. This is known work, not new invention.

### D3 — RUNTIME RING, erratum check. Run this FIRST.

`d0_r3`, `d0_row2`, `d2e_ac9`, `d2f_0` and `d2f_a` are **deliverable-keyed
names: they belong to port nodes.**

**Determine whether any of the nine refusing programs falls inside a MERGED port
node's claimed population.** The merged ports are the predecessor's
`depends_on`: `RT-DECL-CLOSURE-PORT`, `RT-SEED-CALL-PORT`,
`RT-PRODUCER-MATCH-PORT`, `RT-RECURSOR-TRANSPORT`, `RT-FNUNIT-RESULT-TOKEN`,
`RT-LEXICAL-RECURSOR-CONSUMERS`, `RT-CLOSURE-CROSSING-ELIMINATE`,
`RT-LEXICAL-CALL-ARG-WITNESS-OR-PORT`.

**If any does, that node's completeness claim is FALSE on `main`.** That is an
erratum, not a retirement question, and it changes who owns the successor.

**HARD STOP: on any hit, stop and hand back before `D2` continues.** Do not
adjudicate the erratum and do not fold it into this node.

### D4 — RUNTIME RING, bounded sweep of the two trace helpers.

At base there are **18** `set_selector_variant_exclusion(Some(...))` sites in
`control.rs` and exactly **three** that discard the compile result (Architect's
census, `evt_7qtgrtwv76vke`). Find each by the discard, not by line:

| site | what it is |
|---|---|
| the sentinel | `let (_excluded_result, _trace) = px8j_capture_source_trace(` — now known to have been hiding a refusal |
| helper `owner(...)` | `let (_result, _trace) = px8j_capture_source_trace(expression, false, symbol);` then `d2k_owner_trace_take()` |
| helper `multiplicity(...)` | the same two lines, followed by a `BTreeMap<String, usize>` of descents |

**Both helpers run the functionized compile purely to collect trace events and
never confirm it succeeded.** ⇒ For any expression that refuses, they harvest
events from a **partially-completed compile**, and every assertion built on them
is a claim about an aborted compilation.

**Determine which expressions their callers actually pass, and whether any
refuses.** Bounded, and it must be run: **this is the same defect shape that
concealed the present finding for the whole campaign.**

**Three of eighteen is narrow, not systemic.** The Architect censused it
precisely so nobody has to assume either way. Do not widen the sweep.

> #### THAT BOUND WAS A COUNT WITHOUT ITS PREDICATE
>
> **A bound nobody can re-take is not a bound. Stating the predicate.**
>
> **Adversary `evt_7ar8w31nr88wh`; every number below re-measured by the
> Steward.** *"Discards the compile result"* has **two distinct mechanisms**,
> and the count did not say which it used:
>
> | predicate | base `e2e15f8e1` | merged `8b78b48cd` |
> |---|---|---|
> | `set_selector_variant_exclusion(Some(...))` sites | 18 | 16 |
> | `let _ = rt_run*` — outcome discarded at the call | 2 | 2 |
> | helper that never returns an outcome (`owner`, `multiplicity`) + the sentinel | 3 | 1 (sentinel) |
>
> ⇒ **"Three of eighteen" counted the second mechanism. By the first there are
> two, both in the route-equality control, and the repair left them
> byte-unchanged** — the deleted helpers discarded their results *inside*
> themselves, so they never appeared under a `let _ =` census at all.
>
> **The claim is true on the metric that produced it and was not checkable as
> written.** The reason an independent census could not be taken is that the
> predicate was missing, not that nobody tried. **Record base, tip, count, AND
> predicate.**
>
> The surviving `let _ =` pair carries its own non-vacuity anchor and its own
> bound; both are recorded at
> [[RT-ROUTE-EQUALITY-PIN-AT-THE-BINDINGS]]. **That does not widen this
> node's sweep** — it is filed where the control lives.

### D5 — RUNTIME RING, measurement

**Does any REAL SOURCE program select `RecursiveDescent`?**

> **This deliverable replaces an earlier `D5` instruction that asked for an
> observation the real path cannot produce. That instruction was the Steward's
> and it was wrong; the ring was right to stop rather than substitute a
> reachable route.** History and the refutation are in the node.

**The fixed input, located in the tree by the Steward — not transcribed from
the ruling, and it is larger than the ruling states.**

**The route into native lowering is `ken native-build`**, and the chain is
unbroken: `main.rs:51` dispatches `native_build_file` (`:81`) into
`ken_cli::build_native_program` (`lib.rs:21`), into
`ken_elaborator::compiler_driver::compile_native_program_sources` (`:2524`)
which **takes real Ken source text**, into
`build_bound_process_starter_executable_artifact`
(`object_linker_packaging.rs:879`), into
`emit_bound_process_program_object_with_cranelift` (`:937`), which holds the
sole production call of `select_body_emission_authority`.

**`build_native_program` has 35 executable call expressions across 17 files
under `crates/ken-cli/tests/`**, plus child `ken native-build` routes in an
18th. Every one is **outside `-p ken-runtime --lib`**, the scope `D1`'s 805
arrivals and `D2c`'s 943/0/4 were measured over.

**Corrected from "18 test files and 36 call sites", which was a textual grep
reported as a call census** — 37 hits on 37 lines across 18 files, two of them
comments. **A textual census is not a call census.**

**Two measurements, in this order.**

1. **The census that was never taken.** Over the `ken-cli` native-build corpus,
   which `BodyEmissionAuthority` does each compile select? **This is the
   population the earlier census structurally excluded, and it is the one that
   actually reaches the selector.** Report the distribution, not a summary
   verdict.
2. **The two-recursive-position probe.** Compile a Ken source program declaring
   a **two-recursive-position constructor** through `build_native_program` and
   observe whether it reaches the no-worker guard in `units.rs` — the
   `backend_module` error reading *"the selected case has a recursive position
   {position} that the continuation specialization projects no worker for"*.

**Three bounds, all binding.**

- **`tree-traversal.ken` is in `NEEDS_COLLECTIONS`**, so the prelude must be
  prepended **exactly as `rosetta.rs` does** if you use it as the input.
- **`native-build` requires a checked `Program I main`.** A build failure is
  evidence **only if attributed to the tree shape rather than to the harness.
  A refusal that is not attributed is not a result.**
- **Readings only, no adjudication**, and **`D2c` `036e8ee91` is not rebased,
  not published, not applied to this branch.** If a reroute reading is wanted,
  take it in a disposable tree.

**What each outcome does to the node**, so nobody has to infer it:

| result of measurement 1 | consequence |
|---|---|
| **no native-build compile selects `RecursiveDescent`** | the fixture-only finding survives contact with the real-source population, and the recorded-gap disposition holds |
| **any native-build compile selects `RecursiveDescent`** | a real source program depends on the retiring lane. **`D2`'s recorded-gap disposition is reopened for that construct** and the port is owed before `D3` |

**`D1`'s four verdicts are not reopened by either outcome.** `D5` moves
reachability, never correctness.

## 5. Acceptance criteria

**AC-1.** `D1` records **four** verdicts, one per construct in section 3, each
naming which of the two answers it takes and why. A single node-wide verdict
does not discharge it.

**AC-2.** `D2` reports, per refusing program, its source-reachability **and**
whether its rendering mapping was established or is unestablished. A program
whose mapping is unestablished is reported as such, never as fixture-only.

**AC-3.** `D2`'s conclusion is stated as one of the two rows of its table, and
the node's blocked-versus-recorded-gap disposition follows from it mechanically.

**AC-4.** `D3` reports, for each of the nine, which merged port node's claimed
population it falls in or that it falls in none — with the claim quoted from
that node, not paraphrased.

**AC-5. WITHDRAWN 2026-08-16, and the withdrawal is a frame amendment I own.**
It required the increment to stop and hand back on any `D3` hit. The hit is
universal and the erratum verdict is downstream of `D1`, so the stop was
unsatisfiable and would have stalled the node. **Replaced by: the erratum is
not adjudicated inside this node, and its per-construct disposition follows
`D1` mechanically.**

**AC-6.** `D4` names the callers of both helpers, the expressions they pass, and
whether any refuses — and states explicitly whether any landed assertion rests
on an aborted compilation.

**AC-7.** `D2c` is still at `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`,
unpublished and unrebased, when this node closes. No production code under
`crates/` is modified by this node.

**AC-8.** The five programs in section 3a are untouched.

**AC-9. EACH CONSTRUCT LEAVES A PIN, NOT PROSE.** Architect `evt_3bkkjpps1bcpe`.
For each of the four, `D1`'s verdict lands as a checkable artifact before any
`D6` deletion touches the tests that currently carry the fact:

| verdict | the pin it owes |
|---|---|
| **correct semantics** | an **asserted-refusal pin** that reds if the behaviour changes |
| **missing port** | a **recorded obligation with a NAMED OWNER** |

**Prose in a retired test's place does not discharge this.** `D6` deletes the
last trace otherwise, and a wrong record is worse than none: *"a gap the lane
must someday close"* against a correct-semantics refusal misdirects every future
reader into porting what should not be representable.

**AC-10.** The helper-evidence defect is **NOT** repaired inside this node. It
is [[RT-TRACE-HELPER-ABORTED-COMPILE-EVIDENCE]], cut separately, and folding it
here would make a live `main` defect ride a gated node.

**AC-11. THE EXISTING REFUSAL PIN CANNOT BE THE SURVIVING PIN — RE-HOME IT.**
Architect `evt_5cxzxp4b6q31v`. The pin
`d2k_0_the_five_no_longer_reach_a_static_worker_value_read`
pins the exact refusal sentences for constructs 1 and 2 — **but it does so
through `set_selector_variant_exclusion(Some(RecursiveDescentResidual::…))`,
and `D3` of the retirement DELETES `RecursiveDescentResidual` itself.**

⇒ **The pin rides the mechanism the retirement removes.** `AC-9` is **not**
discharged by citing it. It must be **re-homed onto the surviving lane** so the
refusal is asserted without reference to the retiring enum.

> **This is the `nc22` reasoning a third time in one node: do not retire a fact
> and its only detector in the same commit.** It caught the capstone's release
> gate, it set the record requirement, and here it catches the record's own
> instrument. **Check what a pin is built ON, not just what it asserts.**

**CONFIRMED: constructs 1 and 2 keep TWO rows and TWO pins.** Architect
`evt_74f5ppk3tnh1q`, answering the Steward's question rather than letting it be
assumed. *"One law at two callable kinds"* explains **why** both refuse; it is
**not** a licence to cover both with one assertion.

**The arms are independently mutable** — `Lowered::Closure`/`DeclarationClosure`
and `Lowered::ComputationalRecursorClosure` are separate arms of the same
`match`. **A single pin exercising a closure-in-a-constructor-field would stay
green if the recursor-closure arm changed**, and vice versa. A pin's job is to
red on a behaviour change, and a merged pin cannot red for the arm it does not
exercise.

> **The general form, worth keeping beside the one above: a shared REASON never
> justifies shared COVERAGE.** That is a property of the category, not of either
> site.

**AC-12. Constructs 3 and 4 owe a RECORDED OBLIGATION with a named owner, and
the record must outlive `D6`.** Discharged by tracked nodes
[[RT-FNUNIT-MULTI-WORKER-CONTINUATION]] and
[[RT-FNUNIT-CHECKED-ROOT-AUTHORITY-ROUTING]], both owner `runtime`. **Prose in a
deleted test's place does not discharge this** — that is why they are nodes and
not paragraphs.

**Discharge state at `3c9b8bbd5` (runtime-leader `evt_2fmjv69z5bg2g`):** `AC-2`
**discharged** — per-program mapping established for all nine, none reported as
unestablished. `AC-4` **discharged as input** — per-program overlap reported
with each record's present disposition. `AC-6` **discharged** — both helpers'
callers, expressions and abort constructs named, with the explicit finding that
their assertions rest on aborted compilations. `AC-7` and `AC-8` **hold**: no
candidate or instrumentation remains, `D2c` untouched. `AC-1` and `AC-3` remain
open on `D1`.

**AC-13. `D5` REPORTS A DISTRIBUTION AND AN ATTRIBUTION, NOT A VERDICT.**

The census in `D5` measurement 1 reports, per `ken-cli` native-build call site,
**which `BodyEmissionAuthority` was selected** — the distribution over the
corpus, not a summary sentence. **A report naming only the conclusion
("none select `RecursiveDescent`") does not discharge this**; the whole
point is that the earlier census's population was invisible in its summary.

The probe in `D5` measurement 2 reports **either** that the guard was reached
**or** that it was not, **and in the failure case names which of the two causes
applies** — the tree shape, or the harness (`NEEDS_COLLECTIONS` prelude, or the
checked `Program I main` requirement). **An unattributed refusal does not
discharge this criterion**, and it is the criterion most likely to be
accidentally satisfied by a red that means nothing.

**Control:** `AC-13` fails if the report contains a refusal whose cause is not
named, or a census whose per-site results are not given.

**This AC does NOT gate `D1`.** `D5` moves reachability only; `AC-1` and `AC-3`
are unaffected by its outcome.

## 6. Two foreclosed shortcuts. Do not take either.

- **"Fixture-only, so it doesn't count."** `0/12` bounds the **blast radius**,
  not the **mechanism**. The `StaticWorkerBinding` refusal is stated generally
  — *"a constructor carrying an unconsumed static worker denotes a value
  containing the callable and has no runtime representation"* — and is about
  the lane, not one fixture. **The fixture is how it was found, not the extent
  of what was found.** This is exactly why `D2` exists as a measurement rather
  than an inherited number.
- **"`RecursiveDescent` compiled it, so port it."** **May be backwards.** A
  refusal that reads as principled can mean the surviving lane is **correctly**
  rejecting what the monolithic lane let stay implicit — in which case
  retirement removes a hole and nothing is owed. **Neither reading may be
  assumed from the error text.** That is `D1`'s whole content.

## 7. Judgment calls, recorded so they are not re-litigated

**Why this is a split rather than a framed repair.** The standing default is to
state a best-guess repair and have the ring build it. It does not apply here:
`D1`'s two answers differ by **zero code** versus **four unported constructs**,
which is the order-of-magnitude fork the split rule names. Writing a guess would
also require guessing a soundness verdict that is the Architect's to make.

**Why `D3` is sequenced first rather than gating the cut.** The Architect asked
for the merged-port check *before* the node is cut. The check is assigned to the
ring and could not have run before the cut existed to carry it. **The node is
therefore cut provisionally on ownership**: `D3` runs first, and a hit re-homes
the successor rather than being absorbed here.

**Why this is not folded into [[RT-DESCENT-RETIRE]].** The predecessor's frame
is the retirement itself; this is the precondition question, has a different
owner mix (`D1` is the Architect's), and folding it would make the predecessor
unbounded. The constraint is grounded in an Architect ruling cited by event id,
not in frame prose.

## 8. The staging is why this cost nothing

`D2c` is **one revertible commit that never landed.** Nothing to undo, no
evidence destroyed, and the blocker arrived as a **concrete failing program
instead of an argument.** The predecessor's `AC-7` — that the reroute and the
deletion are separate candidates — is what bought that, and it is the campaign's
strongest vindication of the two-step cut. Keep the same discipline here: this
node measures and adjudicates, and deletes nothing.
