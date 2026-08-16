---
id: RT-STATIC-WORKER-WITNESS-PROGRAM
title: "Write a Ken PROGRAM that reaches the static-worker conservation refusal -- the incidence question the operator's narrowing decision rests on, which every existing demonstration answers only for hand-built fixtures"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Operator request, 2026-08-16, on the Steward's capability-loss brief: the refusal's cost SHAPE is known and its MAGNITUDE is not. The operator asked for a Ken program demonstrating the refusal and proposed the Adversary; the Steward's routing is runtime authors, Adversary attacks -- rationale in Sequencing. Steward-filed per COORDINATION section 2."
---

> # THE QUESTION IS *"DOES A PROGRAM HIT THIS"*, NOT *"CAN THE REFUSAL FIRE"*.
>
> **The second is already answered and is not worth re-answering.** Five
> instrumented compiles reached it in `RT-UNSUPPORTED-LANE-REFUSAL-REACH` `D0`.
>
> **Every construct that has ever reached this refusal is a hand-authored
> fixture** — `ctor:fixture::PX8JScopeTree::Node`,
> `ctor:fixture::PX8JHoleOutput::Node`, and their siblings. **A witness authored
> to exhibit a shape proves the shape is expressible, not that anyone writes
> it.**

> ### CORRECTED: the expected outcomes are TWO test files, not one. 40, not 32.
>
> **As first filed this node said the fixtures *"both live in exactly one file,
> `control.rs`"*.** That was false. Adversary `evt_10mrp6jyykm7z`; the Steward
> re-ran the census against the tree at `3f56561ed` and it reproduces exactly:
>
> | file | `StaticWorkerBinding` | role |
> |---|---|---|
> | `lowering/mod.rs` | 29 | production, raises it |
> | `lowering/core.rs` | 28 | production, raises it |
> | `planning/static_transition.rs` | 1 | production |
> | `lowering/core/tests/control.rs` | **32** | expected outcomes |
> | `lowering/core/tests/constructors.rs` | **8** | expected outcomes |
>
> ⇒ **`D0`'s baseline census must cover both test files.** If
> `constructors.rs`'s eight are driven differently from `control.rs`'s
> thirty-two, **they are a second fixture family** and must be looked at before
> authoring a witness — a witness modelled only on the `control.rs` family may
> be answering the narrower question.
>
> ### The premise's STRONGEST form is now measured rather than asserted
>
> **Zero occurrences in `ken-cli`, `ken-elaborator`, `ken-verify`, `ken-host`,
> `ken-interp`.** All 98 are inside `ken-runtime`. **No end-to-end layer has
> ever expected this refusal.**
>
> ⇒ **A witness, if one exists, would be the first `ken-cli`-level program to
> reach it.** That is a sharper statement of the target than *"not a fixture"*,
> and it gives `AC-1` an objective tell: the witness lives where none of the 40
> does.

## Why this node exists: an operator decision rests on the missing number

The operator is being asked to accept two recorded capability narrowings as the
price of retiring `RecursiveDescent`. The Steward's brief stated the cost as
**known in shape, unknown in magnitude**, and this node is the magnitude.

**The refusal** (`lowering/mod.rs:4726-4740`):

> *"a constructor carrying an unconsumed static worker denotes a value
> containing the callable and has no runtime representation"*

It fires when a constructor transports a static worker in a field and the
recognition is **neither consumed nor erased** — nothing statically rebinds it,
and its transport reaches no consumer.

**In source terms, the shape to write is:** store a function value in a
constructor field, and do not call or discard it at any statically visible
site.

## `D0` — a Ken program, compiled through the real path

**Write it in Ken source** and compile it the way a user's program is compiled.
**A hand-built `RuntimeExpr` does not discharge this** — that is what the
existing fixtures already are, and it is the exact gap the node exists to close.

**Baseline the existing families first, across both test files.** Thirty-two
expected outcomes in `control.rs` and eight in `constructors.rs`; say whether
the eight are driven the same way as the thirty-two. **If they are a second
family, a witness modelled only on the first answers a narrower question than
the one asked.**

**Report, whichever way it goes:**

- **If it refuses:** the source, the refusal text as emitted, and **how natural
  the program is** — would someone write this to solve a problem, or only to
  trip the refusal?
- **If it does NOT refuse:** that is the more valuable result and it must not be
  buried. It would mean the shape is reachable in fixtures and not in source,
  which bounds the operator's cost at or near zero and **changes the
  recommendation.**

## `D1` — say which way the difficulty ran

**`D0` is a search, and how hard the search was IS the finding.** Record
whether the program fell out naturally or had to be contrived, and what had to
be true for the recognition to go unconsumed. **"I had to work to trip it" and
"it happened on my second try" are different answers to the operator's
question**, and only the author knows which occurred.

## Acceptance criteria

**`AC-1`. The witness is Ken source compiled through the production path.**
Not a `RuntimeExpr` literal, not a `cfg(test)` fixture constructor. **Name the
compilation entry point used** so a reader can tell it apart from the existing
fixture route.

**`AC-2`. The naturalness assessment is explicit and is allowed to be
unflattering.** A sentence such as *"this program has no reason to exist except
to trip the refusal"* is a **complete and valuable** discharge of this AC. **Do
not manufacture plausibility.**

**`AC-3`. A negative result is reported as a result, not as a failed turn** —
**and it must be reported as REACHABILITY, never as absence of expectations.**
If no reasonable source program reaches the refusal, say so and hand back; a
turn that produces that has succeeded.

> ### A ZERO HERE IS CONSISTENT WITH TWO WORLDS. SAY WHICH ONE YOU MEASURED.
>
> **Architect `evt_737c6vk66jztr`, correcting this AC as first written.** It
> said a zero result *"makes the operator's decision easier."* **The first half
> — that a zero is a real result — stands. The inference does not.**
>
> | world | what a census shows | implication |
> |---|---|---|
> | no well-typed Ken program **can** reach the refusal | zero end-to-end expectations | the stance costs little |
> | no end-to-end layer **has ever tried** | zero end-to-end expectations | the cost is unmeasured, and the refusal may be squarely in the way |
>
> **These are indistinguishable from the outside**, and the corrected census —
> 40 expected outcomes in `ken-runtime`, zero in `ken-cli`, `ken-elaborator`,
> `ken-verify`, `ken-host`, `ken-interp` — is equally consistent with both.
>
> ⇒ **A census of what the corpus EXPECTS cannot answer this node's question.**
> The deliverable is an attempt to *drive a well-typed Ken program into the
> refusal*, and the report must say which of the two worlds the attempt
> established. **"I found no expectations" discharges nothing.**
>
> **This is the third instance of one shape today** — `subst_qterm_at`'s
> decrement arm and the six ignored tests were both unexercised-versus-
> unreachable. **Unexercised and unreachable look identical until something
> tries.**

**`AC-4`. Row 1 is OUT OF SCOPE.** `NativeJoinPlanV1`'s *"terminal answer has
no affine checked-root authority"* is a **different kind of thing** — an affine
proof token internal to the lowering machine (`RootTerminalAnswerAuthority`,
minted once at the root, moved not copied, consumed by `.take()` at
`emit_result`), not a statement about Ken programs. **Do not fold it into this
count.** If it needs an incidence question it gets its own node.

**`AC-5`. No repair.** Do not make the refusal go away, do not widen the lane,
do not touch `RecursiveDescent`. **This node measures.**

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation
targeted only — `-p ken-runtime` / `-p ken-cli`, never `--workspace`.

## Banned scope

- **Repairing or narrowing the refusal.** See `AC-5`.
- **Reopening the five dead ledger dispositions** — see
  [[RT-NESTED-CONSTRUCT-ROUTE-EXCLUSIVITY]] and
  [[RT-LEDGER-UNNAMEABLE-OBLIGATION-CONTRACT]].
- **Row 1 / `NativeJoinPlanV1`** — see `AC-4`.
- **Adding the witness to the ignored corpus.** If it becomes a test it is a
  live one; an `#[ignore]`d witness answers nothing and re-creates the exclusion
  problem [[RT-IGNORED-CORPUS-MEMBERSHIP-RULE]] just repaired.

## Sequencing

**Ahead of the operator's ruling if the operator wants magnitude first;
otherwise alongside.** Not a hard gate — but it is the one measurement that
bounds the cost.

> ### WHAT THIS NODE FEEDS CHANGED. It is now an input to a SPEC question.
>
> **Architect `evt_737c6vk66jztr`.** The refusal is not merely an undecided
> stance — **it is in tension with a contract that already exists.**
>
> `45 §3` **BE-Model (`AC3`)** states, with no carve-out for a function in a
> constructor field and none at a unit boundary:
>
> > *"**Functions lower to ordinary closures** … ordinary closures and **graphs
> > containing them** are runtime-local opaque values."*
>
> **A construct where a function value cannot be represented is a case where
> functions do not lower to ordinary closures** — and a constructor carrying a
> closure is precisely a *graph containing* one.
>
> ⇒ **If `45 §3` admits no exception, the refusal is a backend defect against
> `AC3` rather than a permitted narrowing.** That question is Spec's and is
> being framed separately. **This node measures; it does not settle it.**
>
> **The TCB defence is on the wrong axis and is not available here.** `45 §2`
> **BE-NotInTCB (`AC1`)** already places the native backend **outside** the
> type-soundness TCB, netted by differential agreement rather than trust. A
> boxed-closure representation would add **no trusted surface**. The real cost
> is **tested surface** — new backend machinery the corpus must cover — which
> is a genuine but much smaller cost than the one it replaces.
>
> **Accepting the narrowing now is not declaring the stance ever.** If the
> operator accepts, it lands as *"an interim limitation, recorded in the
> `unsupported` construct lane, pending the `45 §3` question"* — **not** as
> *"Ken has decided static workers are not first-class."* That second one is a
> language commitment and nobody has made it.

> ### RUNTIME AUTHORS. THE ADVERSARY ATTACKS. Not the other way round.
>
> The operator proposed the Adversary. **The construction needs the lowering's
> own vocabulary** — recognition, static elimination, transport — which is
> runtime's.
>
> **The Adversary's question is the one that decides whether this node
> succeeded: is the witness a fixture in disguise?** That is the same
> distinction it has enforced three times in one arc — the barred per-test
> measurement, forced-versus-observed returns, six-versus-thirty-three corpus
> membership. **Give it the finished witness and let it ask whether the program
> is real.**
>
> **It has stated the test in advance so you can aim at it rather than discover
> it** (`evt_10mrp6jyykm7z`):
>
> > **Does the construct arise from what the program is trying to compute, or
> > from an arrangement chosen because it reaches this refusal?** A witness that
> > only differs from the existing 40 by living in a different file is a fixture
> > that moved.
>
> **This is not a fourth gate and it does not change `AC-2`.** A witness that
> fails this test is still a valid `D0` outcome — it is reported as *"contrived"*
> under `AC-2`, which explicitly permits the unflattering answer. The test tells
> you what to *record*, not what to *achieve*.
>
> It also keeps the Adversary **report-only** (`COORDINATION §10⁻a`) rather
> than turning it into an author, which is not a posture change to make
> casually.

## Provenance

Operator request in session, 2026-08-16, following the Steward's capability-loss
brief. The unmeasured-incidence gap it closes was first surfaced by Adversary
`evt_6d81evnk2nyfn` — *"the population that would test the claim is still
unmeasured; both the original 81 and the new 18 are drawn from programs that
cannot"* — and has survived three repairs without being measured.
