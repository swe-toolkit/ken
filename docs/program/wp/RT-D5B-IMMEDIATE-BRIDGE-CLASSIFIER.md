# WP frame — `RT-D5B-IMMEDIATE-BRIDGE-CLASSIFIER`

**Owner:** Team Runtime · **Size:** M · **Risk:** low (additive, no live path) ·
**Tier:** T1 · **Gate:** none · **Deps:** none

**Origin:** operator directive 2026-09-16, verbatim: *"factor small mergeable
pieces out of the long string of commits and merge those... Small achievable
pieces, not one monolithic PR."* PR #3676 is closed and its branch
`wp/ABI-S6-d5b-file-backed` is kept at
`0f71ab5b9267781ae1d91bc654011cad42b926af` as a read-only reference. This is
slice 1 of the re-cut, and it is the first `crates/` diff the lane has produced
since `ff9d0f0ca`.

## 1. Objective

Land the **plan-independent immediate-bridge classifier** as a standalone
module on `main`, exercised by tests that fail without it. No plan coupling, no
field on `StaticTransitionPlan`, no behavioural change to any existing path.

## 2. Why leaf-extraction was abandoned, so nobody retries it

Four extraction attempts against the 36-commit stack, all measured, all
refuted:

    24 of 36 commits are one- or two-file        looked promising
     5 cherry-pick cleanly onto main             looked mergeable
     0 survive inspection
       5fcfaabf5  include_str! a module ABSENT from main
       57ef38eeb  same module reference
       6cf61ea8c  cherry-pick produces an empty diff
       7e6eca324 + 936406cc3   comment-only, single file, and BOTH
                               CONFLICT on cherry-pick: units.rs has
                               diverged around the hunks

**A clean cherry-pick is not a compile, and a comment-only diff is not a
context-free diff.** The dependency root `bc7cb0304` is 17 files and
`+3194/-648`; every small leaf refines it. **There is no small piece to peel
off because the bottom of the stack is not small.** The mechanism is therefore
re-cut from `main`, using the branch as a source rather than as a merge
candidate.

## 3. Fixed inputs, measured at the implementation base

**Implementation base is `origin/main` at
`80d3ff78042d47b841ad64165f3ee4f50b524f4c`, and that is a ruling, not a
default.** Source is
`0f71ab5b9:crates/ken-runtime/src/cranelift_backend/planning/static_transition/immediate_bridge.rs`,
622 lines.

**The seam is a single truncation point.** `StaticTransitionPlan` is named at
`:342 :457 :515 :598 :609` — every occurrence at or past `:341`. Nothing
plan-coupled sits above it.

    STRATUM A   :1-340    this WP      ends on the closing brace of
                                       shifted_aggregate_ihs
    STRATUM B   :341-622  successor    derive_/build_/publish_/validate_,
                                       the extension impl, the field,
                                       the cfg-gated mutation guard

Verified in both directions: **Stratum A references zero Stratum-B names**, and
`impl ImmediateBridgeRealization` touches no plan.

**Stratum A's import surface, counted over the body with the `use` block
excluded:**

    USED       RuntimeExpr 31   StaticOriginId 8   BTreeSet 4
               RuntimeTrap 3    ContinuationCallIdentity 2
    NOT USED   planner_error 0   planner_capacity_error 0
               CraneliftBackendError 0   StaticTransitionPlan 0   BTreeMap 0

The error-handling surface belongs to Stratum B. Stratum A's import block is
two lines.

**The slice builds on `main`, measured rather than predicted:**

    baseline    main as-is, no slice          EXIT=0    88 warnings
    with slice  mod + 338-line Stratum A      EXIT=0   101 warnings
    CONTROL     inject a syntax error         EXIT=101
    RESTORE     revert the injection          EXIT=0

The control is recorded because a build that silently never compiled the module
would produce the same `EXIT=0`. Diagnostics cite line coordinates inside the
new file.

## 4. THE TRAP — the `+31` hunk on `static_transition.rs` is NOT registration

Architect ruling, `evt_2a3q52kafnsvw`, and this is the part most likely to be
got wrong by someone lifting the branch's diff:

    + mod immediate_bridge;  + the pub(in ..) use list   declarative, fine
    + the immediate_bridge_realizations field            STRATUM B
    + with_d5b_hs10_inline_response_mutation             test-support
    + let owns_seat = match row.sub_case() {             LIVE BEHAVIOURAL CHANGE
    +     DeferredResponseSubCase::InlineBridgeNoCall

**`InlineBridgeNoCall` is absent from `main`** (0 files, against a control of
`DeferredResponseSubCase` at 3 files). That hunk carries a new enum variant
**and** a rewrite of response-seat ownership. **Anyone taking "the module plus
its registration" by lifting that file's hunk gets an unreviewed behavioural
change inside what they believe is a declarative slice.** It is slice 3 at the
earliest and needs its own frame.

**This WP's diff to `static_transition.rs` is ONE LINE:** `mod
immediate_bridge;` between `mod effects;` and `mod joins_traps;`.

## 5. Deliverables

1. `crates/ken-runtime/src/cranelift_backend/planning/static_transition/immediate_bridge.rs`
   — Stratum A only, with the two-line import block from §3.
2. The one-line `mod` registration.
3. **Unit tests over `RuntimeExpr` values that fail without the classifier.**
   This is the WP's real work — see §7.

## 6. Acceptance

**AC-1. Stratum A only.** `StaticTransitionPlan` occurs **zero** times in the
new file; no field is added to `StaticTransitionPlan`; no extension `impl`
exists. Control: `grep -c StaticTransitionPlan` on the new file returns 0, and
`git diff --numstat` shows `static_transition.rs` at `1` insertion, `0`
deletions.

**AC-2. The tests FAIL WITHOUT THE MODULE, and this is demonstrated, not
asserted.** Control: stub the classifier's return (or delete the module and its
`mod` line) and record the tests going red; restore and record green. **Cite
both runs.** A green that survives the module's removal proves only that the
crate compiles, and this AC exists because that is precisely the failure mode
the slice was designed against.

**AC-3. No behavioural change to any existing path.** Control: the diff touches
the new module, the one `mod` line, **and nothing else outside a `#[cfg(test)]`
region.** A change to any `crates/**/src/` file reachable in a production build
fails this AC.

> **AMENDED 2026-09-16 by the Steward, after the implementer hit the conflict
> and escalated rather than picking a side (`evt_ef39ahrmv58k`). The original
> control read "exactly two files", and that was WRONG — not too strict, but
> measuring the wrong thing.**
>
> `ken-runtime`'s lib suite carries a **closed-inventory pin**,
> `the_backend_production_surface_inventory_is_closed` at
> `lowering/core/tests/control.rs:3806`, which asserts the backend's complete
> `mod` roster by equality. **Adding any backend module necessarily reds it.
> That is the pin working, not breaking** — it exists to force exactly this
> review, and its roster carries a provenance comment per entry.
>
> **The decisive fact is the cfg profile, not the intent.** `core.rs:55-56`
> declares `mod tests` under `#[cfg(test)]`, so the whole `core/tests/` subtree
> — `control.rs` included — **cannot be reached by a production build.** Adding
> `("planning/static_transition.rs", "immediate_bridge")` to that roster is
> therefore *provably* incapable of changing any existing path's behaviour,
> which is this AC's actual criterion. **The file-count control was a proxy for
> that criterion, and the proxy was wrong while the criterion held.**
>
> The alternative — ship two files and leave the pin red — was rejected on
> standing operator direction (2026-09-09): **red CI is a priority to fix, not a
> merge-past to normalize.** It would also disqualify this candidate from being
> the `crates/` merge that fires `D0`'s sweep.
>
> **The pin update is REQUIRED, not merely permitted**, and it carries a comment
> naming this node, matching the roster's existing convention.

**AC-4. `InlineBridgeNoCall`, the `owns_seat` rewrite, the
`immediate_bridge_realizations` field, and
`with_d5b_hs10_bridge_plan_mutation` are ABSENT from the candidate.** Control:
each name greps to 0 hits across the diff. These are slices 2 and 3.

**AC-5. The production-build warning delta attributable to the new file is
accounted for explicitly** — see the open decision in §7. Control: build at the
base and at the candidate, diff the warning lists, and attribute every new
warning to a source file. **Warnings arising in files other than the new module
fail this AC.** At the base measurement the delta was **13**, all in the new
module and none elsewhere.

> **CORRECTED: this frame first recorded 12. The implementer measured 13 and is
> right; the Steward's count was an undercount of one.** The census pattern
> matched `is never used` and missed `is never constructed`, which two structs
> emit. **The measured warning-count delta was `101 - 88 = 13` all along and the
> frame's own arithmetic disagreed with its own number** — the sort of
> discrepancy that is invisible when a count is transcribed rather than
> recomputed.
>
> **This is precisely the re-measurement the phrasing was designed to permit,
> and it is why the number is not a bar.** A candidate producing eleven or
> fourteen satisfies this AC with that figure. **The failure condition is a
> warning outside the new module, never a different count.**

**AC-6. Every test case carries its provenance.** A case whose expected verdict
is **lifted** cites its ancestor as `file:line` at `0f71ab5b9`. A case with no
cited ancestor is labelled **new intent** in the test itself. Control: every
case has exactly one of the two.

> **`AC-6` IS NOT REDUNDANT WITH `AC-2`, AND THE REASON IS THE POINT OF BOTH.**
> `AC-2` cannot catch a **by-construction** suite: stub the classifier's return
> and a test whose expectation was simply read off the classifier still goes
> red. **`AC-2` measures COUPLING, not FAITHFULNESS.** It is necessary, it is
> well-designed for what it does, and it is not sufficient — and the gap is
> exactly the size of `AC-6`. Same family as the entailment ruling on
> `call_seeds`: a value and the thing it is offered to confirm, sharing one
> producer. (Architect, `evt_5rbgwyamv4y2n`.)
>
> **MEASURED, and the gap is larger than the argument predicted.** The
> implementer ran `AC-2`'s control and reported `6 FAILED, 8 passed`
> (`evt_ef39ahrmv58k`). **The stub returns `None`, and a test that EXPECTS
> `None` cannot be reddened by it** — so all seven `AC-7` refusal cases and the
> one negative-predicate case survive the stub. **`AC-2` is structurally blind
> to the refusal half of the suite: 6 of 14 coupled, 8 not.**
>
> This is not a defect in `AC-2` and it is not repairable by a better stub — it
> is what a return-value control can see. **`AC-6` carries the whole weight on
> those eight cases**, which is the concrete reason provenance is per-case and
> non-negotiable rather than a documentation nicety.

**AC-7. Each refusal point has a case.** The classifier's refusals are the
extraction-fragile part, and a suite authored by walking the `Some` arms will
miss them. **Every coordinate below was re-verified at source; the arm heads are
NOT the refusal sites:**

    :190        body is none of the four spellings
    :163/:182   Checked wrapper with the wrong inner form
                (:156 / :175 are the ARM HEADS -- the `return None` lives
                 in the `let ... else` at :163 and :182)
    :193        scrutinee is not Var
                (:192 is the `let ... else` head)
    :196        offset underflow -- index.checked_sub(argument_binder_offset)?
    :197        selected_field past the end of producer_args
    :205        neither cause

**`:195` IS DELIBERATELY EXCLUDED AND MUST NOT BE ADDED.** It is
`usize::try_from(*index).ok()?`, and `RuntimeExpr::Var` carries a `u32`
(`0f71ab5b9:crates/ken-runtime/src/ir.rs:691`). **`usize::try_from(u32)` cannot
fail on any target this project supports**, so an AC requiring a case for it
would be unsatisfiable and a test written to satisfy it could not go red. That
is the same defect as an uncovered refusal, approached from the other side.

**`:205` does NOT require a lifted negative verdict.** There are two routes to
`heterogeneous == false` at `:198`, and one needs no producer evidence at all:

    :198  let heterogeneous = !checked_ih_slots_wrapper
                              && requires_heterogeneous_deforestation(producer);

**Place an attested POSITIVE producer — any of the three at `mod.rs:1518` /
`:1392` / `:2691` — inside the `CheckedComputationalIHSlots` spelling and
`heterogeneous` is false by the wrapper, whatever the producer's verdict.** So
`:205` is reachable from evidence already in hand, and the case composes with
`AC-8` into one shape rather than two.

**The honest remainder, stated rather than smoothed:** `:205` also needs
`static_host_operation == false`, and `statically_selects_host_operation` has
zero ancestors, so **that leg is new intent whichever route is taken** and is
labelled so per `AC-6`. This repair does not make it attestable; it stops it
being compounded by a second unattested leg.

**AC-8. The `checked_ih_slots_wrapper` interaction has a case.** At **`:198`**
that flag **suppresses** the heterogeneous test, and it is the only place the
flag does anything. **The same producer yields `Heterogeneous` under `Match` and
does not under `CheckedComputationalIHSlots`.** Two lines, and it pins the one
flag the four-way dispatch sets. Compose it with the `:205` case above.

**AC-9. The candidate adds no `#[ignore]` rows, or names and excludes any it
adds.** Control: one grep at candidate time; the expected answer is zero, since
§5.3 gives no reason for ignored rows. **This is a `D0` precondition, not a
style rule:** the ignored-row sweep measures the merged tree, so any row added
here enters `D0`'s population and its dispositions would be working a roster
nobody framed. Check it rather than assume it.

## 7. The WP's real risk: how the tests get their expectations

**RULED by the Architect, `evt_5rbgwyamv4y2n`. The framing below replaces this
section's earlier "the tests are new authorship" claim, which was too coarse.**

The tests are **not** uniformly new authorship. The classifier's expectations
split into two kinds with completely different risk, and only one of them is
hazardous:

    SEMANTIC   "does this producer require heterogeneous
                deforestation?"                              MUST BE LIFTED
    STRUCTURAL "a Match whose scrutinee is Var(3) with
                offset 1 selects producer_args[2]"           AUTHOR IT FREELY

Read at `0f71ab5b9:...immediate_bridge.rs:132-213`, everything above `:198` is
total structural dispatch and arithmetic over values the test author chooses.
**The only judgment in the function is the two calls at `:198-199`** —
`requires_heterogeneous_deforestation` and `statically_selects_host_operation`.
So the `RuntimeExpr` handed to the classifier is a **composition**: an attested
producer, lifted and citable, placed at an index of the author's choosing inside
a wrapper of the author's choosing.

**Attested positives for `requires_heterogeneous_deforestation`, verbatim at
`0f71ab5b9`:**

    core/tests/mod.rs:1518-1522  Call { callee: Closure -> Construct }   TRUE
                                 fixture constructs it at :1509-1514
    core/tests/mod.rs:1392-1395  a declaration call producing an
                                 aggregate -- a distinct second shape    TRUE
    core/tests/mod.rs:2691-2693  Match { scrutinee:
                                 Construct("ctor:prelude::Bool::True") } TRUE
                                 inside seed_call_port_producer_match_example()

**THE NEGATIVES ARE SCARCE AND WE HAVE NO ATTESTED SOURCE FOR THEM. An earlier
version of this frame claimed `core/tests/specialization_binding.rs:4295` and
`:4301-4302` were one. THAT CLAIM IS STRUCK — it is false.** Read to the end of
that comment at `:4304-4323`, the three rejected shapes reject on the *other*
facts in its four-fact conjunction:

    :4304-4309  planner refusal, "computational continuation is outside its
                source owner subtree"
    :4310-4314  the ordinary producer route declined -- "that route defined no
                units, so FACT 3 failed silently"
    :4315-4323  a runtime merge materializes a source join the deferred-
                constructor case "refuses outright"

**None is a negative verdict from `requires_heterogeneous_deforestation`.** The
citation is still useful as context for the fixture; it is **not** evidence for
`AC-7`. The failure was reading a comment's headline sentence and substituting
it for the bullets underneath — *three shapes were rejected* became *three
shapes with an attested negative verdict on the predicate I needed*.

A suite authored by reading the function is almost always all-positive — the
author walks the `Some` arms. **The refusals are where an extraction bug
lives**, which is why `AC-7` enumerates them structurally instead of relying on
lifted negatives that do not exist.

**`produces_deforestable_aggregate_with_ih` gets a DIFFERENTIAL case, and it is
required.** Production code at `0f71ab5b9:lowering/mod.rs:11739-11740`:

    produces_deforestable_aggregate_with_ih(expr, &recursive_hypotheses)
        && !produces_deforestable_aggregate_with_ih(expr, &BTreeSet::new())

Same `expr`, two IH sets, opposite verdicts — attested by every green run of
that branch's suite. The reason it is required is specific: `shifted_aggregate_ihs`
at `:337` shifts the IH set by binder depth, `:276` recurses **without** the
shift and `:280` **with** it. **A single-point test that never crosses a binder
cannot tell those two arms apart, so an off-by-one in the shift is invisible to
it.** The differential over a binder-crossing shape is what catches it.

**THE HONEST GAP, declared rather than papered over.**
`statically_selects_host_operation` has **zero** plan-level ancestors — measured
across `crates/` at `0f71ab5b9`, two hits, both inside the module itself (the
call at `:199`, the definition at `:215`). **Every case whose expectation turns
on `ImmediateBridgeCause::StaticHostOperation` is NEW INTENT and is labelled so
by name in the test.** That cause gets attested by Stratum B, which is what
gives it a consumer.

**Rejected approach, recorded so it is not re-proposed:** instrumenting
`core.rs:7144` on the reference branch to capture the `case_body.expr` values
the existing tests drive through it. It is a genuine lift and it is not ruled
out on cost — it yields *whatever the existing tests happen to drive*, which is
a sample of the input distribution rather than a characterization of the
classifier, in opaque values that tell the next reader nothing. Held in reserve
for the `StaticHostOperation` gap alone, and not ordered for this slice.

## 7a. The dead-code window: RULED (a), ACCEPT THE TWELVE WARNINGS

**Architect ruling, `evt_5rbgwyamv4y2n`, against this frame's earlier
recommendation of `#[allow(dead_code)]`. The ruling is adopted and the reasoning
is better than the recommendation it replaces.**

**In this slice the warnings are not debris — they are the instrument.** The
twelve `never used` warnings are the only live indicator that the module is on
no live path, which is the property `AC-3` defines the slice by. Suppressing
them makes that property unobservable for the whole window:

    AC-5 clause "no warnings outside the new module"    SURVIVES
    AC-5 clause "attribute the delta to a source file"  LOSES ITS CONTENT

Option (b) does not make `AC-5` vacuous; it kills one of its two clauses, and it
is the clause carrying `AC-3`'s evidence.

Three further reasons, in order of weight:

1. **(a) is self-clearing; (b) needs someone to remember.** The warnings vanish
   the moment Stratum B wires the classifier in. (b)'s removal obligation would
   live in a frame that does not yet exist — and *"an `allow` with no named
   remover is how a temporary suppression becomes permanent"* was this frame's
   own hazard note. The condition attached to (b) was right; it just costs more
   than the thing it protects.
2. **A recorded baseline turns noise into a discriminator.** With twelve written
   down, a thirteenth warning or one outside the module is a finding. Silence
   has no such property.
3. **This lane's history argues against (b).** "One slice's duration" has been
   meaning longer than intended, and the longer the module sits without a
   consumer, the more useful a standing signal saying exactly that.

**No `#[allow(dead_code)]` is added. The twelve warnings ship.**

## 8. Contention

Writes one new file and one line of
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`. No
open candidate touches that file. `wp/RT-IGNORED-PASSING-ROWS-D0` is read-only
over test files and `.github/`. No contention.

**This WP produces a `crates/` diff, which sets `classify-paths` `mode=full`
and fires the `ignored-row sweep` job — the instrument `D0` needs and the only
compliant venue for it under `COORDINATION §12`.** That is a consequence worth
knowing, not a reason to rush the slice.
