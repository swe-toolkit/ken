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

## AC-0 ruling (Architect `evt_22nte6kpr12t9`, probed at `3150e9331`)

- **A named step function**, not a trace reading. `private_write_all_fuel`
  is reshaped into `private_write_all_after_wrote`,
  `private_write_all_step` and the loop, plus `private_write_all_next`
  after `writeAll`. The four declarations are as given in the ruling.
- **The laws live prelude-side**, after `writeAll` and before
  `hide_prelude_names`. The new helpers and `write_all_refl` join
  `private_names`, and the theorem names stay public. The step must stay
  private: it would otherwise produce a public `BufferSpan`.
- **The premise** (respelled by Architect `evt_7s1szzxdss4s6`).
  `write_all_count_fits span count` in `System.IO` is
  `add (transfer_count_nat count) (transfer_count_remaining count) =
  buffer_span_budget span`, with `add` from `Data.Numeric.Nat.Arithmetic`.
  It holds on each `Wrote` response, and `Err` responses carry no premise.
  - `System.IO` does not import `Data.Numeric.Nat.Order`, whose trust
    closure carries `LawfulClasses` axioms.
  - The prelude adds the public `transfer_count_predecessor` and
    `transfer_count_nat_succ`.

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

- **AC-1 (amended to the AC-0 ruling, Architect `evt_22nte6kpr12t9`).**
  - Each clause is a pairing. The loop half is a prelude law whose
    statement names `writeAll` or the loop: `write_all_entry`, `_stop`,
    `_request`, `_first_error_step`, `_done`, `_continue` and
    `_advance_start`. The arithmetic half is a `System.IO` theorem (S1-S3,
    in additive-witness form, `evt_7s1szzxdss4s6`) under
    `write_all_count_fits`. No single catalog theorem can name the
    loop (G1, G2, visibility).
  - Both halves are checked in the `System.IO` load. The `System.IO` card
    maps each §1.7.3 clause to its laws.
  - `cat_capability_laws_prelude_move` passes unchanged, and `System.IO`
    adds no trusted declaration.
- **AC-2 (controls).**
  - Success completeness without the response premise is rejected, and the
    failure is the fuel-`Zero` case.
  - First error with a different error on the right is rejected. It names
    private constants, so it is a Rust-side test: it elaborates before
    `hide_prelude_names`, or it kernel-checks the false `write_all_refl`
    instance. If neither route exists without a production change, stop to
    the Architect.
  - The premise-free S2 is a `System.IO` `ken reject` fence.
- **AC-2b (SEAL-2 oracle, Architect `evt_zek8rgw9ssg7`).** Scope adds
  `crates/ken-elaborator/src/seal2_tests/support.rs` and
  `seal2_tests/adversary_repros.rs`. This is test support only.
  - `result_type_produces` classifies the unreduced type before it reduces
    it. An Ω-sorted result is not a producer, so `write_all_request` stays
    public with its statement unchanged.
  - Reverting to reduce-then-classify turns both TransferCount closure pins
    red with `{"write_all_request"}`.
  - The alias-producer repro and every existing positive repro stay green.
  - A new positive control: a public `fn` whose result is a `def` alias of
    `TransferCount` under a Π is still flagged.
  - Each `ken reject` fence rejects a false claim and accepts its adjacent
    true twin. A fence closed by `Refl` on a closed inductive equation
    rejects true twins too, and does not count (Architect
    `evt_bbf8w6ww9208`).
- **AC-3.** These suites stay green:
  - `px8f_buffer_io_surface`, with the new private names in its sealed
    list;
  - `cat_capability_laws_prelude_move`;
  - the `System.IO` catalog load;
  - `seal2_tests::adversary_repros`;
  - `writeAll` execution behavior is unchanged: the executing
    `ken-verify` `px8f_write_partition` row and the buffer-io conformance
    seed, since the step closure changes the lowered shape. The
    `rt_parity_native.rs` range is a source-scope seal, not an execution
    row.
  - the full `lang_mod_strict_resolution_d0` target (Architect
    `evt_70qeaw87agv03`). Scope adds
    `crates/ken-elaborator/tests/lang_mod_strict_resolution_d0.rs`, for one
    change: the `Capability.System.IO` row of
    `catalog_ambient_passthrough_migration_census` gains the five
    prelude-intrinsic names `buffer_span_budget`, `transfer_count_nat`,
    `transfer_count_nat_succ`, `transfer_count_predecessor` and
    `transfer_count_remaining`, with a comment saying why. If any other row
    moves, stop to the Architect.

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

## Hard-stop inventory (§1b)

§1a count: 2 (Architect `evt_zek8rgw9ssg7`).

1. The premise was spelled over a prelude name that is migrating to a
   catalog owner, and the bounds used a trust-carrying module (keyed on
   assuming a name's home and its trust closure without measuring the
   module graph). Ruled `evt_7s1szzxdss4s6`.
2. A proof-only public theorem whose statement mentions private carriers
   was classified as a producer (keyed on the reduced form of the type
   rather than its sort).
