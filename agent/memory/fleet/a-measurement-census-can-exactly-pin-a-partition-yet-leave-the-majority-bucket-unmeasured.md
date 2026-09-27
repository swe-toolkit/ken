---
name: a-measurement-census-can-exactly-pin-a-partition-yet-leave-the-majority-bucket-unmeasured
description: "A census that pins an exact, disjoint, exhausting N-way partition looks like total coverage, but a bucket whose membership criterion is 'the measurement did not complete here' (residual, error, skipped, baseline-red) is a hole wearing a partition's clothes. Find that bucket, size it against the whole, and check whether any downstream AC is scoped to the measured buckets only: if so it can be satisfied exactly while its purpose fails. Measured on the LANG-MOD-STRICT-RESOLUTION D0 ambient census (c64c62190): 32 of 44 entries were residuals with no measured vector. The converse: an item leaving the unmeasured bucket surfaces a vector that reads as growth; locate its bucket transition before filing a regression (residual -> ambient is Err -> Ok, only clean -> ambient loses ground; 40e7f1199)."
metadata:
  type: feedback
---

# A measurement census can exactly pin a partition yet leave the majority bucket UNMEASURED

**Measured 2026-08-23 on `c64c62190`** (LANG-MOD-STRICT-RESOLUTION D0 probe,
`dec_w8v8p99bnhsn`). Change-triggered hunt; verdict clean. The node did the
right thing for D0. This is a lens for the next census or probe, handed to the
Steward as a triage flag for WP-4's framing.

## The shape

`catalog_ambient_passthrough_migration_census`
(`crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`) walks every
catalog leaf and sorts each into one of three buckets, each pinned to an exact
hardcoded set, asserted mutually disjoint, and asserted to exhaust the
discovered address set:

- `census` (10): loads under the full env, and under strict-floor stripping its
  ambient dependency vector is measured.
- `clean` (2): loads with an empty ambient vector.
- `residuals` (32): fails the single-root baseline load, or its first
  strict-floor error is not an `UnresolvedCon` of an available name.

The arithmetic is airtight: `10 + 2 + 32 = 44 = discovered`, disjoint, union
equals discovered. It reads like total coverage.

## Why the exact partition is not coverage

The residual bucket is 32 of 44 (73%), and its membership criterion is **"the
measurement procedure did not complete here."** A residual's
ambient-passthrough vector is never computed. On the axis the probe exists to
measure, the majority of the population is a hole.

Exhausting the population is NOT measuring every member on the axis. A bucket
defined by "measurement bailed early" is a hole wearing a partition's clothes.

## Where it bites downstream

The probe's own comment (line ~347) stated the migration target as "D1 must use
the same strict floor and make every dependency vector empty." That target
ranges only over the `census` rows, the only rows with a measured vector. So an
AC read as "all census vectors empty ⇒ catalog migration complete" is
satisfiable exactly while its purpose (every entry off ambient passthrough)
fails, because the residual entries were never measured or migrated. The AC
named the measured bucket when it meant the whole population.

The same masking shows up per consumer: a reuse migration's inherited ambient
debt is invisible for a residual consumer, so "its row did not change" is not
evidence it escaped the debt (see
[[classify-a-reuse-migrations-ambient-census-case-before-filing-a-missing-or-wrong-row]]).
The flag held up: the residual set was later migrated down to nothing, and at
`19c105b97` `expected_residuals` is empty (emptied by `1b7af23bd`).

## How to apply

When a census or probe pins an N-way partition:

1. Find the bucket whose membership criterion is a MEASUREMENT FAILURE, not a
   measured property: residual, error, skipped, unreachable, baseline-red.
2. Size it against the whole. A residual that is a rounding error is fine; one
   that is the majority means the census measured a minority precisely, and
   precision on a minority reads as rigor.
3. Trace every downstream AC or migration target the probe feeds. If any is
   scoped to the MEASURED buckets ("every vector empty", "all rows green"), it
   can be satisfied exactly while the unmeasured bucket is untouched. File that
   even when the probe is a correct measurement-only node: the defect is latent
   in how the sentinel will be consumed, and cheapest to fix at AC-framing time.
4. Distinguish honest from misleading. A probe that keeps residuals in a
   disjoint, printed, exact-pinned bucket (as here) is honest; the finding is a
   triage flag for the consuming WP. A probe that silently folded residuals into
   `clean` would be a defect.

Reported `evt_1e3tpt44qxjkm` (Steward side thread `thr_4g49g6pqvhq7x`). Related:
[[a-projection-partial-in-the-direction-of-its-own-blind-spot-cannot-report-its-miss]]
(a residual bucket reads as a classification result rather than a failure) and
[[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]].

## The converse: an item leaving the unmeasured bucket reads as growth

*Merged from the former fleet lesson
`in-a-multi-bucket-partition-census-a-growing-per-item-vector-is-not-a-regression-until-you-locate-the-items-bucket-transition`
(2026-09-27 scope pass).*

Because a residual row carries no measured vector, the moment it becomes
measurable its vector appears from nothing, and a vector-only reading files
that as a regression. **In a multi-bucket partition census, a growing
per-item vector is not a regression until you locate the item's bucket
transition.**

**Measured 2026-08-27 on the landed squash `40e7f1199`**
(LANG-MOD-CANONICAL-PAIR-PACKAGE, reviewed `db3b8af78`, decision
`dec_42y4955whef7g`; dispatch base `5b914b7bd` stale, real parent
`72a97e1dd`; lieutenant-dispatched M8 post-merge hunt; ken-elaborator
prelude/module bootstrap, own delta 8 files +684/-98, byte-identical to
reviewed `db3b8af78`). Verdict: **CLEAN verified, no finding.** Two lenses;
the first is a near-miss. Reported `evt_66bqnbkp0s04d` (lieutenant M8 thread
`thr_3ktdqsw5rvhqg`).

### What landed

A "floor realization": `Pair` joins the closed prelude type floor
(`PRELUDE_FLOOR_NAMES` grows 9->10, `modules.rs`), making it
**unshadowable**, and three companion bindings `mk_pair`/`pair_fst`/`pair_snd`
(`PRELUDE_COMPANION_BINDING_NAMES`) are admitted as **checked-transparent
strict-builtins keyed to the exact `Pair` identity**.
`capture_strict_builtin_names` went infallible->fallible (`filter_map`
silent-skip -> `map`+`ok_or_else` hard-error), and `lib.rs:249` now
propagates with `?`.

### Lens 1 (the near-miss): find the bucket transition, not the vector delta

The same census, `catalog_ambient_passthrough_migration_census`
(`lang_mod_strict_resolution_d0.rs`), partitions every catalog leaf into
**three buckets**: `ambient` (baseline loads + has a non-empty residual
dependency vector), `clean` (baseline loads + empty residual), `residual`
(baseline elaborate FAILED). It asserts each bucket by **exact set equality**
and that `ambient U clean U residual == discovered` (exhaustion).

`Data.Collections.Deque` gained `["Equal"]` and `Data.Collections.Derived`
gained an 11-name vector. That looks like the **wrong direction**: adding
`Pair` to the floor makes `strict_floor_env` retain `Pair`+companions, which
can only **remove** names from a given package's residual vector, never add
them. Reading the vector delta alone, you would file a phantom regression.

**The resolution is the bucket transition, not the vector.** Both packages
**moved**: the old file had them in `expected_residuals` (baseline FAILED) at
lines 664-665; the new file has them in the `ambient` census map at 560/564.
`residual -> ambient` is **`Err -> Ok`** on baseline load: the floor `Pair`
now supplies what the (removed-over-the-9-commit-range) compatibility `Pair`
used to, so baseline elaboration that previously failed now succeeds and
reaches far enough to surface a residual. A widening of what compiles,
**not** a regression. A non-empty ambient vector on a newly-arrived package is
the *expected* shape of a `residual -> ambient` move.

**The discipline:** in a multi-bucket partition census, a per-item vector
growing is meaningless until you answer *which bucket did this item leave*.
The only regression direction here is `clean -> ambient` (a floor-buildable
package newly needing an ambient dep), and `expected_clean` was confirmed
byte-identical parent->HEAD, so nothing took it. Sibling of
[[a-nodes-status-is-a-claim-about-a-node-not-evidence-about-the-tree]] (a
row's meaning is its position, not its text) and of "a partition that keeps
failing across reachability refinements may be keyed on the wrong axis" (an
earlier lesson, since retired).

### Lens 2: the new Err arms were pre-source bootstrap self-checks

`capture_strict_builtin_names` went infallible -> fallible, but it is called
from exactly one site (`lib.rs:249`, `ElabEnv::empty()`) over fixed bootstrap
`globals` before any user source loads, and the ten floor names are pinned
always-present by `prelude_signature_inventory_is_executable_and_closed`. So
each new `Err` arm either always passes or reds every elaborator test; no user
program can drive it, and `filter_map` -> `map` + `ok_or_else` turns a silent
drop into a loud error. This is the "pre-source venue" class in
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]]:
locate a new production `Err` arm's call site relative to user input before
ranking it.

### Why this is not the inert-plane / name-only shape

Unlike the recent checked-IH planner hunts, this WP is **not output-inert**
and **does** carry a real definitional behavioral oracle.
`pair_floor_beta_eta_are_definitional` proves `pair_fst (mk_pair a b) = a`
and `pair_snd = b` **by conversion** (`= Proved`), `eta` by `Refl`, each with
negative `bad` controls (swapped terminal `Refl`/`Proved`, swapped
`True`/`False`) that **must reject**. A projection body-swap would be caught.
So
[[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]]'s
name-only degradation does not apply. And `pair_providers.is_empty()`
(`cat_ord_nat_canonical_owner.rs`) grounds that no catalog `Pair` provider
competes, so unshadowability is collision-free.

### How to apply (to a landed floor realization)

For a landed **floor-realization** (a type made unshadowable + companion
bindings admitted as strict-builtins):

1. If a partition census test moves, read the **bucket transition** of every
   changed item, not its vector delta: `residual -> ambient` is `Err -> Ok`
   (improvement) even though the ambient vector is non-empty; only
   `clean -> ambient` is a regression, so check the `clean` bucket is intact.
2. For each new production `Err` arm, find its **call site relative to user
   input**: an arm in pre-source construction (single call site, fixed
   bootstrap `globals`) is a self-check that CI catches wholesale, not a
   user-drivable `Ok->Err`. Rank it accordingly.
3. Confirm the realization is collision-free (no competing provider for the
   now-unshadowable name) and that it carries a real **definitional** oracle
   (conversion theorems with negative controls), not name-resolution only. If
   it does, the inert-plane and name-only concerns do not apply.

Do not file a regression off a growing census vector alone.
