---
id: SPEC-32-PATH-VS-PROJECTION-CHAIN
title: "32 §3 now has TWO productions that both match an ident-headed dot chain -- `path ::= ident (\".\" ident)*` (:86, the attached-proof subject) and the postfix projection chain `application_atom ::= primary (\".\" ident | \".1\" | \".2\")*` (:277) added by SPEC-32-PROJECTION-PRECEDENCE -- so `a.b.c` is admitted by both and the grammar does not say which wins or where the choice is made; rule the overlap explicitly (position-determined, or a precedence, or a restriction on one side) rather than leaving it to whichever production an implementation reaches first; NO change to projection's precedence, NO parser work in this node"
status: draft
owner: spec
size: S
gate: none
depends_on: [SPEC-32-PROJECTION-PRECEDENCE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16. Raised by spec-author as a consequence of the SPEC-32-PROJECTION-PRECEDENCE amendment they had just authored, and taken by the Steward rather than left as an unfiled enclave item. Premises re-measured at main b34ffd182 before filing, not quoted from the raising post. Queued behind the three active lanes: this is a filing, not a release."
---

> # DRAFT -- not framed, do not start. Filed so "queued" is distinguishable
> # from "dropped."

# The measurement

At `origin/main` `b34ffd182ca726ee7920cc7c14a28fd6ef0df1a4`, in
`spec/30-surface/32-grammar.md`:

    :86    path             ::= ident ("." ident)*                  -- attached-proof subject path
    :277   application_atom ::= primary ("." ident | ".1" | ".2")*   -- postfix projection chain

**The first `ident` of a `path` is also a `primary`.** So for an ident-headed
dot chain with no numeric components -- `a.b`, `a.b.c` -- the two right-hand
sides accept exactly the same strings.

The amendment did not create `path`; `path` predates it. What the amendment did
was give projection a production of its own at `primary`, and that is the first
time the two have been in contact.

# What is actually unspecified

**Not "which is correct."** Both readings are wanted, in different places. The
gap is that the grammar never says the choice is position-determined, so an
implementation is free to resolve it either way and still claim conformance.

The two positions that already exist, measured:

    :52    | "proof" ident "for" path binder* ":" type "=" expr   -- attached proof theorem
    :280   | path "::" ident                                       -- canonical attached-proof path
    :293   proof_ref ::= "proof" ident "for" path

    :357   ... Its subject is exactly one `path`; subsequent ...

⇒ Every `path` use site sits after a **keyword** (`for`, or the `::`-suffixed
canonical form). Every projection-chain use site sits in **expression
position**. That is very likely the intended discriminator, and if so the
amendment is that saying it costs one sentence.

**Do not treat "it is obviously position-determined" as the answer.** It is the
likely answer and it is unstated, and an unstated discriminator is what a second
implementation gets wrong. `:357`'s "exactly one `path`" is the sentence closest
to ruling it, and it constrains the count, not the production.

# The three shapes an amendment could take

Named so the framing does not re-derive them, not to pre-empt the ruling:

1. **Position-determined.** `path` is reachable only after `for` / in the `::`
   form; expression position always takes the projection chain. Cheapest, and
   matches every existing use site.
2. **A precedence.** One production is preferred where both apply. More
   machinery than the situation needs unless a use site exists where both are
   genuinely reachable -- and none was found.
3. **Restrict `path`.** Spell the attached-proof subject so it cannot collide
   (a distinct separator, or a non-terminal that is not ident-headed). Largest
   blast radius; touches `37 §6` compatibility spellings at `:203-204`.

# Not this node

- **Not projection's precedence.** Settled by
  [[SPEC-32-PROJECTION-PRECEDENCE]]; projection binds tightest and this node
  does not move it.
- **Not `qualified_global_ref`.** A separate and BLOCKED overlap --
  [[SPEC-32-QUALIFIED-GLOBAL-REF-VS-PROJECTION]]. Do not merge the two: that one
  cannot be ruled at all yet, and folding it in here would block a groundable
  node behind an ungroundable one.
- **Not projection in type position.** [[LANG-TYPE-PROJECTION-SURFACE-FORM]].
- **No parser work.** If the elaborator already resolves this the way the
  amendment would rule, that is a conformance observation for a later node, not
  a reason to skip stating the rule.

# Acceptance, indicative

Whatever it rules, the amendment owes a statement of **where the choice is
made** that a reader can apply to `a.b.c` without knowing the implementation.
The control: an example of an ident-headed dot chain in each position, with its
required reading given.

# Related

- [[SPEC-32-PROJECTION-PRECEDENCE]] -- the amendment that opened this.
- [[SPEC-32-QUALIFIED-GLOBAL-REF-VS-PROJECTION]] -- the blocked sibling.
