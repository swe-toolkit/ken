---
id: LANG-INDEXED-RECURSIVE-IH-DISCHARGE
title: "Transport the mutual-recursion sibling-call result along the dependent-match refinement equality at the elaboration boundary, so the source branch reconciles a recursive-group call's concrete indexed result with the refined motive index -- route (c) genuine J/cast transport over a PROPOSITIONAL equality, NOT the reflexive same-owner discharge; the elaborator prerequisite V3-FO-CHECKER-SOUNDNESS D3 is blocked on, measure-first between exposing the equality for the proof (c-proof) and elaborator auto-transport (c-elab)"
status: ready
owner: language
size: M
gate: none
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-21. RE-FRAMED on the Architect's RE-RULE evt_349bjakvjj9yp (hard-stop 4 of the D3 chain, grounded via a full locus map of check_match_dependent). The node's first framing (on ruling evt_n4q1da1qp68) targeted the same-owner reflexive-discharge case; the language-implementer built that mechanism (candidate b89b4a2cf) and the Architect APPROVED it as sound (soundness line + application-spine boundary correct), but the AC-7 real-family control REDDED honestly: the held-D3 consumer (a84d7100561a89f3ded6300fbb4fb4b45b947888) stays kernel-rejected because its recursive call crosses a MUTUAL-RECURSION helper edge (callee != owner_label) and the leading equality is a dependent-match RESULT REFINEMENT, not bare same-owner-child evidence -- so the reflexive helper is by-design dead on this path. Architect's corrected locus + soundness frame below: the equality is PROPOSITIONAL, the fix is a genuine J/cast transport (route c). The reflexive helper is RETAINED as a sound building block. Elaborator-scoped + kernel-backstopped (the kernel re-checks and rejects any ill-typed transport), so NOT a TCB change and NOT operator-gated. Scope routed to the Steward per COORDINATION section 2. Estimated tier T1 (soundness-adjacent dependent-elaboration transport; reasoning-dense, must hold the propositional-vs-definitional line and be measured, not guessed)."
---

# WHY THIS NODE EXISTS (re-framed -- hard-stop 4)

`V3-FO-CHECKER-SOUNDNESS` D3 (the propositional `checker_soundness` proof)
hard-stopped a fourth time. The node's FIRST framing targeted the wrong seam of
the same defect: it ruled the fix was the same-owner reflexive-IH discharge at
`check_match_dependent`'s structural self-call shortcut. The language-implementer
built exactly that (candidate `b89b4a2cf`, single file `elab.rs`), the Architect
reviewed it and confirmed the MECHANISM is sound and faithful -- the discharge
gates on the kernel's definitional `convert` of the equality endpoints, retains
the `Pi` on non-reflexive premises, and never manufactures `Refl`. But the
**AC-7 real-family control redded**: the exact held-D3 source `a84d71005`, run
through `b89b4a2cf`, stays kernel-rejected with the identical mismatch, and the
required unapplied-`Pi` mutation stayed GREEN -- proving the reflexive helper is
never reached on this path. The implementer reported the exact failure rather
than papering it with a proxy control (the right call), and the Architect
re-ruled the locus from a full map of `check_match_dependent` rather than a fifth
guess. **The reflexive helper is sound and is kept; it is simply not sufficient,
because the held consumer takes a different presentation path.**

# CORRECTED LOCUS -- Architect re-rule `evt_349bjakvjj9yp` (definitive, grounded)

The held-D3 mismatch is a **result-refinement gap on the MUTUAL-RECURSION edge**,
not the direct-self-call reflexive-discharge case:

- **The recursive call crosses a sibling edge.** `fok_prop_imp_children_sound ->
  fok_prop_check_tree_sound_by_cert` is a call to a *sibling* in the recursive
  group. `ElabCtx` carries only `owner_label: String` (`elab.rs:314`) +
  `lift_bindings` -- there is NO mutual-group roster. The owner-label self-call
  fast path (~`elab.rs:3526-3538`, the `return Ok(evidence)`) requires
  `name == owner_label` plus a bare `RVar` child; a sibling call fails that guard
  and falls through to the GENERIC application arm (~`elab.rs:3539-3545`): an
  ordinary global-constant `Pi`-application whose result sits at the CONCRETE
  child sequent, with no access to the match refinement / motive / roster.
- **The leading `Equal FokSequent @8 @0` is the dependent-match RESULT
  REFINEMENT** -- the motive/IH equality premise minted by `motive_index_premises`
  (`elab.rs:2635`) / `method_index_premises` at the IH slot (`elab.rs:2408-2410`),
  wrapped by `wrap_premise_pis` (`elab.rs:3220`), around the indexed
  `FokDerivation` result. The three existing result-side transports --
  `install_index_refinements` (`2953`), the sibling-convoy (`3029-3084`),
  `refine_branch_goal` (`2880`) -> `check_match_dependent_refined_fallback`
  (`1943`) -- ALL key on the branch's own FIELDS, an OUTER binder, or the GOAL.
  **None transports a CALL's result.** So nothing reconciles
  `FokDerivation (concrete child sequent)` (found) with the refined
  `FokDerivation @7` (expected).

# THE SOUNDNESS FRAME -- route (c), a GENUINE transport over a PROPOSITIONAL equality

> ## THE ONE SOUNDNESS-CRITICAL LINE. This equality is PROPOSITIONAL, so the fix
> is a REAL transport -- never Refl.
>
> The index equality `@7 = child_sequent` is **propositional, not definitionally
> reflexive.** `@7` is the abstract motive/refinement index; the sibling call
> returns at the child's *concrete* sequent; the two are bridged only by
> propositional evidence (the IH's own equality premise / the sub-derivation's
> index). The reconciliation is therefore a **genuine J/Cast transport along that
> propositional equality -- route (c), NOT a reflexive discharge.** This is why
> the reflexive helper (`b89b4a2cf`) is dead here: its endpoints do not convert,
> so it correctly declines, AC-7 stays red, and its unapplied-`Pi` mutation stays
> green.
>
> **NEVER erase the equality `Pi`, NEVER assume `Refl` for the non-convertible
> endpoints, NEVER use proof irrelevance to identify the function type with its
> result.** The kernel backstops the emitted transport -- it re-checks and
> rejects any ill-typed `J`/cast -- so this is a completeness/correctness change,
> elaborator-scoped, NOT a TCB change. But a wrong transport the kernel rejects
> reintroduces the hard-stop; a wrong transport the kernel accepts is the unsound
> direction, which AC-SOUND below exists to catch.

This is squarely research's Agda with-abstraction case (advisory
evt_3jdqjhds50z35): "a later abstraction can change the type of an
already-computed recursive call; the repair is to abstract the recursive result
before the generalization so its type is preserved -- otherwise an explicit
equality/transport." **No new research pull** (this is hard-stop 4; the existing
advisory already covers route (c), and the §1a research re-trigger is the 6th
hard-stop). The work is applying the covered route at the correct locus.

# THE FIX -- route (c) transport, with a MEASURE-FIRST sub-fork

Two seams could carry the fix: **A** = the IH-slot / branch-body boundary
(`elab.rs:2302` + `2389-2453`); **B** = the recursive-call site, which would need
a mutual-group roster surfaced into `ElabCtx`. Prefer the MORE BOUNDED path and
**measure before committing** (the entry-3 miss is the lesson -- do not guess the
mechanism):

- **(c-proof) -- the bounded path.** The elaborator EXPOSES the refinement
  equality as a source-nameable hypothesis; the D3 proof applies the explicit
  J/cast transport of the child result itself. This avoids adding a mutual-group
  roster and auto-transport to the delicate dependent-match core -- smaller,
  auditable, conservative (`docs/PRINCIPLES.md`: small auditable TCB,
  subsume-don't-proliferate). The elaborator change, if any, is only to cleanly
  expose/name the equality.
- **(c-elab) -- the fallback.** The elaborator gains mutual-group awareness (a
  roster in `ElabCtx`) and AUTO-transports the recursive-group call's result
  along the refinement equality (Seam A/B). Bigger; matches mature-system
  ergonomics. Take it only if (c-proof) is infeasible.

# DELIVERABLES

**`D0` -- the deciding measurement (measure-first; picks the sub-fork, gives QA
its control).** Determine: **can the source branch body currently NAME and APPLY
the in-scope refinement equality** (the IH equality premise / the
`INDEX_REFINEMENT_SENTINEL` binders) **to transport the sibling call's result?**

- If **YES** -> **(c-proof)** is the bounded fix. The elaborator change, if any,
  is only to cleanly expose/name the equality for the proof to apply; the D3
  proof carries the explicit transport.
- If **NO** -> the elaborator must at minimum EXPOSE the equality (a source-usable
  hypothesis -- smaller than full auto-transport). **(c-elab)** auto-transport is
  the fallback only if exposing-for-proof-use is itself infeasible.

Record the measurement and the chosen sub-fork before authoring `D1`.

**`D1` -- the route-(c) transport at the corrected locus.** Per D0's sub-fork:
expose (and, for c-elab, auto-transport) the dependent-match refinement equality
so a mutual-recursion sibling call's concrete indexed result reconciles with the
refined motive index, via explicit J/cast over the propositional equality. The
`b89b4a2cf` reflexive helper is RETAINED unchanged as a sound building block for
the definitionally-reflexive same-owner self-call case. If (c-elab) is chosen,
the `ElabCtx` mutual-group-roster addition is scoped explicitly here. No kernel,
checker, reference, verdict, proposition, trust, or D3-proposition change.

# ACCEPTANCE CRITERIA

**`AC-0` (the D0 sub-fork discriminator).** The deciding measurement above is
recorded and names the chosen sub-fork (c-proof / c-elab), with the evidence
(whether the branch body can name+apply the in-scope refinement equality).

**`AC-REAL` (the real-family control the AC-7 gap demanded).** A FokDerivation /
mutual-recursion consumer control -- the sibling-edge result-refinement shape,
NOT a synthetic Nat -- that is RED without the fix and GREEN with it, and whose
discriminating mutation (drop the transport / leave the refinement equality
unapplied) goes RED. The implementer correctly refused to accept a proxy that
stayed green under its own mutation; this control must genuinely reach the
transport.

**`AC-7` (the decisive buildability gate -- the held-D3 consumer).** The exact
held checkpoint `a84d71005`, rebased onto this node, ELABORATES and
KERNEL-CHECKS through `D1`: the `FokDerivImpRight` mutual-recursion premise
self-call (`owner child (FokMkSequent ...) frag accepted`, crossing
`fok_prop_imp_children_sound -> fok_prop_check_tree_sound_by_cert`) type-checks,
and the no-`ForallRight` theorem goes through. This is the control that matters
most (Architect): synthetic green must NOT stand in for real-family
buildability. It gates the close.

**`AC-SOUND` (soundness negative).** A mutation substituting `Refl` / erasing the
`Pi` for the non-convertible propositional endpoints is REJECTED by the kernel.
The transport is applied only over the actual propositional evidence; the
definitional-vs-propositional line holds.

**`AC-REFLEX` (the retained helper stays sound).** The `b89b4a2cf` reflexive
same-owner discharge is unchanged and its existing controls (one reflexive
premise; multiple premises; full spine ordering; direct vs nested-All) stay
green. This node ADDS the transport path; it does not weaken the reflexive path.

**`AC-8` (no-regression).** CI whole-suite green (`COORDINATION section 12`);
targeted `ken-elaborator` / `ken-kernel` validation locally, never `--workspace`.

# BANNED SCOPE

- **Assuming `Refl`, erasing the equality `Pi`, or proof-irrelevance-identifying
  the function type with its result** for the propositional endpoints. The
  unsound direction; AC-SOUND forbids it. The equality here is propositional --
  it needs a real transport, not a discharge.
- **Removing or weakening the `b89b4a2cf` reflexive helper.** It correctly handles
  the definitionally-reflexive same-owner self-call case and is a retained
  building block (AC-REFLEX).
- **Any kernel change.** The kernel's IH contract and reductions are correct and
  backstop the emitted transport. A kernel conversion-completeness gap is a
  SEPARATE node routed through the Steward, not in scope here.
- **Any checker / reference / verdict / trust / proposition / semantic / D3-proof
  change.** Those belong to D3's proof scope. If (c-proof) is chosen, the D3-side
  transport application is authored on the D3 node after this lands, not here.
- **A point-fix for the single `FokDerivImpRight` edge.** The transport must be
  structural over the mutual-recursion result-refinement presentation, not a
  per-call-site patch.

# SEQUENCING

Prerequisite AHEAD of `V3-FO-CHECKER-SOUNDNESS` D3. `blocks:
[V3-FO-CHECKER-SOUNDNESS]`; D3 stays HELD (language-implementer at
`a84d7100561a89f3ded6300fbb4fb4b45b947888`) until this lands, then rebases onto
it and authors the proof against a transport-aware IH (applying the explicit
transport itself if the sub-fork is c-proof). `depends_on` empty -- grounds on
current `main`. `gate: none` -- elaborator-scoped, kernel-backstopped, not TCB,
not operator-gated. **Owner: language ring.** Author = `language-implementer`;
independent design/soundness review = Architect (author is not reviewer). First
move on pickup is the **D0 deciding measurement**, NOT further implementation.
Tier **T1** -- a dependent-elaboration transport that must hold the
propositional-vs-definitional line and be measured, not guessed.

**Bookkeeping (Architect, evt_349bjakvjj9yp).** Hard-stop 4 of the D3 chain.
Entry-4 inventory of `V3-FO-CHECKER-SOUNDNESS`'s section-1b area: entry 2 (kernel
`iota_reduct` unused-IH, the reduction face) LANDED as
`KERNEL-RECURSOR-UNUSED-IH-REDUCTION`; entry 3's FIRST locus prediction (same-owner
reflexive discharge) was sound-but-off-path -- the real locus is this
mutual-recursion result-refinement transport. `§1a`'s next research re-trigger is
the 6th hard-stop; this is the 4th, and the existing route-(c) advisory covers
it, so no new research pull.
