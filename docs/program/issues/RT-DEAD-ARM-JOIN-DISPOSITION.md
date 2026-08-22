---
id: RT-DEAD-ARM-JOIN-DISPOSITION
title: "The merged RT-DEAD-ARM-EFFECT-LOWERING trap short-circuits a provably-dead arm's lowering but leaves that arm's planned source-join origins neither emitted nor dispositioned, so finalize_join_disposition (joins.rs:1675) fires 'neither emitted nor statically unselected' once a downstream fix clears the effect-seat layer in front of it (19/19 unconsumed origins measured inside provably-dead arms, StaticOriginId(20)); complete the trap by dispositioning a PROVED-dead arm's joins as statically unselected (add to dispositioned_join_origins), reusing the RT-LEXICAL-RECURSOR-CONSUMERS D2b abandoned-region mechanism -- a latent completeness gap SURFACED (not caused) by RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL clearing the projection layer, co-landing with it as one candidate"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-DEAD-ARM-EFFECT-LOWERING]
blocks: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]
github: null
origin: "Measured by runtime-implementer during RT-FSREADAT D1 (evt_3xy5qvjbt8zqe): the resource-aware carried projection cleared the effect-seat layer and the compile then stopped on `Cranelift backend failure: function left planned source join StaticOriginId(20) neither emitted nor statically unselected` -- 37 required joins, 18 covered; the dead-arm predicate over all 19 unconsumed origins returned 19/19 inside provably-dead arms. Architect ruling evt_230wt9hcynjmh: this is a distinct completeness gap in the merged [[RT-DEAD-ARM-EFFECT-LOWERING]] node, cut as a successor exactly as RT-FSREADAT's own AC-1 prescribes ('a further distinct blocker exposed behind this one is a measurement to report and CUT ... not a failure of this node'). Latent since RT-DEAD-ARM merged; the invariant was unreachable until RT-FSREADAT cleared the layer in front of it. Steward-filed per COORDINATION section 2."
---

# WHAT THIS NODE IS

Complete the `RT-DEAD-ARM-EFFECT-LOWERING` trap. When the trap short-circuits a
PROVABLY-dead effect arm's lowering, it must ALSO disposition that arm's planned
source-join origins as statically unselected -- not merely skip emitting them.
Today it skips lowering but leaves the joins undispositioned, so the source-join
completeness invariant (`finalize_join_disposition`) is left an origin it can
neither find emitted nor find statically unselected.

This is a distinct mechanism in a distinct (already-merged) node, NOT a
widen-in-place of the FsReadAt projection. It is cut as a successor per
[[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]]'s AC-1. "Trap an arm" and "disposition
that arm's joins" are one semantic unit WITHIN the dead-arm mechanism -- they
belong together here, in the dead-arm node's territory, not in the projection.

# THE MEASURED FACT (authoritative; runtime-implementer evt_3xy5qvjbt8zqe, Architect grounded evt_230wt9hcynjmh)

Grounded on `origin/main`:

- `finalize_join_disposition` (`joins.rs:1675`) requires, over the function's
  required joins, `covered = consumed_join_origins ∪ dispositioned_join_origins`.
- A provably-dead arm short-circuited by the trap (`effects.rs:489`/`1995`,
  `effect_arm_is_provably_dead`) is neither emitted nor added to
  `dispositioned_join_origins`.
- So the invariant fires `function left planned source join StaticOriginId(20)
  neither emitted nor statically unselected`. Measured: 37 joins required, 18
  covered; all 19 unconsumed origins are inside provably-dead arms (19/19, no
  exceptions).

The direction is SAFE: the invariant refuses (fail-closed, no miscompile). It is
latent -- unreachable until `RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL`'s projection
cleared the effect-seat layer that previously refused first. Surfaced by that
node, not caused by it.

# MECHANISM (Architect design ruling evt_230wt9hcynjmh; self-contained, precedented)

When the trap fires on a provably-dead arm, disposition that arm's planned
source-join origins as statically unselected -- add them to
`dispositioned_join_origins` -- reusing the EXACT mechanism
`RT-LEXICAL-RECURSOR-CONSUMERS` `D2b` used for an abandoned `Let` body (its doc
records that it removed this identical "neither emitted nor statically
unselected" refusal). A trapped dead arm is the same category of abandoned
region. Bounded and precedented, not a redesign.

# ACCEPTANCE (the soundness envelope is binding; Architect required reviewer)

- **AC-1 (completeness closed).** With the fix, the `cap41_*` compile no longer
  stops at `finalize_join_disposition`; the 19/19 provably-dead-arm origins are
  dispositioned as statically unselected and the function's source joins are
  covered. Report the per-origin disposition.
- **AC-2 (disposition-follows-deadness, NEVER the reverse).** Disposition ONLY
  the joins of arms the trap has ALREADY proved dead
  (`effect_arm_is_provably_dead` / `origin_is_in_provably_dead_arm`). Never widen
  the dead-arm predicate to make disposition succeed; never disposition a join
  whose arm is not provably dead -- that would statically-unselect a reachable
  region, a miscompile.
- **AC-3 (route through the existing backstop).** Route through
  `validate_materialized_dead_join_cfg` (`joins.rs`); do NOT bypass it. It
  already verifies a dispositioned join's blocks are unreachable-from-entry with
  no live predecessor, so a wrongly-dispositioned LIVE join fails closed there.
  Preserve this built-in negative control.
- **AC-NEG (mandatory negative control).** A LIVE arm with an unemitted join
  must STILL trip `finalize_join_disposition`. The fix must not
  blanket-disposition to silence the invariant -- only provably-dead arms' joins
  get dispositioned. Ship one durable test in that shape.
- **AC-BACKSTOP.** Keep `finalize_join_disposition`'s "neither emitted nor
  statically unselected" refusal as the fail-closed backstop -- it is what caught
  this; do not weaken it.
- **AC-NO-REGRESSION.** No lowering currently on `main` changes disposition;
  whole-suite green in CI (`COORDINATION §12`). Local: targeted `-p` only, never
  `--workspace`; the runtime respin gate is `-p ken-runtime` all-binaries + `-p
  ken-cli` + `-p ken-verify`.
- **Required reviewer.** The Architect is the required soundness reviewer on the
  landed code (dead-arm-completeness invariant); the Adversary hunts it for a
  dead-arm-predicate over-widen (the one over-accept shape: disposition made to
  precede/loosen the deadness proof).

# CO-LANDING WITH RT-FSREADAT (one candidate, two nodes)

The projection D1 already built on `wp/RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL` is
a correct, complete unit and a clean ancestor, but it greens nothing observable
alone. RT-FSREADAT's full `AC-1`/`AC-4`/`AC-5` all need the completing compile
THIS node delivers. Per the Architect and the §8 green-witness bar, build this
fix immediately on the SAME seat and the same branch, and land BOTH mechanisms
(resource-aware projection + dead-arm join-disposition) as ONE candidate so the
`cap41_*` compile greens and both nodes' ACs are met together -- rather than
landing a projection partial that greens nothing. The single merge Decision
covers both nodes; both close on that merge. The Architect reviews both
mechanisms; the Adversary hunts both.

# EXPLICITLY NOT IN SCOPE

- **Widening the dead-arm predicate.** Deadness is proved first, independently;
  disposition follows it. Never the reverse.
- **Bypassing or weakening `validate_materialized_dead_join_cfg` or
  `finalize_join_disposition`'s refusal.** Both stay as fail-closed backstops.
- **Any kernel / TCB edit.** This is `ken-runtime` cranelift lowering
  (source-join completeness); no operator authorization is in play.
- **The FsReadAt projection mechanism** -- that is
  [[RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]]; this node completes the dead-arm
  trap only.

# SEQUENCING

`depends_on: [RT-DEAD-ARM-EFFECT-LOWERING]` -- this completes that merged node's
trap mechanism. `blocks: [RT-FSREADAT-REPLY-BUFFER-GATE-REMOVAL]` -- RT-FSREADAT's
AC-1/AC-4/AC-5 need this node's completing compile, and the two co-land as one
candidate. Single ring, single lane; `NATIVE-HANDLE-CARRIER` `D-final` re-runs
once this + the projection land and the `cap41_*` rows go all-green.

# CONTENTION

`ken-runtime` cranelift lowering: `joins.rs`
(`finalize_join_disposition`/`dispositioned_join_origins`/`validate_materialized_dead_join_cfg`)
and `effects.rs` (the dead-arm trap at `489`/`1995`). Same branch as the
RT-FSREADAT projection (co-landing). No other lane touches this region.

# CAPABILITY TIER

T1: a soundness-adjacent completeness invariant on the source-join layer, with a
four-condition envelope and a mandatory negative control. Bounded and precedented
(RT-LEXICAL-RECURSOR-CONSUMERS `D2b`), so not a redesign, but the over-widen risk
is real and the review turns on the disposition-follows-deadness argument.
runtime-implementer's Opus seat (T1) is correct. Size M.
