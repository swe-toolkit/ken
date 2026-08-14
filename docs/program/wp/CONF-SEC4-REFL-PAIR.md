# WP frame: CONF-SEC4-REFL-PAIR

**Node:** `docs/program/issues/CONF-SEC4-REFL-PAIR.md`
**Owner:** spec-enclave
**Size:** S
**Measurement base:** `b217d8c5` (`origin/main`, 2026-08-14)
**Candidate base:** whatever `origin/main` is when you cut; re-measure the
fixed inputs below if it has moved off `b217d8c5`.

## Objective

Bring the Sec4 trust-model seed's C1/C2 rows into agreement with the acceptance
suite that already supersedes them, and repair four stale `check.rs` locators
in the same file.

**The repair strategy is settled and half-landed. Do not re-derive it.**
`SEC4-TCB` (merged) re-pointed the executing pair onto abstract binders and
added an honest control for the closed operands. This WP is the seed-side half
that did not land with it.

## The design judgment, front-loaded

Three calls are already made. You are implementing them, not reopening them.

**1. The rows move to the abstract framing, not to a repaired closed one.**
`eq_at_registered_literal` (`obs.rs:110`) reduces closed registered literals to
`Top`/`Bottom` before `Refl`'s conversion check is consulted, so no choice of
closed operands can restore the conversion boundary the rows claim to measure.
Abstract binders leave the goal unreduced, which is why the landed test uses
them.

**2. The `why` prose changes, not only the operands.** The seed frames C1/C2 on
*the proposition's truth*; distinct abstract binders make the goal
**unprovable**, not **false**. The landed test states this gap explicitly
rather than papering it. The seed must state it too, in its own voice. A
re-pointed operand under unchanged `why` text is the exact failure this node
was filed to prevent.

**3. The closed arms are kept as their own rows.** `Top`/`Bottom` collapse is
real landed behavior worth pinning, and it is the honest home for the mechanism
C1 currently mislabels. The suite already asserts it; the seed should carry the
matching rows.

## Fixed inputs, measured at `b217d8c5`

| object | location | fact |
|---|---|---|
| C1 `false-proposition-certificate-rejected` | `conformance/security/trust-model/seed-trust-model.md:234` | `expect` claims rejection by conversion failure; conversion is never reached |
| C2 `genuine-proof-accepted` | same file, `:247` | `expect` claims acceptance; goal reduces to `Top`, the `Term::Eq` arm never fires |
| C3 `check-signature-exposes-no-provenance-channel` | same file, `:259` | sound and unaffected — do not modify except its stale locator |
| `eq_at_registered_literal` | `crates/ken-kernel/src/obs.rs:110` | `IntLit m`/`IntLit n` reduce to `Top` if equal, `Bottom` otherwise; returns neutral when either side is not a literal |
| `pub fn check` | `crates/ken-kernel/src/check.rs:386` | the four-argument kernel entry |
| stale `check.rs:373` | seed `:35`, `:237`, `:250`, `:259` | now resolves to `check_level_arity` |
| landed re-scoped pair | `crates/ken-elaborator/tests/sec4_acceptance.rs`, `kernel_check_flips_on_abstract_index_convertibility_without_provenance` | `Refl x` accepted at `x = x`, rejected `BadEliminator` at `x = y`, one two-binder context |
| landed honest control | same file, the closed-operand test | `0 = 0` to `Top`, `0 = 1` to `Bottom`, `Refl` at the latter rejects `TypeMismatch` |
| suite warrant | same file, `:34` | cites `CONF-SEC4-REFL-PAIR` by name |

## Deliverables

**D1.** Re-point C1/C2 onto the abstract-binder framing the suite executes, and
rewrite their `expect` and `why` so neither asserts a truth-valued flip. Each
row names the executing test function.

**D2.** Add the closed-literal rows asserting the `Top`/`Bottom` collapse, each
naming the suite's honest control as its executing test.

**D3.** Correct `check.rs:373` to `check.rs:386` at all four sites in the seed.

**D4.** Update the node's own `origin` locator (`obs.rs:113` to `obs.rs:110`).

## Acceptance criteria

**AC-1.** No row in group C asserts that the C1/C2 verdict flips on the
proposition's truth. **Control:** the phrase *"the only difference is the
proposition's truth"* and any restatement of it is absent from the group.

**AC-2.** Every group-C row, and every new closed-literal row, names the
`sec4_acceptance.rs` test function that executes it. **Control:** grep the seed
for each test's function name and require a hit for each row. This is the AC
that makes a future seed/suite divergence visible instead of silent.

**AC-3.** `check.rs:373` appears nowhere in `conformance/`. **Control:** grep
returns zero. `check.rs:386` resolves to `pub fn check` at the candidate base.

**AC-4.** The abstract framing is stated as *unprovable*, not *false*, wherever
the rows describe the negative arm.

**AC-5.** `ken-cargo test -p ken-elaborator --test sec4_acceptance` is green at
the candidate base. It should be untouched by this WP — this is a control that
you did not edit the suite, not a control on new behavior.

## Banned scope

- **Do not edit `crates/`.** The suite is Team Verify's artifact and is already
  correct. Its one stale `obs.rs:113` comment locator at `:34` is recorded in
  the node's locator table for whoever next touches that file; it is **not**
  this WP's to fix, and taking it would make a single-team WP cross teams.
- Do not modify C3 beyond its `check.rs` locator.
- Do not add a conformance currency checker. None exists, and building one is
  not grounded in this node.
- Do not re-scope the pair a second time or revisit the abstract-binder choice.

## Contention

None. Every other node touching `conformance/security/trust-model/` is `merged`
or `closed`; `SEC4-TCB` is merged. The spec enclave's in-flight node
(`SPEC-AUTH-EX`) touches `spec/60-security/62-authority.md`, a different file.

## Note for the reader who lands this

The reusable lesson is in the node's closing section. Once the suite was
re-pointed and the seed was not, the corpus held a row whose prose and whose
executing test disagreed about what the row measures — invisible both to a
sweep for never-green rows and to a currency check on the suite. **AC-2 is the
enforcement of that lesson**; it is the reason the rows must name their tests
rather than merely be covered by them.
