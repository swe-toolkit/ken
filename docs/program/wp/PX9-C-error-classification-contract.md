# PX9-C — normative contract: honest retry classification + revoked-unification

**Owner:** Spec enclave (spec-leader coordinates; spec-author elaborates; **CV
casts the Spec/Fidelity vote**). **Size:** S. **Capability tier:** T1 — a
soundness-bearing normative property, small but load-bearing. **Route:** **CV
(Spec/conformance) + Architect (soundness** — the Architect pre-committed this vote
at `evt_5rabg6z700p9s`).

**Design authority / why this is distinct from the type WP.** The Architect's PX9
ruling (`evt_5rabg6z700p9s`, grounded at `origin/main @ a93fbee33`) delivers the
type SHAPE and classification MODEL as a design call. This node pins the
**normative CONTRACT** those satisfy, so that PX9-INC1's acceptance criteria cite a
`/spec` property rather than a convo ruling. The Architect deliberately kept the
design edge (theirs) and the behavioral-contract edge (Spec's) distinct — this WP
is that second edge.

## Objective

Pin, in `/spec` (the surface error contract / effect-protocol error-identity
section — spec-author's placement call), the two normative properties PX9 makes
load-bearing, as *statements a conformance seed can discriminate*, not prose:

1. **Transient does not imply retry-safe.** Transience (a property of the error
   identity) and idempotence (a property of the operation) are **orthogonal**; a
   retry-safe verdict is obtainable ONLY from `(Transient, Idempotent)`. The spec
   states this as a property of the retry-guidance relation, with the honest
   corollary that **`Revoked` is `Permanent`** (authority is gone; retry cannot
   help), never transient. The negative form is the load-bearing one: **there is no
   spec-blessed `retryable : error -> Bool`** — a verdict of "safe to retry" that
   does not consume the operation's idempotence is non-conformant.
2. **Revoked is a single identity.** The three prior `revoked` carriers
   (`IOError.Revoked`, `ResourceError.ResourceRevoked`, the host wire's revoked)
   name one semantic identity; the contract pins that a conforming error surface
   exposes **one** `Revoked` identity, classified `Permanent`.

## Deliverables

- **D1 — the normative statements.** Add the two properties above to `/spec` as
  normative clauses, worded so PX9-INC1's kernel-checked laws are the executable
  witness of them (`retry_guidance Transient NonIdempotent =
  RetryUnsafeNonIdempotent`; `RetryAdvised` only from `(Transient, Idempotent)`;
  `error_transience Revoked = Permanent`). Cite the ABI-completion authority
  (`docs/program/10-linux-abi-completion.md §4/§7`) and PX9's objective.
- **D2 — discriminating conformance seed(s).** A seed that a conforming
  implementation passes and that a `retryable`-from-transience-alone
  implementation (or a `Revoked = Transient` classification) **fails** — the
  control that makes the negative property testable, not decorative.

## Acceptance criteria

- **AC-NORMATIVE-ORTHOGONALITY.** `/spec` states the transience⊥idempotence
  property and the "no `retryable : error -> Bool`" prohibition as a normative
  clause a seed can cite. **Control:** the seed in D2 fails a
  transience-alone-retry implementation.
- **AC-REVOKED-PERMANENT-NORMATIVE.** `/spec` states `Revoked` is `Permanent`.
  **Control:** the seed fails a `Revoked = Transient` classification.
- **AC-REVOKED-SINGLE-IDENTITY.** `/spec` states one `Revoked` identity across the
  error surface. **Control:** a conformance check names the single identity; a
  triplicated-revoked surface is non-conformant.
- **AC-CITED-BY-INC1.** PX9-INC1's law ACs can cite this clause by section anchor
  (the coupling is real: this is the normative source PX9-INC1 upgrades to at
  candidate-land).

## Sequencing

`gate: none` (Spec/CV/Architect route, no separate gate). **Build in parallel with
PX9-INC1** (foundation type + laws); the two are coupled only at merge-order —
PX9-INC1's candidate holds until this clause is on `main`. This is the PX8-T
(spec contract) / PX8-F (foundation surface) split applied to PX9. The `/spec`
placement (surface error contract vs effect-protocol error-identity section) is
spec-author's call.
