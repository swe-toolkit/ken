---
id: KERNEL-SUBST-OUTER-INDEX-SCOPE
title: "Rule whether kernel subst_outer should bound its parameter index -- it panics on an out-of-range params[p_idx] and is defended only by a reachability argument enumerating one of its 29 call sites"
status: draft
owner: spec-enclave
size: S
gate: none
depends_on: []
blocks: []
github: null
origin: Adversary finding evt_52q0gzgt9gwnd on KERNEL-NESTED-IND D10 (cfc86a83), measured on 145a4dd0. Filed draft rather than ready because it needs an Architect ruling and an operator TCB call before it can be framed; nothing is blocked on it and D10 is not affected. The Steward re-measured the call census and it is larger than the report stated.
---

## What it is

`crates/ken-kernel/src/subst.rs:245`, inside `subst_outer`:

```rust
let p_idx = inner_depth + m - 1 - *i;
weaken(&params[p_idx], inner_depth as i64)
```

**The index is unchecked and panics when `params` is shorter than `m` implies.**
That is what `KERNEL-NESTED-IND` `D9` hit: a checked, erased artifact reached
`method_type` with an empty `params` for a parameterized host, and the
interpreter panicked rather than refusing.

**`D10` repaired the caller, and that siting is right.** It fixed the
coordinate-origin introduction in `lift_recursive_value` rather than the layer
nearest the panic. This node is not a criticism of that repair and does not
reopen it.

## The measured question

**`D10`'s defence is a reachability argument, and this node asks how wide the
enumeration behind it actually is.** The claim as stated after the merge — that
"the caller can no longer reach it with a mismatched host" — is singular, and
the function's exposed surface is not.

Measured at `145a4dd0`, `subst_outer(` call sites outside its own module:

| location | sites | trust |
|---|---|---|
| `crates/ken-elaborator/src/elab.rs` | 15 | outside the kernel |
| `crates/ken-kernel/src/obs.rs` | 7 | **inside the kernel** |
| `crates/ken-kernel/src/inductive.rs` | 5 | **inside the kernel** |
| `crates/ken-kernel/src/check.rs` | 2 | **inside the kernel** |
| **total** | **29** | **14 in-kernel** |

**This corrects the Adversary's own census in the direction that strengthens
its point.** The report gave roughly 20 sites with two inside the kernel;
measured, there are 29 with **14** inside the kernel — seven times the in-kernel
figure. An unchecked index defended by reachability is only as strong as the
enumeration behind it, and the enumeration performed so far is **one path: the
one that panicked.**

## What is NOT in question

- **`D10` is sound and stays.** Both its properties were discharged and the
  repair sits at the right layer.
- **No caller is known to be defective.** The other 28 sites construct `params`
  from the declaration. Nothing here is a reported bug; this is a question
  about a defence's scope, not a claim that it fails.
- **This is not a `main` safety concern.** Ken has no users, and "a working
  path could go red" is not a grounded constraint.

## The three questions, and they route differently

| question | owner | state |
|---|---|---|
| Should `subst_outer` bound the index and refuse rather than panic? | Architect (mechanism) | **RULED — yes** |
| Is refusing a **contract change** to a kernel function, or defensive behaviour within the existing contract? | Architect | **RULED — it is a contract change** |
| Does adding a refusal path to a kernel primitive grow or shrink the TCB? | **Operator — I forward this, I do not decide it** | **forwarded 2026-08-10, `evt_561jx5e0ffy40`** |

**The second question is the one that decides whether this is small.** `D10`'s
frame made a `subst_outer` contract change a stop condition precisely because
the distinction was not obvious then and is not obvious now. A bounds check that
returns an error is a new failure mode in a trusted primitive; a bounds check
that cannot fire is dead code in the TCB. **Neither is free, and that is the
argument for ruling rather than for quietly adding one.**

## The Architect's ruling, recorded 2026-08-10

Ruled at `evt_4aqgwcanhhkgn` (thread `thr_6b5f267m16jwk`). **Recorded here
because a ruling that lives only in a thread is not a durable deliverable** —
this node, not the thread, is what a future framer reads.

**Both Architect questions resolve yes.** The reachability defence is not an
adequate permanent boundary for a public cross-crate primitive with 29 external
call sites, 14 of them in-kernel: it can justify `D10`'s local repair, but it
cannot make unchecked indexing an invariant carrier for future callers. And
returning an error adds an observable outcome and forces caller propagation, so
calling it merely defensive would hide the real API/TCB decision.

**The classification does not make the change semantically expansive.** On valid
inputs the resulting term must be byte-for-byte identical; the new outcome exists
only where the old function was undefined-by-precondition and actually panicked.
At the kernel entry surface that is a refinement toward the existing rule that
raw-well-formed input yields yes/no or a minimal reason, never a crash.

**The guard must cover the whole arithmetic domain**, not merely swap
`params[p_idx]` for `params.get(p_idx)`:

- establish `params.len() == m` once at the public entry;
- for a variable at or beyond `inner_depth`, compute the relative index with
  checked/subtractive structure and require it strictly below `m`;
- only then derive the reversed parameter index;
- preserve variables below `inner_depth`;
- return a typed substitution/scoping error for either arity mismatch or a
  variable beyond `inner_depth + m`.

**No unsigned underflow, wrapped index, unchanged invalid variable, sentinel
term, or panic is an acceptable failure policy.** The old unchecked public route
must not remain beside a checked one — the thirtieth caller would simply select
the easier API.

### What this does to the sizing, and it is the operative constraint

**Not shovel-ready as size S on Architect authority alone.** The frame must bind
all 29 current sites, not merely the `D10` path. Every current caller propagates
or contextually translates the refusal, and **no caller may `unwrap` it**.
Required controls: both arity directions, the first out-of-scope outer variable,
the two valid endpoint parameters under nonzero inner depth, and at least one
additional binder descent. Valid-domain equivalence is pinned across
representative `check`, `obs`, `inductive`, and elaborator consumers.

> **There is no interim partial.** The Architect ruled out landing a panic-only
> assertion, and ruled out a bounds check that cannot reach a typed boundary.
> If authorized, the typed refusal and the full caller migration are framed and
> land **together**. A Steward tempted to cut this smaller should read that as
> the ruling forbidding the cut, not as a sizing problem to solve.

**`D10` is not reopened.** Its coordinate-origin repair remains sound and
necessary. The ruling replaces a census-dependent absence claim with a local
totality boundary; it does not claim another caller is defective.

## Why draft rather than ready

**It needs a ruling before it can be framed, and it blocks nobody.** ~~Both build
lanes are working on unrelated nodes.~~ Filing it `ready` would claim a
shovel-ready frame exists; filing it `draft` records the measurement durably and
routes the question, which is the honest state. Same disposition as
[[LANG-SORT-META-CAPABILITY]].

> ### THE PREMISE THAT SENT THIS TO THE OPERATOR HAS CHANGED — Steward,
> ### 2026-08-12
>
> **Both Architect questions are RULED. The sole remaining gate is the operator
> TCB call forwarded 2026-08-10 at `evt_561jx5e0ffy40`, and it is unanswered.**
> Nothing above is reopened; what changed is the cost of leaving it open.
>
> **The struck sentence was true when written and is false now.** Measured
> 2026-08-12 ~21:0xZ: **the Kernel ring is idle on all three seats** —
> leader, implementer and QA all report awaiting an explicitly released
> deliverable. [[KERNEL-NESTED-IND]] stays correctly `active` with its only open
> criterion `AC-K12`, which is **Runtime-blocked** behind
> [[RT-DYNAMIC-ARM-SCALAR-MERGE]] and [[RT-NESTED-IH-NATIVE-REALIZATION]] and
> which the Steward ruling in that node forbids releasing a lane against.
>
> ⇒ **Kernel's idleness is a genuine block, not framing debt** — and this node
> is the one kernel-adjacent item whose gate is **not** Runtime. That does not
> make it urgent and it does not change a single technical fact above. **It
> changes only the answer to "what does waiting cost", from "nothing" to "the
> one lever that is not behind Runtime."**
>
> **Be accurate about what authorization would buy:** `owner` here is
> `spec-enclave`, so an all-clear yields a **framable** node, not an immediately
> releasable Kernel WP — and the Architect's no-interim-partial ruling means the
> typed refusal and the full 29-site caller migration are framed and land
> together. Whether that work sits in Kernel's lane is a framing call that has
> not been made. **Do not restate this as "authorizing it puts Kernel back to
> work."**

**The alternative disposition is legitimate and should be considered on the
merits:** state the reachability claim with its scope wherever it is recorded,
and close this without code. That survives only until a thirtieth caller is
added without scrutiny, which is the trade the Architect is being asked to
price.
