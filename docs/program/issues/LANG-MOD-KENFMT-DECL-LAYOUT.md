---
id: LANG-MOD-KENFMT-DECL-LAYOUT
title: "kenfmt breakable layout for the module/import declaration surface: Doc-builders for ExportDecl and ImportDecl (and selective-import / re-export name lists) so an overflowing name list wraps under CANONICAL_WIDTH and round-trips as a formatter fixed point. The Component B formatter-gate gap."
status: draft
owner: language
size: M
gate: none
depends_on: [LANG-MOD-CATALOG-COMPLETENESS]
blocks: []
github: null
origin: "Component B (LANG-MOD-CATALOG-COMPLETENESS) f5be017f7 CI red on two kenfmt gates (kenfmt_b3_layout::ac7 width-bound on a 125-col OrdResult export line; kenfmt_c_capstone canonical-fixed-point on LawfulClasses import/proof wrapping). Diagnosed candidate-only REAL by the language ring (evt_7exmbjsvfrk7f); Steward-confirmed as a formatter-surface gap by reading the Doc-construction code, not the symptom. Filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # FRAMED + SCHEDULED 2026-08-25 — LANG-MOD campaign tail (operator direction)
>
> WP frame authored: `docs/program/wp/LANG-MOD-KENFMT-DECL-LAYOUT.md`. Operator
> (2026-08-21) directed the two new LANG-MOD nodes be scheduled at the END of the
> module/import work — after Component B
> ([[LANG-MOD-CATALOG-COMPLETENESS]]) — and BEFORE the interrupted verify/z3
> FO-checker lane resumes
> ([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]] / [[V3-FO-EMBEDDING-ADEQUACY]]).
> `depends_on: [LANG-MOD-CATALOG-COMPLETENESS]` now encodes that schedule
> (technically buildable earlier, but sequenced here so D2 recombines B's
> already-landed stopgap decls). Stays `draft` until Steward releases it as the
> campaign tail. Not released yet.

> # DRAFT — buildable; the RECOMBINATION TRIGGER for Component B's split stopgap
>
> This node records a CONFIRMED gap: the module/import surface shipped its
> grammar/AST/elaboration (`l4-export-reexport-declaration`, merged) without a
> matching kenfmt layout, so the formatter cannot lay out its own new decls.
> The Architect ruled path (a) (evt_qapcbnjm7m2g): Component B does NOT block on
> this fix — it authors its wide module-surface decls as adjacent partitioned
> declarations (each line <=96 and a kenfmt fixed point), a semantics-preserving
> additive-fold stopgap (`crates/ken-elaborator/src/modules.rs:1752-1772`), so
> `blocks: []` stands. THIS node is the recombination trigger: when the
> Doc-builder lands, recombine Component B's split OrdResult export and Derived
> selective import back to the single canonical name-list form and let kenfmt
> reflow it (the Architect took this tracked node, in place of an inline TODO, as
> the durable recombination record). No `trusted_base()` impact: kenfmt is a
> source-to-source formatter whose output is re-checked; it is not in the TCB.

## The gap (Steward-confirmed at f5be017f7 / formatter source at d5ad700b7)

kenfmt has NO breakable layout for the module/import declaration surface. A
comma-separated `export A, B, C, ...` or selective `import Mod (a, b, c)` name
list is rendered as a flat, UNBREAKABLE run, so a name list that overflows the
formatter's `CANONICAL_WIDTH` (96, `crates/ken-elaborator/src/layout.rs:12`)
cannot be reflowed by running the formatter, and the module-surface decls do not
round-trip as a fixed point.

The Doc-construction proves it (`crates/ken-elaborator/src/layout.rs`):

- `print_decl` has explicit arms for `DataDecl`/`ViewDecl`/`LetDecl`/
  `TheoremDecl`/`AttachedProofDecl`/`AxiomDecl` but NONE for `ImportDecl` /
  `ExportDecl` (AST variants `ast.rs:386` / `ast.rs:394`); they fall through to
  the generic `_ => self.print_span(decl.span())` (`:393`).
- `print_span` replays tokens generically. Breakpoints come only from
  `soft_break_between` (`:1921`), which emits a break solely before
  `Arrow`/`requires`/`ensures`/`where` or at an `atom_can_end` -> `atom_can_start`
  boundary. `Token::Comma` is in NEITHER `atom_can_start` (`:1931`) nor
  `atom_can_end` (`:1952`).
- After a comma the layout emits a HARD space (`needs_space`, `:1900`), not a
  `Doc::line()`. The enclosing group therefore contains zero breakpoints, so
  `render(doc, 96)` has nothing to break: a 125-column `export ...` line stays
  125 columns.

Both Component B failures are this one gap: `kenfmt_b3_layout::ac7_whole_
catalog_is_parse_preserved_idempotent_and_width_bounded` (the 125-col export
exceeds 96) and `kenfmt_c_capstone::canonical_live_corpus_is_a_fixed_point` (the
generic replay does not round-trip the author's module-surface wrapping).

## Why it is grounded, not aesthetic (COORDINATION section 2 / program 4c)

The canonical-corpus invariant is a project commitment enforced in CI: the
catalog is kenfmt's canonical output and stays parse-preserved, idempotent, and
width-bounded (`kenfmt_b3_layout`, `kenfmt_c_capstone`). The module/import
surface is now IN the corpus (the catalog imports/exports canonical modules —
the whole point of [[LANG-MODULE-IMPORT-SYSTEM]]), so the formatter that owns
that corpus must lay out that surface. The gap is not an author style
preference; it is the formatter unable to canonicalize a construct the corpus
now contains. Leaving it open means every catalog module-surface decl over ~96
columns is a latent CI red.

## No existing node covers it

`l4-export-reexport-declaration` (released) is the parser/AST/elaboration wiring
of `export`, not formatter layout. `kenfmt-p0-separator-reconciliation`
explicitly leaves `import (a, b)` comma-lists alone. The other kenfmt layout WPs
(`kenfmt-signature-layout`, `kenfmt-b3-doc-algebra-layout`, `kenfmt-catalog-
wide`, `kenfmt-c-capstone`) cover signatures, let-bindings, match-arms, and
catalog-wide reflow — none covers export/import name-list wrapping.

## The fix (deliverable)

A dedicated Doc-builder in `layout.rs::print_decl` for `ExportDecl` and
`ImportDecl` (and the selective-import / re-export name lists) that emits a
breakable, nested group: the name list becomes a group whose comma boundaries
carry `Doc::line()` breakpoints, so an overflowing list wraps onto indented
continuation lines and `render(_, 96)` brings it under `CANONICAL_WIDTH`. The
result must be a kenfmt fixed point (idempotent) and parse-preserving.

Acceptance controls: a >96-column `export`/`import` name list formats to a
wrapped, width-bounded result; formatting is idempotent on it; the whole catalog
passes `kenfmt_b3_layout::ac7` and `kenfmt_c_capstone`; no change to elaboration
or `trusted_base()`. Cross-cutting: the lossless (`kenfmt_b1_lossless`) and
token-kind gates stay green.

## Sequencing

Buildable now — the formatter and the AST variants both exist; this only adds
the missing layout arms. It is [[LANG-MODULE-IMPORT-SYSTEM]] campaign work
(completing the surface the campaign introduced), so framing/sequencing it is
lane work, not a competing claim. Its relationship to Component B is RESOLVED
(Architect path (a), evt_qapcbnjm7m2g): no `blocks` edge — B ships the
partitioned stopgap on 4a79dc0fe and this node is B's recombination trigger.
