---
id: SPEC-OPTIMIZATION-ARGMAX-OPTION
title: "Spec 72 section 3 supplies argmax only on NonEmpty and states that it gives no public argmax_option: a client holding a possibly empty List must rebuild the NonEmpty split itself, as it had to for argmin before section 1.4. Rule the greatest-cost Option selector's name, its Nil and Cons equations, and its member and greatest corollaries from a checked D0 at zero trust, and state them as a section 3 extension with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: [CAT-OPTIMIZATION-ARGMAX]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 72 section 3.3 excludes Option-returning maximization on a possibly empty input from section 3 and supplies no argmax_option. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md). Same shape as SPEC-OPTIMIZATION-ARGMIN-OPTION. Steward-filed per COORDINATION section 2."
---

# A greatest-cost selector over a possibly empty List

## Objective

Spec 72 states a public selector that takes a cost and a `List a` and
returns `None` on the empty list and `Some` of `argmax` on a non-empty
one, with checked laws that make that equivalence usable by a client.
It mirrors `argmin_option` (§1.4).

## Settled inputs (spec 72 §1.4 and §3, after `CAT-OPTIMIZATION-ARGMAX`)

- **`argmax` (§3.1).** `pub fn argmax (a : Type) (cost : a → Nat)
  (xs : NonEmpty a) : a`, in
  `catalog/packages/Algorithm/Optimization/Argmin.ken.md`, with the
  seven §3.2 laws, including membership, greatest cost and the
  earliest-tie rule. It is exported once `CAT-OPTIMIZATION-ARGMAX`
  (`0d63ab843`) lands.
- **The exclusion.** §3.3: "Option-returning maximization on a
  possibly empty input ... remain outside §3. In particular, §3
  supplies no public `argmax_option`."
- **The precedent.** §1.4 states `argmin_option` and four laws:
  `argmin_option_nil`, `argmin_option_cons`, `argmin_option_member`
  and `argmin_option_least`. Their private `_base` helpers split `Nil`
  from `Cons` and reuse the §1.2 laws.
- **Construction.** `NonEmpty`'s constructor is private. Clients build
  one with `nonempty_cons`. `Option` with `None` and `Some` is prelude
  data.
- **Seeds.** Under `conformance/stdlib/optimization/`, next to
  `seed-argmin-option.md`.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A §3 extension in spec 72 stating the ruled selector and each law by
its public type, the §3.3 exclusion sentence corrected, and a closed
conformance seed under `conformance/stdlib/optimization/`, with the
status lines changed in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0, no `Axiom`, and `trusted_base()` unchanged. It rules:
  - the selector's name and definition;
  - its `Nil` and `Cons` equations as public laws, the `Cons` one
    stated against `argmax` of `nonempty_cons`;
  - whether membership and greatest cost for a `Some y` result are
    public corollaries, as they are for `argmin_option`;
  - the private helpers the proof needs, and whether it shares
    `argmin_option`'s split.
- **AC-1.** The extension states each ruled operation and law by its
  public type, and the §3.3 sentence is corrected. The Spec leader and
  CV review it, with a Decision.
- **AC-2.** The seed checks the selector on `Nil`, on a singleton, and
  on a list whose earliest greatest-cost element is not its head and
  ties with a later one, so a selector that returns the head, the last
  tie, the least-cost element or `None` on a non-empty list fails. It
  has a refused block that claims `Some` on `Nil`. The candidate's
  handoff shows the seed parses (a `ken check` rc on its blocks),
  since the landed `seed-argmax.md` did not.

## Stop conditions

- A law that needs a decidable equality on `a`, an order on `a`, or
  function extensionality, unless the D0 rules it as the stated premise.
- Any new primitive, kernel or surface feature, or a change to `argmin`,
  `argmin_option`, `argmax` or their laws.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: `argmax` and its seven laws are `pub` at
`0d63ab843` (`Argmin.ken.md:161-229`), and `nonempty_cons` is imported
there; `argmin_option`'s four laws are stated in §1.4. 8 fired: AC-2's
tie row separates earliest from last tie, and its least-cost-element
case separates `argmax_option` from `argmin_option` on the same list.
