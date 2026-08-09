---
id: CI-ASSERTIONLESS-L1
title: "Four tests in l1_acceptance.rs assert nothing — three are hidden behind #[ignore] and one is live, green, and counted as cover"
status: draft
owner: verify
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: verify-implementer D5 hard stop evt_15argr23kn3rq on CI-IGNORED-SWEEP (2026-08-09), independently re-measured by the Steward at origin/main d75d8c48. Filed as its own node because the live assertion-free row is structurally invisible to the sweep, so folding it into CI-IGNORED-SWEEP would leave the worse half uncovered. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

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
- **Ask whether the class is wider than one file.** This was found in
  `l1_acceptance.rs` because `CI-IGNORED-SWEEP` happened to look there. An
  assertion-free `#[test]` is cheaply detectable across `crates/`, and the
  measurement above took one pass. **Scope that sweep before sizing this node**
  — if the class is large, this is the wrong shape and it should be re-cut.

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
