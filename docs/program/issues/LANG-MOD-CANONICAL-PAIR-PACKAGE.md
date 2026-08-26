---
id: LANG-MOD-CANONICAL-PAIR-PACKAGE
title: "Canonical Pair-package identity prerequisite — atomically migrate to ONE ordinary package-defined public Pair identity over the existing kernel Sigma/Pair/Proj formers, retire the four compiler-installed public Pair globals, and migrate every consumer; re-enters the deferred Compare/LawfulClasses/Derived/Order/Gcd cluster into strict closure."
status: draft
owner: language
size: L
gate: none
depends_on: [LANG-MOD-PAIR-FLOOR-PROVIDER]
blocks: [CAT-GCD-REFACTOR]
github: null
origin: "Spec enclave boundary ruling (spec-author evt_6nk4xxkppz3k5, spec-leader evt_w7v4dvvzjr8k) + Architect mechanism ruling (evt_53295hb0v21mw), on Component B hard stop #1. The deferred cluster's re-entry prerequisite. Steward-filed under [[LANG-MODULE-IMPORT-SYSTEM]]."
---

> # REDIRECTED 2026-08-26 — now the FLOOR-REALIZATION build WP under [[LANG-MOD-PAIR-FLOOR-PROVIDER]]
>
> The operator + Architect recut (evt_7d0ecgkd8ate3) supersedes the exact-nine
> boundary: Pair is admitted to the prelude FLOOR by ONE general
> internal-provision arm, REUSING the four existing compiler-installed Pair
> `GlobalId`s (`prelude.rs:951-1041`) — NOT retiring them for a package Pair. This
> node is redirected from its package-migration plan to the floor-realization
> build WP that realizes the split inventories and flips the strict rows AFTER the
> spec WP [[LANG-MOD-PAIR-FLOOR-PROVIDER]] lands (`depends_on` repointed
> accordingly). The title/objective/deliverables below still describe the OLD
> retire-and-migrate plan and are superseded; they will be rewritten to the
> reuse-the-four-ids floor-realization contract against the actually-landed spec
> text, not before it.
>
> # FRAMED 2026-08-25 — deferred-cluster re-entry prerequisite (mechanism RULED)
>
> Component B's recut ([[LANG-MOD-CATALOG-COMPLETENESS]]) defers the whole
> native-`Pair`-dependent cluster (`Core.Logic.Compare`,
> `Core.Classes.LawfulClasses`, `Data.Collections.Derived`,
> `Data.Numeric.Nat.Order`, `Algorithm.Numeric.Gcd`, and every transitive
> consumer) as `deferred-on-canonical-Pair-package`. This node establishes the ONE
> public `Pair` identity that lets that cluster re-enter strict closure. The
> mechanism is RULED (Architect evt_53295hb0v21mw) — no open Decision fork.
>
> Held: DRAFT until its coupled boundary artifact [[LANG-MOD-PAIR-STRICT-BOUNDARY]]
> lands — "no Pair implementation is authorized before its coupled spec/conformance
> frame lands" (Architect). It is the gate on foundation [[CAT-GCD-REFACTOR]] (Gcd
> is in the deferred cluster).

# Objective

Establish exactly ONE public `Pair` identity in an ordinary catalog package and
retire the compiler-installed public convenience, so that under Strict every
Pair-dependent unit resolves `Pair`/`mk_pair`/`pair_fst`/`pair_snd` through an
ordinary explicit import from a defining public interface. No tenth floor member,
no native-provider registry, no special-case admission, no coexisting public
`Pair` identity, no compatibility alias.

# Mechanism (RULED — Architect evt_53295hb0v21mw; the spec's second arm)

Atomic migration to one package-defined identity, NOT exposed-native and NOT a
provider registry:

- The package defines TRANSPARENT `Pair` / `mk_pair` / `pair_fst` / `pair_snd`
  over the EXISTING kernel Sigma/Pair/Proj formers, and exports that one
  interface. No new kernel former; no `trusted_base()` growth.
- Every dependent unit imports that one interface.
- IN THE SAME LANDING: remove the four compiler-installed public `Pair` globals
  and migrate every consumer. Compiler internals may continue constructing raw
  kernel Sigma/Pair terms — the retirement is of the public convenience globals,
  not the kernel formers.
- INVARIANT: there is NEVER a tree with two public `Pair` identities, and NO
  compatibility alias to the retired identity. Exactly one identity serves the
  public `Pair` contract; import and re-export preserve it and allocate no
  replacement.
- The canonical package PATH is a taxonomy decision for this node (spec-owned);
  it does not change the mechanism.

# Deliverables

- D1 — the package: transparent `Pair`/`mk_pair`/`pair_fst`/`pair_snd` over the
  kernel formers, one `export` of that interface, at the canonical package path.
- D2 — atomic retirement + migration: remove the four compiler-installed public
  `Pair` globals; add the explicit import to every Pair-dependent consumer; land
  in one change so no tree ever carries two public `Pair` identities.
- D3 — re-enter the deferred cluster: the Component B deferred units
  (`Compare`/`LawfulClasses`/`Derived`/`Order`/`Gcd` + transitive) resolve `Pair`
  through the import and go strict-green; re-run Component B's disposition census
  from scratch so the deferred rows flip to `StrictGreen` and the temporary
  exclusions are removed. This also discharges the deferred ACs re-homed here:
  AC-B2 (Order/Gcd standalone-strict), AC-B2a (Order provider identity), AC-B4
  (Gcd imports), the Pair-dependent portion of AC-B7/AC-B9, and the whole-catalog
  end-state invariant.

# Acceptance criteria

- AC-1 — exactly one public `Pair` identity exists in the tree (the package's);
  the four compiler-installed public `Pair` globals are gone; no compatibility
  alias; no second `data Pair`.
- AC-2 — every Pair-dependent consumer resolves `Pair`/`mk_pair`/`pair_fst`/
  `pair_snd` by explicit import to the ONE package identity, with zero import
  allocation and no competing identity (established by IDENTITY, not repo text).
- AC-3 — the Component B deferred cluster
  (`Compare`/`LawfulClasses`/`Derived`/`Order`/`Gcd` + transitive) is strict-green
  through the real loader; Component B's disposition census re-runs from scratch
  and every previously deferred row is now `StrictGreen`.
- AC-4 — the whole catalog is strict-green in CI (the whole-catalog end-state that
  the Component B recut withdrew now closes here); the campaign catalog criterion
  in [[LANG-MODULE-IMPORT-SYSTEM]] closes and foundation [[CAT-GCD-REFACTOR]]
  unblocks.
- AC-5 — the [[LANG-MOD-PAIR-STRICT-BOUNDARY]] conformance pins hold: strict
  bare-`Pair` still rejects when NOT imported; a same-shaped local/package `Pair`
  is a distinct identity; the exact-nine floor is unchanged (no `Pair` added).
- AC-6 — zero `trusted_base()` delta; flat-Σ pin green; no new kernel former, no
  surface-syntax change, no mixed-resolution escape.
- AC-7 (definitional behavior — Architect evt_4tq5ad3sstkky / spec-author
  evt_3tgjjqmham8s0). `Pair` is transparently definitionally equal to the
  non-dependent kernel Σ; `mk_pair`/`pair_fst`/`pair_snd` are ordinary checked
  transparent declarations over Σ introduction/projection; fst/snd β and
  reconstruction η remain DEFINITIONAL (the existing map/view proofs that rely on
  them stay green). These are behavior, not a compatibility alias.
- AC-8 (nested-inductive positivity through reduction). Positivity reduces the
  transparent alias and judges the underlying Σ structure — it must NOT recognize
  the spelling `Pair`: a positive nested use accepts structurally, while
  `Pair (Bad → Empty) Unit` still rejects through the ordinary negative occurrence
  (the `14 §8` pin from [[LANG-MOD-PAIR-STRICT-BOUNDARY]]).
- AC-9 (prior obligations honored). The sole `instance Ord Nat` dictionary stays
  class-owned by `Core.Classes.LawfulClasses` and is only imported/re-exported by
  Order (not absorbed into it); the owed AC-B8 foreign-attached-proof conversions
  (from the deferred `LawfulClasses`/`Order` units) are discharged as part of the
  re-entry, not silently dropped.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p` only, never
  `--workspace`.

# Reviewers

Architect (mechanism fidelity: one package-defined identity over existing kernel
formers, atomic retirement, no second identity / alias / registry / floor member)
+ conformance-validator (identity-preserving import resolution; the deferred
cluster re-run; the boundary pins) + spec-author/spec-leader advisory (the
package path taxonomy + coupling to [[LANG-MOD-PAIR-STRICT-BOUNDARY]]).

# Capability tier

T1 — a cross-cutting, identity-preserving atomic migration touching the kernel
former surface (read-only), the compiler-installed public globals, and every
Pair-dependent consumer, with an exact one-identity end-state invariant. The
review turns on the identity argument, not a mechanical diff. Size L.

# Sequencing

Campaign work under [[LANG-MODULE-IMPORT-SYSTEM]]. DRAFT until
[[LANG-MOD-PAIR-STRICT-BOUNDARY]] lands (no Pair implementation before its coupled
spec/conformance frame). It is the true gate on the campaign's whole-catalog
criterion and on foundation [[CAT-GCD-REFACTOR]] — both of which the Component B
recut can no longer close. Sequenced within the LANG-MOD campaign tail ahead of
the verify/z3 FO-checker resume.
