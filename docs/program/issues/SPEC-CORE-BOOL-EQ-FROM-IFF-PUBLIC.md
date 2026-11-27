---
id: SPEC-CORE-BOOL-EQ-FROM-IFF-PUBLIC
title: "Three catalog packages (Rational, Dependency, RationalInterval) each carry a byte-equivalent private bool_eq_from_iff, the proof that two Booleans are equal when each being True implies the other, because every WP that needed it was scoped to its own package. Publish it once from Core.Classes.LawfulClasses at zero trust, migrate the three packages to import it, retire the private copies, and update the spec text and privacy tests that name them"
status: ready
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect carry evt_15q6qwa3yyh67 (third copy arrived with SPEC-NUMERICAL-INTERVAL-ADD-POINT-EXACT, 648e4b782); Steward disposition evt_5pwnz6g6q94mm: an L3 WP after the Time node. Spec 62 §15 already names it as a separate Core node. Operator 2026-10-08: \"start l3 on the establish catalog program\". Steward-filed per COORDINATION section 2."
---

# One public Boolean equality from mutual implication

## Objective

`Core.Classes.LawfulClasses` exports a public checked theorem. For every
`p` and `q` it states

`(Equal Bool p True → Equal Bool q True) → (Equal Bool q True → Equal Bool
p True) → Equal Bool p q`.

It holds at zero trust. Rational, Dependency and RationalInterval import it,
and none of them keeps a private copy.

## Settled inputs (main `5f6140e24`; anchors re-checked at release)

- **The three copies.** Each is a private `theorem bool_eq_from_iff` that
  splits both Booleans and closes each arm by `Proved` or `absurd`:
  - `Data/Numeric/Rational.ken.md:1455`, used at `:1291`;
  - `Algorithm/Graphs/Dependency.ken.md:1708`, used at `:483`, `:561`,
    `:592`, `:632` and `:657`;
  - `Algorithm/Numeric/RationalInterval.ken.md:2009`, used at `:1921`.
- **The imports.** Rational (`:40`) and RationalInterval (`:63`) import
  `Core.Classes.LawfulClasses` selectively. Dependency (`:41`) imports it
  whole, so a public Core name collides with its private copy unless that
  copy is retired in the same candidate.
- **The privacy pins.** Two tests assert that the helper is private:
  - `cat_numeric_interval_add_point_exact.rs:259`, in
    `point_shift_proof_helpers_are_private`;
  - `cat_graphs_union_reachable_comm.rs:270`, in
    `union_reachable_comm_helpers_remain_private`.
- **The spec text that names it.**
  - Spec 62 §15 (`62-graphs.md:1515`) says a separate Core node "would
    publish `bool_eq_from_iff`". `:1502`, `:1596`, `:1688`, `:1774` and
    `:2034` call it private.
  - Spec 68 §24 (`68-numerical.md:2102-2104`) uses "the private Boolean
    equality proof" and says "All new helpers stay private."
  - The package prose at `RationalInterval.ken.md:2367` and `:2415`, and at
    `Dependency.ken.md:1175-1201`.
- **Why this supersedes the keep-private choices.** Spec 62 and spec 68 kept
  the helper private because each WP forbade a new export outside its own
  package. Neither ruled that it should stay private. Three copies of one
  generic Core fact is the proliferation `docs/PRINCIPLES.md` rejects, and
  spec 62 §15 already planned this node.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

- The Core theorem and its spec 51 statement.
- The three migrations and the retirement of their private copies.
- Updates to the two privacy pins.
- The spec 62, spec 68 and package-prose sentences that call the helper
  private.
- A seed, and the roster and count lines.

No other law type moves, and there is no kernel or surface change. A
`bool_or` commutation law is out of scope.

## Deliverable

One candidate carrying the Core, spec, seed and migration halves.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on the
  landed base, with rc 0, no `Axiom`, and `trusted_base()` unchanged as an
  identity set. It rules:
  - the name, the binder order and the placement in LawfulClasses and
    spec 51;
  - whether each migrated call site type-checks unchanged against the
    public binder order;
  - every consumer root of the touched exports, swept across `catalog/`,
    `conformance/`, `crates/`, `spec/` and `library/`, including any exact
    export roster and count-bearing sentence for LawfulClasses, Rational,
    Dependency and RationalInterval.
- **AC-1.** Spec 51 states the law by its public type. The spec 62 and 68
  sentences no longer call it private. CV and the Architect review it, with
  a Decision.
- **AC-2.** The seed's accepted row applies the law to open `p` and `q` with
  both implications. Its refused twin, on a fresh accepted prefix and in the
  same form, supplies only the forward implication and claims `Equal Bool p
  q`. That claim must refuse.
- **AC-3, migration.**
  - Each of the three packages loads with no `bool_eq_from_iff` declaration
    of its own.
  - Its existing package tests pass.
  - The two privacy pins drop the helper from their private lists.
  - A new pin shows that the helper imports from LawfulClasses.
- **AC-T.** The package tests assert `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-4.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise.
- A migrated call site needs more than an import change and the removal of
  its copy.
- The export creates an import cycle or a name collision that the
  migration does not remove.
- AC-T finds `trusted_base()` changed as an identity set.

Checks: 4 fired. I read the three copies, their call sites and their
packages' LawfulClasses imports on `5f6140e24`. The copies match up to
binder names, and Dependency's whole-module import makes retiring its copy
in the same candidate necessary. 3 fired. I grepped `catalog/`, `spec/`,
`crates/`, `conformance/` and `library/` for the name and found the two
privacy pins and the spec 62 and 68 sentences listed above. 8 fired. The
refused twin drops one implication, so a law that needs both is
distinguished from one that does not.
