---
id: LANG-MOD-CATALOG-COMPLETENESS
title: "WP-4 Component B — catalog completeness: give Nat and OrdResult (dedup two private copies) canonical public homes plus the fixpoint homeless-convenience census, deliver Order's provider surface + identity, migrate the consuming units (Gcd imports add/mul + leq_nat/sub + Nat and drops its reimplementations), and satisfy whole-catalog strict-green. The module/import campaign's catalog-reuse success step."
status: ready
owner: language
size: L
gate: none
depends_on: [LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Architect ruling evt_214z6r6qnwme0 (2026-08-24), unbundling the WP-4 strict whole-catalog co-gate off Component A. The co-gate gates a deliverable outside WP-4's authorized surface (a canonical home for the type Nat), so it is re-homed here. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # RELEASED 2026-08-24 — Component A merged (574eb90c0 / ee2631ff8); B is ready.
>
> Component B carries the substance the WP-4 hard stop exposed
> (language-implementer evt_3j60e77n0ahsy, Architect ruling evt_214z6r6qnwme0):
> the catalog cannot go whole-catalog strict-green while the type `Nat` (and
> other prelude conveniences the 42 red leaves consume) has no canonical catalog
> home. Component A delivers the loader realization + the provider public surface;
> B releases the missing homes and migrates the consumers. It depends on A (the
> providers must be public before consumers migrate) and is released once A lands.
>
> This is the module/import campaign's catalog-reuse SUCCESS step: when B lands
> its whole-catalog strict-green, the campaign root [[LANG-MODULE-IMPORT-SYSTEM]]
> can close and foundation [[CAT-GCD-REFACTOR]] unblocks.

# Objective

Catalog completeness under strict resolution: every convenience the catalog uses
resolves from a defining public interface (no ambient), and the whole catalog is
strict-green through the real loader.

# Deliverable

- ONE canonical `Nat` HOME (Architect canonical-home ruling evt_60na0wbpydg0y,
  base 3a7114cf70) — a SINGLE public defining interface for `Nat`/`Zero`/`Suc` in
  the `Data/Numeric/Nat` package (recommend a dedicated facade
  `Data/Numeric/Nat/Nat.ken.md`, sibling to Arithmetic/Order; name is the team's
  to finalize). Every strict consumer (Arithmetic/Order/Gcd/InsertionSort/
  Diagnostics) imports `Nat` (and `Zero`/`Suc`) from this one interface.
  Grounded basis: spec §33 §3.3 (declarations.md:259-263) — the closed prelude
  floor is EXACTLY {Bool,Char,List}; `Nat` is native/kernel but NOT in the floor,
  so under Strict it is unbound unless imported from a defining public interface.
  IDENTITY CONSTRAINT (decisive): the home must yield the SINGLE canonical native
  `Nat` GlobalId the kernel/native-arithmetic/existing proofs already use — it
  must NOT mint a second `Nat` identity. A fresh `data Nat = Zero | Suc Nat` the
  elaborator treats as a NEW type forks identity and breaks unification — that is
  the one outcome forbidden. Spec-sanctioned vehicle: an `export`/re-export facade
  (§33 §4.3 "re-export preserves identity and visibility"); `pub import` is NOT a
  valid declaration (grammar.md:89), so the vehicle is `export`, not `pub import`.
  NAT MECHANISM PREREQUISITE (potential hard stop — Architect left this to Spec's
  lane, not ruled): the census must confirm the elaborator today realizes EITHER
  (a) an `export` facade naming the native global (identity by §4.3), OR (b) a
  `pub data Nat` the elaborator RECOGNIZES as the native inductive by identity
  coincidence (the ES2/Bool pattern, LawfulClasses.ken.md:215). If NEITHER is
  implemented for a non-floor native type, STOP before authoring the `Nat` home
  and hand back that precise hard stop to Spec + Steward — do NOT improvise a
  redeclaration to work around it (the redeclaration IS the identity-fork this
  forbids). Location (`Data/Numeric/Nat`) and the single-native-GlobalId
  constraint stand regardless of which mechanism Spec confirms.
- ONE canonical `OrdResult` HOME = a NEW leaf module in the `Core/Logic` package
  (Architect ruling evt_60na0wbpydg0y; recommend `Core/Logic/OrdResult.ken.md`,
  sibling to `Or.ken.md`; AVOID the module name "Ordering" — the type name
  `OrdResult` was chosen deliberately and "Ordering" explicitly rejected at
  Derived.ken.md:45-52). Author EXACTLY this interface, mirroring the landed
  canonical-Or shape (Architect correction evt_22r45y0x8nzbh — NOT `pub data`,
  which is an ABSTRACT export that WITHHOLDS the constructors per spec §33 §4.2,
  producing the implementer's `UnresolvedCon OrdResult.Eq`): ordinary `data`,
  three `const`s, and ONE `export` line naming the type + its constructors + the
  consts — `data OrdResult = Lt | Eq | Gt`; `const ord_eq : OrdResult = Eq`;
  `const ord_lt : OrdResult = Lt`; `const ord_gt : OrdResult = Gt`;
  `export OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt`. Identity-preserving
  (§33 §4: `export foo` has the same interface effect as `pub foo` with no second
  identity) AND constructor-publishing (the Or precedent, Core/Logic/Or.ken.md:
  13-18). OrdResult authoring is UNBLOCKED.
  WHY Core/Logic (not LawfulClasses, not Derived): OrdResult is a foundational
  3-way comparison-result primitive, a sibling of `Core.Logic.Or`, and sits BELOW
  the Ord class layer (the Ord class returns it, never the reverse). Core/Logic
  already homes exactly these primitives (Or/Transport/EmptyDec) and is already
  imported by all three consumers — ZERO new package edge, no cycle. Homing in
  LawfulClasses forces Derived onto the whole Ord-class module; homing in Derived
  creates a backwards Numeric/Order -> Collections edge.
  DEDUP BOUNDARY: DELETE the two private competing decls (Derived.ken.md:69 data +
  :71/:73/:75 consts; Order.ken.md:188 data); ADD `import Core.Logic.OrdResult
  (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)` to Derived, Order, AND
  LawfulClasses (the last closes its homeless references; it declares none today).
  Derived's derived helpers (ord_result_leq, ord_result_dispatch2, eliminators,
  pair_compare + lemmas) STAY in Derived — not cross-referenced at base; promote
  one later only if the census shows a second importer.
  IDENTITY CONSTRAINT: after dedup EXACTLY ONE defining `data OrdResult` => ONE
  canonical GlobalId; every consumer resolves OrdResult/Lt/Eq/Gt/ord_* to THAT
  single identity, else LawfulClasses's `Equal OrdResult (compare ...) ord_eq`,
  Order's `compare : Nat -> Nat -> OrdResult`, and Derived's `pair_compare`
  do not unify. Migration is identity-preserving: only the DECL relocates; consumer
  bodies are byte-unchanged apart from the added import line.
- Canonical public homes for the OTHER homeless conveniences the provider +
  consumer closure requires — census-driven, see the homeless-convenience closed
  predicate below.
- Order's PROVIDER SURFACE (moved from Component A, HS#5 — Order is not
  self-measurable in A because its closure needs homeless `OrdResult`): make
  `leq_nat`/`sub` `pub`; add Order's provider-internal imports `import
  Core.Logic.Transport (cong, trans)` (retaining Or) and `import
  Core.Classes.LawfulClasses (IsTrue, bool_or, Ord)`; make Transport's cong/trans
  and LawfulClasses's IsTrue/bool_or/Ord `pub` as needed. (Arithmetic's provider
  surface — Transport import + pub add/mul — is delivered by Component A and
  carries forward; B does NOT re-add it.)
- Migrate the consuming units to import the canonical `Nat` (and `OrdResult`)
  homes so they resolve under strict. Gcd selectively imports `add`/`mul`
  (Arithmetic) + `leq_nat`/`sub` (Order) + `Nat`, and REMOVES its four local
  reimplementations.
- Move the real caller to STRICT (`elaborate_module_from_roots` strict mode)
  after the dependency census has migrated — this is the flag-day the legacy A
  loader defers.
- HOMELESS-CONVENIENCE CENSUS as a CLOSED PREDICATE (Architect HS#5 ruling
  evt_613d9fm7j45qj — census all at once, not one hard-stop at a time). A name is
  a homeless convenience iff it is referenced within a genuine provider closure an
  AC requires, has NO defining PUBLIC interface in any catalog module, and is not
  native-prelude / floor {Bool,Char,List} / kernel. METHOD (mechanical): run the
  legacy roots-load of B's full provider+consumer closure to FIXPOINT, collecting
  EVERY `UnresolvedCon`/`UnboundName` that is not native/floor/kernel — that set
  IS the homeless census; author a canonical home for each. Known members:
  `OrdResult` (+ `ord_eq`/`ord_lt`/`ord_gt`). `Nat` is NOT homeless for the legacy
  path (native prelude) but IS a strict-home item here. Do not rediscover members
  one stop at a time.
- FORWARD-COMPAT identity preservation (Architect ruling evt_47t9dwz0chstv):
  strict excludes the native prelude `Nat`, so B re-homes `Nat` to a canonical
  catalog interface and migrates the providers to import it — but B MUST PRESERVE
  Arithmetic's provider identities (`add`/`mul`) that Component A's pub surface
  already exposed and measured under AC-A2, when it re-homes `Nat` and moves the
  caller to strict. No competing provider identity is minted by the `Nat`/
  `OrdResult` re-homes. This is the NODE B canonical-home pattern (as
  `Core.Logic.Or` replaced the prelude `Or`).

# The census (re-homed from WP-4; drift corrected)

The behavior census at base `f0e0b92fa` is 45 leaves = 3 strict-green (the
self-contained green set, delivered by Component A) + 42 red. WP-2 D0's recorded
premise of 32 baseline-red residuals is STALE: the current behavior census is 34
baseline-red residuals (language-implementer evt_3j60e77n0ahsy). Completion
ranges over the real population, not the stale count.

# Acceptance criteria

- AC-B1 (the re-homed co-gate). The whole catalog is strict-green in CI — the
  co-gate with [[LANG-MOD-STRICT-RESOLUTION]], whose remaining whole-catalog
  strict enforcement / CI closure co-closes here. Local targeted `-p` only;
  whole-catalog strict-green is a CI gate, never a local `--workspace` run.
- AC-B2. Arithmetic, Order, and Gcd each check STANDALONE through the real loader
  under strict (the AC-1 re-homed from WP-4 — now satisfiable because `Nat` and
  `OrdResult` resolve through their canonical homes).
- AC-B2a (Order provider identity — moved from A's AC-A2, HS#5). Order's
  `leq_nat`/`sub` FULLY elaborate and publish their genuine `GlobalId`s through
  the real loader, observed by IDENTITY (not repo text, not a frozen id, no
  competing identity) — measurable in B once Order's closure (Transport +
  LawfulClasses + canonical OrdResult) resolves.
- AC-B3 (residual triage). Completion ranges over the 34-residual population:
  each residual is either migrated to strict-green OR explicitly excluded with a
  stated reason. "Every census vector empty" is NOT sufficient — enumerate the
  disposition of all 34.
- AC-B4. Gcd's four imports resolve to the exact provider IDs with no Gcd-owned
  competing identity — establish no-reimplementation by IDENTITY, not repo text.
- AC-B5 (canonical-home identity — Architect ruling evt_60na0wbpydg0y). Exactly
  one defining `data OrdResult` exists in the catalog (at `Core/Logic/OrdResult`),
  the two private copies at base deduped away; and `Nat` is served by a single
  `Data/Numeric/Nat` public interface that RE-EXPORTS the native identity (NO new
  catalog `data Nat` — a redeclared/forked `Nat` GlobalId is forbidden). Every
  consumer resolves `Nat`/`Zero`/`Suc` to the single native GlobalId and
  `OrdResult`/`Lt`/`Eq`/`Gt`/`ord_*` to the single OrdResult GlobalId. No second
  identity, no ambient/floor `Nat`.
- AC-B5a (Nat native-export mechanism — CONFIRMED Spec prerequisite, fired at
  3a7114cf7; Architect evt_22r45y0x8nzbh). Grounded: `export Nat, Zero, Suc`
  succeeds under Legacy (binds native identities) but under STRICT fails at the
  export declaration with `UnboundName Nat` — the facade unit's own strict scope
  has no native Nat (non-floor, unimported, non-ambient), and dependency units
  inherit one coherent ResolutionMode (no strict-consumer-loads-facade-via-legacy
  escape). The elaborator has NO strict-capable identity-preserving native export
  facade today. Nat-home authoring is HELD pending a Spec mechanism ruling on the
  SHARP QUESTION: under Strict, how does a non-floor native inductive
  (Nat/Zero/Suc) acquire a strict-resolvable public defining interface WITHOUT
  forking its canonical identity? Two candidate mechanisms (Spec rules which):
  (a) make strict export-name resolution bind a kernel/native global (an
  elaborator change — §4.3 preserves identity but never runs because the export
  cannot RESOLVE the native name under strict); (b) a real `data Nat = Zero | Suc
  Nat` the elaborator RECOGNIZES as the native inductive by identity coincidence
  (the ES2/Bool path, LawfulClasses.ken.md:215) — if that recognition generalizes
  to Nat it is NOT the forbidden fork but the canonical native identity (still
  requiring an import, Nat being non-floor). Architect leans (b)/ES2-Bool as the
  likely clean answer; Spec first determines whether native-inductive recognition
  already covers Nat or must be extended. B does NOT improvise a redeclaration
  that forks identity. OrdResult migration + census + Gcd reuse are UNAFFECTED and
  proceed as a partial increment while Nat waits (COORDINATION §10-).
- AC-B7 (homeless census closed). The fixpoint homeless-convenience census (see
  the Deliverable) is run and its FULL set is enumerated with a canonical home
  authored for each — not rediscovered one hard-stop at a time. An empty
  next-iteration census is the completion signal.
- AC-B6 (cross-cutting invariant). Zero `trusted_base()` delta; flat-Σ pin stays
  green.
- AC-B-NO-REGRESSION. Whole-suite green in CI; local targeted `-p` only.
- END-STATE INVARIANT (Architect ruling point 4). The strict contract is
  correct: every name resolves from a defining public interface, no ambient. B
  delivers that end state; it does NOT weaken strict to reach green.

# Reviewers

Architect (canonical-home component fit; one-canonical-Nat; no invented identity)
+ conformance-validator (identity-preserving import resolution; strict-green
census disposition).

# Capability tier

T1 for the canonical `Nat`-home design (the defining interface + its catalog
location, one-canonical-Nat) and the strict-green closure argument; T2 for the
mechanical convenience-home authoring + consumer migration + census execution.
Size L (may grow with the convenience-home breadth the 34 residuals require).
