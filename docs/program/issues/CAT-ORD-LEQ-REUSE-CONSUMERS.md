---
id: CAT-ORD-LEQ-REUSE-CONSUMERS
title: "Drain the group-5 ordered-list reimplementations (OrderedSearch, InsertionSort) to their canonical providers LC.ord_leq_at, D.eq_from_ord, D.count via selective import."
status: active
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-ORD-LEQ-PUB-EXPORT]
blocks: []
github: https://github.com/swe-toolkit/ken/pull/3195
origin: "Steward group-5 census drain, cat-reuse-census.md §4.4 item 5, sites measured 2026-08-31 at origin/main 5083f2e46. Gated on CAT-ORD-LEQ-PUB-EXPORT (exports LC.ord_leq_at); D.eq_from_ord and D.count are already public via CAT-DERIVED-PUB-EXPORT."
---

> # D1 (OrderedSearch) LANDED 2026-08-31 as ACCEPTED PARTIAL — node `active`, NOT
> # `merged`. D2 (InsertionSort) is RELEASED and Working under Architect ruling
> # `evt_5921k9xswdazg` (2026-08-31): the caution-2 first-import HARD STOP was
> # adjudicated — proceed with the EXACT three operation imports as dispatched;
> # the base `Ord` -> candidate `bool_or` raw-boundary movement is the expected
> # class-namespace/strict-resolution boundary (loading LawfulClasses installs
> # its public `Ord` class regardless of the selective item list; `bool_or` is an
> # ordinary qualified provider in neither list), NOT a blocker and NOT a
> # different component design. Record AC-RAW-BOUNDARY as the two negative
> # coordinates (base exit-1 `UnresolvedCon Ord`, candidate exit-1 `UnresolvedCon
> # bool_or`), never a false standalone-success; the fixture-backed 3/3 positive
> # remains the real acceptance path. If the raw check advances BEYOND `bool_or`
> # or succeeds without a separately authorized dependency migration, hard-stop
> # again. D2 landing closes this node `merged`.
> #
> # D1 landed at origin/main `65bfa52db` (respin candidate 22892065c, first
> # candidate 0f9b9746 CV-rejected on a mixed-provider evasion, recut with a
> # universal retained-call population identity control). Steward-verified from the
> # objects: OrderedSearch blob 4fcf97955 -> 34696428e (delete local
> # ordered_search_leq, widen import to (Ord, ord_leq_at), repoint all 16 refs);
> # acceptance test blob a39f8ddf -> 5a1def9e (universal population elem=2 /
> # sorted_for_search=1 / search=22, each 4-arg order-call GlobalId == canonical LC
> # ord_leq_at); InsertionSort base-identical 02190bc4 (D2 absent). Gates QA
> # evt_z6vsr33vsbrq + CV evt_2r110khrpfq0s, Decision dec_5fs94zk27awax, routed
> # evt_3awhm814yfcz1. OrderedSearch source-attestation refresh routed to Librarian.
> # D2 release: re-measure InsertionSort at the landed SHA; element_count is
> # RECURSIVE (distinct-head migration, NOT kernel-equivalence) and InsertionSort
> # carries zero imports today (first-import standalone question) — a [higher]
> # surprise is a HARD STOP to the Architect.
>
> # The group-5 consumer drain — SELECTIVE IMPORT, not un-shadow.
>
> The providers are real modules (LawfulClasses, Derived), NOT the ambient
> prelude, so this is a selective-import drain (delete the local reimplementation,
> import the canonical symbol, repoint references) — the CAT-BOOL /
> CAT-DERIVED-REUSE-CONSUMERS shape, not the CAT-PRELUDE un-shadow shape.
> Gated on [[CAT-ORD-LEQ-PUB-EXPORT]] for the two `ord_leq_at` sites; the two
> `D.*` sites need no prerequisite.

## Fixed inputs (measured at origin/main `5083f2e46`; RE-MEASURED at `551de8084` on the pub-export landing — all four sites byte-identical, InsertionSort still has zero imports; re-measure again at your build SHA)

| site | file:line | current local body | canonical target |
|---|---|---|---|
| `ordered_search_leq` | `Algorithm/Searching/OrderedSearch.ken.md:22` | `= d.leq x y` | `LC.ord_leq_at` |
| `ordered_leq` | `Algorithm/Sorting/InsertionSort.ken.md:14` | `= d.leq x y` | `LC.ord_leq_at` |
| `order_eq` | `Algorithm/Sorting/InsertionSort.ken.md:16-17` | `= bool_and (ordered_leq ...) (ordered_leq ...)` | `D.eq_from_ord` |
| `element_count` | `Algorithm/Sorting/InsertionSort.ken.md:19-26` | recursive `match` using `order_eq` | `D.count` |

## Two cautions the drain turns on — read before framing a candidate

1. **`element_count` is RECURSIVE, so it is a distinct rigid head from `D.count`
   and NOT convertible by unfolding** — the exact refutation the CAT-PRELUDE
   recut established (Architect `evt_7spzy25qqdsqx`: separately declared
   recursive globals are non-convertible, `false` + halts). Its migration
   evidence is candidate-specific (inventory / resolution-flip / inverse-patch),
   the `AC-RECURSIVE-UNSHADOW-MIGRATION` shape — NEVER a kernel-equivalence
   claim. By contrast `ordered_search_leq`, `ordered_leq` (and `order_eq`) are
   NON-recursive transparent wrappers; they unfold definitionally to the same
   body and are convertible, so their downstream theorems survive the repoint
   without a migration proof. Do not conflate the two classes.

2. **`InsertionSort.ken.md` has ZERO `import` declarations today** — it is
   authored against ambient/prelude scaffolding (uses unqualified `bool_and`,
   `Ord`, `List`, `Nat`). Adding the first selective import to a file with none
   changes its scope resolution; re-measure standalone status and decide what
   "standalone" even means for a fixture-backed file here, echoing the
   ambient-scaffolding caution CAT-DERIVED-PUB-EXPORT raised. `OrderedSearch`
   already imports `Core.Classes.LawfulClasses (Ord)` at :20, so its import edge
   only widens.

## Deliverables (one increment per consumer module, released one at a time)

- **D1 — OrderedSearch.** Delete `fn ordered_search_leq`; import
  `Core.Classes.LawfulClasses (ord_leq_at)`; repoint its callers. Non-recursive
  wrapper, convertible; no migration proof.
- **D2 — InsertionSort.** Delete `ordered_leq` / `order_eq` / `element_count`;
  import `LC.ord_leq_at`, `D.eq_from_ord`, `D.count`; repoint. `element_count`
  carries the recursive-head migration evidence (caution 1); the file's
  first-import standalone question is caution 2.

## Acceptance criteria, each with its control

- **AC-CENSUS-ROW-DRAINED.** Each local reimplementation is gone and every
  reference resolves through a selective import to the provider `GlobalId`.
  Control: a resolution-flip probe shows the name binds to the provider, not a
  local; the deleted local no longer resolves.
- **AC-RECURSIVE-MIGRATION (D2 `element_count` only).** Candidate-specific
  inventory + resolution-flip + inverse-patch evidence per the CAT-PRELUDE recut;
  NOT a kernel-equivalence assertion.
- **AC-DOWNSTREAM-GREEN.** Every downstream theorem that referenced a drained
  name still elaborates (the convertible wrappers unfold; the recursive migration
  is discharged by AC-RECURSIVE-MIGRATION). Control: elaborate the consumer
  modules and their dependents.
- **AC-RAW-BOUNDARY / STANDALONE.** Re-measure the consumer module standalone
  behaviour after the import edge changes; for InsertionSort this is caution 2,
  and a `[higher]` surprise is a HARD STOP to the Architect, not a workaround.
- **AC-CLOSURE-TARGETS / AC-NO-REGRESSION.** Complete affected-target closure,
  scoped by changed PATHS; `scripts/ken-cargo` only, never `--workspace`;
  whole-suite green is CI's.

## Gate and sequencing

`active` — D1 (OrderedSearch) MERGED at `65bfa52db` (PR #3195); D2
(InsertionSort) remains held. [[CAT-ORD-LEQ-PUB-EXPORT]] was MERGED at
`551de8084` (its `ord_leq_at`
provider is now loader-visible), so the premise is true and the Steward flipped
this `draft` -> `ready` and re-measured the four sites and both cautions at that
SHA before releasing to foundation (all four byte-identical; InsertionSort still
carries zero imports). On each
candidate: fresh Foundation QA + CV on the exact SHA (Architect only if the
recursive-head migration or a standalone surprise opens a design fork), then
Steward M1-M4. Groups 7 and the deferred law-carrying group-1 items remain
unmeasured and are NOT part of this drain.
