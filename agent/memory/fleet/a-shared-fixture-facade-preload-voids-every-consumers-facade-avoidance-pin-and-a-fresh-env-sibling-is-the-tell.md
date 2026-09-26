---
name: a-shared-fixture-facade-preload-voids-every-consumers-facade-avoidance-pin-and-a-fresh-env-sibling-is-the-tell
description: "A reuse migration that adds a facade dependency to a SHARED test dependency_env retroactively makes every consumer test's negative facade-avoidance assertion (`!globals.contains_key(\"<Facade>.\")`) necessarily false, so it gets deleted with no replacement, silently un-pinning that property for every consumer on the shared fixture. The sibling consumer that keeps its pin via a FRESH env is the tell that names which ones were dropped."
metadata:
  type: feedback
---

# A shared-fixture facade preload voids every consumer's facade-avoidance pin, and a fresh-env sibling is the tell

**Measured 2026-08-28 on CAT-NAT-REUSE-CONSUMERS D5 (`aa0e5cc44`).** D5
migrated Cursor to reuse canonical `Data.Numeric.Nat.Order.sub`, so it added a
`Data.Numeric.Nat.Order` preload to the *shared* `dependency_env()` of
`cc6a_process_arguments_exit_acceptance.rs`. That fixture is also used by
`arguments_reuses_the_canonical_lawful_classes_relation`, whose purpose was to
prove Arguments imports `leq_nat` from its canonical home **without** loading
the Order facade, asserted as
`!globals.keys().any(|n| n.starts_with("Data.Numeric.Nat.Order."))`.

Once the shared env preloads Order, that assertion is **necessarily false**, so
D5 deleted it, with no replacement. The facade-avoidance property for Arguments
is now **unpinned**: a later regression where Arguments imports `leq_nat`
*through* the Order facade (same resolved GlobalId, zero added trust) passes
every surviving cc6a assertion (trust delta, no local mint, `term_mentions`)
undetected. Still the case at `19c105b97`: `dependency_env()` loads
`Data.Numeric.Nat.Order` and the Arguments test carries no facade pin.

⇒ **The tell that named the drop: the sibling consumers that kept the pin.**
`cc4` (Diagnostics) and `cc5` (Doc) assert the *same* property but run it in a
**fresh `ElabEnv::empty()`** loading only the canonical owner and the one
consumer, so their shared-fixture Order preload never reaches the assertion.
The lone consumer whose facade pin ran on the shared `dependency_env` is exactly
the one that lost it. (At `19c105b97` cc4 still pins it in a fresh env; cc5's
test no longer asserts the Order namespace absent.)

## How to apply

When a migration adds a facade or provider **preload to a shared fixture
env**, diff-grep the whole fixture family for a **deleted negative assertion**
(`!contains_key` / `!keys().any(starts_with(...))`): a deletion, not just a
changed one. For each, check whether a **sibling** consumer still pins the same
property in a fresh env. A property pinned in a fresh env by one sibling and
dropped in the shared env by another is a silent coverage hole, not a real
change in what must hold. The fix is cheap and already demonstrated: give the
dropped consumer's assertion its own fresh env, as the sibling does.

The property usually still HOLDS (so this is a leak or gap, not a live
regression). Grep the consumer's own `import` line to confirm before ranking
severity.

Related: [[suppressing-a-row-can-empty-a-whole-named-gate]] and
[[certify-a-catalog-reuse-migration-by-body-identity-and-the-proofs-that-still-check]]
(why the facade-avoidance pin matters).
