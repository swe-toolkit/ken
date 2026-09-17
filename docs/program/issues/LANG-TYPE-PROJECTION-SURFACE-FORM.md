---
id: LANG-TYPE-PROJECTION-SURFACE-FORM
title: "the surface prerequisite LANG-MEMBERSHIP-OPERATOR-SURFACE cannot be written without: Ken's surface type grammar has no projection form, so a parameter typed by a projection from an earlier parameter (`(d : Membership c) (q : d.Query)`) is unspellable in a .ken.md declaration even though the kernel admits the telescope -- add a projection form to RType and resolve it to the kernel's existing Term::Proj1/Proj2, keeping the elaborator-builtin escape hatch CLOSED because a builtin would falsify the ordinary-top-level-binding premise A-track's completion policy rests on"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: [LANG-MEMBERSHIP-OPERATOR-SURFACE]
github: null
tier: T1
origin: "Steward cut 2026-09-16 on the Architect's explicit routing consequence in evt_4tmt0n7era4w3 -- 'the only routing consequence is that LANG-MEMBERSHIP-OPERATOR-SURFACE gains a language prerequisite (a projection form in RType) that it does not currently carry. Worth a node before that build is cut, so it is discovered at framing rather than by an implementer who cannot write the signature.' Discovered while ruling SPEC-MEMBERSHIP-CLASS-CONTRACT's D0-1: the kernel CAN express the telescope (Term::Proj1/Proj2, class as right-nested Sigma per 33 §5.2) and the spec CAN state it, but the surface cannot spell it. Cut BEFORE the dependent build is framed, so the gap is a named dependency rather than an implementer's hard stop."
---

> ## MERGED 2026-09-17 at `294cb5e28843dff8e2edeae9945e1cdc20b0318f`
>
> Blob-verified subsumed: `diff(merge-base, 294cb5e28)` against current
> `origin/main` is `LIVE=0` across all 9 touched files under
> `crates/ken-elaborator/`.

> # READY 2026-09-17 (Steward). Frame:
> # `docs/program/wp/LANG-TYPE-PROJECTION-SURFACE-FORM.md`.
>
> **Released as the language lane's next work.** `depends_on: []`, verified at
> `origin/main` `ac4b5957120c3da8b63409d161f4290cb72fcc45` — nothing gates it,
> so it can lead.
>
> **Cut AFTER `LANG-BARE-OPERATOR-ATOM-REJECTION` (`ca4ef3472`) lands.** That
> candidate is in the publisher and touches `parser.rs`, which this node also
> touches. This is a contention instruction, not a dependency: cut from a
> `main` that already contains it, per frame §7's base pin.
>
> **The frame corrects this node's scope in one respect: the gap is in TWO
> surface layers, not one.** `Type` (ast.rs, ten variants) needs a variant as
> well as `RType` (resolve.rs, eleven), plus the parser production that builds
> it. Re-measured at `ac4b59571`. The frame also carries the `TTrunc`/`RTrunc`
> path as the worked precedent — it is this exact three-layer shape, already
> landed and reviewed.
>
> The original cut note, retained:
>
> **This node was cut so the gap is DISCOVERED AT FRAMING, not by an
> implementer who cannot write the signature.**
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

# The consumers: THREE bindings, ONE production (added 2026-09-16, Steward)

`SPEC-MEMBERSHIP-CLASS-CONTRACT` landed at `origin/main` `a63eebe3e` (candidate
`9b9586dc0`, 6/6 blobs verified), and it settles both the count and the shape of
what this node has to serve. Measured at that ref, not taken from the ruling:

| binding | where the projection sits | spec |
|---|---|---|
| `membership_member_at (q : d.Query) … : Bool` | parameter telescope | `58b §2` |
| `member_holds (q : d.Query) … : Ω` | parameter telescope | `58b §2` |
| `same_members … : Ω`, body `(q : d.Query) -> …` | **Π domain in the body** | `58b §4` |

**This node's title names only the first case.** That was written before the
contract landed and understates the consumer set: `58b:148-150` says **three**
projection-typed bindings, and a fix that serves `membership_member_at` alone
leaves two of three unwritable.

**`same_members` sits in a different place, and the question of whether that
makes it a second grammar requirement is CLOSED — it does not.** Its signature
carries no projection at all; the `d.Query` is the domain of a `Π` in an
expression-position definition body. Measured at `a63eebe3e`:

    RType::RPi (String, Box<RType>, Box<RType>, Span)    domain is RType
    RExpr::RPi (String, Box<RType>, Box<RExpr>, Span)    domain is RType

**Both Π forms annotate their domain with an `RType`**, so a single projection
variant on `RType` reaches all three. The scope stated elsewhere in this node —
one variant, resolved to `Term::Proj1`/`Proj2` — is correct as written; what
changes is only that it has three consumers rather than one, and the acceptance
surface should exercise all three rather than the telescope case alone.

**Re-measure before relying on this.** `RType`'s variant list is perishable, and
the two-productions worry was live until the enum was opened. The finding is
recorded so it is not re-derived, not so it is trusted.

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
