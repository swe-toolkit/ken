# `RT-PLANNER-KRET-GRAFTED-SPINE` — frame

**Owner:** runtime. **Size:** XL. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `ddbc803228dbb56f73df31a906ec1fe9a413910e`.

> ## RECUT 2026-09-19. THE LIVE DELIVERABLE IS `§0`. EVERYTHING FROM `§1` ON IS
> ## THE DIAGNOSIS THAT PRODUCED IT — HISTORY, NOT WORK.
>
> **The deliverable is still the two ignored rows, and a candidate whose roster
> contains no change under `crates/` has not advanced this node.** That banner
> survives the recut unchanged.
>
> **Size M → XL. The M was not a mis-estimate of the repair** — it was correct
> for the defect as understood at stop one. Nine hard stops are what established
> that **the defect is not at a site**, and a node cannot be sized for a
> conclusion its own investigation had not yet reached.

# 0. The recut — the live frame

Architect rulings `evt_3ynad2h315w1v` and `evt_64d51mf464fmw`, on the research
advisory `evt_5ny4tmskqx3n4`. Cut and sizing are the Steward's
(`evt_5whemb2pkgzsp`).

## Settled inputs — measured. Do not re-derive.

**1. Six dispatches, ONE resource token, residual zero** (runtime-leader
`evt_5dbc20bwkmym1`). **The under-release reading is DROPPED.** It was live for
one exchange: decomposed by disposition the aggregate reads 1 `Released`
against 3 expected, which would have meant two resources silently never
released. One measurement closed it rather than an argument. **The defect is
over-emission.**

**2. The 1-`Released`/5-`Closed` split is arrival order at one central host
authority**, not two families disagreeing about disposition. Inventory entry
10. `request_release` admits only a `Live` slot and refuses every later arrival
with `Closed`.

**3. Carrying an existing coordinate has already been tried and is not
enough.** At `bc4764fb8` `DeferredResponseRow` **already carries**
`owner_pair: Option<DeferredResponseOwnerPair>` with a live accessor, and
lowering already consumes it — **and the over-emission persists.** Entry 4's
discard was **necessary and measured NOT SUFFICIENT.** ⇒ What is missing is an
identity **no coordinate in the system currently expresses**. The owner pair is
emission *placement* and was never a release-obligation identity. **This is new
substrate, not a threading exercise**, and that is a settled result rather than
an open question.

**4. The mechanism is STATIC cross-family claim reconciliation, not a
compiler-level dynamic flag.** Ken already holds the dynamic authority and it is
working, so a compiler-level consumption flag would install **a second owner for
one property** — the defect shape being left, not a fix for it. And there is no
control-flow uncertainty for a flag to resolve: **both sites execute in the same
run**, one dispatch from `ken_static_response_0` and five from
`ken_continuation_context_1`. A flag would hide the over-emission at runtime
while leaving the planning defect in place, which is worse than the present
state because the present state is at least loud.

**5. A bare resource identity is not sufficient on its own.** It makes
disagreement *expressible*; the **consumption state** is what enforces
at-most-once. Rust's `MovePathIndex` plus drop flags and `resourcet`'s
`ReleaseKey` plus registry are one representation under two enforcement
regimes, and the advisory found no third shape.

## Deliverables

**`D0` — mint, carry, reconcile the observed two, and fail closed on the rest.**
Releasable on its own (`steward.md §4a`).

- One `ReleaseObligationId` **minted at bracket planning**, distinct from source
  origin and from every family key, associated with the runtime operand and its
  source provenance.
- **Carried** demand → row → emission claim → dispatch claim.
- The claims of `ken_static_response_0` and `ken_continuation_context_1`
  **reconciled in ONE table keyed by that id.** Family keys place code and
  never own disposition.
- **A release claim that carries no obligation id is a COMPILE ERROR.**

**`D1` — producer closure.** Enumerate **every family that can mint a release
claim** and reconcile the roster. The closure is over **producers**, not over
the two families that happened to be observed.

## Why the fail-closed default sits in `D0` and not in `D1`

Architect `evt_64d51mf464fmw`, and it is the sharpest thing in the recut.

`D0` reconciles two families and leaves three. **That is not a regression** —
those three behave exactly as they do today. But look at what `D0` does to the
incentive: **`D0` clears the ignored rows, and the ignored rows are the only
visible pressure for `D1`.** After `D0` the symptom is gone and the class is
still open, and **a silently unreconciled family looks identical to a
reconciled one.**

⇒ **Make the unenumerated family a compile error rather than a silent pass.**
Not a diagnostic, not a warning, not a table lookup that misses. The three
families `D0` does not reconcile then announce themselves the moment they mint
a claim, and `D1` becomes what it should be — enumerate the roster and
reconcile it — **with the gap loud in the meantime instead of resting on `D1`
being prioritised after its symptom has disappeared.**

## Acceptance criteria

**`AC-R1` — the two ignored rows clear, and the planner is what moved.**
*Control:* `linked_public_right_denial_preserves_exact_masks`
(`px7f_resource_native.rs:314`) goes green, **and** the native envelope matches
the interpreter's — `exit_status: 0` and exactly three events, per the `M2`
oracle differential. **Those three are the whole envelope — `FsOpen`,
`FsHandleMetadata`, and ONE `ResourceRelease` settlement for identity 1 — NOT
three releases.** Stated because the source carries three release expressions,
and an implementer holding that population reads "three events" as one per
demand; the Architect started `evt_4fgzxc5ba8x4q` reading it that way and it
inverted the answer on cardinality. The reference performs **one** settlement,
so three dispatches would be a five-event envelope and `AC-4` forbids it in
terms. **A candidate that reaches green by changing the fixture,
its assertions, or its ignore attribute has not advanced this node.** Do not
read `:348`'s pass as a second confirmation; `§1` records that it discriminates
nothing.

**`AC-R2` — the fail-closed default is real, and this is the control that can
fail.** *Control:* delete the obligation id from **one reconciled family's**
claim minting; **the build must fail to COMPILE**, restored byte-exact
afterwards. A diagnostic, a warning, a panic, or a runtime error **is not a
pass** — the whole point is that the gap is caught before anything runs. Run
this arm on a family `D0` actually reconciles; a mutation of an unreconciled
family tests nothing, because those are expected to error already.

**`AC-R3` — the five per-family injectivity guards survive unweakened.**
*Control:* the roster shows **none of the five deleted, merged, or relaxed**,
and each still reds on its own duplicate-identity input. See the does-not-touch
section: removing one is an explicit **non**-discharge.

**`AC-R4` — no new trust, and the host table is not the mechanism.** *Control:*
the added lines contain no `Axiom`, postulate, primitive, `Omega` carrier, or
kernel/TCB surface; **and the diff touches no resource-table code in
`ken-host`.** `ResourceTableV1` stays exactly what it is. A candidate whose
argument is that the host already enforces at-most-once has restated the
problem rather than fixed it.

## Stop condition

**If `D0` cannot be built without either weakening one of the five guards or
promoting the host table to the primary mechanism, stop and report.** Both are
the recut being discharged by substitution rather than by addition, and a report
that says so is worth more than a candidate that does it quietly.

## What this recut does NOT touch

This section is retained scope. Everything named here is already proved or
already correct, and none of it is a deliverable of this WP. A candidate that
changes any of it has exceeded the recut, not advanced it.

**The five per-family injectivity guards stay exactly as they are.** One pass
forward-declares five declaration families -- units, continuations, responses,
contexts, fusions -- each keyed by its own typed identity, and each already
fails closed when two descriptors claim one identity in its own key space.
Those guards are sound, they remain live, and they remain fail-closed. What
changes is only how they are READ: **they stop being read as the completeness
argument, they are not deleted.** They were never wrong; they answer "do two X
claim one X-identity?", and the property this WP installs -- exactly one
release per acquisition -- ranges over a population none of their keys names.
Local well-formedness and cross-family completeness are orthogonal, and this
WP adds the second without weakening the first.

**Removing, weakening, or merging any of those five guards is an explicit
NON-discharge of this WP.** If a candidate's roster shows one of them deleted,
that candidate has substituted the new mechanism for the old one instead of
adding it, and the reviewer should reject on that alone.

**The grafted-spine work already proved is retained.** The semantic mechanisms
established on this node stand; the owner-pair repair at the
`DeferredResponseRow` boundary stands and is not to be reverted. Carrying the
owner pair was NECESSARY and was measured NOT SUFFICIENT -- that is a settled
result, not an open question, and it is the reason this WP mints a new
identity rather than threading an existing coordinate.

**The narrowing is retained as a measured fact.** Of the eighteen compile-time
claims, SIXTEEN have no executing dispatch site: origin 190's thirteen in
Specialization(1) and origin 622's three in Predeclared(4). The executing
population is exactly origin 190's two Specialization(0) claims, and it maps
one-to-one onto the two observed dispatch sites. This WP does not re-open that
measurement and does not need to re-derive it.

**The host resource table is retained as the runtime backstop and is NOT the
mechanism.** `ResourceTableV1` admits only a `Live` slot, refuses every later
arrival with `Closed`, and settles `Released` once. It is working. It is the
first common authority and it sits too late to prevent the dispatches, which
is why this WP acts before dispatch. **This WP must not be discharged by
leaning on the host table**, and a candidate whose argument is that the host
already enforces at-most-once has restated the problem rather than fixed it.

## 1. Fixed inputs

    crates/ken-runtime/src/cranelift_backend/planning/static_transition/responses.rs
      exact_response_ret_identity          the defect site

    crates/ken-runtime/.../units.rs        LOWERING, added by amendment below
      define_static_response_owner_bodies  the SECOND statement of the same
                                           expectation (~:3430-3432 at
                                           1be846b2d)

Re-derive by **symbol**, not by line — the `:1379` coordinate in the ruling was
measured on another tree and coordinates are perishable.

**THE EXPECTATION IS STATED TWICE, AND ONLY ONE STATEMENT IS DERIVED**
(Architect `evt_1cwnksydmx1rz`). In `define_static_response_owner_bodies`,
two lines apart:

    check 1  the carrier's TAG          parameterised by k_ret_identity
    check 2  the carrier's FIELD COUNT  a hardcoded literal 1, which is
                                        ITree::Ret's arity and nothing else

**On these two rows**, whose composed K result is predicted to be `Vis`, a
correct derivation makes check 1 pass and check 2 fail. **Predicted, not
measured** — nobody has read what `emit_carrier_field_count` returns for a
`Vis` carrier. It is **not** a general property of the repair: wherever the
composed K's result is itself a `Ret`, the literal `1` is accidentally correct
and both checks pass. Both live in the same `require_i64` family, produce the
same trap and the same `UnclassifiedRuntimeTrap { terminal_value: -1 }`, and
therefore have an **identical external signature** — which is why a repair that
works **can** look exactly like a repair that did nothing. Do not attribute a
`-1` to a particular check without a probe.

**The defect, settled and not to be re-litigated:**

    exact_response_ret_identity reads ONE syntactic occurrence and composes
    nothing.
    42 §6.4 requires the grafted spine.

**The witnesses**, in `crates/ken-cli/tests/px7f_resource_native.rs`, owned by
`RT-PX7F-LINKED-PUBLIC-ROWS` until this node's repair lands:

    :314  linked_public_right_denial_preserves_exact_masks   DISCRIMINATING
    :348  linked_public_second_release_is_closed_and_the_handle_closes_once

`:348` **discriminates nothing** — its `K` operation equals its own origin's
operation by coincidence of constructor, which is value equality and not
identity. Do not read it as a second confirmation.

## 2. Deliverables

> **`D0` IS DISCHARGED** (Architect `evt_3t3ynhwbr8jv`, 2026-09-19). Both
> statements of the K-result expectation are now derived, at checkpoint
> `401be7c89367cb8c07aec0f24a982260b720df3d`. The discriminator returned
> positive — **the trap MOVED**, so the worker-body derivation was right and
> the field count was the second expectation. The planner's expectation no
> longer disagrees with the carrier the runtime produces: validation passes and
> the program executes end to end.
>
> **What remains is `D1`, and it is a different class of failure.**
> `right-denial` is one assertion from green: `exit_status` is asserted before
> the mask assertion and returns 82 where 0 is expected, so the mask assertion
> is not reached — and the trace carries exactly the
> `RightNotHeld { required: 32, held: 1 }` that assertion demands. **Do not
> re-open `D0`.**

- **`D0`** — the repair: derive `k_ret_identity` following `bind`'s grafting
  rather than the immediate occurrence, **and derive BOTH statements of the
  K-result expectation**. The planning-side tag derivation and the lowering-side
  field count are one deliverable: a literal `1` two lines below the check you
  parameterised **is** that derivation with the derivation missing. Finishing
  one and not the other leaves the row red with no correction upstream able to
  move it. **This is `D0` finished, not `D0` widened** — the expectation was
  written in two places by an author who had one answer for both.
> **`M1` AND `M2` ARE DISCHARGED** (runtime-implementer `evt_13acrd0jy9r7f`,
> ruled at `evt_5s3q6v3a53v7r`, 2026-09-19). They were the two measurements the
> Architect ordered — not repairs, and **not a fourth hard stop.** What remains
> is `M3`, then `D1`.

- **`M1` — THE RECORDED-OUTCOME COUNT. DISCHARGED: THE COUNT IS 1.** For
  `ResourceTraceIdentityV1(1)` exactly one settlement outcome is recorded. The
  native trace carries two `ResourceRelease` dispatch events, but only the first
  carries `Success(ResourceSettlement(... outcome: Released))`; the second is
  `Error(Resource(Closed))` with no resource binding and no settlement
  observation. ⇒ **`spec/60-security/62-authority.md:325` is SATISFIED.**

  > **THE TWO-WAY DISJUNCTION THIS DELIVERABLE ORIGINALLY CARRIED WAS
  > INCOMPLETE, AND THE ARCHITECT SAID SO OF THEIR OWN TEXT.** As written it
  > read: *"recorded outcomes == 1 ⇒ the duplicate dispatch is CONFORMING, a
  > cost-and-clarity finding."* **That keyed "defect" to the settlement clause
  > when the envelope clause is the one that binds here.** Verbatim from
  > `spec/40-runtime/42-evaluation.md`: `§6.8` judges a native backend by
  > *"requiring the same envelope and observations"*, and `§6.4`'s
  > retained-constraint table requires *"One structural event per `Vis` in
  > sequence"*, whose stated failure mode is *"Dropping or reordering an
  > interaction makes the recorded run false."* **Native envelope: 4 events.
  > Interpreter envelope: 3.** `§5`: the interpreter is right by definition.
  >
  > ⇒ **THE DUPLICATE DISPATCH IS A CONFORMANCE DEFECT AGAINST `§6.4` AND
  > `§6.8`.** It is not a settlement violation, and it is **not** a
  > cost-and-clarity finding.
  >
  > Recorded rather than deleted: a disjunction that was wrong is exactly what a
  > later reader re-derives if only the corrected conclusion survives. **One
  > satisfied clause is not conformance** — ask which other clauses the same
  > observation answers to before reading a pass as a disposition.

- **`M2` — THE ORACLE DIFFERENTIAL. DISCHARGED: THE INTERPRETER RETURNS 0.**
  The oracle is reachable. The same `RIGHT_NOT_HELD` source and seeded
  `held.bin`, through `ken_cli::run_program_effect_observation`, produced
  `terminal_error: None`, `terminal_exit: NormalReturn`, `exit_status: 0`, and
  exactly three events: `FsOpen` success, `FsHandleMetadata` with the exact
  `RightNotHeld { required: 32, held: 1 }`, and one successful `ResourceRelease`
  settlement for identity 1. The native run at the same checkpoint returns 82
  and carries a fourth event, `ResourceRelease -> Closed`. ⇒ Per `42 §5` the
  fixture's first assertion is **correct** and the blocker is a native defect.

- **`M3` — THE PRODUCER OF THE SECOND `ResourceRelease` DISPATCH.** Find what
  emits it and **what it is keyed on**. Report and stop; **no repair on that
  read.** This is also the read that decides the (A) filing trigger — `§7`,
  which carries the source facts to aim it with.

  **`release_if_live` IS `M3`'s SUBJECT, AND IT IS NOT CASE-SPECIFIC.**
  `crates/ken-elaborator/src/prelude.rs:2488-2501` — its body is **exactly one
  `Vis`** carrying `PrivateResourceRelease`, with a `Ret` continuation — and
  `:2518` `private_with_resource_after_open` calls it **exactly once**, in the
  `bind` continuation over `body resource`. ⇒ **It is the release that every
  `withResource` bracket runs, `right-denial`'s included.** It is not
  background and it is not a `DOUBLE_RELEASE` helper.

  **DO NOT SCOPE THE READ TO `release_if_live`'s BODY.** A second, near-identical
  emitter exists that this path does not call — `proc release` at `:2366`, whose
  `Vis` at `:2371` carries the same `PrivateResourceRelease a FsHandle resource`
  head and differs only in its continuation. **`M3` must check whether the
  native path reaches `prelude.rs:2371`**; looking only inside `release_if_live`
  would miss it.
- **`D1`** — the two rows above un-ignored and green, or a grounded statement of
  what still stops each one.

## 2a. The causal chain — REFUTED BY `M3`. IT IS TWO DEFECTS, NOT ONE.

**Architect `evt_12mfx5rxrp3g4` (2026-09-19), ruling on `M3`: the chain this
section asserted is WRONG, and the "one defect" unification is WITHDRAWN by its
own author.** The refuted text is kept at the end of this section, quoted and
marked, so the next reader does not re-derive it from scratch.

**THE DISCRIMINATOR THAT REFUTED IT.** One arm's Boolean flipped at a time,
everything else held: `BodyAndReleaseError |-> True` leaves the exit at 82;
`ResourceBracketOk |-> True` moves it to 0. **The constructor the native run
actually builds is `ResourceBracketOk`** — neither of the two the refuted chain
ranged over. The unread link named below was run, and it did not return either
candidate.

**THE CORRECTED CHAIN**, verified at source in
`crates/ken-elaborator/src/prelude.rs`:

    resource_settle_ok_error_for
        Closed |-> ResourceBracketOk e r value
    resource_settle_body_error_for
        Closed |-> ResourceBracketBodyError e r body_error

Reaching `ResourceBracketOk` means `resource_settle_result_for` took the **OK**
route — so the `body_result` it saw was `ResourceBodyOk`, **not**
`ResourceBodyErr (RightNotHeld 32 1)`.

⇒ **THE BODY ERROR WAS LOST BEFORE CLASSIFICATION.** The `RightNotHeld` is in
the trace and absent from the value classified. What reaches
`bracket_has_right_denial` is not "a bracket carrying a release error" — it is
**a bracket that has forgotten the body failed.**

**THE TWO DEFECTS, AND THE AC EACH ANSWERS TO:**

    duplicate dispatch  ->  the ENVELOPE defect  (AC-4)
    lost body_result    ->  the EXIT defect      (AC-1)

**Had the duplicate carried the correct `body_result`, `Closed` maps to
`ResourceBracketBodyError`, `right_masks` returns True, and the exit is 0.** So
**a duplicate release returning `Closed` is INVISIBLE in the exit status.** That
is why these are two defects and not two symptoms of one, and it is why `AC-1`
and `AC-4` are independent rather than one implying the other.

**WHAT SURVIVES THE REFUTATION.** `bracket_has_right_denial` **is still
correct** and is still not to be repaired — it faithfully classified the value
it was handed. What changed is *which value that was, and why*. The exit-mapping
filing call also still stays deferred: 82 remains the fixture's own literal at
`:148`, and no exit-status mapping defect is claimed on this row.

### 2a-i. "REMOVE THE SECOND DISPATCH" IS NOT A SUFFICIENT ACCEPTANCE

Remove the specialization copy from this route and the predeclared copy's
correct result is the one that propagates: the exit goes to 0, the envelope goes
to three, **both ACs go green and the lost-`body_result` defect is untouched —
merely no longer on this path.** That is turning the row green while leaving the
defect, which `AC-4`'s own rationale forbids. The positive per-property
criterion that closes it is **`AC-5`** in `§3`.

### 2a-ii. THE REFUTED TEXT, KEPT SO IT IS NOT RE-DERIVED

This stood as `§2a` and was Architect-approved. **It is false. Do not act on
it**; it is retained only because a corrected conclusion with the wrong
reasoning deleted invites the next reader to rebuild the wrong reasoning.

> `interpreter` one release, settles Released ⇒
> `ResourceBracketBodyError (RightNotHeld 32 1)` ⇒ `:136` `right_masks` True ⇒
> `:147` Success ⇒ 0. `native` — the second release returns Closed, so the
> bracket carries a release error AS WELL AS the body error ⇒
> `ResourceBracketBodyAndReleaseError` ⇒ `:138`, a hardcoded False ⇒ `:148`
> Failure 82 ⇒ 82.
>
> ONE UNREAD LINK: that the native bracket is
> `ResourceBracketBodyAndReleaseError` and not `ResourceBracketReleaseError`.
> Both map to `False`, so exit 82 alone does not discriminate them, and only
> `BodyAndReleaseError` can hold both the `RightNotHeld` and the `Closed`.

**How it failed is the discipline, and the first account of that was also
wrong.** It said the chain ranged over the two constructors that produce 82.
**It does not: three of the four produce 82 unconditionally.**
`bracket_has_right_denial` (`crates/ken-cli/tests/px7f_resource_native.rs:133`)
sends `ResourceBracketOk` (`:135`), `ResourceBracketReleaseError` (`:137`), and
`ResourceBracketBodyAndReleaseError` (`:138`) all to `False`, and `:148` sends
any `False` to `Failure 82`. Only `ResourceBracketBodyError` (`:136`) can reach
`Success`, and only when `right_masks` accepts its error. **The candidate set
was never "everything that produces 82",** so a reader applying the earlier
account would look for the wrong thing.

**What actually happened.** `ResourceBracketOk` was excluded by an *inference* —
the trace carries the `RightNotHeld`, therefore the bracket must carry the body
error. The unread link was then phrased as *"`BodyAndReleaseError` or
`ReleaseError`?"* — **a choice between the two survivors of that inference.** So
the check that was ordered sat *inside* the assumption that built the list, and
**no answer to it could have tested that assumption.** Both answers confirm
"the bracket carries a release error", which was the false part.

⇒ **WHEN YOU NAME AN UNREAD LINK, ASK WHETHER RESOLVING IT EITHER WAY LEAVES
YOUR INFERENCE STANDING. If both answers keep your reasoning intact, you have
named a detail, not a link.** This one had exactly that defect. The
one-arm-at-a-time flip found it because it varied each candidate
**independently** rather than asking which of two was taken — a discriminator
over the whole domain, not over the survivors.

## 3. Acceptance criteria

> **NO AC ON THIS NODE OR ANY SUCCESSOR MAY QUANTIFY OVER THE ABSENCE OF
> OCCURRENCE-KEYED DERIVATIONS** — not *"no occurrence-keyed derivation
> remains"*, and not any variant of it. **Such an AC cannot be failed.** By
> `bind_bind` at `≅` (`§7`), the only evidence against it is a witness needing a
> source bracketing nobody wrote, so its absence from the corpus is a fact about
> the corpus and not about the planner. An AC you cannot fail is not a gate, and
> this one would read as coverage for exactly the property that is now known not
> to be established. **ACs on this class are positive and per-property:**
> *"property P is derived from the grafted object at site S"* — one property at
> a time, each independently checkable. That is honest about what a per-site
> repair buys: it fixes P and says nothing about Q. Architect
> `evt_5gja8y3y23nt4`.

- **`AC-1` — `right-denial` GREEN IN CI.** The discriminating witness passes.
  *(Control: the row FAILS at this node's base — run it before the repair and
  record the refusal. An AC met at the base measures nothing.)*
- **`AC-2` — THE REPAIR IS DERIVATIONAL, NOT A SPECIAL CASE.** No branch keyed
  on an operation identity, an arena ordinal, a fixture name, or a bracket
  shape.
  *(Control: state which construct in the repair supplies the composed
  continuation, and name one program NOT in the px7f family whose planning path
  goes through the changed code. If no such program exists, say so — that is a
  finding about reach, and it is a result.)*
- **`AC-3` — NO REGRESSION.** Green in CI, never a local `--workspace` run
  (`COORDINATION §12`). Local work is `scripts/ken-cargo -p ken-runtime` and
  `-p ken-cli --test px7f_resource_native`, nothing wider.
- **`AC-4` — THE ACCEPTANCE EVIDENCE IS THE ENVELOPE, NOT THE EXIT STATUS.**
  The native and interpreter envelopes must match: **three events, not four.**
  State the envelope match as the evidence.
  *Rationale, and two false passes the Architect ruled out in advance*
  (`evt_5s3q6v3a53v7r`) — **both turn `AC-1` green while leaving the defect:**

  - making the second release **settle successfully** leaves 4 events against
    the reference's 3, and silently re-decides the bracket constructor as well;
  - **suppressing the event** while still dispatching matches the envelope by
    hiding a performed effect, which is the same falsehood `§6.4` names.

  **The repair is to NOT DISPATCH it.**

  ⇒ **`AC-1` is necessary and by itself NOT SUFFICIENT**, which is exactly what
  those two false passes mean. That half stands (Architect `evt_5tth40ek8zv60`).

  **THE OTHER DIRECTION IS REFUTED — `AC-4` DOES NOT IMPLY `AC-1`.** This
  paragraph previously read *"under `§2a`'s chain `AC-4` IMPLIES `AC-1` — no
  fourth event means `BodyError`, so `right_masks` is True and the exit is 0."*
  **That implication rested entirely on the chain `M3` refuted**
  (`evt_12mfx5rxrp3g4`, `§2a`). The unread link it hedged against did not
  resolve to `BodyAndReleaseError`; it resolved to `ResourceBracketOk`, and the
  body error is lost before classification. ⇒ **the two ACs answer to two
  independent defects** — the envelope defect and the exit defect — and neither
  implies the other in either direction. Keep both, and that is now the reason.
- **`AC-5` — THE TWO EMISSION COPIES MUST AGREE, AS A PROPERTY OF THE
  EMISSION.** Architect `evt_12mfx5rxrp3g4`, verbatim: **the specialization copy
  and the predeclared copy of `effect_origin: 190` must compute the SAME bracket
  result from the same `body_result`** — or the specialization copy must be shown
  not to be emitted for this seat at all. **State which.**
  *(Control: show it as a property of the emission, not as a passing fixture. A
  green `px7f_resource_native` row does not discharge this — see `§2a-i`:
  deleting the specialization copy makes `AC-1` and `AC-4` both green while
  leaving the lost-`body_result` defect intact and merely off this path. The
  discharging artifact names the two copies and the result each computes, or
  names the emission condition under which only one exists.)*

## 4. Hard stops — report, do not work around

- A repair that reaches outside `ken-runtime` planning, **except for the one
  emission call named in `§1`**: in `define_static_response_owner_bodies`, the
  literal `1` in `require_i64(ret_fields, 1)` becomes the arity of the
  constructor `k_ret_identity` names. Nothing else in emission is in scope, and
  needing a second site is a fresh hard stop.
  *(Steward scope ruling `evt_88t2h5g77dkg`, 2026-09-19, on the Architect's
  recommendation. The original bullet was this frame's own prose, not a spec
  rule — and it made `D0` unachievable, because the expectation is stated in two
  files and only one is reachable from planning. A frame whose deliverable
  cannot be reached inside its own scope is a defective frame. The implementer
  was correct to stop rather than reach: the bullet fired exactly as written.)*
- A row that needs a **new** `#[ignore]` anywhere to make progress.
- The repair lands and `right-denial` still fails — **stop and report, but do
  NOT report it as "the mechanism statement is incomplete" until you have
  distinguished which of two findings it is.** Both outrank finishing the node;
  they have opposite consequences.

      the failure is the SAME CLASS      the diagnosis is incomplete
      the failure CHANGED CLASS          the diagnosis was RIGHT and the
                                         repaired check was MASKING a second
                                         defect downstream of it

  *(Repaired 2026-09-19 after this bullet fired and its stated finding was
  wrong. As originally written it keyed on "still fails" — **the same symbol
  for "the diagnosis was wrong" and "the diagnosis was right and was hiding
  something"** — so it could not distinguish them, and it asserted the first.
  Here it was the second: an expectation mismatch became a semantic outcome,
  the trace acquired the exact `RightNotHeld { required: 32, held: 1 }` the row
  demands, and the program began executing end to end. **A fail-closed
  validation hides everything downstream of itself; removing one NECESSARILY
  reveals whatever was behind it, and revealing a downstream defect is not
  evidence the diagnosis was incomplete.** Architect `evt_3t3ynhwbr8jv`. The
  frame defect was the Steward's.)*
## 4a. The mapping check — RUN AND PASSED before this node started

**Nothing is owed here. This section records a discharged check rather than
asking for one, and it is kept precisely because it passed.**

The Architect's operation-to-ordinal mapping rested on one link **they named
rather than glossed**: that `declare_inductive` allocates constructor ids
sequentially in declaration order. They did not read the allocator, and said so.

**The check, run by runtime-implementer at
`29e6b21e1f8c5b80557428693a66eb61c19ea4c4`** — count the `FSOp`-parented entries
in the `names` buffer already dumped. Predicted 21: ten carrying real spellings
and eleven carrying `ctor_NNN`, the eleven strictly above every named one.
Measured:

    FSOp total = 21    named = 10    unnamed = 11

    named     AppendFile ChangeMode CreateDirectory Metadata ReadDirectory
              ReadFile RemoveDirectory RemoveFile Rename WriteFile
    unnamed   ctor_541 .. ctor_551, contiguous

**Exactly the predicted partition**, and the named ten are exactly the public FS
operations. ⇒ The window's position is fixed **by the artifact**, not by a read
of the prelude, and `ctor_543 = PrivateResourceRelease` stands on a measurement.

**Why this section survives having passed.** A discharged check deleted from a
frame leaves the mapping looking *assumed* to every later reader, and the next
seat cannot tell the difference between a thing that was verified and a thing
nobody thought to question. **Keep passed checks; mark them passed.** The
prediction is recorded alongside the measurement so the agreement is legible as
an agreement rather than as a lone number.

**The ruling never depended on this.** `§1`'s two sentences are normative and
would stand whatever the buffer said. What the check protected is the
*row-level* reading in `§1` of the node file — **which row departs** — and that
reading is now measured: `right-denial`.

## 4b. Reporting and reasoning disciplines — NOT hard stops

These do not fire and do not count. Violating one is an error in the report,
not a condition that halts the node, and none of them increments the hard-stop
count in `§5`.

- **Do not report a red row as "still red" when it is one assertion from
  green.** State which assertion fails, which are not reached, and whether the
  unreached ones would pass on the trace you have. `D1` closes on "un-ignored
  and green, or a grounded statement of what still stops each one", and those
  are different statements.
- **Do not build a mechanism statement on a field you have not shown was
  written.** `terminal_exit: ReturnedError` and `terminal_error: None` are the
  `EffectObservation` constructor's INITIAL VALUES (`ken-runtime`
  `native_effect_v1.rs`); they carry no information unless something on the
  path overwrites them. A fail-closed default and a measurement are the same
  symbol at the point of reading — the same shape as the `-1` this node already
  learned not to attribute.
- **Do not chase a salient symptom before establishing it is anomalous — and
  establish it ON THE FIXTURE IN FRONT OF YOU.** Chasing the salient-but-normal
  is this node's own recurring failure: span length, then continuation shape,
  then the arena ordinals.

  **THE INSTANCE THIS BULLET ORIGINALLY NAMED HAS BEEN REFUTED, AND HOW IT
  FAILED IS THE DISCIPLINE.** It read: *"a second `ResourceRelease` returning
  `Closed` is the DESIGNED behaviour the sibling row
  `linked_public_second_release_is_closed_and_the_handle_closes_once` exists to
  test."* **That sibling row runs a DIFFERENT PROGRAM.** `right-denial` runs
  `RIGHT_NOT_HELD` (`px7f_resource_native.rs:316`, const `:92`); the sibling
  runs `DOUBLE_RELEASE` (`:350`, const `:165`), which releases twice on purpose.
  `RIGHT_NOT_HELD` has no second source-level release at all — the interpreter
  emits three events for it (`M2`, `§2`). **The bullet imported one program's
  designed behaviour onto another program.**

  ⇒ **On `right-denial` the second dispatch is the DEFECT, not the
  distraction** (`§2`'s `M1` blockquote, `§2a`, `AC-4`). **Nothing in this
  bullet licenses deferring or softening `M3`.**

## 5. Symptom inventory — ARMED AT FILING

**Seeded at framing this time.** The parent node reached two hard stops before
anyone seeded one, and reconstructing it from the thread was recovery work
`§1b` exists to make unnecessary: the count is re-derivable from a thread and a
pattern across stops is not, and the thread is the first thing a compaction
discards.

```text
SYMPTOM INVENTORY (Architect appends one line per hard-stop; never rewritten)
NEXT PREDICATE CHECK = 3rd entry, then 6th, 9th, ...

1. the K-result expectation is stated TWICE in the emission and only one
   statement is derived (tag parameterised by k_ret_identity, field count
   a literal) -- keyed on a constructor's ARITY hardcoded as the Ret
   assumption
2. the fail-closed validation was masking a second defect -- with the
   expectation repaired the program executes and returns exit 82 where 0
   is asserted -- keyed on a fail-closed check hiding every downstream
   state from observation
3. 82 is the program's own value: after_right_outer returns
   host_exit (Failure 82) only when bracket_has_right_denial is FALSE,
   while the trace carries the exact RightNotHeld -- and the native run
   performs TWO PrivateResourceRelease dispatches where the reference
   interpreter performs ONE (M2) -- keyed, MEASURED AT THE PRODUCER BY
   M3 AND NO LONGER ON THE ARCHITECT'S READING, on SYNTACTIC CONTAINMENT
   of the seat's source occurrence inside a continuation case body's
   occurrence: inline_synthesized_seat_emission_owners
   (static_transition.rs:743) pushes a Specialization owner per case body
   satisfying occurrence_subtree_contains (occurrences.rs:284), a pure
   walk over source origin ids. That key is NOT a function of the grafted
   object -- re-bracketing under bind_bind moves a Vis into or out of a
   given case body while producing the identical grafted tree.
```

> **ENTRY 3 STANDS; THAT WORDING NARROWS WHAT IT CLAIMS RATHER THAN
> WITHDRAWING IT.** As first written it said *"one source-level bracket release
> path produced TWO runtime dispatches"* — **which is exactly what `M3` is
> assigned to find out**, and which `§7` of this same frame calls open. **The
> two standards are deliberately different:** an inventory entry is a reading,
> meant to be cheap and revisable; the (A) trigger in `§7` needs the keying
> **established**, because it authorizes a structural change. Stating the
> reading as fact collapsed them.

Entries 4, 5 and 6, appended verbatim from the Architect at
`evt_734ccm2xzxp5r`. Entries 1-3 above are deliberately not rewritten.

```
4. phase A derives the release's (base owner, caller emission owner) pair
   and the conversion to DeferredResponseRow DROPS BOTH, retaining only
   origins and counts -- so downstream reconstructs ownership from local
   occurrence containment and effect 190 dispatches TWICE, once under the
   predeclared / specialization-0 copies while constructing the Vis and
   again when specialization 1 processes that same Vis -- keyed on a source
   occurrence standing for the runtime object, the owner relation having
   been discarded at the row boundary
5. with the owner pair retained the dispatch is correct and single, and the
   RETURN CONTRACT then refuses: k_ret_identity is derived from
   continuation_result_origins(worker body 568) as Vis arity 2, while the
   locally driven path consumes Vis 555, dispatches, lowers exact K body
   542 and returns ITree::Ret arity 1 -- keyed on the SOURCE worker body's
   occurrences naming the result of the DRIVEN object. This is entry 1
   recurring at a second consumer after D0 repaired it at the first.
6. deriving the driven result identity from the exact K body and
   propagating it to the matching response row finds NO row at
   StaticOriginId(503): the pairing resolves the driven object through
   specialized.base_owner == Specialization(k_specialization) matched
   against the deferred row's caller emission owner, a coordinate that is
   not a function of the driven object, and at 503 the correspondence is
   empty -- keyed on an owner coordinate standing for the driven object
```

> **WHAT ENTRY 6 DOES NOT CLAIM, kept so the frame does not overstate it.**
> The zero-match is measured, and *that the pairing key is not a function of
> the driven object* is grounded at `responses.rs:2984` and `:4359` plus the
> two accessor docs at `continuations.rs:1489` and `:1508`. **NOT established:
> that the intended pairing relation is wrong** — only that it resolves nothing
> at 503. The entry's keying clause is held to what those citations carry.

Entries 7, 8, 9 and 10, appended verbatim from the Architect at
`evt_3kx5y2nwqbxq9`, numbered against this file's sequence. Entries 1-6 above
are deliberately not rewritten.

```
7. the live-claim census counts seats claimed PER ARM and pruned
   afterwards, so 13 claims in Specialization(1) is not 13 duplications
   and the magnitudes cannot be read off it -- keyed on a count whose
   UNIT is the arm and whose reader took it for the duplication, the
   pruning that separates the two happening after the count is taken
8. StaticOriginId is erased before native host dispatch and the runtime
   trace, ABI and call-site map carry no origin, so the coordinate the
   attribution question was posed in does not exist at the point the
   observation is made -- keyed on an ORIGIN coordinate that does not
   survive to the observation point, where the OWNER does, as a function
   symbol
9. the owner label was read as naming an emitted function -- five
   declaration families in one pass (units, continuations, responses,
   contexts, fusions) each emit under the SAME enclosing specialization
   and are keyed by five disjoint typed identities (UnitBundle
   :303-327), so Specialization(0) is many-to-one onto emitted functions
   -- keyed on a LABEL taken for an IDENTITY of a population it does not
   range over
10. the two executing dispatch sites are ken_static_response_0 (one
   dispatch) and ken_continuation_context_1 (five), residual zero. The
   one Released / five Closed split is NOT the two families disagreeing
   about disposition: request_release admits only a Live slot and
   refuses every later arrival with Closed, so the split is ARRIVAL
   ORDER AT ONE CENTRAL HOST AUTHORITY. The host converts duplicate
   compiler dispatches into one release plus five refusals -- it is
   working, and it is the FIRST common authority, which is too late to
   prevent the dispatches -- keyed on the absence of any PRE-DISPATCH
   obligation identity shared by the emission families
11. three release demands share one effect origin (190) and are told apart
    only by response provenance, so the implementer could not tell whether
    identity is per-resource or per-demand and had to stop -- keyed on the
    demand's provenance rather than on the obligation the frame mints
```

> **THE TWO COUNTERS HAVE SEPARATED, AND THEY STAY SEPARATED. `§1a` FOLLOWS THE
> STOP COUNT; `§1b` FOLLOWS THE ENTRY COUNT.** Architect ruling
> `evt_269dk8msteb89`. At that ruling, nine hard stops had produced ten symptom
> entries: a single stop can yield more than one symptom line. `§5` records
> that the two
> coincided 1:1 *"today"* — **that was the coincidence, never the design.**
>
> Derived from what each trigger is *for*, not from the coincidence. **`§1a`
> asks whether an unaided pair has run out of road**; its subject is the round —
> build, wall, hard stop, rule again — so **that question ranges over stops**.
> **`§1b` asks whether the accumulated symptoms share a predicate**; its subject
> is the symptom, so a stop yielding two symptoms contributes two data points
> and a purely procedural stop contributes none. **That question ranges over
> entries.**
>
> ⇒ **Next `§1a` check at stop 12. Next `§1b` check at entry 12.** Different
> moments, correctly so.
>
> **This is an instance of the predicate the Architect has been ruling on all
> evening: each counter keyed on the population its own question ranges over.**
> Recorded with their own note that **entries lead stops, so keying `§1b` to
> entries makes it fire sooner and costs them more** — that is not the argument
> for it, the population is, but the ruling relieves them of nothing.

**Hard-stop count on this WP: 10; symptom entries: 11.** **The parent node's
count of 2 does NOT
carry** — different WP, different question. `§1a` fired at three
(`evt_7d3h7mtff5acd`), **re-fired at six** (`evt_3t5nq11ernjfa`), and has
**re-fired at nine**, holding the ruling on the next locus and calling
Research. **That ninth trigger is DISCHARGED** by the research advisory
(`evt_5ny4tmskqx3n4`) and the ruling it produced (`evt_3ynad2h315w1v`); the
hold on the locus is released. `§1b` fired at three (`evt_1mv0phbj0zcn7`), was
re-run at six (`evt_734ccm2xzxp5r`), and was re-run at ten entries
(`evt_3kx5y2nwqbxq9`).

> **THE COUNT IS 6 AND NOT 5, AND THE ADJUDICATION IS THE ARCHITECT'S.** The
> implementer reported "count remains 5" and that reading is defensible — they
> stopped as directed, and measurements taken inside a stop are not new stops.
> The Architect overrode it as their own trigger. **Recorded here because the
> number is close and the next reader must not inherit the losing side of it
> without knowing there was one.**

### The `§1b` answer, and a pre-commitment the Architect declined to discharge

Entries **1 and 3** share a predicate: **a source-level occurrence is being used
to name a runtime object, and grafting makes that correspondence not
one-to-one.** **Entry 2 is NOT an instance** and was deliberately not forced in
— it says a fail-closed check hid everything downstream of itself, which is a
statement about what we could OBSERVE, not about what was WRONG. *"Forcing it in
would give a three-for-three that reads stronger and means less."*

**INVENTORY ENTRY 3 STANDS, WITH ITS EVIDENCE UPGRADED** from the Architect's
reading to an oracle differential (`evt_5s3q6v3a53v7r`).

The Architect had pre-committed: *"if `M1` returns one recorded outcome, the
duplicate dispatch is conforming and is NOT an instance of the predicate."*
**`M1` returned one, and they declined to draw that consequence** — stating the
refusal out loud rather than quietly keeping the entry, because the convenient
move was to take `M1` and shed half their own predicate.

**The defect in the pre-commitment: it tied instance-hood to conformance with a
single clause.** The predicate is a claim about the planner's KEYING, not about
which spec clause the result happens to trip. And `M2` arrived in the same
breath and bears on the keying directly — **the reference spine has one `Vis`
where the native path performs two.** That is *better* evidence for entry 3 than
existed when the amendment was written, not worse.

> **A pre-commitment discharged mechanically against evidence it did not
> contemplate is worse than no pre-commitment.** The guard is DIRECTION:
> declining was legitimate here only because honouring it was the CONVENIENT
> move — it would have shed half the Architect's own predicate. **Where
> honouring a pre-commitment is the INCONVENIENT move, unanticipated evidence
> is not a licence to drop it.**

**The associativity theorem in `§7` is independent of all of this** and does not
need entry 3 at all.

### The `§1b` support set restated at nine, and what it still lacks

The Architect's `§1b` re-run at nine (`evt_3kx5y2nwqbxq9`), against **this
file** as the artifact. An earlier statement of the support set was measured
against `wp/RT-PLANNER-KRET-GRAFTED-SPINE-INVENTORY-45`, whose wording for
entries 4 and 5 is superseded; that branch is **behind** this file and is not
the artifact.

The predicate: **the release's identity is never CARRIED. At each boundary it
is dropped and reconstructed from whatever coordinate is locally available, and
every such coordinate ranges over a population coarser than the resource
itself.**

- **Fitting: 3, 4, 5, 6, 7, 8, 9, 10** — eight of ten. Entry 5 ends *"keyed on
  the SOURCE worker body's occurrences naming the result of the DRIVEN
  object"*, a source-syntax coordinate standing in for a runtime object's
  identity, and entry 6 is *"keyed on an owner coordinate standing for the
  driven object"*. Both fit; the earlier "5 does not fit" was read off the
  superseded wording.
- **Not fitting: 1 and 2.** Entry 1 is a constructor's arity hardcoded as a
  literal beside a derived one. Entry 2 is a fail-closed check hiding
  downstream state — an instance of the Architect's *other* named predicate,
  observation gated by its own subject.

> **EIGHT OF TEN IS THE COUNT; ONE SURVIVED PREDICTION IS WHAT IT DOES NOT YET
> HAVE.** Recorded in the Architect's own words because the correction made
> their own claim *stronger*, which is the direction that earns more scrutiny
> rather than less. Two checks they ran on it instead of banking it: the
> predicate **discriminates** — entries 1 and 2 are real, are on this WP, and
> do not fit; and it has **not yet made a prediction that could have failed**.
> The predicate was named before the 1+5 attribution and listed "any other
> split" as a stop, so the disposition-disjoint result of entry 10 is
> *consistent with* it but was **not predicted by** it. The strengthening came
> from re-reading corrected artifact text, **which is weaker evidence than a
> survived prediction.**

**TWO ACTS FIRE AT THREE, BOTH THE ARCHITECT'S, BOTH BEFORE THEY RULE**
(`evt_3t3ynhwbr8jv`). **They are driven by two different counters and the
frame must not be what teaches someone they are one:** `§1a` fires at the third
**HARD STOP** — consecutive hard stops on the same design question — and they
hold the ruling and call research in-thread with the WP, the thread, the
hard-stop event ids, the clean checkpoint SHA and the exact question. `§1b`
fires at the third **INVENTORY ENTRY** and they answer in one paragraph whether
the three entries share a predicate. **The two counters coincide on this node
today, 1:1, and that is a fact about this node and not about the counters** — a
stop that produces no entry, or an entry appended for something other than a
stop, separates them. **Reporting a third stop is
therefore not a round-trip like the first two. Expect a pause and do not read
it as a stall.** The predicate question is deliberately NOT answered at two —
answering early is how it becomes a formality.

## 5a. A known boundary — RECORD IT, do not work on it

`k_ret_identity` is a **single** `ConstructorIdentity`, so the plan assumes the
composed K's result constructor is statically unique. **Under grafting that
assumption can fail:** `bind (f r) k` may reduce to `Ret` on one response arm
and `Vis` on another. The checkpoint code already fails closed on it ("more
than one constructor result"), which is the right behaviour.

This is here so the next reader meets it as a known boundary rather than as a
surprise. **Not this node's work unless a row reaches it** (Architect,
`evt_1cwnksydmx1rz`).

## 6. Contention

The repair is in `ken-runtime` planning; the rows are in `ken-cli` tests and are
**owned by `RT-PX7F-LINKED-PUBLIC-ROWS` until this node clears them**. That node
may relabel those rows to point here before this node starts. Coordinate through
the runtime-leader rather than both editing
`crates/ken-cli/tests/px7f_resource_native.rs`.

`RT-COMPOSED-RETURN-PRODUCER-SINK-COLOCATION` is **not** this subject — the
Architect read its row and it is producer/sink placement at the true
`StaticWorker` producer. Same family, different question.

## 7. The (A)/(B) fork — RULED. TRIGGER MET AT `M3`; (A) IS ORDERED.

Architect ruling `evt_5gja8y3y23nt4` on research advisory `evt_3yz2ek90jrnkt`,
2026-09-19. **The fork was decided by a theorem in the formalism the spec
already cites, not by precedent.**

**THE THEOREM, WHICH IS A STATEMENT ABOUT THE METHOD AND NOT ABOUT THE THREE
SITES HIT SO FAR.** Spec `42 §6.4` fixes the spine by ITree grafting, and
`bind_bind` holds at `≅` — **strong bisimulation, not up to taus**. So **two
different source bracketings produce the SAME grafted object**, and therefore
*"the source occurrence that textually produced this continuation"* **is not a
function of the grafted object.** It is not a fragile key and not a key with
exceptions: it is undefined on the domain the semantics quantifies over.

    (B)  derive each property from a worker-body / closure lookup AT EACH SITE
    (A)  represent the grafted spine explicitly in the IR, so that deriving a
         property from a source occurrence is not expressible

**(B) IS DEAD AS A CLOSURE AND ALIVE AS A REPAIR.** Per-site derivation from
the object is correct and is exactly what `D0` did — it is what makes the
property a function of the object again. What the theorem kills is the claim
that finishing a list of such sites finishes the class: a failing witness needs
a particular source bracketing, so the enumeration could never be shown closed.
**The Architect has WITHDRAWN the enumeration half of their own `§1b` recut on
this basis, naming it a withdrawal rather than quietly replacing it. No node is
to be filed around it.**

**(A) is the only known closure in this space** — a closure by construction
rather than a sweep. GHC's **join points** are the precedent in shape: the IR
represents the thing and Lint enforces the invariant, rather than each analysis
re-deriving it. **The ruling takes the associativity argument and NOT the
history** — the advisory flagged the (B)-to-(A) narrative as its own
characterisation rather than something it verified, and the ruling does not
need it.

**THE TRIGGER FOR FILING (A), stated so it is not a matter of mood:** a **live
occurrence-keyed derivation that the immediate repair does not reach.** When it
is met, **(A) gets its own node, framed by the Architect and priced before it is
scoped.** The Steward files it when the Architect says the condition is met,
**and not before.**

**STATUS AFTER `M3`: THE TRIGGER IS MET, ESTABLISHED AT THE PRODUCER**
(Architect `evt_12mfx5rxrp3g4`, 2026-09-19). The key is **syntactic containment
of the seat's source occurrence inside a continuation case body's occurrence**:

    static_transition.rs:743  inline_synthesized_seat_emission_owners
                              pushes ContinuationEmissionOwner::Specialization
                              for each case body where
    occurrences.rs:284        occurrence_subtree_contains(plan, root, needle)
                              -- a pure syntactic walk over source origin ids

Under `bind_bind` at `≅`, re-bracketing moves a `Vis` into or out of a given
continuation's case body **while producing the identical grafted tree**. So
*"which specializations contain this seat"* **is not a function of the grafted
object** — the met arm of the discriminator below, reached from the producer
rather than by inference. ⇒ **(A) is ordered. The Architect frames and prices
it; the Steward files the node.**

> **THE DISCHARGE ALMOST WENT THE OTHER WAY, USING THE REFUTED TABLE.** `M3`'s
> report sorted this same measured fact with the **occurrence-counting** table
> the Architect had already removed — *"not two source occurrences naming one
> runtime object, therefore the second-distinct-emission-site / local-defect
> arm."* That is the elimination the replacement exists to prevent, and it
> lands on the arm that relieves the trigger's owner of filing (A). **The
> trigger-met fact had been measured correctly and was sorted by the wrong
> criterion.** Keep the two apart: a measurement is not its classification.

**`M3` WAS THE ONE READ THAT DECIDED IT** — what emits the second
`ResourceRelease` on the native path, and **on what key.** It has been run.

    the second dispatch is derived from something that is NOT a function of
    the grafted object -- a source or continuation occurrence, a syntactic
    site, any key a different bracketing would change
        => the trigger IS met; the Architect frames and prices (A)

    the second dispatch is derived from the grafted object, or from anything
    that IS a function of it, and is simply wrong
        => a local defect; (A) stays unordered

> **THIS DISCRIMINATOR REPLACED ONE THAT COULD BE DISCHARGED BY ELIMINATION,
> AND THE ARCHITECT BLOCKED ON THEIR OWN SENTENCE TO DO IT**
> (`evt_3rzp5wh3bnvva`). The first table's met-arm read *"one release emitted
> twice because two source occurrences each name it"* — **already refuted at
> the source**, because `RIGHT_NOT_HELD` calls only `withResource` and never the
> standalone `release`, so there is exactly one source-level occurrence on this
> path. `M3` could not have returned that arm. **Left standing, it would have
> answered the fork by elimination and landed on "a local defect; (A) stays
> unordered" — the Architect's own trigger discharged in the direction that
> relieves them of filing (A), by an arm they wrote wrong.** The second arm was
> mis-sorted the same way: reaching `prelude.rs:2371` on a path that never calls
> it is not a benign "second emission site", it is a keying failure of exactly
> the kind the theorem is about.
>
> **Counting occurrences was the wrong axis.** The replacement is the theorem's
> own criterion, so it is stable however many emission sites exist — and it
> cannot be satisfied by elimination.

**SOURCE FACTS TO AIM `M3` WITH — these are inputs, NOT `M3`'s answer.** What
the native lowering emits and what it keys on is still unread.

    ONE SOURCE OCCURRENCE ON THIS PATH:
      prelude.rs:2518   private_with_resource_after_open calls
                        release_if_live ONCE, in the bind continuation
                        over `body resource`
      prelude.rs:2496   release_if_live's single Vis carries
                        PrivateResourceRelease; its continuation is a Ret
      px7f_resource_native.rs:92-163   RIGHT_NOT_HELD calls only
                        withResource; never the standalone release

    A SECOND, NEAR-IDENTICAL EMITTER EXISTS THAT THIS PATH DOES NOT CALL:
      prelude.rs:2366   proc release, Vis at :2371, same
                        PrivateResourceRelease a FsHandle resource head,
                        different continuation

    M3 MUST CHECK WHETHER THE NATIVE PATH REACHES prelude.rs:2371.
    Looking only inside release_if_live would miss it.

Neither of these is a hard stop and neither is a new inventory entry — **this
ruling increments neither counter in `§5`.** (Stated as a delta rather than as
a value: `§5` is where the count lives, and a second copy of the number here
would go stale the moment it changes.)

Worth recording: the spec's exactly-once obligation is **already keyed on
resource identity rather than on occurrence**, so on that axis the semantics
has taken (A)'s shape and it is the planner that diverges from it.
