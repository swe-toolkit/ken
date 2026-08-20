---
id: KERNEL-ELIM-SHAPE-NORMALIZE-DIVERGES
title: "Eliminator/recursor shape derivation over a proof-carrying inductive family diverges -- derive_recursive_shape and structured_lift normalize a constructor premise before the occurrence test, the elimination-time twin of LANG-CTOR-PREMISE-ELABORATION-DIVERGES"
status: active
owner: kernel
size: M
gate: none
depends_on: [LANG-CTOR-PREMISE-ELABORATION-DIVERGES]
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward, 2026-08-20, on the Architect's mechanism ruling evt_4z25rrhmfwxv6 for the derive_recursive_shape latent divergence the Adversary found (evt_3y17fz4jzxzne) while hunting kernel D2 [[LANG-CTOR-PREMISE-ELABORATION-DIVERGES]] at 80630656f. The Adversary's finding moved the Architect's prior 'no current consumer' filing (dec, LANG-CTOR thread) onto the V3-FO-CHECKER-SOUNDNESS critical path: eliminating over a proof-carrying inductive is exactly what fok_check_rule must do. TCB-edit authorized by the Steward under the operator-dark delegation, same envelope as LANG-CTOR D2 (a liveness fix that must preserve positivity soundness). Steward-filed per COORDINATION section 2. Coordinates are the Architect's, verified against origin/main with D2 landed."
---

## What is broken

**The D2 fix for [[LANG-CTOR-PREMISE-ELABORATION-DIVERGES]] closed the
DECLARATION-time divergence and left its ELIMINATION-time twin.** D2 switched
`check_pos_arg` from `normalize` to `whnf` and every occurrence guard in the
admission path from `occurs` to `occurs_delta`. That fix is landed, sound, and
**stays untouched** — this node does not reopen it.

**`derive_recursive_shape` (`inductive.rs`) still calls the identical diverging
operation D2 removed:** a full `normalize(env, &Context::new(), term)` run
*before* the occurrence test that would short-circuit it. It is reached only via
eliminator/recursor construction, which is lazy ("on use"), so the declaration
repro passes while the divergence sits latent until something ELIMINATES over
the family.

**Why it is on the critical path, not a filed-and-parked curiosity.** A
derivation checker like `fok_check_rule` — i.e. exactly
[[V3-FO-CHECKER-SOUNDNESS]] — must eliminate over the `FokDerivation` family.
The moment it does, `method_type` -> `recursive_shapes` -> `derive_recursive_shape`
runs `normalize(&Context::new(), premise)` on the constructor premise
`Equal <T> (<recursive fn> <free vars>) <value>` — the exact term the parent
issue MEASURED diverging (>10 GiB RSS, unbounded, @ `8d6d7d545`). So the
capability D3 of the parent claims is half-delivered: you can DECLARE a
proof-carrying inductive family, but eliminating over one re-triggers the
identical hang. `V3-FO-CHECKER-SOUNDNESS`'s `D1b` is held on this node.

**Severity: liveness, latent** (Adversary `evt_3y17fz4jzxzne`, Architect
`evt_4z25rrhmfwxv6`). Not a soundness hole in the shipped fix; a divergence that
blocks elimination. But see Part 2 below — the BUILDER carries one soundness
obligation the admission gate did not.

## Epistemic status

**A very strong read plus one measured anchor, not an end-to-end measurement** —
the same honest status the parent used for its own frame. Measured:
`normalize` diverges on this premise shape (parent issue, `8d6d7d545`). Read:
elimination reaches `derive_recursive_shape(that premise)` via
`method_type`/`recursive_shapes` with an unconditional normalize-first. The one
unrun link is forcing an elimination to observe the divergence; nobody has run it
(`COORDINATION §12` forbids the unbounded repro on the shared box). `D1`'s bounded
confirmation is what turns the read into a measurement.

## The mechanism -- Architect ruling `evt_4z25rrhmfwxv6`, transferred verbatim

**The two-part D2 discipline transfers, but `derive_recursive_shape` is a
BUILDER, not just an admission gate, so it carries one added soundness obligation
`check_pos_arg` did not.** Re-derive every site by NAME at pickup; the line
numbers below are the Architect's locators against origin/main with D2 landed and
decay silently.

### Part 1 -- whnf-per-level replaces the upfront normalize

- `inductive.rs:1259` `let normalized = normalize(env, &Context::new(), term);`
  -> `let head = whnf(env, &Context::new(), term);` (`:1260` `let term = &head;`).
- The function already recurses structurally (into body / domain / codomain /
  former-arguments), so each recursive call head-reduces its own subterm:
  per-level `whnf` gives the same layer-by-layer exposure `normalize` gave, but
  terminates — it stops at a stuck eliminator/neutral instead of recursing into
  its methods (the >10 GiB source). Same termination proof as `check_pos_arg`:
  K1 delta is acyclic; `whnf` halts at neutrals.

### Part 2 -- occurs_delta at all four occurrence guards, for a DIFFERENT reason

- `:1261` `if !occurs(d, term)`, `:1268` `domains...any(|domain| occurs(d, domain))`,
  `:1304` `arguments...any(|argument| occurs(d, argument))`, `:1333`
  `if occurs(d, &argument)` — every `occurs(d, x)` -> `occurs_delta(env, d, x)`.
- **The unsound direction here is under-REPORTING at the `DFree` classifier
  (`:1261`)**, not the under-rejection `check_pos_arg` guarded. A missed
  recursive occurrence classifies the argument `DFree`, drops its induction
  hypothesis, and the built eliminator is **UNSOUND** — an IH-free recursor over a
  recursive argument proves `False`. `occurs_delta` closes this exactly: it
  unfolds transparent `Const`s (delta-closure) AND recurses through
  `Term::children()`, which for `Term::Elim` includes `methods`. So a `D` hidden
  behind a definition OR inside a stuck eliminator's methods is seen ->
  classified `Recursive` -> routed to a clean error, never a dropped IH.
  Missing-`D` is the only unsound direction and `occurs_delta` cannot miss it.
- **Over-approximation is SAFE**, as in `check_pos_arg`: reporting `D` where
  deep-normalize+occurs would not costs at most a spurious `Recursive`
  classification hitting an error arm = a cleanly-rejected eliminator (liveness),
  never an unsound one. Same over-approximation lemma: reduction never
  synthesizes `IndFormer(d)`; delta-closure sees every definition `normalize`
  would unfold.

### The siblings -- IN SCOPE, because "enumerate sites, faithful conversion,
sibling survives" already bit this exact fix once

- **`structured_lift_type:1555` and `structured_lift_term:1697`** each
  `normalize(field_type)` to RE-EXPOSE the field's `Pi`/`Sigma`/`IndFormer`
  structure and walk it in lockstep with the shape skeleton — the identical
  shape-exposure divergence. Fixing only `derive_recursive_shape` moves the hang
  one function downstream. **RULED IN SCOPE:** same whnf-per-level swap, same
  soundness (pure shape-exposure), and the **SAME spine-grouping obligation** —
  here LOAD-BEARING, because `structured_lift_type` asserts
  `actual_domains.len() == domains.len()`, so if `whnf` exposes one `Pi`-layer at
  a time the implementer MUST `whnf`-iterate the `Pi`/app spine at each level so
  the arity matches the skeleton.

### The four proj sites -- a per-site determination, NOT a mechanical swap

- **`structured_lift_type:1601-1602` and `structured_lift_term:1759-1760`**
  `normalize(proj1/proj2(value))` are **NOT ruled a whnf swap.** They compute a
  VALUE component that is `subst0`'d into a codomain type and passed as the
  recursive `value` — value-computation feeding a substitution, not
  shape-exposure. The call is whether downstream relies on definitional equality
  (then under-reduction is sound — the built type stays convertible) or on a
  syntactic normal form (then a normal form is required, and if `proj`-of-neutral
  can drive `normalize` into a recursive-method divergence, that is a genuine
  FORK, not a swap). **The implementer reads the consumers of
  `first_value`/`second_value` and brings back a per-site determination; if any
  needs a divergence-capable normal form, route it back to the Architect — do NOT
  blanket-swap.**

### Empty context -- same K2c carry-forward as D2

`&Context::new()` is inert in K1 (`whnf`/`normalize` do not consult ctx in K1),
sound as written. But these functions go under binders (peeling `Pi`/`Sigma`), so
under K2c a real `Context` must be threaded through the recursion. **A
carry-forward contract note, not a K1 blocker.**

### Buildability caveat -- the fix guarantees termination and soundness, NOT that
V3-FO-CHECKER's eliminations SUCCEED

If the `FokDerivation` family's recursive shape is only exposed by deep
reduction (`whnf` leaves a stuck neutral -> clean reject), the fix rejects rather
than hangs — correct and conservative, but it blocks V3-FO-CHECKER at a NEW point
(the family needs a different shape-exposure strategy). **That is a new fork, not
this fix's failure**, and it is why `D1`'s confirmation must test BUILDABILITY,
not merely termination.

## Deliverables

**`D1` -- the repair, in `ken-kernel/src/inductive.rs`.** Apply Part 1
(whnf-per-level at `derive_recursive_shape` + the two `structured_lift` field_type
sites, with the spine-grouping obligation) and Part 2 (`occurs_delta` at the four
guards). Bring back a per-site determination on the four `proj` sites; route any
site needing a divergence-capable normal form to the Architect rather than
swapping it. **This is a TCB edit; the authorization is recorded in `origin`. The
mechanism is the Architect's and is transferred above verbatim — hand the
implementer the ruling, do not re-derive it.**

**`D2` -- the bounded confirmation, the D-gate** (Architect: YES, as this node's
gate, not a precondition to the mechanism ruling — the mechanism stands on the
over-approximation lemma + acyclic-delta termination). Three controls, all under
QA's `D1` RSS/time bound, **NEVER unbounded** (`§12`; the >10 GiB repro OOMs the
shared box):

- **(a) Termination.** Force `derive_recursive_shape` over the minimal divergent
  family — pre-fix hits the bound, post-fix terminates.
- **(b) Soundness discriminator** (the derive-shape analog of the parent's
  `AC-4` control 2). A family with a recursive occurrence hidden behind a
  transparent definition in a former-argument position, where occurs-after-whnf
  classifies `DFree` (drops the IH) but `occurs_delta` classifies `Recursive`. QA
  mutation-proves `occurs_delta` -> `occurs` flips the classification (drops the
  IH / changes method arity) — proving the delta-closure is load-bearing, not
  incidental.
- **(c) Buildability.** Confirm the minimal `FokDerivation` family's eliminator
  actually BUILDS, not just terminates. A clean rejection here is the new-fork
  signal above — **surface it, do not read it as success.**

## Acceptance criteria

- **`AC-1`. Termination control (a) passes** — the minimal divergent family's
  shape derivation, which hits the bound pre-fix, terminates post-fix, in
  ordinary CI under the bound.
- **`AC-2`. The soundness discriminator (b) holds and is load-bearing.** State
  explicitly which occurrence check still runs on the repaired path. The
  mutation `occurs_delta` -> `occurs` must flip the `DFree`/`Recursive`
  classification (drop the IH / change method arity) and redden a control —
  proving the delta-closure is not incidental. **A repair that under-reduces the
  shape exposure and drops a real IH fails here**; this is the criterion an
  over-narrow fix breaks, and it is the eliminator-soundness twin of the parent's
  `AC-4`.
- **`AC-3`. Zero new `trusted_base()` entries**, pinned before and after. A
  liveness/termination repair adds no trust surface. If the fix requires one,
  that is the signal it is the wrong fix.
- **`AC-4`. The affected library and targeted test configurations both compile**
  (`scripts/ken-cargo -p ken-kernel`, scoped; the workspace gate is CI's, never a
  local run).
- **`AC-5`. Buildability is reported, not assumed.** Control (c) states whether
  the minimal `FokDerivation` eliminator BUILDS or is cleanly REJECTED. A clean
  rejection is a complete result that opens the new shape-exposure fork — surface
  it to the Steward/Architect; do not tick this as success.
- **`AC-6`. The four `proj` sites carry a per-site determination**, each stating
  whether downstream relies on definitional equality (under-reduction sound) or a
  syntactic normal form (route to the Architect). An unexplained blanket swap of
  a `proj` site fails this.
- **`AC-7`. No-regression, in CI** (`COORDINATION §12`). Targeted local
  validation only. **Never run the diverging repro unbounded on the shared box.**

## Banned scope

- **Reopening or editing the landed D2 fix** in `check_pos_arg` /
  `check_pos_arg_normalized` / the admission-path occurrence guards. That fix is
  sound and stays. This node is the elimination-time twin, in the shape-derivation
  builders only.
- **Blanket-swapping the four `proj` sites** without the per-site determination.
- **Weakening positivity or eliminator soundness to make an elimination BUILD.**
  If the family needs deep-reduction shape exposure that `whnf` cannot give, that
  is a new fork to route, not a check to skip.
- **Restructuring `FokDerivation` to avoid the shape** — the V3-FO-CHECKER
  constraint requires its premises to be the checks `fok_check_rule` performs.
- **Proving anything, emitting FO `proved`, or touching `attempt_fo`,
  `fok_check_cert`, or the Rust reference checker.**

## Sequencing

**`active` at filing** — released to the kernel ring on the Architect's mechanism
ruling. `depends_on: [LANG-CTOR-PREMISE-ELABORATION-DIVERGES]` (merged); the
mechanism extends that fix's discipline to the elimination-time builders.

**`D1` and `D2` are one turn where practical** — the edit and its bounded
confirmation are tightly coupled (the discriminator (b) is what proves the
`occurs_delta` conversion is load-bearing). Hand back before merge for the
Architect's diff walk and QA's mutation proof, as LANG-CTOR D2 did.

**This unblocks [[V3-FO-CHECKER-SOUNDNESS]] `D1b`** (the eliminator-construction
path). The language ring is holding `D1b` on this fix and running `D1a`'s bounded
per-rule controls meanwhile, which construct/apply no eliminator — so `D1a` does
not wait on this node, but `D1b` does.
