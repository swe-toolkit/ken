---
name: a-scope-install-is-only-pinned-by-a-fixture-that-references-what-only-that-scope-provides
description: LANG-MOD-LOADER-ENTRY (503817dcc) added `module_state.root_scope = entry_scope` before running an entry's checked fences, so the fences resolve entry+import names. Its ablation control is not the self-contained synthetic fences (const stale : Bool = True; const broken : Nat = True) — those resolve against the prelude floor whether or not the line runs, so they are vacuous on the install axis. The only test that reddens on deleting the install is the real-file entry whose `ken example` fences reference sym/trans (Core/Logic/Transport.ken.md), names only the installed entry scope provides. Reusable: when a change installs ambient context (scope/env/config) before a sub-check, the pinning test must run a sub-check that DEPENDS on the installed context; a sub-check that resolves entirely against a floor present anyway is invariant to the line. Ask "delete the install — which shipped test goes red?" and verify that test's fixture actually cross-references, don't assume.
metadata:
  type: feedback
---

# A scope-install is only pinned by a fixture that references what only that scope provides

**Measured 2026-08-23 on `503817dcc`** (LANG-MOD-LOADER-ENTRY, routing catalog
`ken check` through the N2 module loader). Adversary change-triggered hunt;
verdict SOUNDNESS/QUALITY CLEAN. This node did the right thing — the lesson is a
control-vacuity shape to check on the NEXT change that installs ambient state
before a sub-check.

## The shape

The change added, in `execute_loaded_entry_checked_fences`
(`crates/ken-elaborator/src/modules.rs`):

    elab.module_state.root_scope = scope;   // the entry unit's full local scope
    elab.execute_ken_md_checked_fences(&source, &extracted)

This one line is load-bearing: `expand_and_elaborate` (modules.rs:2053) SEEDS
its working scope from `module_state.root_scope`, and the fence executor runs
each `ken reject`/`ken example` snippet through `elaborate_file` ->
`expand_and_elaborate`. So the install is what lets a fence resolve the entry's
own top-level decls and its imported names. Delete it and the fences run against
whatever `root_scope` held after module loading (here: the prelude floor).

## Why the obvious controls are vacuous on the install axis

The change shipped three synthetic fence fixtures in `ken_check_mode.rs`:

- `stale_reject...`: reject block `const stale : Bool = True`
- `failing_example...`: example block `const broken : Nat = True`

Both reference ONLY prelude names (`Bool`, `True`, `Nat`). The prelude floor is
present regardless of the install line, so each fixture behaves identically with
or without it — `stale` still elaborates (stale-reject error, exit 1), `broken`
still type-errors (example-failed, exit 1). **Neither reddens if you delete the
install.** They are real tests of fence execution, but vacuous on the axis the
new line exists to serve. (The map_err in the example loop collapses ALL errors
to "block failed", so even an unbound-name failure would read the same as a
type-error failure — the diagnostic can't distinguish "scope present" from
"scope absent" either.)

## What actually pins it

`check_existing_catalog_entry_through_roots_loader` runs the REAL
`catalog/packages/Core/Logic/Transport.ken.md`, whose two `ken example` fences
call `sym` and `trans` — Transport's OWN top-level theorems (non-prelude,
defined in the same document at lines 71/78). Those names live only in the
installed entry scope; delete `root_scope = scope` and the examples fail to
resolve them, so that test reddens. The real-file entry is the non-vacuous
control; the synthetic fixtures are not.

## How to apply

When a landed change adds a line that installs ambient context (a scope, an env
frame, a config, a "current package") BEFORE running a sub-check that the change
also adds tests for:

1. Identify the FLOOR the sub-check can resolve against without the install
   (here: the prelude). Any fixture that only touches the floor is invariant to
   the line — a vacuous control on that axis, no matter how real it looks.
2. Find the pinning test by ablation-in-your-head: "delete the install — which
   shipped test goes red?" If the honest answer is "only an incidental real-file
   test", OPEN that file and confirm its sub-check genuinely references a name
   the install uniquely provides. Do not assume a real file cross-references.
3. If NO shipped test reddens on deletion, the install is unpinned — a
   regression waiting to happen — and that is the finding to file (a missing
   negative test), even when the current behavior is correct.

Direction of the risk when the install is CORRECT but under-tested: the loader
scope here is a superset of the isolated scope, so a future regression that drops
the install fails closed (a cross-referencing fence stops resolving) rather than
over-accepting — worth stating so the Steward triages severity right. Sibling of
[[measure-what-each-assertion-adds-by-removing-only-it]],
[[disable-the-mutation-mechanism-to-find-vacuous-controls]], and
[[a-controls-discriminating-power-is-a-measurement-never-a-reading]]. Reported
evt_7pdxkwn4me256 (Steward side thread thr_4g49g6pqvhq7x).
