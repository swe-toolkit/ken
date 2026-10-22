---
id: KERNEL-SCT-GROUP-TRANSPARENT-COMPANIONS
title: "Refined structural recursion whose open obligations mention the recursive call's result is refused with K0014, because the obligation constants are typed against the public group member and the worker split cannot rewrite them. Admit the member group while its non-recursive public wrappers are transparent companions, so those programs are accepted with their open obligations"
status: ready
owner: kernel
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-22: \"approve change\" (the kernel companion-admission repair from LANG-REFINED-PARAMETER-DEPENDENT-RESULT, Architect stop evt_e23n6wvtsd2s, Steward closure evt_5q797p3jqwgsf). Trusted-base growth approved by the operator. Steward-filed per COORDINATION section 2."
---

# SCT groups with transparent companions

## Objective

With `def AnyNat = { x : Nat | Top }`, `fn g (n : AnyNat) : { y : Nat |
Equal Nat y n } = match n { Zero ↦ Zero; Suc m ↦ g m }` is accepted with
its open obligations, and non-decreasing recursion is still refused. The
kernel admits a recursive member group while the group's public wrappers
are transparent companions.

## Settled inputs (Architect D0 `evt_552xk2gh3dvdb`, on `7994ed9e4`)

- **The refusal.** Four shapes give K0014 on `main`: d2 (above), d2s
  (`Suc (g m)`), r2 (`: Nat ensures Equal Nat result n`), and r4 (an
  accumulator, `g m (Suc acc)`). They are listed in
  LANG-REFINED-PARAMETER-DEPENDENT-RESULT.
- **Companions.** A companion is a public wrapper (`g648 := λn. worker
  n.1 n.2`) whose body mentions the members. It is a staged placeholder of
  an earlier transaction than the member group, and its body:
  - is checked against its staged type with every placeholder opaque;
  - is non-recursive: it reaches no companion, directly or through any
    transparent body;
  - is not reached by any member.

  The open-obligation holes whose types mention a member (`g649`,
  `g651`) stay opaque and are not companions.
- **SCT.** SCT runs on the members only, and `sct.rs` is unchanged. A
  member that reaches a companion is refused by the existing escape check.
  Conversion unfolds companions during the member check.
- **The D0.** `/workspaces/ken/local/arch-sctc-d0/`: `prototype.diff` (7
  files), the kernel probe, row, suite and CLI logs, and the row sources
  in `src/`. Kernel half: `admit_pending_with_companions` and
  `CompanionRefusal` in `check.rs`, re-exported from `lib.rs`, and the
  `pub(crate)` `commit_checked_view` in `env.rs`. Failure is atomic
  through a cloned view and the existing rollback; there is no
  transparent-to-opaque downgrade. `check`, `infer`, `conv`, `sct.rs` and
  `admit_bodies` are unchanged.
- **Trust, measured.** No new assumption kind in `trusted_base()`.
  Per program the growth is exactly the open-obligation holes: for each
  row, `decls = [T648, O649, T650, O651, T652r]`, trusted added 2.
- **Operator approval.** The trusted-base growth is approved (2026-10-22,
  "approve change").

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

One landing unit on base `7994ed9e4`, built by the kernel ring. Neither
half reaches a surface program alone.

1. The kernel half, as in the D0.
2. The elaborator half:
   - cherry-pick the `refined_recursion.rs` hunk of WIP `47c25c5c3`, the
     `ProjectionRedexes::Induced` pre-check (the file is byte-unchanged
     between `d6f94aa47` and `7994ed9e4`);
   - in `admit_pending_refined`, replace the two-step workers-then-wrappers
     admission with `admit_pending_with_companions`, falling back to
     `admit_pending` on the returned companions.
3. Committed kernel probes and row tests. Do not carry the WIP test file
   as written: its "all three goals remain open" assertion on the direct
   harness is false on `main`.

## Acceptance

- **AC-0 (D0, Architect).** Done, `evt_552xk2gh3dvdb`.
- **AC-1.** Through the `ken check` path, d2, d2s, r2 and r4 each give
  rc 0, 3 open obligations, status unknown, at the same three sites as on
  `b22a41649`. Goal text and labels are fixed at elaboration and are not
  this WP's to match.
- **AC-2 (negative).** These stay refused:
  - `g n` and `g (Suc m)` with the dependent result, and a
    source-level literal-pair projection: K0014;
  - a recursive companion, and a companion that reaches itself through an
    external transparent definition;
  - recursion routed member to companion to member (escape);
  - a companion with no body, and companions staged after the members.

  The plain control keeps its 1 open obligation.
- **AC-3.** `lang_refined_parameter_structural_recursion`, the subset-Σ
  suites and `ken-kernel` stay green. A census of SCT verdicts over the
  catalog and examples is unchanged except for the four rows.
- **AC-4 (mutation).** Without companions, the kernel positive gives
  TypeMismatch and the four rows give K0014. Admitting a recursive
  companion reddens its AC-2 probe.

## Stop conditions

- Trust growth beyond the D0's measurement, or a change to `check`,
  `infer`, `conv`, `sct.rs` or `admit_bodies`.
- Any changed SCT verdict outside the four rows.

Checks: 4 fired: the D0 ran its kernel code against the real API on
`7994ed9e4` (7/7 probes, four rows rc 0 with 3 open). 7 fired: AC-2
carries each path by which recursion could reach a companion, which a
blanket companion admission would wrongly accept.
