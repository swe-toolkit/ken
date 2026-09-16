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

> ## CORRECTION 2026-09-16 (Steward, on spec-author's measurement). THE FIRST
> ## VERSION OF THIS SECTION ASSERTED A PREMISE THAT IS FALSE.
>
> It said: *"Every `path` use site sits after a keyword ... That is very likely
> the intended discriminator."* **Two of the three do. `:280` does not, and
> `:280` is the site the whole node is about.** The wrong text is replaced below
> rather than annotated, because a likely-answer hint that is false exactly
> where the ambiguity is live is worse than no hint.
>
> **How it survived a re-measure:** the premise was checked by grepping `path`
> and reading the hit lines. A grep returns a coordinate; membership in a
> production requires resolving the **enclosing** right-hand side, which is
> upward of the hit and was never read. The two-of-three half confirmed, so the
> check stopped.

**Not "which is correct."** Both readings are wanted, in different places. The
gap is that the grammar never says how the choice is made, so an implementation
is free to resolve it either way and still claim conformance.

The three `path` use sites, measured at `b34ffd182` with each hit's **enclosing
production** resolved, not just its line:

    :52    | "proof" ident "for" path binder* ":" type "=" expr    in decl      AFTER A KEYWORD
    :293   proof_ref ::= "proof" ident "for" path                  own rule     AFTER A KEYWORD
    :280   | path "::" ident                                       an arm of `primary`   NO KEYWORD

    :357   ... Its subject is exactly one `path`; subsequent ...

**`:280` is an arm of `primary`, and that is the whole finding.** The two
productions are not adjacent, they are **nested**:

    application_atom ::= primary ("." ident | ".1" | ".2")*
    primary          ::= literal | ident | ConId | qualified_global_ref
                       | path "::" ident
                       | ...

⇒ **`path` sits inside `primary`, and `primary` is the head of the projection
chain.** Parsing `a.b::c`, the choice at `a` is between taking the
`path "::" ident` arm (so `path` consumes `a.b`) and taking the `ident` arm (so
the postfix chain consumes `.b`) — and it cannot be made until the `::`, or its
absence, is seen **after the dotted run has already ended**.

⇒ **The discriminator at the live site is a TRAILING token at unbounded
distance, not a leading keyword.** Position-determined is therefore not merely
unstated; **it is not available as the ruling where it is needed.**

`:357`'s "exactly one `path`" is the sentence closest to ruling any of this, and
it constrains the count, not the production.

# The three shapes an amendment could take

Named so the framing does not re-derive them, not to pre-empt the ruling.
**Shape 1 was the recommended one before the correction above; it is now the
one shape known not to work.**

1. **Position-determined. RULED OUT, not merely disfavoured.** `path` would be
   reachable only after a keyword. It is not: `:280` is an arm of `primary`,
   in expression position. Retained here so the framing does not re-propose it.
2. **Longest-match / lookahead on the trailing `::`.** Commit to
   `path "::" ident` only if a `::` follows the dotted run; otherwise take the
   projection chain. This is what the grammar appears to *intend*, and it is a
   real cost to state: unbounded lookahead, or backtracking, or a lexical
   rule that makes `::` decidable earlier.
3. **Restrict `path`.** Spell the attached-proof subject so it cannot collide
   (a distinct separator, or a non-terminal that is not ident-headed). Largest
   blast radius; touches `37 §6` compatibility spellings at `:203-204`. **This
   becomes materially more attractive under the correction** — it is the only
   shape that removes the unbounded-lookahead requirement rather than
   specifying it.

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
made** that a reader can apply without knowing the implementation.

*Control, and it must include the hard case:* give the required reading for
`a.b` (no trailing `::`, so projection chain) **and** for `a.b::c` (the
`primary` arm). A statement that only covers the first is satisfied by the
reading that was already assumed and does not discriminate — it is the same
blindness that put a false premise in this node's first version.

# Related

- [[SPEC-32-PROJECTION-PRECEDENCE]] -- the amendment that opened this.
- [[SPEC-32-QUALIFIED-GLOBAL-REF-VS-PROJECTION]] -- the blocked sibling.
