# Error-classification conformance — seed cases (PX9)

Format: `../../README.md`. These pin the two normative error-classification
properties of `spec/30-surface/38-ffi-io.md §1.8` (the cross-domain error
surface, `docs/program/10-linux-abi-completion.md §4`/`§7`, PX9): **transience
does not imply retry-safety** and **`Revoked` is a single `Permanent`
identity**. The cases net the *discriminating* form of each property — each
reds a plausible non-conforming implementation — not the prose.

The classification model (`error_transience`, `retry_guidance`, and the verdicts
`RetryAdvised`/`RetryUnsafeNonIdempotent`, the transience `Transient`/`Permanent`,
the idempotence `Idempotent`/`NonIdempotent`) is the foundation deliverable
**PX9-INC1**; its kernel-checked laws are the executable witness of §1.8. Until
PX9-INC1 lands, every case here is **RED-UNTIL-BUILT** — the model types do not
yet exist, so the fixtures cannot elaborate. The seed is the control that makes
§1.8's negative properties testable rather than decorative, staged now so
PX9-INC1's law ACs cite it.

## Reading disciplines

**The net is a discriminating pair, never a lone positive.** "Retry is advised
for a transient, idempotent operation" passes green-vs-green against an
implementation that advises retry from transience alone. The property is the
*flip*: on one shared `Transient` error, the retry verdict must differ by the
operation's idempotence, and on `Revoked` it must be `Permanent` regardless.
Each case below carries the non-conforming implementation it must red.

**Verdict, not build state.** These cases assert the normative verdicts the
model must satisfy; they do not assert PX9-INC1 is built. When PX9-INC1 lands the
fixtures elaborate and its laws witness these verdicts, and the RED-UNTIL-BUILT
tag lifts then.

### surface/ffi-io/retry-guidance-requires-idempotence (orthogonality)

- promise class: **normative property** — transience ⊥ idempotence; no
  `retryable : error → Bool`
- spec: `38 §1.8` (property 1); `../../../docs/program/10-linux-abi-completion.md §4`
- given: one error identity classified `Transient`, evaluated by `retry_guidance`
  under an `Idempotent` operation and, unchanged otherwise, under a
  `NonIdempotent` operation — a non-degenerate pair on a shared transience
  differing only in the operation's idempotence.
- expect: **RED-UNTIL-BUILT** — `retry_guidance Transient Idempotent =
  RetryAdvised` while `retry_guidance Transient NonIdempotent =
  RetryUnsafeNonIdempotent`; the verdict flips on the shared `Transient`, keyed
  only by idempotence, and `RetryAdvised` arises from no other pairing.
- fixture: **BLOCKED-ON-PX9-INC1** — `error_transience`, `retry_guidance`, and
  the verdict/idempotence types are the PX9-INC1 foundation deliverable, not yet
  landed.
- control (the one that makes it net): a `retryable : error → Bool`
  implementation — retry decided from the error alone — returns the **same**
  verdict for both arms, so the pair cannot flip and the case reds. A lone
  positive `(Transient, Idempotent) = RetryAdvised` would pass such an
  implementation; the paired `NonIdempotent` arm is what refutes it.
- why: the whole property is that transience is not permission to retry. Only a
  pair that holds the error fixed and varies the operation's idempotence observes
  the orthogonality; a single positive is green-vs-green.

### surface/ffi-io/revoked-is-permanent (permanence)

- promise class: **normative property** — `Revoked` is `Permanent`
- spec: `38 §1.8` (property 1 corollary); `38 §1.3.1`; `38 §1.7.2`
- given: the `Revoked` error identity under `error_transience`, and
  `retry_guidance` for `Revoked` under an `Idempotent` operation (the arm that
  would advise retry for a transient error).
- expect: **RED-UNTIL-BUILT** — `error_transience Revoked = Permanent`, and
  `retry_guidance` for `Revoked` is never `RetryAdvised` even under `Idempotent`,
  because permanence dominates: a revoked authority is gone.
- fixture: **BLOCKED-ON-PX9-INC1**.
- control: a `Revoked = Transient` classification reds — it makes
  `error_transience Revoked = Transient`, so the idempotent arm reaches
  `RetryAdvised`, wrongly advising retry on a revoked authority, against the
  expected `Permanent`/no-retry. Pinning permanence through **both**
  `error_transience` and the retry verdict catches a relabel that changes only
  one.
- why: `Revoked` is the case where honest classification matters most — retry
  cannot restore a lost authority, so a transient misclassification is a live
  liveness hazard (an unbounded retry of a permanently-failing operation), not
  cosmetic.

### surface/ffi-io/revoked-single-identity (unification)

- promise class: **normative property** — one `Revoked` identity across the error
  surface
- spec: `38 §1.8` (property 2); `38 §1.3.1`; `38 §1.7.2`;
  `../../../docs/program/10-linux-abi-completion.md §7`
- given: the revocation observable reached through a path/capability operation
  (`IOError.Revoked`), a resource-token operation (`ResourceError.Revoked`), and
  the host-boundary progress/error partition.
- expect: **RED-UNTIL-BUILT** — all three resolve to **one** `Revoked` semantic
  identity, classified `Permanent`; the conforming surface exposes exactly one
  `Revoked`, and the check names that single identity.
- fixture: **BLOCKED-ON-PX9-INC1** — the unification is PX9-INC1's, sequenced
  after `ABI-REVOKE` (`../../../docs/program/10-linux-abi-completion.md §7`).
- control: a triplicated-revoked surface — three distinct `Revoked` constructors
  — is non-conformant and reds the single-identity check. Unification is not
  collapse: the check must equally red a surface that maps `Revoked` into
  `CapabilityDenied`, `Closed`, or a host I/O failure (`38 §1.7.2`), which loses
  the identity in the other direction.
- why: three carriers for one observable is the pre-PX9 state; a client cannot
  match one `Revoked` if the surface names three, and cannot trust a single one
  if it silently aliases a neighbour.
