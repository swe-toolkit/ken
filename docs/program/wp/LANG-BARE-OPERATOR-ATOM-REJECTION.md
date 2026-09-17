# WP frame — `LANG-BARE-OPERATOR-ATOM-REJECTION`

Owner: **language**   Size: **S**   Tier: **T1**   Gate: none

> **Released 2026-09-17 (Steward).** The node's `draft` hold was mechanical —
> it waited on [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]], which is now
> `merged` on `origin/main`. The hold is lifted and this frame discharges the
> node's own instruction: *"The Steward flips it `ready` and frames it when the
> predecessor merges."*

## 1. Objective

Bring the **bare-operator** row of `32-grammar.md §3`'s application-atom
contract pin into conformance: an ungrouped `operator_name` is admitted **only**
as the head of an `operator_prefix`, and therefore only with **at least one
following atom**. With zero following atoms it rejects at its leading token.

One parser edit, one rejection-span test, one set of keep-alive controls.

## 2. Fixed inputs

Measured at `origin/main`
`a749618aa334f3e55747a7255cd41cbd1f21b31c`. **Re-measure before you start —
line numbers move**, and this node cuts from a later `main` than the
measurement it inherits.

### 2a. The divergence, and whose measurement it is

Measured by language-implementer (`evt_4wthdbfybzdq4`) against
`wp/LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` at base
`394a5545fe14982d75327d4af549d93c4a8d6c07`:

    bare `<+>`   -> PARSES as EVar("<+>")    must REJECT
    bare `≤`     -> PARSES as EVar("≤")      must REJECT

**That base is not this node's base.** Re-run both probes at your own base and
record the result in the handback; if either already rejects, stop and report
rather than building a fix for a divergence that has closed.

### 2b. The two arms that must stay live — the controls

    (<+>)  and  (≤)        parse                     grouped form, KEEP
    <+> Zero / ≤ Zero      parse as A(G(OP), Zero)   applied form, KEEP

These are what make a red attributable to this change rather than ambient. They
must hold for **all six** names A0 landed, generic and reserved alike.

### 2c. The mechanism site, located

`crates/ken-elaborator/src/parser.rs:2965` — the atom parser's operator arm:

```rust
operator if canonical_operator_name(&operator).is_some() => {
    let span = self.peek_span().clone();
    let name = canonical_operator_name(&operator)
        .expect("guarded by operator-name recognition")
        .to_owned();
    self.advance();
    Ok(Expr::EVar(name, span))
}
```

**This arm is unconditional on what follows.** It is why a bare operator name
becomes a variable reference. The change is a **narrowing** of this consumer —
admit only when an atom follows — not a relocation of it, and not a change to
the infix path at `parser.rs:2254`, which reads operators in operand position
and is out of scope.

### 2d. The conformance row, and a live inconsistency on `main`

`conformance/surface/operators/seed-reserved-infix-names.md:126` carries the
`expect-negative` row for exactly this behaviour, tagged
**`RED-UNTIL-LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE`**.

> **That tag now names a `merged` node while the row it gates is still red.**
> A reader checking the tracker sees the named node landed and would expect the
> row green. The predecessor merged without re-pointing it. **This node closes
> the inconsistency by making the row green and retiring the tag** — not by
> re-pointing it at a new `RED-UNTIL` name.

### 2e. The census zero — state it, do not inherit it

The predecessor's D0 AST walk reported this row at **0 catalog sites**, and
`size: S` rests on there being no migration tail.

> **`0 sites` is a claim about MIGRATION EXTENT. This row asserts a REJECTION
> BEHAVIOUR.** A catalog containing no instances of a construct says nothing
> about whether the parser accepts it. The two are different propositions about
> different objects and the first does not entail the second.

**Re-run the walk at your base and report the number**, whatever it is. A
textual grep is not a substitute: it cannot distinguish a definition from a
mention, so it can neither confirm the zero nor refute it. If the walk returns
non-zero, **stop and report** — the size is wrong and the cut is mine, not
yours.

## 3. Acceptance

Each criterion names the control that makes its result attributable.

| # | criterion | control |
|---|---|---|
| AC-REJECT | bare `<+>` and bare `≤` each reject syntactically, before resolution | the same probes pass at 2a's base and fail after the edit — measure red first |
| AC-SPAN | the diagnostic points **at the operator token**, not at EOF or the next production | assert the span's start, not merely that an error occurred |
| AC-GROUPED-LIVE | `(OP)` still parses to that fixture's exact `G(OP)` | all six A0 names, generic and reserved |
| AC-APPLIED-LIVE | `OP atom` still parses to `A(G(OP), atom)` | all six A0 names |
| AC-ROW-GREEN | `seed-reserved-infix-names.md`'s `expect-negative` row is green and its `RED-UNTIL` tag is **deleted** | the row asserts the behaviour directly; a retired tag with a still-red row is a fail |
| AC-CENSUS | the AST-walk site count is stated in the handback as measured at your base | the walk, not a grep (2e) |

**No-regression means green in CI**, not a local `--workspace` run. Build and
test only through `scripts/ken-cargo`, scoped to `ken-elaborator`
(`COORDINATION §12`).

**AC-SPAN is the one that carries the design.** A rejection raised by whatever
production later encounters the leftover tokens satisfies "it rejects" and
fails this node: the pin is about *where* the contract is enforced.

## 4. What this node is NOT

- **The `if` and projection rows** — conformed by
  [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]].
- **Any change to `32 §3`.** The pin is the authority this node answers to, not
  an object it edits.
- **The temporal rows.** `RED-UNTIL-TEMPORAL-EXPRESSION-SURFACE` is a separate
  gate. Note that it currently **names no node in the tracker at all** — a
  dangling tag. That is a real defect and it is **not yours**; the Steward holds
  it as a queued filing. Do not repair it here and do not let it block you.
- **Standard meanings or fixities** for the operator names —
  [[SPEC-STANDARD-INFIX-BINDING]] and [[LANG-STANDARD-INFIX-CALL-COMPLETION]].
- **The infix operand path** (`parser.rs:2254`).

## 5. Contention

`crates/ken-elaborator/src/parser.rs`, single file.

The `depends_on` serialization is **discharged**: the predecessor is `merged` at
`origin/main`, verified against the node's status in the object store rather
than a branch ref. No other live language node touches this file — the language
ring's other seven nodes are all `draft` and unreleased, so the lane is yours
alone.

## 6. Why T1 on a small diff

The diff is a few lines. The **argument** is what needs a T1 seat: narrowing an
atom consumer has to preserve two live arms across six names while turning off a
third, and the reviewable claim is a *span* assertion about where enforcement
happens. That is a reasoning review, not a differential one — the same reason
the predecessor carried T1 on a small diff.
