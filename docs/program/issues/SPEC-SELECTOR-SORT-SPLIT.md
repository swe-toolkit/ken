---
id: SPEC-SELECTOR-SORT-SPLIT
title: "split the recursive-result selector by motive sort -- `recursive result for x` when Type-classified, `induction hypothesis for x` when Omega-classified -- and remove `structural result of x`"
status: merged
owner: spec
size: M
gate: none
depends_on: []
blocks: [LANG-SELECTOR-SORT-SPLIT-ELAB]
github: null
origin: Operator directive 2026-08-10, relayed through the research seat at evt_25q3mb2pe0tay and confirmed by the operator directly to the Steward. Prompted by the research terminology pass at evt_5b30njk18nmph on landed 2c0f4c03. Steward-filed (agents cannot create tracked work per COORDINATION section 2).
---

## The change

Replace the single sort-agnostic selector with **two spellings chosen by the
sort of the selected hidden result**:

- **`recursive result for xs`** when its type is classified by `Type l`;
- **`induction hypothesis for xs`** when its type is classified by `Omega l`.

**`structural result of x` is removed outright** (operator, 2026-08-10). Ken has
no users, so there is no migration burden and no reason to carry a third
permanent spelling or a deprecation window.

## Why, and it is two separate reasons

**`of` is wrong, not merely unidiomatic.** It reads as a possessive, as though
the result were an intrinsic projection from `x`. It is not: the landed contract
states that *"shadowing, copying, projecting, or merely reusing a spelling does
not transfer an association"*, and the identity control merged at `e551e735`
demonstrates it — a fresh binding in the same arm gets
`StructuralResultOutOfScope`. **The surface currently connotes exactly what the
semantics forbid.** `for` signals a branch-context association keyed by binding
identity.

**`structural result` is Ken-local vocabulary for something with settled names.**
The Type-classified case is the recursive result of a fold or catamorphism; the
Omega-classified case is an induction hypothesis. Lean's recursor documentation
says that with a `Prop`-valued motive the additional recursive arguments are
induction hypotheses while the same recursor supports primitive recursive
computation; Rocq's nested-inductive documentation says registered
`All`/`AllForall` support creates an induction hypothesis for the nested
argument. The split follows prior art unusually closely.

**`induction hypothesis`, not `inductive hypothesis`** — the former is the
conventional proof-assistant and mathematical noun phrase.

## The rule is semantic, not two aliases

Both phrases resolve the same source binding and pass the same validated
association gate, then **classify the selected hidden result's type**. The
spelling must agree with the classification; a mismatch gets a focused
diagnostic naming the correct spelling.

**Classify the result, not its support evidence.** Ken's own conformance text
notes that the topology-carrying `All^Omega` application lives in `Type 0` even
though its leaves and an Omega-valued recursive result are proofs. Keying on
the support would invert the answer in exactly that case.

**A Type-valued proof-relevant witness still uses `recursive result`** — the
split follows the actual sort, never programmer intent. Every Omega-valued
result is proof vocabulary.

**If classification remains metavariable-ambiguous, reject** under the existing
no-guessing rule. Do not default to either spelling.

## Deliverables

**Spec enclave first.** `spec/30-surface/34-data-match.md §3.1.1` is the
authority for the spelling and must state the classification rule, the mismatch
diagnostic, and the ambiguity rejection. The conformance rows follow.

**Then Language** implements, as [[LANG-SELECTOR-SORT-SPLIT-ELAB]]: the parser's
contextual-keyword forms, the AST variant or variants, the classification, the
diagnostics, the crate-wide removal of the old spelling, and the control repair
below. **No crate path is this node's work.**

**The two gated conformance rows are respelled, not ungated.**
`nested-size-uses-lift` and `nested-dependent-motive-uses-lift` keep their
gates; they are blocked on [[LANG-NESTED-MATCH-LIFT-ALIGNMENT]] and on their own
separate unmeasured blocker respectively. **This node changes their spelling and
nothing about their status.**

## A control defect, routed to the successor

**This is no longer this node's work.** spec-leader ruled at the D0 scope
checkpoint that the Rust control repair is Language successor work rather than a
spec-enclave implementation action, and it is now a deliverable of
[[LANG-SELECTOR-SORT-SPLIT-ELAB]]. It is restated here only because this node is
where the Adversary's finding was first recorded.

The identity control landed at `e551e735` tests the right property **at the
wrong resolution**.

```rust
assert!(matches!(
    env.elaborate_file(&source),
    Err(ElabError::StructuralResultOutOfScope { .. })
));
```

The mutated arm holds **two** selectors — the deliberate `xs` comparator and the
new `_`. `elaborate_file` returns the **first** error and `{ .. }` pins only the
variant. **If a flipped boundary ever stopped the arm being an active lift
scope, the comparator would raise the identical variant and this control would
pass, attributing nothing.** That is the precise green-on-green the
non-degenerate pair exists to prevent: the pair inverts and the test still sees
`Err(OutOfScope)`, so **the pair fails to fail**.

**Required:** the assertion must attribute the refusal to the `_` occurrence,
not merely to the variant. Binding either span suffices, and the payload is
already carried — the stronger idiom is in this mechanism's own unit test at
`elab.rs:7986-7992`, matching on `selector_span` and `binding_span`.

Keep the `assert_ne!(source, STRUCTURAL_SIZE_SOURCE)` substitution guard: it is
what makes a drifted fixture redden instead of passing vacuously.

## Excluded

Not a mechanism change. The association still derives from the kernel
`method_type`/`recursive_shapes` telescope, binding identity still governs, and
the wildcard and anonymous-scope repairs are untouched. Do not reconstruct a
nested lift and do not add a second topology rule.

`crates/ken-runtime` is out of scope, as is the elaborator's nested-match check
alignment, which is [[LANG-NESTED-MATCH-LIFT-ALIGNMENT]]'s work.
