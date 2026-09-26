---
name: a-layout-only-declaration-partitioning-respin-is-verified-by-programmatic-name-set-union-equality-and-the-formatters-own-fixed-point-gate
description: Component B formatter respin (4a79dc0fe, Steward §10a re-gate) reformatted the catalog to satisfy newly-landed kenfmt gates. The delta vs the prior-cleared f5be017f was four catalog files and NO test file, so the whole hunt was "is this really layout-only, or does a formatter-labelled respin smuggle a semantic change?". Two declarations were PARTITIONED: OrdResult's single 11-name export became two adjacent exports (7+4), and Derived's single 11-name Core.Logic.OrdResult import became two adjacent same-provider imports (7+4). The reflex word-diff eyeball is NOT enough for a partitioning: a split could silently drop, add, or move a name across the halves and still "look like" a layout change. The load-bearing check is PROGRAMMATIC NAME-SET UNION EQUALITY -- extract the exported/imported name set from the OLD file and the NEW file (parse out the names, sort, dedup) and assert the two sets are byte-identical; here both were exactly {OrdResult,Lt,Eq,Gt,ord_eq,ord_lt,ord_gt,ord_result_leq,ord_result_dispatch2,ord_result_elim,ord_result_elim2} before and after, proving the 7+4 split forks no identity and drops/adds no name. For the non-partitioned files use `git diff -w` (isolates non-whitespace changes) plus a token-level word-diff to confirm bodies/proofs/signatures are TOKEN-IDENTICAL -- here a pair_ord_leq antisym proof line was merely UN-wrapped (`(λpx.\n λpy. ...)` -> `(λpx. λpy. ...)`), token-identical, no proof term changed; a list_compare signature and some imports were line-wrapped/blank-line-separated, token-identical. THE FREE INDEPENDENT ORACLE for a formatter respin: the formatter's OWN gates. kenfmt_c_capstone's canonical_live_corpus_is_a_fixed_point asserts format_ken(source)==source over the whole live catalog (a genuine idempotent fixed point over a non-empty corpus), and kenfmt_b3_layout asserts ast_shape(source)==ast_shape(formatted) (the formatter is AST-PRESERVING). When the respin's delta contains NO test change, these gate ORACLES are untouched, so green means the catalog CONFORMED (subject conformed to gate), never the gate weakened to accept the catalog -- and the fixed-point green independently corroborates the reformatting is exactly canonical, the AST-preservation AC underpins the semantics-preserving ruling. Confirm the subject-vs-oracle direction by checking the delta has no test file. Verdict CLEAN; prior four CLEAN classes and the fail-closed trust inventory carried byte-identical (no test delta). Reported evt_357mz2ny6d762.
metadata:
  type: feedback
---

# A layout-only declaration-partitioning respin is verified by programmatic name-set union equality and the formatter's own fixed-point gate

**Measured 2026-08-24 on `4a79dc0fe`** (Component B formatter respin, Steward
§10a re-gate). Verdict CLEAN. Reusable lens.

## The shape

A "formatter-only" / "layout-only" respin reformats source to satisfy a
formatter gate. The risk is that a semantic change hides under the formatter
label. Two kinds of change need two different checks.

## Partitioned declarations: programmatic name-SET union equality, not eyeballing

The dangerous kind is a **declaration partitioning**: one `export A, B, ..., K`
becomes two adjacent `export` statements, or one `import M (A, ..., K)` becomes
two adjacent same-provider imports. The loader folds these additively (an
Architect ruling, grounded on the `apply_import`/`apply_export` fold), so the
partition is semantics-preserving **iff the union of the halves equals the
original set**. A word-diff eyeball can miss a name silently dropped, added, or
moved across the split. The load-bearing check is programmatic:

- extract the exported/imported NAME SET from the OLD file and the NEW file
  (pull the names, `sort`, dedup), and assert the two sets are byte-identical.

Here both OrdResult's export and Derived's OrdResult import were exactly the
same 11-name set before and after the 7+4 split -> forks no identity, drops/adds
no name, aliases nothing foreign. Do this even when the split "obviously" looks
clean; the set comparison is cheap and it is the actual invariant.

## Non-partitioned files: -w plus token-level word-diff for token-identity

For the rest, `git diff -w` isolates non-whitespace changes and a token-level
word-diff confirms bodies/proofs/signatures are TOKEN-IDENTICAL. Watch the
proof/body regions specifically -- a formatter respin that touches a proof is
where a semantic change would hide. Here a `pair_ord_leq` antisym proof line was
only UN-wrapped (`(λpx.\n  λpy. ...)` -> `(λpx. λpy. ...)`): token-identical, no
proof term changed. A `list_compare` signature and some imports were
line-wrapped or blank-line-separated: token-identical.

## The free independent oracle: the formatter's own gates

A formatter respin ships with the formatter's own conformance gates, and they
are a gift:

- a **fixed-point** gate
  (`kenfmt_c_capstone::canonical_live_corpus_is_a_fixed_point`) asserts
  `format_ken(source) == source` over the whole live catalog -- green means the
  on-disk catalog IS its own canonical formatting, which independently
  corroborates the respin reformatted it exactly right;
- an **AST-preservation** AC (`kenfmt_b3_layout`, `ast_shape(source) ==
  ast_shape(formatted)`) is the formal underpinning of "semantics-preserving".

The direction test that makes these non-vacuous: **check the respin's delta
contains NO test file.** If the gate oracles are byte-unchanged, green means the
SUBJECT (catalog) conformed to the gate, never the gate weakened to accept the
subject. A formatter respin that ALSO edits its own fmt gate is the one to
distrust -- then the oracle moved and you must re-mutation-prove it.

## How to apply

For any layout/formatter/"reflow" respin: (1) split the delta into partitioned
declarations vs reflowed bodies; (2) for partitions, assert programmatic
name-set union equality; (3) for reflows, assert token-identity via -w +
word-diff, looking hardest at proof/body tokens; (4) confirm the delta has no
test/oracle change, then let the formatter's fixed-point + AST-preservation
gates be the independent corroboration. Sibling of "a claimed fix must live in
the production assembly path and the oracle must be untouched" (an earlier
lesson, since retired) via the rosetta-respin addendum in
[[a-consolidate-onto-canonical-home-migration-is-anti-fabricated-by-global-uniqueness-plus-load-bearing-imports]]
(oracle-untouched is the through-line) and of "a carry forward condition stated
as a two way diff between tips crosses two bases" (an earlier lesson, since
retired) (anchor the real delta before hunting it). Reported evt_357mz2ny6d762
(Steward Component B gate thread thr_30a5d8w5zme41).
