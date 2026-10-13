---
id: LANG-CONVOY-C1-AFTER-C2-DEPENDENCY
title: "LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE orders every C1 convoy entry before the companions and C2, so a C1 hypothesis whose type mentions a C2 binder names sentinels bound later in the telescope, and the elaborator panics on well-formed source where its parent returned a typed error. Order premises by what each mentions, and never panic"
status: ready
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE]
blocks: []
github: null
origin: "Adversary M8 finding evt_28a76ystwx81e on 295ff612e (LANG-CONVOY-AMBIENT-BINDER-COEXISTENCE): a robustness regression against 559f76166, fail-closed, with no unsound acceptance. It violates that WP's Composition clause (Architect evt_mgkvvs0f31b7, part 2) and its Objective. Check 7 (the sibling of the ruled ordering). Steward-filed per COORDINATION section 2."
---

# A C1 convoy entry may depend on C2

## Objective

Well-formed source never reaches a panic or an internal assertion in convoy
premise construction. Each premise follows everything its rewritten type
mentions.

## Settled inputs (Adversary `evt_28a76ystwx81e`, on `295ff612e`)

- **The repro.** Fixture `env(true)` from
  `crates/ken-elaborator/tests/lang_convoy_ambient_binder_coexistence.rs`,
  with L = `dual_lookup a (Suc m) xs (DualFZ m)`:
  - **P2:** `theorem p2 (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y :
    a) (h : Equal a (L) y) (phi : Equal a (L) y → Prop) (r : phi h) :
    Equal a (L) y = match xs { DualCons k x rest ↦ h }`.
  - **P4:** P2 with `phi` declared before `h`.
  - **P6:** P2 with the goal `Equal a (L) (L)` and the leaf `Refl`.

  All three panic with "refinement sentinel slot exceeds its own premise
  telescope" at the `.expect` in `finalize_refined_body` (elab.rs:9619).
  The path is `wrap_dependent_motive_convoy` (:8050), then
  `wrap_premise_pis_finalized` (:10558). On the parent `559f76166` the
  same three rows are typed `KernelRejected`. No elaborator caller catches
  the panic.
- **The mechanism.**
  - `h` and `phi` mention `dual_lookup`, so they are C2. `r : phi h` has
    no embedded eliminator of its own, so it is C1.
  - `convoy_entry_order` (:10596) puts every C1 entry before the
    companions and C2. Its consumers are at :4689 and :4697, :6927 and
    :6937, :7391 and :7444, :8088 and :8121, :10619, and :10774 and
    :10824 (the `build_dependent_constructor_frame` path).
  - `r`'s premise therefore names the sentinels of `h` and `phi`, which are
    bound later.
  - `reject_convoy_companion_cycle` (:10723; called at :7435, :8113 and
    :10815) guards only a companion that depends on C2, never a C1 entry
    that does.
- **Controls that stay Ok on `295ff612e`:**
  - H5;
  - P2 without `r`;
  - P2 without the match, with the leaf `h` alone;
  - P7, where an inner `q : pe xs` follows `h`.
- **Unchanged, not this WP.** P5, the goal `phi h` with the leaf `r`, is
  `KernelRejected TypeMismatch` both before and after the merge.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. **D0 (Architect).** Rule the order rule over C1, companions and C2. One
   option orders premises by dependency, a C1 entry following each binder
   its rewritten type mentions. Another refuses the dependency as a typed
   error, as `reject_convoy_companion_cycle` does for companions. The D0
   says which, and whether P2, P4 and P6 then check. It covers every
   `convoy_entry_order` consumer above, since check 2 applies: the motive,
   method, refined-type and finish sites must agree.
2. **The ruled repair**, plus replacing the `.expect` at :9619 with a typed
   error, so that any slot underflow left over is a refusal, not a panic.

## Acceptance

- **AC-1.** P2, P4 and P6 reach no panic. Each one checks, or returns the
  typed refusal the D0 names. A false twin of P2, with the goal `Equal a
  (L) z` for a fresh `z : a`, is refused.
- **AC-2 (control).** The four rows in the controls list stay Ok, and P5
  is unchanged. `lang_convoy_ambient_binder_coexistence` 7/7, the 19-test
  set (`lang_embedded_context_convoy` 6, `lang_nested_index_match_proposition_goal`
  4, `dependent_match_coherent_frame_acceptance` 9) and the true rows'
  `trusted_base()` are unchanged.
- **AC-3 (mutation, QA).** Restoring the C1-first order reddens AC-1 at
  the ruled site.

## Stop conditions

- Any kernel rule, conversion or `trusted_base()` change.
- A false twin checks.
- A suite or census delta outside the named rows: stop to the Architect.
