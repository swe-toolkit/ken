---
scope: build/qa
audience: (see scope README)
source: private memory `two-arm-producer-needs-a-case-per-arm`; fleet lesson
  `a-fail-closed-over-accept-gate-can-have-correct-production-code-but-an-untested-safety-net-negative-arm`
  (merged 2026-09-27)
---

# A two-arm (or multi-arm) producer needs a discriminating case per arm

When a soundness AC bottoms out in a producer that is a **multi-arm
`match`/filter** and the spec enumerates **multiple categories** the producer is
meant to cover, author a **discriminating case per arm**. Exercising only one
arm is **green-vs-green** under a bug that dropped the *other* arm — the corpus
passes while the enumeration is silently incomplete.

Live (Sec4 trust-model): `trusted_base()` is
`matches!(Decl::Opaque | Decl::Primitive)` and §64 §1 says the TCB is *exactly
three things* — kernel, **primitive reductions (item 2)**, postulates/`foreign`
(item 3). My AC1/AC2 covered item 3 (postulate/foreign/hole) exhaustively but
left item 2 (`declare_primitive`→`Decl::Primitive`) **unexercised** — a producer
that dropped the `Primitive` arm would pass B1–B3 green. spec-author's Fidelity
caught it (non-blocking); I folded a B4 `registered-primitive-surfaces-in-delta`
case.

**Tell:** anchoring on the *security-critical* face of a producer (here: item-3
assumptions *hiding*) can under-cover the enumeration's other category.
Enumerate the producer's arms and the spec's categories; give each its own case.
The enumeration-arm analog of "a multi-dimensional guard needs a case per
dimension" (soundness AC static vs runtime face); sibling of discriminating
conformance verdict must flip.

## The deny arm of a fail-closed gate is an arm too

For an over-accept (or admission) gate the natural focus is the arms that
ACCEPT. The thing that PREVENTS over-acceptance is the negative arm, the
`else => empty/unowned` fallback, and it needs its own negative test. Positive
controls feed in-class inputs, which take the accept arm by construction and
structurally cannot reach the else. So a regression that flips the safety net
into an over-accept passes every control.

**Measured 2026-09-10 on `1874361` (RT-MAPPING-MULTIOP-DISPATCH, +1943/-54,
16 crates files), a pre-merge over-accept gate. Verdict NO DEFECT / APPROVE;
two non-blocking missing-negative-test observations (routed
evt_fsd2bf8e4hrv).** The change let one specialized response owner dispatch a
statically bounded P2 suffix of Deferred responses only when the suffix is a
repeated-producer call chain or a Mapping read/write access chain.
`bounded_deferred_response_suffix` classifies with conjunctive `.all(...)` and,
for anything else, `else { Ok(Vec::new()) }` (responses.rs ~2879-2882), so the
sequence falls through to its established owner unchanged (the "S7" fail-closed
safety net). Every acceptance arm read sound, and the design is pervasively
fail-closed (None/empty on opacity, cycles, unknown Vis, multi-owner
hard-error). But every positive control fed an in-class input, and the one test
whose name said "heterogeneous" (`abi_s6...read_write_read`) alternated
direction WITHIN the Mapping class, still `mapping_access_chain`, still the
accept arm. No test built an FS producer interleaved with a Mapping access, or a
Mapping op followed by an unrelated AmbientOp, and asserted
`bounded_deferred_response_handler_owner` returns None or the route is
byte-identical to base. Flipping `Ok(Vec::new())` to `Ok(suffix)` would pass the
whole suite.

**Report it at its size.** A correct-but-untested guard branch is a
regression-detectability gap (missing negative test), gap-tier and
non-blocking, not a present over-acceptance. Say plainly the code is sound
today and name the exact cross-class input a control should use.

**A parity control without a discriminator cannot prove the NEW path fired.**
In the same change the Mapping-chain class had a real mutation discriminator:
flipping `HandlerOwnedDeferredResponseMutation::SuppressLocalContinuationDrive`
makes the native build TRAP (`UnclassifiedRuntimeTrap{terminal_value:-1}`)
instead of succeeding, so the new owner-drive is load-bearing. The
repeated-producer class had only a native-vs-interp `effect_trace` PARITY
assertion. Parity proves the output is correct; it cannot tell the new
bounded-suffix mechanism from a pre-existing lowering path that happens to emit
the same trace. For any "the new mechanism generalizes" claim, require a
discriminator (a mechanism-off mutation, or an owner-identity assertion), not
behavioral parity alone. This is the runtime twin of the CHECKS.md
reach-and-discriminate check (check 7 when first written, the proxy-fact check
now): the control must be able to FAIL when the claimed mechanism is absent, and a
sibling of
[[downstream-code-coupled-to-a-kernel-admission-invariant-needs-a-negative-control-on-the-changed-branch]]
(verify the discriminating axis the change itself introduces, not an
orthogonal one).

**How to apply:** for a fail-closed over-accept or admission gate, find the
negative arm (the `else`/`None`/`empty` that denies the new capability) and
check a test drives an input INTO that arm and asserts the denial. The
acceptance arms being correct does not cover it.

**Disposition of the B4 fold itself** (a non-blocking strengthening surfaced at
review, folded after gates were already cast, racing the merge on `a81da90`) —
that is a distinct discipline from the discriminator-per-arm rule above; see
[[mid-review-fix-inline-escalate-or-track]] for the full fold-vs-track timing
rule and the Sec4 B4 SHA-race evidence.
