---
id: RT-DESCENT-LANE-COMPLETENESS
title: "Is the functionized lane a complete replacement for RecursiveDescent, or has it been carrying only the ported subset? D2c refused NINE programs the retiring lane compiles, across FOUR independent constructs -- a pattern, not a missing case, so this is a lane-completeness question and not a port"
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Architect ruling evt_7qtgrtwv76vke, 2026-08-16, on runtime-leader's corroboration and construct inventory evt_6bvnv6t4teech: the D2c reds are four distinct refusing constructs across nine programs, the artifact hypothesis is closed, and the successor is a lane-completeness node rather than a missing port. Node cut assigned to the Steward in that ruling; D1 soundness retained by the Architect. Steward-filed per COORDINATION section 2."
---

Frame: `docs/program/wp/RT-DESCENT-LANE-COMPLETENESS.md`. Read it before
pulling anything here — this node's shape is the whole point of it.

## Why this node exists

[[RT-DESCENT-RETIRE]]'s `D2c` rerouted `select_body_emission_authority` to never
return `BodyEmissionAuthority::RecursiveDescent`, deleting nothing. It reded 17
of 943. Fourteen were inside the frozen `D2b` set; **nine of those fourteen are
the surviving lane refusing a program the retiring lane compiles, across four
independent constructs.**

**Four separate representability gaps is a pattern, not an omission.** The
question is therefore not *"add the missing case"* but **whether the
functionized-units lane is a complete replacement for `RecursiveDescent`, or has
been carrying only the ported subset.**

## The artifact hypothesis is CLOSED

The identical `UnsupportedLowering` / `StaticWorkerBinding` — same constructor
origin 36, same static worker field 0, same origin 35, same recognition 2 —
reproduces at **untouched base `c98f72ba8`** through the **pre-existing**
exclusion mechanism, touching no production code (runtime-leader,
`evt_6bvnv6t4teech`).

⇒ **Two independent instruments, one of which does not involve `D2c`'s edit at
all. The finding is about the lane, not about the reroute.** That was the one
way this could have been an artifact and it is now excluded.

**The uncomfortable half: the evidence predates `D2c` entirely.** The exclusion
mechanism was a complete differential instrument the whole time; the sentinel
ran the functionized route, held the answer, and discarded it. `D4` bounds how
far that shape spread.

## The four constructs

| construct | n |
|---|---|
| `ComputationalMatch` / in-flight non-transferable activation | 4 |
| `StaticWorkerBinding` | 2 |
| Backend `Module` / missing recursive-position-1 worker projection | 2 |
| Backend `PlannerInvariant` / missing affine checked-root authority | 1 |

Exact test names are in the frame, section 3. A further **five** reds assert the
retiring lane's own control, lifecycle or route state with no program refusing;
those are `D6` rewrites in the predecessor and **stay gated behind the nine**.

## Deliverables

| id | owner | what |
|---|---|---|
| **`D3`** | runtime ring | **RUN FIRST.** Does any of the nine fall inside a **merged** port node's claimed population? A hit makes that node's completeness claim false on `main` — an **erratum**, which re-homes the successor. **Hard stop and hand back on any hit.** |
| **`D1`** | **Architect** | Per construct: is the refusal **correct semantics** or a **missing port**? **Four verdicts, not one** — they may not answer alike. Soundness question; the ring does not decide it as engineering. |
| **`D2`** | runtime ring | Source-reachability of the nine. **Decides BLOCKED versus RECORDED GAP.** Do not inherit `0/12` — it was measured over renderings, and the mapping to test names is established for the sentinel only. |
| **`D4`** | runtime ring | Bounded sweep of the two trace helpers that run a functionized compile purely to harvest events and never confirm it succeeded. Same defect shape that concealed this finding for the campaign. |

## Standing

- **`D2c` stays UNPUBLISHED and unrebased** at
  `036e8ee916844fb91a4f42f2a2b04ebaea0dde2f`. Its base `c98f72ba8` is what the
  pin is measured against.
- **No Runtime implementation is authorized by this node.** It measures and
  adjudicates; it deletes and ports nothing.
- **[[RT-DESCENT-RETIRE]]'s `D3`-`D8` stay gated**, and **no `D6` re-home is
  lawful** while this node is open.
- **Two shortcuts are foreclosed** — *"fixture-only so it doesn't count"* and
  *"`RecursiveDescent` compiled it, so port it."* Both are argued in the frame,
  section 6; neither may be assumed from the error text.
