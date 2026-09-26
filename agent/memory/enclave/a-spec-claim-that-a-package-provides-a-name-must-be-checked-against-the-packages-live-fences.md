---
name: a-spec-claim-that-a-package-provides-a-name-must-be-checked-against-the-packages-live-fences
description: When a spec says a catalog package supplies a name, possibly with its own identity, check the package's live (non-ignore) code fences. The name may appear only in an illustrative `ken ignore` block, while the live code consumes the compiler's ambient identity.
metadata:
  type: feedback
---

# A package's claimed name must be found in its live fences

**Measured 2026-09-25 on SPEC-EMPTY-INTERNAL-LEDGER.** Squash
`ee22122af1fa37fef5c1e06428b55a1d3d16af2d`, reported at `evt_3150m78jteyb2`.

## The shape

The spec change said the source name `Empty` comes from the catalog package
`Core.Logic.EmptyDec`, "with an identity distinct from the compiler's". Both
the Architect and the CV passed it. The package does name `Empty`: in its
title, in its public-API list, and in a `data Empty` block. But that block is
a `ken ignore` fence. The package's own prose says the names are ambient and
are "not declared by this entry". Its live functions use the prelude's
`Empty`.

A second joint made the claimed target state incoherent. The
compiler-installed `Dec.No` fixes the compiler's `Empty` ID in its
constructor type. So a distinct catalog `Empty` could not type the package's
own `no`.

## How to apply

1. For any "package P provides name N" claim, list P's code fences with their
   info strings, for example with
   `awk '/^```/{...}'`. Look for a declaration of N in a fence that is not
   `ignore`. A mention in prose or in the public-API list is not a
   declaration.
2. If N is ambient, find its installer (`prelude.rs`) and every other
   compiler declaration that embeds its ID (`indformer(<id>)`). Each of those
   is a reader, and a "distinct identity" target has to survive every one of
   them.
3. Check whether an existing test already measures ambient resolution, such
   as an `elaborate_decl` over N with no import. That test is the repro, so no
   new probe is needed.

Related: [[my-own-tracker-capability-landed-line-can-be-stale]],
[[a-guard-added-to-a-pattern-can-consume-the-token-the-pattern-searches-for]].
