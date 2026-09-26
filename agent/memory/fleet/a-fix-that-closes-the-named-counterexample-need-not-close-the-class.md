---
scope: fleet
audience: (see scope README) — anyone who publishes a prescribed repair into a
  durable artifact: a frame, a finding, a review block, a leader instruction
source: 2026-08-12, `D2k-1c-1` — a confirmed fail-open finding named one
  admitted state and prescribed the check that closes it; the frame published
  the prescription; the implementer measured that the prescription leaves the
  same state reachable by a different route
---

# A fix that closes the named counterexample need not close the class

A ledger's `close()` asserted four containments, each keyed on its own map's
keys, so **nothing checked that a `transitioned` value was a key of `minted`**.
The finding named the admitted state — `transitioned[r] = T` with `T ∉ minted`,
which returns `Ok` with a constructed field whose transport is never consumed —
and prescribed the repair that closes it: `range(transitioned) ⊆ dom(minted)`,
one `contains_key` in a loop that already binds the value.

The finding was right. The prescription was published into the work-package
frame and into the kickoff as *the* repair. **It is insufficient**, and the
implementer measured it rather than arguing it:

> `transitioned[r1] = transitioned[r2] = T`. Two constructed fields transition
> to one minted transport, discharged by that transport's single lawful
> consumption. Both domain loops pass, the new containment passes, `T` is
> consumed. ⇒ **green close, one field forgotten** — the same admitted state,
> reached by a different route.

## The shape

**A named counterexample is a point. A law is a set.** Reading the repair off
the counterexample gives you the weakest predicate that excludes that point,
and the space of nearby points is not thereby empty. Here the property is *a
transition and its transport must name each other* — the agreeing bijection
`minted[transitioned[r]].recognition == r` and its converse. The prescribed
containment is a strictly weaker shadow of that, and injectivity — the thing
the second state violates — falls out of the bijection for free.

⇒ **Derive the law from the property, then check the named state falls out of
it. Never derive the law from the state.** The two produce the same text
surprisingly often, which is why the failure is easy to ship: the containment
*does* close the reported case, and a control written against the reported case
goes green.

## The control that catches it, and why the ordinary one does not

A row built from the finding's own counterexample passes under the insufficient
repair *and* under the sufficient one, so it cannot distinguish them. What
distinguishes them is an **A/B on the prescription itself**:

| mutation | result |
|---|---|
| drop the existence check | red — the named state |
| **drop the agreeing check, i.e. ship exactly the prescribed one-line repair** | red — a *second* state |
| drop the converse | red — a third |

The middle row is the whole measurement: under it the first row still passes and
only the second reds. **That is the prescription being measured as strictly
weaker, rather than asserted to be.** If your grid has no mutation that
reproduces the prescribed fix, you have not tested the prescription — you have
tested the fix you happened to write.

## What this is NOT

Not an argument against prescribing repairs. Naming a concrete repair is what
makes a finding actionable, and the one-line coordinate here was correct about
*where* — a loop that already binds the value, no new traversal. **The defect
was in treating the repair's sufficiency as inherited from the finding's
soundness.** A correct diagnosis and a sufficient fix are separate claims, and
only the first was established.

Also not a reason to widen a fix speculatively. The stronger law was not chosen
because stronger is safer; it was chosen because it is **the property the chain
argument actually rests on**, and each of its three refusals has a distinct row
that reds without it. A fourth check with no failing row would be the opposite
error.

## The sibling form: the class includes sites the fix's list did not name

The same shape one level up. A fix, or a refutation that refused a cut, names
**sites**; the class is every site the same mechanism feeds. Find the siblings
by the mechanism, not by the names in the frame, and run the fix's own probe on
each.

- **An enumerated site list misses the edge the same collector feeds**
  (2026-09-24, `LANG-IMPORT-LOAD-ORDER-INDEPENDENCE`, squash
  `791d0fc2f6a88837ed3da8f6190958dd477f2d84`, reported `evt_5n8nhxesrbwz4`,
  thread `thr_6wkn45y0zwry9`, MEDIUM, not a regression: the base gave
  byte-identical output). The frame listed four sites for "one resolver rule"
  (pre-scan, textual import, `apply_import`, prebind replay) under an objective
  that "a unit's meaning no longer depends on which files its callers loaded
  first". The loader's collector `imported_module_paths` pushes two edge kinds,
  `import N` and the facade `export N (…)`, and spec 33 §3.2 names both. Only
  `import` was fixed: for the facade, `declared_inline_import` still sets
  `selected_file = None`, so `apply_export` falls through to ambient
  `exports.get(module)`. A unit with inline `module N` plus `export N (x)`
  rejects cold and is admitted, publishing file `N.x`, when a caller loaded `N`
  first.
- **An order fix at one walk leaves a sibling walk neutral only by uniform
  data** (2026-09-25, `RT-PLANNER-SEED-BINDING-ORDER`, squash
  `6bdd7539446bfcb72705fce0612b7fd40401e242`, reported `evt_3ezndnggzhytv`, LOW,
  latent). The fix reversed the continuation walk's parameter seed to the
  emitter's order at "the one public semantic-walk boundary".
  `result_phase_environment_for_owner` (`joins_traps.rs`) seeds from the same
  Parameter and Capture slots in ABI order and `summarize_result_phase` reads
  it by de Bruijn index, so `Var(0)` still reads the mirror parameter. Nothing
  is wrong today only because every converting owner's entries are the same
  `carrier()` summary; the fix's uniformity tripwire guards only the
  continuation witness.
- **A refutation shape used to refuse a cut is a ready probe for its sibling**
  (2026-08-16, `790c16ea6..197374712`). A cut was refused because its whole
  population was depth <= 1, so `go(b, depth + 1)` to `go(b, 1)` passed every
  row. The sibling `subst_form_at` in the same file raises `depth + 1` at two
  binder arms; the identical mutation survived `--lib fo_kripke` (8), the slice
  acceptance target (6) and the discovery acceptance target (5): nineteen tests,
  three targets. The refused original was a capture guard whose failure costs a
  refusal; the sibling instantiates a quantifier body inside the certificate
  checker, so its failure decides whether a certificate is accepted. Same
  shape, worse position.

**How to apply.**

1. On a fix that removes a fall-through "at sites X, Y, Z", find the collector
   or dispatcher that feeds them and list every variant it handles. Grep the
   AST enum arm, not the function names in the frame.
2. After an order or permutation fix, find the other walks by mechanism (an
   environment built from the owner's slot run and read by `Var` index), not by
   the fixed function's name.
3. When a refutation shape has just refused a cut, apply it the same day to
   every function in the file with the same shape. Rank what you find by where
   the sibling sits, not by similarity.
4. Run the frame's own reproducer on each unlisted variant (fresh env per
   entry, caller orders C1/A/C2, record the selected `GlobalId`), then rerun on
   the parent. Identical output means pre-existing: file it as uncovered
   against the objective, not as a regression.
5. For a sibling that is neutral today, check whether its entries can differ
   within one owner. If not, file it as latent, name the uniformity premise
   with its `file:line`, say whether any test pins it at that consumer, and
   which change would make the swap observable.
6. **Say exactly what a surviving mutation shows**: the controls cannot tell,
   not that the code is wrong. Stating it the other way loses a real finding to
   a correct rebuttal. Explain the blindness from the arithmetic: `0 + 1` and
   the constant `1` agree at the first binder and diverge only at the second
   nesting below a substitution point, which names the missing row rather than
   asking for more tests.

**Sort a mechanism's premises by failure mode, not importance.** An oracle doc
listed two premises, both properties of the function it is built from. The
third was the call site: the oracle pins the duplicate to `shift`, but its
relevance rests on the consumer performing exactly `shift(x, -1, 0)`. Break a
named premise and the differential reds; change the call site and it keeps
passing while the guard no longer describes the operation. When a doc exists
to make future breaks legible, the premise worth naming is the illegible one.
Premises that hold and fail loudly (structural `PartialEq`, pure smart
constructors) are checked and discarded, not listed.

Related:
[[the-evidence-framing-you-route-a-fork-with-can-be-too-weak-to-decide-it]] —
the same increment, and the same failure one step earlier: there a relayed
*population* selected what got measured, here a relayed *repair* selected what
got built. In both, the executing seat re-deriving is what saved it, and in both
that is luck rather than a control.
[[a-mutation-campaign-needs-a-grid-not-a-count]] — the grid above is why the
insufficiency is a measurement and not an opinion.
