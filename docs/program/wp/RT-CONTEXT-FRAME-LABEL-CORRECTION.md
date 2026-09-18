# RT-CONTEXT-FRAME-LABEL-CORRECTION — work package

- **Node:** `[[RT-CONTEXT-FRAME-LABEL-CORRECTION]]`
- **Owner:** runtime
- **Size:** M
- **Tier:** T1 (section 7)
- **Depends on:** nothing. **This WP is not blocked on any unlanded candidate.**
- **Branch:** `wp/RT-CONTEXT-FRAME-LABEL-CORRECTION-body-unit`

## 1. Objective

Four failing-ignored rows name an owning node that does not exist. Create the
owner, establish the readmission condition their labels record as **UNKNOWN**,
and decide whether `agreeing_recursive_body_unit`'s refusal is correct for
these programs or is reporting an artifact of how body origins are resolved.

Four of the fourteen failing-ignored rows on `main`.

## 2. Fixed inputs, measured

Code read at `origin/main` `2f046723e71be210df86666042fd061dae86b746`. Row
coordinates are the `#[ignore]` **attribute** line. Label content was measured
at `b0421afd0` by its author and is carried, not re-derived.

    px7m_hostresult_computational_match.rs:163    369 versus 332
    px7m_hostresult_computational_match.rs:206    378 versus 341
    px7l_checked_host_recursive_bind.rs:163       343 versus 322
    px7l_checked_host_recursive_bind.rs:241       362 versus 347

**The surviving refusal** — `core.rs:1230-1252`:

```rust
fn agreeing_recursive_body_unit<Unit>(
    declared_units: impl IntoIterator<Item = Unit>,
) -> Result<Option<Unit>, CraneliftBackendError>
where Unit: Copy + Eq + std::fmt::Debug,
{
    let mut declared_units = declared_units.into_iter();
    let Some(expected) = declared_units.next() else { return Ok(None) };
    for unit in declared_units {
        if expected != unit {
            return Err(unsupported("ComputationalMatch", format!(
                "plain Match branches declare different recursive body units: \
                 {expected:?} versus {unit:?}")));
```

**The admission gate, and why it is NOT a repair site** — `core.rs:13577-13584`,
with the comment above it at `:13572-13576`:

```rust
if let Some(frame) = self.function_local.constructed_context_frame.as_ref() {
    if frame.worker_body_origin == body_origin
        && frame.worker_captures.len() == captures
        && frame.context_captures.len() == claims.len()
    { return Ok(true); }
}
```

> "Admission here is a PERMISSION, not the authority. The consumer re-matches
> the frame on the complete planner-issued coordinate key and re-checks both
> cardinalities ... The two cannot disagree silently."

**The producer of the disagreeing units** — `resolve_recursive_unit_body` at
`core.rs:13597`, which on a `Match` scrutinee walks `cases` and collects one
declared unit per case via `case_body_occurrence`.

**Already measured and REFUTED — do not re-open:** the single-`Option`-slot
overwrite reading (the slot is written exactly once per compile on all four
rows), and the `worker_body_origin`-keying reading (keying readmits nothing).
The admission axis is red at **both** ends.

## 3. THE DESIGN JUDGMENT, FRONT-LOADED

**The admission gate is retired as a repair site before this WP starts.** Two
independent things say so: the labels measured both ends of its axis red, and
the code comment says it is a permission backed by a consumer-side authority.
**Do not spend a deliverable re-establishing that.**

What is open is one question with two readings, stated in the node and not
ranked: the branches genuinely declare different units and the requirement that
they agree is too strong, **or** they denote the same body and resolution is
producing two origins for it.

## 4. Deliverables

**D0 — IS `agreeing_recursive_body_unit` THE LAST LAYER? Run this first.**

The labels measured the forced configuration only to its first stop. Force past
this refusal and record what happens next, per row.

> **TERMINATING OBSERVATION, written before the run, and it can come out either
> way.**
>
>     (a) the row COMPILES and runs -- 1230 is the last layer. The whole
>         question becomes whether relaxing it is SOUND, and D1 is a
>         soundness argument, not a search.
>     (b) ANOTHER REFUSAL -- name it, per row. 1230 is one of a queue and the
>         remaining stack is not yet bounded. Say how many layers were seen,
>         and do NOT report the new refusal as "the" cause.
>     (c) the rows DIFFER from each other -- that is a result. Two of these
>         rows are px7m and two are px7l; if the pairs diverge, the four are
>         not one population and this WP's scope is wrong.
>
> **(c) is named because the four rows have been treated as one population on
> the strength of a shared label, and a shared label is not a measured
> population.**
>
> **Bound it the way the labels bound theirs:** forcing past a refusal shows
> the next stop, never an inventory of what remains behind it.

**D1 — DECIDE BETWEEN THE TWO READINGS.** For one row, determine whether the
two disagreeing `StaticOriginId`s denote the same body.

> **TERMINATING OBSERVATION:**
>
>     SAME body, two origins  -> reading (2). The defect is in resolution and
>                                agreeing_recursive_body_unit is faithful.
>     DIFFERENT bodies        -> reading (1). The branches really do declare
>                                different units, and the question is whether
>                                the backend must support that shape.
>
> Say which evidence decided it. "The origins differ" is the premise, not the
> answer — the question is whether the objects they name differ.

**D2 — The four labels.** Rewrite the readmission half of each. The
attribution halves are measured and are not to be touched. Every label must
name an owning node **that exists**.

**D3 — The repair**, at whichever unit D1 selects, or a written statement of
why it is out of this WP's scope with the successor named.

## 5. Acceptance criteria

**AC-0 — The node exists and every one of the four rows names it.** The
routing defect is closed when no row points at a non-existent ID. This is the
one AC that is satisfied by the node file plus four label edits and nothing
else.

**AC-1 — D0 answered per row, with the observation recorded as measured rather
than carried**, and the bound on it stated.

**AC-2 — D1 answered with its deciding evidence named.**

**AC-3 — The admission gate is untouched**, or, if touched, an argument for why
the both-ends-red measurement and the `:13572` comment are both wrong. Either
is acceptable; silently editing it is not.

**AC-4 — The BoundaryCarrier arity refusal is not relaxed.** The labels record
it as correct. A repair that makes these rows pass by relaxing it has moved the
defect rather than fixed it.

**AC-5 — No regression.** Green in CI, per `COORDINATION` section 12. Local
runs are targeted only (`scripts/ken-cargo -p ken-runtime`, or `--test` one
suite).

## 6. What this WP is NOT

- Not the context-frame admission gate. Retired as a repair site in section 3.
- Not `[[RT-CARRIED-RESIDUAL-IH-ARITY]]`, whose two refuted clauses are fixed
  inputs here.
- Not `[[RT-SITEOP-CARRIED-WITNESS]]`. Its D2 is **carried** from `3f4ae2d83`
  and was not re-measured in the current labels. If a deliverable comes to
  depend on it, re-measure it first and say so.
- Not the four rows' passage. If D1 selects a repair larger than this WP, D3 is
  a written scope statement with a named successor, and that is a complete
  outcome.

## 7. Estimated tier: T1

D0 and D1 are structural questions about backend resolution with outcomes that
are not known in advance; D3 is a soundness-bearing decision about whether a
branch-agreement requirement is too strong. The diff may be small and its
review turns entirely on an argument.

## 8. Contention

Code paths are `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` and
the two `crates/ken-cli/tests/` files. Re-measure the intersection against the
routed-and-unlanded set at release; do not inherit this line.

**No dependency on any unlanded candidate.** This WP is deliberately chosen to
be startable while the publisher queue drains.

## 9. Measured outcome

**`D0` and `D1` ANSWERED, 2026-09-18, runtime-implementer. The frame's own
dichotomy is refuted and `D3`'s pricing changes. The authoritative record is in
the node** (`issues/RT-CONTEXT-FRAME-LABEL-CORRECTION.md`, the `ANSWERED`
section); this is the frame-level consequence only.

Measured at `7cb535be5`; `git diff 7cb535be5 origin/main -- crates/` is empty,
so the numbers carry to `9dfa6978e`. All probes env-gated and reverted;
`grep -c RTPROBE` = 0.

### What the frame got wrong

    frame reading (2)        REFUTED in all four rows
    frame reading (1)        true of exactly ONE row (px7m:206)
    frame's population split REFUTED -- it is not px7m versus px7l

The mechanism is neither branch: **the comparison tests node identity where the
property it needs is body equality.** The four rows are one population by shape,
split 3-1 by where the resolved unit sits relative to the arms' divergence, and
the odd row is inside `px7m`.

### The re-pricing, which governs `D2`/`D3`

> **No repair at `core.rs:1230` makes any row pass.** All four stop immediately
> behind it at `RT-CONTSRC-PRODUCER-LOCAL` `D3b` (`core.rs:9558`), same
> construct and same text.

**`D3` remains worth doing and must NOT be reported as progress against the
ignored rows.** A check testing the wrong relation is a real defect; closing it
readmits three rows past `1230` and changes nothing observable about any of the
four.

### The limit that binds any successor

Two forcings revealed two layers. **The stack behind `L2` is NOT bounded** --
forcing past a refusal shows the next stop, never an inventory. Do not frame a
successor as "the last blocker for these rows."

### Two method notes worth carrying, both the implementer's

- The second forcing's free parameter came out **invariant**, and the control
  was shown **not vacuous**: the chosen unit is consumed and changes control flow
  (the disagreement is reached 1 versus 2 times by direction). An invariant
  result from a parameter nothing reads is not a finding.
- The enclosing closures' capture lists are equal in every row and **nothing
  rests on that** -- equal index lists resolved in different environments are not
  equal values.
