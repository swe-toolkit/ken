---
id: CI-L1-EXECUTING-COVER
title: "Two executing, green l1_acceptance rows certify conformance cases they cannot check -- sec62 stands for a soundness row whose discriminator it never queries, and sec61 covers half of a row while its doc comment asserts the half the row denies"
status: draft
owner: verify
size: S
gate: none
depends_on: [CI-ASSERTIONLESS-L1]
blocks: []
github: null
origin: Architect rejection of CI-ASSERTIONLESS-L1 respin dec_7yn4qg6q05t8n (rejected 2026-08-10T04:21:58Z), which found that the candidate's replacement header "overclaims conformance cover for neighboring executing rows whose expectations remain unbound" and directed that the header be narrowed rather than widened. Independently re-measured by the Steward at origin/main 69b1504b against conformance/surface/numbers/seed-numbers.md. Filed as its own node because CI-ASSERTIONLESS-L1 has been rejected three times and widening a thrice-rejected candidate is how it never lands. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## STATUS IS `draft` -- it needs a frame, not a decision
>
> The measurement below is complete and was taken against the seed, not
> inferred from the tests. What is owed before release is a frame under
> `docs/program/wp/`: deliverable cut, AC controls, and the guardrail that
> keeps this from repeating `CI-ASSERTIONLESS-L1`'s three-rejection loop.
>
> **`depends_on: [CI-ASSERTIONLESS-L1]` is file contention, not logic.** Both
> nodes edit `crates/ken-interp/tests/l1_acceptance.rs`. This node must not
> start until that one merges.

## Why this exists

`CI-ASSERTIONLESS-L1` was cut for four rows: three honest `#[ignore]`d
placeholders and one live assertion-free row. Its repair rewrote the file
header to distinguish executing cover from non-cover, and in doing so
certified two **executing, green** rows that do not earn the certificate.

**These two are a worse instance than the four that node covers.** An empty
`#[ignore]`d placeholder advertises that it checks nothing. A green assertion
is read as evidence the property holds.

## The measurement

Taken at `origin/main 69b1504b` against
`conformance/surface/numbers/seed-numbers.md`. The seed is the authority here,
not the test's own doc comment -- that is the whole point.

### `sec62_abstract_add_is_neutral` -- vacuous against a soundness row

`crates/ken-interp/tests/l1_acceptance.rs:315`, standing for
`surface/numbers/algebraic-law-is-proposition-not-reduction  (soundness)`
(`seed-numbers.md:247`).

The seed row's `given` is **the conversion query `a + b ≟ b + a`**, and its
`expect` is that **kernel conversion rejects it**. The row then names its own
bug model verbatim:

> "Under the exact bug this targets -- registering an algebraic law as a kernel
> reduction (or making conversion accept it) -- `a + b ≡ b + a` would be
> **accepted** and this case **flips**."

The test's entire assertion is:

```rust
assert_ne!(result_ab.def_id, result_ba.def_id, "a+b and b+a are distinct definitions");
```

Two separately-elaborated `fn` declarations have distinct `def_id`s. **That is
true of any two declarations, including two with identical bodies.** No
conversion query is issued anywhere in the test.

⇒ **Under the exact bug the row targets, `sec62` stays green.** It is a
discriminator with no discriminating power over a row the seed marks
`(soundness)` and whose guard is described as the TCB line (`35 §6.2`).

The test says so itself, in-source: *"Testing kernel conversion directly
requires the kernel API; simplified here to structural evaluation
observation"* and *"We can't easily drive this without the kernel conversion
API... For now: verify that..."*. **The admission is present and the cover
claim was made anyway** -- the same shape `CI-ASSERTIONLESS-L1` exists to
eliminate.

### `sec61_literal_reduces_in_kernel` -- half a row, denied by its own comment

`crates/ken-interp/tests/l1_acceptance.rs:297`, standing for §6.1, whose seed
row is `surface/numbers/primitive-op-runtime-value-k3-conversion-deferred`
(`seed-numbers.md:225`).

That row has two halves:

| seed half | test |
|---|---|
| "the **real interpreter** evaluates `add_int 2 3` to `Int 5`" | checked -- `eval_def` through `ken_interp` |
| "Kernel conversion does not... the application remains neutral and the equality does **not** close by `Refl`" | not checked |

The row further marks a positive conversion oracle **DEFERRED/RED-UNTIL-K3**.

**The test's doc comment states the opposite of the seed's second half:**
*"`2 + 3 : Int` reduces to `5` definitionally in the kernel evaluator."* The
seed says kernel conversion specifically does **not** reduce `add_int`, because
`add_int` is a `PrimReduction::Op` rather than a `Literal`.

⇒ The assertion is real and correct about the interpreter. **The cover claim
and the doc comment are about the kernel, and they are wrong in the direction
that would hide a K3 boundary change.**

## What this node is NOT

- **Not a repair of the four `CI-ASSERTIONLESS-L1` rows.** Those are that
  node's, and it is mid-flight on a fourth SHA.
- **Not a capability build.** Unlike the three severed rows, both of these
  have their capability today: the kernel conversion API exists, and the
  `PrimReduction::Literal`/`Op` distinction is landed and spec'd.
- **Not a licence to widen the header again.** The standing instruction on
  `CI-ASSERTIONLESS-L1` is that the file header stops certifying per-row
  conformance cover entirely. This node must not reintroduce a hand-maintained
  cover enumeration -- three rewrites got three different subsets wrong.

## Deliverables -- provisional, to be fixed by the frame

- **`D1` -- `sec62` issues the conversion query the row names**, or is severed
  and marked with the capability it waits on. Severing is a legitimate
  outcome only if the conversion query is genuinely not reachable from the
  test tree; that is a measurement, not an assumption, and the frame must
  require it be taken.
- **`D2` -- `sec61`'s doc comment and cover claim are corrected**, and the
  unchecked half is either asserted or recorded as uncovered in the registry.
- **`D3` -- the registry (`.github/ignored-test-exemptions.toml`) accounts for
  whatever ends up uncovered.** An executing green test that does not cover
  its row is invisible to the ignored-test sweep by construction; if that
  residual has no home, this node has moved the gap rather than closed it.

## Acceptance -- the control must flip under the row's own bug model

The seed hands us the discriminator for free, and any AC that does not use it
repeats the defect.

| AC | criterion |
|---|---|
| `AC-1` | `sec62`'s replacement **fails** when kernel conversion is made to accept `a + b ≡ b + a`. The seed names this as the exact bug; a control that cannot flip under it is another `assert_ne!`. |
| `AC-2` | No test or comment in `l1_acceptance.rs` asserts kernel-definitional reduction of a `PrimReduction::Op`. |
| `AC-3` | Every row that remains uncovered after this node is named in the registry with its reason, and the sweep resolves it. |
| `AC-4` | The file header still makes no per-row conformance-cover enumeration. Inherited guardrail, and it is the one this node is most likely to break. |

## Validation -- targeted only

Never `--workspace` (operator, `agent/COORDINATION.md §12`).
`-p ken-interp --test l1_acceptance`, plus
`PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py`.
Workspace, `--locked`, and conformance run in CI.
