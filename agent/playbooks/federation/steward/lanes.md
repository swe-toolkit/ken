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

## Current roster — three-lane trial (operator 2026-08-22; REAFFIRMED 2026-08-25)

Three concurrent lanes. The trial's own purpose is to measure whether three
lanes overburden the Architect (see lane 3).

> **Operator, 2026-08-25 (reaffirmation, correcting a Steward single-lane
> relapse):** "there are three lanes authorized right now. language (lane 2) was
> unblocking rt on priority, and when that was done should have unblocked
> foundation (lane 3) with module/import." The lanes are runtime / language /
> foundation. Language's job, after its runtime-unblocking work, is
> `LANG-MODULE-IMPORT-SYSTEM` (module/import) — which UNBLOCKS lane-3 foundation.
> Foundation is NOT idle-by-design: it has ready CAT WPs to author now, and its
> reuse-remediation node waits on module/import. Do not collapse to one lane.

The objectives below were refreshed 2026-08-25 to current node reality (the
2026-08-22 citations — NHC chain, Z3/FO, "5 CAT WPs" — had advanced). Re-measure
each node (`git fetch`; read status) before acting; a node id decays.

| lane | ring | objective |
|---|---|---|
| 1 | runtime | The native carried-value program `RT-NATIVE-CARRIED-VALUE` (M-series defunctionalization). M6/M4 merged; M3 at its gate; two M3 successors filed. |
| 2 | language | Module/import campaign `LANG-MODULE-IMPORT-SYSTEM` — unblocks lane 3. Next releasable member WP: `LANG-MOD-STRICT-RESOLUTION` (WP-2, `ready`, dep merged). |
| 3 | foundation | Expressibility trial — CAT WPs. `CAT-DEQUE`/`CAT-BSEARCH`/`CAT-VEC` `ready` now; `CAT-GCD-REFACTOR` waits on module/import. The Architect-burden probe. |

**Lane 1 — runtime (priority).** The native carried-value program
`RT-NATIVE-CARRIED-VALUE` (Architect frame `evt_9kat78d438cb`): a finite
compile-time-known defunctionalization carried at runtime as discriminant only.
M-series seats. M6 (Track-1 D0 `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`) and M4
(`RT-CLOSURE-BOUNDARY-RESIDUAL`) merged; M3 (`RT-CARRIED-IH-DISPATCH-SITEOP`) at
its QA + Architect gate (Route A proven). M3's crossing exposed two distinct
successors, now filed: `RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` (ExitCode payload
execution trap) and `RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` (unit call-graph).
The NHC chain + `RT-BACKEND-MODULE-SPLIT` (the 2026-08-22 objective) are drained/
merged. Architect is required reviewer on the M-series — the Architect-heavy lane.

**Lane 2 — language: module/import campaign `LANG-MODULE-IMPORT-SYSTEM`.** This
is the lane's current objective and it UNBLOCKS lane-3 foundation. Framing is
COMPLETE (Architect 4-WP decomposition `evt_hpnhqy1ex286`; spec-surface merged
`def16ecf4`). Member-WP state (re-measure before acting): WP-1
`LANG-MOD-LOADER-ENTRY` merged; WP-3 `LANG-MOD-PUB-ELIGIBILITY` merged; WP-4A
`LANG-MOD-CATALOG-REALIZATION` merged; `LANG-MOD-CATALOG-COMPLETENESS`
(Component B) `active` (authorized partial; remainder held on the Nat Decision
`dec_1kqwn6hdvn7d2`). **WP-2 `LANG-MOD-STRICT-RESOLUTION` (the strict root-loaded
resolution soundness core) is `ready`, dep `LANG-MOD-LOADER-ENTRY` merged — the
next releasable module/import WP, NOT blocked on the Nat Decision.** The
Nat/Order provider drafts (`LANG-MOD-NAT-PROVIDER-INTERFACE`) + `CAT-GCD-REFACTOR`
are what the Nat Decision gates, not WP-2.

The earlier lane-2 objectives are DONE and are history, not current work:
`V3-FO-CHECKER-SOUNDNESS` is `closed` (FO checker-soundness theorem complete,
both fragments); `CI-Z3-BASE-IMAGE` + the FO/Z3 chain landed;
`KERNEL-CONV-TRUNC-CONGRUENCE` merged. The residual FO frontier
`V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY` (rotation fork) is filed and separate; it is
not the module/import priority. Verify/kernel are reviewers here, not a separate
active lane.

**Lane 3 — foundation: expressibility trial (bounded).** Five independent,
non-sequential CAT WPs — `CAT-SORT`, `CAT-GCD`, `CAT-DEQUE`, `CAT-BSEARCH`,
`CAT-VEC` — authoring verified catalog algorithms against Ken's current surface.
Charter: `docs/program/wp/foundation-expressibility-trial.md`. Launched
2026-08-22 (anchor `evt_4r550cbd3fvvb`, simplest-first, VEC last). Current state
(re-measure before acting): `CAT-SORT` + `CAT-GCD` merged; **`CAT-DEQUE`,
`CAT-BSEARCH`, `CAT-VEC` are `ready`, `gate: none`, no deps — releasable to the
foundation ring NOW.** Foundation is NOT idle-by-design when these are ready. The
separate reuse-remediation node `CAT-GCD-REFACTOR` `depends_on`
`LANG-MODULE-IMPORT-SYSTEM` (do not cut/release it until module/import lands).
Architect NOT a default reviewer; QA reviews; a surface-gap report is what routes
to spec/Architect. Stop-on-gap: a gap finding is the trial's payoff. **This lane
is the instrument for the Architect-burden question** — the reason it runs
concurrently is to measure that.

## Not a lane

**Doc track** runs concurrently but is contention-free (`library/`, `agent/`,
not `crates/`) — it is the standing exception, not a lane (`CLAUDE.md`).

## Roster history

- 2026-08-25: operator REAFFIRMED the three-lane trial after a Steward single-lane
  relapse (I had collapsed to runtime-only post-compaction and left WP-2 + three
  ready CAT WPs unreleased). Lane objectives refreshed to current node reality
  (lane 1 = native carried-value M-series; lane 2 = module/import; lane 3 = CAT
  trial). Structure unchanged: three lanes, runtime / language / foundation.
- 2026-08-21 → 2026-08-22: operator moved from one lane (runtime, 2026-08-17) to
  the three-lane trial above.
- 2026-08-17: one lane (runtime, RecursiveDescent-retirement residuals); lanes 2
  and 3 idle. (Superseded.)
