---
id: LANG-MOD-NAT-FLOOR-REALIZATION
title: "Realize Nat prelude-floor membership in the elaborator: admit the existing kernel {Nat, Zero, Suc} into the strict resolution floor, reusing the kernel identity (no second family), fail-closed on non-canonical origin, zero trusted_base() delta. The build half of the Nat prerequisite."
status: draft
owner: language
size: M
gate: none
depends_on: [LANG-MOD-NAT-PROVIDER-INTERFACE]
blocks: [LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Steward-filed 2026-08-25 as the build half of the operator-ruled Nat prelude-floor approach (Decision dec_1kqwn6hdvn7d2), split from the reframed spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]]. Under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # DRAFT — HELD on the spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]]
>
> Release only after the spec WP's normative amendments land (the amended
> `30-taxonomy §4` membership rule + coupled sections + the specified conformance
> pins). This node executes those pins in the elaborator; it does not decide the
> rule.

## Objective

Make `Nat` (and its constructors `Zero`/`Suc`) resolve in a strict root-loaded
scope to the EXACT pre-existing kernel identities, by admitting them into the
strict resolution floor established by [[LANG-MOD-STRICT-RESOLUTION]] (WP-2).
Reuse the kernel-checked `{Nat, Zero, Suc}` (created before source by
`register_prelude`) — allocate no second family, no new `Decl`/`GlobalId`, and
add zero `trusted_base()` entries. Non-canonical origins (a source-level
redeclaration of `Nat`) fail closed: they do not satisfy the identity.

## The seam (grounded by WP-2, re-measure at release)

WP-2 landed the strict floor as a fixed name set + admission filter in
`crates/ken-elaborator/src/modules.rs`: `PRELUDE_FLOOR_NAMES = [Bool, Char,
List]`, installed by `install_prelude_floor`, with `capture_strict_builtin_names`
admitting trusted native names + closed-floor constructors, and strict
`resolve_ref` fail-close. Today `resolve_ref` REJECTS `Nat` under Strict, and
`lang_mod_catalog_realization.rs:81-117` pins that rejection. The realization
extends the floor to include `Nat` (+ `Zero`/`Suc`), binding the names to the
existing kernel GlobalIds — not a re-declared family. D0 of the spec WP confirms
those identities are reachable at this seam.

## Deliverables

- D0 (buildability probe). Confirm admitting `{Nat, Zero, Suc}` to the strict
  floor binds the existing kernel GlobalIds (reuse, not mint) at the WP-2 seam,
  under the amended rule.
- D1. Extend the strict floor / admission to include `Nat` and its constructors;
  flip the strict-Nat control (see AC-2); satisfy the spec WP's conformance pins.

## Acceptance criteria

Inherits the spec WP's AC-1..AC-5 as its executable target, plus:

- AC-2 (the flip, restated for the builder). Re-author
  `lang_mod_catalog_realization.rs:81-117` from asserting strict-REJECT of Nat to
  asserting strict-ACCEPT + the AC-1 exact-GlobalId identity. This red-to-green
  flip is the port landing; it is NOT a regression and must be called out to QA
  as the pre-registered inversion.
- AC-IDENTITY. A second, source-level `data Nat = Zero | Suc Nat` in a unit
  resolves to a DISTINCT family and does not collide with / shadow the floor Nat.
- AC-NO-REGRESSION. Whole-suite green in CI; the WP-2 strict-resolution controls
  (`n2_in_repo_loader.rs`, `n3_import_exclusion.rs`, `l_resolver_globals.rs`
  legacy) stay green; flat-Σ pin green; local targeted `-p` only.

## Reviewers

conformance-validator (identity + accept/reject discriminators with the new
closed floor) + Architect (the identity reuse must not grow the TCB and the
legacy passthrough must be untouched).

## Capability tier

T1 (a soundness-bearing floor extension threading kernel-identity reuse through
the strict admission seam; the fail-closed-on-redeclaration invariant is
load-bearing). Size M — smaller than the spec WP; it executes a specified pin
set against a seam WP-2 already built.
