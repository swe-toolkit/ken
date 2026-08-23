---
id: KERNEL-CONV-TRUNC-CONGRUENCE
title: "Add the truncation-former congruence pair (Trunc + TruncProj) to conv_struct so the ordinary conversion gate (whnf + congruence) recognizes convertible-but-not-syntactically-identical truncation types/eliminations — unblocking V3-FO-EMBEDDING-ADEQUACY's quotation-preservation Or arm"
status: merged
owner: kernel
size: S
gate: none
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Architect ruling evt_579jhptqfzcgn on the V3-FO-EMBEDDING-ADEQUACY D1 Trunc-congruence hard stop (language-implementer evt_4x7zr3jepz62t). DISPOSITION: route a kernel conversion-congruence prerequisite WP; do NOT narrow the fixed five-form slice, do NOT substitute full normalization for the ordinary gate (both refusals correct). Steward-filed per COORDINATION section 2, 2026-08-23. HS 5."
---

> # RELEASED 2026-08-23 — floor WP of the congruence-closure §1b. Frame: docs/program/wp/kernel-conv-trunc-congruence.md
>
> Trust-root change to conv.rs. The BUILD is released to the kernel ring now
> (it unblocks a live Lane-2 blocker); the MERGE is gated by the Steward on
> kernel-QA + Adversary + Architect as-implemented review + a resolved Decision.
> The truncation pair (Trunc AND TruncProj) is the Architect-settled floor; the
> remaining missing arms are the named follow-up
> [[KERNEL-CONV-CONGRUENCE-CLOSURE]] (deliberate, not reactive).

# The measured gap (Architect evt_579jhptqfzcgn, grounded in conv.rs)

`conv_struct` (`crates/ken-kernel/src/conv.rs:404`) whnfs both sides
(`:434-435`), takes the `a==b` syntactic fast path (`:436`), then matches
congruence arms for exactly: Type, Var, Const, IndFormer, Constructor, Pi, Lam,
Sigma, Pair, App, Proj1, Proj2, Elim, Ascript, Absurd, Eq, IntLit — catch-all
`_ => false` (`:578`). There is NO `(Term::Trunc, Term::Trunc)` arm. So two
truncation types `Trunc(p1)`/`Trunc(p2)` whose interiors are
convertible-but-not-syntactically-identical fall through to false.

`normalize` (`:288-289`) DOES recurse `Term::Trunc(a) => Trunc(normalize a)` and
TruncProj. That asymmetry is the whole symptom: equal full normal forms, false
ordinary conversion, because the ordinary gate is whnf + congruence (not full
NbE) and whnf leaves `Trunc(..)` at the head without descending. The Const-spine
fast path (`:419-432`) does not apply (Trunc is not a Const application). The gap
is exact — measured by language-implementer on the FO Or fixture
(evt_4x7zr3jepz62t) and test-driven by language-qa (evt_2rdx75e35bbhb).

# The fix (Architect-settled spelling + soundness)

Add the standard congruence-closure arm for the truncation former, plus its
eliminator TruncProj (the Or/quotation-preservation path reaches TruncProj the
moment it exercises the eliminator; splitting the pair guarantees a second stop):

    (Term::Trunc(p1), Term::Trunc(p2)) => conv_struct(env, ctx, p1, p2),

and the analogous TruncProj arm matching TruncProj's real formation/elimination
rule. language-qa test-drove the Trunc arm at evt_2rdx75e35bbhb: it flips only
the conversion-transition sentinel (5/1), then byte-restored.

WHY SOUND (Architect kernel/TCB analysis):
- Completeness-only / conservative: recognises strictly more TRUE equalities,
  never a false one — `‖A‖ ≡ ‖B‖` holds only when `A ≡ B` (the arm requires
  `conv_struct(p1,p2)`). Fail-closed direction, not a loosening. Same direction
  and justification as the Eq-congruence arm re-landed as a previously-missing
  closure (comment at `conv.rs:560-565`).
- Does NOT interact with proof-irrelevance: element-level Trunc irrelevance (any
  two proofs of `‖A‖` equal) is the Omega-shortcut in `convert` (`:345-347`,
  keyed on `is_omega_type`). This arm is about the truncation TYPE's congruence,
  reached via `convert_type` (`:397`) which deliberately bypasses the
  Omega-shortcut. Different judgments; the arm never squashes distinct
  propositions (only `A≡B ⟹ ‖A‖≡‖B‖`).
- Termination preserved: recurses on a structurally smaller subterm, identical
  shape to the Pi/Sigma/Eq arms. Sound in both the K1 body and its K2c NbE
  replacement.

# Scope call (Steward + kernel-leader, per the Architect)

This WP adds the TRUNCATION PAIR ONLY (Trunc + TruncProj). The complete missing
congruence-arm set the Architect enumerated is:

  Trunc, TruncProj, Quot, QuotClass, QuotElim, Cast, Omega (needs level_eq like
  the Type arm), Refl (mostly shadowed by the Omega-shortcut). (Let is fine —
  whnf reduces it.)

The remaining formers {Quot, QuotClass, QuotElim, Cast, Omega-level, Refl} are
deferred to [[KERNEL-CONV-CONGRUENCE-CLOSURE]], FILED NOW naming the full gap.
Rationale: (1) unblock the live embedding-adequacy Or arm fastest (one-hour
turn); (2) each trust-root arm stays minimal and individually auditable with its
own soundness argument + conformance pair — the Architect explicitly barred a
blanket add ("each added arm must match that former's real formation/equality
rule and carry its own conformance case"); (3) the truncation pair's spelling
and soundness are settled here, the remaining set's are not. Filing the follow-up
now (not deferring until the next consumer trips) keeps the closure deliberate,
not reactive — the anti-pattern the Architect named. kernel-leader may escalate
to fold the whole set into one WP if they judge it cleaner.

# Acceptance criteria

- AC-ACCEPT (per arm). Two convertible truncation types `‖A‖`/`‖B‖` with `A ≡ B`
  but not syntactically identical are ACCEPTED by `convert_type`. Same for
  TruncProj on convertible interiors.
- AC-DISCRIMINATE (per arm). `‖A‖` vs `‖B‖` with `A ≢ B` is still REJECTED — the
  arm is proven NOT to over-accept. Same for TruncProj. (COORDINATION section 7:
  both directions per arm, the arm's own kernel-level conformance cases.)
- AC-CONSUMER. The D1 consumer sentinel (`v3_fo_embedding_adequacy_d1.rs`, the Or
  cell) greens — the conversion-transition sentinel flips — confirming the
  consumer-side unblock. (Owned/verified by language on the held D1 branch after
  this lands; named here as the downstream effect, not this WP's test.)
- AC-ZERO-TRUST. Zero `trusted_base()` delta (completeness-only; no new trusted
  surface, no new axiom/opaque/primitive).
- AC-NO-REGRESSION. Whole-suite green in CI (COORDINATION section 12); local
  targeted `-p ken-kernel` only, never `--workspace`.

# Reviewers

kernel-QA + the Adversary (over-accept hunt: prove the arm does not squash
distinct propositions or admit a non-convertible interior) + the Architect
(as-implemented pseudocode-level pass, COORDINATION section 2/7). This is a
trust-root change; the Steward routes the merge only on all three + a Decision.

# Contention

Touches `crates/ken-kernel/src/conv.rs` only. No in-flight lane work touches
conv.rs (runtime lane: ken-host/ken-runtime/ken-elaborator; language lane:
ken-elaborator). Contention-free. Code change ⇒ full CI on merge.

# Capability tier

T1 (trust-root soundness change; the arm is small but review turns on the
soundness argument, not the diff). Size S.
