---
id: LANG-FORWARD-PROP-INTRO-EDGE
title: "A declaration that names a later prop's intro (M.P.pi) is refused with UnresolvedCon, because the forward-reference graph keys data constructors to their data node but never inserts a prop's intros into node_names, so no edge reaches the prop and textual order wins. Key each prop intro to its prop node"
status: active
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT]
blocks: []
github: null
origin: "Adversary M8 finding evt_5ts6gg4z19rct on 6f3b0aceb (LANG-FORWARD-REFERENCE-ACROSS-DATA-EXPORT). Fails closed: a valid program is refused, nothing mis-binds; not a regression. Steward-filed per COORDINATION section 2."
---

# A later prop's intro is a forward edge

## Objective

A declaration that mentions a later prop's intro is elaborated after that
prop, exactly as a mention of a later data constructor is.

## Settled inputs (Adversary `evt_5ts6gg4z19rct`, at `6f3b0aceb`)

- **The witness.** `module M { prop Q : Omega where { qi : Q } theorem t :
  Q = let h = P.pi in Q.qi data D = MkD prop P : Omega where { pi : P } }`
  fails with `UnresolvedCon { name: "M.P.pi" }`. Without `data D`, so that
  `P` directly follows `t`, it fails the same way.
- **Controls, OK on the same tree:** the backward order; the data analogue
  `const a : Int = let h = C in 0 data E = MkE data D = C` (AC-1(e)); naming
  the later prop by its family (`theorem t (p : P) : Q = Q.qi`); a forward
  attached-proof reference `f::p`; forward type-alias references.
- **The mechanism.** `modules.rs:4474-4494` keys `node_names` by each
  node's own name, plus a data node's constructors as `qualify(prefix, C)`
  and `{family}.C`. Prop intros, checked as `{prop}.{intro}`
  (`register_checked_type_node`), are never inserted, so
  `rdecl_mentions_name` cannot match `RCon("M.P.pi")` to `P`.
- **The rule.** Spec 33 §8.4 point 1: an edge is "the body or type of A
  mentions B". A prop intro is the prop's constructor.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

Each prop intro is keyed to its prop node in the spellings data
constructors already use. The Architect confirms the spellings at review.

## Acceptance

- **AC-1.** Both witnesses check, and `t`'s `Q.qi` and `P.pi` resolve to
  the checked intros of `Q` and `P`.
- **AC-2 (control).** The five control rows keep their results, and
  `lang_forward_reference_across_data_export` and
  `lang_qualified_constructors` stay green.
- **AC-3 (mutation, QA).** Removing the intro keys reddens AC-1.

## Stop conditions

- A prop intro spelling collides with a data constructor spelling in the
  graph.
- Any kernel, trust or spec change.
