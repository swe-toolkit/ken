# WP frame — LANG-MOD-ATTACHED-PROOF-OWNERSHIP (campaign tail, before z3 resume)

> Campaign work under [[LANG-MODULE-IMPORT-SYSTEM]]. Owning team: spec (enclave).
> Size S. Capability tier: T2 in effort but enclave-owned — the design is FULLY
> RULED (convert-to-local, namespace closed under the subject's defining module:
> spec-author evt_6wbz5eeh5v1y6, Architect evt_7d90kztv4n7hv part I, spec-leader
> evt_2fgr39s4ghebn / evt_2grrxacejq6f5); this WP transcribes that ruling into
> normative spec text plus paired conformance, codifying EXISTING behavior with
> no new mechanism and zero `trusted_base()` change. No Decision — no
> product/design tradeoff remains. Scheduled at the END of the LANG-MOD campaign
> (after Component B), before the verify/z3 FO-checker resume.
> `depends_on: [LANG-MOD-CATALOG-COMPLETENESS]`.

## Objective

Make the current attached-proof namespace contract explicit in the spec: a
proof's attached namespace is closed under its subject's defining module, so an
attached-declaration head whose resolved subject is NONLOCAL rejects at
declaration time (rather than the present accept-then-unreachable state). Pair it
with conformance. This closes the delayed/unreachable failure path Component B
surfaced without designing a third-party extension namespace.

## Fixed inputs (the ruling; re-grounded on current main d5c41ec1)

Grounded at `6cadce775` / base `3a7114cf7`, re-checked on current main
`d5c41ec1`.

- Spec `33 §8.2` gives an attached `proof p for s` the canonical identity `s::p`;
  subject resolution runs FIRST, attached lookup consults the SUBJECT'S provider
  second. Attachment is namespacing over an already-checked theorem and carries
  ZERO soundness weight. The canonical key contains NO attaching-module
  coordinate, so a consumer cannot inject names into a foreign subject's
  namespace without a new identity/coherence/orphan scheme that §8.2 does not
  have.
- The as-built loader already matches this: `resolve_attached_ref`
  (`crates/ken-elaborator/src/modules.rs`) resolves a nonlocal subject, splits
  its provider module/leaf, and consults only that provider's export map;
  prebinding deliberately does not create an ordinary local binding for attached
  proofs. The current accept-then-unreachable state is an INCOMPLETE rejection
  path, not authorization for an open extension namespace.
- `33 §8.3` independently authorizes the needed form: a `theorem` is an
  Omega-checked proof definition in its declaring module's ordinary namespace.
  Consumers therefore state facts ABOUT an imported subject with `theorem`, not
  with foreign attachment. A future open extension-proof namespace would be a
  separate, explicit language design with its own owner/coherence identity axis —
  OUT OF SCOPE here.

## Deliverables

- **D1 — normative clarification.** In `spec/30-surface/33-declarations.md §8.2`
  (and the corresponding `spec/30-surface/39-elaboration §2.0` resolver step):
  state that the attached namespace is closed under the subject's defining module
  in the current language, and that an attached-declaration head whose resolved
  subject is NONLOCAL rejects at declaration time. NO `32` grammar change; NO
  third-party extension namespace.
- **D2 — paired conformance controls.** (a) provider-local attachment accepts and
  imports by the canonical subject path; (b) changing ONLY the consumer
  declaration to `proof p for Imported.s` rejects as an unsupported foreign
  attachment (at declaration time, not accept-then-unreachable); (c) the same
  Omega claim stated as a consumer-local `theorem` accepts. Component B's
  convert-to-local change (AC-B8) plus its roots-loader acceptance control is the
  paired build-side evidence.

## Acceptance criteria

- AC-1 — the §8.2 / §39 normative text states the closed-under-defining-module
  contract and the declaration-time rejection of a nonlocal attached head; no
  grammar (`32`) change; no new mechanism.
- AC-2 — conformance: provider-local attachment accepts and imports by the
  canonical subject path.
- AC-3 — conformance: the same declaration with a foreign subject
  (`proof p for Imported.s`) rejects at declaration time as an unsupported
  foreign attachment.
- AC-4 — conformance: the equivalent consumer-local `theorem` for the same Omega
  claim accepts (proving the rejection is about foreign attachment, not the
  fact).
- AC-5 — zero `trusted_base()` change; the existing flat-Σ invariant is
  preserved (this codifies existing behavior).
- AC-NO-REGRESSION — whole-suite green in CI; local targeted only, never
  `--workspace`.

## Contention check

Touches `spec/30-surface/33-declarations.md §8.2`,
`spec/30-surface/39-elaboration` §2.0, and the paired conformance seed(s). May
touch a focused elaborator conformance test asserting the declaration-time
rejection. No overlap with lane 1 (runtime) or lane 3 (foundation). Sequenced
after Component B, whose convert-to-local change is the build-side pair. The
Librarian holds the as-built doc watch on the `33 §8.2` attachment passage
(surface-reference, evt_1gkq66rghyvdp).

## Reviewers

spec-author (normative fidelity: the text codifies the ruled closed-namespace
contract and the declaration-time rejection exactly) + spec-leader (enclave
sign-off) + conformance-validator (the accept/reject/theorem triad is
discriminating and the reject fires at declaration time). Architect advisory on
soundness-neutrality (attachment carries zero soundness weight). No Decision
fork.

## Sequencing

Campaign tail, owner spec. Codifies EXISTING behavior and does NOT gate Component
B's build (B proceeds by converting its two foreign-subject attachments to
Lawful-local `theorem`s, AC-B8). Scheduled after
[[LANG-MOD-CATALOG-COMPLETENESS]] per operator direction (2026-08-21): the two
new LANG-MOD nodes are the end of the LANG-MOD work, before the verify/z3
FO-checker lane resumes ([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]] /
[[V3-FO-EMBEDDING-ADEQUACY]]).
