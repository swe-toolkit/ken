---
id: RT-BRANCH-LOCAL-DECLARED-CALLABLE
title: "recursive_position_unit_body returns one Option<StaticOriginId> for the whole source, so whole-source agreement is too coarse for a Match whose arms differ -- the cut is constructor-and-recursive-position-specific callable authority installed inside the already-selected constructor case, which eliminates the closure crossing rather than opening a durable closure lane"
status: ready
owner: runtime
size: L
gate: none
depends_on: [RT-RECURSIVE-POSITION-ARM-ARITY]
blocks: [NATIVE-HANDLE-CARRIER, PX8-F-CAP-41]
github: null
origin: "Architect ruling evt_7aeb7hqrykgpz, Decision dec_7aajmm0eac45c, resolved 2026-08-18. Cut by the Steward on that ruling's explicit instruction to frame the branch-local design capability separately from the rejected D1 AC-3 recut. Surfaced by RT-RECURSIVE-POSITION-ARM-ARITY D1, whose repair moved the governed rows onto the BoundaryCarrier refusal. Steward-filed per COORDINATION section 2."
---

> # THE BINARY I ROUTED WAS FALSE. Read this before reasoning from the guard.
>
> I asked the Architect whether a function-valued recursive field was out of
> scope by design, because `reject_carried_residual_arguments` fires on CAP-41
> and its doc says the durable closure lane is withheld. **The ruling is that
> both halves are true and they do not conflict:**
>
> **The durable closure lane REMAINS EXCLUDED. A function-valued recursive
> field is NOT out of scope.** There is already a separate lawful route, and
> the gap is elsewhere.

# THE BOUNDARIES THAT STAY. None of these is what this node changes.

- A raw `LoweringOperand::Carried` is a **transferred value, never callable
  authority**.
- **`reject_carried_residual_arguments` remains the fail-closed guard** for
  non-empty invocation through that raw-value arm, before control installation.
- **Not authorized, and none of it is a fallback if this cut gets hard:** no
  `PersistentClosure` lane, no new carrier tag or class admission, no
  `FrozenClosure`, no implicit `StaticCallableRef` conversion, and no metadata
  recovered from the carried word.

# THE LAWFUL ROUTE THAT ALREADY EXISTS

In `lower_recursor_residual_call`, the `recursive_unit_body` /
`FunctionizedUnits` arm **runs before `reject_carried_residual_arguments`**. It
lowers explicit source arguments and calls
`call_declared_recursive_position_unit`, and `call_declared_context` can append
planner-authorized capture operands.

⇒ **Static code identity and capture authority stay compiler-owned; the carried
word contributes only the eliminated value. No `Closure` value crosses.**

# THE ACTUAL GAP

`recursive_position_unit_body` returns **one `Option<StaticOriginId>` for the
whole source**. [[RT-RECURSIVE-POSITION-ARM-ARITY]] `D1` was right to refuse to
select a surviving unit when an arm lacks the recursive position — **but
`Ret`/`Vis` proves whole-source agreement is too coarse.** `Ret` has no
recursive position; `Vis.k` does.

**The cut is constructor-and-recursive-position-specific callable authority,
installed only inside the already-selected constructor case.** It must name the
declared body plus its checked explicit-input/capture plan, from retained
source and planner authority.

- **`Ret` installs none.**
- **`Vis` may install one only when the body and captures are lawfully
  expressible as declared call inputs.** If the captures cannot be supplied
  through planner-owned operands or an already-admitted structural-value route,
  **that case still refuses. The guard is not weakened.**

> ### WHY THIS IS `ready` WHILE ITS DEPENDENCY IS STILL `active`
>
> `gen-progress.sh` warns on this deliberately, so here is the answer rather
> than an ambiguity. **`D0` is classification only and touches no production
> line**, so it does not need the arity node's `D1` recut to land first. The
> `depends_on` edge is real and governs the **implementation** deliverable,
> which contends on the same function and must sequence after that recut.
>
> **Releasing `D0` early is safe; releasing an implementation deliverable early
> is not.** Do not read this block as licence for the latter.

# `D0` — TWO NAMED SETS. Freeze the PREDICATE, never the roster.

**Architect ruling `evt_4cvagpx6enpp8` / `dec_15546q1w6pd8s`, which corrects
this node's first framing and the Steward's proposed repair of it.** I grouped
16 rows across 7 test files by their shared refusal text and called that the
population. **A shared refusal text proves only a shared terminal guard.** I
then offered three replacement framings and **all three were rejected**: crediting
[[RT-SITEOP-CARRIED-WITNESS]] `D2` is **provenance, not design membership**; the
text census plus a bounded non-claim is honest about its roster but **cannot let
`D0` say "the population decides the shape" when `D0` never defined the shape's
predicate**; and "more than one constructor reachable" is **too coarse**, since
arms may produce the **same** constructor with different bodies or capture plans,
and conversely all arms may already agree and need no new route.

⇒ `D0` carries **two named sets, and they are not interchangeable.**

## SET 1 — the SEMANTIC MECHANISM POPULATION. This is the frozen predicate.

An occurrence is a `D0` subject when **all four** hold:

1. a carried computational recursive position is **invoked with nonzero source
   arguments**;
2. its source has **multiple reachable producer outcomes**;
3. whole-source `recursive_position_unit_body` authority is **absent,
   ambiguous, or otherwise too coarse**; and
4. invocation authority must therefore be assessed **after partitioning those
   outcomes by the compiler-owned key `(selected constructor identity,
   recursive position)`**.

**For each selected bucket, inspect every reachable producer outcome.**
Agreement is over a **complete declared-call descriptor**, not merely a matching
`StaticOriginId`:

- body origin,
- checked ABI / signature,
- ordered capture/input plan,
- and the **invocation-coordinate / context authority required at the exact
  consumer**.

**The mechanism-owned subset** is a bucket with **one agreed complete
descriptor** and a **lawful non-`Closure` input route**. A missing or disagreeing
descriptor, an unavailable capture or coordinate, or a durable-export boundary is
**still classified by `D0`** and then dispositioned as a refusal or a hard stop.

> **A different terminal error string does NOT exclude an occurrence that
> satisfies this predicate.** That is the whole point of separating the two sets,
> and it is the hole that this amendment closes: the old `AC-1` defined the
> population by the error string, so a row reaching this mechanism and refusing
> at a **different** guard was invisible to the census while `D0` reported
> complete coverage.

**This predicate is the structural closure the later implementation must enforce
at the authority-minting seam.** Ken programs are unbounded, so **no finite
test-string census can prove exhaustive program membership** — which is why the
predicate, not the roster, is the durable artifact.

## SET 2 — the BOUNDED WITNESS SNAPSHOT. The 16 rows live here, and only here.

**The exact refusal text is a legitimate DISCOVERY SEED. It is not the population
definition.** Record:

- the **exact base SHA**,
- the **exact query**,
- and the resulting stable **`(path, test name)` identities**.

**Require set equality** between that identity set and `D0`'s table. **The count
is informational only** — one removed row plus one added row preserves 16 and
defeats a count-based control. A fresh witness is **a delta to classify under the
same predicate**, not a new population.

Files carrying the text at framing time: `px7f_resource_native`,
`px7l_checked_host_recursive_bind`, `px7m_hostresult_computational_match`,
`px8ta_oriented_subcontinuation`, `px8x_single_schema_observation`,
`rt_parity_native`, `rt_escape_second_resource_native`.

## THE `D0` TABLE — write it DURABLY into this file

The original four axes **omit two facts needed to decide whether the route is
buildable**. Per row, at least:

1. stable **row identity** and its **current terminal guard**;
2. **which of the four carried-residual consumers** it reaches;
3. **reachable producer outcomes**;
4. **selected constructor** and **recursive position**;
5. **whole-source resolution outcome**;
6. the **complete descriptor set within the selected bucket**, and whether it
   agrees;
7. **capture/input representation**;
8. **invocation-coordinate / context availability at that exact consumer**;
9. **measured boundary kind**;
10. **disposition and evidence**.

**Boundary kind uses a CLOSED taxonomy:** intra-artifact generated-unit call,
live-domain cross-artifact call, durable export/persistence, or **unresolved**.

> ### `unresolved` IS A HANDBACK, NOT AN "INTENDED REFUSAL"
>
> An **intended refusal must cite the deciding prohibition or the failed
> authority condition.** A row nobody could classify is an unresolved hard stop
> and comes back to the Steward. Do not let the two collapse into one bucket —
> that is how an unclassified row acquires the appearance of a decided one.

> ### THE COORDINATE COLUMN IS LOAD-BEARING. Do not drop it as redundant.
>
> `call_declared_context` can append planner-authorized captures **only after it
> resolves a context**. **At least one current consumer supplies no coordinates
> and deliberately refuses raw fallback when that body has a generated context.**
>
> ⇒ **"Capture representation is `Record`-shaped" alone does NOT establish that
> the call site can supply it.** A classification that reads capture shape and
> stops has not answered the question the implementation needs answered.

# ACCEPTANCE

- **`AC-1`** — the **semantic predicate** (Set 1) is recorded, and the **single
  authority-minting call plus every `recursive_unit_body` consumer** is
  **re-derived at the `D0` base**, not cited from this frame.
- **`AC-2`** — the **bounded witness query** is recorded, and closure is
  **identity-set equality** with the durable table. **Count equality does not
  discharge this** — a swapped row preserves the count.
- **`AC-3`** — every table row is classified through **complete per-key
  descriptor agreement**, **capture/coordinate availability**, and **measured
  boundary kind**. **Shared refusal text alone fails. Historical credit alone
  fails.**
- **`AC-4`** — every disposition is exactly one of: **mechanism-owned**,
  **already served by the existing whole-source route**, **intended refusal with
  the exact reason or spec clause**, or **unresolved hard stop**. **No generic
  "excluded".**
- **`AC-5`** — `D0`'s merged artifact **is the table and its evidence**.
  **`D0` alone does NOT close this node** and does **not** unblock
  [[NATIVE-HANDLE-CARRIER]] or [[PX8-F-CAP-41]]; it supplies the **post-`D0`
  implementation cut**.

# BANNED SCOPE

- **No implementation before `D0`.** The **predicate** decides the shape, and
  `D0` is what defines it — see `AC-5`: `D0` does not close this node.
- **Nothing from the not-authorized list above**, and it is not a fallback.
- **No weakening of `reject_carried_residual_arguments`.** It stays the
  backstop wherever branch-local authority is absent.
- **No `RT-RECURSIVE-POSITION-ARM-ARITY` work.** That node's `D1` recut is in
  flight and owns its own `AC-3` control; this node does not touch it.

# CONTENTION

Same file as the in-flight `D1` recut —
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs`, the
[[RT-BACKEND-MODULE-SPLIT]] decomposition target. **`D0` is classification only
and touches no production line.** A later implementation deliverable **will**
contend; sequence it after the `D1` recut lands, and re-derive every symbol by
name rather than by offset.
