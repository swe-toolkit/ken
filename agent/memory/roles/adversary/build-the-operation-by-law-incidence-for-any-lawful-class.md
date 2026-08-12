---
name: build-the-operation-by-law-incidence-for-any-lawful-class
description: For any lawful class, list the dictionary's operations against the law statements — an operation appearing in ZERO laws is unconstrained, and if it has a generic consumer, instances will silently assign it contradictory meanings; the divergence first becomes observable at the second instance
scope: roles/adversary
---

# Build the operation × law incidence for any lawful class

**Measured 2026-08-10 on `0daf7170` (DS-9 `D2`).**

`CursorOps` is a four-operation dictionary — `remaining`, `peek`, `advance`,
`locate` — shipped with a `CursorLaws` conjunction of three laws. Reading the
law statements:

| law | operations it mentions |
|---|---|
| `CursorPeekHasRemaining` | `peek`, `remaining` |
| `CursorAdvanceProgress` | `peek`, `advance`, `remaining` |
| `CursorEndValid` | `remaining`, `peek` |

**`locate` appears in zero laws.** It is a member of the dictionary that nothing
constrains, so any instance may define it as anything and `CursorLaws` still
holds.

⇒ **The audit is mechanical and cheap.** For any class/trait/dictionary shipped
with laws: enumerate the operations, enumerate the laws, and build the
incidence. **An operation in zero laws is unconstrained.** Then ask the only
question that makes it a finding rather than a curiosity: **is there a generic
consumer?** Here there was — `decoder_fail` reports
`DecoderRejected loc (cursor_locate …)`, generic over any `CursorOps`.

## The two instances disagreed, oppositely

- `arg_cursor_locate` — a forward offset that **increases** on advance.
- `char_cursor_locate` — defined as `remaining`, so it **decreases** on advance.

One counts from the start, the other counts to the end, both typecheck, both
satisfy every law, and the generic consumer cannot tell them apart. A rejection
reported at *"7"* means opposite things depending on the instance.

**The divergence is invisible at one instance.** With a single implementation
an unconstrained operation is indistinguishable from a constrained one — the
instance simply *is* the definition. ⇒ **The second instance of any lawful class
is the high-yield moment**, and it is exactly when nobody is looking, because
the interface "already shipped." Same family as
[[transport-schema-degenerate-endpoint-trap]]: the first endpoint is always
degenerate.

**Attribute it correctly.** The missing law is the *interface's* gap, not the
new instance's — the new instance is only where it became observable. Say so, or
the finding lands on the wrong node. And note whether the newcomer **disclosed**
its choice: DS-9's §3 did. That makes the defect *"the disclosure is local while
the consumer is generic"*, not dishonesty.

## Read the proof terms; a trust-delta gate cannot see vacuity

The same merge asserted exact `trusted_base()` set identity, and it is tempting
to read that as evidence the laws are real. **It is not** — a hollow law adds no
Axiom and has zero delta either way
([[a-vacuous-law-has-zero-trust-delta]]), so set identity is the wrong
instrument for this question.

**Read the proof bodies instead**, and look for arms that supply a *real term*
versus a constant: here `advance_progress` supplies
`char_cursor_lt_suc (remaining tail)` with `char_cursor_lt_suc` a genuine `Nat`
induction, and the impossible branches discharge via `absurd` on an
uninhabitable peek. That is a proof by computation, not a placeholder — and it
took one read to establish what no amount of trust-ledger assertion could.
