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
named explicit-import, conversion, and positivity cases are **RED-UNTIL
`LANG-MOD-CANONICAL-PAIR-PACKAGE`**; the future graph-closure case additionally
requires separately authorized consumer imports. The representation-only
transparent-Sigma controls execute now through the kernel bindings named in
AC5; they do not provide the named Pair interface. RED-UNTIL is not an accepted
failure and not a current census disposition. AC6 keeps the pre-provider
evidence frontier separate from the later import graph that must be re-derived
after authorized consumer imports exist.

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

- promise class: **normative compatibility vector** — downstream proofs use
  conversion, not a package theorem
- spec: `34 §"Canonical non-dependent pair package"`; `13 §2`/`§6`
- given: after explicit import of all four provider names, check these exact
  surface equality terminals:
  `pair_fst Bool Bool (mk_pair Bool Bool True False) = True` with `Proved`,
  `pair_snd Bool Bool (mk_pair Bool Bool True False) = False` with `Proved`,
  and, for neutral `p : Pair Bool Bool`,
  `mk_pair Bool Bool (pair_fst Bool Bool p) (pair_snd Bool Bool p) = p`
  with `Refl`.
- expect: the concrete fst and snd goals normalize to `Top` and accept
  `Proved`; `Refl` rejects because those goals are no longer `Eq`-shaped. The
  neutral reconstruction remains `Eq`-shaped and accepts `Refl`; `Proved`
  rejects. No imported law, rewrite, eliminator, postulate, or runtime
  evaluation participates.
- controls: in independent arms, change only the first expected endpoint to
  `False`, the second to `True`, and the neutral reconstruction to the
  component-swapped pair; each prescribed terminal rejects. Independently swap
  `Proved` and `Refl` at the three correct equations; each wrong terminal
  rejects at the normalized goal shape.
- why: concrete beta arms distinguish the projections and pin the `Top`
  terminal, while neutral eta pins the distinct `Eq`-shaped terminal. The word
  “reflexivity” alone would not distinguish those surface proof forms.

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

The named Pair instantiation is RED-UNTIL the canonical package. Its
representation rule already has executing controls in
`../../kernel/inductive/seed-nested.md`; those controls use ordinary
`declare_def` checked-transparent Sigma aliases and do not count as a Pair
provider.

### kernel/inductive/canonical-pair-positive-path-unfolds-to-sigma

- promise class: **durable invariant** — transparent representation, not a
  spelling allow-list, determines the verdict across intended extensions
- shape: **soundness/completeness pair** — each positive orientation is paired
  with the corresponding inner-arrow negative
- spec: `14 §8.3`/`§8.5`; `34 §"Canonical non-dependent pair package"`
- given: admit or import the checked transparent Pair definition, then submit
  the first-component pair
  `data Good1 = MkGood1 (Pair Good1 Unit)` and
  `data Bad1 = MkBad1 (Pair (Bad1 -> Empty) Unit)`, plus the second-component
  pair `data Good2 = MkGood2 (Pair Unit Good2)` and
  `data Bad2 = MkBad2 (Pair Unit (Bad2 -> Empty))`, through the ordinary
  inductive admission path. The kernel fixture supplies already-resolved
  `Unit`/`Empty`; this case does not classify their surface availability.
- expect: `Good1` and `Good2` accept by reducing the transparent head to Sigma
  and following the corresponding positive component. `Bad1` and `Bad2` reject
  when the same reduction reaches the recursive occurrence in an arrow domain
  at negative polarity. Neither Sigma component may be discarded.
- controls: rerun `nested-ds9-shapes-admitted` and
  `nested-negative-transparent-sigma-control`. Their exact executing bindings
  are
  `checked_transparent_sigma_aliases_admit_renamed_nested_paths` and
  `checked_transparent_sigma_alias_rejects_inner_arrow_negative` in
  `crates/ken-kernel/tests/nested_inductives_remaining.rs`. They admit two
  distinct checked-transparent definitions with the same Sigma body, require
  both renamed positive paths to accept, and require independently controlled
  direct-recursive acceptance plus inner-arrow rejection in each Sigma
  component. Independently suppressing transparent-head unfolding,
  first-component descent, or second-component descent must redden its
  corresponding executing control. The opaque and unknown-head controls in that
  suite remain
  fail-closed.
- why: the current controls establish representation- and name-independence at
  the same kernel seam without pretending to supply the future public Pair
  identity. The RED-UNTIL arm must instantiate both component orientations
  through the canonical imported declaration rather than a spelling allow-list
  or a second mechanism.

## AC6 — the evidence frontier precedes provider/import closure

### surface/modules/pre-provider-census-stops-at-legacy-evidence-frontier

- promise class: **durable invariant** — evidence availability bounds every
  identity, migration, and closure inference
- stage: **current pre-provider evidence frontier**
- spec: `30 §5`; `33 §3.3`; `34 §"Canonical non-dependent pair package"`; `39
  §2.0`
- given: enumerate the census subjects `Core.Logic.Compare`,
  `Core.Classes.LawfulClasses`, `Data.Collections.Derived`,
  `Data.Numeric.Nat.Order`, and `Algorithm.Numeric.Gcd`. Treat that list only as
  an evidence-collection input, not as a Pair closure. Run each row through
  Legacy. For a successful Legacy row, record only the exact `GlobalId`s
  actually resolved in that run. For every row that does not complete Legacy
  with exact-identity evidence, record its typed first failing stage as
  `Unavailable(stage)`. Source spelling, an error spelling, semantic
  familiarity, or a numeric origin supplies no missing identity.
- expect: a successful Legacy row may carry its exact-identity ledger as
  evidence about that Legacy run. The ledger creates no import edge, chooses no
  future provider, and confers no migration or Strict status. Every other row
  carries only `Unavailable(stage)`: no named Pair prerequisite, current
  migration or provider-closure label, graph membership, or green/closure
  numerator. `Unavailable` records evidence staging, not resolver state,
  migration state, closure, or deferral.
- controls: for a successful row, change one recorded id or delete the ledger
  and require comparison with the resolver output to fail. For an unavailable
  row, change the first failing stage and require the typed-stage assertion to
  fail. Add a named prerequisite, provider identity, migration label, graph
  edge, or numerator contribution to an `Unavailable` row and require the
  census-artifact check to reject it. A source-name or diagnostic-text
  mutation must not create identity evidence.
- why: an enumerated source list and a failed run can select work to investigate
  but cannot manufacture the parsed edge or exact identity needed to classify
  provider closure. Stopping at the evidence frontier prevents planning
  familiarity from becoming resolver authority.

### surface/modules/pair-closure-is-derived-only-after-authorized-imports

- promise class: **durable invariant** — provider/import closure is derived
  from real edges and exact identities, never projected from census staging
- stage: **RED-UNTIL `LANG-MOD-CANONICAL-PAIR-PACKAGE` and separately
  authorized consumer imports**
- spec: `33 §3.3`/`§4.3`; `34 §"Canonical non-dependent pair package"`; `39
  §2.0`
- given: after the canonical package lands and a separately authorized
  migration adds ordinary explicit consumer imports, record the four provider
  ids before consumers load. Rebuild the parsed import/re-export graph from
  those actual sources, derive the candidate population only from real paths to
  the provider, and strict-load every graph member. Inspect every emitted
  Pair-family/helper reference.
- expect: every member of the newly derived graph population strict-loads and
  resolves each reference to the recorded provider id. No prior Legacy ledger
  or `Unavailable(stage)` row enters that population without a real import
  path and a fresh successful Strict run. No implementation-native or competing
  id remains reachable; imports allocate no replacement and add no trust. Rows
  outside the real graph retain only their independently remeasured evidence.
- controls: remove one real Pair import and require both graph membership and
  the Strict arm to change; redirect one import to a same-shaped competing
  provider and require the identity assertion to fail. Add a census row with no
  parsed provider path to the closure population and require exact population
  equality to fail. Retain `Unavailable(stage)` for a row whose authorized
  import and Strict arm both pass and require current-evidence equality to fail.
- why: a provider that nobody imports, a fallback to an old global, projection
  from Legacy evidence, and automatic promotion of an unavailable row are four
  different false completions. The future population must be re-derived rather
  than inferred from today's census.

## AC7 — prior obligations survive future migration and staging

### surface/modules/future-pair-migration-does-not-transfer-ord-nat-ownership

- promise class: **durable invariant** — Pair availability does not change
  instance ownership or declaration provenance
- evidence: **by reference** to the owning module/orphan seed named below
- spec: `33 §5.3`/`§5.5.1`; `51 §7`
- given: after a separately authorized Pair migration makes `LawfulClasses`
  available under Strict, rerun `seed-modules.md`'s
  `ord-nat-class-owner-and-reexport-use-one-dictionary`.
- expect: the sole `(Ord, Nat)` dictionary is still defined by
  `Core.Classes.LawfulClasses`; `Data.Numeric.Nat.Order` imports/re-exports that
  same identity and never redeclares or owns it.
- why: future migration and staging may change availability. They do not change
  the orphan rule or declaration provenance.

### surface/modules/future-pair-migration-does-not-discharge-attached-proofs

- promise class: **durable invariant** — Pair availability does not change
  attached-proof ownership or mint a foreign proof identity
- evidence: **by reference** to the ownership controls named below
- spec: `33 §8.2`; `39 §2.0`; `LANG-MOD-ATTACHED-PROOF-OWNERSHIP`
- given: after separately authorized migrations make `LawfulClasses` and Order
  available under Strict, rerun the provider-local accept, consumer-foreign
  reject, and local ordinary-theorem controls from the attached-proof ownership
  seed.
- expect: `pair_compare_eq_sound`, `pair_compare_lt_asym`, and
  `bool_or_eq_true_of_or` are ordinary private theorems at their settled owning
  units; no foreign attached-proof identity is minted. All remain checked and
  outside `trusted_base()`.
- why: future migration and staging neither reverse the attachment ownership
  rule nor count the conversions as already delivered.

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
- **THE GAP:** no canonical Pair provider or authorized consumer-import
  migration exists yet. The positive package and future graph-closure cases
  remain RED-UNTIL; current native/Legacy acceptance is not evidence for them.
  No successful Legacy exact-identity evidence has authorized a current Pair
  closure population. A row without that evidence is only
  `Unavailable(stage)` and gains no named prerequisite or closure inference
  from this artifact.

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
  future RED-UNTIL vectors it must make executable.
