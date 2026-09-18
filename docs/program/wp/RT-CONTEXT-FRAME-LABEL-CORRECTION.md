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

### `D2` — the four labels, rewritten. `AC-0` and `AC-1`

**Readmission half rewritten on all four; attribution halves untouched**, as
section 4 requires. Each label now carries, for its own row: the measured next
stop with its own operands, the layer count and the explicit statement that
the stack behind `L2` is **not** bounded, the non-vacuity of the two-direction
control with that row's own 1-versus-2 counts, and what the two origins denote
measured by referent.

    px7m:163   owner 5, binding 395, env 391. Bodies BYTE-IDENTICAL, 558 B.
    px7m:206   owner 5, binding 409, env 405. Bodies DIFFER, 1164 vs 1168 B,
               in one leaf. THE REFUSAL IS CORRECT ON THIS ROW.
    px7l:163   owner 3, binding 358, env 356. Bodies BYTE-IDENTICAL, 152 B.
    px7l:241   owner 3, binding 377, env 375. Bodies BYTE-IDENTICAL,  92 B.

**`AC-0` is satisfied and was satisfied before this WP touched anything**: all
four labels already named `RT-CONTEXT-FRAME-LABEL-CORRECTION`, and the node now
exists on `main`. The routing defect closed when the node was filed; what `D2`
adds is that the label's content is no longer a prediction the node has since
tested.

### The acceptance criteria, enumerated BY POSITION

**Corrected: an earlier revision of this section discharged "`AC-1`: no row
readmits".** That is `[[RT-DUPLICATED-RESPONSE-BLOCK]]`'s `AC-1`, not this
frame's. Reading them off section 5 in order instead:

**`AC-0` — SATISFIED.** Above.

**`AC-1` — D0 answered PER ROW, measured rather than carried, with the bound
stated. SATISFIED.** Each of the four labels carries its own next-stop
operands (`owner`/`binding`/`environment`, all four triples distinct), its own
1-versus-2 non-vacuity counts, and the explicit sentence that the stack behind
`L2` was not forced and is **not bounded**. Nothing in the rewritten half is
carried from `b0421afd0`; the carried attribution halves are untouched and are
marked as carried.

**`AC-2` — D1 answered with its DECIDING EVIDENCE named. SATISFIED.** The
deciding evidence is the rendered body at each origin, compared byte for byte,
and it is named as that in every label. "The origins differ" is recorded as
the premise, not the answer. The one row where the bodies differ has the
differing leaf quoted.

**`AC-3` — the admission gate is UNTOUCHED.** `grep -c RTPROBE` over
`cranelift_backend/lowering/core.rs` is zero and the file is not in this
candidate's diff. No argument against the both-ends-red measurement or the
`:13572` comment is offered, because none is needed.

**`AC-4` — the `BoundaryCarrier` arity refusal is NOT relaxed.** Untouched.
`D3` lands zero production change, so no row is made to pass by moving it.

**`AC-5` — no regression.** Workspace-green is CI's, per `COORDINATION`
section 12. The local targeted check is stated with the run beside it in the
`AC-5` note at the end of this section.

### `D3` — the repair is OUT OF SCOPE, and the reason is not cost

`D1` selects the unit: the comparison at `core.rs:1230` tests **node identity**
where the property it needs is **body equality**. The obvious repair is to
compare bodies. **I am not landing it, and the reason is not that it is big —
it is small.**

**The returned unit is not a boolean. It is an occurrence coordinate that is
carried onward.** Read at `9dfa6978e`:

    core.rs:13490   recursive_position_unit_body -> Option<StaticOriginId>
    core.rs:14411   passed as the last argument of make_computational_recursor
    mod.rs:12398    recursive_unit_body: Option<StaticOriginId>

So "the bodies are equal, therefore either origin will do" **does not follow**.
Equality of the body is not equality of the occurrence, and what `1230` returns
names an occurrence that the induction hypothesis then carries.

**And my own `D0` data says the choice is not inert.** The two forcing
directions reach the disagreement a different number of times — 1 versus 2,
inverted between `px7m` and `px7l` according to which unit the constructed
frame is keyed to. **If the two origins were interchangeable the choice would
be inert. It is not.** That is measurement, not analogy.

**The analogy, stated separately because it is an argument and not a
measurement.** `[[RT-DUPLICATED-RESPONSE-BLOCK]]` `8.9a` carries an Architect
ruling of exactly this shape, on a sibling map: two entries agreed on their
operation and differed on their origins, and **relaxing the producer's key
alone — while the consumer still selects by the coarse key — converts a loud
refusal into a silent mis-route.** Here the coarse relation is body equality
and the fine one is occurrence identity, and nothing downstream of `1230`
re-checks which occurrence was chosen. **A repair that accepts either equal
body is the same move.**

**Third, and independently sufficient: it buys nothing observable.** All four
rows stop at `RT-CONTSRC-PRODUCER-LOCAL` `D3b` immediately behind `1230`.

⇒ **`D3` is a written scope statement.** The defect is real and should be
repaired; the repair must give the consumer of `recursive_unit_body` a reason
to accept one of two equal bodies, or make the two indistinguishable to it.
**That is producer-and-consumer work, not a predicate swap at `1230`.**

**Successor: no node owns this, and I am not filing one** — node filing is the
Steward's under `COORDINATION` section 2. What it would have to establish:

    1. what make_computational_recursor's recursive_unit_body coordinate is
       USED FOR downstream, and whether two structurally equal bodies at
       different occurrences are distinguishable to any of those uses
    2. if they are NOT, relaxing 1230 to body equality is sound and small
    3. if they ARE, the repair is to carry the discriminating coordinate, and
       the sibling ruling's clause-3 shape applies verbatim

`[[RT-POST-L2-RESIDUE-DEPTH-READ]]` is **not** that node — it owns the layer
BEHIND `1230`, not `1230`'s relation.

### `AC-5` — the local check, with its target selection stated

This candidate's whole `crates/` footprint is **four `#[ignore]` attribute
lines**, which are string literals inside test files. So the check that
matters is one that **builds and runs the test binaries**:

    ken-cargo test -p ken-cli --test px7m_hostresult_computational_match \
                              --test px7l_checked_host_recursive_bind -- --list

**`cargo check` would not have been evidence** — it does not compile
`#[cfg(test)]`, so a green `check` says nothing about whether a test built.
The touched set is two `crates/ken-cli/tests/` files and `ken-cli` is a leaf
here, so the reverse-dependency closure is those two targets and nothing else.
Workspace-green is CI's, per `COORDINATION` section 12.

**Result: `exit=0`, 5 tests listed** — 2 from
`px7m_hostresult_computational_match` and 3 from
`px7l_checked_host_recursive_bind`. Both binaries built and ran. The escaped-
literal risk in `D2`'s rewritten labels is closed.
