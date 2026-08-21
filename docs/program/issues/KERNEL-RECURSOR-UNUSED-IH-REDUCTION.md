---
id: KERNEL-RECURSOR-UNUSED-IH-REDUCTION
title: "The recursor's iota-rule forces an induction hypothesis for every recursive argument eagerly, so a non-recursive match on a recursive inductive does not reduce at an abstract recursive field -- repair iota_reduct to skip an IH whose method binder does not occur, the reduction-time enabler V3-FO-CHECKER-SOUNDNESS D3 is blocked on"
status: merged
owner: kernel
size: M
gate: operator
depends_on: []
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-21, on the Architect's D3 successor design ruling evt_1r4fw67tzqszt. V3-FO-CHECKER-SOUNDNESS D3 hard-stopped on a measured capability gap (language-implementer evt_7mjd244k6wmf5, thr_13q5): the match elaborator lowers every surface match to the recursor, and iota_reduct builds an IH for every recursive argument eagerly, so a non-recursive match over FokCert does not reduce at an abstract child. The Architect ruled the fix is a reduction repair in iota_reduct (shape B), NOT a new case eliminator (shape A, fallback only), owned by the kernel ring, elaborator untouched. TCB-touching (modifies trusted reduction, adds no trusted surface): the Architect ruled operator authorization warranted and release held for the operator -- so gate: operator, framed but not released. Non-candidate evidence at 99f7a5f4b on wp/V3-FO-CHECKER-SOUNDNESS-D3. Steward-filed per COORDINATION section 2. Estimated capability tier: T1 (soundness-bearing reduction-engine change)."
---

## What is broken

Ken's surface exposes only `match` for data elimination, and
`spec/30-surface/34-data-match.md` makes every surface `match` elaborate to the
generated recursor `elim_D`. The match elaborator
(`crates/ken-elaborator/src/elab.rs`, the IH-slot emission block)
**unconditionally** emits the recursive induction-hypothesis (IH) slots
`method_type` requires, **including dead slots** whose method binder the branch
body never uses.

`iota_reduct` (`crates/ken-kernel/src/inductive.rs:2185`) then builds those IHs
**eagerly**, before the method is applied: it computes `recursive_shapes(...)`
and, per recursive argument, calls `structured_lift_term(...)`
(`inductive.rs:2237-2260`), returning `method ā ++ ihs`. For `FokCert`'s
`children : List FokCert` -- a nested / W-style recursive field -- that IH is a
**stuck neutral at an abstract `children`**.

Consequence, measured exactly by the D3 probe
(`fok_check_node_abstract_children_probe`): a non-recursive match over a
recursive inductive does **not** reduce at an abstract recursive field, because
the recursor's iota-rule forces the dead child-IH into existence even though the
match body never binds it. The probe's expected view has already iota-reduced
the certificate/conclusion structure; the sole residual blockage is the
generated dead child-IH. The probe fails at `Refl`:

```text
TypeMismatch { reason: "Refl: the two sides of the goal are not convertible" }
```

This is why `V3-FO-CHECKER-SOUNDNESS` D3 (the propositional `checker_soundness`
proof) cannot close: `fok_check_tree`'s structural tree guard is not
conversion-invertible at abstract nodes.

## The mechanism -- Architect ruling `evt_1r4fw67tzqszt`

**The gap is a recursor reduction deficiency, not a missing eliminator.** The
kernel has exactly four eliminators -- `Elim` (the recursor), `J`, `QuotElim`,
`Absurd` (`term.rs`); there is no case eliminator. A `casesOn` *derived* from the
current recursor (Lean-style `casesOn := rec` with IH-dropping methods) does
**not** help -- it still routes through `iota_reduct`, which still builds the
stuck IH before the method sees it. A pure-elaborator re-expression cannot close
this; **the reducer itself must change.** Standard kernels do not get stuck here
because their recursor reduction drops unused method arguments rather than
forcing them.

**Shape B (ruled) -- repair the recursor's iota-rule.** `iota_reduct` builds an
IH **only** for a recursive argument whose corresponding method binder actually
occurs in the method body; a dead IH binder gets no `structured_lift_term` call.
This is observationally identical to today's rule by beta -- an unused binder
discards its argument -- so it changes **nothing** the eliminator computes; it
only makes reduction fire in strictly more cases. It subsumes the whole
"non-recursive match on a recursive inductive" class in one place, adds no new
construct, and removes this class from the `structured_lift_term` divergence
surface entirely. The exact reduction-engine mechanism inside B --
free-variable-guarded skip vs. lazy / thunked IH construction -- is the kernel
ring's technical call; the Architect ruled the shape, not the mechanism.

**Shape A is the fallback only** (a first-class non-recursive case eliminator
`casesOn` with its own iota-rule `casesOn (c_k ā) m̄ ⇝ m_k ā` and typing rule,
plus elaborator lowering of IH-free matches to it). It **proliferates** a
parallel eliminator, new typing, new conformance, and a normative surface note,
to route around an unrepaired recursor. Take it **only** if the kernel ring
finds B infeasible; if so, flag the Steward and the Architect (it grows the TCB
and needs a Spec co-review -- see AC-4).

> ### THE ONE SOUNDNESS-CRITICAL DIRECTION. This is the whole risk of shape B.
>
> The "method binder does not occur" test must be **conservative in the safe
> direction: it must OVER-approximate use.** A false "unused" verdict on a
> binder that IS used would drop a live IH and build an IH-free recursor over a
> recursive argument -- which proves `False`. So **if in any doubt, build the
> IH.** Over-building is merely less reduction (a liveness cost);
> under-detecting use is **unsound**. This is the same occurrence-direction
> discipline as the `check_pos_arg` / `derive_recursive_shape` rulings, one
> layer down in the reducer.
>
> The entire soundness obligation of B is one sentence: **building an IH and
> discarding it via beta is identical to not building it, when the IH binder
> does not occur in the method body.**

## Deliverables

**`D1` -- the repair, in `crates/ken-kernel/src/inductive.rs`
(`iota_reduct` / the `Elim` iota-path, and `conv.rs` whnf as needed).** Make IH
construction per recursive argument conditional on the method binder occurring
in the method body, over-approximating use per the soundness-critical direction
above. No new eliminator, no typing-rule change, no elaborator change.

**`D2` -- the discriminating confirmation.** The AC-1 unblock probe plus the
AC-2 live-IH control, as targeted `ken-kernel` / `ken-elaborator` tests, and the
AC-3 conformance cases. `D1` and `D2` are one turn where practical: the edit and
the control that proves it did not over-fire belong together.

## Acceptance criteria

**`AC-1` -- the exact D3 unblock.** The implementer's
`fok_check_node_abstract_children_probe` -- `fok_check_tree`'s outer body at
`FokMkCert (FokMkSequent gamma delta) rule children` with abstract `children` --
is convertible to its IH-stripped view by `Refl` (the measured failing probe now
passes). Targeted kernel/elaborator test; never a `--workspace` run.

**`AC-2` -- the soundness guard, the discriminating negative.** A match whose
body **does** use its recursive IH (a genuine structural fold over the same
inductive) is **unchanged**: it still reduces through the recursor with the IH
intact and still type-checks. A fixture asserts a live-IH recursion's result is
untouched. **This is the control that proves the repair did not over-fire and
silently drop a live IH; without it the suite cannot tell the sound repair from
the unsound version.**

**`AC-3` -- conformance (§7).** One acceptance case (a dead-IH match reduces at
an abstract node) and one invariance case (a live-IH recursion behaves exactly as
before) in the conformance suite.

**`AC-4` -- spec.** Shape B needs **no** normative change:
`spec/30-surface/34-data-match.md` still says match elaborates to the recursor; B
only makes that recursor reduce more. **If the ring falls back to shape A**, that
needs a normative surface note (a new case eliminator) and a Spec co-review --
flag the Architect.

**`AC-5` -- no-regression** in CI (`COORDINATION §12`). Targeted local validation
only; the full-workspace and conformance runs are CI's.

## Banned scope

- **Adding a second eliminator / `casesOn`** as a kernel former. That is shape A,
  the fallback, and it is out of scope for this node unless the kernel ring first
  establishes B is infeasible and re-routes through the Steward + Architect.
- **Any change to match-lowering in `ken-elaborator`.** The Architect ruled the
  elaborator untouched under shape B.
- **Weakening the occurrence test to under-approximate use.** Under-detecting use
  is the unsound direction; see the soundness-critical block.
- **Changing any typing rule, `method_type`, or conformance behavior for the
  live-IH case.** B changes reduction only.

## Sequencing

**AUTHORIZED and released -- 2026-08-21 (operator, "approve decision 1").** This
modifies the trusted reduction engine (`iota_reduct`); the Architect ruled
operator authorization warranted (evt_1r4fw67tzqszt) and the Steward held release
for the operator's return. The operator authorized it on return; `status`
flipped to `active` and released to the kernel ring. `gate: operator` is retained
as the record of the requirement; it is satisfied. The soundness obligation is
the one stateable sentence above (over-approximate use; if in doubt, build the
IH).

**Owner: kernel ring.** Author = `kernel-implementer` (T1 -- soundness-bearing
reduction-engine work), independent soundness review = Architect (the author is
not the reviewer). `depends_on` is empty -- the gap exists on `origin/main`
`3686c827c` today. `blocks: [V3-FO-CHECKER-SOUNDNESS]`: D3 resumes on this
landing with no proof re-authoring (Architect + language-leader concur).

**Bookkeeping.** This is the terminal characterization of D3 hard-stop HS2
(tree-guard non-reduction), not a third hard stop -- no `§1a` research trigger.
It is the `§1b` "one shared predicate ⇒ one structural closure" outcome the
Architect named at HS2: *the checker must reduce at abstract structure, but Ken's
match-compilation over-generates a stuck eliminator dependency* -- this successor
closes that predicate structurally.
