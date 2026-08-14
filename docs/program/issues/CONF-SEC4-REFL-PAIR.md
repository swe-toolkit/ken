---
id: CONF-SEC4-REFL-PAIR
title: "Sec4's C1/C2 refl pair is stale against ADR-0013: the suite half of the repair landed with SEC4-TCB, the seed half did not"
status: ready
owner: spec-enclave
size: S
gate: none
depends_on: [SEC4-TCB]
blocks: []
github: null
origin: "Raised by verify-implementer's SEC4-TCB D2 census hard stop (evt_31jhd3mc169gk, 2026-07-27) at a78a7dae; mechanism verified independently by the Steward at obs.rs (then :113, now :110). Ruled out of SEC4-TCB's scope in evt_ff4m551h40fz and filed here. Re-measured and rescoped by the Steward 2026-08-14 at b217d8c5 — see the banner."
---

> # RESCOPED 2026-08-14 — HALF OF THIS NODE IS ALREADY LANDED. READ THIS FIRST.
>
> **`SEC4-TCB` is `merged`, and it acted on this node by name.** The prior
> revision of this file was written when nothing executed the Sec4 seed and it
> described the whole repair as outstanding. **That is now a claim about the
> past.** Two things changed, and only one of them is done.
>
> **DONE — the suite.** `crates/ken-elaborator/tests/sec4_acceptance.rs` binds
> the re-scoped pair and carries the honest control this node asked for. It
> cites `CONF-SEC4-REFL-PAIR` at `:34` as its warrant.
>
> **NOT DONE — the seed.** `seed-trust-model.md`'s C1 and C2 rows still carry
> the original `expect` and `why` prose, which the suite has superseded.
> **This node's own warning describes the current tree exactly:** *"Do not
> silently re-point the rows and leave the `why` text intact."* The suite was
> re-pointed; the seed text was not.
>
> **The residual is the seed alone, and it is `S`.** Do not re-derive the
> repair strategy — it is settled and landed. Do not touch the suite.
>
> **The old `status: draft` reason is spent.** It read *"`SEC4-TCB` is in
> flight and is bound to the re-scoped pair"*. `SEC4-TCB` is merged, so the
> condition that withheld this node from the frontier has cleared.

## What the suite landed, so nobody rebuilds it

`sec4_acceptance.rs` supersedes the closed-literal operands with two tests:

- **`kernel_check_flips_on_abstract_index_convertibility_without_provenance`**
  — the re-scoped AC3 control. In one two-binder context the same `Refl x`
  certificate is accepted at `x = x` and rejected with `BadEliminator` at
  `x = y`, through the four-argument kernel API. Its own doc comment names the
  gap rather than hiding it: *"distinct binders are unprovable rather than a
  closed false proposition; this deliberately does not retain the seed's
  truth-valued framing."* The four-argument call site doubles as C3's
  compile-time structural pin.
- **An honest control for the superseded operands** — asserting what the closed
  arms actually do: the registered-literal reducer maps `0 = 0` to `Top` and
  `0 = 1` to `Bottom`, and `Refl` at the latter rejects with `TypeMismatch`
  before the `Eq` conversion arm is reached.

That is this node's own suggested disposition, implemented: re-scope the pair
onto abstract binders, and retain the closed arms as their own rows asserting
the `Top`/`Bottom` collapse.

## The measurement, re-verified at `b217d8c5`

`conformance/security/trust-model/seed-trust-model.md`, group **C**
(Authorship-independence, AC3), rows `false-proposition-certificate-rejected`
(C1, `:234`) and `genuine-proof-accepted` (C2, `:247`).

Landed behavior, `crates/ken-kernel/src/obs.rs:110`
(`eq_at_registered_literal`, ADR-0013 Layer 2):

```
Eq ty (IntLit m) (IntLit n)  ⇝  Top     if m == n
                             ⇝  Bottom  if m != n
```

| row | seed operand | seed `expect` | landed behavior |
|---|---|---|---|
| C2 | `refl` at `Id Nat 0 0` | accepts | unreachable — goal reduces to `Top`, so the `Term::Eq` arm never fires |
| C1 | `refl` at `Id Nat 0 1` | rejects *"conversion fails, `0 ≢ 1`"* | rejects, but conversion is never reached — goal reduces to `Bottom` |

## Why C1 is the worse half

C2 fails loudly and gets looked at. **C1 passes.** Its stated mechanism —
conversion failure — is not the mechanism that produces its verdict, and
nothing in the seed distinguishes the two. A rejection control that passes for
any reason is not a control.

## The defect is the PAIR, not either row

The seed's own `why` states the invariant: *"the **only** difference is the
proposition's truth."* Against landed behavior **both** arms bypass `Refl`'s
conversion check entirely and differ only in which constant the reducer picks.

The pair no longer isolates AC3. What it demonstrates — that the reducer
decides closed literal equalities before the certificate is consulted — is true
and interesting, and is **not** authorship-independence.

`Equal Int x y` at distinct binders is **unprovable**, not **false**, and the
seed is framed on the proposition's truth. The landed suite chose the abstract
framing and said so; **the seed must now say the same thing in its own voice**,
rather than keep asserting a truth-valued flip that no longer runs.

## Stale locators, measured at `b217d8c5`

Repair these in the same pass; each is a citation that now resolves to the
wrong object.

| citation | sites | resolves to today | correct |
|---|---|---|---|
| `check.rs:373` for the `check` entry | seed `:35`, `:237`, `:250`, `:259` | `check_level_arity` | **`check.rs:386`** |
| `obs.rs:113` for `eq_at_registered_literal` | this node's `origin`, `sec4_acceptance.rs:34` | three lines into the function's body | **`obs.rs:110`** |

The suite's header block at `:30` already re-derived four other stale seed
locators (`trusted_base`, `declare_postulate`, the prover's hole admission,
`is_prelude`). **It did not catch `check.rs:373`**, because that locator sits
in the seed rather than in the suite's own citations.

## Third instance of one class

A conformance row whose required operand cannot be constructed is
byte-identical, to any reader, to one not yet built. See [[CONF-FMT8-LEVELTOK]]
(no Level token kind) and [[SEC1-IFC-R3]] (no production route to `Disproved`).

This one adds a worse face: the *sibling* row stayed green while measuring a
different mechanism. The other two are silent-red; this is silent-green. A
sweep keyed only on never-green rows would not have found it.

**And the rescope adds a fourth face, which is the one to carry forward:** once
the suite was re-pointed and the seed was not, the corpus held a row whose
prose and whose executing test disagreed about what the row measures. That is
invisible to both a never-green sweep and a currency check on the suite.
