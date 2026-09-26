---
name: a-consumer-wall-watch-predicate-is-discharged-by-a-whole-product-scope-census-and-a-fail-closed-strip-guards-only-the-stripped-edges
description: "A catalog consolidation that MOVES symbols between modules keeps tripping CI-only consumer walls that targeted local scope never runs. Discharge a 'grep for every consumer' watch predicate with a whole-product-scope census against the LANDED tree: crates/ consumers, catalog/ consumers in both import form and BARE-name form (catalog packages reference cross-package names with no import), library/, and an #[ignore] scan. A green squash already vouches for what CI runs, so the census only has to find what CI does not run. Separately: a fail-closed guard over the edges it strips proves nothing about the inputs it does not enumerate."
metadata:
  type: feedback
---

# A consumer-wall watch predicate is discharged by a whole-product-scope census, and a fail-closed strip guards only the stripped edges

**Measured 2026-08-24 on the landed squash `76426e9f9`** (Component B,
LANG-MOD-CATALOG-COMPLETENESS, M8 post-merge landed-range hunt). Verdict clean,
one non-blocking latent note.

## The shape

A catalog consolidation MOVES symbols between modules (Component B moved
`list_eq`/`list_compare`/`pair_compare`/`ord_result_*` out of
`Data.Collections.Derived` into new canonical `Core.Logic.Compare` and
`Core.Logic.OrdResult`). The migration reconciles the consumers it knows about
(30 acceptance fixtures), but a moved symbol has consumers the targeted local
scope never exercises. Component B tripped **two CI-only consumer walls**
before merge: the kenfmt module-surface formatter, then the ken-cli Rosetta
flat-source compat unit. The Architect filed a **watch predicate**: a third
wall means "grep the whole test and tooling tree for consumers of the moved
surface and reconcile all before the next respin."

## Discharge: a whole-product-scope census against the LANDED tree

Run every grep against the merged SHA (`git grep <pat> <sha> -- <paths>`),
never the working tree. Cover the COMPLETE moved surface, the function names
AND the moved-module qualified strings, across four scopes:

1. **crates/**: every hit sits inside the already-reconciled changed-file set
   (byte-identical to the cleared candidate), and none is a new
   `#[ignore]`/un-run consumer.
2. **catalog/**, in BOTH forms: the import form (`import <OldHome>
   (movedname)`) AND the **bare-name** form. Catalog packages reference
   cross-package names with **no import** (flat-scope resolution), so an
   import-only grep misses them. `Posix.ken.md` and `BytesKeys.ken.md`
   referenced bare `list_eq` and were reconciled only through the updated
   `cc6b_path_posix_acceptance` fixture (which swapped `load_core_logic_or` for
   `load_core_logic_compare`), not through any import edge.
3. **library/** literate consumers.
4. An **`#[ignore]` scan**, so an un-run consumer CI never executes cannot hide.

**The leverage:** a squash that merged green means whole-suite CI already
exercised every consumer CI runs. The census only has to find what CI does
**not** run: ignored tests, tools not wired into CI, and external catalog
packages loaded only behind a test fixture. Empty across all four scopes means
the predicate is discharged. Report the census as evidence, not a bare "clean".

## A fail-closed strip guards only the stripped edges

The Rosetta repair strips each flattened cross-module import under an exact
single-occurrence assertion (`remove_flattened_import` in
`crates/ken-cli/tests/rosetta.rs` panics on absent OR duplicate), genuinely
fail-closed on the edges it enumerates. But the assembled flat unit elaborated
only because the **un-stripped** providers (Transport, Or, OrdResult) carried
**zero** import edges, and nothing guarded that. A catalog edit adding a
cross-module import to one of them would silently regress the harness to the
exact `UnboundName` CI-red the repair fixed. **A fail-closed cardinality guard
proves nothing about the inputs it does not enumerate.** When a harness's
correctness rests on "these other sources happen to have no import", that
assumption is the next wall. The cheap hardening: assert the un-stripped
sources contain no residual `import ` line after extraction.

## How to apply

For a landed squash whose delta over a prior clean is a single consumer or test
repair: isolate the delta by diffing the cleared candidate against the squash,
hunt the repair, and run the moved-surface census above to discharge any
standing consumer-wall predicate, covering the bare-name catalog form and the
ignored-test gap. Name any unguarded "these other inputs happen to be safe"
assumption as the next-wall risk. Reported `evt_ryrmff37zene` (lieutenant M8
hunt thread `thr_5f4cz4txebpe0`).

Related:
[[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]]
and [[an-enumeration-needs-a-proven-closure-not-a-better-grep]].
