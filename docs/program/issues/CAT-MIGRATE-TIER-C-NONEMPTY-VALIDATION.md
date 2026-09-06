---
id: CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION
title: "Scaffold-retirement Tier C, held component: migrate the {NonEmpty, Validation} WCC off fixture scaffolding onto real imports, once their split-out predecessors are published. Same per-module publish+import+standalone shape as the Tier C ready lane; NonEmpty before Validation (the intra-tier edge). NO class-instance relocation, NO invented pub instance."
status: draft
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-MIGRATE-LF-SEMIGROUP-PUBLISH, CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS, LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE, LANG-ABSTRACT-EXPORT-PARAM-ELAB, CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]
blocks: []
github: null
origin: "Steward, 2026-09-03, split out of [[CAT-MIGRATE-TIER-C-DATA-VALUE]] on the confirmed D0 census (foundation evt_19kq7r92attpy, Architect confirmation evt_4hp6qxkdaqgbz). The census measured {NonEmpty, Validation} as a WCC (edge NonEmpty -> Validation) that sits behind UNPUBLISHED split-out providers, so the DAG-axis discipline holds it out of the ready lane: NonEmpty needs private LF Semigroup ([[CAT-MIGRATE-LF-SEMIGROUP-PUBLISH]]); Validation needs private EC apply_to/compose/functor_map_of/Applicative AND EC itself roots-loads red at Functor / Functor_instance_Identity, i.e. blocked on the Language roots-loader faces-3 cross-module export+import predecessor ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up) PLUS the EC provider-widen ([[CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS]]). Validation also names Semigroup_instance_NonEmpty (a synthesized dict) in a checked example — that resolves ONLY via the predecessor's imported-head carry, NEVER an invented pub instance. Held until all three predecessors land; then released one behind the ready lane."
---

> # RECUT 2026-09-06 to STEP P3 (flip-only) of the Architect's 3-step additive
> # staging DAG (evt_5fxtzhk104q96). The prior monolithic scope (candidate
> # 94b061de) is SUPERSEDED -- it bundled the smart ctors + abstract flip and
> # reds cc7/cc8, because a strict importable module hides its raw ctor from the
> # ambient Schema/ArgParse consumers that still name it (spec 33 §4.2/§4.3:
> # module pub data is abstract-export only; "importable + public raw ctors" is
> # unexpressible). So the code motion is split ahead of the flip: smart ctors ->
> # P1 [[CAT-MIGRATE-TIER-C-NONEMPTY-SMART-CTORS]], raw call-site swap -> P2
> # [[CAT-MIGRATE-TIER-C-NONEMPTY-CALLSITE-SWAP]] (both additive, ambient). THIS
> # node (P3) does ONLY the logic-free flip: {NonEmpty,Validation} -> strict
> # abstract importable modules (raw ctors hidden, SAFE once P2 removes the last
> # raw consumer); import lines into consumers {Schema, ArgParse,
> # Configuration.Decoder, Forge}; cc7/cc8 (+cc1 if still ambient) harness legs ->
> # roots loader. Atomic over its loading closure (a strict module is not
> # ambiently visible; the roots loader resolves the whole closure together) but
> # small + logic-free because P1/P2 moved the code ahead of it.
> #
> # HELD (status draft) pending P2; depends_on adds the callsite-swap (the
> # original 4 predecessors are all merged). The Steward rewrites the
> # Deliverables/ACs below to the FLIP-ONLY scope and flips this ready when P2
> # lands (one-release-ahead). The BODY BELOW is the OLD monolithic
> # D1-abstract+D2-Validation scope -- NOT the P3 spec; do NOT build from it until
> # the recut. Carry Architect z3570 bounded notes into the P3 rewrite: (a)
> # Validation law witnesses pub proof/theorem -> privatize (option_map precedent)
> # or list in §7; (b) removed no-Monad-registry guard -> behavioral replacement.
> # Architect gives P3 its FULL soundness pass (abstraction boundary + new loading
> # semantics); P1/P2 are additive/mechanical. Prior RE-RELEASED/RE-HELD history
> # retained below for context.
> #
> # RE-HELD 2026-09-06 after a D1 hard stop. It WAS released 2026-09-06 (its
> # three original predecessors merged); the foundation ring ran D0 clean, then D1
> # hard-stopped: `pub data NonEmpty` at the module level flips the checked
> # `nonempty_append::assoc` proof to kernel `NotAFunction { head: Type 0 }` —
> # publishing the data surface while retaining the module's checked law is a
> # genuine semantic/publication boundary, not the framed selective-import
> # migration. Architect ruling evt_4s9y6bpyzgaet: TWO elaborator defects (the
> # module `pub data` divert mints a nullary opaque constant, dropping the
> # parameter telescope, and withholds constructors from the defining module too);
> # component design = make NonEmpty a proper ABSTRACT type with SMART
> # CONSTRUCTORS, NOT concrete public constructors. Spec enclave CONFIRMED both
> # underlying clauses are derivations of the current contract (spec-author
> # evt_7vpq0673kjcyp).
> #
> # So this node is BLOCKED on the elaborator predecessor LANG-ABSTRACT-EXPORT-
> # PARAM-ELAB (the two-faced/arity fix) — the sole depends_on edge. Its other
> # predecessor, the §4.2 spec text LANG-ABSTRACT-EXPORT-PARAM-spec, has ALREADY
> # LANDED (560dd455b) as the confirmed contract; it is a spec-only pin with no
> # tracker node, so it is NOT a depends_on edge (recorded here in prose only). The
> # Steward re-releases after ELAB lands, with D1 rescoped (below). D0's
> # measurement stands (foundation-implementer
> # evt_6whyzawx7qtcr: import-only NonEmpty scratch closure green; the trigger is
> # the public-surface step). Foundation stands down until re-release.
> #
> # Tier C held component: {NonEmpty, Validation}. NO class-instance relocation,
> # NO invented pub instance; the synthesized Semigroup_instance_NonEmpty resolves
> # via imported-head carry ONLY.

This node is the held WCC that [[CAT-MIGRATE-TIER-C-DATA-VALUE]] split out. It is
NOT a regression fix — both modules elaborate today under ambient class-install;
this brings them to the scaffold-retirement end state, exactly as the ready lane
does for SB..Vector. Do not treat standalone-red as a bug: it is the starting
condition each increment closes (see the parent frame's "Not a regression fix").

## Blocked-on (the DAG axis — release only when all three have landed)

- **NonEmpty** needs private LF `Semigroup` -> [[CAT-MIGRATE-LF-SEMIGROUP-PUBLISH]]
  (a visibility-only provider-widen on LF, minted alongside this).
- **Validation** needs private EC `apply_to`/`compose`/`functor_map_of`/
  `Applicative` -> [[CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS]], AND the EC module
  roots-loads red at `Functor` / `Functor_instance_Identity`, blocked on the
  Language roots-loader faces-3 cross-module export+import predecessor
  ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up increment).
- **Validation** names `Semigroup_instance_NonEmpty` (a synthesized dict) in a
  checked example: it resolves via the predecessor's imported-head carry ONLY.
  Never mint a `pub instance` to satisfy it.

## Deliverables

- **D0 — re-measure at the release SHA.** By the time the predecessors land, the
  standalone `UnresolvedCon`/`UnboundName` sets and the exact provider heads may
  have shifted; re-census both modules against the then-published providers
  before authoring the import blocks. NonEmpty before Validation.
- **D1 NonEmpty — RESCOPED to abstract export + smart constructors** (Architect
  evt_4s9y6bpyzgaet; requires the two new predecessors landed). Make NonEmpty a
  proper ABSTRACT data type: drop raw `NonEmptyCons` from the package Public API
  (NonEmpty.ken.md:135); add `pub fn nonempty_singleton (a:Type) (x:a) : NonEmpty
  a = NonEmptyCons a x (Nil a)` and `pub fn nonempty_cons (a:Type) (x:a) (rest:
  List a) : NonEmpty a = NonEmptyCons a x rest`; keep the accessor set
  (head/tail/to_list/map/append) + the Semigroup instance/law public. The raw
  constructor stays internal; the `nonempty_append::assoc` proof stays internal
  and valid (it uses the constructor in the defining module, which the elaborator
  two-faced fix restores). Then the publish + selective-import + standalone-green
  shape as before, over the abstract surface.
- **D2 Validation — publish + import + standalone**, after D1, unchanged in shape:
  publish exactly the measured export surface, selective import from the published
  lower tiers, retire ambient reach, extend the loader inventory, standalone exit
  0. The `Semigroup_instance_NonEmpty` example still resolves via imported-head
  carry only.

## Acceptance criteria — proven shape, adjusted for the D1 abstract-export rescope

From [[CAT-MIGRATE-TIER-C-DATA-VALUE]]: AC-EXPORTED (per published symbol,
loader-resolved with a still-private-sibling control), AC-EXACT-INVENTORY (per
module, per-symbol reddening mutation), AC-STANDALONE-GREEN (removing the import
line restores the exact prior standalone failure), AC-NO-REGRESSION (complete
affected-target closure, scoped by changed paths, targeted via `scripts/ken-cargo`,
never `--workspace` — green in CI is the workspace verdict).

**AC-VISIBILITY-ONLY — adjusted for the abstract-export rescope.** Pub-widening of
EXISTING symbols (accessors head/tail/to_list/map/append, the Semigroup instance
and its law) is a byte-unchanged body. The ONLY permitted new definitions are the
two Architect-specified smart constructors (`nonempty_singleton`, `nonempty_cons`)
whose bodies are exactly the trivial `NonEmptyCons` wrappers named in D1 — no other
new pub symbol, NO second class/instance, NO invented `pub instance`. Raw
`NonEmptyCons` is REMOVED from the client Public API (the type is abstract to
clients); the raw constructor stays internal to the defining module.

**AC-ABSTRACT (new, per Architect evt_4s9y6bpyzgaet).** A CLIENT module cannot
match or construct `NonEmpty` via its raw constructor (opaque view); it constructs
only through the smart constructors and eliminates only through the accessors.
Control: the raw-constructor reference that compiles INSIDE the defining module is
rejected from a client.

## Gate, reviewer, sequencing

`gate: none`. On each increment's candidate: **Architect** (required — surface
correctness + class-uniformity + the imported-head carry for the synthesized
dict) + **Foundation QA + CV** on the exact SHA, then Steward M1-M4 ->
lieutenant. Released one behind the [[CAT-MIGRATE-TIER-C-DATA-VALUE]] ready lane,
once all three predecessors have landed.
