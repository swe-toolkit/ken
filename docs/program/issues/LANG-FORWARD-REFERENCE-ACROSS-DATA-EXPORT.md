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

## AC-0 (Architect `evt_7ahyyb91bzt08`, measured at `440b216f1`)

It is ordering, not name resolution. `prebind_scope_declarations`
(`modules.rs:3478`) already binds every qualifiable name scope-wide,
constructors included. The run at `:4214` ends at the first declaration that
is neither `is_recursive_candidate` nor fixity, so `fn a = b` is elaborated
before `fn b` exists in `elab.globals`. No spec change: a constructor mention
is an edge to its `data` node (33 §8.4 point 1), and induction-recursion is
refused (point 4).

## Deliverable

1. **Segments, not runs.** A scope splits only at declarations that change
   resolution state or own side tables: `import`, `module`, `boundary`,
   `space`, `class`, `instance`, `record`, `law`, `derive`, `foreign`,
   `temporal`, `prove`.
2. **Nodes** are the declarations elaborated through `resolve_scoped_decl`
   plus `elaborate_checked`: `is_recursive_candidate` and the qualifiable
   kinds of the `other` arm (`:4467`), `DataDecl`, `ExplicitDataDecl`,
   `TypeAlias` and `PropDecl`. Each keeps its existing elaboration and
   publication code: a data node registers constructors and `pub` members as
   `:4467-4530` does, and definition SCCs keep the singleton, recursive and
   mutual paths and the `mixed fn/const and proof` guard.
3. **Edges** come from one walker, `rdecl_mentions_name`, over body, type,
   `requires`, `ensures` and the kind's own telescopes, with no `_ =>` arm:
   a barrier kind reaching the node graph is `unreachable!`. A mention of a
   constructor is an edge to its owning data node, through a
   `ctor name -> data node index` map built from the segment's data nodes.
4. **Order.** Condense with the existing `scc_membership` and process
   dependency-first, ties broken by least textual index, so all-backward
   edges give exact textual order. An SCC containing a `data`, `TypeAlias`
   or `Prop` node together with any other member is refused: "a type
   declaration cannot share a dependency cycle with another declaration". A
   `data` self-edge is not a cycle here.
5. **`export` is deferred, not a node.** Every `ExportDecl` in the segment
   applies in textual order after the segment's nodes elaborate and before
   the `pub` publication loop (`:4380`).

Residual: the barrier kinds of point 1 still stop forward references across
them. That pre-existing implementation gap is measured by AC-4, not closed
here.

## Acceptance

- **AC-1.** A module with `fn a = b`, then `data D`, then `fn b` checks, as
  does the same with `export` in place of `data`. A row in which `a`'s type
  names a `data` declared after `a` checks too. (d) An `export b` written
  before `fn b` checks. (e) A later `data D` whose constructor `C` is named
  in an earlier definition's body checks.
- **AC-1b.** A type-declaration cycle (`data D = MkD T` with
  `const T = … D …`) is refused with the point-4 message.
- **AC-2 (fence).** Rejection is unchanged for a genuine cycle that SCT
  rejects, and for a reference to a name that is not in scope. The same
  modules with no intervening declaration give the same verdicts as before.
- **AC-3.** The catalog census has no verdict change and byte-identical
  checked-core package hashes; the `lang_mod_strict_resolution_d0` and
  module-loader targets stay green; `trusted_base()` is unchanged.
- **AC-4 (census).** List every catalog and corpus forward reference still
  blocked by a barrier kind, by kind.
- **Mutations.** M-textual (nodes in source order) reddens AC-1(a);
  M-no-data-node (data positional) reddens AC-1(c) or (e);
  M-export-positional reddens AC-1(d); M-no-tiebreak (arbitrary SCC order)
  reddens AC-3.

## Stop conditions

- The mechanism needs a spec change (for example, a forward reference to a
  `data` constructor is not well-defined under §8.4). Stop to the Architect.
- Any kernel change.
