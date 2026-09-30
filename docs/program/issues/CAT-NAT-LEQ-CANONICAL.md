---
id: CAT-NAT-LEQ-CANONICAL
title: "Five elementary leq_nat facts are proved privately in Map, Gcd and Parsing, each a copy of a law Nat.Order or its provider already owns. Publish the order laws once from Data.Numeric.Nat.Order and retire the private copies"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: [CAT-FORMATTING-DOC-LAWS]
blocks: []
github: null
origin: "Architect carries evt_2w0bcx37ysnen (the Nat leq dedupe) and evt_bbf8w6ww9208 (Order's new private leq_nat_successor_bound is the same fact as Parsing's leq_nat_suc), on CAT-FORMATTING-DOC-LAWS. L3 successor, following the CAT-LIST-APPEND-LENGTH-CANONICAL precedent. Steward-filed per COORDINATION section 2."
---

# One home for the elementary Nat order facts

## Objective

Each elementary `leq_nat` fact is stated once, as a public law of
`Data.Numeric.Nat.Order` or its provider, and every catalog package uses that
law instead of a private copy.

## Settled inputs (measured at `a971be84f` plus `afb528e90`)

- **The owner.** `Nat/Order.ken.md` re-exports the provider's `leq_nat`, whose
  `refl`, `trans`, `antisym` and `total` are public attached proofs
  (`LawfulClasses.ken.md:514-606`). After `CAT-FORMATTING-DOC-LAWS`, Order
  also carries these bounds:
  - a private `leq_nat_successor_bound n : leq n (Suc n)`;
  - a public `leq_nat_add_left_bound a b : leq a (add a b)`;
  - a public `leq_nat_add_right_bound a b : leq b (add a b)`.
- **The private copies**, each written `Equal Bool (leq_nat …) True`:

  | Copy | Statement | Same fact as |
  |---|---|---|
  | Parsing `leq_nat_suc` (`:3239`) | `leq n (Suc n)` | Order's successor bound |
  | Map `leq_nat_add_right` (`:17131`) | `leq a (add a extra)` | `leq_nat_add_left_bound` |
  | Gcd `leq_refl` (`:328`) | `leq a a` | `proof refl for leq_nat` |
  | Map `leq_nat_right_successor` (`:17119`) | `leq a b → leq a (Suc b)` | Gcd's copy; Order has no owner yet |
  | Gcd `leq_weaken_right` (`:315`) | `leq a b → leq a (Suc b)` | Map's copy |

  All five are used only inside their own package.
- **Not duplicates.** These are left alone:
  - Parsing `nat_leq_suc` is over `LessEqNat`, a different relation;
  - Cursor `cursor_leq_suc_add_right` is a shifted strict bound;
  - Gcd `leq_left_of_sum` and `leq_right_of_positive_sum` are derived facts.
- **Import edges.** Gcd already imports Order (`Gcd.ken.md:20`), and Parsing
  imports `sub` from it (`:115`). Map imports `leq_nat` from the provider and
  does not import Order yet. Order imports only `LawfulClasses` and
  `Arithmetic`, so a Map to Order edge is acyclic.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- Order publishes the successor bound, and adds one public weakening law,
  `leq a b → leq a (Suc b)`, under the module's naming convention.
- The five private copies are deleted. Their uses go through the Order law,
  or through `proof refl for leq_nat` for Gcd's `leq_refl`.

## Acceptance

- **AC-1.**
  - Order, Map, Gcd and Parsing check.
  - `trusted_base()` is unchanged, with no axiom or postulate added.
  - A statement sweep over `catalog/`, `library/` and `crates/*/tests` finds
    no remaining private proof of any of the four statements above. Search
    by shape, `leq_nat x (Suc _)`, `leq_nat x (add x _)` and `leq_nat x x`
    equated with `True`, not only by name. Name each surviving hit and why
    it is not a copy.
- **AC-2 (controls).**
  - A typed consumer outside Order applies the weakening law and the
    successor bound at general arguments. Making either law private again
    reddens the consumer.
  - A false variant of the weakening law, the converse
    `leq a (Suc b) → leq a b`, is rejected.
- **AC-3.** `ds2_ord_nat_acceptance`, `map_build_acceptance`,
  `cat_gcd_acceptance`, `cat5_parsing_package` and `cc5_pretty_doc_acceptance`
  stay green.

## Stop conditions

- A copy whose use site needs a statement that differs from the Order law by
  more than argument order: keep the copy, and name it in the handoff.
- A cycle, or any new function, kernel change or trust change: stop to the
  Architect.
