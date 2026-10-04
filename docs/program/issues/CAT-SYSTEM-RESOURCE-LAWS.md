---
id: CAT-SYSTEM-RESOURCE-LAWS
title: "System.Resource promises acquire-before-body, settle-after-body and a fixed body/release result ordering, but only two interpreter fixtures check it. Prove the bracket's sequencing and settlement ordering over the existing effect tree"
status: active
owner: foundation
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "The last unframed row of docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (Capability/System/Resource.ken.md), under operator ruling 2026-09-13 / PRINCIPLES #16. Framed by the Steward 2026-10-04 as L3's successor to CAT-JSON-LAWS. Measured at origin/main e771f7e7d."
---

# System.Resource bracket laws

## Objective

The guarantees `Capability/System/Resource.ken.md` states in prose are
checked theorems: the bracket acquires before its body and settles after it,
and the settled result is fixed by the body result and the release result.
No new trust.

## Settled inputs (measured at `e771f7e7d`)

- The bracket is prelude source, not package source
  (`crates/ken-elaborator/src/prelude.rs`). `withResource` (`:2363`) is a
  `Vis` on `PrivateFsOpen a cap path mode` whose continuation is
  `private_with_resource_after_open` (`:2340`). On `Err open_error` that is
  `Ret (Err open_error)`, and the body never runs. On `Ok resource` it is
  `bind (body resource) (λbody_result. release_if_live ...)`.
- `release_if_live` (`:2325`) is a `Vis` on `PrivateResourceRelease` whose
  continuation is `Ret (resource_settle_result_for o e r body_result
  settled)`.
- `resource_settle_result_for` (`:2278`) dispatches on the body result into
  `resource_settle_ok_for` / `resource_settle_err_for`, then
  `resource_settle_ok_error_for` (`:2214`) /
  `resource_settle_body_error_for` (`:2236`). A release error of `Closed`
  counts as already settled. Each of the other ten
  `ResourceError` constructors is carried into `ResourceBracketReleaseError`
  or `ResourceBracketBodyAndReleaseError`. Every arm returns `Ok`.
- `withBuffer` and `withMapping` share `release_if_live` and the settlement
  functions, so the settlement laws cover them as well.
- The package's own surface is three helpers, so the new theorems are
  private. Do not make anything new `pub`, and do not edit the prelude.
- The evidence being replaced is `px7f_system_resource.rs::{public_bracket_success_and_early_release_settle_once,caller_control_release_failure_preserves_success_and_body_error_ordering}`.
  Those tests stay.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Private theorems in `Resource.ken.md`, over the existing definitions:

1. **Settlement ordering**, for every `o e r`, and for every value or body
   error, and every settled result:
   - body `Ok v` with release `Ok` or `Err Closed` gives
     `Ok (ResourceBracketOk v)`;
   - body `Ok v` with release `Err x`, where `x` is not `Closed`, gives
     `Ok (ResourceBracketReleaseError x)`;
   - body `Err b` with release `Ok` or `Err Closed` gives
     `Ok (ResourceBracketBodyError b)`;
   - body `Err b` with release `Err x`, where `x` is not `Closed`, gives
     `Ok (ResourceBracketBodyAndReleaseError b x)`.

   State "not `Closed`" as `Not (Equal ResourceError x Closed)` and case on
   the bound `x` (CHECKS 11).
2. **Sequencing.**
   - `withResource` equals the open `Vis` continued by
     `private_with_resource_after_open`.
   - On `Err open_error`, after-open is `Ret (Err open_error)`, so no body
     and no release.
   - On `Ok resource`, after-open is `bind (body resource)` into
     `release_if_live` on that same resource.
   - `release_if_live` is the release `Vis` continued by `Ret` of the
     settlement.

Update the package prose to cite the theorems where it states each guarantee.

## Acceptance

- **AC-1.** The package checks with the new theorems at the default 2 MiB
  test worker. `trusted_base()` is unchanged. A focused test resolves each
  theorem as a registered private kernel global with its exact checked
  statement.
- **AC-2 (falsifiers, prelude mutations restored byte-identically).**
  - M1: map `Closed` to `ResourceBracketReleaseError Closed` in
    `resource_settle_ok_error_for`, and the body-ok/`Closed` law reddens.
  - M2: drop `body_error` in `resource_settle_body_error_for`'s
    `ResourceHostIO` arm (return `ResourceBracketReleaseError`), and the
    body-and-release law reddens.
  - M3: continue after-open's `Ok` arm with `release_if_live` before
    `body`, and the sequencing law reddens.

## Stop conditions

- A prelude name the statements need (`private_with_resource_after_open`,
  `release_if_live`, `PrivateFsOpen`, `PrivateResourceRelease`, a settlement
  function) cannot be named from package source under the current resolution
  mode: stop to the Architect. Do not widen visibility.
- Stating or checking a sequencing equation hits an elaborator or kernel gap
  (for example conversion of a `proc` body or a `Vis` continuation): stop to
  the Architect with the failing declaration.
- Out of scope: runtime generation liveness, rights, revocation, controlled
  traps and exactly-once settlement. These are TCB contracts, not Ken proof
  targets.
