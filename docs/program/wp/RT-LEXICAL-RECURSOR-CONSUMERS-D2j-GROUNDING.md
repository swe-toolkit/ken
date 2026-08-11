# RT-LEXICAL-RECURSOR-CONSUMERS D2j — grounding for the provenance matrix

Node: `docs/program/issues/RT-LEXICAL-RECURSOR-CONSUMERS.md`. Frame:
`RT-LEXICAL-RECURSOR-CONSUMERS-D2j.md`. Measurement-led first turn: this
records the rows and the fixture economy, and builds no matrix.

## 0. Provenance

Merge-base re-derived at the moment of writing; state it from `origin/main`
rather than from this record.

**No production or control logic change.** Every measurement below was taken
with temporary instrumentation, which is removed.

**The one `crates/` delta is the provenance-comment correction** in
`planning/static_transition.rs` — `+24/-4` relative to the checkpoint base,
comments only. It corrects two overclaims about how `recursive_position` is
established; it changes no behaviour and no control.

An earlier revision of this section said *"No `crates/` change"* and *"the
planner sources are byte-identical to the base"*. That was true when the
checkpoint was record-only and became false the moment the recut touched a
comment — a provenance claim about a candidate goes stale as soon as the
candidate grows, and this one described the commit before it rather than the
range.

> **The same staleness has now happened again, and this note is the fix rather
> than a rewrite of the numbers above.** Every figure in this section describes
> the *grounding* commit. **The candidate has since grown to carry the matrix
> itself, the five refusals and the segment-owner comparator**, so the `+24/-4`
> comments-only delta describes one commit of the range and not the range.
> Read this whole record as the measurement turn it was; take the candidate's
> shape from the handback and its base from `origin/main`.

## 1. The stop condition that mattered — a non-empty projection is REACHABLE

The frame names three stops. The live one was whether a non-empty ordered input
projection is constructible at all, because `D2h`'s witness has an empty one and
that is why its `continuation_inputs` row could not be exercised.

Measured by instrumenting `exact_continuation_source_environment`'s successful
return and running the whole `-p ken-runtime --lib` suite with `--nocapture`:

| projection size | observations |
|---|---|
| 0 | 308 |
| **1** | **813** |
| **2** | **1225** |
| **3** | **12** |

**2360 observations; the empty projection is the minority case at 13%.** A
non-empty projection is not exotic — it is what the planner ordinarily produces,
and arities up to three occur.

⇒ **The stop does not fire.** Nothing here needed `continuation_result_origins`
widening and no row reached for an eighth fact.

**Population, stated in the claim:** these are *all* successful projections in
the lib corpus, overwhelmingly from continuation-specialization planning. They
establish that the projection machinery yields non-empty runs; they do **not**
establish that a *fusion candidate* reaches one. That is the next measurement
and it is the fixture-economy question below, not a stop.

## 2. The authoritative planner fact per closed-seven member

| # | key member | authoritative planner fact |
|---|---|---|
| 1 | admitted discovery context | `fusion_root_source_for_future_enumerator` — the production admitted ledger |
| 2 | producer construct origin | membership in `continuation_result_origins(admitted.result_root)` |
| 2 | producer owner | `occurrence_authority(construct).owner` |
| 2 | producer alternative | `case_constructor_identity(consumer, alt)` matched against `constructor_symbol_identity(construct)`, required to select exactly one case |
| 2 | recursive position | the case's own declared `recursive_positions` |
| 2 | producer argument origin | `semantic.child_origins(construct)[position]` |
| 2 | producer argument binding | `build_checked_ih_bindings` |
| 3 | selected case-body origin | `semantic.child_origin(consumer, 1 + alternative)` |
| 3 | consuming `Call` | `fusion_through_checked_wrappers(selected_case_body)` |
| 3 | consuming callee | `semantic.child_origin(call, 0)` |
| 3 | consumer binding | `build_checked_ih_bindings` |
| 4 | checked transport coordinate | `build_checked_transport` against the validated oriented plan |
| 5 | unique `StaticBody` triple | `semantic.static_body_call_edges(&plan.edges)`, required unique |
| 6 | owner split / result edge | `occurrence_authority` on both sides, plus the §2 membership above |
| 7 | ordered input projection | `exact_continuation_source_environment(..).inputs` |

### 2.1 One row carries a qualification, and it is not a full independence

**`recursive_position`.** The landed `rederive_fusion_key` reads
`key.consumer_binding.recursive_position` to select the position, *before*
`consumer_binding` has itself been established from the plan.

The case-declaration check is real — the position must appear in the case's own
`recursive_positions` — but its independence from raw key data is
**conditional**: it holds only because `consumer_binding` is independently
established further down and then compared in the final whole-key equality.

⇒ **The row states the authoritative fact as the case declaration, and states
the conditionality explicitly.** It is not claimed as an unconditional
independent derivation, and `D2j`'s matrix must not record it as one.

## 3. Fixture economy — one witness covers many rows

The rows are not one fixture each. They partition by what a perturbation has to
change:

- **rows 2, 3 and 6** are all selected by the producer/consumer pairing, so one
  witness that reaches a fusion candidate exercises the construct origin, the
  owners, the alternative, the position, the argument, both bindings, the case
  body and the call;
- **row 4** needs the oriented plan, which the landed twin already carries;
- **row 5** needs the `StaticBody` edge population, already present;
- **row 1** needs the ledger, already present;
- **row 7** is the one that needs something the current witness does not have.

⇒ **The economy is close to one additional witness, not twenty-two.** What is
outstanding is a fusion-reaching witness whose projection is non-empty; §1 shows
such environments are ordinary, so the question is arranging one at a fusion
candidate rather than whether the planner can produce one.

## 4. Scope

No `D2h` re-productionization and no change to its interner-unit controls. No
`D2f`, `R3`, ABI, emission, edge or traversal work. No matrix built, and no
fixture authored **in the turn this record describes** — it grounds them, and
later commits on the same branch build them.

Provenance: Ken-owned frame, tracker, rulings and this repository's own source
and measurements only. No `local/refs/`, permissive, copyleft or
excluded-prototype contact.
