# WP CI-L1-EXECUTING-COVER — three executing rows that certify what they cannot check

**Owner:** Team Verify. **Branch:** `wp/CI-L1-EXECUTING-COVER`.
**Node:** [`docs/program/issues/CI-L1-EXECUTING-COVER.md`][n].
**Steward frame.** Size M. Gate: none.

> **Re-sized S to M on 2026-08-10** when the third row (§3e) and the machine
> check (`D5`) were folded in. The original cut was two rows and three
> deliverables; it is now three rows and five.

> **Treat every anchor in this frame as perishable. If a fixed input turns out
> false against the landed code, say so and escalate -- do not quietly build
> around it.** Sections 3a-3d were measured at `origin/main faabc2ed`; sections
> 3e and 3f and the deliverables and ACs that follow from them were measured at
> `origin/main 98c3e0fc`. The deliverable edits some of the very code it cites.
>
> **Coordinates are given as grep-able phrases, not line numbers**, because this
> WP rewrites the lines it talks about. Where a number appears, it is an anchor
> to re-find, never a value to check.

## 1. Objective

Three tests in `crates/ken-interp/tests/l1_acceptance.rs` execute, pass, and are
counted as conformance cover for rows they cannot check. Make each one either
check its row or stop claiming it, and add the machine check that would have
found them.

**The certification form is a bare row id in a doc comment** -- `///
surface/numbers/<row>` above a `#[test]`. Eleven of them sit on executing tests.
That population is what `CI-ASSERTIONLESS-L1`'s file-header repair does not
reach, and it is unmeasured: nothing checks that a claimed id resolves to a seed
row, and nothing checks that the assertion beneath it discriminates.

## 2. Why this is a separate node from `CI-ASSERTIONLESS-L1`

`CI-ASSERTIONLESS-L1` was cut for four rows -- three honest `#[ignore]`d
placeholders and one live assertion-free row -- and has been **rejected three
times**, each time on the file header's cover claim. The Architect's third
rejection (`dec_7yn4qg6q05t8n`, 2026-08-10) found these two additional rows and
directed that the header be **narrowed, not widened**.

**These are the worse instance.** An `#[ignore]`d placeholder advertises that it
checks nothing. A green assertion is read as evidence the property holds.

The third row (`ac5`, §3e) and the phantom row id (`sec61`, §3d) arrived after
that node merged, from the Adversary's `65a61416` pass and the Steward's
verification of it. They are folded here rather than filed separately: same
file, same owner, same defect class, and two nodes editing
`l1_acceptance.rs` concurrently would collide.

## 3. Fixed inputs — settled, do NOT reopen

### 3a. The seed is the authority, not the test's doc comment

That is the entire point of this node. Every claim about what a row requires is
read from `conformance/surface/numbers/seed-numbers.md`.

### 3b. `sec62` is vacuous against a soundness row — MEASURED

Search `l1_acceptance.rs` for `fn sec62_abstract_add_is_neutral`. It stands for
the seed row headed
`surface/numbers/algebraic-law-is-proposition-not-reduction  (soundness)`.

- The row's `given` is **the conversion query `a + b ≟ b + a`** on abstract
  `a b : Int`.
- The row's `expect` is that **kernel conversion rejects it**.
- The row names its own bug model: *"Under the exact bug this targets --
  registering an algebraic law as a kernel reduction (or making conversion
  accept it) -- `a + b ≡ b + a` would be accepted and this case flips."*

The test issues **no conversion query**. Its whole assertion compares the
`def_id`s of two separately-elaborated `fn` declarations and requires them to
differ -- true of any two declarations, including two with identical bodies.

⇒ **Under the exact bug the row targets, `sec62` stays green.**

### 3c. The capability EXISTS, so severance is NOT an available disposition

This is the audit that decides the deliverable, and it came out the way that
closes the easy exit. Measured at `faabc2ed`:

| needed | landed |
|---|---|
| a conversion query callable from a test | `pub fn convert(env, ctx, ty, a, b) -> bool` in `crates/ken-kernel/src/conv.rs`, **re-exported at the crate root** (`pub use conv::{convert, convert_type, level_eq, normalize, whnf}`) |
| a context holding abstract operands | `Context::new()` then `ctx.push(ty)` per binder -- the idiom used throughout `crates/ken-kernel/tests/k2c_conversion.rs` |
| the crate edge | `ken-interp/Cargo.toml` already depends on `ken-kernel`, and `l1_acceptance.rs` already has `use ken_kernel::{...}` |
| a `GlobalEnv` from the elaborator | `ElabEnv` exposes it; the landed `nc14_data_match_lowering.rs` reads `env.env.trusted_base()` |

⇒ **`sec62` is writable against landed public API.** The three severed rows in
`CI-ASSERTIONLESS-L1` wait on unbuilt capability; **this one does not**, so
"sever and mark" would be recording a capability gap that does not exist.

⛔ **Do not sever `sec62`.** If the ring believes it cannot be written, that is
a finding that contradicts this audit -- escalate it, do not route around it.

### 3d. `sec61` claims a row id that does not exist — MEASURED

**This supersedes a weaker statement in an earlier draft of this frame.**

Search for `fn sec61_literal_reduces_in_kernel`. Its doc comment claims
`surface/numbers/literal-reduces-in-kernel`.

**There is no such row.** Measured at `98c3e0fc`: the string
`literal-reduces-in-kernel` appears in **no markdown file anywhere in the
repo** -- not in `conformance/surface/numbers/seed-numbers.md`, not in `spec/`.
The seed defines fourteen rows and that is not one of them.

An earlier draft of this frame said the test's row "is"
`surface/numbers/primitive-op-runtime-value-k3-conversion-deferred` and that
the test covers half of it. **That was the Steward supplying a mapping the
artifact does not make.** The seed's own Cases list routes §6.1 to the k3 row,
so it is the row `sec61` *should* serve -- but the test does not name it, and
the second measurement is the sharp one:

> `surface/numbers/primitive-op-runtime-value-k3-conversion-deferred` has
> **zero** claims in the entire code tree -- no `.rs`, no `.toml`, no `.py`
> reference. A real seed row is uncovered, and a phantom id is covered.

The substance of the old §3d still holds and is the reason a rename alone is
not the fix. Against the k3 row:

| seed half | test |
|---|---|
| "the **real interpreter** evaluates `add_int 2 3` to `Int 5`" | checked, via `eval_def` |
| "Kernel conversion does not... the application remains neutral and the equality does **not** close by `Refl`" | not checked |

The row marks a positive conversion oracle **DEFERRED/RED-UNTIL-K3**. The
test's doc comment prose says *"reduces to `5` definitionally in the kernel
evaluator"* -- **the opposite of the seed's second half**, because `add_int` is
a `PrimReduction::Op` rather than a `Literal`.

The assertion is correct about the interpreter. **The id, the prose, and the
cover claim are about the kernel, and all three are wrong in the direction that
would hide a K3 boundary change.**

### 3e. `ac5_no_implicit_cross_type_coercion` reaches its mechanism NEVER — MEASURED

Adversary finding on `65a61416` (`evt_34q2zm16a48pz`), independently
corroborated by the Steward at `98c3e0fc`. **This is the worst of the three:
`sec62` reaches its mechanism and fails only under a specific bug; this one
never reaches its mechanism at all.**

Search for `fn ac5_no_implicit_cross_type_coercion`. It claims
`surface/numbers/no-implicit-cross-type-coercion` -- a **reject** row whose
`expect` is *"a type error; the operands disagree and there is no widening
coercion to make them agree."* Its entire assertion is `result.is_err()` on
`fn f (x : Int) (y : Int64) = x + y`.

**The error it gets is not a coercion refusal.** Adversary measurement:

```
TypeMismatch { span: 29..34, reason: "cannot infer type of lambda without annotation" }
```

Span `29..34` is the body `x + y`. The positive control settles it: the same
declaration with **matching** types, `fn f (x : Int) (y : Int) = x + y`, which
must be legal, fails with the **identical** error. `elaborate_decl_v1` cannot
elaborate an un-annotated `fn` at all, so the type relationship is never
reached.

**Steward corroboration, from the artifact rather than a run.** Every `fn`
declaration probe in the file sorts perfectly along the annotation line:

Reproduce the population with `grep -n '"fn ' l1_acceptance.rs` -- ten probes:

| probes | return annotation | outcome |
|---|---|---|
| seven, including `ac2`'s retained legacy `fn f (x : Int64) : Int64 = x + 1` and `sec62`'s `add_ab`/`add_ba` | present | `.unwrap()`s, executing, green |
| `ac5_explicit_conversion_is_partial_option`, `sec31_int_div_zero_emits_obligation` | absent | `#[ignore]`d -- never run |
| `ac5_no_implicit_cross_type_coercion` | absent | asserts only `is_err()` |

**Every un-annotated `fn` probe in the file is either ignored or asserted to
fail. Not one executes and succeeds.** That is exactly the distribution the
Adversary's positive control predicts.

The consequence, stated as the property: **the row would stay green if Ken grew
a silent implicit `Int`-to-`Int64` coercion tomorrow.**

**Ken's behaviour is correct and this is not a soundness hole.** The annotated
mixed case *is* refused by the kernel. The defect is entirely in the
instrument.

**The repair is one token, and it was measured too:**

| declaration | result |
|---|---|
| `fn a (x : Int) (y : Int) : Int = x + y` | accepts |
| `fn b (x : Int) (y : Int64) : Int64 = x + y` | `KernelRejected { TypeMismatch { expected: g6, found: g10 } }` |
| `fn c (x : Int) (y : Int64) : Int = x + y` | same genuine refusal |

Annotated, the test becomes a real non-degenerate pair on a shared input --
accepted matching, rejected mixed -- exercising the refusal the row is about.

**Why nothing caught it.** The negative check has no positive control *by
construction*: its natural accepting counterpart
`ac5_explicit_conversion_is_partial_option` is `#[ignore]`d and
registry-exempt, so the accept arm is structurally absent from CI. And because
this test executes and is green, `CI-IGNORED-SWEEP` cannot see it either.

### 3f. The row-id population has no machine check

The three defects above are three different failures of one hand-maintained
prose certificate: eleven bare row ids, none of them checked against anything.
`CI-ASSERTIONLESS-L1` landed on exactly this principle for the file header --
stop certifying in prose, point at the machine-checked artifact -- and the
per-test population is the same class at smaller scope.

Two facts sharpen what the check must be. The Adversary's first sweep searched
for the words *cover*, *conformance*, and *certify*, **came back clean, and was
wrong**, because the certification form carries none of those words. And a
resolution check is decidable from two greps, while cover *adequacy* is the
per-row judgment this WP does by hand. **`D5` buys the decidable half only.**

## 4. Deliverables

- **`D1` — `sec62` issues the conversion query its row names.** Build the
  abstract context (`a b : Int`), form `a + b` and `b + a`, call `convert`, and
  assert it returns `false`. ⛔ Severance is excluded by §3c.
- **`D2` — `sec61` is made honest.** Correct the doc comment so it does not
  assert kernel-definitional reduction of a `PrimReduction::Op`. Then either
  assert the unchecked half (conversion does **not** close the equality by
  `Refl`) or record that half as uncovered. **Naming which of the two you did,
  and why, is part of the deliverable.**
  **`D2` includes the id.** `sec61` currently names a row that does not exist
  (§3d). Either point it at
  `surface/numbers/primitive-op-runtime-value-k3-conversion-deferred` and
  satisfy whatever that row requires of it, or remove the cover claim. **A
  rename alone is not available**: the k3 row's second half is unchecked, so
  renaming would move a false claim rather than retire it.
- **`D3` — the residual has a home.** An executing green test that does not
  cover its row is invisible to the ignored-test sweep by construction. If
  anything remains uncovered after `D1`/`D2`/`D4`, it is recorded in
  `.github/ignored-test-exemptions.toml` with its reason, and the sweep
  resolves it. ⛔ A node that moves the gap out of the header and into nothing
  has not closed it.
- **`D4` — `ac5_no_implicit_cross_type_coercion` reaches its mechanism.**
  Annotate the declaration so it elaborates, and make the test a
  **non-degenerate pair on a shared input**: the matching-type declaration
  accepts, the mixed-type declaration is refused. **Asserting `is_err()` on the
  annotated form alone does not discharge this** -- that is the same negative
  check one token further along, and §3e is a demonstration that a negative
  check with no positive control is satisfied by whatever fails first.
  Discriminate on the refusal, not merely on failure: the mixed case must be
  shown to fail *as a type mismatch between the operand types*, distinctly from
  the elaboration limitation §3e measured.
- **`D5` — the row-id claim population gets a decidable machine check.** Extend
  `scripts/test-ci-ignored-sweep.py`, or add a sibling checker wired into the
  same CI job, so that **every `/// surface/...` row id on a test resolves to a
  `### <id>` heading in a `conformance/` seed**, and fail CI otherwise. Scope is
  **resolution only** -- the id exists. **Do not attempt to check cover
  adequacy**; that is the per-row human judgment `D1`, `D2`, and `D4` perform,
  and a checker that claimed to do it would be the fourth hand-maintained
  certificate.

## 5. Acceptance criteria

The seed hands us the discriminator for free. An AC that does not use it
repeats the defect this node exists to fix.

| AC | property | control |
|---|---|---|
| `AC-1` | `sec62` can detect the bug its row names. | **Mutation, and it must flip.** Make kernel conversion accept `a + b ≡ b + a` (the row's own stated bug), run `-p ken-interp --test l1_acceptance`, and show `sec62` **RED**. Restore byte-identically (`git diff --quiet`) and show it GREEN. Report both directions; a green-only run proves nothing. |
| `AC-2` | No test or comment in `l1_acceptance.rs` asserts kernel-definitional reduction of a `PrimReduction::Op`. | Enumerate every doc comment in the file that mentions the kernel, and state per comment whether it survived, was corrected, or was deleted. ⛔ Not "reviewed the file" -- a per-comment list. |
| `AC-3` | Every row left uncovered is named in the registry with its reason. | `PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py` green, and the census and registry subtraction agree. |
| `AC-4` | The file header still makes **no** per-row conformance-cover enumeration. | Inherited guardrail from `CI-ASSERTIONLESS-L1`, and **the one this node is most likely to break** -- the natural instinct on fixing a row is to go write in the header that it is now covered. Quote the header verbatim in the handback. |
| `AC-5` | `sec61`, `sec62`, and `ac5_no_implicit_cross_type_coercion` are the only tests whose behaviour changed. | `git diff origin/main...HEAD -- crates/ken-interp/tests/l1_acceptance.rs`, and name every other hunk with its justification. |
| `AC-6` | `ac5_no_implicit_cross_type_coercion` can detect the bug its row names. | **Mutation, and it must flip.** Make the elaborator accept a widening `Int` to `Int64` on a bare `+` -- the exact hole the row denies exists -- and show the test **RED**. Restore byte-identically (`git diff --quiet`) and show it GREEN. **Separately, run the accepting arm's assertion against the *pre-repair* un-annotated form and show it still passes** -- that is the positive control proving the repair changed something, and §3e is why a green-only report is worthless here. |
| `AC-7` | The row-id checker fails on a claim that does not resolve. | Point one test's `///` id at a fabricated row, run the checker, show it **RED** naming that test and that id. Restore and show GREEN. **Then run it unmutated against the tree as delivered and report the count of ids checked** -- a checker that resolves zero claims also passes. |

## 6. Guardrails — do not reopen

- **Do not reopen the four `CI-ASSERTIONLESS-L1` dispositions.** Named exactly,
  because an earlier draft of this line wrote the first one as a bare `ac5_...`
  prefix and thereby forbade `D4`: they are
  `ac5_explicit_conversion_is_partial_option`,
  `sec31_int_div_zero_emits_obligation`, `sec24_char_excludes_surrogates`
  (the three severed and registry-exempt), and
  `ac2_expected_type_overrides_default` (the AC-2 production-fed seam).
  **`ac5_no_implicit_cross_type_coercion` is a different test and is IN scope
  as `D4`.** That node has merged, so this is no longer a contention
  constraint -- it is a settled-disposition constraint. Touching those four
  means re-arguing something already reviewed four times.
  **One exception, and it is the one connection worth having:** readmitting
  `ac5_explicit_conversion_is_partial_option` on L-classes is *also* what would
  supply the accept arm `ac5_no_implicit_cross_type_coercion` structurally
  lacks. If `D4`'s accepting arm can be built without L-classes, build it; if
  you conclude it cannot, that is a finding, and the registry's readmission
  text for that exemption should record that one capability closes both.
- ⛔ **Do not reintroduce a per-row cover enumeration in the header.** Three
  rewrites produced three different wrong subsets. The machine-checked artifact
  is the registry.
- **Do not widen scope to other `l1_acceptance.rs` rows.** The scope is exactly
  the three named in §3b, §3d, §3e. **Report a fourth and leave it** -- that is
  a finding for the Steward, and widening is how the sibling node reached three
  rejections.
  **This guardrail already worked once and that is why the scope grew by
  Steward action rather than by drift.** `ac5` was found by the Adversary,
  reported, and folded here deliberately. Do the same with the next one.
  The Adversary's sweep found the other eight executing row-id claims sound
  (`ac1` asserts exact *and* not-f64-rounded, `ac4_explicit` asserts the
  modular value, `ac3` asserts obligation kind plus open hole, `ac6`'s
  decimal/float halves pair on their shared row). **That is a fixed input, not
  an invitation to re-audit them** -- `D5`'s checker will cover their ids
  mechanically.
- ⛔ **No kernel change, and no new trusted declaration.** `D1` consumes
  `convert`; it does not modify conversion. A mutation for `AC-1` is reverted
  byte-identically and never committed.

## 7. Contention

**The `CI-ASSERTIONLESS-L1` serialization is DISCHARGED.** That node merged at
`3d6622c9` and is on `main` as of `65a61416`; the file contention that produced
the `depends_on` edge is gone. Cut from current `origin/main`.

Live lanes as of `98c3e0fc`, neither touching `crates/ken-interp`:

- Runtime `RT-DYNAMIC-ARM-SCALAR-MERGE` -- `crates/ken-elaborator/src`,
  `crates/ken-runtime`, `crates/ken-host`.
- Foundation `DS-9` -- `library/` `.ken.md` packages and, by the standing `DS-*`
  pattern, an acceptance test under `crates/ken-elaborator/tests/`.

**Re-derive this at branch time from the live lanes, not from this sentence.**
It is dated, and `D5` widens the touched set beyond `l1_acceptance.rs` to
`scripts/` and `.github/`, which is where a collision would now come from.

## 8. Validation — TARGETED ONLY

⛔ **Never `--workspace`** (operator, `agent/COORDINATION.md §12`).

```
scripts/ken-cargo test -p ken-interp --test l1_acceptance
PYTHONDONTWRITEBYTECODE=1 python3 scripts/test-ci-ignored-sweep.py
```

Plus whatever `D5`'s checker is invoked as, and its own self-test if you add
one. **`D5` must run in the same CI job as the ignored-test sweep**; a checker
that exists in `scripts/` and is wired to nothing is a fourth uncheckable
certificate with extra steps.

Workspace, `--locked`, and the conformance suite run **in CI**. A
"no-regression" criterion here means green in CI, never a local workspace run.

## 9. Reporting

Hand back the exact SHA, the paths touched, the `AC-1` and `AC-6` mutation
evidence in **both** directions, the `AC-7` red/green plus the count of ids
checked, the `AC-2` per-comment list, and the verbatim header for `AC-4`.

**State two choices explicitly** rather than leaving a reader to infer them
from the diff:

1. **`D2`** -- did you repoint `sec61`'s id at the k3 row and satisfy it, or
   retire the cover claim?
2. **`D4`** -- how is the accepting arm built, and if it could not be built
   without L-classes, say so as a finding.

**Sizing.** This is now five deliverables. Slice it so each turn reaches a
releasable increment or a genuine hard stop within about an hour --
`D1`+`AC-1`, then `D2`+`D4`, then `D5`+`D3` is a natural cut, and the Verify
leader owns the actual slicing.

[n]: ../issues/CI-L1-EXECUTING-COVER.md
