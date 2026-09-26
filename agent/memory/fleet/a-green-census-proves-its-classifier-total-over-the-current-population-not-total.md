---
name: a-green-census-proves-its-classifier-total-over-the-current-population-not-total
description: "A census or oracle that landed green proves only that its classifier is total over the CURRENT population, never that it is total. A partial classifier whose unhandled tail panics, returns `_ => None` and panics at the call site, or string-prefixes naively where a sibling helper defends with longest-match, is a latent abort or silent drop keyed on the next population addition. Confirm the central invariant is a real cross-check, then move to the classifiers that feed it: enumerate each one's domain and find the tail it does not handle. Measured on the LANG-MOD evidence-frontier census (027f6bf26): four latent gaps around a sound core."
metadata:
  type: feedback
---

# A green census proves its classifier is total over the current population, not total

**Measured 2026-08-25 on the landed squash `027f6bf26`** (LANG-MOD Component B
evidence-frontier partial, M8 post-merge hunt; the whole landed range was one
new additive test file, zero production code). Verdict: four grounded gaps, all
latent, around a sound core. The file now lives at
`crates/ken-elaborator/src/r_layer_tests/lang_mod_catalog_evidence_frontier.rs`
(line numbers below are from the landed version and have since shifted).

## The core was a genuine cross-check (do not stop there)

The census's central invariant `validate_evidence_frontier` over `ExactLedger`
rows is a real two-sided oracle: strict success requires empty prerequisites,
and a strict refusal must be explained by an unmet identity prerequisite or a
refused dependency. It is not vacuous. But a sound central invariant does not
clear the classifiers that FEED it, and that is where all four gaps sat.

## The meta-lens: green proves total-over-the-current-population, not total

A census that landed **green** in CI proves only that its classifier is total
over the **current** population. A partial classifier whose unhandled tail
**panics** (or returns `_ => None` and panics at the call site, or naively
string-prefixes where a sibling helper defends with longest-match) is a
**latent abort or silent drop keyed on the next population addition**, and the
green says nothing about it.

So when you review a census: **enumerate the classifier's DOMAIN** (the enum
variants it matches, the name shapes it assumes) and check the **TAIL it does
not handle** against what the population can actually produce. The green
landing is still useful for triage: it proves every such gap is currently
dormant, so each is latent, not live.

## The four instances

1. **A `panic!` tail aborts the whole shared census.** `defining_module`
   (`:350-355`) attributes a dependency by dotted prefix
   `name.starts_with("{module}.")`, but class, instance, law, foreign, temporal
   and prove decls are stored **unqualified** (`elab.rs
   globals.insert(rdecl.name...)`; `is_qualifiable` at `modules.rs:1571`
   excludes them). A bare `import M` pulling in such an identity escapes the
   selective-import exemption (`:484-489`, only `Selective` populates the
   bindings), `defining_module` (None on a dot-less name) and `base_support`,
   and hits the final `panic!` at `:518-521`, aborting the `OnceLock` census for
   **all 8** sibling `#[test]`s. Reachable with constructs the catalog already
   uses (bare imports at `Core/Classes/LawfulClasses.ken.md:43,48`). Sibling
   shape: `failure_stage` (`:234-252`) is non-exhaustive over `ElabError` (~26
   unmatched variants, including `ReExportCollision`, in this facade reorg's
   blast radius) and panics at its call sites. COORDINATION section 7
   (exhaustive by construction, no `_ =>` on a load-bearing classification) is
   exactly this.
2. **A naive prefix a sibling helper already defends against.** `owned`
   (`:429-436`) marks an identity root-owned if any name starts with
   `"{module}."`, whereas the sibling `defining_module` is fed
   `modules_longest_first` (`:460-461`). On a future nested stem pair (`R.ken` +
   `R/X.ken`), an identity from unit `R.X` that `R` references without importing
   is classified root-owned and dropped from the ledger. Dormant: no current
   module name dotted-prefixes another (50 modules). **When two helpers classify
   the same hierarchical names and only one carries the longest-match defense,
   the other is the drop.**
3. **Coincidence certified as causation.** `imported_refusal` (`:598-603`,
   stage swallowed by `{ .. }`) makes a strict refusal "explained" (`:626`) if
   ANY direct import strict-refuses at ANY stage, so two independently broken
   units on one import edge certify the importer's own unrelated refusal as
   dependency-explained, an attribution the `:628` error string claims but the
   check never establishes.
4. **Rich ledger, shape-only enforcement.** `validate_population` at the
   headline site (`:655`) is a self-comparison (`discovered = units.keys()`
   against rows built 1:1 from the same units), blind to `discover_units`
   missing a real file (only a 2-file fixture guards discovery).
   `validate_evidence_frontier` destructures `resolved_globals` as `_` and checks
   prerequisites only by `.is_empty()`, and the per-row loop (`:657-663`) only
   checks `parsed_direct_imports` and `println!`s, so the ledger contents and
   each prerequisite's fields (including `provider_interface_available`) are
   pinned only on fixtures. The "exact core identities" naming over-reads what
   is enforced.

## How to apply

For a landed census or oracle over a population (catalog units, enum variants,
files):

1. Confirm the central invariant is a real cross-check, then **move to the
   classifiers that feed it**.
2. Enumerate each classifier's domain and find its unhandled tail (a `panic!`,
   a `_ => None`, a naive string test); ask what population member reaches it.
3. Read the green as "dormant over the current population", never as "total".
4. When two helpers classify the same hierarchical names, diff their defenses.
   The weaker one is the drop.
5. A shared `OnceLock` census makes ONE classifier panic abort EVERY sibling
   test, so a tail crash is a whole-file outage, not one row.

A read-only lens fan-out (helper logic, invariant vacuity, cross-check
soundness, API drift) is the right breadth tool, and here surfaced findings 1
and 4. Verify every finder verdict at source and re-rank severity yourself: a
non-TCB test oracle's mis-classification is correctness or leak-or-gap, not
soundness. Reported `evt_5444h02jvacg3` (lieutenant M8 thread
`thr_7qt7yzncqmc35`).

Related: [[exhaustiveness-comes-from-an-unguarded-arm-not-from-the-match]] (the
compile-time form of the same tail),
[[a-consumer-wall-watch-predicate-is-discharged-by-a-whole-product-scope-census-and-a-fail-closed-strip-guards-only-the-stripped-edges]]
(a fail-closed guard proves nothing about inputs it does not enumerate), and
[[a-measurement-census-can-exactly-pin-a-partition-yet-leave-the-majority-bucket-unmeasured]].
