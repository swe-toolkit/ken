---
id: RT-ITREE-ROLE-IDENTITY-NOT-SPELLING
title: "The native backend recognises the ITree Ret and Vis roles by a suffix or substring match on the constructor's symbol spelling at about 35 production sites, so a change of artifact spelling silently changes which role a constructor plays. Recognise them by the resolved itree_ret and itree_vis identities instead, and pin that no production ITree spelling match remains"
status: ready
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect ruling evt_2vg7zkq5yh2va on the LANG-PRELUDE-FLOOR-ITREE hard stop (evt_4zsf5m00r95g7): the backend's spelling keys for Ret/Vis are a check-10 debt, filed as its own Runtime node, not blocking ITREE. Requested again by the Language Leader in evt_43at79gqj318w. Steward-filed per COORDINATION section 2."
---

# The backend recognises ITree roles by identity

## Objective

Every production site in `crates/ken-runtime/src` that decides whether a
constructor is the ITree `Ret` or `Vis` role compares it with the resolved
role identity. None matches on the symbol's spelling. Behavior on every
current plan is unchanged.

## Fixed inputs (main `6e778c2d5`)

- **The trigger.** Under LANG-PRELUDE-FLOOR-ITREE's first build, scoped
  constructors got the symbol `ctor:<pkg>::ITree::ITree.Ret`.
  - The `ends_with("::ITree::Ret")` and `ends_with("::ITree::Vis")` sites
    stopped matching.
  - `contains("::ITree::")` at `lowering/core.rs:7285` and `:7289` still
    matched.
  - The planner's halves then disagreed at the marked-demand guard,
    `responses.rs:3429`.

  The ITREE fix restores the leaf spelling. That removes the trigger, not
  the debt.
- **The population.** `git grep -n '::ITree::'` over `crates/ken-runtime/src`,
  excluding `*tests*` paths, `test_objects.rs` and `ctor:fixture::` literals,
  finds 35 match lines in 11 files under `cranelift_backend/`:
  - `lowering/`: `calls.rs:2760`; `core.rs:7285`, `:7289`, `:10166`, `:15956`
    and `:16256`; `joins.rs:2681`; `mod.rs:7954`; `source.rs:1926` and
    `:1932`; `units.rs:3468` and `:5723`;
  - `planning/static_transition/`: `aggregates.rs:6665`, `:6687` and
    `:9045`; 13 lines in `responses.rs` from `:1709` to `:4481`; 4 in
    `responses/relay_settlement.rs`; `returned_vis.rs:182` and `:410`;
    `selected_pending_calls.rs:529` and `:694`.

  The Architect's ruling counted about 38. The first act's census is the
  population, not either number.
- **The identity exists.** `NativeProcessSymbols` carries `itree_ret` and
  `itree_vis` (`native_process_entrypoint.rs:105`). The elaborator derives
  them from checked ids through `CanonicalRuntimeRoles` (`prelude.rs:58`).
  `planning/static_transition/brackets.rs` already compares against
  `symbols.itree_ret` and `symbols.itree_vis`.

Treat anchors as perishable. If a fixed input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. **Census first.** On the cut base, list every production role decision
   by spelling. That covers `ends_with`, `contains`, `==`, and any
   `format!`-built or split comparison on `::ITree::`, `ITree`, `Ret` or
   `Vis`. For each site, record whether resolved `NativeProcessSymbols` is
   already in reach.
2. **The migration.** Each site compares against the resolved `itree_ret`
   or `itree_vis`. Where the symbols are not in reach, thread them through
   `ken-runtime` only. Test fixtures that build `ctor:fixture::ITree::*`
   strings supply matching resolved symbols instead.
3. **The sweep pin.** A `ken-runtime` test fails if any non-test source file
   under `crates/ken-runtime/src` contains a `::ITree::` string match. It
   names the file and line.

## Acceptance

- **AC-1.** The sweep pin passes. It goes red when any one migrated site is
  restored to its suffix match.
- **AC-2 (identity, not spelling).** A unit row whose constructor is
  spelled `ctor:other::ITree::Ret` but is not the resolved `itree_ret` is
  not recognised as `Ret`. It is red first on the cut base. A row whose
  resolved `itree_ret` is spelled with no `::ITree::` segment is recognised.
- **AC-3.** The named native population is green on the candidate:
  - the named `rt_parity_native` rows;
  - `rt_span_prov_native`;
  - `px7f_resource_native`;
  - the six `static_transition::responses::tests` fixtures.

  A newly refused plan is a stop, not a pin update.

## Stop conditions

- A site cannot reach the resolved symbols without a change outside
  `crates/ken-runtime`.
- A census site decides something other than the Ret/Vis role, such as a
  package or module boundary. Report it to the Architect rather than
  migrating it.

Checks: 3 fired: the 35-line population was counted at `6e778c2d5` with the
exclusion rule stated, and the census is the deliverable's first act. 8
fired: AC-2's wrongly spelled row is accepted today and refused after the
migration, so a refactor that kept the spelling match would fail it. 10
fired: this node is the check-10 repair: role by checked identity, not by
spelling.
