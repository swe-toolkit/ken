---
id: LANG-MOD-OR-CANONICAL-HOME
title: "Or arm (b), NODE B — author Core.Logic.Or (data Or (a:Omega)(b:Omega):Type with Inl/Inr, field-for-field the prelude shape), migrate the six catalog consumers to import it, and retire the Rust prelude Or/Inl/Inr registration so exactly one catalog Or identity exists"
status: ready
owner: language
size: M
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
> origin/main 3af44ffff by the Steward: NODE A merged (6fae7a918 -> main; Omega
> case at ken-elaborator/src/data.rs:633,648 makes `Or` spellable as a catalog
> data declaration), and STRICT-RESOLUTION D1 merged (5a74301f4 is an ancestor of
> main; strict resolution is the mode under which the six consumers resolve `Or`
> through a legal import). gate:none, no other gate remains. Released to the
> language ring. Capability tier T2 (mechanical authoring + six-consumer migration
> + prelude-registration removal; the soundness is carried by NODE A's Omega
> sort-admission and WP-2 strict resolution). Kicked by the Steward.

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
2. Migrate the six consumers to import `Core.Logic.Or` (public export on `Or`,
   `Inl`, `Inr`; selective import per consumer), under WP-2 strict resolution.
3. RETIRE the Rust prelude `Or`/`Inl`/`Inr` registration
   (`prelude.rs:1253/1262/1264`) so exactly ONE catalog identity exists. The
   catalog `Or` takes a FRESH GlobalId; it cannot adopt the prelude `or_id`.
   Retire ONLY AFTER the six imports land AND a grep confirms no unit still
   resolves `Or`/`Inl`/`Inr` through the legacy passthrough.

# Acceptance criteria

- AC-B1 (standalone). `Core.Logic.Or` checks standalone through the real loader;
  `Inl`/`Inr` carry the exact prelude field shapes (`a -> Or a b`, `b -> Or a b`).
- AC-B2 (one identity, by GlobalId). All six consumers resolve `Or`/`Inl`/`Inr`
  to the single `Core.Logic.Or` GlobalId — established by identity, not repo text
  — with no consumer-owned competing identity. The prelude registration is gone
  and no unit falls through to it (grep-verified plus the strict resolver
  rejecting the bare name absent the import).
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
