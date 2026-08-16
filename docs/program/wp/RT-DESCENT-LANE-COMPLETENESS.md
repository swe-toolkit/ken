# RT-DESCENT-LANE-COMPLETENESS — is the functionized lane a complete replacement for `RecursiveDescent`, or has it been carrying only the ported subset?

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

**Run `D3` FIRST.** Its outcome can change who owns this node, and the
Architect's ruling asked for that check before the cut. It is sequenced first
rather than treated as a precondition so nothing stalls; see section 7.

### D1 — ARCHITECT, soundness. FOUR verdicts, not one.

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

**AC-5.** On a `D3` hit, the increment **stops and hands back** with no `D2`
work performed after the hit. The erratum is not adjudicated inside this node.

**AC-6.** `D4` names the callers of both helpers, the expressions they pass, and
whether any refuses — and states explicitly whether any landed assertion rests
on an aborted compilation.

**AC-7.** `D2c` is still at `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`,
unpublished and unrebased, when this node closes. No production code under
`crates/` is modified by this node.

**AC-8.** The five programs in section 3a are untouched.

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
