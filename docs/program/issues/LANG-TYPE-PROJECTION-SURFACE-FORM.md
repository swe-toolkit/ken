---
id: LANG-TYPE-PROJECTION-SURFACE-FORM
title: "the surface prerequisite LANG-MEMBERSHIP-OPERATOR-SURFACE cannot be written without: Ken's surface type grammar has no projection form, so a parameter typed by a projection from an earlier parameter (`(d : Membership c) (q : d.Query)`) is unspellable in a .ken.md declaration even though the kernel admits the telescope -- add a projection form to RType and resolve it to the kernel's existing Term::Proj1/Proj2, keeping the elaborator-builtin escape hatch CLOSED because a builtin would falsify the ordinary-top-level-binding premise A-track's completion policy rests on"
status: draft
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-16 on the Architect's explicit routing consequence in evt_4tmt0n7era4w3 -- 'the only routing consequence is that LANG-MEMBERSHIP-OPERATOR-SURFACE gains a language prerequisite (a projection form in RType) that it does not currently carry. Worth a node before that build is cut, so it is discovered at framing rather than by an implementer who cannot write the signature.' Discovered while ruling SPEC-MEMBERSHIP-CLASS-CONTRACT's D0-1: the kernel CAN express the telescope (Term::Proj1/Proj2, class as right-nested Sigma per 33 §5.2) and the spec CAN state it, but the surface cannot spell it. Cut BEFORE the dependent build is framed, so the gap is a named dependency rather than an implementer's hard stop."
---

> # DRAFT — cut so the gap is DISCOVERED AT FRAMING, not by an implementer who
> # cannot write the signature. Not framed; do not start.
>
> **This node exists because of where the obstruction STOPPED.** The Architect
> ruled `SPEC-MEMBERSHIP-CLASS-CONTRACT`'s D0-1 by measuring which layer is
> actually missing (`evt_4tmt0n7era4w3`):
>
>     kernel    the telescope is expressible and checkable        AVAILABLE
>     spec      the contract can state it                         AVAILABLE
>     surface   `RType` cannot spell `d.Query`                    MISSING
>
> **Only the third line is a gap.** The spec contract is unblocked and is being
> authored now; this node is what stands between that contract and a catalog
> binding that realizes it.

# The measurement

Steward-verified at `origin/main` `24e9ce03990d4e7163b21a5f65d94dd69ba6860f`,
re-running the Architect's own probe rather than quoting it:

    resolve.rs:390   pub enum RType { RPi, RSigma, RArr, REffectArr, RUniv,
                                      RCon, RVarTy, RPatternAliasTy, RRefine,
                                      RApp, RTrunc }
                     ELEVEN variants, NO projection

    git grep -c 'RType::RProj' -- crates/        0 hits
    control: RExpr::RProj                        4 files -- classes.rs, elab.rs,
                                                 modules.rs, resolve.rs

**Projection exists, in the EXPRESSION category only.** The control matters:
a zero that comes back from an instrument with no live positive result is not
a measurement, and here the positive control fires in four files.

**The kernel side is already there** — `term.rs:301` `Term::Proj1`, `:303`
`Term::Proj2`, and a class is a right-nested Σ (`33 §5.2`). So this is a
surface-to-kernel resolution gap, not a new kernel capability.

# Why the telescope is CLOSED even though it cannot be spelled

**This is the distinction the whole node rests on, and it fooled its own author
for half an hour** (Architect, `evt_6y241yhbamnhf` then `evt_4tmt0n7era4w3`).
With `Query` as field 0:

    [ c : Type,  d : Membership c,  Proj1(d),  c ]

Every entry is a `Term` valid in the context of the entries before it, and
`Proj1(d)` is closed by the binding's own earlier parameter `d`. That is
ordinary dependency, exactly like `(a : Type) (x : a)`.

⇒ **UNSPELLABLE IS NOT UNCLOSABLE.** Do not re-derive this node as a kernel or
typing problem; it is a grammar problem with a kernel target that already
exists.

# Deliverables (indicative — this node is not framed)

- A projection form in `RType`, resolving to `Term::Proj1`/`Term::Proj2`.
- A parameter may be typed by a projection from an **earlier parameter** in the
  same telescope — the case the dependent node needs.
- Rejection with a located span where the projected object is not a record, and
  where the field does not exist.
- Whatever `33`/`34` surface-grammar text must say for the form to be normative
  rather than an undocumented elaborator affordance.

# THE ESCAPE HATCH THAT MUST STAY CLOSED

**Architect, `evt_4tmt0n7era4w3`, and it is the clause they said they cared
about most:**

> it must not be discharged by making `∈` an elaborator builtin: `33 §6.1`
> requires the standard meanings to be **ordinary top-level bindings**, and a
> builtin would break the precondition A-track's completion policy rests on
> (`39 §6.9`, and `SPEC-STANDARD-INFIX-BINDING §2f`).

⇒ **The tempting way out of a missing surface form is to special-case the
operator in the elaborator — and that would quietly falsify the premise the
whole A-track policy is built on, in a way NO TEST ON THE DEPENDENT NODE WOULD
CATCH.** A frame for this node must carry that as a criterion with a control,
not as advice.

# Not this node

- The `Membership` class, its views, or its laws — `SPEC-MEMBERSHIP-CLASS-CONTRACT`
  (spec) and [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] (build).
- `∈`'s elaboration or dispatch — [[LANG-MEMBERSHIP-OPERATOR-SURFACE]].
- Projection in expression position — it already exists (`RExpr::RProj`).
- Any kernel change. `Term::Proj1`/`Proj2` are the target, unchanged.

# Open, and stated as open rather than assumed

**Whether the instance registry and `resolve_instance_dictionary` handle a class
at a non-zero universe level is UNMEASURED** (Architect, `evt_4tmt0n7era4w3`).
`type_id: GlobalId` plus `Term::Const{level_args}` suggests yes; nobody has run
it. It does not gate this node's premise, and **no one should cite the D0-1
ruling as having closed it.** It surfaces here because
`Membership : Type ℓ → Type (suc ℓ)` is the first class that would exercise it.

# Related

- [[LANG-MEMBERSHIP-OPERATOR-SURFACE]] — the build this unblocks; gains this as
  a dependency.
- [[SPEC-STANDARD-INFIX-BINDING]] — A-track; its `§2f` and `39 §6.9` are what
  the builtin escape hatch would falsify.
- [[LANG-STANDARD-INFIX-CALL-COMPLETION]] — A1, the shared use-site resolver.
