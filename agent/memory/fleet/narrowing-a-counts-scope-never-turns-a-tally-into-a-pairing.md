---
scope: fleet
audience: (see scope README)
source: 2026-08-12, `D2k-1b-i` / `D2k-1c-0` — one error class caught four times
  inside a single increment, each catch correctly narrowing the scope of a count
  and keeping the count
---

# Narrowing a count's scope never turns a tally into a pairing

An acceptance instrument had to prove that **each** transport of a compiler-only
field is consumed **exactly once**. It went through four revisions in one
increment. Every revision was a real improvement, every one was accepted by a
reviewer, and the first three were later found unsound in the same way.

| # | the instrument | how it failed |
|---|---|---|
| 1 | seven forbidden **verbs** | losing the field is not a use, so a drop satisfied it vacuously |
| 2 | a total naming **one disposition** (consume) | erasure and refusal are also lawful endings; the ruling widened it to the disposition space |
| 3 | a **compile-wide scalar** — `close()` accepted when `consuming_calls >= entries.len()` | a dropped field could be paid for by another field called twice, or by an unrelated pre-existing call |
| 4 | a **per-origin scalar** — `close()` checks `entry.consumptions != entry.rebinds` | at `rebinds = 2`, transport #1 consumed twice and transport #2 dropped gives `2 == 2` and closes green |

Steps 3 and 4 are the lesson. The rejection at 3 was *"a count over a population
is not a pairing within it"*, and the repair keyed the ledger by field origin —
which narrowed the population the count ranges over from the whole compile to a
single occurrence. It is a strictly better instrument. **It is the same
instrument.** One occurrence can be lowered twice, so a per-origin tally is
still an aggregate over the transports inside that origin, and the rejected
relation reappears one level down.

⇒ **Each iteration correctly narrowed the scope of the count and kept the
count. The instrument was never the scope. It was the counting.**

## The shape

**A count answers "how many", and a pairing asks "which one".** No amount of
narrowing converts the first into the second, because the fact that would
distinguish *which* transport was paid is **not present in the representation**.
An entry holding two `usize`s cannot separate `2 = 1 + 1` from `2 = 2 + 0`; that
is not a gap in the test rows, it is the strongest claim the representation
supports.

The repair is never a smaller counter. It is an **identity per obligation**:
mint a fresh instance when the obligation is created, record it outstanding,
discharge that exact instance once, refuse a second discharge of it, and refuse
every instance still outstanding at closeout. Affine bookkeeping, not
arithmetic.

## How to recognize it before the fourth time

- **Read the doc next to the check.** Here the entry's own comment said *"two
  transports of one occurrence, and each owes its own consumption"* — a pairing
  in prose, sitting three lines above a check that compared two integers. **When
  the comment says "each" and the code says `==`, they have already come apart.**
- **Ask whether equal counts can be reached two ways.** If `n = n` is satisfiable
  by more than one assignment of who paid, the check is an aggregate.
- **A short count is the near miss that hides the unsound one.** The control set
  had a `rebinds = 2, consumptions = 1` row asserting refusal. Short counts are
  easy to build and they look like coverage of the doubling case; the unsound
  case is `2, 2` paid by one party, and it is invisible at that granularity **by
  construction**.
- **A narrowed scope reads as a fix because the rejection named the scope.**
  When a reviewer rejects "compile-wide", the obvious repair is "per-origin",
  and it ships with the rejection's own words as its warrant. **Re-derive what
  the rejection was actually about**, not the noun it used.

## Direction and timing

The step-4 defect was **fail-open**, and it had **no live witness**: every row
sat at zero installs, so the counter was never non-zero in production. It would
have gone live in the *successor* increment, the one that makes the transports
happen. **A fail-open instrument with no current witness is not a low-severity
finding — it is a scheduled one**, and the schedule is the increment that makes
its precondition reachable. Fix it in that increment, ahead of the work that
arms it, not as a follow-up node behind it.

## How to apply

- When an acceptance property says **each**, **exactly once**, **its own**, or
  **respectively**, the check must name the individual. A comparison of totals
  does not.
- **Say out loud which fact distinguishes the members.** If you cannot point at
  it in the data structure, the property is not expressible there yet, and no
  test row will make it so.
- Route the representation question to the design owner, not the ring: what
  pairs a discharge to an obligation is a soundness call.

Related:
[[an-acceptance-property-listing-forbidden-verbs-permits-losing-the-thing]] —
instances 1 and 2 of this same sequence, and the total-over-the-disposition-space
repair that step 2 needed.
