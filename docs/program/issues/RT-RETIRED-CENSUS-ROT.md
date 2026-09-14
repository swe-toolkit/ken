---
id: RT-RETIRED-CENSUS-ROT
title: "Censuses retired by #[cfg(any())] are preserved as a readable record of a property, but cfg-stripping means nothing name-resolves them -- 3 of 3 are dead on revival, and one names a function deleted 19 days after its retirement"
status: merged
owner: runtime
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: "Adversary hunt evt_6npaybf8cznp8 (2026-08-18) on the RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING D3 landing b430d73e0. Steward-filed per COORDINATION section 2. The finding measured all three retired censuses by flipping cfg(any()) to #[test]; every figure below is the Adversary's, reproduced from its report."
---

> # RELEASED 2026-09-14 BY THE STEWARD. STARTABLE NOW, ON A CLEAN BASE.
>
> **The hold is lifted and the reason it was written no longer holds.** It said
> *"held behind the single runtime lane"* — the constraint was the runtime ring's
> attention, not anything about this work. That constraint is inverted today: the
> ABI-S6 / D5b stack is blocked behind a red CI at an unpushed tip, nothing there
> is dispatchable to an implementer, and the runtime ring is idle. **Re-sequencing
> is the Steward's call (`steward.md §3`); nothing in the node's content changed.**
>
> **Why this node and not another of the sixteen ungated `ready` runtime nodes:
> it is the one with a genuinely CLEAN BASE.** It is `size: S`,
> `depends_on: []`, and it touches
> `crates/ken-runtime/src/cranelift_backend/lowering/core/tests/` — clear of the
> 24-commit `wp/ABI-S6-d5b-file-backed` stack that every other runtime item is
> entangled with. It can be cut from `origin/main` and reviewed on its own.
> (The release post named `crates/ken-runtime/src/control.rs`; that path does not
> exist. Corrected here, and the scope is three test files rather than one.)
>
> **It remains true that this is not urgent** — no behaviour is wrong and nothing
> is unsound; the rot is in commentary rather than in compiled code. It is being
> released because it is *available*, not because it became important. **If
> anything in the D5b arc becomes dispatchable, that outranks this.**
>
> **`D0` is a component-design call and is the Architect's, not the
> implementer's.** Take it there before building either arm; the Architect is
> seated and light.
>
> **`D0` WAS RULED THE SAME DAY — ARM 2, DELETE THE THREE.** The paragraph above
> stands as the release record; the design question it points at is closed. **The
> `AC-2` guidance this block originally carried is withdrawn: `AC-2` was
> conditioned on arm 1 and is NOT APPLICABLE.** See the `D0` ruling block and the
> re-cut acceptance criteria below.

## The defect

`control.rs` retires censuses by putting `#[cfg(any())]` on them rather than
deleting them. **The stated reason to prefer that over deletion is that the
retired body stays readable as a record of the property it pinned.** That
presumes the body keeps describing the tree.

**It does not, and the mechanism that would tell you was removed by the same act
that retired it.** `cfg`-stripping precedes name resolution, so a retired body is
neither type-checked nor name-resolved. The tree drifts underneath it silently
and permanently.

## Measured: 3 of 3 retired censuses fail on revival

The Adversary flipped `#[cfg(any())]` to `#[test]` on every retired census in
`control.rs` and ran them under `-p ken-runtime --lib`. **All three fail.**

> # COORDINATE AMENDMENT, 2026-09-14 — THE THREE CENSUSES ARE NOT IN ONE FILE,
> # AND THAT REFUTES ARM 1's CONTROL AS THIS NODE SPECIFIES IT.
>
> **Re-derived by the runtime implementer at `3876edea0` before touching
> anything, and verified independently by the Steward.** Every figure in the
> tables below is the Adversary's from `b430d73e0` on 2026-08-18. **The findings
> hold. The coordinates do not** — `control.rs` has been split since, so the line
> numbers name lines that no longer exist, and the three censuses now sit in
> **three different files**:
>
>     crates/ken-runtime/src/cranelift_backend/lowering/core/tests/
>       mod.rs:1133               exactly_one_plan_origin_to_expression_lookup...
>       control.rs:4354           the_lower_expr_call_population_is_disposition...
>       host_call_carrier.rs:1821 d8_join_helpers_have_the_closed_typed_caller_...
>
> (Lines are the `#[cfg(any())]` attribute itself; the `fn` follows on the next
> line.) All three are present and still retired. **Nothing was lost — they
> moved.** Note the path: this is `.../lowering/core/tests/control.rs`, **not**
> `crates/ken-runtime/src/control.rs`, which does not exist.
>
> **THE HEADLINE FINDING HOLDS AND IS STRONGER FOR BEING RE-MEASURED.**
> `merge_branch_value` exists nowhere under `crates/` except as two string
> literals inside the retired body itself (`host_call_carrier.rs:1826`, `:1836`).
> A subject deleted 19 days after its census was retired, with nothing able to
> notice.
>
> **WHAT THIS BREAKS — AND IT IS THIS NODE'S OWN LOAD-BEARING CLAIM.** Arm 1
> below proposes *"a test that `include_str!`s `control.rs`"* and asserts **"that
> control would have reddened on `1aec3e3e1`."** Measured **at today's tree**: an
> `include_str!` of one file reaches **one of the three**, and the one it misses
> is `d8_join_helpers_...` in `host_call_carrier.rs` — **this node's own headline
> example, the census whose subject was deleted.**
>
> ⇒ **The single-file control is INSUFFICIENT AS A THING TO BUILD TODAY.** The
> claim is true only of a control that enumerates the population **across
> files**. Arm 1 is re-priced accordingly, and arm 2 is **three deletions across
> three files**, not three in one.
>
> > #### CORRECTION, SAME DAY (Architect `evt_4edykxbt49nf1`; re-measured by the
> > #### Steward). AN EARLIER VERSION OF THIS BLOCK SAID *"stale, and not by
> > #### drift — REFUTED."* **THAT WAS HALF RIGHT, AND THE WRONG HALF DISCREDITS
> > #### THE ADVERSARY'S FIGURES.**
> >
> > **The node's `1aec3e3e1` sentence was TRUE WHEN WRITTEN.**
> >
> >     git ls-tree -r --name-only 1aec3e3e1 -- crates/ | grep 'control\.rs$'
> >       -> ONE file.
> >     all three censuses at 1aec3e3e1, in that one control.rs:
> >       :8228   exactly_one_plan_origin_to_expression_lookup_exists
> >       :9216   the_lower_expr_call_population_is_dispositioned_by_owner...
> >       :12885  d8_join_helpers_have_the_closed_typed_caller_population
> >     host_call_carrier.rs was ADDED by ee6e11f95, 2026-08-21
> >       -> FOUR DAYS AFTER the hunt base b430d73e0
> >
> > At `1aec3e3e1`, `include_str!("control.rs")` **did** reach the D8 census. The
> > coordinates are also exact at the hunt base. **The disposition is three-part,
> > and collapsing it to one word was the error:**
> >
> >     insufficient   as a thing to BUILD TODAY
> >     correct        as a claim about 1aec3e3e1
> >     correct        in every coordinate WHEN WRITTEN
> >
> > **A blanket "refuted" tells the next reader to discount the Adversary's
> > figures. The figures are good.** Do not put a word in front of a measurement
> > that implies otherwise — and note that the thing which invalidated the
> > coordinates, `RT-CONTROL-INTEGRATION-TESTS-SPLIT`, is the same file split
> > that killed the three retired bodies. **This node's thesis applied to this
> > node's own prose.**
>
> **THE ENUMERATOR HAZARD RUNS BOTH WAYS, AND THE LIVE ONE IS OVER-COUNTING.**
> `AC-3` warns that an extractor silently *skipping* a spelling reintroduces this
> class one level down. The hazard actually present in the tree is the mirror:
>
>     tests/control.rs:3729   "#[cfg(any())] impl Trait for Type {}"   INSIDE A
>                                                                      STRING LITERAL
>
> A grep-based region-finder reports **four** regions where three exist. **Over-
> counting is not the safe direction** — it manufactures a region containing no
> identifiers, which then passes vacuously and pads the population `AC-3` asks
> for. The measured population is **3 real regions across 3 files, plus 1 false
> positive that a naive finder will pick up.**

| retired census | line | first failure | measured vs asserted |
|---|---|---|---|
| `d8_join_helpers_have_the_closed_typed_caller_population` | 12891 | `control.rs:12899` | `fn merge_branch_value(` defs **0** vs 1 |
| `exactly_one_plan_origin_to_expression_lookup_exists` | 8241 | `control.rs:8255` | pinned planner surface list has 32 entries; the current one is many times that |
| `the_lower_expr_call_population_is_dispositioned_by_owner_not_by_site` | 9234 | `control.rs:9270` | `lower_expr` tokens **70** vs 65 |

**The line numbers in this table are the 2026-08-18 pre-split coordinates and are
retained as the record of what was measured.** Use the amendment block above for
where these censuses actually are.

libtest stops at the first assertion, so the D8 census was re-run with its
asserts replaced by prints, letting the test's own instrument report every value.
**Four of its seven assertions are false**, and the independent greps and the
test's instrument agree exactly:

| the test's own measurement | asserts | actual |
|---|---|---|
| `fn merge_branch_value(` in `mod.rs` | 1 | **0** |
| `fn merge_scalar_branch(` in `mod.rs` | 1 | 1 |
| `fn merge_planned_scalar_branch(` in `mod.rs` | 1 | 1 |
| `.merge_branch_value(` in `core.rs` | 4 | **0** |
| `.merge_scalar_branch(` in `core.rs` | 10 | **4** |
| `.merge_planned_scalar_branch(` in `core.rs` | 1 | 1 |
| `plan: &JoinPlanToken` in `mod.rs` | 3 | **2** |

## The ordering is what makes this a claim about the mechanism

- retirement `6a451b456`, 2026-07-29, put `#[cfg(any())]` on the D8 census
- deletion `1aec3e3e1`, 2026-08-17 (`RT-DESCENT-RETIRE` `D3`-`D6`/`D8`), removed
  `fn merge_branch_value` from `lowering/mod.rs`
- `git merge-base --is-ancestor 6a451b456 1aec3e3e1` returns true

**The subject was deleted 19 days after its census was retired, and nothing could
notice.** `merge_branch_value` now exists nowhere under `crates/` except as two
string literals inside the retired body. A reader consulting `control.rs:12891`
for the D8 join-helper family gets a three-member family whose first member does
not exist.

## Already refuted, so nobody respends it

**The `plan: &JoinPlanToken == 3` assertion falling to 2 is NOT a token-gate
hole.** It was chased as one and it is closed by construction: the live family is
`merge_scalar_branch` (`mod.rs:17919`) and `merge_planned_scalar_branch`
(`mod.rs:18271`), both taking `join_plan: &JoinPlanToken`, both rejecting a
non-`NativeScalarPair` representation before delegating to `merge_scalar_operand`
(`mod.rs:17942`), which takes no token and has **exactly two call sites in all of
`crates/`** — those two wrappers. The count fell 3 to 2 because a helper was
deleted, not because one lost its token.

> #### THE COUNT IN THE PARAGRAPH ABOVE HAS EXPIRED. **THE CONCLUSION SURVIVES
> #### FOR A DIFFERENT AND BETTER REASON.** (2026-09-14, Steward-measured.)
>
>     git grep -n 'merge_scalar_operand' -- 'crates/**/*.rs'     -> 7 hits
>       joins.rs:2425                  the DEFINITION
>       calls.rs:22                    a doc-comment mention
>       joins.rs:967, :978             lower_carried_bool_match  PRODUCTION
>       joins.rs:2412                  merge_scalar_branch       PRODUCTION
>       mod.rs:12672                   merge_planned_scalar_branch PRODUCTION
>       core/tests/constructors.rs:2700  a test, direct
>
> **FIVE call sites, THREE of them production — not two.** The coordinates above
> have also moved: `merge_scalar_branch` and `merge_scalar_operand` are in
> `joins.rs` now, not `mod.rs`. A third production caller,
> `lower_carried_bool_match`, is legitimate — it takes `join_plan:
> &JoinPlanToken` and skips the representation gate because a carried word has no
> native pair, which is what `merge_scalar_branch`'s own doc says it fails closed
> on.
>
> **THE GATE IS STILL CLOSED BY CONSTRUCTION — BY THE CALLEE'S OWN GUARD, NOT BY
> THE CALLER POPULATION BEING SMALL.** `merge_scalar_operand` refuses a
> `LoweringOperand::Carried` with no `required_kind` (*"a carried scalar reached
> an untyped private merge consumer"*), checks the boundary tag against the kind
> claimed, and refuses `ScalarMergeKind::RecursiveBackedge` outright. **That fires
> on every caller, including callers not yet written**, which is exactly what a
> count of callers cannot do.
>
> ⇒ **The ban below stands. Its stated ground is re-grounded here**, so that a
> live prohibition does not rest on a false number. Architect
> `evt_2h8c4fvvrf2jp`; call sites re-measured independently by the Steward.
>
> **NOT ESTABLISHED, and recorded as not established:** only
> `merge_scalar_operand`'s `Carried` branch was read (`joins.rs:2432-2472`, which
> `return`s). **Nobody has read the non-`Carried` continuation past `:2472`**, so
> no one is claiming the guard is complete across every operand shape. Nor was
> `JoinPlanToken` verified unmintable. Closing that is **one bounded read**,
> optional, prices nothing here, and if it comes back "there is a hole" it goes to
> the Architect on its own. **It is not a reason to keep 27 lines of dead text.**
>
> **RE-COORDINATED BY IDENTIFIER, which is this node's own new rule applied to
> its own prose.** The paragraph above cites three `mod.rs:NNNNN` positions and
> **two of the three are not in `mod.rs` at all any more**:
>
>     node says       ->  measured at origin/main, BY IDENTIFIER
>     mod.rs:17919    ->  merge_scalar_branch          joins.rs:2400  pub(super)
>     mod.rs:17942    ->  merge_scalar_operand         joins.rs:2425  pub(super)
>     mod.rs:18271    ->  merge_planned_scalar_branch  mod.rs:12659
>
> **Read the paragraph above by those identifiers, never by its line numbers.**
>
> **AND NAME THE RIGHT FUNCTION WHEN CORRECTING A FALSE PREMISE.** The expired
> "exactly two call sites" claim is about **`merge_scalar_operand`** — the one
> that takes **no** token (`required_kind: Option<ScalarMergeKind>`).
> `merge_planned_scalar_branch` is one of the two *wrappers* and **does** take
> `join_plan: &JoinPlanToken` (`mod.rs:12659-12662`, verified). An earlier
> Steward post attributed the expired count to the wrapper; the measurement and
> the conclusion were unaffected, but the name was wrong, and a note written to
> correct a false premise must not misname the function while doing it.
>
> On the three-production figure: `joins.rs:967` and `:978` are the two halves of
> one site inside `lower_carried_bool_match`, `:967` being the `#[cfg(test)]`
> half. Setting that aside leaves `:978`, `joins.rs:2412` and `mod.rs:12672` —
> **three production callers.**
>
> **This is the FIFTH medium in this WP to rot the same way** — after the three
> retired bodies, this node's coordinates, arm 1's mechanism, and the caller
> count itself. A ban-ground paragraph, in prose, never compiled, inside a node
> about records that rot. **It is also the first one repaired by the rule this WP
> produced:** named by identifier, so the next file split cannot move it.

**What the retired assertion did guard is the family's closure** — a third caller
of `merge_scalar_operand` could be added inside `lowering/mod.rs` with no token
and nothing would object — and the successor the retirement names does not carry
that. **That is a missing guard, not a defect, and it is deliberately not filed
as one.** If the fork below picks deletion, decide separately whether that
closure property is worth a live control.

## `D0` — decide what a retired body IS. The fork is the whole node.

> # `D0` IS RULED, 2026-09-14: **A RETIRED BODY IS NOT A RECORD. ARM 2 — DELETE
> # THE THREE.** Architect `evt_4edykxbt49nf1`.
>
> **The ruling is a read of a controlled comparison the repo already ran, not a
> preference between two untried conventions.** `RT-FNSPLIT-RECUR-PORT` retired
> **four** censuses on one day, in this subtree, on the same family of text
> censuses — and it used **both arms**:
>
>     arm 2 (delete + note)   1 census    27 days on: accurate, resolves, nothing
>                                         to rot.  Successor LIVE at mod.rs:1586.
>     arm 1 (keep the body)   3 censuses  27 days on: 3 of 3 dead, 4 of 7 D8
>                                         asserts false, one names a function
>                                         deleted 19 days later.
>
> **The deleted one is the healthy one.** The surviving note is
> `control.rs:3282` — a retirement note with **no `cfg(any())` and no body**,
> naming its successor by identifier —
> `every_origin_to_expression_resolution_goes_through_the_single_route` — which
> is live today at `mod.rs:1586` under `#[test]`. Verified independently by the
> Steward.
>
> **POPULATION PRECISION, because this node is about exactly this.** The
> amendment above measured **3 `#[cfg(any())]` regions across 3 files**, and that
> is correct *for cfg regions*. The population of **retirements** is **four** —
> three with notes, one (`mod.rs:1133`) with none:
>
>     control.rs:3282           note, NO body        retired by DELETION
>     control.rs:4354           note + cfg body      census 3
>     host_call_carrier.rs:1821 note + cfg body      census 1
>     mod.rs:1133               NO NOTE, cfg body    census 2
>     (grep -c RETIRED:  control.rs 2, host_call_carrier.rs 1, mod.rs 0)
>
> **Two populations, two counts, both right — and an enumerator keyed on
> `#[cfg(any())]` cannot see the retirement that has no body.** That is `AC-3`'s
> skipped-spelling hazard landing on the amendment that raised it.
>
> **NAME THE SUCCESSOR BY IDENTIFIER, NEVER BY POSITION.** The four notes form a
> measured gradation, and it is why the rule is not stylistic:
>
>     control.rs:3282            BY IDENTIFIER   survived the split unchanged
>     host_call_carrier.rs:1817  DESCRIPTIVELY   resolvable, but takes a search
>     control.rs:4351            BY POSITION     "the controls ABOVE" -- cannot be
>                                                checked, silently re-points when
>                                                the file is cut
>     mod.rs:1133                NAMES NOTHING
>
> **WHY NOT A CORRECTLY-BUILT ARM 1.** Not infeasible — not worth having. **Every
> future true red from that control is "your dead text is dead."** The red on
> `1aec3e3e1` would have fired because `RT-DESCENT-RETIRE` deliberately deleted
> `merge_branch_value`; the tree was **right**, and the correct response to that
> red is to delete the retired census. That is a maintenance tax on every
> deletion campaign under `lowering/`, paid forever, to be discharged each time by
> doing what this ruling does once. The repo already says this at the successor
> of the census that *was* deleted (`mod.rs:1581-1584`): *"A pin that froze the
> call count would go red on legitimate work and would be a snapshot wearing an
> invariant's name."*
>
> **THIS IS NOT THE BANNED SPLIT-THE-DIFFERENCE.** Banned is *repairing* the four
> false `D8` assertions or re-deriving the counts. This deletes the assertions
> rather than repairing them; no count in the subtree changes value.
>
> **WHAT THE RULING DOES NOT ESTABLISH — preconditions on each deletion, not on
> the ruling.** The Architect verified each named successor **exists**; it did
> **not** verify each one **carries the property its note claims**. That is a
> behavioural read at each successor's site and it is the ring's. **Census 2 is
> the open one** — no note, so no named successor. If any census comes back *"the
> successor does not carry it,"* that census goes back to the Architect on its
> own and the other two proceed. The prose relocation in `(a)` below is also
> **unpriced**; it is the only non-mechanical part.

The two arms are not equivalent and neither is obviously right.

1. **A retired body is a record.** Then it needs one cheap live control keeping
   it honest: a test that reads **every file holding a retired region** and
   asserts every identifier named inside a `#[cfg(any())]` block still resolves
   somewhere under `crates/`. **A control of that shape would have reddened on
   `1aec3e3e1`.**

   **AMENDED 2026-09-14 — the original wording said `include_str!`s
   `control.rs`, and that form is REFUTED.** The three retired regions live in
   three files, so a single-file read reaches one of them and misses
   `d8_join_helpers_...`, which is the very census whose subject `1aec3e3e1`
   deleted. **The "would have reddened" claim is true of the multi-file form and
   false of the single-file form**, and it is the reason anyone would choose
   this arm — so it must not stay attached to the mechanism that cannot deliver
   it. The control's own population is therefore part of the deliverable, not an
   incidental: see the amendment block above, and `AC-3`.
2. **A retired body is not a record.** Then deleting these three is more honest
   than preserving text that reads as one, and git history remains the record.

**Do not split the difference by fixing the three current failures.** Re-deriving
them restores the appearance of currency without restoring the mechanism, and the
next deletion re-rots them with nobody watching. That is the failure this node
exists to stop, not an instance of it to clean up.

## Acceptance criteria

> # RE-CUT 2026-09-14 AFTER THE `D0` RULING. **`AC-2` IS NOT APPLICABLE — it was
> # conditioned on arm 1, and arm 1 was not taken.**
>
> The Architect ruled the design; the acceptance is the Steward's. Under arm 2:
>
>     AC-1   RE-AIMED  state the convention at core/tests/mod.rs's header (it
>                      already carries this subtree's ruled conventions). The
>                      convention site DOES NOT EXIST yet -- the convention has
>                      only ever been instantiated, never stated.
>     AC-2   N/A       conditioned on arm 1's control. Not applicable, not waived:
>                      there is no control to demonstrate.
>     AC-3   STANDS    and is already answered -- see the population precision in
>                      the D0 block: 3 cfg regions across 3 files, 4 retirements,
>                      1 false positive at tests/control.rs:3729.
>     AC-5   NEW       the three deletions, with their extents.
>     AC-6   NEW       relocate the argued prose; delete only the counts.
>     AC-7   NEW       census 2's successor note WRITTEN.
>     AC-8   NEW       each deletion's successor carries the property.
>     AC-4   UNCHANGED workspace green IN CI.
>
> - **`AC-5` — the three deletions, item head to closing brace.** Measured
>   extents at `3876edea0`:
>
>       core/tests/control.rs            4354-4426     73 lines
>       core/tests/mod.rs                1133-1334    202 lines
>       core/tests/host_call_carrier.rs  1821-1847     27 lines
>
> - **`AC-6` — the ARGUMENT is relocated, the COUNTS are deleted.** *"git history
>   remains the record"* is true of a count and **false of an argument.**
>   `control.rs:4299-4350` is a 52-line argued doc comment attached to the retired
>   `fn`, about the **live** authority — the validated owner partition and the two
>   claims withdrawn from `AC-5` of its own node. `mod.rs`'s 202-line body carries
>   the exported-surface argument, including why `AbiPlane`, `AbiDescriptor`,
>   `build_abi_plane` and `AbiPlane::validate` stay `pub(super)` and are
>   deliberately **not** in the list. **Prose about a live control belongs at the
>   live control.** Relocating it is the one non-mechanical part of this WP and is
>   **unpriced**; if it turns out larger than it looks, stop and say so.
>
> - **`AC-7` — census 2 gets a successor note WRITTEN, not just a deletion.**
>   `core/tests/mod.rs` contains **zero** occurrences of `RETIRED`;
>   `exactly_one_plan_origin_to_expression_lookup_exists` was cfg-retired with no
>   successor named and no reason given. **This is the single case where arm 2 as
>   the node states it genuinely loses something** — there is no note to survive.
>   **If no live control carries the property, the note SAYS THAT**, rather than
>   naming a successor that does not carry it.
>
> - **`AC-8` — each deletion's successor is shown to CARRY the property, not
>   merely to exist.** The ruling verified existence only, and said so. This is a
>   behavioural read at each successor's site and is a **precondition on that
>   deletion**, not on the WP. A census whose successor does not carry the
>   property goes back to the Architect on its own; the others proceed.
>
>   > **`AC-8` FIRED AND IS DISCHARGED, 2026-09-14. ALL THREE CENSUSES PROCEED.**
>   > Architect `evt_2h8c4fvvrf2jp`, on the implementer's precondition reads.
>   >
>   >     census 1  named successor borrowed_ingress_bytes_at_preserves_safe_none_bounds
>   >               DOES NOT carry it -- a behavioural bounds test. Zero live TESTS
>   >               assert the typed-token requirement.  PROCEEDS ANYWAY, see below.
>   >     census 2  no note at all; note to be WRITTEN (AC-7)
>   >     census 3  successor named BY POSITION ("the controls above"); live
>   >               candidates exist at :483/:532/:583/:744 but a positional
>   >               reference cannot be verified to resolve to what its author
>   >               meant.  PROCEEDS; its note is REWRITTEN by identifier (AC-1).
>   >
>   > **Census 1 is the one worth reading twice, because `AC-8` resolved by
>   > finding the record was never the body.** The census asserted
>   > `helpers.matches("plan: &JoinPlanToken").count() == 3` over `mod.rs` — a
>   > count of the token in **caller signatures**, in **one file**, as a proxy for
>   > a guard that lives in the **callee's body**. The property is enforced at the
>   > point of use by `merge_scalar_operand` itself.
>   >
>   > ⇒ **A CALLER-SIDE CENSUS CANNOT ESTABLISH A CALLEE-SIDE INVARIANT.** It
>   > reports the caller population on the day it ran and goes red on every
>   > legitimate new caller — here it would have fired on
>   > `lower_carried_bool_match`, which is correct code. That is the same "your
>   > dead text is dead" tax `D0` ruled against, one level in.
>   >
>   > **The search that returned zero was the wrong search**, and the census's own
>   > framing is what invited it: it looked for a live **test**. The carrier is
>   > not a test and is not on the caller side at all.

- **`AC-1` — RE-AIMED BY THE `D0` RULING.** State the retirement convention at
  `core/tests/mod.rs`'s header: **retirement is DELETION plus a note at the site
  naming the successor control BY IDENTIFIER**, and `#[cfg(any())]` is not a
  retirement idiom. Name a successor by identifier, **never by file or line** —
  `RT-CONTROL-INTEGRATION-TESTS-SPLIT` moved every census in this subtree, and
  the one note that named its successor by identifier survived it unchanged.
  *(Superseded original: "`D0` is decided and `control.rs` says which reading it
  took, at the retirement convention's own site rather than only in this node."
  The site was right in spirit and wrong in file — the convention belongs at the
  module root, and `control.rs` is one of three files it governs.)*
- **`AC-2` — NOT APPLICABLE under the `D0` ruling (arm 2 taken).** Retained
  verbatim below as the record of what arm 1 would have had to satisfy; it is
  not a criterion of this WP and nothing is owed against it.
  > **SUPERSEDED TEXT, retained as a record. NOT a criterion.** *"`AC-2` — if
  > arm 1: the control is demonstrated by the mutation it catches. Delete or
  > rename an identifier named inside a retired body and show the control reds;
  > restore it and show green. A green run on the current tree is not evidence —
  > the current tree already fails 3 of 3, so the control must be shown to red on
  > today's `main` before any repair, and that red is the acceptance evidence,
  > not a regression."*
- **`AC-3` — the population is stated.** Say how many `#[cfg(any())]` regions
  exist **and in which files**, and how the control enumerates them. An
  identifier extractor that silently skips a spelling reintroduces exactly this
  class one level down — **and one that over-counts is not the safe direction
  either.** The measured population at `3876edea0` is **3 real regions across 3
  files, plus 1 false positive** (`tests/control.rs:3729`, a `#[cfg(any())]`
  inside a string literal). A finder that reports four has manufactured a region
  with no identifiers in it, which then passes vacuously. **`AC-3` is satisfied
  by a stated count that matches a stated file set, not by a number alone.**
- **`AC-4` (no-regression).** Workspace green **in CI**, never a local
  `--workspace` run (`COORDINATION §12`).

## Banned scope

- **Reviving any retired census as a live test.** All three fail; turning them on
  is a different and much larger piece of work, and `D3` of
  [[RT-D2-EVIDENCE-INSTRUMENTS-NONDISCRIMINATING]] already ruled that reviving a
  census in this file is not a cheap option.
- **Repairing the four false D8 assertions.** See the fork — that is the
  split-the-difference move.
- **Filing the `merge_scalar_operand` closure gap as a defect.** It is a missing
  guard on a gate that is closed by construction today. **RE-GROUNDED
  2026-09-14: still banned, but NOT because the caller population is small —
  that count has expired (five call sites, three production, not two).** It is
  closed by the **callee's own guard**, which fires on every caller including
  ones not yet written. See the expiry block above.
- **Claiming a regression or reclassifying `D3`'s landing.** `D3` is correct and
  this finding strengthens its choice: it rejected revival because the census is
  compiled out, and revival turns out not to be available for this family at all.

## Sequencing

Blocks nothing. The operator's run order stands. **It is worth doing before the
next large deletion campaign under `lowering/`**, because that is when a retired
body rots, and the campaign in front of it is the backend module split.

## Counts and symptom inventory

**Hard stops: 1.** `§1a` fires at 3, and the Steward's tracker is the count of
record. The one stop is the Architect's own `§8` precondition firing while it
built the ruling — counted deliberately rather than waived, on the ground that
over-counting a structural wall is the safer error.

**Symptom inventory, entry 1:**

> 1. census 1's retirement note named a successor that does not carry the
>    property, and the property turned out to be enforced somewhere else
>    entirely — keyed on **caller signatures** when the guard is in the
>    **callee's** body

Its predicate is this node's own thesis one level in: **a record that names the
tree by text or by position rots, and the guard it proxies for does not.**

**The rot count for this WP is FIVE media, not three.** The three retired bodies
(`cfg`-stripped), this node's coordinates (prose, never compiled), arm 1's
mechanism (prose, never compiled), and the banned-scope caller count (prose,
never compiled). **Two of the four were never compiled at all, so `cfg`-stripping
cannot be the cause** — which refutes the node's own mechanism sentence,
*"cfg-stripping precedes name resolution, so a retired body is neither
type-checked nor name-resolved."* That is true in general about `cfg(any())`
bodies and is **not** what killed these.

**All three retired bodies are TEXT censuses** — every subject is a string
literal (`include_str!` plus `.matches(...)`, or `identifier_occurrences`). **A
live `#[test]` containing `"merge_branch_value"` in a string is not name-resolved
against `merge_branch_value` either.** The compiler never checked these, before
retirement or after. **What kept them honest was being RUN, and the assertion
failing.** Name resolution is not in this story.

## Census 1's note, for the deletion

Suggested text, adjust freely — naming the real carrier **by identifier** is the
point, per `AC-1`:

```rust
// RETIRED by RT-RETIRED-CENSUS-ROT D0 (a retired body is not a record).
// This counted `plan: &JoinPlanToken` in caller signatures in `mod.rs` -- a
// caller-side proxy for a callee-side guard. The property is enforced at the
// point of use by `merge_scalar_operand` itself: a `LoweringOperand::Carried`
// with no `required_kind` is refused ("a carried scalar reached an untyped
// private merge consumer"), the boundary tag is then checked against that
// kind, and `ScalarMergeKind::RecursiveBackedge` is refused outright. That
// fires on every caller, including ones not yet written, which a count of
// callers cannot.
```
