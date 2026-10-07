---
id: LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT
title: "Spec 33 §8.4 delivers forward references across all top-level declarations in a scope, but the module loader groups definitions only within a maximal run that any export or data declaration ends, so a definition cannot name a sibling defined after an intervening data or export. Deliver scope-wide forward references"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_4js9vdbbcgmb2 on CAT-PARSING-PARSER-LAWS (foundation evt_31ye7pqhawygw): a source-reachable spec/implementation gap, found when a theorem placed before `export`/`data BoolExpr` could not name its later helpers. Steward-filed per COORDINATION section 2."
---

# Forward references span the whole scope

## Objective

As spec 33 §1 and §8.4 state, every top-level definition in a module scope
takes part in one dependency-ordered call-graph pass, so a definition may name
a sibling defined later in source whatever lies between them.

## Settled inputs (Architect `evt_4js9vdbbcgmb2`, read at `d97d06028`)

- **Spec.** 33 §8.4 point 1 says "All top-level declarations in a scope ... are
  admitted through one shared, dependency-ordered call-graph pass", and that
  this ordering is what delivers forward references. 33 §1 (`:23`) says "All
  top-level definitions are mutually recursive within a module".
- **Implementation.** `modules.rs:4040` takes `decls[i..run_end]` while
  `is_recursive_candidate` (`:2918`: view, let, theorem, axiom and attached
  proof, plus fixity) holds. Any other declaration, `export` and `data`
  among them, ends the run, and the call graph is built per run.
- **Witness.** In `Parsing.ken.md` at `da21f556c`, a theorem placed before
  `export`/`data BoolExpr` (`:481-487`) that names a helper defined after it
  fails with `UnresolvedCon`. The same names resolve when no `export` or
  `data` lies between them.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 (Architect, at kickoff)

The Architect rules the mechanism before code, for example `data` and `export`
nodes in the scope's dependency order, or a run that spans them. The
deliverable is the spec's behaviour, not a particular mechanism.

## Deliverable

A definition may reference a later sibling across an intervening `data` or
`export` declaration. Each dependency, including a later `data` type that
the definition names, is elaborated before the definition.

## Acceptance

- **AC-1.** A module with `fn a = b`, then `data D`, then `fn b` checks, as
  does the same with `export` in place of `data`. A row in which `a`'s type
  names a `data` declared after `a` checks too.
- **AC-2 (fence).** Rejection is unchanged for a genuine cycle that SCT
  rejects, and for a reference to a name that is not in scope. The same
  modules with no intervening declaration give the same verdicts as before.
- **AC-3.** The catalog census has no verdict change, the
  `lang_mod_strict_resolution_d0` and module-loader targets stay green, and
  `trusted_base()` is unchanged.

## Stop conditions

- The mechanism needs a spec change (for example, a forward reference to a
  `data` constructor is not well-defined under §8.4). Stop to the Architect.
- Any kernel change.
