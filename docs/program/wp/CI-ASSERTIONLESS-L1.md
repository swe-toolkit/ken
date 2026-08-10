# CI-ASSERTIONLESS-L1 — four conformance claims their cover does not check

Owner: Team Verify. Size: S. Gate: none. Depends on: `CI-IGNORED-SWEEP`.

Frame authored by the Steward. All fixed inputs measured at
`origin/main = a22f1a873357f5b854b15290c9d3893173a2d1dc`.

> **Treat every anchor in this frame as perishable.** If a fixed input turns
> out false against the landed code, say so and escalate — do not quietly
> build around it. Read every line number as an anchor to re-find by the
> phrase beside it, never as a value to check.

## 0. The property, stated before any mechanism

**A test that names a conformance claim must check what that claim's `expect:`
clause says.**

That is the whole node. It is deliberately *not* "every test contains an
`assert`" — that is a spelling, it has legitimate counterexamples (a
must-not-panic smoke test correctly has none), and stating it that way is what
made the first draft of this node unbuildable. Name the property; the artifact
is downstream of it.

## 1. Fixed inputs

### 1a. The population, and how to re-derive it

`crates/ken-interp/tests/l1_acceptance.rs` holds **17** `#[test]` functions.
Exactly **4** contain no `assert*`, no `panic!`, and no `expect(`:

| test | claim id in its doc comment | `#[ignore]` | body |
|---|---|---|---|
| `ac2_expected_type_overrides_default` | `surface/numbers/expected-type-overrides-default` | **no** | 3 statements, no assertion |
| `ac5_explicit_conversion_is_partial_option` | `surface/numbers/explicit-conversion-is-partial-option` | yes | one `unwrap()` |
| `sec31_int_div_zero_emits_obligation` | **NONE — no doc comment at all** | yes | one `unwrap()` |
| `sec24_char_excludes_surrogates` | **NONE — no doc comment at all** | yes | comment only, zero statements |

> ### CORRECTED 2026-08-09 — this column was WRONG for the bottom two rows
>
> **Steward ruling `evt_7pc7a3she9sgj`, on Verify's measurement.** Re-measured
> at both `a22f1a87` and `4f88d383`: **only `ac2` and `ac5` carry a literal
> claim-id doc comment.** `sec31` and `sec24` carry **no doc comment at all** —
> they reach their claim through section labels, claim-shaped names, and the
> coverage map.
>
> **All four claims ARE registered** — every id resolves in `conformance/`, with
> `char-excludes-surrogates` in two files. So the **population is right and the
> stated linkage was wrong**, which is the worse of the two: a test can be
> counted as cover *without naming what it covers*, and severing a doc comment
> that does not exist discharges nothing.
>
> ⇒ **§1c and the "sever the claim-id doc comment" reasoning below apply to
> `ac2` and `ac5` only.** For `sec31` and `sec24` the marking must be positive —
> an explicit `Not conformance cover; waits on <capability>` — because there is
> no link to sever.

Re-derive the population by walking Rust items, not by regex — see §5.

**Correction to the originating issue node:** it recorded
`ac2_expected_type_overrides_default` as "7 statements". It is **3**. The
figure was a line count. Nothing else in that node's table changed between
`d75d8c48` and `a22f1a87`.

### 1b. All four claims are registered, and all four are in the coverage map

`conformance/surface/numbers/seed-numbers.md` registers each of the four ids
with a `spec:` / `given:` / `expect:` / `why:` entry, and its **"Coverage map
(AC → cases)"** section lists all four against a spec AC — `AC2`, `AC5`,
`§3.1`, and `§2.4` respectively.

⇒ **This is a conformance-integrity defect, not test hygiene.** The registry
says the claim is a case; the doc comment says the test covers the claim; the
body does not check it.

### 1c. Nothing mechanical binds a claim id to a test

Measured at `a22f1a87`: the claim ids appear only as `///` doc comments in
acceptance-test files. No crate, script, or workflow parses
`conformance/**/*.md` and matches ids against tests. The link is convention.

**Two consequences, and they point in opposite directions.** Nothing will ever
notice this on its own — so it must be fixed by hand. And nothing will break
when you sever a claim link — so severing one is a cheap, honest disposition
rather than a fight with a gate.

### 1d. The three ignored rows are blocked on genuinely unbuilt capability

Their `#[ignore]` reasons are accurate at `a22f1a87`: Char literal syntax is
not in the surface, integer division is not registered as an op, and explicit
conversions await L-classes. **Do not un-ignore them.** Un-ignoring converts a
silent non-test into a red for a reason that has nothing to do with this node.

## 2. The design judgment, front-loaded

### 2a. `ac2` is the only live false green, and it is NOT wholly vacuous

Its `unwrap()` is a real witness for **one** of the three things the claim's
`expect:` asserts — that elaboration succeeds with no ambiguity error. The
other two are unchecked: that `1` elaborates at `Int64`, and that the default
table does not fire.

**Its doc comment states the missing inference as a certainty:** *"If this
compiles, `1` was correctly elaborated at `Int64`."* That sentence is doing an
assertion's job in a medium nothing checks, and it is not obviously true —
`ac5_no_implicit_cross_type_coercion` pins that a *variable* of type `Int`
cannot meet an `Int64`, which is a different question from what an *unelaborated
literal* does. A literal that stays polymorphic and is solved to `Int64` by
unification satisfies the body while the defaulting-order property the claim
names never gets exercised.

**A trap, so the obvious fix is not attempted twice.** The file already has an
`assert_def_type` helper (used by `ac2_literal_types_distinct`), and it asserts
the type of a **def**. Applying it to this test's `fn f (x : Int64) : Int64`
proves nothing: that type is written in the annotation. The literal's type is
inside the body. A witness that actually discriminates needs the literal's type
to be observable — a differential between an annotated and an unannotated
binding of the same literal is one shape that works. **Choosing the witness is
the implementer's call; the AC below constrains what it must discriminate, not
how.**

### 2b. For the three ignored rows, an added assertion is not a discharge

**An assertion added to an ignored test never runs.** It is unproven, it can
itself be wrong, and it converts one unchecked claim into a *differently*
unchecked claim while looking like a repair. The registry's `readmission` text
for these three currently reads "add an assertion for…", which is the right
condition for **re-admission** and the wrong description of **this node's**
deliverable.

⇒ **What makes the false cover false is the claim-id doc comment, not the
missing assertion.** Sever that link and the row stops asserting cover it does
not have, immediately, with no dependency on unbuilt capability.

> **Holds for `ac2` and `ac5` only** — see the correction above. `sec31` and
> `sec24` have no doc-comment link to sever, so for them "sever" is a no-op and
> only the positive marker discharges anything.

Per row, the honest dispositions are exactly these three. Pick one per row and
record which and why:

1. **Sever and mark.** Keep the test ignored, remove or explicitly negate the
   claim-id doc comment so no reader or future tool reads it as cover, and
   state in one line which capability it waits on. Cheapest; discharges the
   defect for all three; does not pretend the claim is covered.
2. **Write the assertion and prove its failure mode.** Only if you also run it
   once and record the observed failure — it will fail, that is expected, and
   the recorded failure is what makes the assertion non-vacuous.
3. **Delete**, and record the claim as uncovered.

**"Leave it and register it as a policy exemption" is not among them.** That
entombs it. This is the disposition `CI-IGNORED-SWEEP` `D5` explicitly declined
to make, and it is why this node exists.

### 2c. Order

**`ac2` first.** It is the only row producing a false green today and the only
one not blocked on unbuilt capability. If the turn hits its hour, `ac2` alone
is a releasable increment.

## 3. Deliverables

- **D1.** `ac2_expected_type_overrides_default` checks the claim's `expect:`,
  including the clause that distinguishes an expected-type override from
  unification. Its doc comment no longer states an unchecked inference as a
  certainty.
- **D2.** Each of the three ignored rows carries one of §2b's three
  dispositions, applied, with the choice and reason recorded in the frame's
  own table (edit this file) or the issue node.
- **D3.** The exemption registry is left consistent with whatever D2 did — see
  §4, which is a hard gate, not a cleanup.

### D2 disposition record

| ignored row | disposition | precise waiting capability and reason |
|---|---|---|
| `ac5_explicit_conversion_is_partial_option` | sever and mark | L-classes must expose `Int.toInt64`; only then can the test assert acceptance with result type `Option Int64`. |
| `sec31_int_div_zero_emits_obligation` | sever and mark | Integer division op registration must land; only then can V2 emit and the test assert the non-zero-divisor obligation. |
| `sec24_char_excludes_surrogates` | sever and mark | Char literal syntax must land; only then can the test assert that a valid scalar accepts while a surrogate rejects. |

## 4. Contention and sequencing — read this before starting

### 4a. `CI-IGNORED-SWEEP` owns the registry, and it is in flight

At the time of framing, `wp/CI-IGNORED-SWEEP` is mid-recut and its registry
`.github/ignored-test-exemptions.toml` is **changing shape** — the corrective
child removes the hand-maintained `binary` field and resolves each `test_path`
against nextest's JSON listing. **This node must not start against that file
until the sweep merges.** Hence `depends_on: [CI-IGNORED-SWEEP]`.

### 4b. Three of this node's four rows are registered exemptions

The sweep's registry contains exactly the three ignored rows from §1a, each
with `class = "placeholder-no-assertions"`. **Every D2 disposition changes
them:**

| D2 disposition | required registry action |
|---|---|
| sever and mark (still ignored) | row stays; update `readmission` so it no longer describes a discharge that already happened |
| write the assertion (still ignored) | row stays; `readmission` updated to the real re-admission condition |
| delete the test | **row must be removed in the same commit** |

### 4c. A stale registry row is now a repo-wide merge block

`AC-4a` of `CI-IGNORED-SWEEP` (operative on `main` at `a22f1a87`) rules that a
**registry entry that resolves to no test is instrument failure** — loud,
non-zero, intentionally blocking. Because the publisher fails on any failing
check, that red blocks **every** merge in the repository, not just this
candidate.

⇒ **Deleting a test here without removing its registry row takes the whole
fleet's merge path down.** This is the single highest-consequence mistake
available in this node, it is silent locally, and it surfaces only in CI. Treat
D3 as a gate on D2, not as follow-up tidying.

### 4d. Everything else is disjoint

The only production-adjacent path is one integration-test file. No `crates/*/
src/`, no `spec/`, no kernel surface, no unbuilt capability. Runtime's in-flight
`RT-MATCH-RECURSOR-CONSUMERS` touches `ken-runtime` and `ken-cli`; there is no
overlap.

## 5. Acceptance criteria

**AC-1 — each of the four rows checks its claim, or no longer names one.**
Per row, in a table: the claim id, the disposition, and the specific `expect:`
clause now checked (or the statement that the claim link was severed and the
claim is recorded as uncovered). A row that names a claim and does not check it
fails this AC.

> MEASURED: each of the four tests' bodies and doc comments.
> CLAIMED: no test in this file asserts conformance cover it does not have.
> THE GAP: a row could sever its claim id and still read as cover to a human
> from its *name* alone. So severing requires the explicit one-line "waits on
> ⟨capability⟩" marker, not just deletion of the comment.

**AC-2 — `ac2`'s new witness discriminates.** Show it reds when the property
fails. The control must distinguish "the literal took the expected type" from
"the literal was solved by unification"; a control that merely reds when
elaboration fails does not discharge this, because the pre-existing `unwrap()`
already did that much. State the mutation and the observed failure message.

> ### STEWARD RULING 2026-08-09 — the mutation SITE is the owner's call, and
> ### the producer mutation is retired as uninformative
>
> Verify measured at `D1` that the real producer mutation
> (`elab_num_lit_checked` skips the checked arm for infer+unify) reds, **but
> that the retained legacy body fails first at its existing `unwrap()`
> (`KernelRejected TypeMismatch`), before the new literal-level witness is ever
> reached.** The finding is correct and it is the reason this clause is being
> written, not a reason to amend the property.
>
> **1. AC-2 never prescribed a site, so no amendment is needed.** It prescribes
> what the control must *distinguish* and asks you to state the mutation you
> used. **An observation-seam mutation is permitted.**
>
> **2. The producer mutation is retired for this AC.** In an A/B mutation proof
> the informative side is the one that **greens** — the arm where the legacy
> body still passes and only the new witness reds. Here neither arm isolates
> the new witness, so the mutation cannot support the claim regardless of how
> red it goes. Record it as measured-and-uninformative; do not report it as a
> discharge and do not weaken the legacy body to make it green.
>
> **3. The binding constraint on the substitute.** An observation-seam mutation
> must substitute what the witness **observes at a seam the production path
> actually feeds**. If the test hands the substituted `Term::IntLit` to its own
> assertion, it asserts on a needle it supplied and is vacuous — that is
> `DOC-GATE-NEEDLE`'s defect class, already merged in this repo. State which
> seam you substituted at and what still reaches it from production.
>
> **4. State both cells, not one.** The mutated arm reds **and** the unmutated
> pair greens. A red alone is consistent with a control that reds at
> everything.
>
> **5. The committed pair is already the discriminator AC-2 asks for.**
> `const defaulted = 1` observed as `Term::IntLit` against
> `const expected : Int64 = 1` observed as `Term::Const` at kernel type
> `Int64` is a non-degenerate pair on a shared input, so a flipped boundary
> inverts both. The pair proves the witness *discriminates*; the mutation
> proves the pin *bites*. They are separate obligations and `AC-2` wants both.

**AC-3 — no row is un-ignored, and no capability work is done here.** Positive
control: the count of `#[ignore]` in this file is unchanged unless a test was
deleted under D2.3, in which case state the new count and which row went.

**AC-4 — the registry is consistent, proved against the sweep's own
resolution.** After D2, every registry entry naming a test in this file
resolves to exactly one live test, and every ignored row in this file that the
sweep's population includes is either registered or deliberately not. State
this per row. **Do not hand-verify by reading the TOML** — the sweep's
resolution is the instrument; use it.

**AC-5 — the l1 suite is green in CI.** Locally, `scripts/ken-cargo` scoped to
`-p ken-interp --test l1_acceptance` only. Never `--workspace`
(`COORDINATION §12`); workspace-green means green in CI.

## 6. Out of scope, and do not reopen

- **Do not widen to other files.** Claim ids appear as doc comments across many
  acceptance suites, so this class is plausibly repo-wide — that is an
  observation, not this node's scope. If you want it swept, raise it to the
  Steward as a new node; do not grow this one. Sizing here assumes four rows in
  one file.
- **Do not build a claim-id-to-test binding gate.** §1c establishes none
  exists. Whether one should is a design question for the Architect and the
  conformance-validator, not a deliverable of a size-S test-body node.
- **Do not repair the underlying capabilities** — Char literal syntax, div op
  registration, L-class conversions. Each has or needs its own node.
- **Do not re-litigate `D5` of `CI-IGNORED-SWEEP`.** That `sec24` passes while
  ignored is vacuity, not over-annotation; the hard stop that produced this
  node was correct and its disposition stands.

## 7. Note for the mechanical-sweep trap

A regex sweep for assertion-free `#[test]` functions was attempted at
`d75d8c48` and **its output did not survive audit** — it matched Ken `fn`
declarations inside fixture string literals, and its brace matching walked into
nested `impl` blocks inside real tests. Both reported numbers were
contaminated. If any part of this node reaches for a mechanical population
count, it must walk Rust items and be validated against a known-answer set
first. The four rows in §1a were established by reading, and that is why they
are trustworthy.
