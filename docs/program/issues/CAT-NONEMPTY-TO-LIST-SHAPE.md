---
id: CAT-NONEMPTY-TO-LIST-SHAPE
title: "Data.Collections.NonEmpty keeps its constructor abstract but exports no law relating nonempty_to_list to nonempty_head and nonempty_tail, so an importer cannot prove anything about the list view of an arbitrary NonEmpty. Export the shape law"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: []
blocks: [SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT]
github: null
origin: "Architect D0 STOP evt_6xr53agbj43fr on SPEC-STATISTICS-EMPIRICAL-SAMPLE-CONTRACT: the empirical tier's carrier is NonEmpty q, and an importer needs this provider law; a duplicate head-plus-tail carrier was rejected. Steward-filed per COORDINATION section 2."
---

# NonEmpty exports its list shape

## Objective

An importer of `Data.Collections.NonEmpty` can rewrite
`nonempty_to_list a xs` to `Cons a (nonempty_head a xs) (nonempty_tail a
xs)` for any `xs`, while `NonEmpty` stays abstract.

## Settled inputs (Architect `evt_6xr53agbj43fr`, at `68e40d757`)

- `NonEmptyCons` is not exported: importing it gives `UnboundName`, and
  matching on it after a function import gives `UnresolvedCon`.
- An importer's `Proved` of the shape equation on an arbitrary `xs` is
  `KernelRejected TypeMismatch`.
- The Architect checked this pair in a scratch copy of the provider, placed
  before `nonempty_map`, at rc=0:

```ken
theorem nonempty_to_list_shape_cons (a : Type) (x : a) (rest : List a) :
    Equal (List a)
      (nonempty_to_list a (NonEmptyCons a x rest))
      (Cons a (nonempty_head a (NonEmptyCons a x rest))
              (nonempty_tail a (NonEmptyCons a x rest))) = Refl

pub theorem nonempty_to_list_shape (a : Type) (xs : NonEmpty a) :
    Equal (List a) (nonempty_to_list a xs)
      (Cons a (nonempty_head a xs) (nonempty_tail a xs)) =
  match xs { NonEmptyCons x rest ↦ nonempty_to_list_shape_cons a x rest }
```

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The public theorem `nonempty_to_list_shape` in
`catalog/packages/Data/Collections/NonEmpty.ken.md`, with a sentence of
literate prose, and the private helper. No constructor export.

## Acceptance

- **AC-1.** An importing module that imports only the functions and the
  law proves a fact about an arbitrary `NonEmpty`'s list view at rc=0, and
  the same statement with `Suc` added to its length side is kernel-rejected.
- **AC-2.** The package's existing tests, and the importer and pin files a
  grep of every source root finds for `Data.Collections.NonEmpty`, stay
  green. Targeted runs only.

## Stop conditions

- The law needs `NonEmptyCons` exported, an Axiom, or a kernel, trust or
  spec change.
