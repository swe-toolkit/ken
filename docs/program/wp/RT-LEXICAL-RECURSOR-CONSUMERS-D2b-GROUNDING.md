# RT-LEXICAL-RECURSOR-CONSUMERS — `D2b` grounding

**Evidence only: no repair, no control, `crates/` byte-identical.**

## 0. Provenance — two coordinates

| coordinate | exact | moves? |
|---|---|---|
| **measurement base** — the tree every figure was taken on | `a6186741` | **no.** A commit on `main` |
| **candidate merge-base** — where this candidate sits | `a6186741` | **yes.** Every re-anchor moves it |

They coincide **today**. They are still recorded separately, because one of them
is not stable and this lane has been blocked three times for collapsing them
into one sentence.

> # THE GATING CONDITION PASSES. `D2b` IS NOT BLOCKED.
>
> The ruling's stop clause — *"if closed projection is unavailable, stop rather
> than widening guard/ABI"* — **does not fire.** The closed projection is
> available from existing interned planner facts, measured below.
>
> This checkpoint stops for **budget, not for a blocker.** §4 states exactly
> what remains.

## 1. The defect, confirmed and sharper than stated

`ContinuationUnitView::ordinary_envelope`
(`planning/static_transition.rs:1726`) builds its nonrecursive population as:

```rust
let mut nonrecursive = (0..field_count)
    .filter(|position| *position != selected)   // selected = key.recursive_position
```

⛔ **It omits exactly ONE position**, `self.key.recursive_position` — because
the key carries a **singular** `recursive_position`, not a set. Every other
constructor field is called nonrecursive by construction.

Row 3's producer has **two** recursive positions, so the second one survives
into the ordinary ABI run and a `Specialized(Closure)` is cloned into it. That
is the pre-loop misclassification the ruling names, and the method's own
`D8l2` comment block is about a *different* defect in the same loop (envelope
index vs source position) that was already repaired.

## 2. The projection IS closable — measured, not argued

The ruling asks for a projection keyed by *(emission owner, producer
result/construct, alternative, consumer/frame, recursive position)*, **set-equal
to checked `recursive_positions`**.

**Every one of those coordinate fields already exists on the interned
continuation key** (`emission_owner`, `producer_result_origin`,
`producer_construct_origin`, `producer_alternative`, `consumer_owner`,
`continuation_origin`, `recursive_position`). What does not exist is a *set* —
the planner interns **one unit per recursive position**.

⇒ So the projection is closable **iff** grouping interned units by the
coordinate-minus-position recovers the whole checked set. Measured on row 3:

**Checked:** `case[0] ctor:fixture::PX8JSiblingTree::Node → recursive_positions
[0, 1]`.

**Interned units, grouped:**

| emission owner | result / construct | alt | consumer | continuation | recursive positions |
|---|---|---|---|---|---|
| `Predeclared(0)` | `36` / `36` | 0 | `0` | `5` | **{0, 1}** |
| `Specialization(1)` | `25` / `25` | 0 | `0` | `5` | **{0, 1}** |
| `Specialization(0)` | `34` / `34` | 0 | `0` | `5` | **{0, 1}** |

**Three groups, each set-equal to the checked `[0, 1]`, each unique by source
position.** No group is short, and no position appears twice within a group.

⇒ **No new planner population is needed.** The projection is a *grouping* of
facts the planner already interns, which is what makes it readable-only by
lowering without reconstructing membership from lowered shape.

**Corroborating the defect from the same rows:** every unit reports
`ordinary_parameters = 1` with `captures = 0`, so `field_count = 2` and the
envelope emits **one** ordinary parameter while **two** positions are recursive.
Exactly one recursive field survives — the closure.

## 3. Bounds on this grounding

- **One row.** Row 3 is `D2b`'s entire population, so this is closure for the
  population — but it is not a claim about producers with three or more
  recursive positions, or with captures, neither of which occurs here.
- **Grouping is not yet validated in code.** That the grouping *recovers* the
  set is measured; the ruling additionally requires it be *enforced* set-equal,
  unique, and resolved to interned worker facts. That enforcement is `D2b`
  implementation, not this checkpoint.
- **Nothing about the four fail-closed caller conditions is measured here** —
  captured sibling, generated-context suffix, missing exact sibling unit/callee,
  recursive-set disagreement. Row 3 is stated by the ruling to be the
  zero-capture raw-route case; the measurement above shows `captures = 0`,
  consistent with that, and does not establish the other three are absent.

## 4. What remains, in the ruling's own order

1. planner-owned closed projection, with set-equality / uniqueness / worker-fact
   resolution **enforced**;
2. `ordinary_envelope` omitting **every** recursive position;
3. reconciliation of descriptor `ordinary_parameters`, slots, offsets, caller
   inputs and callee loads to the runtime-only envelope;
4. compiler-only binders — sealed case-binder plan, nonselected built at callee
   as `StaticWorkerBinding`, zero ABI slots, never a `LoweringOperand`;
5. caller-side reconciliation before `call_declared_unit_target`, with the four
   fail-closed conditions;
6. the control matrix;
7. the carried `control.rs` rider — make `D2a`'s `arrivals > 0` visibly prior to
   or inseparable from the prospective `forwards == arrivals`.

**Item 7 is due with the first candidate that touches `control.rs`**, which is
whatever lands item 6. This checkpoint touches no `crates/` path, so the rider
correctly does not apply to it.

## 5. Scope

No repair, no control, no `D2c`/`R3` traversal, no closure-transferability
change, no new `Lowered`/ABI/capture lane, no `StaticWorkerBinding`
successor-wall work, no `D3`, no `R4`, no tracker `status:` change, no
retirement. The probes were temporary and are removed —
`planning/static_transition.rs` sha256 `609e5bec…869f` matches base.
