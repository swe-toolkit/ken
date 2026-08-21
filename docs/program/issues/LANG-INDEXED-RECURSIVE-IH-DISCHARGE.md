---
id: LANG-INDEXED-RECURSIVE-IH-DISCHARGE
title: "Transport the mutual-recursion sibling-call result along the dependent-match refinement equality at the elaboration boundary, so the source branch reconciles a recursive-group call's concrete indexed result with the refined motive index -- route (c) genuine J/cast transport over a PROPOSITIONAL equality via c-elab roster-aware auto-transport (the c-proof/c-elab fork RESOLVED to c-elab: D0 measured no lawful existing source carrier), surfacing the recursive-group roster into ElabCtx and inserting the transport at the equality-holding seam, NOT the reflexive same-owner discharge and NOT a new source-language carrier; the elaborator prerequisite V3-FO-CHECKER-SOUNDNESS D3 is blocked on"
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

# THE FIX -- route (c), c-elab (roster-aware auto-transport). SUB-FORK RESOLVED.

The c-proof/c-elab sub-fork is CLOSED: the D0/AC-0 measurement came back
definitively **NO** (below), and the Architect ruled (evt_2md0cx2817yhn,
hard-stop 5) that **the absence of a lawful existing surface carrier SELECTS
c-elab.** Do NOT introduce a new source-language carrier.

Why c-elab, not a new source carrier (Architect design authority): a new
selector/modifier exposing the refinement premise would LEAK the internal
Fording/index-refinement encoding into the SURFACE language (reflect-don't-extend
and honesty-about-the-boundary run backwards); it PROLIFERATES surface where the
need is SUBSUMED into elaboration; and it would REVERSE a deliberate existing
boundary -- `selected_recursive_result` refusing to expose a hidden method binder
(the measured `StructuralResultOutOfScope`) is that boundary being ENFORCED, not a
bug to tear down for one proof. Mature systems agree (research evt_3jdqjhds50z35):
Lean pre-specializes the IH, Agda refines the branch by unification -- neither
makes the source author thread refinement equalities by hand. And the authority
boundary is cleaner: a new source carrier is a surface-contract decision that
routes to Spec (section 4); c-elab is a purely internal elaboration mechanism --
component-design, no Spec surface ruling.

**Mechanism boundary for D1 (Architect):**

- **Surface the recursive-GROUP roster into `ElabCtx`** -- today it carries only
  `owner_label: String` (`elab.rs:314`), so a call to a mutual SIBLING in the
  recursive group is not recognizable, only the single owner. The roster makes
  sibling-group calls recognizable.
- **Insert the transport at the seam that HOLDS the equality.** At the
  branch-body / IH-slot boundary (Seam A, `elab.rs:2302` + `2389-2453`, which
  already has the motive + refinement equality in scope), when a recursive-group
  call's result sits at a CONCRETE index while the slot expects the REFINED index,
  insert a GENUINE J/cast transport along the in-scope propositional refinement
  equality (the IH's own equality premise / the `INDEX_REFINEMENT_SENTINEL` binder
  bridging the child's concrete index and the refined index). Do NOT thread it to
  the bare call-site arm (Seam B) if Seam A already holds the evidence -- prefer
  the seam with the evidence, and **pin the exact seam by measurement during D1,
  do not hard-code it blind** (the entry-3 miss is the lesson).

# DELIVERABLES

**`D0` -- the deciding measurement. RESOLVED = NO (recorded, selects c-elab).**
The question was: can the source branch body currently NAME and APPLY the
in-scope refinement equality (the IH equality premise / the
`INDEX_REFINEMENT_SENTINEL` binders) to transport the sibling call's result? The
implementer measured it (evt_1jh2b72g1w6t3 / evt_4ynhyk6xq9vtx): **NO.** The two
existing carriers cannot express it -- `match ... eqn: h` binds scrutinee
equality on the flat-nullary no-refinement path, not the method/result-index
premise; and the existing `recursive result for child` selector REJECTS the exact
held child with `StructuralResultOutOfScope` (selector span 53154..53182, child
53154 area), because `selected_recursive_result` deliberately declines to expose a
hidden method binder. So no lawful existing surface carrier exists, and (per the
FIX section) the fix is c-elab, NOT a new source carrier. This D0 is DONE; D1 may
proceed.

**`D1` -- c-elab roster-aware auto-transport at the corrected locus.** Add the
recursive-GROUP roster to `ElabCtx` (explicit deliverable) so mutual-sibling calls
are recognizable, and at the equality-holding seam (prefer Seam A, pinned by
measurement in D1) insert the GENUINE J/cast transport of the sibling-call's
concrete indexed result along the propositional refinement equality, so it
reconciles with the refined motive index. The `b89b4a2cf` reflexive helper is
RETAINED unchanged as the sound path for the definitionally-reflexive same-owner
self-call case. **The held D3 source stays UNCHANGED -- no new source carrier, no
proof rework; that the source needs no new surface is the whole point of c-elab.**
No kernel, checker, reference, verdict, proposition, trust, surface-syntax, Spec,
or D3-proposition change.

# ACCEPTANCE CRITERIA

**`AC-0` (D0 recorded -- SATISFIED).** The deciding measurement is recorded as
NO (no lawful existing surface carrier; the `StructuralResultOutOfScope` probe is
the decisive evidence), selecting c-elab per the Architect's ruling. D1 delivers
the `ElabCtx` recursive-group roster as an explicit, reviewable addition.

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
- **Any new source-language carrier** (a new selector / modifier / `eqn:`-style
  binder exposing the refinement premise to the surface). Ruled out by the
  Architect: it would leak the internal index-refinement encoding into the
  surface and is a Spec surface-contract decision, out of this node's authority.
  The transport is internal (c-elab); the held source is untouched.
- **Any checker / reference / verdict / trust / proposition / semantic / D3-proof
  change.** Those belong to D3's proof scope; the held source stays unchanged
  (c-elab needs no source-side transport).
- **A point-fix for the single `FokDerivImpRight` edge.** The transport must be
  structural over the mutual-recursion result-refinement presentation, not a
  per-call-site patch.

# SEQUENCING

Prerequisite AHEAD of `V3-FO-CHECKER-SOUNDNESS` D3. `blocks:
[V3-FO-CHECKER-SOUNDNESS]`; D3 stays HELD (language-implementer at
`a84d7100561a89f3ded6300fbb4fb4b45b947888`) until this lands, then rebases onto
it and authors the proof against a transport-aware IH (the elaborator carries the
transport under c-elab; the source needs no new carrier). `depends_on` empty --
grounds on current `main`. `gate: none` -- elaborator-scoped, kernel-backstopped,
not TCB,
not operator-gated. **Owner: language ring.** Author = `language-implementer`;
independent design/soundness review = Architect (author is not reviewer). D0 is
recorded (NO -> c-elab); first move is **D1 authoring** -- the `ElabCtx` roster +
the Seam-A transport, pinning the exact seam by measurement. Tier **T1** -- a
dependent-elaboration transport that must hold the propositional-vs-definitional
line and be measured, not guessed.

**Bookkeeping (Architect, evt_349bjakvjj9yp re-rule + evt_2md0cx2817yhn c-elab
finalization).** Hard-stops 4 and 5 of the D3 chain. Entry-4 inventory of
`V3-FO-CHECKER-SOUNDNESS`'s section-1b area: entry 2 (kernel `iota_reduct`
unused-IH, the reduction face) LANDED as `KERNEL-RECURSOR-UNUSED-IH-REDUCTION`;
entry 3's FIRST locus prediction (same-owner reflexive discharge) was
sound-but-off-path -- the real locus is this mutual-recursion result-refinement
transport, and hard-stop 5 resolved its c-proof/c-elab sub-fork to c-elab (D0
measured no lawful existing source carrier). `§1a`'s next research re-trigger is
the 6th hard-stop; hard-stops 4 and 5 are covered by the existing route-(c)
advisory, so no new research pull.
