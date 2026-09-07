# PX9-INC1 — `SystemError` type + honest retry classification (fs-populated)

**Owner:** Foundation build (foundation-leader coordinates; foundation-implementer
elaborates; **Foundation QA** reviews). **Size:** M. **Capability tier:** T1 —
novel prelude type design plus soundness-bearing classification laws; the laws are
the risk, not the line count. **Route:** **Architect (REQUIRED** — the
classification laws are the soundness core, the Architect's own statement) **+
Foundation QA + CV** (behavioral conformance of the classification), on the exact
SHA → Steward M1-M4 → lieutenant M5-M9.

**Design authority:** Architect PX9 design + decomposition ruling
`evt_5rabg6z700p9s`, grounded at `origin/main @ a93fbee33`. This is increment 1 of
PX9 (`docs/program/issues/PX9.md`), which the Architect ruled *unambiguously in
PX9*. The type SHAPE and the classification MODEL below are the Architect's
component-design call — **do not reopen them**; this frame turns that design into
deliverables and acceptance criteria.

**Normative contract (coupled, in flight):** the transient≠retry-safety property
and the revoked-unification are being anchored in `/spec` by **PX9-C** (Spec-owned,
routed to spec-leader this turn; Architect casts its soundness vote). This frame's
law ACs cite PX9-C as their normative source; until PX9-C lands they cite the
Architect ruling + PX9 §7/objective as interim authority. **Merge-order:** the
candidate holds at Steward M1-M4 until PX9-C is on `main` (build in parallel; the
build does not depend on PX9-C landing — it is a pure prelude type + laws provable
from this design).

## Objective

Land, in the prelude, a **cross-domain `SystemError`** whose shape is
domain-general by construction but whose `Operation`/`ResourceRef` slots are
populated with **filesystem as the first and only domain**, plus the **honest
retry classification** and its **kernel-checked laws**. No host-wire change, no
re-threading of any FS/resource operation — those are increment 2. Increment 1 is
a pure prelude type + total functions + proved laws: self-contained and provable
before any operation is migrated.

The point of doing this before PX10/PX11 (§7, restated by the Architect): the
identity vocabulary and the classification are **domain-independent**; only
`Operation` and `ResourceRef` are domain-tagged and grow per domain. Landing the
domain-general shape now — with fs first-populated — is what lets process/socket
add arms as **pure additions** later instead of forcing a retrofit across two
large surfaces.

## Fixed inputs — Architect-grounded at `a93fbee33`; RE-MEASURE at your cut (D0)

Lines drift; re-measure every one at the release SHA before authoring.

- **`IOError`** (`crates/.../prelude.rs:581`) = `NotFound | PermissionDenied |
  CapabilityDenied | BrokenPipe | Interrupted | AlreadyExists | InvalidInput |
  IsDirectory | NotDirectory | NotEmpty | Unsupported | Revoked | Other Int`. This
  is **already** the cross-domain semantic-identity vocabulary (`Other Int` = the
  raw-errno escape; `Interrupted` and `Revoked` already members). `ErrorIdentity`
  generalizes this, keeping every member.
- **`FileError`** (`prelude.rs:588`) = `MkFileError FileOperation (Option Bytes)
  IOError` — the **filesystem-only carrier** PX9 generalizes: it binds (operation,
  resource=path, identity) with fs-typed slots. `SystemError` is precisely the
  generalization of this carrier's two typed slots, not a new identity vocabulary.
- **`ResourceError`** (`prelude.rs:1646`) = `ResourceHostIO IOError` + its own
  `ResourceRevoked`. So `revoked` is **triplicated** (`IOError.Revoked`,
  `ResourceError.ResourceRevoked`, and the wire's revoked). Increment 1 gives
  `ErrorIdentity` a **single** `Revoked`; retiring the other two carriers is
  increment 2.
- **`FileOperation`** (the 10 fs ops) — its arms become the **fs arms** of the
  generalized `Operation`.
- **`Errors.ken.md`** renderer — the deliberate "stable non-leaking label; raw
  path only for structured inspection" discipline that `SafeContext` generalizes.

## Design — Architect-ruled, do not reopen

```
data SystemError = MkSystemError Operation ResourceRef ErrorIdentity SafeContext
```

- **`ErrorIdentity`** = generalized `IOError`: keep all members incl. `Other Int`
  (raw errno where present) and `Interrupted`; **fold the three `Revoked`
  identities into ONE `Revoked`**. Domain-INDEPENDENT — a future socket
  `ECONNREFUSED` maps to an identity; genuinely socket-only identities are added by
  PX11 as pure arm additions.
- **`Operation`** = generalized `FileOperation` (its 10 arms become the fs arms).
  ONE extensible sum, grown per domain; **not** a `(Domain, opcode)` pair
  (subsume-don't-proliferate). PX10 adds process arms, PX11 socket arms — later,
  not here.
- **`ResourceRef`** = cross-domain resource descriptor; fs = path (`Option Bytes`,
  as today). Extensible sum, fs first-populated.
- **`SafeContext`** = a redaction-aware context that never carries
  authority-sensitive bytes verbatim and renders to a stable non-leaking label (the
  security-boundary field; generalizes the `Errors.ken.md` renderer discipline).
  **[Architect's reading of "safe context"; flagged to the operator/Spec for
  confirmation — see PX9's operator scope note. Build to this reading unless the
  confirmation narrows it.]**

The classification — **two orthogonal axes, never one bit**:

- `Transience = Permanent | Transient`; `Idempotence = Idempotent |
  NonIdempotent`; `RetryGuidance = RetryAdvised | RetryUnsafeNonIdempotent |
  DoNotRetryPermanent`.
- `error_transience : ErrorIdentity -> Transience` (total; a function of the
  identity alone). `EINTR/EAGAIN/EWOULDBLOCK` = `Transient`; `ENOENT/EACCES/EEXIST/
  EINVAL/…` = `Permanent`. **`Revoked` = `Permanent`** (authority is gone; retry
  cannot help) — the honesty pin, not `Transient`.
- `operation_idempotence : Operation -> Idempotence` (a property of the OPERATION,
  not the error). read-at-offset = `Idempotent`; append / create-exclusive /
  rename-over = `NonIdempotent`.
- `retry_guidance : Transience -> Idempotence -> RetryGuidance` — the ONLY function
  that yields a retry verdict, and it takes BOTH axes. **There is deliberately NO
  `retryable : SystemError -> Bool`.** `RetryAdvised` is reachable ONLY from
  `(Transient, Idempotent)`; transience alone yields at most
  `RetryUnsafeNonIdempotent`.

**Designed-for, NOT populated in increment 1:** a third axis — commit/partial-
progress status (a non-idempotent op that took `EINTR` mid-write). `retry_guidance`
is shaped so a third input can be added additively (PX11/PX12 streaming/socket
domains will need it). Increment 1 establishes the two-axis core only.

## Deliverables

- **D0 — re-measure at the release SHA.** Re-run every fixed input above: the exact
  `IOError`/`FileError`/`ResourceError` members + line ranges, the `FileOperation`
  arm set (→ fs `Operation` arms), the three `Revoked` sites, and the
  `Errors.ken.md` renderer discipline. This node's premise — fs as the first
  populated domain of a cross-domain shape — rests on this census.
- **D1 — the types.** In the prelude: `SystemError`, `ErrorIdentity` (single
  `Revoked`), `Operation` (fs arms only), `ResourceRef` (fs = path only),
  `SafeContext`, `Transience`, `Idempotence`, `RetryGuidance`. Domain-general
  shape: nothing fs-specific is baked into `SystemError`'s shape — the ONLY
  domain-tagged slots are `Operation` and `ResourceRef`.
- **D2 — the classification functions.** `error_transience` (total over
  `ErrorIdentity`), `operation_idempotence` (total over `Operation`),
  `retry_guidance : Transience -> Idempotence -> RetryGuidance`. **No** `retryable :
  SystemError -> Bool`, and no other function from which a retry-safe verdict is
  reachable without an operation's idempotence.
- **D3 — the kernel-checked laws.** Proved, not commented:
  1. `retry_guidance Transient NonIdempotent = RetryUnsafeNonIdempotent`;
  2. `retry_guidance Permanent i = DoNotRetryPermanent` for both `i`;
  3. `RetryAdvised` is derivable ONLY from `(Transient, Idempotent)`;
  4. `error_transience Revoked = Permanent`.

**Natural partial boundary (§ merge-policy: a partial merges as soon as it is
done).** D1+D2 (types + classification functions) is a releasable increment on its
own; **D3 (the laws) is the risk and the natural hard-stop seam.** If a law resists
proof within the turn, land D1+D2 and re-cut D3 — do not stall silently.

## Acceptance criteria (each with its control)

- **AC-UNSAFE-RETRY-UNREPRESENTABLE.** A retry-safe verdict (`RetryAdvised`) is
  derivable ONLY via `retry_guidance` applied to `(Transient, Idempotent)`; there
  is no total `SystemError -> Bool`/`-> retry-safe` reachable without supplying the
  operation's idempotence. **Control:** an attempt to obtain `RetryAdvised` from a
  `Transience` alone has no constructor/function path (does not typecheck).
- **AC-REVOKED-PERMANENT.** `error_transience Revoked = Permanent`, kernel-checked.
  **Control:** a mutation `Revoked ↦ Transient` reddens the law.
- **AC-LAW-TRANSIENT-NONIDEMPOTENT.** `retry_guidance Transient NonIdempotent =
  RetryUnsafeNonIdempotent`, kernel-checked. **Control:** mutating the RHS to
  `RetryAdvised` reddens.
- **AC-LAW-PERMANENT.** `retry_guidance Permanent i = DoNotRetryPermanent` for both
  `i`, kernel-checked. **Control:** a mutation reddens.
- **AC-TOTAL-CLASSIFIERS.** `error_transience` is total over `ErrorIdentity` and
  `operation_idempotence` total over `Operation`, **exhaustive, no wildcard/catch-
  all** hiding an unclassified case. **Control:** adding an `ErrorIdentity` arm
  without extending `error_transience` fails totality (the classifier is exhaustive
  by construction, not by a `_ ->` default).
- **AC-REVOKED-SINGLE.** `ErrorIdentity` carries **exactly one** `Revoked` arm,
  classified `Permanent`. (Retiring `IOError.Revoked` and
  `ResourceError.ResourceRevoked` is increment 2; increment 1 only establishes the
  single unified identity.) **Control:** a second `Revoked`-role arm is rejected /
  flagged by the classifier totality check.
- **AC-CROSS-DOMAIN-SHAPE.** Nothing fs-specific is wired into `SystemError`'s
  shape or into `error_transience`/`retry_guidance` — they range over the
  domain-general `ErrorIdentity`/`Transience`/`Idempotence`, not over
  `FileOperation`. **Control:** a reviewer confirms PX10/PX11 could add
  `Operation`/`ResourceRef` arms + identities as pure additions with no reshape of
  `SystemError` or the classification.
- **AC-SCOPE-GUARD (no wire, no re-thread).** Increment 1 adds `SystemError`
  **alongside** the existing types; it does NOT modify `effect_wire.rs`, does NOT
  re-thread any FS/resource operation to carry `SystemError`, and does NOT remove
  `IOError`/`FileError`/`ResourceError`. **Control:** the diff touches only prelude
  type/function/law definitions and their tests/proofs — no `effect_wire.rs`, no
  operation signatures.
- **AC-NO-REGRESSION.** Full `-p ken-elaborator` (and whichever crate homes the
  prelude proofs) green in CI over the complete affected-target closure. Targeted
  via `scripts/ken-cargo`, never `--workspace` — green in CI is the workspace
  verdict.

## Gate, reviewer, sequencing

`gate: none`. On the candidate: **Architect (REQUIRED** — classification laws are
the soundness core) **+ Foundation QA + CV**, on the exact SHA → Steward M1-M4 →
lieutenant M5-M9. **Depends on PX9-C** (the Spec normative anchor) for
**merge-order only**: build in parallel, hold the candidate at M1-M4 until PX9-C is
on `main`. **Increment 2** (fs migration + host-wire unification — TCB-adjacent) is
a **separate WP**; whether it sits inside PX9 or a sequenced successor node is an
**operator scope call** (forwarded), and is not a dependency of this increment.
