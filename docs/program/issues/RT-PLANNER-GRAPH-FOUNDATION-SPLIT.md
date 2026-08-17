---
id: RT-PLANNER-GRAPH-FOUNDATION-SPLIT
title: "Move the planner's shared substrate out of the static-transition monolith — the root plan type and the identity vocabulary that all six planner domain slices quote"
status: active
owner: runtime
size: TBD
gate: none
depends_on: [RT-BACKEND-SPLIT-CENSUS]
blocks: []
github: null
origin: Cut item 3 of RT-BACKEND-MODULE-SPLIT, filed 2026-08-17 once RT-BACKEND-SPLIT-CENSUS merged and supplied the evidence the campaign deliberately withheld filing ahead of. Framing constraints binding per RT-BACKEND-MODULE-SPLIT:330-359 (operator, 2026-08-08). Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

> # `D0` REPORTED 2026-08-17 — A HARD STOP, AND IT REFUTES THIS NODE'S PREMISE
>
> **`AC-0` is DISCHARGED.** `D0` was commissioned to return a bounded ownership
> proof *or* the reason none exists, and it returned the second with every
> starting candidate refuted by name — which is exactly what `AC-0` asks for.
> Handback `evt_zres5xedzpdt`, leader's disposition request `evt_18tmxra00wh75`.
> **No branch, no candidate, no code.** `D0` cost one implementer turn.
>
> ### THE FINDING: THE ROOT IS NOT A SUBSTRATE, IT IS THE CONTAINER
>
> `StaticTransitionPlan<'src>` is quoted by all six planner domains, **and its
> private fields directly hold all six domains' storage.** Steward-verified
> against `6dedadd77`, struct spanning `static_transition.rs:2638-2791`:
>
> | domain | fields in the root |
> |---|---|
> | graph/occurrence | `entries`, `nodes`, `edges`, `stores`, `semantic_sources`, `semantic_material`, `semantic` |
> | units/ABI | `abi`, `root_ingress` |
> | continuation | `continuation_specializations` |
> | aggregate | `aggregate_ownership` |
> | effects | `host_effect_seats` |
> | joins/traps | `join_results`, `trap_catalog` |
>
> ⇒ **The root is shared, but its ownership boundary is not separable from the
> domain-owned storage inside it.** This node's title says "move the planner's
> **shared substrate**"; `D0` shows there is no substrate under the domains — the
> root sits *above* them and owns them.
>
> **Every identity candidate is excluded by the frame's own one-domain rule:**
> `PlannedTrapIdentity` joins/traps-only, `ContinuationSourceCoordinate` and the
> continuation identities continuation-only, `AggregateOccurrenceId`
> aggregate-only, `EffectSeat*` effects-only, `EmittableUnit` units/ABI-only. The
> shared vocabulary (`StaticOriginId`, `FieldIdentity`, `ConstructorIdentity`,
> `PredeclaredFunctionId`) **is already carved into `semantic_ir`** and must not
> move twice.
>
> ### BOTH CLOSURES ARE BARRED, AND NEITHER IS `D0`'s TO PICK
>
> - **Move the root declaration alone** ⇒ its private fields must widen to
>   `pub(super)` so the parent-owned impls still reach them. That is an internal
>   **production-surface widening**, which `AC-2` bars.
> - **Preserve privacy by moving both root `impl` blocks** (Steward-verified at
>   `:13621` and `:13721`, and those are the **only** two) ⇒ pulls most of the
>   planner, and the second crosses the inline test boundary at `:18428`,
>   importing the `cfg(test)` and `r3-4b-observation` profiles. That is the
>   mega-diff §2 exists to prevent.
>
> ### DISPOSITION — `D1` IS NOT CUT, AND THE PIVOT IS AN ARCHITECT QUESTION
>
> **I am not cutting `D1` against a seam report.** The live third option is to
> decompose the root's private storage into **per-domain sub-structs in place**,
> moving nothing across a module boundary: fields stay private, the same impls in
> the same file reach them, and each domain's storage becomes a named movable
> unit. Item 3 then moves a thin root plus six handles, and items 4-9 each take
> their own storage.
>
> **Whether that is barred by `AC-2`'s "no representation change" is a
> component-design call, and it is the Architect's** (§3). It is not obviously
> barred — nothing diagnostic, hash, serialization, behaviour or trust moves —
> but `AC-2` does say *representation*, and I will not read my own preferred
> answer into a campaign gate.
>
> ### ARCHITECT RULED, AND AGAINST THE PIVOT. `evt_559kbpkvacqyg`, 22:14.
>
> **`AC-2` BARS the in-place per-domain storage decomposition inside this node.**
> *"Representation" is an independent member of that list*; reading it as only
> externally observable would make it redundant with the later clauses and erase
> §4/§8's explicit hard stop. Replacing the flat private fields with six nested
> records changes the type/field graph, construction and access paths,
> layout/borrow topology, and what the existing `Clone` derive traverses.
> **Absence of `repr`, serialization, hash or a public field shows low observable
> risk — it does not show representation identity, and only identity authorizes
> a pure move.**
>
> **And a SECOND closure failure, independent of `AC-2`, which I had not seen.**
> Decomposing in place **does not** make "move the thin root, leave the two impls
> in the parent" private: once the root is declared in a child module, **the
> parent cannot reach the child-private six handle fields.** The same three
> choices recur — widen, move the impls, or add an accessor layer — and `D0`
> already barred the first two. ⇒ **My proposed precursor would not have
> established the movable set even if `AC-2` had allowed it.** A fresh closure
> proof would still be owed.
>
> ### THE CAMPAIGN RECUT, WHICH IS MINE — AND ITEM 3 MAY NOT SURVIVE IT
>
> The Architect's disposition: treat `StaticTransitionPlan` as the **container /
> orchestrator ABOVE the six domains**, not a foundation beneath them, and keep
> it in the parent. **A child domain module may host domain-specific `impl`
> blocks that access ancestor-private root state** — that direction preserves the
> private root boundary and is the one that stays open.
>
> ⇒ **Re-evaluate item 3 as only the genuinely shared graph/identity foundation,
> and if no useful closed set exists independently of the root, DELETE OR REPLACE
> item 3 rather than manufacturing one.**
>
> **`D0` has largely answered that already, and the answer points at deletion.**
> It found every identity candidate single-domain (`PlannedTrapIdentity`,
> `ContinuationSourceCoordinate`, `AggregateOccurrenceId`, `EffectSeat*`,
> `EmittableUnit`) and the shared vocabulary — `StaticOriginId`, `FieldIdentity`,
> `ConstructorIdentity`, `PredeclaredFunctionId` — **already carved into
> `semantic_ir`.** Subtract the root, subtract the single-domain identities,
> subtract what `semantic_ir` owns, and the residue of a "shared foundation" is
> empty. **This is §4c: the constraint demanding this node may simply not be
> real.**
>
> **I am not deleting a campaign item on my own inference.** Item 3's framing
> constraints are the operator's (`RT-BACKEND-MODULE-SPLIT:330-359`, 2026-08-08),
> so deleting or replacing it is scope, not sequencing. **Routed to the
> operator** with the recommendation above.
>
> **If the permanent design still wants six private storage records**, the
> Architect's terms are: a **separately authorized representation-change
> precursor**, not part of a pure-move `D1` — replacing the flat form once with
> no parallel old/new representation, carrying exact old-to-new storage,
> constructor/writer, accessor, derive/layout and `cfg` ledgers, landing without
> module motion, **and then `D0` is rerun against the landed representation.** Do
> not pre-claim the root is movable.
>
> Grounded in `research/compiler-refactoring-program.md` §§5.2 and 7.3, and the
> obligation-IR report's parallel-representation warning.

> # `ready` 2026-08-17 — CUT ITEM 3, THE FIRST PLANNER SLICE
>
> **Frame: `docs/program/wp/RT-PLANNER-GRAPH-FOUNDATION-SPLIT.md`.** It is
> shovel-ready for `D0` and deliberately unsized past it.
>
> ### WHY IT IS FILED NOW AND WAS NOT BEFORE
>
> [[RT-BACKEND-MODULE-SPLIT]] filed only cut items 1 and 2 on purpose: *"filing
> all sixteen now would create work ahead of the evidence that sizes it."* The
> evidence was the census, and **[[RT-BACKEND-SPLIT-CENSUS]] merged** — its
> type-ownership inventory records all 76 planner-owned type declarations with
> visibility and full external consumer sets. **The constraint that deferred this
> node has lapsed; the node is not new work.**
>
> ### THE CENSUS IS CURRENT FOR THE PLANNER — BUT NOT FOR THE REASON FIRST WRITTEN
>
> The inventories pin measurement SHA `4de48651`. This block used to say
> `planning/` had **zero commits and an empty diff** since then. **True at
> `c03331ad8`, false at `af29848f7`** — `168e8bbf8`
> (`RT-SITEOP-CARRIED-WITNESS` `D2`) landed `+54/-6` in `static_transition.rs`.
>
> **The rows are still current, and now that is measured against the property
> rather than a proxy:** the type-declaration set is unchanged (161 both sides,
> declaration-line diff empty), and `lowering/mod.rs`'s
> `use super::planning::{…}` blocks are byte-identical. Bodies moved; the
> surface the census inventoried did not.
>
> ⇒ `D0` starts from the census rather than re-taking it. **The pickup check in
> the frame's §1 was corrected accordingly** — a non-empty diff over `planning/`
> is *not* a stale signal, and the old one-command form would have stopped the
> ring on a good base.
>
> ### `size: TBD` IS DELIBERATE. Do not fill it in from a line count.
>
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] landed cleanly because the census's
> `D6` had already returned a **bounded ownership proof** before anything moved.
> **No such proof exists for the planner foundation.** `D0` produces it, and the
> Steward cuts `D1` onward against what it returns.
>
> **`RT-BACKEND-MODULE-SPLIT:330-359` bars carrying today's line counts into this
> frame**, and the landed research reports independently warn against optimizing
> for equal-sized files. The frame omits the counts for that reason, not by
> oversight.

## What it owns

**The smallest set that unblocks cut items 4-9** — the six planner domain slices
(units/ABI, occurrences, continuations, aggregates, effects, joins/traps). They
are separable from each other and **not** separable from the root plan type and
the identity vocabulary they all quote.

The starting hypothesis, read off the census and stated so `D0` can refute it:
`StaticTransitionPlan<'src>` (`planning/static_transition.rs:2638`) plus the
shared identity and coordinate types. **A type only one domain references belongs
to that domain's slice, not here.**

## What it is not

- **Not a planner mega-diff.** `RT-BACKEND-MODULE-SPLIT:89-93` — a census merge
  permits one frame with independently reviewable commits and nothing more.
- **Not `#8` closure.** One accepted phase partial among eighteen.
- **Not a venue for the IR architecture.** The frame cites both landed research
  reports for the Architect; **reference is not adoption**, and this stays a
  behaviour-preserving split unless the Architect rules otherwise.

## The known trap

`planning.rs` carries `#[cfg(test)]` and `#[cfg(any(test, feature = ...))]`
gated re-exports, and warns three times in its own doc that **an ungated use of a
`cfg(test)`-gated re-export is an unresolved import in the production build that
the test profile cannot see.** A targeted test run is precisely the instrument
that cannot catch it. Same class as the `cfg`-context and path-relative
resolution gap the Adversary named on the primitive-lowering split.

## The carve pattern already exists in this file

`abi.rs` and `semantic_ir.rs` were carved out of the same monolith by
[[RT-NATIVE-FNSPLIT]]'s `B1`/`B1R` recuts, declared at
`planning/static_transition.rs:8-9`. **Copy that shape; do not invent another.**
