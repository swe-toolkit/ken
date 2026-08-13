# RT-4B-UNIQUENESS-GATE-REACH — count arrivals at exit 12 before building anything that classifies them

**Owner: runtime. Size: S. Gate: none — inside 4b's already-authorized
observation gate (Architect, `evt_5gck3qg72xe37`). No exception needed.**

**Base: re-derive `origin/main` at cut time**, after `RT-4B-ENUMERATION-INPUT-SIZE`
(`81f46822`) lands. Fixed inputs measured at `7c080543` plus that candidate.

## Fixed inputs

| fact | site |
|---|---|
| the gate, and its sole call site | `planning/static_transition.rs:10099`, called at `:10332` |
| the refusal that collapses both arms | `:10122-10126` — `if matching.len() != 1 { return Ok(None) }` |
| the observer to report through | `D2fGateArrival`, `lowering/core.rs:540-560`, filled at `:2182-2194` |
| the population under investigation | four admitted discoveries in, `keys = []` out — `(4, 2, 0, 2, 1)` |
| exit 12 of 13; eleven exits precede it | `:10254, 10265, 10272, 10279, 10288, 10296, 10301, 10313, 10321, 10324, 10327` |

## D1 — count arrivals at the call site

Record **how many candidates reach `fusion_unique_static_body_triple`** at
`:10332`, and report it through the existing `D2fGateArrival`.

**A counter at the call site. No signature change, no control-flow change, no
plan change, no second observer.** If it cannot be taken that way, that is a
hard stop — it means this is not the node the Architect authorized.

## D2 — re-run the 4b witness and report

Drive the same real `C2_MIXED_SOURCE` through `compile_native_program_sources`
and report the count beside the existing fields.

## Acceptance criteria

- **AC-1 — the count is taken at the call site**, not re-derived from a parallel
  walk and not inferred from any other field.
- **AC-2 — enabled and disabled runs produce identical artifacts**, proven by
  identity where identity is available. **This is a read. If the produced
  artifact differs, stop.**
- **AC-3 — the count is reported through `D2fGateArrival`.** No second channel
  and no parallel recorder — the Architect refused that proliferation when he
  ruled the observer already existed.
- **AC-4 — a mutation reds the exact new value, not a proxy.** Forcing the
  natural read must fail on the count itself. **A recorded zero and an
  unrecorded zero are the same artifact without this**, which is the entire
  reason this node exists.
- **AC-5 — the report states the licensing limit in the artifact itself.** Reach
  attributes nothing: it cannot see which arm fired, and it must not claim to.

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **reach = 0** | **Exactly one thing: the eliminations on this witness are upstream of exit 12.** Fourteen routes narrow to eleven. It attributes **none** of them, does **not** reopen the fourteen-exit census, and does **not** change 4b's status — still exhausted, now with one route excluded rather than zero. **This is a real result, not a failed increment.** |
| **reach > 0** | `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` has a subject and becomes lawful. |

> **Neither row licenses a finding against the planner, and neither licenses
> "the uniqueness gate ate our candidates."** That sentence has no true reading
> available from this node.

**If reach is zero, do not build the attribution increment. Bring the zero
instead** — the Architect will close that node rather than kick it.

## Banned scope

- **Widening any function's return type.** That is the conditional successor.
- Attributing absence versus multiplicity; improving, relaxing or reordering
  either arm.
- The fourteen-exit census — ruled out, staying out.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **The count cannot be taken without a signature or control-flow change.**
- **The artifact differs between enabled and disabled.**
- **The mutation cannot red the count specifically.**

## Sequencing and contention

Runtime, one lane, after `RT-4B-ENUMERATION-INPUT-SIZE` merges. Touches
`planning/static_transition.rs` and `lowering/core.rs` — the same files that
candidate touches, so it follows rather than runs beside it.
