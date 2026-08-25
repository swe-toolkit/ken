---
id: LANG-MODULE-IMPORT-SYSTEM
title: "Module/import capability campaign — declaration visibility (public export), a selective-import surface, and cross-package plus prelude symbol resolution, sufficient for catalog packages to reuse canonical modules instead of reimplementing them"
status: draft
owner: language
size: XL
gate: none
depends_on: []
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Operator directive 2026-08-22: module/import is its own campaign and, because it blocks the foundation catalog trial, ranks above the smaller language surface-syntax items. Framing is routed to the spec enclave (spec-surface) plus the Architect (elaborator/loader component-design); the language ring builds it. Steward-filed per COORDINATION section 2."
---

> # CAMPAIGN ROOT — framing COMPLETE (2026-08-23); member WPs filed, release HELD
>
> The enclave + Architect framing is delivered and the member WP nodes are filed:
> - Spec surface: MERGED 2026-08-23 at `def16ecf4` (PR #2812, candidate
>   `2711fc566a9af2195d29f24a2de6024a0d2b6495`) — spec/30-surface 32-grammar +
>   33-declarations + 39-elaboration plus the bounded module-conformance fold
>   (conformance/README + seed-modules.md). Approvals on exact 2711fc: Architect
>   soundness (evt_1qef8ewzc13yx), spec-author independent conformance fidelity
>   (evt_9g2ggrmmetkb), CV /spec fidelity carrying (byte-identical /spec tree);
>   Decision dec_67cjs8p38k904. This is the SPEC-SURFACE half; the build WPs below
>   implement it and are still held.
> - Component-design + 4-WP decomposition: Architect evt_hpnhqy1ex286, with the
>   WP-2 keying REVISED to strict-all-root-loaded in evt_xtscdw8r3q3k (see below).
>
> MEMBER WPs (Steward-filed, this campaign):
> - [[LANG-MOD-LOADER-ENTRY]] (WP-1) — route `ken check` catalog through the
>   loader; behavior-preserving, non-strict. Independent, can lead.
> - [[LANG-MOD-PUB-ELIGIBILITY]] (WP-3) — semantic gate rejecting ineligible
>   `pub` placements. Independent, can lead.
> - [[LANG-MOD-STRICT-RESOLUTION]] (WP-2) — strict root-loaded resolution (the
>   soundness core); D0 probe then D1. Rides on WP-1 for catalog observability.
> - [[LANG-MOD-CATALOG-REALIZATION]] (WP-4 Component A) — module-graph/roots
>   loader realization + provider public surface (the self-contained green set
>   checks standalone-strict; Arithmetic/Order providers made public). Gated on
>   WP-1..3 AND the Or realization chain. RE-FRAMED 2026-08-24 (hard stop): `Nat`
>   has no catalog home, so the consumer migration + whole-catalog strict co-gate
>   split off to Component B (Architect ruling evt_214z6r6qnwme0). MERGED
>   2026-08-24 at `574eb90c0` (auto-closed ee2631ff8); reduced to the loader fix
>   + Arithmetic-only provider surface after the HS#5 convergence.
> - [[LANG-MOD-CATALOG-COMPLETENESS]] (WP-4 Component B, RELEASED 2026-08-24) —
>   one canonical `Nat` AND `OrdResult` home (+ other convenience homes via the
>   fixpoint census), Order's provider surface + identity, migrate
>   Arithmetic/Order/Gcd to import their providers, whole-catalog strict-green.
>   The catalog-reuse SUCCESS step; depends on Component A. Closing B closes the
>   campaign's catalog criterion.
>
> B PREREQUISITE NODES (discovered during Component B build, 2026-08-24 — two
> structural mechanism/contract gaps the strict-migration surfaced plus one
> formatter-surface gap the respin CI surfaced; §1b full-closure now governs the
> remaining cluster, Steward evt_21bem2w7rzj2k):
> - [[LANG-MOD-NAT-PROVIDER-INTERFACE]] (spec WP) +
>   [[LANG-MOD-NAT-FLOOR-REALIZATION]]
>   (build WP) — Nat's canonical home by PRELUDE-FLOOR MEMBERSHIP. Decision
>   dec_1kqwn6hdvn7d2 RESOLVED (2026-08-25): the operator ruled the prelude
>   membership rule (`30-taxonomy §4`) the defect and superseded the earlier
>   provider-registry mechanism; realize by amending the general membership rule
>   (bootstrapping arm) and admitting the existing kernel {Nat,Zero,Suc} into the
>   strict floor, reusing identity. Blocks B's Nat criterion (AC-B5a) and
>   [[CAT-GCD-REFACTOR]]'s Nat import. Spec WP `ready` (release first); build WP
>   held on it.
> - [[LANG-MOD-ATTACHED-PROOF-OWNERSHIP]] — normative clarification that a
>   proof's attached namespace is closed under its subject's defining module (a
>   nonlocal attached head rejects), + conformance. Codifies EXISTING behavior;
>   does NOT gate B's build (B converts its two foreign attachments to
>   Lawful-local theorems, AC-B8). Enclave-owned, sequenced alongside B.
> - [[LANG-MOD-KENFMT-DECL-LAYOUT]] — kenfmt has no breakable layout for the
>   module/import declaration surface (export/import name lists render as a flat
>   unbreakable run; a >96-col line cannot reflow), so the Component B respin's
>   "run the formatter" fix is invalid for its width failure. Language-owned,
>   buildable now. Its `blocks` edge onto B is set by the ring's determination of
>   whether B can author its module-surface decls idempotent-and-<=96 without the
>   fix. Steward-confirmed from the Doc-construction; no `trusted_base()` impact.
>
> OR REALIZATION CHAIN (operator ruled the Or/Inl/Inr fork arm (b),
> evt_6b9wrt1kwswcp — canonical package home, not refactor-away; the six consumers
> and the proof-relevant `total_leq_nat` reuse made refactor-away the wrong call):
> - [[LANG-MOD-OR-OMEGA-PARAM-ELAB]] (NODE A) — teach explicit-data param/index
>   elaboration to honor an Omega-sorted binder (Architect finding
>   evt_21gve67p385jh; enclave sort-discipline GO evt_3j02n0pkgze3a). Buildable now.
> - [[LANG-MOD-OR-CANONICAL-HOME]] (NODE B) — author `Core.Logic.Or`, migrate the
>   six consumers, retire the prelude registration. Feeds WP-4.
>
> Ring order: WP-1 -> WP-3 -> WP-2 -> WP-4 (or WP-2's D0 parallel with WP-1/WP-3).
> Or chain sequenced NODE A -> WP-2 D1 -> NODE B, all before WP-4.
> Every WP: Architect (soundness/component fit) + conformance-validator
> (resolution/visibility discriminators). Cross-cutting invariant on every WP:
> flat-Σ / zero trusted_base delta (extend `module_elaborates_to_identical_flat_
> sigma`, never weaken it).
>
> THE REVISED STRICT/LEGACY KEY (Architect evt_xtscdw8r3q3k — supersedes the
> boundary-header key in evt_hpnhqy1ex286): strict resolution is keyed on
> ROOT-LOADED (loader use), not on a package boundary header. `/spec` candidate
> 860c605 makes every root-loaded unit strict; the Architect aligned to it (the
> boundary-header carve-out would introduce a third mode the spec does not have).
> Mode threads from the ENTRY: `elaborate_module_from_roots` ⇒ strict;
> `elaborate_file`/`elaborate_ken_md_file` (isolated-file) ⇒ legacy passthrough
> verbatim. The flag-day is SEQUENCING, not impossibility: co-gate WP-2's
> CI-greenness with the catalog migration, and make the migration set
> CENSUS-driven (every catalog file the strict flip breaks, measured — not just
> the Gcd trio). NOTE 2026-08-24: that migration + strict co-gate is Component B
> ([[LANG-MOD-CATALOG-COMPLETENESS]]), not Component A — the hard stop showed the
> census population needs a canonical `Nat` home first (34 baseline-red, not 32).
>
> RELEASE GATE (all members): held until the language ring FINISHES
> embedding-adequacy ([[V3-FO-EMBEDDING-ADEQUACY]]) per operator finish-then-
> switch — do NOT interrupt the in-flight WP. WP-4 additionally gated on the
> Or/Inl/Inr fork (escalated to operator, evt_6b9wrt1kwswcp) and on WP-1..3. The
> wp/ frames are authored at release time grounded on the MERGED spec SHA (now
> landed at def16ecf4), so WP-2/WP-3 conformance ACs cite CV's landed fold.

## The measured gap (Steward, on main 2026-08-22)

The catalog-reusability standard requires a catalog entry to IMPORT canonical
operations rather than reimplement them. Ken cannot do that today:

- No public-export / import surface. `add`/`mul`
  (`catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md:16,22`) and
  `leq_nat`/`sub` (`Order.ken.md:37,208`) are plain `fn`s — no `pub` marker — and
  there is no `import`/`use` statement anywhere in `catalog/packages/`. Each
  package elaborates as an island. No module-visibility surface exists under
  `spec/30-surface/`.
- No cross-package / prelude symbol resolution for standalone checking.
  `Arithmetic.ken.md`'s own proofs call `cong` (l.33/41/50...), which has no
  definition in `catalog/packages/`; checked in isolation the module fails
  `UnresolvedCon { name: "cong" }`. So even with an import keyword, a dependency
  module does not resolve on its own to be depended upon.

## Objective

Give Ken a module system sufficient for catalog reuse:

1. Declaration visibility — public export markers.
2. A selective-import surface — grammar and elaboration to bring named public
   declarations from one package into another.
3. Cross-package + prelude symbol resolution, so a package can both import
   canonical operations and resolve shared lemmas.

Success criterion: a catalog entry (`Gcd`) imports `add`/`mul` from
`Data/Numeric/Nat/Arithmetic` and `leq_nat`/`sub` from `Data/Numeric/Nat/Order`
instead of reimplementing them, and both dependency modules check.

## Framing split (who designs what)

- Spec enclave (spec-leader/spec-author): the spec surface — `spec/30-surface/`
  `33-declarations` (visibility), `32-grammar` (import syntax), `39-elaboration`
  (resolution/loader semantics). Likely one or more spec-enclave-owned member
  WPs.
- Architect: the elaborator/loader component-design and the WP decomposition.
- Steward: files the member WP nodes from the decomposition; sequences the build.

## Sequencing (operator directive)

Frame the campaign with the enclave WHILE the language ring continues its current
work; do NOT interrupt the in-flight FO re-elaboration
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]]). When the campaign is READY (framed, WPs
shovel-ready), the language ring's then-current WP finishes and the ring switches
to this campaign AHEAD of the remaining surface-syntax items
([[LANG-SYMBOLIC-OPERATOR-NAMES]] and siblings). This campaign unblocks the
factored catalog trial ([[CAT-GCD-REFACTOR]], and the remaining CAT-* trial WPs
authored factored).
