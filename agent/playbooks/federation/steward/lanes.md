# Live lane roster (Steward)

**The single source of truth for the current lane configuration — how many
lanes, which ring each is, and each lane's objective.** This is TIME-VARYING
operator direction. The playbook (`steward.md` §0) holds the stable lane
discipline and points here for the roster; the roster does NOT live in the
playbook.

**Read this at session start and after every compaction**, in the same startup
sequence as `COORDINATION.md`, `MODELS.md`, and your memory scopes. A lane is a
state, not an event — you must know the roster before you act, so it is a
resident startup read, not a fetch-when-needed pointer.

**Update this file only on an operator ruling, and cite the ruling.** No
measurement of yours adds, retires, or re-scopes a lane (`steward.md` §3). When
you measure something that bears on the roster, surface it to the operator; do
not act on it against the roster.

> **Why this file exists.** The roster used to be baked into `steward.md` §0. On
> 2026-08-21 the operator moved from one lane to a three-lane trial; the playbook
> was not where that changed, and the Steward ran the retired one-lane premise
> for a day (it did not launch the authorized foundation lane, and held live
> lane-2 successors as "retired"). A roster in the playbook is time-varying state
> wearing a permanent hat. Operator, 2026-08-22: playbook = stable discipline,
> this file = mutable roster.

## Current roster — 2026-08-22 (operator, three-lane trial)

Three concurrent lanes. The trial's own purpose is to measure whether three
lanes overburden the Architect (see lane 3).

| lane | ring | objective |
|---|---|---|
| 1 | runtime | Finish the `NATIVE-HANDLE-CARRIER` carried-observation chain, THEN pivot to `RT-BACKEND-MODULE-SPLIT`. |
| 2 | verify + language (+ kernel) | Z3 integration. |
| 3 | foundation | Foundation expressibility trial — 5 independent CAT WPs. Bounded; the Architect-burden probe. |

**Lane 1 — runtime.** Finish the `NATIVE-HANDLE-CARRIER` / `cap41_*`
carried-observation chain: `RT-EXACTINT-CARRIED-OBSERVE` (in review) →
`RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` → NHC `D-final` re-run → closes
`NATIVE-HANDLE-CARRIER` and `PX8-F-CAP-41` Phase 2. Then pivot to
`RT-BACKEND-MODULE-SPLIT` (currently `draft` — needs framing before it is
startable). Operator confirmed 2026-08-22 the NHC chain is the right runtime
work (not a drift). Architect is required reviewer on the carried-observation
nodes — the Architect-heavy lane.

**Lane 2 — verify + language (+ kernel): Z3 integration.**
`V3-FO-CHECKER-SOUNDNESS` and `LANG-INDEXED-RECURSIVE-IH-DISCHARGE` are `active`;
`CI-Z3-BASE-IMAGE` landed (verify infra). NOTE: `V3-FO`'s `D3` and letting
`LANG-INDEXED` close are both gated on the operator-gated kernel SCT successor
(`KERNEL-SCT-TELESCOPE-CANON`), which is open operator question Q3 — see the
briefing. Re-confirm each node's status (`git fetch`; read the node) before
acting; a node id decays.

**Lane 3 — foundation: expressibility trial (bounded).** Five independent,
non-sequential CAT WPs — `CAT-SORT`, `CAT-GCD`, `CAT-DEQUE`, `CAT-BSEARCH`,
`CAT-VEC` — authoring verified catalog algorithms against Ken's current surface.
Charter: `docs/program/wp/foundation-expressibility-trial.md`. Launched
2026-08-22 (anchor `evt_4r550cbd3fvvb`, simplest-first, VEC last). `gate: none`,
no deps, Architect NOT a default reviewer; QA reviews; a surface-gap report is
what routes to spec/Architect. Stop-on-gap: a gap finding is the trial's payoff.
**This lane is the instrument for the Architect-burden question** — the reason it
runs concurrently is to measure that.

## Not a lane

**Doc track** runs concurrently but is contention-free (`library/`, `agent/`,
not `crates/`) — it is the standing exception, not a lane (`CLAUDE.md`).

## Roster history

- 2026-08-21 → 2026-08-22: operator moved from one lane (runtime, 2026-08-17) to
  the three-lane trial above.
- 2026-08-17: one lane (runtime, RecursiveDescent-retirement residuals); lanes 2
  and 3 idle. (Superseded.)
