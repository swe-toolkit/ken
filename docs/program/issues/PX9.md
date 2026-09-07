---
id: PX9
title: "cross-domain System.Error — semantic identity, raw errno, operation, resource, safe context, and honest retry classification"
status: active
owner: foundation
size: L
gate: none
depends_on: [PX8, ABI-REVOKE]
blocks: [ABI-S1, ABI-S5, PX10, PX11]
github: null
origin: docs/program/10-linux-abi-completion.md §4 (the ABI-completion program); node filed by the Steward 2026-07-25 on the operator's directive to frame the remaining program. Agents cannot create tracked work (COORDINATION §2).
---

> ## Authority: `10-linux-abi-completion.md` §4 — read that, not this
>
> ⛔ **This is a tracker/DAG node, NOT a shovel-ready WP frame.** A
> `docs/program/wp/` frame carrying deliverables, acceptance criteria, fixed
> inputs, negative controls, and a contention check **must be authored before
> release** (§2c front-load rule). **Do not release this on the strength of this
> file.**

## Objective — **the charter's own undelivered WP**

Cross-domain `System.Error`: semantic identity, raw errno where present,
operation, resource, and safe context; plus retry/interruption/transience
classification.

⛔ **The classification must NOT promise that retry is always safe.** An
error that is *transient* is not thereby *idempotent to retry* — conflating them
is how a retry loop corrupts state.

⛔ **It must reach BEYOND filesystem** — process, socket, and later completion
contexts. A filesystem-only error type is precisely the floor that already
exists and is what makes Track T impossible.

## ★ PX9 gates most of Track T — and that is a sequencing argument, not a preference

Sockets and processes need error context the **filesystem floor cannot express**.
Retrofitting it afterwards means **re-touching every operation added in
between** (`10-linux-abi-completion.md:131-133`, restated §7 as an operator/
Architect sequencing ruling: *"PX9 before PX10/PX11 … error context retrofitted
across two large surfaces"*).

⚠ **`ABI-REVOKE` is sequenced BEFORE PX9 deliberately** so PX9 absorbs the
distinct `revoked` identity rather than having it retrofitted (§7).

## Decomposition (Architect design ruling `evt_5rabg6z700p9s`, 2026-09-07)

The Architect delivered the type shape + classification model + increment cut,
grounded at `origin/main @ a93fbee33`. Core insight: the semantic identities are
**already domain-general** (`IOError` incl. `Other Int`/`Interrupted`/`Revoked`);
only the `(operation, resource)` binding is fs-bound and the retry classification
is net-new. So `SystemError` = generalize `FileError`'s carrier + add the honest
classification + unify the triplicated `revoked`. The increments and their coupled
Spec contract are framed as:

- **PX9-INC1** (`docs/program/wp/PX9-INC1-system-error-type-and-classification.md`,
  owner Foundation, size M, tier T1): the prelude `SystemError` type + honest
  two-axis retry classification (`error_transience` ⊥ `operation_idempotence` →
  `retry_guidance`; **no** `retryable : SystemError -> Bool`) + kernel-checked
  laws, with **filesystem as the first and only populated domain**. No host-wire
  change, no operation re-threading. Unambiguously in PX9 (Architect). **Released
  to the foundation ring 2026-09-07.**
- **PX9-C** (`docs/program/wp/PX9-C-error-classification-contract.md`, owner Spec,
  size S): the `/spec` normative anchor for the transient≠retry-safety property
  and the revoked-unification, so PX9-INC1's law ACs cite a normative clause.
  Coupled to PX9-INC1 at merge-order only (build in parallel). **Routed to
  spec-leader 2026-09-07.**
- **Increment 2** (fs migration + host-wire unification, TCB-adjacent): re-thread
  the ~16 FS/resource operations onto `SystemError` (`FileError` becomes the
  fs-domain projection; `ResourceError`'s revoked folds in) and unify the host wire
  revoked enums. **Not yet framed** — whether it lives inside PX9 or a sequenced
  successor node is an **operator scope call** (TCB-adjacent), forwarded
  2026-09-07. Not a dependency of PX9-INC1.
- **PX10/PX11-owned (NOT PX9):** each later domain adds `Operation`/`ResourceRef`
  arms + new identities as **pure additions** — never a reshape. This is what
  landing the domain-general shape now (fs first) buys.

The **`SafeContext`** field's meaning (redaction-aware context rendering to a
stable non-leaking label) is the Architect's reading of the charter's "safe
context"; flagged to the operator/Spec for confirmation alongside the increment-2
scope call. PX9-INC1 builds to that reading unless the confirmation narrows it.

D0 reconciliation folded into the PX9-INC1 frame 2026-09-07 (Architect ruling
`evt_20n7vgagrk9zy`): the identity slot reuses the existing `IOError` directly (no
new sum — avoids the constructor-collision class), single-Revoked is scoped to the
canonical `IOError.Revoked` classification with the surface-wide fold deferred to
increment 2, and the retry-safety AC is stated behaviorally
(`AC-RETRY-CONSUMES-IDEMPOTENCE`: the sanctioned classifier consumes both axes and
no error-only retry classifier is provided) rather than as an unrepresentability
claim.
