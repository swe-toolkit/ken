---
id: RT-MATCH-DIFFERENCE-REACHABILITY
title: "The narrowed MatchScrutineeRecursor difference is what now blocks the retirement -- settle whether it is source-reachable under the method gate, because it is the capstone's only discharge"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
origin: "Successor to RT-MATCH-SCRUTINEE-DISPOSITION D2b, which attempted this measurement and honestly stopped short. Architect ruling evt_29rrwtbh48n8z authorized D3-delete only on a source-unreachability argument; the narrowing landed instead (PR #2458), leaving RT-DESCENT-RETIRE barred with this as the sole named discharge. Steward-filed per COORDINATION section 2 so the bar cites a node that exists and is owned, rather than an un-taken branch of a merged node."
---

## What this node is

**The one measurement standing between the federation and retiring
`RecursiveDescent`.** Everything else in the campaign is done.

[[RT-MATCH-SCRUTINEE-DISPOSITION]] narrowed the retention guard so
`MatchScrutineeRecursor` is retained **if and only if** the ordinary producer
route declines its immediate computational scrutinee. That was
behaviour-preserving by construction and needed no reachability claim — which is
exactly why the reachability question is still open.

**`D2a` proved the difference is non-empty as a backend-IR shape.** So the
variant is load-bearing on *something*. **Whether that something can be written
by a user is unmeasured**, and [[RT-DESCENT-RETIRE]] may delete nothing until it
is settled.

> ### THIS IS THE CAPSTONE'S ONLY DISCHARGE. Nothing else lifts the bar.
>
> `RT-DESCENT-RETIRE` records, verbatim: *"the capstone is barred pending that
> named measurement, not permanently."* **This node is that named measurement.**
> Emptiness is **not** available as a discharge — `D2a` constructed a member, so
> no measurement will ever empty the difference.

## The difference, stated exactly

**Do not re-derive this.** A program is in the difference when the retention
guard fires and the routing guard declines:

- the `Match` scrutinee is a `ComputationalMatch` with **non-empty** `cases`; and
- **some** case has non-empty `recursive_positions`; and
- **some** case body fails `produces_deforestable_aggregate_with_ih` under that
  case's `case_ihs`.

**The two "some"s need not be the same case**, and the minimal witness is a
single case satisfying both. **Empty `cases` is not in the difference** — `any`
over an empty slice is false, so retention does not fire either.

**The IH environments are asymmetric, and this is where to look.**
`requires_heterogeneous_deforestation` enters with an empty `BTreeSet`, and the
`ComputationalMatch` arm rebuilds `case_ihs` from `recursive_positions.len()`.
**A case with no recursive positions therefore has no IHs available to its
body**, which is a strong requirement on that body.

## What `D2b` already tried, and why it did not settle this

**It is not wasted and it must not be repeated as-is.** A bounded native-build
attempt used a natural nested recursive Ken program. **It normalized before the
runtime classifier** and arrived with residuals `[]` and authority
`FunctionizedUnits`. The search was stopped and the narrowing taken instead.

⇒ **The obstacle is not that nobody has tried hard enough.** The obstacle is
that a **failed search is not the argument this question needs**, and one more
round of program-writing will produce another failed search.

## The method gate — this node exists BECAUSE of it

> **A NEGATIVE EXISTENCE CLAIM IS NOT ESTABLISHED BY FAILED ATTEMPTS.** *"We
> wrote N programs and none reached the difference"* is consistent with the
> N+1th reaching it. **Argue from the surface grammar, the elaborator's
> admission rules and the kernel gates** — what a user can write at all — never
> from a sample of attempts. This is
> [[RT-REFUSAL-SOURCE-WITNESS-OR-INVARIANT]]'s gate, established by the
> operator's 2026-08-16 instruction.
>
> **READ ANY REFUSAL YOU CITE FOR AN ESCAPE HATCH.** The lexical node's first
> `D1` was blocked for citing a real kernel refusal and then naming *that
> refusal's own recommended workaround* as the thing closing the route — the
> error text read *"(use ascription)"* verbatim. **A diagnostic that tells the
> user how to proceed is naming a route your claim must then close separately.**
> Name the stage that **actually** refuses, from an observed run.
>
> **A WITNESS IS THE EASY DIRECTION.** One `.ken` file reaching the difference
> settles this outright, and it needs no gate argument at all.

## Deliverables

**`D1` — settle source-reachability of the difference.** The productive attack
is the normalization step `D2b` hit: **why** did the program normalize before
the classifier, is that normalization total over the shapes that would otherwise
land in the difference, and is it a rule of the elaborator or an artifact of the
program that was chosen? Answer from the rules, not from more programs.

**`D2` — record the outcome where [[RT-DESCENT-RETIRE]]'s bar reads it.** The
bar names this node; a result not written there leaves the capstone barred
regardless of what was measured.

## OPERATOR DIRECTIVE, 2026-08-16 — read this BEFORE the outcome list

> Prior art indicates that retiring `RecursiveDescent` is possible and observed
> resource usage by that implementation makes it desirable. If outcome 2 or 3 is
> returned by `runtime-implementer`, that is a clear signal to refer the matter
> to research for guidance.

**Two things change, and neither is what this node measures.**

1. **The objective is not in question.** The Steward escalated that lane 1 may
   not be achievable as scoped and offered partial retirement as the fallback.
   **The operator declined that fork**: retirement is held possible on prior art
   and desirable on resource usage. **Do not soften a finding to protect the
   objective, and do not read outcome 2 as ending the campaign.**
2. **Outcomes 2 and 3 hand off to the research seat, not to a re-scope.** The
   ring still stops and hands back; the Steward routes onward. **A witness or an
   honest "cannot settle" is a complete, useful delivery under this directive** —
   it is what triggers the referral, and prior art is readable only in a seat
   with reference access (`CLEAN-ROOM.md`), which no build seat has.

⇒ **The measurement itself is unchanged.** Report what the rules say. The
routing of the answer is the Steward's problem, and it is now settled in advance
for all three answers.

## THREE outcomes are authorized. Do not force a fit into two.

**This frame's predecessor authorized two outcomes and the true answer was a
third.** That cost a review cycle, and the ring was right to stop rather than
force it. All three below are complete, releasable answers:

1. **UNREACHABLE, with a gate argument** — the difference cannot be written.
   Report it sized; **`D3-delete` then needs a fresh Architect ruling** before
   any deletion. **Do not delete on your own finding.**
2. **REACHABLE, with a witness** — one `.ken` file in the difference.
   `MatchScrutineeRecursor` is then load-bearing **on Ken's present rules**.
   **Stop and hand back; the Steward refers it to research** per the directive
   above. **Not permanently load-bearing** — that was this frame's earlier
   wording and the operator's prior-art ground supersedes it. A witness says the
   shape is reachable **today**, not that no implementation has ever closed it.
3. **NOT SETTLED within the bound** — the rules do not decide it and no witness
   was found. **This is a legitimate outcome, not a failure**, provided the
   report states *which* rules were consulted and *where* the argument runs out.
   **An honest "cannot settle" beats a gate argument that does not hold**, and
   the campaign has now paid twice for the difference between them. **Stop and
   hand back; this also routes to research**, and `AC-5`'s "where the argument
   runs out" is precisely the input that referral needs.

## Acceptance criteria

**`AC-1`. A reachability claim in either direction names the mechanism, by file
and line.** For unreachable: the grammar production, admission rule or kernel
gate that refuses, **observed**, not inferred. For reachable: the `.ken` source
and the observed residual set.

**`AC-2`. No claim rests on a sample of attempts.** Restating `D2b`'s failed
search, at any N, discharges nothing. This is the criterion the node exists to
enforce.

**`AC-3`. Any refusal cited is quoted in full and read for an escape hatch.**

**`AC-4`. Use `observed_recursive_descent_residuals()`, never the selector.**
Its `Match` arm tests `MatchScrutineeRecursor` **first**, so this is the one
variant the short-circuit reads **optimistically**.

**`AC-5`. Outcome 3 states where the argument runs out.** *"Could not
determine"* alone fails this; naming the rule that would decide it and why it is
unavailable passes.

**`AC-6`. No-regression, in CI** (`COORDINATION §12`). Targeted local validation
only. **This node may land no production change at all.**

## Banned scope

- **Deleting the variant, the enum, the selector, the authority or the lane**,
  on any outcome. Outcome 1 reports; the Architect rules; deletion is a separate
  act.
- **Re-narrowing or otherwise editing the retention guard.** It landed at
  PR #2458 and is behaviour-preserving; this node measures, it does not repair.
- **Re-scoping the retirement** on any outcome. Hand it back — and note that
  under the 2026-08-16 directive a re-scope is not the disposition anyway; the
  referral to research is.
- **The `RecursiveDescent`-as-oracle framing** (operator, 2026-08-15). What a
  program does under `RecursiveDescent` is not evidence that it should compile.

## Sequencing

**`active`, `depends_on: []`.** Released to the runtime ring at `evt_4y1rq71vbg6zd`; nothing gates it; the difference is defined above
and the narrowing has landed.

**It blocks [[RT-DESCENT-RETIRE]], and it is that node's SOLE remaining
dependency that is not `merged`.** Eight of the capstone's nine original
dependencies merged, and the ninth
([[RT-MATCH-SCRUTINEE-DISPOSITION]]) merged at PR #2458 — **so without this
node the capstone's graph would read fully unblocked while its own text bars it
from deleting anything.** This node is what makes the generated views agree with
the record.

**Lane 1 under the operator's two-lane directive.** The Steward flagged that
lane 1's objective may not be achievable as stated; **the operator answered on
2026-08-16 that it is** — see the directive block above. The objective stands,
the fallback the Steward offered was declined, and **this node's result is now a
routing input rather than a re-scope trigger.**
