# WP frame — `ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT`

Node: `docs/program/issues/ABI-S6-HS18-CHECKED-IH-CONSUMER-PORT.md`.
Owner: **runtime**. Size **M**. Estimated capability tier **T1**.

> # THIS NODE UNBLOCKS INCREMENT A OF `ABI-S6-HS18-MAIN-BASED-CLOSURE`.
>
> Increment A holds the call sites and **none** of the definitions. It is at
> `bd3e1ff676444f48771d261371982a8f289ce26a` — Category A cleared, 21 errors
> down to 11, **every remaining error attributable to this node**. Increment A
> cannot reach green until this lands.

## 1. Objective

Port the **checked-IH post-call consumer machinery** — a capability `main` never
grew — from the tree that has it, onto a `main`-based line, so that the verifier
substance's call sites resolve.

**Named for the capability, not for the errors that exposed it** (Architect,
`evt_6mptkrtvysd8s`). The eleven compile errors are a symptom that located it;
they are not its scope.

## 2. D0 — the question this node answers BEFORE it ports anything

**D0. What is the transitive closure of machinery absent from `main` that the
consumer machinery itself reaches?**

**UNMEASURED.** The Architect measured the five seed symbols and explicitly
nothing beyond them:

> Whether the consumer machinery *itself* calls into anything else that is 0 on
> `main` — I measured the five symbols you named and nothing beyond them. If it
> has its own absent dependencies, we find a fourth population at exactly this
> point again — so the new node's first deliverable is the transitive census,
> not the port. **Do not let it be cut as "add these five things."**

**The census is deliverable one and it gates the port.** Do not begin porting
before D0 is answered and posted.

### 2a. D0 is already known to be non-trivial, and that is measured

The five-symbol seed **was already short by one before this node was framed.**
`constructor_identity` surfaced only when Category A was attempted, and its
resolution took two corrections:

    constructor_identity, domain crates/**              main: 4 refs, 1 def
    constructor_identity, domain crates/ken-runtime/**  main: 0 refs
      positive control cranelift_backend, ken-runtime, main:  2239

`main` carries an unrelated free function of that name in
`crates/ken-elaborator/src/erasure.rs:6872`. **A census over `crates/**` reports
a false hit.** In `ken-runtime` the name is a *local variable*, and the actual
dependency is the method that produces it:

    b601e2ec7  lowering/core.rs:14247
      let constructor_identity = self.static_transition_plan
                                     .constructor_symbol_identity(origin)?;

    constructor_symbol_identity, ken-runtime   main:      52 refs / 2 defs
                                               b601e2ec7: 53 refs / 3 defs

⇒ **The symbol is on `main`; one `impl` of it is not.** That is a **different
shape** from the other five, which are absent symbols. **Do not assume the
census's remaining hits all have the first shape**, and do not treat a non-zero
count on `main` as proving a symbol is present in the sense this node needs.

## 3. Fixed inputs, measured at named refs

Steward-measured 2026-09-16 at `origin/main`
`a75a470106248514a53a989987449b49c62834b8`, each with a live positive control.

### 3a. The seed — absent from `main`, present at the port source

    symbol                                        main   b601e2ec7 (defs/refs)
    CheckedIhPostCallConsumerStep                    0        1 / 20
    CheckedIhPostCallConsumer                        0        1 / 40
    checked_ih_generated_context_result_contract     0        1 /  5
    static_response_forwarded_result_identity        0        1 /  2
    checked_post_call_consumer_frame                 0        1 /  2

      domain           crates/ken-runtime/**
      port source      b601e2ec78989d32222b34b9ec68e01844c6d70b
      positive control cranelift_backend   main 2239 / b601e2ec7 2350

**Exactly one definition site per symbol, all under `planning/**`.** This is a
**port from a named tree**, not an authoring task — and the frame says so
because *"port these definitions"* and *"write this machinery"* are very
different sizes and the second is the wrong one.

### 3b. The shape is narrower than "planner machinery absent"

**`StaticTransitionPlan` is on `main`** at 291 references. It is not a missing
layer. Absent is: one type family (`CheckedIhPostCallConsumer{,Step}`, 48 refs
at base), two accessors on that existing type, and one on `Lowering`.

### 3c. `b601e2ec7` IS EVIDENCE, NOT A BASE

> Read it, port from it, and **never make it an ancestor of a candidate.** The
> candidate bases on `origin/main`, per the operator's 2026-09-16 direction
> ("do not build on an unmerged commit"). A port whose provenance is a tree is
> still a `main`-based candidate.

Identical in force to the rule the parent node applies to its two `preserve/`
refs.

## 4. Deliverables

1. **The transitive census** (D0), posted before any port work begins, with its
   ref and domain pinned and a positive control on every zero.
2. **The ported definitions**, based on `origin/main`.
3. **The `constructor_symbol_identity` impl** — resolved per whatever shape the
   census determines it to have, not assumed to be a straight port.
4. A statement of what the census found **beyond** the seed, by name, including
   "nothing" if that is the answer — stated as a measurement, not a silence.

## 5. Acceptance criteria, each with its control

**AC-CENSUS-IS-A-PREDICATE.** The census's scope is stated as a predicate —
*every symbol whose fix requires machinery absent from `main` in the
`ken-runtime` domain* — and its closure is asserted over that predicate, **never
as "the listed symbols are ported."**

> **Control — read the exit criterion and ask whether a NEW symbol would
> violate it.** If the criterion can be satisfied by a tree that still fails to
> compile on a twelfth symbol, it is an enumeration wearing a predicate's
> clothes and it fails this AC. **A list cannot report being incomplete**, and
> this list already proved it by being short by one.

**AC-DOMAIN-PINNED.** Every census measurement names **both** its ref and its
path domain.

> **Control — `constructor_identity` is the regression case.** Re-run the
> census's own command for it over `crates/**` and over
> `crates/ken-runtime/**`. The two disagree (4 vs 0). A census that reports one
> number for it without a domain fails this AC even if the number is the right
> one, because **an unanchored figure is unfalsifiable rather than false.**

**AC-CONTROL-ON-EVERY-ZERO.** No zero is reported without a positive control
returning non-zero from the same instrument, ref and domain.

> **Control — the zeros in §3a are only measurements because `cranelift_backend`
> returns 2239/2350 beside them.** A census with no live positive result cannot
> distinguish "absent" from "my pattern was wrong."

**AC-NOT-A-BASE.** `b601e2ec7` is not an ancestor of the candidate.

```sh
git merge-base --is-ancestor b601e2ec78989d32222b34b9ec68e01844c6d70b HEAD \
  && { echo "FAIL: port source is an ancestor of the candidate"; exit 1; }
echo "OK: port source is evidence, not a base"
```

**AC-NO-CAPABILITY-FLIP.** The refused `MappingAcquireFile` capability grant does
not ride along. The preserved checkpoint carries it across five coordinated
sites and it is out of scope here exactly as in the parent.

> **Control — grep the candidate's diff for `MappingAcquireFile` and expect
> zero.** Read the parent frame `§4` before touching either tree.

**AC-NO-INVENTED-DEFAULTS.** No absent dependency is discharged by substituting
a value that merely compiles.

> **Control — `OwnedContext.result_contract` is the worked example.** It could
> be set `None` today and **would build**, silently disabling the identity check
> the field carries. The correct disposition, already taken by the implementer,
> is to write the real expression and let the file **fail on a named dependency
> rather than pass on an invention.** A candidate that "fixes" that back fails
> this AC.

**AC-NO-REGRESSION. Workspace-green in CI**, never a local `--workspace` run.
Local verification is targeted only, through `scripts/ken-cargo`, scoped
`-p ken-runtime`.

> **Build under memory pressure: use `-j 1`.** Two default-parallelism
> `-p ken-runtime` builds were OOM-killed at ~5.5 GB available on 2026-09-16;
> the same build at `-j 1` succeeded at the same headroom. See §7.

## 6. Contention

`crates/ken-runtime/src/cranelift_backend/planning/**` and `lowering/**`.

**Increment A of the parent node is live in the same files** at
`bd3e1ff676444f48771d261371982a8f289ce26a`. That is not a conflict to resolve —
it is the reason this node is sequenced first. **The runtime ring holds
increment A until this lands**, and its own Category A work is complete, so
there is no concurrent authoring in those paths.

## 7. The box, and why `-j 1` is in the frame rather than in folklore

Measured 2026-09-16. Two `-p ken-runtime` builds were OOM-killed while a
concurrent `-p ken-elaborator` suite ran; the box has 15995 MB total with
~5.5 GB available and fourteen resident seats. **The same build, run alone at
`-j 1`, succeeded at the same ~5.5 GB.**

⇒ **The binding constraint is codegen parallelism, not machine size.** `-j 1`
uses one codegen job and no parallel `rustc` instances. Build that way here
rather than rediscovering it through a kill.

## 8. Not this node

- Increment A's eleven Category-A errors. Complete, and not this node's.
- `MappingAcquireFile`. Refused, and out of scope in both nodes.
- Increments B (Q1 resume-exit repair) and C (amendment 8's consumer
  relocation). Both stay in the parent.
