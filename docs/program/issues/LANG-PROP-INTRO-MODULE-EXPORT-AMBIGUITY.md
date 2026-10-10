---
id: LANG-PROP-INTRO-MODULE-EXPORT-AMBIGUITY
title: "A prop intro P.pi and a same-named module export (module P { pub const pi }) both bind the spelling P.pi. In one order the export silently displaces the intro, leaving the canonical intro path unaddressable; in the other the program is refused as a duplicate proof name. Spec 33 section 3.3 requires AmbiguousReference, as data constructors already get"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-FORWARD-PROP-INTRO-EDGE]
blocks: []
github: null
origin: "Adversary M8 finding evt_3457d4adns6w7 on 09bb394d1 (LANG-FORWARD-PROP-INTRO-EDGE). Not a regression: identical rows on 4329188c4. Correctness, order-dependent; proof uses fail closed in the kernel, a non-proof use binds the export. Steward-filed per COORDINATION section 2."
---

# A prop intro and a module export of the same path are ambiguous

## Objective

A written path that denotes both a prop's intro and a module export is
`AmbiguousReference`, in either declaration order, exactly as a data
constructor and a module export already are.

## Settled inputs (Adversary `evt_3457d4adns6w7`, at `09bb394d1`)

- **The rows** (`ElabEnv::new()` + `elaborate_file`):
  - `prop P : Omega where { pi : P } module P { pub const pi : Int = 0 }`
    is admitted, and `globals["P.pi"]` is the module const.
  - Adding `theorem t : P = P.pi` gives `KernelRejected TypeMismatch`
    (`P` against `Int`); adding `const z : Int = P.pi` is admitted.
  - The reverse order is refused with `duplicate proof name 'P.pi'`.
  - Nested in `module M`, consumed as `M.P.pi`: the export wins.
- **The data control.** `data D = C module D { pub const C : Int = 0 }
  const z : Int = D.C` is refused with `AmbiguousReference { sources:
  ["module D.C", "type D.C"] }`.
- **The mechanism.** The intro helper refuses only when it arrives
  second (`elab.rs` `elaborate_checked_theorem`, `globals.contains_key`).
  A module export arriving second displaces it as a lawful Source over
  Source insert. The module-versus-type tie-break in
  `resolve_constructor_path` (`modules.rs`) covers only data
  constructors; `{family}.{intro}` has none.
- **The rule.** Spec 33 section 3.3: a path that could denote both a
  module export and a type's constructor is `AmbiguousReference`, and
  neither lookup order breaks the tie. A prop intro is the prop's
  constructor (section 8.4, as settled for LANG-FORWARD-PROP-INTRO-EDGE).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A reference to a path that is both a prop intro and an authorized module
export resolves to `AmbiguousReference`, in both declaration orders, and
the intro stays addressable when there is no export. The Architect
confirms at review whether the decision sits in the reference choke or
at declaration.

## Acceptance

- **AC-1.** The term-position rows above (both orders, top level and
  nested) give `AmbiguousReference` with the module and type sources.
- **AC-2 (controls).** The data control keeps its result, and
  `lang_forward_reference_across_data_export` and
  `lang_qualified_constructors` stay green. The distinct-owner control
  `modules.rs` `file_prop_intro_reexports_retain_checked_helper_identity`
  stays green unedited: a later, distinct owner's `module A { module
  HasProof { pub const intro } }` still replaces the flat
  `globals["A.HasProof.intro"]`, and the file-owned intro is still reached
  by checked identity through its imports (`evt_7yabeg715nmtq`).
- **AC-3 (mutation, QA).** Removing the intro arm of the tie-break returns
  the export-first-wins row to admitted.

## Stop conditions

- A catalog or test consumer resolves a written path, in a scope where
  both the intro and the export are visible, to the export. Replacement
  of a flat global by a distinct later owner is not this population; it
  is the AC-2 control above. A mechanism that reddens that control is a
  stop to the Architect.
- Any kernel, `trusted_base()` or spec change.

Not in scope: pattern position, and intro paths reached through an import
or a hidden member.
