---
id: LANG-QUALIFIED-CONSTRUCTOR-PRIVACY
title: "Qualified T.C resolution reaches the prelude's hidden resource constructors (BufferHandle.PrivateBufferHandle, MappingHandle.PrivateMappingHandle, BufferSpan, TransferCount, ResourceTraceIdentity) in ambient elaborate_file: key constructor privacy on identity so every resolution path refuses a hidden constructor"
status: merged
owner: language
size: S
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Adversary finding evt_5ebm836qffqrq (HIGH) after LANG-QUALIFIED-CONSTRUCTORS landed at 8dbd5c773: the checked-surface guarantee of spec 38 §1.7.1 and §1.9 is broken on main. Regression repair; preempts LANG-EXPRESSION-SIGMA's rebase review. Steward-filed per COORDINATION section 2."
---

# Hidden prelude constructors stay hidden under T.C

## Objective

No checked user code can construct or match on a constructor the prelude
hides, whatever spelling reaches it. `38 §1.7.1` and `§1.9` hold on `main`
again: user code cannot forge a resource/capacity pairing, project the raw
`Resource`, or construct a handle outside `withBuffer`.

## Settled inputs -- measured by the Adversary at `8dbd5c773`

- **Accepted on main, refused on `f246e43eb`**, all under `ElabEnv::new()`
  with `elaborate_decl` or `elaborate_file` (the path `ken run` and `ken
  check` use, `ken-cli` `main.rs:395`, `lib.rs:130`):
  - `BufferHandle.PrivateBufferHandle r (5 : Int)` in an expression;
  - `match b { BufferHandle.PrivateBufferHandle r c |-> c }`, returning the
    capacity or the raw `Resource ResourceKind.Buffer`;
  - `MappingHandle.PrivateMappingHandle`, `BufferSpan.PrivateBufferSpan`,
    `TransferCount.PrivateTransferCount` and
    `ResourceTraceIdentity.PrivateResourceTraceIdentity`. The last two are
    unmeasured on the base.
- **Controls that already hold.** The bare spelling `PrivateBufferHandle r 5`
  is `UnresolvedCon`. Strict roots refuse, since `BufferHandle` is unbound
  there.
- **Mechanism.**
  - The prelude's hiding step (`prelude.rs:2889-2931`) only removes the
    spelling from `elab.globals`.
  - `data` admission records each family in
    `module_state.constructor_members` (`modules.rs:4321-4324`) and the type
    in the root scope's `checked_local_ids`.
  - `resolve_constructor_path` (`modules.rs:705-757`) accepts the owner at
    `:730` and returns the constructor id without consulting `globals`.
    `resolve_checked_ref` (`:771`) runs it before `resolve_ref`.
  - So privacy is keyed on a spelling, and the new resolver keys on identity.
    The two disagree and the resolver wins.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Constructor privacy is keyed on the constructor's checked identity, and every
resolution path consults it: qualified expression, qualified pattern,
qualified type argument (`32 §2`, AC-3a of `LANG-QUALIFIED-CONSTRUCTORS`),
and bare. The Architect rules the form at D0, for example an identity-keyed
hidden set that the prelude hiding step writes and the resolver reads, or
removing hidden constructors from the family record. No kernel,
`trusted_base()` or spec change. The prelude's driver record (`PreludeEnv`)
keeps its ids.

## Acceptance

- **AC-0 (D0, fan-in before the fix).** List every site that hides a name
  by removing a spelling (`git grep -n "globals.remove"` in
  `crates/ken-elaborator/src`), and mark each one that can hide a
  constructor or a type whose constructors `T.C` reaches. The fix covers all
  the marked sites, not only the prelude loop. Post the list and the
  Architect's choice of form.
- **AC-1 (witness, red on `8dbd5c773`).** Under `elaborate_file`, each of the
  Adversary's accepted probes is refused. That covers construction and
  pattern position for `BufferHandle` and `MappingHandle`, and construction
  for `BufferSpan`, `TransferCount` and `ResourceTraceIdentity`. Each is red
  on base and green after the fix.
- **AC-2 (controls).**
  - Public constructors still resolve as `T.C`, including
    `ResourceKind.Buffer` in type position.
  - The `LANG-QUALIFIED-CONSTRUCTORS` suite stays green.
  - The prelude's own checked definitions that use the hidden constructors
    still elaborate.
  - Reverting only the resolver-side check reddens AC-1.
  - Targeted builds only, through `scripts/ken-cargo`; no-regression means
    green in CI.

## Stop conditions

- The fix needs a kernel, `trusted_base()` or spec change, or changes a
  public constructor's identity.
- A hidden constructor stays reachable on any path AC-0 lists.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.
