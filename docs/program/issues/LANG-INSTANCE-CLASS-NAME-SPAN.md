---
id: LANG-INSTANCE-CLASS-NAME-SPAN
title: "An instance declaration that names an absent class refuses as UnboundName, but the refusal spans the whole declaration, because Decl::InstanceDecl carries no span for its class name and resolve_class_ref is handed the declaration's span. Carry the class name's own span from the parser to the resolver, so the refusal points at the class name, and re-pin the I4 control from 0..37 to 9..20 deliberately"
status: merged
owner: language
size: S
tier: T2
gate: architect
depends_on: [LANG-DELETE-LEGACY-RESOLUTION]
blocks: []
github: null
origin: "LANG-DELETE-LEGACY-RESOLUTION I4 follow-on. Architect span ruling evt_5qq7etvb52p2h (pin the measured 0..37 with a GAP; localization is its own change, modelled on LetBinding.name_span); Steward recut evt_3ptq4fm5qdpxa; Research advisory evt_2g6erj8mptwrr. L2 successor on I4's landing. Steward-filed per COORDINATION section 2."
---

# A missing instance class is reported at the class name

## Objective

`instance MissingFlag a { tag = True }` refuses as `UnboundName` naming
`MissingFlag`, and the refusal's span is the class name's own token,
`9..20`, not the whole declaration, `0..37`.

## Settled inputs (main `fcd7e8156`; anchors re-checked at release)

- **The AST.** `Decl::InstanceDecl` (`ast.rs:395`) holds `class_name:
  String` and one `span` for the whole declaration. `LetBinding.name_span`
  (`ast.rs:658`) is the in-repo template for a separate name span.
- **The parser.** `parse_instance_decl` (`parser.rs:3312`) builds the node
  at `:3330`.
- **The resolver.** `synthesized_dictionary_name` (`modules.rs:5338`)
  calls `resolve_class_ref` (`:1744`) with the declaration span at
  `:5346`. Its callers are `:5733` and `:6835`.
- **The pin.** `generic_instance_missing_class_refuses_in_declaration_entry`
  (`modules.rs:7305`) asserts `(0, SOURCE.len())` and names this change as
  its GAP.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: AC-3 ruling (2026-11-21)

Language Leader `evt_5jcy6rabqrhpa` found one settled input false. `:5391`
resolves a public `ClassDecl` export, not a constraint. Both constraint
rewrites pass `&rdecl.span` (`:4277`, `:4439`), and `InstanceConstraint`
has no class-name span. Steward ruling `evt_5pbprtbjjwmqy`: the constraint
AST is not extended here. AC-3 below is replaced by the amended AC-3. A
constraint class-name span is carried, not authorized.

## Scope

`crates/ken-elaborator` only: the AST field, the parser, the construction
and pattern sites that the new field forces, the resolver call, and the
pin. No resolution rule, error variant or spec change.

## Acceptance

- **AC-1.** Through `elaborate_decl_v1`, the I4 pin's source refuses as
  `UnboundName { name: "MissingFlag" }` with span `9..20`. Through
  `elaborate_file_v1`, the same declaration inside a module refuses at the
  class token's offset in that file. The pin's GAP note is removed.
- **AC-2 (QA mutation).** Passing the declaration span to
  `resolve_class_ref` again, with everything else unchanged, reddens both
  AC-1 rows. Restore it byte-identically.
- **AC-3 (amended).** A missing class in a `where` constraint still
  refuses as `UnboundName` naming that class. Its span is the base
  behaviour, measured on `69458aa89` and pinned as current behaviour with
  a GAP note. AC-2's mutant leaves this row green, and
  `generic_instance_head_binder_has_checked_identity_after_child_replay`
  passes unchanged.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The change needs a resolution-rule or error-variant change, or touches
  `spec/`.
- A consumer outside `crates/ken-elaborator` matches `InstanceDecl` by
  its full field list and cannot take the new field mechanically.

Checks: 4 fired. I read the I4 pin, `resolve_class_ref`'s call sites and
the parser's construction site on `fcd7e8156`; the declaration span is
the only span the resolver sees. 8 fired. AC-2's mutant passes AC-3 and
reddens only the AC-1 rows, so those rows are what pin the localization.
