---
id: LANG-MOD-NAT-FLOOR-REALIZATION
title: "Realize the landed nine-name prelude floor in the elaborator: extend the strict resolution floor from the implemented {Bool, Char, List} to the landed closed set {Auth, Bool, Char, List, Nat, Option, ResourceKind, Result, Utf8Error}, binding each name to its EXISTING GlobalId (kernel {Nat, Zero, Suc}; the five signature-arm names the implemented floor under-counted), no new family/Decl/GlobalId, fail-closed on non-canonical origin, zero trusted_base() delta. The build half of the Nat prerequisite."
status: ready
owner: language
size: M
gate: none
depends_on: [LANG-MOD-NAT-PROVIDER-INTERFACE]
blocks: [LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Steward-filed 2026-08-25 as the build half of the operator-ruled Nat prelude-floor approach (Decision dec_1kqwn6hdvn7d2), split from the reframed spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]]. Under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # RELEASED 2026-08-25 — spec WP landed (nine-floor); operator CLEARED the 3->9
>
> The spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]] MERGED (b7f73f1d): the amended
> `30-taxonomy §4` membership rule + coupled sections + conformance pins are on
> `main`, and the landed closed floor is the NINE-name set, not the four this
> node first drafted. This node executes those landed pins in the elaborator; it
> does not decide the rule.
>
> RELEASED (kickoff evt_5wdckv9xbp11r). The floor expansion — strict scope
> resolves all nine floor names ambiently, six more than the implemented
> `[Bool, Char, List]` — is a real behavior change; the Steward flagged it to the
> operator (Pat) and the operator CLEARED it this session. The merge gate is
> LIFTED: this proceeds as a normal build WP (candidate -> CV + Architect review
> -> merge), no operator hold. (The Steward ruled the expansion mechanical closure
> of dec_1kqwn6hdvn7d2, zero-TCB; the operator concurred.)

## Objective

Extend the strict resolution floor from the implemented `{Bool, Char, List}` to
the LANDED closed nine-name set `{Auth, Bool, Char, List, Nat, Option,
ResourceKind, Result, Utf8Error}` (landed `30-taxonomy §4`: "floor installation
reuses all nine existing `GlobalId`s and allocates nothing"). Each name binds to
its EXISTING identity — the kernel-checked `{Nat, Zero, Suc}` (created before
source by `register_prelude`) for the bootstrap-arm member, and the existing
surface/stdlib identities for the eight signature-arm names — allocating no
second family, no new `Decl`/`GlobalId`, and adding zero `trusted_base()`
entries. Non-canonical origins (a source-level redeclaration of a floor name, e.g.
a fresh `data Nat`) fail closed: they do not satisfy the identity. `Nat` (with
`Zero`/`Suc`) is the headline new admission; the five signature-arm names the
implemented floor under-counted (`Auth`, `Option`, `ResourceKind`, `Result`,
`Utf8Error`) are the rest of the gap.

## The seam (grounded by WP-2, re-measure at release)

WP-2 landed the strict floor as a fixed name set + admission filter in
`crates/ken-elaborator/src/modules.rs`: `PRELUDE_FLOOR_NAMES = [Bool, Char,
List]`, installed by `install_prelude_floor`, with `capture_strict_builtin_names`
admitting trusted native names + closed-floor constructors, and strict
`resolve_ref` fail-close. Today `resolve_ref` REJECTS `Nat` (and the other five
under-counted signature-arm names) under Strict, and
`lang_mod_catalog_realization.rs:81-117` pins the `Nat` rejection. The realization
extends `PRELUDE_FLOOR_NAMES` + the admission filter to the landed nine, binding
each name (and each inductive floor member's constructors, e.g. `Zero`/`Suc`) to
its existing GlobalId — not a re-declared family. D0 censuses exactly which of the
nine are already admitted vs missing at this seam before the flip.

## Deliverables

- D0 (buildability probe / census FIRST). At the WP-2 seam, census which of the
  landed nine floor names are ALREADY admitted by `PRELUDE_FLOOR_NAMES` +
  `capture_strict_builtin_names` and which are missing (expected missing: `Nat`
  and the five under-counted signature-arm names `Auth`, `Option`,
  `ResourceKind`, `Result`, `Utf8Error` — re-measure, do not assume). Confirm each
  missing name (and each inductive member's constructors) binds its EXISTING
  GlobalId (reuse, not mint) under the landed rule. Output: the exact admission
  gap the D1 flip must close.
- D1. Extend `PRELUDE_FLOOR_NAMES` + the admission filter to the landed nine,
  binding each name and its constructors to existing GlobalIds; flip the
  strict-Nat control (see AC-2); satisfy the landed conformance pins. Add no new
  family/`Decl`/`GlobalId`; zero `trusted_base()` delta.

## Acceptance criteria

Inherits the spec WP's landed acceptance pins (AC-1 identity, AC-2 strict-accept,
AC-3 closed-floor-not-catch-all, AC-4 zero-trust/zero-allocation) as its
executable target, plus:

- AC-2 (the flip, restated for the builder). Re-author
  `lang_mod_catalog_realization.rs:81-117` from asserting strict-REJECT of Nat to
  asserting strict-ACCEPT + the AC-1 exact-GlobalId identity. This red-to-green
  flip is the port landing; it is NOT a regression and must be called out to QA
  as the pre-registered inversion.
- AC-NINE (whole floor, not Nat alone). Strict resolution ACCEPTS each of the
  landed nine floor names to its exact existing GlobalId, and a non-floor
  kernel-provided-but-not-surface-required name still REJECTS strict (the
  closed-floor guard). The accept/reject discriminating controls extend to ALL
  added families (`Nat` and the five under-counted signature-arm names), not
  `Nat` alone — a per-name control, so silently dropping one family is caught.
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
