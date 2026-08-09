---
id: CI-ASSERTIONLESS-L1
title: "Four registered conformance claims whose only cover does not check them — l1_acceptance.rs, three ignored and one live, green, and counted as cover"
status: ready
owner: verify
size: S
gate: none
depends_on: [CI-IGNORED-SWEEP]
blocks: []
github: null
origin: verify-implementer D5 hard stop evt_15argr23kn3rq on CI-IGNORED-SWEEP (2026-08-09), independently re-measured by the Steward at origin/main d75d8c48. Filed as its own node because the live assertion-free row is structurally invisible to the sweep, so folding it into CI-IGNORED-SWEEP would leave the worse half uncovered. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> ## FRAMED 2026-08-09 — `docs/program/wp/CI-ASSERTIONLESS-L1.md`
>
> Status is `ready`, and `depends_on` now names `CI-IGNORED-SWEEP`: three of
> the four rows are registered exemptions in that node's registry, whose
> **shape is changing** under the `AC-4a` re-cut. The frame's section 4 is the
> hard part — under `AC-4a` a registry entry resolving to no test is loud
> instrument failure, so **deleting a test here without removing its registry
> row blocks every merge in the repository.**
>
> ### Two corrections to the measurement below, re-grounded at `a22f1a87`
>
> **`ac2_expected_type_overrides_default` has 3 statements, not 7** — that
> figure was a line count. Everything else in the table below is unchanged
> from `d75d8c48`.
>
> **The defect is larger than "asserts nothing".** All four rows name a
> **registered conformance claim** in their doc comments, and
> `conformance/surface/numbers/seed-numbers.md` lists all four in its coverage
> map against a spec AC. So this is conformance integrity, not test hygiene.
> Nothing mechanical binds a claim id to a test — the link is doc-comment
> convention — which means nothing will ever catch this, and also that
> severing a claim link is cheap and honest. The frame reframes the node on
> that axis: **a test that names a claim must check what the claim's `expect:`
> says.**
>
> ## The finding came from a hard stop that was CORRECTLY taken
>
> `CI-IGNORED-SWEEP` `D5` mandates a stop if an ignored row **passes**.
> `sec24_char_excludes_surrogates` does. The implementer stopped, and then did
> the more valuable thing: it **refused to read the pass as a repair**, because
> the body is empty. Recorded here so the reasoning is not lost — the row is
> **not over-annotation**, and it must not be un-ignored.

## The measurement

`crates/ken-interp/tests/l1_acceptance.rs` at `origin/main` `d75d8c48`. Of **17**
`#[test]` functions, **4 contain no `assert*` and no `panic!` anywhere in the
body**:

| test | line | ignored? | body | status today |
|---|---|---|---|---|
| `sec24_char_excludes_surrogates` | 335 | yes | **comment only, zero statements** | passes **vacuously** |
| `ac5_explicit_conversion_is_partial_option` | 243 | yes | `let _result = env.elaborate_decl_v1(..).unwrap();` | fails at the `unwrap`, exit 101 |
| `sec31_int_div_zero_emits_obligation` | 285 | yes | `let _result = env.elaborate_decl_v1(..).unwrap();` | fails at the `unwrap`, exit 101 |
| `ac2_expected_type_overrides_default` | 110 | **no** | 7 statements, no assertion | **passes, green, counted as cover** |

**Three of the four assertion-free tests are exactly the three ignored rows.**
That correlation is the finding: `#[ignore]` has been acting as the storage
mechanism for unfinished tests.

## Why this is a defect and not a style complaint

**A test whose name claims a property and whose body never checks it is false
cover.** Compare two neighbours in the same file:

- `sweep_int8_overflow_emits_obligation` asserts
  `result.obligations.len() == 1` and matches `ObligationKind::PartialPrim`.
- `sec31_int_div_zero_emits_obligation` — same naming shape, same domain —
  binds `let _result` and checks **nothing**.

⇒ **The day integer division is registered, `sec31` goes green and reports that
a divide-by-zero obligation is emitted. It will never have looked.** The
`#[ignore]` is currently the only thing preventing a false green, and it is
being held in place by an unrelated fact (the capability is unbuilt). Two
independent defects are cancelling, and the cancellation expires on repair.

**`ac2` is the same defect with nothing cancelling it.** It is live, green, and
in the suite's pass count right now.

## Why a separate node rather than folding into `CI-IGNORED-SWEEP`

**The sweep is structurally blind to the worst row.** `CI-IGNORED-SWEEP` selects
on the `#[ignore]` attribute; `ac2` carries no `#[ignore]` and can never appear
in its population, whatever the sweep does. Folding this in would fix the three
rows that are *already suppressed* and leave the one that is *actively
miscounted*.

It also keeps the sweep's diff minimal, which is the same operator ruling
(2026-08-07) that filed `CI-IGNORED-SWEEP` separately rather than folding it
into `RT-SRCBODY-BIND-ORDER`.

## Deliverables owed by the frame

- **Decide each row's disposition explicitly.** The honest options per row are:
  write the missing assertion; convert it to a documented placeholder that
  cannot be mistaken for cover; or delete it. **"Leave it and register it as a
  policy exemption" is not among them** — that entombs it.
- **`ac2` first.** It is the only one producing a false green today, and it is
  not blocked on any unbuilt capability.
- **Do not un-ignore the three ignored rows.** Their capabilities really are
  unbuilt; un-ignoring converts a silent non-test into a red. The assertion is
  what is owed, not the attribute.
- **Ask whether the class is wider than one file.** Partially answered below;
  the answer is *yes, but do not trust my count.*

## Scoping attempt, 2026-08-09 — the sweep is HARDER than it looks

**Steward attempted the cross-crate scoping sweep this node asks for, and it did
not survive its own audit. Recorded so nobody repeats it.**

A regex over `crates/` for `#[test]` functions whose bodies contain no
`assert`/`panic!` reported **115 of 2731**; restricted to genuinely empty bodies
it reported **54**. **Both numbers are contaminated and must not be used.**

Two independent defects in that detector, each found by reading its output
rather than its count:

1. **It matched Ken source inside fixture string literals.** Ken test fixtures
   are embedded Ken programs in Rust raw strings, and they contain `fn`
   declarations. `surface_named_proof_claims.rs` alone produced seven hits all
   named `id` — that is `fn id (x : A) = x` inside a fixture, not a Rust test.
   Names like `local_resp`, `vectorHead`, `J`, `keep`, `main`, `spin` are the
   same artifact.
2. **Its brace-matching walked the wrong span on nested items.**
   `ken-host/src/abi_v1.rs:2245 resolve_effective_user_home` was reported as an
   empty body. It is a **trait method inside a nested `impl` inside a real
   test**, and it contains `assert_eq!`.

⇒ **"Assertion-free `#[test]`" is not cheaply detectable by grep in this
repository**, which is the opposite of what this node's first draft assumed.
Any frame that specifies a mechanical sweep must say how it excludes fixture
strings and nested items — a syntactic pass over Rust items, not a regex — and
must be validated against a known-answer set before its count is believed.

### What the attempt DID establish, by reading

A distinct and milder sub-class exists, and it is **self-declaring**:

| row | shape |
|---|---|
| `ken-elaborator/tests/v2_acceptance.rs:516,530,541,555` | four tests named `*_placeholder`, each body a single comment `[placeholder — reifies in V3]`, each doc comment saying the same |
| `ken-elaborator/tests/v1_acceptance.rs:372` | `disproved_distinct_from_unknown` — comment-only body whose comments say the property "is covered by `unknown_hole_distinct_from_proved`" |

All five are **live, green, and counted as cover**.

**These are a different defect from `sec24`, and probably a much smaller one.**
They announce themselves in the name, the doc comment, and the body; a reader is
not misled about what they check. `sec24_char_excludes_surrogates` and
`sec31_int_div_zero_emits_obligation` **claim a property in the name that the
body never tests**, which is the shape that becomes a false green.

⇒ **Frame this node on the name/body mismatch, not on "asserts nothing".** The
second is a style question with legitimate instances (a "must not panic" smoke
test correctly has no assertion). The first is the defect. Sizing should assume
the honest placeholders are a cheap second-order cleanup, not the main body of
work, and that no reliable population count exists yet.

## Owner

**`verify`, provisionally, and this is a Steward call rather than a derivation.**
The rows originate in Team Language's `L1-numbers`, but Language is **parked
under the operator's ABI wind-down** (2026-07-28) and takes no new work, so
routing there parks the defect too. Test integrity is Verify's domain and Verify
found it. `D4` of `CI-IGNORED-SWEEP` routes a finding whose `#[ignore]` names no
live owner node to the Steward, which is how this arrived; repository search
finds no live owner id for the Char-literal surface-syntax WP.

**Nothing here touches `crates/ken-elaborator` behaviour or any unbuilt
capability.** It is test-body work.
