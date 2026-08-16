---
id: SPEC-45-CLOSURE-IN-CONSTRUCTOR-EXCEPTION
title: "Does 45 section 3's 'functions lower to ordinary closures' admit an exception for a function value held in a constructor field with no statically visible consumer -- if not, the native backend's current refusal is a defect against AC3 rather than a permitted narrowing"
status: ready
owner: spec-enclave
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect evt_737c6vk66jztr, 2026-08-16, ruling on the Steward's routing of research advisory evt_580k0qyxkbcz6. The Architect ruled the reframing (Ken would be declaring static workers non-first-class at a unit boundary) and then declined to rule the clause question itself: it is a behavioural-contract question -- what the backend must do to be correct -- and therefore Spec's. The Architect specified this node's subject wording; the Steward verified every cited clause against the tree before filing. Steward-filed per COORDINATION section 2."
---

> # FRAME IT AGAINST THE CLAUSE. DO NOT RE-DERIVE THE CONTRACT.
>
> **The Steward's first draft of this question was *"are static workers
> first-class runtime values at a unit boundary?"* — and the Architect rejected
> that shape.** It invites Spec to re-derive a contract that already exists,
> and **a re-derivation can land anywhere.**
>
> **The question is answerable by reading `45` against a measurement, and
> either answer is a complete result.**

## The question

> **Does `45 §3`'s *"functions lower to ordinary closures"* admit an exception
> for a function value held in a constructor field with no statically visible
> consumer?**
>
> **If it does not, the native backend's current refusal is a defect against
> `AC3` rather than a permitted narrowing** — and `48`'s `unsupported` construct
> lane is where it must be recorded until repaired.

## The clause, verified verbatim at `spec/40-runtime/45-native-backend.md:101`

> **Contract BE-Model (`AC3`).** … **Value representation follows the K3
> model** (`41`, `OQ-7`): … ordinary closures and **graphs containing them**
> are runtime-local opaque values (`41 §2.1`). …
>
> **Functions lower to ordinary closures** — application is **call-by-value,
> strict, left-to-right**. … No closure identity, equality, hash, or
> persistence is observable.

**Two features of this text are load-bearing and neither is an inference.**

1. **It is a positive statement about what lowering DOES**, not a permission.
   There is **no carve-out for a function in a constructor field, and none at a
   unit boundary.**
2. **`"graphs containing them"` names this exact construct.** A constructor
   carrying a closure in a field **is** a graph containing a closure, and `AC3`
   says such graphs *are* runtime-local opaque values. The clause does not
   merely fail to forbid the construct — **it appears to contemplate it.**

⇒ **A construct where a function value cannot be represented is a case where
functions do not lower to ordinary closures.**

## What the backend does today

`lowering/mod.rs:4726-4740` refuses:

> *"a constructor carrying an unconsumed static worker denotes a value
> containing the callable and has no runtime representation"*

It fires when a constructor transports a static worker in a field and the
recognition is **neither consumed nor erased** — nothing statically rebinds it,
and its transport reaches no consumer.

## `41` is NOT the governing clause, and that is a repeat pattern worth naming

**`41 §2.1` is permissive here** — *"Such an aggregate may exist as a runtime-
local value"* grants nothing and compels nothing. **Reading `41` for this
question returns "no obstacle" and stops.**

> ### THREE QUESTIONS ON THIS CAMPAIGN HAVE BEEN ROUTED TO `41`. THE GOVERNING
> ### TEXT WAS ELSEWHERE EVERY TIME.
>
> | question | routed to | governing text actually was |
> |---|---|---|
> | the domain question | `41` | clause 3 (Architect `evt_7wzkzpjmttbht`) |
> | the reporting gap | `41` | `48 §5.4` |
> | this one | `41` | **`45 §3`** |
>
> **`41` is about the observable validity of closure VALUES.** It keeps
> answering *"no obstacle"* to questions that are really about **what the
> backend is obliged to LOWER.** Check the obligation clause before the value
> clause.

## The TCB defence is on the wrong axis and is not available

**The natural argument for refusing is *"a small TCB is worth a capability
cost."* It does not apply here**, and this should be settled before it is
raised.

`45 §2` **BE-NotInTCB (`AC1`)**, verified at `:62`:

> *"The backend is therefore **not in the type-soundness TCB** — its correctness
> is **`tested`** (differential agreement with the interpreter oracle, §4),
> never kernel-certified."*

⇒ **A boxed-closure representation plus an indirect-apply path would add no
trusted surface at all.** It grows a component the spec has **already declared
untrusted**.

**The real cost is tested surface, not trusted surface.** Differential
agreement only nets what the corpus exercises, so new backend machinery is new
ground the corpus must cover. **That is a genuine cost, and it should be stated
as that one** — it is much smaller than the argument it replaces.

**The dependent-types escape is also closed.** Bowman and Ahmed give
type-preserving closure conversion for a Calculus of Constructions subset with
**separate-compilation correctness proved**, so *"Ken is dependently typed, so
this is different"* is not available as a defence. Recorded here so it is not
reinvented.

## Prior art, as an input and not as an authority

Research advisory `evt_580k0qyxkbcz6`, commissioned by the Steward on the
operator's prompt. **Approach and behaviour only, per `CLEAN-ROOM.md` — nothing
vendored, nothing copied.**

- **GHC, OCaml/Flambda, and Lean 4** all represent the field as an ordinary
  pointer to a boxed closure with an indirect-application path. **Lean's
  crosses real separately-compiled module boundaries** (one C file and native
  object per module).
- **MLton** defunctionalizes into tagged records plus dispatch — **because it is
  a whole-program compiler.** That is the contrast, not a counterexample.
- **Nobody refuses at an ordinary compilation-unit boundary.** The closest
  production refusal is OCaml marshaling failing on a functional value, which
  is a **persistence/process ABI** — a strictly stronger boundary.
- **Separate compilation forces a representation choice, not a language
  impossibility.** Modular defunctionalization exists but needs link-time
  machinery to close the universe of closure tags.

**The load-bearing sentence:** *"No visible consumer is not the same as proven
dead."* Production compilers erase the allocation only when escape/use analysis
**proves** the value cannot survive; if it remains live in constructor data they
materialize it even when every eventual call is indirect. **Ken's refusal fires
on exactly the condition where the others materialize.**

## Deliverables

**`D0` — read `45 §3` against the construct and answer the question.** Does
`AC3` admit the exception? **Either answer is a complete result.** Ground it in
the clause text, including the `"graphs containing them"` phrase, and say
whether that phrase contemplates this construct or is about something else.

**`D1` — if the answer is NO, say what follows.** The refusal is then a backend
defect against `AC3`, and `48`'s `unsupported` construct lane is the recording
site until it is repaired — which is [[RT-UNSUPPORTED-BINDING-ON-REFUSAL]],
already filed and `ready`. **Do not size or author the repair here.**

**`D2` — if the answer is YES, write the exception into `45 §3` explicitly.**
An exception that exists only as a disposition in a tracker node is not a
contract. State its condition in the clause's own vocabulary.

## Acceptance criteria

**`AC-1`. The answer is grounded in `45 §3`'s text**, not in `41`, and not in
what the backend currently does. **The backend's behaviour is the subject of
the question, so it cannot be its own warrant.**

**`AC-2`. The `"graphs containing them"` phrase is addressed explicitly.** It is
the strongest textual evidence either way and a reading that does not mention it
is incomplete.

**`AC-3`. The TCB argument is not re-raised without engaging `45 §2`.** If a
soundness or trust argument for the refusal is advanced, it must say how it
survives the backend already being outside the TCB. **`AC1` is quoted above; a
disposition that ignores it is not answering.**

**`AC-4`. The incidence measurement is an input, not the answer.**
[[RT-STATIC-WORKER-WITNESS-PROGRAM]] measures whether a Ken program reaches the
refusal. **A zero result there does not answer this question** — the clause
question is about what the backend is obliged to do, not how often the
obligation is exercised. **And a zero there is itself two-valued:** no program
*can* reach it, versus no end-to-end layer *has tried*. Do not consume it as
though it were one.

**`AC-5`. No repair, no backend change, no `RecursiveDescent` work.** This node
answers a contract question.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`). Local validation targeted
only, never `--workspace`.

## Banned scope

- **Re-deriving whether closures are first-class in Ken generally.** The
  question is scoped to one clause and one construct. See the opening block.
- **Sizing or authoring the materialization work.** The Architect explicitly
  **did not** cost it, and neither does this node.
- **Settling the operator's narrowing decision.** That is separate and is the
  operator's — see Sequencing.

## Sequencing

**Does not block lane 1** (Architect, explicitly). The runtime ring continues
the `RecursiveDescent` campaign regardless of when this is answered.

> ### ACCEPTING THE NARROWING NOW IS NOT DECLARING THE STANCE EVER.
>
> The operator holds an open decision (`evt_7b2vh3pjvfcc6`) on accepting two
> recorded capability narrowings as the price of the retirement. **If the
> operator accepts, it lands as *"an interim limitation, recorded in the
> `unsupported` construct lane, pending the `45 §3` question"*** — not as
> *"Ken has decided static workers are not first-class."*
>
> **The second is a language commitment and nobody has made it.** Keeping these
> separate is the same separation already applied to the `48` binding gap
> (`RT-UNSUPPORTED-BINDING-ON-REFUSAL`), and for the same reason: **an accepted
> decision sitting where a repair should be is how the repair stops being
> owed.**

## The Architect's lean, recorded because a route with no lean is worth less

**Architect `evt_737c6vk66jztr`, verbatim in substance:** *"I do not think the
refusal should be adopted as Ken's stance. The prior art is uniform, the clause
we have points the other way, and the defence I would have expected to carry it
is on the wrong axis."*

**Qualified in the same breath, and the qualification travels with it:** *"I
have not costed the materialization work, so I am not asserting it is cheap —
only that the reasons for refusing are weaker than they looked."*

**This is a lean, not a ruling.** The Architect routed the clause question to
Spec precisely because it is not the Architect's to settle, and Spec is not
bound by the lean.

## Provenance

Architect `evt_737c6vk66jztr`, on the Steward's routing (`evt_3fbmm17cy6xqg`) of
research advisory `evt_580k0qyxkbcz6`. The research was commissioned on the
operator's prompt in session, 2026-08-16. **Every clause coordinate cited above
was re-read against the tree by the Steward before filing** — `45 §2 AC1` at
`:62`, `45 §3 AC3` at `:101`, the `"graphs containing them"` phrase at `:108`.
