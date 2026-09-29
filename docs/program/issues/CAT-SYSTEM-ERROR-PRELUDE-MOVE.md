---
id: CAT-SYSTEM-ERROR-PRELUDE-MOVE
title: "Move the PX9 System.Error classification model (SystemError, Operation, ResourceRef, SafeContext, Transience, Idempotence, RetryGuidance, file_error_to_system, error_transience, operation_idempotence, retry_guidance) out of the prelude into a catalog package, with its two spec-normative laws as kernel-checked proofs; fifth L3 slice of the minimal-prelude program"
status: merged
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
  It names the surface once, in prose; it does not fix a catalog path. The
  existing System family (`System.IO`, `Buffer`, `Resource`) lives under
  `Capability/System/`.
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
that serves the spec's `System.Error` surface. It sits inside the chartered
Sections (`06-catalog-campaign.md`), beside the existing `System.IO` at
`catalog/packages/Capability/System/`. The Architect rules the exact path, and
there is no new top-level Section (Steward `evt_7k4ntmc8xem5y`). The package
proves the two §1.8 properties as checked theorems, not tests. The prelude
registers none of the eleven, and its PX9 bracket goes with them; the package
keeps the zero-trust property as its AC-1 row. Each reader imports the package
or is re-keyed to its owned ids.

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
  the stale labels and name the `Capability.System.Error` module (Architect
  `evt_1qk0gkr2tysw2`). That path belongs to the enclave; Foundation does not
  edit it.

## Closeout

Landed `e5cd36c54` (PR #4343; candidate `4ad7a039b`; Foundation QA
`evt_7jwzc7h5nr286`, Architect `evt_6qgpyh2yqeexa`, Decision
`dec_3633zj9xp2n4w`).
- The eleven declarations now live in `Capability.System.Error`
  (`catalog/packages/Capability/System/Error.ken.md`), beside `System.IO`,
  unchanged in meaning. The prelude registers none of them, and its PX9
  bracket is gone. The package adds zero trust.
- Both §1.8 laws are checked theorems. The converse,
  `retry_advised_only_for_transient_idempotent`, is proved by case analysis;
  `error_transience_revoked_permanent` keeps the prelude's `Revoked`.
- The three PX9 readers are re-keyed by owned id, one through a real
  selective import. The strict-resolution census gains the package's row.
- Controls: restoring the prelude block reddens the no-registration row. A
  wrong retry arm fails the converse and case theorems, and `Revoked ↦
  Transient` fails the Revoked law.
- Carries: the package's ambient `And`, `and_intro`, `Prod` and `MkProd`
  join `CAT-AND-SORTED-PRELUDE-MOVE`'s consumer inventory. The conformance
  seed refresh landed `0eaf30352` (PR #4349; Spec APPROVE
  `evt_2rs5ptxrf39gb`, Decision `dec_485sgghjdrwfn`). It is
  conformance-only, and the revoked-single-identity case is gated on the
  `ResourceError.Revoked` versus `ResourceHostIO Revoked` question. The Revoked law sits before
  `error_transience` on purpose; forward references to functions check.
