# Error-classification conformance — seed cases (PX9)

Format: `../../README.md`. These pin the three normative properties of
`spec/30-surface/38-ffi-io.md §1.8`: **transience does not imply
retry-safety**, **`Revoked` is a single `Permanent` identity**, and
**`Unsupported` is `Permanent`**. The cross-domain error surface is also
specified in `docs/program/10-linux-abi-completion.md §4`/`§7` (PX9). The
cases net the *discriminating* form of each property — each reds a plausible
non-conforming implementation — not the prose.

The classification model (`error_transience`, `retry_guidance`, and the verdicts
`RetryAdvised`/`RetryUnsafeNonIdempotent`, the transience `Transient`/`Permanent`,
the idempotence `Idempotent`/`NonIdempotent`) is in the landed module
`Capability.System.Error` (`catalog/packages/Capability/System/Error.ken.md`).
Its checked §1.8 theorems are
`retry_advised_only_for_transient_idempotent` and
`error_transience_revoked_permanent`.

## Reading disciplines

**The net is a discriminating pair, never a lone positive.** "Retry is advised
for a transient, idempotent operation" passes green-vs-green against an
implementation that advises retry from transience alone. The property is the
*flip*: on one shared `Transient` error, the retry verdict must differ by the
operation's idempotence, and on `Revoked` it must be `Permanent` regardless.
Each case below carries the non-conforming implementation it must red.

### surface/ffi-io/retry-guidance-requires-idempotence (orthogonality)

- promise class: **normative property** — transience ⊥ idempotence; no
  `retryable : error → Bool`
- spec: `38 §1.8` (property 1);
  `../../../docs/program/10-linux-abi-completion.md §4`
- given: one error identity classified `Transient`, evaluated by `retry_guidance`
  under an `Idempotent` operation and, unchanged otherwise, under a
  `NonIdempotent` operation — a non-degenerate pair on a shared transience
  differing only in the operation's idempotence.
- expect: `retry_guidance Transient Idempotent = RetryAdvised` while
  `retry_guidance Transient NonIdempotent = RetryUnsafeNonIdempotent`; the
  verdict flips on the shared `Transient`, keyed only by idempotence, and
  `RetryAdvised` arises from no other pairing.
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
- expect: `error_transience Revoked = Permanent`, and `retry_guidance` for
  `Revoked` is never `RetryAdvised` even under `Idempotent`, because permanence
  dominates: a revoked authority is gone.
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

### surface/ffi-io/unsupported-is-permanent (facility error)

- promise class: normative property — `Unsupported` is `Permanent`.
- spec: `38 §1.8` property (3).
- given: the `IOError.Unsupported` identity under `error_transience`, and
  `retry_guidance` for that same identity with `Idempotent` operation
  idempotence.
- expect: `error_transience IOError.Unsupported = Permanent`, and
  `retry_guidance` does not return `RetryAdvised`. The §1.8(1) rule allows
  that verdict only for the pair `(Transient, Idempotent)`.
- control: relabeling `Unsupported` as `Transient` reds the permanence
  assertion. Together with the `retry-guidance-requires-idempotence` positive
  arm, that mutation also reaches `RetryAdvised` for `Idempotent` and violates
  the expected no-retry result.
- why: the host-facility row observes the error identity, but not its retry
  classification. This case pins the separate normative permanence claim.

### surface/ffi-io/revoked-single-identity (current carriers)

- promise class: **normative property** — one `Revoked` identity across the error
  surface
- spec: `38 §1.8` (property 2); `38 §1.3.1`; `38 §1.7.2`/`§1.11`;
  `../../../docs/program/10-linux-abi-completion.md §7`
- given: the current revocation observable reached through a path/capability
  operation (`IOError.Revoked`), an ordinary FS/Net resource-token operation
  (`ResourceError.ResourceHostIO IOError.Revoked`), a Process-host operation
  (`ResourceHostIO IOError.Revoked`), and the host-boundary progress/error
  partition.
- expect: all current carriers resolve to **one** `Revoked` semantic identity,
  classified `Permanent`. The nullary `ResourceError.Revoked` is a staged
  post-unification projection, not a current checked constructor; its carrier
  transition is pinned by the separately gated resource-token cases.
- control: a surface that assigns different `Revoked` identities to the same
  current `IOError.Revoked` cause, or maps it into `CapabilityDenied`,
  `Closed`, or an unrelated host-I/O failure (`38 §1.7.2`), is non-conformant.
- why: the current carrier shapes differ by operation family but retain the
  same cause. The staged resource-token projection changes the carrier, not the
  semantic denial or its `Permanent` classification.

## Provenance

This as-built update is grounded in the landed Ken source at
`e5cd36c5432d1af8b62aba988ef93d159d481ffd`. No external prior-art reference,
`local/refs/` implementation, or excluded prototype was consulted.
