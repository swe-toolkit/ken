---
id: CORE-FO-CHECK-TREE-SORT-VALIDATION
title: "Make both checker surfaces' own domain honest: derive and validate world/object sorts and binder scope in check_tree and in the Ken checker, fail-closed, instead of relying on every caller to pass embed's image"
status: ready
owner: language
size: M
tier: T1
gate: none
depends_on: [V3-FO-QUOTE-GUARD-FAIL-CLOSED]
blocks: [V3-FO-SORTED-EIGENPARAMETER-DERIVATION]
github: null
origin: "Steward, 2026-08-15, filing the future-hardening item the Architect separated out in evt_71g1xf5vkf1ek while dispositioning a V3-FO-QUOTE-GUARD-FAIL-CLOSED QA block. RECUT AND PROMOTED by the Steward 2026-08-27 on Architect ruling evt_6hx31xvw9tqs2, which REJECTED the current FO checker/derivation/adequacy interface as a semantic soundness gate. This node is no longer optional hardening: it is items 1 and 2 of that ruling's six-item repair envelope and the PREDECESSOR of the sequence. Its former 'Why this is hardening and NOT a soundness fix' section is FALSIFIED and has been removed -- see 'What changed, and what falsified the old framing'. Steward-filed per COORDINATION section 2."
---

## Objective

Give both checker surfaces — the Rust `check_tree` and the Ken checker — a
**derived world/object sort discipline and a real binder context**, validated
**fail-closed at each boundary**, so that neither surface depends on `embed`
having been its caller.

This node claims **nothing new** about semantic lawfulness. It only makes both
checkers **stricter**. That is precisely what makes it releasable on its own.

## The measurement, which is not in dispute

Language QA built a literal checker probe and ran it:

```
ForallWorld (ForcingP Bound(0) Bound(0) -> ForcingP Bound(0) Bound(0))
  with ForallRight { eigen: Parameter(0) }, then ImpRight, then Init
```

The second `Bound(0)` is an **object** slot and the eigenparameter is a
**world**. **`check_cert` returned `true`.**

`Form` and `QTerm` carry no sort tag, `check_tree` validates no sorts, and
`Init` closes the derivation syntactically. **Freshness of the eigenparameter is
a condition on the derivation's variable discipline and says nothing about which
sort slot a parameter lands in.**

## What changed, and what falsified the old framing

This node used to carry a section headed *"Why this is hardening and NOT a
soundness fix"*, resting on Architect ruling `evt_71g1xf5vkf1ek`: `check_cert`
is total over `Form`, `Form` is strictly larger than the image of `embed` on
`IForm Sigma`, the QA probe lived in that **excess**, and so the
accepted-but-ill-sorted certificates were **unreachable from the route**.

**That containment is keyed on FORMULA REACHABILITY, and it no longer holds the
weight it was carrying.** The language-implementer produced an accepted
certificate that starts from a **closed source form** and targets its **actual
`fok_embed` image** (`evt_2yh515wg0mczy`, exact base `ef91b8225`), and a
kernel-checked Ken theorem derives `Bottom` from the landed
`fok_embedding_adequacy_statement`. The old partition never constrained
**certificates** — only formulas — so it never bore on this axis at all.

Architect ruling `evt_6hx31xvw9tqs2` then **rejected the current
checker/derivation/adequacy interface as a semantic soundness gate**, and ruled
it **not repairable by finishing the current proof**.

⇒ **The caller-obligation option the earlier ruling chose is withdrawn.** The
guarantee moves into the checkers, on both surfaces, and it is a **predecessor
of the soundness repair** rather than optional future work.

> **Production is unaffected and needs no emergency change.** The rejection
> invalidates the proposed **theorem gate**, not the current production verdict
> boundary. `attempt_fo_with_signature` (`prover.rs:562-604`) still returns
> `emit_unknown_hole_fo_withheld` — an audited `Unknown`, **never `Proved`** —
> when `quote_fo` + `find_certificate` + `check_cert` all accept. The defect is
> **real but latent**, and the gate containing it is the very theorem now
> rejected. State this whenever quoting "REJECT".

## THE DESIGN FORK IS RULED, AND THE OTHER ARM IS A HARD STOP

The old `D0` left the carrier open: **a sort tag on `QTerm`/`Form`**, or **a
sort-checking pass over the existing untyped representation**. That choice is no
longer free, because it decides whether this node can exist at all.

**RULED: the validation-pass arm.** Derive each occurrence's sort from its
position — `ForallWorld`/`ForallObj` already distinguish the binders, and the
atom roles are fixed (`Access` takes two worlds; `DomainA` and `ForcingP` take a
world then an object) — and check consistency against a derived binder context.
**Do not add a sort tag to the target datatypes.**

**Why, and this is the whole reason the sequence has three nodes instead of
one.** `fok_checker_soundness` is a **structural reflection** theorem stated
over the Ken target datatypes. A **validation pass** leaves those datatypes
alone and only removes acceptances, so the landed reflection theorem survives
untouched and this node claims nothing it cannot support. **A carried sort tag
changes the datatype `FokDerivation` and `fok_checker_soundness` are stated
over** — which drags items 1 and 2 into the lockstep bundle of
[[V3-FO-SORTED-EIGENPARAMETER-DERIVATION]] and **collapses the whole repair into
one atomic frame.**

**If `D0` finds the validation pass insufficient to enforce items 1 and 2 —
that is, if some malformed term cannot be rejected without carrying a tag —
that is a HARD STOP to the Steward.** It is a re-cut of the sequence, not
something to absorb inside this node. Report which term shape forced it. The
Architect's item 3 explicitly admits either *"unrepresentable"* or *"explicitly
rejected before substitution"*, so the pass arm is within the ruling; the hard
stop is for discovering it is not within the code.

## Deliverables

**`D0` — the sort derivation and the binder context, stated before it is
built.** Write down the sort assignment rule for every `QTerm` position and
every atom role, and the binder-context discipline a traversal maintains
(`ForallWorld` pushes a world binder, `ForallObj` an object binder). **Say
explicitly whether the pass can reject every term shape items 1 and 2 name.** If
it cannot, stop here per the fork above — a `D0` that hard-stops with that
answer is a **complete result**, not a failure.

**`D1` — the Rust boundary.** `check_tree` derives sorts and scope and
**rejects fail-closed** a derivation that places a parameter or bound reference
of one sort into a slot of another, or that references a binder not in scope.
The QA probe above is the positive control and must flip from accepted to
rejected.

**`D2` — the Ken boundary.** The same discipline on the Ken checker. **The
checker may not rely on `embed` being its caller** — this is the half the
earlier disposition left to the caller, and it is the half that matters, because
the refuting certificate arrives through `fok_embed`'s image rather than around
it.

**`D3` — retire the caller obligation, or state the exact residual.** The
comment at the `check_cert` site assigning the guarantee to the caller becomes
false where validation is complete and must be corrected. **Where it is partial,
name precisely which residual the caller still owns.** A partially-honest domain
that reads as fully honest is worse than the current state. This includes
`AC-4a` on [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]], which is live in the
meantime and requires every `check_cert` call to take a `Form` produced by
`embed Sigma f`.

## Acceptance criteria

**`AC-1`.** The QA probe is a landed control that **fails before the change**
and rejects after it, on **both** boundaries.

**`AC-2`. Lawful certificates are still accepted, on both surfaces.** A
strictly-stricter checker that also rejects well-sorted parameter certificates
has broken the route rather than hardened it. Pin at least one accepted lawful
certificate per quantifier rule.

**`AC-3`. Independent wrong-sort mutations each preserve refusal**, measured
separately, not as one bundled row: a **world** eigenparameter landing in an
**object** binder; an **object** eigenparameter landing in a **world** binder; a
malformed **atomic argument role**; an **out-of-scope bound reference**.

**`AC-4`. Positive controls show refusal is not caused by an unrelated
malformed tree.** For each `AC-3` mutation, a near-identical well-sorted tree
differing only in the mutated coordinate is **accepted**. Without this, `AC-3`
is satisfied by a checker that rejects everything.

**`AC-5`. Compile-preserving mutations that collapse the sort check redden a
control.** Name the injection point of each mutation, not its effect — two
sites share one English description.

**`AC-6`. No behaviour change for any well-sorted `Form` in the image of
`embed` on `IForm Sigma`.** This is a domain restriction on the excess and on
the ill-sorted, not a semantic change to the route.

**`AC-7`. No `proved` for FO and no slice widening.** Route FO continues to
return the audited `Unknown`. This node does not move the verdict boundary and
must not be reviewed as if it did.

**`AC-8`. No new kernel primitive and no trusted axiom.**

**`AC-9`. The landed `fok_checker_soundness` still elaborates unchanged.** This
is the mechanical check that the validation-pass arm was actually taken — if a
datatype moved, this reddens, and that is the fork's hard stop arriving late.

**`AC-10`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **The eigenparameter rule itself.** `ForallRight` accepting only a fresh
  sorted parameter, the typed instantiation replacing arbitrary-term `subst0`,
  the `FokDerivation` constructors, and the reflection proofs are the **atomic
  lockstep** increment [[V3-FO-SORTED-EIGENPARAMETER-DERIVATION]]. Taking any
  part of it here splits a bundle that cannot be split.
- **Re-establishing adequacy.** [[V3-FO-EMBEDDING-ADEQUACY]], last in the
  sequence.
- **Adding a sort tag to the target datatypes.** Ruled out above; discovering it
  is necessary is a hard stop, not a licence.
- **Widening the slice.** Unchanged by this node.
- **Signature discovery.** [[V3-FO-OBLIGATION-SIGNATURE-DISCOVERY]].

## Sequencing

**First of three.** The repair sequence is this node, then the atomic lockstep
increment [[V3-FO-SORTED-EIGENPARAMETER-DERIVATION]], then
[[V3-FO-EMBEDDING-ADEQUACY]] re-established over the corrected relation.

**It is the predecessor because it is the only part that can land honestly on
its own.** It removes acceptances and asserts no theorem, so a partial state of
the repair that stops here leaves both checkers stricter, both landed theorems
intact, and production unchanged at the audited `Unknown`.

## Provenance

Language QA's probe and block on `V3-FO-QUOTE-GUARD-FAIL-CLOSED` exact
`404ac2c39963e2b9a285f65249267382aeec1b5a` (`evt_2e0zejwcw8j3m`); Architect
disposition `evt_71g1xf5vkf1ek`, which chose the honest-comment option and is
now **superseded on this point**; the refuting certificate
`evt_2yh515wg0mczy` on exact `ef91b8225`; Architect rejection ruling
`evt_6hx31xvw9tqs2` (base `ef91b8225`, tree
`19e0543a4ac006b24b256a038e25e83f29894162`), items 1 and 2 of its repair
envelope; Steward disposition `evt_55w8hgwbc053r`.
