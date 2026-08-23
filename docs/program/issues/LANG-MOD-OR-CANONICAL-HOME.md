---
id: LANG-MOD-OR-CANONICAL-HOME
title: "Or arm (b), NODE B — author Core.Logic.Or (data Or (a:Omega)(b:Omega):Type with Inl/Inr, field-for-field the prelude shape), migrate every checked-source consumer to import it, and retire the Rust prelude Or/Inl/Inr registration so exactly one catalog Or identity exists"
status: ready
owner: language
size: L
gate: none
depends_on: [LANG-MOD-OR-OMEGA-PARAM-ELAB, LANG-MOD-STRICT-RESOLUTION]
blocks: [LANG-MOD-CATALOG-REALIZATION]
github: null
origin: "Operator ruled the Or/Inl/Inr fork arm (b) — canonical package home, not refactor-away (evt_6b9wrt1kwswcp). Architect NODE-B framing evt_21gve67p385jh. Home = Core.Logic (its own .ken.md). Steward-filed per COORDINATION section 2, 2026-08-23, under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # RELEASED 2026-08-23 — both dependencies landed; flipped draft -> ready.
>
> Architect sequence [[LANG-MOD-OR-OMEGA-PARAM-ELAB]] (NODE A) ->
> [[LANG-MOD-STRICT-RESOLUTION]] D1 -> NODE B is satisfied, both verified on
> origin/main by the Steward: NODE A merged (6fae7a918 -> main; Omega case at
> ken-elaborator/src/data.rs:633,648 makes `Or` spellable as a catalog data
> declaration), and STRICT-RESOLUTION D1 merged (5a74301f4 is an ancestor of
> main; strict resolution is the mode under which the six consumers resolve `Or`
> through a legal import). gate:none, no other gate remains. Released to the
> language ring. Capability tier T2 (mechanical authoring + six-consumer migration
> + prelude-registration removal; the soundness is carried by NODE A's Omega
> sort-admission and WP-2 strict resolution). Kicked by the Steward.

> # SCOPE AMENDED 2026-08-23 — census corrected six -> seven consumers.
>
> The build hard-stopped: the frame's "exactly six consumers" premise is false on
> current main. The census keyed on `.ken.md` and missed plain `.ken`, so it
> omitted a SEVENTH consumer — `catalog/packages/Tooling/Verification/FoKripke.ken`
> (line 316, `FokScopedOr p q ↦ ‖Or ...‖`, plus a later `Inr`), which today
> resolves bare `Or` to the one prelude registration. Retiring the prelude with it
> unmigrated leaves 12 FoKripke/V3 binaries failing `UnresolvedCon("Or")`.
> STEWARD RULING (scope, within the operator's already-ruled arm (b); not a new
> design fork): option (A) — migrate FoKripke to `import Core.Logic.Or (Or, Inl,
> Inr)`, single canonical identity preserved. Option (B) (a private FO Or identity)
> is REJECTED: FoKripke uses the same proof-relevant Or today, so (B) would invent
> a second identity, contradicting arm (b) and tripping invention-in-costume.
> The Steward re-censused the whole tree (all extensions): seven checked-source
> consumers, no eighth (`library/guide/surface-reference.ken.md:411` is prose in
> inline backticks, not a checked unit — but the guide corpus is gated, so the
> predicate AC-B2 must clear it too). Deliverable 2, deliverable 3, and AC-B2 are
> amended below; AC-B2 is reframed from a count to a predicate so a miscount can
> never again be a scope landmine. Size bumped M -> L; tier stays T2.

# Why arm (b), not refactor-away (the census that decided it)

`Or`/`Inl`/`Inr` are used across SIX catalog packages
(`Core.Classes.LawfulClasses` heavily, `Core.Logic.EmptyDec`,
`Data.Collections.Derived`, `Data.Collections.Map`, `Capability.Formatting.Doc`,
`Data.Numeric.Nat.Order`), defined in NONE (only the Rust prelude,
`prelude.rs:1253/1262/1264`). `Order.ken.md`'s `total_leq_nat` returns a
proof-relevant `Or` (which side holds is recoverable; it CANNOT live at Omega, and
`IsTrue (bool_or ...)` is strictly weaker — it forgets the side), and that
proof-relevant helper is reused verbatim in `Data.Collections.Map`. So `Or` is a
corpus-wide proof-relevant primitive; refactoring it to a Bool formulation would
erase the branch tag six places. The operator ruled arm (b): give it a canonical
home.

# Deliverable

1. Author `catalog/packages/Core/Logic/Or.ken.md`:
   `data Or (a:Omega)(b:Omega):Type where {Inl : a -> Or a b; Inr : b -> Or a b}`
   — field-for-field the prelude shape, so all six consumers typecheck unchanged.
2. Migrate EVERY checked-source consumer of `Or`/`Inl`/`Inr` to import
   `Core.Logic.Or` (public export on `Or`, `Inl`, `Inr`; selective import per
   consumer), under WP-2 strict resolution. The complete known population is
   SEVEN: the six catalog `.ken.md` packages plus the plain-`.ken`
   `catalog/packages/Tooling/Verification/FoKripke.ken`. The census must cover
   BOTH extensions across the whole tree — re-run it to confirm none remain.
3. RETIRE the Rust prelude `Or`/`Inl`/`Inr` registration
   (`prelude.rs:1253/1262/1264`) so exactly ONE catalog identity exists. The
   catalog `Or` takes a FRESH GlobalId; it cannot adopt the prelude `or_id`.
   Retire ONLY AFTER every consumer import lands AND a tree-wide grep confirms no
   unit still resolves `Or`/`Inl`/`Inr` through the legacy passthrough. This
   retirement is ATOMIC with the imports in one increment — the 12 FoKripke/V3
   `UnresolvedCon` reds prove the prelude cannot be retired with any consumer
   unmigrated.

# Acceptance criteria

- AC-B1 (standalone). `Core.Logic.Or` checks standalone through the real loader;
  `Inl`/`Inr` carry the exact prelude field shapes (`a -> Or a b`, `b -> Or a b`).
- AC-B2 (one identity, by GlobalId — a PREDICATE, not a count). EVERY
  checked-source consumer of `Or`/`Inl`/`Inr` resolves to the single
  `Core.Logic.Or` GlobalId — established by identity, not repo text — with no
  consumer-owned competing identity. After prelude retirement, NO unit resolves
  the bare name absent a legal import, enforced by (i) the strict resolver
  rejecting the bare name and (ii) a tree-wide grep (all `.ken`/`.ken.md`,
  catalog + guide) showing zero residual bare-`Or` consumers lacking the import.
  The consumer count (seven, currently) is descriptive; the predicate is the gate,
  so a miscount cannot silently pass.
- AC-B3 (proof-relevance preserved). `total_leq_nat` and its
  `Data.Collections.Map` reuse still return the proof-relevant `Or` (`Inl`/`Inr`
  distinguishable — a case-split recovers the side); not collapsed to a
  proof-irrelevant / Omega-valued form. This is the property the whole fork exists
  to protect.
- AC-B4 (zero trust — cross-cutting invariant). Zero `trusted_base()` delta; the
  catalog `Or` allocates the same flat-Sigma shape the prelude `Or` did; the
  `module_elaborates_to_identical_flat_sigma` pin is extended, never weakened.
- AC-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.

# Reviewers

Architect (identity/component fit — one `Or`, no invented identity) +
conformance-validator (identity-preserving import resolution) + Adversary
(invention-in-costume: the migrated `Or` must be the SAME proof-relevant sum, not
a proof-irrelevant look-alike that happens to typecheck the six consumers).

# Capability tier

T2 for the mechanical authoring + six-consumer migration + registration removal;
the SOUNDNESS is carried by NODE A's Omega-parameter sort admission and WP-2's
strict resolution. Size M.
