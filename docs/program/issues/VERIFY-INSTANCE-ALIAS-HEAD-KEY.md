---
id: VERIFY-INSTANCE-ALIAS-HEAD-KEY
title: "An instance declared at a transparent def alias (def N2 = Nat; instance E N2) is keyed on the alias's own global id, not on head(A) after unfolding, so instance E N2 and instance E Nat are both admitted with no OverlappingInstances, and where E Nat misses the alias instance with NoInstance. Key instance registration, overlap and resolution on the unfolded head"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary M8 finding evt_2em0e8ma3sgwb on b5ba6619c (VERIFY-STRUCTURAL-HEAD-INSTANCE-IDENTITY); pre-existing, not introduced by that merge. Spec 39 sections 6.1-6.2 key the instance registry on head(A), the outermost type constructor. Steward-filed per COORDINATION section 2."
---

# An instance at an alias is keyed on the head it unfolds to

## Objective

Instance registration, the overlap check and resolution agree on `head(A)`
after transparent `def` aliases unfold. One class at one head has at most
one instance, however its head is spelled.

## Settled inputs (Adversary `evt_2em0e8ma3sgwb`, at `b5ba6619c`)

- **The witness.** `class E carrier { label : String }`, `def N2 = Nat`,
  `instance E N2 { label = "a" }` and `instance E Nat { label = "b" }`
  are both admitted, as `E_instance_N2` and `E_instance_Nat`, with no
  `OverlappingInstances`. `fn up (h : Nat) : N2 = h` and its reverse both
  check, so `N2 ≡ Nat`.
- **Resolution follows the spelling.** With only `instance E N2`
  registered, `const lb : String where E Nat = d.label` fails with
  `NoInstance { ty: "Nat" }`.
- **Other shapes** that admit both instances: `def F = Nat -> Nat`
  against `E (Nat -> Nat)`; `E (N2 -> Nat)` against `E (Nat -> Nat)`;
  across modules, `module B { import A (E, T) def T2 = T instance E T2 }`
  beside A's `E T`.
- **The site.** `instance_head_key` (`crates/ken-elaborator/src/elab.rs:15382`)
  resolves the alias to `Global(<def id>)` from the checked global, or
  `core_type_head_id` on the core before whnf. The overlap test at
  `:15457-15458` therefore never compares the alias with its target.
- **The rule.** Spec 39 §6.1 and §6.2: the registry is keyed on `head(A)`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (measure only).** Census the catalog and corpus instances whose
   head is a transparent alias, and report the dictionary name, owner and
   package hash each would get when keyed on the unfolded head. The
   Architect rules the key from the D0, including what the importable
   dictionary name of an alias-headed instance becomes.
2. **The ruled repair**, applied to registration, overlap and resolution.

## Acceptance

- **AC-1.** In every witness shape, the second instance is refused with
  `OverlappingInstances`. With only the alias instance registered,
  `where E Nat` resolves to it.
- **AC-2 (control).** Instances at distinct heads, the structural-head and
  named-head rows, and every catalog package keep their results, or each
  moved hash is listed with its cause and ruled.
- **AC-3 (mutation, QA).** Keying on the head before whnf again reddens
  AC-1.

## Stop conditions

- A catalog or corpus package is newly refused: stop with the list.
- Any kernel, `trusted_base()` or spec change.
