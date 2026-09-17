---
id: LANG-BARE-OPERATOR-ATOM-REJECTION
title: "The third divergence from 32-grammar.md section 3's application-atom contract pin: an ungrouped operator_name with ZERO following atoms must reject syntactically, before resolution, but the A0 prefix-atom consumer admits an operator name as an atom unconditionally -- bare `<+>` and bare `≤` both parse as EVar. Admit an operator_name only as an operator_prefix head with at least one following atom, keeping the grouped form `(OP)` and the applied form `OP Zero` live."
status: merged
owner: language
size: S
gate: none
depends_on: [LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]
blocks: []
github: null
tier: T1
origin: "Steward cut 2026-09-16 on the fold-or-spin call routed by language-implementer (evt_4wthdbfybzdq4), ruled SPIN. Carries the third of seed-reserved-infix-names.md's rows, `expect-negative`, which LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE's frame named at its section 2b as an open fifth question and excluded from its four-row AC table. Cut BEFORE the predecessor's disposition is written so the re-point names a node that exists -- a RED-UNTIL tag naming an absent node is not a gate, which is the defect that produced the predecessor itself."
---

> ## MERGED 2026-09-17 at `5492ff97a4eb863986df9aad56efb82b5cb2c680`
>
> Blob-verified subsumed: `diff(merge-base, 5492ff97a)` against current
> `origin/main` is `LIVE=0` across all 3 touched files
> (`conformance/surface/operators/seed-reserved-infix-names.md`,
> `crates/ken-elaborator/src/parser.rs`,
> `crates/ken-elaborator/tests/lang_bare_operator_atom_rejection.rs`).

> # READY 2026-09-17 — the hold is discharged
>
> Frame: `docs/program/wp/LANG-BARE-OPERATOR-ATOM-REJECTION.md`
>
> The `draft` hold was mechanical and is now lifted:
> [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] is **`merged` on
> `origin/main`** (verified against the node's status in the object store, not a
> branch ref), so the `parser.rs` contention that motivated serializing this
> node behind it is discharged.
>
> **Read the frame, not this node, for the build.** The node states the
> reasoning and the boundary; the frame carries the fixed inputs measured at
> `a749618aa334f3e55747a7255cd41cbd1f21b31c`, the located mechanism site, and
> the acceptance criteria with their controls.
>
> **One input changed while this node sat.** The `expect-negative` row's
> `RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` tag now names a
> **merged** node while the row is still red — so the disposition this node was
> cut to receive did not happen. The frame resolves it by **retiring** the tag
> on a green row rather than re-pointing it at a new name.

# Objective

Bring the **bare-operator** row of `32-grammar.md §3`'s application-atom
contract pin into conformance: an ungrouped `operator_name` is admitted **only**
as the head of `operator_prefix`, and therefore only with **at least one
following atom**. With zero following atoms it rejects at its leading token.

# The divergence — measured on the predecessor's candidate

Measured by language-implementer (`evt_4wthdbfybzdq4`) against
`wp/LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` at base `394a5545fe14982d75327d4af549d93c4a8d6c07`:

    bare `<+>`   -> PARSES as EVar("<+>")    must REJECT
    bare `≤`     -> PARSES as EVar("≤")      must REJECT

**Both arms that must stay live are already correct**, and the controls
establishing that are what make the red attributable rather than ambient:

    (<+>)  and  (≤)        parse                     grouped form, KEEP
    <+> Zero / ≤ Zero      parse as A(G(OP), Zero)   applied form, KEEP

Only the zero-atom arm diverges. The change is therefore a **narrowing** of the
A0 prefix-atom consumer, not a relocation of it.

# Why this is a separate node and not a fold — the reasoning, so it is not re-litigated

The predecessor's frame quotes `32 §3`'s bare-operator paragraph at its §2a, so
an in-scope argument is available. It does not carry, for three reasons:

1. **Quoting the pin establishes what the SPEC says, not what the NODE
   delivers.** The predecessor's AC table is its scope, and §2b lists this row
   explicitly as an open fifth question.
2. **The regression surface is different.** This narrows the operator-name atom
   across the **six reserved names A0 just landed**, and it needs its own
   rejection-**span** assertion. Folding it in makes the predecessor's review
   turn on two unrelated arguments at once, and its current one — a 24-site
   migration ledger — is the weaker for it.
3. **The predecessor is DONE on its framed scope** and holds a lane. Both framed
   rows conform, the ledger is migrated, the acceptance suite is green on the
   AST, and the discriminator passes with parser-fix-plus-migration after
   measuring red with the fix alone. Landing that is not a partial result; it is
   the result.

# The census zero that does NOT apply here, and why

The predecessor's D0 AST walk reported this row at **0 catalog sites**. That
measurement is correct and **it does not bear on this node.**

⇒ **`0 sites` is a claim about MIGRATION EXTENT — how much catalog source must
change. This row asserts a REJECTION BEHAVIOUR — what the parser must refuse.**
A catalog containing no instances of a construct says nothing about whether the
parser accepts it. The two are different propositions about different objects,
and the first does not entail the second.

**So this node has no migration tail and that is a real finding, not an
assumption**: size `S` rests on it. The frame must still state the zero as a
measured input rather than inherit it, because it was measured on the
predecessor's candidate and this node cuts from a later `main`.

# Deliverables (indicative — this node is not framed)

- `operator_name` admitted only as an `operator_prefix` head with `>= 1`
  following atom; zero-atom occurrences reject at the leading token.
- A rejection-**span** assertion, not merely a rejection: the diagnostic points
  at the operator token.
- Controls that the two live arms are **unchanged** — `(OP)` grouped, and
  `OP atom` applied — across all six A0 names, generic and reserved alike.
- `conformance/surface/operators/seed-reserved-infix-names.md`'s
  `expect-negative` row goes green, and its `RED-UNTIL` tag retires.

# Not this node

- The `if` and projection rows — conformed by
  [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]].
- Any change to `32 §3` itself. The pin is the authority this node answers to.
- The temporal rows (`RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE`), a separate gate.
- Standard meanings or fixities for the operator names —
  [[SPEC-STANDARD-INFIX-BINDING]] and [[LANG-STANDARD-INFIX-CALL-COMPLETION]].

# Contention

`crates/ken-elaborator/src/parser.rs`. Serialized behind
[[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] by `depends_on`; check that
node's status at `origin/main`, not for a branch ref, before releasing.

# Related

- [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] — the predecessor; carries the
  other two rows and re-points this one here.
- [[SPEC-RESERVED-INFIX-APP-BOUNDARY-CORRECTION]] — merged; re-gated the seed
  rows onto a named successor, which is why a dangling `RED-UNTIL` name was the
  defect to avoid.
- [[LANG-RESERVED-INFIX-NAMES]] — A0; landed the six names this narrowing must
  not regress.
