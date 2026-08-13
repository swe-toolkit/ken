# V3-KRIPKE-DECOMPOSITION — say what the FO embedding requires and how it cuts into one-hour increments

**Owner: verify. Size: M. Gate: none.**
**This node REPORTS a decomposition. It builds no part of the embedding.**

**HELD `draft`.** Release only after **both** `V3-VERDICT-CENSUS` and
`SEC1-R3-MINIMAL-ROUTE` land, and only if their answers still point here. The
Steward makes that call; **a seat that finds this frame and starts is working an
unreleased node.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`3cfdfdce`.

## Fixed inputs

| fact | site |
|---|---|
| the FO arm, and that it does nothing FO-specific | `crates/ken-elaborator/src/prover.rs:332` — calls `attempt_ipc` unchanged |
| the four deferred pieces, named in that function's own doc | same, `:326-331` |
| the translation and its clauses | `spec/20-verification/23-prover.md:168-196` |
| the two discharge routes, (a) reflective and (b) reconstruction | same, `:198-215` |
| the solver's standing — assumed nothing | same, `:244` |
| the trusted base gains nothing from the prover | same, `:341` |
| the DAG row that has never had a node | `docs/program/05-implementation-dag.md:166` |
| what the two RESIDUAL nodes actually produced | the single `IntLit` refutation arm, `prover.rs:298-300` |

## D1 — enumerate the pieces and their real dependencies

For each of the four — translation `φ ↦ φ#`, the `World` sort with preorder and
monotone forcing, the adequacy lemma `classically_valid(φ#) → φ`, and
`check_cert` soundness — state **what it is, where it would live, and what it
depends on**.

**Dependencies between them, specifically.** The translation looks like the easy
first piece; say whether its shape is constrained by the adequacy lemma, because
if it is then "start with the translation" is wrong and that is the single most
useful sentence this report can contain.

## D2 — assign each piece to a lane, and flag what is not the prover's

**The adequacy lemma and `check_cert` soundness are kernel-facing** — spec
`23 §4` route (a) wants them mechanized once and in the kernel. Say for each
piece whether it is prover work, kernel work, or spec work.

**Anything that lands in the kernel is TCB-adjacent and routes to the Architect
and the operator.** Name it; do not scope it.

## D3 — cut the prover-side work into one-hour increments

For the pieces that are prover work, give **an ordered list of increments, each
sized so an implementer reaches a releasable result or a genuine hard stop in
about an hour**, each with its own fixed inputs and the property it would
establish.

**If the smallest honest first increment is larger than that, say so and say
why.** That is a finding about the work, not a failure of the cut, and it is
what the Steward needs in order to sequence rather than guess.

## Acceptance criteria

- **AC-1 — every piece is assigned a lane** (prover / kernel / spec), with the
  reason. **An unassigned piece is the one that stalls.**
- **AC-2 — the dependency between the translation and the adequacy lemma is
  stated explicitly**, either way. It is the ordering question the whole
  decomposition turns on.
- **AC-3 — increments are sized against the one-hour target and say what each
  would ESTABLISH**, not what it would touch. A list of files is not a cut.
- **AC-4 — route (a) versus route (b) is addressed.** Spec `23 §4` offers
  reflective discharge and SMTCoq-style reconstruction, and names (a) as the
  target with (b) as a feasibility hedge. **Say which this decomposition
  assumes**; they cut differently and a decomposition silent on it is
  ambiguous where it is most expensive to be.
- **AC-5 — nothing is built.** `git diff` under `crates/` is empty.
- **AC-6 — the report says what it does NOT establish**: that V3 should proceed,
  that the embedding is the binding constraint, or that any piece is feasible at
  the cost stated. **A decomposition is a plan for work, not evidence the work
  is worth doing.**

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **a clean cut into prover-sized increments** | The Steward frames and sequences them. **Nothing starts on the strength of this report alone** — the priority call against 4b and the surface work is the operator's. |
| **the first honest increment is larger than an hour** | **A finding, not a failure.** It says V3 is spine work that cannot be entered incrementally, which is exactly what the operator needs before committing a lane. |
| **the adequacy lemma must land in the kernel** | Architect and operator. **TCB growth is not this node's to weigh** and not the Steward's to authorize. |

## Banned scope

- Implementing the translation, the `World` sort, the lemma, or `check_cert` —
  including a "sketch" that would ship.
- Choosing between route (a) and route (b) as a commitment. **Name what each
  implies; the choice is the Architect's.**
- Deciding whether V3 proceeds, or arguing for it.
- Anything about the solver: its adoption, its deferral, or its merits.

## Hard stops — return to the Steward

- **The spec does not determine a piece well enough to size it.** That is a spec
  gap and it routes to the enclave, not to a guess.
- **Every viable cut requires kernel work first**, which would make the whole
  decomposition contingent on a TCB ruling.

## Sequencing and contention

Verify, one lane, after both predecessors. Reads `prover.rs`, `spec/20-verification/23-prover.md`
and the DAG; writes nothing under `crates/`.
