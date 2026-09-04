---
id: V3-FO-ROUTE-CONSUMPTION-APPARATUS
title: "FO honest-reach prerequisite: the theorem-consumption apparatus — thread the two 23 section 4.4 theorem handles to the accepted route (Component A) and build the kernel-guarded Rust<->catalog encoding bridge (Component B), so the checker_soundness then embedding_adequacy composite is assemblable and kernel-checkable at the accepted return. Prerequisite to D3's one-return flip."
status: ready
owner: language
size: L
gate: none
tier: T1
depends_on: [V3-FO-CHECKER-SOUNDNESS, V3-FO-EMBEDDING-ADEQUACY]
blocks: [V3-FO-ROUTE-PROVED-COMPOSITION]
github: null
origin: "Steward, 2026-09-04, at origin/main 3cdd61a02, cutting the prerequisite the D3 hard stop surfaced (language-implementer evt_kw6zgeshvnp3, independently confirmed language-leader evt_2y37p7bbs9heg): the two theorems are proved+merged but nothing delivers their TERMS or the encoded ARGUMENTS to the accepted return, so the mandated composite cannot be assembled at line 597. Apparatus decomposition and the contingent-gate ruling are the Architect's evt_76rkjsahaf6dt (sharpening z2610 evt_3th40hnvytpzp): component design is the Architect's, WP-cut/sequencing is the Steward's. Steward-filed per COORDINATION section 2."
---

> # OPERATIVE (Steward, 2026-09-04). This node is the PREREQUISITE apparatus for
> # FO honest-reach. It does NOT flip line 597 (that is D3,
> # V3-FO-ROUTE-PROVED-COMPOSITION, which this node blocks) — it lands the
> # encoder + handle-threading so the composite becomes assemblable and
> # kernel-checkable there. Component A is free plumbing. Component B is the
> # soundness crux and carries the gate: gate=none IFF the encoder's faithfulness
> # is KERNEL-GUARDED, contingent on ONE measurement (phi_closed independence,
> # see Gate). If that measurement fails, STOP — trusted encoder = operator
> # funding call.

## Objective

Build the theorem-consumption apparatus so the mandated `checker_soundness`
then `fok_embedding_adequacy` composite can be assembled and kernel-checked at
the accepted-certificate return (`prover.rs:597`). The D3 hard stop established
that the two theorems are proved+merged but nothing delivers their TERMS or the
encoded ARGUMENTS to the route: `attempt_fo_with_signature` receives only
`&mut GlobalEnv` + `FoSliceSignature`; `Decl::Transparent` carries no source
name (no `fok_*` by-name lookup); and no Rust->catalog encoder exists
repo-wide. This node lands that apparatus. D3's one-return flip is the LAST
step, gated on this node.

## Fixed inputs, measured at origin/main `3cdd61a02`

- `attempt_fo_with_signature` `prover.rs:574`, accepted branch line 597 —
  UNCHANGED by this node (the flip is D3's).
- `FoSliceSignature` `fo_kripke.rs:51` `{sort_a, pred_p, or_id}` — the natural
  carrier for the theorem handles (Component A already carries the slice's
  catalog identities here).
- `FOProblem` `fo_kripke.rs:550` `{Carriers, AtomEnv, IForm}` and the Rust
  `Cert` — the plain-Rust inputs the encoder consumes (Component B).
- The catalog term types the theorem TYPES require:
  `FokSignature`/`FokCarriers`/`FokAtomEnv`/`FokScopedIForm`/`FokForm`/`FokCert`
  (the encoder's outputs).
- The two theorem constants `fok_checker_soundness` (`FoKripke.ken:2577`) and
  `fok_embedding_adequacy` (`:5859`) — resolved as `GlobalId`s where
  `ElabEnv::globals` IS available (construction / route-env setup), NOT by name
  inside the prover.
- `phi_closed` — the FO target the prover already holds at the accepted return
  (its independence is the gate measurement, below).

## Deliverables

### Component A — theorem-handle threading (free plumbing, no TCB)

Resolve the two theorem `GlobalId`s where `ElabEnv::globals` is available
(signature construction / route-env setup, not inside the prover); add the
`checker_soundness` + `embedding_adequacy` handles to `FoSliceSignature` the
same way `sort_a`/`pred_p`/`or_id` are carried; pass them to
`attempt_fo_with_signature` by `GlobalId`. No by-name lookup, so
`Decl::Transparent`'s missing name is a non-issue. Pure plumbing of handles to
already-kernel-checked theorems: zero trusted-authority delta.

### Component B — the Rust<->canonical-Ken encoding bridge (the soundness crux)

An encoder producing catalog TERMS
(`FokSignature`/`FokCarriers`/`FokAtomEnv`/`FokScopedIForm`/`FokForm`/`FokCert`)
from the plain-Rust `FoSliceSignature`/`FOProblem`/`Cert`. This is the missing
repo-wide producer. Its load-bearing obligation is FAITHFULNESS: the encoded
catalog term must correspond to the Rust problem's semantics — KERNEL-GUARDED,
not trusted, via the denotation equation (see Gate).

## Gate (Component B decides it; A is free) — CONTINGENT

**gate = NONE / buildable-in-lane-2 IFF the encoder's faithfulness is
KERNEL-GUARDED rather than trusted** (Architect evt_76rkjsahaf6dt). Concretely:
the composite's kernel-check must include the denotation equation
`fok_denote(encode(problem)) == phi_closed`, checked against `phi_closed` as the
INDEPENDENT obligation (the actual FO target, NOT itself derived from the same
encoder). Then a mis-encoding makes `fok_denote(encode(problem)) != phi_closed`,
the kernel-check FAILS, and the path falls to Unknown — the encoder is not
trusted and there is no TCB growth.

**REQUIRED MEASUREMENT, settle the gate FIRST:** is `phi_closed` (the FO target
at the accepted return) available as an INDEPENDENT kernel term the composite
can be kernel-checked against, distinct from the encoder's output?

- **YES** => encoder kernel-guarded => gate=none, lane-2 buildable. Proceed.
- **NO** (phi_closed can only be produced by the same encoder, nothing
  independent to check against) => the encoder's faithfulness is TRUSTED => it
  enters the TCB => **STOP and flag.** That is the operator funding call
  (Component B's, not A's). Route the finding to the Steward, who queues it for
  the operator (away until ~13:00 UTC 2026-09-04). Do NOT ship a trusted encoder
  without that funding.

The Architect expects the FO target is independently present (it is the
obligation the prover already holds), so gate=none is the likely outcome — but
it is CONTINGENT on this measurement, exactly as D3's flip gate was.

## Acceptance criteria

- **AC-A1 (handle threading).** The two theorem handles are resolved at
  construction (where `ElabEnv::globals` is available) and threaded to
  `attempt_fo_with_signature` via `FoSliceSignature` by `GlobalId`; no by-name
  lookup; `trusted_base()` unchanged by Component A.
- **AC-B1 (LOAD-BEARING — kernel-guarded faithfulness).** The composite's
  kernel-check includes `fok_denote(encode(problem)) == phi_closed`, checked
  against the INDEPENDENT `phi_closed`. The composite is reachable (kernel-checks
  as `phi_closed`) only when that denotation equality kernel-checks.
- **AC-B2 (NON-DEGENERATE PAIR).** A faithful encoding kernel-checks and,
  composed with the two theorems, yields the composite proof of `phi_closed`; a
  deliberately WRONG encoding fails the denotation-equality kernel-check and
  yields Unknown, NEVER a composite that proves the wrong problem. These tests
  assemble and kernel-check the composite directly (a test harness), WITHOUT the
  line-597 flip.
- **AC-B3 (TCB).** `trusted_base()` unchanged — the encoder is kernel-guarded,
  not trusted. (Contingent on the gate measurement; if it fails, this AC cannot
  be met without the operator funding call.)
- **AC-C (PRESERVATION).** This node does NOT change line 597's return (that is
  D3's flip). No prover VERDICT changes as a result of this node alone — the
  accepted branch still returns `emit_unknown_hole_fo_withheld` until D3 flips
  it. The apparatus lands validated-but-not-yet-wired.

## Banned scope

No change to line 597's return (D3's). No new postulate or trusted declaration.
No change to `FoKripke.ken` theorems (inputs). A trusted (non-kernel-guarded)
encoder is OUT until/unless the operator funds it (the gate measurement). No
change to any prover verdict from this node alone.

## Review

Architect reviews the apparatus candidate (evt_76rkjsahaf6dt): Component B's
encoder-faithfulness being kernel-guarded-not-trusted is the load-bearing item,
with the non-degenerate pair (a faithful encoding kernel-checks and yields the
composite; a wrong encoding fails the denotation-equality and yields Unknown,
never a false Proved of the wrong problem). Language QA reviews the test
controls. CV N/A unless a spec/conformance surface is touched (none expected).

## Sequencing and contention

**Contention: LOW.** Touches `crates/ken-elaborator` (`fo_kripke.rs` encoder +
`FoSliceSignature` handle fields + route-env setup) and tests;
`catalog/packages/Tooling/Verification/FoKripke.ken` is READ-ONLY (theorems and
term types as inputs). No overlap with lane-1 runtime (`crates/ken-runtime`,
`crates/ken-cli`) or lane-3 foundation (`catalog/` value modules). This node
BLOCKS V3-FO-ROUTE-PROVED-COMPOSITION (the D3 flip).

## Capability tier: T1

Component B is soundness-bearing — encoder faithfulness kernel-guarded via the
denotation equation is the crux the whole node exists to get right, and a
trusted encoder is a TCB breach. Component A is plumbing. Overall T1.
