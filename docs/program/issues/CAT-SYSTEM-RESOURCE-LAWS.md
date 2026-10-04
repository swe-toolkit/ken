---
id: CAT-SYSTEM-RESOURCE-LAWS
title: "System.Resource promises acquire-before-body, settle-after-body and a fixed body/release result ordering, but only two interpreter fixtures check it. Prove the bracket's sequencing and settlement ordering as prelude theorems beside the definitions, before the hide"
status: active
owner: foundation
size: S
tier: T2
gate: none
depends_on: []
blocks: []
github: null
origin: "The last unframed row of docs/program/CATALOG-PROOF-COMPLETENESS-SURVEY.md (Capability/System/Resource.ken.md), under operator ruling 2026-09-13 / PRINCIPLES #16. Framed by the Steward 2026-10-04 as L3's successor to CAT-JSON-LAWS. Measured at origin/main e771f7e7d. Reframed into the prelude by Architect ruling evt_7zjnsss4tdbt2 on the D0 stop evt_1yj7djtjw3s5g."
---

# System.Resource bracket laws

## Objective

The guarantees `Capability/System/Resource.ken.md` states in prose are
checked, citable theorems: each of `withResource`, `withBuffer` and `withMapping`
acquires before its body and settles after it, and the settled result is
fixed by the body result and the release result. No new trust, and no
new resolvable definition.

## Reframe (Architect `evt_7zjnsss4tdbt2`, measured on `origin/main`)

The first frame put the theorems in package source. The implementer's D0
stop (`evt_1yj7djtjw3s5g`) is correct: that frame's constraints cannot all
hold.

- Every name the laws need is in prelude.rs's `private_names` (`:2832`) and
  removed from resolution by `hide_prelude_names` (`:2878`). That covers
  `PrivateFsOpen`, `PrivateResourceRelease`,
  `private_with_resource_after_open`, `release_if_live` and the five
  `resource_settle_*_for`. The PX7-F absence pin
  (`px7f_system_resource_acceptance.rs:10`) observes the hide.
- The public settlement wrappers (`:2289`) fix `o := FileError`.
  `withBuffer` and `withMapping` settle with `o := ResourceError` (`:2390`,
  `:2427`), so laws over the wrappers would cover only `withResource`.
- The prelude already has the shape for this case. At `:2663`: "Each law
  below is checked before its private terms are hidden".
  `write_all_entry` (`:2674`) and its siblings are prelude theorems over the
  hidden `private_write_all_fuel`.

## Settled inputs (measured at `e771f7e7d`, re-read on the reframe)

- `withResource` (`:2363`) is a `Vis` on `PrivateFsOpen` continued by
  `private_with_resource_after_open` (`:2340`). On `Err` it gives `Ret (Err
  open_error)`; on `Ok resource` it gives `bind (body resource)` into
  `release_if_live` on that resource. `withBuffer` and `withMapping` have
  the same shape through their own after-acquire functions.
- `release_if_live` (`:2325`) is a `Vis` on `PrivateResourceRelease`
  continued by `Ret (resource_settle_result_for o e r body_result settled)`.
- `resource_settle_result_for` (`:2278`) and its helpers (`:2214`, `:2236`,
  `:2258`, `:2268`) treat release error `Closed` as already settled and
  carry each of the other ten `ResourceError` constructors. Every arm
  returns `Ok`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

Theorems in `crates/ken-elaborator/src/prelude.rs`, placed after the
`withMapping` block and before the hide, proved over the real definitions:

1. **Settlement ordering**, over the generic `resource_settle_result_for o
   e r`, so all three brackets are covered:
   - body `Ok v` with release `Ok` or `Err Closed` gives
     `Ok (ResourceBracketOk v)`;
   - body `Ok v` with release `Err x`, where `x` is not `Closed`, gives
     `Ok (ResourceBracketReleaseError x)`;
   - body `Err b` with release `Ok` or `Err Closed` gives
     `Ok (ResourceBracketBodyError b)`;
   - body `Err b` with release `Err x`, where `x` is not `Closed`, gives
     `Ok (ResourceBracketBodyAndReleaseError b x)`.

   Take `x : ResourceError` and `h : Not (Equal ResourceError x Closed)` as
   binders, and `match x` with `Closed ↦ absurd (h Refl)` (CHECKS 11).
2. **Sequencing**, each closed by conversion as `write_all_entry` is:
   - for each bracket, the acquire `Vis` continued by its after-acquire
     function;
   - after-acquire on `Err` is `Ret (Err ...)`;
   - after-acquire on `Ok` is `bind (body ...)` into `release_if_live` on
     the same resource;
   - `release_if_live` is the release `Vis` continued by `Ret` of the
     settlement.
3. **Visibility: visible** (operator 2026-10-04, choosing the simpler form).
   Follow the `write_all_*` precedent. Leave the theorem names out of
   `private_names`, so they are nameable proofs that users can cite. The
   helpers they mention stay hidden. No definition, constructor or effect
   becomes resolvable, and a theorem adds no trust.
4. **Package prose.** `Resource.ken.md` names the prelude theorems where it
   states each guarantee. Add no package theorem.

## Acceptance

- **AC-1.** The prelude loads at the default 2 MiB test worker. The
  prelude-load suites, the PX7-F suites and the stack-victim set that the
  prelude stack-floor carry tracks all pass. `trusted_base()` is unchanged.
  A focused test resolves each theorem by name and checks its checked
  statement against an independently written client statement.
- **AC-2 (falsifiers, restored byte-identically).**
  - M1: map `Closed` to `ResourceBracketReleaseError Closed` in
    `resource_settle_ok_error_for`, and the body-ok/`Closed` law reddens.
  - M2: drop `body_error` in `resource_settle_body_error_for`'s
    `ResourceHostIO` arm, and the body-and-release law reddens.
  - M3: continue `private_with_resource_after_open`'s `Ok` arm with
    `release_if_live` before `body`, and the sequencing law reddens.
- **AC-3.** The PX7-F absence pin stays green, so the helpers stay
  unresolvable.

## Stop conditions

- A sequencing equation does not close by conversion: stop to the Architect
  with the failing declaration.
- The prelude no longer loads at the default worker: stop with the frame
  measurement.
- Out of scope: runtime generation liveness, rights, revocation, controlled
  traps and exactly-once settlement. These are TCB contracts, not Ken proof
  targets.
