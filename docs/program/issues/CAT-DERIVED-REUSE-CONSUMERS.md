---
id: CAT-DERIVED-REUSE-CONSUMERS
title: "Drain catalog-reuse census group 4 (derived-list computational reuse) — replace six reimplementations of list_append, reverse, concat_map, and length across five packages with selective imports from Data.Collections.Derived. The consumer half of CAT-DERIVED-PUB-EXPORT, shaped on the landed CAT-NAT-REUSE-CONSUMERS per-package increment pattern."
status: active
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-DERIVED-PUB-EXPORT]
blocks: []
github: null
origin: "Steward, 2026-08-29, filed to remove lane-3 framing debt: CAT-DERIVED-PUB-EXPORT had no successor, so lane 3 would have gone idle on its landing. Group 4 membership is quoted verbatim from docs/program/cat-reuse-census.md section 4.4 at origin/main ac9b681e1f5a684b40a2da8b9ac0c0d19a13b2fc. The four providers group 4 consumes are exactly four of the six names CAT-DERIVED-PUB-EXPORT exports, so the prerequisite covers the consumers with nothing left over. Steward-filed per COORDINATION section 2."
---

> # RELEASED by the Steward, 2026-08-29 03:47 UTC. `active`.
>
> The prerequisite is confirmed `merged` **by blob identity, not ancestry** —
> the publisher squashes. At `origin/main`
> `04e157a450a0d17f9fab5437c8f1f60c523ff052` (PR #3079, merged 03:38:54 UTC):
> `catalog/packages/Data/Collections/Derived.ken.md` is blob
> `fcaa8e4b60fc97a394b635f2d4bf3ea600ba61aa` and
> `crates/ken-elaborator/tests/cat_derived_pub_export.rs` is blob
> `624a33aa295b28c1e320a3771650679317cb0cd5`, both identical to the gated
> candidate.
>
> **The fixed inputs below were re-measured at that SHA before release**, as
> §"Fixed inputs" itself requires. All six group-4 consumer sites are still
> present, and all four providers this node needs — `list_append`, `length`,
> `reverse`, `concat_map` — are now `pub` in `Derived.ken.md` at lines 75, 163,
> 238 and 342. Nothing in the table decayed.

## Fixed inputs, measured at `origin/main` `ac9b681e1f5a684b40a2da8b9ac0c0d19a13b2fc`

Census group 4, **"Derived-list computational reuse"**, quoted verbatim from
`docs/program/cat-reuse-census.md` §4.4 — **six consumer sites across five
packages**:

| consumer site | reimplements | package |
|---|---|---|
| `Data/Collections/Deque.ken.md#deque_list_append` | `list_append` | Deque |
| `Data/Collections/Deque.ken.md#deque_list_reverse` | `reverse` | Deque |
| `Capability/Parsing/Parsing.ken.md#list_append` | `list_append` | Parsing |
| `Core/Classes/EffectfulClasses.ken.md#concat_map` | `concat_map` | EffectfulClasses |
| `Capability/Parsing/Cursor.ken.md#cursor_list_length` | `length` | Cursor |
| `Tooling/Testing/Property.ken.md#property_list_length` | `length` | Property |

**The provider set covers this exactly.** Group 4 consumes four names —
`list_append`, `reverse`, `concat_map`, `length` — and all four are in the six
`CAT-DERIVED-PUB-EXPORT` marks `pub`. Nothing in group 4 needs a provider that
node does not supply, and `eq_from_ord` and `count` are exported for other
reasons and are not consumed here. **Re-measure this before acting** — §4.4 says
its membership is mechanically recountable from §3, and a census row describes
when it was written, not what the tree currently contains.

## Deliverables — ONE PER PACKAGE, five increments

Shaped on the landed `CAT-NAT-REUSE-CONSUMERS`, which drained groups 2 and 3 as
six per-package increments. §4.4 explicitly permits this: *"The Steward may
split a group per package to preserve the charter's single-package
preference."*

- **D1 — Deque** (`deque_list_append`, `deque_list_reverse`). Two sites, one
  package; the only increment with more than one site.
- **D2 — Parsing** (`list_append`).
- **D3 — EffectfulClasses** (`concat_map`).
- **D4 — Cursor** (`cursor_list_length`).
- **D5 — Property** (`property_list_length`).

Order is the ring's call. **Each increment is independently landable** and
shares no file with the others.

## Carried authorizations — do NOT hard-stop for these again

> **A consuming TEST FIXTURE's root set is part of an increment's path set.**
> Established on `CAT-NAT-REUSE-CONSUMERS` D1 (cc6a/cc7/cc8) and D2
> (cc2/cc3/cc4/ds9/d0), then ruled again for D4 at `evt_1b31assx1ktg8`,
> `evt_6snwh0xy60jh8`, and `evt_2r8cavz7b1bms`. **It is carried into this node
> at filing time so the ring does not stop for it a fourth time.** An increment
> that must touch a fixture's roots to keep the fixture honest is in scope.

## Acceptance criteria

- **`AC-REUSE-NOT-REIMPLEMENT`.** Each site IMPORTS the `Derived` operation and
  the local reimplementation is DELETED. A site that keeps its local definition
  and adds an import has not drained the census row — the census counts
  reimplementations, and one that still exists still counts.
- **`AC-SELECTIVE-IMPORT`.** The import is selective and names the operation.
  Control: the probe must FAIL for a name deliberately not imported, or it is
  measuring module presence rather than the binding.
- **`AC-NO-COMPUTATIONAL-CHANGE`.** Reuse is a provenance change, not a
  semantic one: the operation computed at each site is unchanged. Control: a
  differential showing the consumer's observable behaviour identical before and
  after.
- **`AC-CENSUS-ROW-DRAINED`.** State which §4.4 group-4 rows this increment
  closes and re-measure the remainder. **Report the count you measure, not the
  count this frame predicts** — six is a fixed input measured at a SHA, and the
  census is recountable from §3.
- **`AC-BOTH-CENSUSES`. "Census" names TWO different artifacts in this node, and
  satisfying one says nothing about the other.** Every increment must re-measure
  **both**:
  1. the **reuse census**, `docs/program/cat-reuse-census.md` §4.4 — which
     group-4 rows drained (that is `AC-CENSUS-ROW-DRAINED` above); and
  2. the **strict-resolution ambient census**,
     `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`.

  **The strict-resolution census is a THREE-way classification gated on baseline
  load, not a two-way clean/census split. Get this right before acting on it.**
  At `:380-390` each leaf first attempts a baseline elaboration. **If the
  baseline FAILS the leaf is pushed to `residuals` and `ambient_dependencies` is
  NEVER CALLED for it.** Only a baseline-green leaf is measured for ambient debt
  and then sorted into `expected_clean` or a measured census row.

  ⇒ **Importing `Derived` can only move a leaf that is baseline-GREEN.** For a
  leaf already in `expected_residuals` the import changes nothing measurable,
  because the census never reaches the ambient step for it.

  Worked example of each side, both verified against the landed tree:
  - `Data.Collections.Deque` is baseline-green, so D1's import DID expand its
    measured row (`4f6d340c6`, adding `eqChar`, `is_sorted`, `leqChar`).
  - `Capability.Parsing.Parsing` is in **`expected_residuals`** — the list opens
    at `:683` and Parsing sits at `:695`; `expected_clean` opens at `:674` and
    closes at `:682`. Its baseline stops at `UnresolvedCon SourceId`, a
    pre-existing failure unrelated to `Derived`. **D2 therefore correctly leaves
    this file untouched.**

  > **ERRATUM — this AC's first version asserted the opposite for Parsing, and it
  > was wrong.** It claimed Parsing was `expected_clean` and would move to a
  > measured row, making D2 deterministically CI-red. That came from the M8 hunt,
  > which read line 695 and attributed it to the `expected_clean` bucket; the
  > bucket boundary is at 682. **The Steward propagated it into this frame
  > without independently resolving which binding line 695 belongs to, then
  > voided a valid merge authorization on it.** Caught by the foundation
  > implementer's hard stop, which reproduced the exact commanded test GREEN 1/1
  > on the five staged blobs.
  >
  > **The reusable form: a line number is not a bucket.** Two readers cited
  > `:695` and drew opposite conclusions; the discriminator was the enclosing
  > `let`, which neither had opened. When a finding turns on set membership,
  > resolve the ENCLOSING BINDING, never the line.
  >
  > **The AC itself survives — "re-measure both censuses" is correct and still
  > binds.** What was wrong is the predicted DIRECTION for one package. Do not
  > read this erratum as retiring the criterion.

  > **This is not a new criterion; it is a disambiguation, and the record shows
  > exactly why it was needed.** `AC-CLOSURE-TARGETS` already covered this file
  > as an affected target, and **D1 applied it correctly** — landing at
  > `4f6d340c6`, D1 edited this exact file to expand Deque's row with exactly
  > `eqChar`, `is_sorted`, `leqChar`. **D2 then made the structurally identical
  > change to `Parsing` and did not touch it, and both gates approved.** The
  > Architect's approval explicitly checked census evidence — the *reuse* census
  > — and read as census-complete. **One word naming two artifacts is what let a
  > satisfied criterion and an unsatisfied one look like the same check.** Found
  > by the mandatory M8 hunt, not by the gates.
  >
  > **D3, D4 and D5 each make this same `Derived`-import change**, so the trap is
  > ahead three more times; that is why this is written into the frame rather
  > than left in a rejection message. **A requirement that lives only in gate
  > rejection prose is not a criterion.**
- **`AC-CLOSURE-TARGETS`.** Re-run the COMPLETE AFFECTED-TARGET CLOSURE, not the
  diff-touched set: every target that loads any module whose closure the
  increment changes, whether or not the increment touches its file. **This is
  here because a diff-touched set is blind to exactly the consumers an increment
  breaks by changing a closure rather than a file** — measured twice now, on the
  `CAT-NAT-REUSE-CONSUMERS` D6 respin (28 diff-touched targets passed 170/170
  while CV found a candidate-caused red in an untouched file) and again in the
  runtime lane. Not a relaxation of the targeted-build rule: what changes is
  which targets count as affected, not how many crates build at once.

## Capability

**T2.** Behaviour-preserving reuse with a landed precedent in
`CAT-NAT-REUSE-CONSUMERS` and a provider prerequisite that supplies every name.
The judgment is in the closure and the census re-measurement, not in the edits.

## Sequencing

Blocked on `CAT-DERIVED-PUB-EXPORT` merging — the imports cannot resolve until
the provider names are `pub`. **Verify that node reached `merged` by blob
identity before releasing this one**; `ken-ci` auto-close has been unreliable,
so a landed node can still read `active` with `github: null`.
