# `RT-LEXICAL-RECURSOR-CONSUMERS` `D2l` — row 3's singular-specialization wall

The sixth and last of `#6d`'s remaining expressions. `D2k` owns the other five
and excludes this one in terms: *"Row 3's singular-specialization wall. Same
node, different increment."*

Fixed inputs derived at `main` **`154ce760`**. Re-derive them at your base;
a merge-base goes stale without your branch moving.

**`#6d` closure gates [[RT-RECURSOR-TRANSPORT]] `D3`, which gates
[[RT-DESCENT-RETIRE]]** — the operator's stated priority. If this increment is
large, that is a schedule fact worth knowing early, which is most of why the
frame is written now rather than when `D2k` closes.

## 1. What this increment owns

| cell | expressions | wall |
|---|---|---|
| row 3 | 1 | singular-specialization hard stop |

Rows 1, 4 and 5-after-hole are `D2k`. Row 2
([[RT-LEXICAL-ROW2-MISSING-MINT]], merged) and row 5's before-hole
([[RT-LEXICAL-R3-FUSION-EMITTER]]) have left `#6d` entirely.

## 2. Fixed inputs — the wall has ONE production emitter

`build_continuation_case_binder_run`'s segment 1, at
`crates/ken-runtime/src/cranelift_backend/lowering/units.rs:1249`:

```rust
for position in recursive_positions.iter().rev().copied() {
    if position != worker_position {
        return Err(backend_module(format!(
            "the selected case has a recursive position {position} that the continuation \
             specialization projects no worker for, so its induction-hypothesis prefix cannot \
             be built"
        )));
    }
    run.push(ContinuationCaseBinderSource::InductionHypothesis);
}
```

**`"projects no worker for"` is emitted from this site and no other**
production site. The committed `D2b` control asserts row 3's rendered result
contains that string — `control.rs:30576` (`const HARD_STOP`) and
`control.rs:30610`, inside
`d2b_the_abandoned_let_body_joins_are_dispositioned_at_the_arm_that_abandons_it`.

⇒ **Derived, from one emitter and one committed assertion:** row 3's selected
case carries a recursive position that is **not** the ruled `worker_position`,
and the continuation specialization projects a worker for one position only.
The adjacent comment states the design intent plainly — *"`D6a` deliberately
does not generalize to a multi-worker population"* (`units.rs:1287`).

> **I did not run row 3.** The above is a derivation from a single emission
> site plus a committed control's assertion, not a measurement. `D2l-0` exists
> to measure it. If your measurement disagrees with this section, **your
> measurement wins and the frame is wrong** — post it and stop.

Row 3's fixture and activation, from that same control:
`host_result_closure_match(px8j_recursive_sibling_result(1, 2,
px8j_aggregate_result()))`, run through `px8j_capture_source_trace` under
`RecursiveDescentResidual::LexicalCallArgumentRecursor` exclusion.

## 3. The design judgment that is NOT front-loaded, and why

**This section deliberately does not tell you the repair.** `D2k` shipped a
front-loaded judgment marked *"do not re-derive it"*, and it was measured false
at the first contact with the population — the instruction works by
discouraging exactly the check that would have caught it. So what follows is
the **shape of the question**, not its answer.

**The wall is a deliberately retained hard stop, not a defect.** `D2b`
explicitly kept it — *"row 3 still does not compile; it advances to the
singular-specialization hard stop, which this deliverable keeps"* — and four
committed controls take the singular-specialization model as their subject, one
of which exists solely to assert this stop.

⇒ **The question `D2l-0` answers is which of these is true:**

1. **Row 3 has a lawful repair that leaves the stop standing** — the recursive
   position it presents is wrong for a reason upstream of the specialization,
   and fixing that makes the position the ruled `worker_position`. Then this is
   an ordinary increment and the stop keeps doing its job.
2. **Row 3 requires the specialization to project workers for more than one
   recursive position.** Then the repair is a plural-specialization build,
   `D6a`'s stated non-goal, and it is a **graph amendment, not an increment.**

**Do not assume 2 because it is the more interesting answer, and do not assume
1 because it is the cheaper one.** Both are live at the evidence available.

## 4. THE TRAP — the `R2` root record measures a wall that no longer exists

`docs/program/wp/RT-LEXICAL-RECURSOR-CONSUMERS-R2-R3.md` carries a careful root
measurement of row 3, taken at `760a0eff`: operand representation `Closure`,
refusing arm `Lowered::Closure | DeclarationClosure`, first missing owner
`call_declared_unit_target`, route `claim_and_call_resolved_continuation` into
a declared-unit call argument.

**`D2b` removed that wall.** The node's own issue record states it: *"`D2b`
removed the `R2` closure/ordinary-ABI misclassification; advanced to its
retained singular-specialization wall."*

⇒ **That record is not withdrawn — it was true of the tree it was taken on —
and it is not the current wall.** It is the most detailed row-3 artifact in the
corpus and it is the one most likely to be found and designed against. **Read
it as history. Design against your own measurement.**

## 5. Deliverables

**`D2l-0` — measure row 3's current wall and settle section 3's fork. Evidence
only; no repair.**

- The exact refusal at your base, with the position value, the ruled
  `worker_position`, and the `StaticOriginId` the case belongs to.
- **Whether the presented recursive position is wrong upstream, or genuinely
  plural.** State which of section 3's two answers holds and what measurement
  settles it. *"Probably plural"* is not an answer; *"I could not tell, and
  here is what would tell"* is.
- **The four singular-specialization controls, named** — `file:line` each — and
  for each, whether the candidate direction would red it. A control this repair
  must red is not thereby a wrong repair; a control it reds **by surprise,
  after landing,** is a defect.
- **`d2b_the_abandoned_let_body_joins_are_dispositioned_at_the_arm_that_abandons_it`
  must stay green throughout.** It asserts the advance *positively*: a row that
  starts failing earlier stops rendering the join refusal too, and that is a
  regression wearing the shape of a fix.

**`D2l-1` — the repair. NOT framed and NOT released.** It is framed after
`D2l-0` answers, because its size and its owner both depend on that answer. If
the answer is section 3's case 2, it is not framed here at all — it returns to
the Steward and the Architect as a graph amendment.

## 6. Acceptance criteria

**`AC-1` — the measurement is committed**, not posted. The position, the ruled
`worker_position`, the origin, and the fork verdict live in the tree.
*Control:* the committed record.

**`AC-2` — every component of any new assertion is compared against a
LITERAL**, never against a population. `D2k-1a`'s anchor is the pattern: a
sameness check across rows is green under a uniform move, which is the case
that matters most. *Control:* read the assertion; the expected side is
literals.

**`AC-3` — `crates/` production behaviour is unchanged.** Instrumentation is
`#[cfg(test)]`-gated, as in `D2k-1a`. *Control:* `git diff` shows every new
production-file hunk under a `cfg(test)` gate.

**`AC-4` — zero new `#[ignore]`**, and no tracker `status:` change in the
candidate. *Control:* `git diff`.

**`AC-5` — CI green** on the merge. Not a local `--workspace` run
(`COORDINATION §12`).

## 7. Excluded scope

- **The repair.** `D2l-0` is evidence only.
- **Lifting or weakening the singular-specialization stop**, and editing any of
  its four controls. If the answer is that it must be lifted, that is the
  finding — stop and hand it back.
- **`D2k`'s five expressions** and the constructor-field distinction.
- **Retirement and lane deletion** — [[RT-RECURSOR-TRANSPORT]] and
  [[RT-DESCENT-RETIRE]].
- **Touching [[RT-LEXICAL-R3-FUSION-EMITTER]]'s fusion machinery.**

## 8. Stop conditions — return to the Steward, do not decide

- **The repair requires the continuation specialization to project workers for
  more than one recursive position.** That is `D6a`'s stated non-goal and new
  planner population. **Stop. This is a good outcome and a real finding** —
  carry the measurement that establishes it.
- **The measurement contradicts section 2.** The frame is wrong; post and stop.
- **Row 3's wall has moved again** since `D2k`'s authoring and is no longer the
  singular-specialization stop. Post the new wall and stop for sizing.
- **The repair cannot be expressed without editing one of the four
  singular-specialization controls** to keep it green.

> **The handback states the RUNNING TOTAL, not this increment's delta** — new
> planner population, continuation-source setters, ABI/carrier/descriptor
> entries, `#[ignore]`s and `value_at` callers, summed across every `#6d`
> increment landed so far, `D2k`'s included. The stop above is about an
> accumulating quantity, and a per-increment reading of it can never fail: the
> parent node's identical stop sat silent through eleven merged partials and
> fired only once a handback carried a total.

## 9. Contention and sizing

`crates/ken-runtime/src/cranelift_backend/lowering/units.rs`, `.../core.rs`,
`.../mod.rs`, and `.../core/tests/control.rs` — **the same file set as `D2k`
and as [[RT-LEXICAL-R3-FUSION-EMITTER]].**

**Sequence after `D2k`.** Runtime is a sequential ring and these three compete
for the same files; re-derive the intersection at candidate time regardless.

`scripts/ken-cargo test -p ken-runtime --lib` plus your focused suite. **Never
`--workspace`** — that is CI's gate, and `AC-5` means green in CI.

**Sizing.** `D2l-0` is one turn and should land well inside one: it is a
measurement against a wall with a single emitter and a committed control that
already names it. Per `§4b`, a hard stop inside an hour is a good outcome — say
so and hand back. **`D2l-1` is not sized here on purpose**, because section 3's
fork decides whether it is an increment or an amendment.
