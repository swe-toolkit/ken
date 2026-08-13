# RT-4B-ENUMERATION-INPUT-SIZE — record the population enumeration walks, beside the presence flag

**Owner: runtime. Size: S. Gate: none — inside 4b's authorized observation gate.**

**Base: re-derive `origin/main` at cut time.** Fixed inputs measured at
`origin/main` = `89050686576962a2d1ec4d2e977a7f601daedcd2`.

## Fixed inputs

| fact | site |
|---|---|
| the observer, and the struct it fills | `lowering/core.rs:540-560`, called at `:2182-2194` |
| `oriented_present` is a boolean over an `Option` | `lowering/core.rs:2188` |
| the interning loop has **no decline path**, so `keys.len() == candidates.len()` identically | `planning/static_transition.rs:10030-10053` |
| candidate enumeration iterates the **admitted-discovery ledger**, not the oriented plan | `planning/static_transition.rs:10242`, the `for admitted in fusion_root_source_for_future_enumerator(plan)?` loop |
| `OrientedSubcontinuationPlanV1` has four vectors and no single size | `oriented_subcontinuation_plan.rs:152-158` |
| thirteen elimination exits, none distinguished | `static_transition.rs:10254, 10265, 10272, 10279, 10288, 10296, 10301, 10313, 10321, 10324, 10327, 10332, 10343` |

## D1 — record the size of the population that is actually walked

Add to `D2fGateArrival` the **length of the admitted-continuation-discovery
ledger** — the collection the enumeration loop iterates — recorded at the same
production site as the existing fields.

**Record the oriented plan's four vector lengths as well**
(`frames`, `recursive_calls`, `computational_ih_slots`, `computational_ih_calls`),
each named. **Do not collapse them into one scalar.** There is no single
"oriented plan size"; inventing one is how a number gets read as more than it
measures, which is the exact failure this increment exists to stop.

**This is recording only.** No plan changes, no artifact changes, no branch
changes, no new enumeration behaviour. If the read cannot be taken without
changing something, that is a hard stop.

## D2 — re-run the 4b measurement and report the populations

Drive the same real `C2_MIXED_SOURCE` through `compile_native_program_sources`
as gate 4b did, and report every field.

## Acceptance criteria

- **AC-1 — the admitted-ledger length is recorded at the production site**, beside
  the existing fields, from the same call that enumeration consumes. Not
  re-derived, not recomputed from a parallel walk.
- **AC-2 — the oriented plan's four vector lengths are recorded separately and
  named.** No single collapsed scalar.
- **AC-3 — enabled and disabled runs produce identical artifacts.** This is a
  read; if the produced artifact differs, stop.
- **AC-4 — THE AMBIGUITY LIMIT IS STATED IN THE ARTIFACT ITSELF**, not in a
  handback and not in a comment on the frame. A non-empty walked population with
  `keys = []` remains ambiguous across **all fourteen** elimination routes
  (thirteen exits plus an empty admitted ledger) and **must not be reported as
  the first missing producer relation.** This is a constraint, not a note: the
  increment is not done until the artifact says it.

## Pre-stated outcome licensing — read this BEFORE reporting

**Architect, `evt_7011z8x4x2j3d`. Both outcomes are enumerated here so nobody
needs a round trip to find out what their measurement licenses.**

| outcome | what it licenses |
|---|---|
| **walked population EMPTY** | `keys = []` is **fully explained**. 4b is answered: there was nothing to fuse, the planner is not implicated, and the gap is upstream in what 4a produces for this source. **Clean close.** |
| **walked population NON-EMPTY** | **Nothing about the planner.** Not a finding, not a stop against it, not a lawful-absence conclusion. It licenses exactly one thing: **the observation route is exhausted**, and further progress on 4b requires either attribution (a builder change, its own scope call) or waiting for the emitter. |

**The second outcome returns the arc to today's ambiguity having spent an
increment. That is a known and accepted cost of the read, not a failure of it —
and it is not a licence to widen scope to rescue the increment.**

## If attribution is ever needed, it is ONE function — recorded, NOT authorized

**Do not act on this in this node.** Architect, `evt_7011z8x4x2j3d`, recorded so
a future scope call is a one-function question rather than an all-thirteen one:

`fusion_unique_static_body_triple` is the highest-information exit in the table
and the cheapest to attribute. It is the only elimination with **documented
lawful-refusal semantics** — it refuses on *both* absence and multiplicity, with
the reasoning in the code (*"'the only edge' would be an existential and
choosing among several would be a guess"*). Every other exit is a structural
mismatch saying the shape was wrong; **this one says the shape was right and the
edge population was not.**

That distinction exists inside the function and is **destroyed one line later**
at `:10332` — `let Some(...) = ... else { continue; }` collapses both arms
before anything downstream sees them. Recovering it means widening one
function's return type.

**That is a builder change, not recording, and it is OUT of the observation
gate.** If this increment returns a non-empty population and the arc needs to go
further, that is the increment to bring the Steward, **scoped to
`fusion_unique_static_body_triple` alone.**

## Banned scope

- **The per-elimination-point census.** Measured and ruled out: none of the
  thirteen exits is distinguished, so it would require teaching enumeration to
  distinguish them, which is a builder change.
- Widening any function's return type, including `fusion_unique_static_body_triple`.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair.
- Any change to what a plan or artifact contains. Gates 5 and 6 held; production
  unarmed.

## Hard stops — return to the Steward

- **The read cannot be taken without changing behaviour.**
- **The admitted ledger is not reachable at the observation site** without a new
  call or a parallel walk. Do not build one — one invocation, one ledger.
- **The measurement tempts a conclusion the table above does not license.**

## Sequencing and contention

**Runtime, after `RT-CONTSRC-FRAME-FINALIZE`.** One implementer lane. Touches
`lowering/core.rs` (the observer) and reads from
`planning/static_transition.rs`; `RT-CONTSRC-FRAME-FINALIZE` touches
`planning/static_transition.rs`, so they run in sequence rather than together.
