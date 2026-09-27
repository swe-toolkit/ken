---
name: certify-a-merge-block-unification-refactor-by-enumerating-the-environment-divergence-axes-and-proving-each-vacuous-in-tree-or-the-intended-fix
description: A lowering refactor that unifies N separate lowerings of one body, fed different environments, into one downstream merge block is not automatically faithful - the absorbed route now runs under the survivor's environment. Enumerate the environment-divergence axes and prove each either vacuous in tree (exhaustive construction-site census, struct-literal and computed forms) or the intended fix (a reaching negative control recreates the exact pre-repair error).
metadata:
  type: feedback
---

# Certify a merge-block-unification refactor by enumerating the environment-divergence axes

**Measured 2026-08-29 on `RT-CHECKED-SUCCESSOR-EMIT-REACHABILITY` exact
`e89206d88638ef003e5e9dcfe610170e3968f886` (range `7a78929c1..e89206d88`, 4
paths, +311/-79, single clean unrebased commit -- parent == range base ==
merge-base with origin/main).**
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs` (+ `mod.rs`,
`object_linker_packaging.rs`, new `core/tests/constructors.rs`). Reported CLEAN,
no adversarial objection (`evt_2zkhkdagatzwb`, thread `thr_5ndst6yn7fb3t`).
Architect (`evt_4891egcadyrnj`) + QA (`evt_3y0z98cc19w23`) approved this exact
SHA; Decision `dec_79k1ahq4g12sp` resolved.

## The shape

A native-codegen eliminator lowering had TWO separate lowerings of the SAME
source Ret body: the ordinary constructor arm inside the case loop, and the
checked whole-answer arm after the loop. The base fed each a DIFFERENT
environment (the ordinary arm's case-loop env with IHs; the checked arm's flat
`[Carried(scrutinee)] ++ eliminator.env`). The candidate creates one downstream
`return_body` merge block (a Cranelift block with an I64 param), makes BOTH old
predecessors `jump(return_body, <their word>)`, and lowers the body ONCE after
`switch_to_block(return_body)`, consuming the block param.

**Why this is not automatically faithful.** Unifying two lowerings onto ONE code
path means the path NOT chosen as the survivor now runs under the OTHER's
environment. So the soundness question is per-axis: for every way the two base
environments could differ, is the survivor's environment correct for the route
that used to have the other environment?

## The certification: enumerate the divergence axes, discharge each

The case-loop builds `case_env = [IHs_reversed] ++ [children] ++ [frame_env]`,
where `frame_env = eliminator.env` possibly with `Carried(scrutinee)` inserted at
`retained_scrutinee_index`. The base checked arm built flat
`[Carried(scrutinee)] ++ eliminator.env`. Two divergence axes:

1. **`retained_scrutinee_index` -- VACUOUS IN TREE.** The case-loop path inserts
   an extra binding at `retained_scrutinee_index`, shifting every subsequent de
   Bruijn binding; the base checked arm did not. This would be a real miscompile
   IF the index were ever `Some` for the checked eliminator. It is not:
   enumerate EVERY construction site -- `git grep -nhoP
   "retained_scrutinee_index:\s*\K[A-Za-z_][A-Za-z0-9_:]*" <sha> --
   crates/ken-runtime/src/ | sort | uniq -c` -- and every occurrence is `None`
   except the two `Option<usize>` type declarations and one `Some(` inside a
   comment. Also check the computed form (`retained_scrutinee_index\s*=`): none.
   The checked eliminator at `core.rs:3900` hard-codes `None`. So `frame_env ==
   env` with no insertion, and for the common no-recursive-position Ret case the
   shared `case_env` reduces to `[Carried(word)] ++ env` -- byte-for-behavior
   identical to the base checked arm (the checked predecessor feeds
   `scrutinee.word` into the block param, so the leading binder's word value
   matches). This is the "measure every construction site, not a grep that
   happened to hit" discipline (kin of the settlement comment's own method and
   of the Adversary's baseline-gated census lesson) -- an apparent divergence
   proved vacuous by exhaustive enumeration.

2. **IH prepending -- the INTENDED FIX.** For a Ret case WITH a recursive
   position, the ordinary arm prepends an induction hypothesis into the env; the
   base checked arm's flat env did NOT, so a Ret body referencing the IH-bound
   variable (`Var(n)`) was unbound / mis-resolved on the checked route -- a real
   pre-repair bug. Unification routes both through the ordinary (IH-bearing) path,
   which FIXES it. This half is not a no-op; it is a soundness improvement, and it
   is proved by the negative control, not asserted.

## The negative control must recreate the EXACT pre-repair error

`CHECKED_SUCCESSOR_UNCONDITIONAL_SEPARATE_LOWERING` (a `#[cfg(test)]`
thread-local cell; production hard-codes `lowered_separately = false` under
`#[cfg(not(test))]`) restores the pre-repair SEPARATE lowering at the exact
seam. The test
`unconditionally_separate_checked_successor_lowering_recreates_the_regression`
arms it and asserts the fixture reproduces the EXACT `Unsupported(Var, "no
runtime binding for index 2")`, then clears it and asserts `Ok`. The
discriminator is the fixture's `Var(2)` Ret body under `recursive_positions:
vec![0]`: only the IH-bearing shared path binds index 2. This is reaching +
non-degenerate + exact-identity -- green-vs-green does not confirm a fix. A
companion `architect_probe_*` test asserts the repaired path compiles AND emits
no false `CarriedAnswerRouteEmitted` for a None-frame fixture.

## Supporting checks (do all)

- **de Bruijn order.** `[IHs_reversed] ++ [children] ++ [frame_env]` must match
  the base survivor's order; for the no-recursive Ret case it collapses to
  `[Carried(word)] ++ env`, identical to the base flat env. Order is load-bearing
  (positional indices); a reorder silently rebinds every body variable.
- **cfg matrix (the -D warnings class only the consumer crate sees).** A body/
  binding computed under `cfg(any(test, feature="..."))` and used only under a
  narrower cfg is an unused-variable red in the wider config. Here `body` and
  `body_origin` each have a syntactic use in every config that binds them (test:
  the `lowered_separately` block; test-support: `body_origin` -> observer call);
  `_return_index`/`_return_case` are underscore-guarded. Check every build combo,
  not just `-p <crate>`.
- **route-control instrumentation is additive + guarded.** The `mod.rs`
  `initial-checked-to-direct` arm fires only under the
  `KEN_RT_ITREE_D1_ROUTE_CONTROL` env var AND `checked_frame_id == Some(7)`
  AND the edge/route guards; otherwise it falls to the appended `=> emitted`
  passthrough. Unknown modes still `panic!`. Production (env unset) emission
  is unaffected.
- **linked-outcome execution proof.** The `object_linker_packaging` test was
  strengthened from a weak `assert_ne!(success != trapped)` to an exact
  `assert_eq!((success_status, trapped_status), (Some(0), Some(1)))`. THIS is the
  end-to-end proof the merged `return_body` produces a runnable artifact
  exiting exactly 0 on the checked success route -- do not hand-diff the
  Cranelift SSA merge; name the linked-outcome + native-parity greens as the
  execution certificate, disclose you did not run the minutes-long suite, and
  name the two gate seats.

## Hunt rule

For any "merge N separate lowerings of one body that had different environments
into a single downstream block":
1. List the env-divergence axes between the merged predecessors (extra inserted
   bindings, prepended IHs, env order, the leading carried word's provenance).
2. For each axis, prove VACUOUS-IN-TREE (exhaustively enumerate the field's
   construction sites -- struct-literal AND computed-assignment forms -- and show
   the two envs coincide) OR the INTENDED FIX (base was buggy; a reaching
   negative control recreates the exact pre-repair error and clearing restores
   green).
3. If ANY axis is neither vacuous nor a control-backed fix, that is the finding:
   the survivor's environment is wrong for the absorbed route.

Kin of
[[verify-an-out-parameter-or-sentinel-to-sum-type-trampoline-refactor-by-the-continuation-site-invariant-not-by-reading-each-mechanically-wrapped-arm]]
(same family -- certify a Runtime lowering refactor by a structural invariant
over the exact diff, not arm-by-arm; there the invariant is WHERE continuation
happens, here it is the ENVIRONMENT SHAPE across merged predecessors) and of
[[certify-a-negative-control-topology-refactor-by-cfg-gating-exact-is-identity-compiler-enforced-roster-and-per-conjunct-sibling-preservation-not-by-rerunning-the-suite]]
(the cfg-gating + exact-is-identity + reaching-control discipline; here
exact-is-identity is the vacuous-in-tree axis and the reaching control recreates
the exact pre-repair error).
