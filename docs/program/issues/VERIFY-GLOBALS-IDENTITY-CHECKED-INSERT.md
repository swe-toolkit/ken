---
id: VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT
title: "A data constructor spelled like an instance dictionary (data D where { Lbl_instance_A : D } beside instance Lbl A) is admitted in both orders, and the later one overwrites the other in the flat elab.globals key, because constructors enter globals outside the elaborate_checked_as identity guard. Bind every declaration-identity key to at most one checked GlobalId through one checked insert that refuses with DeclarationIdentityCollision"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-NAMED-HEAD-INSTANCE-IDENTITY]
blocks: []
github: null
origin: "VERIFY-NAMED-HEAD-INSTANCE-IDENTITY AC-0 item (iii) (verify-implementer evt_4f97s0b2srg5h), measured at b5ba6619c. Architect recommendation evt_60ttbeh7wp6j6: one successor node over the globals insert sites, not a per-path guard. Steward-filed per COORDINATION section 2."
---

# One checked insert for declaration identities

## Objective

A declaration-identity key in `elab.globals` (a declaration name,
constructor, class, field accessor or dictionary) is bound to at most one
checked GlobalId per elaboration. A second, distinct binding is refused
with `DeclarationIdentityCollision`, which VERIFY-NAMED-HEAD introduces.

## Settled inputs (at `b5ba6619c`)

- **The witness** (verify-implementer `evt_4f97s0b2srg5h`).
  `class Lbl carrier { mark : carrier -> Int }`, `data A = MkA`,
  `instance Lbl A { mark = \x. 0 }` and
  `data D : Type where { Lbl_instance_A : D }` admit in both orders. The
  later registration overwrites `elab.globals["Lbl_instance_A"]`: the
  survivor is the constructor (`g652`) when the instance comes first, and
  the instance (`g652`) when the constructor comes first.
- **The mechanism** (Architect `evt_60ttbeh7wp6j6`). Constructors are
  inserted inside `elaborate_rdecl` and never pass the
  `elaborate_checked_as` owner key, so the named-head guard cannot see
  them. It is the same predicate on another path: a distinct checked
  GlobalId overwrites a flat spelling key (check 10).
- **The fan-in.** 27 `globals.insert` sites on the base: 22 in `elab.rs`
  and 5 in `modules.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (census, first act).** Classify every `globals.insert` site as
   (a) an identity binding, (b) an intentional rebind of the same
   GlobalId, or (c) an intentional shadow or overwrite, with the reason it
   is lawful (prelude confinement, property-class duplicates, retain or
   rollback). The Architect rules the classification.
2. **The repair.** A single checked insert used by every (a) site, with (b)
   and (c) left explicit. No `_` or default arm decides the class.

## Acceptance

- **AC-1.** The witness is refused in both orders with the typed error
  naming the instance and the constructor. So are the rows that
  VERIFY-NAMED-HEAD leaves admitted (Architect `evt_7r6p7bj8wj22y`): R2
  with `instance Lbl A` before `const Lbl_instance_A`, in one-, two- and
  three-file layouts, and a class, data or const declared after a minted
  dictionary at its spelling.
- **AC-2 (control).** VERIFY-NAMED-HEAD's R1 and R2 rows still refuse, its
  non-colliding controls keep their results, and no catalog package is
  refused or changes hash.
- **AC-3 (mutation, QA).** Routing the constructor site around the checked
  insert returns the witness to admitting with the overwritten key.

## Stop conditions

- A catalog or corpus package is newly refused or changes hash: stop with
  the list.
- Any kernel, `trusted_base()` or spec change.
