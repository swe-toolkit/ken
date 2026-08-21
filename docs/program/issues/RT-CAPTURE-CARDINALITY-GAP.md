---
id: RT-CAPTURE-CARDINALITY-GAP
title: "the recursive-position witnesses stay word-only because 1-3 captures per closure carry NO planner claim of any class -- the planner's capture projection (<=2 claims) is smaller than the closure's declared capture set (3-5). Three consecutive results (RT-BRANCH partition, RT-CAPTURE-SUPPLY provenance, RT-CONTSRC-ENTRY-FRAME-WIDEN widening) each refined the provenance of the claims the planner PRODUCES and each greened zero; this node attacks the projection gap itself. D0-first: measure the CAUSE of each unclaimed capture -- planner under-projection (grow the projection) vs elaborator over-capture (prune the declared set) -- two hypotheses with OPPOSITE fixes"
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-CONTSRC-ENTRY-FRAME-WIDEN]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Steward scope-consequence call on RT-CONTSRC-ENTRY-FRAME-WIDEN D0 (087849760, Architect-approved evt_37mt6t65vvw39, Steward disposition evt_2emh7rzd9zb1h). That D0 measured the entry-frame widening route sound and OPEN yet witness-inert (0 of 16 greened): the decisive arithmetic is at-most-2 planner claims against 3-5 source captures, claim-count reaching capture-count in 0 of 25 closures. The Architect's read (and the Steward's): the three necessary-but-not-sufficient results all attacked the provenance/resolution of the claims the planner PRODUCES, while the witnesses are blocked by 1-3 captures per closure that produce no planner claim of any class -- a structurally different question. Node scoping is the Steward's per COORDINATION section 3; the feasibility/cause fork is the D0's to measure, routed to the Architect."
---

# WHY THIS NODE EXISTS

Three consecutive results on this chain came apart at the same seam -- each was
**necessary but not sufficient**, and each greened zero witnesses:

- [[RT-BRANCH-LOCAL-DECLARED-CALLABLE]] fixed the branch-local partition (a Ret
  arm no longer over-vetoes a Vis case's callable authority).
- [[RT-CAPTURE-SUPPLY-DECLARED-INPUTS]] measured the provenance of the claims the
  planner produces (25 of 30 are `ProducerLocal`, refused by
  `resolve_context_capture_claim`).
- [[RT-CONTSRC-ENTRY-FRAME-WIDEN]] measured that widening every one of those 25
  producer-local claims into the generated context's entry-source enumeration is
  sound and open -- and still greens zero.

The shared predicate across all three: **they attack the provenance and resolution
of the claims the planner PRODUCES.** But the RT-CONTSRC-ENTRY-FRAME-WIDEN D0
surfaced the actual blocker as arithmetic: the owning context's capture plan holds
**at most 2** claims against a source capture set of **3 to 5**, and claim-count
reaches capture-count in **0 of 25** closures. A witness needs *every* capture
resolvable. So even resolving every claim the planner produces leaves **1-3
captures per closure with no planner claim of any class at all.** That is the
cardinality gap, and it is the real remaining gate for [[NATIVE-HANDLE-CARRIER]]
and [[PX8-F-CAP-41]] for this population -- which is why their `blocks` edge moves
here from RT-CONTSRC-ENTRY-FRAME-WIDEN on its closure.

# THE OPEN FORK -- the D0 measures the CAUSE, the Architect rules it

Why is the planner's capture projection smaller than the closure's declared
capture set? Two hypotheses, and **their fixes point in opposite directions**, so
the D0 must discriminate them before any implementation is scoped:

- **H1 -- planner under-projection.** The declared capture set is correct; the
  planner *should* produce a claim for each of the 3-5 captures but emits only
  <=2. The missing 1-3 captures are genuine values the projection drops. Fix
  direction: **GROW the projection** so the planner mints claims for them -- and
  if those added claims are `ProducerLocal`, they then need the parked
  [[RT-CONTSRC-ENTRY-FRAME-WIDEN]] widening (which this outcome would revive).

- **H2 -- elaborator over-capture.** The declared capture set is INFLATED. The
  elaborator emits, for every expression-position lambda,
  `LexicalClosure { captures: (0..runtime_depth).map(Var) }` -- the whole
  enclosing runtime environment, with no free-variable analysis (erasure.rs:2210;
  the Architect-routed finding, evt_59ra3yk8j1tbq; measured captures=5 on a
  continuation that references nothing). Under H2 the missing 1-3 captures are
  **spurious slots that reference nothing in the body**, and the planner's <=2 is
  already correct. Fix direction: **SHRINK the declared set** via elaborator
  free-variable pruning in closure conversion -- the witnesses then green because
  every remaining (genuine) capture is already resolvable.

The two are not exclusive per-closure -- some unclaimed captures may be H1, others
H2 -- so the D0 classifies **each unclaimed capture**, not each closure.

# THE SEPARATE POPULATION -- 6 of 16 witnesses are empty

RT-CONTSRC-ENTRY-FRAME-WIDEN's D0 recorded that **6 of the 16 witnesses have an
empty population** -- no generated context owns their closure body, so no
projection-gap question even arises for them; a capture-supply route does not
reach them in principle. The D0 must keep these as their **own disposition**
(reached-not-at-all), distinct from the H1/H2 classification of the 10 witnesses
that do have a population. If the empty-population 6 need a different mechanism
entirely, that is a separate successor, not this node's burden to close.

# DELIVERABLES

**`D0` -- the cause measurement, no implementation.** For each of the 10
populated witnesses, for each capture in the closure's declared set that carries
NO planner claim: classify the cause as **H1** (a value the planner should have
projected -- name what in the projection drops it) or **H2** (a spurious
over-captured slot that references nothing in the body -- confirm the body makes
no use of it). Record the 6 empty-population witnesses as their own disposition.
The measurement reads only planner-owned + retained-source + elaborator-output
state; an audit shows zero capture-value reads from the carried word (the
inherited inviolable line). Route the closed D0 to the Architect.

The D0's disposition drives the next scoping call, which is the Steward's:
- **Mostly/all H2** -> the fix is elaborator free-variable pruning; frame that as
  the closing deliverable, and the parked widening D1 likely stays parked.
- **Mostly/all H1** -> the fix grows the planner projection; the added
  producer-local claims revive the RT-CONTSRC-ENTRY-FRAME-WIDEN widening D1, and
  the two compose into the closing deliverable.
- **Mixed** -> both, sequenced by which subset each witness needs.

# `D0` CAUSE MEASUREMENT at base `ebeedc213`

**Result: the gap is H1 everywhere. H2 does not occur in this population at
all** — across the 10 populated witnesses, **22 closure observations, 0 captures
unreferenced**. Every capture the elaborator declared is genuinely used by the
body, so the declared set is not inflated *here* and pruning it would green
nothing.

**This contradicts the leading hypothesis, and that is the point of measuring
it.** The routed elaborator finding is real as a *mechanism* — `erasure.rs:2210`
does emit `(0..runtime_depth).map(Var)` with no free-variable analysis, and a
purpose-built continuation that references nothing still measured `captures = 5`.
But a mechanism that *can* over-capture did not over-capture on these witnesses.
`AC-0` demanded H2 be shown, not inferred from the mechanism; shown, it is absent.

## The discriminator, and why it is decidable

A capture is **H2** exactly when the body references it nowhere — a static
property of the closure body, independent of what the planner did. So the
measurement is a free-variable scan: with the body's environment laid out
`[params ++ captures]`, capture `i` is referenced iff the body contains
`Var(params.len() + i + d)` at binder depth `d`.

**Both halves of that were grounded rather than assumed**, because getting either
backwards inverts the classification:

- **The layout** is read off the application site, not off a field name: the
  callee builds `call_env` as the lowered arguments, then
  `extend_captures(...)`, then the producer environment — so `Var(0)` is
  parameter 0 and `Var(params.len() + i)` is capture `i`.
- **The binder arities** are mirrored from
  `ken-elaborator/src/erasure.rs::shift_runtime_vars`, which is the codebase's
  own authority on them, and copied exhaustively with no catch-all arm:
  `Let` binds 1; a `Match` case binds `case.binders`; a `ComputationalMatch` case
  binds `argument_binders + recursive_positions.len()` (the IH slots count, and
  omitting them would have shifted every index); `Closure` binds `params`;
  `LexicalClosure` binds `params + captures` in its body while its capture
  expressions sit in the outer scope. **A missed binder under-counts references
  and manufactures a false H2**, which is the miscompile-risk direction, so the
  arities are copied rather than derived.

**The scanner was validated against a known-answer oracle before any verdict was
read off it.** `RT-BRANCH-LOCAL-DECLARED-CALLABLE`'s D1 fixture holds a
`LexicalClosure { captures: [Var(0)], params: ["arg0"], body: Value(Int 7) }` —
a literal body that provably references nothing. The scanner reports
`body_free = {}`, `referenced = []`, `unreferenced = [0]`. So the instrument does
return "unreferenced" when that is the truth, and the uniform "all referenced" on
the real witnesses is a reading rather than a stuck instrument.

## Per-witness measurement

All ten populated witnesses show the same shape: `params = 1`, a contiguous
`body_free` covering the whole scope, and an empty unreferenced set.

| Witness | Captures | `body_free` | Referenced | Unreferenced | Class |
| --- | --- | --- | --- | --- | --- |
| `px7f :: linked_public_right_denial_preserves_exact_masks` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `px7f :: linked_public_second_release_is_closed_and_the_handle_closes_once` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `px7m :: dynamic_ok_payload_selects_a_multistep_tree_across_real_executors` | 4 | `{0..4}` | 0-3 | none | **H1** |
| `px7m :: dynamic_err_payload_selects_a_multistep_tree_across_real_executors` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `px8ta :: px8ds_real_same_depth_path_rejects_flat_order_and_runs_exact_edges` | 4 | `{0..4}` | 0-3 | none | **H1** |
| `rt_escape :: escaped_buffer_used_by_fanning_host_op_matches_interpreter` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `rt_parity :: fs_read_at_malformed_offset_narrows_to_invalid_offset` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `rt_parity :: fs_read_at_malformed_offset_without_read_right_narrows_to_invalid_offset` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `rt_parity :: fs_read_at_malformed_window_narrows_to_invalid_bounds` | 5 | `{0..5}` | 0-4 | none | **H1** |
| `rt_parity :: fs_write_at_malformed_offset_without_write_right_narrows_to_invalid_offset` | 5 | `{0..5}` | 0-4 | none | **H1** |

**Totals: 22 closure observations, 0 unreferenced captures, 0 H2, every
unclaimed capture H1.**

## H1's required half — the exact projection step that drops them

`AC-0` requires naming what drops a referenced value. It is one line,
`continuations.rs:6075`, where a generated context is interned:

```rust
captures: enclosing_unit.key.continuation_inputs.clone(),
```

**A generated context's capture set is CLONED from the enclosing
specialization's `continuation_inputs`.** It is not derived from — and never
consults — the worker closure's own declared capture set. The two are
independent populations: `continuation_inputs` is the specialization key's
continuation-input projection, sitting beside `recursive_positions`, `worker`
and `ordinary_parameters`, while the closure's captures are the elaborator's
whole-environment list.

⇒ **The `<=2` versus `3-5` mismatch is not a projection that loses entries; it
is a projection of a different set.** Nothing in this step could make the counts
agree except coincidence, which is why the arithmetic held uniformly across all
25 closures in the predecessor's D0. That is the step to grow, and it is the
whole H1 finding.

## The separate population — 6 of 16, reached-not-at-all

Carried forward unchanged from [[RT-CONTSRC-ENTRY-FRAME-WIDEN]]'s D0: `px7l`
(both rows), `px8ta :: public_one_level_bracket_finishes_and_releases`, `px8x`,
`rt_escape :: escape_resource_plus_plain_matches_interpreter`, and
`rt_parity :: buffer_allocate_malformed_capacity_narrows_to_invalid_bounds` have
**no generated context owning the closure body**, so no projection-gap question
arises for them. They are neither H1 nor H2 — they are **reached-not-at-all**,
and closing the projection gap does not reach them. If they need a mechanism at
all it is a separate successor, as the frame states.

## What this decides

Per the frame's own routing, an all-H1 disposition means **the fix grows the
planner projection**, and the added claims would be `ProducerLocal` — which
**revives the parked [[RT-CONTSRC-ENTRY-FRAME-WIDEN]] widening `D1`**, since
those claims then need seating in the generated context's entry run. The two
compose into the closing deliverable. The elaborator free-variable pruning route
is **not** indicated for this population: it is a real latent defect and a fair
cleanup, but it would green none of these witnesses, and framing it as the
closing deliverable would be the fourth necessary-but-not-sufficient result on
this chain.

**The scoping call is the Steward's, not this D0's.** What the measurement
settles is only which of the two opposite fix directions the evidence supports.

## `AC-0` — the invariant audit

The measurement reads only retained-source and planner state: the closure's own
declared capture list and body (retained source), and the per-closure free-variable
scan. It was taken with a temporary, env-gated (`KEN_D0GAP`), **uncommitted**
probe whose 96 added lines were audited by grep for `Carried`, `carrier_field`,
`emit_carrier` and `word` and matched **nothing** — **zero capture-value reads
from the carried word**, the inherited inviolable line. `core.rs` was then
restored and verified **byte-identical by blob hash** (`c0e7ee42`) with a clean
`git status`. This candidate carries only this document.

One reproduction note worth keeping, because it cost a wasted run on the
predecessor: production short-circuits before the capture-bearing arm on most
witnesses, so the scan needs a read-only walk over every arm to reach the site.
A probe on the production path alone returns silence, which reads as "no captures
found" rather than "not reached".

# ACCEPTANCE CRITERIA

**`AC-0` (D0)** -- every unclaimed capture across the 10 populated witnesses is
classified H1 vs H2 on evidence (H2 requires showing the body references the slot
nowhere; H1 requires naming the projection step that drops a referenced value).
The 6 empty-population witnesses are recorded as reached-not-at-all. The
measurement reads only planner/retained-source/elaborator-output state; an audit
shows zero carried-word capture-value reads.

**`AC-1` (the fix, conditioned on D0)** -- deliverables and ACs are the D0's to
fix once the cause is measured. If H2: an elaborator free-variable prune drops a
spurious capture and a seam fixture greens where over-capture previously inflated
the set, with a discriminating control that a genuinely-referenced capture is
NEVER pruned (a pruned-live-capture must fail loudly, not silently miscompile).
If H1: the projection mints a claim for a previously-unclaimed referenced capture,
composing with the parked widening.

**`AC-2`** -- the discriminating control appropriate to the measured cause (H2: a
referenced capture stays; H1: an unreferenced slot is not spuriously claimed).

**`AC-3`** -- conformance for the greened witness case if D0 finds a non-empty
fixable subset.

**`AC-4`** -- no-regression in CI.

# BANNED SCOPE

- **Implementing before D0.** The H1-vs-H2 fork has opposite fixes; measuring the
  cause first is the whole point.
- **Reading any capture value from the carried word** -- inherited from
  RT-CAPTURE-SUPPLY / RT-BRANCH; still barred.
- **Pruning a genuinely-referenced capture (H2 path).** Free-variable pruning must
  drop only slots the body provably never references; a dropped live capture is a
  miscompile, not a cleanup. The control is fail-loud, never silent.
- **Relaxing `verify_entry_frame`'s membership guard** -- inherited from
  RT-CONTSRC-ENTRY-FRAME-WIDEN; the widening, if revived, still extends
  membership and never opens the guard.

# SEQUENCING

`depends_on: [RT-CONTSRC-ENTRY-FRAME-WIDEN]` (closed at D0). Released now as the
runtime ring's next node. `gate: none` -- runtime lowering / elaborator closure
conversion, no TCB or trusted-reduction change; the cause fork is a design
question the Architect rules on the D0, not an operator gate. Tier **T1**
(correctness-sensitive: the H2 fix drops captures from generated closures, where a
wrong prune is a miscompile; the H1 fix mints new authority claims). Review:
**Architect** (author is not reviewer), who reviews the D0 and any conditioned
fix. This is a D0-first measurement node -- the runtime implementer measures the
cause; the Steward cuts the fix node from the disposition.
