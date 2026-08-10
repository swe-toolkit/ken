---
id: CI-L1-EXECUTING-COVER
title: "Three executing, green l1_acceptance rows certify conformance cases they cannot check -- sec62 never issues the conversion query its soundness row turns on, sec61 names a row id that does not exist, and ac5_no_implicit_cross_type_coercion is satisfied by an elaboration limitation rather than by the coercion refusal it claims"
status: ready
owner: verify
size: M
gate: none
depends_on: [CI-ASSERTIONLESS-L1]
blocks: []
github: null
origin: Architect rejection of CI-ASSERTIONLESS-L1 respin dec_7yn4qg6q05t8n (rejected 2026-08-10T04:21:58Z), which found that the candidate's replacement header "overclaims conformance cover for neighboring executing rows whose expectations remain unbound" and directed that the header be narrowed rather than widened. Independently re-measured by the Steward at origin/main 69b1504b against conformance/surface/numbers/seed-numbers.md. Filed as its own node because CI-ASSERTIONLESS-L1 has been rejected three times and widening a thrice-rejected candidate is how it never lands. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## WIDENED 2026-08-10 — a third row and a machine check folded in
>
> **`ac5_no_implicit_cross_type_coercion` is a third row on this node**, from
> Adversary finding `evt_34q2zm16a48pz` on `65a61416`, corroborated by the
> Steward at `98c3e0fc`. It is the worst of the three: `sec62` reaches its
> mechanism and fails only under a specific bug, while `ac5` never reaches its
> mechanism at all -- its `is_err()` is satisfied by `elaborate_decl_v1`'s
> inability to elaborate an un-annotated `fn`, which the matching-type positive
> control fails identically. **Ken's behaviour is correct; the defect is
> entirely in the instrument.**
>
> **Verifying that finding surfaced a fourth defect the frame had wrong.**
> `sec61` claims `surface/numbers/literal-reduces-in-kernel` -- **a row id that
> exists in no markdown file in the repo.** Meanwhile the real seed row it
> should serve, `primitive-op-runtime-value-k3-conversion-deferred`, has zero
> claims anywhere in the code tree. The frame's original §3d supplied that
> mapping itself and called it "covers half a row"; the artifact makes no such
> mapping. §3d now states the measurement instead.
>
> **`D5` adds the check that would have caught all of this**: every `///
> surface/...` row id on a test must resolve to a `### <id>` heading in a
> `conformance/` seed. Resolution only -- cover adequacy stays human judgment.
> The certification form is a **bare row id**, which is why the Adversary's
> first keyword sweep for "cover"/"conformance"/"certify" came back clean and
> was wrong.
>
> **Re-sized S to M.** Three rows, five deliverables.
>
> ## FRAMED 2026-08-10 — [`docs/program/wp/CI-L1-EXECUTING-COVER.md`][f]
>
> `ready` per `steward.md §4e`: a successor must be shovel-ready **while** its
> predecessor is in flight, so the frontier advances on the merge with no
> Steward pass in between. **`ready` is NOT released** -- both lanes are
> occupied under the two-lane cap (Runtime `RT-DYNAMIC-ARM-SCALAR-MERGE`,
> Foundation `DS-9`) and this node has not been kicked.
>
> **`depends_on: [CI-ASSERTIONLESS-L1]` was file contention, not logic, and it
> is DISCHARGED.** That node merged at `3d6622c9`; the edge is satisfied. Cut
> from current `origin/main`.
>
> **The frame's §3c is the load-bearing audit and it forecloses the easy exit.**
> `ken_kernel::convert` is public and re-exported at the crate root, the
> `Context::new()` + `push` idiom is landed in `k2c_conversion.rs`, and
> `ken-interp` already depends on `ken-kernel`. So `sec62` is writable against
> landed public API, and **"sever and mark" is not an available disposition** --
> unlike the three severed rows in the sibling node, this one waits on no
> unbuilt capability.
>
> [f]: ../wp/CI-L1-EXECUTING-COVER.md

## Why this exists

`CI-ASSERTIONLESS-L1` was cut for four rows: three honest `#[ignore]`d
placeholders and one live assertion-free row. Its repair rewrote the file
header to distinguish executing cover from non-cover, and in doing so
certified **executing, green** rows that do not earn the certificate.

**These are a worse instance than the four that node covers.** An empty
`#[ignore]`d placeholder advertises that it checks nothing. A green assertion
is read as evidence the property holds.

**The certification form is a bare row id** in a doc comment (`///
surface/numbers/<row>`), eleven of them on executing tests. That population is
what the file-header repair does not reach, and nothing measures it -- neither
that a claimed id resolves, nor that the assertion beneath it discriminates.
Three of the eleven fail, for three different reasons.

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

### `sec61_literal_reduces_in_kernel` -- claims a row id that does not exist

Search for `fn sec61_literal_reduces_in_kernel`. Its doc comment claims
`surface/numbers/literal-reduces-in-kernel`. **Measured at `98c3e0fc`: that
string appears in no markdown file anywhere in the repo.** The seed defines
fourteen rows and that is not one of them.

Symmetrically, `surface/numbers/primitive-op-runtime-value-k3-conversion-`
`deferred` -- the row the seed's own Cases list routes §6.1 to -- has **zero**
claims in the code tree (no `.rs`, `.toml`, or `.py` reference). **A real seed
row is uncovered and a phantom id is covered.**

The paragraph below was the original statement here, and it assumed the
mapping the artifact does not make. It is retained because it is why a rename
alone is not the fix: the k3 row's second half is unchecked, so repointing the
id would move a false claim rather than retire it.

That row (`seed-numbers.md:225`) has two halves:

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

### `ac5_no_implicit_cross_type_coercion` -- never reaches its mechanism

Adversary finding on `65a61416` (`evt_34q2zm16a48pz`), corroborated by the
Steward at `98c3e0fc`. Search for `fn ac5_no_implicit_cross_type_coercion`.
It claims `surface/numbers/no-implicit-cross-type-coercion`
(`seed-numbers.md:168`), a **reject** row: *"a type error; the operands
disagree and there is no widening coercion to make them agree."*

Its entire assertion is `result.is_err()` on
`fn f (x : Int) (y : Int64) = x + y`. The error it actually gets:

```
TypeMismatch { span: 29..34, reason: "cannot infer type of lambda without annotation" }
```

Span `29..34` is the body. **The positive control settles it:** the same
declaration with **matching** types, `fn f (x : Int) (y : Int) = x + y`, which
must be legal, fails with the identical error. `elaborate_decl_v1` cannot
elaborate an un-annotated `fn`, so the type relationship is never reached.

Steward corroboration from the artifact rather than a run: of the ten `fn`
probes in the file, **every un-annotated one is either `#[ignore]`d or asserted
to fail -- not one executes and succeeds.** Exactly the distribution the
control predicts.

⇒ **The row would stay green if Ken grew a silent implicit `Int`-to-`Int64`
coercion tomorrow.** Adding the return-type annotation makes the declaration
elaborate and the real kernel refusal fire (`TypeMismatch { expected: g6,
found: g10 }`), so the repair is one token plus a genuine accept/reject pair.

**Ken's behaviour is correct. This is not a coercion hole -- it is a row with
no evidence, and the defect is entirely in the instrument.**

Nothing caught it because the negative check has no positive control *by
construction*: its accepting counterpart
`ac5_explicit_conversion_is_partial_option` is `#[ignore]`d and
registry-exempt, so the accept arm is structurally absent from CI. And because
this test executes green, `CI-IGNORED-SWEEP` cannot see it.

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
  `D5` is the opposite move: a machine check, not a prose inventory, and
  deliberately scoped to the decidable half.
- **Not an audit of the other eight executing row-id claims.** The Adversary
  swept them and found them sound. That is a fixed input; `D5` covers their ids
  mechanically.
- **Not a soundness finding.** All three rows measure a correct Ken. Every
  defect here is in the instrument.

## Deliverables -- the frame is authoritative, this is the summary

**The frame carries five deliverables and seven ACs.** The list below is the
original three; `D4` (the `ac5` repair) and `D5` (the row-id resolution check)
are stated in the frame's §4 and are not restated here.

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

**The frame carries three more**: `AC-5` (only the three named tests changed
behaviour), `AC-6` (the `ac5` repair flips under a mutation that grants the
widening its row denies, *and* a positive control showing the pre-repair form
still passes), and `AC-7` (the row-id checker reds on a fabricated id and
reports how many ids it resolved -- a checker that resolves zero also passes).

## Validation -- targeted only

Never `--workspace` (operator, `agent/COORDINATION.md §12`).
`-p ken-interp --test l1_acceptance`, plus
`PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py`.
Workspace, `--locked`, and conformance run in CI.
