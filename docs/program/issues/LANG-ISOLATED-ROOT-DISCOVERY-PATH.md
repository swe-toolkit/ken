---
id: LANG-ISOLATED-ROOT-DISCOVERY-PATH
title: "Isolated catalog-root discovery breaks two spec 33 §3.2 rules. It resolves `..` lexically, so a spelling through a symlink takes its root from a directory that is not an ancestor of the file the OS reads. And ken run and ken native-build skip the catalog-unit path rule that ken check applies first, so a catalog entry is reloaded as its own dependency and its import cycle is rooted at the wrong unit. Discover from the file actually read, and apply the catalog-unit rule first on every route"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [LANG-STRICT-ISOLATED-ROUTES]
blocks: []
github: null
origin: "Adversary evt_1f13n2n4mwrzp and evt_2bqgw09fpnfc6 on Inc 3e 5a3cc337b; the mechanism landed in Inc 3b 4f7b64631. Mechanism re-read by the Steward on 5a3cc337b. L2 objective: one resolution mode (operator 2026-09-25). Steward-filed per COORDINATION section 2."
---

# Roots come from the file that is read

## Objective

Every CLI route discovers the same root for the same file, however its
path is spelled. A source under `catalog/packages` is a catalog unit on
`check`, `run` and `native-build` alike, and it is loaded once.

## Fixed inputs (main `5a3cc337b`)

- **The rule.** `spec/30-surface/33-declarations.md:171`-`:177`:
  discover "from its source path: inspect the file's parent, then its
  ancestors nearest first", and "a source already under
  `catalog/packages` takes the existing catalog-unit path rule above
  first; isolated-root discovery does not reinterpret a catalog entry."
- **Lexical `..`.** `isolated_catalog_root` (`modules.rs:2193`-`:2211`)
  takes `std::path::absolute(path)` and pops one component for each
  `..` without resolving symlinks. Its comment says lexical removal was
  chosen deliberately. The routes then read the source through the OS,
  which resolves `lnk/..` physically (`main.rs:311`, `:612`, `:495`).
- **The symlink repro** (Adversary `evt_1f13n2n4mwrzp`). `ken check
  R/app/lnk/../sub/Entry.ken`, with `lnk` pointing to `elsewhere/a/b`, is
  accepted with R's root. The same file spelled
  `elsewhere/a/sub/Entry.ken` refuses with UnboundName.
- **Catalog rule on check only.** `ken check` tries
  `catalog_module_from_path` first (`main.rs:516`). `native_build_file`
  (`:321`-`:323`) and `run_file` (`:621`-`:623`) call only
  `isolated_catalog_root`.
- **The cycle repro** (Adversary `evt_2bqgw09fpnfc6`). For
  `catalog/packages/Demo/App.ken`, which imports `Demo.Helper`, which in
  turn imports `Demo.App`, `check` reports the cycle
  `[Demo.App, Demo.Helper, Demo.App]`. `run` and `native-build` report
  `[Demo.Helper, Demo.App, Demo.Helper]`, because they reload the entry
  file as a unit.
- **The conformance case.** Inc 3e's
  `isolated-file-discovers-catalog-root-from-source-path`
  (`conformance/surface/modules/seed-modules.md`) has a `..` arm with no
  symlink.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`isolated_catalog_root` and the three CLI discovery sites, plus a
conformance row if AC-0 changes the case's contract. The module-path
rule and Strict resolution do not move.

## Deliverable

One candidate: discovery over the path of the file actually read, the
catalog-unit rule applied first on `run` and `native-build`, and the
rows below.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Rule two things:
  - whether "its ancestors" means the canonical path's ancestors, and
    what a path that cannot be canonicalized does;
  - what `run` and `native-build` do with a catalog entry. Spec 70 §1.4
    sets `module` to null for a non-check catalog entry, so the run
    treatment may need a Spec co-turn. Name the owner if it does.
- **AC-1.** The symlink repro refuses through the symlinked spelling,
  exactly as through the direct spelling, on `check`. A symlink-free
  `..` spelling still discovers its root.
- **AC-2.** For the cycle repro, `run` and `native-build` report the
  cycle rooted at the entry unit, `[Demo.App, Demo.Helper, Demo.App]`,
  as `check` does. A cycle-free catalog entry still runs.
- **AC-3.** Restoring lexical `..` makes AC-1's symlink row accept, and
  removing the catalog-rule call from `run` restores the
  wrong-rooted cycle.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- AC-0 rules that `run` must refuse a catalog entry, or that the
  answer changes spec 70's report contract. That is a Spec enclave
  turn first.
- A change to Strict resolution or to the module-path rule.

Checks: 2 fired: the three discovery call sites and
`isolated_catalog_root` were read at `5a3cc337b`. 8 fired: each AC row
pairs a spelling or route that diverges on main with one that must stay
the same. 9 fired: the conformance `..` arm holds symlinks constant, so
AC-1 varies only the symlink.
