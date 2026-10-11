---
id: VERIFY-EXAMPLE-DUPLICATE-SPAN-ABSOLUTE
title: "A DuplicateDefinition or AmbiguousReference raised in a ken example fence carries a span relative to the fence body, so ken check points at the wrong bytes of the .ken.md file. Rebase the span to the file"
status: merged
owner: verify
size: S
tier: T2
gate: architect
depends_on: [VERIFY-PACKAGE-DUPLICATE-TOP-LEVEL-REFUSAL]
blocks: []
github: null
origin: "Adversary M8 finding evt_6pnm35t50myf on 7832e4e16 (VERIFY-PACKAGE-DUPLICATE-TOP-LEVEL-REFUSAL). Low severity, error-path diagnostic regression; spec 39 section 5.6 requires an L1 error to carry the source span of the offending region. Steward-filed per COORDINATION section 2."
---

# An example's duplicate points at the example

## Objective

A refusal raised inside a `ken example` fence carries a span in the
original `.ken.md` source, as every other `.ken.md` error does.

## Settled inputs (Adversary `evt_6pnm35t50myf`, at `7832e4e16`)

- `execute_ken_md_checked_fences_v1` (`crates/ken-elaborator/src/lib.rs`
  ~:721-731) elaborates `&src[range]`, so resolver spans are offsets into
  the fence slice. It passes `DuplicateDefinition` and
  `AmbiguousReference` through unchanged; before `7832e4e16` every example
  error was rebased to `Span::new(range.start, range.end)`.
- `KenMdExtraction::example_ranges` are original-source offsets.
- **Repro.** `compile_ken_source` on `"# A\n\n```ken\nconst main : Bool =
  True\n```\n\n```ken example\nconst main : Bool = False\n```\n"` gives
  `DuplicateDefinition { name: "main", span: 0..25 }`; the duplicate is at
  57..82. `ken check` and `ken run` print the span verbatim.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The two passed-through variants keep their variant and name, and their
span is shifted by `range.start` into the original source. Any other
fence-relative span the same executor returns is shifted the same way.

## Acceptance

- **AC-1.** The repro gives 57..82, and the `base` variant gives its true
  offset; an `AmbiguousReference` row through the checked-fence executor
  does the same.
- **AC-2 (control).** The package-route rows of
  `verify_package_duplicate_top_level_refusal` keep their variants and
  names, and a plain-fence duplicate keeps its absolute span.
- **AC-3 (mutation, QA).** Dropping the shift reddens AC-1.

## Stop conditions

- The fix needs a change to `KenMdExtraction`, the CLI printer, or a
  kernel, trust or spec path.

## Closeout

Merged `a2b9f4145` from exact `3418e812e` (PR #4664). Verify QA
`evt_691nr8twtb7g8`, Architect `evt_41ej794mry0df`, Decision
`dec_3c9cb5c1arbkf`. The private checked-fence executor rebuilds
`DuplicateDefinition` and `AmbiguousReference` with the span shifted by
the example's `range.start`, keeping variant, name and sources. Those two
are the whole population of fence-relative spans the executor returns;
the whole-fence `ParseError` fallback and compiled-fence errors were
already absolute. Each removed shift reddened exactly its own pin with
the slice-relative 0..25. Diagnostics only. §1a: 0.
