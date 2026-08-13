# RT-4B-UNIQUENESS-GATE-REACH — count arrivals at exit 12 on the real C2 witness, before building anything that classifies them

**Owner: runtime. Size: S. Gate: none — inside 4b's already-authorized
observation gate (Architect, `evt_5gck3qg72xe37`). No exception needed.**

**Base: re-derive `origin/main` at cut time**, after
`RT-4B-OBSERVATION-FEATURE-GATE` (`4f55f012`) lands. Fixed inputs measured at
that candidate.

> ## RE-POINTED 2026-08-13. This frame previously named the wrong witness and a call site that does not exist.
>
> It was pulled back to `draft` before any work started (Architect
> `evt_6hfw027f43cgg`), because its `(4, 2, 0, 2, 1)` input was measured on
> **perturbed D2j comparators authored so that fusion does not form** — not on
> C2. The actual blocker underneath was that the observation was `#[cfg(test)]`
> inside `ken-runtime` while C2 drives through `ken-elaborator`, which links a
> build where those calls do not exist.
>
> **`RT-4B-OBSERVATION-FEATURE-GATE` removed that blocker** and nothing else:
> the observation is now reachable from `ken-elaborator` behind a default-off
> feature. **It did not measure C2** — its own artifact control runs on a
> purpose-built `R3_4B_IDENTITY_SOURCE`, and says so in its doc comment.
>
> **Three fixed inputs in the old frame are wrong. They are corrected below and
> each one changes what you build**, so do not work from a remembered version of
> this node.

## Fixed inputs

| fact | site |
|---|---|
| the gate itself | `planning/static_transition.rs:10124` — `fn fusion_unique_static_body_triple` |
| **the elimination call site — this is exit 12 of 13, and the one this node counts** | `:10367`, inside the enumeration loop; a `None` becomes `else { continue; }` |
| **a SECOND production call site, which this node must NOT count** | `:8927`, inside `rederive_fusion_key`; a `None` becomes a hard `planner_error("a fusion key's invocation edge does not re-resolve")` |
| the refusal that collapses both arms | `:10148-10151` — `if matching.len() != 1 { return Ok(None) }` |
| the enumeration loop the gate sits in | `:10297` — `for admitted in admitted_continuation_discoveries` |
| **ten `continue` exits precede the gate inside that loop** | `:10302, 10309, 10316, 10325, 10333, 10338, 10350, 10358, 10361, 10364` — the gate's own is `:10369` |
| the observer to report through | `D2fGateArrival`, `lowering/core.rs:564-604`, now under `cfg(any(test, feature = "r3-4b-observation"))` |
| the read mechanism, new and the reason this node is now possible | `ken_runtime::d2f_gate_observation_scope()`; `finish()` returns `Vec<D2fGateArrival>` |
| **the real witness, and what it actually reaches** | `crates/ken-elaborator/tests/r3_c2_source_mixed_branch.rs:435` — `prepare_native_program_sources` with `C2_MIXED_SOURCE`, which reaches *"an immutable pre-object preparation"* |
| **the identity control, which does NOT use C2** | same file — `r3_4b_observation_feature_is_native_artifact_identical`, driving `R3_4B_IDENTITY_SOURCE` |

## D1 — count arrivals at the ENUMERATION call site only

Record **how many candidates reach `fusion_unique_static_body_triple` from
`:10367`**, and report it through the existing `D2fGateArrival`.

**A counter at that call site. No signature change, no control-flow change, no
plan change, no second observer.** If it cannot be taken that way, that is a
hard stop — it means this is not the node the Architect authorized.

**One number in the inherited description is not confirmed here.** "Exit 12 of
13" comes from an earlier census over a differently-drawn enumeration; what is
measured at this SHA is **ten** `continue` exits ahead of the gate inside the
loop at `:10297`. Re-derive the ordinal yourself if the artifact states one —
do not carry "twelfth of thirteen" forward as measured. Nothing in this node
depends on the ordinal.

**The old frame called `:10367` the function's "sole call site". It is not, and
a counter placed inside the function body instead of at that call site is
therefore a different measurement.** `rederive_fusion_key` calls the same
function at `:8927` on an **already-formed key**, where a `None` is a hard
planner error rather than an elimination. Those two populations answer
different questions, and summing them produces a number that is not "candidates
eliminated at exit 12" while looking exactly like one.

## D2 — drive the real C2 witness and report

Drive `C2_MIXED_SOURCE` through `prepare_native_program_sources` with the
`r3-4b-observation` feature on, open the observation scope, and report the
count.

**This is the correction that matters most.** Every 4b measurement so far,
including the `(4, 2, 0, 2, 1)` this node was built on, was taken on in-crate
D2j fixtures — and the three unperturbed rows of that same assertion show the
planner **does** fuse. C2's count is what has never been measured.

## D3 — record what the feature's inertness now rests on

**Folded from an Adversary finding on `0902e62f` (`evt_qq5h94eq504j`),
triaged and evidence-corrected by the Steward. One sentence of doc, at the
feature's own declaration.**

**The inertness argument changed KIND when the gate widened, and nothing marks
the change.** Under `cfg(test)` the `walked` field **did not exist** in a
non-test build, so a structural-equality divergence in production was
impossible by construction — the compiler enforced it. Under
`cfg(any(test, feature = "r3-4b-observation"))` the field exists **whenever the
feature is on**, and the only thing keeping it out of a production build is
that no `Cargo.toml` enables it.

⇒ **That is a fact about configuration, not about code. It can change without
touching any file that states the invariant, and nothing goes red when it
does.**

State at the feature declaration: that inertness rests on no crate enabling
this feature on a non-dev dependency edge, and that the check is
`cargo tree -e features,no-dev`.

> ### The Adversary's precedent is REAL but its stated form is WRONG. Do not copy it.
>
> The finding named `crates/ken-cli/Cargo.toml:25` — which enables the sibling
> `px8-ds-test-support` feature — as *"a normal `[dependencies]` edge, not
> dev"*, and concluded it is *"compiled into the shipped CLI today."*
> **Measured: line 25 sits under `[dev-dependencies]`** (the section opens at
> line 24; the plain `[dependencies]` entry for `ken-runtime` is line 22 and
> carries no features). **It is the only enabling edge in the workspace.**
>
> ⇒ **`px8-ds-test-support` is NOT in the shipped CLI binary**, and writing
> that sentence into a durable doc would plant a false fact in exactly the
> place people go to check one.
>
> **The real effect is narrower and it still matters here.** A dev-dependency
> enabling a feature unifies it across the whole `cargo test` build graph, so
> any test build that includes `ken-cli` compiles `ken-runtime` **with**
> `px8-ds-test-support`. **That is precisely why AC-6's control needs
> `--no-default-features` and two separate target directories** — feature-on
> and feature-off artifacts cannot coexist in one compilation.
>
> **So: cite the mechanism, not the misread edge.** The precedent worth naming
> is that a test-support feature already leaks across this workspace's test
> graph — not that one ships.

## Acceptance criteria

- **AC-1 — the count is taken at `:10367`**, not inside the function body, not
  re-derived from a parallel walk, and not inferred from another field.
- **AC-2 — the count EXCLUDES the `rederive_fusion_key` call site**, and the
  artifact says how that exclusion is achieved. **A count that cannot
  distinguish the two call sites has not measured exit 12** — it has measured
  every use of one function.
- **AC-3 — the witness is `C2_MIXED_SOURCE` through
  `prepare_native_program_sources`**, named in the artifact by test and source
  constant. **The predecessor frame stated its witness in prose and never
  carried it into an AC, and an in-crate D2j implementation then satisfied
  every criterion while answering a different question.** This AC exists so
  that cannot recur.
- **AC-4 — the count is reported through `D2fGateArrival`.** No second channel
  and no parallel recorder — the Architect refused that proliferation when he
  ruled the observer already existed.
- **AC-5 — a mutation reds the exact new value, not a proxy.** **A recorded
  zero and an unrecorded zero are the same artifact without this**, which is
  the entire reason this node exists. If C2's count is zero, this control is
  the only thing separating your result from a counter that never ran.
- **AC-6 — the landed two-compilation identity control still passes**, unchanged,
  with the counter added: `r3_4b_observation_feature_is_native_artifact_identical`.
- **AC-7 — the report states the licensing limit in the artifact itself.** Reach
  attributes nothing: it cannot see which arm fired, and it must not claim to.
- **AC-8 — D3's sentence names the CONDITION and the CHECK**, not just the
  current state. "No crate enables this today" is the fact that expires;
  "inertness rests on no crate enabling it on a non-dev edge, check with
  `cargo tree -e features,no-dev`" is the one that stays useful. **Do not
  repeat the `ken-cli:25` claim as written** — verify the section yourself
  before citing that line at all.

> ### Do NOT write an AC requiring artifact identity on the C2 run. It is not available.
>
> The old AC-2 read *"enabled and disabled runs produce identical artifacts,
> proven by identity where identity is available."* On this witness it is not
> available: C2 reaches **an immutable pre-object preparation** and emits no
> native object, so there are no bytes to compare. That is why the landed
> control uses `R3_4B_IDENTITY_SOURCE` instead.
>
> **The identity question is already answered for the feature as a whole** — by
> two Cargo compilations with separate target directories, at AC-6. Your counter
> lives inside that same feature-gated region, so AC-6 covers it. **An AC
> demanding C2 bytes would be unsatisfiable, and the cheapest way to make an
> unsatisfiable AC pass is to quietly substitute the witness** — which is the
> exact failure that pulled this node back the first time.

## Pre-stated licensing — read BEFORE reporting

| outcome | what it licenses |
|---|---|
| **reach = 0 on C2** | **Exactly one thing: on this witness the eliminations are upstream of exit 12.** Fourteen routes narrow to eleven. It attributes **none** of them, does **not** reopen the fourteen-exit census, and does **not** change 4b's status — still exhausted, now with one route excluded rather than zero. **This is a real result, not a failed increment.** |
| **reach > 0 on C2** | `RT-4B-UNIQUENESS-GATE-ATTRIBUTION` has a subject and becomes lawful. |

> **Neither row licenses a finding against the planner, and neither licenses
> "the uniqueness gate ate our candidates."** That sentence has no true reading
> available from this node.

**If reach is zero, do not build the attribution increment. Bring the zero
instead** — the Architect will close that node rather than kick it.

**And a zero here does not carry back to the D2j measurements, in either
direction.** They are different witnesses; that is the whole point of the
re-pointing.

## Banned scope

- **Widening any function's return type.** That is the conditional successor.
- **Counting the `rederive_fusion_key` call site**, or unifying the two sites
  behind one counter.
- Attributing absence versus multiplicity; improving, relaxing or reordering
  either arm.
- Repairing the closure-boundary refusal, or making C2 emit an object. **C2
  stopping at pre-object preparation is a fixed input here, not a defect to
  fix.**
- The fourteen-exit census — ruled out, staying out.
- Enumeration, classifier, checker, marker, fusion-candidate, representation,
  ledger or closure-boundary repair. Gates 5 and 6 held; production unarmed.

## Hard stops — return to the Steward

- **The count cannot be taken at `:10367` without a signature or control-flow
  change.**
- **The two call sites cannot be distinguished** without changing the function.
- **The mutation cannot red the count specifically.**
- **C2 does not reach the observation at all** with the feature on. That would
  mean `RT-4B-OBSERVATION-FEATURE-GATE` did not deliver the reachability its
  successor was sequenced on, and it is a scope question rather than an edit.

## Sequencing and contention

Runtime, one lane, after `RT-4B-OBSERVATION-FEATURE-GATE` merges. Touches
`planning/static_transition.rs`, `lowering/core.rs` and the C2 test — the same
files that candidate touches, so it follows rather than runs beside it.

Local runs targeted only. **Never `--workspace`.** Note that AC-6's control
spawns two nested Cargo compilations and took 118 seconds in the predecessor;
budget for that rather than reading it as a hang.
