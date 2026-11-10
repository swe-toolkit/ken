---
id: LANG-EFFECT-ROW-PHANTOM-TYPE-ARGUMENT
title: "type_carried_row charges every type argument of a checked type, so a scrutinee or lambda binder of type `Tag Box`, whose family never stores its parameter, inherits Box's Console row, and a pure callback applied in its arm is refused E0038. Charge a type argument only when its family's constructor fields can hold a value of it, on the binder planes and the annotation plane alike, so a phantom row argument is free and every held row is still charged"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Adversary report evt_8sz0sggh211c on LANG-EFFECT-ROW-CELL-SCRUTINEE-CARRIED-ROW (43a869435): a valid program is refused, carried in the L2 row. L2 objective: one resolution mode (operator 2026-09-25); Verify runs L2's effect-row work. Steward-filed per COORDINATION section 2."
---

# A phantom type argument carries no row

## Objective

A value's carried row is the rows it can hold. A type argument whose
parameter no constructor field mentions adds nothing, so a pure callback
read out of a `Tag Box` runs under the empty row. A type argument the
fields do hold is still charged, as today.

## Settled inputs (main `ea5842524`)

- **The over-charge.** `type_carried_row` (`elab.rs:18860`) walks both
  sides of every `Term::App` (`:18872`) and joins each head's
  `carried_ids` row, whatever position the argument fills.
- **Its consumers.** The scrutinee join `join_scrutinee_carried_row`
  (`:1503`), the lambda binder `push_lambda_binder` (`:3767`) and
  `let_latent` (`:18892`, joining at `:18911`).
- **The annotation plane.** `written_type_row` (`:18522`) joins both
  sides of an `RApp` (`:18532`), so `let t : Tag Box = ...` already
  refused before 43a869435. `let_latent` exists so the annotated and
  unannotated forms agree.
- **A family's own row** is `declared_type_carried_row` (`:18545`), the
  join of its constructor fields' written rows, registered by id at
  `modules.rs:4276` and `:4308`. A field that names a parameter
  contributes nothing there. So the argument walk is the only thing
  that charges `List Box`, and it has to stay for held parameters.
- **The repro** (Adversary, `ken run`, inline file, with an action and
  a `main` that prints `b`):

  ```ken
  data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
  data PBox = MkPBox (Unit -> HostIO APartial Unit)
  data Tag (a : Type) : Type where { MkTag : PBox -> Tag a }
  fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
  proc fire (u : Unit) : Unit visits [Console] =
    match MkTag Box (MkPBox quiet) {
      MkTag p ↦ match p { MkPBox g ↦ let ignored = g MkUnit in MkUnit } }
  ```

  It is refused E0038 at 43a869435 and prints `b` at its parent. The
  same program at `Tag Unit` prints `b` on both. The lambda-binder form
  and the nested-pattern form `MkTag (MkPBox g)` flip the same way.
- **The law.** Spec 36 §1.2 (`36-effects.md:79`): `g MkUnit` has the
  rows of `g`, `MkUnit` and `g`'s latent, all empty here.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One candidate. `type_carried_row` and `written_type_row` charge a type
argument only at a position its family's constructor fields can hold. A
head whose fields are not visible (an opaque, postulated or primitive
type) charges every argument, as today. The occurrence rule (direct,
nested, under a function domain or codomain, at an index) and where it
is computed are AC-0's.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Write the rule as one
  obligation over the checked family declaration. Name each position
  that stays charged, and why the rule cannot drop a row a value holds.
  Rule whether the annotation plane is fixed here or is a named
  refusal.
- **AC-1, the repro.** All four forms run and print `b`: scrutinee,
  lambda binder, nested pattern and annotated let, each against the
  `Tag Unit` control. The annotated form is the one AC-0 may exclude by
  name.
- **AC-2, held rows still charged.** In the same file, with
  `data Hold (a : Type) : Type where { MkHold : a -> Hold a }`, a
  `Hold Box` scrutinee whose arm runs the Box callback still refuses,
  with the E0087 carried-row refusal (Architect `evt_247ynwz3gmv50`). So
  does a `List Box` scrutinee, and so does each position AC-0 keeps
  charged. A candidate that charges nothing passes AC-1 and fails
  here.
- **AC-3, mutation.** Restore the unconditional argument walk: AC-1
  reddens and AC-2 stays green, reported per assertion (Steward
  `evt_7jd0chpyyh6yj`). A mutation that does not redden is reported as
  non-discriminating, not replaced.
- **AC-4.** Every committed effect-row test that passed before still
  passes, and full-workspace CI is green on the exact candidate.

## Stop conditions

- The rule needs a kernel change, or a family's constructor fields are
  not available where the walk runs.
- A charged row is dropped on any AC-2 row.

Checks: 4 fired: the obligation is the family's checked declaration,
which carries its constructor fields (`Decl::Inductive` constructors,
read by `declared_type_carried_row` at `:18545`). 5 fired: the row a
value can hold is a property of its constructors' field types, not of
its type's spelling. The plane that carries it is the family
declaration, not the applied type, so AC-0 computes from that. 8 fired:
AC-2 is the failing configuration's observation. A walk that charges
nothing passes AC-1, and only AC-2 refuses it.
