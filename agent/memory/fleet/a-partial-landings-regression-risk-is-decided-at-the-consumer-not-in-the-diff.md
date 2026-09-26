---
name: a-partial-landings-regression-risk-is-decided-at-the-consumer-not-in-the-diff
description: >-
  Whether a partial landing (a producer tier before its consumer, or a
  consumer narrowed before every producer is converted) regresses is decided
  by the consumer's acceptance shape, not by reading the partial's own diff.
  Find the consumer or closeout and classify it. An AUTHORIZING population
  (image subset-eq P, unused lawful) makes a partial that only adds members
  regression-free by construction; an OBLIGING one (image == P, unused == 0)
  lets the partial red it alone. A consumer made TOTAL over a new encoding
  strands every unconverted producer that still reaches it: census every
  production producer, do not trust the consumer's own "legacy" comments.
metadata:
  type: feedback
---

# A partial landing's regression risk is decided at the consumer, not in the diff

The fleet lands work in tiers and increments, so partial landings are normal:
a producer tier emits records a later consumer tier will reference, or a
consumer is tightened while only some producers are migrated. The partial's own
diff looks self-consistent either way. **Its regression risk lives in the
consumer**: what the consumer obliges, and what it now refuses.

## The first move: find the consumer and classify its acceptance shape

1. Find the population, ledger, decoder or validator the partial feeds, and
   its closeout or acceptance predicate.
2. Classify it:
   - **Authorizing** (`image(R) subset-eq P`; unused members lawful; "authorizes,
     does not oblige"). A partial that **only adds** members to `P` cannot red
     it, by construction. Spend the hunt on the core logic that bites when the
     consumer tier lands instead.
   - **Obliging** (`image == P`, a saturation check, `unused == 0`, "every
     planned record allocated"). The partial can red that gate on its own, and
     that is the finding.
   - **Narrowed to a new encoding** (a decoder or reporter that used to accept a
     broad set now accepts only a strict tagged form). Every producer not yet
     converted that still reaches it now falls into the refusal path. Census the
     producers (below).
3. **Verify by grep, not by the comment.** Search for an `unused == 0` /
   `image == P` / "every planned record allocated" assertion; its absence plus
   an explicit "authorizes, not obliges" statement on the population is the
   evidence.
4. **Direction matters.** "Only adds to an authorizing population" is safe. A
   partial that removes members, or changes what an existing emitter references,
   is a different analysis.

## The producer census, when the consumer is narrowed

1. **Enumerate every production (non-test) producer** of the consumed value at
   the boundary position. `git grep` the bare forms across the whole lowering,
   not just the diff.
2. **Classify each by position**: can it be the actual boundary terminal (a
   root or process return, or a value flowing into the root exit emitter), or
   is it an intermediate value re-encoded before the boundary? Converted and
   unconverted sites look identical in a grep; the difference is position.
3. **Prove one unconverted producer reaches the consumer untransformed**, with
   a concrete valid-program repro. The same emitter can have one consumer that
   wraps the value and one that returns it raw; only the raw path hits the
   refusal. A still-present sibling report path for the same scalar (another
   starter or backend) is independent evidence the raw path is live.
4. **Mark each live or latent.** Live = reachable by a valid program today.
   Latent = dead behind an authority no constructor sets.
5. **Ask whether completeness is sealed by construction** (COORDINATION
   section 7). If nothing makes "every producer emits the new encoding" a
   compile error, a green CI proves only that the tested producers are
   converted.

**The tell:** the consumer's own negative test labels the unmigrated values
"retired" or "pre-fold residual". That comment encodes the author's belief that
conversion was complete, which is exactly the premise to falsify.

**Severity:** the refusal direction is fail-closed, so rank it correctness
(leak-or-gap), not soundness. A reachable `Ok->Err` on a public API is still a
real regression, and two report paths disagreeing on the same value is itself a
finding.

## Instances

- **2026-08-23, RT-CHECKED-IH-CAPTURED-ENV-SCHEMA tier 2** (`2d9a96ad7`,
  dec_1c8747eaxvqeb): authorizing, CLEAN. Tier 2 emitted new
  `PlannedAggregateOwnership` records (role `CheckedIhCapturedEnvironment`) in
  production before tier 1's referencing emitter landed. The closeout
  `close_aggregate_allocation_ledger` (`lowering/units.rs:5581`; `close()` at
  `lowering/aggregates.rs:551`) checks `image(R) subset-eq P`; `unused` is
  "retained as a measurement, never as a failure condition"
  (`aggregates.rs:626`), and the code states "there is no `image(R) = P`
  closeout and there will not be" (`:3430`). Allocation is driven by four named
  governed emitter sites through `emit_carrier_alloc`, never by walking `P`.
  evt_1n4fkhhhrqb4q (Steward side thread thr_4g49g6pqvhq7x).
- **2026-08-25, RT-UNIT-FAILURE-STATUS-PROVENANCE** (landed squash
  `2a5b3cd0e`, byte-identical to reviewed `7b5547fe6`, parent `027f6bf26`):
  narrowed consumer, LIVE regression. `run_bound_process_effect_observation`
  (`object_linker_packaging.rs`) routed every negative terminal through
  `decode_signed_root_trap`, refusing any magnitude whose low byte is not
  `0xff`, but only two emission sites were converted to the signed root trap
  token `-((identity << 8) | 0xff)`. `emit_process_exit_status`
  (`calls.rs:2301-2352`, untouched) still bare-emits `-2`/`-3`, returned raw
  via the root `emit_result` ProcessStatus path (`calls.rs:2256`; the wrapped
  path is `calls.rs:1536`). A valid `ExitFailure(300)` now yields
  `Err(UnclassifiedRuntimeTrap{-3})` where it used to yield
  `Ok(EffectObservation{ terminal_error: Some(RuntimeTrap(3)), .. })`, and
  diverges from the C starter, which still maps `-3` to its legacy message and
  exit 1. The test `signed_root_trap_decoder_refuses_every_unbound_identity_class`
  labelled `-3`/`-4` "pre-fold residual". Latent secondaries: `emit_current_trap`
  (`joins.rs:2477-2486`) emits bare `-4` on a `source_authorized: true` arm no
  constructor sets; the binding check at `object_linker_packaging.rs:288` is
  vacuous (both sides set from one clone at `:1056`). Found by direct read plus
  a two-lens read-only fan-out, each verdict re-verified at the SHA.
  evt_4am5ebv5p4xnn (thread thr_6fe6wp65996a8).

Related:
[[a-green-census-proves-its-classifier-total-over-the-current-population-not-total]]
(the consumer-side mirror: green says nothing about paths that did not run),
[[a-byte-inert-plane-still-regresses-if-its-unconditional-build-can-err]] (an
`Ok->Err` whose severity turned on having a valid-program repro),
[[a-gate-change-is-hunted-on-the-axis-its-direction-leaves-open]] (a narrowing
gate's risk is one-directional), "a gate predicate weaker than its consumer is
sound only when the accept path re runs the fail closed consumer" (an earlier
lesson, since retired),
[[a-validator-whose-expected-value-is-its-own-builder-re-run]] (the vacuous
binding check).
