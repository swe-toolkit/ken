---
id: CAT-NAT-REUSE-CONSUMERS
title: "Catalog-reuse rework, first scoped batch: the nine unblocked low-risk Nat arithmetic/order consumer duplicates from the census (groups 2 and 3), across six packages — each package imports add/leq_nat/sub/min from its canonical owner and drops the local reimplementation, one independently-releasable increment per package"
status: active
owner: foundation
size: M
gate: none
depends_on: [CAT-REUSE-CENSUS, CAT-GCD-REFACTOR, CAT-ORD-NAT-CANONICAL-OWNER, CAT-ORDER-PUB-EXPORT]
blocks: []
github: null
origin: "Steward, 2026-08-26, campaign step 3 (scoped rework) of the catalog-reuse modernization charter docs/program/wp/catalog-reuse-modernization.md. Framed FROM the merged census (docs/program/cat-reuse-census.md, landed 6f00843de) rather than blind, per the charter's census-first shape. Scope is the intersection of the census's low-risk work groups 2 and 3 with the providers that are ACTUALLY public and loadable at origin/main 5e49b944571b96f5a05a5ec2abd32c0ba31796fe — re-measured by the Steward, because the census's evidence base ed5b4063f predates both CAT-ORD-NAT-CANONICAL-OWNER and CAT-ORDER-PUB-EXPORT and therefore records the Nat-order providers as unavailable when they are now live."
---

## Objective

Apply the recipe the `CAT-GCD-REFACTOR` pilot proved — import the canonical
tool, delete the local duplicate — to the **nine** remaining low-risk Nat
arithmetic and Nat-order duplicates the census recorded, across six packages.
This is the campaign's first scoped rework batch. It removes duplication only;
it changes no computational meaning, moves no proof owner, and adds no
capability.

## Why this batch, and why now

The census (`docs/program/cat-reuse-census.md` §4.4) proposed seven low-risk
work groups. Most are gated: §4.2's missing-export set shows their providers are
not `pub` yet, and §4.3 shows four provider modules do not even elaborate
standalone. **Groups 2 and 3 are the exception** — their providers are public
and their prerequisite has already landed:

- `Data.Numeric.Nat.Arithmetic` — `add`, `mul` are `pub`; the census records it
  `[all-public]` / standalone `[ok]`, and §4.2 states it is already public and
  therefore absent from the missing-export set.
- `Core.Classes.LawfulClasses` — `pub fn leq_nat` at `LawfulClasses.ken.md:479`
  with its three attached laws `pub proof`. The census's `N` record marked this
  `[absent]` / `[higher]`; that record is **stale**. The atomic owner migration
  it called for is `CAT-ORD-NAT-CANONICAL-OWNER`, which has since merged.
- `Data.Numeric.Nat.Order` — `pub fn min` at `:49` and `pub fn sub` at `:69`.
  The census's `O` record marked these `[all-private]` / `[higher]` on the
  containing module's ownership failure; `CAT-ORDER-PUB-EXPORT` and the owner
  migration have both since merged, and `CAT-GCD-REFACTOR` landed a consumer
  importing from this module.

So the wall the census measured on this axis is down, and the pilot has already
walked the path once. Nothing else in the census is unblocked to this degree.

**Do not re-derive scope from the census's `N`/`O` provider records.** They
describe a tree that no longer exists. The fixed inputs below are the operative
measurement.

## Fixed inputs

Measured at `origin/main` `5e49b944571b96f5a05a5ec2abd32c0ba31796fe`.

Providers (all `pub` at this base; verified by the Steward, not inherited):

| Name to import | Canonical owner | Site at this base |
|---|---|---|
| `add` | `Data.Numeric.Nat.Arithmetic` | `pub fn add` |
| `leq_nat` | `Core.Classes.LawfulClasses` | `LawfulClasses.ken.md:479` |
| `sub` | `Data.Numeric.Nat.Order` | `Order.ken.md:69` |
| `min` | `Data.Numeric.Nat.Order` | `Order.ken.md:49` |

Consumers — the nine local duplicates, with their package blob at this base:

| # | Package (`catalog/packages/...`) | Blob | Local def | Line | Import target |
|---:|---|---|---|---:|---|
| 1 | `Capability/Process/Arguments.ken.md` | `f6b3c110` | `argument_nat_leq` | 57 | `leq_nat` |
| 2 | `Capability/Diagnostics/Core.ken.md` | `474db7a8` | `diagnostic_nat_leq` | 107 | `leq_nat` |
| 3 | `Capability/Parsing/Parsing.ken.md` | `c23dd723` | `nat_leq_bool` | 161 | `leq_nat` |
| 4 | `Capability/Formatting/Doc.ken.md` | `c38b64d1` | `pretty_nat_add` | 73 | `add` |
| 5 | `Capability/Formatting/Doc.ken.md` | `c38b64d1` | `pretty_nat_leq` | 79 | `leq_nat` |
| 6 | `Capability/Parsing/Cursor.ken.md` | `2edd1c0f` | `cursor_nat_add` | 114 | `add` |
| 7 | `Capability/Parsing/Cursor.ken.md` | `2edd1c0f` | `cursor_nat_sub` | 120 | `sub` |
| 8 | `Data/Collections/Derived.ken.md` | `1003406a` | `min` | 167 | `min` |
| 9 | `Data/Collections/Derived.ken.md` | `1003406a` | `nat_sub` | 815 | `sub` |

Two measured facts that shape the work:

- **No import cycle.** None of `Arithmetic`, `Order`, or `LawfulClasses` imports
  any of the six consumer packages. `Order` imports and re-exports
  `LawfulClasses`; `LawfulClasses` imports only `Core.Logic.*`. Importing from
  `LawfulClasses` is already demonstrated in-catalog by
  `OrderedSearch.ken.md:20`.
- **Four of the six consumers import nothing today** (`Arguments`,
  `Diagnostics/Core`, `Parsing/Parsing`, `Cursor`). `Doc` and `Derived` already
  carry `import` lines. So this batch also exercises the import mechanism on
  packages that have never used it.

## Deliverables — one independently-releasable increment per package

Ordered simplest first. **Each package is a separate, self-contained increment
that may be handed off and merged on its own** (`COORDINATION §14b`; accepted
work merges as soon as it is done, even a partial WP). Do not hold a finished
package waiting for the rest.

- **D1 `Capability/Process/Arguments.ken.md`** — import `leq_nat` from
  `Core.Classes.LawfulClasses`; delete `argument_nat_leq`; retarget its 3 refs.
- **D2 `Capability/Diagnostics/Core.ken.md`** — same, for `diagnostic_nat_leq`
  (3 refs).
- **D3 `Capability/Parsing/Parsing.ken.md`** — same, for `nat_leq_bool` (5 refs).
- **D4 `Capability/Formatting/Doc.ken.md`** — import `add` from `Arithmetic` and
  `leq_nat` from `LawfulClasses`; delete `pretty_nat_add` and `pretty_nat_leq`.
- **D5 `Capability/Parsing/Cursor.ken.md`** — import `add` from `Arithmetic` and
  `sub` from `Order`; delete `cursor_nat_add` and `cursor_nat_sub`.
- **D6 `Data/Collections/Derived.ken.md`** — import `min` and `sub` from `Order`;
  delete local `min` and `nat_sub`. **This is the risk increment; see AC-PROP
  and AC-PROSE. Do it last.**

## Acceptance criteria

- **AC-IDENTITY.** For each of the nine, the local body is compared against the
  canonical provider's body and the replacement is made only when they agree in
  complete type and transparent computation (up to bound-variable names) — the
  census's own standard, and the standard the Adversary applied to the pilot
  (`evt_17vavj42x7ebp`: all four removed Gcd bodies were byte-identical up to
  alpha). A name-spelling match is not evidence. If a local body is genuinely
  distinct, **keep it, say why in one line, and report it** — that is a correct
  outcome, not a failure.
- **AC-CANONICAL-OWNER.** Each name is imported from its canonical owner in the
  table above — `leq_nat` from `Core.Classes.LawfulClasses`, not through the
  `Order` facade's re-export. Imports are selective (name the imported names);
  no blanket module import is added.
- **AC-AMBIENT-DELTA.** For each touched package, the strict-resolution D0
  ambient census (`lang_mod_strict_resolution_d0`) is recorded before and after,
  and the delta is reported in the handoff. This AC requires a **measurement and
  a report, not a shrink** — the direction is not assumed. Rationale: the
  Adversary's pilot finding F1 (`evt_17vavj42x7ebp`, LOW, no defect) measured
  that Gcd's ambient census went from `{Equal, Proved}` to 12 names, the 9 added
  being lawful-classes machinery Gcd's own source never references, inherited
  transitively through the `Order` facade's blanket
  `export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)`. The
  Adversary explicitly did **not** run the census and stated it could not confirm
  whether importing from the canonical owner instead of the facade tightens the
  closure. This batch imports `leq_nat` from the canonical owner in five places
  and so answers that open question with evidence. Report the number either way.
- **AC-PROP (D6, the risk item).** In `Derived.ken.md`, `min` occurs inside the
  **statements** of two proved laws (around `:206` `length_take_min` and `:333`
  the `zip` length law), not only in computations. The census tagged this `[low]`;
  that tag is contestable on the census's own definition of `[higher]` ("the
  change moves ... a proposition"). So: replacing `min` must leave both law
  statements provable with **no new proof scaffolding**. If either proof does not
  go through by unfolding, **drop `min` from this WP, restore it, and report the
  item for a per-item decision** — do not repair it with added proof machinery.
  That escalation is the charter's "genuine mechanism gap HARD-STOPS to
  spec/Architect", and a gap finding here is a payoff, not a setback.
- **AC-PROSE.** `Derived.ken.md` §4.5 (around `:802`) carries a written
  rationale for the local `nat_sub` ("saturating `Nat` monus ... identical to the
  landed `val1_string_literals.rs:327` precedent"). Before deleting it, confirm
  `Order.sub` has that same saturating shape; then update the prose to name the
  import rather than leaving a rationale for a definition that no longer exists.
  Apply the same check to any other package whose prose justifies a def being
  removed.
- **AC-ORACLE.** Each touched package's existing acceptance oracle and attached
  proofs stay green, and the increment declares **no new `Axiom` of its own**.
  This is a behavior-preserving refactor; the review is differential.

  > ### OPERATOR RULING 2026-08-28 — ARM A. This supersedes the flat
  > ### "`trusted_base` delta zero" criterion, which is RETIRED as an oracle.
  >
  > **Zero-delta was a PROXY, and reuse is what kills it.** It tracked "this
  > package has not pulled in a provider" — true only while the package
  > reimplemented what it needed. Since importing from the canonical owner is the
  > campaign's entire objective, the proxy now measures nothing and would block
  > every remaining group. **Retiring it is authorized ONLY as a replacement by a
  > stronger direct pin, never as a deletion** — the same proxy-vs-property shape
  > already ruled for the `OrdResult` assertion at `evt_2r8cavz7b1bms`.
  >
  > **The replacement, stated as a predicate — do NOT re-express it as a per-file
  > roster.** For an increment that imports from a canonical provider, the
  > inherited axiom set must be **exactly equal, BY QUALIFIED NAME, to an
  > independently computed canonical-provider delta.** Independently computed
  > means derived from the provider itself, not read back off the increment —
  > deriving the expected set from the observed set makes the check vacuous.
  >
  > **Compare qualified names, never counts.** A count-equality passes on a
  > substitution that swaps one axiom for another.
  >
  > **The new assertion must be proven to DISCRIMINATE BY MUTATION**: an
  > inherited set differing from canonical must RED. QA blocked a prior respin
  > for a control that could not fail; a replacement oracle shipped on its own
  > say-so repeats that defect one level up. A control that cannot fail is not
  > weaker evidence — it is none.
  >
  > **THE BOUNDARY IS MEASURED — the growth is REAL, and a Steward hypothesis
  > that it might be illusory was REFUTED.** Measured `evt_11z9chtz3p9jj` at the
  > whole-catalog roots-loader boundary
  > (`elaborate_module_from_roots(..., Data.Collections.Derived)`), toggling only
  > the D6 Derived product blob and byte-restoring:
  >
  > - WITHOUT the import (base blob `1003406a`): Derived delta `{}`.
  > - WITH the import (candidate blob `c288e556`): Derived delta
  >   `{Ord.Int.antisym, Ord.Int.refl, Ord.Int.total, Ord.Int.trans}` — exactly
  >   equal by qualified name to the independently roots-loaded canonical `Order`
  >   delta, with **zero Derived-local addition**.
  >
  > **The Steward had hypothesized that `Nat.Order` was already inside the
  > accepted closure, which would have made the boundary delta zero and the
  > per-package number a measurement artifact. It is not, and it does not.** The
  > accepted boundary delta is genuinely non-zero. **This is why the fork was the
  > operator's to rule and not the Steward's** — it is real trusted-base growth
  > at the boundary the frame accepts against, not a bookkeeping shadow.
  >
  > **This is the campaign's standing rule for inherited provider axioms, BOUNDED
  > as follows.** Groups 4 and 5 hit the same inheritance and do NOT re-escalate
  > **while the measured inherited set is exactly the canonical provider's own
  > footprint with zero consumer-local addition** — that is the shape ruled
  > acceptable here. **An import whose boundary delta exceeds its provider's own
  > footprint, or adds any consumer-local axiom, is NOT covered and escalates
  > fresh.** The ruling authorized a shape, not an unbounded licence: the reason
  > this instance is acceptable is that the consumer pays exactly the provider's
  > published price and nothing more, and that is a measured property of each
  > import rather than a property of the campaign. A genuine mechanism gap still
  > hard-stops to spec/Architect.
  >
  > **Why this was the operator's call and not the Steward's:** a surface `Axiom`
  > is `declare_postulate` -> `Decl::Opaque` -> a real `trusted_base()` entry
  > (`docs/PRINCIPLES.md` principle 5, "postulating does not avoid TCB growth"),
  > and `trusted_base()` is a kernel function (`crates/ken-kernel/src/env.rs`).
  > Ken-source axioms are therefore NOT a weaker category than kernel trust.
  > **But note the open question the boundary measurement settles:** inheriting an
  > axiom already present in the accepted closure may grow nothing at all, in
  > which case this class of increment needs no escalation at all in future.
- **AC-SCOPE.** Only the six packages listed are edited. No provider module is
  edited — this WP adds no `pub`, closes no missing-export prerequisite, and
  touches none of the 27 `higher`-risk items or the 12 `BU` arrangement entries.
  Arrangement is **out of scope** here: this batch is reuse only.
- **AC-STOP.** The census records (§2, §4.3) that `Capability.Parsing.Cursor`
  does not elaborate standalone at its evidence base (`UnresolvedCon
  bytes_nat_length`). That is a **pre-existing** condition and is explicitly NOT
  this WP's to fix. Acceptance is at the whole-catalog roots-loader boundary, as
  the pilot's was. If a package cannot be completed for any such reason, drop
  that package's increment, land the others, and report the blocker — do not
  expand into a provider or loader repair. Expanding a consumer WP into
  dependency-package repair is the exact failure that stalled `CAT-GCD-REFACTOR`
  for four days.
- **AC-NO-REGRESSION.** Whole-suite green in CI (`COORDINATION §12`). Local
  checks are targeted only, never `--workspace`.

## Reviewers

foundation-qa + conformance-validator, against the catalog implementation
standard, as in the pilot. No Architect review is required for the reuse itself;
route to the Architect only if AC-PROP fires or another genuine design gap
surfaces.

## Capability tier and sequencing

**Tier T2.** The work is behaviour-preserving import-and-delete against
already-proved modules, and its review is differential (same laws, same
`trusted_base` delta, fewer local defs). The one reasoning-dense item is D6's
AC-PROP, which is a bounded hard-stop decision rather than open design. Size M,
structured as six independent increments so each lands within about an
implementer turn (`steward.md §4b`).

Lane 3 (foundation), the catalog-reuse modernization campaign's first scoped
rework batch (charter `docs/program/wp/catalog-reuse-modernization.md`, step 3).
Contention-free with the runtime priority lane: it touches `catalog/` only.

The census's remaining low-risk groups (1, 4, 5, 6, 7) stay unframed on purpose —
each needs a provider `pub`-export WP first, and three of their provider modules
carry standalone failures (`LawfulFunctors`, `BytesKeys`, `Cursor`, census §4.3).
Those export prerequisites are the next Steward framing step after this batch,
sequenced highest-leverage first: `Data.Collections.Derived` (standalone `[ok]`,
nine names demanded) and `Core.Classes.LawfulClasses` (standalone `[ok]`).
