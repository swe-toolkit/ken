---
id: CAT-MIGRATE-TIER-C-DATA-VALUE
title: "Scaffold-retirement Tier C (critical path): migrate the Data value modules off fixture scaffolding onto real imports from the already-published Tier-A/B providers, so each elaborates standalone. Per-module: publish the module's own export surface, replace ambient resolution with a real selective import from the published lower tiers, extend the loader-visible inventory, standalone-green. The proven Tier-A / EC publication-and-import shape; NO relocation of class instances, NO proof authoring beyond attached-owner migrations the Architect names."
status: merged
owner: foundation
size: L
gate: none
tier: T2
depends_on: [CAT-MIGRATE-TIER-B-PROVIDERS, CAT-MIGRATE-TIER-B-CLASSES]
blocks: []
github: null
origin: "Steward, 2026-09-03, framed one release ahead per CAT-SCAFFOLD-RETIREMENT sequencing (each tier framed as its predecessor lands). Tier B is complete — P (CAT-MIGRATE-TIER-B-PROVIDERS) and the class-owner relocation (CAT-MIGRATE-TIER-B-CLASSES) both merged (main 65ef1fae7) — so the 'gated on Tier B' hold is discharged. Tier C is the DecEq/class-consumer tier the relocation unblocks. Partition axis + module set from the Architect decomposition evt_2e0pee5jxzv07 (the measured catalog VALUE-dependency DAG, not the directory tree). Intra-tier ordering + final WP granularity were routed to the Architect (my consult evt_2yz0f22qaq33e); the Architect CONFIRMED the delegated-D0 model (evt_2r4mr5bqg0mhr) — measure the intra-tier DAG in D0 (Architect confirms completeness), emit both the order and the granularity from the census. Frame FINALIZED and release-ready; released when a foundation seat frees (EC + its predecessor are off the critical path and must not preempt this)."
---

> # Scaffold-retirement Tier C: the Data value modules. Critical path.
> # Per-module publication + import clean-ification (the Tier-A / EC shape):
> # publish own surface, repoint own consumption to already-published Tier A/B,
> # extend loader inventory, standalone-green. NO class-instance relocation.
>
> Each Data value module in the set below resolves one or more provider symbols
> ambiently (via the whole-catalog class-install / scaffolding fallback) and
> fails standalone elaboration. This tier gives each a real selective import
> from the already-published lower tiers (Tier A providers: Transport, Derived,
> Compare, Arithmetic, Nat.Order; Tier B: LawfulClasses the class owner,
> StringBijection) so each elaborates on its own. Every provider this tier
> imports is ALREADY PUBLISHED — no consumer-only increment references an
> unpublished provider (the DAG axis).

## Not a regression fix (read before treating standalone-red as a bug)

These modules elaborate TODAY in the full-catalog build via ambient class-install
(the operator's class-uniformity ruling, 2026-09-02). Nothing is on fire. This
tier is a standalone-CLEANNESS quality tier: it brings each Data value module to
the scaffold-retirement end state (`zero catalog dependence on fixture
scaffolding / ambient resolution`, [[CAT-SCAFFOLD-RETIREMENT]]). "Module X fails
standalone at UnresolvedCon/UnboundName Y" is the STARTING condition each
increment closes, not a defect on `main`.

## D0 CENSUS DONE + CONFIRMED — this node = the READY lane (8 modules)

D0 census (foundation evt_19kq7r92attpy) measured TEN modules (authoritative,
not nine) as an acyclic DAG with 7 WCCs and exactly three intra-tier edges
(`SB -> StringKeys`, `Sums.Combinators -> Map`, `NonEmpty -> Validation`). The
Architect CONFIRMED it (evt_4hp6qxkdaqgbz): the census IS the completeness proof;
the WCC split is correct; the ready-first order is sound. Granularity call
(Steward, Architect-confirmed): this node carries the READY lane as per-module
increments; **{NonEmpty, Validation} are SPLIT OUT to a held node** —
[[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]] — because they sit behind unpublished
split-out predecessors (below), the DAG-axis discipline every tier respects.

## Module set — READY lane, this node, per-module increments IN THIS ORDER

1. StringBijection (SB) -> 2. StringKeys (edge `SB -> StringKeys` is TRANSITIVE
   through LawfulClasses — SB -> LC -> StringKeys — NOT a direct StringKeys
   free-symbol edge; increment 2 is a CLOSEOUT, see the disposition below)
3. BytesKeys (BK) — pure consumer since Tier B (`DecEq UInt8`/`DecEq Bytes`
   relocated to LC); standalone-cleanness closeout, no new proof content
4. Sums.Combinators -> 5. Map (edge `Sums.Combinators -> Map`)
6. Codec  7. Deque  8. Vector (independent consumers of Tier A/B)

Map (increment 5) additionally carries the `bool_and`-family attached-owner
relocation to LC, ruling (a) below.

### Increment 2 (StringKeys) — RECLASSIFIED to a standalone-cleanness CLOSEOUT (Steward disposition 2026-09-03)

D0 re-measurement at clean base `c3bb29c81` (foundation-implementer
evt_6222whgedadsr, foundation-leader evt_7rj2smke4t430) FALSIFIED the
direct-import premise: the `SB -> StringKeys` edge is TRANSITIVE through
LawfulClasses. StringKeys is already standalone-green (`ken check` exit 0),
owned/public inventory EMPTY, and directly consumes only LC's `string_deceq_eq`
and `string_ord_leq` (from its existing selective LC import). A direct SB import
is REDUNDANT and its removal stays green — so `AC-STANDALONE-GREEN`'s
missing-import reddening control is INAPPLICABLE here (no required direct import
to red). The ring is right to refuse an unused import; none is added.

Increment 2 is therefore the same shape as increment 3 (BytesKeys): a measured
production NO-OP, test-only, NO source edit to StringKeys. Deliverable = the
controls that ARE honest and DO have power:
- StringKeys owned/public inventory is EMPTY — a mutation adding a
  StringKeys-local decl reds; a selective import of any StringKeys-local name
  rejects `UnboundName`.
- StringKeys's exact direct consumed set from LC is `{string_deceq_eq,
  string_ord_leq}`, per-symbol reddening — dropping either from the LC import reds
  the corresponding checked fence (measured); dropping `DecEq`/`Ord` as class
  names stays green, and that asymmetry is itself an assertion.
- Do NOT re-assert the transitive canonical SB identity or the no-competing-mint
  negative — increment 1's landed StringKeys control
  (`string_keys_closure_retains_the_published_injectivity_identity`) already owns
  those. CITE it; do not duplicate.

Reviewers unchanged (Architect confirms the closeout is a true no-op + the
transitive-edge refinement; Foundation QA + CV; then Steward M1-M4 -> lieutenant).

### SPLIT OUT — held, NOT this node
- **{NonEmpty, Validation}** -> [[CAT-MIGRATE-TIER-C-NONEMPTY-VALIDATION]], held
  behind: [[CAT-MIGRATE-LF-SEMIGROUP-PUBLISH]] (NonEmpty needs private LF
  Semigroup), [[CAT-MIGRATE-EC-APPLICATIVE-PROVIDERS]] (Validation needs EC
  apply_to/compose/functor_map_of/Applicative, all private + EC roots-loads red),
  and the Language roots-loader faces-3 predecessor
  ([[LANG-ROOTS-LOADER-LOCAL-INSTANCE-DICT-SCOPE]] follow-up).
- **LF private `bool_and` consolidation** -> [[CAT-MIGRATE-LF-BOOL-AND-CONSOLIDATION]]
  (Core.Classes, sequenced AFTER Map's (a) lands) — NOT part of Map's Data
  increment; it only DROPS + repoints, mints nothing.

## Deliverables

- **D0 — census + intra-tier order at the build SHA (T1-adjacent judgment;
  Architect is the confirmer).** For each module above, measure its standalone
  `UnresolvedCon`/`UnboundName` set (the loader is the authority) and the exact
  provider it needs, and MEASURE the intra-Tier-C value-dependency edges among
  the module set (does any listed module consume another listed module beyond the
  named `SB -> StringKeys`?). Emit the increment order from that measured DAG.
  If D0 surfaces a provider NOT owned by an already-published Tier A/B module
  (a hidden dep on a still-scaffolded module), CITE it and split it out rather
  than dragging an unpublished provider in (the same DAG axis every tier
  respects). D0 also decides whether the tier stays one node with per-module
  increments or is split (see 'Open decomposition input').
- **D1..Dn — per-module publish + import + standalone, in the D0 order.** For
  each module: publish exactly the export surface its downstream consumers need
  (measured, not guessed); add a selective import from the published lower tiers
  for the exact D0-confirmed set; retire the ambient reach; extend the module's
  loader-visible inventory to reflect both the new exports and the imports
  (imported names are not new exports — reflect them the way an existing import
  is reflected); the module elaborates standalone (exit 0).
- **Map's increment (5) — the `bool_and` (a) relocation, RULED (Architect
  evt_4hp6qxkdaqgbz).** DROP Map's byte-identical `bool_and` dup, import LC's
  PUBLIC `bool_and`, and RELOCATE Map's six proofs
  (`comm`/`assoc`/`idempotent`/`left_identity`/`right_identity`/`true_intro`) to
  LC as a reviewed-verbatim-modulo-`pub` move — they fill genuine LC gaps and do
  NOT collide with LC's `intro`/`left`/`right`. This establishes LC's ownership
  of `assoc` + the identity laws. ALSO measure `Map.true_intro` vs `LC.intro`:
  same proposition -> drop/reuse under one name; else keep distinct. One
  proposition, one proof. BINDING CONSTRAINT: LC is the SOLE canonical `bool_and`
  owner; every proposition exists there EXACTLY ONCE under ONE name (the LF
  private dup consolidation that reconciles the divergent names lives in
  [[CAT-MIGRATE-LF-BOOL-AND-CONSOLIDATION]], sequenced after this).

## Acceptance criteria, each with its control (proven Tier-A / EC shape)

- **AC-EXPORTED (positive, per published symbol).** Each newly-published symbol
  is LOADER-VISIBLE from its owning module — a selective import resolves it to
  that module's `GlobalId`, measured by the loader, not a `^pub` grep. Control:
  the probe resolves; a still-private sibling name in the same module still
  rejects `UnboundName`.
- **AC-EXACT-INVENTORY (per module).** Each module's loader-visible inventory
  equality extends by EXACTLY the intended names (exports + imports; nothing
  else changes visibility); population from the module's own definitions,
  verdict from the loader, a per-symbol reddening mutation each reds distinctly.
- **AC-STANDALONE-GREEN (per module).** Each migrated module elaborates
  standalone (exit 0) after its import block — no ambient/scaffolding fallback.
  The control per module is that removing the new import line restores the exact
  prior standalone failure (the `UnresolvedCon`/`UnboundName` D0 recorded).
- **AC-VISIBILITY-ONLY (class-uniformity).** Any `pub` added to a class or a
  class-adjacent symbol changes visibility only: a differential shows the body
  BYTE-UNCHANGED; publishing mints no second class/instance; every consumer
  resolves to the single existing owner. No computational change. (Map's
  `bool_and` migration is the one exception that moves a body — it is an
  attached-owner RELOCATION to LC, reviewed as a verbatim move modulo `pub`, not
  a visibility flip.)
- **AC-NO-REGRESSION.** Re-run the COMPLETE affected-target closure (every target
  loading any changed module or a module whose closure this changes), scoped by
  changed PATHS. Targeted via `scripts/ken-cargo`, never `--workspace` (green in
  CI is the workspace verdict).

## Decomposition input — RESOLVED (Architect evt_2r4mr5bqg0mhr)

Both questions the Architect owns (the measured value-dependency DAG is its axis)
were routed on this draft (consult evt_2yz0f22qaq33e) and CONFIRMED. The
Architect does NOT hold the fine intra-Tier-C DAG — its decomposition
(evt_2e0pee5jxzv07) measured the coarse TIER partition, not the edges among these
nine — so both the order and the granularity EMIT from the D0 census, with the
Architect confirming the DAG is complete before either locks.

1. **Intra-tier ordering — delegation to D0 CONFIRMED as the correct discipline**
   (not a fallback). EC's D0 this session proved why: an a-priori set under-counts
   (EC's "exact-four" LF surface was measurably insufficient; its census found the
   real closure), so an a-priori intra-tier order would carry the same risk.
   Measure, don't guess. D0 acceptance, so it is a closure not a sample:
   - EXHAUSTIVE by construction — each of the nine modules' free cross-module
     symbol set (every referenced symbol MINUS every locally-declared one) is
     censused against the already-published Tier A/B providers AND against the
     other eight Tier-C modules (the same free-symbol-closure predicate the
     Architect ruled for EC, applied pairwise within the tier).
   - PROVEN closed by each module checking standalone-clean with exactly its
     measured import set — a missed intra-tier edge shows up as a standalone-check
     failure, so the check IS the completeness proof.
   - The one a-priori edge is `SB -> StringKeys` (umbrella); every other
     intra-tier edge is whatever the census finds. The Architect confirms the
     emitted DAG is complete before the increment order locks.
2. **WP granularity — DAG-grounded decision rule, emitted from the census (do NOT
   fix before the DAG exists).**
   - Census shows the nine are MOSTLY INDEPENDENT consumers of Tier A/B (only
     `SB -> StringKeys`, no other intra-tier edges) => ONE node, per-module
     increments (the CAT-NAT-REUSE-CONSUMERS precedent); each increment is an
     independent releasable candidate and the one-WP rule keeps ring coherence.
   - Census reveals a strongly-connected CLUSTER separable from independent
     singletons => split on THAT cut (a weakly-connected-component boundary in
     the measured DAG), never on the module count. "Nine is large" is not a seam;
     a set of modules with no cross-edges to the rest IS one.
   - Default: one node with per-module increments UNLESS the DAG exhibits such a
     cut. The Architect confirms the granularity call on the census output.

Both non-collapse confirmations from the Architect (folded above): Map's
increment carries the `bool_and`-family attached-owner migration verbatim-modulo-
pub (D0 pins the destination owner; the Architect verifies verbatim-ness at
candidate), and BK/StringKeys are the pure-consumer standalone-green closeout
their Tier B relocation set up (no new proof content). The Architect is the
required reviewer on each Tier C increment/candidate.

## Capability tier: T2 (with a T1-adjacent D0)

Per-module mechanical export publication + bounded import-block clean-ification
(the proven Tier-A / EC shape) — no class-instance relocation, no proof
authoring beyond Map's named `bool_and` attached-owner move. The one
judgment is D0 completeness per module (are all ambient deps owned by an
already-published Tier A/B module, or is there a hidden non-tier provider?)
plus the intra-tier ordering measurement, which either folds in or splits
out with a cited reason. The Architect is the required reviewer on surface
correctness (exactly the intended surfaces published, nothing over-published;
class-uniformity by construction) and on the D0 order.

## Gate, reviewer, sequencing

`gate: none` (no TCB touch; the operator ruled the class-owner model). On
each increment's candidate: **Architect** (required — surface correctness
+ class-uniformity + D0 order) + **Foundation QA + CV** on the exact SHA,
then Steward M1-M4 -> lieutenant. Critical-path successor to Tier B;
released when a foundation seat is free (EC —
[[CAT-MIGRATE-EC-FUNCTOR-IMPORT]], off critical path — may still be in
flight; Tier C is the priority and takes the seat when EC lands or the
leader parallelizes). Tier D ([[CAT-SCAFFOLD-RETIREMENT]]) is framed
after this tier lands, one release ahead.
