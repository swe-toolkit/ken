# Strict boundary for the named Pair package

Format: `../../README.md`. These cases pin the boundary between the closed
prelude floor, implementation-global checked conveniences, and ordinary public
package interfaces (`spec/30-surface/30 §4`/`§5`, `33 §3.3`, `34
§"Canonical non-dependent pair package"`, `39 §2.0`).

The named surface declarations are `Pair`, `mk_pair`, `pair_fst`, and
`pair_snd`. They are distinct from the kernel term constructor `Term::Pair`,
the interpreter representation `EvalVal::Pair`, and a source declaration that
happens to use `Pair` as its local name. Those three uses do not create a
package provider.

The rejection and exact-floor cases describe the current Strict boundary. The
positive explicit-import, conversion, positivity, and cluster-closure cases are
**RED-UNTIL `LANG-MOD-CANONICAL-PAIR-PACKAGE`**. RED-UNTIL is not an accepted
failure and not a green numerator: it identifies the prerequisite that must
flip the same seam.

## AC1 — compiler possession is not Strict provider availability

### surface/modules/strict-bare-pair-name-matrix-rejects

- promise class: **durable invariant** — Strict scope comes from the closed
  floor and explicit interfaces, never the whole implementation global map
- spec: `30 §4`/`§5`; `33 §3.3`; `34 §"Canonical non-dependent pair package"`;
  `39 §2.0` steps 4–5
- given: create a fresh environment and record the checked transparent
  `GlobalId`, declaration kind, and type of each implementation-global name
  `Pair`, `mk_pair`, `pair_fst`, and `pair_snd`. Snapshot declaration count,
  allocator position, and `trusted_base()`. In four independent strict-roots
  runs, use exactly one of those bare names as the body of an otherwise
  well-formed inferred `const`. Install no import or test-only export map.
- expect: every row reports that row's name unresolved at surface resolution.
  No row resolves to the recorded implementation id, reaches kernel type
  checking, or allocates a source declaration. All four recorded declarations
  remain checked transparent and outside `trusted_base()`; the snapshots are
  unchanged.
- controls: in a fifth root, define and export an unrelated transparent
  `provided` constant from an ordinary source provider and explicitly import it
  into the entry. That root accepts and resolves the provider id. Loading the
  provider first without importing it is a separate reject arm.
- why: one combined source would observe only the first error. The per-name
  matrix proves the rule covers the family and all three helpers. The imported
  control separates a closed scope from blanket rejection of non-floor checked
  definitions.

### surface/modules/pair-remains-unbound-after-unrelated-provider-load

- promise class: **durable invariant** — loading is not importing
- spec: `33 §3.3`; `39 §2.0` steps 2, 4, and 5
- given: successfully strict-load an unrelated provider, then strict-load an
  entry containing bare `Pair` with no import edge. Repeat after loading a
  re-export facade unrelated to Pair.
- expect: both entry runs reject `Pair` as unresolved. The loaded-unit cache and
  flattened environment do not enlarge either entry's source scope.
- why: a fresh-environment-only rejection could coexist with a cache-dependent
  ambient leak. This case reaches the cache-reuse path and keeps scope
  construction per-unit.

## AC2 — spelling, shape, and conversion do not confer identity

### surface/modules/definitionally-equal-pair-is-a-distinct-identity

- promise class: **durable invariant** — canonical identity is not structural
  equality
- spec: `30 §5`; `33 §3.3`/`§4.3`; `34 §"Canonical non-dependent pair
  package"`; `39 §2.0`
- given: record the implementation-global checked transparent `Pair` id, type,
  and body plus a checked declaration whose type or body contains that exact
  id. Through the ordinary checked-definition producer, admit a second
  transparent declaration with the identical type and non-dependent Sigma body
  under a fresh name. Snapshot the referenced first declaration before and
  after.
- expect: the second checked declaration receives a fresh id distinct from the
  implementation id. The pre-existing declaration remains byte-for-byte keyed
  to the first id; it is not rewritten to the second. Applications may be
  definitionally equal after transparent unfolding, but declaration identity
  and provenance remain distinct.
- controls: reverse which id the identity-keyed reference is required to contain
  and require the assertion to fail. Independently normalize both bodies before
  the id comparison: conversion succeeds while explicit id inequality still
  holds.
- why: a type-conversion success cannot prove provider identity. The property
  is which declaration a reference names and whether an existing identity-keyed
  reference changed.

### surface/modules/local-inductive-pair-cannot-substitute-for-transparent-pair

- promise class: **normative compatibility vector** — a new family does not
  become an existing transparent definition
- spec: `30 §5`; `33 §3.3`; `34 §"Canonical non-dependent pair package"`
- given: record the checked transparent implementation `Pair`, then strict-load
  an ordinary local `data Pair a b = MkPair a b` without importing a Pair
  interface.
- expect: the local family and constructor receive fresh declaration
  identities. The resolved family is neither the recorded transparent id nor
  definitionally the non-dependent Sigma alias, and it cannot satisfy an
  assertion requiring the recorded id. No identity is added to
  `trusted_base()`.
- why: this reproduces the catalog-redeclaration fork directly and prevents a
  same-spelling source family from being reported as an identity-preserving
  provider.

## AC3 — exact-nine stability

### surface/modules/pair-is-not-a-floor-member

- promise class: **durable invariant** — the exact floor is closed in both
  directions
- spec: `30 §4`; `33 §3.3`; `39 §2.0`
- given: run the executable primitive-signature traversal and independent
  bootstrap-identity inventory from `../taxonomy/minimality.md`, case
  `surface/taxonomy/prelude-signature-inventory-is-executable-and-closed`.
  Record the pre-installed checked Pair id separately.
- expect: the signature set is exactly `{Auth, Bool, Char, List, Option,
  ResourceKind, Result, Utf8Error}`; adding bootstrap `Nat` yields exactly
  `{Auth, Bool, Char, List, Nat, Option, ResourceKind, Result, Utf8Error}`.
  Pair occurs in neither inventory and is unavailable under Strict. Floor
  installation allocates nothing and changes no trust entry.
- controls: add only the recorded Pair id to the configured floor while leaving
  the producer and bootstrap inventories unchanged. Exact equality must fail
  with Pair as the sole extra member. Restore, then rerun the pristine arm
  green.
- why: a generic non-member such as `Prod` proves closure machinery, while this
  Pair-specific mutation proves the disputed convenience is actually excluded.
  Production must not auto-widen from the observed compiler population.

## AC4 — the future package contract computes through Sigma

The cases in this section are RED-UNTIL the canonical Pair package. The
concrete module path is a realization input; the behavioral assertions do not
choose it.

### surface/modules/canonical-pair-interface-is-checked-transparent

- promise class: **durable invariant** — the package reflects existing kernel
  structure and adds no trusted primitive
- spec: `34 §"Canonical non-dependent pair package"`; `33 §4.3`; `39 §2.0`
- given: load the selected canonical provider and record its public declarations
  `Pair`, `mk_pair`, `pair_fst`, and `pair_snd`, plus declaration count,
  allocator position, and `trusted_base()`. Inspect the checked core types and
  transparent bodies after ordinary elaboration.
- expect: all four are checked transparent declarations. `Pair A B` unfolds to
  non-dependent `Sigma A (_ : A. B)`; `mk_pair` unfolds to Sigma introduction;
  the projections unfold to the kernel first and second projections. No
  declaration is opaque or primitive and no id enters `trusted_base()`.
- controls: independently change the expected declaration kind of each helper
  to opaque and require the assertion to fail; compare each body against a
  freshly constructed wrong kernel former so a name-only check also fails.
- why: zero trust alone would permit a wrong checked body. Kind, type, and body
  together pin the transparent derivation.

### surface/modules/canonical-pair-beta-eta-are-definitional

- promise class: **normative compatibility vector** — downstream proofs may use
  reflexivity, not a package theorem
- spec: `34 §"Canonical non-dependent pair package"`; `13 §2`/`§6`
- given: after explicit import of all four provider names, check reflexivity at
  these three neutral or concrete equations:
  `pair_fst Bool Bool (mk_pair Bool Bool True False) = True`,
  `pair_snd Bool Bool (mk_pair Bool Bool True False) = False`, and, for neutral
  `p : Pair Bool Bool`,
  `mk_pair Bool Bool (pair_fst Bool Bool p) (pair_snd Bool Bool p) = p`.
- expect: all three close by conversion and reflexivity alone. No imported law,
  rewrite, eliminator, postulate, or runtime evaluation participates.
- controls: change only the first expected endpoint to `False`, the second to
  `True`, and the neutral reconstruction to the component-swapped pair in
  independent arms. Each reflexivity proof rejects.
- why: concrete beta arms distinguish the projections, while the neutral eta
  arm cannot pass by reducing a freshly constructed pair on both sides.

### surface/modules/pair-import-and-reexport-preserve-one-identity

- promise class: **durable invariant** — one public provider, no import
  allocation or compatibility alias
- spec: `33 §4.3`; `34 §"Canonical non-dependent pair package"`; `39 §2.0`
- given: load the provider, one direct consumer, and one facade/re-export
  consumer. Snapshot all four provider ids and the environment accounting after
  provider admission. Inspect every resolved reference in both consumers.
- expect: direct import and re-export select the same four ids. Imports allocate
  no replacement; only each unit's own source declarations advance the
  allocator. Exactly one public defining identity exists per name, no second
  ambient or compatibility route resolves, and `trusted_base()` is unchanged.
  A third unit with no import still rejects every bare name.
- controls: route the facade to a same-shaped competing provider in a mutation;
  the identity assertion must fail even though conversion may succeed. Remove
  the unimported negative arm and the control is incomplete.
- why: successful source checking alone cannot distinguish identity-preserving
  import from re-elaboration, alias fallback, or a definitionally equal second
  provider.

## AC5 — nested positivity is representation-derived

These cases are RED-UNTIL the canonical Pair package and compose with
`../../kernel/inductive/seed-nested.md`.

### kernel/inductive/canonical-pair-positive-path-unfolds-to-sigma

- promise class: **soundness/completeness pair** — transparent representation,
  not a spelling allow-list, determines the verdict
- spec: `14 §8.3`/`§8.5`; `34 §"Canonical non-dependent pair package"`
- given: admit or import the checked transparent Pair definition, then submit
  `data Good = MkGood (Pair Good Unit)` and
  `data Bad = MkBad (Pair (Bad -> Empty) Unit)` through the ordinary inductive
  admission path. The kernel fixture supplies already-resolved `Unit`/`Empty`;
  this case does not classify their surface availability. Repeat both
  declarations with a freshly named transparent
  `Product A B = (x : A) × B` substituted for Pair.
- expect: both `Good` variants accept by reducing the transparent head to Sigma
  and following a positive component. Both `Bad` variants reject when the same
  reduction reaches `Bad` in the arrow domain at negative polarity. Renaming
  the transparent head leaves both verdicts unchanged.
- controls: replace `Product` with an opaque or unknown type former of the same
  kind; a recursive payload fails closed as unknown. Replace only
  `(Bad -> Empty)` with `Bad`; that positive control accepts.
- why: a Pair spelling allow-list fails the renamed positives; blanket traversal
  fails the opaque control; treating an outer positive component as recursively
  all-positive fails the negative pair.

## AC6 — deferred cluster and later closure use one flipping seam

### surface/modules/pair-cluster-is-deferred-not-green

- promise class: **transitional compatibility vector** — a prerequisite is not
  an accepted failure
- spec: `30 §5`; `33 §3.3`; `34 §"Canonical non-dependent pair package"`; `39
  §2.0`
- given: before the canonical package lands, run the real strict roots loader on
  the authoritative Pair-dependent whole-unit seeds
  `Core.Logic.Compare`, `Core.Classes.LawfulClasses`, and
  `Data.Collections.Derived`, and on the ruled mandatory consumers
  `Data.Numeric.Nat.Order` and `Algorithm.Numeric.Gcd`. Use the loader-derived
  catalog population and parsed import graph for any further transitive
  consumers; do not derive membership from the first error or a spelling grep.
- expect: every authoritative Pair-dependent whole unit is recorded
  `DeferredOnCanonicalPairPackage`, never `StrictGreen` and never counted in a
  green numerator. A non-authoritative strict failure remains a blocker rather
  than entering this set. The current loader refusal is an observation, not the
  membership authority.
- controls: deleting a population row fails exact population equality; marking
  a direct seed green must reach the real strict rejection; adding an import
  edge from a formerly green unit to a seed changes reverse reachability and
  invalidates a stale ledger; a catch-all defer mutation fails on an unrelated
  strict error.
- why: emptying a selected failure list can falsely claim closure. Whole-unit,
  authority-derived disposition makes the temporary boundary explicit. This
  Pair-specific case does not assert that the two dispositions cover the whole
  current catalog: any unit outside the authoritative closure that fails Strict
  remains a separately routed blocker.

### surface/modules/pair-cluster-flips-only-after-explicit-import-closure

- promise class: **RED-UNTIL forward oracle** — the prerequisite removes the
  deferral rather than normalizing it
- spec: `33 §3.3`/`§4.3`; `34 §"Canonical non-dependent pair package"`; `39
  §2.0`
- given: after the canonical package lands, rerun the authoritative population
  and strict roots from the preceding case. Record the four provider ids before
  consumers load, and inspect all Pair-family/helper references emitted by each
  Pair-dependent unit and transitive consumer.
- expect: each affected unit has an explicit dependency path to the provider,
  strict-loads successfully, and resolves every reference to the recorded
  provider id. No implementation-native or competing id remains reachable;
  imports allocate no replacement and add no trust. Every temporary Pair
  deferral is removed and the full population is re-derived from scratch.
- controls: remove one affected unit's Pair import and require its strict arm to
  reject; redirect one import to a same-shaped competing provider and require
  the identity assertion to fail; retain one stale deferred row after all
  loader arms pass and require disposition equality to fail.
- why: a new provider that nobody imports, a fallback to an old global, and a
  permanently waived red row are three different false completions.

## AC7 — prior downstream obligations survive Pair deferral

### surface/modules/pair-deferral-does-not-transfer-ord-nat-ownership

- promise class: **durable invariant by reference**
- spec: `33 §5.3`/`§5.5.1`; `51 §7`
- given: rerun `seed-modules.md`'s
  `ord-nat-class-owner-and-reexport-use-one-dictionary` after the Pair-dependent
  `LawfulClasses` unit re-enters strict closure.
- expect: the sole `(Ord, Nat)` dictionary is still defined by
  `Core.Classes.LawfulClasses`; `Data.Numeric.Nat.Order` imports/re-exports that
  same identity and never redeclares or owns it.
- why: delaying a whole unit changes availability, not the orphan rule or
  declaration provenance.

### surface/modules/pair-deferral-does-not-discharge-attached-proof-conversions

- promise class: **durable invariant by reference**
- spec: `33 §8.2`; `39 §2.0`; `LANG-MOD-ATTACHED-PROOF-OWNERSHIP`
- given: when `LawfulClasses` and Order re-enter, rerun the provider-local accept,
  consumer-foreign reject, and local ordinary-theorem controls from the attached
  proof ownership seed.
- expect: `pair_compare_eq_sound`, `pair_compare_lt_asym`, and
  `bool_or_eq_true_of_or` are ordinary private theorems at their settled owning
  units; no foreign attached-proof identity is minted. All remain checked and
  outside `trusted_base()`.
- why: Pair deferral postpones the containing units. It neither reverses the
  attachment ownership rule nor counts the conversions as already delivered.

## Evidence posture

- **MEASURED at `dcfa19210bae01edaf55914a0aeb63e9362042a4`:** the configured
  floor has exactly nine names; all four implementation Pair declarations are
  checked transparent and outside `trusted_base()`; a per-name real strict-
  roots probe rejects each bare name before allocation; and a strict source
  `data Pair a b = MkPair a b` receives a fresh id distinct from the
  implementation Pair. The executable primitive-signature and nonmember-floor
  controls remain green.
- **CLAIMED:** the one-interface transparent-Σ contract, definitional β/η,
  explicit-import identity preservation, and representation-derived positivity
  are the behavior the later canonical package must realize.
- **THE GAP:** no canonical Pair provider exists yet. The positive package and
  cluster-re-entry cases remain RED-UNTIL; current native/Legacy acceptance is
  not evidence for them. The current full-catalog probe also reports strict
  blockers outside the authoritative Pair closure. This Pair artifact neither
  reclassifies them nor claims a complete two-arm catalog disposition.

## Consistency and realization boundary

- The exact floor stays `{Auth, Bool, Char, List, Nat, Option, ResourceKind,
  Result, Utf8Error}` in every current and future arm.
- Current strict rejection, future explicit-import success, and the local
  lookalike positive all use the same resolver. No provider registry, floor
  change, convenience-global fallback, or compatibility alias is a permitted
  explanation.
- The conversion cases speak about transparent Σ behavior; the identity cases
  speak about canonical declaration ids. Definitional equality never
  substitutes for identity evidence.
- Core `Term::Pair`, runtime `EvalVal::Pair`, and local declarations named Pair
  remain outside the provider question.
- The canonical package realization chooses its module path and transition
  mechanism separately. These cases fix the behavior it must expose and the
  temporary vectors it must flip.
