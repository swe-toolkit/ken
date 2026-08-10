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

| question | owner |
|---|---|
| Should `subst_outer` bound the index and refuse rather than panic? | Architect (mechanism) |
| Is refusing a **contract change** to a kernel function, or defensive behaviour within the existing contract? | Architect |
| Does adding a refusal path to a kernel primitive grow or shrink the TCB? | **Operator — I forward this, I do not decide it** |

**The second question is the one that decides whether this is small.** `D10`'s
frame made a `subst_outer` contract change a stop condition precisely because
the distinction was not obvious then and is not obvious now. A bounds check that
returns an error is a new failure mode in a trusted primitive; a bounds check
that cannot fire is dead code in the TCB. **Neither is free, and that is the
argument for ruling rather than for quietly adding one.**

## Why draft rather than ready

**It needs a ruling before it can be framed, and it blocks nobody.** Both build
lanes are working on unrelated nodes. Filing it `ready` would claim a shovel-ready
frame exists; filing it `draft` records the measurement durably and routes the
question, which is the honest state. Same disposition as
[[LANG-SORT-META-CAPABILITY]].

**The alternative disposition is legitimate and should be considered on the
merits:** state the reachability claim with its scope wherever it is recorded,
and close this without code. That survives only until a thirtieth caller is
added without scrutiny, which is the trade the Architect is being asked to
price.
