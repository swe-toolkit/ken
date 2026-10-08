---
id: VERIFY-GLOBALS-IDENTITY-CHECKED-INSERT
title: "A data constructor spelled like an instance dictionary (data D where { Lbl_instance_A : D } beside instance Lbl A) is admitted in both orders, and the later one overwrites the other in the flat elab.globals key, because constructors enter globals outside the elaborate_checked_as identity guard. Bind every declaration-identity key to at most one checked GlobalId through one checked insert that refuses with DeclarationIdentityCollision"
status: active
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

1. **D0 census: done** (verify-leader `evt_1gk40emxp4jpj`,
   `evt_6vzwnjdvav2mw`; ruled `evt_5tabgdgdagjcz` on `48f4bacb2`). The
   census found 94 production sites and 59 test-only sites. A physical
   insert can be lawful on one input and colliding on another, so the class
   belongs to the (prior binding, new binding) pair, not to the line. Every
   production source declaration passes one of three windows,
   `with_owner(.., with_env_mark_rollback(..))`, at `modules.rs:3386`,
   `:4526` (spaces) and `:4802` (mutual groups). Prelude-phase sites,
   `modules.rs:318`, `declare_postulate_raw` and the 59 test writes stay
   unrouted, as classified.
2. **The repair, in `modules.rs`** (the code is in `evt_5tabgdgdagjcz`).
   - One helper, `with_declaration_identities`, replaces the three window
     compositions. No `globals` signature changes.
   - `MintedSpelling` becomes an enum: `Dictionary`, `LawField` and
     `SpaceOperation`. Each binding carries an `IdentityProvenance` of
     `Source` or `Minted(..)`. The criterion (Architect
     `evt_mjer0v3wp8jx`, spec 33 §3.1): Minted is a key some consumer
     resolves by its flat spelling (dictionary owners, law fields, space
     operations); Source is a key every consumer reaches through an
     owner's interface (declaration names, constructors, prop intros).
   - An exhaustive enumerator, `declared_identities(rdecl, owner, minted)`,
     with no `_` arm, lists the keys each declaration binds: constructors
     and each prop intro helper (`format!("{}.{}", rdecl.name,
     intro.name)`, byte-identical to the key at `elab.rs:17827`) as
     Source; law fields and space operations as Minted. The Architect's
     sweep on `c51e9351f` found no other undeclared key family.
   - The window refuses with `DeclarationIdentityCollision` when a
     displacement is not lawful: Source over Source and an equal Minted
     pair are lawful, a mixed pair is not, and `same_instance_key` is kept
     unchanged. It refuses any undeclared new key or unbound declared key
     with a typed Internal error. On failure it restores a displaced
     predecessor.
   - NAMED-HEAD's owner-only guard and its restore at `:3410` are deleted;
     the window subsumes them.
   - **The completeness check is the authority on the enumerator.** If a
     row refuses with "undeclared identity keys" or "was not bound", stop
     and report the key, the `RDeclKind` and the inserting site; the
     Architect rules that arm's provenance. Never add a key to an arm on
     your own.

## Acceptance

- **AC-1.** The witness is refused in both orders with the typed error
  naming the instance and the constructor. So are:
  - R2 with `instance Lbl A` before `const Lbl_instance_A`, in one-, two-
    and three-file layouts;
  - a class, data or const declared after a minted dictionary at its
    spelling;
  - `law Foo { a : .. }` against `const Foo_a`, in both orders;
  - a space operation against a same-spelled const, in both orders.

  A prop with one intro elaborates: its helper key is bound to the
  theorem's GlobalId, `declaration_descriptions` records "introduction
  intro of prop ...", and `minted_spellings` has no entry for it. A helper
  arriving over an existing key keeps the baseline "duplicate proof name"
  refusal.
- **AC-2 (control).** VERIFY-NAMED-HEAD's suite stays 7/7,
  `lang_instance_registry_identity_key` stays 5/5, and
  `modules::namespace_effect_tests` is 77/77 with no test edited. The full
  `scripts/ken-cargo test -p ken-elaborator --lib` passes and is part of
  the QA gate. No catalog package is refused or changes hash.
- **AC-3 (mutation, QA).** Making `(Minted(_), Source)` lawful returns the
  constructor-after-instance witness to admitting with the key overwritten,
  and reddens AC-1, including the law-field and space-operation rows.
- **AC-4.** A declaration that displaces a key and then fails kernel
  checking leaves its predecessor bound. Mutation: deleting the restore loop
  reddens AC-4.
- **AC-5 (mutation, QA).** Dropping the constructor keys from the DataDecl
  arm makes every data declaration with constructors refuse with exactly
  "declaration bound undeclared identity keys", never a silent admit.
- **AC-5b (mutation, QA).** Moving the Prop arm back into the `source()`
  group reddens the three namespace rows with exactly `declaration bound
  undeclared identity keys ["A.HasProof.intro"]`.
- **AC-6 (cost; Architect `evt_46s0nyx1j09sg`).** The added elaboration
  work (`WINDOW_NS + ENUM_NS`, measured instrumented on
  `map_build_acceptance`) is at most 5% of the base median wall (7.90s of
  158.01s). The interleaved base and candidate walls are recorded, not
  gating; their base-only spread (16.3%) cannot resolve 5%. **Stop** above
  the bound. The scan's O(declarations × globals) cost is a recorded carry.

## Stop conditions

- A catalog or corpus package is newly refused or changes hash: stop with
  the list.
- The enumerator stop in Deliverable item 2, or the AC-6 cost stop.
- Any kernel, `trusted_base()` or spec change.
