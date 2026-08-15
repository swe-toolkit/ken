---
id: LANG-FIXITY-DECL-SURFACE
title: "`infixl N op` / `infixr N op` / `infix N op` populate a fixity table the parser consults -- the third and last part of user-defined operators, and the only one carrying a real design call: declaration-before-use versus whole-module collection, and scoping across imports"
status: draft
owner: language
size: M
gate: none
depends_on: [LANG-INFIX-APPLICATION-DEFAULT]
blocks: []
github: null
origin: "Filed 2026-08-15 by the Steward from CONF-FMT8-LEVELTOK's census, originally scoped to fixity alone and left `draft`/unsized pending a scope ruling. Architect ruling evt_1s7mqjg4tyxx1 answered it: declared fixity is OWED, not an endpoint -- 32-grammar.md:392-393 names its existence as explicitly NOT an open question. The same ruling established the node was scoped to a third of the feature and decomposed it; this node is now part (iii). Import-scoping measured and costed by the Architect at evt_33rw7w8xkdya2, re-verified by the Steward. Steward re-cut per COORDINATION section 2."
---

> # THE ID IS DELIBERATELY UNCHANGED. DO NOT RENAME THIS NODE.
>
> The Architect's ruling observed that the *feature* is user-defined operators,
> not fixity, and initially instructed a rename to match. **He then withdrew
> that instruction** (`evt_33rw7w8xkdya2`), and the reason is worth keeping:
> **his rename was conditioned on this staying ONE node covering all three
> parts.** Once it was cut into three, **(iii) genuinely is just fixity** — the
> id is the accurate name for what this node now contains, not a legacy
> compromise.
>
> There is a second, independent reason. `seed-canonical-format.md` links
> `[[LANG-FIXITY-DECL-SURFACE]]` at three live sites (`:27`, `:58`, `:424`),
> landed at `e6d2716cf`. **Renaming would dangle every one of them, recreating
> the exact defect [[CONF-BLOCKER-MARKER-RECONCILE]] existed to remove**, hours
> after it merged.
>
> The feature as a whole is [[LANG-SYMBOLIC-OPERATOR-NAMES]] +
> [[LANG-INFIX-APPLICATION-DEFAULT]] + this node. **No node is named
> `LANG-USER-DEFINED-OPERATORS`** — that would be a fourth name for work three
> nodes already cover.

## `draft` for one reason, and one further thing is owed

**Its predecessors have not landed** — `a <+> b` cannot yet parse, so a fixity
table would have nothing to bind. Flip it `ready` when
[[LANG-INFIX-APPLICATION-DEFAULT]] goes `active`.

**Unlike its two siblings, this frame is NOT yet fully shovel-ready.** The
Architect named a live design call inside it and handed the decomposition
rather than a frame. `D1` reports the remaining fork rather than deciding it —
the pattern that worked on [[LANG-MEMBERSHIP-OPERATOR-SURFACE]].

## The scope question is ANSWERED. Recorded so it is not re-litigated.

This node was `draft` and unsized because the Steward would not paraphrase
`spec/30-surface/32-grammar.md:393` into a ruling. The Architect ruled on the
verbatim sentence:

> *"The remaining spellings and the levels of non-arithmetic user operators
> stay `OQ-syntax`; the existence of declared fixity, and this arithmetic
> ordering, are **not**."*

`OQ-syntax` is the open-question marker; the sentence names the **existence of
declared fixity** as explicitly not open. ⇒ **Not an endpoint. It is owed.**

**The V0 objection is a category error and is closed.** `32-grammar.md:410`
excludes *"operators or fixity"* — and in the same list excludes **literals**
and **`match`**, both landed. It is a bootstrap staging subset, not a statement
about what the L-level owes. Do not re-raise it.

## Deliverables

**`D1` — report the design fork with citations, before any table is built.**

**1. Declaration-before-use, or whole-module collection?** Must `infixl 5 <+>`
appear textually before the first use of `<+>`, or does the parser collect
every fixity declaration in the module first? **These differ observably** on a
module that uses an operator above its declaration, and the choice constrains
whether parsing can stay single-pass. **Open — report what the spec settles.**

**2. Scoping across imports — MEASURED AND COSTED. A two-option fork, not an
open question.** Measured by the Architect (`evt_33rw7w8xkdya2`) and
re-verified by the Steward at `e6d2716cf`:

```rust
exports: HashMap<String, HashMap<String, String>>    // modules.rs:54
```

**Module → local name → resolved name. Names only, no attributes.** ⇒ **A
fixity declaration on an exported operator has nowhere to live in the current
representation.** The two options and their prices:

| option | meaning | price |
|---|---|---|
| **module-local fixity** | an operator's fixity does not cross an `import` | **zero change to `modules.rs`** |
| **propagating fixity** | the export value widens from `String` to a record carrying fixity | **10 threading sites**: `:54`, `:176`, `:221`, `:248`, `:317`, `:819`, `:868`, `:1011`, `:1031`, `:1327` |

**Do not re-derive this.** Report which option the spec supports, against the
two above. The conflicting-fixity-on-two-imports sub-question only arises under
the second option.

> **The Architect has committed to ruling question 2 at
> [[LANG-INFIX-APPLICATION-DEFAULT]]'s merge Decision** — deliberately not
> earlier, because the right base to rule it on is one where the parser's
> actual table-consultation shape is visible, rather than asserting it from the
> export type alone. **Steward: raise it at that Decision** so this node is not
> waiting on a round-trip when the ring reaches it. If the ruling has landed by
> release, pin it here and drop this sub-question from `D1`.

**3. Redeclaration within a module.** Error, or last-wins?

**Ground each against `spec/30-surface/33-declarations.md §6` and the module
chapter, and report what the spec settles versus what it leaves open.** Where
the spec settles it, that is the answer and no ruling is needed.

**`D2` — the fixity table and the parser's consultation of it.** `infixl N op`,
`infixr N op`, `infix N op` populate it; the infix parser consults it in place
of the hard-wired default from [[LANG-INFIX-APPLICATION-DEFAULT]].

**`D3` — `infix N` (non-associative) rejects `a <+> b <+> c`.** With a
diagnostic that says so. **This is the arm most likely to be silently omitted**,
because both associative arms have obvious behaviour and this one only shows up
as an error path.

**`D4` — precedence level bounds.** State what range of `N` is accepted and
what happens outside it. `32-grammar.md:373` gives the default as `9`; if the
spec bounds the range, cite it — **if it does not, say so and pick, stating the
choice as a choice.**

## Acceptance criteria

**`AC-1`.** `D1` is reported with citations before `D2` is implemented. **A
candidate that implements the table without first reporting the fork has
skipped the deliverable**, even if the table is right.

**`AC-2`.** Declared fixity changes parse shape. **Control:** the same source
text parses to **different** trees under `infixl 5` and `infixr 5`, asserted
structurally. **A value assertion fails this** — a symmetric operator evaluates
identically under both.

**`AC-3`.** An operator with no fixity declaration still parses at `infixl 9`.
**Control:** [[LANG-INFIX-APPLICATION-DEFAULT]]'s fixtures stay green
unchanged. **This node adds a table; it must not make the default path
conditional on one existing.**

**`AC-4`.** `D3`'s non-associative rejection has its own fixture and its own
diagnostic assertion.

**`AC-5` — direction stated.** Declared fixity **changes the parse** of
programs that declare it. Programs that declare nothing are unaffected. **If a
program with no fixity declaration changes meaning, stop** — that is `AC-3`
failing by another route.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Targeted:
`-p ken-parser`, `-p ken-elaborator`.

## What this unblocks, and the follow-through that is NOT yours

`conformance/surface/formatting/seed-canonical-format.md` carries
`BLOCKED-ON-USER-FIXITY-SURFACE ([[LANG-FIXITY-DECL-SURFACE]])`, and FMT1's
aggregate names it. **When this lands, that fixture becomes producible and FMT1
loses one of its three blockers.**

**Do not edit `conformance/`.** Say in the handback that the row is now
producible; the spec enclave owns the flip.

## Banned scope

- **Do not re-open whether fixity is owed.** Ruled, `evt_1s7mqjg4tyxx1`, on the
  spec's own words.
- **Do not decide the `D1` fork unilaterally** where the spec leaves it open.
- **Do not change the arithmetic ordering.** Landed, normative, and settled by
  the same sentence that settles this node's existence.
