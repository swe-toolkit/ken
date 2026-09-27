---
name: an-omitted-congruence-arm-is-sound-and-complete-only-if-its-reducer-fires-on-neutral-operands-not-just-canonical-ones
description: >-
  KERNEL-CONV-CONGRUENCE-CLOSURE increment 2 (742d1628, over-accept gate on
  conv_struct_path), verdict NO DEFECT / APPROVE. Adds Cast (A/B/e/t) and neutral
  QuotElim (motive/method/respect/scrut) same-former congruence, each field via
  conv_struct_path(child_path); deliberately NO J arm (documented-unreachable).
  Two transferable methods. (1) For a conversion congruence over a term carrying
  a PROOF field (Cast.e, QuotElim.respect), comparing the proof STRUCTURALLY is
  the sound direction -- it can only under-accept; proof-irrelevance-SKIPPING the
  proof is the over-accept hazard when the proof is proof-RELEVANT (type equality
  under OTT/univalence is not a mere prop). So the over-accept probe is "does the
  arm genuinely recurse on the proof field", not "does it skip it". (2) A
  documented-unreachable / omitted arm's claim ("head X cannot present here")
  holds only if the REDUCER that eliminates X fires on NEUTRAL operands, not just
  canonical constructors -- the tell is that the reducer keys on the operand's
  TYPE (or an availability fact), never on its syntactic shape. j_nonrefl fires
  for every eq whose inferred type whnfs to Eq, INCLUDING a neutral eq (variable
  proof), so a well-typed J always reduces (base on refl, else a Cast) and never
  stays neutral; only an ill-typed J reaches conv and hits _ => false. It is also
  COMPLETE, not just fail-closed: well-typed J reduces to a Cast the NEW Cast arm
  covers, and the reduct's B = motive-at-(b,eq) captures the proof so a
  proof-relevant motive still rejects inconvertible proofs there. A reducer that
  keyed on canonical structure would leave well-typed neutral heads stuck and the
  omission would be a real completeness gap. Sibling of the over-accept-gate
  fail-closed-arm family.
---

# An omitted congruence arm is sound AND complete only if its reducer fires on neutral operands, not just canonical ones

**Measured 2026-09-10 on `742d1628` (KERNEL-CONV-CONGRUENCE-CLOSURE inc 2), an
over-accept gate on the kernel conversion checker. Verdict NO DEFECT / APPROVE
(routed evt_1rrx3g3q6e0zp).** The change adds neutral `Cast` (A/B/e/t) and
`QuotElim` (motive/method/respect/scrut) same-former congruence to
`conv_struct_path`, each field recursed via `conv_struct_path(..., child_path)`,
and deliberately adds NO `J` arm.

## Method 1: a proof field is compared structurally; skipping it is the hazard

For a congruence over a term that carries a PROOF field -- `Cast A B e t` (e is
`A = B`), `QuotElim{..respect..}` -- the over-accept question is NOT "did it skip
the proof by irrelevance" as a safe simplification. In an observational /
univalent system a type-equality proof `e : A = B` is proof-RELEVANT (different
transports differ), so:

- comparing `e` STRUCTURALLY (recurse via conv) can only UNDER-accept (reject two
  casts whose proofs are inconvertible) -- sound;
- proof-irrelevance-SKIPPING `e` would OVER-accept iff the proof is proof-relevant
  (admit `Cast A B e t == Cast A B e' t` with e != e', collapsing distinct
  transports) -- the unsound direction.

So the probe is: does the arm genuinely recurse on EVERY field including the
proof, with `child_path` (not `path`/`&[]`)? Here it does (conjunctive `&&` over
all four fields, `e`/`respect` structural, comment "deliberately not skipped by
proof irrelevance because this type-agnostic path has no trusted field type").
Cannot over-accept. State the direction of any weakness: over-strict is
safe, over-accept is unsound.

## Method 2: an omitted arm's unreachability is a fact about the REDUCER on NEUTRALS

The J arm is omitted with the claim "a well-typed neutral J cannot present at
conv." Do not take the claim on the type rule alone -- verify it at the reducer,
and specifically on NEUTRAL operands:

1. Find the reducer meant to eliminate the head. `whnf_progress` calls
   `j_reduce`; if it returns `Some`, whnf continues on the reduct.
2. Confirm it fires on NEUTRAL operands, not just canonical constructors. The
   tell is what it keys on. `j_nonrefl` does `infer(eq)` and checks the inferred
   TYPE whnfs to `Eq` -- it never inspects whether `eq` is `refl`/canonical vs a
   neutral variable proof. So it fires for EVERY `eq : Eq`, neutral included, and
   returns a `Cast`. ⇒ a well-typed J always reduces; the only residual neutral
   J is ill-typed (infer fails / type not Eq) -> `_ => false` (fail-closed).
   A reducer that pattern-matched `eq` for a canonical constructor would leave a
   well-typed NEUTRAL-eq J stuck, and the omission would be a real completeness
   gap that should have been an arm.
3. Confirm the reduct's head IS covered by an existing/new arm. Here well-typed J
   reduces to a `Cast`, covered by the new Cast arm -- so J-equality is
   congruence-handled via its reduct. The omission is COMPLETE for well-typed
   input, not merely fail-closed.
4. Confirm the reduct preserves the needed DISCRIMINATION. The J reduct's
   `B = motive` applied at `(b, eq)` captures the proof `eq`, so a proof-relevant
   motive still forces inconvertible proofs to reject at the B field (sound), and
   a proof-constant motive makes the two J terms genuinely equal (complete). The
   reduction does not launder proof-relevance away.

For an OVER-accept gate specifically, an omitted arm can only UNDER-accept
(head-pair -> `_ => false`), so it is never itself an over-accept defect; but
the Steward/Architect will ask whether the omission is a soundness-relevant
COMPLETENESS gap, and steps 1-4 answer that at the reducer rather than by
assertion. Sibling of
[[two-arm-producer-needs-a-case-per-arm]]
and of the inc-1 clean-congruence approve.
