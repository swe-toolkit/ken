---
id: LANG-NESTED-ARM-STALE-OUTER-COORDINATE
title: "A nested match on an indexed field refuses well-typed programs with KernelRejected since 76064f39c: an enclosing arm's installed coordinates are not rebased through the nested arm's field refinements, so outer names resolve stale. Rebase them, and keep the inner names' resolution"
status: active
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-SOLVED-INDEX-SUBSTITUTION]
blocks: []
github: null
origin: "Adversary M8 finding evt_1beqd0ntawbqc (F1) on LANG-SOLVED-INDEX-SUBSTITUTION (76064f39c): an accept-to-refuse regression. It fails closed (KernelRejected, trust unchanged). Steward-filed per COORDINATION section 2."
---

# Stale outer coordinates in a nested indexed match

## Objective

With `data Vec (A : Type) : Nat -> Type` (`VNil : Vec A 0`, `VCons : (n :
Nat) -> A -> Vec A n -> Vec A (n+1)`) and the usual `vlen`, both of these
are accepted again, as they were before `76064f39c`:

- `fn p3 (n : Nat) (v : Vec Nat n) (w : Vec Nat n) : Nat = match v { VNil
  ↦ Zero; VCons m a xs ↦ match xs { VNil ↦ vlen n w; VCons m2 b ys ↦
  vlen n w } }`;
- `fn p3b (n : Nat) (v : Vec Nat n) : Vec Nat n = match v { VNil ↦ VNil
  Nat; VCons m a xs ↦ match xs { VNil ↦ v; VCons m2 b ys ↦ v } }`.

## Fixed inputs (main `d8ec57615`, Adversary `evt_1beqd0ntawbqc`)

- **The rows.** On `60f4527f2` both are accepted. On `76064f39c` both fail
  with `KernelRejected` (`p3`: expected `Vec Nat (Suc @7)`, found `Vec Nat
  (Suc Zero)`). The bodies need no refinement, so the elaborator is
  emitting an ill-typed core term.
- **Invariant.** Spec 34 §3.3: the arm-local name substitution adds no
  kernel conversion, so a resolved name must agree with the arm's own
  refined context.
- **Mechanism, inferred by the Adversary from code, not instrumented.**
  - `install_arm_coordinates` (elab.rs ~:9198, extended at ~:9280)
    stores each value at the outer arm's depth.
  - The lookups at ~:1609, ~:2577 and ~:13624 return it weakened, ahead
    of any `var_refinements` entry.
  - The only rebase, `rebase_arm_coordinates_through_fields` (~:9160),
    runs only on the generalized-goal path (~:7800).
  - The ~:2577 lookup also drops the `unify_types` result and skips the
    `convert_type` view selection that the `var_refinements` branch
    below it performs. That is why the mismatch surfaces at the kernel.
- **Rows that moved the right way and must stay accepted:** `p3c` (`VCons
  Nat m a xs` spelled out in place of `v`) and `p3e` (`vlen n v` in both
  inner arms). `p3d` (`repl n` at goal `Vec Nat n`) is KernelRejected
  both before and after; it is not in scope.
- Adversary scratch source:
  `/tmp/claude-1000/-workspaces-ken--worktrees-adversary/5589bc59-4228-460f-b01f-52b36a013e71/scratchpad/zz_adversary_probe.rs`
  (read-only reference; do not commit it).

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

`crates/ken-elaborator/src/elab.rs` and one test file. Must not move: the
kernel, `trusted_base()`, and the verdicts of
`lang_solved_index_substitution.rs`.

## Deliverable

Repair hypothesis, to try first: when a nested method installs its field
refinements, rebase every enclosing arm coordinate through them, as the
generalized-goal path already does. The ~:2577 lookup should then honour
its unification result instead of returning the raw value. The rows join
`lang_solved_index_substitution.rs` or a sibling test file.

## Acceptance

- **AC-0 (route, Architect).** Rule the repair point: a rebase at nested
  installation, a resolution-time rebase, or `var_refinements` taking
  precedence. Also say whether the ~:2577 dropped-unify path is the same
  defect.
- **AC-1.** `p3` and `p3b` are accepted. `p3c` and `p3e` stay accepted.
  `lang_solved_index_substitution.rs` is unchanged and green.
- **AC-2 (differential).** A depth-3 variant (`match xs { VCons m2 b ys ↦
  match ys { … ↦ v } }`) is accepted, so a fix that rebases only one level
  fails it.
- **AC-3 (mutation, QA).** Removing the new rebase reddens `p3` and `p3b`
  back to KernelRejected.

## Stop conditions

- Any kernel or `trusted_base()` change.
- A changed verdict in `lang_solved_index_substitution.rs`.
- The rebase needs a coordinate the enclosing arm does not record.

Checks: 4 fired: `install_arm_coordinates`, the three lookups and
`rebase_arm_coordinates_through_fields` with its single generalized-goal
call site are on `d8ec57615` at the lines cited. 7 fired: the three
lookups (~:1609, ~:2577, ~:13624) read the same table, so AC-0 rules
all three, and AC-2's depth-3 row rejects a one-level rebase. 8 fired:
a fix that only falls back to `var_refinements` on mismatch would also
turn `p3` green, so AC-3 names the rows that the mutation must redden.
