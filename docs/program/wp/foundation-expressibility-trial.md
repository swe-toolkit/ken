# Foundation expressibility trial (bounded, operator-directed)

**Owned by the Steward.** A bounded trial, directed by the operator
(2026-08-21), of a third build lane seated on Foundation (openai): **can
Foundation's catalog translation work be expressed in Ken's current surface?**
Foundation authors the catalog as Ken's first user (`06-catalog-campaign.md`),
and catalog authoring is essentially a **translation task** — taking algorithms
and data structures well understood in other languages (some dependently typed,
some not) and expressing them, with their laws proved, in Ken. This trial
re-tests that expressibility against the surface as it stands today.

This doc fixes the trial's purpose, target set, stop-on-gap protocol, and the
decision rule the Steward executes autonomously. Each target has a thin issue
node (`docs/program/issues/CAT-*.md`) pointing here.

## Why now

The catalog is actively built — the Core and Data Sections are largely landed
(equality/transport, `Dec`, lawful classes, `Nat`/`List`/`Map`/`Set`, the
effectful classes, a `Json` carrier). The **Algorithm Section named in the
charter is entirely absent** — sorting, searching, numeric and structural
algorithms — and it is the canonical translation frontier: classic procedures
whose reference formulations are widely known, to be re-expressed and **proved**
in Ken. A LOT of language-surface work has landed recently, so the expressibility
frontier has moved; this trial measures where it now sits.

## The target set — five independent translation WPs

Chosen non-sequentially: none depends on another (each is a self-contained
package; where one names another's concept — e.g. an ordered search over a
sorted structure — it takes that as a **hypothesis**, not a build dependency).
The set spans the dependency-typedness axis the operator named. Run order is
simplest-first.

| # | node | home | dep-typing | probes |
|---|---|---|---|---|
| 1 | `CAT-SORT` | `Algorithm/Sorting/InsertionSort.ken.md` | non-dependent | induction/motive over `List`, decidable `Ord` use, permutation reasoning |
| 2 | `CAT-DEQUE` | `Data/Collections/Deque.ken.md` | non-dependent | invariant maintenance, a sequence-abstraction homomorphism law |
| 3 | `CAT-BSEARCH` | `Algorithm/Searching/OrderedSearch.ken.md` | mildly dependent | `Dec`/`EmptyDec` ergonomics, sortedness as a refinement hypothesis |
| 4 | `CAT-GCD` | `Algorithm/Numeric/Gcd.ken.md` | non-dependent | **well-founded / structural recursion + termination presentation** |
| 5 | `CAT-VEC` | `Data/Vector/Vector.ken.md` | fully dependent | length-indexed inductive surface, total elimination (`Fin`/`Lt` — **`Fin` is absent today**) |

Per-target objective + law obligations are in each node. `CAT-VEC` is the
deliberate one fully-dependent probe and carries the highest expressibility risk
(`Fin` and length-indexed elimination); if it cannot be expressed cleanly today,
that is a designated finding, not a surprise.

## Scope: functional build (phase 1), refinement deferred

Per the two-phase catalog cadence (`06` → "Two-phase quality cadence"), each
target is a **functional discovery/build**: the component exists, runs, and its
required laws are **really proved** (not postulated), with the derivation path
stated and the `trusted_base()` delta honest. Guide-quality literate refinement
(`07-catalog-style-guide.md`) is **deferred** — it is not part of this trial and
does not gate a target. Author as `.ken.md` with minimal functional narrative.

The §2c enclave abstraction-boundary pin is **deferred by design**: the trial is
the triage. The implementer (T1) chooses the encoding; a genuine surface fork
becomes a filed gap (below), not an up-front enclave pull. This is what keeps the
third lane's Architect load near zero during the operator-away window.

## The stop-on-gap protocol (per target)

Run the five non-sequentially. For each:

- **It builds** (component + laws proved, ACs met) → hand back to the lieutenant
  as a normal candidate; move to the next.
- **It hits a Ken expressibility / surface failure** → **STOP that target**,
  **file the gap as open work** (a `docs/program/issues/` finding node naming the
  exact surface/kernel capability missing and the reference formulation it
  blocks), **move to the next**. Do **not** grind, do **not** pull the Architect
  per-gap — filed gaps queue for batched enclave triage after the window.

**What counts as an expressibility failure** (the countable issue): the
component or one of its required laws **cannot be expressed or proved in Ken's
current surface** without a new language-surface or kernel capability that does
not exist today. **What does NOT count:** a proof that is merely hard; a missing
catalog lemma below it (that is demand-pull — build it, or file a sub-target); an
ergonomics annoyance or a sugar wish (that is a **Finding**, filed and routed per
`06`, but it is not a stop and not a counted issue).

## Decision rule (Steward executes autonomously)

Across the five, count expressibility failures as defined above:

- **0 or 1** → the lane's expressibility hypothesis holds; **continue** the lane
  (queue more catalog/Algorithm-Section translation WPs).
- **2 or more** → **close** the lane; stand Foundation down, keep the filed gaps
  as open work, and report "try something else" on the operator's return.

The rule is deterministic, so the Steward applies it during the operator-away
window and reports the outcome — per-target result (built / expressibility-blocked
with the filed node) and the count — on the operator's ~05:00 UTC return.

## Routing

Owner: Foundation. Author = foundation-implementer (T1, gpt-5.6-sol);
foundation-leader (T2) sequences and routes; foundation-qa (T2) reviews.
`git_request` routes **direct to the lieutenant** (all-lanes merge router); the
Steward audits landings by blob identity. Gate: none on every target (catalog
outer-ring components, no TCB surface). Findings route per `06`
(kernel-reduction defect → Kernel via enclave; sugar/abstraction → Ergo) but are
filed, not pulled, during the window.
