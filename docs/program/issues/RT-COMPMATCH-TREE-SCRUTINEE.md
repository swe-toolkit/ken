---
id: RT-COMPMATCH-TREE-SCRUTINEE
title: "rt_span_prov sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines now stops EARLIER than the refusal this node is named for -- the native static-transition planner refuses with 'an exact detached required consumer has no computational occurrence' because the source-return-context template carries no ComputationalMatchCase step. The id names the second-layer scrutinee refusal and is NOT renamed; the title is. Force past the first stop, record whether the scrutinee refusal is still behind it, and repair."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Measured by the RT-SRCBODY-BIND-ORDER D12 complete no-fail-fast enumeration (evt_2n9wq8xyj0aa1); re-measured by RT-IGNORED-FAILING-ROWS-INVENTORY's ledger (docs/program/evidence/rt-ignored-failing-rows-ledger.md) row 14, which records a DIFFERENT and EARLIER refusal than the one this node is named for and label agrees? NO. Fails at frozen base 21fd46dc as well as at the candidate, so it is pre-existing base debt and not a regression. Framed by the Steward 2026-09-18 under the operator L1 directive 2026-09-17 ('Is L1 still working on clearing the ignored tests? That is the top priority until it is done.'). D0 is a single forcing step with a named target and a binary outcome, NOT a peeling ladder -- the terminating observation is written down in D0 itself. depends_on was [RT-SRCBODY-BIND-ORDER]; that node is merged and the edge was a READ, not a dependency. Steward-filed per COORDINATION section 2."
---

> # TREAT EVERY ANCHOR IN THIS FRAME AS PERISHABLE
>
> If a fixed input below turns out false against the landed code, **say so and
> escalate — do not quietly build around it.** Every signature here was measured
> at a named tree, and this node exists because a signature moved underneath the
> claim that rested on it. The phrases are given so you can re-find the sites;
> **re-measure at your own base before you build.**

# The measurement

`crates/ken-cli/tests/rt_span_prov_native.rs`,
`sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`.

**Measured at `04d4dd38a9cfb40da9b5d68bfeb42aaddbb76661`** by the failing-rows
ledger, row 14:

```text
Cranelift backend failure: native static transition planner invariant
failed; please report this compiler bug: an exact detached required
consumer has no computational occurrence
```

**Superseded as the FIRST refusal, NOT disproved as a property of the row:**

```text
ComputationalMatch: tree-producing match scrutinee is not Bool or a
constructor
```

The ledger records `label agrees? NO`: the label predicts the scrutinee
refusal, which is the *second* layer, not the one the row hits first.

> ### THE `id` IS NOT RENAMED. THE `title` IS.
>
> The id is cited from other artifacts — including the descent campaign — and
> renaming it breaks those citations. **A title is not a citation key**; it is
> the line a reader consults *instead of* the body. Fix the title, keep the id.

## The row is DIFFERENTIAL and it asserts SUCCESS on both engines

The body asserts `diff.native.exit_status == diff.interpreted.exit_status`, that
**each** is `0`, that `buffer_freeze_outcomes` agree, and then runs
`assert_freeze_sequence` on each. The interpreter is expected to succeed. **The
native lane is the one refusing to build.**

⇒ This is not a "both engines are wrong" row. It is a native-backend refusal on
a program the interpreter runs to completion, which makes the interpreter a
live oracle for what the native lane should be producing.

# THE IN-TREE ANNOTATION IS STALE IN BOTH PLACES

The comment block asserts the superseded scrutinee refusal under *"Observed
signature, exactly"*, and the `#[ignore]` reason string repeats it:

```text
"RT-COMPMATCH-TREE-SCRUTINEE: a tree-producing match scrutinee is neither
 Bool nor a constructor; fails at base 21fd46dc"
```

**Both are audited for presence and consumed for content, and both drifted.**
Correct them under `D2`.

# Where the refusal is produced

`crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`,
in `pair_detached_required_consumer`. **Cited by phrase because a line number is
destroyed by the edit this node performs:**

    grep -n "has no computational occurrence" \
      crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs

The mechanism is exact and small. The function walks the
`SourceReturnContextTemplate`'s steps **in reverse** — the comment records why:
*"Template steps run root-to-result; reversing selects the first computational
consumer reached by this exact result on return."* It takes the first step whose
role matches `SourceReturnContextRole::ComputationalMatchCase(_)`, and
`ok_or_else`s into the refusal when **no step matches at all**.

⇒ **The refusal means the projection for this result carries no
`ComputationalMatchCase` step anywhere.** It is an absence, not a mismatch — and
that is the fact the repair turns on.

# `D0` — force past the first stop, once, with a named target

**This is a SINGLE forcing step with a named target and a binary outcome. It is
not a peeling ladder and must not become one.** The terminating observation is
written down here, in advance, and the world can produce it either way:

    the scrutinee refusal IS behind it       -> recorded, the witness holds
    it is NOT -- something else, or success  -> recorded, the witness is gone

**If a THIRD, unexpected layer appears, that is an ANSWER.** Report it and stop.
Do not force again to see what is behind that.

Run at **your own named base**, not a SHA pinned in this frame. Un-ignore the
row locally and run it targeted (`scripts/ken-cargo`, `--test
rt_span_prov_native`; never `--workspace`).

    (a) the detached-consumer refusal reproduces
          => force past it (see "How to force") and record the next
             observation.
    (b) a DIFFERENT first refusal appears
          => that is the result. Record it, correct the annotation, STOP.
    (c) the row PASSES
          => the premise is gone. Report it and propose un-ignoring. A
             REPORTABLE OUTCOME, not a failure to reproduce.

**How to force:** the refusal is an `ok_or_else` on an empty `find_map`. Forcing
means letting the planner proceed past the absent computational occurrence for
this one run — a local, throwaway edit — **not** a candidate change. **The
forcing edit must not appear in the handback diff.** Its only product is the
recorded observation.

# `D1` — what the projection actually carries, and the best-guess repair

`D0` establishes the second layer. `D1` establishes the cause of the first.

1. **Enumerate the roles the template actually carries for this row.** The
   refusal says no step is a `ComputationalMatchCase`. Record what the steps
   *are*. This is a bounded, by-hand pass over one program's projection — it
   terminates because you run out of steps.
2. **Decide which of two things is true:**

    (i)  the projection legitimately HAS a computational consumer, and the role
           it carries is not the one the reverse-find matches
           => the consumer search is over-narrow. Widen it to the role set that
              genuinely denotes a computational consumer. THE BEST GUESS.
    (ii) the projection genuinely has NO computational consumer
           => this shape should not be on the detached-required-consumer path
              at all, and the defect is in whatever routed it there. The repair
              is upstream of this function.

**Attempt `(i)`.** One honest try plus a handback. The guess is written down so
a reviewer can attack it directly: **if the SP-A program's nested
`ResourceBracketResult` matches produce steps under a bracket or constructor
role rather than a match-case role, the reverse-find is asking for the wrong
role and the value is right there under a different name.** If `(i)` turns out
false from the inside, that is `(ii)` measured, and it is a better measurement
than a standalone probe.

**The interpreter is the oracle.** It runs this program to exit 0. Whatever the
native lane concludes about this projection, the interpreter's successful run is
evidence about what the right answer is.

# `D2` — correct the annotation, whatever `D0` and `D1` return

**In the same change.** Both the *"Observed signature, exactly"* block and the
`#[ignore]` reason string. **Correct it to what is observed, not to this node's
name**, and record the SHA it was measured at so the next reader can tell
staleness from disagreement.

# `D3` — report the descent-campaign witness verdict

**This node is load-bearing for something outside itself and `D0` settles it.**

`docs/program/16-recursive-descent-retirement.md` entry `#6g` carried this row
as the descent campaign's **only real-program witness** for the non-constructor
`ComputationalMatch` scrutinee class — *"the sole failure was this node"*. The
evidence **was** the row's refusal text, and the row no longer produces it, so
the claim was **WITHDRAWN 2026-09-18, not refuted**. Trap 1's population is
currently two hand-built values.

⇒ **`D0`'s recorded observation is what restores or retires that witness.** State
the verdict explicitly in the handback:

    scrutinee refusal IS behind the first stop  -> the witness HOLDS; the
                                                   class does occur in a real
                                                   program, at a deeper layer.
    it is NOT                                   -> the witness is RETIRED. Say
                                                   so; do not leave it withdrawn
                                                   and unresolved.

**This is a report, not a licence to reopen `#6g`.** Do not re-run the
cross-crate census: it can only witness the retained lane, because
`SELECTOR_VARIANT_EXCLUSION` and the activation seam are `#[cfg(test)]`. **That
ban covers re-running the census to CLOSE Trap 1. It does not cover reading this
one row's current refusal**, which is exactly what `D0` does and exactly the
check whose absence let the entry go stale (Architect scoping,
`evt_1efjy1jmsg0ry`).

# Acceptance criteria

- **AC-0.** `D0` is run at the implementer's own named base, the verbatim first
  refusal is pasted, and the base SHA is stated. A `(b)` or `(c)` outcome
  discharges the measurement obligation and **stops** the work — a pass, not a
  failure.
- **AC-1.** The observation behind the forced first stop is recorded verbatim,
  and the handback states which of the three `D0` outcomes occurred. A third
  unexpected layer is reported and **not** forced past.
- **AC-2.** The forcing edit is **absent from the handback diff**. A reviewer
  can confirm this from `git diff --stat` against the named base.
- **AC-3.** The roles the template carries for this row are enumerated, and the
  handback states `(i)` or `(ii)` with the evidence.
- **AC-4.** If `(i)`: the widened consumer search is accompanied by a statement
  of **which roles are now accepted and why each denotes a computational
  consumer**. Widening to "any role" is not a repair and fails this AC.
- **AC-5.** Both stale annotation records are corrected and carry the SHA they
  were measured at. A grep for `tree-producing match scrutinee` in the row's file
  returns **zero live hits** outside a block explicitly labelled superseded.
- **AC-6.** The `D3` witness verdict is stated in the handback in `D3`'s own
  words — HOLDS or RETIRED — so the Steward can carry it into `#6g` without
  re-deriving it.
- **AC-7.** The row is differential: if un-ignored, both engines exit 0, the
  freeze outcomes agree, and `assert_freeze_sequence` passes on **each**. A
  native-only green does not satisfy this.
- **AC-8.** No-regression is **green in CI**, never a local `--workspace` run
  (`COORDINATION §12`).

# `D4` — the hard stop

`pair_detached_required_consumer` is a **planner invariant** whose refusal text
says *"please report this compiler bug"*. Relaxing it is not the same kind of act
as fixing a lookup. **If the repair requires the planner to proceed with no
computational occurrence at all — rather than finding one it was failing to
recognise — stop and route to the Architect.** Under `(i)` you are teaching it to
see a consumer that is there; under anything else you are teaching it to run
without one, and that is a design call.

# Contention

`aggregates.rs` also hosts the subject of [[RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT]]
(`ready`, runtime), in a different region and a different function — the
host-result recipe, not the consumer pairing. **Same file, disjoint functions.**
Expect a merge-tree union check rather than a conflict, and do not sequence
behind it. The in-flight `RT-CARRIER-PRODUCER-OCCURRENCE` candidate does **not**
touch this file (measured: `constructors.rs` plus its WP doc).

# What must not happen

- **Do not rename the `id`.** It is cited by the descent campaign among others.
- **Do not let `D0` become a ladder.** One forcing step, named target, binary
  outcome, and a third layer is an answer rather than a rung.
- **Do not repair the row by making the native lane refuse more politely.** The
  assertion is that native and interpreter agree and both succeed.
- **Do not cite this row as the descent campaign's real-program witness** until
  `D3` returns, and **do not delete the claim** either — it is unconfirmed, not
  wrong.
- **Do not re-run the cross-crate census.** See `D3`.

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that re-measured this row.
  **Read `docs/program/evidence/rt-ignored-failing-rows-ledger.md`, not the
  node**; the node is the commissioning document and the ledger is the result.
- [[RT-PROCESS-EXIT-STATUS]] — the sibling row framed the same day, same ledger,
  same drift mechanism, also with both annotation records stale.
- [[RT-HOST-RESULT-ARM-SHAPE-DISAGREEMENT]] — shares `aggregates.rs`. See
  Contention.
- [[RT-SPECIALIZED-ACTIVE-RESUME]] — the cross-crate census this row's witness
  claim came from. Cited for provenance; not to be re-run.
