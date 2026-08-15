---
id: LANG-FIXITY-DECL-SURFACE
title: "user-declared operator fixity -- `infixl`/`infixr`/`infix N op` -- is in the grammar, the declaration section, and the taxonomy's affordance list, and has zero implementation hits under crates/, so every operator in the landed language is at the default precedence"
status: draft
owner: language
size: null
gate: none
depends_on: []
blocks: []
github: null
origin: "Surfaced by CONF-FMT8-LEVELTOK's census (merged 2026-08-15 at 2ed8bbfd8): the conformance-validator found `surface/formatting/ascription-binder-fixity-and-associativity-survive` unproducible and it landed marked `BLOCKED-ON-USER-FIXITY-SURFACE (no blocker node exists)`. Steward-filed per COORDINATION §2 to supply that blocker. Filed `draft` and UNSIZED deliberately -- the gap is certain, the remedy is a design call the Steward may not make, and the scope question below has to be answered before this can be cut."
---

> # THE GAP IS CERTAIN. THE SCOPE IS NOT, AND THAT IS WHY THIS IS `draft`.
>
> **Do not flip this `ready` without an Architect ruling on the scope question
> below.** A precedence-parsing change is not a bounded lexer addition, and the
> spec's own framing leaves genuinely open how much of it Ken intends to land
> now. Filing it `draft` records the obligation without asserting a cut I have
> no grounds for.

## What this is

`spec/30-surface/33-declarations.md:755` states it normatively:

> `infixl N op` / `infixr N op` / `infix N op` declare operator fixity (`32 §6`).

It is not an isolated line. It appears in the grammar as a production —
`spec/30-surface/32-grammar.md:47`, `| fixity_decl -- infixl/r N op` — in the
expression grammar at `32-grammar.md:199` (`| expr binop expr -- operators
(declared fixity)`), in the lexical section's operator definition
(`31-lexical.md:494`, *"symbolic, from a fixed set plus user-defined (`33`)"*),
and in the taxonomy's affordance list at `30-taxonomy.md:82`.

**`32-grammar.md:373` supplies the default:** operators *"take declared fixity
(`infixl`/`infixr`/`infix N`, default `infixl 9`)"*.

**A census of `crates/` for `fixity`, `Fixity`, `infixl`, or `InfixL` returns
zero hits** — no token, no AST node, no parser arm, no test. Not a partial
implementation: the surface is entirely absent.

⇒ **Every operator in the landed language sits at the default `infixl 9`**,
which is a coherent state and is why nothing is visibly broken. What is absent
is the author's ability to change it.

## The scope question, which is the reason this is not framed

`32-grammar.md:393` says the arithmetic ordering *"and this fixity"* **are
not** — the sentence's subject matters and I am not going to paraphrase it into
a ruling. `32-grammar.md:410` separately describes a restricted profile with
*"no operators or fixity"*.

**So the spec contains both the full fixity surface and at least one profile
that explicitly excludes it.** Before this is cut, someone with the authority
has to answer:

1. **Does the landed L-level owe user-declared fixity at all**, or is
   `infixl 9` for every operator the intended endpoint for now — the same shape
   of ruling that settled the level-token question as endpoint (b) in
   `CONF-FMT8-LEVELTOK`?
2. If it is owed, **is the deliverable the declaration form alone** (parse and
   record `infixl N op`, honour it in expression parsing) **or does it include
   the operator-definition surface** that `31-lexical.md:494`'s *"plus
   user-defined"* implies? Those are different nodes.
3. **What does the formatter owe?** The blocked conformance row is about
   fixity and associativity **surviving formatting**, which is a distinct
   obligation from parsing them.

**Question 1 is the one that decides whether this node exists at all.** If the
answer is that `infixl 9` is the endpoint, this node closes and the conformance
row's marker becomes permanent-by-ruling rather than pending — exactly the
distinction `CONF-FMT8-LEVELTOK` was filed to make visible, arrived at from the
other direction.

## Why it is filed rather than left in the seed

`conformance/surface/formatting/seed-canonical-format.md` now carries
`BLOCKED-ON-USER-FIXITY-SURFACE (no blocker node exists)` on
`ascription-binder-fixity-and-associativity-survive`, and FMT1's aggregate
marker names the same surface. **That is an honest statement in a landed
artifact that the tracker owns nothing behind it.** The seed said the true
thing; this node is what makes it stop being true.

**A `RED-UNTIL-BUILT` row blocked on an unfiled surface is the exact defect
class `CONF-FMT8-LEVELTOK` existed to expose** — it reads as pending work
forever. Filing this converts it from *unowned* to *owned and unscheduled*,
which is a different and honest state.

## What must not happen here

- **Do not implement precedence climbing on the ring's authority.** The scope
  question above is unanswered and question 1 may close this node entirely.
- **Do not delete or weaken the conformance row.** Its intent is sound; if
  fixity is ruled out of the landed surface, the row's marker changes to name
  that ruling, and that edit is the spec enclave's.
- **Do not fold this into a formatter node.** Parsing fixity and preserving it
  across formatting are separate obligations and `32-grammar.md:199` is about
  the first.
