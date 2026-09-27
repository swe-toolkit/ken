---
name: a-mutation-fixture-narrowed-to-the-bool-checker-surface-excising-a-kernel-checked-theorem-is-a-re-pin-not-a-weakening
description: "A respin can recalibrate reddening consumer tests by narrowing their mutation fixtures to a checker_prefix that keeps only the Bool-checker layer and excises a later kernel-checked theorem. The deletions read as a weakening; they are a re-pin when three tells hold: the mutation targets a Bool decision procedure while the excised artifact is a kernel-checked theorem (mutation is category-wrong for it); the verdict flip and the collapse-reddens meta-control still fire on the narrowed env; and a compensating structural test pins the theorem (dependency-closure walk, a negative pin that it does not consume the checker's own soundness lemma, inline exact-statement inhabitation, zero trust delta). Confirm the prefix holds no theorem proof."
metadata:
  type: feedback
---

# A mutation fixture narrowed to the Bool-checker surface, excising a kernel-checked theorem, is a re-pin not a weakening

**Measured 2026-09-04 on `V3-FO-EMBEDDING-ADEQUACY` D2b ripple-respin
`5c705a4d74dbf264a06e49e762f93946993b31e2` (PR #3305), verdict
`evt_7pppne6csegbc` (thread `thr_7vb4452cn7m1y`).** Respin of the withdrawn
`54e4d0eb`, whose defect was an affected-closure red: the adequacy proof
reddened two pre-existing FoKripke consumer tests the prior candidate had not
brought into scope.

## The trap

The respin fixes the red by editing the two consumer tests, and the edits
include **deletions** (-21 total). That makes "no weakening in deletions" live:
was a reddening test made green by loosening an assertion? The edit looks
alarming. Two mutation fixtures swap

```
env_for(&mutated)  ->  env_for_checker_mutation(&mutated)
```

where `env_for_checker_mutation(s) = env_for(checker_prefix(s))` and
`checker_prefix` splits the catalog source at a section marker and keeps only
the part before it, **excising the later kernel-checked theorem** from the
source the mutation runs against. (The tests are
`v3_fo_sorted_eigenparameter_freshness_guard_mutation_proof.rs` and
`v3_fo_sorted_eigenparameter_wrong_sort_controls.rs`.)

## Why it is a re-pin: three tells

**1. The mutation targets a Bool DECISION PROCEDURE; the excised artifact is a
KERNEL-CHECKED THEOREM.** A checker (`fn ... : Bool` whose guards can be
silently wrong) is what needs mutation controls: neuter each guard and show the
verdict flips (reject -> accept). A kernel-checked theorem is the opposite
category; the kernel is its guard. Neutering a premise it consumes either
leaves the proof inert or **breaks the whole module's elaboration**, so the
fixture fails to elaborate instead of measuring anything. Keeping the theorem in
the fixture buys no coverage and can only break the control.

**2. Discriminating power over the checker is preserved.** On the NARROWED env,
(a) the reject -> accept flip still fires, and (b) the meta-control asserting a
collapsed guard reddens the suite still runs and still REDS
(`collapsing_the_sort_check_reddens_the_wrong_sort_controls` via
`env_for_checker_mutation`). If either is gone, it is a weakening.

**3. The excised theorem gets a COMPENSATING structural pin, not a comment.**
`kernel_checked_adequacy_has_the_exact_d2a_statement_and_structure` (now in
`v3_fo_embedding_adequacy_d1.rs`):

- walks the compiled proof's **transitive dependency closure** and asserts it
  consumes the intended lemmas (`fok_target_soundness`, `fok_target_k_sigma`,
  `fok_embedding_formula_forward`);
- a **negative pin**: the proof does NOT consume the checker's own soundness
  lemma (`fok_checker_soundness`), the anti-degenerate guard against the
  "adequacy = validity narrowed to checker output" vacuous-proof shape;
- an inline `elaborate_decl` re-declaring the **exact expected statement**, so
  the proof must inhabit the strong type independently of the in-file
  statement definition;
- a **zero trusted-base delta** assert.

Division of labor: whether the theorem genuinely consumes a premise is the
KERNEL's guarantee, not a Rust mutation control's. If a premise were droppable
while the theorem still checks the same statement, there is no soundness loss.
Do not file "the theorem's premise consumption isn't mutation-tested" as a gap.

## Confirm the narrowing premise structurally

The argument rests on the checker prefix NOT containing the theorem's proofs.
Verify against the landed blob, not the worktree:

- the split marker occurs exactly once, at a clean section boundary
  (`git show <sha>:<path> | grep -n <marker>`; here
  `-- === D2b: embedding adequacy proof ...`);
- no theorem PROOF is defined before the marker; only the STATEMENT
  proposition may be. Grep the prefix for `^\s*(fn|theorem|def)\s+<name>`. A
  name-prefix false match is common: `fok_embedding_adequacy_statement` (the
  proposition, allowed) matches a grep for `fok_embedding_adequacy` (the proof).

## The verdict rule

Deletions that narrow a mutation fixture to a `checker_prefix` are a re-pin when
the excised artifact is a kernel-checked theorem, the checker-surface flip and
the collapse meta-control still fire on the narrowed env, and a new structural
test compensates (closure walk, negative anti-collapse pin, exact-statement
inhabitation, zero trust delta). Confirm the prefix contains no theorem proof,
and clear.

Related: [[a-negative-check-passes-for-any-reason-so-it-needs-a-positive-control]].
