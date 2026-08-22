---
id: RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL
title: "The carried-operand projection path for FsReadAt's Argument(2) buffer reply arm handles ONLY ONE observation kind (BytesPointerLength, observe_carried_bytes_span at effects.rs:1405) and re-refuses a carried ResourceScalar buffer; removing the dead specialized-only gate at 3267 (whose span_origin binding is genuinely unused) only RELOCATES that refusal one function deeper and greens nothing, so the fix is NEED-DIRECTED resource-awareness in site_operand_argument's carried branch (a seat whose declared need is ResourceScalar projects via lower_resource_token_seat, not the byte-span observer) WITH the dead gate removed as part of that one change -- the ResourceScalar-family reader RT-EXACTINT moved onto the cap41_* critical path (D0 re-scoped 2026-08-22 from removal-only: point (1) was FALSE, Architect ruling evt_7h23767bakhgm)"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-EXACTINT-CARRIED-OBSERVE, RT-DEAD-ARM-JOIN-DISPOSITION, RT-COLD-LOWERING-PATH-ENUMERATION]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: "Adversary M8 completeness flag on the landed [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] route (evt_5wx3bax63yak); Architect removal-not-reroute ruling (evt_2qdpkfvtqrxzy: 3226's destructured span_origin is unused, the span is projected from site_operand_argument(.., 2, ..) at 3233, so the specialized(SEAT_2)? match is a vestigial gate whose only post-D1 effect is the spurious carried-buffer refusal); runtime-implementer critical-path re-disposition (evt_6vxb4f1rxh3jk: with ExactIntU64 closed the witness terminal is now this Arg(2) reply-path refusal, so the deferral's off-critical-path ground is invalidated and it must be re-dispositioned from a carry to a cut). Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

> D0 RE-SCOPE 2026-08-22 (Architect evt_7h23767bakhgm; Steward adopts arm (a),
> widen-in-place). The original framing -- "remove a vestigial gate, no
> mechanism" -- was FALSE at point (1): removing the dead gate only RELOCATES the
> reply-arm refusal one function deeper and greens nothing (measured, below). The
> node now carries ONE change: teach the carried-operand projection to be
> need-directed (project a ResourceScalar seat via the resource-token
> observation, not the byte-span observer), with the dead gate removed as part of
> it. No kernel/TCB edit; no operator authorization in play.

> D1 OUTCOME 2026-08-22 (implementer evt_3xy5qvjbt8zqe; Architect evt_230wt9hcynjmh).
> The resource-aware projection is BUILT and reads SOUND against the AC-2 envelope
> (Architect on-report affirmation; formal review at the merge Decision). It
> cleared the whole effect-seat layer and the compile then stopped on a LATENT
> completeness gap in the merged [[RT-DEAD-ARM-EFFECT-LOWERING]] node (a
> provably-dead arm's source-joins left neither emitted nor dispositioned; 19/19
> measured). Per this node's AC-1 that distinct downstream blocker is CUT, not
> folded: the successor [[RT-DEAD-ARM-JOIN-DISPOSITION]] completes the dead-arm
> trap. The projection greens nothing alone, so the two CO-LAND as ONE candidate
> on this branch (§8 green-witness); this node's AC-1/AC-4/AC-5 are met when the
> successor's completing compile lands. `depends_on` now includes the successor.

> LAYER-3 RULING 2026-08-22 (Architect evt_r3tt1gpv4tkn on research advisory
> evt_5f0rzjghjhmy9; HS=3 closed, next re-trigger HS=6). Clearing the
> join-consumption layer exposed TWO disjoint pre-existing downstream refusals at
> once -- a materialized-dead join reconciliation (StaticOriginId(288)) and a
> distinct-subsystem completeness gap (OrientedSubcontinuationPlanV1 IH-marker) --
> so the stacked refusals are the absence of a traversability discipline for this
> cold path, not defects in the built projection+disposition (which stay, measured
> sound; they green nothing only because of these gaps -- do NOT revert). Ruled
> shape (SCOPE/SEQUENCING to the Steward): (a) run an enumeration/coverage step
> FIRST to bound the COMPLETE remaining refusal set before sequencing --
> [[RT-COLD-LOWERING-PATH-ENUMERATION]], cut and released, `depends_on` above; (b)
> cut a bounded successor per genuine gap it surfaces, each on its own
> reconciliation/completeness merits with BOTH validators untouched; (c) co-land
> the whole set as one green candidate (§8). This node's full AC-1/AC-4/AC-5 green
> now waits on the enumeration + the per-gap successors. No kernel/TCB; no operator
> authorization.

> ENUMERATION SUCCESSOR CUT 2026-08-22 (Steward, on both enumeration reports;
> Architect evt_4ag90qfacmgwy + evt_4jcnbhx8nqwdy). Report 1 (RT_PARITY_SOURCE):
> five mechanisms. Report 2 (checked-program family): the IH-marker only, over two
> programs, depth-1. Per-gap ledger: (1) materialized-dead join 288+301 as a class
> => [[RT-MATERIALIZED-DEAD-JOIN-RECONCILE]]; (2) IH-marker producer-fix + a
> mandatory post-fix re-enumeration gate => [[RT-IH-MARKER-PRODUCER-COMPLETE]];
> (3) BoundaryCarrier carried-recursive-hypothesis (`rt_allocate_stage`, 1 entry)
> is a NEW WITNESS of THIS node's OWN layer-1 effect-seat mechanism -- FOLDED HERE
> (extend the need-directed carried-operand projection to cover it; diagnose
> same-family at fix time; do NOT treat as novel), not a separate node; (4)
> closure-crossing (`rt_write_writable_stage`) is the tracked standing limitation
> of the merged RT-CLOSURE-BOUNDARY-LANE lane, excluded from this arc; (5) the
> `rt_write_pair_source` elaboration KernelRejected TypeMismatch is NOT a lowering
> refusal (never reaches the backend) -- explicit disposition owed (correct
> rejection of an ill-typed fixture, or a real elaboration gap routing to
> language/elaboration), not silently dropped. Both validators stay byte-untouched.

Make `FsReadAt`'s `Argument(2)` buffer reply/ok-construction arm admit a carried
`ResourceScalar` buffer, by giving the carried-operand projection path
(`site_operand_argument` / the carried branch it delegates to) need-directed
dispatch. After [[RT-RESOURCE-RELEASE-CARRIED-OBSERVE]] made the REQUEST-arm
resource-token readers carried-capable and [[RT-EXACTINT-CARRIED-OBSERVE]] closed
the `ExactIntU64` terminal, the `cap41_*` rows advance to this arm, where two
refusals sit in series: the dead specialized-only gate (3267), and behind it the
byte-span observer (`observe_carried_bytes_span`, 1405) which refuses any need
that is not `BytesPointerLength`. Removing only the first exposes the second.

This is a `ResourceScalar`-family item (RT-RESOURCE-RELEASE's leftover reader),
NOT `ExactIntU64` work. It is kept distinct in the accounting from
[[RT-EXACTINT-CARRIED-OBSERVE]] even though it shares the `effects.rs` edit
region for contention.

This node is also the tracked restoration home for two carried-observation-family
TEST items the campaign deferred here because this removal is what unblocks the
full `cap41_*` compile (Architect `evt_4wkc748vgfhhf`): the `ExactIntU64`
runtime-half end-to-end test (AC-4) and a durable keyed-on-need discriminator for
the ResourceScalar route (AC-5). Both are safe-direction coverage matters, not
soundness holes -- the production code (route key, ledger re-derivation, the
proven decoder) is intact.

# WHY REMOVAL ALONE IS INSUFFICIENT -- the D0 finding

(Measured: runtime-implementer evt_vtyk18cp0zcv. Architect ruling:
evt_7h23767bakhgm.)

The binding is dead; the refusal is load-bearing. Two different objects, the same
word "vestigial". The earlier ruling (`evt_2qdpkfvtqrxzy`) confirmed the
destructured `span_origin` at 3267 is UNUSED (TRUE) and reasoned from that to
"the gate's refusal is spurious" (FALSE). Grounded as-implemented on
`origin/main` `720f31e34`, `effects.rs`:

- `3267` gate: `let Lowered::ResourceToken { value: span_origin } =
  seats.specialized(SEAT_2)? else {...}` -- `specialized(SEAT_2)?` refuses a
  CARRIED SEAT_2 (the pre-removal witness); `span_origin` unused.
- `3275` live projection: `site_operand_argument(.., 2, ..)` binds the real
  `span_argument` for `PrivateBufferSpan`. For a carried SEAT_2 it routes to
  `observe_carried_bytes_span`, which refuses any non-`BytesPointerLength` need
  at `1405` (the post-removal witness). `FsReadAt` `Argument(2)`'s need is
  `ResourceScalar`.

Measured by applying the removal in scratch and reading the terminal (reverted):
the same witness, same blocker, moves from the `3267` gate to
`observe_carried_bytes_span` one function deeper -- the compile does not complete
and the `cap41_*` rows do not advance. `observe_carried_bytes_span` is a
BYTE-SPAN observer; its `1405` refusal is a deliberate fail-closed contract and
must NOT be weakened. `PrivateBufferSpan` arg0 wants the buffer RESOURCE, so the
byte-span route is simply the wrong projector for this seat.

# MECHANISM (Architect design ruling evt_7h23767bakhgm)

Need-directed dispatch in the carried-operand projection path
(`site_operand_argument` / the carried branch it delegates to): a seat whose
declared need is `ResourceScalar` projects via the resource-token observation,
NOT the byte-span observer. Reuse `lower_resource_token_seat` (`1936`) -- its
`Carried(word)` branch already carries the full fail-closed envelope
(`require_i64(tag == InvocationBorrowed)`, `require_i64(class ==
BorrowedOpaque)`, then `emit_carrier_scalar`). Make it NEED-DIRECTED (dispatch on
the planner's declared need), NOT an `FsReadAt`-SEAT_2 special-case -- that
subsumes the whole `ResourceScalar` family in the carried projection rather than
proliferating per-seat patches (subsume-don't-proliferate). The dead `3267` gate
is removed as PART of this change.

# HOW THIS REACHED THE CRITICAL PATH

The runtime-implementer originally dispositioned this DEFERRED on three grounds
(evt_6vxb4f1rxh3jk): (a) closing it is a mechanism change, not a fold; (b) the
direction is safe (a clean refusal, no miscompile); (c) it was off the `cap41_*`
critical path, because the rows hit the `ExactIntU64` terminal at `Arg(1)`
first. Closing `ExactIntU64` is precisely what moves it onto the path -- the
witness terminal is now this `Arg(2)` reply-path refusal. Grounds (a) and (b)
stand unchanged; ground (c) is invalidated, which is why the disposition is now
a CUT (this node) rather than a carry. NHC blocks on it.

# `D0` -- CLASSIFY (COMPLETE 2026-08-22; hard-stop worked as designed)

D0 ran the three-point classification and HARD-STOPPED, exactly as the frame's
"if any of (1)-(3) needs real design, hard-stop and re-scope" clause directs:

- (1) FALSE (the stop). `site_operand_argument(.., 2, ..)` does NOT project the
  buffer correctly when `Arg(2)` arrives CARRIED -- its carried branch routes to
  `observe_carried_bytes_span`, which refuses the `ResourceScalar` need. Removing
  the gate relocates the refusal one function deeper; it greens nothing.
- (2) CONFIRMED. `Arg(2)` is already validated as a resource token on the REQUEST
  path (`2471` via `lower_resource_token_seat`); the reply-path gate is a genuine
  redundant re-validation.
- (3) CONFIRMED. `span_origin` is consumed by nothing; the crate builds with the
  gate deleted, no unused-variable diagnostic.

Outcome: the node re-scoped to arm (a) (widen-in-place, mechanism included) per
the Architect's ruling and the Steward's disposition. See MECHANISM above.

# `D1` -- THE RESOURCE-AWARE PROJECTION (the removal rides inside it)

Implement need-directed dispatch in the carried-operand projection path so a
`ResourceScalar` seat is projected via the resource-token observation
(`lower_resource_token_seat`'s carried branch), and remove the dead `3267` gate
as part of the same change. Dispatch on the planner's declared `EffectSeatNeed`
(structural signal), family-wide, not an `FsReadAt`-SEAT_2 special-case. This is
a `ken-runtime` cranelift-lowering change -- no new `Avail` partition, no kernel
edit. The soundness envelope below (from the required reviewer) is binding on the
landed code.

# ACCEPTANCE

- **AC-1 (the reply path admits carried).** The `FsReadAt` `Argument(2)`
  reply/ok-construction path no longer refuses a carried buffer; the `cap41_*`
  rows advance past this blocker. Report the full per-row disposition. A further
  distinct blocker exposed behind this one is a measurement to report and cut
  (or, if the rows go green, hand back to [[NATIVE-HANDLE-CARRIER]]'s
  `D-final`), not a failure of this node.
- **AC-2 (soundness envelope -- the Architect's required-reviewer conditions on
  the landed code).** All three, on the resource-aware carried projection:
  1. The carried `ResourceScalar` projection RE-RUNS the fail-closed tag+class
     guard on the carried word (satisfied by routing through
     `lower_resource_token_seat`); never project a raw carried word as a
     resource. Re-observation on the reply path is idempotent and sound;
     threading the request-path value (validated at `2471`) instead is an
     optional optimization, not a soundness requirement.
  2. Dispatch on the STRUCTURAL signal (the planner's declared `EffectSeatNeed`),
     never a self-reported value.
  3. CENSUS every seat/need routed through the carried branch of
     `site_operand_argument`: state which needs map to the byte-span observer and
     which to the resource-token path, so no seat silently changes projection.
     `observe_carried_bytes_span`'s `1405` refusal stays as the byte-span
     observer's own backstop (not weakened).
- **AC-3 (no regression).** All currently-compiling lowering preserved;
  workspace-green in CI. (Local: targeted `-p` only, never `--workspace`; the
  respin gate is `-p ken-runtime` all-binaries + `-p ken-cli` + `-p ken-verify`,
  the coverage the predecessors ran.)
- **AC-4 (restore the ExactIntU64 runtime-half end-to-end test).** Carried in
  from [[RT-EXACTINT-CARRIED-OBSERVE]] AC-3, whose runtime half was deferred
  here purely by sequencing (Architect `evt_4wkc748vgfhhf`, Part C): on that
  node the witness terminated at THIS Arg(2) gate before building fully, so the
  generated code's runtime behaviour was not observable end-to-end. Removing the
  gate unblocks the full compile, which is exactly when it becomes observable.
  Add the end-to-end test: an in-range carried `Int` at the positioned seats
  advances; an out-of-range carried `Int` returns `valid=0` into the operation's
  existing narrow-failure lane (InvalidBounds/InvalidOffset). This is missing
  end-to-end-on-this-op coverage, not an unproven mechanism -- the decoder is
  `narrow_carried_int_u64`, the same one `BufferAllocate` `0` already ships and
  runs.
- **AC-5 (restore a durable keyed-on-need discriminator for the ResourceScalar
  route).** [[RT-EXACTINT-CARRIED-OBSERVE]] had to drop the ResourceScalar
  route's keyed-on-need negative test (its vanishing-contrast form -- "an
  un-closed need still refuses" -- inverted to a false failure as the
  `ExactIntU64` need closed; a real, green-suite-invisible coverage loss,
  Architect `evt_4wkc748vgfhhf`, Part B). The route's key stays STRUCTURALLY
  enforced in code by the ledger's independent `(CarriedWord, ResourceScalar)`
  second-admissibility re-derivation (byte-unchanged; the Architect verifies
  this at RT-EXACTINT review) -- what was lost is the persistent negative TEST.
  Restore it in a DURABLE, positive cross-key form: one witness where a
  ResourceScalar carried seat and an `ExactIntU64` carried seat each route
  through their OWN decoder. This node's completing compile is what makes such
  a witness available. Do NOT restore the fragile vanishing-contrast form.
- **Required reviewer:** the Architect is the required reviewer on this node's
  merge Decision (soundness-adjacent completeness removal, plus the two
  carried-observation-family test-hardening ACs above) and confirms the D0
  classification. Adversary hunts the landed code.

# EXPLICITLY NOT IN SCOPE

- **The `ExactIntU64` need** -- closed by [[RT-EXACTINT-CARRIED-OBSERVE]]. This
  node touches only the `ResourceScalar` reply-path gate.
- **Any `Avail` partition change or new route.** The fix reuses the EXISTING
  `lower_resource_token_seat` observation, need-directed; it adds no `Avail`
  partition and no new route.
- **Weakening `observe_carried_bytes_span`'s `1405` refusal.** It stays the
  byte-span observer's fail-closed backstop; the fix routes ResourceScalar seats
  AROUND it, it does not relax it.
- **A per-seat / `FsReadAt`-SEAT_2 special-case.** Dispatch is need-directed and
  family-wide.
- **The REQUEST-path resource-token validation (`2471`).** It stays; it is the
  real check.
- **Any kernel / TCB edit.** No operator TCB authorization is in play.

# CONTENTION

`ken-runtime` cranelift backend lowering (`effects.rs`), the same file region as
[[RT-EXACTINT-CARRIED-OBSERVE]] (predecessor). Single ring, single lane;
released to the runtime ring only after RT-EXACTINT merges (its D1 candidate is
in review). [[NATIVE-HANDLE-CARRIER]] is held on this node.

# CAPABILITY TIER

T1 (confirmed by D0, 2026-08-22). The frame's own tier line said it escalates
from T2-leaning if D0 finds real design; it did -- point (1) was false and the
node is now genuine mechanism work (need-directed dispatch in the carried
projection, with a three-condition soundness envelope). runtime-implementer's
default seat (Opus) is T1, correct for this. Size M -- the projection mechanism
plus the two carried-in test-hardening ACs (AC-4, AC-5).
