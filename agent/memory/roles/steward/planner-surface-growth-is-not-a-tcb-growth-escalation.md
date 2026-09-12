---
scope: roles/steward
---

# Growing private compiler/planner surface is not TCB growth — do not route it to the operator

The operator gate in `steward.md §3` is **TCB growth** — a new kernel rule,
primitive, axiom, trusted-base registrant, checked source-plan encoding, or
runtime carrier/frame/schema. Correctness-bearing **compiler-internal** surface
(elaborator, planner, lowering, control-proof records) is NOT that. The compiler
is the untrusted, kernel-backstopped layer: a wrong planner extension yields a
wrong native compile that the existing gate chain catches (Architect + QA +
Adversary + CI native==interp differential), never a soundness hole in the proof
kernel. So planner-surface growth is **Steward-fenceable**, not operator-gated.

MEASURED 2026-09-12 (ABI-S6 D5b, HS16). The Architect ruled a real capability
gap — a detached continuation's result had no retained source-return-context —
and proposed a bounded fix that grew private planner/control-proof surface
(`DETACHED_RESULT_FLOW_HAS_NO_RETAINED_SOURCE_RETURN_CONTEXT`, evt_1y4vcy2t35kb0).
The Architect was scrupulous: it adds NO kernel rule/primitive/axiom/trusted-base
registrant/runtime carrier, but it IS "compiler implementation whose correctness
matters to native semantics," so it declined to call it "no new mechanism." The
Steward had earlier set a **"planner-surface growth -> operator" gate**; the
Architect deferred to that gate and held the ring pending operator authorization.
The Steward escalated it (recommending AUTHORIZE). The operator's ruling, verbatim
substance: **"The planner surface does not require my approval, and most aspects
of that surface have not been approved by me."** The gate was mis-calibrated and
is retired; HS16 was reclassified FENCED and the ring resumed under the standard
gates with no operator sign-off.

This is the same class the operator already flagged once: HS15 on the same chain
was ruled OVER-ESCALATED (reclassified FENCED). Two escalations of the same axis
in one campaign is the tell that the gate, not the instance, is the defect.

**Why it happens:** "correctness-bearing" reads as "must be someone senior's
call," and the honest Architect statement that it is "not no new mechanism" reads
as "a new mechanism the operator must authorize." But *native-semantics
correctness* is exactly what the whole build fleet produces every turn under the
untrusted-compiler posture; if that needed operator sign-off, every elaborator
and lowering change would too. The operator owns the TRUST ROOT, not the
implementation that the trust root backstops.

**How to apply:** when a hard-stop's fix grows compiler/planner/elaborator
surface, classify it by the TCB test, not by whether it is "correctness-bearing"
or "a new mechanism." If it adds a kernel rule, primitive, axiom, trusted-base
registrant, checked source-plan encoding, or runtime carrier — operator. If it is
private compiler surface the kernel re-checks — FENCE it: Architect (required) +
domain QA + Adversary -> Steward M1-M4 -> lieutenant, CI native==interp. Do not
invent a "surface growth -> operator" gate; the operator has now said planner
surface is not theirs to approve. A genuinely new operator-owned fork (TCB
growth, a spec commitment, a scope fork) surfacing during the build is what comes
back — not the compiler-internals decision. Related:
[[a-fleet-derived-design-constraint-is-not-an-operator-boundary]] (same "whose
call is this" discipline on the constraint axis) and the fleet lesson that a
compiler-internal repair leaving an observe fence is not a §3 operator
escalation.
