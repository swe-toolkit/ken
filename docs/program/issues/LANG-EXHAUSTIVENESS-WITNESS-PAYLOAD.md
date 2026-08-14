---
id: LANG-EXHAUSTIVENESS-WITNESS-PAYLOAD
title: "34 §4.1 requires naming the unmatched PATTERN WITNESS, and ExhaustivenessError's payload is a single String documented as a constructor NAME -- so no change at any emission site can discharge the obligation, and it reads as satisfied today only because every landed omission test uses a zero-arity constructor where name and most-general pattern coincide"
status: merged
owner: language
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "LANG-GADT-SEQUENCE-TRACKER-GAP's D3 question 2, answered by the Language ring and then sized past the node's own claim by the Architect at evt_d685wfdctrat, measured at origin/main 3ea9bef4. That node called it 'future work for whoever owns error.rs next'; the Architect established it is representation-level and no emission-site change can reach it. Steward-filed per COORDINATION §2 rather than left as a closing sentence in a merged node."
---

## What this is

**An expressibility gap, not a diagnostic-quality complaint.** The obligation is
not expensive to satisfy and it is not false — it is **unsayable in the shape as
landed**, which is why no amount of green touches it.

`spec/30-surface/34-data-match.md` `§4.1` requires that a non-exhaustive match
on an **indexed** family name the **unmatched pattern witness** — an applied
pattern such as `VCons _ _ _`. The landed payload is
`ElabError::ExhaustivenessError { missing: String, span }`, whose own doc says
`missing` **names the first uncovered constructor**, and every production site
populates it from a bare name — one of them through a helper whose parameter is
literally `missing: &str`.

⇒ **A constructor name is strictly less than an applied pattern.** The payload
has nowhere to put the arguments, so the obligation has no in-shape home.

## WHY THIS READS AS SATISFIED, AND WHY THAT IS THE LOAD-BEARING PART

**Every landed omission test uses a zero-arity constructor** — `EmptyVector`,
`Blue` — where **the constructor name and the most-general pattern coincide**.
`Blue` *is* the witness for `Blue`. The tests are true, they pass honestly, and
they cannot distinguish the two properties.

**This is the whole reason the gap survived an audit that was looking for it.**
The measurement was correct and simply did not entail the property. Carry that
into the ACs: **a discriminating case needs a constructor with arity ≥ 1.**

## The census is the Architect's, at `3ea9bef4`, and it is NOT yours to inherit

Four production emission sites in `crates/ken-elaborator/src/elab.rs`, plus the
type and its doc in `error.rs`. **Cited by symbol because coordinates move:**
`ExhaustivenessError`'s declaration and doc comment, and the helper taking
`missing: &str`. **`D0` re-derives the set at your base** — the count is the
input to every sizing decision below, and this node's parent had its own census
wrong by a factor of two and a half.

## Deliverables

**`D0` — re-derive the emission-site set and the payload shape at your base.**
Report the count, each site, and whether any site already has the applied
pattern in hand at the point of construction. **That last question decides the
size of `D2`** — if the arguments are not in scope where the error is built,
this is a plumbing change and not a payload change, and that is a report.

**`D1` — the payload carries the witness.** Whatever shape you choose must be
able to express an applied pattern with its argument placeholders. **Do not
widen `missing: String` by convention** — encoding `"VCons _ _ _"` into the same
`String` satisfies the letter and reproduces the defect, because the next
consumer still cannot tell a name from a pattern and nothing checks which one it
holds. If you conclude the string encoding is right, **that is a stop and a
report to me**, not a decision to take inside this node.

**`D2` — every consumer that formats it, migrated.** The payload change is the
easy half. Enumerate the formatters, the tests that match on the variant, and
any conformance expectation keyed on the current text.

**`D3` — a discriminating test on an indexed family with arity ≥ 1.** The
existing zero-arity cases stay; they are correct. Add the case they cannot
distinguish.

## Acceptance criteria

**`AC-1` — a test fails against the OLD payload and passes against the new
one.** Name it. **This is the criterion the whole node turns on**: a green suite
after a payload change proves nothing here, because the landed suite is already
green against a payload that cannot express the property.

**`AC-2` — the discriminating case uses a constructor of arity ≥ 1**, and the
node says why arity 0 cannot discriminate. A new zero-arity test does not
discharge `D3`.

**`AC-3` — the spec obligation is quoted and the payload is checked against
it.** `34 §4.1`'s own words for the witness requirement, beside the shape you
landed. If the shape satisfies the letter but not the requirement, say so rather
than declaring victory on the quote.

**`AC-4` — no consumer is left formatting the old field.** A missed formatter is
a silently degraded diagnostic, which is the class this node exists to close.
Report the enumeration you ran, not a count.

**`AC-5` — `34 §4.1` is not amended.** If the obligation turns out to be
unimplementable as written, **stop and report** — narrowing a spec obligation to
fit the implementation is a Spec-enclave question and an Architect one, never a
deliverable here.

**`AC-6` — no-regression, in CI.** `COORDINATION §12` — the venue is CI, never a
local `--workspace` run. Build targeted, `-p ken-elaborator`.

## Sizing

**`M`.** The payload is small; `D2`'s consumer set is the unknown and `D0`
measures it before anything is committed. **The one-hour target applies to
`D0`+`D1`.** If `D2` turns out to reach beyond `ken-elaborator`, hand that back
as the finding rather than absorbing it.

## Not this node

- **Not a general diagnostic-quality pass** on `ElabError`. One variant, one
  obligation.
- **Not the redundancy/`already covered` half.** `§4.1`'s witness requirement is
  the scope; the redundant-arm diagnostic is untouched.
- **Not an amendment to `34 §4.1`.** See `AC-5`.
- **Not a re-audit of the four `SURF-gadt-*` slices**, all merged and accurately
  tracked by [[LANG-GADT-SEQUENCE-TRACKER-GAP]].
