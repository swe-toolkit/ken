# RT-LEXICAL-RECURSOR-CONSUMERS D2d — no-carrier static continuation fusion

Owner: runtime. Size: M. Node: [[RT-LEXICAL-RECURSOR-CONSUMERS]] (`#6d`).
Architect ruling `evt_2wwh9yamyhs7p`, 2026-08-10. Fixed inputs were measured at
`main` `5756ff74`; `D2c` has since landed and `main` is `35231088`. **Re-derive
your merge-base — do not reuse a SHA from this frame.**

**Seat tier: T1.** The suspension of Runtime's T1 exception covers campaign node
`#8` only; `#6d` is a campaign node and stays T1. Do not downgrade this seat.

## What `D2c` established, and why it is the premise here

`D2c` withdrew the retained-lane reuse premise by measurement: on the identical
expression `RecursiveDescent` performs **zero** carrier transfers while
functionized B-only performs six and refuses on the sixth. **The retained lane
never carries the closure**, so there was no lawful counterpart to copy. **That
evidence has landed** — it merged as `e810a227` into `main` `8c72ed62`, not as
the earlier `66688fa4` this frame was drafted against. Take the withdrawal as
settled and do not re-measure it.

## The ruling: both obvious repairs are rejected

**Eager owner-side forcing is rejected.** The inner case constructs
`PX8JHoleOutput::Node[Var(0)]` where `Var(0)` is a
`ComputationalRecursorClosure`, and its invocation arguments are supplied only
by the downstream outer case's `Call Var(0) [unit]`. **The inner constructor
owner does not possess those arguments.** Consuming there would manufacture a
call, alter evaluation order, and fail to generalize to recursive fields whose
IH takes other arguments.

**Closure representation is rejected.** `ComputationalRecursorClosure` names a
**live activation and cursor, not a value**. `boundary_transfer_admissibility`
must keep rejecting it **at every depth**. No carrier tag or class, no ABI slot,
no `Closure`/`DeclarationClosure` representation, and no
`call_declared_unit_target` escape is authorized.

## The ruled mechanism

1. The planner issues a **static continuation-specialized unit identity** for
   the exact producer occurrence plus its exact composed consumer suffix. That
   identity is compiler-owned and keyed by **planner-issued occurrence, checked
   frame, or invocation identities**.
2. Emit the producer and that suffix in **one functionized emission region**.
   When the inner carried computational case produces its specialized result,
   route the `RoutedAnswer` directly through the already-owned composed source
   continuation — **before `carried_join_arm`, and therefore before
   `transfer_into_carrier`**.
3. The downstream source `Call` remains the **sole** consumer of the recursor
   closure, consuming the same activation, cursor, selection and unwind
   authority the closure already carries. **Only the closure-free final result**
   may enter a carrier join or a declared-unit result slot.
4. **Unit reuse is legal only when the complete static continuation identity
   agrees.** Distinct suffixes require distinct planner-issued specialized
   units. If the planner cannot establish that identity, **the path refuses** —
   it must not fall back to the unspecialized result-returning unit.
5. This is **deforestation of an unobservable intermediate under a statically
   owned immediate consumer**. It is not eager normalization and not a runtime
   continuation representation. A result with no such exact consuming suffix
   keeps the existing carrier path and keeps refusing if its graph contains a
   recursor closure.

> **The identity keying is the soundness-bearing clause, not bookkeeping.** The
> ruling forbids keying on constructor spelling, type, row number, a runtime
> tag, or *"the only continuation."* That last one is the same existential shape
> that got `d94ef37e` rejected on `D2b` — *some* call rather than *this* call.
> An identity that happens to be unique in the measured population is not an
> identity.

## Acceptance criteria

**AC-1.** On the exact `R3` before-hole compile under B-only exclusion, the
inner `Node` recursor closure and its exact activation and cursor are produced
and then consumed by the downstream call **in the same specialized unit**, with
**zero** `transfer_into_carrier` attempts for that intermediate
`Node[ComputationalRecursorClosure]`.

**AC-2 — the A/B witness.** Suppressing **only** the continuation fusion
restores the measured origin-23 refusal. Suppressing anything else does not
count; the point is that the fusion is what changed the outcome, not some
neighbouring edit.

**AC-3.** A closure-free sibling result continues through the ordinary carried
join **unchanged**.

**AC-4.** A same producer **lacking** the exact consuming suffix still
**refuses** — it does not force, drop, or represent the closure. This is the
control that distinguishes a real identity from a population that happens to be
singular.

**AC-5.** `D2b`'s controls are retained and still prove `Closure` and
`DeclarationClosure` unconditionally non-transferable, and
`call_declared_unit_target` free of any closure lane.

**AC-6 — finish the `D2a` repair on the leg it missed.** Confirmed Adversary
finding `evt_6enwsf0jrdezx`, measured on `8c72ed62`. `D2c` hardened one leg of
the A/B and left the **suppressed** leg byte-untouched, in the same function
thirty lines below:

```rust
let (s_arrivals, s_forwards, suppressed) = run(true);
assert!(s_arrivals > 0, "the suppressed run saw no arrivals, ...");
assert_eq!(s_forwards, 0, "the suppression did not actually suppress ...");
```

**Delete the denominator and `assert_eq!(s_forwards, 0)` passes more easily, not
less** — because nothing arrived. `s_forwards == 0` is the clause establishing
that the suppression actually suppressed; vacuous, the negative leg no longer
separates *"suppression restored R1"* from *"R1 returned for an unrelated reason
while zero forwards happened because zero arrived."*

Apply the same idiom verbatim: build `s_arrivals` as a `NonZeroUsize` and read
`.get()` where the count is needed. **Two lines. No predicate change, no count
pinned, no new control.** The `.contains(R1)` assertions below are positive and
do not go vacuous — this is specifically `s_forwards == 0`.

**Why this is in scope rather than tidy-up:** the trim motive is a pass that
strips `#[cfg(test)]` machinery out of production, both counters feed both legs,
and that pass reaches the unhardened leg exactly as easily as it would have
reached the hardened one. **The hazard was never leg-specific; the repair was.**

**And it is a standing caution on your own AC-2**, which is an A/B of exactly
this shape. When you author it, make the clause that establishes the suppression
worked inseparable from its denominator by construction — do not restate the
guarantee in a comment.

## Excluded scope

- **Row 5 after-hole stays outside the claim.** Its `StaticWorkerBinding`
  refusal remains **reported-only**. Do not claim a row green, and do not assume
  that wall will be repaired.
- **`R4` is not yours** — it belongs to [[RT-LEXICAL-ROW2-MISSING-MINT]].
- `#6d` keeps its status. `D3` and the retirement are untouched.

## Stop conditions — return to the Architect, do not decide

Any need for a runtime continuation or callback; any **inference** of suffix
identity; any eager recursor invocation; any closure carrier or ABI
representation; any weakening of the whole-graph admissibility walk.

## Sizing and validation

One turn to a releasable increment or a genuine hard stop; both are good
outcomes. Targeted validation only — `-p ken-runtime`, or `--test <name>` for
one suite, **never `--workspace`**. If an enum variant is added or changed, the
floor is a full `-p ken-runtime` test build, because a suite-scoped run cannot
observe an exhaustive `match` in a sibling target.

## Contention

None. Paths are `crates/ken-runtime/src/cranelift_backend/**`. Kernel is idle,
Language owns `crates/ken-elaborator`. No `spec/` or `conformance/` path, so no
Spec vote is required on the merge Decision.
