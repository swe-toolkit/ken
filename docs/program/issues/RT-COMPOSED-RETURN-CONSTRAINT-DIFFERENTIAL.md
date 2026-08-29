---
id: RT-COMPOSED-RETURN-CONSTRAINT-DIFFERENTIAL
title: "RESEARCH REPORT (comparative, not novel research): the tail-resumptive composed return `bind t (\\x. Ret (f x))` is a simple, common construction that comparable systems lower, yet Ken's native (PX8) backend cannot. Report the EXACT Ken constraints that block shape (a) and shape (b), and how each DIFFERS from the constraint set of languages/compilers that DO support the construction — so the fix is a known constraint to relax, not an open research question."
status: ready
owner: research
size: M
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Operator ruling 2026-08-29 (this session), overriding the Steward's accept-the-boundary recommendation. Verbatim: a compiler which cannot support this construction is of little use; it is a simple and common construction; other systems achieve it, so it is not a deep research question. Directive: revisit shape (a) and shape (b) through the constraint DIFFERENTIAL — the constraints of Ken that prevent each shape and how they differ from the constraints of other languages that support each shape — and ask research for a report. Inputs are the two closed discovery verdicts (RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY shape (a) INSUFFICIENT; RT-COMPOSED-RETURN-SHAPE-B-DISCOVERY shape (b) REFUTED axis 3) and the closed RT-COMPOSED-RETURN-PRODUCED-TRANSFER (D0b=NO, partial-order contradiction). Steward-filed per COORDINATION section 2."
---

> # RELEASED — lane 1, the new runtime objective. Research report only. `ready`.
>
> The operator REJECTED accepting the native Tail composed-return wall as a
> boundary. The construction is simple and common and comparable systems lower
> it; therefore the Ken wall is a consequence of specific Ken design
> constraints, not a fundamental impossibility. This node commissions a
> comparative constraint-differential report from the research seat. It is a
> paper report: no production edit, no candidate, no PX8 witness.

## Framing the operator set — read this before scoping the work

**This is NOT open-ended novel research, and must not be scoped as one.** The
working assumption, set by the operator, is that a fix EXISTS because comparable
systems support the construction. The report's job is to LOCATE the constraint
delta, not to discover whether one is possible. A report that concludes "no
known technique" without first identifying which Ken constraint the comparators
lack has not answered the question — it has restated the wall.

The earlier Research advisory (`evt_774v5fjnxcfcw`, Q1) asked a different, weaker
question: does any surveyed family PRESERVE Ken's current emit-then-validate
order and still deliver the result. The answer was no. **That is the wrong
question now.** The operator's question is the inverse: comparable systems do
NOT share Ken's order/quotient/backedge constraints — so name precisely which
constraint each system lacks, and what Ken would change to lower the
construction the way they do.

## The construction

A tail-resumptive composed return — an effectful operation whose result is
transformed and returned in tail position inside a resource bracket. Semantic
core:

```
bind t (\x. Ret (f x))
```

The two complete `SourceFormat::Ken` witnesses, both ordinary effectful I/O, are
**fs-read-at-offset** (`readAt`) and **fs-write-at-offset** (`writeAt`). The
Tail vs Direct arrivals (48 Tail / 3 Direct) are compiler arrivals within those
two real programs, not fixtures. This is bread-and-butter I/O, so the gap sits
on the ABI / native-completeness critical path.

## The two Ken-side obstructions the report starts from (grounded, closed)

Both are established Ken facts from the closed discovery nodes; the report does
NOT re-derive them, it uses them as the Ken side of each comparison.

- **Shape (a) — establish source-specific validation authority BEFORE the
  producer.** Ken constraint that makes it INSUFFICIENT
  (`RT-COMPOSED-RETURN-PRODUCER-ORDER-DISCOVERY`, Architect
  `evt_6bq9q76rmzm90`): the producer at `source.rs:4369-4374` emits
  `RoutedAnswer::checked(returned)` and the Tail `Ret` consumer at
  `core.rs:12743` jumps to the shared `return_body` with `scrutinee.word`. A
  pre-producer authority proof can only REFUSE earlier; it cannot alter the
  already-emitted `returned` word, so it flips no Tail row from
  `PatternMatchFailure` to the exact result.
- **Shape (b) — statically de-quotient the generated entry and relocate the
  producer AFTER Tail route selection.** Ken constraint that REFUTES it
  (`RT-COMPOSED-RETURN-SHAPE-B-DISCOVERY`, axis 3): the relocated call's
  ordinary operand envelope — the source constructor's nonrecursive fields —
  lives in the selected-case environment before self-resumption and is gone
  after the two-argument active backedge (`core.rs:12225-12258` carries only
  `scrutinee.word` + a route-control word). The distinction is typed, not just
  positional: for a `Vis` node (`spec/30-surface/36-effects.md:489-491`) the
  generated-context raw worker argument is the `E.Resp e` RESPONSE while the
  envelope's nonrecursive field is the `E.Op` OPERATION, and the response-only
  continuation never captures the operation.

The deeper shared root the report should test: Ken's current native lowering
ORDER is emit-R2 -> collapse -> quotient away source identity -> validate later,
and the carried-value defunctionalization reduces the tail-resume backedge to a
two-word ABI. Both obstructions are downstream of that ordering + that backedge
ABI + that quotient. The comparators presumably order and carry things
differently.

## Deliverable — a comparative constraint-differential report

A durable report artifact (Markdown under `docs/program/` or an attached ledger
with a stated SHA-256) plus a convo advisory summarizing it. The report answers,
for BOTH shapes:

1. **The Ken constraint, named exactly.** For shape (a): what forces the
   producer to emit before the validated authority exists. For shape (b): what
   makes the tail-resume backedge unable to carry the ordinary operand
   envelope. Ground each in the current-main executable coordinates above.
2. **At least one comparator system that lowers this construction**, with the
   PRECISE mechanism it uses at the analogous point — the tail-resume of an
   effectful operation whose result is transformed and returned. Candidates the
   report should consider (choose the most illuminating, do not survey
   exhaustively): Koka (evidence-passing / CPS effect handlers), Interaction
   Trees (Coq `ITree`), OCaml 5 / multicore effect handlers, WasmFX / effect
   handlers for Wasm, Eff / Frank, and ordinary CPS or SSA-with-join lowering of
   monadic `bind` in a tail position. For each cited system, state its lowering
   ORDER and how it carries the operand/handler environment across the resume.
3. **The constraint DELTA, made explicit.** What does the comparator NOT require
   that Ken requires, or provide that Ken lacks? Concretely: does it establish
   handler/continuation identity BEFORE producing the response; does it avoid
   quotienting away the per-source identity; does it carry a richer resume
   environment (not a two-word backedge); does it make the continuation a
   first-class function/block rather than a carried discriminant?
4. **Which Ken design choice would change, and its cost.** For each delta, name
   the Ken-side design element that would move (the producer-order, the
   generated-entry quotient/confluence, the two-word carried-value backedge ABI,
   or the defunctionalization itself) and the rough cost/blast-radius. This is
   input to the operator+Architect build decision, not a build design.

## Acceptance criteria

- **AC-BOTH-SHAPES** — the report covers shape (a) AND shape (b); each gets its
  named Ken constraint, at least one comparator mechanism, the explicit delta,
  and the Ken design element that would change. A report on only one shape is
  incomplete.
- **AC-CONSTRAINT-REAL** — for each Ken constraint the report names, it states
  whether the constraint is SPEC-MANDATED (cite the spec rule) or INCIDENTAL to
  the current native lowering (a design choice, not a requirement). This is the
  load-bearing distinction: an incidental constraint is a candidate to relax; a
  spec-mandated one changes the operator's decision. Do not assert intrinsic
  without a spec citation.
- **AC-MECHANISM-CONCRETE** — each cited comparator mechanism is described at the
  level of "here is the order it emits/validates in and here is what it carries
  across the resume," not "system X supports effects." A named system with no
  mechanism is not evidence of a delta.
- **AC-GROUNDED-KEN-SIDE** — the Ken constraints are tied to the current-main
  coordinates already established (`source.rs:4369-4374`, `core.rs:12225-12258`
  / `:12743`, the generated-entry quotient at `aggregates.rs:6323-6543`,
  `spec §6.2`, `spec/30-surface/36-effects.md`), re-measured at the working SHA.
- **AC-REPORT-ONLY** — no production edit, no Runtime QA gate, no semantic
  candidate, no CI run, no PX8 closure. The deliverable is the report + advisory.

## Clean-room boundary (binding on this node)

The research seat MAY read the permissive references (and copyleft references for
approach/behavior only, under the leakage recheck) to describe the comparator
mechanisms — `CLEAN-ROOM.md`. It must NOT copy any reference into Ken, and where
a comparator description rests on the seat's general knowledge of a system rather
than a mounted reference, say so. The AGPLv3 `yon` prototype is NOT consulted. No
implementer consumes this report as source; it informs the operator+Architect
build decision only.

## Reviewers

Architect — the soundness of the comparative claims: whether each named Ken
constraint is correctly attributed to the executable coordinates, whether each
comparator mechanism is accurately described (a wrong "system X does Y" would
mislead the build decision), and whether the spec-mandated-vs-incidental call in
AC-CONSTRAINT-REAL is right. This is a report the Architect will consume to shape
any follow-on build, so the Architect gates its accuracy. A design fork the
report surfaces (competing ways to relax a Ken constraint) is NOT resolved here —
it is returned to the operator.

## Capability tier

T1 — a comparative design-reasoning report on a soundness-bearing lowering
ordering, reviewed on the argument and the accuracy of its attributions, not a
diff. Size M.

## Sequencing

Lane 1 (runtime), the new objective per the operator ruling 2026-08-29 that
reopened the composed-return repair. No `depends_on` — all inputs are closed
nodes at current main and the seam is measurable now. On the report's return:
the Steward briefs the operator; the build disposition (which Ken constraint to
relax, at what cost) is a fresh operator+Architect decision, NOT authorized by
this report. The two discovery verdicts and `RT-COMPOSED-RETURN-PRODUCED-TRANSFER`
stay closed as the Ken-side obstruction record; the held shape-(a) build
`RT-COMPOSED-RETURN-PRODUCER-ORDER-BUILD` stays `draft`. No revival of the closed
axes (Produced-transfer / D3 / Direct-only / recovery / store / tag / carrier /
HS15) is authorized by commissioning this report — the report may RECOMMEND
relaxing a constraint, but adopting it is the later decision.
