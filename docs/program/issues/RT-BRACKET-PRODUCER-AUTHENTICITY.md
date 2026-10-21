---
id: RT-BRACKET-PRODUCER-AUTHENTICITY
title: "CHILD 1 of 3 of RT-BRACKET-CONTROL-REGION-IR, by route C (operator 2026-10-18). The checked bracket producer and its AUTHENTICITY boundary: the canonical producer releases through a confined finalizer constructor, PrivateBracketRelease, that source cannot name, and each bracket region is extracted from the normalized plane before erasure as that constructor plus the Ok binder of its own acquire. Two shapes are REJECTED ON MEASUREMENT and must not be retried: ordinary prelude proc declarations as markers (forgeable), and locating ports by scanning for an acquisition HostOpV1 plus every ResourceRelease (a lawful public early release makes the finalizer ambiguous). HostOpV1 may VALIDATE already-extracted ports; it may never DISCOVER them. THIS CHILD DOES NOT LAND ALONE: it is a held, reviewed input assembled by child 3."
status: active
owner: runtime
size: M
gate: none
tier: T1
depends_on: []
blocks: [RT-BRACKET-SETTLEMENT-PLANE]
github: null
origin: "Steward recut 2026-09-22 on the Architect's second WIP audit verdict evt_3rvns2yxm898r, which ruled RT-BRACKET-CONTROL-REGION-IR MIS-SIZED and directed a retained-work recut into three ordered children. The Steward owns the recut; the mis-sizing was the Steward's, not the ring's. The producer-authenticity boundary alone needs a finished design, controls and review before the planner or lowerer can safely depend on it. Retained work is preserved at evidence checkpoint 81f222b7f012829cd9f8d0f3dc684410a9b2b9ee on wp/RT-BRACKET-CONTROL-REGION-IR, parented on pre-registration commit c22f4861d which stays unchanged. That checkpoint is EVIDENCE AND REUSABLE WORK, NEVER A CANDIDATE."
---

> **STARTABLE: route C authorized** (operator 2026-10-18: "concur with
> rec on px8ta decision", on `evt_360c971y74gy6` and `evt_28p8pa97c35w4`:
> the umbrella with child 1 by route C; children 2 and 3 sized after
> child 1). Checkpoint `4b4c8565c` stays a held input: never moved,
> rebased or landed.

## 0. D0: settlement instance identity (scratch, lands nothing)

This D0 covers both px8ta blockers. They share one predicate: a bracket's
settlement runs on a shared, unowned Deferred route keyed by its template,
not by the instance's exact call/return context. Depth 2 shows it as
order, and depth 3 as a cross-specialization claim. The rows are px8ta
depths 2 and 3 and the `o1 c1 c1` lookalike.

**First D0, done (Architect `evt_yj783ca8pd2d`).** On `f555f4a37` each
settlement instance has its own Vis (depth 2: 515, 639; depth 3: 515, 737,
863), and each Deferred row keeps its own `vis_origin` and
`operation_root_origin`. The pairing is lost where native lowering keys on
the shared `effect_origin` 190. The erasure host-op leaf is generic by
design and is not the plane to test.

**D0-2, the authenticated tie.** For each checked producer occurrence (a
distinct `withResource` call of template `g587`/`bind g249`, with its
argument-5 settlement continuation), follow it to exactly one settlement
Vis through normalization and erasure, using only checked occurrence
provenance. Acquisition order, nesting depth, origin-number order,
`HostOpV1` shape and token `target` order are forbidden as the tie. Report
per row: producer occurrence, settlement Vis, `operation_root_origin`,
Phase-A token, the mechanism carrying each arrow, and the first arrow that
cannot be carried. The lookalike's tie yields one region (Vis 421) and
claims neither console Vis 441 nor 460.

**Outcomes, both first-class:**

- **(a′) Every arrow carries.** The Steward rewrites section 3's
  body-to-settlement edge on the settlement-Vis occurrence. Its region is
  minted from that Vis's Phase-A call/return context as an opaque
  `BracketSettlementId`; child 2 carries it into the Deferred row, and
  child 3 pairs lowering by region, not by `effect_origin`.
- **(b′) The producer-to-Vis arrow breaks at normalization.** That is a
  normalization restructure in kernel-normalize territory: **stop and
  return to the Steward** (an operator question).
  `wp/KERNEL-NORMALIZE-ORIGIN-TRACE` stays unmoved.

**D0-2 result: (b′)** (`evt_7m579fj57c8h6` on `f894d9c9f`; Architect
`evt_41yjkr1kp0egs`). `normalize` returns only a Term (conv.rs:495), so
no producer chain maps. This is the third stop on the source-occurrence
requirement, and that requirement is dropped for px8ta.

**D0-3, option D, withdrawn** (Architect `evt_7bcyrfavc68e5`).
`leaf_dispatch` has no Vis or root input, and carrying instance identity
there is the route parked at hard stop 7. Option K (a kernel origin
observer) is not taken.

**D0-4 result: option C's enabler holds** (Architect `evt_14r9629fb5mzp`,
scratch on `f894d9c9f`). A confined finalizer constructor gives one
private release per instance (2/3/1/1 in the four bodies), each tied to
its own acquire's Ok binder. A public early release stays separate,
source cannot name the constructor, and the regions extract before
erasure. Estimated at about 177 lines over 7 paths, with no kernel or
`trusted_base()` change. The operator authorized route C on
2026-10-18; §3, §6 and AC-5 below are rewritten for it, and AC-9 adds the
multi-leaf population D0-4 did not have.

**Prohibitions:** no lookup widening or added declaration lane for the
depth-3 claim; no order or shape guess and no `HostOpV1` port discovery;
no Mapping or operation carve-out. px8ta stays `#[ignore]`d. The `(H)`
probe caution in `RT-DEPTH3-CONTINUATION-CLAIM-UNDECLARED` stands. §1a for
the px8ta chain is 0.

## 0a. Landing order (Steward recut 2026-10-21, `evt_1gxk3pcr6rgw2`)

The chain lands bottom-up, one link at a time, each green on `main` by
itself:

1. This WP, as a fresh candidate on `main` with its own suites green,
   after the Architect's A1 measurement.
2. `RT-BRACKET-REGION-TRANSPORT` (A2 and A3).
3. `RT-BRACKET-SETTLEMENT-PLANE`.
4. `RT-BRACKET-LOWERING-AND-D0-REFUTER` (B and C).
5. `RT-BRACKET-PRODUCER-ROUTE-NESTED-BODY-ORDER`, paused and rebuilt
   on `main` after link 4.

The held refs are inputs only: never moved, rebased or landed. A link
that cannot land alone is held, and the Architect rules the landing
unit.

## 1. What is settled. Do not re-derive.

- The representation is not in question. The Architect's second audit says
  outright that these findings **do not invalidate explicit Control IR**. What
  failed was the CUT, and the recut is the Steward's.
- **Retained and NOT to be rebuilt** (measured at `81f222b7f012829cd9f8d0f3dc684410a9b2b9ee`):
  `bracket_templates_by_owner` and `collect_term_constant_ids` are GONE;
  `bracket_release_only_suffix` is GONE; the exact-call transformation idea
  stands. Carry those forward.
- The semantic unit is a **bracket control region**. One region owns one
  acquisition-success lifetime from delayed-body entry through
  settlement-result merge.

## 2. REJECTED ON MEASUREMENT. Do not retry either.

1. **Ordinary prelude `proc` declarations as the marker.** The three
   `private_checked_resource_bracket_*` markers were ordinary prelude procs. A
   Ken program can NAME them, so the claimed compiler-private authority is
   FORGEABLE and AC-4 is not met. A declaration whose spelling says "private"
   is not a privacy mechanism.
2. **Port discovery by uniqueness scan.** Locating ports by scanning the
   expanded Runtime expression for an acquisition `HostOpV1` and every
   `ResourceRelease` breaks on a LAWFUL PUBLIC EARLY RELEASE inside the bracket
   body: the extra release makes the finalizer port ambiguous.

## 3. Deliverable (route C, Architect `evt_14r9629fb5mzp`)

- **A confined finalizer constructor.** The canonical `withResource`
  producer's final settlement releases through `PrivateBracketRelease`, a
  constructor that source cannot name (D0-4: bare and qualified spellings
  refuse for Decl and File entry). A public early `release` stays the
  ordinary public constructor. Both checked IDs map to the existing
  `HostOpV1::ResourceRelease`.
- **Regions extracted on the normalized plane, before erasure.** A
  collector runs on the `ken_kernel::normalize` output. Each region is
  one `PrivateBracketRelease` occurrence group keyed by its operand, the
  `Resource` binder of its own `Ok` arm, whose scrutinee is its own
  acquire Vis's continuation binder. No source-occurrence map is needed,
  and none is built.
- **Group by `Ok` binder, never by count.** `bind` is linear in its
  continuation and the release is built at one site, so each `Ok` binder
  owns exactly one static finalizer serving every dynamic Ret leaf of the
  body (Architect `evt_31249h63pja2n`). That region is the outcome-merge
  role.
- **Reference evaluation unwraps `body` unchanged.**

## 4. Acceptance criteria

- **AC-1.** A Ken program that names or constructs the marker identity REFUSES.
  Direct forgery is the control, not an argument about visibility.
- **AC-2.** A bracket body containing a LAWFUL PUBLIC EARLY RELEASE still has an
  unambiguous finalizer port, and the early release stays ordinary body work.
- **AC-3.** Nested brackets each carry their own distinct role markers.
- **AC-4.** Bracket-SHAPED but unmarked code gets NO region. The control is ONE
  declaration chain carrying a canonical `withResource` bracket AND the proven
  lookalike declaration, asserting `regions.len() == 1`. RESEMBLANCE, not an
  unrelated acquire. An acquisition outside a canonical producer is
  UNCONSTRUCTIBLE FROM SOURCE -- prelude confinement removes a 41-name roster
  from `elab.globals`, `private_resource_acquire` among them -- so the earlier
  "unrelated acquire/release pair" control demanded a state nothing can reach.
  Do NOT hand-build the Runtime IR here: that bypasses erasure, and erasure's
  selection is the thing AC-4 measures.
  - **What ONE mutation establishes, written as that mutation and not as a
    class.** Widening the selector to key on application SHAPE rather than on
    the injected identity reds FAIL-CLOSED at marker normalization with
    `compiler bracket marker <id> has malformed application`. Measured at
    `daee72575`; located at `compiler_driver.rs:1275-1281`, inside
    `transform_bracket_producer_term` under `NormalizeMarkerPayloads`, firing
    only when `markers.ids().contains(&id)` holds for the application head and
    the spine is malformed. SUPERSEDES the count-of-2 discriminator in ruling
    `evt_7nrdfv7ekvcqt` point 5, which was a published PREDICTION.
  - **This does NOT generalize to over-wrapping.** The guard is keyed on MARKER
    IDENTITY, so a SOURCE LOOKALIKE -- different `GlobalId`, so
    `contains(&id)` is false -- passes normalization untouched and NOTHING
    refuses. `source_lookalike_identity_does_not_forge_bracket_metadata` shows
    exactly that: `regions.is_empty()`, no error. Do not restate refusal as a
    property of over-wrapping in general; that is the same move as the
    prediction it replaced.
  - **THE COUNT IS DISCHARGED ON HORN (b) at `0528c13e1`**, inside the accepted
    tip. Test file only, +35/-3, with `compiler_driver.rs` byte-identical and
    every mutation reverted -- so the guard was NOT relaxed to manufacture a
    reachable failure, which would have been the explicit non-discharge.
  - **The recorded reason is STRONGER than the one this frame predicted, and
    the prediction should not be read as established.** This frame offered
    "the guard is identity-keyed and normalization structurally precedes region
    construction" as the expected horn-(b) reason. Measured, three mutations
    are stopped by TWO barriers, not one. Widening to any 2-argument
    application, and the same restricted to well-typed applications, are
    refused at `NormalizeMarkerPayloads` on the arity guard. But marking a
    SATURATED 2-argument transparent application CLEARS normalization and then
    fails erasure lowering with `normalized HostIO body is neither
    identity-checked Ret nor Vis`. Normalization is therefore not the only
    thing in the way, and a second region still needs a second bracket-shaped
    producer occurrence that source cannot express.
  - **The count assertion is not inert, and that is measured rather than
    argued.** AC-8 runs the byte-identical sweep over a two-producer program
    and reads 2, so the instrument does distinguish 1 from 2. What is
    unconstructible is a FALSE 2 on the AC-4 fixture.
- **AC-5.** Regions come only from the confined constructor and its `Ok`
  binder on the normalized plane. No port is selected by `HostOpV1`, name,
  expression resemblance, or a `ResourceRelease` search. Validation by
  `HostOpV1` is permitted and must be shown to be validation, not
  selection: a mutation that maps `PrivateBracketRelease` to a different
  host op is refused at validation, and one that drops the constructor
  from extraction yields no region.
- **AC-6.** Reference evaluation unwraps `body` unchanged, shown by a control
  that would fail if it did not.
- **AC-7.** The marker is metadata: no runtime value, no public Ken syntax, no
  effect, no ABI field.
- **AC-8.** Owner disambiguation. Two distinct producers in one chain yield two
  distinct occurrences, each settling exactly once. This is what defeats the
  refuted section 2 selector. It is a SEPARATE claim from AC-4 and does not
  discharge it.
- **AC-9 (multi-leaf body; reframed, Architect `evt_31249h63pja2n`).**
  On `rt_bracket_region_multileaf.ken`, single and nested:
  - **AC-9a.** The single multi-leaf body gives exactly 1 region with
    exactly 1 static finalizer under its `Ok` binder, the continuation
    both `Ret BodyOk` and `Ret BodyErr` reach. The count is asserted as 1,
    not at least 1.
  - **AC-9b.** The nested body gives exactly 2 regions with 1 finalizer
    each, the inner one attributed to the `Ok` binder its `resource`
    argument resolves to.
  - **Paired controls on AC-9b**, each restored: (P1) keying regions by
    owner declaration or by the constructor symbol alone gives 1 region;
    (P2) keying a finalizer by its outermost enclosing `Ok` binder gives
    1 region holding 2 finalizers.

## 5. Stop condition

**STOP and the deliverable is the stop and its evidence if** the marker cannot
be made non-source-resolvable without adding a new public surface, or if exact
occurrence capture cannot survive normalization without a second authority.

Do not answer a stop with a census, a narrowed control set, a carve-out, or a
"private by convention" marker. On any stop branch it comes to the Steward and
then the operator.

## 6. Scope

The prelude producer and its confined constructor, the driver op map
(both IDs to the existing `ResourceRelease`), the normalized-plane
collector, the interpreter op ids, the reference evaluator unwrap, and the
test rosters. Measured enabler about 177 lines over 7 paths. **No new
host operation or ABI field, and no kernel, `trusted_base()`, carrier,
return protocol, KRET lane, dispatcher reorder, trace sort or fallback.**
px8ta stays `#[ignore]`d: its depth-2 order and depth-3 refusal belong to
children 2 and 3.

## 7. Symptom inventory

One entry per advancing hard stop on this WP. The Architect owns the count;
the Steward seeds and amends the frame. Counters are per WP and per design
question and do not pool: the umbrella and the parked predecessor keep their
own. Triggers fire at three.

**Entry 1 -- AC-4 demanded an unreachable state.** The prescribed control was a
declaration carrying a canonical bracket plus an UNRELATED acquire/release
pair. Measured: prelude confinement is `elab.globals.remove` over a 41-name
roster, `private_resource_acquire` included, so an acquisition outside a
canonical producer cannot be written from source at all. The two producers
also cannot be siblings -- unnested sibling brackets fail at erasure with
`checked_oriented_marker_location`, which is the pre-existing `RT-AGG-COMPOSE`
and NOT this WP's to repair. The claim AC-4 makes is sound; only its
prescribed control was unbuildable, and it is replaced above by resemblance.
The owner-disambiguation fixture built while discovering this is retained as
AC-8 rather than discarded, because it measures a real and different property.

The replacement's DISCRIMINATOR was then mis-stated in turn, TWICE. First the
count: an over-wrapping selector does not yield 2, because widening the
selector to key on application shape is refused at marker normalization before
any region is built. Then the correction itself OVER-GENERALIZED, replacing a
predicted class with an asserted one -- refusal holds for that named mutation
and NOT for a source lookalike, whose different `GlobalId` passes normalization
untouched with nothing refused. Both are corrected in AC-4 above, which now
names the mutation and its message rather than a property of over-wrapping.
NON-ADVANCING by the Architect's ruling `evt_7m05yqa48pxhk` point 4: a defect
in frame TEXT surfaced by measurement, no mechanism rework, no new structural
wall met. The count stays at 1. The consequence AC-4 carried is now DISCHARGED
on horn (b) at `0528c13e1`, and nothing on this WP is owed to child 3.

The frame was wrong a THIRD time in the same direction and the ring caught it
again: the horn-(b) reason offered above was a single barrier, and there are
two. Every one of these was a general claim stated at a scope that had not been
run. The standing remedy is in AC-4's wording, which now names the mutation and
its message instead of a property.
