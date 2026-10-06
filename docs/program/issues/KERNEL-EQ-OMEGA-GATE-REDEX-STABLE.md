---
id: KERNEL-EQ-OMEGA-GATE-REDEX-STABLE
title: "The three Omega gates that KERNEL-EQ-OMEGA-CARRIER-REDUCTION added decide Omega-ness with classify on the substituted instance, which errs on a lambda-headed redex and so answers false for a well-typed Omega type. An Omega carrier then reduces (R1), a subset Sigma stays stuck (R2), and an inductive Eq reduct is ill-formed or stuck (R3). Make each gate's Omega decision stable under substitution"
status: merged
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-EQ-OMEGA-CARRIER-REDUCTION]
blocks: []
github: null
origin: "Adversary finding evt_2j6624tn16srz on e15c9d35c. The repair restores the deliverables of the operator-approved KERNEL-EQ-OMEGA-CARRIER-REDUCTION (2026-10-02) and adds no trust, so it is inside that approval. Placed on the kernel ring ahead of KERNEL-J-NONREFL-ENDPOINT-SHARING, because a correctness defect in landed code outranks a performance repair. Steward-filed per COORDINATION section 2."
---

# The Ω gates hold when an instance carries a λ redex

## Objective

R1, R2 and R3 (spec 16 §2.2 and §8.4, `SPEC-EQ-FORM-OMEGA-CARRIER`) apply
whenever the carrier or field is Ω, including when an earlier field value
or a Σ first component is a λ. `infer(whnf(Eq …))` succeeds for every well-
typed `Eq` the gates see.

## Fixed inputs (Adversary `evt_2j6624tn16srz`, read at `e15c9d35c`)

- `omega_classified` (`crates/ken-kernel/src/obs.rs:121`) is
  `matches!(classify(..), Ok(Sort::Omega(_)))`, so a `classify` error is
  read as "not Ω". Its three callers are R1 (`:130`), R2 (`:326`) and R3
  (`:630`).
- `classify` of an instance containing `(λn. P) z` errs, because `infer`
  has no rule for a λ head (`check.rs:475`, "cannot infer an introduction
  form").
- The four Adversary rows, each with a control that differs only by a
  postulated predicate in place of the λ. With `N` a one-constructor family
  (`z`), `P, Q : Ω_0`, distinct postulated `p, q : P` and `q2 : Q`:
  - **R3 ill-formed:** `D : Type 1` with `mk : (F : N → Ω_0) → F z → D`.
    `Eq D (mk (λn.P) p) (mk (λn.P) q)` reduces to a conjunct
    `Eq ((λn.P) z) p q`, the source proof, and `infer(whnf)` errs.
  - **R3 stuck:** `Eq D (mk (λn.P) p) (mk (λn.Q) q2)` stays neutral.
  - **R2 stuck:** `S := Σ (F : N → Ω_0). F z`.
    `Eq S ((λn.P), p) ((λn.P), q)` stays neutral, and the J fallback's
    `infer(source)` errs as well.
  - **R1, an Ω carrier reduces.** The Adversary's `Eq (Π x:N. (λn.P) x)
    f0 g0` fails `infer` at Eq-Form and cannot be formed. R1 is reached
    through a reduct, so the row is the Architect's reachable instance
    (`evt_3ent3naf8qv7q`): `D2 : Type 1` with
    `mk2 : (F : N → Ω_0) → (Π x:N. F x) → D2`, and postulated
    `f1, g1 : Π x:N. P`. `Eq D2 (mk2 (λn.P) f1) (mk2 (λn.P) g1)` infers
    `Ω_1`. Its field-1 conjunct takes the Π arm on main.
- Every AC-1/AC-2 fixture of the parent WP uses postulated predicates, so
  none reaches this axis.

## AC-0 ruling (Architect `evt_3ent3naf8qv7q`, prototype on `669a5cf91`)

- One structural, fail-closed judgment, `omega_sort` / `omega_sort_whnf`,
  replaces `omega_classified`. It decides Π and Σ by their formation rules
  (`sort_pi`, `sort_sigma`) after WHNF, and classifies only a WHNF type
  with no former head. `None` (undecided) leaves the Eq neutral. R1 uses
  the `_whnf` form, because re-normalizing its already-WHNF carrier is
  exponential through nested Eq carriers.
- Covered: all three gates (R1 `:130`, R2 `:326`, R3 `:630`), and with R3
  both of its callers. Outside: `type_eq_by_j`, `type_eq_by_j_with_base`,
  `type_level`, the Quot sites and `cast_at_inductive` `:1219`. These
  already fail closed and need a Type level, not Ω-ness.

## Deliverable

Each of the three gates decides "this carrier or field is Ω" by a
judgement that is stable under substitution of a λ value. The mechanism is
the Architect's to rule at AC-0. Candidates include classifying after
reducing the instance's redexes, and classifying the binder form (the
constructor telescope, or the Σ codomain under its binder). R1 has no
binder form.

## Acceptance

- **AC-0 (inside this WP).** List every site in `obs.rs` that calls
  `classify` or `infer` on a substituted instance and turns an `Err` into
  a reduction decision, including the R2 J fallback's `infer(source)`. The
  Architect rules the mechanism, and which of those sites it covers,
  against that list.
- **AC-1.** The four rows above are kernel tests. In each, `Eq …` infers,
  `infer(normalize(whnf(Eq …)))` succeeds, and:
  - R2 and both R3 rows reduce, with the Ω conjunct target/target, checked
    syntactically as in the parent's R3 pin;
  - R1: the field-1 conjunct is exactly `Eq (Π x:N. (λn.P) x) g1 g1`, and
    its `whnf` in the empty context equals it (R1 neutral).

  `infer(whnf(…))` is not the observation: a correct reduct still carries
  the uninferable redex `(λn.P) z`. The reduct's level may sit below the
  Eq's (16 §8.4 (1)).

  The four postulated controls stay green and unchanged.
- **AC-2.** Restoring classify-on-instance at each gate in isolation
  reddens that gate's row at its named assertion. The parent's suite
  `obs_eq_omega_carrier_reduction.rs` stays 8/8, and
  `conv::tests::stuck_nested_components_take_linear_reducer_entries` stays
  green.
- **AC-3.** One catalog census run, compared against the same run on its
  base, shows no change. An undecided carrier now stays neutral instead of
  reducing.

## Stop conditions

- The fix would give `infer` a rule for a λ head, or otherwise change the
  kernel's typing rules: stop to the Architect.
- `trusted_base()` changes: an operator question.
- A term outside the four rows and the AC-0 list starts or stops checking:
  stop to the Architect with it.

## Closeout

Merged `e140cad94` from exact `b5c72d681` (PR #4535 from
`wp/KERNEL-EQ-OMEGA-GATE-REDEX-STABLE-respin`, main push run 37399851282;
the withdrawn head's PR #4521 is closed). Kernel QA `evt_32vfnsw7amgpj`,
Architect `evt_3852cbkmea262`, Decision `dec_6yf02gy6vn212`.

- One judgment, `omega_sort` / `omega_sort_whnf`, replaces
  `omega_classified`. It decides Π and Σ by their formation rules after
  WHNF, decides an inductive-former spine as not Ω in any context, and
  keeps the fail-closed `classify` fallback for other neutral heads. An
  undecided sort leaves the Eq neutral at all three gates.
- Four kernel rows pin R1-R3, each with a postulated control, and
  restoring the old decision at any gate reddens its row. `check.rs`,
  `conv.rs` and `trusted_base()` are unchanged.
- Carry, unframed: the R1 context item named in the Architect's approval
  `evt_3852cbkmea262`.
