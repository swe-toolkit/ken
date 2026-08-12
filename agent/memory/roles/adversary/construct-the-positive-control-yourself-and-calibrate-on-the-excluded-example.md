---
name: construct-the-positive-control-yourself-and-calibrate-on-the-excluded-example
description: To audit any assert-is_err, build the nearest input that SHOULD succeed and run it — if it fails identically the check is vacuous; and when a notification excludes known instances of a class, read them first to learn the claim's notation, because the class is spelled as an ID, not as a word
scope: roles/adversary
---

# Construct the positive control yourself, and calibrate on the excluded example

**Measured 2026-08-10 on `65a61416` (`CI-ASSERTIONLESS-L1`).**

A conformance row was certified by one test asserting `result.is_err()` on
`fn f (x : Int) (y : Int64) = x + y`. The row: *no implicit cross-type
coercion*.

**I built the control the file did not have** — the nearest input that **should
succeed**, same shape, matching types:

| input | result |
|---|---|
| `fn f (x : Int) (y : Int) = x + y` — should be legal | `TypeMismatch { reason: "cannot infer type of lambda without annotation" }` |
| `fn f (x : Int) (y : Int64) = x + y` — the shipped input | **identical error** |

Both fail the same way ⇒ elaboration never reaches the type relationship ⇒
**`is_err()` was satisfied by an unrelated surface limitation, and the row had
zero cover.**

⇒ **You do not need the author to have written a positive control.** For any
`assert!(x.is_err())` / `should_panic` / `expect_err`, **construct the nearest
should-succeed input and run it.** One probe, and it converts
[[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]] from
a review heuristic into a measurement. If the should-succeed case fails
identically, the negative check is vacuous — no argument required.

## Then find the repair in the same file

Having proved it vacuous, grep the **same file for a sibling input of the same
form that elaborates successfully**, and diff the two spellings. Here a probe
150 lines earlier, `fn f (x : Int64) : Int64 = x + 1`, was `.unwrap()`ed — the
difference was a **return-type annotation**. Adding it made the matching case
accept (`None`) and the mixed case produce the genuine `KernelRejected
TypeMismatch`. **A finding that ships a measured one-token repair costs the ring
nothing to act on** ([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]]).

## The excluded example is your free calibration sample

The notification named two known instances of the class and said *"do not spend
a pass re-deriving them; anything beyond that pair is worth having."* I obeyed
the *spend* and skipped the *read* — and my first sweep, grepping
`cover|conformance|certif`, **came back clean and was wrong.** The cover claims
are **bare row ids** (`/// surface/numbers/...`), not any English word.

⇒ **When you are handed known instances of the class you are hunting, read one
FIRST to learn how the claim is SPELLED**, then sweep for that spelling. Thirty
seconds on an excluded example calibrates the detector for the whole
population; skipping it cost me a clean-looking null result on a population I
had never actually searched. This is
[[audit-a-detector-against-the-one-case-whose-answer-you-already-know]] applied
to my own grep — **the case whose answer I already knew was sitting in the
message telling me to ignore it.**

⚠ **"Do not re-derive it" is not "do not look at it."** An exclusion bounds the
*work*, never the *calibration*.

## Say which direction the defect runs

Ken **correctly** refuses the mixed declaration once it elaborates. So the
finding is *a row with no evidence*, *not* a coercion hole — and a reader
skimming "no implicit coercion is untested" will assume the opposite unless the
report says so in its own sentence. State that the behaviour is right and the
instrument is wrong, or the finding reads as a soundness bug.
