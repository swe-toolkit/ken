---
id: LANG-INDEXED-RECURSIVE-IH-DISCHARGE
title: "Discharge the generated index-equality evidence at the dependent-match elaboration boundary so the source branch sees the recursive IH already specialized at the recursive value and its constructor-determined indices -- the elaborator prerequisite V3-FO-CHECKER-SOUNDNESS D3 is blocked on, structural over every rule/constructor of the family, not a point-fix for FokDerivImpRight"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-21, on the Architect's D3 ruling evt_n4q1da1qp68 (with research advisory evt_3jdqjhds50z35 in hand). V3-FO-CHECKER-SOUNDNESS D3 hit its 3rd hard-stop in the abstract-certificate elimination chain (language-implementer, thr_13q5, holding at clean non-candidate a84d7100561a89f3ded6300fbb4fb4b45b947888). The Architect ruled the defect is elaborator-side, NOT kernel and NOT a missing conversion law: Ken's kernel recursor-IH contract is already correct and its whnf/j_reduce already fire; the elaborator's dependent-match layer wraps the direct IH with generated index-equality premises and its structural self-call shortcut fails to discharge them for constructed-argument self-calls, leaking an undischarged leading equality Pi into the source branch. Scope routed to the Steward per COORDINATION section 2; this is the runtime-chain prerequisite-node pattern applied to the language proof chain. Elaborator-scoped + kernel-backstopped (the kernel re-checks and rejects any ill-typed refl/transport), so NOT a TCB change. Estimated tier T1 (soundness-adjacent dependent-elaboration completeness repair; reasoning-dense, must be structural)."
---

# WHY THIS NODE EXISTS

`V3-FO-CHECKER-SOUNDNESS` D3 (the propositional `checker_soundness` proof)
hard-stopped a third time, and the language-implementer correctly declined a
fourth proof-architecture guess. Every proof-side transport it tried -- direct
`Cons child rest` / `rest = Nil` site, the recursive soundness function passed
unapplied to a nonrecursive builder, an isolated exact-index `FokDerivImpRight`
builder, a named recursor -- reached the **identical** mismatch, because a proof
helper cannot erase an equality Pi that the elaborator generates internally and
leaves undischarged. The Architect ruled the locus (evt_n4q1da1qp68) grounded in
Ken source, and research supplied the prior art (evt_3jdqjhds50z35): the fix is
in the elaborator's presentation of the recursive IH, not in the kernel and not
in a new conversion law.

# LOCUS -- Architect ruling `evt_n4q1da1qp68`

**The kernel is correct; do not touch it.** Ken's normative recursor-IH contract
already matches Lean: `spec/10-kernel/14-inductive.md` section 3 gives the direct
IH as `M idxs recursive_value` and section 3.2 gives the nested-`All` leaf as
`lambda x. Lift_D(M, A, x)` -- no equality telescope is part of the public IH
contract. The kernel producer emits exactly that (`crates/ken-kernel/src/inductive.rs`,
`method_type` -> `structured_lift_type`, direct arm `apply_motive(motive, recursive
indices, recursive value)`). Ken's `whnf` already beta-reduces head lambda
applications (`conv.rs:49-72`) and `obs::j_reduce` already reduces `J P d refl`
to `d` (`obs.rs:645-661`). The reduction face of this area landed in
`KERNEL-RECURSOR-UNUSED-IH-REDUCTION`; this node is the **presentation** face.

**The defect is elaborator-side.** In `check_match_dependent`:

- IH-slot construction wraps the direct IH with generated index-equality
  premises: `wrap_premise_pis(ih_body, method_index_premises(...))`
  (`crates/ken-elaborator/src/elab.rs` ~2360-2432, the premise wrap ~2405-2410),
  so the exposed IH type is `Pi (e : idx = target). M idxs field`.
- the structural self-call shortcut (`elab.rs:3522-3575`) discharges the paired
  evidence ONLY for the bare `owner child_var` form -- its guard is
  `if let (RExpr::RCon(name, _), RExpr::RVar(index, _, _)) = (&**f, &**a)`.
  `FokDerivImpRight`'s recursive premise self-call applies a **constructed**
  sequent plus extra arguments (`owner child (FokMkSequent ...) frag accepted`),
  so the shortcut does not fire; the generic application arm exposes the
  equality-premise IH to the source branch WITHOUT discharging the equality. The
  branch's ordinary arguments are then consumed at the equality-premise position,
  and the kernel correctly rejects (`Pi e : i = j. B` is not `B`; `refl` is not
  manufactured).

That producer split is exactly why every proof-side rearrangement reached the
same mismatch.

# THE FIX -- direction (b), structural

Expose the recursive IH **already specialized** at the recursive value and its
constructor-determined indices: discharge the generated index-equality evidence
at the elaboration boundary -- applied exactly once, BEFORE any ordinary
recursive-function arguments -- so the source branch sees the direct
`M idxs field`, matching Ken's kernel contract and Lean's `C (indices of u) u`.

**This must be a STRUCTURAL closure over the elaborator's indexed-recursive-IH
presentation** -- every rule and constructor of the family, direct recursive
fields AND nested-`All` leaves -- NOT a point-fix for `FokDerivImpRight`. A
point-fix makes the chain reappear per rule.

> ## THE ONE SOUNDNESS-CRITICAL LINE. Discharge is valid ONLY when the equality is DEFINITIONALLY reflexive.
>
> Constructor-determined indices make the index equality definitionally reflexive
> -- `refl` is well-typed and beta/J yields the direct IH, so discharge is sound.
> For a genuinely propositional or neutral index equality, the elaborator MUST
> emit an explicit `J`/cast transport (route c); the residual transport is real.
> **NEVER erase the equality Pi, NEVER assume `Refl` for non-convertible
> endpoints, NEVER use proof irrelevance to identify a function type with its
> result.** Soundness is kernel-backstopped -- the kernel re-checks and rejects
> any ill-typed `refl`/transport -- so this is a completeness/correctness change,
> elaborator-scoped, not a TCB change. But a wrong discharge that the kernel then
> rejects reintroduces the hard-stop; a wrong discharge the kernel accepts is the
> unsound direction and is what AC-6 exists to catch.

# DELIVERABLES

**`D0` -- the discriminating measurement (measure-first; pins the locus, gives QA
its control).** Normalize the exact expected type at the mismatch. If it whnf's
to a literal `(lambda e. B) refl` or `J P d refl` with definitionally-equal
endpoints, the reduction already exists and the gap is a narrow KERNEL
conversion-completeness defect -- STOP and escalate to the kernel ring via the
Steward (this node's premise is then wrong). If it shows an unapplied
`Pi e : i = j. B`, a `refl` at non-convertible endpoints, or the equality
consumed at the wrong application position, the kernel is correct and the fix is
elaborator-side per below. Source evidence favors the latter; verify by
normalizing, do not assume.

**`D1` -- the structural elaborator fix.** In `check_match_dependent`'s IH-slot
construction (`elab.rs:2360-2432`) and the self-call discharge
(`elab.rs:3522-3575`): discharge the generated index-equality evidence at the
elaboration boundary for every rule/constructor of the family, direct fields and
nested-`All` leaves alike, with `J`/cast transport preserved for non-reflexive
equality. No kernel, checker, reference, verdict, proposition, or spec change.

# ACCEPTANCE CRITERIA -- research's six controls (evt_3jdqjhds50z35) are binding

**`AC-0` (the D0 discriminator).** The normalization measurement above is recorded
and shows the elaborator-side shape (unapplied `Pi e:i=j.B` / wrong-position
consumption), confirming the elaborator locus. If it shows the kernel shape,
this node stops and re-routes.

**`AC-1` (orientation).** Equality orientation and target-index order match the
method telescope.

**`AC-2` (uniform contract).** Nested-`All` leaves and direct recursive fields
expose the SAME specialized-motive contract.

**`AC-3` (applied once, in order).** Generated evidence is applied exactly once,
BEFORE ordinary recursive-function arguments.

**`AC-4` (transport preserved).** Neutral / non-reflexive equality remains an
explicit `J`/cast transport, never made definitional.

**`AC-5` (discriminating negative).** A mutation that leaves the equality Pi
unapplied reproduces THIS mismatch -- the control that proves the discharge is
what fixed it.

**`AC-6` (soundness negative).** A mutation that substitutes `Refl` for unequal
endpoints is REJECTED. This is the control that proves the discharge did not take
the unsound direction.

**`AC-7` (the D3 unblock).** With the discharge in place, the
`FokDerivImpRight` recursive-premise self-call type-checks and the language ring's
held D3 checkpoint `a84d71005` rebases onto this node and resumes with a
discharge-aware IH -- no kernel/checker/proposition change on the D3 side.

**`AC-8` (no-regression).** CI whole-suite green (COORDINATION section 12);
targeted `ken-elaborator` / `ken-kernel` validation locally, never `--workspace`.

# BANNED SCOPE

- **Any kernel change** -- the kernel IH contract and its reductions are correct;
  this is elaborator-side. If D0 shows a kernel conversion-completeness gap,
  that is a SEPARATE kernel node routed through the Steward, not in scope here.
- **Erasing the equality Pi, assuming `Refl` for non-convertible endpoints, or
  proof-irrelevance-identifying a function type with its result.** The unsound
  direction; AC-6 forbids it.
- **A point-fix for `FokDerivImpRight`.** The fix is structural over the whole
  family; a per-rule patch reintroduces the chain.
- **Any checker / reference / verdict / trust / proposition / semantic change**
  -- those belong to D3's proof scope, not here.

# SEQUENCING

Prerequisite AHEAD of `V3-FO-CHECKER-SOUNDNESS` D3. `blocks: [V3-FO-CHECKER-SOUNDNESS]`;
D3 stays HELD (language-implementer at `a84d71005`) until this lands, then rebases
onto it and authors the proof against a discharge-aware IH. `depends_on` empty --
the reduction prerequisite (`KERNEL-RECURSOR-UNUSED-IH-REDUCTION`) already landed
(main `89dafc8c5`); this presentation-face repair grounds on current main.
`gate: none` -- elaborator-scoped, kernel-backstopped, not TCB. **Owner: language
ring.** Author = `language-implementer`; independent review = Architect (the
author is not the reviewer). Tier **T1** -- a dependent-elaboration completeness
repair that must be structural and must hold the one soundness-critical line
(discharge only on definitional reflexivity, else transport).

**Bookkeeping (Architect, evt_n4q1da1qp68).** `V3-FO-CHECKER-SOUNDNESS`'s
section-1b area splits into two DIFFERENT layers with DIFFERENT repairs, not one
closure: the ML iota-reduction face landed as `KERNEL-RECURSOR-UNUSED-IH-REDUCTION`
(entry 2), and this elaborator IH-presentation face is entry 3. Within entry 3
the fix is a structural closure over the elaborator layer.
