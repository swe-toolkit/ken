---
id: RT-CONTKEY-CONSUMING-OCCURRENCE
title: "The continuation specialization key names the owner of the continuation's own occurrence and has nowhere to name the occurrence that CONSUMES its answer; the enclosing eliminator is measurably not in hand at the interning site, so the fact must be seeded at the outer-match walk and threaded there -- a plan-construction change, not a field addition"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-LEXICAL-RECURSOR-CONSUMERS]
github: null
origin: "Architect ruling evt_3zjhbbr7k3ky6 (2026-08-14) on the RT-LEXICAL-RECURSOR-CONSUMERS D2k frame section 8 hard stop, which fixed the amendment shape and made sizing turn on one probe; the Steward released only that probe (evt_hkqp2pjrknaj) and filed no node until it answered. Runtime measured it at origin/main 0644ab95 (evt_3tkyp322dh4c7, leader confirmation evt_1f4yp49cx23m4): NOT in hand. Steward-filed per COORDINATION §2."
---

> # MERGED 2026-08-14 at `a998d3f6` (PR #2209), exact `16eb2618`.
>
> **Every AC discharged, and the two that existed to catch a wrong answer both
> produced real measurements rather than assertions.** `AC-1`: rows 4 and 5
> carry body `16`/`12` with eliminator `5`, each agreeing with an independent
> direct derivation. `AC-2`: the wrong-own-occurrence seed mutation refuses
> verbatim with *"a continuation specialization's consuming occurrence is not
> the exact outer selected case body derived from its eliminator"*. `AC-3`:
> population is one carrying edge per governed plan, two in the two-row control.
> `AC-6`: rows 4 and 5 **still refuse** at `StaticWorkerBinding` with their prior
> conservation sentence -- no route repair rode along.
>
> **`AC-1` AND `AC-3` ARE WEAKER THAN THEY READ. Adversary `evt_7b75nbgqbw04z`,
> triaged CONFIRMED; follow-up is [[RT-CONTKEY-ELIMINATOR-ORIGIN-UNFIRED]].**
> `consuming_occurrence` has two fields and only `body_origin` is re-derived --
> `eliminator_origin` is **copied from the input** into every candidate before
> the comparison, so `AC-1`'s assert is `x == x` on that field. `AC-2`'s mutation
> perturbs `body_origin` only, so the measured refusal is the scan rejecting a
> wrong body and **step 1 has never been fired**. Nothing is known to be wrong;
> there is simply no evidence that half is right.
>
> **`AC-3`'s "population of two" is one on the axes that matter.** Both rows
> carry `eliminator_origin: StaticOriginId(5)` and
> `consumer_owner: PredeclaredFunctionId(0)`; only `body_origin` differs. They
> are **one eliminator with two bodies**, not two samples -- so a control passing
> for the wrong reason on `eliminator_origin` passes identically on both. Do not
> cite that count as two independent samples.
>
> Architect Decision `dec_7ta70j8a2dqk3`, QA `evt_4v9xz45y4443z`.
> `scripts/ken-cargo test -p ken-runtime --lib`: 928 passed, 0 failed.
>
> **[[RT-LEXICAL-RECURSOR-CONSUMERS]] `D2k-1c` IS NOW UNBLOCKED** -- the relation
> it was waiting for exists.

> # THIS NODE SUPPLIED A RELATION. IT DID NOT REPAIR A ROUTE.
>
> The route repair is `D2k-1c` in [[RT-LEXICAL-RECURSOR-CONSUMERS]], which is
> `active`, already framed in section 5 of its frame, and blocked on exactly the
> fact this node mints. **Do not do both here.** If the relation lands and the
> route repair looks like two more lines, that is still the other node's turn —
> the two have different reviewers' questions, and folding them puts a
> plan-construction change and a lowering change in one candidate.

## Why this node exists rather than an increment on `D2k-1c`

**The probe the Architect made sizing turn on was run, and it came back on the
larger side.** Section 5 of the ruling stated the fork exactly: if the enclosing
eliminator occurrence is already in scope at the specialization-key interning
site, the repair is a field on the key plus a forward write at a site that
already holds both ends — small. If it is not, the fact must be seeded by the
walk that visits the outer match and threaded to the interning site — a
plan-construction change that deserves its own node.

**Measured at `origin/main` `0644ab95`, three planner invocations per row, both
rows agreeing:**

| row | outer scan occurrence `m` | `m`'s `children[0]` | key `continuation_origin` | occurrence in hand AT interning | its `children[0]` |
|---|---|---|---|---|---|
| 4 | `5` | `21` | `21` | `31` | `30` |
| 5 | `5` | `17` | `17` | `27` | `26` |

The outer scan holds the qualifying `m` — `children[0]` equals the key's
`continuation_origin` in both rows, which is the relation the amendment needs.
**At interning that occurrence is gone.** What is in hand there is the current
*producer* occurrence, whose `children[0]` is a different origin entirely. The
two sites are in the same file and the relation is exact at one of them and
absent at the other.

⇒ **Not in hand. The threading is the work, and it is why this is `M` and not
`XS`.**

**Read the identities as anchors to re-find, not as values to check.** They were
measured at `0644ab95` and the deliverable below edits the very code that
produces them. `D0` exists to re-derive them at your base; if they differ, that
is a report, not a reason to adjust the table.

## Fixed inputs — settled by ruling, do not re-derive and do not reopen

Every one of these is from `evt_3zjhbbr7k3ky6`. They are cited by symbol
because the coordinates move; where a number appears it is *"at `0644ab95`,
around ..."* and is an anchor to re-find.

**`F1` — widening or reinterpreting `consumer_owner` is CLOSED, structurally,
and it is the cheapest wrong turn available.** It would present as a one-line
change and it reds its own validator. `exact_continuation_source_environment`
(in `crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`,
at `0644ab95` around `:7005`) refuses unless
`occurrence_authority(plan, continuation_origin)?.owner == consumer_owner`.
**The all-`Fn(0)` equality that made this field look right is an enforced
derivation, not a coincidence you can promote.** The implementer who declined
to promote it was right to; do not re-open that call.

**`F2` — do NOT build on `ContinuationInputProjection`.** It carries
`producer_owner` and `consumer_owner` per *input* and is genuinely
per-occurrence, which is exactly why it is the natural place to look and the
wrong place to build. **Rows 4 and 5 have zero inputs**: arity 1, zero captures,
no ordinary parameters, empty ordinary envelope, empty projection population. A
per-input carrier cannot express a per-edge fact for an edge with no inputs —
adding a field there yields a population of zero rows for precisely the two rows
the repair exists for. This is the dual of the failure this arc already caught
on the producer side (`RT-LEXICAL-R3-FUSION-EMITTER` `DP`).

**`F3` — the carrier is the edge's own key, BESIDE `consumer_owner`, never
replacing it, and its value is an OCCURRENCE coordinate, not an owner.** The
precedent is landed in the same struct under ruling `evt_609am4v7cdt5b`:
`ContinuationSpecializationKey.producer_owner` is provenance-only and
`emission_owner` was added beside it precisely because one field could not carry
two roles. `consumer_owner` now has the identical defect on the consumer half.
**Owner coarseness is what manufactured the near miss** — naming an owner again
reproduces it.

**`F4` — mint the fact FORWARD at the enclosing eliminator. Never reconstruct it
from the continuation.** The relation is only a reverse search if you enter at
the continuation. Entered at the outer match, every link is a forward,
ordinal-indexed read the planner already performs in production:

- **match to scrutinee** — `occurrence_authority(plan, match_origin)?.children`
  filtered on `child.position == 0`, at `producer_local_source`'s
  `CaseArgumentBinder { match_origin }` arm.
- **match to selected case body** —
  `plan.semantic.child_origin(origin, 1 + alternative)`, at the fusion-key
  rebuild, where `selected_case_body` is reached from a continuation origin by
  exactly this call.
- **match to its case emission records** — `plan.case_emissions.iter().filter(...)`
  on `record.match_origin == origin`, in `continuation_result_origins`.

**`F5` — no Kernel block is inherited.** The needed fact is planner-side
occurrence provenance inside the static-transition plan. It is **not** the typed
terminal-All structured-IH elimination relation, so it does not touch
[[RT-TERMINAL-ALL-ELIM-AUTHORITY]] and does not inherit its
[[KERNEL-NESTED-IND]] dependency. Ruled; do not re-derive the classification.

**Treat every anchor above as perishable. If a fixed input turns out false
against the landed code, say so and escalate — do not quietly build around it.**

## Deliverables

**`D0` — re-derive the two rows' identities at your base, before anything
else.** Print the outer scan's `m` and its `children[0]`, the interned key's
`continuation_origin`, and the occurrence in hand at interning, for rows 4 and 5.
This is the probe re-run and it is cheap. **If the enclosing eliminator turns
out to BE in hand at your base, stop and report** — that is `D2k-1c` being small
after all, and it changes which node owns the work.

**`D1` — seed the relation at the walk that visits the outer match.** At the
outer eliminator, where `m` is in scope and `children[0]` is the continuation
origin, record the consuming occurrence: the outer selected case body, reached
by `child_origin(m, 1 + alternative)`, together with the eliminator `m` through
which it consumes. **Both, as origins.** The eliminator alone does not name the
consumer and the body alone does not name the path.

**`D2` — thread the seeded fact to the specialization-key interning site.**
This is the plan-construction half and the reason for the node. **The threading
must not become a lookup keyed on the continuation origin** — a map from
continuation to eliminator is the reverse relation `evt_609am4v7cdt5b` forbids,
wearing a data structure. If the only shape you can find is such a map, that is
a stop, not a design choice.

**`D3` — the new field on `ContinuationSpecializationKey`, beside
`consumer_owner`.** An occurrence coordinate, not an owner. Follow
`producer_owner`/`emission_owner`'s landed shape, including their doc discipline:
`producer_owner` carries an explicit provenance-only warning, and the new field
needs the converse — what it *does* confer.

**`D4` — a doc clause on `consumer_owner` recording that it is NOT this
relation and why widening it is closed.** Name
`exact_continuation_source_environment` by symbol and say that it validates the
equality and fails closed, so the field cannot be made to name the outer-case
relation. **This is the clause that stops the next reader making the one-line
change `F1` warns about**, and a symbol is the durable citation — do not write a
line number.

## Acceptance criteria

**`AC-1` — the field is populated for rows 4 and 5, and its value is verified
against an INDEPENDENT derivation of the consuming occurrence.** Print the
field alongside the outer selected case body derived directly from the plan by
`child_origin(m, 1 + alternative)`, and show they agree. **A field that is
merely non-empty discharges nothing** — the near miss this whole arc turned on
was a field that was populated, plausible, and the wrong relation.

**`AC-2` — a positive control that reds.** Mutate the seeding to record the
continuation's own owner or its own occurrence instead of the consuming one, and
show a named test fails. Report the verbatim failure. **If the mutation passes,
the AC is vacuous and that is the finding** — it would mean nothing reads the
field yet, and an unread field is not a landed relation.

**`AC-3` — the population is non-zero and you say what it is.** Report how many
edges carry the new field at your base and confirm rows 4 and 5 are among them.
`F2` is the reason this AC exists: the obvious carrier had a population of zero
for exactly these rows, and the same trap is available on any carrier chosen
here.

**`AC-4` — `consumer_owner` is unchanged in meaning, in validation, and in
value.** `exact_continuation_source_environment` still refuses on inequality and
still passes on rows 4 and 5. Show it. **If the new relation requires relaxing
that validator, stop and report** — that is `F1` being refuted, which is an
Architect question, not a repair.

**`AC-5` — no reverse map, and the review can tell.** State, per link, which
forward read supplied it (`F4`'s three are the whole vocabulary). **A parent
map, a scan over occurrences to find one whose child matches, or a cached
continuation-to-eliminator table all fail this AC** regardless of how they are
spelled.

**`AC-6` — rows 4 and 5 still refuse, with the same refusal.** This node does
not turn them from refuse to consume; `D2k-1c` does. A row that starts consuming
here means the route repair rode along uninvited, which is scope, not a bonus.
Report the refusal text unchanged.

**`AC-7` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build and test targeted: `scripts/ken-cargo test -p
ken-runtime --lib` plus your focused suite.

## Excluded scope

- **`ContinuationSourceSlotAuthority` and the [[RT-CONTSRC-CALLABLE-CONTRACT]]
  edge.** Architect limit 3, and the ruling restates it: if this node finds
  itself needing that surface to express the fact, **that is a second stop, not
  a widening.** Return it the way `D2k-1c`'s stop was returned.
- **The route repair itself** — turning rows 4 and 5 from refuse to consume.
  That is `D2k-1c`. See `AC-6`.
- **Row 1.** It is blocked by this same absent relation (measured,
  `evt_1f4yp49cx23m4`) and it *also* refuses earlier at `NativeJoinPlanV1` for
  *"terminal answer has no affine checked-root authority"*. **That second
  dependency is separate and this node does not supply it.** Do not scope row 1
  in, and do not treat this node's landing as unblocking it.
- **Row 3's singular-specialization wall**, retirement, lane deletion, and any
  landed `D2f` partial.

## Stop conditions — return to the Steward, do not decide

- **`D0` finds the eliminator IS in hand.** Wrong node; report and stop.
- **The only threading available is a continuation-keyed lookup** (`D2`).
- **The relation cannot be expressed without `ContinuationSourceSlotAuthority`.**
- **`AC-4` cannot hold without relaxing the validator** (`F1` refuted).

## Sizing

**`M`.** The threading is a plan-construction change through code that currently
has no reason to carry this fact, and the Architect declined to price it below
that. **A hard stop inside an hour is a good outcome here and several of the
stops above are live** — this arc has fired four of them and every one was
correct.

**Contention:**
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs`,
shared with [[RT-LEXICAL-RECURSOR-CONSUMERS]], which is `active` but stalled on
this node and holds no candidate. Re-derive the intersection at candidate time.
