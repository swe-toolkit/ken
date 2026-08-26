---
id: CAT-ORD-NAT-CANONICAL-OWNER
title: "Migrate the canonical Ord Nat component to its defined-at home — move leq_nat (with refl/trans/antisym), total_leq_nat, the bool_or::eq_true_of_or bridge, and the sole instance Ord Nat from Data/Numeric/Nat/Order into Core/Classes/LawfulClasses, and make Order a reader-facing facade that imports and re-exports the LawfulClasses Ord surface. The atomic two-package ownership migration that resolves the OrphanInstance blocking CAT-ORDER-PUB-EXPORT."
status: ready
owner: foundation
size: M
gate: none
depends_on: [LANG-MOD-PUB-ELIGIBILITY, LANG-MOD-CATALOG-REALIZATION]
blocks: [CAT-ORDER-PUB-EXPORT]
github: null
origin: "Architect ruling evt_6f4h4mhejp4bm, 2026-08-26, splitting the held CAT-ORDER-PUB-EXPORT. The pub-only frame's repair fork was incomplete: neither an Order-local theorem nor a provider-owned bool_or bridge alone is lawful closure, because both leave the current instance Ord Nat orphaned in Order (reproduced as OrphanInstance { class: Ord, head_type: Nat } on a scratch tree at 2afacd0c0). The root predicate is one defect: Order was authored against ambient-provider scaffolding, so one file appears to own declarations that strict package ownership assigns to different defined-at homes. Steward-filed per the ruling; Steward owns the split, replacement ACs, and fresh kick."
---

> # FRESH PREREQUISITE 2026-08-26 — Architect split of CAT-ORDER-PUB-EXPORT (evt_6f4h4mhejp4bm)
>
> This is the prerequisite half of the split. [[CAT-ORDER-PUB-EXPORT]] stays
> `draft` depending on this node and resumes NARROWLY after it lands (mark Order's
> remaining `min`/`max`/`sub`/`compare` public). [[CAT-GCD-REFACTOR]] stays blocked
> until BOTH successor nodes land. Do NOT start any source repair before this frame
> lands. The design is fully specified by the Architect below; a genuine NEW gap the
> execution surfaces HARD-STOPS to spec/Architect, but the ownership direction,
> exact code, and controls here are the ruling and are not open for re-derivation.

## Objective

One atomic two-package ownership migration. Bring the complete canonical `Ord Nat`
component to its spec-mandated defined-at home in
`catalog/packages/Core/Classes/LawfulClasses.ken.md`, and reduce
`catalog/packages/Data/Numeric/Nat/Order.ken.md` to a reader-facing facade plus its
own Nat operations. This resolves the `OrphanInstance { class: "Ord", head_type:
"Nat" }` that the pub-only frame could not reach: adding canonical imports and
moving only `eq_true_of_or` advances the error to the orphan instance rather than
closing it.

## Authority (deductively fixed by settled spec — do not re-derive)

The direction is fixed by spec blobs identical at evidence base `2afacd0c0` and
current main (Architect evt_6f4h4mhejp4bm):

- `spec/30-surface/33-declarations.md` §5.3 — a compiler-floor head has no source
  head-owner; its canonical structure instance must use the class-owner arm. The
  `instance Ord Nat` is defined-at `Core.Classes.LawfulClasses`, keyed by the exact
  bootstrap `Nat` identity; an Order-local declaration is an orphan.
- `spec/30-surface/39-elaboration.md` §6.1 — same defined-at authority; Order does
  not register a second entry.
- `spec/50-stdlib/51-lawful-classes.md` §7/AC6 — LawfulClasses defines the one
  dictionary; the reader-facing Order module imports/re-exports the class surface
  and carries that same dictionary without redeclaration.
- `33 §8.2` — an attached proof takes the canonical `subject::proof_name` identity
  after resolving the subject, so a reusable `bool_or::eq_true_of_or` attachment
  belongs with the canonical `bool_or` in LawfulClasses, not in Order.

## Deliverables

### D1 — LawfulClasses becomes the defined-at home of the canonical Ord Nat component

In `catalog/packages/Core/Classes/LawfulClasses.ken.md`:

- MOVE, do not copy, `leq_nat`, its `refl`/`trans`/`antisym` attached proofs, and
  `total_leq_nat` from Order.
- Make `leq_nat` public. Make the three attached laws public at their provider if
  the public relation contract retains them. `total_leq_nat` may remain
  provider-private (only the local instance consumes it).
- Add the provider-owned public bridge below, composed from the two existing
  provider-local `bool_or` intro proofs:

```ken
pub proof eq_true_of_or for bool_or
      (p : Bool) (q : Bool) (h : Or (Equal Bool p True) (Equal Bool q True))
    : IsTrue (bool_or p q) =
  match h {
    Inl hp ↦ proof left_true_intro for bool_or p q hp;
    Inr hq ↦ proof right_true_intro for bool_or p q hq
  }
```

- Move the sole `instance Ord Nat` here, preserving its field terms exactly:

```ken
instance Ord Nat {
  leq = leq_nat;
  refl = proof refl for leq_nat;
  antisym = proof antisym for leq_nat;
  trans = proof trans for leq_nat;
  total = λx.λy.proof eq_true_of_or for bool_or (leq_nat x y) (leq_nat y x) (total_leq_nat x y)
}
```

### D2 — Order becomes the reader-facing facade plus Nat operations

In `catalog/packages/Data/Numeric/Nat/Order.ken.md`:

- DELETE, rather than retain aliases for, its local `leq_nat` proof spine, local
  `bool_or::eq_true_of_or`, `total_leq_nat`, and `instance Ord Nat`.
- Import and facade-re-export the relevant LawfulClasses surface. The
  probe-verified dependency/facade shape is:

```ken
import Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)
export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)
```

- Retain `min`, `max`, `sub`, `compare`, and the local `OrdResult` content
  unchanged in this prerequisite (their pub markings are the narrow resume of
  [[CAT-ORDER-PUB-EXPORT]], not this node).
- Update examples/prose so none claims Order defines `Ord_instance_Nat` or the
  bridge. A consumer demonstration exercises the carried dictionary through
  `where Ord Nat`, not a private generated global.

The import direction is load-bearing: LawfulClasses owns the instance and must NOT
import Order, or Order cannot import/re-export LawfulClasses without an
`ImportCycle`. Moving the relation/proof spine WITH the dictionary avoids both a
cycle and duplicate computation.

## Acceptance criteria (the ruling's required controls; each must be fail-able)

- AC-1 (strict roots) — strict roots checks for LawfulClasses and Order each pass
  independently green.
- AC-2 (closed instance census) — exactly one `(Ord, exact-floor-Nat)` instance
  entry exists, defined-at LawfulClasses; zero Order registration and zero second
  dictionary.
- AC-3 (carried dictionary identity) — an Order-only consumer that imports the
  re-exported `Ord`/`leq_nat` resolves `where Ord Nat` and receives that same
  dictionary identity.
- AC-4 (canonical leq_nat identity) — the Order public path for `leq_nat` resolves
  to the LawfulClasses canonical `GlobalId`; no alias or replacement declaration is
  minted.
- AC-5 (canonical bridge identity) — direct provider lookup of
  `bool_or::eq_true_of_or` resolves to the LawfulClasses attached proof; no
  `Data.Numeric.Nat.Order.bool_or::eq_true_of_or` identity exists.
- AC-6 (mutation controls, must redden) — moving the instance back to Order reddens
  as `OrphanInstance`; dropping the facade export reddens the Order-only consumer;
  introducing a local alias reddens the identity assertion (AC-4/AC-5).
- AC-7 (TCB and behavior) — zero `trusted_base()` delta; unchanged relation and
  dictionary behavior (this is an ownership migration, not a semantic change).
- AC-NO-REGRESSION — whole-suite green in CI (COORDINATION section 12); local
  targeted checks only, never `--workspace`.

## Reviewers

foundation-qa (the census is closed and defined-at LawfulClasses; the carried
dictionary, canonical `leq_nat`, and canonical bridge resolve to the LawfulClasses
identities; the mutation controls redden; zero TCB delta) + conformance-validator
(catalog implementation standard and the LawfulClasses defined-at/re-export
contract). A genuine NEW design/spec gap HARD-STOPS to spec/Architect — but the
ownership direction, exact bridge/instance code, and controls above are the
Architect's ruling, not open for re-derivation.

## Capability tier

T1 — a soundness-bearing ownership migration whose review turns on identity and
ownership arguments (a closed instance census, `GlobalId` resolution, the
`OrphanInstance` mutation controls), even though the Architect pre-built the exact
ledger and code. The reasoning is in verifying the controls, not in designing the
move. Size M.

## Sequencing and guardrails

Lane-3 (foundation), the prerequisite half of the CAT-ORDER-PUB-EXPORT split. It
unblocks [[CAT-ORDER-PUB-EXPORT]]'s narrow resume, then [[CAT-GCD-REFACTOR]].

The Architect mechanically prototyped this exact ownership direction on a scratch
tree at `2afacd0c0` — LawfulClasses standalone, Order standalone, and an Order-only
consumer selectively importing `Ord`, `leq_nat`, and `sub` and resolving
`where Ord Nat` all exited 0. That is buildability evidence ONLY, not a candidate
and not permission to reuse scratch prose; the durable identity and mutation
controls above remain required.

Guardrails (from the ruling and `docs/PRINCIPLES.md`):

- Move, do not copy — the relation/proof spine migrates WITH the dictionary.
- A new `OrderCore` package is NOT authorized (proliferates a module solely to
  route around ownership); a second local comparator is NOT authorized (violates
  canonical factoring).
- LawfulClasses must not import Order (cycle direction is load-bearing).

## Symptom inventory (carried from the split; never rewrite history)

1. Order's standalone boundary borrowed unimported canonical dependencies and
   declared both a foreign `bool_or` attachment and the orphan `Ord Nat`
   instance — keyed on ambient-provider scaffolding instead of defined-at package
   ownership. (Hard stop 1; Architect evt_6f4h4mhejp4bm.)
