---
id: LANG-MOD-CATALOG-COMPLETENESS
title: "WP-4 Component B — catalog completeness: give Nat and OrdResult (dedup two private copies) canonical public homes plus the fixpoint homeless-convenience census, deliver Order's provider surface + identity, migrate the consuming units (Gcd imports add/mul + leq_nat/sub + Nat and drops its reimplementations), and satisfy whole-catalog strict-green. The module/import campaign's catalog-reuse success step."
status: active
owner: language
size: L
gate: none
depends_on: [LANG-MOD-CATALOG-REALIZATION]
blocks: []
github: null
origin: "Architect ruling evt_214z6r6qnwme0 (2026-08-24), unbundling the WP-4 strict whole-catalog co-gate off Component A. The co-gate gates a deliverable outside WP-4's authorized surface (a canonical home for the type Nat), so it is re-homed here. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # ACTIVE — authorized PARTIAL landed; remainder HELD. Not terminally closed.
>
> ## AUTHORIZED PARTIAL LANDED 2026-08-24 at squash `76426e9f9`
>
> The unblocked partial-B increment MERGED: the OrdResult canonical home + the
> homeless-convenience census + the Rosetta flat-source compat repair. Gates on
> exact `1c68a1ad`: QA `evt_1nwzamjjrw2pq`, Architect `evt_533cq9p3jx0ew`, CV
> `evt_1tvvfenq4gmfa`; Decision `dec_7j5v11ejfh933`; Adversary M8 CLEAN
> `evt_ryrmff37zene`.
>
> This node is NOT terminally closed. The corpus close `526e920e8` ("corpus:
> close LANG-MOD-CATALOG-COMPLETENESS") was an M7 over-close of an authorized
> PARTIAL and is corrected here (status merged -> active): the whole-catalog
> strict-green criterion and the Nat home (AC-B5a) remain HELD on operator
> Decision `dec_1kqwn6hdvn7d2`. `active` (accepted-partial, in-flight), not
> `ready`, because the remainder is held on an operator decision, not startable.
> The node closes only when that Decision resolves and the Nat prerequisite lands.
>
> Post-merge follow-up (Adversary `evt_ryrmff37zene`, non-blocking, for the
> record): the Rosetta compat harness guards the stripped Compare/Derived import
> edges with exact-cardinality panics, but has NO guard that the three un-stripped
> providers (Transport/Or/OrdResult) carry zero import edges — a future catalog
> import on any of the three would silently regress the harness to the UnboundName
> CI-red class (the same consumer-under-CI family as the two prior walls). Cheap
> hardening for the ring's next `rosetta.rs` touch: assert no residual `import`
> line remains in those three sources after extraction.
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
- AC-B5a (Nat native-export mechanism — RULED a Spec+build PREREQUISITE, not an
  in-WP fix; spec-author evt_33bwgcx226bxv, spec-leader evt_7nvtrx1fs6wf0,
  Architect deferred evt_22r45y0x8nzbh). Grounded at 3a7114cf7: under STRICT
  `export Nat, Zero, Suc` has no lawful source (Nat is non-floor native,
  unimported, non-ambient); Legacy cannot cross the mode boundary; a fresh
  `data Nat` allocates a SECOND family (forks the canonical kernel identity —
  structural similarity is not canonical identity); ambient/prelude promotion
  contradicts the closed-floor/package contract. The Architect's ES2-Bool
  recognition lean is FORECLOSED by that identity reasoning. The enclave RULED
  the mechanism: a narrowly-scoped compiler-realized package-provider registry
  binding one designated module path to the existing kernel-checked
  {Nat,Zero,Suc} identities (constructor-parent-validated), local to the
  provider unit under Strict, with ordinary export/import/re-export thereafter;
  NO new Decl / GlobalId / `trusted_base()` entry / surface syntax /
  mixed-resolution escape; all invalid registry/origin states fail closed.
  Durable artifact: coupled normative amendments to spec 30-taxonomy §4/§5,
  33 §3.3, 39 + identity / strict-accept / zero-allocation conformance pins.
  The prerequisite is now the reframed spec WP [[LANG-MOD-NAT-PROVIDER-INTERFACE]]
  + its build half [[LANG-MOD-NAT-FLOOR-REALIZATION]]. Decision dec_1kqwn6hdvn7d2
  RESOLVED (2026-08-25): the provider-registry mechanism is SUPERSEDED — the
  operator ruled the prelude membership rule itself the defect, and Nat's home is
  prelude-floor membership (admit the existing kernel {Nat,Zero,Suc} into the
  strict floor, reuse identity). B does NOT improvise a redeclaration; Nat
  authoring, strict-caller migration, and whole-catalog strict-green stay STOPPED
  until the build WP lands. Per the
  Architect one-pass ruling (evt_1d2cwjd3e4tjx, AC-B9): the OrdResult home + the
  relocated Core/Logic combinators + Derived + LawfulClasses + the census
  proceed as the partial increment meanwhile; ALL of Order and Gcd's Order/Nat
  imports are held with Nat (COORDINATION §10-).
- AC-B7 (homeless census closed). The fixpoint homeless-convenience census (see
  the Deliverable) is run and its FULL set is enumerated with a canonical home
  authored for each — not rediscovered one hard-stop at a time. An empty
  next-iteration census is the completion signal.
- AC-B8 (foreign-subject attached proofs — RULED convert-to-local; spec-author
  evt_6wbz5eeh5v1y6, Architect evt_7d90kztv4n7hv part I, spec-leader
  evt_2fgr39s4ghebn). A consumer may NOT attach a proof to an IMPORTED subject:
  spec 33 §8.2 closes the attached namespace under the subject's defining module
  (attachment is namespacing with ZERO soundness weight), and
  `resolve_attached_ref`'s provider-anchored lookup is the DESIGNED ownership
  rule, not a defect. B converts LawfulClasses' `proof eq_sound for pair_compare`
  and `proof lt_asym for pair_compare` to ordinary private Lawful-local
  `theorem`s (`pair_compare_eq_sound`, `pair_compare_lt_asym`), repointing the
  one `lt_asym` and three `eq_sound` selector uses in `pair_ord_leq`; same proof
  bodies, no pub/export/import edge, `pair_compare` keeps its exact Derived
  identity, no `Data.Collections.Derived.pair_compare::{eq_sound,lt_asym}`
  identity/export minted, zero `trusted_base()` delta. A roots-loader acceptance
  control pins it (both Lawful-local theorem identities exist and are used by
  `pair_ord_leq`; the two forbidden Derived attached identities/exports absent).
  The coupled durable enclave artifact — the 33 §8.2 / 39 §2.0 closure
  clarification + its provider-local-accept / consumer-foreign-reject /
  consumer-local-theorem-accept conformance — is node
  [[LANG-MOD-ATTACHED-PROOF-OWNERSHIP]]; it codifies the existing reject path
  and does NOT gate B's build.
- AC-B9 (cluster placement — §1b full-closure RULED in ONE pass; Architect
  evt_1d2cwjd3e4tjx on the ring's fixpoint report evt_407n7demb9ppm; Steward
  adjudication evt_21bem2w7rzj2k). After three surface-mechanism walls (pub-data
  spelling, class-Eq collision, attached-proof ownership), the remaining
  OrdResult/Order/LawfulClasses/Derived placement was ruled in one pass, not
  per-wall (§1b accumulate-then-rule, NOT the §1a stuck-pair trigger). Five
  rulings:
  1. The Lawful-INDEPENDENT combinators (`pair_compare`,
     `pair_compare_result_of`, the `pair_compare_lt_cases` family + its
     provider-owned `eq`/`eq_cases` attachments, `list_compare`, `list_eq` —
     OrdResult-only, no Ord/compare dep) move DOWN into Core/Logic (the OrdResult
     home or a dedicated `Core/Logic/Compare` sibling — team's factoring choice),
     imported thence by both Derived and LawfulClasses. The compound
     `Ord (Pair a b)`/`Ord (List a)` instances DO NOT move (they need Lawful
     Ord/compare) — they stay in LawfulClasses and import the combinators
     downward. This removes the Core/Classes -> Data/Collections inversion by
     relocating combinators, not instances; do NOT leave them in Derived.
  2. `instance Ord Nat` is HEAD-SIDE (orphan check, spec 33 §5.3) — it lands with
     the Nat home once the Nat mechanism makes Nat head-local. The class-side
     move to LawfulClasses is REJECTED (does not unblock Order; couples a
     Nat-specific instance into the general class module). All of Order stays in
     the Nat-blocked bucket.
  3. Order's `proof eq_true_of_or for bool_or` converts to a private Order-local
     `theorem bool_or_eq_true_of_or` (same closed-attachment law as AC-B8) —
     part of Order's held content.
  4. `Prop -> Omega` on the two promoted eliminator motives
     (`ord_result_elim`/`elim2`) is PERMITTED as an identity-preserving
     strict-spelling repair (Omega is the reserved kernel proposition former, in
     strict scope; `Prop` is the non-floor native alias strict correctly refuses
     — same class as Nat). It is checker-guarded: a non-coinciding sort would red
     every consuming proof, not go unsound.
  5. UNBLOCKED partial-B (proceed now): the OrdResult home (type + ctors + consts
     + the four promoted eliminators with Prop->Omega) + the relocated Core/Logic
     combinators + Derived (97 decls) + LawfulClasses (106) with the
     `pair_compare_{eq_sound,lt_asym}` local-theorem conversions + the homeless
     census. HELD on the Nat prerequisite (Decision dec_1kqwn6hdvn7d2): all of
     Order (`leq_nat`/`sub`/`compare` + `instance Ord Nat` +
     `bool_or_eq_true_of_or`) and Gcd's Order/Nat imports. Per-consumer OrdResult
     selective imports (Architect evt_6apbbf5bmfcj0): LawfulClasses
     `(OrdResult, ord_eq, ord_lt, ord_gt)` consts-only (avoids the `class Eq`
     collision); Order `(OrdResult, Lt, Eq, Gt)`; Derived all seven. Hand the
     unblocked increment back at the merge gate; Architect + CV review.
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
