---
name: the-unlawed-link-is-the-join-between-two-maps
description: A relational ledger's laws get written over each map's own KEYS, so the join — one map's value being another map's key — is the link with no law; and when an artifact states a redundancy standard, the place it did not apply that standard is the finding
---

# The unlawed link is the join between two maps

**Measured 2026-08-12 on `e78f0d0d` (`D2k-1c`).**

A conservation ledger was rebuilt from two counters into four maps and a chain
of three links, **construct -> transition -> consume**. Its closeout asserts
four relations:

| law | quantified over |
|---|---|
| `dom(recognized) ⊆ dom(transitioned)` | `recognized`'s keys |
| `dom(transitioned) ⊆ dom(recognized)` | `transitioned`'s keys |
| `dom(minted) ⊆ dom(consumed)` | `minted`'s keys |
| `dom(consumed) ⊆ dom(minted)` | `consumed`'s keys |

**Every law is keyed on the map's own keys. The middle link — that a
`transitioned` VALUE is a key of `minted` — has no law at all**, and it is
supplied in prose as *"`minted` in bijection with `transitioned` because one
transition mints exactly one transport."* That is a claim about a function
body, not something the closeout can fail on.

⇒ **In any multi-map relational ledger, enumerate the laws by what each one
QUANTIFIES OVER.** Four laws over four maps reads as complete coverage; it is
coverage of the *nodes*. **The edges between maps are values-into-keys, and a
loop over a map naturally reaches for its keys** — so the join is the thing the
shape of the code steers you away from checking.

⚠ **The tell is a loop that BINDS the value and uses it only in the message.**
`for (recognition, transport) in &self.transitioned { if
!self.recognized.contains_key(recognition) { … {transport:?} … } }` — the
transport is in scope, printed in the error, and never validated. A
`contains_key` away. Inverse of the destructure-discards-the-join-key tell in
[[a-single-site-claim-is-checkable-by-counting-the-operation-it-names]]: there
the key was thrown away, here it is bound and merely unused.

⇒ **Check the failure state each missing edge admits, and its direction.** Here
`transitioned[r] = T` with `T ∉ minted` passes all four laws while leaving `r`
with no consumption required — fail-open, and exactly the state the ledger
exists to forbid. The opposite orientation was safe. **One direction of one
edge; say which.**

## THE STRONGER MOVE — hunt where a STATED standard was not applied

The closeout's own doc says:

> *Both directions are checked below anyway, because **a law worth stating is
> worth being able to fail** — and the `⊆` re-check is what catches a future
> second writer that skips the call-side guard.*

**That standard was adopted for a strictly weaker case than the one it skipped.**
The re-checked relation was *already* enforced at the call site; the unlawed
join is enforced nowhere. So the finding needs no argument for the standard —
**the artifact supplies it, and applied it two loops away.**

⇒ **When an artifact states a redundancy or defence-in-depth principle, grep
for where it was NOT applied.** This is strictly cheaper than a finding against
an unstated standard, because the usual answer — *"that is unreachable today"* —
is already refuted by the author's own reasoning for the case they did cover.
The uneven application is the whole finding.

⚠ **Do not upgrade it to a live defect.** Today the bijection holds by
construction: one writer, two infallible inserts, no branch between. Report it
as *the one link with no law*, not as a reachable drop
([[preventive-findings-are-unfalsifiable-so-keep-them-cheap]] — and here the
repair is one line in a loop that already binds the value, which is what earns
it the weight).

## State the hypotheses you REFUTED, especially your first guesses

Two of my three opening attacks died on reading: a `BTreeMap::insert`
overwriting a second transition (refused explicitly, before any mutation), and
the recognition/transport ordering being a caller obligation a refactor could
reorder (it is one function, with both inserts infallible and nothing between).

⇒ **Say so.** A report that lists only the surviving finding hides that the
obvious attacks were run, and the recipient cannot tell a thorough pass from a
lucky one. It also cost me nothing: the refutations are what located the loop
that *was* unlawed.

⚠ **And the ordering axis I was handed was stronger than its description** —
worth reporting as such. Confirming that a flagged risk does not exist is a
deliverable when it carries the reason.
