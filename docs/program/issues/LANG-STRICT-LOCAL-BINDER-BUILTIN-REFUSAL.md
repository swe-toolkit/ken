---
id: LANG-STRICT-LOCAL-BINDER-BUILTIN-REFUSAL
title: "Spec 33 forbids any binder from introducing a built-in name, local binders included, but Strict refuses one only at module scope (E0058 from reject_decl_prelude_bindings); every lambda, let, parameter, pattern, Pi/Sigma, where and record-field or class-method binder is pushed through Scope::push_spanned with no check, so `fn lexical (pair_fst : Bool)` is accepted and shadows the floor. Refuse a built-in name at every binder introduction under Strict, migrating the source binders that use one"
status: ready
owner: verify
size: L
tier: T1
gate: architect
depends_on: [LANG-STRICT-ISOLATED-ROUTES]
blocks: []
github: null
origin: "Operator 2026-09-25: binding a prelude name anywhere, local binders included, is an error (L2 objective, one resolution mode). spec/30-surface/33-declarations.md:525-541 states it; LANG-PRELUDE-FLOOR-FIFTEEN (merged) installed the module-scope half. Verify runs L2's Strict-surface repairs. Steward-filed per COORDINATION section 2."
---

# No binder overrides a built-in

## Objective

Under Strict, every binder that introduces a name in the built-in set B
is a hard error at its introduction, as spec 33 states: lambda, `let`,
function, constructor and record parameters, Π and Σ, match patterns,
`eqn`, `where` dictionary binders, record fields and class methods.
Module-scope refusals keep their current behaviour. Sources that bind a
B name locally are migrated so every suite that checked before still
checks.

## Settled inputs (main `5a3c931fe`)

- **The rule.** `spec/30-surface/33-declarations.md:525`-`541`, "No
  binding of a built-in name": "Lexical shadowing applies only to names
  outside B; an inner binder cannot override B."
- **The module-scope half.** `reject_decl_prelude_bindings`
  (`modules.rs:5001`) calls `reject_prelude_binding` (`:4766`), which
  returns `AmbiguousReference` (E0058) for a name in
  `prelude_binding_names`: the floor, its constructors, the Pair
  companions, `Proved` and the State bindings (`:709`-`:759`). It covers
  top-level definitions, data, record and class names, constructors,
  attached proofs, instance dictionaries and import aliases. It does not
  check record fields, class methods or law fields.
- **The local half is absent.** All local binders go through
  `Scope::push`, `push_spanned` and `push_pattern_alias`
  (`resolve.rs:568`-`583`), which take no mode and check nothing.
  `resolve.rs` has no `ResolutionMode` and no floor reference.
- **Controls that invert.** `lang_mod_pair_floor_realization.rs:458`
  expects `fn lexical (pair_fst : Bool)` to be accepted under Strict, and
  `modules.rs:6852` expects `fn keep (Proved : Nat)` to be accepted
  through `elaborate_file`. The module-scope pin is
  `lang_prelude_floor_fifteen.rs:231`.
- **The base.** Routes Inc 3b edits `modules.rs`, so this node starts on
  the main that carries Inc 3b.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One candidate: the Strict binder check at every introduction form, the
error it raises, the migrated sources, and the inverted controls.

## Acceptance

- **AC-0 (D0, Architect, before product code).** Install the check in
  scratch and count the binder reach by form and by source root
  (`catalog/`, `examples/`, `conformance/`, `ken-cli` fixtures, every
  `crates/*/tests` and `r_layer_tests` suite). The two controls above
  must appear in the count. Then rule:
  - whether B for binders is `prelude_binding_names` or the 51-name
    floor alone, which decides `pair_fst`;
  - the error variant and code;
  - where the check sits: in `Scope`, or in a binder walk in
    `resolve_raw_scoped_decl` (`modules.rs:4743`);
  - the rename for each migrated binder.
- **AC-1.** Under Strict, one named row per binder form refuses a B name
  at the binder's span. The row for `fn lexical (pair_fst : Bool)` is
  refused if AC-0 puts companions in B.
- **AC-2.** A local binder spelled outside B still shadows lawfully, and
  qualified use of a floor name is not a binding. Every module-scope
  E0058 row still passes.
- **AC-3.** Removing the binder check returns the AC-1 rows to
  acceptance.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The reach includes a catalog package whose public law or export name
  would change. Post it with the count; renaming a public identity is a
  separate node.
- A kernel change, or a change to Legacy resolution.

Checks: 2 fired: the fan-in to binder introduction was read at
`5a3c931fe`: the module-scope refusal and all three local `Scope` push
entry points, with no check at any of them. 3 fired: AC-0 counts the
reach over every source root before any rename is chosen, since binders
are spelled in catalog, conformance and test sources alike. 8 fired:
AC-3's revert returns the binder rows to acceptance, which is the
failing configuration itself.
