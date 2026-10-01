---
id: CAT-NAT-SUB-ADD-CANCEL
title: "Gcd re-proves ten laws privately, including that sub cancels add under leq_nat, because Order does not publish the cancel law and Arithmetic keeps mul_add_distrib_r private. Publish add_cancel for sub in Order and mul_add_distrib_r in Arithmetic at zero TCB, and retire Gcd's ten private copies"
status: ready
owner: foundation
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_373qe5k8n5626 on Steward evt_68dx3wpvzgtbn: next L3 proof-backfill obligation (operator 2026-09-13), catalog §2a. Steward-filed per COORDINATION section 2."
---

# Order publishes the cancel law; Gcd reuses public tools

## Objective

A client proves `add (sub b a) a = b` from `leq_nat a b` with one public
Order law. Gcd keeps no private copy of a law that a package publishes.

## Settled inputs (Architect `evt_373qe5k8n5626`, read at `5d5e7bf02`)

- **Order** (`Data/Numeric/Nat/Order.ken.md`) has `pub fn sub` (`:115`),
  `pub proof zero_left for sub` (`:133`) and `pub proof saturates for sub`
  (`:139`). It imports `add` (`:39`) and re-exports `IsTrue` and `leq_nat`
  (`:37`). The name `add_cancel` is free.
- **Arithmetic** has `zero_l`, `suc_l`, `assoc` and `comm` for `add`
  (`:32-55`). `theorem mul_add_distrib_r` (`:153`) is private, and its
  statement is word for word Gcd's `mul_add` (`:452`).
- **Transport** has `pub theorem cong`, `sym` and `trans` (`:53`, `:71`,
  `:78`), and is already in Order's closure through Arithmetic `:16`.
- `IsTrue b` is a transparent alias for `Equal Bool b True` (LawfulClasses
  `:54`).
- **Kernel-checked bodies already exist.** The new law's body is Gcd's
  `add_sub_cancel_leq` (`:321-332`).

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

1. **Order.** Add `pub proof add_cancel for sub (a : Nat) : (b : Nat) →
   IsTrue (leq_nat a b) → Equal Nat (add (sub b a) a) b`, as in the ruling,
   directly after `pub proof suc_decreases for sub`. Add `import
   Core.Logic.Transport (cong)`. The new text must be kenfmt-canonical
   (`kenfmt_signature_layout.rs:129` byte-pins Order).
2. **Arithmetic.** `theorem mul_add_distrib_r` becomes `pub`, with name and
   body unchanged.
3. **Gcd.** Delete the ten private copies and rewrite their call sites:
   - `add_sub_cancel_leq` becomes `(proof add_cancel for sub)`;
   - `mul_add` becomes `mul_add_distrib_r`;
   - `add_comm`, `add_zero_left`, `add_assoc` and `add_suc_left` become
     Arithmetic's `comm`, `zero_l`, `assoc` and `suc_l` for `add`;
   - `sub_zero_left` becomes `(proof zero_left for sub)`;
   - `trans`, `sym` and `cong` come from Transport.

   The imports become `Core.Logic.Transport (cong, sym, trans)` and
   `Data.Numeric.Nat.Arithmetic (add, mul, mul_add_distrib_r)`. Keep
   `subst`, `BoolView`/`bool_view`, `leq_not_flip` and the Gcd-specific
   glue. The prose at `:10-12` and `:562-567` names the reused laws.

Scope:

- `catalog/packages/Data/Numeric/Nat/Order.ken.md`;
- `catalog/packages/Data/Numeric/Nat/Arithmetic.ken.md`;
- `catalog/packages/Algorithm/Numeric/Gcd.ken.md`;
- `crates/ken-elaborator/src/r_layer_tests/cat_order_pub_export.rs`;
- `crates/ken-elaborator/tests/cat_gcd_acceptance.rs`: re-point the
  `Gcd.mul_add` lookup (`:134-136`) to `Arithmetic.mul_add_distrib_r`,
  keeping its assertions, and extend the duplicate list at `:109`.

## Acceptance

- **AC-1.** `ken check` passes on the three packages. `add_cancel for sub`
  and `mul_add_distrib_r` are public.
- **AC-2 (falsifiers, committed as test rows).**
  - In `cat_order_pub_export.rs`, a clean-environment client imports only
    `Order (sub, leq_nat, IsTrue)` and `Arithmetic (add)`, and closes the
    law by `(proof add_cancel for sub) a b h`. It fails on `5d5e7bf02`
    because the name is absent.
  - A committed mutant row concludes `Equal Nat (add (sub a b) b) a` and is
    rejected with a typed `TypeMismatch`, not by name resolution.
  - `trusted_base()` equality is asserted across the Order load.
  - In `cat_gcd_acceptance.rs`, each of the ten retired names is asserted
    absent from `Algorithm.Numeric.Gcd`. That row is red on base.
- **AC-3.**
  - `trusted_base()` is unchanged.
  - The catalog census is byte-identical except for the three packages.
  - These stay green: the Order, Arithmetic and Gcd acceptance targets,
    `nat_arithmetic_laws_acceptance.rs`, `kenfmt_signature_layout.rs` and
    `rosetta.rs`.

## Stop conditions

- Any new primitive, postulate or axiom, or an import cycle.
- A consumer of a retired name that was not counted: stop and name it.
