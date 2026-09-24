---
id: CAT-CONCAT-MAP-APPEND-DISTRIBUTIVITY-LAW
title: "Proof-backfill for Data/Collections/Derived.ken.md: prove privately that concat_map distributes over list_append -- for every a, b, f, xs and ys, concat_map a b f (list_append a xs ys) = list_append b (concat_map a b f xs) (concat_map a b f ys) -- with a test that pins the law's checked proposition, not only its name and trust"
status: active
owner: foundation
size: S
gate: architect
tier: T1
depends_on: [CAT-VEC-MAP-IDENTITY-LAW]
blocks: []
github: null
origin: "Architect ruling evt_62vxwb1ymyc18 at ac0f42f0b, retargeting the withdrawn singleton-identity nomination evt_3vrcj7nytczq8 (its type-position lambda does not parse; no importable identity without an import cycle). K3-independent L3 runway successor to CAT-VEC-MAP-IDENTITY-LAW (start only after it lands). Operator ruling 2026-09-13: 'schedule the proof backfill before extending the catalog.' Steward-filed per COORDINATION section 2."
---

# `concat_map` is unproved over `list_append`

## Settled inputs -- measured at `ac0f42f0b`. Re-ground before acting.

- `pub fn concat_map (a : Type) (b : Type) (f : a → List b) (xs : List a)
  : List b` in `catalog/packages/Data/Collections/Derived.ken.md` §4.2 is
  transparent and structurally recursive; its `Cons h t` branch is
  `list_append b (f h) (concat_map a b f t)`.
- `pub fn list_append` and its checked `pub proof assoc for list_append` are
  in Derived. `Core.Logic.Transport (cong, sym, trans)` is already imported.
  No distributivity law for `concat_map` exists.
- As with Vec, a test that checks only a private law's name, kind and trust
  passes when its statement is weakened. This frame pins the proposition.
- A lambda in a theorem's type does not parse. This statement has none; a
  lambda may appear only in the proof expression.

## Deliverable

One checked private law in Derived §4.2:

```
theorem concat_map_append
    (a : Type) (b : Type) (f : a → List b)
    (xs : List a) (ys : List a)
  : Equal (List b)
      (concat_map a b f (list_append a xs ys))
      (list_append b (concat_map a b f xs) (concat_map a b f ys))
```

Induct on `xs`. `Nil` is definitional. For `Cons h t`, lift the recursive
proof under `λw. list_append b (f h) w` in the proof expression, then use
`sym` of `(proof assoc for list_append)` to reassociate the right side. No new
operation (public or private helper), constructor, import, axiom, public
export or trust entry.

## Acceptance criteria

- **AC-1 (expressibility first).** Show that the literal statement parses
  and closes in Derived, or STOP. This is a fresh law, not a proof of the
  withdrawn singleton statement.
- **AC-2 (falsifier).** In a scratch copy, change only `concat_map`'s `Cons`
  recursive tail to `Nil b`, keeping `list_append b (f h) …`. A concrete
  two-element `Nat` append under a singleton mapper differs between baseline
  and mutation, and the new law rejects **at its own span** before any other
  law. Restore byte-identically.
- **AC-3 (proposition pin).** After checked roots loading, locate the
  private theorem by its exact `GlobalId`. Decode its raw checked type
  structurally, without conversion or text grep:
  - five binders `a`, `b`, `f`, `xs`, `ys`;
  - `Equal` at exact `List b`;
  - both endpoints built from the exact `concat_map` and `list_append` ids;
  - correct de Bruijn indices.

  Control: replace only the statement with `Equal (List b) LHS LHS = Refl`.
  The package still loads, and the pin must redden.
- **AC-4.** No new trust: compare the loaded closure's trust ledger before
  and after, with a nonzero targeted test count. Any public-export census is
  a transition sentinel. Targeted builds only, through `scripts/ken-cargo`.
  No-regression means green in CI.

## Stop conditions

- If an older check fails first under the AC-2 mutation, report it and do
  not claim the falsifier.
- If the reassociation does not close as sketched, STOP and return the exact
  goal for rescope.
- No String or Bytes literal use, new operation, publication or catalog
  extension.
