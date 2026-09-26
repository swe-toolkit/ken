---
name: a-measurement-census-can-exactly-pin-a-partition-yet-leave-the-majority-bucket-unmeasured
description: "A census that pins an exact, disjoint, exhausting N-way partition looks like total coverage, but a bucket whose membership criterion is 'the measurement did not complete here' (residual, error, skipped, baseline-red) is a hole wearing a partition's clothes. Find that bucket, size it against the whole, and check whether any downstream AC is scoped to the measured buckets only: if so it can be satisfied exactly while its purpose fails. Measured on the LANG-MOD-STRICT-RESOLUTION D0 ambient census (c64c62190): 32 of 44 entries were residuals with no measured vector."
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
