---
id: LANG-PRELUDE-NONHOME-FAMILIES-HOME
title: "Deleting Legacy resolution would strand pre-source names that tests and the spec treat as public but no package exports: 48 checked and saturating fixed-width operations, RangeError and three ABI-scalar narrowings, Decimal and its operators, isScalar and inRangeBool, and zip. Home each family at its existing identity, or route it through the surface it already has, as the Architect ruled by registration site, so I4 can delete Legacy"
status: ready
owner: language
size: L
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect evt_13j52ac43jrw8 and evt_3pxvhj0yngpxm on LANG-DELETE-LEGACY-RESOLUTION I1's 85-spelling census (implementer evt_bj4em6fyrmz3): one homing node, landing before I4. Operator 2026-09-25: one resolution mode. Steward-filed per COORDINATION section 2."
---

# Every public prelude name has a route

## Objective

Every pre-source name the I1 census found positively used without a
package home has a Strict route at its existing identity: an
explicit-import home (H), a prelude floor member (B), or an existing
surface form (R). After this node, I4 can delete Legacy without
stranding a positive use.

## Settled inputs (main `68d4a6ab4`)

The scope is the Architect's ruling `evt_3pxvhj0yngpxm` ("THE HOMING
NODE" and sites 1 to 5), which is the contract for this node. In short:

- **H in `Data.Numeric.FixedWidth`.** The 48 `checked`/`saturating`
  `{Add,Sub,Mul}` operations over the eight widths (`conversions.rs`
  :244-310), and `RangeError`, `MkRangeError` and
  `intTo{USize,ISize,CInt}` (`:301`-`:365`). FixedWidth exports only the
  eight `intTo{W}` today. The raw narrowings stay unexported.
- **H in `Core.Classes.LawfulClasses`.** `isScalar` and `inRangeBool`
  (`decimal_char.rs` :238-257), beside `charToInt` and `intToChar`. The
  lower and upper scalar helpers stay unexported.
- **B, `Decimal`.** A floor member at `decimal_id`. Literal typing
  (`elab.rs` :16166) and `+`/`==` dispatch (`decimal_char.rs` :207,
  :217) key on its exact id; spec 35 §2.3 names it. The pair
  representation stays internal.
- **R, Decimal arithmetic.** `decimalAdd` and `decimalEq` through `+`
  and `==`. `-` and `*` at Decimal have no dispatch entry today
  (`numbers.rs` :752-755); this node registers them to `decimalSub` and
  `decimalMul`.
- **R, `zip`.** `Data.Collections.Derived`'s private List zip
  (`Derived.ken.md` :1564, with three proved laws) becomes public, and
  spec 37 §4 and §9 settle it there.
- **Consumers.** Five user sources declare a same-spelled `Decimal`
  (listed in the ruling) and rename their model type, keeping their
  verdicts. Importers of the three packages are swept for local spellings
  of the new exports.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Amendment: formers named by public signatures (2026-11-15)

Steward recut `evt_7pm6wefbx2mb` on LANG-DELETE §1a 9; form ruled by the
Architect in `evt_5w50s3dsa2k0f` (part A) on Research `evt_6jq14qkszqzar`.
A public catalog export whose checked type names a pre-source former
with no home can be called from Strict source, but the types of its
arguments cannot be written there. This node homes those formers too.

- **Census (Architect `evt_1hkvqvwcwcfy`, replacing the earlier rule).**
  A former is a homing obligation when it occurs in the checked type of
  a public catalog export. The walk is transitive through argument
  positions. It unfolds a transparent definition only when that
  definition's head is not Strict-writable, which means a floor member, a
  strict native builtin or a `PRELUDE_PACKAGE_HOMES` row. A head that is
  not Strict-writable is itself the obligation and is homed as that
  alias, not as its body. Formers reached only beneath a Strict-writable
  alias, such as HostIO's body (`FSOp`, `AmbientOp` and their response
  families), are representation formers: private by design, with no
  home. `Cap` and `Resource` are strict native builtins and already
  have a Strict route; one probe confirms it.
- **Form.** The amended delta is exactly one row,
  `("Capability.Time.WallClock", "ClockIO")`, a name-only home at the
  existing id that adds `ClockIO` to WallClock's export line. `ClockIO`
  (`prelude.rs:1888`) is a transparent alias, so the guard admits it as
  it is. Its body formers (`ClockOp`, `clock_resp`, `MonotonicInstant`,
  `Deadline`) stay unhomed. No `FSOp` row and no former-only granularity
  mechanism is added, and `private_ids` is not edited.
- **Pins (rows C1 to C3 of `evt_1hkvqvwcwcfy`).** C1: a Strict client
  importing `ClockIO` and `wall_now` is accepted, with the declared head
  Const at the pre-source `ClockIO` GlobalId, and the same source without
  the import refuses `UnboundName`. C2: in that environment, `ClockOp`
  and `clock_resp` each refuse `UnboundName`. C3: a client importing
  every package that reaches `FSOp` still refuses `UnboundName` at
  `FSOp`, and separately at `AmbientOp`. The re-census arms N1 and N2,
  and the held rows' HR-i, HR-ii and HR-iii disposition, are as ruled
  there.
- **Rows inherited from LANG-DELETE I1b-b**, held there by identity:
  `cat_system_resource_laws`
  `resource_laws_have_exact_independent_checked_clients` (carry patch
  `/workspaces/ken/tmp/l2-i1b-b-cat_system_resource_laws-identity-theorem-carry.rs`;
  its And clients take the identity-theorem plus CJ form, and its two
  Not-premise clients wait for LANG-DELETE I4), the two
  `px8f_buffer_io_surface` rows (Architect `evt_2ps81n9eenrsn`: PX-H on
  `write_all_checked_step_preserves_the_exact_first_error`, and the
  FSOp hold on `checked_surface_is_public_but_proof_carrying_constructors_stay_private`),
  and `lang_let_hostio_action`. These rows take the HR-i or HR-ii
  disposition of `evt_1hkvqvwcwcfy`. The `lang_let_hostio_action`
  `clock_entropy_and_multilabel_inclusions_use_checked_helpers` row (on
  `random_bytes`) is held out of this node whole and unchanged. It waits
  on `LANG-PRELUDE-ENTROPY-SURFACE-ROUTE` (Architect `evt_2kb4r05x2dc1y`).
  ken-interp `px8p_checked_buffer.rs` :20-29 is not held here: it is an
  I4 carry that binds `FSOp` on the internal kernel-term route.

## Scope

Spec taxonomy rows and the 35 §2.3, 35 §5 and 37 §4/§9 sentences (Spec
enclave); the three catalog export lists and their prose; in
`crates/ken-elaborator`, `PRELUDE_PACKAGE_HOMES` (+56 pairs), the Strict
floor entry for Decimal and the two Decimal dispatch entries; the
consumer renames; CV seed rows. No new law, identity or kernel change.
The I1 rows that wait on this node migrate in I4, not here.

## Acceptance

- **AC-0 (D0, before code).** A scratch Strict probe shows that `USize`,
  `ISize`, `CInt` and their `*_to_int` conversions resolve without an
  import, and that `-` and `*` at Decimal refuse on main. A failing probe
  is a stop to the Architect.
- **AC-1 (H rows).** Each homed name, imported from its home, is
  accepted at its pre-source id; the same use without the import refuses
  `UnboundName`.
- **AC-2 (B row).** `const c : Decimal = 2.0d` checks without an import,
  and a same-spelled source `Decimal` is refused.
- **AC-3 (R rows).** `x + y`, `x - y`, `x * y` and `x == y` at Decimal
  each elaborate to the exact pre-source operation id.
- **AC-4 (seed).** The CV's rows from the ruling: an imported checked
  operation overflowing to `None`; `intToUSize` at manifest max + 1
  giving `Err`; Decimal without an import with exact `*`; one imported
  `isScalar` use.
- **AC-5.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A home needs a new identity, a new law or a kernel change.
- An operator R that does not elaborate to the exact id.
- A renamed Decimal user whose verdict changes arm.

Carried, not framed: after I4, the pre-source `zip` has no source route
and no keyed mechanism; retire it with its `ac5` bracket (Architect
`evt_3pxvhj0yngpxm` site 5). Sibling: `LANG-PROCESS-CHILD-PIPE-WRITE-HOME`
homes three pipe operations by the same H route.

Checks: 6 fired: the population is the implementer's 85-spelling census
(`/workspaces/ken/tmp/l2-i1-provider-audit.tsv`), disposed by
registration site, so each generated family is homed whole. 4 fired: the
Decimal `-`/`*` R rows and the zip R cannot be written on main; their
enablers (dispatch entries, the public zip) are in this node, and AC-0
probes them. 8 fired: each H row pairs the import with its absence on
one source, so a re-declared copy would fail AC-1's id check.
