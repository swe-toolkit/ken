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

The objectives below were re-measured **2026-08-27 against `origin/main`
`61c2fefa0`** (previous refresh 2026-08-25). **The roster STRUCTURE — three
lanes, runtime / language / foundation — is operator-owned and UNCHANGED; only
the node citations were re-measured.** Re-measure each node (`git fetch`; read
status) before acting; a node id decays, and at the 2026-08-27 measurement
**seven of the cited nodes had advanced past the state this table claimed** —
five to `merged`. Treat every id below as a pointer to check, not a fact.

| lane | ring | objective |
|---|---|---|
| 1 | runtime | The native carried-value program `RT-NATIVE-CARRIED-VALUE` (`active`, M-series defunctionalization). M6/M4/M3 merged; `RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE` and `RT-UNIT-FAILURE-STATUS-PROVENANCE` now MERGED. CURRENT: `RT-CHECKED-IH-GENERATED-ENTRY-ACCESS` (`active`) — the HS10 total-admission-map predecessor, re-released 2026-08-27 `evt_7qh4fnzcg9t2c`; it blocks `RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (`active`), whose D3A+D3B stays FROZEN pending its own separate release. `RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` still `draft`. |
| 2 | language | CURRENT: `LANG-INDEX-REFINEMENT-OMEGA-ARM` (`active`) — D1 MERGED 2026-08-27 as `e13df606a` (both blobs blob-verified); D2 excluded and needs its own release. Also `active`: `V3-FO-EMBEDDING-ADEQUACY` (D2 needs a SECOND separate re-release after D1 and D2 both land) and `LANG-MOD-CATALOG-COMPLETENESS`. The Pair recut `LANG-MOD-PAIR-FLOOR-PROVIDER` is MERGED, and its successor `LANG-MOD-CANONICAL-PAIR-PACKAGE` is `active`. Both Nat nodes MERGED. NEXT after the current actives drain: the z3 integration campaign (operator 2026-08-26). Module/import umbrella `LANG-MODULE-IMPORT-SYSTEM` COMPLETE. |
| 3 | foundation | Catalog-reuse modernization. Expressibility trial COMPLETE (3-lane feasibility PROVEN, operator 2026-08-26). The pilot chain is DONE: `CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR` and `CAT-REUSE-CENSUS` are all MERGED. CURRENT: `CAT-NAT-REUSE-CONSUMERS` (`active`) — six per-package increments, D1/D2 merged, **D3 `Parsing.ken.md` released 2026-08-27 `evt_49h0h5hffm9yy`**; D4/D5/D6 held, D6 last as the risk increment. Each increment needs its own explicit release. |

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
— an owner-bound probe of the causal dynamic-constructor dispatch
residual — which is now **MERGED**. The `-3` reporter alias, split out as
`RT-UNIT-FAILURE-STATUS-PROVENANCE`, is **MERGED** too.
`RT-RETAINED-UNIT-CALL-TARGET-DERIVATION` (`draft`) stays distinct. The NHC chain
+ `RT-BACKEND-MODULE-SPLIT` are drained/merged.
Architect is required reviewer on the M-series — the Architect-heavy lane.

**CURRENT lane-1 work (measured 2026-08-27):**
`RT-CHECKED-IH-GENERATED-ENTRY-ACCESS` (`active`), the complete planner-owned
predecessor that replaced the repeated last-gap decomposition. It has taken
**six** Architect hard stops (HS5-HS10); the last four share one root — a partial
instrument asked a question it structurally cannot answer — and the frame defects
were the Steward's. HS10 replaced the partial governed-only projection map with
ONE TOTAL `Governed`/`NonGoverned` admission map; recut landed `61c2fefa0` and
was re-released `evt_7qh4fnzcg9t2c`. It blocks
`RT-RESULT-CONTINUATION-BINDING-PROVENANCE` (`active`), whose **D3A+D3B stays
FROZEN and needs its own separate explicit release** — neither the frame landing
nor the predecessor landing authorizes the consumer. Next mandatory §1a/§1b
research-advisory trigger on this node is **stop 12, NOT 10** (Architect
`evt_2s144kdddyckn`, verbatim — HS9 consumed the ninth-stop advisory).

**Lane 2 — language. CURRENT (measured 2026-08-27):**
`LANG-INDEX-REFINEMENT-OMEGA-ARM`
D1, routed; `V3-FO-EMBEDDING-ADEQUACY` and `LANG-MOD-CATALOG-COMPLETENESS` also
`active`. The z3 integration campaign is NEXT, once these drain.** The prelude
recut below is DONE and is history, not current work.

Operator 2026-08-26 (direction, unchanged): "first launch the internal-provision
prelude recut and finish that effort, then return to the z3 integration
campaign." That recut — `LANG-MOD-PAIR-FLOOR-PROVIDER`, a Steward-owned spec WP
(Architect shaping `evt_7d0ecgkd8ate3`), frame landed `c1945c6fb`, spec ring
anchor `evt_6yc0k921tf3j` — **is MERGED as `8f3b6fd2`**, so the operator's "then"
clause is the live half. It generalizes
prelude membership to ONE internal-provision arm (kernel or compiler origin),
admits Pair as the first compiler-provided member reusing the four existing
compiler-installed Pair `GlobalId`s, and supersedes the exact-nine boundary of
`LANG-MOD-PAIR-STRICT-BOUNDARY`. Its build successor
`LANG-MOD-CANONICAL-PAIR-PACKAGE` (`depends_on` repointed) realizes the split
inventories after the spec lands. After the recut lands, lane 2 returns to the
z3 integration campaign (verify/FO-checker resume). The module/import history
below is DONE.

**Module/import campaign `LANG-MODULE-IMPORT-SYSTEM` (history — essentially
COMPLETE).** This was the lane's prior objective and it UNBLOCKED lane-3
foundation. Framing is
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

Current state (measured 2026-08-27 against `origin/main` `61c2fefa0`;
re-measure before acting):
- PILOT CHAIN: **DONE.** `CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR` and
  `CAT-REUSE-CENSUS` are all **MERGED**. The per-package recipe is proved and the
  catalog-wide inventory that sizes the rework is landed. (The CAT
  reuse-remediation was never blocked on the `LANG-MODULE-IMPORT-SYSTEM` umbrella
  in the end — the import + pub-export capability it needed landed, and the Order
  half went through the pilot prerequisite.)
- CURRENT BATCH: `CAT-NAT-REUSE-CONSUMERS` (`active`) — the census's first scoped
  rework batch, six independently-releasable per-package increments. D1
  (`Arguments.ken.md`) and D2 (`Diagnostics/Core.ken.md`) MERGED; **D3
  (`Parsing.ken.md`) RELEASED 2026-08-27 `evt_49h0h5hffm9yy`**. D4/D5/D6 are
  HELD and each needs its own explicit release; D6 (`Derived.ken.md`) is the risk
  increment and goes LAST — its `AC-PROP` can hard-stop to spec/Architect, and
  that is a payoff, not a setback.
- MEASURED CARRY-FORWARD for every remaining increment (Adversary
  `evt_5sw5w9w4jj35z`, M8 on D2): importing direct from a canonical owner makes
  the consumer transitively inherit that provider's OWN un-migrated ambient
  surface. `AC-AMBIENT-DELTA` asks for a measurement and a report, **not a
  shrink** — census growth there is inherited debt, not a defect in the
  increment. Note also that a package sitting in the RESIDUAL bucket (e.g.
  `Parsing` at `UnresolvedCon(SourceId)`) has that inheritance MASKED rather than
  absent, so report the exact post-edit residual rather than an expected vector.

Reviewers: foundation-qa + conformance-validator (catalog implementation
standard); a genuine design/spec gap (eligibility, attached-proof ownership)
HARD-STOPS to spec/Architect — a gap finding is the payoff. The three-lane
Architect-burden question is now ANSWERED (feasible), so this lane runs as normal
directed foundation work, not a probe.

## Not a lane

**Doc track** runs concurrently but is contention-free (`library/`, `agent/`,
not `crates/`) — it is the standing exception, not a lane (`CLAUDE.md`).

## Roster history

- 2026-08-27: **no roster change — citation re-measurement only.** A watchdog
  step-5 sweep against `origin/main` `61c2fefa0` found SEVEN cited nodes had
  advanced past what this table claimed, five of them to `merged`: lane 1's named
  active successor and its `-3` alias, and lane 3's entire pilot chain
  (`CAT-ORDER-PUB-EXPORT`, `CAT-GCD-REFACTOR`, `CAT-REUSE-CENSUS`). Lane 2's
  "CURRENT" still named a merged spec WP. None of the three lanes' actual current
  work appeared in the table at all. Structure untouched — three lanes,
  runtime / language / foundation, operator 2026-08-22 and 2026-08-25. **This is
  the decay the file's own header warns about, and it is worth re-running that
  sweep at any tick where a lane looks unexpectedly quiet**: a stale objective row
  is what produced the documented single-lane relapse, and it fails silently
  because every individual row still reads plausibly.
- 2026-08-25: operator REAFFIRMED the three-lane trial after a Steward single-lane
  relapse (I had collapsed to runtime-only post-compaction and left WP-2 + three
  ready CAT WPs unreleased). Lane objectives refreshed to current node reality
  (lane 1 = native carried-value M-series; lane 2 = module/import; lane 3 = CAT
  trial). Structure unchanged: three lanes, runtime / language / foundation.
- 2026-08-21 → 2026-08-22: operator moved from one lane (runtime, 2026-08-17) to
  the three-lane trial above.
- 2026-08-17: one lane (runtime, RecursiveDescent-retirement residuals); lanes 2
  and 3 idle. (Superseded.)
