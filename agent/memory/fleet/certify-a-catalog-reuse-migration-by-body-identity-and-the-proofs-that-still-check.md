---
name: certify-a-catalog-reuse-migration-by-body-identity-and-the-proofs-that-still-check
description: "A reuse-not-reimplement migration (a package drops its local copy of an op and imports the canonical provider) is certified by body identity up to alpha plus the unchanged proofs that still kernel-check against the import, which are the differential oracle for same denotation and same definitional reduction. Do not manufacture a correctness finding when the bodies coincide. Check the reuse pins are reaching (exact GlobalId, definitional anti-duplication, reaching negatives), grep a rename drain by its OLD name including prose, certify a forced attached-proof re-home as naming-only, and look for the hidden cost: the consumer's ambient surface grows with the provider's and any facade's un-migrated debt."
metadata:
  type: feedback
---

# Certify a catalog reuse migration by body identity and the proofs that still check

A "reuse, not reimplement" migration removes a package's local definition of an
operation and imports the canonical provider. Its whole soundness question is:
**does the imported op have the same denotation AND the same definitional
reduction as the removed local one?** The diff usually hands you the answer.

## The oracle is the unchanged proofs

1. **Diff the removed local bodies against the provider bodies.** Byte-identical
   up to alpha (bound-variable names only) means a pure factoring: same normal
   form, same reduction. When the bodies coincide there is nothing to attack on
   the correctness axis. **Do not manufacture a finding.**
2. **The retained proofs are the differential oracle and the kernel is the
   coincidence check.** Proofs that pin the op's reduction through
   `Refl`/`Proved`/`absurd`, or that pass it as a `cong`/`trans` argument, only
   type-check against the import if it reduces identically to the removed local.
   So CI-green re-elaboration of the consumer is a stronger fidelity
   certificate than the byte diff: a non-faithful provider would red those
   proofs.
3. **When the op lands in a `Prop` former** (e.g. `ValidByteRange = Equal Bool
   (leq_nat ..) True`), the oracle could degrade to name resolution. Find the
   conversion theorem that exercises it in both directions (a positive that must
   elaborate on the `True` case and a negative that must be `KernelRejected` on
   the `False` case), or the in-package `Proved`/inductive proofs that force the
   reduction, before concluding it went name-only. Confirm zero lingering
   references to the removed local name.
4. Confirm zero `trusted_base` delta, no `Axiom`, and no import cycle between
   consumer and provider.

## Reuse pins that are real, not green-vs-green

- **Exact-GlobalId occurrence**: `term_reference_count` or a saturated
  application-head occurrence keyed on the provider's GlobalId (not a name),
  with the provider excluded from the consumer's own declaration population. If
  the walk uses `Term::children()`, read
  [[a-syntactic-occurrence-census-over-proof-carrying-bodies-counts-erasable-motive-and-proof-term-positions-so-map-each-padded-member-to-its-real-protector-before-accepting-or-rejecting-a-padding-concern]].
- **Definitional anti-duplication**: `module_transparent_kernel_equivalents`
  (in `cat3_collections_package.rs`, `map_build_acceptance.rs`,
  `cat_map_bool_and_owner.rs`) looks up the provider as `Decl::Transparent`,
  iterates every zero-level transparent global of the consumer module, and flags
  any whose type and body are kernel-convertible to the provider
  (`convert_type` and `convert`). It catches a differently-named identical
  reimpl, which a name-keyed `!globals.contains_key(..)` cannot. **Limit:**
  separately declared recursive globals are distinct rigid heads with no
  definitional-equality check, so "a future differently-named isomorphic
  recursive helper" stays review- and census-enforced. A test that discloses
  that is honest, not a gap.
- **Reaching negatives**: import withdrawal and a wrong-name import, each via a
  `replace_exactly_once` mutation (exactly one occurrence changed), each failing
  at exactly `UnresolvedCon` of the provider name under the same fixture that
  keeps other deps green. A non-import control (`import .. (length)` plus a use
  of the unimported sibling `reverse` -> `UnresolvedCon { name: "reverse" }`)
  proves a selective import is exclusive. A fixture that withholds the flat
  alias keeps the import load-bearing.
- **A discriminating behavioral pair** through the consumer's own combinator
  (one `False`, one `True`, so a degenerate provider reds at least one arm), or
  a concrete eval vector through the import.
- **Identity equality** for a drain to an ambient provider with no import line:
  capture the installed identity from a bare `ElabEnv::new()` before loading
  the consumer, then assert `env.globals[name] == installed` and
  `!contains_key("<Consumer>.<name>")`.
- **Facade avoidance**: a negative `!globals.contains_key(<a facade member>)`,
  or the stronger `!globals.keys().any(|n| n.starts_with("<Facade>."))`, makes
  "the facade was not loaded" a checked fact. It must run in a fresh env; see
  [[a-shared-fixture-facade-preload-voids-every-consumers-facade-avoidance-pin-and-a-fresh-env-sibling-is-the-tell]].
- A trusted-base test re-scoped to consumer-local zero by preloading the
  provider into `before` is correct when the prose discloses that the consumer
  inherits the provider's audited trust footprint.

## A rename drain: grep the OLD name, and expect stale prose

When the retired local's name differs from the provider's (`option_is_some` ->
`is_some`):

1. The reuse-roster grep must use the **old** name; grepping the new name
   mis-scopes the roster and misses the retirement.
2. The absence pin correctly names the old qualified global
   (`!contains_key("Data.Collections.Map.option_is_some")`).
3. A same-name or rename drain of a **public** local characteristically leaves
   stale prose naming the retired local (`Map.ken.md:48`; Derived's header and
   §8.2 Public API after the prelude drain). Outside a ```ken fence it breaks
   nothing: low-severity as-built drift for the Librarian. Grep the old name
   across the whole literate file, prose included. A drain of a **private**
   helper that never appeared in prose or the Public API leaves no drift.

## A forced attached-proof re-home is naming-only

When the reused op carried an attached proof `proof NAME for FUNC` (canonical
`FUNC::NAME`, referenced as the atom `(proof NAME for FUNC)`), the migration
may convert it to a standalone `theorem FUNC_NAME`. The hypothesis to attack is
that the attached form carries semantics a plain theorem drops (a rewrite
table, instance-like resolution, ambient proof search). Spec
`30-surface/32-grammar.md` refutes it: "`prop`, `theorem`, and attached
`proof` all elaborate to existing checked terms only... None of these forms
adds a new kernel declaration class, a trusted proof table, or ambient proof
search." Certify the conversion structurally, without re-deriving the proof:

1. type unchanged (only the header keyword line differs);
2. body unchanged modulo the recursive self-reference;
3. every reference repointed, grepping **both** spellings (the atom and
   `FUNC::NAME`) across `catalog/` and `crates/`. A missed one is CI-red, but
   census it so you can say CI is green and catch one that accidentally
   resolves to a different symbol;
4. the new name collides with no existing symbol.

The 2026-08-29 instance treated the conversion as forced by 33 §8.2 (no
attaching to an imported subject). The current §8.2 text requires only that
the subject be an already-resolved definition, so check a forcing claim against
the spec at the SHA under review. The naming-only argument does not depend on
it.

## The hidden cost: the consumer's ambient migration surface

A reuse changes not only the consumer's TCB (usually zero delta) but its
ambient / strict-resolution surface, measured by
`catalog_ambient_passthrough_migration_census`. QA and CV certify that the
census is accurate. Whether the surface it measures is **unnecessarily wide**
is a separate question no positive gate owns. Ground it by grepping the
consumer's own source for each added name (0 occurrences means transitive),
then separate the two transitive sources:

- **A blanket-`export` facade** drags in what its export forces. Importing
  `leq_nat` through `Data.Numeric.Nat.Order` (whose
  `export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)` forces
  the Ord dictionary, an And/Pair record) added the Pair-record machinery
  `{Pair, mk_pair, pair_fst, pair_snd}`. Importing at the canonical owner
  avoids it, and a negative facade pin proves it was avoided.
- **The provider's own un-migrated ambient fallbacks** are inherited through
  the import edge even at the canonical owner. `LawfulClasses` uses
  `{And, Proved, and_fst, and_intro, and_snd}` internally without importing
  them, so a direct-home consumer still gains those plus `Prop`/`Bottom`.
  Sequencing corollary: migrate the provider's own ambient fallbacks to
  explicit imports first, else every consumer inherits the debt regardless of
  use.
- **The debt is visible only in the ambient bucket.** A baseline-red residual
  consumer has no measured vector, so the same inherited debt is masked there.
  A direct-home import plus a negative facade pin tightens the Pair-record cost
  but is not proof of a clean surface.

This is LOW, a tracked migration cost, not a defect, when the migration sentinel
fired and the expectation was updated with no `clean -> ambient` move. What the
census owes on each migration is in
[[classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row]].

## Instances

- **CAT-GCD-REFACTOR**, landed squash `877dca4d3` (byte-identical to reviewed
  `5864ab352` over the three WP paths; dispatch base `271ea95f6` stale, real
  squash parent `e4e512ed8`), reported `evt_17vavj42x7ebp`, thread
  `thr_1fhrjbg7f1j1m`; Foundation QA `evt_331w2symncp6y`, CV
  `evt_7z0mfayjhc0r8`. Gcd's local `add`/`mul`/`leq_nat`/`sub` replaced by
  imports from `Data.Numeric.Nat.Arithmetic` and `Data.Numeric.Nat.Order`; all
  four bodies alpha-identical; proofs `add_sub_cancel_leq`, `leq_refl`,
  `leq_weaken_right`, `leq_not_flip`, `fuel_bound_sub_left`/`_right` over 39
  content-unchanged decls; `GcdSpec` machine-proven with `gcd_spec` supplying
  the fuel bound. Census row `{Equal, Proved}` grew to 12 names, 9 of them
  never referenced by Gcd (`Equal` 36x and `Proved` 10x were). Pins in
  `cat_gcd_acceptance.rs` (:111, :122, :130-135, :179) plus a 7-vector oracle.
- **CAT-NAT-REUSE-CONSUMERS D1**, landed `6ba6f6bef` (reviewed `621540c9`),
  `evt_7zjr4g045tt22`, thread `thr_6kvgg1a8dmrf`. `Capability/Process/Arguments`
  imports `leq_nat` direct-home; `cc6a_process_arguments_exit_acceptance.rs`
  `arguments_reuses_the_canonical_lawful_classes_relation` pinned
  `!contains_key("Data.Numeric.Nat.Order.sub")` (later dropped, see the
  shared-fixture lesson); value oracle
  `structural_slice_location_keeps_nonzero_argument_and_range`. Arguments was a
  residual, so its inherited debt was masked, and D1's inference that
  direct-home escapes all law-record machinery was too strong.
- **CAT-NAT-REUSE-CONSUMERS D2**, landed `428ea1188` (reviewed `096617e94`;
  real parent `66d46ba6e`, dispatch base `56c51fe31` stale).
  `Capability/Diagnostics/Core` row `{Bottom, Equal, Prop, Top}` grew by exactly
  `{And, Proved, and_fst, and_intro, and_snd}`, none referenced in its source;
  cc4 pins the whole Order namespace absent; cc4's
  `cc4_valid_range : ValidByteRange (MkByteRange 2 5) = Proved` must elaborate
  and `MkByteRange (Suc Zero) Zero` must be `KernelRejected`.
- **CAT-NAT-REUSE-CONSUMERS D3**, landed `9de02daff` (reviewed `9725b1d9`; real
  parent `97a34542`, dispatch base `61c2fefa0` stale), `evt_vrfy380gqe9a`,
  thread `thr_47v3yfaecfq9c`. `Capability/Parsing/Parsing` drops
  `nat_leq_bool` for `leq_nat` (LawfulClasses:479, byte-identical, scrutinizes
  `m` first), its first import edge; `proof zero_left for LessEqNat` and
  `proof refl` are the reduction oracle, kernel-checked by `cat5_d1`. The green
  residual sentinel proved no residual -> ambient move.
- **CAT-DERIVED-REUSE-CONSUMERS D3** `Core.Classes.EffectfulClasses`, exact
  `b935dec7469e9f3ba39fc5e74839aec55e0dce1a` (range `548c47de5..b935dec74`,
  six paths, +213/-51 mostly rustfmt reflow), `evt_3d9yqp61acqx4`, thread
  `thr_2vwarkhtpt9k1`. `proof pointwise_eq for concat_map` became
  `theorem concat_map_pointwise_eq` (declaration plus three uses, zero
  old-form references). `ds7_applicative_monad_acceptance.rs`
  `derived_concat_map_import_identity_and_concrete_vector_are_pinned` evaluates
  `list_bind [T,F] (lambda x. [x,x]) = [T,T,F,F]`. Derived imports only
  `Data.Numeric.Nat.Order` and `Core.Logic.*`, so no cycle. Verdict rested on
  the exact diff and spec, not a local run.
- The pins above also appear on the census instances listed in the
  classify-census lesson: `map_build_acceptance.rs` for the Map rename drain
  (`4c909d402`), `cat3_collections_package.rs` for the Derived Bool drain
  (`11ce6f3aa`, with `eq_from_ord Bool bool_leq` pair and the `is_sorted` /
  `sort_bool_*` lemmas as fidelity oracle), and the Property drain
  (`2e7796d80`, `property_list_length` had zero Rust refs so no central roster
  drain was owed).
