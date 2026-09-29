---
id: CAT-SYSTEM-ERROR-PRELUDE-MOVE
title: "Move the PX9 System.Error classification model (SystemError, Operation, ResourceRef, SafeContext, Transience, Idempotence, RetryGuidance, file_error_to_system, error_transience, operation_idempotence, retry_guidance) out of the prelude into a catalog package, with its two spec-normative laws as kernel-checked proofs; fifth L3 slice of the minimal-prelude program"
status: active
owner: foundation
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator rulings 2026-09-25 (minimal fixed prelude; convenience names are technical debt moved to packages) and 2026-09-13 (a catalog package is not finished until its proofs are complete). spec/30-surface/38-ffi-io.md §1.8 names the surface System.Error and makes its two classification properties normative. Steward-filed per COORDINATION section 2 as the L3 successor to CAT-PRELUDE-FOLD-REMOVAL."
---

# Move System.Error out of the prelude

## Objective

The PX9 error-classification model is a catalog package, not prelude
built-ins, and its two normative properties are proved in that package.

## Settled inputs (read at `a09f28843`)

- **The block.** `crates/ken-elaborator/src/prelude.rs:608-695`, inside the
  `px9_surface_trusted_before`/`after` zero-trust bracket (`:602`, `:696-705`),
  registers:
  - data: `Transience`, `Idempotence`, `RetryGuidance`, `Operation`,
    `ResourceRef`, `SafeContext`, `SystemError`;
  - functions: `file_error_to_system`, `error_transience`,
    `operation_idempotence`, `retry_guidance`.
- **None of it is on the floor.** The fifteen floor types
  (`spec/30-surface/30-taxonomy.md §4`) do not include it.
  `FileError`, `FileOperation` and `IOError`, which it reads, stay in the
  prelude.
- **The spec names the surface as a module.** `38-ffi-io.md §1.8` calls it
  "the cross-domain error surface (`System.Error`)". It makes two properties
  normative:
  - `RetryAdvised` comes only from `(Transient, Idempotent)`;
  - `error_transience Revoked = Permanent`.
  A catalog package with that module name matches the spec. It is not a spec
  change.
- **Consumers by spelling (to be confirmed by identity at AC-0).**
  - `crates/ken-elaborator/tests/px9_system_error_classification.rs`,
    `px9_file_error_to_system.rs` and `px9_revoked_unification.rs`;
  - the prose seed `conformance/surface/ffi-io/seed-error-classification.md`;
  - no catalog, `ken-runtime` or `ken-cli` reader, and no `PreludeEnv`
    capture.
  - `Idempotence` also appears as an unrelated word in
    `effects/row.rs:144` and a formatting seed.

## Deliverable

The eleven declarations move, unchanged in meaning, to a new catalog package
whose module path is `System.Error` (the package location is the owner's
call). The package proves the two §1.8 properties as checked theorems, not
tests. The prelude registers none of the eleven, and its PX9 bracket goes
with them; the package keeps the zero-trust property as its AC-1 row. Each
reader imports the package or is re-keyed to its owned ids.

## Acceptance

- **AC-0 (census, then ruling; no edit).**
  - List every reader of the eleven by resolved `GlobalId` across `catalog/`,
    `crates/*/tests`, `r_layer_tests`, `examples/`, `conformance/` and the
    CLI fixtures (Check 3).
  - Name any runtime, ABI or host key on any of them (Check 4).
  - The Architect rules the module path and the reader dispositions before
    any edit.
- **AC-1.**
  - `ElabEnv::new()` registers none of the eleven.
  - The package elaborates with zero growth of `trusted_base()`.
  - Its two law theorems check.
  - Every census reader is green on its import.
- **AC-2 (control).**
  - Restoring the prelude block turns the AC-1 "registers none" row red.
  - Each law theorem fails to check against a mutated definition: a
    `retry_guidance` that answers `RetryAdvised` for
    `(Transient, NonIdempotent)`, and an `error_transience` with
    `Revoked ↦ Transient`.
  - Prelude ids after the moved block shift, and every pin is re-keyed by
    identity, not by number.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A reader that needs any of the eleven in the prelude, such as a runtime,
  ABI or host key, is a stop to the Architect with its site.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

## After landing

- **Conformance carry, outside this WP** (Architect AC-0 ruling
  `evt_5vmakg3p8qb7d`). `conformance/surface/ffi-io/seed-error-classification.md`
  still carries `RED-UNTIL-BUILT` and `BLOCKED-ON-PX9-INC1` labels. When
  this WP lands, the Steward routes the refresh to the Spec enclave: clear
  the stale labels and name the `System.Error` module. That path belongs to
  the enclave; Foundation does not edit it.
