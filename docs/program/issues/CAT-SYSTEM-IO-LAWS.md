---
id: CAT-SYSTEM-IO-LAWS
title: "System.IO's five writeAll theorems prove facts about helper functions, not about writeAll: write_all_first_error error = Err error, write_all_complete Zero = True. Spec 38 §1.7.3 requires kernel-checked terms over the real loop for strict decrease, fuel sufficiency, success-implies-full-transfer and error-prefix preservation. Prove them over the actual writeAll tree at zero TCB"
status: ready
owner: foundation
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Catalog proof-completeness survey row Capability/System/IO (CAT-SYSTEM-IO-LAWS recommendation), grounded in spec 38 §1.7.3. L3 proof backfill (operator 2026-09-13). Steward-filed per COORDINATION section 2."
---

# writeAll's theorem, over writeAll

## Objective

Spec 38 §1.7.3 (`38-ffi-io.md:651-690`) requires kernel-checked terms for
the `writeAll` theorem over the transparent loop. These are termination,
exact prefix, success completeness, first-error preservation, and the
all-success corollary. None may be an `Axiom` or a postulated theorem. The
catalog proves each clause about `writeAll` itself.

## Settled inputs (read at `c4f1812f4`)

- **The loop.**
  - `private_write_all_fuel` (`prelude.rs:2593-2620`) binds `writeAt`, then
    matches the outcome with an inline continuation:
    - `Err e` gives `Ret (Err e)`;
    - `Wrote count` with zero remaining gives `Ret (Ok MkUnit)`;
    - `Suc remaining` recurses on the advanced offset and on
      `write_all_advance_span span count`, with fuel `rest`.
  - Fuel `Zero` returns `Ok MkUnit`.
  - `writeAll` (`:2623`) starts it at `buffer_span_budget span`.
- **The current theorems observe helpers only.**
  - `IO.ken.md:21-43` proves `write_all_call_bound fuel = fuel`,
    `write_all_complete Zero = True`, `write_all_first_error e = Err e` and
    `write_all_all_success fuel = True`.
  - It also proves `write_all_exact_prefix_prop`, which is `Refl` on the
    advance function.
  - No statement mentions `writeAll`, `private_write_all_fuel` or `writeAt`.
- **Fuel sufficiency needs a response premise.**
  - `TransferCount` is `PrivateTransferCount Nat Nat`, with count
    `Suc predecessor` and a separate `remaining` (`prelude.rs:1722`,
    `:1798-1806`).
  - Nothing in the type ties `count + remaining` to the span's budget.
  - With a response where `Suc remaining > rest`, the loop hits fuel `Zero`
    with bytes left and returns `Ok`.
  - So success completeness holds only under the host write contract.
    Spec 38 treats that contract as a reasonable-from contract of the opaque
    operation (`:646-649`).
  - The Ken statement carries it as a hypothesis on each response, in the
    way the zero-TCB String laws are hypothesis-carrying (operator
    2026-09-30). It is not a new `Axiom`.
- **Visibility.**
  - `private_write_all_fuel`, `PrivateBufferSpan` and `PrivateTransferCount`
    are removed from the source name map. `writeAll`, `writeAt` and the
    span and count projections are public.
- **Consumers of the helper names:**
  - `IO.ken.md`;
  - `crates/ken-elaborator/tests/cat_capability_laws_prelude_move.rs:11-60`;
  - `px8f_buffer_io_surface.rs:124`;
  - `lang_mod_strict_resolution_d0.rs`;
  - `seal2_tests`;
  - spec ledger `30-intrinsic-ledger.md:216-225`.
  - The helpers stay; this WP adds the laws.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## AC-0 (Architect, before the build)

Rule the statement form. It must be writable in the vocabulary the code
delivers. One obligation it must be able to state is first-error
preservation: for every fuel `Suc rest`, span and error `e`, the tree
`private_write_all_fuel … (Suc rest)` continues with `Ret (Err e)` after
`writeAt` answers `Err e`.

The ruling decides three things:

- whether the continuation becomes a named, same-representation step
  function, or the laws are stated through a pure response-trace reading of
  the tree;
- where the laws live, prelude-side or in `System.IO`, given the private
  names;
- how the host response premise is spelled.

## Deliverable

`System.IO` publishes kernel-checked theorems over `writeAll`, or over the
loop it names, for the five §1.7.3 clauses:

- termination as at most `L` `writeAt` requests;
- exact prefix: the k-th request's offset and span start advance by the
  running `N`, and the remaining length is `L - N`;
- success completeness under the response premise;
- first-error preservation;
- the all-success corollary.

There is no `Axiom`, no `trusted_base()` change and no kernel change.

## Acceptance

- **AC-1.**
  - Each clause is a theorem whose statement names `writeAll` or the loop
    AC-0 names.
  - Each is checked in the `System.IO` load.
- **AC-2 (controls).**
  - Success completeness without the response premise is rejected, and the
    failure is the fuel-`Zero` case.
  - First error with a different error on the right is rejected.
  - Each `ken reject` fence rejects a false claim and accepts its adjacent
    true twin. A fence closed by `Refl` on a closed inductive equation
    rejects true twins too, and does not count (Architect
    `evt_bbf8w6ww9208`).
- **AC-3.** These suites stay green:
  - `px8f_buffer_io_surface`;
  - `cat_capability_laws_prelude_move`;
  - the `System.IO` catalog load;
  - `writeAll` execution behavior is unchanged.

## Stop conditions

- A clause that cannot be stated without a new public producer of
  `BufferSpan` or `TransferCount`: stop to the Architect.
- Any new `Axiom`, trust change or spec change: stop to the Steward, since
  TCB growth goes to the operator.

Checks: 4 fired. I wrote the first-error obligation in code vocabulary; it
needs a named continuation or a trace reading, so AC-0 rules it. Success
completeness is false without the response premise (fuel `Zero` returns
`Ok`). 3 fired: I grepped `.rs`, `.md` and `.ken` across the repository for
the helper names and found six consumer sites, none of them migrated.
