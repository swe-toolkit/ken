---
id: RT-COMPOSED-RETURN-FORWARD-RET-EDGE
title: "Close the composed-return Tail wall by shape (a): a governed Tail result takes ONE certified forward SSA edge from the existing producer to the existing shared Ret block, bypassing the source-machine answer collapse, constructor transfer, active carried backedge, and checked fallback. The single relaxed constraint is the lossy middle; everything else (spec, kernel, ABI, runtime state) is unchanged. Flips the base-red Tail `ResourceBodyResult` `PatternMatchFailure` rows to exact `InvalidOffset` on the two `SourceFormat::Ken` witnesses."
status: active
owner: runtime
size: L
gate: none
tier: T1
depends_on: []
blocks: [PX8]
github: null
origin: "Operator ruling 2026-08-30 (this session, verbatim: \"use shape (a)\") FUNDED the shape-(a) relaxation after the constraint-differential report (docs/program/rt-composed-return-constraint-differential-report.md, merged 5b20fe84f) found BOTH native walls INCIDENTAL, not spec-mandated. The build design is the fresh operator+Architect decision the report fed; Architect ruling evt_70n2y6s9wanf9 (base origin/main 7d807a78e, tree 37da2a975) mints this fresh T1 node and closes the held authority-only build RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD (refuted/superseded without implementation). Steward-filed and released per COORDINATION section 2."
---

> # ACTIVE — lane 1, funded shape-(a). D1+D2 LANDED; the old D3 release is
> STALE, RETIRED, superseded by D3-RECUT (fresh atomic closeout; see the
> governing section below). `active`.
>
> The operator funded shape (a); the Architect designed it (`evt_70n2y6s9wanf9`)
> and minted this fresh node in place of the refuted authority-only build. This is
> a T1 soundness-bearing native-lowering build; the runtime ring's standing
> hard-stop protocol applies. A design fork HARD-STOPS to the Architect.
>
> **D1 (sink seam, byte-inert) MERGED** — squash `f193b074e` (2026-08-30),
> candidate `dd0b824272`, Decision `dec_6gctzq9sann54`, gates Runtime QA
> `evt_5synxy6g2q9yy` + Architect `evt_5svnf4fdqdes5`/`evt_70h7f8nyv3c8h`, all four
> blobs Steward+lieutenant+Adversary verified, Adversary M8 NO OBJECTION
> (`evt_5865012wcz8qa`: the seam is a deliberately-inert validation gate, byte-inert
> differential proves identical executable bytes). Accepted PARTIAL — node stays
> `active`. **D2 (authority/plan, byte-inert) MERGED** — squash `e7caf60be`
> (2026-08-30), candidate `41e46ccf`, Decision `dec_5xvyb20rjjht5`, gates Runtime
> QA `evt_498ky02hdtvm8` + Architect `evt_rnk7mnxfcxfm`; the seven-mode /
> 14-native-child control split, byte-inert (production D2 row + D1/D2 lowering
> bytes unchanged). Accepted PARTIAL — node stays `active`. **D3 (atomic
> activation) RELEASED** (Steward sequencing, 2026-08-30, after D2 landed — the
> frame's "separate explicit release" gate): consume the proof at
> `source.rs:4369-4373`, emit the returned-word jump, retire the old Tail
> header/fallback observer claims, add the two durable `InvalidOffset` products,
> and run affected closure. D3 code, causal controls, and products land together
> (AC-CAUSAL-PAIR / AC-FORWARD-SSA / AC-PRODUCTS-EXACT). Base is current main
> `e7caf60be`; D1/D2 moved the source.rs/core.rs/aggregates.rs coordinates —
> re-measure before acting.

## D3-RECUT — fresh atomic closeout (GOVERNING; supersedes the banner's D3 release)

**This is the current governing contract for the terminal deliverable of this
node.** Architect (B)-ruling `evt_3hsmvkh5za39d` (2026-09-03) + the option-(a)(i)
build design `evt_381dzjykr4knn`/`evt_5963far74b735` (2026-09-01); runtime-leader
sequencing/vehicle call `evt_7gmdjyjpvyc1m`. Released by the Steward.

**Why the banner's D3 release is retired, not resumed.** The old D3 predates (a)
the 12-stop LIVE-K/checked-IH D0 measurement chain that ran 2026-08-30/31 on this
exact seam and terminated at NO_UNIQUE_EDGE (`evt_mx6scjje1yjp`,
Architect-accepted; the boundary is closed and this closeout does NOT re-open
it), and (b) RT-SSA's landing (`ad9905a7e`), which rewrote `responses.rs` (+2590,
the WP's own core production file) and shifted the
`source.rs`/`core.rs`/`aggregates.rs` coordinates the old D3 named. Cut D3-RECUT
FRESH from the current `origin/main` tip and RE-MEASURE every coordinate below; a
line number here decays.

### What RT-SSA did and did NOT do (Architect item 1, grounded on `f84e231dc`)

RT-SSA landed the response-owner SPECIALIZATION component ONLY: it decides and
installs WHICH response owners specialize (k_specialization,
specialized-vs-deferred owner partitioning, worker-body-origin pairing).
`responses.rs` contains ZERO
`InvalidOffset`/`ResourceBodyErr`/`ResourceBodyOk`/`ExitCode`/`PatternMatchFailure`
construction. It does NOT construct the exact `InvalidOffset` product nor close
the errored composed return out to the process exit. That closeout is this WP.

### Mechanism (Architect item 3; re-measure coordinates at your base SHA)

Consume `ComposedReturnForwardRetAuthority` (Architect read `core.rs:12822`,
move-only) TOGETHER WITH the specialized response owner's Trap-checked Result,
and branch to the exact function-local shared `Ret` block — with PER-ARRIVAL
PAIRING and EXACT-ONCE CLOSEOUT constructing the exact read/write `InvalidOffset`
PRODUCT at the `RoutedAnswer`/constructor collapse, so
`ResourceBodyErr(InvalidOffset)` reaches the exit instead of the
malformed-`ExitCode::Failure` payload trap. Also complete the forward-Ret-edge
CONSUMPTION if D1/D2's shared-block wiring does not already supply it (Architect's
conditional — measure it on the built recut, do not assume).

The pieces exist but are NOT wired end-to-end for the fs-at-offset error arms
(re-measure): the synthesized `ResourceInvalidOffset` constructor role
(`effects.rs:3402`, semantic_ir `ResourceInvalidOffset`); the Int narrow-failure
lane feeding `InvalidBounds`/`InvalidOffset` (`effects.rs`
`narrow_positioned_int_seat`); the move-only `ComposedReturnForwardRetAuthority`
(`core.rs:12822`).

### Boundary — this is option (a), NOT option (b) (Architect item 4)

The closeout introduces NO runtime callable/continuation identity object, NO
`HostResult` reuse, NO seed substitution, NO side table, NO stored Cranelift
value, NO cross-Function `FuncRef`, NO new header/frame field, NO second
selector. It is the tag-only, co-located forward-SSA-edge mechanism the operator
funded; it creates no later owner and does not re-open the NO_UNIQUE_EDGE
boundary. No TCB growth, no spec/kernel/ABI/wire change, no operator loop. A
design fork that would need any of the option-(b) machinery is a HARD STOP to the
Architect, never a workaround.

### Acceptance (predicate form; the fixtures are the acceptance, not a roster)

- **AC-INVALIDOFFSET-FLIP (positive).** Un-ignore and green the four
  `fs_*_narrows` differentials on the `assert_narrowed_alike` (interp==native,
  exit 0 iff exactly the expected variant): `fs_read_at_malformed_offset`
  (`rt_parity_native.rs:4066`), `..without_read_right` (`:4110`),
  `fs_write_at_malformed_offset` (`:4144`), `..without_write_right` (`:4161`).
  Each now observes `InvalidOffset`, not the malformed-`ExitCode::Failure`
  `PatternMatchFailure` trap. Re-measure the four line numbers at your base.
- **AC-SWEEP-INVALIDBOUNDS.** Sweep in
  `fs_read_at_malformed_window_narrows_to_invalid_bounds` (`:4079`) in the SAME
  un-ignore — it rides the same Int narrow-failure lane (`InvalidBounds` is that
  lane). Un-ignore + green it here.
- **AC-EXCLUDE-BUFFER-ALLOC.** Do NOT bundle
  `buffer_allocate_malformed_capacity_narrows_to_invalid_bounds` (`:680`) — a
  different owner (RT-SITEOP-CARRIED-WITNESS D2: a carried recursive hypothesis
  is an eliminated value, not a callable). It stays `#[ignore]`d; touching it is
  a scope violation.
- **AC-EXACT-ONCE / AC-PER-ARRIVAL (soundness controls).** Per-arrival pairing is
  exact-once: a control that reds if a closeout fires twice or zero times for one
  arrival; a control that reds if the forward-SSA edge is taken without the
  Trap-checked Result gate. The `InvalidOffset` product is constructed at the
  collapse, not a reused `HostResult`.
- **AC-NO-BOUNDARY-REOPEN (boundary control).** No new runtime callable/identity
  object, side table, stored value, cross-Function `FuncRef`, header/frame field,
  or second selector appears in the diff — the option-(a) boundary holds.
  `crates/ken-kernel` byte-unchanged; no `/spec`, no `/conformance`.
- **AC-NO-REGRESSION.** Green across the transport-source population in CI (px8f +
  rt_parity native shards); targeted `scripts/ken-cargo` locally, whole-suite is
  CI's.

### Size, gate, base

- **SIZE: L** (~1-2 T1 held checkpoints — the WP3 closeout, plus the
  forward-Ret-edge consumption if D1/D2 did not fully wire it). Same atomic merge
  unit as the original three-checkpoint decomposition; the un-ignore is this WP's
  ACCEPTANCE, not a separate trivial follow-up.
- **GATE: runtime-qa+architect.** The Architect is the required soundness
  reviewer on the candidate (their own commitment, `evt_3hsmvkh5za39d`); Runtime
  QA; standing Adversary independent. On the candidate: fresh Architect + Runtime
  QA on the exact SHA, then Steward M1-M4, lieutenant M5-M9. `gate: none` in the
  operator sense (no TCB touch, no operator authorization — option (a) is already
  funded).
- **BASE:** cut fresh from the current `origin/main` tip; re-measure
  `ComposedReturnForwardRetAuthority`, `ResourceInvalidOffset` constructor role,
  the Int narrow-failure lane, and the five test line numbers before the first
  edit.
- **Grounding honesty (Architect §7a):** the Architect grounded from source
  (`responses.rs`/`effects.rs`/`core.rs`/`rt_parity_native.rs`) + runtime-leader's
  blob-verified analysis, NOT a fresh native run (box memory pressure).
  Belt-and-suspenders check for the ring: un-ignoring `fs_read_at_malformed_offset`
  on the base should still trap on the malformed `ExitCode::Failure` before the
  closeout lands.

## Increment-1 hard-stop inventory (§1b-i durable holder)

Maintained by the Steward for the runtime b2 D3-RECUT build (increment 1, the
funded READ half). Live thread `thr_13yeftxjnxz2z`; clean checkpoint base
`c3bb29c81` (all green). The Architect carries the working copy in-thread +
`ARCHITECT-STATE.md`; THIS is the durable copy that survives compaction (§1b-i).
Coordinates decay — re-measure.

- **HS1 (capsule reach)** — `evt_1jxh8epayhnqe`. The read `Ret{Match}` closeout's
  `Complete(RecursiveBackedge)` short-circuits BEFORE the downstream governed
  arrival that recorded certificate E's capsule reach (`reached_count`,
  `source.rs:4265`). Ruled Case-B: record E's reach at the closeout consumer,
  `px8-ds-test-support`-gated, zero production change / zero TCB.
- **HS2 (narrowing placement)** — `evt_h0vgd11g5xfb`. Shape-narrowing at
  FORMATION dropped the effect Tail from `formed` while `planned` kept it
  (`planned == formed` invariant, role-witness 3560). Ruled (A): move the shape
  gate FORMATION to CONSUMPTION — unnarrow formation so `planned == formed ==
  base`; dispatch at both consumption seams (`Ret{Match}` to the closeout, effect
  Tail to the base `call_tail -> Continue`). Net inc1 production delta = exactly
  the funded READ half.
- **HS3 (raw arrival)** — `evt_38f5r814tbh55`. Under (A), the read closeout's
  short-circuit ALSO prevents the flow that at base recorded E's generated-entry
  raw arrival (`raw_arrival_count`, `rt_parity_native.rs:2470`): E installed,
  `raw_arrival_count == 0`. This is inc1's 3rd structural wall.

**Shared predicate (HS1 + HS3), named at the 3rd entry (§1b).** The read
closeout's `Complete(RecursiveBackedge)` short-circuits the source machine, and
base's observation model recorded downstream observations by CONTINUING PAST
EVERY TAIL. HS1 was the capsule-reach counter; HS3 is the raw-arrival counter —
ONE defect hitting TWO counters. Patching counter-by-counter is the unbounded
chain §1b warns about (the next displaced counter is the 4th entry). The fix must
be a STRUCTURAL CLOSURE — the short-circuiting forward edge must assume the
COMPLETE downstream-observation obligation of the continuation it replaces — not
another single-counter patch.

**§1a research check (inc1 HS3) — DISCHARGED; HS3 RESOLVED (§1b structural
closure).** Fired by the Architect on his own committed criterion
(`evt_19x6xf1n2a539`); research advisory in (`evt_7rn86avam5y57`); ruling
`evt_18x2n8yta31xz`. The problem, named: tail position IS an empty continuation;
the read `Ret{Match}` closeout's `Complete(RecursiveBackedge)` turns a NON-EMPTY,
observation-bearing continuation into a tail, and base recorded observations as a
side effect of the continuation structure. RESOLUTION — a SEALED forward-edge
observation obligation discharged on the `call_tail`-coherent path, completeness
enforced STRUCTURALLY:

1. REIFY + SEAL the obligation as a CLOSED enum/type the forward edge carries
   (the complete set of observations base's elided continuation recorded) — the
   DUAL of the RT-COMPOSED-RETURN SSA-Deferred "classify once / first-class
   Deferred" closure; reuse that discipline, do not invent a parallel one.
2. COMPLETENESS IS STRUCTURAL: the closeout discharge is a TOTAL match over the
   sealed set with NO catch-all. A newly-displaced observation (HS4) becomes a
   COMPILE error (non-exhaustive), not a 4th hard-stop. A wildcard arm re-admits
   the §1b chain and is itself the defect.
3. PER-MEMBER discharge, decided by measurement: (b) reconstruct if the
   observation is a pure function of E's carried state; (c) relocate onto a
   convergence point ON THE `call_tail` path; else (a) hoist at the bypass keyed
   on E's governed coordinate. `reached_count` (HS1) is already (a)-hoisted —
   fold it in as one sealed member. `raw_arrival_count` (HS3): the offered
   eager-at-seam vs downstream measurement is the (b)/(c)-vs-(a) discriminator.
4. NEVER buy the observation from a FOREIGN producer — do NOT route the formed
   effect Tail through `checked_ih_captured_environment` (the (ii)
   remedy-in-wrong-frame: a distinct eager event standing in for the missing
   one). The effect Tail STAYS `call_tail`, the coherent production path.
5. BOUNDARY: name the cfg-profile of the `raw_arrival_count` recording site —
   test-support-gated ⇒ the whole closure is test-support-only (zero prod/TCB,
   inc1 delta stays exactly the funded READ half); production ⇒ Architect
   re-scrutinizes under means-(a).

Re-review scrutiny (a)-(e) per the ruling: sealed type + total match with a
demonstrated compile-error property (a dummy member reds the build); both
counters discharged on the `call_tail` path (never (ii)-bought); all ~13
generated_entry tests + forward_ret 13/13 + 3 read narrows green with
`planned == formed`; boundary/cfg stated; each observation recorded exactly once
per discharge.

**Count of record: §1a DISCHARGED at inc1 HS3; next mandatory §1a re-trigger =
HS6 — but the sealed-set closure is engineered to terminate the chain (a 4th
displaced counter is a compile error, not a hard-stop).** Implementer builds the
sealed-obligation discharge → fresh SHA → Architect design+soundness +
runtime-qa → fresh Decision (runtime-leader) → Steward M1-M4. `dec_22xpy0mnz221a`
is void; `711724f32` never merges. inc2 (write-half migration) resumes after inc1
lands — now ALSO the unified edge-control-migration home; see the Increment-2 frame
seed below.

## Increment-1 control-side census disposition + Increment-2 frame seed

**Scope call — Steward AUTHORIZES option (b)** (2026-09-03), on the Architect's
design ruling `evt_5kpshvbx32gnr` and control-side census (implementer
`evt_7j9pvzgfgsy6m`, Architect z2145 requirement discharged). The read-half fix
derives the read answer from the edge CARRIER, not the capsule, so ~14 base
controls that probed the read via the CAPSULE projection have NO surviving control
home. Retarget-to-write is empirically REFUTED — the write program lacks the read's
specialized computational-recursor capsule structure; capsule wrong-slot on the
write did not redden (`rt_parity_native.rs:3065`). This is a coverage-scoping call
(a WP-cut, no TCB delta, inc1 stays its funded READ half), the Steward's to make;
the design half (soundness + (a)-vs-(b)) is the Architect's and is settled.

Per-dimension disposition (Architect three groups, `evt_5kpshvbx32gnr`):

1. **Direct-control capsule** (wrong-destination-owner / body / binding /
   locator-*): KEEP — already on the write, still redden. No change.
2. **Arrival seam-counters:** SUBSUMED per-variant — for EACH variant, either
   (i) its exact property is "a discharge member can be skipped / a seam arrival
   missed" ⇒ power UPGRADED to the HS3 sealed-closure compile-time totality + the
   green admission/confluence/reach/raw_arrival counters ⇒ RETIRE with citation
   (NOT `#[ignore]`); or (ii) a distinct fine-structure the sealed match does NOT
   cover ⇒ joins group 3. The per-variant line is the implementer's to produce —
   not "arrival = same shape as capsule."
3. **Non-direct capsule fine-structure** (wrong-slot / frame / invocation /
   non-carried-residual / provenance-index / outer-carried / specialized-sibling /
   static-worker + retained-access-*): DEFERRED — regression-catchers over the
   carrier's fine structure, NOT soundness gates; re-key to a new read-edge-carrier
   mutation family (a control-design build) in inc2.

**Soundness of landing inc1 core under the deferral: AFFIRMED (Architect).** The
read EDGE stays controlled at four independent levels — carrier WORD
(`forward_ret_edge_substituted_word_reds` / `SubstituteForwardEdgeWord`, green→trap
flip), governed coordinate (role-witness paired family, `planned == formed`
3549/3560/3561), observation discharge (HS3 sealed closure compile-totality +
admission/confluence/reach/raw_arrival counters), and end-to-end answer (3 read
narrows, independent of the capsule slot). The deferred item is fine-grained
per-slot mutation power over the carrier's internal structure — a coverage
refinement, not a correctness gate; deferring it cannot admit a wrong read answer.
(b) is chosen over (a) because inc2 migrates the WRITE half onto the SAME edge
mechanism and re-keys both halves' controls once; building read-edge-only controls
in inc1 and re-merging them in inc2 is duplicative, and box contention makes a fresh
control build's per-dimension validation especially costly now.

### Increment-2 frame seed: unified read+write edge-control migration

inc2 migrates the WRITE half onto the same forward-edge mechanism and is the single
natural home for re-keying BOTH halves' controls. It carries this named, tracked
obligation — the Architect's z2145 "power transfers, never vanishes," made
enforceable:

- **AC-EDGE-CONTROL-REKEY.** Re-key the read-edge carrier-mutation controls deferred
  from inc1: for each deferred dimension build the read-edge-carrier analog mutation
  (corrupt the closeout's carrier-slot / frame projection / sealed discharge),
  un-ignore its base test, and green it — a genuine green→trap flip, never a vacuous
  pass. Every inc1 `#[ignore]` cites THIS AC by id
  (`RT-COMPOSED-RETURN-FORWARD-RET-EDGE / AC-EDGE-CONTROL-REKEY`), never a prose
  "follow-up." The census in this section is the completeness ledger: no dimension
  silently loses coverage, the Architect re-checks it against the deferred set at
  inc1 landing, and inc2 discharges it.

  Deferred set — FINAL (implementer per-variant disposition `evt_55my3tmvbmkf0` on
  candidate `07c31b0c5`; Architect-confirmed 3-retire/3-keep arrival split
  `evt_3zba50hydkpdb`). EXACTLY these 8 non-direct capsule fine-structure controls
  are `#[ignore]`'d citing this AC — nothing else:
  - outer-carried, specialized-sibling, static-worker, wrong-frame, wrong-slot,
    wrong-invocation, non-carried-residual, provenance-index

  NOT deferred — recorded here so the census is complete (no dimension silently
  loses coverage; this is the ledger the Architect re-checks):
  - RETIRED (subsumed, deleted WITH citation + a bidirectional tie at
    `admission_population_is_total`'s governed branch so a future weakening trips a
    reviewer against the deletion): arrival skip-validation + duplicate-validation
    (⇒ `raw_arrival == governed_validation`, line 2513, + sealed exactly-once),
    governed-through-non-governed (⇒ governed ⇒ `ordinary_continuation == 0` lines
    2517-2518 + `raw_arrival > 0` line 2511 + governed gate `aggregates.rs:7565-7577`).
  - KEPT (still live, verified passing — inject a detectable fault for the
    populations that do NOT transit the edge): arrival skip-lookup / duplicate-lookup
    / non-governed-through-governed; direct-control capsule (write); retained-access
    / forward-ret-access (Tail layer). **`retained-access-*` is KEPT, not deferred**
    (corrects the provisional seed).

**Count of record (Architect `evt_5kpshvbx32gnr`): NOT a new hard-stop** — a
control-side consequence of the already-ruled read-half fix (z2145 continued), a
coverage-scoping question. inc1 stays at 3 hard-stops; §1a HS3 check discharged,
next mandatory §1a re-trigger = HS6; §1b same predicate already named (read closeout
derives from the carrier, not the capsule) — no new inventory entry, no new recut.

## The single relaxed constraint, and what stays closed

The ONE relaxed constraint: a governed Tail result BYPASSES the source-machine
answer collapse, the constructor transfer, the active carried backedge, and the
checked fallback — it takes one certified forward SSA edge from the existing
producer to the existing shared `Ret` block. This is the "lossy middle" the
constraint-differential report identified as incidental (comparators keep the
operation/continuation association live and move the response FORWARD; Ken alone
collapses the answer and reconstructs a seed word backward).

Everything else stays closed. **Produced-transfer, D3, Direct-only salvage,
recovery, storage, runtime tags, general carriers, active-header ABI widening, a
second `Ret` lowering, and HS15 remain closed.** No spec, kernel, trust, or wire
change. The forward-edge block capability is COMPILER STATE, not a runtime lane —
it never enters a Ken value, carrier, frame, ABI, or memory.

## Exact base and coordinates (Architect `evt_70n2y6s9wanf9`)

Base: `origin/main` `7d807a78e014f768311b53380c77fa1daf4c6e06`, tree
`37da2a97565b0e968ed1927ea44906be3c886597` (the Architect's cited tree; a
disjoint Steward doc publish moved the commit label off `8455964281`, the tree is
identical). Coordinates decay — re-measure before acting.

- `crates/ken-runtime/src/cranelift_backend/lowering/source.rs` blob
  `88fcc401b0e078f78298a0998d09364b22e64a27`: governed validation `3976-4117`;
  transport selection `4309`; call/result `4369-4371`; current collapse entry at
  `4373`. The lossy middle is `1260-1264`, `1647`, and `1691-1710`.
- `crates/ken-runtime/src/cranelift_backend/lowering/core.rs` blob
  `79ec94b749836a6e1747d6b6da0b572f919105cd`: active loop/header `12225-12282`;
  shared `Ret` block creation `12364-12368`; ordinary `Ret` jump `12433-12445`;
  stale checked-fallback jump `12634-12743`.
- `crates/ken-runtime/src/cranelift_backend/planning/static_transition/aggregates.rs`
  blob `9eb2c118e227c3a7db2849e03046db02d93a48eb`: fresh route `5461-5582`; Tail
  validation `5751-5855`; member/projection row `5902-5977`; confluence
  `6123-6257`; access publication `6323-6543`; validation `6624-6684`.

## Required component shape (Architect `evt_70n2y6s9wanf9`)

1. **Replace the planner's false Tail emission description.** `TailResumedRetInput`
   currently claims `ActiveSelfResumption` plus `CheckedAnswerFallbackDirect` — the
   path that loses the result. REPLACE it, do not add a sibling, with a Tail
   producer-to-Ret plan naming the governed source/binding, the selected case, the
   active frame, the exact `Ret` body, the exact
   `ConstructorChild { frame_origin, field_position: 0 }`, forward direction, and
   producer-result-direct delivery. `DirectInvocationReturn` stays.
2. **Extend the existing compiler-only `active_carried_computational_eliminations`
   stack entry.** When `core.rs:12364` creates the shared `Ret` block, install
   `{active_frame_origin, ret_case_body_origin, binder field 0, return_body block}`
   in that exact entry. Missing, duplicate, or wrong-frame/body/binder lookup
   refuses. This block handle never enters a Ken value, carrier, frame, ABI, or
   memory.
3. **Reuse the retained confluence authority.** Governed validation retains its
   projection. After selecting the transport and BEFORE emitting at `:4369`, an
   exact lookup requires the access coordinate, projection equality, and
   membership of that transport's own `source_call_identity`; then the Tail plan
   must match the unique emission sink. This yields ONE move-only compiler proof,
   not a copied member map or positional choice.
4. **Under that proof, emit the declared call once.** Direct preserves the old
   return. Tail requires the returned carried word and immediately emits
   `jump(return_body, [returned.word])`, switches to an unreachable builder block,
   and returns a sealed `RecursiveBackedge` disposition so no remaining source
   continuations emit on that predecessor. No second `Ret` body.
5. **The later fallback may serve other honest populations, but a governed Tail
   producer-to-Ret plan neither claims nor reaches it.**

## Deliverables — one-hour turns (Architect `evt_70n2y6s9wanf9`)

- **D1 — sink seam, byte-inert.** Extend the active-stack record, install the
  unique strict `Ret` sink, add population/uniqueness controls. No consumer or
  product claim.
- **D2 — authority/plan, byte-inert.** Exact post-selection confluence/member
  join plus the replacement Tail plan, and wrong-member/projection/source/sink
  controls. No live-edge claim.
- **D3 — atomic activation.** Consume the proof at `source.rs:4369-4373`, emit the
  returned-word jump, retire the old Tail header/fallback observer claims, add the
  two durable products, and run affected closure. D3 code, causal controls, and
  products land together. D1/D2 may land alone ONLY with byte-inert evidence and
  honest prose.

## Acceptance criteria (Architect `evt_70n2y6s9wanf9`)

- **AC-PLAN-EXACT** — one governed source and one `Ret` sink. Removal,
  duplication, reversal, cross-variant, and wrong-body/binder/frame mutations RED
  before emission. The old Tail header/fallback fields are ABSENT (replaced, not
  siblinged).
- **AC-ONE-AUTHORITY** — the edge proof forms ONLY after exact transport selection
  and confluence-member validation. Wrong member/transport, projection
  disagreement, missing class, or a proof bypass RED. No second catalog.
- **AC-FORWARD-SSA** — the exact call result is the SOLE governed predecessor
  argument to the shared `Ret` block. Ordinary `Ret` is unchanged. A governed Tail
  does not enter `RoutedAnswer::checked`, `ConstructArgument`, active
  self-resumption, or the checked fallback.
- **AC-CAUSAL-PAIR** — with call emission retained, route its result through the
  old collapse: both products revert to base `ResourceBodyResult`
  `PatternMatchFailure`. Keep the edge but substitute an independent non-result
  word: both likewise fail. Suppress or duplicate the call/edge and the exact
  emission/effect census REDs. This separates producer existence, result identity,
  and binding — a control that cannot distinguish the three is manufactured and is
  a HARD STOP.
- **AC-PRODUCTS-EXACT** — pin the merge-base negatives. The candidate
  `SourceFormat::Ken` witnesses `fs-read-at-offset-single` / `rt_read_offset_stage`
  and `fs-write-at-offset-single` / `rt_write_writable_stage` agree with the
  interpreter on exact `InvalidOffset`; each target effect and tail resumption
  occur exactly once. If write exposes another blocker, HARD-STOP rather than
  weaken or substitute. Both rows become durable, non-ignored tests.
- **AC-NEGATIVE-SCOPE** — Direct remains 3/3 through `DirectInvocationReturn`;
  ordinary `Ret`, non-governed calls, malformed topology, and non-checked routes
  preserve behavior. No spec/kernel/trust/wire change.
- **AC-AFFECTED-CLOSURE** — local commands use `scripts/ken-cargo` ONLY: targeted
  `-p ken-runtime`, exact `-p ken-cli --test rt_parity_native` product/mutation
  rows, and the existing `ken run` end-to-end targets. CI supplies all `ken-cli`
  integration targets plus the workspace gate. Record base and candidate; closure
  is NOT a hand-picked source grep.
- **AC-PROHIBITIONS** — zero new runtime words, active-header params, frame slots,
  captures, stores, tags, recovery, general carriers, a second `Ret` body, or
  revival of Produced-transfer / D3 / Direct-only / HS15.

## Reviewers

Architect — the implemented mechanism matches this ruling: the plan is replaced
(not siblinged) and names the exact governed source/case/frame/Ret-body/child; the
edge proof forms only after transport selection and confluence-member validation
and is one move-only compiler proof; the governed Tail emits a single
`jump(return_body, [returned.word])` and never touches the collapse/fallback; the
block handle is compiler state only; prohibition-clean. Runtime QA — the controls
red/green as specified, `AC-CAUSAL-PAIR` genuinely separates producer existence
from result identity from binding (a control that stays green under any of the
three substitutions is manufactured and is a HARD STOP), and the two witnesses
produce exact `InvalidOffset` as durable non-ignored tests. A design fork
HARD-STOPS to the Architect.

## Capability tier

T1 — a soundness-bearing native-lowering construction reviewed on the
provenance/forward-edge argument and its causal controls, not a differential diff.
Size L.

## Sequencing

Lane 1 (runtime), the funded shape-(a) objective (operator 2026-08-30). No
`depends_on` — the base is current main and the coordinates are measurable now.
`blocks: [PX8]` — this closes the Tail composed-return route the
`RT-NATIVE-CARRIED-VALUE` ignored full-program rows exercise. The refuted
authority-only build `RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD` is `closed`
(superseded, built nothing). All other closed axes stay closed unless a candidate
names and relaxes a further constraint — which is a fresh Architect decision, not
this frame.
