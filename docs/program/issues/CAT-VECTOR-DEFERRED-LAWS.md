---
id: CAT-VECTOR-DEFERRED-LAWS
title: "Prove the Vector laws spec 60 §5 defers: the lookup/map/zip_with cons computations, map fusion, lookup after map and after zip_with, and zip_with/map naturality, replacing the concrete Bool examples as the package's evidence"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "L3 proof backfill (operator 2026-09-13: 'A catalog package is not finished until its proofs are complete'; Architect evt_5f1ewknxv3m6h: L3 turns to proof backfill). Named deferred laws in spec/50-stdlib/60-length-indexed-vectors.md §5. Steward-filed per COORDINATION section 2."
---

# Prove the deferred Vector laws

## Objective

`Data/Vector/Vector.ken.md` proves the laws its spec defers that are
expressible over its landed functions. Its lookup-after-map and
lookup-after-zip evidence becomes general theorems, not Bool examples.

## Fixed inputs (read at `96818f2e4`)

- **Landed functions:** `head`, `tail`, `map`, `zip_with`, `lookup`
  (`Vector.ken.md:50-87`).
- **Landed theorems:** `head_vcons`, `tail_vcons`, `map_vnil`,
  `vec_map_identity`, `zip_with_vnil`, `lookup_fzero` (`:89-127`).
- **Examples only:** `vec_example_lookup_second`, `vec_example_map_second` and
  `vec_example_zip_second` (`:186-226`) check one concrete Bool vector each.
- **Deferred by the spec** (`spec/50-stdlib/60-length-indexed-vectors.md`
  §5): the `lookup … (fsuc …)` computation, `zip`/`map` naturality, the
  `zip`-`unzip` round trip, and the length / `to_list` bridge.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Symptom inventory

1. Generic `zip_with_vcons` `Refl` rejected; keyed on the tail binder of an
   index-refined sibling match at an open index (Architect
   `evt_5p7xx5e6tmegw`; §1a 0 to 1). `zip_with_vcons`, and any law that
   needs it, is deferred to `LANG-REFINED-SIBLING-MATCH-TAIL`. The rest
   lands now.
2. `lookup_zip_with` `ExhaustivenessError` at inner sibling match after
   i→xs; keyed on the refined sibling binder (Architect
   `evt_31z4jzrt6v8wf`; §1a stays 1, same family). A later law failing with
   one of the family's three signatures is deferred without a new stop.
3. `zip_with_map` as framed puts a lambda in a type position, which the
   surface grammar does not express, and it also meets the sibling family
   (Architect `evt_7aem5zqk3dqm8`). **Steward: the resumption statement is
   the pointwise generalization.** It takes `k : a → a2 → c` and
   `hk : (u : a) → (v : a2) → Equal c (k u v) (f (g u) (h v))`, and
   equates `zip_with f (map g xs) (map h ys)` with `zip_with k xs ys`. The
   lambda instance is recovered at the use site. It stays deferred until
   `LANG-REFINED-SIBLING-MATCH-TAIL` lands.

## Increments landed

- `10af5f45f`: `lookup_fsuc`, `map_vcons`, `vec_map_compose`, `lookup_map`.
- `79a6e5ac7`: `zip_with_vcons`, admitted by
  `LANG-GENERATED-J-PROOF-ASCRIPTION` (`b40f28977`). Inventory item 1 is
  closed.
- Open, released now: the pointwise `zip_with_map` (item 3). It checks on
  `f8bc5d4e1` as a scratch proof: VNil `Proved`, VCons `sym hk`, `cong` on
  head and tail, `trans`, with `sym`/`trans` from `Core.Logic.Transport`
  (language-implementer `evt_1hpn2yffh1jax`).
- Open, waiting: `lookup_zip_with` (item 2). It fails at `elab.rs:3535`
  `Internal("index-equation convoy unexpectedly overlaps an ambient
  context convoy")`, which is `LANG-SIBLING-GOAL-REFINEMENT` increment 2's
  repair target. The WP stays open until it lands.

## Deliverable

Checked theorems in `Vector.ken.md`, stated over arbitrary `a`, `b`, `c`,
`n`, `f`, vectors and `Fin n` indices:

1. `lookup_fsuc`: `lookup a (Suc n) (VCons a n x xs) (FSuc n i)` equals
   `lookup a n xs i`.
2. `map_vcons` and `zip_with_vcons`: the cons computations.
3. `vec_map_compose`: `map g (map f xs)` equals `map (λx. g (f x)) xs`.
4. `lookup_map`: `lookup (map f xs) i` equals `f (lookup xs i)`.
5. `lookup_zip_with`: `lookup (zip_with f xs ys) i` equals
   `f (lookup xs i) (lookup ys i)`.
6. `zip_with_map`: `zip_with f (map g xs) (map h ys)` equals
   `zip_with (λx. λy. f (g x) (h y)) xs ys`.

The three Bool examples may stay as illustrations. The prose then names the
theorems as the evidence.

Not this node: the `zip`-`unzip` round trip and the `to_list` bridge. Both
need functions the package does not have.

## Acceptance

- **AC-1.** All six theorems check. `trusted_base()` is unchanged, with no
  axiom or postulate added.
- **AC-2 (controls).** Each of the following is rejected, and each mutation is
  restored:
  - a false variant of `lookup_map`, with `f` applied to the wrong index;
  - a false variant of `zip_with_map` with `g` and `h` swapped, at a type
    where they differ;
  - a well-typed mutation of `map`, `zip_with` or `lookup` that makes at
    least one of items 3-6 fail, chosen by the implementer and named in the
    handoff. If a theorem admits no well-typed mutation, because the index
    type already forces the law, say so for that theorem rather than
    inventing one.
- **AC-3.** The existing Vector acceptance suites stay green:
  `cat_vec_acceptance`, `cat_vector_closeout`, and the Vector rows in
  `cat_idf_trust_free_provider` and `lang_mod_strict_resolution_d0`.

## Stop conditions

- A law that the current dependent `match` cannot state or prove, for
  example a `Fin n` and `Vec a n` pair that does not refine (check 11): stop
  to the Architect with the obligation as written.
- Any new function, export, kernel change or trust change.
