---
name: classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row
description: "A catalog reuse migration (a consumer drains a local reimpl to a provider import, or a provider flips fn to pub fn) owes an edit to the ambient-passthrough census only when a baseline-GREEN consumer's measured closure changes. An untouched census file was correct in seven measured instances for five different reasons: baseline-red short-circuit, provider row a subset of the consumer's row, retired name surviving only in prose, provider name already in the row from a prior sibling migration, and a provider-only pub flip. Classify the case (bucket from the enclosing binding, subset over ground-truth rows, term-vs-prose grep, existing row) before filing a missing edit; when a row IS owed, verify it equals provider-whole-module-ambient union consumer-own-retained, from other rows in the same expected vec."
metadata:
  type: feedback
---

# Classify a reuse migration's ambient-census case before filing a missing or wrong row

A catalog reuse migration has a companion obligation on
`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs::catalog_ambient_passthrough_migration_census`.
On a consumer drain, a diff that **does not touch the census file** reads
reflexively as a missing companion edit. Across the CAT reuse campaigns
(2026-08-29 to 2026-08-30) that reading was wrong in every measured instance but
the two where a row genuinely grew, and the one time it was filed as a finding
it voided a valid merge authorization.

**The rule: a row edit is owed only when a baseline-GREEN consumer's measured
ambient closure gains or loses a name.** Everything else is a reason the
untouched file is correct. Classify the case before filing.

## How the census works (verify at the SHA under review)

The classification loop, per discovered catalog leaf:

```
baseline = ElabEnv::new()
if baseline.elaborate_module_from_roots(root, entry).is_err():
    residuals.push((entry, "baseline: {error}"))
    continue                       # ambient_dependencies NEVER runs
match ambient_dependencies(root, entry):
    Ok(names) if !names.is_empty() => census.push((entry, names))
    Ok(_)                          => clean.push(entry)
    Err(error)                     => residuals.push((entry, error))
```

- **Three buckets, gated in order**: baseline-red -> `residuals`;
  baseline-green with a non-empty ambient vector -> `census`; baseline-green
  and empty -> `clean`.
- **Behavioral, not source-text.** `ambient_dependencies` elaborates the entry
  in `strict_floor_env`, admits each `UnresolvedCon` name found in the full
  global inventory, and retries until it resolves. It measures what the
  consumer's elaboration **requires**, including the whole loaded closure of
  every import. A consumer therefore inherits a provider's **whole-module**
  ambient debt, not the imported function's slice.
- **Reaching and exhaustive.** `assert_eq!(census, expected)` ("WP-4 migration
  sentinel"), `clean == expected_clean`, `residual_names == expected_residuals`
  (module entries, not error strings), three disjointness asserts, and
  `partition == discovered` over `source_leaves(catalog/packages)`. Green in CI
  means `expected` equals the real computation, so a green delta is exact by
  construction. You confirm the assert is reaching; you do not re-derive the
  closure name by name.

**Current state, checked 2026-09-26 at `19c105b97`:** `expected_residuals` is
empty (emptied by `1b7af23bd`, CAT-LOGIC-PRELUDE-MOVE, 2026-09-25), so every
catalog leaf is baseline-green and case 1 below does not currently fire. The
gate is still in the loop. Read the residual set at the SHA you review rather
than assuming either state.

## The cases

| case | consumer / change shape | census obligation |
|---|---|---|
| 1 | baseline-RED residual (short-circuits before `ambient_dependencies`) | none, whatever it imports |
| 2 | baseline-GREEN; provider closure brings names the consumer's row lacks | exact grown row, in place, no bucket move |
| 3 | baseline-GREEN; provider's row is a subset of the consumer's existing row | none |
| 4 | a retired local name survives only in prose or comments | none for that name |
| 5 | the provider name is already in the consumer's row from a prior sibling migration | none |
| 6 | provider-only `fn` -> `pub fn` flip, no consumer edited | none |

The drained local itself contributes nothing when its body referenced only
floor constructors (`True`/`False`, `Nil`/`Cons`): it was never an admitted
ambient name, so retiring it drops nothing from the row.

## The discriminating checks, in order

1. **Baseline bucket first, resolved from the enclosing binding.** Is the
   consumer in `expected_residuals`? Does `ken check` on its staged tree exit
   nonzero at some `UnresolvedCon`? If baseline-red, no import can move it.
   Resolve the enclosing `let` of any "X is in set S" claim; never attribute a
   bucket from a line number. On D2 Parsing, line 695 was read as
   `expected_clean` (lines 674-682, four entries) when it sat inside
   `expected_residuals` (opens at :683); two readers cited :695 and reached
   opposite buckets.
2. **Subset over ground-truth rows.** Compute `provider_row` subset-of
   `consumer_row` from rows in the same `expected` vec. Subset -> no edit. A
   provider name absent from the consumer's row is the owed grow, and its
   omission is the finding.
3. **Grep the retired name as a TERM** (word boundary, outside prose) in the
   consumer. Still referenced by a surviving law -> unresolved under the strict
   floor -> ambient -> owed. Referenced only in prose -> never ambient -> none.
4. **Check the consumer's existing row for the provider name.** A prior
   sibling migration in the same node may already have placed it there
   transitively.
5. **A same-import split is the bucket, not an inconsistency.** "A and B both
   `import Consumer (length)` but only A gained the name" is explained by A
   being baseline-green and B baseline-red. Resolve both sides' buckets before
   calling the census under-grown.
6. **A prediction is not a measurement.** If you cannot run the one census
   test, route it as "predicted, unverified: run this test", not as a CI-red
   finding that stops a publication.

## When a row IS owed: verify it is exact

The new row must equal **(provider's whole-module ambient) union (the
consumer's own ambient names that survive the migration)**. Take both operands
as ground truth from other rows in the same `expected` vec, never by hand:

- the provider's own row is its whole-module ambient;
- a sibling baseline-green importer of the same provider with no own-extra
  ambient must have a row **identical** to the provider's own row. That is the
  independent corroboration that inheritance is whole-module, not
  import-scoped; if it differs, the model is wrong and the census is suspect.

Then new row minus provider set must be exactly the consumer's retained names.
Confirm no bucket move: a baseline-green `census` consumer's row grows in
place, so `expected_clean` and `expected_residuals` stay byte-identical; a diff
that edits them for the migrated leaf is the smell. Do not cry wolf that the
row "looks too big": names the imported function never touches (on
`Data.Collections.Derived`, `is_sorted`/`leqChar`/`eqChar`) are the tell of
genuine whole-module inheritance. A delta that is anything else (a missing
provider name, a spurious name, a consumer-own name the migration should have
removed) is the finding: name the missing or extra name and the two
ground-truth rows it fails against.

## A residual sentinel certifies census-neutrality, not only masking

- A residual consumer drain that ships a raw-boundary sentinel reaching the
  **same** first `UnresolvedCon` name before and after the drain is
  self-certifying: the drain touched something unrelated to the residual
  cause, so no bucket move is owed.
- The green `residual_names == expected_residuals` sentinel ("WP-4 strict-floor
  residual sentinel") proves no residual -> ambient transition happened. When a
  reuse adds a consumer's **first** import edge, that sentinel is what rules
  out an unaccounted bucket move; its greenness is evidence, not silence.
- The flip side: a residual has no measured ambient vector, so inherited
  provider debt is invisible there. "The sibling's census did not change" is not
  evidence it escaped the debt. See
  [[certify-a-catalog-reuse-migration-by-body-identity-and-the-proofs-that-still-check]]
  for the surface-widening cost.

## The provider side: a `fn` -> `pub fn` flip owes nothing

`pub` gates cross-module **importability** (the selective-import and export
set, spec `33 §4`; `conformance/surface/modules/seed-modules.md`: resolution
"keyed on the pub export set"), not ambient or floor resolution. A consumer
that references a bare name it neither defines nor imports resolves against a
GlobalId that already existed as a non-pub loaded global. `pub` adds or
removes no GlobalId, so the floor resolver sees the identical global set and
no row moves. The census-side checks:

1. **Denotation**: flipped bodies byte-identical (visibility-only).
2. **Importers**: `git grep -n 'import.*<Module>'`. A selective importer not
   naming a newly-pub symbol is unaffected; a whole-module glob importer is the
   only shape that could inject the name and collide.
3. **Ambient**: pub is not floor resolution and the GlobalId pre-existed.
4. **Tests outside the gate's run**: cross-crate tests (e.g.
   `crates/ken-cli/tests/rosetta.rs`, not in `-p ken-elaborator`) and
   conformance seeds (CI-only). Ones built over unchanged selective edge
   strings, or testing the pub mechanism with their own inline `module M
   {...}` fixtures, are insensitive to new pub names.

The one shape that would owe an edit: a **pre-existing exact-set pub-surface
assertion** over the flipped module, or a whole-module glob importer. Grep for
`BTreeSet`/exact-inventory `assert_eq!` naming the module. The candidate's own
new exact-set test is satisfied by construction and is not a debt. The full
attack list for a flip (export-surface oracle, collision, trust, signature
closure, prose) is in
[[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]].

## What the false positive cost

On D2 Parsing (`510c857e0`) a clean -> census flip was predicted, and a red on
`assert_eq!(clean, expected_clean)` was filed, without checking baseline status
(Parsing failed at `baseline: unresolved type 'SourceId' at 1958-1966`) and
from a line number. Three seats reproduced the census green with the five D2
blobs staged (implementer `evt_21cnavpgt24nz`, Steward, Architect
`evt_2e3swt6j1pm2b`). Cost: a wrongly voided merge authorization
(`dec_5xw3k1xqsbd7h`, reinstated), about 20 minutes of fleet time, and a frame
erratum (`e00554ec` -> main `2d0fcdfb4`). Stopping on an M8 red was correct;
the finding under it was false because the box could not run the one test and
the prediction was shipped as a measurement.

## Instances

- **Case 1, false positive.** CAT-DERIVED-REUSE-CONSUMERS D2
  `Capability.Parsing.Parsing`, exact
  `510c857e0693a5bc3c79091116d746b65c48b7b7`; census blob
  `17b97745ede7ffac4b5ddbb0dce0249a655f5a2f` (loop at lines 377-392). Imports
  `Data.Collections.Derived (list_append)`; no edit owed; `AC-BOTH-CENSUSES`
  discharged by the five-path content.
- **Case 1, positive mirror.** CAT-DERIVED-REUSE-CONSUMERS D3
  `Core.Classes.EffectfulClasses`, exact
  `b935dec7469e9f3ba39fc5e74839aec55e0dce1a` (range `548c47de5..b935dec74`),
  `evt_3d9yqp61acqx4`, thread `thr_2vwarkhtpt9k1`. In `expected_residuals`; the
  reused `concat_map` was local before, never the unresolved symbol, so the
  baseline-red cause is untouched. Even if the baseline error text changes the
  entry stays a residual. D4 Cursor/Json were the same shape.
- **Case 1 with in-suite sentinel, rename drain.** CAT-BOOL-REUSE-CONSUMERS D2
  `Data.Collections.Map`, landed squash
  `4c909d402c6ebe36db4e6002688a238d88511649` (candidate
  `e2d4a265d95e0ff6255f36473eb9fec292e5dfa1`, range `0ddd49b3..e2d4a265`),
  `evt_2fvn6hgkhjy1z`, thread `thr_5ftze8z9yrrr8`. Map in `expected_residuals`
  (:719); sentinel `cat_bool_reuse_d2_raw_boundary_remains_unresolved_list_append`
  reached the same `UnresolvedCon "list_append"` while the drain touched
  `is_some`. The closure repair was deferred to the named
  `CAT-MAP-DEPENDENCY-CLOSURE-REPAIR`, which landed as `b6a79ae8c` and retired
  the sentinel. Binding: origin/main `d95bc2df` was a doc-only M7 closure on
  top of the squash and the squash parent `bde01522` was not the range base,
  so the landed diff was bound by two blob-equalities (product blobs
  candidate == squash == origin/main; base blobs range base == squash parent).
- **Case 2.** CAT-DERIVED-REUSE-CONSUMERS D1 Deque (`c5147580d`): row
  `["Equal"]` -> Derived's 13 names. D5 `Tooling.Testing.Property`, exact
  `2e7796d8080c794a694aa0301a5793405a7a169e` (range `863bf0fbf..2e7796d80`),
  `evt_7m31h3450anb3`, thread `thr_dzk7695m1cbc`: Derived's row (line 591) =
  `{And, Bottom, Equal, Prop, Proved, Top, Unit, and_fst, and_intro, and_snd,
  eqChar, is_sorted, leqChar}`; Deque's row (line 570) identical; Property
  `{MkUnit, Unit}` -> 14 = Derived's 13 union `{MkUnit}`. Only the Property row
  (lines 647-673) changed; `expected_clean` (692) and `expected_residuals`
  (701) byte-identical.
- **Case 3.** CAT-BOOL-REUSE-CONSUMERS D1 `Data.Collections.Derived`, landed
  squash `11ce6f3aa68591793d12ff156d0bb003e7ce5f1a` (range
  `ba1c92214..6bd2d0ed`, parent `15fbb14db`; blobs Derived `d30756c0`, rosetta
  `45c6c5be`, cat3 `a1cdc9e8`), `evt_5ynvdmymxmmf3`, thread
  `thr_5ftze8z9yrrr8`. Retired local `bool_and`/`bool_leq` for
  `import Core.Classes.LawfulClasses (bool_and, bool_leq)`. Provider row
  `{And, Bottom, Equal, Prop, Proved, and_fst, and_intro, and_snd}` (8) is a
  strict subset of Derived's 13; the drained locals referenced only
  `True`/`False`.
- **Cases 2 and 4 in one drain.** CAT-PRELUDE-REUSE D1, landed squash
  `cb20749d1a28038a5cb3c115e23c90cbe01e7b8a` (range `f8b2dd642..3d60517a`,
  squash parent `c7527bd8`, origin/main `e324ac9a5` = squash plus a doc-only
  tracker commit), `evt_7t4aefce3dx0x`, thread `thr_59hfapm6qz0wm`. Derived
  retired local `map` and `filter` so bare references fell through to the
  prelude. `map` was still referenced by the surviving `map_length` law and
  was added to exactly `{Derived, Deque, Property}`; `filter` survived only in
  prose (its membership law was held out) and was added nowhere. Four other
  Derived importers (`Cursor`, `Parsing`, `EffectfulClasses`, `Json`; two of
  them importing the same `length` as Property) did not gain `map` because
  they were baseline-red. This drain-to-prelude shape was later reversed:
  `8b324e5fb` (CAT-COLLECTIONS-MAP-FILTER-PRELUDE-MOVE) moved `map`/`filter`
  back into Derived as `pub fn`, and its identity pin
  `derived_unshadows_installed_prelude_map_and_filter` is gone.
- **Case 5.** CAT-PRELUDE-REUSE D2 `Tooling.Testing.Property`, squash
  `bc9ff6fb7` (range `e324ac9a..b4adf046`), `evt_5x2g4f78j169j`, thread
  `thr_59hfapm6qz0wm`. Property retired private `gen_map_list` and repointed
  `gen_map` to `map`; `map` was already in Property's row, placed there
  transitively by D1 through `import Data.Collections.Derived (length)`.
- **Case 6.** CAT-BOOL-PUB-EXPORT exact
  `43d1f33ddbe27ead96e669a0df7dccb94805ea18` (range `6516e760a..43d1f33dd`,
  commits `07bdad34c` and `43d1f33dd`), `evt_3mwhc2xescpcn`, thread
  `thr_60501rtqvm4d9`; CV approve `evt_5w8tbrgz83hv1`. Three flips:
  `bool_leq`/`bool_and` (`Core/Classes/LawfulClasses.ken.md:320,636`) and
  `is_some` (`Data/Sums/Combinators.ken.md:75`). All six LawfulClasses
  importers were selective, naming only already-pub `{Ord, IsTrue, bool_or,
  leq_nat}`; Combinators had none. Contrast the consumer drain D5
  `2e7796d80`, which may owe a row.

Related: [[a-pattern-match-is-evidence-about-what-encloses-it]] (the line-number
bucket error is one of its instances),
[[a-measurement-census-can-exactly-pin-a-partition-yet-leave-the-majority-bucket-unmeasured]]
(the residual bucket is where this census did not measure),
[[in-a-multi-bucket-partition-census-a-growing-per-item-vector-is-not-a-regression-until-you-locate-the-items-bucket-transition]]
(which bucket moves are regressions on this same census), and
[[certify-a-catalog-reuse-migration-by-body-identity-and-the-proofs-that-still-check]]
(certifying the reuse itself on the same candidates).
