---
name: an-ordered-validators-first-stage-subsumes-most-of-its-later-laws
description: "A validator that recomputes its subject up front, then checks N laws against the recomputation, has far fewer live detectors than laws — the early stage rejects the inputs the later ones were written for. Probe each arm for a witness; do not count them."
scope: fleet
---

# An ordered validator's first stage subsumes most of its later laws

`RT-FNSPLIT-B2O`'s `validate_function_units` (`semantic_ir.rs:987-1125`)
opens by **recomputing** the ownership partition from the graph, then checks
twelve named laws against that recomputation. Twelve laws; **five live
detectors.** Measured, arm by arm, by building the input each law was written
to reject:

| law | the input that should trip it | the error actually returned |
|---|---|---|
| *"scheduling entry has an incoming static body edge"* | a `StaticBody` edge aimed at an entry | *"scheduling entry is also a static body target"* — from the recomputation, line 1 |
| *"descriptor population is not exact for the partition"* | pop a descriptor | *"planned node lacks exactly one semantic definition"* |
| *"names an unknown function unit"* | an out-of-range unit id | *"owner is not the node's derived function unit"* |
| *"static body edge targets a shared exit"* | aim one at the terminal | *"planned node has no function unit owner"* |

**The first row is not merely shadowed — it is unreachable.** The
recomputation on the function's own first line rejects the *identical*
condition with a different message. It can never fire, and it was also the
only quadratic check in the file (`Vec::contains` inside a loop over every
edge).

## Why the count is the trap

Each law reads as an independent guarantee. Each has its own error string, its
own comment, often its own explaining a subtlety a reader would otherwise
miss. **Nothing in the source distinguishes a law with a witness from one
without** — that difference lives in the *ordering*, which is invisible at any
single arm. So the file honestly advertises twelve detectors and a downstream
consumer plans against twelve.

**The general shape: strengthening an early check silently weakens the
evidence for every later one, and no artifact records the transfer.** The
recomputation was added precisely to make a corrupted record loud — a good
change — and it converted seven independently-motivated laws into dead prose
in the same commit. This is why "we added a check" is never by itself an
increase in coverage.

## How to apply

- **Authoring:** for each law you write, name the input that reaches it *and
  no earlier arm*. If you cannot, the law is documentation — say so in the
  comment, or delete it. A law nobody can trip still costs review attention
  and, as above, can cost runtime.
- **Auditing:** enumerate the arms (they are grep-able — one per error
  message), then build a witness per arm and record **which error came back**.
  Do not accept `is_err`; see below.
- **`assert_eq!` on the exact error, never `expect_err`.** Every row of the
  table above is legible *only* because the surrounding suite asserted exact
  errors. Under `expect_err` all seven probes are green and teach nothing —
  and the same choice had already caught the authors' own control selecting
  the wrong victim node.
- **Suspect the earliest stage first.** The arms most likely dead are the ones
  whose condition restates, in different words, something the constructor or
  the recomputation already guarantees.

## The same shape in a ledger checker: the outer failure hides the inner one

**Measured 2026-08-14 on `57bf1721`, from a Steward hand-off that said outright
*"I have not investigated how long it has been that way or whether other rows
share the shape."*** One row of a 135-row content-attestation ledger was known
stale. **The sweep found fifteen**, three of them `spec/` files, the worst 25
commits past its attested state. And the checker validates two things in order:
(1) the ledger's path set equals the manifest-cited source set, (2) each row's
blob still matches the tree. **Twelve cited sources have no row at all, so gate
1 exits 1 and gate 2 never runs.**

⇒ ***When a checker validates on more than one axis, find out which axis fails
first, because the later ones are unreachable while it does.*** Someone running
the check sees the twelve and would reasonably conclude that is the whole
problem. The fifteen are not merely undetected; they are undetectable through
the intended instrument until an unrelated failure is cleared. **The tell is a
checker with early `exit 1`s and a list-building loop after them.** Read the
control flow before trusting a green or a red: a red says the earliest gate
fired and nothing about the rest.

The same review turned up four further lessons worth keeping:

- **Two files that must agree, written by different commits.** The ledger was
  last written on one day; the manifest that defines its required contents on a
  later one, and the later commit added the citations now missing. Both were
  correct when written, no single commit is wrong, and the invariant is
  violated. Look for pairs with an exact-agreement invariant and separate
  authorship: a generated file and its input, a ledger and a manifest, a
  lockfile and a manifest, an index and a corpus. **Report the decay rate**
  (here, twelve days after a correct-at-write ledger, 11% of rows drifted and
  twelve citations unattested); a rate makes a disposition arguable in a way a
  snapshot does not.
- **"Last matched at commit C" is about the PATH, not the ledger.** Several
  drifted rows' last-matching commit predated the ledger's own last write by
  weeks, which suggested the ledger had been written already stale. It is a good
  story and wrong: **measured at the ledger's own commit, all fifteen matched.**
  A path may simply not have been touched between its last edit and the
  attestation. One `git rev-parse <ledger-commit>:<path>` per row settles it.
  Same family as
  [[a-census-is-a-number-a-scope-a-predicate-and-a-tree]],
  inverted: there the count was right and the scope was doubted; here the dates
  were right and the wrong subject was inferred from them.
- **The correct template is often in the same file as the defect.** A recorded
  residual: an observation feature enabled unconditionally on a dev-dependency,
  so feature unification puts it into every crate's test build, with no
  artifact-identity control unlike its sibling. The sibling's correct shape (an
  optional feature that forwards) was seventeen lines above it in the same
  `Cargo.toml`. When a residual reads as a design question, check whether the
  tree already holds the answer next to the defect; it turns *"decide what to
  do"* into *"match the line above."*
- **Say compile-time or run-time when you say "outside the guard."** Work said
  to sit *"outside the observation guard"* was inside a
  `#[cfg(any(test, feature = …))]` **compile-time** gate and outside the
  **runtime** `Cell<bool>`, which is read only inside the recording function
  after the value is built. Those give different blast radii (absent from
  production entirely, versus paid on every merge in every test build) and only
  the second was true. "Outside the guard" with two guards present is ambiguous
  in the direction that sounds worse. Check the repair is writable while there:
  hoisting the runtime read to the call site is one `if`, established by the
  three existing bare `.get()`/`.set()` uses of the same thread-local.

Siblings: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]]
— there the check fires for the wrong reason; here it cannot fire at all.
[[close-a-class-partition-the-declared-population]] is the method that makes
this tractable: the error arms *are* the declared population, so the closure is
free. And [[a-requirement-in-an-advisory-section-is-never-discharged]] is the
mirror — there a real requirement sits where no gate reads it; here a real gate
sits where no input reaches it.
