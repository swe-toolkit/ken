# WP frame — LANG-MOD-KENFMT-DECL-LAYOUT (LANG-MOD campaign tail, before z3 resume)

> Campaign work under [[LANG-MODULE-IMPORT-SYSTEM]]. Owning team: language.
> Size M. Capability tier: T2 (a bounded formatter-layout addition that reuses
> the existing `Doc`/group/line algebra and follows the established
> `print_decl` arm pattern; the review is differential on the idempotence /
> width / fixed-point controls, not on a design argument). No Decision — the fix
> shape is settled and grounded in the Doc-construction; the Architect already
> ruled its relationship to Component B (path (a), evt_qapcbnjm7m2g). Scheduled
> at the END of the LANG-MOD campaign (after Component B), before the verify/z3
> FO-checker resume. `depends_on: [LANG-MOD-CATALOG-COMPLETENESS]`.

## Objective

Give kenfmt a breakable layout for the module/import declaration surface, so an
overflowing `export`/`import` name list wraps under `CANONICAL_WIDTH` (96) and
round-trips as a formatter fixed point. When it lands, recombine Component B's
partitioned stopgap decls (the split OrdResult export and Derived selective
import) back to the single canonical name-list form and let kenfmt reflow them.

## Why it is grounded, not aesthetic (COORDINATION §2 / program 4c)

The canonical catalog corpus is kenfmt's own output and is a CI-enforced project
commitment: parse-preserved, idempotent, width-bounded (`kenfmt_b3_layout`,
`kenfmt_c_capstone`). The module/import surface is now IN that corpus (the whole
point of the campaign — the catalog imports/exports canonical modules), so the
formatter that owns the corpus must be able to lay that surface out. The gap is
the formatter unable to canonicalize a construct the corpus now contains; every
catalog module-surface decl over ~96 columns is otherwise a latent CI red.

## Fixed inputs (Steward-confirmed; re-grounded on current main d5c41ec1)

Grounded at Component B merge `f5be017f7` / formatter source `d5ad700b7`,
re-checked on current main `d5c41ec1`.

kenfmt has NO breakable layout for export/import name lists — they render as a
flat, unbreakable run. The Doc-construction proves it
(`crates/ken-elaborator/src/layout.rs`):

- `print_decl` has explicit arms for `DataDecl`/`ViewDecl`/`LetDecl`/
  `TheoremDecl`/`AttachedProofDecl`/`AxiomDecl` but NONE for `ImportDecl` /
  `ExportDecl` (AST variants `ast.rs:386` / `ast.rs:394`); they fall through to
  the generic `_ => self.print_span(decl.span())`.
- `print_span` replays tokens generically. Breakpoints come only from
  `soft_break_between`, which breaks solely before `Arrow`/`requires`/`ensures`/
  `where` or at an `atom_can_end` → `atom_can_start` boundary. `Token::Comma` is
  in NEITHER `atom_can_start` nor `atom_can_end`.
- After a comma the layout emits a HARD space (`needs_space`), not a
  `Doc::line()`. The enclosing group contains zero breakpoints, so
  `render(doc, 96)` has nothing to break: a 125-column `export …` line stays 125.

Both Component B formatter failures are this one gap:
`kenfmt_b3_layout::ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded`
(a 125-col OrdResult export exceeds 96) and
`kenfmt_c_capstone::canonical_live_corpus_is_a_fixed_point` (the generic replay
does not round-trip the author's module-surface wrapping).

`CANONICAL_WIDTH` = 96 (`crates/ken-elaborator/src/layout.rs`). kenfmt is a
source-to-source formatter whose output is re-checked; it is NOT in the TCB, so
zero `trusted_base()` impact.

## No existing node covers it

`l4-export-reexport-declaration` (merged) is the parser/AST/elaboration wiring of
`export`, not formatter layout. `kenfmt-p0-separator-reconciliation` explicitly
leaves `import (a, b)` comma-lists alone. The other kenfmt layout WPs
(`kenfmt-signature-layout`, `kenfmt-b3-doc-algebra-layout`, `kenfmt-catalog-wide`,
`kenfmt-c-capstone`) cover signatures, let-bindings, match-arms, and catalog-wide
reflow — none covers export/import name-list wrapping.

## Deliverables

- **D1 — the Doc-builder.** A dedicated arm in `layout.rs::print_decl` for
  `ExportDecl` and `ImportDecl` (and the selective-import / re-export name lists)
  that emits a breakable, nested group: the name list becomes a group whose comma
  boundaries carry `Doc::line()` breakpoints, so an overflowing list wraps onto
  indented continuation lines and `render(_, 96)` brings it under
  `CANONICAL_WIDTH`. Parse-preserving and idempotent. Reuse the existing `Doc`
  algebra and the established `print_decl` arm pattern — do not touch elaboration
  or `trusted_base()`.
- **D2 — recombination of Component B's stopgap.** Recombine Component B's split
  OrdResult export and Derived selective import (the additive-fold stopgap at
  `crates/ken-elaborator/src/modules.rs`, path (a)) back to the single canonical
  name-list form and let the new layout reflow them. This is the durable
  recombination the Architect tracked here in place of an inline TODO.

## Acceptance criteria

- AC-1 — a >96-column `export` name list formats to a wrapped, width-bounded
  result (every line ≤ 96).
- AC-2 — a >96-column selective `import Mod (a, b, c, …)` name list formats to a
  wrapped, width-bounded result.
- AC-3 — formatting is idempotent on both (a second pass is a no-op / fixed
  point).
- AC-4 — parse-preserving: the wrapped form parses to the same AST as the flat
  form (no token loss; `kenfmt_b1_lossless` and the token-kind gate stay green).
- AC-5 — the whole catalog passes `kenfmt_b3_layout::ac7` and
  `kenfmt_c_capstone` with Component B's decls recombined to the canonical
  name-list form (no partitioned stopgap remaining for the recombined decls).
- AC-6 — a mutation that drops the comma-boundary `Doc::line()` (reverting to the
  flat run) REDS AC-1/AC-2 (proves the breakpoints, not incidental width, carry
  the wrap).
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-elaborator`
  / `--test` only, never `--workspace`. No change to elaboration or
  `trusted_base()`.

## Contention check

Touches `crates/ken-elaborator/src/layout.rs` (`print_decl` gains the two arms +
the name-list group builder) and the kenfmt fixture/gate tests
(`kenfmt_b3_layout`, `kenfmt_c_capstone`, `kenfmt_b1_lossless`), plus the
recombination edit to `crates/ken-elaborator/src/modules.rs` (removing Component
B's partitioned stopgap for the recombined decls). No overlap with lane 1
(runtime) or lane 3 (foundation). Sequenced after Component B so the decls it
recombines already exist on main.

## Reviewers

Architect (component fit: the new arms reuse the existing `Doc`/group/line
algebra and the `print_decl` pattern; no elaboration or TCB change) +
language-qa (the wrap/idempotence/fixed-point controls are discriminating against
a flat-run mutation, and the recombination leaves no stopgap behind). No Decision
fork. Librarian as-built advisory, non-gating.

## Sequencing

Campaign tail. Technically buildable earlier (formatter + AST variants both
exist), but scheduled after [[LANG-MOD-CATALOG-COMPLETENESS]] per operator
direction (2026-08-21): the two new LANG-MOD nodes are the end of the LANG-MOD
work, before the verify/z3 FO-checker lane resumes
([[V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY]] / [[V3-FO-EMBEDDING-ADEQUACY]]).
Sequencing it here also lets D2 recombine B's already-landed stopgap decls.
