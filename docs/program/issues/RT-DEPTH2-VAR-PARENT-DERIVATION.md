---
id: RT-DEPTH2-VAR-PARENT-DERIVATION
title: "Name the parent of the depth-2 Var occurrence at source-machine origin 25 index 0, so the route fork can be ruled on a derived premise instead of an inherited one"
status: closed
owner: runtime
size: S
gate: none
depends_on: [RT-CONTKEY-ROUTE-CLOSURE-PROBE]
blocks: []
github: null
origin: "Architect ruling evt_4xmz4n8n49w1d (2026-08-14) on RT-CONTKEY-ROUTE-CLOSURE-PROBE stop condition 2, alongside merge approval dec_6fj6f6t4hcpa6 for exact 866fab52. He declined to rule the fork and specified this bounded derivation instead. Steward-filed (agents cannot create tracked work per COORDINATION §2)."
---

> # CLOSED 2026-08-14 — ANSWERED, CANDIDATE-FREE. `closed`, NOT `merged`.
>
> **D1 was delivered and ruled on, and nothing landed** — the node's whole
> deliverable was a measurement, its AC-3 required every probe reverted, and the
> ring confirmed blob identity to `f26167e22` on all three paths before deleting
> its branch. **No QA vote, no PR, no merge gate applies**, and `merged` would
> assert a landing that does not exist.
>
> **The result: `parent_arm = RuntimeExpr::Construct`, `parent_origin =
> StaticOriginId(26)`, `parent_construct = ctor:fixture::PX8JScopeTree::Node`,
> demanding arm `core.rs:7494`** (installs `SourceContinuation::ConstructArgument`
> at `:7522`, evaluates child 0 at `:7531`), with `pending_remaining=0` and
> `lowered_fields=0`. Handback `evt_78ddkrtvxkebp`.
>
> **Branch (b) selected; the fork is DISSOLVED, not decided** — Architect ruling
> `evt_57atrppgx2exe`. Neither banned surface is established as required, and
> **row 4 deep and row 5 are one need**. He also **withdrew his own
> `needs confirmation` flag against himself**: the boundary did **not** change
> class, only its enforcement point, so the "inherited premise is stale" framing
> below is retired. Read the rest of this file as the specification that produced
> the measurement, not as live claims.
>
> **The durable record is [[RT-CTOR-TRANSPORT-RECOGNITION-ASYMMETRY]]**, which
> carries the ruling and the two-trees warning. That node is the successor.

## Why this node exists: the deciding premise was inherited, not derived

The Architect declined to choose between `ContinuationTemplate` population and
continuation-source projection, and the reason is a finding rather than a lack
of appetite.

**The inherited claim is that the depth-2/3 residual requires the banned
continuation-source projection surface.** That claim traces to `D2k-1c`, which
recorded depth-2/3's boundary as a further worker-bearing-**constructor**
boundary. That is `close()`'s law at `lowering/mod.rs:4607` — **the site row 5
hits.** The refusal now firing at depths 2/3 is a different one: the pointwise
`Var` read.

⇒ **If both records are accurate, the boundary changed class**, and the
attribution that would decide the fork is pinned to a refusal that is no longer
the one firing. Choosing a vehicle now would be picking a mechanism for an
undiagnosed need — the exact failure the probe's own stop condition 2 warns
about: *"a bundled mechanism anchors the owner and its rejection then reads as
'the need cannot be met'."*

**The Architect marked this comparison `needs confirmation`** against the
`D2k-1c` stop record, explicitly because he was comparing prose to prose there
rather than two things he measured. **Carry that qualifier; do not harden it.**

## One label, two different laws

Both residuals print the class `StaticWorkerBinding`. **That label names a
region, not a mechanism.**

| residual | site | kind of law |
|---|---|---|
| row 4 depths 2/3 | `lowering/mod.rs:3915` `LoweringEnvironmentBinding::value_at` | pointwise fail-closed read — a `StaticWorker` binding read where a machine value is demanded |
| row 5 after-hole | `lowering/mod.rs:4607` `close()` | whole-relation conservation ledger — a recognized constructor-field transport that nothing rebound |

Different laws, different firing times, different remedies. **Row 5 stays
separate** (probe stop condition 4 — the two are not averaged). Row 5's own
attribution is at least *consistent* with the site that fired, so it does not
carry depth-2/3's staleness problem.

## Where depth 2/3 actually stops, measured by the Architect

The reported message begins *"a source-machine Var in value position is …"*.
`value_at`'s template is `{edge} is a value-producing position…`, so the edge
is the literal **`"a source-machine Var in value position"`**, which appears at
exactly one site:

**`lowering/core.rs:7471`** — the `RuntimeExpr::Var(index)` arm of the
source-machine value descent.

The only other edges in that tree are `"a continuation capture input"`
(`core.rs:12650`) and `"a Var in value position"` (`core.rs:17278`); neither
matches, **so the attribution is unambiguous.**

⇒ At depths 2/3 the source machine descends into a bare `Var` in value position
whose binding is a static worker. **It is not a continuation-capture failure,
and it is not row 5's constructor ledger.**

⇒ **The carried consumer is never reached** — the probe's handback says so
twice. So the node's question, *"with the relation available, does the route
close?"*, is **not answered "no" at depths 2/3. It is unanswered.** The route
stops upstream of the consuming site.

## Deliverable

**D1.** At **row 4 depth 2's compile**, name the parent of the `Var` occurrence
at **source-machine origin 25, index 0** — that is, which enclosing
`RuntimeExpr` arm demanded a value there.

Report:

- the demanding arm's `file:line`;
- the parent construct.

> **The level is named deliberately and it is the whole risk in this node.**
> The parent of that occurrence **in depth 2's own compile** — *not* "in the
> route". A probe specified one level off answers correctly and selects the
> wrong branch, which is precisely how the off-by-one on this lane happened
> once already.

## The three branches, and the warrant that refutes each

**Do not take any of them.** They are the discrimination criteria.

- **(a) The parent is a call whose callee slot lost the exact-`Var` shape.**
  Warrant: `value_at` states that a worker's *"only admissible use is as the
  callee of a call with an exact `Var` callee"*. ⇒ the need is
  **call-shape/recognition**, and **neither banned surface is implicated** —
  this is the frame's explicit "third route".
- **(b) The parent genuinely demands a value** (aggregate field,
  primitive/effect argument, scrutinee, stored value, projection subject). ⇒
  the worker must be erased or rebound before construction, which is
  **`close()`'s remedy** — meaning row 4 deep collapses onto **row 5's**
  residual rather than onto continuation-source projection. **That would be a
  simplifying result: one need, not two.**
- **(c) The parent is the continuation's own body reached through a source
  surface no route can rebind.** ⇒ continuation-source projection is genuinely
  required and the original attribution stands.

## Acceptance criteria

**AC-1.** The parent is reported as `file:line` plus construct, for the
occurrence at origin 25 index 0, **in depth 2's own compile**. A report that
cannot state which compile it measured does not satisfy this.

**AC-2.** Raw values are reported, not a verdict. The probe that preceded this
node earned its result by reporting construct, edge, owner, origin and index
rather than a conclusion; hold that standard.

**AC-3.** No branch is selected, and no repair is retained.
`crates/ken-runtime/src/cranelift_backend/lowering/core.rs` and
`crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs` are
byte-identical to the candidate base, checked by blob id rather than by report
— that is how the predecessor's equivalent AC was actually discharged.

**AC-4.** The `D2k-1c` stop record is quoted verbatim where this node's
boundary-changed-class claim depends on it, so the Architect's `needs
confirmation` qualifier can be discharged against the text rather than against
a paraphrase.

## Banned scope

- **Do not populate `ContinuationTemplate`** and **do not add a
  continuation-source projection surface.** Both are the fork; the fork is not
  yours and is not ripe.
- Do not repair row 4 or row 5.
- Do not average rows 4 and 5 into one need.

## Not this node

Row 5's after-hole residual. It is separate by ruling, and its remedy question
is open on its own terms.

## Settling this needs an INSTRUMENTED RUN, not a read — and this node owns it

Recorded 2026-08-14 from the Adversary's hunt on `afdabc502`
(`evt_pg42y72y6hrx`), which independently reached the same conclusion about the
boundary-class comparison and named it as unhunted rather than implying
coverage.

**The comparison at the head of this node is prose against prose** — the
`D2k-1c` stop record against the refusal now firing — which is exactly why the
Architect marked it `needs confirmation` rather than ruling on it. **Settling it
requires instrumenting the residual path; no amount of further reading closes
it.** That is `D1`'s measurement and it belongs to Runtime here.

**It is therefore not an open invitation elsewhere.** The Adversary offered to
instrument it as its next pass; that would duplicate `D1` and risk two
differently-specified measurements of one question, which is the failure this
node is built to avoid. **The measurement is specified here, at a named level,
with the three branches pre-committed — that specification is the deliverable's
value, and a second unspecified run would not have it.**
