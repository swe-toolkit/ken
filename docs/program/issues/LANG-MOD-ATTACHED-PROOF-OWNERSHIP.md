---
id: LANG-MOD-ATTACHED-PROOF-OWNERSHIP
title: "Attached-proof namespace closure: normative clarification that a proof's attached namespace is closed under its subject's defining module (a nonlocal attached-declaration head rejects at declaration time), with paired conformance. The coupled durable artifact of the Component B convert-to-local ruling."
status: draft
owner: spec
size: S
gate: none
depends_on: [LANG-MOD-CATALOG-COMPLETENESS]
blocks: []
github: null
origin: "Component B third structural wall (language-implementer evt_4757hgk2t2mj6): LawfulClasses attaching lt_asym/eq_sound to imported Derived.pair_compare. Ruled convert-to-local, no loader extension, by spec-author evt_6wbz5eeh5v1y6, Architect evt_7d90kztv4n7hv part I, spec-leader evt_2fgr39s4ghebn / evt_2grrxacejq6f5. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # FRAMED + SCHEDULED 2026-08-25 — LANG-MOD campaign tail (operator direction)
>
> WP frame authored: `docs/program/wp/LANG-MOD-ATTACHED-PROOF-OWNERSHIP.md`.
> Operator (2026-08-21) directed the two new LANG-MOD nodes be scheduled at the
> END of the module/import work — after Component B
> ([[LANG-MOD-CATALOG-COMPLETENESS]]) — and BEFORE the interrupted verify/z3
> FO-checker lane resumes ([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]] /
> [[V3-FO-EMBEDDING-ADEQUACY]]). `depends_on: [LANG-MOD-CATALOG-COMPLETENESS]`
> now encodes that schedule. The design is fully ruled (convert-to-local, closed
> namespace); this WP transcribes it into normative spec text + conformance,
> owner spec. Stays `draft` until Steward releases it as the campaign tail. Not
> released yet.

> # DRAFT — codifies EXISTING behavior; does NOT gate Component B's build
>
> Component B proceeds NOW by converting its two foreign-subject attachments to
> Lawful-local `theorem`s (COMPLETENESS AC-B8). This node is the coupled durable
> spec/conformance artifact the enclave asked to be framed/sequenced ALONGSIDE
> B — it makes the current accept-then-unreachable reject path explicit in the
> contract. No new mechanism, no `trusted_base()` change, no operator gate.

## The ruling (grounded at 6cadce775 / base 3a7114cf7)

Spec `33 §8.2` gives an attached `proof p for s` the canonical identity
`s::p`; subject resolution runs FIRST, attached lookup consults the SUBJECT'S
provider second. Attachment is namespacing over an already-checked theorem and
carries ZERO soundness weight. The canonical key contains NO attaching-module
coordinate, so a consumer cannot inject names into a foreign subject's
namespace without a new identity/coherence/orphan scheme that §8.2 does not
have. The as-built loader matches this: `resolve_attached_ref` resolves a
nonlocal subject, splits its provider module/leaf, and consults only that
provider's export map (`modules.rs:310-334`); prebinding deliberately does not
create an ordinary local binding for attached proofs (`:1650-1657`). The
current accept-then-unreachable state is an INCOMPLETE rejection path, not
authorization for an open extension namespace.

`33 §8.3` independently authorizes the needed form: a `theorem` is an
Omega-checked proof definition in its declaring module's ordinary namespace.
Consumers therefore state facts ABOUT an imported subject with `theorem`, not
with foreign attachment. A future open extension-proof namespace would be a
separate, explicit language design with an additional owner/coherence identity
axis — out of scope here.

## Deliverables

1. Normative clarification in `33 §8.2` (and the corresponding `39 §2.0`
   resolver step): the attached namespace is closed under the subject's defining
   module in the current language; an attached-declaration head whose resolved
   subject is NONLOCAL rejects at declaration time. No `32` grammar change; no
   third-party extension namespace.
2. Paired conformance controls: provider-local attachment accepts and imports by
   the canonical subject path; changing only the consumer declaration to
   `proof p for Imported.s` rejects as an unsupported foreign attachment; the
   same Omega claim as a consumer-local `theorem` accepts. This closes the
   delayed/unreachable failure without designing third-party extensions.

## Sequencing

Sequenced alongside [[LANG-MOD-CATALOG-COMPLETENESS]]; not a blocker of it. The
Component B catalog change (the convert-to-local + its roots-loader acceptance
control, AC-B8) is the paired build-side evidence. The librarian holds the
as-built doc watch on the `33 §8.2` attachment passage (surface-reference) for
this increment's landing (evt_1gkq66rghyvdp).
