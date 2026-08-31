---
id: CAT-ORD-LEQ-PUB-EXPORT
title: "Export ord_leq_at as pub from Core.Classes.LawfulClasses so the group-5 ordered-list consumers can import the canonical dictionary-projection wrapper instead of reimplementing it."
status: merged
owner: foundation
size: S
gate: none
tier: T2
depends_on: [CAT-BOOL-PUB-EXPORT]
blocks: [CAT-ORD-LEQ-REUSE-CONSUMERS]
github: null
origin: "Steward group-5 census drain, measured 2026-08-31 at origin/main 5083f2e46 (cat-reuse-census.md §4.4 item 5). ord_leq_at is private at LawfulClasses.ken.md:977 and is the ONLY group-5 provider symbol not yet public — D.eq_from_ord/D.count already landed via CAT-DERIVED-PUB-EXPORT. Same shape as CAT-BOOL-PUB-EXPORT (which exported bool_and/bool_leq from this same module) and CAT-DERIVED-PUB-EXPORT."
---

> # The single-symbol pub-export prerequisite for the group-5 drain.
>
> This is the exact shape landed twice already (CAT-BOOL-PUB-EXPORT,
> CAT-DERIVED-PUB-EXPORT): mark one canonical provider symbol `pub`, and prove
> the export is real by extending the module's loader-visible inventory equality
> control, not a `^pub` text grep. It unblocks [[CAT-ORD-LEQ-REUSE-CONSUMERS]].

## Fixed inputs (measured at origin/main `5083f2e46`)

- **The symbol.** `catalog/packages/Core/Classes/LawfulClasses.ken.md:977`
  `fn ord_leq_at (a : Type) (d : Ord a) (x : a) (y : a) : Bool = d.leq x y`
  — currently PRIVATE (bare `fn`, no `pub`). A transparent, unconditional
  dictionary-projection wrapper.
- **It carries an attached proof.** `proof true_of_equal for ord_leq_at` at
  `LawfulClasses.ken.md:979-982` (bare `proof`, private). See the ATTACHED-LAW
  HAZARD note below — this is the most likely hard-stop trigger and must be
  checked first.
- **Current LawfulClasses public surface = 10 loader-visible names**, asserted
  by the existing equality control in
  `crates/ken-elaborator/tests/cat_bool_pub_export.rs::boolean_provider_loader_visible_inventories_are_exact`
  (:294-310): `IsTrue, Ord, bool_and, bool_leq, bool_or, bool_or::eq_true_of_or,
  leq_nat, leq_nat::antisym, leq_nat::refl, leq_nat::trans`. Exporting
  `ord_leq_at` (the function only) extends this to **exactly 11** — adds
  `ord_leq_at`, nothing else.
- **Providers-only.** The four group-5 consumer sites
  (`OrderedSearch#ordered_search_leq`, `InsertionSort#ordered_leq`,
  `InsertionSort#order_eq`, `InsertionSort#element_count`) are NOT touched by
  this node — they are the successor [[CAT-ORD-LEQ-REUSE-CONSUMERS]].

## Scope decision: export the FUNCTION only, keep the attached proof private

The consumers need the computational wrapper `ord_leq_at` to import in place of
their local `= d.leq x y` reimplementations. They do NOT need the attached law
`true_of_equal` (it serves LawfulClasses' own internal theorems, which stay in
this module). **Mark `ord_leq_at` `pub`; leave `proof true_of_equal` private.**
If the consumer drain later proves it needs the law, that is a measured
follow-up export, not a speculative one here. Keeping the export minimal also
keeps the inventory equality at a clean +1.

## ATTACHED-LAW HAZARD — check this first, it is the hard stop

`ord_leq_at` has an attached proof. CAT-DERIVED-PUB-EXPORT excluded three names
(`Perm`, `insert`, `sort`) precisely because their attached-law ownership was
`[higher]`-risk. Here the module is not moving and the proof stays private, so
the risk is expected to be inert — but do not assume it. If pub-marking
`ord_leq_at` triggers a `[higher]` nonlocal-position hazard from its attached
law (an eligibility rejection, an ownership/orphan error, or a standalone
elaboration failure that was not present on the untouched base), that is a
**HARD STOP to the Architect** — do not work around it, do not weaken a control.
Measure the base standalone-green first, then measure it again after the single
`pub` keyword.

## Deliverable

- **D1 — mark `ord_leq_at` `pub` in LawfulClasses and extend the loader-visible
  inventory.** One `pub` keyword at `LawfulClasses.ken.md:977`; update the
  LawfulClasses equality assertion in `cat_bool_pub_export.rs` to the 11-name
  set; add the required reddening mutation and re-run the affected closure.
  Re-measure the exact line (a line number decays) at the SHA you build on.

## Acceptance criteria, each with its control

- **AC-EXPORTED.** `ord_leq_at` is LOADER-VISIBLE from
  `Core.Classes.LawfulClasses`, measured by the loader — a selective import
  `import Core.Classes.LawfulClasses (ord_leq_at)` resolves it to the
  transparent provider `GlobalId`, not by a `^pub` grep. Control: the
  selective-import probe resolves; a probe for a still-private name (the
  attached proof `true_of_equal`, or `Eq`/`DecEq`) still rejects as
  `UnboundName`.
- **AC-EXACT-INVENTORY.** The LawfulClasses loader-visible inventory equality
  (`cat_bool_pub_export.rs::boolean_provider_loader_visible_inventories_are_exact`)
  is updated to equal the 11-name set (the 10 above plus `ord_leq_at`).
  Population from the module's own definitions, verdict from the loader,
  **EQUALITY** — exactly `ord_leq_at` flips private to public and no other
  name's visibility changes — never a per-name privacy spot-check, never a
  hand-edited roster.
- **AC-EVASION-REDDENS.** Publishing an excluded name in any compile-preserving
  spelling — including one leading space before `pub`, which the roots loader
  accepts and publishes at the exact provider `GlobalId` — must RED this
  control. Byte-restore afterwards and show the restoration.
- **AC-STANDALONE-GREEN.** LawfulClasses still elaborates standalone (exit 0)
  after the pub-marking: no `UnresolvedCon`, no eligibility rejection, no
  attached-law nonlocal-position hazard. A `[higher]` hazard is the HARD STOP
  above, not a workaround.
- **AC-NO-COMPUTATIONAL-CHANGE.** `pub` is export-visibility only; a differential
  over function bodies shows byte-unchanged. Only LawfulClasses changes; the
  consumer sites are untouched (AC-PROVIDERS-ONLY).
- **AC-CLOSURE-TARGETS / AC-NO-REGRESSION.** Re-run the COMPLETE affected-target
  closure (every target that loads LawfulClasses or a module whose closure this
  changes), scoped by which PATHS changed. Targeted via `scripts/ken-cargo`,
  never `--workspace`; whole-suite green is CI's job.

## Gate and sequencing

On the candidate: fresh Foundation QA + CV on the exact SHA, then Steward
M1-M4. The Architect enters only if the attached-law hazard fires (a design
fork), per AC-STANDALONE-GREEN. Size is one symbol plus a test extension — a
sub-hour turn or a genuine hard stop.

Successor [[CAT-ORD-LEQ-REUSE-CONSUMERS]] is framed (`draft` with a complete
frame; it flips to `ready` when this node merges). Do not fold the consumer
drain into this node — this is the providers-only prerequisite.
