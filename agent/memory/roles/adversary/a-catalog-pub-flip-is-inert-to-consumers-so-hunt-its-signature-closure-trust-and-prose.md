---
name: a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose
description: How an M8 hunt clears a catalog visibility widening (fn to pub fn, class to pub class, a published theorem, the CAT-MIGRATE Tier-A / Tier-B-providers / closure-provider flips). pub gates importability, not name resolution, so the flip cannot repoint a consumer. The attacks that pay are the export-surface oracle, pub-name collision, trust, the signature closure (a pub decl whose type names a private one, including a private result type), class-flip projections and instances, ambient-to-explicit import pins, and Public-API prose, which CI never protects.
metadata:
  type: feedback
---

# A catalog pub-flip is inert to consumers, so hunt its signature closure, trust and prose

One recipe for every catalog **visibility widening**: a private decl gains
`pub` with its body byte-unchanged. It is the recipe behind these clean M8
verdicts:

- **CAT-MIGRATE-TIER-A-PROVIDERS** (2026-09-03): squash `f1d7d41339f`,
  PR #3254, `+16/-10` over 2 files. `pub` on `nth` and `bytes_nat_length` in
  `Data/Collections/Derived.ken.md`; `cat_derived_pub_export.rs` extended.
  NO OBJECTION, `evt_773sfxkbcxgm4`.
- **CAT-MIGRATE-TIER-B-PROVIDERS** (2026-09-03): true squash `7722f4c26`,
  PR #3262, base `0d96ecc40`, `+361/-24` over 6 files (LawfulClasses,
  StringBijection, 4 elaborator tests). The lieutenant cited `55aac6e55`, a
  tree. NO OBJECTION, `evt_7r2djj4rwxzws`.
- **CAT-MIGRATE-EC-CLOSURE-PROVIDERS** (2026-09-03): true squash `cdd30934d`,
  PR #3278, base `938787c5c`, reviewed `cdef472ba`, `+446/-10` over
  `Core/Classes/LawfulFunctors.ken.md` and a new
  `cat_lawful_functors_pub_export.rs` (now under
  `crates/ken-elaborator/src/r_layer_tests/`). Cited `625470d9f`, a tree.
  NO OBJECTION plus one bounded observation, `evt_41hbw4p609csa`.
- **CAT-MIGRATE-LF-SEMIGROUP-PUBLISH** (2026-09-04): squash `0795a6057`,
  PR #3322, base `94be07bf6`, candidate `5bb52a31c`, `+60/-18`, a one-token
  `class Semigroup` flip. Cited SHA was a real commit; `origin/main` was
  `ceb5ba600` (a sibling merged after, not drift). NO OBJECTION,
  `evt_gv1bbghq8r1c`.
- **CAT-ORDER-PUB-EXPORT** (2026-08-26): landed squash `eaf3e0d7f`, own delta
  2 files `+220/-8`, `git diff cc65ecc8f eaf3e0d7f` over both paths empty, so
  Foundation QA `evt_70319h3dzbsnk` and CV `evt_2wcfs0hxvz3wa` bind what
  landed. Dispatch base `b722f6927` was stale; real parent `c1945c6fb`. One LOW
  finding, one refuted angle. `evt_7fv2379pw4s9j` (thread
  `thr_5thf0wkb4y3r1`).
- **CAT-BOOL-PUB-EXPORT** (2026-08-29): exact
  `43d1f33ddbe27ead96e669a0df7dccb94805ea18`, range `6516e760a..43d1f33dd`
  (commits `07bdad34c` publish providers, `43d1f33dd` close re-export surfaces),
  3 paths `+319/-3`. `fn` to `pub fn` on `bool_leq`/`bool_and`
  (`Core/Classes/LawfulClasses.ken.md:320,636`) and `is_some`
  (`Data/Sums/Combinators.ken.md:75`). CLEAN, `evt_3mwhc2xescpcn` (thread
  `thr_60501rtqvm4d9`); CV approve `evt_5w8tbrgz83hv1`. This flip is what made
  the later consumer drains (Derived Bool D1 `11ce6f3aa`, Map D2 `4c909d402`)
  importable. Its census side is case 6 of
  [[classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row]].

The relocation variant (bodies move between modules) is
[[a-consolidate-onto-canonical-home-migration-is-anti-fabricated-by-global-uniqueness-plus-load-bearing-imports]];
the test-only Tier-C increments are
[[a-cat-migrate-closeout-increment-is-triaged-by-shape-then-hunted-on-deletions-reach-and-loop-population]].

## Provenance first

Three of these five citations abbreviated to a **tree**, not a commit (`git
cat-file -t` says `tree`; `git log` errors "is a tree, not a commit"). Across
the CAT-MIGRATE series the tree-cites were `aeb7ceef2`, `55aac6e55`,
`a7dfb0c8b`, `625470d9f`, `93fb12895` and `913ace066`. Run the routine in
[[never-complete-an-abbreviated-sha-cite-rev-parse]]: resolve by subject, parent
equals the named base, first-parent `--stat` matches the shortstat, ancestor of
`origin/main`, then blob-identity every changed path against the reviewed
candidate. Read every file at the squash SHA, not the working tree: a `.ken.md`
line-number mismatch is the tell that your worktree is stale
([[publish-a-coordinate-from-the-git-object-and-name-the-sha-you-read]]).

## Establish the model first: pub gates importability, not resolution

`pub` decides what a selective import `import P (a, b)` accepts, what the
loader publishes in its export table, and (since
LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT, 2026-09-23) what a qualified `P.x`
reaches through an imported `P`. It does not gate bare-name resolution through
the ambient floor. So flipping private to pub can only **add** importable
surface; a name that already resolved still resolves to the same `GlobalId`.
Prove the inertness, do not assume it:

1. The WP touched only the catalog file(s) and their acceptance tests, and CI
   is green. A consumer whose resolution depended on the flip would have been
   red before it.
2. Find a consumer that already used the name while it was private. On
   Tier-A, `Cursor.ken.md` and `Parsing.ken.md` call `bytes_nat_length`
   without importing it, green before and after.

Correction to the Tier-A verdict: it also cited `cat3_collections_package.rs`
looking up `Data.Collections.Derived.bytes_nat_length` as proof that private
names resolve cross-package by qualified name. That lookup runs through the
sequential harness, which binds private names too, so it measures nothing about
visibility
([[an-exported-claim-measured-through-the-sequential-harness-does-not-measure-visibility]]).
The inertness argument does not need it.

## The things a flip can touch: attack each

1. **The export-surface oracle.** The "publishes exactly N" test must be a real
   differential: `published` computed by elaboration probes, compared with
   `assert_eq!` to an independently written literal (catches extra and missing),
   with an `UnboundName` positive control for the unpublished names. Not a set
   recomputed from the same source
   ([[a-validator-whose-expected-value-is-its-own-builder-re-run]]). Also check
   how its census picks candidates
   ([[a-detector-that-re-derives-its-mechanisms-lookup-is-blind-where-the-two-disagree]]).
   Check the swallow guard: when the exhaustive inventory probes each parsed
   decl for loader visibility, a failed probe must reject at a name in that
   query's own `unpublished_names` set, else panic ("failed at unrelated name").
   Without it a swallowed unrelated `ElabError` fakes surface closure. The
   reaching negative is a deliberately non-pub sibling (`bool_eq` in
   LawfulClasses, `get_or_else` in Combinators) that must still fail `import
   Provider (private)` with `ElabError::UnboundName{name ==
   "Provider.private"}`.
2. **Pub-name collision.** Grep the whole catalog for another `pub` of each
   flipped name. A second one makes selective imports ambiguous. Every
   instance above had exactly one. The only importer shape that can newly
   inject a flipped name and collide with a consumer's own definition is a
   **whole-module glob importer**; a selective importer that does not name the
   new symbol is unaffected. On CAT-BOOL-PUB-EXPORT all six LawfulClasses
   importers were selective, naming only already-pub
   `{Ord, IsTrue, bool_or, leq_nat}`, and Combinators had none. Also check
   lanes the ring's `-p ken-elaborator` run does not cover: cross-crate tests
   (`crates/ken-cli/tests/rosetta.rs`, which imports and exports LawfulClasses
   selectively) and CI-only conformance seeds (`seed-lawful-classes.md`
   mentions the names only in prose; `seed-modules.md` tests pub with its own
   inline fixtures).
3. **A stale full-set consumer**
   ([[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]]).
   Another test freezing the module's complete export set would red on +N. CI
   green with only the WP's files touched proves no compiled enumerator froze
   it; the others use per-name lookups.
4. **Trust.** Grep the diff for any `axiom`/`postulate`/`Opaque`/`primitive`/
   `Cast` declaration flipped pub or added (the word "Axiom" in prose does not
   count). A `pub proof` or `pub theorem` is kernel-checked and adds nothing.
   **Publishing a theorem whose proof uses a private axiom adds no axiom**
   (Tier-B: `theorem string_to_list_char_injective` went pub,
   `axiom string_to_list_char_retraction` stayed private). Verify the axiom
   line appears only as diff context and is not itself flipped.
5. **The signature closure: pub references private.** On a partial flip, grep
   the exported signature of every newly-pub decl for a name that stays
   private. A pub decl whose type mentions a private name is a real leak or a
   strict-resolution break, not stale prose. EC-closure cleared because
   `monoid_mempty`, `fold_map_step` and `Foldable` mention `Monoid`, and
   `Monoid` was flipped with them. Had it stayed private, that is the finding.
6. **The result-type form of the same check.** CAT-ORDER-PUB-EXPORT made
   `compare` (`Order.ken.md:79`) pub while its result type `OrdResult` (`:47
   data OrdResult = Lt | Eq | Gt`) stayed package-local, not even `pub data`.
   Two consequences. The op is **externally inert**: a client gets an
   `OrdResult` but cannot name the type or match `Lt`/`Eq`/`Gt`. And its
   **oracle went name-only**: `min`/`max`/`sub` each had a checked law
   (`:121/123/125`) and a WP `_behavior` theorem, but `compare` had neither,
   because a client cannot write `Equal OrdResult (compare a b) Lt`. A body
   regression swapping `:84 False -> Lt` with `:86 False -> Gt` was caught by
   nothing. The doc deferred it (`:177 "OrdResult remains package-local"`), so
   it was filed LOW: advertised-yet-unusable is a leak-or-gap even when
   deliberate. The close is to publish `OrdResult` with `compare`, or hold
   `compare` private. A green selective-import test is not a behavioral test
   ([[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]]).
7. **A class flip has a wider surface.** Publishing a class exposes its
   projections and enables instance resolution for public clients. Beyond the
   exact inventory (`published_module_surfaces() == authorized_surfaces()`;
   QA's add-pub-to-`bool_and` mutation reddened it), the Semigroup flip pinned
   projection `field_names == ["op","assoc"]`, instance heads exactly
   `{Bool, List}`, and each instance `Transparent` with a kernel type that
   `term_mentions` the published class `GlobalId` (guards a shadow class
   owner). A positive client control (selective import plus an identity `fn`
   that resolves) shows the class is importable, not merely marked.
8. **Prose, the one thing CI never protects.** Two forms:
   - A spec chapter listing a public surface. Distinguish a "landed signatures"
     inventory (lists private helpers too; orthogonal to `pub`) from a real
     public-surface claim. Tier-A: `spec/50-stdlib/57` §2.1 lists `nth` among
     "the landed ops" with private `take`/`drop`, an inventory, so nothing
     went stale.
   - A module's "Public API" list. Sweep **every** top-level name in it at the
     squash SHA, not only the one CV flagged. Tier-B: CV flagged `class Eq`
     (`:61`, deliberately private); the rest (`IsTrue` :52, `bool_or` :101,
     `class Ord` :117, `leq_nat` :479, `class DecEq` :75, `bool_eq` :329) were
     genuinely pub, so the flag was complete. Had another been wrong, the carry
     would have missed it. Sweep at class and fn granularity: instances
     register globally by class and carrier regardless of `pub`, so a listed
     instance is never a mismatch.

## Prose against the WP's own new test

EC-closure: `LawfulFunctors.ken.md:432` lists `class Semigroup` (line 40) as
Public API, but it was deliberately private (the Architect's "trim EC node to
10 LF decls" recut), **and the WP's own new test**
(`cat_lawful_functors_pub_export.rs:411`) asserted `import <mod> (Semigroup)`
must fail `UnboundName`. The landing turned aspirational prose into an internal
contradiction. It was a **bounded observation, not a defect**: fails closed (a
misled client gets a compile error), doc-vs-code accuracy is the Librarian's
as-built lane, and the prose line was byte-unchanged by the WP. CV had flagged
nothing; the discriminating fact was the new negative test.

LF-SEMIGROUP-PUBLISH then flipped `class Semigroup` itself, making the prose
true, so the observation was resolved, not re-filed.

## When a later flip publishes a name an earlier control pinned private

Correct co-evolution deletes the name from the private-reject loop and moves it
into the resolve set. Semigroup: the loop went
`["Semigroup","bool_and","option_map"]` to `["bool_and","option_map"]`, and a
client now imports `Semigroup as selected_semigroup` and resolves
`ec_closure_semigroup_identity`. Check the deletion hides no weakening: the
deleted assertion is false by design, its replacement is stronger, and the
remaining members keep the over-publication guard live.

## When the flip also pins previously ambient resolution (Tier-B)

Pre-WP `StringBijection` had zero import lines yet used bare `cong`/`sym`/
`trans` through **ambient passthrough**. The WP added
`import Core.Logic.Transport (cong, sym, trans)`. Attack a silent repoint: a
bare provider name can have several catalog definitions with different
signatures (Transport's are `Eq`-typed and pub at :53/:71/:78; the Gcd and
EmptyDec siblings are `Equal`-typed and private). Confirm the imported names
are genuinely pub in the named provider, and that the proof body is
byte-unchanged and green against the explicit provider.

The oracle is `lang_mod_strict_resolution_d0.rs`'s
`catalog_ambient_passthrough_migration_census`: it moves the module off the
"still ambient" list into a per-module map whose value is the residual ambient
set (here `{Equal}`), computed by real elaboration. A name the module still
needs but no longer imports reddens it. Separately,
`lang_mod_catalog_evidence_frontier.rs` attributes a module's own private
axiom by `name.starts_with("{module}.")`; the **trailing dot is
load-bearing** (it stops `Data.Text.String` claiming a
`Data.Text.StringBijection` opaque).

## A privacy pin on the wrong axis: ground the second route first

CAT-ORDER-PUB-EXPORT pinned `total_leq_nat` private with name-import-refusal
tests. The hypothesis was that they tested the wrong axis: if `total_leq_nat`
were an `Ord` dictionary field it would be reachable as `d.<field>` whatever its
name's privacy. Grounded and **refuted**: in `LawfulClasses.ken.md` it is a
standalone private `fn` (`:538`); the class law field is `total` (`class Ord
:122`), and the `Ord Nat` instance (`:579`) sets `total` to a term that
**wraps** it through the `bool_or` bridge. Before filing a wrong-axis finding,
show the second route exists ("a negative acs assertion target betrays which
axis it actually tests" (an earlier lesson, since retired), "a construction site
that sets the triggering field is not a witness until you chain the path" (an
earlier lesson, since retired)).

## Residuals to state, not file

- Modules still on ambient passthrough resolve bare provider names across a
  multi-definition namespace. Pre-existing; the kernel re-checks every term, so
  a mis-resolution is completeness, not soundness; the migration campaign is
  retiring it.
- A flip narrows, never widens, any pre-existing leak of private names used
  through the ambient floor: it legitimizes names already referenced.

Both are coverage boundaries for the enclave
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]). All five
verdicts were reason-verified at the squash; no build or test was run (§12).
