---
id: LANG-MOD-CATALOG-COMPLETENESS
title: "WP-4 Component B — catalog completeness: give Nat and OrdResult (dedup two private copies) canonical public homes plus the fixpoint homeless-convenience census, deliver Order's provider surface + identity, migrate the consuming units (Gcd imports add/mul + leq_nat/sub + Nat and drops its reimplementations), and satisfy whole-catalog strict-green. The module/import campaign's catalog-reuse success step."
status: draft
owner: language
size: L
gate: none
depends_on: [LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Architect ruling evt_214z6r6qnwme0 (2026-08-24), unbundling the WP-4 strict whole-catalog co-gate off Component A. The co-gate gates a deliverable outside WP-4's authorized surface (a canonical home for the type Nat), so it is re-homed here. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # DRAFT — held on Component A; released when A lands.
>
> Component B carries the substance the WP-4 hard stop exposed
> (language-implementer evt_3j60e77n0ahsy, Architect ruling evt_214z6r6qnwme0):
> the catalog cannot go whole-catalog strict-green while the type `Nat` (and
> other prelude conveniences the 42 red leaves consume) has no canonical catalog
> home. Component A delivers the loader realization + the provider public surface;
> B releases the missing homes and migrates the consumers. It depends on A (the
> providers must be public before consumers migrate) and is released once A lands.
>
> This is the module/import campaign's catalog-reuse SUCCESS step: when B lands
> its whole-catalog strict-green, the campaign root [[LANG-MODULE-IMPORT-SYSTEM]]
> can close and foundation [[CAT-GCD-REFACTOR]] unblocks.

# Objective

Catalog completeness under strict resolution: every convenience the catalog uses
resolves from a defining public interface (no ambient), and the whole catalog is
strict-green through the real loader.

# Deliverable

- ONE canonical `Nat` — `data Nat` with `Zero`/`Suc`, exported from a SINGLE
  defining public interface at the right catalog location, imported by
  Arithmetic/Order/Gcd and every other consumer. Subsume-don't-proliferate: one
  canonical home, the NODE B (`Core.Logic.Or`) canonical-home pattern — NOT
  per-consumer copies, NOT ambient, NOT an invented identity (Architect design
  constraint, ruling point 5).
- ONE canonical `OrdResult` (Architect HS#5 ruling evt_613d9fm7j45qj) — `data
  OrdResult = Lt | Eq | Gt` plus `ord_eq`/`ord_lt`/`ord_gt`, in a SINGLE defining
  public interface, DEDUPing the two private competing declarations at base
  (Data.Collections.Derived.ken.md:69 with its ord_* constants at :71/:73/:75,
  and Data.Numeric.Nat.Order.ken.md:188). One home serving BOTH
  `Data.Numeric.Nat.Order` AND `Data.Collections.Derived` (list/string/char
  compare) — a SHARED location both import (likely a Core/shared home, not under
  Numeric or Collections; B's placement call). Same charter/class as the
  canonical `Nat` home; migrate Order, Derived, and LawfulClasses to import it.
- Canonical public homes for the OTHER homeless conveniences the provider +
  consumer closure requires — census-driven, see the homeless-convenience closed
  predicate below.
- Order's PROVIDER SURFACE (moved from Component A, HS#5 — Order is not
  self-measurable in A because its closure needs homeless `OrdResult`): make
  `leq_nat`/`sub` `pub`; add Order's provider-internal imports `import
  Core.Logic.Transport (cong, trans)` (retaining Or) and `import
  Core.Classes.LawfulClasses (IsTrue, bool_or, Ord)`; make Transport's cong/trans
  and LawfulClasses's IsTrue/bool_or/Ord `pub` as needed. (Arithmetic's provider
  surface — Transport import + pub add/mul — is delivered by Component A and
  carries forward; B does NOT re-add it.)
- Migrate the consuming units to import the canonical `Nat` (and `OrdResult`)
  homes so they resolve under strict. Gcd selectively imports `add`/`mul`
  (Arithmetic) + `leq_nat`/`sub` (Order) + `Nat`, and REMOVES its four local
  reimplementations.
- Move the real caller to STRICT (`elaborate_module_from_roots` strict mode)
  after the dependency census has migrated — this is the flag-day the legacy A
  loader defers.
- HOMELESS-CONVENIENCE CENSUS as a CLOSED PREDICATE (Architect HS#5 ruling
  evt_613d9fm7j45qj — census all at once, not one hard-stop at a time). A name is
  a homeless convenience iff it is referenced within a genuine provider closure an
  AC requires, has NO defining PUBLIC interface in any catalog module, and is not
  native-prelude / floor {Bool,Char,List} / kernel. METHOD (mechanical): run the
  legacy roots-load of B's full provider+consumer closure to FIXPOINT, collecting
  EVERY `UnresolvedCon`/`UnboundName` that is not native/floor/kernel — that set
  IS the homeless census; author a canonical home for each. Known members:
  `OrdResult` (+ `ord_eq`/`ord_lt`/`ord_gt`). `Nat` is NOT homeless for the legacy
  path (native prelude) but IS a strict-home item here. Do not rediscover members
  one stop at a time.
- FORWARD-COMPAT identity preservation (Architect ruling evt_47t9dwz0chstv):
  strict excludes the native prelude `Nat`, so B re-homes `Nat` to a canonical
  catalog interface and migrates the providers to import it — but B MUST PRESERVE
  Arithmetic's provider identities (`add`/`mul`) that Component A's pub surface
  already exposed and measured under AC-A2, when it re-homes `Nat` and moves the
  caller to strict. No competing provider identity is minted by the `Nat`/
  `OrdResult` re-homes. This is the NODE B canonical-home pattern (as
  `Core.Logic.Or` replaced the prelude `Or`).

# The census (re-homed from WP-4; drift corrected)

The behavior census at base `f0e0b92fa` is 45 leaves = 3 strict-green (the
self-contained green set, delivered by Component A) + 42 red. WP-2 D0's recorded
premise of 32 baseline-red residuals is STALE: the current behavior census is 34
baseline-red residuals (language-implementer evt_3j60e77n0ahsy). Completion
ranges over the real population, not the stale count.

# Acceptance criteria

- AC-B1 (the re-homed co-gate). The whole catalog is strict-green in CI — the
  co-gate with [[LANG-MOD-STRICT-RESOLUTION]], whose remaining whole-catalog
  strict enforcement / CI closure co-closes here. Local targeted `-p` only;
  whole-catalog strict-green is a CI gate, never a local `--workspace` run.
- AC-B2. Arithmetic, Order, and Gcd each check STANDALONE through the real loader
  under strict (the AC-1 re-homed from WP-4 — now satisfiable because `Nat` and
  `OrdResult` resolve through their canonical homes).
- AC-B2a (Order provider identity — moved from A's AC-A2, HS#5). Order's
  `leq_nat`/`sub` FULLY elaborate and publish their genuine `GlobalId`s through
  the real loader, observed by IDENTITY (not repo text, not a frozen id, no
  competing identity) — measurable in B once Order's closure (Transport +
  LawfulClasses + canonical OrdResult) resolves.
- AC-B3 (residual triage). Completion ranges over the 34-residual population:
  each residual is either migrated to strict-green OR explicitly excluded with a
  stated reason. "Every census vector empty" is NOT sufficient — enumerate the
  disposition of all 34.
- AC-B4. Gcd's four imports resolve to the exact provider IDs with no Gcd-owned
  competing identity — establish no-reimplementation by IDENTITY, not repo text.
- AC-B5 (canonical-home identity). Exactly one `data Nat` AND exactly one `data
  OrdResult` exist in the catalog (one defining interface each); every consumer
  resolves `Nat`/`OrdResult` to it; the two private `OrdResult` copies at base are
  deduped away. No second identity, no ambient/floor `Nat`.
- AC-B7 (homeless census closed). The fixpoint homeless-convenience census (see
  the Deliverable) is run and its FULL set is enumerated with a canonical home
  authored for each — not rediscovered one hard-stop at a time. An empty
  next-iteration census is the completion signal.
- AC-B6 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-B-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.
- END-STATE INVARIANT (Architect ruling point 4). The strict contract is
  correct: every name resolves from a defining public interface, no ambient. B
  delivers that end state; it does NOT weaken strict to reach green.

# Reviewers

Architect (canonical-home component fit; one-canonical-Nat; no invented identity)
+ conformance-validator (identity-preserving import resolution; strict-green
census disposition).

# Capability tier

T1 for the canonical `Nat`-home design (the defining interface + its catalog
location, one-canonical-Nat) and the strict-green closure argument; T2 for the
mechanical convenience-home authoring + consumer migration + census execution.
Size L (may grow with the convenience-home breadth the 34 residuals require).
