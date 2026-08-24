---
id: V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY
title: "The FoKripke checker_soundness mutual-recursion clique's real termination is not a single structural size-change thread on its declared parameters under the current size_rel abstraction -- it fails by ROTATION (the bare-Var matched-field descent arrives in one slot while the outgoing edge decreases from another) -- so full SCT admission of the real consumer needs an UPSTREAM resolution (re-elaborate the soundness recursion to one structural thread; preferred, no TCB), with a narrow size_rel completeness fix or a richer measure as operator-gated conditional arms; this is the real AC-CONSUMER home V3-FO-CHECKER-SOUNDNESS D3 waits on, distinct from the arity fix (KERNEL-SCT-TELESCOPE-CANON) and from any Cast/J or lexicographic-SCT node"
status: active
owner: language
size: L
gate: none
depends_on: [KERNEL-SCT-TELESCOPE-CANON]
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-22, executing the Architect's KERNEL-SCT D1 final ruling (evt_1gtmndpzh3xda) as corrected (evt_134z6mr80ymqp). The route-A arity fix (KERNEL-SCT-TELESCOPE-CANON) is correct and landing, but the D1 measurement on the exact FoKripke consumer (a84d71005 rebased onto 93d82a398) refuted the premise that arity was the whole SCT-pass gate: the real clique fails by ROTATION under the current size_rel abstraction, a measured size-change-abstraction gap the Architect routed to the language/spec enclave (owner of the proof's termination structure) with three exhaustive outcomes. This node is the real AC-CONSUMER's home and the successor V3-FO-CHECKER-SOUNDNESS.depends_on re-points onto -- NOT the helper-return documentation node and NOT a kernel node. Steward-filed per COORDINATION section 2. Estimated capability tier: T1 (open expressibility / proof-termination-structure question; if it lands on a kernel size_rel touch, that is soundness-bearing TCB work)."
---

> # AUTHORIZED as the FoKripke SCT-pass residual home -- Architect final ruling (evt_1gtmndpzh3xda) + correction (evt_134z6mr80ymqp), 2026-08-22
>
> This is the real `AC-CONSUMER` home that `V3-FO-CHECKER-SOUNDNESS` D3 waits on.
> It is NOT the helper-return documentation node
> [[LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN]] (that node is at most a PART of arm
> (b) below, and only if the binding rotation is shown to be helper-hidden --
> which the measurement does not show), and it is NOT a kernel
> lexicographic/permutation-aware node (that arm is ruled out -- see below).
>
> **`D0` first is a FORK, not a build.** No SCT widening is proposed until the
> enclave rules which of outcomes (a)/(b)/(c) the rotation is. Outcome (a) needs
> no TCB authorization; (b) and (c) are operator-gated and only arise if D0 rules
> them in. **The Steward takes no TCB authorization to Pat until D0 lands on a
> kernel arm.**

## The measured fact (authoritative, do not re-measure by instance-chasing)

The KERNEL-SCT D1 hard-stop ran the discriminating measurement on the exact
FoKripke consumer -- the nine-member `checker_soundness` mutual-recursion clique
(`fok_prop_imp_children_sound` .. `fok_prop_check_tree_sound_by_cert`), unique
direct cycle `0→8→7→6→5→4→3→2→1→0`, sourced from `a84d71005` rebased onto the
landed transport `93d82a398`. With the route-A arity fix (`WIP 27a84fcc5a94`)
the matrices are correctly dimensioned. The measurement (kernel-implementer
evt_1p7c98936ejb) established:

- The surviving strict thread is REAL bare-`Var` matched-field structural
  descent: `0.p6 ↓→ 8.p0` is `@2 child`, a matched-field `Var`; the continuing
  edges are bare `Var`s.
- It fails by ROTATION: after one lap the thread arrives at member-0 `p0`/`p1`,
  but member-0's outgoing `0→8` edge decreases only from `p6`, and `p0`/`p1` do
  not feed `p6`. So the one-lap `0→0` product has `Down` only off-diagonal at
  `[6][0]`/`[6][1]`, is not idempotent, and squares to the all-`Unknown`
  idempotent self-loop. No single structural thread survives a lap.
- Zero of the 50 descending call arguments are `Term::Cast` or `Term::J` (the
  `J`s surround call *results*, not recursion arguments) -- the Cast/J
  transport-opacity arm is refuted.

Three shapes surfaced on this consumer -- Cast/J opacity (refuted),
helper-return opacity, rotation. **The Architect's correction collapses them to
ONE predicate:** the FoKripke clique's real termination is NOT a single
structural size-change thread on its declared parameters under the current
`size_rel` abstraction. `sct_check`'s full idempotent closure has no
strict-diagonal self-loop. This is MEASURED, not conjectured. Diagnosing the
exact sub-shape by successive kernel-side dumps is the instance-chase to end;
the structural closure is to hand the framed question to the owner of the
proof's termination structure -- this node.

## The kernel-lexicographic arm is RULED OUT (decisive)

SCT with the idempotent-closure strict-diagonal criterion is already
sound-and-complete for the size-change abstraction (Lee-Jones-Ben-Amram, POPL
2001), INCLUDING lexicographic and permuting descents: a genuine rotating
descent expressible in the size-change graphs WOULD produce a strict-diagonal
idempotent matrix in the closure. The closure has none. Therefore the gap is
NOT the closure criterion (no lexicographic-awareness is missing -- building it
would be building a capability SCT already has), it is the GRAPHS: the
`size_rel` abstraction does not capture the clique's real decreasing measure.
**Do not cut a lexicographic/permutation-aware SCT node.**

## D0 -- the rotation fork (enclave's to rule; three EXHAUSTIVE outcomes, ordered by TCB cost)

The Architect routed the cause to the LANGUAGE/SPEC enclave as the owner of the
proof's termination structure. D0 rules which outcome the rotation is; it is not
the Steward's to settle and no widening is designed before it lands.

**(a) UPSTREAM, no TCB [PREFERRED, smallest-TCB].** Re-elaborate / restructure
the FoKripke soundness recursion so its decrease is a single structural thread
on ONE parameter across the mutual group (well-founded recursion on the
derivation / check-tree in a consistent slot), making termination
size-change-visible. The rotation may be an elaboration placement artifact (the
c-elab transport threads the sibling decrease into a rotating slot) or a
proof-signature choice; either is fixable upstream without touching the kernel
gate. This needs no operator authorization.

**(b) NARROW kernel `size_rel` completeness improvement [operator-gated TCB;
Architect required soundness reviewer; own AC-NEG-style negative control].**
ONLY if the enclave shows a genuine-but-currently-invisible STRUCTURAL relation
lies ON the binding thread (e.g., member-0's `p6` recursion argument IS a
structural subterm of its incoming `p0`/`p1`, hidden behind a helper return that
`size_rel` calls `Unknown`). That is a bounded "capture a real structural
relation" fix -- NOT a closure-criterion change and NOT a semantic-measure
checker. The helper-return node
[[LANG-SCT-OPAQUE-THROUGH-HELPER-RETURN]] is at most a PART of this arm, and
only if the binding rotation is shown helper-hidden rather than genuinely absent
-- which the current measurement does not show. Burden of proof is a
non-rotating closing thread. Its mandatory negative control (AC-NEG discipline):
a helper/coercion wrapping a NON-decreasing argument must still be REJECTED.

**(c) RICHER MEASURE [escalate to operator; DISPREFERRED].** If the real measure
is neither a single structural thread (a) nor a recoverable structural relation
(b) -- e.g., a size measure on a derived value spread across slots -- then it is
genuinely beyond the size-change abstraction, and admitting it would need
size-annotated types / a semantic termination facility. That is a large,
dangerous TCB direction the Architect does NOT recommend; it is an operator
decision, not a routine successor.

## Acceptance criteria

**`AC-D0-FORK`. D0 rules (a) vs (b) vs (c) with the measured non-rotating
closing thread (or its absence) as the burden of proof.** The ruling names the
outcome and, for (b)/(c), states the TCB touch precisely enough for the Steward
to route its operator authorization. No SCT widening is proposed before this.

**`AC-CONSUMER` (the real decisive buildability gate -- preserved here, not
lost).** The exact FoKripke `checker_soundness` clique (`a84d71005` rebased onto
`93d82a398`, with the arity fix `KERNEL-SCT-TELESCOPE-CANON` landed) passes FULL
admission (`kernel_check` AND `sct_check`) through whatever outcome (a)/(b)
D0 selects. This is the buildability gate the real consumer imposes; synthetic
green does not substitute. (If D0 lands on (c), this AC is what the operator
escalation is about.)

**`AC-NEG` (only if D0 selects (b)).** The narrow `size_rel` completeness fix
carries its own mandatory negative control: a helper/coercion wrapping a
NON-decreasing argument is still REJECTED. The fix must be
`Down`/`DownEq`-preserving, never `Down`-manufacturing.

**`AC-NO-REGRESSION`.** No SCT verdict already on `main` changes (only relevant
to arms (b)/(c)). Whole-suite green in CI (`COORDINATION §12`); targeted local
validation only, never `--workspace`.

**`AC-REVIEW`.** For any kernel `size_rel` touch (arm (b)), the Architect is the
required soundness reviewer (author not reviewer), and the Adversary hunts the
landed code for an over-accept hole. For the upstream arm (a), the change is
proof/elaboration and reviewed on the language ring's normal path; if D0 finds
the rotation is a surface-contract question, the spec enclave rules that part.

## Banned scope

- **Cutting a kernel lexicographic / permutation-aware SCT node.** Ruled out --
  the closure criterion is already complete for the abstraction.
- **Cutting a Cast/J transport-transparency node.** Refuted by measurement --
  zero descending args are `Cast`/`J`.
- **Proposing any SCT widening before D0 rules the fork.** D0 is a fork, not a
  build.
- **Taking a TCB authorization to the operator before D0 lands on arm (b)/(c).**
  Outcome (a) needs none.
- **Re-opening the arity fix (`KERNEL-SCT-TELESCOPE-CANON`) or the c-elab
  transport (`LANG-INDEXED-RECURSIVE-IH-DISCHARGE`).** Both are correct; this
  node is downstream of the arity fix.

## Sequencing

`depends_on: [KERNEL-SCT-TELESCOPE-CANON]` -- the real clique cannot pass full
admission until the arity is correctly dimensioned, so the arity fix is a
predecessor. `blocks: [V3-FO-CHECKER-SOUNDNESS]` -- this node is the real
`AC-CONSUMER` home, and V3-FO D3 resumes on its resolution. `gate: none` for the
D0 fork (investigation); any SCT widening surfaced by D0 (arm (b)/(c)) is fresh
operator-gated TCB the Steward routes to the operator at that point. Owner:
language ring, with the spec enclave ruling any surface-contract portion of the
fork and the Architect as required soundness reviewer on any kernel touch.

**Program consequence (for the operator briefing).** `V3-FO-CHECKER-SOUNDNESS`
-- a soundness-critical node -- now waits on this residual, correctly framed as
an upstream expressibility question first. It is materially harder than the
arity fix and its soundness is genuinely open (tracing a real decrease that
rotates across slots is not a structure-preserving peel). This is a status fact
for the operator, not a decision request; a TCB authorization arises only if D0
rules in arm (b) or (c).

## QUEUED FOLLOW-UP (non-blocking, from NODE B review) — fo_kripke.rs internal abstract Or

Attached by the Steward, 2026-08-24, at the Architect's request in the
[[LANG-MOD-OR-CANONICAL-HOME]] (NODE B) APPROVE (evt_7f2gt5dv2wycv, relayed by
language-leader evt_4gh9ddgnynk55). NON-BLOCKING and NOT part of this node's SCT
scope — parked here because this is the active FO-soundness home and the reconcile
question below is a soundness-design call the FO-soundness owner should make. It
did NOT gate NODE B (FokScopedOr correctly binds the canonical catalog Or).

Finding: after NODE B retires the Rust-prelude `Or`/`Inl`/`Inr` and homes a single
canonical `Core.Logic.Or`, `crates/.../fo_kripke.rs` still carries a SECOND
Or-shaped inductive — `FoSliceSignature.or_id`, a Rust-level `declare_inductive`
with a distinct GlobalId. It is the FO-Kripke soundness checker's INTERNAL abstract
slice signature for obligation-signature discovery (structural-shape matching
against a private template), not any consumer's Or denotation, so NODE B's
single-identity invariant is intact.

Two follow-ups for this node's owner (both non-blocking, address when the FO lane
is next active — do NOT interrupt an active lane for them):
- (a) Comment refresh: fo_kripke.rs comments now cite "the prelude's own Or
  declaration (prelude.rs)" as their pattern; that prelude Or no longer exists
  after NODE B. Refresh the stale citations.
- (b) Soundness-design decision: should the FO checker's internal abstract Or
  reconcile with the canonical catalog `Core.Logic.Or` GlobalId, or is
  structural-shape-matching against a private template the intended design? This
  is a soundness question for this node / the enclave, not for the canonical-home
  migration. Decide with the enclave when the FO soundness work resumes.
