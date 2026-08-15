---
id: LANG-INTERVENING-LET-FRAME-WEAKENING
title: "An intervening let between an outer match's premise and a nested match reaches install_index_refinements and dies in refine_branch_goal with 'could not classify the branch goal: TypeMismatch' -- and the Architect refused 'orthogonal', because the reported found term carries LANG-CONVOY's own D1 signature and there is an influence path through RVar resolution"
status: ready
owner: language
size: S
gate: none
depends_on: [LANG-CONVOY-MATCH-FIELD-PROVENANCE]
blocks: []
github: null
origin: "language-implementer's bounded section 5 witness attempt (evt_4n7wdytrehs23), routed for separate ownership by language-leader and language-qa. The Architect made the three-way attribution measurement a REQUIRED follow-up of his approval evt_5b3c38r3xrqm6, owned by this filing rather than by a new SHA. Steward-filed per COORDINATION section 2."
---

> # `D1` IS A REGRESSION CHECK ON A JUST-MERGED NODE. RUN IT FIRST, ALONE.
>
> The Architect approved [[LANG-CONVOY-MATCH-FIELD-PROVENANCE]] at
> `dac4d16af7584b68adbcb0ed45109dbd146cf3ba` **with this measurement outstanding**,
> and stated the branch condition himself:
>
> | outcome of `D1` | what it means |
> |---|---|
> | the failure is **invariant across all three** runs | genuinely independent of the merged predicate; this node is a clean pre-existing gap and `D2` proceeds |
> | it **fails only under the region set** | **it is that node's acceptance regression on a merged node, and it returns to the Architect immediately** |
>
> ⇒ **Do not start `D2` before `D1` is reported.** The second row is the reason
> this node exists at all, and it is cheap to settle.

## Why "orthogonal" was refused, and it is not a formality

The implementer reported the failure as *"apparently unrelated"* and hedged it
correctly. The **disposition** rested on that orthogonality, so the Architect
tested it and found the argument insufficient in a specific way worth carrying:

`refine_branch_goal` (`elab.rs:2880-2942`) reads **neither** `match_field_regions`
**nor** `var_refinements` — no site for either falls in that range. **That is a
function-local argument, and the property needed is reachability.** The influence
path he traced:

> region skip → fewer entries in `var_refinements` → **which term an `RVar`
> resolves to changes** (`elab.rs:3403` — a `Cast`-wrapped alias versus the bare
> `Var`) → that term flows into a nested match's scrutinee/indices → **which are
> arguments to `refine_branch_goal`.**

Capability 2 inserts into the map only (`:3020`, `:3080`) and never into
`cx.ctx`, so context **types** are untouched; the path runs through
*resolution*, not through the context. Narrow, but real.

**And the prior is wrong-signed.** The reported `found` term is
`((Dg574 Dg67) @N)` — the same shape as the predecessor's own `D1` signature,
`((Dg574 Dg67) @9)` versus `@4`. That is evidence of the **same defect family**
(an index/frame disagreement reached by a different path), not of an unrelated
gap. **Frame the investigation on that prior, not on independence.**

## Fixed inputs

| input | pin |
|---|---|
| the program | a fresh `let k` bound to a `Vec Nat n` value (**not** an already-refined alias), interleaved between an outer match's premise computation and a nested match, consumed by a further nested match or the recursive call |
| the error | `index refinement: could not classify the branch goal: TypeMismatch`, raised at `crates/ken-elaborator/src/elab.rs:2913-2917` |
| the observation | a temporary `try_reindex_cast` operand trace showed `k`'s weakened raw type and the middle match's `b2` **disagreeing on which absolute position they name** |
| the three bases | `43bd0d597` (predecessor's merge-base), the floor mutation `if abs_pos >= 3`, and the shipped region set |
| `try_reindex_cast` | `elab.rs:2830` — returns `Ok(None)` when `subst_term_generalize(cur_ty, old_idx, new_idx) == cur_ty`, i.e. when `cur_ty` does not depend on `old_idx` at all |

**Re-derive at your candidate base.** The predecessor moved `elab.rs`; these line
numbers are from its review, not from your tree.

## Deliverables

**`D1` — the three-way attribution, and nothing else until it is reported.**
Run the failing program at each of the three bases above. Report the exact error
(or its absence) for each, with the run.

**`D2` — conditional on `D1` reading "invariant".** Locate the disagreement: two
things name an absolute position and do not agree on it. Say which two, and
which one is wrong. **A diagnosis is the deliverable; the repair is sized after
it.**

**`D3` — the reachability question, answered rather than assumed.** The
Architect's influence path is a *possible* route, traced by reading. Establish
whether it is the **actual** route here — if `RVar` resolution is not involved
in this failure, say so with the evidence, because that materially narrows `D2`.

## Acceptance criteria

**`AC-1`.** `D1` is reported with three runs and three results. **A `D1` that
reasons about what the three bases would do fails this** — that is precisely the
error (asserting a mechanism's behaviour from a reading) that produced the
predecessor's recut, twice.

**`AC-2`.** No repair lands before `D2`'s diagnosis is stated. A fix whose
rationale is "this makes the error go away" is not accepted here.

**`AC-3`.** The predecessor's shipped region-stack mechanism is **unchanged**
unless `D1` fires the second row. If it does, stop and return to the Architect —
**do not repair a merged node's regression on the ring's authority.**

**`AC-4`.** No widening to other `install_index_refinements` consumers beyond
what `D2`'s diagnosis names. The predecessor banned that scope and the ban is
what kept this finding honest.

**`AC-5`.** No-regression, in CI (`COORDINATION §12`).

## Why this earns a slot

**It is the closeout of a merge, not new work.** The Architect approved on
soundness with this measurement explicitly deferred to a filed node, and his
reason for not blocking was that the measurement is cheap and the predecessor's
frame forbade the repair. **Leaving it unfiled would convert "cheap and
deferred" into "never run."**

**A finding that lives only in the prose of a test about something else is lost
the first time someone greps for it** — his words, and the reason this is a node
rather than a doc comment.
