---
id: SPEC-COLLECTIONS-PERM-PIN-ALIGN
title: "Spec 37 §6 still pins Perm as the comparator-free truncation and calls count equality rejected, against the later stdlib pin in 57 §3.1 that the catalog follows, so LANG-DELETE I4 cannot retire the pre-source Perm. Amend 37 §6 to the 57 §3.1 form, re-root C5's two sources on Data.Collections.Derived.Perm by import, and revise the prose rows"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: [LANG-DELETE-LEGACY-RESOLUTION]
github: null
origin: "Architect evt_3agt8q8hbz7sb (5) on LANG-DELETE I1's 42-row Perm population (implementer evt_458dkvstt138): an I4 precondition, beside the spec 33 co-turn. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# One Perm in the spec

## Objective

Spec 37 §6 and the verified-sort obligation state the Perm that 57 §3.1
pins, and the C5 challenge reaches it by import, so nothing in the spec
or the corpus depends on the pre-source truncation Perm.

## Settled inputs (main `cfaee2951`)

- **The pin.** 57 §3.1 (`spec/50-stdlib/57-collections-and-views.md`
  :125-160) pins `Perm (a) (eqf : a → a → Bool) (xs ys : List a) : Prop`
  as count equality, and keeps ‖Perm_rel‖ only as the fallback for
  carriers with no decidable equality. 57 §3.4 states the sort law as
  `Perm a (eq_from_ord a le) xs (sort a le xs)`. `Derived.ken.md` :1790
  and :1803 match it.
- **The stale text.** 37 §6 (`spec/30-surface/37-strings-collections.md`
  :674-692) pins the truncation and calls count equality rejected;
  :818-822 states the obligation as `is_sorted leq ys ∧ Perm ys xs`.
- **C5.** `conformance/challenge/C5-verified-sort/` has two sources,
  sound-verified-sort and unsound-const-nil, that use the pre-source
  Perm. The prose rows to revise are those in the Architect's ruling: the
  C2 and C5 READMEs, the taxonomy minimality seed, seed-collections, and
  the challenge and conformance READMEs.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Every spec 37 sentence that states the old Perm or obligation form (Spec
enclave); C5's sources, the closed pair and the named prose rows (CV);
the two C5 rows of one crate test (Steward evt_r8q1h0trvt51). No catalog,
kernel or prelude change; the pre-source Perm's retirement follows I4.

## Acceptance

- **AC-1.** 37 §6 says Perm is count equality applied to `eq_from_ord`
  of the sort's comparator, that no Perm is prelude, and that ‖Perm_rel‖
  is the named fallback. The obligation reads `is_sorted leq ys ∧ Perm
  (eq_from_ord leq) ys xs`. The Spec Leader and CV review it, with a
  Decision.
- **AC-2 (restated, Architect evt_583dm8zxz2fzc).** C5 is re-rooted by
  import from `Data.Collections.Derived`; `ken check` is the observation.
  - sound-verified-sort: rc 0. Every open obligation names both
    conjuncts, `is_sorted` and `Perm` over `eq_from_ord` of the sort's
    comparator, as the Derived identities; none is reported proved.
  - unsound-const-nil: rc 0 with exactly one open obligation, which
    contains the Perm conjunct over the Nil result and `xs`. Zero
    obligations, a Perm-free obligation or a proved one is a soundness
    stop to the Architect.
  - A new closed pair carries the accept/refuse split, both in
    `conformance/challenge/C5-verified-sort/`, with the README's Expected
    section listing them. `sound-closed-count.ken` is accepted, rc 0 with
    no open obligation. `unsound-closed-const-nil.ken` is refused, rc 1
    with KernelRejected TypeMismatch inside its theorem. A scratch
    mutation of the second list to `Nil Bool` is accepted.
  - The `lang_strict_isolated_routes_inc3a_diverge` C5 rows are deleted
    (15 to 13 cases); Language QA votes that path.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The amendment needs a change to 57 §3.1 or to a catalog law.
- `unsound-closed-const-nil` is accepted, or unsound-const-nil reports
  no Perm obligation.

Checks: 7 fired: the const-Nil counterexample was re-run against the count
form by the Architect. With leq quantified, the constant-True comparator
makes count the list length, so `Perm a (eq_from_ord a leq) Nil xs`
fails for nonempty xs; AC-2 pins that refusal. 8 fired: unsound-const-nil
accepts under a mutant Perm that ignores counts, and refuses under the
real one, so its verdict discriminates.
