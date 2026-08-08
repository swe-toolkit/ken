---
id: PX8-ERRID-SCOPE
title: "PX8 clause-(a) A2b — five PR-C error identities have no independent production-reaching evidence; Architect ruled all five inside the closure"
status: merged
owner: verify
size: L
gate: none
depends_on: [PX8-ERRID-ALLOC]
blocks: [PX8]
github: null
origin: "Split out of PX8-WROTE-ABS by the Steward 2026-07-27 when framing its A2a half. Both halves trace to the Architect's PX8 closure-property verdict evt_163mfgjs7fkh8 (2026-07-23)."
---

> ## ⚖️ ANSWERED 2026-07-27 — Architect `evt_6tzss92ckj2by`
>
> **Partition: inside = all five; outside = none.** *"There is no lawful
> normative trim of these failure classes from PX8's clause-(a) closure. The
> boundary is error-versus-progress, not 'errors convenient to induce.'"*
>
> ⇒ ⭐ **Route 1 applies to all five. No operator narrowing decision is
> needed** — the Architect said so explicitly. `owner` moves off the enclave
> once the prerequisite lands; this is now evidence work, not a scoping call.
>
> ⛔ **But row 3 is "inside, but currently not representable"** and was split
> out as [[PX8-ERRID-ALLOC]] (Foundation, **released**). This node is now
> `depends_on: [PX8-ERRID-ALLOC]`.

> ## ✅ FRAMED 2026-07-27 — shovel-ready; blocked ONLY on `PX8-ERRID-ALLOC`
>
> **Frame:** `docs/program/wp/PX8-ERRID-SCOPE.md`, inputs pinned by blob at
> `origin/main = 012aa56d`. Owner **Verify**, size **L**.
>
> ### ⭐ Two premise corrections the frame carries
>
> **1. The locator below is stale.** `seed-buffer-io.md:619-645` is **PR-B**.
> The real row is `buffer-io/transfer-failures-remain-errors` at **`:653-676`**.
>
> **2. The row is a PARTIAL, not a zero.** Its status is
> `RED-UNTIL-REMAINING-PR-C-ARMS — partial reaching evidence only`, and **five
> of its ten arms already reach independently**, with the seed naming where:
> `Closed`, `ResourceKindMismatch`, `RightNotHeld`, and `BufferLimit` at
> `effect_v1.rs:3764`, and `Interrupted` at `px8f_write_partition.rs:339`.
>
> ⇒ ⭐ **This WP extends an established pattern rather than inventing one**, and
> those two tests are required reading before writing anything.
>
> ### ⛔ And the row flips GREEN only when all five land
>
> Four of five leaves the row exactly as red as it is today — a clause whose
> whole point is universality, quietly half-closed. An unreachable row is a
> **hard stop with a named mechanism**, not a partial delivery.
>
> ⭐ **Status stays `draft` because `depends_on` is unmet**, not because it is
> unframed.

## Per-row constraints the Architect fixed — ⛔ settled inputs, do not re-derive

1. **`MalformedResource`** — a real `ResourceErrorV1` identity emitted by
   resource resolution **before** positioned backend I/O. Because checked
   `BufferHandle`/resource constructors are sealed, the honest reaching seat
   may be the **production runtime/ABI boundary**, not a forged checked Ken
   value. ⛔ A malformed ABI envelope dying as a terminal decode error **is not
   this row**.
2. **`InvalidBounds`** — must reach the actual read/write consumer, return
   exact `InvalidBounds`, produce **no** progress value, and prove the backend
   was **not invoked** where the fault is pre-I/O. ⛔ An unrelated
   buffer-allocation bounds rejection does not discharge the positioned row.
3. **allocation ≠ `BufferLimit`** — ⛔ blocked on [[PX8-ERRID-ALLOC]].
4. **unsupported/nonblocking posture** — inside **only** as the synchronous
   rejection identity: exact `ResourceHostIO Unsupported`
   (`IoErrorIdentityV1::Unsupported`) from the real synchronous positioned
   backend, preserved as an error. ⛔ **PX8 gains no nonblocking mode and no
   retry/status result.** Retain a **negative** assertion that `WouldBlock` is
   absent from PX8's progress vocabulary; ⛔ a test that manufactures
   `WouldBlock` or adds a nonblocking input belongs to **PX12**.
5. **host-I/O ≠ `Interrupted`** — reach a different **stable named** backend
   identity through the real positioned path (prefer write-side
   `ResourceHostIO BrokenPipe` or another exact named `IOError`), ⛔ not an
   undifferentiated harness failure. If exercised after a successful
   `writeAll` prefix, the exact prefix is preserved and no bytes past it are
   claimed.

## ⛔ What the closure claim may NOT count (Architect, verbatim scope)

> *enum construction, a hand-fed error, a terminal ABI-decode failure, or one
> engine's result as the other engine's oracle.*

⇒ **Each row must bind its real producer to the real interpreter/native
reification route and assert the exact locked-spec identity independently.**

---

⛔ **This was the half of [[PX8-WROTE-ABS]] that could not be sized.** A2a (the
interpreter capped-short `Wrote` absolute oracle) is framed and released;
this is A2b.

## The gap

`conformance/behavioral/buffer-io/seed-buffer-io.md:619-645`. Five PR-C error
identities have **no independent reaching evidence**:

- `MalformedResource`
- `InvalidBounds`
- allocation-failure distinct from `BufferLimit`
- unsupported-nonblocking posture
- host-I/O-failure distinct from `Interrupted`

These are values reified by the positioned/partial IO path, so clause (a)'s
**universal** absolute-evidence claim cannot be made while they are unreached.

## ⛔ SUPERSEDED — the scoping question this section posed is ANSWERED

⚠ **This section formerly said the in-scope set was an open normative call and
that the node must not be framed as an implementation WP.** Both are now false:
the Architect answered on 2026-07-27 (`evt_6tzss92ckj2by`) — **all five inside,
none outside, no operator narrowing needed** — and the per-row constraints are
recorded at the top of this file.

⭐ Kept rather than deleted because the *reason* the question had to precede the
sizing still holds and is worth transferring: a team handed "add reaching
evidence for five error identities" would have silently picked route 1 for all
five, which would have made the scoping call by default instead of answering
it. **The answer happened to be route 1 — but it was not the ring's to make.**

⇒ The residual risk is now different and narrower: route 1 for all five is
correct, so the way to get this wrong is no longer a silent trim but a **weak
witness** — see the "may NOT count" list above.

## Disposition

⭐ **The scoping ruling is DONE** (`evt_6tzss92ckj2by`), so this is now an
ordinary build WP: production-reaching absolute evidence for five exact rows.
Owner moved `spec-enclave` -> `verify` — it is oracle/evidence work of the same
class as `BUDGET-EFF` and [[PX8-WROTE-ABS]].

⛔ **Not releasable until [[PX8-ERRID-ALLOC]] merges** — row 3 has no identity
to assert against until then, and releasing four-of-five invites a silent
partial close of a clause whose whole point is universality.

⚠ **`PX8` does not close until this and [[PX8-WROTE-ABS]] and
[[PX8-F-CAP-41]] all discharge**, and the closure property is re-verified.

## CLEARED by the Adversary on merged `dea1e064` — with one prose caveat

**No defect.** Triaged and accepted by the Steward; **do not re-file any of the
below.**

**The reaching claim holds in its strong form**, which was the axis with least
independent confidence. `abi_v1.rs` drives `ken_host_dispatch_v1` — the real FFI
entry — and its token is **derived from live inventory**: taken from a genuine
`BufferAllocate`, wrapped via `ResourceTokenV1::from_erased_identity`, and
**asserted to resolve in `context.resources`** before a future generation is
derived from it. It neither forges a checked Ken resource value nor freezes a
literal slot index, which are the two fakes the frame's own gap paragraph names.
`eval.rs`'s ordering claim is **asserted**, not merely named
(`backend.write_calls == 0`), and — the stronger point — the backend is rigged
to be **loud** if visited, so the zero-visit assertion proves something an inert
backend could not.

**No row pins an ordinal in either numbering.** `eval.rs` contains **zero**
numeric-literal assertions; `abi_v1.rs` passes `reply.detail` through the
production decoder and asserts on the **named variant**, so a moved code moves
decoder and assertion together.

> ### THE ONE THING THAT CAN GO STALE SILENTLY, AND IT IS PROSE
>
> A doc comment states **"writes wire resource-error code 1"** as a fact. It is
> **prose, not a pin**, and it is safe today only because the wire numbering is
> **append-only**, so code 1 cannot move. **It is the single place here where a
> number could rot with no test failing.** If the wire numbering ever stops
> being append-only, this sentence is wrong and nothing will say so.

**On the QA no-new-run gap: record the STRONGER reason.** The handback closed it
with *"prose-only, executable bytes unchanged"*, which is only as good as a
token-level claim nobody re-derived. **The gap is closed because CI ran the
suite on the exact merged SHA**, so whatever the final child changed, the merged
bytes were executed. The prose-only premise would become load-bearing only if CI
had **not** run on that SHA.

**Bounded, and left open on purpose:** duplication was checked **among the three
added rows only**, not against the pre-existing corpus.

### The predecessor's `effects.rs` loop is closed as a CHARACTERIZATION

`PX8-ERRID-ALLOC`'s `+223/-5` in
`cranelift_backend/lowering/core/tests/effects.rs` — flagged unaudited twice —
adds two `#[test]`s plus six per-role projection fixtures, with the nullary
rejection control kept from being a bare negative by the six positives beside
it. **The `-5` is the notable part and runs the right way:** it deletes a
hard-coded `if fixture.call_index == 0 { 11 } else { 22 }` literal pair in favour
of per-role fixtures — **the same class of frozen magic value as the
`err.alternative(10)` witness that had to be derived.**

**Still open, and not to be reported as cleared:** whether the six fixtures
discriminate from one another, and the contention assessment for that tree.
