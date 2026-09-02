---
id: RT-COMPOSED-RETURN-SSA-SPECIALIZATION
title: "Composed-return native repair — PRIMARY: polyvariant compile-time response-owner specialization (Architect mechanism ruling evt_29jfzzw9j5xjz), the operator-preferred compile-time SSA path OVER the runtime closure. For every statically attributable (response producer, K) pair, emit one Function whose code identity fixes K, whose frame carries all K captures + enclosing continuation inputs as explicit ABI slots, and whose body does host dispatch/validation then directly calls that K with the exact response as operand 0. NO K tag, closure word, apply dispatcher, environment aggregate, code pointer, process-global slot, or runtime selector. Checkpoint 1 is a FEASIBILITY LEDGER ONLY; an irreducibly-multiple or dynamic-K reached edge is a typed SsaInfeasible finding that STOPS and routes to the operator before the fallback is selected. No public ABI change (unit_signature unchanged); no kernel primitive; no spec commitment."
status: active
owner: runtime
size: L
gate: runtime-qa+architect
tier: T1
depends_on: [RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT]
blocks: []
github: null
origin: "RECUT 2026-09-02 (HS3 structural closure, Architect ruling evt_5yjjsrhpmt204 on research advisory evt_3z83vwpenscft): the absence-based decline is replaced by a first-class Deferred residual (classify->Disposition=Specialized|Deferred{payload}, total-match every stage, §7 sealed-enum no catch-all); the proved Specialized path is retained; base cut fresh from origin/main 4a088d8aa; ACs 1-7 in the RECUT body section; HS3 discharged. AMENDED 2026-09-02 (Architect evt_4ar3rxzrra5v4, on implementer pre-impl hard stop evt_33teszvwarz6 = HS4, not a CI-red): Deferred = P1 UNION P2 (P1 absent-residual + P2 present-but-unconsumed placeholder = the HS3-b leak); discriminator = caller-consumption (Specialized IFF specializable AND caller consumed), retargetability hoisted to planning (answer b, D0 preserved); added AC-7 pins classify vs lowering-time CandidateDisposition. Next re-trigger HS6. Prior origin: Architect mechanism ruling evt_29jfzzw9j5xjz (2026-09-01), the authoritative byte-level contract for the Specialized path; do NOT re-derive it, fold and cite it. Issued under the operator preference for compile-time SSA handling OVER the runtime closure (2026-09-01, correcting a Steward mis-scope of the approach fork; Steward direction correction evt_10dfspc3ssk5). Research extension advisory SHA-256 19bc67e5dada7cbac4445875cdcfd5ab079aecb3bc56b6df712696bcd296f3c1. The Architect confirmed this SSA path opens NO new operator fork (no public ABI change — unit_signature stays (frame_ptr, services_ptr) -> i64; no kernel primitive; no spec commitment), so the Steward frames and releases it. Bound base = the clean held checkpoint ad191d1c29af288b059bbb00c1b573c3c4356ab3, tree 342e3b735 (carries WP1's preserved environment/result role split, and BoundaryClosureEnvironment / ContinuationCallIdentity.worker as the body/arity/capture-schema authorities). The invocation-owned runtime closure (RT-COMPOSED-RETURN-RUNTIME-CLOSURE, ruling evt_3j6vshm83rk5q) remains the FALLBACK, held draft, selected ONLY if this SSA path returns SsaInfeasible and the operator so rules; it is NOT built in parallel. Halted runtime-closure scratch aee8c9408c986bb946d228069a5104c70db84ea4 is evidence only. WP1 RT-COMPOSED-RETURN-DIRECT-ROLE-SPLIT preserved as the base asset and depends_on predecessor; the delayed-SSA WP2/WP3 (RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE, RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT) stay closed. Origin funding evt_3met6tbk5wrnd after accepted terminal NO_UNIQUE_EDGE evt_mx6scjje1yjp. RE-RECUT 2026-09-02 (HS5 structural, Architect ruling evt_7eh84c8n6w08e + #3 refinement evt_6v4yp7arvd4f4 + design-close evt_53s106a7btrb8/evt_411q6cvg74yrn, research advisory evt_5ryjd4dk65x05 adopted 2-of-3): the 5th CI-red (e193dc631, PR #3241) is a hard-stop-ruled RESCOPE — same deliverable, restructured mechanism. The install-time causal-proxy classify is REPLACED by a TWO-PHASE, owner-additive, domain-total classify (phase A owner-less context entries over has-K-unit population, causal-prefix; phase B disposition over the sealed whole-population classify, post-:1251), closedness preserved by same-state re-derivation. Seal domain (whole population P1+P2+Specialized) strictly wider than context-entry domain (has-K-unit = P2+Specialized). e193dc631 DEAD; next candidate cut fresh from origin/main (tip 4e5481c57), NEW SHA. #3 (fs_read/write un-ignores) measurement-gated three-way ((a) in S_causal-minus-S_record, (b) Specialized lowering observes InvalidOffset): rides this WP if (a)&&(b), else splits to the InvalidOffset-product-witness piece. See RECUT 2 governing section."
---

## RECUT 2 — HS5 structural: post-install, domain-total response classify (Architect ruling evt_7eh84c8n6w08e, #3 refinement evt_6v4yp7arvd4f4)

**This section is now the CURRENT governing contract and supersedes the HS3
recut below wherever they conflict, on the response-CLASSIFICATION structure.**
Architect structural ruling `evt_7eh84c8n6w08e` (2026-09-02), refined on the #3
attribution by `evt_6v4yp7arvd4f4`, on research advisory `evt_5ryjd4dk65x05`
(adopted 2 of 3 conditions; see RESEARCH DIVERGENCE). The 5th CI-red
(`e193dc631`, PR #3241) is a hard-stop-ruled RESCOPE, not a candidate respin:
same deliverable (two fixed products via first-class Deferred), restructured
mechanism. `e193dc631` is DEAD; the next candidate is cut fresh from
`origin/main` on a NEW SHA. Thread retained: `thr_4cyk4x5wwjb3e`.

**Diagnosis confirmed (Architect, independently verified at e193dc631).** The
closed-derivation fix WORKED (writeAll compiles; the plane-closure invariant no
longer fires) and in doing so exposed that the causal set is a STRICT SUPERSET
of the record set, and over-admission is UNSOUND, not merely imprecise:
`checked_ih_coordinate_run` (aggregates.rs:4234) is NECESSARY, not sufficient,
for "unconsumed transport caller"; the transport-DESTINATION test is the real
Deferred/Specialized discriminator, and it is genuinely post-install.

### RECUT-2 AMENDMENT — HS6 staging correction: collapse the owner/aggregate_ownership boundary (Architect ruling evt_7yegg3t1sdn8d)

GOVERNS THE NEXT RESPIN. The 6th CI-red (candidate 46951a660, PR #3242,
held, nothing landed) is a BOUNDED staging correction against the two-phase
recut in this section — NOT a seal redesign. Research advised (evt_7txg04f1sxhma,
then reconciled evt_4kgv6g9fnjfwg); the ring grounded (evt_4ggkeyhnsdvgd); the
ring's source trace is the record of authority (§7a) where it corrects research's
hypothesized mechanism. The two-phase split HOLDS.

THE §1b PREDICATE NOW SPANS THREE BOUNDARIES — ONE PREDICATE (extends the
SYMPTOM INVENTORY / SHARED PREDICATE below with the 6th red):
- HS3: install classified before the transport fact existed (over-admission).
- HS5: the post-install transport read computed at install (phase-instability).
- HS6 #1: owner assignment finalized in phase B (construction.rs:1257) AFTER
  aggregate_ownership's derivation closed (:1249-:1250).
Shared predicate: A DERIVATION IS FINALIZED BEFORE A FACT IT TRANSITIVELY
DEPENDS ON IS FROZEN. Not three bugs — one predicate at three boundaries. The
closure is frozen-phase-ordering: each derivation runs ONCE where all its
transitive inputs are frozen, downstream CONSUMES. The recut already applied
that closure to the RESPONSE-CONTEXT plane (first-class Deferred) and CLOSED it
there; #1 is the SAME predicate at the owner/aggregate_ownership boundary. The
closed-derivation validator is CORRECT and CAUGHT #1 — it is not over-coupled.

RULING 1 — CONVERGENCE, not cascade. The response-plane closed-derivation class
that cascaded HS3->HS5 is drained AS A CLASS (writeAll compiles; the
plane-closure invariants no longer fire; 7/8 shards green). That meets the exact
(B)-drainable discriminator research set (the class closed structurally, not one
member at a time). What remains is two BOUNDED things: (i) #2/#3/#4 = one root,
the recut's CORRECT transport-caller reclassification exposing stale test
expectations; (ii) #1 = one bounded force-path aggregate_ownership divergence.

RULING 2 — #1 DISCIPLINE: collapse preferred, re-validate-after-B fallback. The
ring's trace REFUTES a direct-perturbation story: phase B writes only
static_response_continuations/deferred; build_aggregate_ownership_plan
(aggregates.rs:4358-4714) reads plan.source_occurrences / semantic /
record_field_identity / escape+meet — none of them. #1 is a TRANSITIVE,
force-only divergence (a forced Specialized owner coexisting with a
transport-source record on one member re-derives record_field_identity /
escape-meet differently at closure.rs:2091 vs :1249). Because the coupling is
transitive and unlocalized, research's "partition disjoint non-meet-read owner
rows" sub-option is DEAD — you cannot partition a coupling no one has localized.
Preference order:
- PREFERRED (collapse): finalize the WHOLE owner assignment BEFORE
  aggregate_ownership builds (:1249), so :1249 and the final validator (:2091)
  close over the SAME frozen owner state — direct AND transitive coupling both
  vanish (no later phase to perturb). Removes the hazard CLASS
  (subsume-don't-proliferate), not the field the transitive path runs through.
- FALLBACK (re-validate-after-B): if collapse is infeasible, schedule
  aggregate_ownership's validate/re-derivation AFTER phase B so :1250 and :2091
  both close over final owner state (LLVM invalidate-and-rerun). Minimal correct
  fix; disciplines the boundary rather than removing it.

RULING 3 — #2/#3 DISCIPLINE: gate on measurement (a); reframe the ledger to
TOTALITY. (a) = is the write-BufferAllocate k_identity in
checked_ih_environment_transport_source_identities()?
- (a) YES: the member IS a transport source -> the recut CORRECTLY classifies it
  Deferred -> the test encodes the pre-first-class-Deferred leak (the HS3-b shape
  the recut removes) -> STALE: update the expectation to 0 Specialized + 1
  Deferred. #3 shares the fixtures -> same disposition (currency, #4's family).
- (a) NO: genuine phase-B over-deferral (a member that should be Specialized is
  deferred) -> a classification defect to FIX in phase B, not a test update.
- REGARDLESS of (a): the demand ledger asserts TOTALITY, not a raw Specialized
  count — Specialized-count + Deferred-count = expected population for the
  fixture, each member in EXACTLY ONE column, never absent from both (§7
  sealed-enum; the SEAL-domain ruling). "0 Specialized" is acceptable ONLY as
  "0 Specialized AND 1 Deferred," never "0 and 0." Robust to the correct
  reclassification AND still catches a genuine drop.
- #4 (rt_cold_lowering_path_enumeration rt_allocate_stage :610): currency — move
  to Disposition::Completes (the mechanism retired its blocker).

THE GATING MEASUREMENTS (ring's ground; CI/native-instrumentable; the respin
RELEASES on them — do NOT respin before measurements 1 and (a) land; 3 is
conditional). Adopted research's fallback precondition (evt_717p9fgzzg3sv),
Architect evt_5786jn8ty5hkr:
- MEASUREMENT 1 — FEASIBILITY (decides collapse vs fallback): does
  checked_ih_environment_transports @construction.rs:1251 — the input phase B
  keys on — DEPEND on build_aggregate_ownership_plan's OUTPUT (:1249)?
  Independent -> LIFT the transport build above :1249, do the whole owner
  assignment there -> collapse feasible -> PREFERRED. Dependent -> collapse
  infeasible -> re-validate-after-B. (The ring already grounded the other half:
  aggregate_ownership does not read phase B's output.)
- MEASUREMENT 3 — FALLBACK PRECONDITION (CONDITIONAL: ground ONLY if
  measurement 1 forces the fallback branch; do NOT let it delay 1 and (a)): does
  aggregate_ownership's plan have any SURVIVING consumer BETWEEN :1249 and phase
  B (:1257)? invalidate-and-rerun at :2091 is sound only if EVERY consumer of
  aggregate_ownership's result is covered by the re-derivation, not just the
  final validator; an intermediate pass that already consumed the pre-B result
  and committed on it is an HS7 waiting to happen. If a surviving consumer
  exists -> the fallback must RE-RUN those consumers after phase B too, not only
  :2091; if none survive -> re-validate-after-B at :2091 alone is sufficient.
  COLLAPSE (Resolution 1) discharges this obligation BY CONSTRUCTION — no re-run
  to schedule, no intermediate-consumer check — a second, independent reason to
  prefer collapse over the fallback.
- MEASUREMENT (a) — decides #2/#3 currency vs over-deferral bug: as in Ruling 3.

ACCEPTANCE DELTA (HS6; additive to the ACCEPTANCE ACs below):
- AC-HS6-1: #1 boundary closed — by collapse (transport+owner assignment
  finalized before aggregate_ownership @:1249; :1249 and :2091 close over
  identical frozen owner state) OR by re-validate-after-B; the AC-7
  force-specialize path that red at px8f :778 now greens. Control: the
  closed-derivation validator still REDS a deliberately-perturbed owner state —
  it stays a real check, not disabled to pass.
- AC-HS6-2: the demand ledger is a TOTALITY assertion (Specialized+Deferred =
  fixture population, exactly one column per member). Control: a mutation that
  drops a member reds (0 and 0); a mis-column reds; the correct
  0-Specialized-1-Deferred write-path reclassification greens.
- AC-HS6-3: #4 rt_allocate_stage at Disposition::Completes.
- RELEASE EVIDENCE: measurements 1 and (a) RECORDED (and 3, if 1 forced the
  fallback); the chosen #1 discipline (collapse vs fallback) stated with the
  measurement that chose it, and — on the fallback — the re-run scope from
  measurement 3.

Honesty on the z1400 placement (Architect §7a/§1b-iii, carried into the frame):
z1400's "the exact discriminator needs post-install data" was locally correct
(the transport fact IS post-install) but conflated post-INSTALL with
post-AGGREGATE_OWNERSHIP, placing phase B at :1257 when the fact may only need
to be after INSTALL. That conflation is the origin of the #1 boundary hazard.
The 6th red vindicates neither a seal redesign nor the exact z1400 placement — it
vindicates a MORE PRECISE staging: transports + owner assignment lift to before
aggregate_ownership IF transports are independent of it (Resolution 1).

CARRY — total freeze-order (POST-respin design item, NOT a blocker on this
respin; Architect evt_5786jn8ty5hkr, research completeness note evt_717p9fgzzg3sv;
Steward-held release lever). The frozen-phase-ordering closure applied
PER-BOUNDARY closes HS6#1 exactly as HS3/HS5 were closed at the response-context
plane — but a per-boundary closure does not guarantee against an HS7 at some
FURTHER boundary. The class-level guarantee is to establish the phase order as a
TOTAL, ACYCLIC FREEZE-ORDER over ALL derivations: a topological order in which no
derivation is finalized before any of its transitive inputs is frozen. Proven
structurally, that closes the predicate as a CLASS ("no such pair can exist") —
the "prove the closure of the set, not a better grep" bar and the strongest form
of the convergence ruling; it is the durable answer to "is this the LAST of its
kind," which the operator's priority-calculus on this WP will want. This is a
follow-up, not a re-open: the bounded correction above is the right immediate
move, and cutting a total freeze-order into scope now would over-reach a
green-able fix. DISPOSITION: after this respin lands green, the Steward assesses
whether the static_transition pipeline's phase order should be made an explicit
total freeze-order; the Architect frames it on the Steward's release. Held, not
released.

### SYMPTOM INVENTORY (Architect §1b)

1. HS3 `c93babfde` — FM1 leak: forward-declared owner whose transport caller is
   never consumed — keyed on an UNDER-classified install (P2 missed).
2. `cb68866e5` — closed-derivation invariant fires: record-derived P2 set empty
   at install, populated at validate — keyed on reading a POST-INSTALL fact AT
   install (phase-unstable).
3. `e193dc631` — over-admission traps consumed responses (px8f mixed :713;
   rt_parity fs_read/fs_write :568) — keyed on substituting a CAUSAL PROXY for
   the post-install fact (unsound superset).

### THE SHARED PREDICATE (stop iterating predicates)

Response classification is NOT YET A REAL, FROZEN, WHOLE-PROGRAM PHASE. Two
coupled gaps, both instances of that:

- **(A) PHASE/DATA.** The exact discriminator (coordinate-run source WITH vs
  WITHOUT a transport destination) needs the destination fact, resolved
  POST-install: `aggregate_ownership` at construction.rs:1249, transports at
  :1251. The classify runs at :1213 — structurally too early. Every red is a
  different way to fail to compute a post-install fact at install.
- **(B) DOMAIN/TOTALITY.** Classification's domain + its §7 total match is the
  modeled owner-call/response-Vis set, but the CONSUMERS reach the wider
  transport-SOURCE population (fs_read/fs_write). An unclassified member
  surfaces as a RUNTIME PatternMatchFailure — the §7 "exhaustive by
  construction" closure was scoped to a SUBSET, not the real population
  (COORDINATION §7 violated).

### THE RECUT — retain everything proved; replace the thing the predicate names

**RETAIN** (unchanged, authoritative): the first-class-Deferred direction,
classify-once (R2), residual-carries-routing (R3), the §7 total-match seats, the
AC-7 owner-call-coverage backstop, the closed-derivation validator. HS3's
direction STANDS. The HS3-recut Specialized-path detail below remains
authoritative for the Specialized path.

**REPLACE** the install-time, causal-proxy, subset-domain classify with a
POST-`aggregate_ownership`, DOMAIN-TOTAL classify. Two axes:

- **(i) PHASE.** Run the Deferred/Specialized DETERMINATION after
  construction.rs:1251, where the exact record-derived destination set exists.
  Preserve closedness by having `validate_static_response_context_plan`
  re-derive from the SAME post-install state (install and validate read
  identical finalized `aggregate_ownership`/transports). Sound because the
  destination fact is genuinely post-install
  (`build_checked_ih_environment_transports` reads `aggregate_ownership` +
  finalized — the z1315 cycle, reconfirmed).
- **(ii) DOMAIN.** Enlarge classification's domain to the WHOLE reachable
  transport-source population so domain(classify) superset-or-equal
  domain(every consumer), and make the total match SEALED over that population —
  an unclassified member becomes a COMPILE error, never a runtime
  PatternMatchFailure. This permanently closes the sig-3 class (fs_read/fs_write
  and any sibling).

### THE RESOLVED STRUCTURE — TWO-PHASE, owner-additive (ring measured; Architect confirmed evt_411q6cvg74yrn)

The move-wholesale-vs-two-phase measurement is DONE (runtime-implementer
`evt_1c798rdvzrcam`, Architect confirm `evt_411q6cvg74yrn`): it is TWO-PHASE,
owner-ADDITIVE (no retraction). The main-keying grep found exactly three
response-aware lowering seats — `response_disposition_at_operation_root`
(core.rs:13922), `_at_effect` (effects.rs:2205),
`is_static_response_selected_caller` (core.rs:8970) — all keying on
OWNER/disposition-row membership, none on `continuation_contexts` membership, so
a Deferred member routes `Some(Deferred)` -> main with no sealed marker needed
for response dispatch.

- **Phase A** (install-side, causal-prefix): mint owner-LESS
  `PlannedContinuationContext` entries over the has-K-unit population. `has-K-unit
  = matching = continuation_units().filter(producer_construct_origin ==
  vis_origin)`; `continuation_units` is fixed by continuation_specializations +
  the continuation-specialization ABI, both installed BEFORE the response
  install, so has-K-unit is causal-prefix-determinable and SPLIT-INDEPENDENT (no
  :1249 destination fact needed).
- **Phase B** (post-:1251): assign owners to Specialized (has-K-unit AND
  has-destination); leave P2 and P1 Deferred. Owner-ADDITIVE — phase B ADDS
  owners, never retracts.
- **Two-phase validator**: re-derive phase A from `continuation_units`
  (causal-prefix state) and phase B from finalized post-:1251
  `aggregate_ownership`+transports. This is the closed-derivation invariant,
  preserved by same-state re-derivation (AC-9).

Inertness (structural support, CONFIRM on the built recut via the two-phase
validator + native, per the implementer — the authoritative check, not the
read): a P2 Deferred owner-less context entry is enclosed by its K unit, which
lowers regardless (a transport caller is a real continuation unit), so it
materializes in that K's frame, not dangling; owner-additive leaves no unfilled
response slot.

The implementer's first EDIT is this two-phase structure; read-only pre-work is
exhausted. Sized to reach a releasable increment or a hard stop within an hour.

### RESEARCH DIVERGENCE (advisory evt_5ryjd4dk65x05; research advises, Architect rules)

ADOPT fully: DOMAIN TOTALITY and STRUCTURAL ENFORCEMENT (sealed sum -> compile
error) — exactly the sig-3 closure. DIVERGE on "keep classification an ACYCLIC
pre-install prefix / factor only a causal-prefix-pure subset upstream": that
presupposes the exact discriminator is causal-prefix-derivable, and it is NOT
(the transport-destination fact genuinely needs post-install
`aggregate_ownership` — the cycle `cb68866e5` proved by going phase-unstable and
`e193dc631` proved by going unsound on a causal proxy). The DETERMINATION runs
post-install; the "real frozen phase" is achieved by staging it AFTER
`aggregate_ownership` over the total domain, not by pretending it is a
pre-install prefix. If a genuine PRE-install consumer of the classification
exists, research's shared-subset factoring applies to THAT consumer only — the
ring names it or confirms none exists.

### #3 STAGING — MECHANISM-FIRST; #3 is a FOLLOW-UP, not a mechanism-SHA gate (Architect evt_1enehr9nxkjz9, refining evt_6v4yp7arvd4f4 + evt_53s106a7btrb8; implementer three-way table evt_6trkwrt5gh7ge)

The mechanism candidate does NOT un-ignore
`fs_read_at_malformed_offset_narrows_to_invalid_offset` /
`fs_write_..._narrows_to_invalid_offset`. The mechanism SHA is a PARTIAL-WP
candidate (§8); its OWN acceptance is the 5-red-chain closure — writeAll
fixtures AC-1..9 green + the AC-7 backstop reds correctly + NO regression to the
previously-passing native suite. #3 is NOT a gate on the mechanism SHA's
release. Do NOT un-ignore the fs_*_narrows tests on the mechanism SHA: a
fs-only red would conflate a rider-red with a mechanism-red and burn a SHA on a
bet that cannot be checked locally (native OOMs).

#3 rides as a SEPARATE follow-up un-ignore on the SAME branch AFTER the
mechanism greens, measured three-way on the BUILT recut (positive (a)/(b)
instrumentation rides the candidate; do not declare from static reads).
ATTRIBUTION: H_over (over-admission) is strongly indicated and STANDS
(runtime-leader `evt_1fb7kkw4bbxv5`) — the current PatternMatchFailure /
ResourceBodyResult signature is IDENTICAL across the two `fs_*_narrows` fixtures
that carried two DIFFERENT historical `#[ignore]` debts; two different old debts
do not converge to one signature, so H_debt-reappearance is refuted. This is
INDIRECT (negative-half elimination); the POSITIVE (a) instrumentation rides the
candidate. NOTE: an earlier revision of this frame claimed the fs_*_narrows
`#[ignore]` text drifted on 4e5481c57 — that was a RETRACTED grep-artifact
(implementer `evt_4ey9qwc18kfj9`, Architect concurred `evt_2yw2ptn6hj5rk`);
there was NO base drift and the elimination above carries. Two measurements:

- **(a)** fs_read/write's response is in (S_causal minus S_record) — instrument
  the recut classify, re-read on 4e5481c57.
- **(b)** the domain-total post-install classify makes it Specialized AND that
  Specialized lowering OBSERVES InvalidOffset (the un-ignored assertion), not
  the ResourceBodyResult frontier trap — native run.

THREE-WAY DISPOSITION (governs the FOLLOW-UP un-ignore; measured on the built
recut after the mechanism greens, nobody asserts #3 beforehand):

- **(a) AND (b)** -> fold the follow-up un-ignore in on the same branch.
- **(a) AND NOT (b)** -> over-admission was the immediate cause but the correct
  classify still hits the frontier -> the InvalidOffset product is a SEPARATE
  deliverable (RT-RESULT-CONTINUATION-BINDING-PROVENANCE territory); SPLIT via
  re-`#[ignore]`, not a respin.
- **NOT (a)** -> the response is not a coordinate-run source at all;
  over-admission never touched it, the un-ignore surfaced independent
  pre-existing debt -> SPLIT.

Do not conflate the follow-up un-ignore with the mechanism's own green — that
conflation is the failure this staging prevents.

### SEAL x PARITY-DEBT — sealed-as-classified, never carve out (Architect evt_53s106a7btrb8)

The domain-total seal (AC-8) is a STATIC classification-COMPLETENESS invariant
(every reachable member gets a disposition; an UNCLASSIFIED member is a compile
error), NOT a runtime-liveness invariant. Because Deferred is the TOTAL residual
arm (routes to main's pre-WP lowering), every reachable member classifies as
Specialized-with-owner OR Deferred by construction — there is no "unclassifiable"
member, so the seal is ALWAYS satisfiable (worst case Deferred). Consequences,
binding on the recut:

- Sealing does NOT convert un-closeable native-parity debt into a compile error;
  it converts an UNCLASSIFIED member (the sig-3 domain-bleed bug) into a compile
  error. A member carrying genuine orthogonal runtime parity-debt classifies
  fine (seal passes) and, if it still traps, fails at RUNTIME — surfaced by its
  test, handled by re-`#[ignore]`/split at the (b) layer.
- **Do NOT carve parity-debt members out of the seal.** Carving a reachable
  member out IS the subset-domain gap that produced sig-3. Domain-totality means
  NO reachable member sits outside the seal. The seal covers fs_read/write as
  classified members in ALL THREE sig-3 branches; the (a)/(b) measurement
  decides ride-vs-split at the runtime layer, never by shrinking the seal.

### ACCEPTANCE (this recut)

Retain the HS3-recut ACs 1-7 for the Specialized path. Add:

- **AC-8 (domain-total seal; two distinct domains — keep separate).** The SEAL
  DOMAIN is the WHOLE reachable response-Vis population (P1 no-unit UNION P2
  transport-caller UNION Specialized); the §7 total match over
  `Option<ResponseDisposition>` covers all of it, and an unclassified member is a
  COMPILE error — this is what closes sig-3 (consumers key on disposition rows
  over the whole population). The CONTEXT-ENTRY DOMAIN is has-K-unit (P2 UNION
  Specialized) ONLY; a unit-less P1 structurally cannot get a
  `PlannedContinuationContext` (it requires an enclosing_specialization) and must
  not — P1 is sealed via the Deferred arm (disposition = P1-Deferred), never via
  a context entry. The seal is STRICTLY WIDER than the context-entry set; that is
  the design, not a gap, and conflating the two domains is the trap. Control: a
  witness member outside the old owner-call subset classifies, and removing its
  disposition arm REDS at COMPILE time, not at runtime.
- **AC-9 (phase closedness preserved).** install and validate derive the
  Deferred/Specialized determination from IDENTICAL finalized post-install state
  (`aggregate_ownership`/transports); `validate_static_response_context_plan`
  passes by same-state re-derivation, not a second independent computation.
- **AC-10 (#3 is a FOLLOW-UP, not a mechanism-SHA gate).** The mechanism SHA
  ships WITHOUT the fs_*_narrows un-ignores (partial-WP, §8); its acceptance is
  AC-1..9 + AC-7 backstop + AC-NO-REGRESSION. #3 rides as a separate follow-up
  un-ignore on the same branch after the mechanism greens, three-way measured on
  the built recut (the e193dc631-era H_over elimination carries; positive (a)/(b)
  ride the candidate): (a)&&(b) folds the un-ignore in, else split. Not a gate on
  the mechanism's release.
- **AC-NO-REGRESSION.** Green across the transport-source population in CI (px8f
  + rt_parity native shards), not just the modeled owner-call fixtures. Targeted
  `scripts/ken-cargo` locally; whole-suite is CI's.

Base: fresh from `origin/main` (current tip `4e5481c57`), NEW SHA. `e193dc631`
dead.

## RECUT — HS3 structural closure: first-class Deferred residual (Architect ruling evt_5yjjsrhpmt204)

**This section is the CURRENT governing contract and supersedes the mechanism
below wherever they conflict.** It is the Architect's HS3 structural-closure
ruling (`evt_5yjjsrhpmt204`, 2026-09-02), issued on the research prior-art
advisory (`evt_3z83vwpenscft`, SHA-256
370795b09f783f52d3650a2888e96c2ee4b14c7f2ce7e6039bf726b0af3b576e — advisory
only, the call is the Architect's). It RETIRES the point-fix chain: no further
patch to the absence-based decline. The everything-below-here detail on the
**Specialized** path is RETAINED and remains authoritative for that path; only
the **decline / residual** handling is replaced.

### AMENDMENT (Architect ruling evt_4ar3rxzrra5v4) — Deferred = P1 UNION P2; discriminator = caller-consumption. READ FIRST.

A runtime-implementer pre-implementation hard stop (evt_6cp1w4mac9jaa /
evt_33teszvwarz6; HS4 in the chain, NOT a CI-red and NOT held against the chain —
stopping before a runtime-unverifiable blind push is the correct move) CORRECTED
the premise of this recut, and the Architect adopted the correction. The recut
scope stands, amended as follows; this block governs the Mechanism section below
wherever they differ.

**Premise correction.** The original recut said "the ledger computes the
Specialized side soundly; the defect is the absent residual." The second half is
refuted by provenance: `StaticResponseDeferred` is produced ONLY at
`core.rs:13922` (Construct) and `effects.rs:2205` (Effect), both gated on the
ledger-SPECIALIZED set (`is_static_response_operation_root` /
`is_static_response_effect` reading `static_response_continuations`). So HS3-b's
leaking response is INSIDE the specialized set — a present-but-unconsumed
placeholder — not the absent `1229` residual. **Deferred therefore has TWO
sub-cases, both -> residual:**

- **P1** — no continuation unit (`matching.is_empty()`, the `1229` residual /
  absent complement that Q1 declined). The original Deferred captured only this.
- **P2** — has a unit + owner + `StaticResponseDeferred` placeholder, but its
  selected caller is never retargeted/consumed (HS3-b; same root as HS3-a
  `disposition=None`). Capturing only P1 would compile clean and still leak =
  HS4.

Complete Deferred = **P1 UNION P2**.

**The discriminator (answer (b), with rigor).** `classify` emits **Specialized
IFF (has a unit) AND (its selected caller will be consumed — retargeted to a real
`DirectCall`/`ComposedCall`)**; otherwise **Deferred**. The consumption fact is
today settled at lowering as `CandidateDisposition`
(`DirectCall`/`ComposedCall` vs `InlineNoCall`/`TransportDormant`); it must be
HOISTED to planning so `classify` decides it ONCE (R2). Hoisting changes WHEN the
fact is computed, not the emitted code, so **D0 holds** — Specialized still emits
direct calls, no selector, no environment transport.

Do NOT classify on a syntactic proxy (e.g. the D3 `CheckedIhCapturedEnvironment`
shape) UNLESS that shape is PROVEN equal to caller-non-consumption. A proxy that
merely correlates recurs as HS4 (fix the class, not the instance). If the
retargetability predicate turns out to BE a nameable static shape, that is the
concrete form of (b) and (a)/(b) coincide — but only with the equality proof,
never assumed. Answer (c) is REFUTED by the provenance above (both production
sites are specialized-gated). Do NOT block on the OOM'd native trace for this
decision; the static provenance settles it. Cheap confirmation short of the full
native suite: a planning-time classification log on the `writeAll` fixture — does
its response have a unit, and is its caller `InlineNoCall`/`TransportDormant`? —
confirms P2 directly.

**GUARDRAIL (the one thing that could still obstruct (b)).** The ledger already
inspects the caller edge (rejecting a non-ordinary-callable continuation edge) —
NECESSARY but not SUFFICIENT for consumption; the missing piece is whether that
ordinary caller is retargeted to a real call vs stays
`InlineNoCall`/`TransportDormant`. **VERIFY that distinction is derivable from
PLANNING-available facts** (the caller edge's shape; `CandidateDisposition` keys
on caller shape the ledger already has) BEFORE threading. If it genuinely depends
on an emission-time-only decision not derivable at planning, **HARD-STOP back to
the Architect** — that is a real obstruction to R2 (the partition cannot be
classified once) and the Architect re-rules the structure, not the implementer.

**Integration.** `classify` (extended ledger) emits Specialized only for
**P0 = specializable-AND-consumed**; Deferred for **P1 UNION P2**. The Deferred
verdict gates ALL downstream: no owner forward-declaration for a Deferred response
(closes HS3-a), no `StaticResponseDeferred` placeholder emitted for it (closes
HS3-b), lowers to main's pre-WP path carrying its payload (R3). The proved
Specialized path (feasibility-ledger specialized computation,
`verify_static_response_finished_body`, Q1/Q2) is retained unchanged for P0.

**ADDED AC (AC-7 below; sharpens AC-5).** Pin `classify`'s agreement with
lowering-time `CandidateDisposition`: a response classified Specialized MUST have
its caller consumed at lowering (`DirectCall`/`ComposedCall`), and a Deferred
response MUST NOT acquire an owner or placeholder. A control that REDS if
`classify` says Specialized while lowering finds `InlineNoCall`/`TransportDormant`
(the HS3-b leak reintroduced) — the soundness pin that makes `classify` a
faithful planning-time predictor of the lowering fact.

### Why the recut (the hard-stop chain, one predicate)

Three CI-reds on the px8f/rt_parity native population, each a real distinct
defect, all one predicate: the deferred/declined response `Vis` was modeled as
an **absence** (no demand, no owner, empty set, a bare `continue`) that each
downstream stage had to independently reconstruct and route to main's lowering.
Each point-fix un-masked the next consumer:

- HS1 (Q1): demand filter ABORTS a declined deferred-frontier `Vis`
  (`SsaInfeasible` -> fatal backend abort).
- HS2 (found 0): the px8-ds mutation helper requires an owner and finds zero for
  a fully-fallback program (test-support-only, production-inert).
- HS3-a (`rt_resource_release_carried_observe`): a forward-declared response
  owner has no verified selected incoming call (`disposition=None`).
- HS3-b (`writeAll`): `unsupported runtime-IR lowering: StaticResponseDeferred`
  — a deferred host response is compiler control with no supported lowering arm.

A census/grep cannot close this — it failed twice. The tell: HS3-b names
`StaticResponseDeferred`, a runtime-IR variant that **already exists** — so
"deferred" is partially first-class but not exhaustively handled.

### Mechanism — reify the specialize/residualize partition, classify ONCE

Compute a positive two-valued classification on the response IR, once, consumed
by total matches everywhere:

```rust
classify : ResponseVis -> Disposition          // one pass; positive verdict
lower    : Disposition  -> RuntimeIR            // total match, NO `_ =>`

enum Disposition {
    Specialized { owner, captures, k_route, .. }, // the proved path, unchanged
    Deferred    { payload },                      // routes to main's pre-WP lowering
}
```

- `classify` is the EXISTING feasibility ledger EXTENDED to emit a populated
  `Deferred{..}` instead of an empty complement / no-owner / `None`. The ledger
  already computes the Specialized side soundly; the whole defect is that its
  residual output today is the absent complement rather than a constructed object.
- `Deferred` is a constructor of the SAME sealed sum the already-half-born
  `StaticResponseDeferred` (HS3-b) belongs to. Promote it to a FULL peer at every
  stage: planning/classify, forward-declaration, caller-edge verification,
  retained-unit declaration, runtime-IR lowering. Each stage matches
  `Disposition` with **no catch-all**, so an unhandled stage is a Rust COMPILE
  error (COORDINATION §7 sealed-enum), not a CI-red. This converts "a census must
  find every consumer" into "the type enumerates the consumers for you" — the
  reason the census failed twice and a sealed variant cannot.

### Three binding requirements (each kills one symptom face)

- **R1 — REACHABILITY IS ORTHOGONAL TO COLOR.** Demand/reachability and
  Specialized-vs-Deferred are separate analyses; a `Vis` can be reached AND
  Deferred (the normal residual). HS1 conflated "no static demand" with "not
  present" and aborted. A reached-but-Deferred `Vis` is expected and MUST pass to
  residual lowering, never abort.
- **R2 — CLASSIFY ONCE, ON THE OBJECT.** The verdict lives on the IR object as a
  constructor, not re-derived from local negative evidence at each consumer. This
  removes the reconstruct-obligation from every stage simultaneously — it is the
  closure, and it is why HS2 (found 0) and HS3-a (`disposition=None`) both vanish
  rather than getting a third and fourth patch.
- **R3 — THE RESIDUAL CARRIES ITS PAYLOAD.** HS3-b is a tag with no
  payload-carrying case, so lowering has nowhere to send it. `Deferred{payload}`
  carries exactly the data to route to main's existing pre-WP lowering (the path
  that compiled and ran at `4a088d8aa`), so its lowering arm is a real
  translation, not an unsupported stub.

### Boundary — DO NOT VIOLATE D0 (evt_29jfzzw9j5xjz)

The sealed-tag + total-match STRUCTURE is for the **Deferred** case ONLY. The
Specialized population stays exactly as proved (the mechanism below): direct
calls to the exact K context, the finished-CLIF read-back
(`verify_static_response_finished_body`), NO runtime selector / K tag /
environment aggregate / closure word / shared apply. A tagged variant carrying an
environment is the closure-conversion form D0 excluded; it must never touch the
Specialized side. Only `Deferred` is tagged, and `Deferred` lowers to the
pre-existing main path, which introduces no selector.

### Retain vs replace

- **RETAIN** everything proved: the feasibility ledger's Specialized-side
  computation, the finished-CLIF `Ret`/`Trap` read-back, and the Q1/Q2
  Specialized logic — all correctly reviewed, none is the defect. The entire
  Specialized-path detail below this section is retained and authoritative.
- **REPLACE** only the absence-based decline: Q1's `continue`, HS2's
  empty-substitute, and every stage's implicit "no owner => fall through" become
  one populated `Deferred` threaded by total matches.

### Acceptance criteria — the totality proof (carry ALL SIX)

1. **Congruence before the passes run.** Every `ResponseVis` receives exactly one
   of Specialized/Deferred; no third "unclassified" leak. Assert exhaustiveness of
   `classify`.
2. **Per-stage §7 control.** At EACH stage, deleting/adding a `Disposition`
   variant reddens the Rust build — a compile-time pin per stage, not a runtime
   test. This is the closure's own proof that no stage silently drops the residual.
3. **Positive Deferred program.** The `writeAll` deferred-frontier fixture (the
   `4a088d8aa` shape) COMPILES and RUNS through main lowering with UNCHANGED
   effect order — a real "still compiles+runs" positive control.
4. **Mixed program.** One unit carrying BOTH colors (a Specialized response and a
   Deferred response together). Polyvariance is only real when both coexist; a
   single-color program cannot discriminate genuine threading from a flag.
5. **Specialized-through-Deferred-arm control.** A test that REDS if a Specialized
   response ever flows through the Deferred lowering arm — proving the proved path
   is untouched and the D0 boundary holds.
6. **CI-native whole-binary population green on the exact SHA:**
   `px8f_buffer_native`, `rt_parity_native` all shards,
   `rt_resource_release_carried_observe`, all 8 test shards — the authoritative
   close that caught HS1/HS2/HS3.
7. **classify/lowering agreement pin (AMENDMENT AC, evt_4ar3rxzrra5v4).** A
   response classified Specialized MUST have its caller consumed at lowering
   (`DirectCall`/`ComposedCall`); a Deferred response MUST NOT acquire an owner or
   `StaticResponseDeferred` placeholder. A control REDS if `classify` says
   Specialized while lowering finds `InlineNoCall`/`TransportDormant` (the HS3-b
   leak reintroduced). This is the soundness pin proving `classify` is a faithful
   planning-time predictor of the lowering-time `CandidateDisposition` fact.

Also fold in the **deferred option-2 coverage fixture** (`evt_55jt2yydg0661`)
while the response-IR is being restructured — it is the same surface.

### Scope, gates, base

- **Scope:** elaboration/backend only (`crates/ken-runtime`). NO kernel, TCB,
  `/spec`, or `/conformance` change; research confirmed no operator escalation on
  the mechanism.
- **Base / fixed inputs:** cut fresh from `origin/main` **`4a088d8aa`** (the
  pre-WP baseline where the node is held and the `writeAll` deferred-frontier
  program compiled and ran). This supersedes the stale `ad191d1c2` base cited
  below.
- **Candidate is NEW with fresh gates:** Architect soundness + runtime-QA +
  CI-native on the exact SHA. NO prior approval carries. On a gated candidate:
  Steward M1-M4 -> lieutenant M5-M9. Node stays HELD at the pre-WP baseline until
  the recut lands.
- **§1a bookkeeping:** HS3 is DISCHARGED by this ruling; the next re-trigger is
  HS6.

## Authoritative contract (Specialized-path detail — RETAINED under the recut above)

The Architect mechanism ruling **`evt_29jfzzw9j5xjz`** is the authoritative,
byte-level contract **for the Specialized path**. This node folds its structure,
types, and controls for release; where any detail here is thinner, the ruling
governs. Do not re-derive. The **decline / residual** handling in the sections
below is SUPERSEDED by the RECUT above (first-class `Deferred`); the Specialized
mechanism, representation, feasibility trichotomy, emission seam, and Specialized
controls remain in force.

## AMENDMENT — context-demand extension (Architect evt_4ta6cchxvjrrt); CP1 CORRECTED

The CP1 `SsaInfeasible` on both FsReadAt rows was a PHASE-ORDERING artifact, not
genuine infeasibility, and does NOT go to the operator. `intern_generated_contexts`
at the held checkpoint interned contexts only from the pre-existing
`PlannedContinuationSpecializationCall` (old-caller) population; the response-owner
call does not exist yet, so `continuation_context_for(...)` answered "was some old
caller already enough to mint this target?" — the zero is over the old caller
population, not over K or its ABI inputs. The two FsReadAt rows are SINGLETON-K
rows; the already-issued `PlannedContinuationContext` contract is exactly the
missing target shape (Parameter run = raw K arity + K captures; Capture run = the
K specialization's ordered continuation inputs). No new runtime representation or
ABI family.

Extend the SSA planner:

1. Split response planning into a PRE-CONTEXT DEMAND phase and a POST-CONTEXT
   RESOLUTION phase. Pre-context derives every response producer/K row from the
   semantic graph + the just-built continuation specialization/call population and
   finishes capture/source validation (a count is not enough).
2. Add a typed `StaticResponseContextDemand` keyed by the existing pair
   `(K ContinuationSpecializationId, k_body_origin)`, carrying the response row
   identity for closure checking — NOT a second context-identity domain or a
   response-specific ABI kind.
3. Intern ordinary `PlannedContinuationContext`s from the UNION of existing
   causal-call demands + response demands: intern the existing call population
   FIRST (preserving existing context IDs), then append new unique response keys
   in deterministic response-row order. Same key reuses one context; same key with
   disagreeing worker/input schema is a planner error.
4. Build/install/finalize the existing context ABI from the K specialization's own
   `worker` + `continuation_inputs` authorities — do NOT reconstruct schema from
   response syntax.
5. Resolve each response demand by exact key to the now-issued
   `ContinuationContextId`, then publish `StaticResponseContinuation`. A missing
   context is now a planner population-closure ERROR, not `SsaInfeasible`.
6. Keep `SsaInfeasible` ONLY for the real semantic arms: an incoming edge carries
   multiple/opaque K values, or a K capture/input cannot be expressed as one
   explicit static frame source. Continue the all-producer walk PAST the FsReadAt
   rows (later rows are not assumed feasible).

Still compile-time SSA/lambda lifting: Function identity fixes K; response +
captures/inputs stay explicit slots; the call target stays an ordinary
`ContinuationContextId`. No K tag / closure word / environment aggregate / apply
dispatcher / code pointer / runtime selector; no public ABI change; no kernel
change; no spec commitment — NO new operator fork.

Checkpoint correction (supersedes the "Deliverable" list below where they differ):

- **CP1** completes only when the full read/write all-producer population has
  EITHER a fully-validated context demand for every singleton-K row OR a real
  typed dynamic/non-expressible `SsaInfeasible`. "No context existed before the
  new edge" is NO LONGER an infeasible arm.
- **CP2** interns the union context population, installs its existing ABI,
  forward-declares response owners, statically retargets exact callers.
- **CP3** defines the response owner and emits the exact context call after
  validation.
- No emitted context discharges reachability by declaration; at the atomic tip
  every newly demand-issued context must have >=1 selected response-owner call
  (delete it -> population-closure gate reds).

Added controls (with the existing mutation grid): READ demand key
`Specialization(0)` / body `766`, WRITE `Specialization(0)` / body `1075` — both
resolve to planner-issued contexts and the all-producer walk continues; delete
only the response demand -> FsReadAt row reds before emission; duplicate demand ->
one context, not two; vary body / K identity / capture source / continuation-input
source -> reject the disagreement; prove existing causal-call context IDs +
descriptors unchanged when response demands are appended; remove/retarget the sole
response-owner call -> reject declared-but-unentered; call raw worker -> reject
wrong ABI; RETAIN a genuine dynamic-K row that returns typed `SsaInfeasible` (so
this extension does not make the fallback arm unreachable).

Held checkpoint `48fa6c9d6` remains evidence, not a candidate; `dac8edab`
diagnostic only.

## The mechanism

Select **polyvariant, compile-time response-owner specialization**. For every
statically attributable `(response producer, K)` pair, emit one Function whose
code identity fixes K, whose frame carries all K captures and enclosing
continuation inputs as **explicit ABI slots**, and whose body performs host
dispatch/validation and directly calls that K with the exact response as
**operand 0**. No K tag, closure word, apply dispatcher, environment aggregate,
code pointer, process-global slot, or runtime selector exists.

**Why the terminal D0 negative does not refute this.** D0 proved the
*unspecialized* graph has no future owner joining response, K, captures, and
target. The missing object is a **statically keyed response-owner Function plus
its retargeted caller** — not an index into the existing operand run. Concretely:
WRITE `Vis` 1250 has K closure 1246 / body 1238, arity one, seven captures,
target context 0; READ application 138 and WRITE application 175 each select one
live K. Do not patch context 0 (unentered), context 1 (parameter 0 is the prior
response), or app486 (precedes the future response).

## Required planner representation

```rust
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StaticResponseContinuationId(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseCapture {
    ordinal: u32,
    origin: StaticOriginId,
    source: ContinuationSourceCoordinate,
    producer_abi_slot: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseContinuation {
    id: StaticResponseContinuationId,
    base_owner: ContinuationEmissionOwner,
    producer_call_origin: StaticOriginId,
    response_origin: StaticOriginId,
    vis_origin: StaticOriginId,
    k_identity: ContinuationCallIdentity,
    k_closure_origin: StaticOriginId,
    k_body_origin: StaticOriginId,
    k_context: ContinuationContextId,
    captures: Vec<StaticResponseCapture>,
    continuation_inputs: Vec<(u32, ContinuationSourceCoordinate, u32)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StaticResponseProducerSpecialization {
    base_owner: ContinuationEmissionOwner,
    continuation: StaticResponseContinuationId, // singular by construction
    header: AbiFrameHeader,
    slots: Vec<AbiSlot>,
}
```

`BoundaryClosureEnvironment` and `ContinuationCallIdentity.worker` remain the
independent body/arity/capture-schema authorities; their runtime environment
record is **not** emitted. `ContinuationContextId` is the K target (it preserves
the K parameter/capture run plus enclosing inputs); calling a raw worker is a
wrong ABI. "Explicit parameter" means a named `AbiSlotKind::Parameter` or
`Capture` and a mapped operand on every direct edge. `unit_signature` stays
`(frame_ptr, services_ptr) -> i64` — no public ABI change.

## Feasibility and specialization algorithm (the load-bearing trichotomy)

Build the relation independently from both ends: host-response producer/caller
edges from the closed semantic graph, and K binder/body/context/capture schema
from the exact `Vis` recursive field plus `ContinuationCallIdentity`. Group by
unspecialized producer P.

- **`|K(P)| = 1`** → emit one specialization.
- **finite `|K(P)| > 1` with a singleton K on every incoming edge** → emit one
  specialization per `(P,K)` and statically retarget each caller. At a join,
  split critical edges or sink the wrapper into predecessor arms. This is
  ordinary polyvariant SSA / lambda lifting, **not** closure conversion.
- **an incoming edge still carries multiple Ks as data, an opaque/higher-order
  parameter supplies K, or capture sources cannot be expressed as explicit frame
  slots** → return a typed **`SsaInfeasible`** record naming that exact edge and
  STOP. Do not choose the first K; do not add a selector-plus-environment. The
  Steward routes that finding to the operator **before** the fallback is
  selected.

Use a memoized worklist keyed by `(base owner, producer edge, static K
identity/schema)`, inserting before descent. Clone an SCC once per static key;
capture values never enter it. A numeric clone cap is a resource refusal, never
permission to merge keys.

## Required emission seam

Forward-declare every response specialization with the existing unit bundle
before defining bodies. Each generated Function declares its own `FuncRef` for
`k_context`. The selected Function contains `lower_process_host_effect` through
`ken_host_dispatch_v1`, status/tag/resource-error validation, and exact
`Lowered::HostResult` materialization. **Do not** factor HostResult across a new
helper ABI in this repair — clone the lowering IR so response and K coexist in
one Function (a later shared-H optimization needs its own explicit checked
response ABI). After validation and before returning HostResult or entering
answer collapse, invoke:

```rust
fn call_static_response_continuation(
    &mut self,
    builder: &mut FunctionBuilder<'_>,
    route: &StaticResponseContinuation,
    response: LoweringOperand,
) -> Result<CheckedIhApplicationResult, CraneliftBackendError>;
```

It requires the specialized owner and exact `response_origin`; reads captures
only from mapped current-Function ABI slots; assembles `[response, capture_0,
..., capture_n, continuation_inputs...]`; calls the Function-local context
target; checks Trap/status before Result; constructs `CheckedIhApplicationResult`
only from that call. Raw `HostResult` may not leave; only K Result reaches Ret.

## Deliverable — one atomic SSA repair, four checkpoints

Built on one branch from the clean held checkpoint `ad191d1c2`. **No QA,
Decision, publication, or merge before the atomic tip.**

1. **Static feasibility ledger + context-demand validation (CORRECTED — see the
   AMENDMENT above).** Publish every producer/caller-edge → exact K/schema row for
   both fixed products. For every SINGLETON-K row, derive and fully validate its
   typed context DEMAND (capture/source validation, not a count). **No production
   emission yet.** CP1 completes only when the full all-producer population has
   either a validated context demand for every singleton-K row OR a real typed
   dynamic/non-expressible `SsaInfeasible` (multiple/opaque K, or a capture/input
   not expressible as one explicit static frame source). "No context existed
   before the new edge" is NOT infeasible — it is a demand to intern at CP2. ONLY
   a real dynamic/non-expressible `SsaInfeasible` is the hard stop the Steward
   routes to the operator.
2. **Typed specialization population** — fixed-point/SCC closure, explicit ABI
   slots, forward declarations, caller retargeting. Prove every emitted
   specialization has at least one selected incoming caller; a
   declared-but-unentered Function is non-discharge.
3. **Response-local K application** at the validated host seam
   (`call_static_response_continuation`), target-context call, Trap-before-Result,
   exact Ret route.
4. **Full controls and the sole atomic candidate** — the acceptance boundary and
   mutation grid below. The tip cuts the SOLE candidate (Runtime QA + Architect
   on the exact SHA, then Steward M1-M4, lieutenant M5-M9).

## Acceptance boundary

Both `rt_parity_fs_read_at_offset_single` and `rt_parity_fs_write_at_offset_single`
reach `InvalidOffset` with unchanged effect order. One dynamic row must join:
selected incoming caller Inst, specialized Function, exact host dispatch,
validated response Value, ordered explicit captures, local K `FuncRef` / call
Inst, Trap-checked K Result, and exact Ret argument.

## Mutation grid (each negative must reach and red for its OWN claim, then restore exactly)

Independently: drop / duplicate / vary a producer-to-K row; merge two K keys;
restore the shared unspecialized producer; remove or retarget an incoming caller;
substitute context 0 without a caller; replace response with operation, prior
response, or app486 environment; drop / permute / vary every capture and
continuation input; call raw worker instead of context; omit / duplicate the K
call; move it before validation or after collapse; bypass Trap-before-Result;
vary Ret. **Include one statically shared two-K producer whose callers split into
two direct specializations, and one genuinely dynamic-K negative that yields
`SsaInfeasible`.**

## Fallback disposition

The invocation-owned runtime closure (`RT-COMPOSED-RETURN-RUNTIME-CLOSURE`,
ruling `evt_3j6vshm83rk5q`) is the FALLBACK, held draft. It is selected ONLY if
checkpoint 1 returns `SsaInfeasible` and the operator so rules. It is NOT built
in parallel; halted scratch `aee8c9408` is evidence only.

## Contention

Single-writer runtime lane, priority lane 1. Touches the Cranelift backend
specialization/emission and the response seam (`lower_process_host_effect`) — no
overlap with the doc track (`library/`, `agent/`) or the language lane's FO
adequacy work. No kernel crate touch (`crates/ken-kernel` byte-unchanged); no
`/spec` change; no public ABI change. Base is the held runtime branch
`ad191d1c2`, not `main`; the sole candidate merges at the atomic tip.
