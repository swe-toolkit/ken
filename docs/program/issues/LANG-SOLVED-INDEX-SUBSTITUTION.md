---
id: LANG-SOLVED-INDEX-SUBSTITUTION
title: "A solved match index is moved per variable by cast aliases whose proofs stay pinned to the raw index premise, so a nested match cannot abstract the index, a match on an alias takes the equation convoy at a stuck eliminator, and a written sibling type disagrees with its unmoved field. Eliminate a solved leaf with a variable side by substitution and one J transport of the reverted arm"
status: active
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-LET-ANNOTATION-MOVED-FIELD-COORDINATE, KERNEL-OMEGA-STRUCTURAL-POSITIONS, SPEC-INDEXED-ARM-SCRUTINEE-COORDINATE]
blocks: []
github: null
origin: "Architect D0 for LANG-THIRD-LEVEL-MATCH-MOVED-FIELD (evt_3nbk4bv8c7kpe), which also covers the Adversary finding on 8e9e2c2c7 (evt_563avf72ddez3, ruled real evt_1ndrpz0mg3fan). The ruled change outgrows that M node; restructuring returns to the operator to size (2026-09-21); operator authorized 2026-10-17. Steward-filed per COORDINATION section 2."
---

# A solved index is eliminated, not aliased

## Objective

A match arm whose index premise solves a variable is elaborated against
real binders at the solved coordinate, so later matches, written types
and the goal all see one coordinate.

## Settled inputs (Architect `evt_3nbk4bv8c7kpe`, on `8e9e2c2c7`)

- **The alias.** In the `DualCons` arm, `rest : DualEnv a k` is moved to
  the scrutinee coordinate `m` by
  `var_refinements[rest] = cast (DualEnv a k) (DualEnv a m) (J … e) rest`,
  installed by `install_index_refinements` (elab.rs:10031) through
  `try_reindex_cast` (:9595). Its proof uses the premise
  `e : Eq Nat (Suc k) (Suc m)`.
- **Three producers, one predicate.**
  - A (sec, secpp): `build_checked_dependent_motive` (:8075) rejects the
    inner `match j` motive. Generalizing `m` rewrites the alias's types
    but not its proof, which still goes through `e`.
  - B (secrest): the inner `match rest` scrutinizes the alias, so the
    guard at :4684-4708 sees a refined scrutinee and selects the equation
    convoy. Its motive is a stuck `elim_Nat` at `m`, and
    `finish_dependent_elim` (:7284) fails with NotAFunction.
  - q2/q3: producer (a) (`elab_type_named_variable`, :1148) resolves
    written `rest` through the alias, while the sibling
    `h : Equal Nat (dual_len a k rest) k` keeps its raw type. The result
    is K2 at `h`. Reverting (a) is not a repair: r1, r1pp and r2 regress.
- **Substituted arms check.** With the fields as real binders at the
  scrutinee coordinate, sub_sec, sub_secrest, s3 and s2 are Ok.
- **Population.** 59 of 59 leaf installs on 14 rows have a variable side
  (the constructor's field variable or an outer scrutinee variable).
- **Enabler.** The kernel J (check.rs:893) takes a Type-valued motive;
  the aliases on main already carry one. The arm-closing obligation for
  leaf `e' : Eq Nat k m` is
  `λ k x rest e. J (λ i _. Π (rest : DualEnv a i). G_i) (λ rest. B) (sym e') rest`.
  It is read, not built.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

For a leaf with a variable side, substitute the solved variable through
its dependents (later fields, hidden result variables, the goal),
elaborate the arm against real binders, and close the method with one J
transport of the reverted arm. This replaces, on that path:
`install_index_refinements` capability 1 and
`install_fields_from_goal_transport` (:8469), on field-variable leaves only.
The build route is R1: the generalized-goal telescope engine with a
dependency-closure selector and one J per leaf (Architect
`evt_7ppfqyymvx6qn`).
- **Eliminated (Architect `evt_6gq7rm62sy7qf`):** only a leaf whose field
  side is a constructor-field variable (`target = Var(v)`, `v < n`). When
  both sides are fields, eliminate the later-bound one.
- **Retained on today's path:** a leaf whose only variable is the outer
  scrutinee index, which belongs to the motive and convoy path. So do
  `install_hidden_result_variable_refinements` and
  `install_plain_declared_index_aliases`.
- A method that mixes variable and non-variable leaf plans is a stop to
  report.

**Composition (amendments 3 and 4, Architect `evt_20k6hnw9evhy5`,
`evt_44mfvbvp38fjy`).**
- In `compute_context_convoy`, seed `substituted` with the bare-variable
  scrutinee indices. A Field-origin binder is transported iff its type
  mentions a substituted variable; a UserLocal binder keeps today's
  dependency seeds; every other origin is skipped. The rule's code is in
  the ruling.
- Premise references for the eliminated leaf resolve to `Refl` at the use
  coordinate, never to a Δ binder. Binders that R1 restores count as
  Field. The user-name redirect follows any re-transport.

**Recut (Architect `evt_2hy029razcfgq`).** The method goal's convoy Cast
carries the raw premise `e`, which structural conversion compares
syntactically (inventory entry 3). The closure is the kernel node
KERNEL-OMEGA-STRUCTURAL-POSITIONS, which skips Ω-by-formation positions.
With its probe, WIP `fff3a5a8c` measured sec and secrest green. Resume
from that WIP once the node lands. secpp and secrestpp are in the G2
class: inside an indexed arm, the names of a variable scrutinee and its
solved index must denote the constructor coordinate (Spec front desk
`evt_7qmn27nc7th3m`; SPEC-INDEXED-ARM-SCRUTINEE-COORDINATE). That
scrutinee-side substitution is built in this WP, after the spec lands,
because without it the elimination regresses secrestpp, which is green
on main.

Retained: leaf projection, injectivity and conflict leaves; the convoy
machinery; the kernel; the cast-alias path for a leaf with no variable
side. Producer (a) and `collapse_own_leaf_field_transports` become
no-ops on eliminated leaves; keep them until the acceptance rows show
no change without them.

## Acceptance

- **AC-0 (measure, before building).** Census the leaf installs across
  the LET-ANNOTATION AC-2 suites, split into field-variable side,
  scrutinee-variable side and neither. If "neither" is nonzero and has
  no retained path, stop and report.
- **AC-1.** sec, secrest, q2 and q3 are Ok, with `trusted_base()`
  unchanged. Amendment 3 and 4 rows: p1, p2_new, p5, p6_let_new, p7_new,
  p9, p11_new and p10_new (or p10 dropped with the reason), red on main
  and green here; control q1; the 15 convoy and dependent-match targets.
- **AC-1b (gated on SPEC-INDEXED-ARM-SCRUTINEE-COORDINATE).** secpp is
  Ok, and secrestpp stays Ok. The old-coordinate bodies (p2_mixed,
  u11_mixed, u7_pred_old, u6_let_old, u10_old, and `v_outer` in
  LET-ANNOTATION `vec_siblings`, flipped to Ok in the same commit as the
  mechanism) follow the landed spec rule. All of these hold together,
  with `v_indep` still refused and `trusted_base()` unchanged; a
  `v_outer` flip alone is not evidence (Architect `evt_5pesq48fpdrrs`).
- **AC-2.** secrestpp, r1, r1pp, r2, r2len, p8, b1 and b2 stay Ok; their
  false twins stay refused with a typed error; the LET-ANNOTATION AC-2
  suites stay green, except `v_outer`, which moves to AC-1b; its
  false-twin role is carried by `v_indep` (Architect
  `evt_5pesq48fpdrrs`). q4 (written at `n`) is measured, not required.
- **AC-3 (mutation, QA).** Restoring the alias installer for
  variable-side leaves returns sec to its motive rejection and q3 to K2
  at `h`.

## Stop conditions

- Any kernel or `trusted_base()` change beyond the
  KERNEL-OMEGA-STRUCTURAL-POSITIONS dependency.
- A leaf class outside AC-0's census with no retained path.
- A local repair for one producer in place of the elimination: that is
  the next entry on the same predicate, not this node.

## Hard-stop inventory (§1b)

**Paused at the §1a 6 Research hold (`evt_68jp7ct8055x3`).** Resume
checkpoint `fc15cad6f`. AC-1 is recut after the Research advisory and the
Architect's route ruling on it (Steward scope `evt_5asyh12k755v`):
- AC-1 is recut around one closure, not entry by entry:
  - one occurrence telescope, ordered by dependency and deduplicated;
  - equations solved per premise slot;
  - a per-method plan that admits mixed leaves.
  The AC-1b name layer, the convoy dedupe, ConvoyRebound admission and
  the kernel node are retained.
- AC-0 becomes program-wide: every ken-elaborator integration binary plus
  the catalog loaders, with zero failures that are green on main.
- The gate to the proved population is measured beside the full closure.

§1a: the variable-side leaf elimination chain is at 6 (Architect
`evt_6gq7rm62sy7qf`, `evt_20k6hnw9evhy5`, `evt_4f8gr0yddtjzk`,
`evt_7ex6qpqscy58q`, `evt_7ge5kbggd1n1m`, `evt_68jp7ct8055x3`).

**§1b answer (`evt_68jp7ct8055x3`): entries 2, 4, 5 and 6 share one
predicate.** R1's selector and restoration are indexed by syntactic units
(leaf, source position, origin tag). Those units do not coincide with
what the J must transport: the premise slot, the occurrence, and its
dependency order. Entries 1 and 3 are separate. Entry 1 is closed by the
retained scrutinee-only path, and entry 3 by
KERNEL-OMEGA-STRUCTURAL-POSITIONS.

**Shared predicate (Architect `evt_4f8gr0yddtjzk`).** The raw index
premise `e : Eq Nat (Suc k) (Suc m)` is a free proof variable inside
computational Casts, and Ken compares those Casts with `e` treated
syntactically. Every re-coordination must reproduce `e` byte for byte or
carry it as a binder. The closure must address `e`'s representation.

1. A leaf whose only variable is the outer scrutinee index was routed to
   elimination, whose dependency closure (here `xs : DualEnv a n`) is
   necessarily outside the method. Keyed on which side of the leaf is a
   variable.
2. A nested match that substitutes an outer variable does not transport
   enclosing match-origin binders (enclosing Fields and R1's restored
   dependents) whose types mention it; `compute_context_convoy` decides
   transport by the binder's origin tag, not by what the match
   substitutes.
3. The restoration replaced the eliminated premise by `Refl` or by a
   J-derived proof, but the method goal's convoy Cast
   `cast (DualEnv a (Suc k)) (DualEnv a (Suc m)) e _` carries the raw
   premise `e`, and neutral Cast congruence (conv.rs ~:1410) compares
   proofs syntactically. Keyed on proof-term identity.
4. The R1 dependency closure refused an outer let whose arm occurrence is
   this method's context-convoy binder (`v_fresh`: `k : Vec Nat (Suc m)`
   aliased to `Vec Nat (Suc m')`), because it judged crossing by the
   binder's declared position, not by where its alias lives. Keyed on
   binder position (origin), as entry 2 was keyed on origin tag. Ruled
   (`evt_7ex6qpqscy58q`): admit `frame.convoy_originals` as Original
   dependents rebound ConvoyRebound; re-derive only Eq premises.
5. A nested match's context convoy carried one arm-visible value twice:
   the original binder whose alias is the R1-rebound binder, and the
   rebound binder itself. It also carried an original whose alias is the
   scrutinee. The arm name went to one slot and the goal to the other.
   Keyed on binder position, not on the occurrence the arm sees. Ruled
   (`evt_7ge5kbggd1n1m`): §3.3 coordinates live in a separate name layer,
   `arm_coordinates`, read only by the three name resolvers; the convoy
   keys on the occurrence and skips the scrutinee's alias.
6. R1's unit of work is the syntactic leaf and source position, not what
   the J transports:
   - (A) a premise slot that projects to several leaves must be reflexive
     after its first leaf;
   - (B) the reverted telescope is ordered by source position but binds
     alias values, so a binder whose type mentions an inner-bound alias
     keeps an old-coordinate occurrence;
   - (C) a variable leaf beside a constructor leaf in one method (frame
     stop);
   - (D) nested-elimination output is rejected by the kernel, untraced.
   Keyed on leaf and position, not on premise slot and occurrence.
