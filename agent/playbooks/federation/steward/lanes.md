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
| 1 | runtime | The native carried-value program `RT-NATIVE-CARRIED-VALUE` (M-series defunctionalization). M6/M4/M3 merged. Active successor: `RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE` (`ready`, the ExitCode recut). `RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` + `RT-UNIT-FAILURE-STATUS-PROVENANCE` `draft`. |
| 2 | language | Module/import campaign `LANG-MODULE-IMPORT-SYSTEM` — essentially COMPLETE (WP-1/2/3/4A merged incl. strict resolution). Remaining member work (Component B remainder, Nat provider, `CAT-GCD-REFACTOR`) is gated on the Nat Decision `dec_1kqwn6hdvn7d2`; lane-2 interim direction is an operator call (surfaced 2026-08-25). |
| 3 | foundation | Catalog-reuse modernization. Expressibility trial COMPLETE (all CAT algos merged; Architect not overloaded = 3-lane feasibility PROVEN, operator 2026-08-26). Lane now: pilot `CAT-ORDER-PUB-EXPORT` (released 2afacd0c0) then `CAT-GCD-REFACTOR`; a catalog-wide census/campaign is PROPOSED, pending operator scope. |

**Lane 1 — runtime (priority).** The native carried-value program
`RT-NATIVE-CARRIED-VALUE` (Architect frame `evt_9kat78d438cb`): a finite
compile-time-known defunctionalization carried at runtime as discriminant only.
M-series seats. M6 (Track-1 D0 `RT-CHECKED-IH-FUNCTIONAL-REPRESENTATION`), M4
(`RT-CLOSURE-BOUNDARY-RESIDUAL`), and M3 (`RT-CARRIED-IH-DISPATCH-SITEOP`) merged.
M3's crossing exposed two successors; the first was recut 2026-08-25 after three
consecutive Architect hard stops on a shared predicate (a downstream semantic
classification used as upstream producer/provenance authority): the ExitCode WP
`RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT` is `closed`/falsified (Architect
evt_1vhmndq7fscd1) and REPLACED by `RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE`
(`ready`) — an owner-bound probe of the causal dynamic-constructor dispatch
residual, no production mechanism authorized before D0 selects a class. The `-3`
reporter alias is split out as `RT-UNIT-FAILURE-STATUS-PROVENANCE` (`draft`,
sequenced after; not folded). `RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` (`draft`)
stays distinct. The NHC chain + `RT-BACKEND-MODULE-SPLIT` are drained/merged.
Architect is required reviewer on the M-series — the Architect-heavy lane.

**Lane 2 — language: module/import campaign `LANG-MODULE-IMPORT-SYSTEM`.** This
is the lane's current objective and it UNBLOCKS lane-3 foundation. Framing is
COMPLETE (Architect 4-WP decomposition `evt_hpnhqy1ex286`; spec-surface merged
`def16ecf4`). Member-WP state (re-measure before acting): WP-1
`LANG-MOD-LOADER-ENTRY` merged; WP-3 `LANG-MOD-PUB-ELIGIBILITY` merged; WP-4A
`LANG-MOD-CATALOG-REALIZATION` merged; `LANG-MOD-CATALOG-COMPLETENESS`
(Component B) `active` (authorized partial; remainder held on the Nat Decision
`dec_1kqwn6hdvn7d2`). WP-2 `LANG-MOD-STRICT-RESOLUTION` (the strict root-loaded
resolution soundness core) is **`merged`** — its D0 census (`c64c62190`) + D1
enforcement (`5a74301f4`) shipped inside the Component A/B realization; a
2026-08-25 Steward re-release off its stale `ready` node was withdrawn on the
implementer's hard stop (no non-duplicative delta). **So module/import is
essentially complete; the remaining member work is the Nat prerequisite +
Component B's remainder + `CAT-GCD-REFACTOR`.** The Nat Decision
`dec_1kqwn6hdvn7d2` is RESOLVED (2026-08-25): the operator ruled the prelude
membership rule (`30-taxonomy §4`) itself the defect and superseded the
provider-registry mechanism — Nat's home is PRELUDE-FLOOR MEMBERSHIP (amend the
general rule to a bootstrapping arm; admit the existing kernel {Nat,Zero,Suc}
into the strict floor, reuse identity). Reframed into a spec WP
(`LANG-MOD-NAT-PROVIDER-INTERFACE`, `ready`, release FIRST) + a build WP
(`LANG-MOD-NAT-FLOOR-REALIZATION`, held on the spec WP). Releasing the spec WP to
the enclave unblocks the chain; do NOT re-release WP-2 off its node — re-measure
the tree.

The earlier lane-2 objectives are DONE and are history, not current work:
`V3-FO-CHECKER-SOUNDNESS` is `closed` (FO checker-soundness theorem complete,
both fragments); `CI-Z3-BASE-IMAGE` + the FO/Z3 chain landed;
`KERNEL-CONV-TRUNC-CONGRUENCE` merged. The residual FO frontier
`V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY` (rotation fork) is filed and separate; it is
not the module/import priority. Verify/kernel are reviewers here, not a separate
active lane.

**Lane 3 — foundation: catalog-reuse modernization campaign.** The expressibility
trial (five CAT algos — `CAT-SORT`/`CAT-GCD`/`CAT-DEQUE`/`CAT-BSEARCH`/`CAT-VEC`,
charter `docs/program/wp/foundation-expressibility-trial.md`) is COMPLETE — all
merged. Its purpose (measure whether three lanes overload the Architect) is
DISCHARGED: the operator ruled 2026-08-26 that feasibility is PROVEN and directed
the lane to continue.

The lane's new objective is the **catalog-reuse modernization** campaign
(operator 2026-08-26; charter `docs/program/wp/catalog-reuse-modernization.md`):
now that the prelude is expanded and module imports work, rework catalog packages
along three axes — (a) remove defs redundant with the prelude, (b) import canonical
tools from sibling modules instead of reimplementing, (c) restructure files
top-down. Census-first, conservative/risk-tagged depth, lane-3 priority.

Current state (re-measure before acting):
- PILOT (proves the per-package recipe): `CAT-ORDER-PUB-EXPORT` (`ready`, RELEASED
  2afacd0c0 — the Order pub-export prerequisite) then `CAT-GCD-REFACTOR` (`draft`,
  Gcd-only import+dedup+top-down, `depends_on CAT-ORDER-PUB-EXPORT`). Note the CAT
  reuse-remediation is NO LONGER blocked on the `LANG-MODULE-IMPORT-SYSTEM`
  umbrella — the import + pub-export capability it needs is LANDED; only the Order
  half remained, now extracted to the pilot prerequisite.
- CAMPAIGN SCOPING: `CAT-REUSE-CENSUS` (`ready`) — the catalog-wide inventory that
  sizes the rework. Runs on the single foundation ring AFTER/beside the pilot.

Reviewers: foundation-qa + conformance-validator (catalog implementation
standard); a genuine design/spec gap (eligibility, attached-proof ownership)
HARD-STOPS to spec/Architect — a gap finding is the payoff. The three-lane
Architect-burden question is now ANSWERED (feasible), so this lane runs as normal
directed foundation work, not a probe.

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
