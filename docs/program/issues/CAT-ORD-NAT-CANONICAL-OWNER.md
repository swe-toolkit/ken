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

> # AC-1 AMENDED 2026-08-26 — impossible inherited Strict prerequisite (Architect evt_1x8hypwysf9n3)
>
> The ownership migration DIRECTION remains correct and is NOT recut. The WIP
> object `4fc929982a2afa149943e85fb23f998229d84820` (tree `57a3417cc`, base
> `6f00843de`) is EVIDENCE ONLY, not a candidate — it moves the full
> relation/laws/totality/bridge/dictionary component to LawfulClasses, leaves
> Order the exact facade import/re-export + its own Nat ops, empty current-main
> changed-path intersection. No QA, no Decision on it.
>
> The hard stop (foundation-implementer evt_bwc4qxj9stcs) is that AC-1's strict-
> root green is IMPOSSIBLE and inherited, NOT caused by the Nat move: LawfulClasses
> imports `Core.Logic.Compare`, whose public signatures use `Pair`/`pair_fst`/
> `pair_snd`; `spec/30-surface/33-declarations.md §3.3` fixes the Strict floor at
> exactly `{Auth, Bool, Char, List, Nat, Option, ResourceKind, Result, Utf8Error}`
> and EXCLUDES `Pair`; current main has no catalog `Pair` provider; the unchanged
> kickoff-base Compare root fails identically at `UnboundName Pair`. Requiring this
> two-file Nat ownership move to make LawfulClasses/Order strict-green asks it to
> discharge a pre-existing deferred-cluster prerequisite outside its scope.
>
> That prerequisite is ALREADY OWNED by the durable program:
> [[LANG-MOD-CANONICAL-PAIR-PACKAGE]] (atomic one-identity Pair migration — its
> AC-3 re-enters Compare/LawfulClasses/Order into Strict closure; its AC-9 preserves
> the sole `Ord Nat` dictionary as class-owned by LawfulClasses while Order
> re-exports it). [[LANG-MOD-CATALOG-COMPLETENESS]] classifies this whole cluster
> `DeferredOnCanonicalPairPackage` and separately says the Nat ownership home
> proceeds. Hard stop 1 (misplaced defined-at ownership in Order) and hard stop 2
> (inherited canonical-Pair absence via Compare) are INDEPENDENT — no shared
> predicate, no Ord/Nat recut. Hard-stop count 2; Research not triggered.
>
> PROHIBITED (all cross the fixed component scope): add `Pair` to the floor, invent
> a local `Pair`, create another Pair node, fold the existing Pair realization into
> this WP, or weaken Strict resolution. This WP introduces NO Pair provider, floor
> member, fallback, or resolution change. Steward AC amendment below; symptom-
> inventory fold `93dae30cf88885031a42f0b76fffd24dbb612236`. Foundation HELD until
> the amended frame lands and is re-released.
>
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

- AC-1 (deferred-boundary control — REPLACES the impossible strict-green roots,
  Architect evt_1x8hypwysf9n3) — on BOTH base and candidate, derive the exact
  dependency path `Order -> LawfulClasses -> Compare -> canonical Pair interface
  unavailable` from parsed module identities and the current canonical-provider
  inventory. Strict loading of LawfulClasses/Order remains
  `DeferredOnCanonicalPairPackage`; the `UnboundName Pair` text is CORROBORATION,
  not membership authority. The candidate must introduce NO Pair provider, floor
  member, fallback, or resolution change. The unchanged-BASE Compare strict refusal
  is the positive control proving the deferred boundary is live.
- AC-1b (buildability controls only, NOT Strict-closure evidence) — the two
  ordinary/compatibility-root elaborations pass for LawfulClasses and the Order
  facade on the candidate. These are buildability evidence ONLY; they are
  explicitly NOT Strict-closure evidence and the report must say so.
- AC-EXEC-SURFACE (governs AC-2..AC-6, Architect evt_1x8hypwysf9n3) — AC-2..AC-6
  remain MANDATORY and are NOT weakened to source text. Because Strict closure is
  deferred, run them through the loader/artifact identity surface that CAN elaborate
  the current deferred cluster (the compatibility/ordinary-root surface): inspect
  canonical `GlobalId`, defined-at module, re-exported-at path, and instance-registry
  ownership — never grep, frozen numeric IDs, or prose. Compatibility supply of the
  unrelated native `Pair` dependency does not mint or change the Nat identities under
  test, but the report MUST disclose that these results are not Strict evidence. If
  the loader cannot expose those ownership/re-export identities, OR any negative
  control (AC-6) is masked by the `Pair` failure even on the compatibility-root
  surface, HARD-STOP that specific AC — do NOT replace it with a source-text
  assertion and do NOT claim the migration verified.
- AC-2 (closed instance census) — exactly one `(Ord, exact-floor-Nat)` instance
  entry exists, defined-at LawfulClasses; zero Order registration and zero second
  dictionary. Read from the instance registry, not source text.
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
- AC-STRICT-RERUN (deferred gate, Architect evt_1x8hypwysf9n3) — the node records
  that FINAL Strict closure of LawfulClasses/Order and the same identity assertions
  (AC-2..AC-6) RERUN after [[LANG-MOD-CANONICAL-PAIR-PACKAGE]] lands. This WP does
  NOT close or bypass that later Strict gate; it establishes the ownership move and
  the deferred-boundary control, with Strict-green owed downstream.
- AC-NO-REGRESSION — whole-suite green in CI (COORDINATION section 12); local
  targeted checks only, never `--workspace`.

## Reviewers

foundation-qa (the census is closed and defined-at LawfulClasses; the carried
dictionary, canonical `leq_nat`, and canonical bridge resolve to the LawfulClasses
identities via the loader/registry surface, not source text; the mutation controls
redden; zero TCB delta; the AC-1 deferred-boundary control is honest and the
candidate introduces NO Pair provider/floor/resolution change; AC-1b is disclosed
as buildability-only, not Strict evidence) + conformance-validator (catalog
implementation standard and the LawfulClasses defined-at/re-export contract; the
deferred `DeferredOnCanonicalPairPackage` boundary is corroborated, not asserted as
membership). A genuine NEW design/spec gap HARD-STOPS to spec/Architect — but the
ownership direction, exact bridge/instance code, and the amended controls above are
the Architect's ruling, not open for re-derivation. Strict-green is NOT required
here (AC-STRICT-RERUN defers it to [[LANG-MOD-CANONICAL-PAIR-PACKAGE]]).

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
2. AC-1's strict-root green is an impossible inherited prerequisite: LawfulClasses
   imports `Core.Logic.Compare`, whose public interface uses canonical `Pair`, which
   the Strict floor (`33 §3.3`) excludes and no catalog provider supplies; the
   unchanged-base Compare root fails identically at `UnboundName Pair`. Independent
   of hard stop 1 (no shared predicate); owned by [[LANG-MOD-CANONICAL-PAIR-PACKAGE]],
   deferred `DeferredOnCanonicalPairPackage`. (Hard stop 2; Architect
   evt_1x8hypwysf9n3; durable inventory `93dae30cf88885031a42f0b76fffd24dbb612236`.)
