---
id: TEST-CHAR-CONSTRUCTION-ROUTES-SCALAR
title: "Pin that every surface route that builds a Char refuses a surrogate: a Char literal, an integer literal at an expected Char, and an Int-typed expression at Char. Two rows no test pins today; a route that admits D800 stops to the Architect as a Language design fork"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
blocks: []
github: null
origin: "Operator 2026-09-30 ('concur with recs'), on the Architect's Char-refinement ruling evt_h489cs74b8j (question 2), which found the Char scalar refinement unpinned on two surface routes while ruling the String section postulate. Steward-filed per COORDINATION section 2."
---

# Every route that builds a Char refuses a surrogate

## Objective

A `Char` value is a Unicode scalar on every surface route that builds one
(spec 35 §2.4), and a committed test says so per route.

## Settled inputs (Architect `evt_h489cs74b8j`)

- **(a) A Char literal.** `elab.rs:9367` goes through
  `checked_char_literal`, which calls `char::from_u32`. Scalar by
  construction.
- **(b) An integer literal at an expected Char,** as in
  `const c : Char = 55296`. This is row
  `char-expected-integer-literal-scalar-boundary`
  (`conformance/surface/numbers/seed-numbers.md:286`). No test pins it.
- **(c) An Int-typed expression at Char,** as in
  `fn toC (n : Int) : Char = n`. The kernel's Char carrier converts to Int
  (`check.rs:1356`). The prelude's own `intToChar` returns `Some Char n`
  with `n : Int` (`decimal_char.rs:262`), so the elaborator admits an Int at
  Char with no proof. This is the likely source of Verify's unreproduced
  D800 observation, reached through (b).
- **(d) `s2l` of a String.** Scalar by construction; no pair needed.
- **Runtime stakes.** Native and IR `list_char_to_string` hard-error on a
  surrogate head (`runtime_ir_evaluator.rs:1861`), while the interpreter
  substitutes U+FFFD.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One committed refuse/accept pair per route (a), (b) and (c), claiming rows
`char-excludes-surrogates` (`seed-numbers.md:267`) and
`char-expected-integer-literal-scalar-boundary`.

## Acceptance

- **AC-1.** Each pair asserts both of its members on the same harness:
  - (a) `'\u{D800}'` is refused and `'a'` is accepted;
  - (b) `55296` at Char is refused and `55295` at Char is accepted;
  - (c) `toC 55296` is refused, with an accepted non-surrogate control.
- **AC-2 (control).** Each refusal names the surrogate or scalar
  violation, not an unrelated error. Show it with the refusal's verbatim
  message.

## Stop conditions

- **(b) or (c) is accepted today:** commit nothing for that route and stop
  to the Architect with the accepting program and its output. Enforcing
  the refinement at the Int-to-Char boundary is a Language design fork, not
  a test change.
- Any elaborator, kernel, runtime or spec change.
- A new `trusted_base()` entry.

## Stop ruling (Architect `evt_1a3kmh4jf12ga`)

Routes (b) and (c) are accepted at `944ff08ca`. That is an elaborator
defect, not a design fork: spec 34 §5 requires every introduction at a
refinement to emit `φ a`, and a `def`-named refinement emits nothing.
Accepted or refused is not the observable, because a correct elaborator
accepts both members of each pair. The emitted goal is the observable.
- **Route (a) proceeds now.** It claims `char-excludes-surrogates` on the
  literal route only.
- **(b) and (c) are held** on `LANG-REFINEMENT-INTRODUCTION-OBLIGATION`.
  When they resume, their AC-1 is refit:
  - (b) `55296` at Char emits `isScalar 55296`, undischarged, and `55295`
    leaves no open obligation;
  - (c) the obligation fires in `toC`'s body, with `n` free. The control is
    a `toC` that discharges it, for example through `intToChar`.
