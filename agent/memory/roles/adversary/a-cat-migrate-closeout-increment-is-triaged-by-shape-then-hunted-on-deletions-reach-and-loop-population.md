---
name: a-cat-migrate-closeout-increment-is-triaged-by-shape-then-hunted-on-deletions-reach-and-loop-population
description: How an M8 hunt clears a CAT-MIGRATE-TIER-C-DATA-VALUE increment (StringBijection through Vector, 2026-09-03/04). Triage the shape first (byte-identical no-op strengthening, import-adding migration, closeout with providers, provider-free closeout), byte-verify any no-op, then read every deleted line, prove each new control reaches, check a fixture strengthener really runs for the unit kind, and check what an exports loop iterates over. `.source` from extract_ken_md holds only plain ken fences, so a parse_decls census over an import-only package is empty and vacuous.
metadata:
  type: feedback
---

# A CAT-MIGRATE closeout increment is triaged by shape, then hunted on deletions, reach and loop population

The eight Tier-C (data-value) increments of CAT-MIGRATE-TIER-C-DATA-VALUE,
all NO OBJECTION:

| Inc | Package | Squash / PR | Base | Reviewed | Verdict |
|---|---|---|---|---|---|
| 1 | StringBijection | `c3bb29c81` #3284 | `7980902f8` | `d5cdd701e` | `evt_15yhm3jzjbrey` |
| 2 | StringKeys | `7e52f62f4` | `00b4d16f3` | `08503faa9` | `evt_1j1a9v8tc3qtx`, bounded obs. |
| 3 | BytesKeys | `d09af52a6` #3306 | `46433f03d` | `6937b7269` | `evt_2xzqa7fgjhjcx`, same obs. |
| 4 | Sums.Combinators | `af443017f` #3309 | `848560955` | `304687ab3` | `evt_hyxvypp28q5w` |
| 5 | Map | `1b10b86ee` #3314 | `ea0c04eec` | | see relocation lesson |
| 6 | Codec | `abf89686a` #3317 | `2b6a40f6a` | `1c098684e` | `evt_3mtcgb1ev338q` |
| 7 | Deque | `279e0951b` #3319 | `abf89686a` | `24de7bde5` | `evt_7qvsqhxbz0bq` |
| 8 | Vector | `0b40c1008` #3320 | `279e0951b` | | `evt_1p24wka1s0kzx` |

Increment 5 was a relocation, cleared under
[[a-consolidate-onto-canonical-home-migration-is-anti-fabricated-by-global-uniqueness-plus-load-bearing-imports]].
The pub-flip predecessors are in
[[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]].

## Provenance first

Increments 1 and 2 were cited by tree objects (`93fb12895`, `913ace066`), the
fifth and sixth after `aeb7ceef2`, `55aac6e55`, `a7dfb0c8b`, `625470d9f`
([[never-complete-an-abbreviated-sha-cite-rev-parse]]). Resolve by subject,
check parent equals base, first-parent `--stat` equals the shortstat, ancestor
of `origin/main`, and blob-identity each path against the reviewed candidate
(inc 1 test blob `3d45cd5a5...`, inc 3 `940739a9...`, inc 4 `a5b3bbf1ae...`,
inc 8 `ec7fb1859`). Inc 6 was a clean **rebase-forward** (candidate cut on
`1b10b86ee`, landed parent `2b6a40f6a`): per-path blob identity is then the
check, plus `git diff --name-only <old-base> <new-base> -- <paths>` empty to
show the intervening base delta did not touch the WP's paths.

## Triage the shape first; it sets the whole attack

- **No-op strengthening (inc 1).** Catalog byte-identical, whole diff is test
  control. The product axis is empty, so every pub-flip attack is vacuously
  clear and the hunt is test quality. Verify the no-op at byte level, twice:
  `git diff --name-status base..squash` shows only the test path, **and**
  `git rev-parse <squash>:<catalog-file>` equals `<base>:<catalog-file>`
  (inc 1: blob `de931aee1...` both sides).
- **Migration with a real catalog edit (inc 6).** Clear the edit (below) and
  run the untouched-loader attack.
- **Closeout of an already-migrated package (inc 2, 3, 4, 7).** Catalog
  byte-identical, census untouched (the drain happened earlier). Hunt
  non-vacuity and whether the closeout claim is consistent with the tree.
- **Provider-free closeout (inc 8).** No import edge exists to withdraw, so the
  necessity control is replaced by a positive control (below).

## Attack 1: no weakening hides in the deletions

A `+56/-9` labelled "strengthening" can smuggle a weakening in the `-` lines.
Read every one. Inc 1 replaced a containment loop
(`for name in [cong,sym,trans] { assert!(term_mentions(body, Transport.name)) }`)
with an exact set equality: the `BTreeSet` of every `Core.Logic.Transport.`
provider the certificate body mentions equals exactly `{cong, sym, trans}`.
That is strictly stronger (reddens on an extra leaked provider), and nothing
else was deleted. Containment to exact equality is the good direction; the
reverse is the finding. The prefix `"Core.Logic.Transport."` keeps its
**trailing dot**, which stops a `TransportExtra` collision. With zero deletions
(inc 2, 3, 4, 7, 8), this attack is vacuous.

## Attack 2: every new control must reach

A new `term_mentions(consumer_body, published_id)` is a differential only if
the consumer `GlobalId` exists at the SHA and its body names the asserted id.
A wrong name would panic at `env.globals[&name]` (CI red, not silent green),
but confirm reachability anyway so you know the assert exercises something.
Inc 1: `string_deceq_eq::sound` (LawfulClasses :2278) and
`string_ord_leq::antisym` (:2311) both name `string_to_list_char_injective`
(:2281/:2317), and the load target `Data.Text.StringKeys` exists.

## Attack 3: a fixture strengthener must do real work for the unit kind

Inc 1 added `env.execute_loaded_entry_checked_fences(entry).expect(...)`. That
method (`crates/ken-elaborator/src/modules.rs`) is a documented no-op for a
plain `.ken` entry. For a literate `.ken.md` unit it sets `root_scope` and runs
`execute_ken_md_checked_fences` (the Definition and every `ken reject`/
`ken example` fence). StringBijection is `.ken.md`, so it runs; on a plain
`.ken` the line would be decorative
(CHECKS.md check 7).

## Two fence paths, and why a parse_decls census can be empty

`extract_ken_md` (`crates/ken-elaborator/src/literate.rs`) starts `.source` as
an all-whitespace copy of the markdown and copies fence bytes back **only** for
plain ` ```ken ` (Compiled / Definition) fences. `ken example` and `ken reject`
(Checked) fences push their body range but never tangle into `.source`. So:

- `parse_decls(&extract_ken_md(..).source)` sees plain-fence decls only;
- the fence **executor** sees Definition, example and reject fences.

Inc 2: StringKeys' only plain fence is one `import`; its three example consts
are in a `ken example` fence. So `publication_queries()` parsed one
`ImportDecl`, returned empty, and `assert_eq!(published, BTreeSet::new())` was
empty-equals-empty. **Bounded observation, not a defect**: the same property
was pinned non-vacuously by `owned.is_empty()` plus no global under
`Data.Text.StringKeys.`, and by a hardcoded loop over
`[DecEq, Ord, string_deceq_eq, string_ord_leq, string_keys_local_name]` each
rejecting with exact `UnboundName "Data.Text.StringKeys.<name>"`. Attack 2
cleared through the executor path: test 3 measured the before/after id delta
(3 example consts, so `checked_ids` non-empty) and asserted direct LawfulClasses
use equals exactly `{string_deceq_eq, string_ord_leq}`. Flag it once; do not
re-file per increment.

Inc 3 confirmed the prediction with two safe refinements. The harness became
`package_shape()` returning `providers` / `exports` /
`non_import_declarations`, and `shape.providers == {(LawfulClasses, DecEq)}`
plus `non_import_declarations == 0` are real measurements; only the
`shape.exports` loop was vacuous. Any export BytesKeys grew would red those two
and the `owned` check, so the vacuity is fail-closed. Its `_ =>` arm counts
into `non_import_declarations`, which is asserted zero, so it is not a §7
concern.

## Which collection does the exports loop range over?

Inc 4 (Sums.Combinators) was the first **publishing** increment (one
`pub fn is_some`, line 78, among ten `fn`s), so the flag-once disposition did
not apply. Its control iterated the **13 owned surface names** and attempted a
real `env.elaborate_file("import {SUMS} ({surface} as ...)")` for each,
inserting on `Ok`. The body runs 13 times however many publish.

**Rule: a loop over the owned inventory that attempts a real loader operation
per member reaches even when the pass set is a singleton; a loop over a
precomputed exports set is vacuous exactly when that set is empty.** Read what
the `for` ranges over before trusting an exports control. Settle
publishing-versus-import-only from the catalog's `pub` decls first.

Inc 4's teeth worth copying: exact `published == {is_some}`; the negative arm
matches `Err(ElabError::UnboundName { name, .. })` with `name` equal to the
exact `{SUMS}.{surface}`, and `Err(other) => panic!`; an identity pin (a client
importing `is_some as sums_is_some` must `term_mentions` the exact owned
`GlobalId`). Test 1's inventory (33 owned names), external closure (an 11-member
compiler floor) and zero `trusted_base` delta were exact and loaded from real
globals. **A disclosed delegation must be confirmed real**: the 20 `::`
sub-identities were delegated to
`cat_bool_pub_export::class_owner_provider_loader_visible_inventories_are_exact`,
which exists and handles `Decl::AttachedProofDecl` for the same module.

## Clearing a real import edit, and the untouched-loader attack (inc 6)

Codec added `import Core.Logic.Transport (cong)` (`+2`), a new
`cat_codec_import.rs` (`+330`), and a `+7/-1` census move in
`lang_mod_strict_resolution_d0.rs`. It cleared because: the via-Codec `cong`
id equals the direct Transport id; zero trust; the sole non-floor external
identity is `{Transport.cong}`; withdrawing the import gives
`UnresolvedCon cong` (load-bearing); and the census drained Codec from the
un-migrated roster into the migrated map with residual `["Equal"]`.

The attack that only pays on this shape: **acceptance suites not in the diff
that load the edited module** (`cc2`/`cc7`/`cc8` `include_str!` Codec). Each
builds its env from a fixed ordered list, so the new import resolves only if a
predecessor already loaded the provider. Every one loads StringBijection
(which imports Transport) before Codec. Do not clear this on CI green; find
them with `git grep -l '<Module>.ken.md' <sha> -- 'crates/**/tests/*.rs'` and
check each order. An explicit import is still an improvement: a future order
violation fails closed instead of resolving silently.

## Closeout non-vacuity, forked on provider count

With providers (inc 7 Deque, already carrying
`import Data.Collections.Derived (list_append, reverse)`, `+354/-0`):

- exact owned-name set, loader results within owned ids, and `trusted_base()`
  equal to a fresh provider-closure load;
- owner id equals provider id, and the checked external identities equal
  floor plus providers;
- empty exports, and every direct surface rejected `UnboundName`;
- a **necessity** control that withholds each import item with the provider
  closure preloaded and expects `UnresolvedCon` for exactly that name (this
  also shows strict resolution is in force: under ambient passthrough the item
  would resolve).

Use the right error: `UnboundName` for a publication rejection,
`UnresolvedCon` for a body naming an unbound name. Consistency checks that
paid: the test's provider assertion equals the catalog import verbatim; the
module is in the census migrated map once and not in the un-migrated roster;
and the census's source-level residual is deliberately larger than the test's
kernel-term inventory (both carry `Equal`), which the docstrings must say.

Provider-free (inc 8 Vector, `Data/Vector/Vector.ken.md` with zero import,
pub or export lines, `+288/-0`): no import edge exists to withdraw, so a
**positive control** replaces necessity. Import a known-public name
(`cong` from Transport) through the same selective-import path and require
success, then require each of the 16 Vector surfaces to reject with its
exact-qualified `UnboundName`. Without it an all-reject test is vacuous
([[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]).
The external identity inventory equals exactly the compiler floor
`{Proved, Nat, Zero, Suc, Equal}`; `assert_ne!(compiler map, Vector.map)`
guards an owned name aliasing a compiler identity; and `trusted_base()` is
compared to the bare compiler base. Inc 8 closed the lane.

All verdicts reason-verified at the squash; no build or test (§12).
