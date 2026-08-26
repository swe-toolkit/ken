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

This is point-in-time review evidence, not a living checked corpus. Review-time
reproduction uses the stated evidence ref as the git operand. It independently
compares every local name with its exact package source and every qualified
target with its exact provider source. The seven explicitly absent prerequisite
targets are compared as an exact set. Composite families are checked for source
parentage and unique components. Provider identities are source declarations,
not substring markers or allocation-order `GlobalId` values. Each public-depth
cell records the complete source-derived state of referenced targets: exact
`public`, `private`, `absent`, and compiler `ambient` sets, plus the derived
`[ambient]`/`[all-public]`/`[mixed]`/`[absent]`/`[all-private]` tag.

The following judgments remain **review-only** rather than mechanically proved:
semantic equivalence between a local body and a provider body; whether the named
provider is the right canonical owner beyond its source identity; each package's
headline and adjacent one-line arrangement explanation; `low` versus `higher`
risk; attached-proof ownership interpretation; and whether the proposed work
groups are the best sequencing. The standalone exit results and source-order
facts are mechanically reproducible at review time, but this document ships no
persistent oracle for them.

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
| `P` | `compiler-prelude:crates/ken-elaborator/src/prelude.rs` | `[ambient]` public=-; private=-; absent=-; ambient=filter,fold,map | `[installed]` Installed before source; no package attached-owner edge |
| `A` | `Data.Numeric.Nat.Arithmetic` | `[all-public]` public=add,mul; private=-; absent=-; ambient=- | `[ok]` standalone `ken check` exit 0 |
| `T` | `Core.Logic.Transport` | `[mixed]` public=cong,sym,trans; private=subst; absent=-; ambient=- | `[ok]` standalone exit 0; attached ownership local |
| `LC` | `Core.Classes.LawfulClasses` | `[mixed]` public=bool_or; private=DecEq,DecEq-Bool,bool_and,bool_eq,bool_leq,compare_bool_cases,ord_leq_at; absent=bool_or::eq_true_of_or; ambient=- | `[ok]` standalone exit 0; current local attachments resolve |
| `N` | `Core.Classes.LawfulClasses` | `[absent]` public=-; private=-; absent=Ord-Nat,leq_nat,leq_nat::antisym,leq_nat::refl,leq_nat::trans,total_leq_nat; ambient=- | `[higher]` **higher prerequisite:** current Order rejects at `2662..2814`; canonical imports expose foreign `bool_or::eq_true_of_or`, and the instance is orphaned. The component must move atomically to the class owner |
| `O` | `Data.Numeric.Nat.Order` | `[all-private]` public=-; private=min,sub; absent=-; ambient=- | `[higher]` **higher prerequisite:** the containing module has the `N` standalone/ownership failure |
| `D` | `Data.Collections.Derived` | `[all-private]` public=-; private=Perm,concat_map,count,eq_from_ord,insert,length,list_append,reverse,sort; absent=-; ambient=- | `[ok]` standalone exit 0; no provider ownership error |
| `LF` | `Core.Classes.LawfulFunctors` | `[all-private]` public=-; private=comp; absent=-; ambient=- | `[higher]` **higher prerequisite:** standalone rejects `UnresolvedCon list_append` at `4204..4215` |
| `BK` | `Data.Binary.BytesKeys` | `[all-private]` public=-; private=bytes_deceq_eq,uint8_deceq_eq; absent=-; ambient=- | `[higher]` **higher prerequisite:** standalone rejects `UnresolvedCon trans` at `596..601` |
| `SC` | `Data.Sums.Combinators` | `[all-private]` public=-; private=is_some; absent=-; ambient=- | `[ok]` standalone exit 0 |
| `C` | `Capability.Parsing.Cursor` | `[all-private]` public=-; private=cursor_nat_lt; absent=-; ambient=- | `[higher]` **higher prerequisite:** standalone rejects `UnresolvedCon bytes_nat_length` at `1793..1809` |
| `Cmp` | `Core.Logic.Compare` | `[all-public]` public=list_eq; private=-; absent=-; ambient=- | `[ok]` standalone exit 0 |
| `OR` | `Core.Logic.OrdResult` | `[all-public]` public=OrdResult/Lt/Eq/Gt; private=-; absent=-; ambient=- | `[ok]` standalone exit 0 |

The durable identity is the qualified provider path, not an allocation-order
`GlobalId` number. Prelude identities are grounded at their Rust emission;
package identities are grounded at their source declaration and module boundary.
The table records a target identity for a current reimplementation, not a claim
that the current local declaration already reuses that identity.

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
| 30 | `Core/Logic/EmptyDec.ken.md` | — | `DecEq→LC.DecEq [higher]`; `DecEq-Bool→LC.DecEq-Bool [higher]`; `bool_eq→LC.bool_eq [higher]`; `sym→T.sym [higher]`; `trans→T.trans [higher]` | `BU` — `absurd_empty` and constructor helpers precede the duplicate class and lawful instance. |
| 31 | `Core/Logic/Or.ken.md` | — | — | `TD` — the `Or` family is the first and only checked component. |
| 32 | `Core/Logic/OrdResult.ken.md` | — | — | `TD` — the canonical result family opens the package. |
| 33 | `Core/Logic/Transport.ken.md` | — | — | `TD` — `subst` opens the package's five public-facing transport operations. |
| 34 | `Data/Binary/BytesKeys.ken.md` | — | — | `BU` — injectivity proof scaffolding precedes equality functions and lawful instances. |
| 35 | `Data/Collections/Deque.ken.md` | — | `deque_list_append→D.list_append [low]`; `deque_list_reverse→D.reverse [low]` | `TD` — the `Deque` carrier and public endpoint operations precede laws. |
| 36 | `Data/Collections/Derived.ken.md` | `map→P.map [low]`; `filter→P.filter [low]` | `min→O.min [low]`; `nat_sub→O.sub [low]`; `bool_and→LC.bool_and [low]`; `bool_leq→LC.bool_leq [low]` | `BU` — `list_append` and the combinator floor precede the package's verified sort and derived string/byte headlines. |
| 37 | `Data/Collections/Map.ken.md` | — | `option_is_some→SC.is_some [low]`; `bool_dichotomy→LC.compare_bool_cases [higher]`; `bool_and→LC.bool_and [higher]`; `cat4_bool_or→LC.bool_or [higher]`; `leq_nat→N.leq_nat [higher]`; `total_leq_nat→N.total_leq_nat [higher]` | `TD` — the `Tree` carrier and core map operations precede proof machinery. |
| 38 | `Data/Collections/NonEmpty.ken.md` | — | — | `TD` — the `NonEmpty` carrier opens the package. |
| 39 | `Data/Numeric/Nat/Arithmetic.ken.md` | — | — | `TD` — public `add` and `mul` precede their attached laws. |
| 40 | `Data/Numeric/Nat/Order.ken.md` | — | `leq_nat→N.leq_nat [higher]`; `leq_nat::refl→N.leq_nat::refl [higher]`; `leq_nat::trans→N.leq_nat::trans [higher]`; `leq_nat::antisym→N.leq_nat::antisym [higher]`; `total_leq_nat→N.total_leq_nat [higher]`; `bool_or::eq_true_of_or→LC.bool_or::eq_true_of_or [higher]`; `Ord-Nat→N.Ord-Nat [higher]`; `OrdResult/Lt/Eq/Gt→OR.OrdResult/Lt/Eq/Gt [higher]` | `BU` — the misplaced relation/proof spine precedes the headline instance and later Nat operations. |
| 41 | `Data/Serialization/Json.ken.md` | — | — | `TD` — the public `Json` carrier opens the package before its cursor instance. |
| 42 | `Data/Sums/Combinators.ken.md` | — | — | `TD` — the public sum combinator floor begins with `get_or_else`. |
| 43 | `Data/Sums/Validation.ken.md` | — | — | `TD` — the `Validation` carrier precedes its lawful instances. |
| 44 | `Data/Text/Codec.ken.md` | — | — | `TD` — `decode_utf8` is the first checked operation. |
| 45 | `Data/Text/StringBijection.ken.md` | — | — | `TD` — the conversion-layer retraction certificate is the headline prerequisite. |
| 46 | `Data/Text/StringKeys.ken.md` | — | — | `BU` — equality/order functions and proofs precede the lawful dictionaries. |
| 47 | `Data/Vector/Vector.ken.md` | — | — | `TD` — indexed `Vec` and `Fin` carriers precede total operations and laws. |
| 48 | `Tooling/Testing/Property.ken.md` | `gen_map_list→P.map [low]` | `property_list_length→D.length [low]`; `property_nat_lt→C.cursor_nat_lt [low]`; `property_uint8_eq→BK.uint8_deceq_eq [low]`; `property_list_uint8_eq→Cmp.list_eq specialized by BK.uint8_deceq_eq [low]`; `property_bytes_eq→BK.bytes_deceq_eq [low]` | `TD` — the `Gen` carrier and generator operations precede cursor examples and witnesses. |

### 3.1 Arrangement source witnesses

Review-time source inspection records declaration order from each exact package.
`TD` means the reviewed headline is the first checked declaration; `BU` means the
recorded first declaration precedes that headline. The headline choice and the
one-line explanation in §3 remain review-only. First-declaration identity,
headline existence/order, and the derived tag are mechanically reproducible
from the evidence tree at review time.

| Package | Exact first declaration | Reviewed headline | Derived tag |
|---|---|---|---|
| `Algorithm/Numeric/Gcd.ken.md` | `add` | `gcd_fuel` | `BU` |
| `Algorithm/Searching/OrderedSearch.ken.md` | `ordered_search_leq` | `search` | `BU` |
| `Algorithm/Sorting/InsertionSort.ken.md` | `ordered_leq` | `insert` | `BU` |
| `Application/CommandLine/ArgParse.ken.md` | `OptionMode` | `OptionMode` | `TD` |
| `Application/Configuration/Decoder.ken.md` | `EnvConfigOrigin` | `decode_environment_entries` | `BU` |
| `Application/Input/Schema.ken.md` | `SchemaPresence` | `SchemaPresence` | `TD` |
| `Capability/Console/Text.ken.md` | `print` | `print` | `TD` |
| `Capability/Diagnostics/Core.ken.md` | `SourceId` | `SourceId` | `TD` |
| `Capability/Diagnostics/Render.ken.md` | `diagnostic_code_string` | `diagnostic_to_doc` | `BU` |
| `Capability/Filesystem/Authority.ken.md` | `capability_read` | `capability_read` | `TD` |
| `Capability/Filesystem/Errors.ken.md` | `renderIOError` | `renderIOError` | `TD` |
| `Capability/Filesystem/Path/Posix.ken.md` | `Path` | `Path` | `TD` |
| `Capability/Formatting/Doc.ken.md` | `Doc` | `Doc` | `TD` |
| `Capability/Parsing/Cursor.ken.md` | `CursorOps` | `CursorOps` | `TD` |
| `Capability/Parsing/Decoder.ken.md` | `DecoderError` | `DecoderError` | `TD` |
| `Capability/Parsing/Numeric.ken.md` | `NumericErrorKind` | `NumericErrorKind` | `TD` |
| `Capability/Parsing/Parsing.ken.md` | `IsUtf8` | `IsUtf8` | `TD` |
| `Capability/Process/Arguments.ken.md` | `process_arguments` | `process_arguments` | `TD` |
| `Capability/Process/Environment.ken.md` | `process_environment` | `process_environment` | `TD` |
| `Capability/Process/Exit.ken.md` | `exit_success` | `exit_success` | `TD` |
| `Capability/Process/WorkingDirectory.ken.md` | `process_working_directory` | `process_working_directory` | `TD` |
| `Capability/System/Buffer.ken.md` | `buffer_window` | `buffer_window` | `TD` |
| `Capability/System/IO.ken.md` | `write_all_terminates` | `write_all_terminates` | `TD` |
| `Capability/System/Resource.ken.md` | `resource_body_success` | `resource_bracket_succeeded` | `BU` |
| `Capability/Time/WallClock.ken.md` | `instant_nanoseconds` | `instant_nanoseconds` | `TD` |
| `Core/Classes/EffectfulClasses.ken.md` | `apply_to` | `Applicative` | `BU` |
| `Core/Classes/LawfulClasses.ken.md` | `IsTrue` | `IsTrue` | `TD` |
| `Core/Classes/LawfulFunctors.ken.md` | `Semigroup` | `Semigroup` | `TD` |
| `Core/Logic/Compare.ken.md` | `pair_compare` | `pair_compare` | `TD` |
| `Core/Logic/EmptyDec.ken.md` | `absurd_empty` | `DecEq` | `BU` |
| `Core/Logic/Or.ken.md` | `Or` | `Or` | `TD` |
| `Core/Logic/OrdResult.ken.md` | `OrdResult` | `OrdResult` | `TD` |
| `Core/Logic/Transport.ken.md` | `subst` | `subst` | `TD` |
| `Data/Binary/BytesKeys.ken.md` | `uint8_to_int_injective` | `uint8_deceq_eq` | `BU` |
| `Data/Collections/Deque.ken.md` | `Deque` | `Deque` | `TD` |
| `Data/Collections/Derived.ken.md` | `list_append` | `View` | `BU` |
| `Data/Collections/Map.ken.md` | `Tree` | `Tree` | `TD` |
| `Data/Collections/NonEmpty.ken.md` | `NonEmpty` | `NonEmpty` | `TD` |
| `Data/Numeric/Nat/Arithmetic.ken.md` | `add` | `add` | `TD` |
| `Data/Numeric/Nat/Order.ken.md` | `leq_nat` | `Ord-Nat` | `BU` |
| `Data/Serialization/Json.ken.md` | `Json` | `Json` | `TD` |
| `Data/Sums/Combinators.ken.md` | `get_or_else` | `get_or_else` | `TD` |
| `Data/Sums/Validation.ken.md` | `Validation` | `Validation` | `TD` |
| `Data/Text/Codec.ken.md` | `decode_utf8` | `decode_utf8` | `TD` |
| `Data/Text/StringBijection.ken.md` | `string_to_list_char_injective` | `string_to_list_char_injective` | `TD` |
| `Data/Text/StringKeys.ken.md` | `string_deceq_eq` | `DecEq-String` | `BU` |
| `Data/Vector/Vector.ken.md` | `Vec` | `Vec` | `TD` |
| `Tooling/Testing/Property.ken.md` | `Gen` | `Gen` | `TD` |

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
Thus standalone failure is systemic in this reuse slice (**4 of 11 package
provider modules checked**), while the orphan/foreign-attached ownership predicate is
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

These groups contain every recorded `[low]` item exactly once. Their exact
`package#local-name` membership is mechanically recountable from §3 during
review. The grouping itself is a review-only sequencing proposal, not source
authorization, and each provider prerequisite lands first. The Steward may split
a group per package to preserve the charter's single-package preference. No
higher-risk item is proposed in these groups.

1. **Prelude and functional-floor reuse**
   - `Core/Classes/LawfulFunctors.ken.md#list_map`
   - `Core/Classes/LawfulFunctors.ken.md#list_foldr`
   - `Data/Collections/Derived.ken.md#map`
   - `Data/Collections/Derived.ken.md#filter`
   - `Tooling/Testing/Property.ken.md#gen_map_list`
   - `Core/Classes/EffectfulClasses.ken.md#compose`
2. **Public Nat arithmetic reuse**
   - `Algorithm/Numeric/Gcd.ken.md#add`
   - `Algorithm/Numeric/Gcd.ken.md#mul`
   - `Capability/Formatting/Doc.ken.md#pretty_nat_add`
   - `Capability/Parsing/Cursor.ken.md#cursor_nat_add`
3. **Nat-order computational consumers**
   - `Algorithm/Numeric/Gcd.ken.md#leq_nat`
   - `Algorithm/Numeric/Gcd.ken.md#sub`
   - `Capability/Diagnostics/Core.ken.md#diagnostic_nat_leq`
   - `Capability/Formatting/Doc.ken.md#pretty_nat_leq`
   - `Capability/Parsing/Cursor.ken.md#cursor_nat_sub`
   - `Capability/Parsing/Parsing.ken.md#nat_leq_bool`
   - `Capability/Process/Arguments.ken.md#argument_nat_leq`
   - `Data/Collections/Derived.ken.md#min`
   - `Data/Collections/Derived.ken.md#nat_sub`
4. **Derived-list computational reuse**
   - `Data/Collections/Deque.ken.md#deque_list_append`
   - `Data/Collections/Deque.ken.md#deque_list_reverse`
   - `Capability/Parsing/Parsing.ken.md#list_append`
   - `Core/Classes/EffectfulClasses.ken.md#concat_map`
   - `Capability/Parsing/Cursor.ken.md#cursor_list_length`
   - `Tooling/Testing/Property.ken.md#property_list_length`
5. **Ordered-list projections**
   - `Algorithm/Searching/OrderedSearch.ken.md#ordered_search_leq`
   - `Algorithm/Sorting/InsertionSort.ken.md#ordered_leq`
   - `Algorithm/Sorting/InsertionSort.ken.md#order_eq`
   - `Algorithm/Sorting/InsertionSort.ken.md#element_count`
6. **Boolean computational reuse**
   - `Data/Collections/Derived.ken.md#bool_and`
   - `Data/Collections/Derived.ken.md#bool_leq`
   - `Data/Collections/Map.ken.md#option_is_some`
7. **Property support reuse**
   - `Tooling/Testing/Property.ken.md#property_nat_lt`
   - `Tooling/Testing/Property.ken.md#property_uint8_eq`
   - `Tooling/Testing/Property.ken.md#property_list_uint8_eq`
   - `Tooling/Testing/Property.ken.md#property_bytes_eq`

### 4.5 Reader-order follow-on

The 12 `BU` entries form a separate reader-facing exposition follow-on. It is
restricted to dependency-lawful fence placement and implies no forward
reference or proof-owner move.

## 5. Review-time reproduction

This one-time sizing artifact deliberately has **no checked-in test or script**.
A reviewer reproduces its mechanical claims from the exact git object tree; a
prose-only change must not red a software test. The population and source-stasis
checks are:

```sh
EVIDENCE=ed5b4063f434cc7a2311143367928ee98f64fd86

git ls-tree -r --name-only "$EVIDENCE" -- catalog/packages \
  | grep -E '^catalog/packages/.*\.ken\.md$' \
  | LC_ALL=C sort > /tmp/cat-reuse-packages.txt
wc -l /tmp/cat-reuse-packages.txt
sha256sum /tmp/cat-reuse-packages.txt

git diff --exit-code "$EVIDENCE" HEAD -- catalog/packages
```

At this candidate those commands report 48 paths, digest
`50ca4604db917bf7e2758e075c269c064ac181f871c246bea73d0d4b7e197333`,
and an empty catalog diff. Reviewers independently compare §3's path set with
that list, recount the rollups, and inspect local/provider declarations,
visibility, component parentage, and declaration order with `git show` against
`$EVIDENCE`. Prelude identities are inspected at the exact Rust producer named
in §2.

Standalone package-provider results are reproduced with targeted behavior:

```sh
source scripts/ken-env.sh
scripts/ken-cargo run -p ken-cli -- check \
  catalog/packages/<provider>.ken.md
```

The standalone sweep yielded the states recorded in §2 and §4.3. The exact
failure spans are diagnostic observations at the evidence ref, not stable source
coordinates. Reviewers judge the explicitly review-only properties listed in
§1 from the source and record their independent conclusion; this document does
not represent those judgments as machine-checked.

The remaining candidate checks are `git diff --check`, Markdown width, exact
one-file merge scope, and an empty `catalog/packages/` delta. No replacement
corpus-text oracle is part of the deliverable.

This document is census-only. It does not declare any package reworked, does not
close a missing export or standalone gap, and does not authorize source edits.
