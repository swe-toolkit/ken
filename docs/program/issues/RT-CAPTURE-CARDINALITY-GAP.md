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
