---
name: a-consolidate-onto-canonical-home-migration-is-anti-fabricated-by-global-uniqueness-plus-load-bearing-imports
description: How an M8 hunt clears a migration that homes one canonical definition and retires or removes the old copies (LANG-MOD-OR-CANONICAL-HOME, the Component B consolidation, CAT-MIGRATE-TIER-B-CLASSES relocation, Map increment 5, LF-BOOL-AND-CONSOLIDATION). Close the second-identity class globally (exactly one definition, old source deleted not shadowed, bare use fail-closed), verify moved bodies by hash, check denotation on the axis the type is for (sort, Equal as the Type0 alias of Eq, transparent bridges, instance fields), and verify the witness harness makes each import load-bearing, including which identity a partial restore lets move.
metadata:
  type: feedback
---

# A consolidate-onto-canonical-home migration is anti-fabricated by global uniqueness plus load-bearing imports

A **consolidation** homes one canonical definition, points N previously
duplicated or prelude-provided consumers at it, and retires the old source. A
**relocation** is the same thing done by moving source rows between catalog
modules. Both are more dangerous than a pub-flip
([[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]];
Tier-A `f1d7d41339f`, Tier-B-providers `7722f4c26`): a flip leaves bodies in
place, while removal and consolidation can silently **repoint** a consumer.

Instances, all clean:

- **LANG-MOD-OR-CANONICAL-HOME, NODE B** (2026-08-24): `db9129100`, Steward
  adversary gate. Homed `Core.Logic.Or`, migrated seven consumers, retired the
  Rust-prelude `Or`/`Inl`/`Inr`. `evt_h7eerxtavymp` (thread
  `thr_4g49g6pqvhq7x`). Respin `5d5266e9` below.
- **Component B partial** (2026-08-24): `a5b44c8f`, Steward §10a gate. Homed
  `Core.Logic.Compare` and `Core.Logic.OrdResult`; 30 fixtures alias the real
  providers. One latent non-blocking observation. `evt_2j8cfgj1q7gte` (thread
  `thr_30a5d8w5zme41`).
- **CAT-MIGRATE-TIER-B-CLASSES** (2026-09-03): true squash `82f5de01e`,
  PR #3271, base `d55a7e66e`, reviewed `731690d3f`, 10 files `+587/-363`
  (LawfulClasses, EmptyDec, BytesKeys, StringKeys, 6 elaborator tests). Cited
  `a7dfb0c8b`, a tree. NO OBJECTION, no bounded observation,
  `evt_5pncmkv83pjtt`.
- **CAT-MIGRATE-TIER-C-DATA-VALUE increment 5, Map** (2026-09-04): squash
  `1b10b86ee`, PR #3314, base `ea0c04eec`. NO OBJECTION, `evt_4khshgtrgxq9b`.
- **CAT-MIGRATE-LF-BOOL-AND-CONSOLIDATION** (2026-09-04): re-spin `bb52018ce`,
  PR #3325, base `86ca423ed`, reviewed `d1e35868e`, `+278/-128` over 13 files.
  The `d1e35868e..bb52018ce` diff was pure base drift (candidate cut on old
  base `c720f87be` via parent `138cbbc35`) with an empty intersection onto the
  13 paths. NO OBJECTION, `evt_6ky5r7pt8yetc`.

## The three fabrication classes

The Steward named them on NODE B: (1) a laundered **second identity** (a
consumer resolving to a `GlobalId` other than the canonical one, or a
surviving private or prelude copy); (2) a **non-atomic retirement** (the old
source still reachable, or a bare name resolving with no legal import); (3) a
migration that only **appears** load-bearing (an import deletable with the
consumer still green).

## Close classes 1 and 2 globally, not consumer by consumer

1. **Exactly one definition.** Grep the whole tree for the definition form
   (`^\s*data\s+Or\b` across `catalog` and `crates`), discounting test-local
   inline strings. For a relocation, run the census yourself at the squash:
   `git grep -lF <name> -- 'catalog/**/*.ken.md'` for each moved name must hit
   **exactly one** file, the owner. Catalog resolution is by name, so after a
   verbatim move the only repoint risk is a second surviving definition, and a
   one-file census proves there is none. That is stronger than "the fixtures
   assert exactly one": you measured the tree.
2. **Deleted, not shadowed.** NODE B: `prelude.rs::register_prelude` lost its
   whole `declare_inductive` + `globals.insert("Or"/"Inl"/"Inr")` block, a src
   sweep for residual `insert("Or")`/`"Inl"`/`"Inr"` was empty, and nothing
   referenced the retired local id (a surviving `PreludeEnv.or_id` would break
   the build, but grep it). Tier-B-classes: source rows were removed, and
   EmptyDec's byte-identical duplicate `class DecEq`/`fn bool_eq`/
   `instance DecEq Bool` was retired for a real LC import.
3. **Bare use fails closed.** The name is not in `is_unshadowable_kernel_name`
   (only `Omega`/`Refl`/`Axiom` are), so a bare unimported use hits
   `resolve_ref`'s strict reject, `UnboundName`.

One definition, atomic retirement and fail-closed bare use make a second
identity unconstructible, so every un-asserted consumer is covered.

The global grep misses a **renamed** re-mint. LF-BOOL-AND pinned that in-suite:
`transparent_kernel_equivalents` in `cat_lawful_functors_pub_export.rs` runs
`convert_type` and `convert` against each LC provider over LF-owned ids and
must return empty, and it panics if the provider is not transparent. Its
per-provider binding matrix (Semigroup `[T,T,F,F]`, Monoid `[T,T,T,T]`)
confirms which fields bind LC ids. Map's owner control
`cat_map_bool_and_owner.rs` does the same at test level: Map resolves to the
exact LC `GlobalId` (reference-counted on `Term::Const`), no Map-local
transparent def is convertible to `bool_and`, zero trust delta, and both import
withdrawal and a wrong-name alias red at `UnresolvedCon bool_and`.

## Verify a relocation's moved bodies by hash, not by eye

Extract the removed ken bodies (diff `-` lines minus marker) and the added ones
(`+` lines minus marker **and** minus a leading `pub `), then
`grep -vxF -f added_nopub removed`: every removed line not re-added modulo
`pub` must be pure prose, no code. Hash one representative proof on both sides
after stripping `pub ` (Tier-B-classes: `uint8_to_int_injective`, md5
`c6fae02f...` at BytesKeys@parent and LC@squash). Reverse the grep: the only
owner-added lines with no source in the removals were two import lines and
prose.

## Denotation on the axis the type is for

A consolidation can elaborate and still change what the type means.

- **Sort.** `Or` must stay `Type`-sorted: an `Omega`-sorted two-constructor
  family is proof-irrelevant and erases the `Inl`/`Inr` tag. NODE B's suite
  reduces real `total_leq_nat` results through `Inl`/`Inr` to opposite `Bool`
  tags.
- **Equal is the Type0 alias of Eq.** EmptyDec swapped inlined `Equal`-typed
  `sym`/`trans` (`p : Equal ty x y`) for Transport's `Eq`-typed ones with the
  consumer `dec_eq_decides` unchanged. It looks like a mismatch that should
  red, but Transport's prose fixes `Equal` as the standard alias of the
  kernel-native `Eq`, level-fixed at `Type0`. At ground carriers (Bool, UInt8,
  Bytes, String) they are definitionally identical, and the consumer is
  kernel-checked against `Dec(Equal a x y)` regardless. When a WP swaps a local
  proof for an imported one of a different-looking equality family, check
  whether one is a definitional alias or restriction of the other before
  calling it a repoint.
- **A transparent bridge.** Map's removed `true_intro`
  (`(ha : Equal Bool a True) (hb : Equal Bool b True) : Equal Bool
  (bool_and a b) True`) was repointed to LC's `intro`
  (`IsTrue a -> IsTrue b -> IsTrue (bool_and a b)`). Same proposition, because
  `pub fn IsTrue (b : Bool) : Prop = Equal Bool b True` is delta-transparent.
  Confirm the bridge is a transparent `fn`, not a `postulate` or opaque; that
  is what makes the collapse sound rather than a laundered axiom.
- **An instance-field repoint is checked by elaboration for free.** LF's Monoid
  fields `left_unit`/`right_unit` now take LC's `left_identity`/
  `right_identity`. Supplying the wrong one fails `.expect(...)`: `left_unit`'s
  type is `Equal Bool (bool_and True x) x`, and `right_identity` proves
  `Equal Bool (bool_and x True) x`, which is not convertible. At source, LC's
  two proofs state literally the two field propositions.

## Trust and certificate imports

Grep the whole `+` surface for a new `axiom`/`postulate`/`primitive`/`Opaque`
declaration (prose does not count). Tier-B-classes' irreducible certificates
(`uint8_int_retract`, `bytes_list_roundtrip`, `string_to_list_char_injective`)
pre-existed and moved verbatim. A cross-package certificate the owner now
imports must be genuinely `pub` in its home (`string_to_list_char_injective`
is, in StringBijection), else importing it is a separate leak.

## Ambient-to-explicit pins and "import collisions"

BytesKeys@parent had zero import lines; its relocated proofs used bare
`sym`/`trans`/`cong` through ambient passthrough, and the relocation pinned them
to explicit Transport imports. Catalog-wide bare providers were two (Transport,
pub, `Eq`-typed; Gcd, private, `Equal`-typed); the moved bodies are
kernel-checked against their stated results, so a mis-resolution fails the
kernel. A claimed **import collision** is not real unless the owner defines a
**bare** name of its own: LC only imports `cong`/`sym`/`trans`, and namespaced
proof fields (`leq_nat::trans`, `string_ord_leq::trans`) do not collide with a
bare `trans`. The census oracle `lang_mod_strict_resolution_d0` is a real
differential (drives `elaborate_module_from_roots` with `Term::Const` identity
asserts).

Sweep Public-API prose for every entry the reconcile removed and added, not
only CV's flagged one. Tier-B-classes: removed `class Eq`, `instance Eq Int`,
`instance Eq Bool` were genuinely non-pub; added `uint8_deceq_eq`,
`bytes_deceq_eq`, `string_deceq_eq`, `string_ord_leq`,
`uint8_to_int_injective` genuinely pub.

## The witness harness is what makes class 3 non-vacuous

Per-consumer asserts confirm denotation, and they are sound only with this
harness shape (`tests/support/catalog_or.rs`):

- snapshot provider-only module state, asserting the bare name is not in
  `globals` (the provider registers only `Core.Logic.Or.Or`);
- elaborate the consumer's dependencies;
- **restore**: reset `module_state` and drop the bare `Or`/`Inl`/`Inr`;
- elaborate the **real** consumer via `include_str!` of its `.ken.md`;
- assert the consumer witness's `applied_head(ty)` is `IndFormer { id }` with
  `id == globals["Core.Logic.Or.Or"]`.

The restore-erase removes a bare binding a sibling dependency leaked, so a
missing import reds. Loading real source discharges the gap the harness itself
states ("cannot establish that the caller loaded a real source"). **Absent
either, the pin is vacuous, and that absence is the finding.**

### When the restore is partial: ask which identity moved

On Component B, `restore_core_logic_or_module_state` dropped `Or`/`Inl`/`Inr`
but not the `OrdResult`/`Compare` bare aliases the harness had inserted
(`env.globals.insert("OrdResult", canonical_id)`, and so on).
`ds2_ord_nat_acceptance.rs` then loaded the **held-legacy**
`Data/Numeric/Nat/Order.ken.md`, whose own `data OrdResult = Lt | Eq | Gt`
overwrote the bare aliases with its own ids. A finder called it a violation of
"one canonical identity". The discriminator:

- **Qualified canonical untouched, only the bare test alias overwritten**: a
  latent test-env false-pass hazard. Report non-blocking and name the fix
  (scope the aliases out before the held load, or extend the restore). Here
  `Core.Logic.OrdResult.OrdResult` stayed unique and no assertion read
  `OrdResult` after the held load.
- **The qualified canonical moved, or a production consumer now resolves to the
  held identity**: a real identity defect.

The tell for test-env-only: the failing scenario needs an assertion nobody has
written yet. This recurs wherever a migration aliases into a flat scope while a
held prerequisite still ships its own copy.

Also verified on Component B: two foreign attached proofs (`proof eq_sound`/
`lt_asym` **for** `pair_compare`) became LC-local theorems with byte-unchanged
bodies, `antisym` calls them by plain name (same proof term), and neither is in
`trusted_base`. Its acceptance control used `after.is_superset(before)` for the
LawfulClasses load; that was under-weighted here and CV rejected it
([[a-zero-trust-delta-control-must-assert-an-exact-after-minus-before-inventory-not-a-superset-and-by-identity-not-by-name]]).

## A census red from a new import edge: over-inclusion is fail-closed

LF-BOOL-AND's `import LawfulClasses` pulled LawfulFunctors into the SEAL-2
producer-confinement census's reaching set and reddened `assert_confined`; the
repair added LF to the enumeration (then `seal2_producer_closure.rs`, now
`crates/ken-elaborator/src/seal2_tests/producer_closure.rs`). Over-inclusion in
a confinement census is fail-closed; under-inclusion is the dangerous
direction, and `assert_confined` already names any non-enumerated reaching root.
So the hunt is three checks, not a proof the root belongs there: (a) reaching
is derived from data (`reaching_roots(facts, closure)`, not the enumeration);
(b) the env is non-vacuous (a `loaded_witnesses` precondition such as
`globals.contains_key("...LawfulFunctors.idf")`); (c) `closed_producers` is
empty for every carrier.

## Respin addendum: anchor the delta, and the oracle must not move

NODE B's `db9129100` passed the elaborator gate (1411/0) but broke `ken run`
on four rosetta examples (closures, merge-sort, palindrome, tree-traversal).
The respin `5d5266e9`:

1. **Anchor the delta.** `git diff <cleared> <respin> -- <core paths>` showed
   the elaborator surface byte-identical; the new surface was
   `crates/ken-cli/tests/rosetta.rs`. Re-run the exact-SHA sweeps anyway, but
   spend fresh effort on the delta.
2. **A claimed fix must live in the production path, with the oracle
   untouched.** Grep the range for any change under the oracle tree
   (`examples/rosetta/**`: empty), and confirm the broken cases are still
   must-match (each has `expected`, none a `KNOWN-GAP.md`) under a real
   end-to-end oracle (subprocess `ken run`, exact stdout, exit success,
   timeout fails, no silent skip). Here the fix was pure source assembly
   (inline the canonical provider, strip the now-redundant import for the flat
   runner) with a fail-loud "must carry its import" assertion. A green respin
   whose green came from a moved oracle is the invention in costume.

A flat-source `ken run` shim does not test the module-import execute path;
that path is covered by the elaborator gate on real source, and saying so is
an honest boundary, not a blind spot.

## Provenance refinement

Beyond per-path blob identity, `git diff --stat <reviewed> <landed>` coming
back **empty** proves the trees identical everywhere, including a path outside
the reported name-status. Run both. Tier-B-classes' `a7dfb0c8b` was the
third tree-cite, after `aeb7ceef2` and `55aac6e55`:
[[never-complete-an-abbreviated-sha-cite-rev-parse]].

## Residual to state, not file

Modules still on ambient passthrough resolve bare `sym`/`trans` across a
two-provider namespace; kernel-backstopped and being retired
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]). All verdicts
reason-verified at their SHA; no build (§12).

Siblings: "invention in costume hunt verify denotation matches the reference by
coinciding normal forms" (an earlier lesson, since retired) and
[[a-partial-landings-regression-risk-is-decided-at-the-consumer-not-in-the-diff]].
