# LANG-PRELUDE-COLLECTIONS

**Owner:** language. **Size:** S-M. **Gate:** none.
**Predecessor:** none blocking. The L3a slice landed the mechanism; this ships it.

Move the `List` combinators from the test file that declares them into the
prelude, and build `filter`, whose deferral reason no longer holds.

## The measurement, at `origin/main = 7f9eabbb`

**Re-derive at point of use.**

`spec/30-surface/37-strings-collections.md §9` requires, verbatim, that WS-L
deliver *"in the surface/elaborator + prelude"* the
*"`map`/`filter`/`fold`/`zip` combinators"*. **They are not in the prelude.**
They are declared inside a test.

`crates/ken-elaborator/tests/l3a_acceptance.rs:26-45`, `setup_combinators`,
declares `map`, `fold` and `zip` with `env.elaborate_decl`. The file's own
header states the split without ambiguity:

> *"The combinator / `unfoldUpTo` / `sort` views are declared here (driving the
> recursive-view-through-SCT wiring in `elab.rs`); the prelude (`prelude.rs`)
> supplies the types + Ω constants."*

⇒ **`prelude.rs` supplies `data List a = Nil | Cons a (List a)` (`:428`) and no
operation over it.** A user program that imports the prelude has a list type and
no way to map over it. Measured: of the 135 `elaborate_decl` calls in
`prelude.rs`, the declared names are data types plus four `const`s — there is no
`map`, `fold`, `zip`, or `filter` among them.

| what | where |
|---|---|
| the required deliverable | `spec/30-surface/37-strings-collections.md §9` |
| the three combinators, test-local | `tests/l3a_acceptance.rs:26-45` |
| `filter`'s deferral and its stated reason | `tests/l3a_acceptance.rs:12-15` |
| the prelude's `List`, with no operations | `src/prelude.rs:428` |
| the prelude's authoring idiom | `src/prelude.rs`, `elaborate_decl` string per decl |
| SCT wiring that makes the recursion legal | `src/elab.rs:5705`, `declare_recursive_group` / `sct_check` |

## `filter`'s deferral reason has been removed, and that is half this node

`l3a_acceptance.rs:12-15` defers `filter` and says exactly why:

> *"`filter` is deferred — it needs Boolean branching, but `Bool` is an opaque
> primitive (not `data Bool = True | False`), so it is not pattern-matchable,
> and a CBV `if` primitive would double-evaluate a recursive branch — a separate
> change (tracked follow-on)."*

**Both halves of that are now false, and the follow-on was never tracked.**

`Bool` is pattern-matchable today. Measured at
`tests/case_eq_dependent_match_sugar.rs:106`, which matches a `Bool` **binder**
directly:

```
theorem wrong (b : Bool) : Equal Bool b b = match b eqn: h { True |-> h ; False |-> h }
```

⇒ **`filter` needs no `if` primitive and has no double-evaluation problem.** It
is an ordinary `match` on the predicate's result, in the same shape as `map`.
`LANG-SURFACE-IF` (merged) is what moved this; its own framing named *"real
matchable `data Bool = True | False`"* as `if`'s elaboration target.

**No node was ever filed for the promised follow-on.** A grep of
`docs/program/issues/` finds no `filter` node. This is the same shape as the
finding that produced `LANG-FOREIGN-NAME-FORMAT-CHARS`: an obligation recorded
in prose, in a file nobody re-reads, owned by nobody.

## The design call, front-loaded

**Lift the declaration text unchanged.** The exact strings in
`setup_combinators` already elaborate against a real `ElabEnv` built from the
prelude — that is what the L3a suite proves on every run. Moving them into
`prelude.rs` is therefore a placement change, not a re-derivation. **Do not
rewrite them into a different style while moving them**; a behaviour change and
a move in one diff cannot be told apart.

**`sort` is excluded, deliberately, and this is the boundary of the node.**
`l3a_acceptance.rs`'s AC6 has `sort` emit a conjoined `is_sorted ∧ Perm`
obligation. Putting `sort` in the prelude puts an **undischarged obligation** in
every program that loads the prelude, and an open obligation is admitted as a
postulate (`elab.rs:53-57`: the hole *"is admitted as a postulate in the kernel
(`trusted_base()` membership = `unknown` status)"*). **That is a trusted-base
change and it is not a build call.** It is live with the operator. Ship the
combinators that carry no obligation; leave `sort`, and say so in the node.

`unfoldUpTo` is likewise out of scope — no obligation, but it is the
infinitude idiom rather than a combinator, and it has its own AC4.

## Deliverables

- **D1** — `map`, `fold`, `zip` in `prelude.rs`, text unchanged from
  `setup_combinators`, in the prelude's existing `elaborate_decl` idiom.
- **D2** — `filter`, newly written, matching on the predicate's `Bool` result in
  the same recursive shape as `map`. No `if`, no new primitive.
- **D3** — `l3a_acceptance.rs` stops declaring its own `map`/`fold`/`zip` and
  uses the prelude's. **This is the deliverable that proves D1 shipped**; a
  prelude copy alongside a surviving test-local copy is not a move.
- **D4** — delete the `filter`-is-deferred paragraph from the file header, since
  its two stated reasons are false and it is the artifact a later reader
  inherits.

## Acceptance criteria

- **AC-1 — the combinators are reachable from a bare prelude env.** In a **new**
  test, build an `ElabEnv::new()` and nothing else, then elaborate a declaration
  that **applies** `map`, `fold`, `zip` and `filter`. No `elaborate_decl` of the
  combinators themselves in the test. This is the criterion the node exists for:
  it fails today for all four.
- **AC-2 — `filter` computes, not merely elaborates.** Evaluate a `filter` over a
  literal list with a predicate that rejects at least one element and **assert
  the resulting list's structure**, in the idiom `l3a_acceptance.rs` already uses
  for AC2/AC5 (`eval` / `whnf` to a value). A type-check alone would pass for a
  `filter` that returns its input unchanged.
- **AC-3 — no test-local redeclaration survives.** `setup_combinators` no longer
  declares `map`, `fold`, or `zip`. State the resulting line count of that
  function. If it still declares them, D3 did not happen and the prelude copy is
  dead weight.
- **AC-4 — the L3a suite passes with no assertion amended.** Every existing
  assertion in `l3a_acceptance.rs` holds against the prelude-supplied
  combinators. Changing an assertion while moving the thing it asserts about
  forfeits the control. Adding tests is fine; amending one is a stop.
- **AC-5 — the trusted base does not grow.** Report `trusted_base()`'s size
  before and after. It must be **equal**. This is the criterion that keeps the
  `sort` exclusion honest — if it moved, an obligation-emitting declaration
  reached the prelude and the node overran its boundary.
- **AC-6 — no new red in CI.** Targeted locally: `-p ken-elaborator`. Never
  `--workspace` on the box.

- **AC-7 — a one-character rider in `error.rs`, added mid-flight 2026-08-13.**
  Adversary finding at `evt_7bt65xjjfsgr0`, verified by the Steward at
  `424ab5da`. `error.rs:256-260` formats a bare `char` with `Display`:

  ```rust
  ElabError::NonAsciiIdentifierCharacter { character, span } => write!(
      f,
      "non-ASCII identifier character '{}' at {}-{}: identifiers are ASCII-only",
      character, span.start, span.end,
  ),
  ```

  **Change `'{}'` to `{:?}`.** The correct precedent is seven lines below in the
  same `match`: `ForeignNameControlCharacter` (`:264-271`) already uses `{:?}`
  on its `character`, for exactly this reason.

  **Why it matters more than a formatting nit.** This variant's population is
  precisely the invisible characters — it fires on any non-ASCII character in an
  identifier, which includes C1 controls, `Cf` format characters (U+200B,
  U+FEFF, ZWJ) and bidi overrides (U+202E). So a zero-width character yields
  `non-ASCII identifier character '' at 12-13`, with nothing between the quotes
  and no way for the author to know what to delete; and U+202E is emitted raw
  into the message, **reordering the rest of the terminal line including the
  span numbers.** ⇒ The diagnostic most likely to be triggered by an invisible
  character is the one that prints it raw.

  Verified: this is the **only** site in `error.rs` that prints a bare `char`
  with `{}`. Every other `'{}'` in a `Display` arm formats a `String`, where it
  is correct.

  **This does not wait on the `Cf` threat-model question** that has
  `LANG-FOREIGN-NAME-FORMAT-CHARS` held. That question is about deceiving a
  reader of *source*; this is the compiler's own *output*, where a bidi override
  reorders the message whatever the source policy turns out to be. Different
  artifact, decidable now.

  Severable: if it costs more than the one character plus a test, say so and
  drop it rather than growing the node.

## Not this node

- **`sort`.** Excluded above on the trusted-base argument, which is an operator
  call, not a sizing preference. Do not fold it in because it is nearby.
- **`unfoldUpTo`**, `Array`, `DecEq`/`Ord` instances, or the combinator **laws
  as propositions**. All are `37 §9` and all are separate; the laws in particular
  need a discharge route that is Verify's lane, not a prelude edit.
- Changing what any combinator **means**, its argument order, or its
  implicit/explicit binder structure. The L3a text is the pinned shape.
- Making `Bool` into anything it is not already. The measurement is that it is
  already matchable; this node consumes that, it does not extend it.
- **Anything in `error.rs` beyond `AC-7`'s single format specifier.** Not the
  `Cc` control-character check, not `Cf`, not the identifier rule that raises
  `NonAsciiIdentifierCharacter`, and not any other variant's wording. The rider
  changes how one existing character is rendered and nothing about when the
  error fires or what it covers.

## Contention

`crates/ken-elaborator/src/prelude.rs`,
`crates/ken-elaborator/tests/l3a_acceptance.rs`, and — from the `AC-7` rider
added mid-flight — `crates/ken-elaborator/src/error.rs`. All three are
Language-owned. Runtime is in `crates/ken-runtime`; Verify's lane is
`src/prover.rs`. No other ring holds any of them. `prelude.rs` is loaded by
every elaborator test, so a break there is broad — which is why AC-4 pins the
existing suite rather than sampling it.

**`error.rs` is in scope for `AC-7`'s one-character change only.** Nothing else
in that file is this node's, and the `Cf`/`Cc` checks themselves remain out —
see "Not this node".
