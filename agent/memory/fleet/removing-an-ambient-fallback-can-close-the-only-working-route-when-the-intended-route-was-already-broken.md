---
name: removing-an-ambient-fallback-can-close-the-only-working-route-when-the-intended-route-was-already-broken
description: The qualified-access fallback story. A qualified `M.x` once resolved against any loaded module, so a proof backfill could land an undeclared provider edge that neither the strict census nor an import ledger saw. Its removal was then checked only against the constructs its fixtures exercised, and closed the only route to an inline child module whose intended import was already broken. Tabulate every spelling at parent and squash; check that a repair's walk stops at the unit root and that a guard's data source knows about the order its comment names.
metadata:
  type: feedback
---

# Removing an ambient fallback can close the only working route

A fallback that accepts more than the spec grants causes two kinds of trouble:
while it exists, code lands that depends on it unseen; when it is removed, a
construct whose intended route was already broken loses its last way in.

## While it existed: an undeclared provider edge no census sees

**Measured 2026-09-23 on CAT-CONFIGURATION-DECODER-LAWS.** Squash
`59bf793d48cb5b25c4f21ac061a188bf0ac07b64`. Filed LEAK/GAP to the Steward at
`evt_3spntbt1amt0p` (thread `thr_6z4d2qhb2rdz4`).

The laws needed `Data.Collections.Derived.nth` and seven Schema names in their
statements. The frame authorized those identities as growth of the
checked-closure ledger but no import change, and the laws spelled them fully
qualified. Decoder had no `import Data.Collections.Derived`, and its Schema
import was selective and omitted all seven.

At the time `resolve_ref` (`crates/ken-elaborator/src/modules.rs`) resolved
`M.leaf` against the export table of **any already-loaded module** without
checking that the unit imported M (a private leaf was still `UnboundName`).
Spec 33 §3.2 and ADR 0015 grant qualified access only through `import M`. A
fixture, in strict and legacy mode: `C` imports `B`, `B` imports `A`, and `C`
writes `A.a_id` — ACCEPT; the same text in a module with no imports — REJECT.
Resolution depended on what unrelated imports happened to load.

Nothing saw it: the strict-resolution census admits only **bare**
`UnresolvedCon` names, and an exact import ledger and an exact checked-closure
ledger were both green while disagreeing about which providers exist (the
closure named Derived; the ledger, whose doc claimed it "declares every
provider dependency", had no edge). **Compare two ledgers' module sets, not
each against itself.**

The hunt: scan each module's code fences for `(Upper.)+leaf` references to
catalog modules and classify each against that module's own imports (plain,
selective, none). The first two scans were wrong, and both produced a false
clean: a statement stripper that matched across newlines ate the body and
returned zero, and a selective import whose `(` began on the next line was
misread as plain, hiding the seven Schema references. **Require a scan to hit
the case you already know about before believing it**
([[before-declaring-a-citation-unresolvable-check-that-your-instrument-could-have-produced-a-hit]]).
What held: the laws were kernel-checked and publication was exact (4 direct
names plus 4 attached laws).

## Its removal: the construct no fixture reached

**Measured 2026-09-23 on LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT.** Squash
`04b30eab37663acbb700e9f8a15f0daa7098c945`. One correctness/gap finding at
`evt_7ffv9v4tvkpxc` (thread `thr_48eneav68sjyz`).

The WP removed the fallback above. The fix was correct and well tested. But an
inline `module N { ... }` inside a **roots-loaded** unit had only ever been
reachable through the fallback, as `A.N.x`. Its intended route, `import N`, was
already broken: the loader exempts `N` as a local module, but the child's
exports are stored under `A.N`, so `apply_import("N")` misses. After the fix no
spelling reached the child. CI stayed green because no catalog package or
roots-loader fixture uses an inline module; every inline fixture used the REPL
`elaborate_file` path, where the prefix is empty and the keys agree.

1. For any change that deletes a fallback or tightens a resolver, list the
   constructs that reach the changed branch, not only those the frame names,
   and which entry path each fixture uses (roots loader or REPL). A construct
   exercised on one path is unmeasured on the other.
2. Build one scratch test that tabulates **every spelling** (bare, fully
   qualified, each import form, alias, a sibling scope, a cross-unit client, a
   private control). Run it at the parent and at the squash, then revert. A
   cell OK at the parent and refused at the squash, in a row where every
   conforming spelling is refused at both, is the finding.
3. Name the pre-existing break separately from the squash's contribution: the
   squash removed the last way around a key mismatch it did not create.

## The repair's own walk can cross the unit boundary

Repair `f597a0cf8a14896e266946d210eb5204608fb260` resolved a bare `import N` by
walking the owner path outward with `rsplit_once('.')` over an edge map global
to the run. Nothing stopped the walk at the file unit's root, so `Data.Foo`
reached a `Data.N` another unit had declared (`evt_7twk6f8zns6zw`). The walk was
order-sensitive while the loader pre-scan deciding "inline, so do not load the
file" was not, and the unresolved case fell back to whatever `N` the caller had
loaded. When a repair adds an outward lexical walk, check whether it stops at
the unit root or only at the empty string, and whether pre-scan and resolver
agree on order. Probe each with a fresh env next to a shared one.

## A membership check cannot enforce an ordering claim

Follow-up `0e2fc7bd0a03d9cdb609cc8a9002cc4bc948c071` stopped the walk at the
file root and required the child to be in the unit's declared-module set. Its
comment says the edge must have been "installed by the ordered expansion
above", but the set it checks is the order-blind pre-scan, and the edge map is
still global. So an `import N` before the unit's own later `module N` passes on
a foreign edge; the same unit without the foreign module is refused
`UnboundName`. Reported LOW as a pre-existing residual at `evt_201scy08e8cpn`.
When a guard's comment names an ordering ("installed by", "already", "above"),
check whether its data source knows about order; probe the conforming and
reversed orders, each with and without the decoy.

Related:
[[a-shape-deviation-from-the-frame-or-an-import-the-source-never-names-is-a-candidate-until-the-conforming-variant-is-measured]],
[[a-deleted-guard-can-be-the-only-enforcement-of-a-second-thing-nobody-named]]
(the mirror case: a deleted guard was the only enforcement of a second property;
here a deleted fallback was the only provider of a second route),
[[a-gate-change-is-hunted-on-the-axis-its-direction-leaves-open]].
