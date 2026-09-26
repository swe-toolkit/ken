---
name: in-a-multi-bucket-partition-census-a-growing-per-item-vector-is-not-a-regression-until-you-locate-the-items-bucket-transition
description: >-
  In a census that partitions items into several buckets with exact-equality
  and exhaustion assertions, a per-item dependency vector growing is not a
  regression signal until you locate the item's bucket transition: a package
  moving residual -> ambient is Err -> Ok (a widening of what compiles) even
  though its vector is now non-empty. Only the bucket moves that lose ground
  (here clean -> ambient) are regressions. Measured on LANG-MOD-CANONICAL-PAIR-
  PACKAGE (40e7f1199, reviewed db3b8af78, decision dec_42y4955whef7g; dispatch
  base 5b914b7bd stale, real parent 72a97e1dd). Also: a floor realization's new
  Err arms were pre-source bootstrap self-checks, and the WP carried a real
  definitional oracle. CLEAN; evt_66bqnbkp0s04d (thread thr_3ktdqsw5rvhqg).
metadata:
  type: feedback
---

# In a multi-bucket partition census, a growing per-item vector is not a regression until you locate the item's bucket transition

**Measured 2026-08-27 on the landed squash `40e7f1199`**
(LANG-MOD-CANONICAL-PAIR-PACKAGE, lieutenant-dispatched M8 post-merge hunt;
ken-elaborator prelude/module bootstrap, own delta 8 files +684/-98,
byte-identical to reviewed `db3b8af78`). Verdict: **CLEAN verified, no finding.**
Two lenses; the first is a near-miss.

## What landed

A "floor realization": `Pair` joins the closed prelude type floor
(`PRELUDE_FLOOR_NAMES` grows 9->10, `modules.rs`), making it **unshadowable**,
and three companion bindings `mk_pair`/`pair_fst`/`pair_snd`
(`PRELUDE_COMPANION_BINDING_NAMES`) are admitted as **checked-transparent
strict-builtins keyed to the exact `Pair` identity**. `capture_strict_builtin_names`
went infallible->fallible (`filter_map` silent-skip -> `map`+`ok_or_else`
hard-error), and `lib.rs:249` now propagates with `?`.

## Lens 1 (the near-miss): a growing residual vector is not a regression until you find the bucket transition

The census `catalog_ambient_passthrough_migration_census`
(`lang_mod_strict_resolution_d0.rs`) partitions every catalog leaf into **three
buckets**: `ambient` (baseline loads + has a non-empty residual dependency
vector), `clean` (baseline loads + empty residual), `residual` (baseline
elaborate FAILED). It asserts each bucket by **exact set equality** and that
`ambient U clean U residual == discovered` (exhaustion).

`Data.Collections.Deque` gained `["Equal"]` and `Data.Collections.Derived`
gained an 11-name vector. That looks like the **wrong direction**: adding `Pair`
to the floor makes `strict_floor_env` retain `Pair`+companions, which can only
**remove** names from a given package's residual vector, never add them. Reading
the vector delta alone, you would file a phantom regression.

**The resolution is the bucket transition, not the vector.** Both packages
**moved**: the old file had them in `expected_residuals` (baseline FAILED) at
lines 664-665; the new file has them in the `ambient` census map at 560/564.
`residual -> ambient` is **`Err -> Ok`** on baseline load -- the floor `Pair`
now supplies what the (removed-over-the-9-commit-range) compatibility `Pair`
used to, so baseline elaboration that previously failed now succeeds and reaches
far enough to surface a residual. A widening of what compiles, **not** a
regression. A non-empty ambient vector on a newly-arrived package is the
*expected* shape of a `residual -> ambient` move.

**The discipline:** in a multi-bucket partition census, a per-item vector
growing is meaningless until you answer *which bucket did this item leave*. The
only regression direction here is `clean -> ambient` (a floor-buildable package
newly needing an ambient dep) -- and I confirmed `expected_clean` is
byte-identical parent->HEAD, so nothing took it. Sibling of
[[a-nodes-status-is-a-claim-about-a-node-not-evidence-about-the-tree]] (a row's
meaning is its position, not its text) and of "a partition that keeps failing
across reachability refinements may be keyed on the wrong axis" (an earlier
lesson, since retired).

## Lens 2: the new Err arms were pre-source bootstrap self-checks

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

## Why this is not the inert-plane / name-only shape

Unlike the recent checked-IH planner hunts, this WP is **not output-inert** and
**does** carry a real definitional behavioral oracle.
`pair_floor_beta_eta_are_definitional` proves `pair_fst (mk_pair a b) = a` and
`pair_snd = b` **by conversion** (`= Proved`), `eta` by `Refl`, each with
negative `bad` controls (swapped terminal `Refl`/`Proved`, swapped
`True`/`False`) that **must reject**. A projection body-swap would be caught. So
[[a-catalog-pub-flip-is-inert-to-consumers-so-hunt-its-signature-closure-trust-and-prose]]'s
name-only degradation does not apply. And `pair_providers.is_empty()`
(`cat_ord_nat_canonical_owner.rs`) grounds that no catalog `Pair` provider
competes, so unshadowability is collision-free.

## How to apply

For a landed **floor-realization** (a type made unshadowable + companion
bindings admitted as strict-builtins):
(1) if a partition census test moves, read the **bucket transition** of every
changed item, not its vector delta -- `residual -> ambient` is `Err -> Ok`
(improvement) even though the ambient vector is non-empty; only `clean ->
ambient` is a regression, so check the `clean` bucket is intact;
(2) for each new production `Err` arm, find its **call site relative to user
input**: an arm in pre-source construction (single call site, fixed bootstrap
`globals`) is a self-check that CI catches wholesale, not a user-drivable
`Ok->Err` -- rank it accordingly;
(3) confirm the realization is collision-free (no competing provider for the
now-unshadowable name) and that it carries a real **definitional** oracle
(conversion theorems with negative controls), not name-resolution only -- if it
does, the inert-plane and name-only concerns do not apply. Do not file a
regression off a growing census vector alone. Reported evt_66bqnbkp0s04d
(lieutenant M8 thread thr_3ktdqsw5rvhqg).
