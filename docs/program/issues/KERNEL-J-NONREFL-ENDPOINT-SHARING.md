---
id: KERNEL-J-NONREFL-ENDPOINT-SHARING
title: "A non-refl J reduction re-reduces the level below about eight times, through j_endpoints, the type-equality witness and the Cast arm's components, so a source-derived chain of J over step_i helpers costs work exponential in its depth. Reduce each child J once, so J reductions are linear in depth and reducer entries quadratic, with the J reduct's shape unchanged"
status: active
owner: kernel
size: M
tier: T1
gate: architect
depends_on: [KERNEL-OBS-NESTED-CAST-LINEAR]
blocks: []
github: null
origin: "Architect evt_1k4yc761p6rty on the KERNEL-OBS-NESTED-CAST-LINEAR site D0 (evt_3maz6hzwmq32r), disposition (a): the S residual is a different mechanism from the cast-regularity repeat. Steward-filed per COORDINATION section 2."
---

# One reduction per J endpoint

## Objective

Kernel `check` of the source-derived nested-J fixture S runs J reductions
linear in its depth k and reducer entries O(k²), with the verdict it gives
today. Linear entries are out of reach under the typed witness and the level
from `infer(source)`: each guard types `P a refl` at an endpoint of depth i,
which costs O(i) (Architect `evt_5f79a35xp026p`).

## Settled inputs (D0 `evt_3maz6hzwmq32r`, on `10daa9242` plus N0–N2)

- **The fixture.** S is `KERNEL-OBS-NESTED-CAST-LINEAR`'s source-derived
  `step_i` / `J (λx _. x) t e` chain. It checks `Refl Lk : Eq Type0 Lk
  (type_id Type0 Lk)`, and its k=4 verdict pin lands with that WP.
- **The growth** (Architect `evt_2jph7dvytz0xn`). Totals are measured with
  the `cfg(test)` deferred-fixed-point assertion suppressed
  (`DeferredFixedPointAssertionsGuard::suppress()`, `delta_probe::reset()`
  just before the measured call). That is the production cost.
  - At k = 2, 3 and 4: 8,528, 69,805 and 560,394 at `90ca730f6`; 9,018,
    73,751 and 591,988 at `3695cd16a`.
  - Growth is about ×8 per level on both bases. AC-0 re-measures the
    `j_nonrefl` call counts on main.
  - With the assertion on, totals are ×2.5-4 higher and also grow ×10, so
    they are not comparable to these.
- **Per-call site cost grows with depth.** `type_eq_by_j` alone costs 57,
  75 and 93 entries per `j_nonrefl` call at k = 2, 3 and 4, and 309 at
  k = 16 (`evt_5f79a35xp026p`). Cast regularity and `cast_at_inductive` are
  0 with N0–N2 in place.
- **The mechanism** (Architect `evt_1k4yc761p6rty`). One non-refl J
  reduction re-reduces the level below about 8 times, through D, E and A.
  - `j_nonrefl` is at `obs.rs:1207` (spec 15 §4.3).
  - `type_eq_by_j` and `type_eq_by_j_with_base` each `infer(source)`.
  - `canonical_type_eq_base` decides whether `Eq Type X X` is neutral or
    Top.
  - The Cast arm then reduces `p_a_refl` and `p_b_e`, which `j_nonrefl`
    built unreduced.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## AC-0 result and ruled mechanism

- **AC-0** (implementer `evt_5bze1d19akass`, suppressed mode on
  `3695cd16a`). Each non-refl J spawns 8 children one level down:
  - D = 1 (`j_endpoints`);
  - E = 6, under the witness check at `obs.rs:301` (4 from `infer_j`,
    plus 2 from the final conversion);
  - A = 1 (the Cast source component).
- **Endpoint sharing alone fails** (Architect `evt_64yx9v9m9bgvs`).
  Reducing the endpoints made it ×23.7 worse. Syntactic alignment kept
  ×8.18.
- **Operator 2026-10-05: "(c) then (a)".** Research advisory
  `evt_4xjj6256dapa4`, then the witness is typed by construction. This
  trust change is operator-approved within this WP.
- **Mechanism** (Architect `evt_3kv8v4ajegm7q`), in `obs.rs`:
  1. At the top of `j_nonrefl`, before `j_endpoints`, the redex's own
     typing is established once with `crate::check::infer` on the `J`
     term. On `Err` the J stays neutral.
  2. The witness `check` at `:301` runs only under `#[cfg(test)]`, as an
     assertion on by default and suppressed by
     `DeferredFixedPointAssertionsGuard`. A mismatch panics with the lemma
     instance.
  3. Research's J-witness typing lemma, side conditions (i)-(iv)
     verbatim, is the doc comment of `type_eq_by_j_with_base`.
  4. Unchanged: W's and the Cast reduct's term schema, the level from
     `infer(source)`, the Ω-motive neutral gate (`:269-271`) and
     `canonical_type_eq_base`'s check.

  Spec 15 §4.1 already permits synthesizing pair-eq, so there is no spec
  edit. The prelude-constant variant is not adopted.
- **AC-0' result** (implementer `evt_11rqp6rgvkh2p`). Both rows kept 2 or
  more children per parent. The guard's `infer_j` and `j_nonrefl` each call
  `j_endpoints`, whose `whnf(Eq Type a b)` reduces the child J, and the Cast
  arm reduces it a third time.
- **Mechanism, final** (Architect `evt_5f79a35xp026p`, replacing step 1
  above). At a universe carrier, read J's endpoints from the evidence's
  inferred `Eq` formation, without whnf of the `Eq` type, and run the guard
  once at those endpoints.
  - `check.rs`: `eq_formation_head` (`:835`) becomes `pub(crate)`.
    `infer_j` (`:880`) splits into `j_endpoints` plus a `pub(crate)
    infer_j_at` taking the endpoints; `infer_j_at`'s body is the existing
    `infer_j` text from the motive check on, unchanged.
  - `obs.rs` `j_nonrefl`: its first line is replaced by the formation read
    (other carriers keep `j_endpoints`), then one `infer_j_at` call as the
    guard. The ruling's code block is the text.
  - `eq_at_type` never returns an `Eq` formation (`obs.rs:431-546`), so
    wherever the old read succeeds it gives the same endpoints. The reduct
    is built from them, so its shape is kept.
  - Steps 2-4 above stand.
- **Conversion widens, as spec 15 §4.3 requires.** Where `whnf(Eq Type X
  Y)` is Top, Bottom or a Sigma, the old read failed and the J stayed stuck
  inside a well-typed term after β. With the mechanism it reduces ("never a
  stuck J node"). `infer_j` is unchanged, so kernel typing is unchanged.
- **Prototype** on `7fb8e3226`, suppressed: verdict Ok at every k, and Ok
  with the assertion on at k ≤ 4. `j_nonrefl` runs exactly 6k times.
  Entries are 126k² (1,140 at k=2; 37,498 at k=16). The patch at
  `/workspaces/ken/tmp/arch-jn3-v2-instrumented.patch` is reference only;
  its `arch_site` counters are not committed.

## Deliverable

Whnf of S runs J reductions linear in k and reducer entries O(k²), by the
ruled mechanism. The production
kernel relies on the J-witness lemma where `:301` used to check, and the
handoff names that trust.

## Acceptance

- **AC-0 (parent attribution; measure only).** For each `j_nonrefl` call
  at level i−1, name the site of the level-i reduction that encloses it.
  The ×8 must come out as a sum over D, E and A, at k = 2 to 4. Done
  (above).
- **AC-0' (measure, then stop to the Architect).** Done (above).
- **AC-1.** Three committed pins at k = 4, 8 and 16, counted with the
  assertion suppressed:
  - (a) S's check runs `j_nonrefl` exactly 6k times, by a `cfg(test)`
    counter.
  - (b) S's entries are at most 220·k² (measured 207k², 167k², 146k²).
  - (c) entries(16) / entries(8) is at most 4 (measured 3.51).
  - The verdict equals today's, and the k=4 pin from
    `KERNEL-OBS-NESTED-CAST-LINEAR` becomes a bounded pin.
- **AC-2 (controls).**
  - Reverting the repair reddens the S pin.
  - The T1 and T2 pins and their N1/N2 reverts keep their results.
  - The J-refl rule and a J left neutral at an open index keep their
    results.
  - The 57-package census shows no verdict change, and `trusted_base()` is
    equal.
  - A raw ill-typed J (base not at `P a (refl a)`) stays neutral under
    whnf, and removing the guard reddens it. A well-typed non-refl J
    reduces to the same Cast term as on base, compared syntactically.
  - The `cfg(test)` witness re-check stays on in every kernel test except
    the count pins.
  - The ruling's `rigid_type_eq_j_reduces_after_beta` row, in the `conv.rs`
    tests module, is green: the β-instantiated J whnfs to `t0` and its
    `Refl` check is Ok. Reverting the formation read to `j_endpoints` makes
    the whnf a stuck J and the check a TypeMismatch.

## Stop conditions

- **A repair that changes the J reduct's shape** beyond the ruled
  mechanism, for example taking the universe level from the motive instead
  of `infer(source)`. That is an operator question before any build.
- Any fuel, cache keyed on term identity, or depth cutoff.
- Any change to what the kernel accepts beyond the ruled conversion
  widening: stop to the Architect.

## Carry, not this WP

`infer_j`'s whnf-and-`Eq` requirement is not stable under substitution: on
main a well-typed term β-reduces to a J that `infer_j` rejects. That is a
kernel completeness item for its own frame (`evt_5f79a35xp026p`).
