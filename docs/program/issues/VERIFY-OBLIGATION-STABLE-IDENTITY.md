---
id: VERIFY-OBLIGATION-STABLE-IDENTITY
title: "Obligation ids and hole-postulate symbols are derived from session allocation order (Obligation.id and global_<GlobalId>), and both feed core_semantic_hash, against spec 46 §3.2 and 22 §1. Derive them from stable inputs so the same source yields the same ids and hash in any session"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-CALLER-OBLIGATION-REPORTING]
blocks: []
github: null
origin: "Architect carry evt_6jvxr0czd8cfr in the VERIFY-CALLER-OBLIGATION-REPORTING AC-0 ruling: that WP keys the new package obligation entries on these ids, so its pins assert prefixes only. Placed after it on the verify ring. Steward-filed per COORDINATION section 2. Measured at origin/main 75e458cc1."
---

# Obligation identities are stable

## Objective

The same source, elaborated in any session and after any unrelated
declarations, yields the same obligation ids, hole-postulate symbols and
`core_semantic_hash`.

## Settled inputs (measured at `75e458cc1`)

- **The spec.** 46 §3.2: no exported identity, semantic hash input,
  obligation id or assumption entry may depend on `GlobalId` allocation
  order. 22 §1: an obligation `id` is stable across edits.
- **Hole symbols.** The emitter falls back to `global_<GlobalId>` for a
  declaration with no stable symbol (`compiler_driver.rs:4034`, `:4062`).
  That is the `global_628` in the AC-0 trust delta of
  `VERIFY-CALLER-OBLIGATION-REPORTING`.
- **Obligation ids.** `lift_obligation` (`extract.rs:118`) builds
  `{def}.requires.{obl.id}` (`:140`) from `Obligation.id`, "sequential
  within this elaboration session" (`elab.rs:67`).
- **Unmeasured:** which other declarations reach the `global_` fallback, and
  whether any pinned hash or conformance row depends on today's ids.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **AC-0, at the start of the repair.** Elaborate one file with an open
   `requires` twice: alone, and after unrelated declarations. Record both
   runs' obligation ids, hole symbols and `core_semantic_hash`. List every
   declaration kind that reaches the `global_` fallback, and every test or
   conformance row that pins an affected id or hash. The Architect then
   rules the stable key, for example the owning declaration's stable symbol
   plus the obligation's position within it.
2. **The ruled repair** for every producer of those ids, not only `requires`
   (CHECKS 2).

## Acceptance

- **AC-1.** The two AC-0 runs produce identical ids, symbols and hash. The
  pins of `VERIFY-CALLER-OBLIGATION-REPORTING` can assert the full id.
- **AC-2 (controls).** Two distinct obligations in one declaration keep
  distinct ids. Files without obligations keep their hash.
- **AC-3 (falsifier).** Restoring the session counter in the id turns AC-1
  red.

## Stop conditions

- Any kernel, `trusted_base()` or spec change.
- A pinned hash or conformance row changes: stop with the consumers listed
  (CHECKS 3).
