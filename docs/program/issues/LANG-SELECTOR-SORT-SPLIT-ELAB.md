---
id: LANG-SELECTOR-SORT-SPLIT-ELAB
title: "Implement the sort-split recursive-result selector in the elaborator -- parse `recursive result for x` and `induction hypothesis for x`, classify the selected hidden result by sort, and remove `structural result of x` from the crates"
status: ready
owner: language
size: L
gate: none
depends_on: [SPEC-SELECTOR-SORT-SPLIT]
blocks: []
github: null
origin: The implementation successor to SPEC-SELECTOR-SORT-SPLIT, which is spec-and-conformance only. spec-leader ruled at the D0 scope checkpoint that the crate-side work -- including the Adversary's control-attribution repair -- is "Language successor work, not a spec-enclave implementation action", and spec-author's D0 handoff measured the residue it leaves behind: 22 old-spelling matches under crates/. Steward-filed per COORDINATION section 2, closing the stay-one-release-ahead gap while the enclave holds the node.
---

> # THE CONTRACT IS LANDED SPEC. DO NOT RE-DERIVE IT, AND DO NOT AMEND IT HERE.
>
> `spec/30-surface/31-lexical.md §4`, `32-grammar.md §3`,
> `34-data-match.md §3.1.1`, and `39-elaboration.md §2.3`/`§4` are the
> authority for the spelling, the classification rule, the diagnostics, and
> their payloads. Read them at the merged SHA. Where this node and the spec
> disagree, **the spec wins and the disagreement comes back to me.**

## What it is

`SPEC-SELECTOR-SORT-SPLIT` replaces the single sort-agnostic selector with two
spellings chosen by the sort of the selected hidden result:

- **`recursive result for xs`** when its type is classified by `Type l`;
- **`induction hypothesis for xs`** when its type is classified by `Omega l`.

`structural result of x` is **removed outright** (operator, 2026-08-10). The
spec candidate already sweeps clean; the crates do not, and this node is what
makes the removal real.

## The clause most likely to be got wrong

**Classify the result, not its support evidence.** The topology-carrying
`All^Omega` application lives in `Type 0` even though its leaves and an
Omega-valued recursive result are proofs. Keying on the support inverts the
answer in exactly that case, and the support is what is syntactically to hand —
which is why this is the natural mistake rather than an unlikely one.

A **Type-valued proof-relevant** witness still takes `recursive result`. The
split follows the sort and never programmer intent.

## Deliverables

**1. Surface.** The two contextual four-token forms, per `31 §4` and the
`32 §3` productions `recursive_result` and `induction_hypothesis`, as primary
atoms at the operand and precedence boundary the spec fixes. `structural result
of` no longer parses.

**2. Classification and diagnostics.** Classify the type the checked method
telescope assigns to the selected hidden result. A spelling that disagrees
raises the spec's mismatch diagnostic **naming the required spelling**;
classification that is metavariable-ambiguous raises the spec's ambiguity
diagnostic and **rejects, with no default either way**, under the existing
no-guessing rule.

**3. The sweep.** The 22 old-spelling matches under `crates/` go to zero —
source, tests, fixtures, and diagnostic identifiers alike. **Identifiers follow
the landed spec's names exactly.** Where the spec pins a name, use it verbatim;
where it does not, keep the existing failure condition and respell only the
vocabulary.

**4. The control-attribution repair**, carried over from
`SPEC-SELECTOR-SORT-SPLIT` by spec-leader's routing. The identity control
landed at `e551e735` tests the right property at the wrong resolution:

```rust
assert!(matches!(
    env.elaborate_file(&source),
    Err(ElabError::StructuralResultOutOfScope { .. })
));
```

The mutated arm holds **two** selectors — the deliberate comparator and the new
`_`. `elaborate_file` returns the **first** error and `{ .. }` pins only the
variant. **If a flipped boundary ever stopped the arm being an active lift
scope, the comparator would raise the identical variant and this control would
pass, attributing nothing.** The pair inverts and the test still greens: the
pair fails to fail.

The assertion must attribute the refusal to the `_` occurrence, not merely to
the variant. Binding either span suffices and the payload is already carried —
the stronger idiom is in this mechanism's own unit test at
`elab.rs:7986-7992`, matching on `selector_span` and `binding_span`. Keep the
`assert_ne!(source, STRUCTURAL_SIZE_SOURCE)` substitution guard: it is what
makes a drifted fixture redden instead of passing vacuously.

## Acceptance criteria

**AC-1.** Both spellings elaborate to the same hidden recursive method result
the old selector reached, on a witness for each sort. The association still
derives from the kernel `method_type`/`recursive_shapes` telescope and binding
identity still governs.

**AC-2 — the removal is real, not shadowed.** `structural result of` is a parse
error, and the crate-wide match count for the old spelling and the retired
grammar production is **zero**. Report the count; a sweep that reports "clean"
without a number is not evidence.

**AC-3 — the classification discriminates.** A witness whose result is
`Omega`-classified must **reject** under `recursive result for`, and a
`Type`-classified one must **reject** under `induction hypothesis for`. Both
directions, because a checker that always accepts passes either one alone.

**AC-4 — support evidence cannot invert the answer.** A witness with
`Type`-resident `All^Omega` support over an `Omega`-classified result takes
`induction hypothesis for`. **This is the case the natural implementation gets
backwards**, so it is the one control that must exist by construction rather
than by intent.

**AC-5 — ambiguity rejects.** A metavariable-ambiguous classification raises
the ambiguity diagnostic. The control must show the same source **accepting**
once the metavariable is resolved, so the rejection is attributable to
ambiguity rather than to any other defect in the fixture.

**AC-6 — the repaired identity control attributes.** The assertion binds the
span of the `_` occurrence. Show it by mutation: a fixture drift or a boundary
flip that leaves the comparator raising the same variant must now **red**,
where it previously greened. Report the mutation and its result.

## Excluded scope

- **Not a mechanism change.** No lift reconstruction outside the kernel
  telescope, and no second topology rule. Both are standing prohibitions.
- **The two gated conformance rows stay gated.** `nested-size-uses-lift` and
  `nested-dependent-motive-uses-lift` are respelled by CV under the spec node
  and are blocked on separate work. **An ungating slipped in with a respell
  would read as a capability landing that has not landed.**
- `crates/ken-runtime` is out of scope, as is
  [[LANG-NESTED-MATCH-LIFT-ALIGNMENT]].

## Stop conditions — return to me, do not decide

The landed spec still spelling a removed term in a pinned diagnostic name; a
classification the spec does not settle; any case where honouring the sort split
would require touching the association mechanism.

## Contention and validation

Paths are `crates/ken-elaborator` and its tests. Runtime owns
`crates/ken-runtime`; the enclave's spec and conformance edits land first by
construction. No `spec/` or `conformance/` path, so no Spec vote on the merge
Decision.

Targeted only — `-p ken-elaborator`, or `--test <name>` for one suite, **never
`--workspace`**. Adding or changing an enum variant makes the floor a full
`-p ken-elaborator` test build, because a suite-scoped run cannot observe an
exhaustive `match` in a sibling target. "No regression" means green in CI.
