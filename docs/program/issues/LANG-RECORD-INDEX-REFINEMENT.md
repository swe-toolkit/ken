---
id: LANG-RECORD-INDEX-REFINEMENT
title: "D2b predecessor, RESOLVED TO A KERNEL/TCB BINDER-HYGIENE FIX: eq_at_inductive (obs.rs:228) must weaken the accumulated nested-conjunction codomain past each newly bound proof (Term::sigma(conjunct, weaken(&acc, 1))), the same de Bruijn rule eq_at_sigma already uses. The elaborator transport hypothesis is FALSIFIED by measurement and reverts. OPERATOR-APPROVED 2026-09-01; released to the language ring as a kernel candidate under kernel QA + Architect TCB review."
status: active
owner: kernel
size: S
gate: operator
tier: T1
depends_on: []
blocks: [V3-FO-EMBEDDING-ADEQUACY]
github: null
origin: "Steward, 2026-09-01. Hard-stop chain on the D2b derivation inversion (V3-FO-EMBEDDING-ADEQUACY). Originally framed as an elaborator-only predecessor (the Architect's evt_68t4wwrs274nh transport-rewrite mechanism). That hypothesis was FALSIFIED by the language ring's four-probe measurement (evt_7fqdcjeh7dg63/evt_6t0tt37e7tes4): at base 4cbbdffb3 where the transport rewrite is absent, the single kernel eq_at_inductive weakening greens AC-1/AC-3/bare-control by itself — so the transport rewrite is causally irrelevant and reverts. The Architect confirmed the layer-(c) kernel/TCB disposition on all three points (evt_3f61wtca219hw). The id prefix LANG- is legacy from the elaborator-hypothesis phase; the work is now kernel-owned and operator-gated. Id retained for the V3-FO-EMBEDDING-ADEQUACY dependency edge and thread continuity."
---

> # OPERATOR-APPROVED 2026-09-01 (Pat, at the 13:00 UTC return). The kernel/TCB
> # completeness repair is authorized. The candidate is now RELEASED to the
> # language ring (it holds the proven material and reverts its own transport
> # rewrite); the one-line obs.rs:308 weakening + AC fixtures + no-overaccept
> # control + mutation proof land as ONE kernel candidate under kernel QA + the
> # Architect's required TCB review on the exact SHA. Steward routes (M1-M4),
> # lieutenant executes (M5-M9). Only after it lands does the Steward EXPLICITLY
> # re-release D2b (V3-FO-EMBEDDING-ADEQUACY); held evidence 70a291a96 folds onto
> # the landed fix.
> #
> # Confirmed disposition: Architect evt_3f61wtca219hw. Four-probe evidence:
> # language-implementer evt_7fqdcjeh7dg63, language-leader evt_6t0tt37e7tes4.

## Operator authorization request (the decision)

**What is asked:** approve a one-line kernel/TCB completeness repair to
`eq_at_inductive` in `crates/ken-kernel/src/obs.rs`, so the FO embedding-adequacy
theorem can dependently eliminate a derivation over a constructor-headed record
index. This grows nothing in the trusted base's surface; it corrects a
mal-scoped term the kernel was already constructing. **Nothing lands before your
approval.**

### Exact locus and diff

`obs.rs:228 fn eq_at_inductive` builds the per-constructor equality proposition
as a right-nested Sigma (conjunction) over the field equalities, accumulated in
reverse at `obs.rs:308`:

```rust
// obs.rs:308, inside the `for j in (0..n).rev()` reverse fold
-        acc = Term::sigma(conjunct, acc);
+        acc = Term::sigma(conjunct, weaken(&acc, 1));
```

That is the entire kernel edit.

### Why it is correct (Architect-confirmed, evt_3f61wtca219hw)

Each `conjunct` is constructed in the caller's `ctx`. Wrapping the accumulated
suffix `acc` as a `Sigma` codomain extends that context by one binder — the newly
bound proof of the current conjunct. Every free caller-context index in the
suffix must therefore move by one. `weaken` does exactly that (`shift(.., 1, 0)`:
it moves free indices while incrementing the cutoff beneath the suffix's existing
`Sigma` binders). Repeating it at each fold produces the required right-nested
telescope; `strip_trailing_top` (obs.rs:316) removes only the closed unit and
does not undo the lifts. **This is the identical binder rule `eq_at_sigma`
already implements for its second conjunct** — the single-argument and
non-nested cases never exercised the missing lift, which is why the bug survived.

### Why it is not a soundness relaxation (Architect-confirmed)

Constructor identity, arity checks (obs.rs:248,261), field types, endpoint terms,
the dependent `Cast` transport (obs.rs:296), `convert_type`, and every field `Eq`
are **byte-identical**. The edit supplies no equality evidence and introduces no
shortcut. It only preserves which outer variables the already-constructed
proposition denotes, instead of letting the next proof binder capture them. The
old term was mal-scoped; the repaired term states the intended conjunction. It is
a **completeness** repair (the kernel wrongly rejected a well-typed nested
elimination), not a widening that admits unequal terms.

### Evidence (four probes, language ring)

- **AXIS 1** — held branch `4f206f1bf` (generic transport rewrite present) + only
  the weakening: AC-1 (record-index constant-motive match), AC-3 (D2b inversion
  probe), and the bare-index control all GREEN.
- **AXIS 2 (dispositive)** — base `4cbbdffb3` (transport rewrite ABSENT,
  `elab.rs` reverted) + only the weakening: AC-1, AC-3, bare control all GREEN.
  ⇒ the weakening fixes the record-index match ALONE; the transport rewrite never
  fires and is causally irrelevant.
- **BASELINE** — `4f206f1bf` WITHOUT the weakening: AC-1/AC-3 byte-identically RED.
- **DIRECT KERNEL CONTROL** — new test
  `crates/ken-kernel/tests/eq_at_inductive_multifield_binder.rs`: a two-field
  single-ctor `MkPair2 : Nat -> Unit -> Pair2` with OPEN endpoints
  under one unrelated trailing binder, and a one-field `MkOne : Nat -> One`. WITH
  the weakening: two-field reflexive-eq-under-trailing-binder infers the nested
  proof (OK); one-field stays well-typed (OK). WITH the old buggy `acc` restored:
  two-field FAILS exactly as predicted; one-field OK. The one-field row staying
  green both ways confirms the control reaches the nested-codomain case, not
  generic reflexivity.

### Production reach (not fixture-only)

The real `FoKripke.ken` embedding-adequacy elimination — the second of the two
`spec 23 §4.4` theorems route FO needs before it may return `proved` — requires
exactly this nested-codomain elimination over `FokDerivation (FokMkSequent gamma
delta)`. This is reachable from real Ken source, not a fixture-only capability
(Architect point 3).

## Candidate gate on approval (Architect's required gate)

If the operator approves, the recut lands as a **kernel candidate**, owner
kernel, reviewed by **kernel QA + the Architect's required TCB review** on the
exact SHA. No extra pre-authorization Adversary hop is needed to establish the
disposition; the standing Adversary code-merge hunt attacks it independently
(no new workflow edge).

- **No-overaccept control (must be in the candidate gate).** Under the same
  trailing binder, use the same two-field constructor with a genuinely unequal
  later field and require its proposed whole-record equality witness to remain
  kernel-REJECTED. Retain existing different-constructor / arity behavior.
- **Mutation-prove the seam.** Restore `Term::sigma(conjunct, acc)`: the
  two-field reflexive control and real AC-1/AC-3 must RED, while the one-field and
  unequal-field controls must NOT turn into false positives; then restore bytes
  exactly.
- **Retain** the record-index acceptance fixture as a reaching regression for the
  real repair. **Do not** retain the unused elaborator production mechanism.

## Transport reversion (Architect point 1)

The generic `sym`/`cong`/`subst` elaborator transport production committed at
`4f206f1bf` is correct-but-causally-irrelevant groundwork and does NOT belong in
this fix. Revert it. Keep only the record-index acceptance fixture.

## What stays byte-unchanged either way

`FokDerivation`, `fok_derives`/`fok_classically_valid`, `FoKripke.ken`, `/spec`,
the prover, the public Ken relation, and the FO `proved` verdict. The only TCB
touch is the one-line `obs.rs` weakening above.

## On approval — execution order

1. Revert the `4f206f1bf` transport production (retain the record-index fixture).
2. Land the `obs.rs:308` weakening + the AC fixtures + the no-overaccept control
   + the mutation proof, as one kernel candidate. Kernel QA + Architect TCB
   review on the exact SHA; Steward routes (M1-M4), lieutenant executes (M5-M9).
3. Only after that lands does the Steward EXPLICITLY re-release D2b. The held D2b
   evidence `70a291a96` (the strengthened ledger/inversion spine) is reusable
   material, not a candidate; the language ring rebases/folds it onto the landed
   kernel fix and continues the EXACT unchanged adequacy theorem.

## On denial

D2b (V3-FO-EMBEDDING-ADEQUACY) stays blocked; FO cannot return `proved` without
embedding-adequacy. Any alternative route (proving the elimination without the
kernel completeness fix) needs fresh Architect framing — the measurement shows no
elaborator-only path exists.

## Capability tier: T1

Kernel soundness reasoning: the fix is authorized on the argument that the
weakening is a scoping-completeness repair that does not over-accept, not a
mechanical diff.
