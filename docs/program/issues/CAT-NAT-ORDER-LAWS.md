---
id: CAT-NAT-ORDER-LAWS
title: "Proof-backfill for Data/Numeric/Nat/Order.ken.md: land the inductive sub/leq fragment the package explicitly defers (self-subtraction, saturation, strict decrease), then the min/max/compare theory. Dependency-bottom node of the proof-completeness survey's 17 follow-ons: its sub/leq lemmas are what the downstream *-LAWS packages reduce their own bounds reasoning to."
status: ready
owner: foundation
size: M
gate: none
tier: T2
depends_on: [CAT-PROOF-COMPLETENESS-SURVEY]
blocks: []
github: null
origin: "Operator ruling 2026-09-13 (Pat): 'A catalog package is not finished until its proofs are complete... computational tests are not part of the package, so a reader cannot trust them as intrinsics.' Recorded as docs/PRINCIPLES.md #16. That ruling directed the catalog proof-completeness survey and said its gaps become proof-completion nodes. The survey LANDED 2026-09-19 as ec23d4ab6 (report docs/CATALOG-PROOF-COMPLETENESS-SURVEY.md, +162) naming 18 genuine proof-backfill packages, 17 of them needing nodes framed. This is the first of those 17, selected by the Steward as the dependency bottom (steward.md §3: sequencing is the Steward's call) -- see the frame §1a for the measurement that established the ordering."
---

# Proof-backfill: `Data/Numeric/Nat/Order.ken.md`

The survey's row for this package, verbatim:

> Provider order laws and three local zero laws are intrinsic. The source
> explicitly names `sub n n = Zero` as unproved; broader min/max/sub/compare
> algebra is absent.
>
> `CAT-NAT-ORDER-LAWS`: prove the complete min/max/sub/compare theory,
> including self-subtraction, over the current structural definitions and
> canonical `leq_nat` with no new trust.

The package does not merely lack these proofs — it **names the gap in its own
prose** and carries the failing attempt in a ` ```ken reject ` fence
(`catalog/packages/Data/Numeric/Nat/Order.ken.md:146`). That is the honest
posture PRINCIPLES #16 now supersedes: the package is not finished until the
proof lands.

## Why this node is first of the seventeen

Not a priority judgment about `Nat` — a measured dependency. See the frame's
`§1a`. In short: the downstream `*-LAWS` packages discharge their bounds
obligations by reducing them to `sub`/`leq` facts that **do not exist yet**, so
framing any of them first hands an implementer a node whose first move is to
invent `Nat` lemmas belonging to a different package.

## Scope

Two deliverables, `D1` releasable on its own (`steward.md` §4a: accepted work
merges as soon as it is done, even a partial WP).

- **`D1` — the inductive `sub`/`leq` fragment.** The four lemmas the deferral
  names or the downstream consumers need. This is the increment being released.
- **`D2` — the min/max/compare theory.** D1 landed at
  `47b811be4f618b3671331857294a0d0b84d8276d`, so D2 is unblocked and framed
  below. **D1 alone does not close this node**; D2 does.

## `D2` frame — the min/max/compare theory

Releasable now. Nothing blocks it.

### Settled inputs

Measured at `47b811be4`. Do not re-derive.

**1. There is no predecessor gap, and this is the opposite of the sibling
case.** `Core/Classes/LawfulClasses.ken.md` already carries the full order
theory for `leq_nat` — `refl` (`:499`), `trans` (`:505`), `antisym` (`:527`),
and `total` (`:584`). Every fact `compare` can need about `leq_nat` exists.
`CAT-PARSING-CURSOR-LAWS` had to be cut behind a predecessor because its
`nth` obligation had none; **D2 has all four and needs nothing new.**

**2. `compare` is defined entirely from `leq_nat`.** It matches `leq_nat a b`
and then `leq_nat b a` (`Order.ken.md:124`), so its laws are consequences of
those four facts rather than of anything about `Nat` structure. In particular
the `Eq` arm is exactly where `antisym` is used.

**3. The existing `min`/`max` facts are examples, not package proofs.**
`proof zero_left for min` (`:121`) and `proof zero_left for max` (`:123`) sit
inside a ` ```ken example ` fence in `§4`. They are illustrative and are not on
the package's exported proof surface. Promoting them is part of D2.

### Deliverable

Exported proofs in `Data/Numeric/Nat/Order.ken.md`, for abstract `m`/`n`/`a`/`b`:

- **`min` and `max` are the bounds their names claim** — `min m n ≤ m`,
  `min m n ≤ n`, `m ≤ max m n`, `n ≤ max m n`.
- **`compare` agrees with `leq_nat` on all three arms** — `Lt`, `Eq`, and `Gt`
  each imply the corresponding order fact, with `Eq` yielding
  `Equal Nat a b` through `antisym`.
- The two `§4` example facts, promoted to `pub proof`.

Carry Boolean hypotheses and conclusions as `IsTrue` over `leq_nat`, matching
what D1 landed and what `§4`'s own prose already commits the package to.

Names and binder order are the implementer's.

### Stop condition

**If any law needs a `Nat` or `Bool` fact that `Nat/Order.ken.md`,
`Nat/Arithmetic.ken.md`, or `LawfulClasses.ken.md` do not already carry, stop
and report.** Do not widen into those packages and do not land a general fact
here. Settled input 1 says this should not happen; if it does, the frame was
wrong and that is worth more than a workaround.

### Acceptance criteria

**`AC-D2-1` — the laws are inhabited, exported, and none is degenerate.**
*Control, both halves required:* the package elaborates with the new terms
present; **and** replacing any one law's term with `Refl` makes the package go
RED, restored byte-exact afterwards.

**`AC-D2-2` — the shipped functions are unchanged.** This deliverable proves
what the package already ships; it does not adjust a definition until it
becomes provable. *Control:* `min` (`:49`), `max` (`:59`), `sub` (`:69`), and
`compare` (`:124`) are byte-identical to their text at `47b811be4`, extracted
and compared programmatically.

**`AC-D2-3` — `§4`'s prose and the package's proof surface agree.** *Control:*
no sentence in `§4` describes as an *example*, or as unproved, a fact the
package now exports as a `pub proof`. **This package has produced that exact
drift twice** — a paragraph saying a law was deliberately unproved six lines
above a checked proof of it, and the `example` fence in settled input 3. It is
the node's recurring defect, not a hypothetical.

**`AC-D2-4` — no new trust, and the diff goes exactly one place.** *Control:*
the added lines contain no `Axiom`, postulate, primitive, `Omega` carrier, or
kernel/TCB surface; **and** the diff touches exactly
`catalog/packages/Data/Numeric/Nat/Order.ken.md` and, under `crates/`, at most
`crates/ken-cli/tests/rosetta.rs`. Any other path — test or not — is a hard
stop and a report, not a scope extension.

## Not this node

- No change to any definition in the package. The proofs are over the current
  structural `min`/`max`/`sub`/`compare` and the canonical `leq_nat`.
- No new trust: no `Axiom`, postulate, primitive, `Omega` path carrier, TCB
  change, or kernel change. The survey states this constraint for all 17 rows
  and it is an acceptance criterion here.
- Not machine-checked complexity. The survey excludes it explicitly.
- Not the other sixteen survey follow-ons. They are named in
  `docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md` and stay unframed until
  released one at a time.
