# `RT-PLANNER-KRET-GRAFTED-SPINE` — frame

**Owner:** runtime. **Size:** M. **Tier:** T1. **Gate:** none.
**Ground SHA:** `origin/main` `ddbc803228dbb56f73df31a906ec1fe9a413910e`.

> ## THE DELIVERABLE IS THE REPAIR AND THE TWO ROWS IT CLEARS.
>
> The diagnosis is **finished** and is not this node's work. `D0` is a repair,
> not a measurement. A candidate whose roster contains no change under
> `crates/` has not advanced this node.

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

## 2a. The causal chain — SETTLED. DO NOT REPAIR THE CLASSIFICATION PATH.

Architect `evt_5s3q6v3a53v7r`, read verbatim from
`crates/ken-cli/tests/px7f_resource_native.rs` (`const RIGHT_NOT_HELD` begins at
`:92`). Two sites decide the whole row: `bracket_has_right_denial` (`:133-139`)
and `after_right_outer` (`:141-149`).

    interpreter  one release, settles Released
                 => ResourceBracketBodyError (RightNotHeld 32 1)
                 => :136 right_masks True => :147 Success => 0

    native       the second release returns Closed, so the bracket carries a
                 release error AS WELL AS the body error
                 => ResourceBracketBodyAndReleaseError
                 => :138, a hardcoded False => :148 Failure 82 => 82

**82 IS THE FIXTURE'S OWN LITERAL AT `:148`.** There is no exit-status mapping
defect on this row ⇒ **the deferred exit-mapping filing call stays deferred, and
this row hands it no owner.**

**`bracket_has_right_denial` IS CORRECT** — it classified a different bracket,
faithfully. **Do not repair it, and do not repair anything on the classification
path.** *"The native path computes a different bracket classification"* is true
only in the sense that it was handed a different bracket to classify; nothing
computes a wrong classification.

**ONE UNREAD LINK, AND IT IS A ZERO-COST CHECK — RUN IT BEFORE ACTING ON THIS
CHAIN.** That the native bracket is `ResourceBracketBodyAndReleaseError` and not
`ResourceBracketReleaseError`. Both map to `False` at `:137`/`:138`, so **exit
82 alone does not discriminate them**; the native trace carries both the
`RightNotHeld` and the `Closed`, and only `BodyAndReleaseError` can hold both.
**Name the constructor the native run actually built.** If it is neither, the
chain above is wrong, and the Architect wants that inside one message.

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
  those two false passes mean. The other direction also holds, and the pair is
  only legible with both: under `§2a`'s chain **`AC-4` IMPLIES `AC-1`** — no
  fourth event means `BodyError`, so `right_masks` is True and the exit is 0.
  **`AC-1` nevertheless stays an independent AC precisely because that chain
  carries an unread link** (`§2a`): if the native constructor turns out not to
  be `BodyAndReleaseError`, `AC-4` could hold with `AC-1` still red. Keep both,
  and that is the reason. Architect `evt_5tth40ek8zv60`.

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
   interpreter performs ONE (M2) -- keyed, ON THE ARCHITECT'S READING
   AND NOT YET ON A MEASUREMENT, on a source occurrence standing for the
   runtime object grafting actually produced. M3 is what would measure it.
```

> **ENTRY 3 STANDS; THAT WORDING NARROWS WHAT IT CLAIMS RATHER THAN
> WITHDRAWING IT.** As first written it said *"one source-level bracket release
> path produced TWO runtime dispatches"* — **which is exactly what `M3` is
> assigned to find out**, and which `§7` of this same frame calls open. **The
> two standards are deliberately different:** an inventory entry is a reading,
> meant to be cheap and revisable; the (A) trigger in `§7` needs the keying
> **established**, because it authorizes a structural change. Stating the
> reading as fact collapsed them.

**Hard-stop count on this WP: 3.** **The parent node's count of 2 does NOT
carry** — different WP, different question. Both acts have now FIRED at three
(`§1a` at `evt_7d3h7mtff5acd`, `§1b` at `evt_1mv0phbj0zcn7`); **the next `§1a`
re-trigger is at 6**, and the Architect will scope that one to whatever new
fork the next stop surfaces rather than re-asking this question.

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

## 7. The (A)/(B) fork — RULED. (A) IS NOT ORDERED, AND ITS TRIGGER IS STATED.

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

**STATUS AFTER `M1`/`M2`: NOT MET YET — AND "NOT YET" IS NOT "NO"**
(`evt_5s3q6v3a53v7r`). There is now a **live divergence**: the reference spine
has one `Vis` where the native path performs two. What is not yet known is that
it arises from occurrence-keying — **an extra dispatch can equally come from a
duplicated lowering path with nothing to do with the spine.**

**`M3` IS THE ONE READ THAT DECIDES IT:** what emits the second
`ResourceRelease` on the native path, and **on what key.**

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
