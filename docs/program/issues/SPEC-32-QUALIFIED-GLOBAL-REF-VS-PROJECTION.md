---
id: SPEC-32-QUALIFIED-GLOBAL-REF-VS-PROJECTION
title: "`qualified_global_ref ::= ModPath \".\" global_name` (:87) and the postfix projection chain (:277) both match `M.f`, and the overlap CANNOT BE RULED because `ModPath` has SIX uses and ZERO productions in all of spec/ -- with no definition of which strings ModPath matches there is no way to say where the two productions overlap; the blocker is therefore to GIVE ModPath a production, and this node stays blocked until that exists"
status: draft
owner: spec
size: S
gate: none
depends_on: [SPEC-32-PROJECTION-PRECEDENCE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16, BLOCKED at filing. Raised by spec-author alongside SPEC-32-PATH-VS-PROJECTION-CHAIN as a second consequence of their own amendment. Filed separately BECAUSE it is blocked and its sibling is not -- folding them together would have held a groundable node behind an ungroundable one. The ModPath measurement was re-run at main b34ffd182 before filing rather than quoted."
---

> # DRAFT and BLOCKED. Not framed, do not start -- and unlike its sibling,
> # this one CANNOT be started, by anyone, in its current state.

# The blocker, stated as the measurement

At `origin/main` `b34ffd182ca726ee7920cc7c14a28fd6ef0df1a4`, across **all** of
`spec/`:

    ModPath occurrences:   6     all in spec/30-surface/32-grammar.md
    ModPath productions:   0     no `ModPath ::=` anywhere in spec/

The six uses:

    :17    admits_clause ::= "admits" ModPath ("," ModPath)*
    :20    import        ::= "import" ModPath import_suffix?
    :24    export_decl   ::= "export" ( ModPath selection_list | export_item_list )
    :45      | "module" ModPath "{" (decl ";"?)* "}"
    :87    qualified_global_ref ::= ModPath "." global_name
    :108   In `export_decl`, `ModPath` is governed by the same role-blind path identity ...

**`ModPath` is a non-terminal that six productions depend on and nothing
defines.** `:108` constrains its *identity semantics* -- role-blind path
identity -- which is not a production and does not say which strings it matches.

# Why that blocks THIS node specifically

The question is whether `M.f` is a `qualified_global_ref` or a projection chain
`primary ("." ident)*`. Answering it requires knowing which strings `ModPath`
accepts:

- If `ModPath` is a single `ident`, then `M.f` is matched by both, and the
  overlap is total at length two.
- If `ModPath` is itself dotted (`A.B.C`), the overlap extends to every length
  and also collides with `path` (see [[SPEC-32-PATH-VS-PROJECTION-CHAIN]]).
- If `ModPath` is lexically distinguished -- a separate token class, a
  capitalization rule, a separator that is not `.` -- there may be **no overlap
  at all** and this node closes without an amendment.

⇒ **The three outcomes are not variations on one answer; one of them is "there
is nothing here."** Guessing which, in order to rule the overlap, would be
ruling on an invented premise.

**This is the whole reason the node is filed blocked rather than sized.** A
size, a deliverable, or an acceptance criterion written now would all be
conditional on an unmeasurable fact.

# The unblocking act

**Give `ModPath` a production in `spec/30-surface/32-grammar.md`** (or
`31-lexical.md`, if the right answer is that it is a token class). That is a
prerequisite in its own right and is larger than this node -- it is load-bearing
for `import`, `export`, `module`, and `admits`, not just for the overlap here.

**It should be its own node, not a step inside this one.** Four productions
other than `qualified_global_ref` are waiting on the same definition, and none
of them is about projection.

**Do not unblock this by ruling the overlap narrowly for `qualified_global_ref`
alone.** That would define `ModPath` implicitly, in one use site, by what the
projection rule needed it not to be -- the worst of the available orders.

# Not this node

- **Not defining `ModPath`.** That is the blocker, and it is bigger than this.
  Name it; do not absorb it.
- **Not the `path` overlap.** [[SPEC-32-PATH-VS-PROJECTION-CHAIN]] -- groundable
  now and must not wait on this.
- **Not projection's precedence.** [[SPEC-32-PROJECTION-PRECEDENCE]], settled.
- **Not a defect in the amendment.** The amendment gave projection a correct
  production. It did not create the `ModPath` hole; it is the first thing to
  make the hole matter.

# Related

- [[SPEC-32-PROJECTION-PRECEDENCE]] -- the amendment that surfaced this.
- [[SPEC-32-PATH-VS-PROJECTION-CHAIN]] -- the groundable sibling. Deliberately
  separate.
