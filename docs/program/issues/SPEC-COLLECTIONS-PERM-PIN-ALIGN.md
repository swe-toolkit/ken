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

37 §6's Perm sentences and the obligation form (Spec enclave); C5's two
sources and the named prose rows (CV). No catalog, crate, kernel or
prelude change; the pre-source Perm's retirement follows I4.

## Acceptance

- **AC-1.** 37 §6 says Perm is count equality applied to `eq_from_ord`
  of the sort's comparator, that no Perm is prelude, and that ‖Perm_rel‖
  is the named fallback. The obligation reads `is_sorted leq ys ∧ Perm
  (eq_from_ord leq) ys xs`. The Spec Leader and CV review it, with a
  Decision.
- **AC-2.** C5's two sources import Perm, is_sorted and eq_from_ord
  from `Data.Collections.Derived`. sound-verified-sort accepts and
  unsound-const-nil refuses, both on the checked path. If
  unsound-const-nil accepts, that is a soundness stop to the Architect.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The amendment needs a change to 57 §3.1 or to a catalog law.
- unsound-const-nil accepts under the count form.

Checks: 7 fired: the const-Nil counterexample was re-run against the count
form by the Architect. With leq quantified, the constant-True comparator
makes count the list length, so `Perm a (eq_from_ord a leq) Nil xs`
fails for nonempty xs; AC-2 pins that refusal. 8 fired: unsound-const-nil
accepts under a mutant Perm that ignores counts, and refuses under the
real one, so its verdict discriminates.
