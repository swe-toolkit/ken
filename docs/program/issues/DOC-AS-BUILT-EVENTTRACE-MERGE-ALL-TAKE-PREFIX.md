---
id: DOC-AS-BUILT-EVENTTRACE-MERGE-ALL-TAKE-PREFIX
title: "Two catalog landings add public laws that the library's catalog reference does not summarize: EventTrace's trace_window_merge_all and Derived's take::prefix. Bring the two cards and the laws.md projection up to the landed source"
status: active
owner: doc
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "Librarian as-built check evt_4pkktbea5xq85 on SPEC-TIME-WINDOW-MERGE-ALL (77a4e9907); SPEC-OPTIMIZATION-LEAST-K-PREFIX (e1f8a83af) makes take::prefix public in Data.Collections.Derived. Precedent DOC-AS-BUILT-DERIVED-EVENTTRACE-LAWFULCLASSES (6f74ebb53). Steward-filed per COORDINATION section 2."
---

# The catalog reference catches up with two new laws

## Objective

The library's catalog reference states the public laws that main's
`Data.Time.EventTrace` and `Data.Collections.Derived` declare, including
the two laws added since the cards were last written.

## Settled inputs (main `e1f8a83af`)

- **EventTrace.** `trace_window_merge_all`
  (`catalog/packages/Data/Time/EventTrace.ken.md:2578`) is public. It
  windows a merge of a list of traces and needs every trace in the list to
  be ordered. The card's Law row
  (`library/reference/catalog/Data/Time/EventTrace.md:13`) and its
  `laws.md` row (`:51`) do not mention it.
- **Derived.** `take::prefix` (`Data/Collections/Derived.ken.md`) is
  public: the k-prefix of an n-prefix is the k-prefix, given
  `leq_nat k n = True`. The card's rows (`Derived.md:12`, `:13`) and its
  `laws.md` row (`:44`) do not mention it.
- **The rule.** `library/reference/catalog/card-format.md` requires a Law row
  to summarize the declared laws. It flags a law addition for authored-fact
  review.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Those two cards and their `laws.md` rows, plus any other `library/` row that
the new public names make inaccurate. Library only. No catalog, spec, crate or
attestation edit; `SOURCE-ATTESTATIONS` and `STATUS.md` reconcile at the next
release point.

## Deliverable

One candidate on the doc ring, with Librarian review.

## Acceptance

- **AC-1.** Each new law is summarized with its premise stated:
  - `trace_window_merge_all` needs every input trace ordered;
  - `take::prefix` needs k ≤ n.
  The existing premise wording on each row stays accurate.
- **AC-2.** The Librarian checks each changed row against the source on the
  candidate's base. No row claims a law, premise or count that the source
  does not declare.

## Stop conditions

- A card would need a fact that the source does not settle.
- The change reaches beyond library/.

Checks:
- **3 fired.** No `library/` file names `trace_window_merge_all`,
  `take::prefix` or `least_k_prefix`. Besides the two cards and `laws.md`, the
  modules are cited by `declarations.md`, `assurance.md`,
  `effects-and-capabilities.md`, `guide/surface-reference.ken.md` and the
  manifest. The ring checks each of those for a row that the new laws make
  inaccurate.
- **8 fired.** A row that states a law without its premise is a wrong
  summary. AC-1 names both premises, so a premise-free summary fails review.
