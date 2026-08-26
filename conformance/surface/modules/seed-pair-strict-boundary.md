# Compiler-origin Pair floor-provider boundary

Format: `../../README.md`. These cases pin the general internal-provision arm,
the ten-type floor, and its first compiler-origin member (`spec/30-surface/30
§4`, `33 §3.3`, `34 §"Canonical non-dependent pair floor family"`, `39 §2.0`).
They supersede this seed's former exact-nine, bare-name rejection, and future
package-provider contract.

The named surface family is `Pair`, `mk_pair`, `pair_fst`, and `pair_snd`.
`Pair` is one type-floor member; the other three are its closed companion-binding
inventory, not additional type members. The four checked transparent declarations
are distinct from kernel `Term::Sigma`/`Term::Pair`/`Term::Proj1`/
`Term::Proj2`, runtime `EvalVal::Pair`, and any same-shaped source declaration.
Kernel and runtime forms carry no provider `GlobalId`.

The acceptance rows below are **RED-UNTIL
`LANG-MOD-CANONICAL-PAIR-PACKAGE`**, the redirected floor-realization build WP.
That build must reuse the four existing compiler-installed identities. RED-UNTIL
is not an accepted failure and does not authorize a package, alias, registry, or
ambient fallback. The representation-only transparent-Sigma controls execute
now and remain evidence for the unchanged kernel behavior.

## AC1 — the four existing Pair-family identities become Strict floor bindings

### surface/modules/strict-bare-pair-floor-name-matrix-accepts-exact-ids

- promise class: **durable invariant** — a witnessed closed floor reuses its
  pre-source declarations rather than searching or reallocating
- stage: **RED-UNTIL `LANG-MOD-CANONICAL-PAIR-PACKAGE` floor realization**
- spec: `30 §4`; `33 §3.3`; `34 §"Canonical non-dependent pair floor family"`;
  `39 §2.0` steps 4–6
- given: create a fresh environment before source elaboration and record the
  checked transparent `GlobalId`, type, body, declaration count, allocator
  position, and `trusted_base()` for each of `Pair`, `mk_pair`, `pair_fst`, and
  `pair_snd`. In four independent strict-roots runs, use exactly one bare name
  in an otherwise well-formed declaration. Install no import or test-only
  provider.
- expect: every row accepts and the resolved reference is the corresponding
  recorded pre-source id. `Pair` is the type-floor member. Each companion is a
  floor binding whose checked type references that exact `Pair` id. No run
  declares or imports a replacement, advances the allocator because of floor
  installation, or changes `trusted_base()`.
- controls: an otherwise identical bare `Prod` row rejects as unbound even
  though the compiler global map contains it. A separately registered unrelated
  transparent global also remains unbound without an import. Remove one
  companion from the configured closed inventory while leaving its compiler
  global present; only that companion row must reject rather than fall back to
  the ambient global.
- why: successful checking alone cannot distinguish reuse from re-declaration or
  ambient lookup. The exact-id and accounting assertions pin the floor path.
  `Prod` separates the general internal-provision arm from arbitrary compiler
  convenience exposure.

### surface/modules/pair-floor-remains-available-after-unrelated-loads

- promise class: **durable invariant** — per-unit loading neither supplies nor
  masks an always-present floor binding
- stage: **RED-UNTIL floor realization**
- spec: `33 §3.3`; `39 §2.0` steps 2, 4, and 5
- given: strict-load an unrelated provider and an unrelated re-export facade,
  then load entries that use each bare Pair-family name without imports.
- expect: every Pair-family reference still resolves to the pre-source floor id.
  The loaded-unit cache contributes no provider edge or replacement identity.
- controls: the same roots contain a non-floor `provided` declaration. Its bare
  use rejects until the entry adds an explicit import, then resolves the source
  provider id.
- why: Pair acceptance must be attributable to the closed floor, not to cached
  globals or a hidden import leak.

## AC2 — the floor is exactly ten types plus three Pair companions

### surface/taxonomy/prelude-internal-provision-inventory-is-executable-and-closed

- promise class: **durable invariant** — the two membership arms derive the
  configured floor in both directions
- spec: `30 §4`; `33 §3.3`; `39 §2.0`
- given: traverse every built-in primitive declaration type and collect its
  Ken-defined identities. Independently enumerate the internal-provision
  witnesses by explicit internal origin and exact pre-source identity. Derive
  the configured type floor as their union. Derive companion bindings separately
  from the admitted type contract and checked identity references.
- expect: the signature inventory is exactly
  `{Auth, Bool, Char, List, Option, ResourceKind, Result, Utf8Error}`; the
  internal-provision type inventory is exactly `{Nat, Pair}`, with `Nat` marked
  kernel-origin and `Pair` marked compiler-bootstrap-origin; their union is the
  exact ten-type floor
  `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
  Utf8Error}`. Pair's companion inventory is exactly
  `{mk_pair, pair_fst, pair_snd}`. The floor count is ten: companions are
  bindings, not type formers.
- controls: add `Prod` only to the configured type inventory; exact equality
  fails with one extra type. Move a Pair companion into the type inventory;
  the type-count and kind assertions fail. Delete either internal witness or
  substitute a same-shaped fresh id; exact identity equality fails closed.
- why: a hard-coded ten-name list does not prove derivation. Independent arm,
  origin, identity, kind, and companion assertions prevent both under-inclusion
  and compiler-global-map widening.

## AC3 — spelling, shape, import, and re-export do not manufacture identity

### surface/modules/definitionally-equal-pair-is-a-distinct-identity

- promise class: **durable invariant** — canonical identity is not structural
  equality
- spec: `30 §4`; `33 §3.3`/`§4.3`; `34 §"Canonical non-dependent pair floor
  family"`; `39 §2.0`
- given: record the canonical transparent floor `Pair` id, type, and body plus a
  checked declaration whose term contains that exact id. Through the ordinary
  checked-definition producer, admit another transparent definition with the
  identical type and non-dependent Sigma body under a fresh non-floor name.
- expect: the second declaration receives a fresh id. Both bodies normalize to
  definitionally equal terms, but the identity comparison remains unequal and
  the pre-existing reference remains keyed to the floor id.
- controls: require the pre-existing reference to contain the fresh id and make
  that assertion fail. Independently normalize both bodies before comparing ids
  so conversion success cannot stand in for provenance.
- why: transparent conversion operates on terms, not declaration ownership.

### surface/modules/pair-floor-binding-collisions-reject-before-allocation

- promise class: **durable invariant** — every floor binding is immutable and
  unshadowable at top level
- stage: **RED-UNTIL floor realization for the four Pair-family rows**
- spec: `33 §3.3`; `39 §2.0`
- given: in independent Strict roots, locally declare a top-level binding named
  `Pair`, `mk_pair`, `pair_fst`, or `pair_snd`, with a well-typed body or family.
  Snapshot declarations, allocator position, and trust before each attempt.
  Reuse `seed-modules.md`'s exhaustive type/constructor collision matrix for the
  other nine type-floor members and their exact constructors.
- expect: every same-spelling Pair-family row rejects as a top-level clash before
  source declaration admission. No floor identity is replaced and no snapshot
  moves. An ordinary local `data Pair a b = MkPair a b` is included in the
  `Pair` row and cannot become the transparent floor family.
- controls: rename all four local declarations. They admit with fresh ids and
  remain distinct from the floor declarations. A narrower lexical binder named
  `pair_fst` retains ordinary lexical shadowing and is not a top-level clash.
- why: an immutable floor asserted only for types could leave companion
  replacement open. The four independent rows reach every new binding class.

### surface/modules/pair-reexport-is-identity-preserving-republication

- promise class: **durable invariant** — a re-export creates a public path, not
  a provider or source owner
- stage: **RED-UNTIL floor realization**
- spec: `33 §4.3`; `34 §"Canonical non-dependent pair floor family"`
- given: record all four floor ids, then load one module that explicitly
  re-exports the four bindings and a consumer of that facade.
- expect: every facade and consumer reference selects the recorded floor id.
  Re-export admission allocates no replacement and creates no source
  `defined-at` or head-owner provenance. A direct floor reference and the facade
  path are idempotent, not ambiguous.
- controls: redirect the facade mutation to same-shaped fresh declarations; the
  exact-id assertion fails even when their bodies normalize equally.
- why: republication remains useful, but the former package-import route is not
  retained as a second provider path.

## AC4 — the reused floor family computes through Sigma

### surface/modules/pair-floor-family-is-checked-transparent

- promise class: **durable invariant** — the floor reflects existing kernel
  structure and adds no trusted primitive
- spec: `34 §"Canonical non-dependent pair floor family"`; `39 §2.0`
- given: inspect the four pre-source declarations selected by floor resolution.
- expect: all four are checked transparent definitions. `Pair A B` unfolds to
  non-dependent `Sigma A (_ : A. B)`; `mk_pair` unfolds to kernel pair
  introduction; `pair_fst` and `pair_snd` unfold to the first and second kernel
  projections. Every helper type names the exact floor `Pair` id. No declaration
  is opaque, primitive, or trusted.
- controls: independently require each declaration to be opaque and make the
  assertion fail. Compare each body with a wrong kernel former so a name-only
  check also fails. Treat a kernel former as a `GlobalId` provider and require
  the kind assertion to reject that classification.
- why: zero trust alone permits a wrong checked body. Kind, type, body, and
  identity together pin the representation.

### surface/modules/pair-floor-beta-eta-are-definitional

- promise class: **normative compatibility vector** — downstream proofs use
  conversion, not an imported theorem
- stage: **RED-UNTIL floor realization**
- spec: `34 §"Canonical non-dependent pair floor family"`; `13 §2`/`§6`
- given: with no Pair import, check
  `pair_fst Bool Bool (mk_pair Bool Bool True False) = True` with `Proved`,
  `pair_snd Bool Bool (mk_pair Bool Bool True False) = False` with `Proved`,
  and, for neutral `p : Pair Bool Bool`,
  `mk_pair Bool Bool (pair_fst Bool Bool p) (pair_snd Bool Bool p) = p`
  with `Refl`.
- expect: concrete fst and snd normalize the whole proposition to `Top` and
  accept `Proved`; `Refl` rejects because the goal is no longer `Eq`-shaped.
  Neutral reconstruction remains `Eq`-shaped and accepts `Refl`; `Proved`
  rejects. No law, import, postulate, or runtime evaluation participates.
- controls: change fst to `False`, snd to `True`, and reconstruction to the
  swapped pair in independent arms; each prescribed terminal rejects. Swap
  `Proved` and `Refl` at every correct equation; each wrong terminal rejects at
  the normalized goal shape.
- why: the equations distinguish both projections and the eta endpoint while
  pinning the two non-interchangeable proof terminals.

## AC5 — nested positivity is representation-derived

### kernel/inductive/floor-pair-positive-path-unfolds-to-sigma

- promise class: **durable invariant** — transparent representation, not a
  spelling allow-list, determines positivity
- shape: **soundness/completeness pair** — each positive orientation is paired
  with the corresponding inner-arrow negative
- stage: **RED-UNTIL floor realization for the named Pair instantiation**
- spec: `14 §8.3`/`§8.5`; `34 §"Canonical non-dependent pair floor family"`
- given: through the ordinary inductive admission path submit
  `data Good1 = MkGood1 (Pair Good1 Unit)` and
  `data Bad1 = MkBad1 (Pair (Bad1 -> Empty) Unit)`, plus
  `data Good2 = MkGood2 (Pair Unit Good2)` and
  `data Bad2 = MkBad2 (Pair Unit (Bad2 -> Empty))`. The fixture supplies
  already-resolved `Unit`/`Empty`; this case does not classify their surface
  availability.
- expect: `Good1` and `Good2` accept by unfolding the exact transparent floor
  head to Sigma and descending through the corresponding component. `Bad1` and
  `Bad2` reject when the same descent reaches an arrow domain at negative
  polarity. Neither Sigma component may be discarded.
- controls: the executing bindings
  `checked_transparent_sigma_aliases_admit_renamed_nested_paths` and
  `checked_transparent_sigma_alias_rejects_inner_arrow_negative` remain green.
  Independently suppress transparent-head unfolding, first-component descent,
  or second-component descent and require its corresponding control to red.
  Opaque and unknown heads remain fail-closed.
- why: the existing renamed aliases prove name independence. The named floor
  case proves the selected canonical declaration takes that same kernel path,
  not a Pair-specific positivity shortcut.

## AC6 — evidence staging precedes the floor-realization closure claim

### surface/modules/pre-floor-realization-census-stops-at-legacy-evidence-frontier

- promise class: **durable invariant** — available evidence bounds every
  identity and closure inference
- stage: **current pre-floor-realization evidence frontier**
- spec: `30 §4`; `33 §3.3`; `39 §2.0`
- given: enumerate the census subjects `Core.Logic.Compare`,
  `Core.Classes.LawfulClasses`, `Data.Collections.Derived`,
  `Data.Numeric.Nat.Order`, and `Algorithm.Numeric.Gcd`. Treat that list only as
  an evidence input. Run each through Legacy. For a successful row, record only
  the exact `GlobalId`s resolved in that run. For every row without successful
  exact-identity evidence, record the typed first failure as
  `Unavailable(stage)`.
- expect: a successful Legacy ledger describes only that run. It confers no
  future floor, migration, population, or Strict status. Every other row carries
  only `Unavailable(stage)`: source or diagnostic spelling, semantic
  familiarity, and numeric origins supply no missing identity or closure fact.
- controls: alter or delete a successful row's ledger and require comparison
  with resolver output to fail. Alter the first-failing stage of an unavailable
  row and require the typed-stage assertion to fail. Add migration, population,
  provider, or numerator meaning to `Unavailable` and require the artifact check
  to reject it.
- why: the operator's new target does not retroactively enrich evidence gathered
  before the floor exists.

### surface/modules/pair-floor-closure-is-rederived-after-realization

- promise class: **durable invariant** — target closure comes from fresh Strict
  execution and exact floor identities, never projected staging
- stage: **RED-UNTIL `LANG-MOD-CANONICAL-PAIR-PACKAGE` floor realization**
- spec: `30 §4`; `33 §3.3`; `39 §2.0`
- given: after floor realization, record the four canonical ids before source
  loads. Re-run the census subjects and any coupled catalog population through
  the real Strict roots loader from their actual sources. Derive Pair-family use
  only from successfully resolved emitted references, not source spelling or a
  prior import graph.
- expect: each successful Pair-family reference names the recorded floor id.
  No package import edge, Legacy ledger, `Unavailable(stage)` row, or ambient
  compiler-global route enters the successful population without a fresh Strict
  result. Rows that still fail retain their newly measured typed evidence. Floor
  installation allocates no declaration and adds no trust.
- controls: remove one Pair companion from the floor capture and require affected
  Strict rows to fail rather than fall back. Substitute a same-shaped competing
  id and require exact identity comparison to fail. Add a census row to the
  successful population without a fresh Strict result and require population
  equality to fail.
- why: a universal floor removes the former provider-import graph, but it does
  not authorize projecting today's census into tomorrow's closure numerator.

## AC7 — prior ownership obligations survive floor realization

### surface/modules/pair-floor-does-not-transfer-ord-nat-or-pair-instance-ownership

- promise class: **durable invariant** — availability does not change instance
  declaration provenance
- evidence: **by reference** to the owning module/orphan seed
- spec: `33 §5.3`/`§5.5.1`; `39 §6.1`; `51 §7`
- given: after Pair floor realization makes `LawfulClasses` Strict-loadable,
  rerun `seed-modules.md`'s
  `ord-nat-class-owner-and-reexport-use-one-dictionary` and inspect canonical
  parameterised `Ord Pair`/`DecEq Pair` registrations.
- expect: the sole `(Ord, Nat)` dictionary remains defined by
  `Core.Classes.LawfulClasses`; Order carries the same identity without
  redeclaration. Canonical Pair dictionaries are likewise class-owned in
  `LawfulClasses`, keyed by the exact floor head. No floor arm creates a source
  head owner or orphan exception.
- why: moving Pair from a future package to the floor changes availability, not
  the orphan rule.

### surface/modules/pair-floor-does-not-discharge-attached-proofs

- promise class: **durable invariant** — floor availability does not mint a
  foreign attached-proof identity
- evidence: **by reference** to the attached-proof ownership seed
- spec: `33 §8.2`; `39 §2.0`; `LANG-MOD-ATTACHED-PROOF-OWNERSHIP`
- given: after floor realization makes the coupled units available under Strict,
  rerun the provider-local accept, consumer-foreign reject, and local ordinary
  theorem controls.
- expect: `pair_compare_eq_sound`, `pair_compare_lt_asym`, and
  `bool_or_eq_true_of_or` are ordinary private theorems at their settled owning
  units; no foreign attached-proof identity is minted. All remain checked and
  outside `trusted_base()`.
- why: changing Pair's availability tier neither reverses proof ownership nor
  counts the owed conversions as delivered.

## Evidence posture

- **MEASURED, historical at
  `dcfa19210bae01edaf55914a0aeb63e9362042a4`:** the former configuration had
  nine type names and rejected each bare Pair-family name under Strict. Those
  measurements remain truthful evidence of the superseded implementation
  boundary, not the target contract.
- **MEASURED at the floor-provider spec base
  `c1945c6fbbd7b0d8422123904fc6f7138fc85df9`:** one fresh environment installs
  checked-transparent `Pair = g232`, `mk_pair = g233`, `pair_fst = g234`, and
  `pair_snd = g235`; all are outside `trusted_base()`, every helper type names
  `g232`, and the bodies are non-dependent Sigma, pair introduction, first
  projection, and second projection. The current configured floor still has
  nine type names. Across all 48 catalog `.ken.md` leaves, no source declaration
  defines `Pair`; catalog occurrences are consumers under the current Legacy
  route.
- **CHECKED now:** the representation-only transparent-Sigma positivity controls
  execute through both components and reject both inner-arrow negatives.
- **CLAIMED until the build lands:** the ten-type floor, exact three companions,
  four Strict accepts on the existing ids, collision behavior at the new
  bindings, and fresh Strict catalog closure.
- **THE GAP:** the floor-realization build has not yet admitted the four existing
  ids through the closed Strict inventory. Spec text alone does not make a
  current Strict run green.

## Consistency and realization boundary

- The type floor is exactly
  `{Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result,
  Utf8Error}`. The internal-provision inventory is exactly `{Nat, Pair}`. Pair's
  companion bindings are exactly `{mk_pair, pair_fst, pair_snd}`.
- The four names reuse one compiler-installed transparent declaration family.
  There is no package Pair, second identity, alias, registry, owner map, or
  convenience-global fallback.
- Conversion cases speak about transparent Sigma behavior; identity cases speak
  about canonical declaration ids. Definitional equality never substitutes for
  identity evidence.
- Kernel formers, runtime pair values, and local declarations named Pair remain
  outside the provider question.
- `Prod` remains the non-floor compiler-convenience negative control.
