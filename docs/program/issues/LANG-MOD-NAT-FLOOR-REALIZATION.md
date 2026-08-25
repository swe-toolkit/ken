---
id: LANG-MOD-NAT-FLOOR-REALIZATION
title: "Realize the landed nine-name prelude floor in the elaborator, BOTH halves: (a) admission — extend the strict resolution floor from the implemented {Bool, Char, List} to the landed closed set {Auth, Bool, Char, List, Nat, Option, ResourceKind, Result, Utf8Error}, binding each name to its EXISTING GlobalId (no new family/Decl/GlobalId, zero trusted_base() delta); and (b) immutability — fail closed on every same-spelling floor parent OR constructor collision with AmbiguousReference BEFORE any allocation, in every top-level module scope regardless of prefix, qualified/renamed access the lawful escape. The build half of the Nat prerequisite."
status: merged
owner: language
size: M
gate: none
depends_on: [LANG-MOD-NAT-PROVIDER-INTERFACE]
blocks: [LANG-MOD-CATALOG-COMPLETENESS]
github: null
origin: "Steward-filed 2026-08-25 as the build half of the operator-ruled Nat prelude-floor approach (Decision dec_1kqwn6hdvn7d2), split from the reframed spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]]. Under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # MERGED 2026-08-25 at squash `d5c41ec1` (PR #2917)
>
> The transparent-Pub evidence candidate `a9233e9dcf7a4bcb7b5f3b738ebcf55c385e9c05`
> (tree `179f90ea`, 8 `crates/ken-elaborator` paths, +1492/-207) merged. All
> three exact-SHA domain gates approved: language-qa `evt_2ema4k3m93jrc`, CV
> `evt_65rnja90g62vk`, Architect component-fit `evt_10zrdx6rt7vb3` (the Architect
> verdict discharged the transparent-wrapper closure that caused the three prior
> Nat-floor rejections). Decision `dec_cer1cc2p5fwn` resolved; Steward blob-audit
> confirms all 8 paths byte-identical to the approved candidate. This discharges
> Component B's Nat criterion on the build side (the `blocks` edge onto
> [[LANG-MOD-CATALOG-COMPLETENESS]] is cleared) — B's remainder is now
> prerequisite-unblocked.

> # FRAME CORRECTED 2026-08-25 — floor-immutability half added (Architect reject)
>
> The Architect REJECTED the first respin `b6a576c63` (evt_wnjne3e48qbz): the
> nine-member admission and identity reuse are SOUND, but the candidate left the
> landed floor-IMMUTABILITY contract unimplemented and added a test that
> positively required the forbidden allocation. That was a frame/spec
> contradiction, not ring discretion — the prior AC-IDENTITY permitted a
> source-level `data Nat` to allocate a distinct `Entry.Nat`, which the landed
> spec forbids. The Steward owns the frame; this correction implements the
> landed contract before the ring respins. Any respin voids the prior verdict.
>
> What changed: AC-IDENTITY (allocate-distinct) is REPLACED by AC-FLOOR-IMMUTABILITY
> (reject same-spelling before allocation, the landed one-axis matrix); D2 (the
> fail-closed collision half) is added; the objective/seam now name the
> whole-binding-set contract, not admission alone. The admission half (AC-NINE,
> AC-2 flip, zero-trust) the Architect approved is UNCHANGED.
>
> RELEASED for respin (kickoff evt_5wdckv9xbp11r stands; merge gate LIFTED per
> operator clearance of the 3->9). Normal build WP: candidate -> CV + Architect
> review -> merge.

## Objective

Realize the landed nine-name prelude floor as a whole-binding-set contract with
two halves.

Half (a) ADMISSION. Extend the strict resolution floor from the implemented
`{Bool, Char, List}` to the LANDED closed nine-name set `{Auth, Bool, Char,
List, Nat, Option, ResourceKind, Result, Utf8Error}` (landed `30-taxonomy §4` /
`spec/30-surface/33-declarations.md:250-266`). Each name binds its EXISTING
identity — the kernel-checked `{Nat, Zero, Suc}` (created before source by
`register_prelude`) for the bootstrap-arm member, and the existing
surface/stdlib identities for the eight signature-arm names — allocating no
second family, no new `Decl`/`GlobalId`, adding zero `trusted_base()` entries.

Half (b) IMMUTABILITY. The floor is not merely admitted, it is unshadowable.
Per landed `33-declarations.md:250-266`, prelude bindings are the immutable
primitive floor: a top-level local that clashes with the floor by spelling —
whether a floor PARENT (`Nat`, `Option`, …) or a floor CONSTRUCTOR (`Zero`,
`Some`, `Err`, … whose kernel-recorded parent is a floor parent) — must fail
closed with `AmbiguousReference` naming the retained floor spelling, BEFORE any
declaration or `GlobalId` is allocated, in EVERY top-level module scope
regardless of module prefix. Qualified or renamed access remains the lawful
escape; an all-renamed same-shape family is accepted with distinct local ids.
A source-level `data Nat = …` does NOT allocate a distinct `Entry.Nat`; it is
rejected. This is the half the first respin omitted.

## The seam (grounded by WP-2 + the landed spec; re-measure at release)

Admission seam (built): `crates/ken-elaborator/src/modules.rs`
`PRELUDE_FLOOR_NAMES = [Bool, Char, List]`, installed by `install_prelude_floor`,
`capture_strict_builtin_names` deriving admitted constructors by exact kernel
parent, strict `resolve_ref` fail-close. `lang_mod_catalog_realization.rs:81-117`
pins the strict-`Nat` REJECT (flipped to ACCEPT by AC-2).

Immutability seam (the omitted defect, from the Architect reject): the collision
guard at `modules.rs:1676-1683` rejects a floor-name local ONLY when
`prefix.is_empty()`. Strict roots elaborate a unit under a non-empty module
prefix such as `Entry`, so `data Nat = …` bypasses the guard and binds
`Entry.Nat` locally. The constructor loops at `modules.rs:1687-1698` call
`bind_local` without consulting the floor binding set at all, so a renamed
parent carrying a constructor spelled `Zero`/`Some`/`Err`/… shadows the
canonical floor constructor. Conformance
`conformance/surface/modules/seed-modules.md:564-592`
(`prelude-floor-clash-and-lookalike-matrix`) records this exactly: "current root
loading still admits and shadows a same-spelling floor declaration" is the
implementation defect this realization must redden on.

## Deliverables

- D0 (census FIRST). At the admission seam, census which of the landed nine are
  already admitted vs missing (expected missing: `Nat` + `Auth`, `Option`,
  `ResourceKind`, `Result`, `Utf8Error` — re-measure). Confirm each missing name
  (and each inductive member's constructors) binds its EXISTING GlobalId (reuse,
  not mint). Output: the exact admission gap D1 closes.
- D1 (admission). Extend `PRELUDE_FLOOR_NAMES` + the admission filter to the
  landed nine, binding each name and its constructors to existing GlobalIds; flip
  the strict-`Nat` control (AC-2); satisfy the landed admission pins. Add no new
  family/`Decl`/`GlobalId`; zero `trusted_base()` delta.
- D2 (immutability — the fail-closed collision half). Per the Architect
  correction (evt_wnjne3e48qbz):
  1. Keep `PRELUDE_FLOOR_NAMES` exactly the nine TYPE names — do not list
     constructors there, do not widen to arbitrary globals.
  2. Derive a SEPARATE unshadowable floor-binding set from those exact parent
     `GlobalId`s plus only the constructor names whose kernel-recorded parent is
     one of them. Do NOT reuse all `strict_builtin_names` — that would conflate
     the closed floor with trusted/native vocabulary.
  3. Before allocation, make `prebind_scope_declarations` reject every top-level
     local binding in that set regardless of module prefix, including
     constructor bindings inside a renamed family. Apply the same binding set to
     selective-import collisions; qualified or renamed access remains lawful.

## Acceptance criteria

Inherits the spec WP's landed admission pins (AC-1 identity, AC-2 strict-accept,
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
- AC-FLOOR-IMMUTABILITY (REPLACES the prior AC-IDENTITY; the landed one-axis
  matrix from `seed-modules.md:564-592`). Using a fresh strict-roots environment
  per row, for each of the eight inductive floor parents:
  1. keep only the PARENT spelling canonical while renaming all its constructors
     — rejects with `AmbiguousReference` naming the retained floor spelling;
  2. in separate entries, keep only ONE CONSTRUCTOR spelling canonical while
     renaming the parent and all sibling constructors — rejects with
     `AmbiguousReference`;
  3. as the reaching positive, rename the parent and every constructor while
     preserving the same declaration shape — ACCEPTS with ids distinct from
     every floor id and each constructor's parent its renamed local former.
  For constructor-free `Char`: `def Char = Int` rejects; the same-production
  positive `def LocalChar = Int` accepts as a distinct checked transparent id.
  Every same-spelling reject fires BEFORE any declaration or `GlobalId` is
  allocated (assert unchanged declaration count / `next_global_id` / allocator at
  the reject); every row preserves `trusted_base()`. The reject must fire under a
  NON-EMPTY module prefix (e.g. `Entry`), pinning the `prefix.is_empty()` bypass
  closed. A generic `expect_err` or a single all-names-collide fixture does not
  satisfy this — the one-axis rows plus same-production positives are required.
- AC-NO-REGRESSION. Whole-suite green in CI; the WP-2 strict-resolution controls
  (`n2_in_repo_loader.rs`, `n3_import_exclusion.rs`, `l_resolver_globals.rs`
  legacy) stay green; flat-Σ pin green; local targeted `-p` only.

## Reviewers

conformance-validator (admission identity + the floor-immutability collision
matrix against the landed seed) + Architect (identity reuse must not grow the
TCB; the fail-closed collision must cover the whole binding set — nine parents
plus exact-parent-derived constructors — in every top-level scope, not just the
empty-prefix type-name case).

## Capability tier

T1 (a soundness-bearing floor realization: kernel-identity reuse through the
strict admission seam AND a fail-closed immutability invariant over the whole
binding set; both are load-bearing). Size M — it executes a specified pin set
against a seam WP-2 already partly built; the collision half is the added work.
