---
id: KERNEL-CONV-CONGRUENCE-CLOSURE
title: "Complete conv_struct_path's congruence closure for the five reachable arm-less formers — Omega, Cast, Quot, QuotClass, QuotElim — each a same-former congruence arm matching its real equality rule with a live source consumer + directional rejects; plus documented unreachability proofs for Refl, Let, and J (no arm). Completeness-only, trust-delta zero"
status: active
owner: kernel
size: M
gate: none
depends_on: [KERNEL-CONV-TRUNC-CONGRUENCE]
blocks: []
github: null
origin: "Architect kernel-level section 1b in evt_579jhptqfzcgn: conv_struct's congruence closure is incomplete for a SET of formers, not just Trunc; adding arms reactively one-per-consumer is the anti-pattern. This node captures the DELIBERATE structural closure of the remainder after the truncation pair, so the gap is named and tracked now rather than tripped later. Steward-filed per COORDINATION section 2, 2026-08-23."
---

> # FRAMED + SCHEDULED 2026-09-10 (operator ruling). Floor WP
> # [[KERNEL-CONV-TRUNC-CONGRUENCE]] is MERGED; the operator directed this node
> # DONE, with SYNTHESIZED live consumers for the arm-less formers.
> #
> # The earlier "HELD, no blocked consumer" rationale is SUPERSEDED. The operator
> # ruled (2026-09-10) the missing arms be closed PROACTIVELY and the WP itself
> # SYNTHESIZE the live consumers that exercise each one — rather than wait for a
> # quotient/cast/level-eq program to surface and trip the `_ => false` catch-all.
> # See the new consumer-synthesis deliverable and AC-CONSUMER-REACHES below.
> #
> # SEQUENCING: inserted at the FRONT of the L2 queue and run via a TEMPORARY
> # KERNEL RESEAT of the L2 slot (KERNEL-NESTED-IND precedent). The language ring
> # resumes its own queue (head LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN) after this
> # lands. Each arm STILL needs an Architect design confirmation of its REAL
> # equality rule BEFORE build — that per-arm design pass is the first step, and
> # it is where a genuinely-unreachable former (e.g. Refl, shadowed by the
> # Omega-shortcut) is discharged by a documented unreachability proof instead of
> # a synthesized consumer.
> #
> # SETTLED SCOPE — REVISED (Architect evt_116724dpe5s6e supersedes the
> # J-reachable call in evt_3d4823xtmab8). The `_ => false` set is EIGHT formers,
> # split FIVE reachable arms (Omega, Cast, Quot, QuotClass, QuotElim) + THREE
> # documented unreachability proofs (Refl, Let, J). J was briefly ruled a
> # reachable arm (evt_3d4823xtmab8) then re-ruled UNREACHABLE on the well-typed
> # path (evt_116724dpe5s6e): it joins Refl and Let — documented proof, NO arm,
> # NO consumer. The remaining departure from the ORIGINAL frame is Let (must be
> # documented, not built). The per-arm ruling (rule, consumer shape, directional
> # rejects, invariants INV-1..5) remains the exact mechanism for the five
> # reachable arms.
> #
> # INCREMENTS: increment 1 = Omega + Quot + QuotClass (evt_5mbj1dfj3j23h) —
> # LANDED at origin/main f1e7e516, blob-verified 5/5. Increment 2 = Cast +
> # QuotElim ONLY, plus the J unreachability proof and the correction of the
> # stale whnf-J comment (evt_116724dpe5s6e). Branch
> # `wp/KERNEL-CONV-CONGRUENCE-CLOSURE` re-cut off f1e7e516.

# The settled gap (Architect design ruling evt_3d4823xtmab8, grounded on 661d988d7)

The function is `conv_struct_path` (the delta-ledger-threaded form;
`convert_type` calls it with an empty ledger), `crates/ken-kernel/src/conv.rs`,
heterogeneous default `_ => false`. It ALREADY has same-former arms for Type,
Var, Const, IndFormer, Constructor, Pi, Lam, Sigma, Pair, App, Proj1, Proj2,
Elim, Ascript, Absurd, Eq, IntLit, **Trunc, and TruncProj** (the frame's
original list understated it — those are done, do not re-open them). TruncProj
`|t|` congruence-on-operand (`conv.rs:795`) is the landed precedent the QuotClass
arm must match.

The true set falling to `_ => false` with both sides the same former is EIGHT,
split by `whnf_progress` reachability (whnf is the oracle: a former needs an arm
IFF whnf can leave it as the head — a value or a stuck neutral):

- REACHABLE — need a same-former congruence arm + a live source consumer (FIVE):
  **Omega** (whnf leaf; `level_eq` on levels, mirrors the Type arm),
  **Cast** (neutral when cast_reduce is None; four fields compared structurally),
  **Quot** (whnf leaf; carrier + relation),
  **QuotClass** (whnf leaf; representative ONLY — the TruncProj precedent; does
  NOT decide R-relatedness), and
  **QuotElim** (neutral on a non-canonical scrut; motive/method/respect/scrut,
  mirrors the Elim arm).
- UNREACHABLE — documented unreachability proof, NO arm, NO consumer (THREE):
  **Refl** (every well-typed Eq-conversion returns at the `is_omega_type` guard
  `conv.rs:566` and the App-arg re-entry `:782-787` BEFORE conv_struct_path —
  the Omega-shortcut the frame hypothesized, confirmed genuine), **Let** (whnf
  reduces it unconditionally, no stuck case — the `_` is dead), and **J**
  (re-ruled unreachable on the well-typed path per evt_116724dpe5s6e: a
  well-typed neutral J cannot present two should-convert-yet-inconvertible forms
  at conv_struct_path — the increment-2 deliverable carries the documented proof
  and corrects the stale whnf-J comment).

The substantive departures from the original frame: **Let must be documented,
not built** (unreachable), and **J is documented-unreachable** — the
evt_3d4823xtmab8 "J is a reachable arm" call was superseded by
evt_116724dpe5s6e.

# Deliverable

Five new same-former congruence arms in `conv_struct_path` — Omega, Cast,
Quot, QuotClass, QuotElim — each matching its REAL equality rule per the
Architect's per-arm rulings in evt_3d4823xtmab8 (that ruling is the exact
mechanism; the per-arm rule, consumer shape, and directional rejects are
enumerated there). Three documented unreachability proofs (Refl, Let, J) in
place of arms. The closure-level invariants BIND every arm:

- INV-1 fail-closed / completeness-only — every arm is congruence (all fields
  equal ⇒ terms equal); it can only recognise MORE true equalities, never admit
  a false one. The trust root cannot weaken; same posture as the landed Eq arm.
- INV-2 proof components compared STRUCTURALLY, not skipped (Cast's `e`,
  QuotElim's `respect`) — deliberately NOT proof-irrelevance (that needs
  the type, would grow the TCB surface, and is a SEPARATE future increment,
  out of scope here).
- INV-3 whnf is the reachability oracle — no arm for an unreachable former (dead
  TCB), no omission for a reachable one.
- INV-4 thread `child_path` on every recursive compare (never `path` / `&[]`);
  Omega is exempt (leaf, no recursion).
- INV-5 Omega levels via `level_eq`, not `==` (mirrors the Type arm).

**SYNTHESIZE a live source consumer per REACHABLE arm (operator ruling).** For
each of the five, author a ken-source artifact (a `.ken` fixture or catalog
module) that FORCES `conv_struct_path` to decide convertibility with that former
as the head and elaborates/type-checks ONLY once the arm exists — failing on the
pre-arm tree because the catch-all rejects the genuine convertibility, passing
once the arm lands. The acceptance must be equal-but-not-syntactically-identical
(a non-`==` field difference that resolves by recursion / level_eq) so the
`a == b` fast path does not mask the arm and make the test vacuous. The three
unreachable formers (Refl, Let, J) are each discharged by their documented
proof, NOT a consumer.

# Acceptance criteria

- AC-ACCEPT / AC-DISCRIMINATE (per REACHABLE arm — the five). Both directions per
  COORDINATION section 7: an acceptance case (equal-but-not-syntactically-
  identical instances accepted, reaching the arm) AND a directional
  discrimination case for EACH compared field (a lone difference in that field
  still rejects — proven not to over-accept).
- AC-CONSUMER-REACHES (per REACHABLE arm). The synthesized ken-source consumer
  FAILS to elaborate on the arm-less base (cite the exact refusal, attributable
  to the catch-all) and ELABORATES / type-checks once the arm lands — a reaching
  witness that the arm is exercised by real source, not only a conv-level probe.
  A consumer that passes on the base (never reaches the arm) does not discharge
  this. If a source-level consumer cannot be made to reach an arm, HARD-STOP and
  report rather than substituting a Rust-only unit probe.
- AC-UNREACHABLE-PROVED (Refl, Let, J). Each is discharged by a documented
  unreachability proof in place of an arm/consumer, per the Architect ruling
  (J per evt_116724dpe5s6e) — no dead TCB arm is added. Increment 2 also
  corrects the stale whnf-J comment.
- AC-ZERO-TRUST. Zero `trusted_base()` delta (completeness-only; INV-1).
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p ken-kernel` only.

# Reviewers

kernel-QA + the Adversary (over-accept hunt per arm) + the Architect (equality
rule confirmation per arm + as-implemented pass). Trust-root change; Steward
routes the merge.

# Capability tier

T1 (multiple trust-root soundness arms, several with non-trivial equality rules —
quotients, Cast, level equality). Size M. The truncation pair is NOT here (that
is the floor WP).
