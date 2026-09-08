---
id: PX9
title: "cross-domain System.Error — semantic identity, raw errno, operation, resource, safe context, and honest retry classification"
status: merged
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
  2026-09-07; LANDED `ede8b6b72` (PR #3416).** Gates Architect (required,
  soundness core) + Foundation QA + CV all APPROVE on the exact reviewed SHA
  `92f424250`; Adversary M8 post-merge hunt NO OBJECTION with zero-TCB confirmed
  independently (the in-prelude `trusted_base()` before/after guards are
  non-vacuous). The PX9-C conformance seed
  `conformance/surface/ffi-io/seed-error-classification.md` lifted RED→GREEN in
  CI as ruled.
- **PX9-C** (`docs/program/wp/PX9-C-error-classification-contract.md`, owner Spec,
  size S): the `/spec` normative anchor for the transient≠retry-safety property
  and the revoked-unification, so PX9-INC1's law ACs cite a normative clause.
  Coupled to PX9-INC1 at merge-order only (build in parallel). **Routed to
  spec-leader 2026-09-07.**
- **Increment 2** (fs migration + host-wire revoked unification). Operator ruling
  2026-09-08 (Pat): do INC2 now — deferring it is tech debt. Bookkeeping (Steward):
  INC2 stays under PX9 as WPs (no new nodes, matching INC1); PX9 stays `active`
  until WP-B lands; the four downstream (ABI-S1/S5, PX10/PX11) correctly wait for
  the unified type+wire. **Architect decomposition `evt_6r9scjqs2qjbg` then D0
  recut `evt_6tz8jecxhhkas`:** the foundation D0 census (322 `FileError` lines /
  45 active files) REFUTED the original "11 fs ops -> `SystemError`" trunk cut —
  the producer is keyed to the flat `FileError` shape, so a trunk migration is a
  ~45-file TCB-adjacent producer re-key, and a `SystemError -> FileError` derived
  view goes PARTIAL at PX10/PX11. The deciding argument is PX9's own additivity
  guarantee: a trunk reading makes every fs consumer's match non-exhaustive the
  moment a `SocketOp` arm is added — a reshape at every new domain. So the design
  is **injection UP, not migration**: fs KEEPS `Result FileError _`, `FileError`
  + all fs signatures + the fs reifier/planner role stay UNCHANGED, and one
  total additive prelude bridge `file_error_to_system : FileError -> SystemError`
  makes fs reachable through the cross-domain `SystemError` trunk. `SystemError`
  is the trunk each domain injects into (fs now; socket/process later, each a
  pure additive `*_to_system`). Cut into two WPs, A then B:
  - **PX9-INC2A** (`docs/program/wp/PX9-INC2A-fs-surface-unification.md`, owner
    Foundation, size S-M, tier T1): the additive `file_error_to_system` bridge
    (total, permanent, monotone under PX10/PX11; `SafeContext = NoSafeContext` at
    the bridge) + the INC1 docstring cite-fix rider. `FileError`, all 11 fs op
    signatures, and the fs producer stay UNCHANGED. **Genuinely TCB-neutral**
    (ordinary kernel-checked `.ken` fn; `trusted_base()` delta empty vacuously —
    no host/reifier touch); Architect reviews as design-fit, NOT a required-TCB
    gate. **Releasable first; released 2026-09-08 (recut same day).**
  - **PX9-INC2B** (`docs/program/wp/PX9-INC2B-revoked-unification.md`, size S-M,
    TCB-ADJACENT): ALL revoked unification, atomic — BOTH the Ken-side decode
    collapse (nullary `ResourceRevoked` role re-keyed to `ResourceHostIO
    (IOError.Revoked)`, reifier collapses the two wire tags at decode) AND the
    host-wire schema fold (`crates/ken-host/src/abi_v1.rs`: add `Revoked` to
    `IoErrorIdentityV1`, route both former origins through it, retire domain wire
    `Revoked` variants + detail tags 10/11). The decode collapse MOVED out of A
    into B (it re-keys a producer role, so the TCB boundary falls on the WP
    boundary). `ResourceError` lifecycle otherwise UNCHANGED (no flatten). Wire
    round-trip discriminating control; `trusted_base()` delta empty; D0 pins ABI
    version-bump vs in-place V1 edit. Architect REQUIRED reviewer. **Releasable
    second**, gated on WP-A's surface; framed 2026-09-08 while A builds. PX9 stays
    `active` until it lands.
- **PX10/PX11-owned (NOT PX9):** each later domain adds `Operation`/`ResourceRef`
  arms + new identities as **pure additions** — never a reshape. This is what
  landing the domain-general shape now (fs first) buys.

The **`SafeContext`** field's meaning (redaction-aware context rendering to a
stable non-leaking label) is the Architect's reading of the charter's "safe
context"; flagged to the operator/Spec for confirmation alongside the increment-2
scope call. PX9-INC1 builds to that reading unless the confirmation narrows it.

Carried to increment 2 (non-blocking riders from the INC1 review, folded here so
the next increment that touches this surface picks them up):
- **CV/Adversary cite fix:** the INC1 test docstring
  (`crates/ken-elaborator/tests/px9_system_error_classification.rs`) cites
  `spec/40-effects/41-system-effects.md §1.8`, which does not exist — §1.8 landed
  in `spec/30-surface/38-ffi-io.md`. Comment-only; not worth a respin (would drop
  three exact-SHA approvals). Fix it when INC2 next edits that file.
- **Sole-producer lock (Adversary obs 2):** the INC1 control locks
  `retry_guidance` as the ONLY `RetryGuidance` producer. If INC2 adds a composed
  `SystemError -> RetryGuidance` convenience, that control must be updated (it is
  intended INC1 tightness, not a defect).
- **Prelude namespace (Adversary obs 1):** INC1 adds generic prelude globals
  (`Operation`, `Transient`, `Permanent`, `Idempotent`, `NonIdempotent`) to every
  program's namespace. CI-green = no current collision; the user-namespace
  crowding tradeoff is the Architect's namespace call as PX10/PX11 add arms.

D0 reconciliation folded into the PX9-INC1 frame 2026-09-07 (Architect ruling
`evt_20n7vgagrk9zy`): the identity slot reuses the existing `IOError` directly (no
new sum — avoids the constructor-collision class), single-Revoked is scoped to the
canonical `IOError.Revoked` classification with the surface-wide fold deferred to
increment 2, and the retry-safety AC is stated behaviorally
(`AC-RETRY-CONSUMES-IDEMPOTENCE`: the sanctioned classifier consumes both axes and
no error-only retry classifier is provided) rather than as an unrepresentability
claim.
