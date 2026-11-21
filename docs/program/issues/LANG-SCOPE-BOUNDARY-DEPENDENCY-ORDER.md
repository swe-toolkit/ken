---
id: LANG-SCOPE-BOUNDARY-DEPENDENCY-ORDER
title: "Spec 33 elaborates a scope's declarations in one dependency-ordered pass, exactly as if written flat, but the elaborator orders only runs of segment members, so a declaration that references a later one across a module, class or import boundary refuses UnresolvedCon, at the root and from an inline child. Bring those boundaries into the one dependency-ordered pass, refusing an unorderable cycle by name, so forward references resolve to checked identities as spec 33 rules"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-INLINE-MODULE-ENCLOSING-SCOPE]
blocks: []
github: null
origin: "Spec Leader evt_5bnakm62drgdt (Q1, arm S-b) on Architect D0 evt_6h2cebbwrsmbs for LANG-INLINE-MODULE-ENCLOSING-SCOPE: the forward-reference repair is outside that frame and is recut here. Spec contract evt_318gqrbkefpmt. Steward-filed per COORDINATION section 2."
---

# Forward references cross module, class and import boundaries

## Objective

Under Strict, a declaration resolves a later declaration of the same scope
or of an enclosing scope to its checked identity, across intervening
`module`, `class` and `import` declarations, as spec 33 §3.1 and §8.4
rule. A dependency cycle the pass cannot order refuses with a named
error. No reference resolves by spelling through `globals`.

## Settled inputs (main `9bde60e21`)

- **The contract.** Spec 33 §3.1 (`spec/30-surface/33-declarations.md:94`)
  elaborates a module "exactly as if written flat", and §8.4 (`:1461`)
  orders all top-level declarations of a scope in one dependency-ordered
  pass. Spec Leader `evt_5bnakm62drgdt` rules that the "before or after
  the child" contract binds, so the measured refusals are an
  implementation gap.
- **The measured gap (Architect D0, base `8cf27bd2c`).** A root `fn`
  calling a later `fn` across a `module` (r3) or a `class` (r4) refuses
  `UnresolvedCon`, and a root `fn` using a later class (r1) refuses
  `UnresolvedCon`. Inline-child forms p1 (class after) and p3 (fn after)
  refuse the same way. Within one run of segment members, forward
  references resolve (r2). Clients are in
  `/workspaces/ken/tmp/arch-inline-src/`.
- **The mechanism.** `expand_scope` walks declarations in textual order.
  Only runs of segment members (`is_segment_member`,
  `crates/ken-elaborator/src/modules.rs:4253`) are condensed into SCCs and
  dependency-ordered; a `ModuleDecl`, `ClassDecl` or `ImportDecl` ends the
  run and is handled at its position. `prebind_scope_declarations`
  (`:5289`) binds every local before the walk, but a checked id exists only
  after the declaration's own segment or class arm runs. A fn's forward
  refusal comes from a missed `globals` lookup by spelling
  (`infer_spelling_global`, `elab.rs:14335`), a guard by absence.
- **The ordering constraint.** Later siblings read an inline child's
  export table at the child's position (the held rows'
  `module Core.Operators.Standard { export Provider (..) }`), so a child
  and a later sibling can depend on each other.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

The scope walk's ordering in `crates/ken-elaborator`, the refusal for an
unorderable cycle, and the pins. The enclosing layer from
LANG-INLINE-MODULE-ENCLOSING-SCOPE is reused, not redesigned. No kernel,
catalog or §8.4 contract change.

## Acceptance

- **AC-0 (D0, Architect, before product code).** On the landed base with
  the enclosing layer, rule how module, class and import declarations
  join the dependency-ordered pass, and what an unorderable cycle (a child
  that needs a later declaration which needs the child's exports) refuses
  with. Census which existing rows change verdict, measured, not read.
- **AC-1.** r1, r3 and r4 accept at the root, each resolving to the later
  declaration's checked identity beside a same-spelled flat-global decoy
  where the harness allows one.
- **AC-2.** p1 and p3 accept inside an inline child, resolving to the
  enclosing checked identities, with the decoy.
- **AC-3.** The ruled cycle refuses with the ruled error at its own span.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The repair resolves a forward reference by spelling through `globals`.
- It needs a change to §8.4's pre-pass contract, or a kernel change.

Checks: 4 fired: I wrote the obligation against the delivered walk; the
Architect measured r1 to r5 and p0 to p3 on `8cf27bd2c`, and I confirmed
`is_segment_member` (:4253), `prebind_scope_declarations` (:5289) and
`infer_spelling_global` (elab.rs:14335) on `9bde60e21`. 10 fired: the
current fn refusal is a missed spelling lookup, so AC-1 and AC-2 require
the decoy, which a globals-fallback repair would bind. 8 fired: AC-3's
cycle separates a real dependency order from a repair that defers child
bodies, which would break the held rows' export reads.
