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

### Increment-2 hard-stop chain + §1b inventory

- **inc2-HS1 (write-half run-at-collapse mechanism)** — implementer measured
  (`evt_gky6xevjq935`), Architect RULED (`evt_36eczhwqsdk64`). NOT a §1a research
  trigger (next §1a in the inc2 chain = inc2-HS3). The write's governed Tail
  continuation body is an effect-performing checked-IH computational match belonging
  to a DIFFERENT predeclared function than the collapse's function; a naive inline
  re-run (generalize the read closeout to run the whole body) fails object emission
  (`ContinuationSpecialization: the claimed continuation target was not declared into
  this function`, fn-8's nested continuation absent from fn-9's `continuation_calls`).
  Fork (a) declare the other function's continuations into this one — REJECTED
  doubly: dissolves the per-function continuation-scope invariant, and re-running the
  body RE-performs the `writeAt` (double-effect soundness bug). Fork (b) RULED: the
  effect is performed upstream, the InlineNoCall carrier holds the post-`writeAt`
  result, and the collapse ROUTES that carried result to `return_body` and runs ONLY
  the post-effect k-narrowing (the `Match` on `Var(0)`) — mirrors the read (route +
  narrow), one extra narrowing match, never re-running the producing function's body,
  never importing its continuations. Conditional on b-confirm: the carrier holds the
  post-effect result (structurally confirmed — `Match{scrutinee: Var(0)}` narrows the
  write `Result`, so the effect is already performed; functional confirm = (b) narrows
  to InvalidOffset with native==interpreter parity).
- **§1b inc2 inventory entry 1:** predicate = "the write effect continuation is a
  nested cross-function checked-IH continuation (`Match`-on-result), unlike the read's
  pure `Ret` — route the carried post-effect result, do not re-run the producing
  function's body." Acceptance for (b): (1) exactly-once effect (`writeAt` once
  upstream + narrowed once, native==interpreter parity on the write route); (2) a
  non-degenerate pair (valid-write route narrows correctly AND a wrong post-effect
  narrowing — mis-routed `Var(0)` / wrong `Result` arm — is caught).
- **inc2-HS1 b-confirm = NO (`evt_7dys597356kts`) → (b) refuted.** The FULL write-body
  shape shows the route is a READ-THEN-WRITE: the outer `Match{scrutinee: Var(0)}`
  narrows the *readAt* result (`Err → Ret`; `Ok →` frames 5/6 that PERFORM the
  `writeAt`), so the InlineNoCall carrier holds the *read* result and the `writeAt` is
  performed WITHIN the body's `Ok` arm, not upstream. (b)'s premise (carrier holds the
  post-`writeAt` result) is false; there is no post-effect result to route. (The first
  400-char shape was truncated and misled the (b) premise; the gate caught it.)
- **inc2-HS2 (write route is READ-THEN-WRITE; PARTIAL collapse)** — implementer
  measured/flagged (`evt_7dys597356kts`), Architect RE-RULED **(c)** (`evt_xfscq4shy8rj`).
  NOT a §1a trigger (next §1a = inc2-HS3). (a) stays rejected (importing fn-8's
  continuations into fn-9 breaches the per-function continuation-scope invariant).
  **(c) principle: the forward-edge collapse is PARTIAL** — collapse only the in-scope
  outer read-narrowing (route the carried read result + run `Match{Err → Ret; Ok →
  writeAt}` on it, as the read collapse does for its narrowing); the `writeAt` effect
  subtree (frames 5/6) belongs to fn-8 and MUST run in fn-8's scope via base call/edge
  machinery, NEVER inlined into fn-9. The forward edge applies only to in-scope `Ret`s
  (the `Err` arm's `Ret`, and the final `Ret` once the `writeAt` completes in fn-8),
  never to the cross-function effect subtree. **c1 vs c2 measurement-gated:** does the
  `writeAt` subtree already have its own Tail/Direct route in fn-8? YES → c1 (Ok arm
  invokes fn-8's existing writeAt route; forward edge on the outer/final `Ret` only);
  NO → c2 (base `call_tail` machinery for the effect subtree — base already runs frames
  5/6 correctly, just un-narrowed — forward edge on the final `Ret` only); NEITHER →
  STOP and flag (c3).
- **§1b inc2 inventory entry 2:** predicate = "the write route is a READ-THEN-WRITE
  whose effect subtree is a cross-function checked-IH continuation, so the forward-edge
  collapse must be PARTIAL (collapse the in-scope read-narrowing; run the cross-function
  effect in its declaring function via base machinery) — NOT a whole-body inline and
  NOT a single-carrier route+narrow." Refines the inc1 model: read = pure route+narrow;
  write = read-narrow + a cross-function effect subtree that stays in its function.
  Acceptance for (c): (1) exactly-once per effect (readAt once AND writeAt once,
  native==interpreter parity — not zero, not double); (2) barrier respected (the
  `ContinuationSpecialization` "declared into this function" check passes because the
  `writeAt` subtree runs in fn-8 — no fn-8 continuation imported into fn-9); (3) a
  non-degenerate pair (valid read-then-write with both effects once AND a wrong
  narrowing / mis-scoped or duplicated effect is caught).
- **inc2-HS3 (c1 does not realize → COLLAPSIBILITY GUARD)** — implementer
  built the (c1) partial collapse and hit a wall (`evt_1bz8gywkmz2fc`): the
  write's outer Tail is a RECURSIVE EFFECTFUL continuation that resumes OUTWARD
  via the source-machine join (`SourceJoinTarget`/`JoinPlanToken`), so its
  answer cannot be captured and jumped to the composed-return sink the way the
  read's bare value is; and the base is InlineNoCall (no fn-8 return value to
  forward-edge). This is the **§1a research check for the inc2 chain —
  DISCHARGED** by the research advisory (`evt_3cys0pcpfzj7y`): return-collapse is
  a VALUE-RETURNING-TAIL optimization; an outward-resuming effect tail is not in
  tail position and is not collapsible. Architect RULED outcome **(iii) the
  COLLAPSIBILITY CRITERION** (`evt_375m4aqpag6ck`; discriminator grounded
  `evt_1c1j3vjeeaqrr`; build spec `evt_3cjmm6j2gbtxb`): NOT a new write
  mechanism — a SCOPING guard. Collapse iff the producer result reaches the
  strict-Ret sink through pure value-narrowing; reject (→ base) on any
  intervening pending control (effect / recursor / outward join). The
  implementer's own facet measurement showed every LOCAL route/tail signal is
  identical read vs write, so the discriminator is a NON-LOCAL producer→sink
  PATH property (build (A)): the cross-unit follow
  `checked_ih_invocation_recursive_unit_body(producer_step)` — `Ok(Some(pure
  body))` ⇒ collapsible; `Err`("units disagree on declared recursive body") /
  `None` / impure ⇒ not — positive-determination + fail-safe-to-base.
- **§1b inc2 inventory entry 3:** predicate = "a recursive-effectful
  outward-resuming continuation is NOT a value-returning tail and is NOT
  forward-edge-collapsible; the RT forward edge scopes to value-returning tails,
  and collapsibility is a PATH property of the producer→strict-Ret-sink walk
  (pure narrowing vs an intervening effect / recursor / join), not a local
  tail-body field." Closes the inc2 write-half design question.
- **inc2-HS4 (acceptance contradiction → PREVENTIVE-ONLY; §7a corrected)** —
  building (A), the implementer ran the decisive forced-all-base experiment
  (`evt_1qgbd8evdw3xv`) and REFUTED the acceptance premise. Measured: the
  non-collapsible outer route (af=483) ALREADY takes base
  (`tail_worker_body_is_ret_kmatch = false`); the only collapsing route
  (af=696) is genuinely collapsible; and forcing ALL Tail routes off the
  collapse leaves the write trapping IDENTICALLY
  (`PatternMatchFailure`/`ResourceBodyResult`, no readAt/writeAt) while the
  READ regresses (also traps). So **base was NEVER parity-correct for these
  composed-return programs; the forward edge is the only thing that narrows them
  on native.** The collapse is NOT the write-trap cause — base is — so build
  (A), however built, cannot un-ignore the write narrowing. Architect RULED
  **(1) preventive-only** (`evt_1z9x00y6ydjtf`) and corrected the record: it
  accepts the retraction of the earlier §7a "the collapse fires and traps" (a
  misread). A fresh hard-stop distinct from HS3's (c1)-does-not-realize; neither
  still-HS3 nor HS4 is a multiple of 3, so **no §1a trigger; next mandatory §1a
  research = HS6.**

### (A) is PREVENTIVE; the write narrowing is inc3 (do not conflate them)

**(A) is the correct collapsibility SCOPING and it lands as inc2** — it keeps
the forward-Ret edge OFF effect tails (the write's outer effect tail af=483
already takes base and provably stays there), future-proofing the edge against
any tail whose worker body is `Ret{Match}` but whose producer→sink path carries
intervening control. **(A) does NOT and cannot narrow the write.** The write
narrowing requires the EFFECT-PERFORMING CONTINUATION MECHANISM — native
execution of the read-then-write effect recursor + narrowing — a genuinely
different and larger design (an effect-execution capability, not a
value-return optimization). That is **inc3** (Architect `evt_1z9x00y6ydjtf`;
Steward cuts the WP). The two write tests
(`fs_write_at_malformed_offset_narrows_to_invalid_offset` and
`..._without_write_right_...`) STAY `#[ignore]` through inc2 and un-ignore in
inc3; the 8 AC-EDGE-CONTROL-REKEY capsule controls + the coupled
census-exemption removal also ride inc3 (re-keying a still-`#[ignore]` test to a
guard that does not un-ignore it is the deferral-marker-inert trap). A future
reader must not mistake the (A) guard for the write fix.

**Landed (A) mechanism (inc2):** `checked_ih_forward_edge_route_collapsible`
(aggregates.rs, plan predicate) + thin
`tail_route_is_forward_edge_collapsible` wrapper (core.rs), ANDed with
`tail_worker_body_is_ret_kmatch` at both consumption seams (source.rs);
formation untouched (`planned == formed == base`, evt_h0vgd11g5xfb). Acceptance
(preventive, corrected — NOT "write narrows"): (a) read collapses
(`forward_ret_edge_substituted_word_reds` GREEN); (b) the discriminator's
determination is PINNED directly
(`forward_edge_collapsibility_discriminates_value_and_effect_tails`, a
px8-ds-test-support observation recorder, asserts the non-degenerate pair — the
write program has exactly one non-collapsible effect tail AND at least one
collapsible tail, the read program none non-collapsible — recorded independent
of the `is_ret_kmatch` short-circuit so the guard is pinned even where it flips
no arm); (c) no regression (forward_ret 13/0, composed_return 7/0,
generated_entry 66/0, the 8 capsule controls stay `#[ignore]`); (d) the two
write tests untouched/`#[ignore]`. gate=none, backend/control-flow only, zero
TCB delta.

## Increment-1 M5 CI red on 07c31b0c5 — symptom dispositions (Steward)

The re-spin candidate `07c31b0c5` (Decision `dec_22r1rbn9qnn81`) passed M1-M4 and was
ROUTED, then hit **M5 CI red** on the first full native suite (run 33813131586;
lieutenant `evt_5ty9e9q8xrtej`). NOTHING merged (PR #3288 open); `dec_22r1rbn9qnn81`
is VOID/SPENT — a re-spin is a FRESH SHA + FRESH Decision (leader `evt_dg3t5hpavgby`,
Steward `evt_5k35tfcqhdh35`). The CI contingency fired as designed: reviewers approved
on box-OOM-forced ISOLATED runs, and the first full native run exposed it — the
Architect's explicit greenness caveat named exactly this. Architect re-review
`evt_1hqe1kst2ygak`; the design+soundness findings STAND (the red is runtime/semantic,
not a structural/soundness defect in the sealed closure).

**Symptom 1 (stack overflow) — BOUNDED-depth TEST-HARNESS GAP, NOT a mechanism defect,
NOT a hard-stop.** The Architect's initial NON-TERMINATION hypothesis was REFUTED by
measurement (both reviewers, runtime-qa `evt_20aa62vg55tmv` reproduced directly,
runtime-leader relay `evt_4bak5ksy1s5re`): 4 shards SIGABRT deterministically on
`rt_cold_lowering_path_enumeration` (rt_write_writable/rt_read_norights/rt_read_offset/
rt_write_readonly), but `RUST_MIN_STACK=268435456` (256 MiB) PASSES cleanly — the
recursion is BOUNDED, not runaway. Exact cause: that fixture's `entry_outcome()` calls
`build_native_program` on the DEFAULT thread with NO `in_large_stack_thread` wrapper,
while every other call site in the WP's own `rt_parity_native.rs` already wraps in the
256 MiB thread. The candidate's closeout (`emit_composed_return_ret_kmatch_closeout ->
lower_expr` on the k-Match payload) genuinely adds BOUNDED codegen depth on exactly the
4 crashing routes — which is why base was green and this candidate reds — and the same
programs already compile inside the wrapped threads. FIX: wrap
`rt_cold_lowering_path_enumeration.rs`'s `entry_outcome` in the same
`in_large_stack_thread`-shaped 256 MiB thread, matching the existing precedent — a
TEST-FILE change, NOT production. **COUNT (Steward, tracker-authoritative): NOT a
hard-stop — the mechanism is sound (bounded, correct); this is a test-harness
stack-provisioning gap. inc1 STAYS at 3 hard-stops; no §1a trigger.** Lesson (worth
keeping): a local "box OOM" that forced a CI-pending posture was the candidate's own
BOUNDED stack demand meeting an unwrapped fixture — measure before calling it either
contention OR non-termination. Fix this FIRST — it is the CI blocker.

**Symptom 2 (release-order) — RT-BRACKET class; disposition (Architect identity +
Steward sequencing).** The Architect made the identity call the lieutenant deferred
(`evt_1hqe1kst2ygak`): the divergence in `d1_route_control_direct_read`
(rt_parity_native.rs ~4210) is PURELY relative release order (identical 4 effect
events; only FsHandle/Buffer order flips) = the [[RT-BRACKET-RELEASE-ORDER-PARITY]]
class, NOT a new soundness hole. The candidate applied the exclusion at
`assert_narrowed_alike` (~557-608) but this test reaches the `_ =>` arm's raw
`assert_eq!(effect_trace)`, a full-ordered compare NOT under the exclusion.
runtime-qa traced the exact mechanism (`evt_20aa62vg55tmv`): of the 6
`d1_route_case!` variants (rt_parity_native.rs:4228-4233),
`d1_route_control_direct_read` is the ONLY one with `expected_family: None` — the
other 5 take the trap-provenance branch and never reach the strict full-order
comparison; the read-half fix makes direct-read return cleanly for the FIRST time, so
it newly falls through to `assert_d1_route_control_child`'s `_` arm (4210-4213). QA has
NOT confirmed the same pattern covers all 6 rt_parity_native shard failures — the
re-spin MUST check the other 5 before assuming full coverage.
**Sequencing (Steward): the fix stays IN the inc1 re-spin as a test-harness
exclusion-completion** — route every native/interpreter parity assertion on a
release-bearing route (the `_ =>` arm + its audited siblings) through the same
non-release split, consistent with the exclusion scope the Architect ruled.
[[RT-BRACKET-RELEASE-ORDER-PARITY]] remains the underlying-class tracker; it is NOT
reopened, and symptom 2 is NOT a new node. **REQUIRED CONFIRMATION before excluding
(Architect):** measure the divergence is purely relative release order — same release
SET (resources/requests/outcomes) AND non-release events agree in order. If the SET or
non-release order differs, it is NOT the tracked class but a real closeout regression
⇒ reopen, do NOT exclude.

**§1b CONDITIONAL (Steward, on symptom 2's measurement).** IF that measurement shows
the closeout actually REORDERED releases (not the RT-BRACKET class), THEN the HS3
sealed closure's datum-set was too NARROW — it sealed the observation counters
{reach, raw_arrival, admission_outcome, governed_validation} but the short-circuit
also displaces RESOURCE RELEASES, which the closure did not carry ⇒ a §1b signal to
WIDEN the sealed closure to release-order (structural), NOT a per-site patch. IF
symptom 2 IS the RT-BRACKET class (the likely case), NO closure widening is owed.

**Symptom 3 (ignored-row census 45 vs 37) — PRE-EXISTING DRIFT, not candidate-caused
(runtime-qa CONFIRMED, `evt_4bak5ksy1s5re`/`evt_20aa62vg55tmv`).** The 9 flagged files
are byte-identical base vs candidate, and the census mechanism itself correctly
handles the candidate's new `#[ignore]`s (45-37 = 8 = the added ignores is a red
herring — the discrepancy is the pre-existing ledger, not the candidate). FILE
SEPARATELY (census/ledger maintenance); it does NOT gate this re-spin's mechanism but
must be green for the eventual merge.

Re-spin path (both reviewers, `evt_4bak5ksy1s5re`): impl fixes the test-harness stack
wrapper FIRST (the CI blocker — wrap `entry_outcome` in the 256 MiB thread), then
completes the release-order exclusion at `assert_d1_route_control_child`'s `_` arm +
runs the required symptom-2 measurement + checks the other 5 shards, then re-verifies
symptom 3 is still clear; FRESH SHA -> both reviewers re-review -> fresh Decision
(runtime-leader) -> Steward M1-M4. `07c31b0c5` / `dec_22r1rbn9qnn81` never merge.

## Increment-1 re-spin measurements + B/C rulings — dispositions (Steward)

The re-spin measurement round (implementer `evt_4exdczywc7v7w`; Architect B/C
rulings `evt_63dg2292sqwgv`) RESOLVED symptoms 1-3 and surfaced two more items (B
recalibration, C layering) plus a WP-scope reconcile that is the Steward's. No
merge yet — the FRESH SHA + fresh Decision still follow.

**Symptoms 1-3 RESOLVED.** (1) Stack: PROPORTIONATE (all 4 routes pass at
`RT_COLD_STACK_MIB=3` and `=4`; candidate peak <3 MiB, a ~1 MiB constant bump off
the ~2 MiB base, not super-linear), SINGLE-LOWERING (code-confirmed one
`lower_expr` on the k-Match payload), PRODUCTION-SAFE (<3 MiB << 8 MiB main) —
Architect's three conditions met, wrapper is right. (2) Release-order: all 4
reaching variants (direct/unknown/ordinary/misroute-read) measured
`native_ops == interp_ops`, release SET equal, non-release events equal IN ORDER =
the RT-BRACKET class per variant; fix factored `non_release_events`/`release_set`
into shared helpers used by both `assert_narrowed_alike` and the `_` arm.
(3) Census: **§7a CORRECTION — CANDIDATE-caused, not pre-existing drift** (Architect
accepted the implementer finding, correcting BOTH reviewers' earlier "pre-existing"
read): the 8 AC-EDGE-CONTROL-REKEY `#[ignore]`s sat BEFORE the
`generated_entry_checked_case!` invocation where the macro's `$(#[$attr:meta])*`
matcher never sees them, so `generated_entry_capsule_outer_carried` RAN and failed
(45-37=8 = the non-applying ignores). Fixed by moving each `#[ignore]` inside the
macro call; all 8 verified firing via `--list --ignored`. So the earlier
"file separately, does not gate" disposition is SUPERSEDED — it IS candidate-caused
and part of the re-spin.

**§1b CONDITIONAL (symptom 2) — RESOLVED NEGATIVE.** The per-variant measurement
showed same release SET + non-release order agreeing = RT-BRACKET class, NOT a
release reorder. The HS3 sealed closure's observation-counter datum-set was NOT too
narrow; NO closure widening to release-order is owed.

**B (composed_return_ret_sink_population_is_unique, base test): recalibration, IN
SCOPE, mechanism not defect.** Read 35->17 AND write 26->17. Mechanism: the forward
SSA edge `Complete(RecursiveBackedge)` short-circuits the source machine, so
downstream strict-Ret seams the base continued past are no longer REACHED = fewer
sinks installed (production behavior; the sealed closure is unaffected). Architect
§7a self-correction of the z2300 "count must go UP" guard: this test counts REACHED
seams (`applications == observations.len()`), not FORMED authorities, so DOWN is
correct. Recalibration requirements: keep `applications == observations.len()` and
`observations.len() > 1` green; DERIVE (not paste) that the lost coords
(read (301,465),(511,676); write (525,691)) are exactly the backedge-subsumed
downstream seams.
- **WP-SCOPE RECONCILE (Steward's call, per Architect's corollary).** The closeout
  is SHAPE-gated (pure `Ret{Match}` composed-return at consumption, ruling A), NOT
  operation-kind-gated. A valid write-writable stage whose tail worker body is a
  pure `Ret{Match}` is the SAME shape as a read's, so it legitimately takes the edge
  (write 26->17) — this is subsume-don't-proliferate, NOT scope creep; an
  operation-kind special-case would be the defect. "read-half" names which half of
  the observation/edge-control migration inc1 lands (the AC-EDGE-CONTROL-REKEY
  controls), NOT which operations the shape-gated closeout optimizes;
  effect-performing continuations are inc2. The frame carries no "read-operations-only"
  wording to contradict, so this is a clarification: inc1's shape-gated scope
  INCLUDES valid pure-`Ret{Match}` writes, and both read and write recalibrate to 17.
  Malformed-write still trapping pre-composed-return (write trap tests green)
  confirms the gate fires only on genuine pure-`Ret{Match}`.

**C (checked_ih_inheritance_and_fresh_result_route_are_byte_inert): mechanism-
IMPLEMENTATION layering defect — distinct inventory entry, NOT a hard-stop.** Under
`SuppressForInertness` on the READ, only `executable_hash` + raw bytes differ;
`plan_transport_hash`, `core_semantic_hash`, `artifact_hash` ALL AGREE = the
spurious-codegen-dependency signature the Architect pre-registered as precondition
(i), which FAILS the retirement license. Diagnosis: `emit_composed_return_ret_kmatch_closeout`
reads a planner-only inheritance CERTIFICATE at codegen and lets it reach machine
bytes — the exact layering the byte-inertness invariant exists to enforce (inheritance
is a certificate, not a codegen input). Architect ruling = option (b): fix the closeout
to derive its emission from the inert continuation STRUCTURE
(`continuation_units`/`inputs`/`worker_body`, which live in the plan/artifact and are
inert), keep the invariant GREEN on read AND valid-write; do NOT retire. Required
confirming measurement: identify the exact datum the closeout reads that is affected
by `SuppressForInertness` AND absent from the three agreeing hashes — if it is the
inheritance well-formedness proof (the default, and what the signature says) it is the
layering fix; only if it is a genuine load-bearing EXECUTION input under-captured by
`core_semantic_hash` is it instead a hash-coverage defect (fix the hash, re-measure) —
either way a FIX, not a bare retire.
- **COUNT (Steward, tracker-authoritative): C is NOT a hard-stop.** It is a fixable
  mechanism-implementation layering bug with a KNOWN fix (option b), caught PRE-MERGE
  by the re-spin measurement — a gate-quality outcome, not a structural wall requiring
  a ruling/recut to pass. The DESIGN stands (z2250 sealed closure + shape-gated
  forward-Ret closeout unchanged); the implementation needs the layering fix before
  re-approval. **inc1 STAYS at 3 hard-stops.** C's predicate — codegen consuming a
  planner-only certificate — is DISTINCT from HS1-3's observation-displacement
  predicate, so §1b-independent: record C as its OWN inventory entry, not a 4th of the
  same. This CORRECTS the earlier "the 3 reds are all test issues" framing (C is a
  mechanism-implementation defect, not a test issue). Below HS6 regardless, so no §1a
  research trigger.

**Re-spin path (updated):** apply the C layering fix (read + write byte-inertness
green) + the B recalibration (derived to 17, invariants green) + the resolved
symptom-1/2/3 fixes -> strip temp instrumentation -> FRESH SHA + fresh Decision ->
both reviewers re-review (a)-(e) + C byte-inertness green on read and write + the B
derivation -> Steward M1-M4.

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
