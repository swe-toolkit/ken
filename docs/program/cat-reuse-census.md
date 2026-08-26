# CAT-REUSE-CENSUS — catalog reuse modernization inventory

This census records the catalog-reuse surface at exact source base
`ed5b4063f434cc7a2311143367928ee98f64fd86`. It inventories every literate
package under `catalog/packages/`; it changes no package and closes no rework.

The release estimate of 39 packages was stale. The git object tree at the exact
base contains **48** `*.ken.md` packages. The population list has SHA-256
`50ca4604db917bf7e2758e075c269c064ac181f871c246bea73d0d4b7e197333`.

## 1. Method and classifications

A duplicate is recorded only when the local declaration's complete type and
transparent computation or proof role agrees with a named canonical provider.
A shared spelling, a similar match shape, and a domain-specific wrapper are not
enough. In particular, indexed `Vec.map` is not prelude `map`, the `Pair`-valued
`Derived.zip` is not the prelude's `Prod`-valued `zip`, and field projectors for
different `ProcessInput` fields are not duplicates.

The table uses these tags:

- **`[low]`** — a computational definition can be replaced by the same canonical
  operation without changing a proposition or attached-proof owner.
- **`[higher]`** — the change moves a class/instance, a proposition, an attached
  proof, or another proof-bearing identity. It requires an individual ruling.
- **`TD`** — the first checked declaration is the package's public carrier,
  operation, or law: headline-first at the checked package boundary.
- **`BU`** — a private helper or proof prerequisite precedes the package's
  headline carrier, operation, instance, or theorem: fundamentals-first.

`TD`/`BU` describes the observed checked declaration order, not a demand for
illegal forward references. The catalog style guide's reader-facing document
can still introduce the result in prose before dependency-first Ken declarations.

## 2. Canonical provider and prerequisite ledger

Every sibling arrow in the census points to one of these provider records. A
record states both prerequisite depths: export visibility and standalone
elaboration/attached ownership.

| Code | Canonical identity at the evidence base | Public surface | Standalone / ownership state |
|---|---|---|---|
| `P` | Compiler prelude captured before package source: `map=g115`, `fold=g116`, `filter=g118` in a fresh `ElabEnv` | Ambient compiler identity; no package export edge | Installed before source; no package attached-owner edge |
| `A` | `Data.Numeric.Nat.Arithmetic.add=g583`, `mul=g584` in the fresh provider run | both `pub fn` | standalone `ken check` exit 0 |
| `T` | `Core.Logic.Transport.subst=g578`, `cong=g579`, `sym=g581`, `trans=g582` | `cong`/`sym`/`trans` public; `subst` private | standalone exit 0; attached ownership local |
| `LC` | `Core.Classes.LawfulClasses`: `bool_or=g617`, `bool_eq=g628`, `bool_and=g634`, `compare_bool_cases=g638`, `ord_leq_at=g665` | only `IsTrue`, `bool_or`, and `Ord` are public among relevant names | standalone exit 0; current local attachments resolve |
| `N` | Canonical `Ord Nat` component required by `spec/30-surface/33-declarations.md` §5.3 and `spec/50-stdlib/51-lawful-classes.md` §7 at `Core.Classes.LawfulClasses` | absent at this base; current Order `leq_nat` is private and is not the lawful defined-at identity | **higher prerequisite:** current Order rejects at `2662..2814`; canonical imports expose foreign `bool_or::eq_true_of_or`, and the instance is orphaned. The component must move atomically to the class owner |
| `O` | Order-owned `Data.Numeric.Nat.Order.min` and `sub` declarations at lines 190 and 210 | private at this base | **higher prerequisite:** the containing module has the `N` standalone/ownership failure |
| `D` | `Data.Collections.Derived`, including `list_append=g614`, `length=g621`, `reverse=g626`, `concat_map=g633`, `eq_from_ord=g641`, `count=g642`, `Perm=g643`, `insert=g644`, `sort=g645` | relevant identities are private | standalone exit 0; no provider ownership error |
| `LF` | `Core.Classes.LawfulFunctors.comp` at line 246 | private | **higher prerequisite:** standalone rejects `UnresolvedCon list_append` at `4204..4215` |
| `BK` | `Data.Binary.BytesKeys.uint8_deceq_eq` and `bytes_deceq_eq`, lines 31 and 87 | private | **higher prerequisite:** standalone rejects `UnresolvedCon trans` at `596..601` |
| `SC` | `Data.Sums.Combinators.is_some=g581` in the fresh provider run | private | standalone exit 0 |
| `C` | `Capability.Parsing.Cursor.cursor_nat_lt`, line 130 | private | **higher prerequisite:** standalone rejects `UnresolvedCon bytes_nat_length` at `1793..1809` |
| `Cmp` | `Core.Logic.Compare.list_eq=g612` in the fresh provider run | public | standalone exit 0 |
| `OR` | `Core.Logic.OrdResult.OrdResult=g578` with `Lt/Eq/Gt=g579/g580/g581` | explicit facade export | standalone exit 0 |

Global IDs above are scoped to a fresh environment for each provider run. They
are not compared numerically across runs; the durable identity is the qualified
provider path. The run-local ID makes the provider target unambiguous. It does
not claim that a current local reimplementation already reuses that identity.

The identity probe's filtered output has SHA-256
`792af5d299e28a4e13a169666c4fbfc75b31c755350de66769cfb293ddf47180`.
The provider-check ledger has SHA-256
`558fd2f3284fd5b1a534e50f4cd1e3e28017ec00b178fe537585d3d7dc78a8d5`.

## 3. Complete per-package census

`—` means that the full declaration comparison found no item on that axis. Each
provider code expands to both prerequisite depths in §2.

| # | Package | Prelude redundancy | Sibling reimplementation | Arrangement and one-line evidence |
|---:|---|---|---|---|
| 1 | `Algorithm/Numeric/Gcd.ken.md` | — | `add→A.add [low]`; `mul→A.mul [low]`; `leq_nat→N.leq_nat [low]`; `sub→O.sub [low]`; `subst→T.subst [higher]`; `cong→T.cong [higher]`; `sym→T.sym [higher]`; `trans→T.trans [higher]` | `BU` — `add`/`mul`/order/transport scaffolding precedes `gcd_fuel`, `gcd`, and `divides_gcd`. |
| 2 | `Algorithm/Searching/OrderedSearch.ken.md` | — | `ordered_search_leq→LC.ord_leq_at [low]` | `BU` — the one-line order projection precedes membership, sortedness, and `search`. |
| 3 | `Algorithm/Sorting/InsertionSort.ken.md` | — | `ordered_leq→LC.ord_leq_at [low]`; `order_eq→D.eq_from_ord [low]`; `element_count→D.count [low]`; `permutation→D.Perm [higher]`; `insert→D.insert [higher]`; `sort→D.sort [higher]` | `BU` — order/count helpers precede the sort operations and their attached laws. |
| 4 | `Application/CommandLine/ArgParse.ken.md` | — | — | `TD` — the public specification/result carriers open the checked package before parsing helpers. |
| 5 | `Application/Configuration/Decoder.ken.md` | — | — | `BU` — local provenance and lookup machinery precede the environment/config decoding headlines. |
| 6 | `Application/Input/Schema.ken.md` | — | — | `TD` — schema vocabulary and the `Schema` carrier precede validation traversals. |
| 7 | `Capability/Console/Text.ken.md` | — | — | `TD` — the public `print` procedure is the first checked declaration. |
| 8 | `Capability/Diagnostics/Core.ken.md` | — | `diagnostic_nat_leq→N.leq_nat [low]` | `TD` — source/range and origin carriers open the diagnostic abstraction. |
| 9 | `Capability/Diagnostics/Render.ken.md` | — | — | `BU` — code/origin string helpers precede `diagnostic_to_doc`. |
| 10 | `Capability/Filesystem/Authority.ken.md` | — | — | `TD` — `capability_read` is the first checked authority manifest. |
| 11 | `Capability/Filesystem/Errors.ken.md` | — | — | `TD` — public error rendering starts with `renderIOError`. |
| 12 | `Capability/Filesystem/Path/Posix.ken.md` | — | — | `TD` — the public `Path` carrier precedes its operations and normalization proofs. |
| 13 | `Capability/Formatting/Doc.ken.md` | — | `pretty_nat_add→A.add [low]`; `pretty_nat_leq→N.leq_nat [low]`; `pretty_bool_cases→LC.compare_bool_cases [higher]` | `TD` — the `Doc` carrier opens the checked algebra before rendering helpers. |
| 14 | `Capability/Parsing/Cursor.ken.md` | — | `cursor_nat_add→A.add [low]`; `cursor_nat_sub→O.sub [low]`; `cursor_list_length→D.length [low]` | `TD` — the generic `CursorOps` carrier is declared before concrete cursor helpers. |
| 15 | `Capability/Parsing/Decoder.ken.md` | — | — | `TD` — decoder error/result carriers and the `Decoder` type open the package. |
| 16 | `Capability/Parsing/Numeric.ken.md` | — | — | `TD` — the public numeric error carrier precedes parsing and formatting. |
| 17 | `Capability/Parsing/Parsing.ken.md` | — | `nat_leq_bool→N.leq_nat [low]`; `list_append→D.list_append [low]` | `TD` — source/span and parser contracts precede the worked Boolean grammar. |
| 18 | `Capability/Process/Arguments.ken.md` | — | `argument_nat_leq→N.leq_nat [low]` | `TD` — `process_arguments` is the first checked process-input view. |
| 19 | `Capability/Process/Environment.ken.md` | — | — | `TD` — `process_environment` is the first checked process-input view. |
| 20 | `Capability/Process/Exit.ken.md` | — | — | `TD` — `exit_success` and exit policy are the opening declarations. |
| 21 | `Capability/Process/WorkingDirectory.ken.md` | — | — | `TD` — `process_working_directory` is the first checked view. |
| 22 | `Capability/System/Buffer.ken.md` | — | — | `TD` — the public `buffer_window` constructor opens the package. |
| 23 | `Capability/System/IO.ken.md` | — | — | `TD` — the first declaration is the headline `write_all_terminates` law. |
| 24 | `Capability/System/Resource.ken.md` | — | — | `BU` — result-construction helpers precede the bracket success theorem. |
| 25 | `Capability/Time/WallClock.ken.md` | — | — | `TD` — the public `Instant` projection opens the checked view. |
| 26 | `Core/Classes/EffectfulClasses.ken.md` | — | `compose→LF.comp [low]`; `concat_map→D.concat_map [low]` | `BU` — `apply_to`/`compose` helpers precede `Applicative`, `Monad`, and `Traversable`. |
| 27 | `Core/Classes/LawfulClasses.ken.md` | — | — | `TD` — the public truth predicate and class declarations open the package. |
| 28 | `Core/Classes/LawfulFunctors.ken.md` | `list_map→P.map [low]`; `list_foldr→P.fold [low]` | `bool_and→LC.bool_and [higher]` | `TD` — `Semigroup` is the first checked declaration, before instances and helpers. |
| 29 | `Core/Logic/Compare.ken.md` | — | — | `TD` — public `pair_compare` is the first checked operation. |
| 30 | `Core/Logic/EmptyDec.ken.md` | — | `DecEq→LC.DecEq [higher]`; `DecEq Bool instance→LC.DecEq-Bool [higher]`; `bool_eq→LC.bool_eq [higher]`; `sym→T.sym [higher]`; `trans→T.trans [higher]` | `BU` — `absurd_empty` and constructor helpers precede the duplicate class and lawful instance. |
| 31 | `Core/Logic/Or.ken.md` | — | — | `TD` — the `Or` family is the first and only checked component. |
| 32 | `Core/Logic/OrdResult.ken.md` | — | — | `TD` — the canonical result family opens the package. |
| 33 | `Core/Logic/Transport.ken.md` | — | — | `TD` — `subst` opens the package's five public-facing transport operations. |
| 34 | `Data/Binary/BytesKeys.ken.md` | — | — | `BU` — injectivity proof scaffolding precedes equality functions and lawful instances. |
| 35 | `Data/Collections/Deque.ken.md` | — | `deque_list_append→D.list_append [low]`; `deque_list_reverse→D.reverse [low]` | `TD` — the `Deque` carrier and public endpoint operations precede laws. |
| 36 | `Data/Collections/Derived.ken.md` | `map→P.map [low]`; `filter→P.filter [low]` | `min→O.min [low]`; `nat_sub→O.sub [low]`; `bool_and→LC.bool_and [low]`; `bool_leq→LC.bool_leq [low]` | `BU` — `list_append` and the combinator floor precede the package's verified sort and derived string/byte headlines. |
| 37 | `Data/Collections/Map.ken.md` | — | `option_is_some→SC.is_some [low]`; `bool_dichotomy→LC.compare_bool_cases [higher]`; `bool_and→LC.bool_and [higher]`; `cat4_bool_or→LC.bool_or [higher]`; `leq_nat→N.leq_nat [higher]`; `total_leq_nat→N.total_leq_nat [higher]` | `TD` — the `Tree` carrier and core map operations precede proof machinery. |
| 38 | `Data/Collections/NonEmpty.ken.md` | — | — | `TD` — the `NonEmpty` carrier opens the package. |
| 39 | `Data/Numeric/Nat/Arithmetic.ken.md` | — | — | `TD` — public `add` and `mul` precede their attached laws. |
| 40 | `Data/Numeric/Nat/Order.ken.md` | — | `leq_nat→N.leq_nat [higher]`; `leq_nat::refl→N.leq_nat::refl [higher]`; `leq_nat::trans→N.leq_nat::trans [higher]`; `leq_nat::antisym→N.leq_nat::antisym [higher]`; `total_leq_nat→N.total_leq_nat [higher]`; `bool_or::eq_true_of_or→LC.bool_or::eq_true_of_or [higher]`; `instance Ord Nat→N.Ord-Nat [higher]`; `OrdResult/Lt/Eq/Gt→OR.OrdResult/Lt/Eq/Gt [higher]` | `BU` — the misplaced relation/proof spine precedes the headline instance and later Nat operations. |
| 41 | `Data/Serialization/Json.ken.md` | — | — | `TD` — the public `Json` carrier opens the package before its cursor instance. |
| 42 | `Data/Sums/Combinators.ken.md` | — | — | `TD` — the public sum combinator floor begins with `get_or_else`. |
| 43 | `Data/Sums/Validation.ken.md` | — | — | `TD` — the `Validation` carrier precedes its lawful instances. |
| 44 | `Data/Text/Codec.ken.md` | — | — | `TD` — `decode_utf8` is the first checked operation. |
| 45 | `Data/Text/StringBijection.ken.md` | — | — | `TD` — the conversion-layer retraction certificate is the headline prerequisite. |
| 46 | `Data/Text/StringKeys.ken.md` | — | — | `BU` — equality/order functions and proofs precede the lawful dictionaries. |
| 47 | `Data/Vector/Vector.ken.md` | — | — | `TD` — indexed `Vec` and `Fin` carriers precede total operations and laws. |
| 48 | `Tooling/Testing/Property.ken.md` | `gen_map_list→P.map [low]` | `property_list_length→D.length [low]`; `property_nat_lt→C.cursor_nat_lt [low]`; `property_uint8_eq→BK.uint8_deceq_eq [low]`; `property_list_uint8_eq→Cmp.list_eq specialized by BK.uint8_deceq_eq [low]`; `property_bytes_eq→BK.bytes_deceq_eq [low]` | `TD` — the `Gen` carrier and generator operations precede cursor examples and witnesses. |

## 4. Rollup

### 4.1 Axis counts

| Axis | Packages | Items / disposition |
|---|---:|---:|
| Prelude redundancy | 3 | 5, all `low` |
| Sibling reimplementation | 16 | 58 total: 31 `low`, 27 `higher` |
| Checked arrangement | 48 | 36 `TD`, 12 `BU` |

The sibling count treats each moved Order declaration separately: `leq_nat`, its
three attached laws, `total_leq_nat`, the foreign `bool_or` attachment, the
instance, and the local `OrdResult` family are eight distinct identities even
though one atomic owner migration must move the first seven together.

### 4.2 Complete missing-export set

These are the provider surfaces required by at least one recorded sibling item
and not public at the evidence base:

- `Core.Logic.Transport`: `subst`.
- `Core.Classes.LawfulClasses`: `DecEq`, `ord_leq_at`,
  `compare_bool_cases`, `bool_eq`, `bool_and`, `bool_leq`; plus the absent
  canonical `leq_nat` component and public `bool_or::eq_true_of_or` bridge
  required by the Nat-owner migration. The canonical `DecEq Bool` dictionary
  is an instance-registration identity, not a selective export name.
- `Core.Classes.LawfulFunctors`: `comp`.
- `Data.Numeric.Nat.Order`: `min` and `sub` after the canonical-owner
  prerequisite. `total_leq_nat` is deliberately not placed here: the settled
  owner migration keeps it provider-private, so consumers may not treat a pub
  modifier as closure.
- `Data.Collections.Derived`: `list_append`, `length`, `reverse`, `concat_map`,
  `eq_from_ord`, `count`, `Perm`, `insert`, and `sort`.
- `Data.Binary.BytesKeys`: `uint8_deceq_eq` and `bytes_deceq_eq`.
- `Data.Sums.Combinators`: `is_some`.
- `Capability.Parsing.Cursor`: `cursor_nat_lt`.

`Arithmetic.add`/`mul`, `Transport.cong`/`sym`/`trans`,
`LawfulClasses.bool_or`, `Compare.list_eq`, and the `OrdResult` facade are
already public and therefore absent from this set.

### 4.3 Complete higher-risk prerequisite set

Standalone elaboration failure recurs beyond Order. Four exporting modules used
by recorded items fail direct standalone checking:

1. **`Data.Numeric.Nat.Order`** — the calibrated ownership wall. The base fails
   inside `leq_nat::antisym`; complete closure is the atomic class-owner
   migration required by `33 §5.3` and `51 §7`, not a visibility patch.
2. **`Core.Classes.LawfulFunctors`** — unresolved `list_append` at
   `4204..4215`.
3. **`Data.Binary.BytesKeys`** — unresolved `trans` at `596..601`.
4. **`Capability.Parsing.Cursor`** — unresolved `bytes_nat_length` at
   `1793..1809`.

The last three are closure facts, not permission to guess an import repair.
Each requires its own standalone/ownership prerequisite before an export WP.
Thus standalone failure is systemic in this reuse slice (**4 of 10 package
providers checked**), while the orphan/foreign-attached ownership predicate is
currently measured only for the Nat-order component.

The 27 higher-risk definition items are deliberately ungrouped:

- Gcd: `subst`, `cong`, `sym`, `trans` — proof transport scaffolding.
- InsertionSort: `permutation`, `insert`, `sort` — proposition identity and
  attached `insert`/`sort` law ownership.
- Formatting.Doc: `pretty_bool_cases` — proof-relevant disjunction evidence.
- LawfulFunctors: `bool_and` — its attached monoid laws would become nonlocal.
- EmptyDec: `DecEq`, its Bool instance, `bool_eq`, `sym`, and `trans` — class,
  dictionary, and proof identity consolidation.
- Map: `bool_dichotomy`, `bool_and`, `cat4_bool_or`, `leq_nat`, and
  `total_leq_nat` — proof-returning helpers plus attached Boolean/Nat laws.
- Nat.Order: the eight identities enumerated in §4.1 — one spec-fixed atomic
  owner migration, including the orphaned dictionary and foreign attachment.

### 4.4 Proposed low-risk work groups

These groups contain only `[low]` items. They are sequencing proposals, not
source authorization; each provider prerequisite above lands first.

1. **Prelude list reuse** — LawfulFunctors `list_map`/`list_foldr`, Derived
   `map`/`filter`, and Property `gen_map_list`.
2. **Public Nat arithmetic reuse** — Gcd `add`/`mul`, Formatting.Doc
   `pretty_nat_add`, and Cursor `cursor_nat_add`.
3. **Nat-order computational consumers** — after both Nat-owner and Order-export
   prerequisites: Gcd `leq_nat`/`sub`, Diagnostics `diagnostic_nat_leq`,
   Formatting `pretty_nat_leq`, Cursor `cursor_nat_sub`, Parsing
   `nat_leq_bool`, Arguments `argument_nat_leq`, and Derived `min`/`nat_sub`.
4. **Derived-list computational reuse** — Deque append/reverse, Parsing append,
   EffectfulClasses `concat_map`, Cursor length, and Property length.
5. **Ordered-list projections** — OrderedSearch `ordered_search_leq` and the
   three low InsertionSort projections/count helpers.
6. **Boolean computational reuse** — Derived `bool_and`/`bool_leq` and Map
   `option_is_some` after their provider exports.
7. **Property equality reuse** — Property's UInt8/list/Bytes equality helpers,
   after BytesKeys standalone closure and exports.
8. **Reader-order refinement** — the 12 `BU` entries, restricted to
   reader-facing exposition and dependency-lawful fence placement; no forward
   reference or proof-owner move is implied.

The Steward may split a group per package to preserve the charter's
single-package preference. No higher-risk item may ride one of these groups.

## 5. Reproduction and validation commands

The evidence was produced from the exact branch head with:

```sh
git rev-parse HEAD origin/main

git ls-tree -r --name-only \
  ed5b4063f434cc7a2311143367928ee98f64fd86 catalog/packages \
  | grep '\.ken\.md$' | sort

source scripts/ken-env.sh
scripts/ken-cargo test -p ken-elaborator \
  --test zz_cat_reuse_census_probe -- --nocapture

scripts/ken-cargo run -p ken-cli -- check \
  catalog/packages/<provider>.ken.md
```

The identity probe was a temporary test-only instrument and was removed after
its successful run. It created no candidate path. The provider command was run
for all ten package providers in the standalone sweep; its exit codes and exact
diagnostics form the hashed provider ledger in §2.

The final coverage oracle established an exact 48-row/48-path set equality,
with no duplicate row, and recomputed the rollup as prelude `5 = 5 low + 0
higher`, sibling `58 = 31 low + 27 higher`, and arrangement `36 TD + 12 BU`.
It also established that every non-empty item cell has exactly one risk tag.

Targeted controls were green:

- `lang_mod_strict_resolution_d0`: 4 passed; log SHA-256
  `33618c221a5c4c337674292e37f40c8fcc937fa2438e40bb7ffe969851c70c4c`;
- `lang_mod_pub_eligibility`: 5 passed; log SHA-256
  `be940f2464f46ab4616dcdc7397274e63ecb374e7a5f557e2ad3c2425bb9c39c`;
- `lang_mod_catalog_realization`: 5 passed; log SHA-256
  `b69839fbb4e56b9df94101a830250ca5ed934c864967183099d41671cc4ea04c`;
- `git diff -- catalog/packages`: empty;
- `git diff --check`: clean; and
- non-table, non-fence Markdown lines over 85 columns: zero.

This document is census-only. It does not declare any package reworked, does not
close a missing export or standalone gap, and does not authorize source edits.
