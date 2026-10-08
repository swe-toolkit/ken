# RT-CARRIED-NAT-MATCH-BOUNDED-IMMEDIATE D0

Measurement only. The baseline was `e7260a1cd979d12e28e7c906a7ad168facc913bf`
(the landed RT-ESCAPE squash). The later main change `e1b609597` touched only
Verify's compiler-driver/example paths; this report was committed on that
newer main without claiming a new execution measurement. The probe changed
no candidate or production file. It ran in a detached disposable worktree,
was restored byte-identically, and the worktree was removed. Probe patch and
logs are in `local/rt-carried-nat-d0/` (gitignored).

## Boundary and membership rule

The executable census is the active `rt_escape_second_resource_native` file,
its three-variant Nat fail-closed control, and the two named `px8n` ReadAt and
WriteAt unit rows. It is not a census of every future Ken program or the full
189-row parity suite. A counted **carried** site entered
`lower_carried_match`; its cases contained the checked `Nat::Zero` and
`Nat::Suc` identities. A site is identified by source and planned Match
origin, not by the number of generated function emissions. An immediate host
field is attributed only when the scrutinee's planned source occurrence
projects a named field of a host response, or when the fixture provides an
independent typed field path. A `Var` alone proves no such provenance.

The `FsReadAt` host result recipe in
`planning/static_transition/aggregates.rs:3772-3787` makes
`ReadSome(PrivateBufferSpan(buffer, start, count), transfer_count)`.
`PrivateBufferSpan` field 2 is `BoundedNat`; the two
`PrivateTransferCount` fields are also `BoundedNat` (the same recipe's
`TRANSFER_COUNT`). `FsWriteAt` returns `Wrote(TRANSFER_COUNT)`.
`lowering/effects.rs:4830-4881` mints a validated count, predecessor and
remaining value before building these nodes. The scalar disposition in
`lowering/boundary.rs:1148-1155` assigns `BoundedNat` the
`ImmediateBoundedNat` tag with `Int` spill class. It also assigns ordinary
`Int` and structural Nat separate tags; their existence does not make an
ordinary Int a Nat. The current closed host-result recipe constructs scalar
leaves through `bounded_nat()` and `native_int()` only; the early direct Bool
host return is outside that aggregate tree. Of these scalar leaf types, only
`BoundedNat` belongs to a structural constructor family (`Nat`). These
are producer shapes, not an assertion that every Nat Match over a host result
uses a carried route or that all future source programs were enumerated.

The observational probe at `lower_carried_match` read the planner's actual
scrutinee occurrence, case identities and composed-suffix length. The active
escape file emitted twelve records, but only eight distinct `(source, origin)`
identities: four BufferSpan-field sites and four `Var` sites. Repeated emits
were not counted as distinct sites.

| Source and planned Nat Match | Scrutinee | Suffix | Status |
|---|---|---:|---|
| live base, 1070 | BufferSpan match, field 2 | 4 | reached, native `-1` |
| live Zero-extra, 1208 | BufferSpan match, field 2 | 4 | reached, native `-1` |
| live Suc-extra, 1209 | BufferSpan match, field 2 | 4 | reached, native `-1` |
| closed-file fanout, 1177 | BufferSpan match, field 2 | 3 | emitted, not reached |
| r2 pre-schema, 18 and 1067 | `Var(0)` | 0 | compile-only; provenance unresolved |
| r2 cross-buffer, 18 and 1067 | `Var(0)` | 0 | compile-only; provenance unresolved |

The active closed-file fanout follows its `Err` path; its native/interpreter
parity does not establish that origin 1177 executes. The r2 pre-schema row
measures plan selection and builds but does not assert successful execution;
its cross-buffer execution row is ignored. The four r2 `Var` sites must not be
silently counted as host-field consumers or excluded from the wider source
universe by their spelling. Their exact upstream producer is a residual for
any open-world population claim. No `PrivateTransferCount` Nat Match was
identified as a carried, reached consumer by this measured corpus; its
bounded-scalar fields remain potential inputs for other source programs.

## Fresh landed-base localization

The unchanged active Nat pin passed 1/1 with `RUST_MIN_STACK` unset and its
stated 32 MiB test thread. The observational probe also passed that pin 1/1.
It recorded the exact source Match origins 1070, 1208 and 1209, each a
`BufferSpan` projection to the zero/successor Nat family with **four pending
composed eliminators**. An unchanged trap decoder recorded for each variant
`terminal=-1` and the native prefix
`[FsOpen, BufferAllocate, FsReadAt, ResourceRelease,
ResourceRelease]`. The existing interpreter checks require 2/2/3 reads, so
these native executions stop after their first read. The pre-existing D6 log
`local/rt-forward-tail-ret-d1/nat-three-d6-localization.log` independently
encoded class `Int` at these three origins on its earlier candidate; its
class code is historical, not a fresh dynamic-class observation on this base.

`lowering/joins.rs:507-518` has an **existing** exact Nat-family adapter for
`ImmediateBoundedNat` and `ImmediateStructuralNat`, but admits it only when
the composed suffix is empty. The three measured suffixes are length four,
so they take the ordinary non-Bool carried constructor path, where
`joins.rs:1122-1140` still requires `BoundaryClass::Constructor` before the
node tag/arity/field chain. The guard premise holds for these sites, **not**
for every carried Nat Match: the empty-suffix adapter is a counterexample to
that global phrasing. The fresh prefix, producer disposition, origin IDs and
gate choice agree with D6, but the current logged BufferSpan constructor
identity ends in `::ctor_0`, whereas the settled frame and old D6 log name
`::ctor_423`.
That exact-spelling anchor changed; the field path and binder count (3)
remain the same. Treat the suffix discrepancy as a perishable settled-input
mismatch for the Architect to rule, not as proof of identical identity. This
probe also does not independently read runtime tag/class bits on the newly
landed base. A disposable diagnostic return intended to encode those bits
was rejected during native object
emission as an unregistered dynamic-status return. It was abandoned, not
weakened into a bypass of frame validation; the successful probe only logs
compile-time routing and the original trap decoder's effect prefix.

## Active structural control on another route

On the unmodified landed base,
`px8n_fs_read_at_arm_distinguishes_eof_and_short_read_some` passed 1/1:
EOF returns 10; the short `ReadSome 1 of 4` returns 12. The fixture
`lowering/core/tests/effects.rs:2645-2735` destructures `ReadSome`, then
`PrivateBufferSpan` and its third field `Var(2)`, and tests exactly one
`Nat::Suc` followed by `Nat::Zero` through `px8n_exact_nat` (`:3201`). The
observational probe recorded two distinct **specialized** Nat Match origins
17 and 22 (`structural=false`, meaning bounded rather than StructuralNat
representation), with no logged carried Nat-family entry. Its two emissions
per origin are the EOF/short fixture builds, not four independent sites. The existing
specialized BoundedNat Match route in `core.rs:15884-15893` and
`joins.rs:1631-1655` selects the structural Zero/Suc arms without a carrier
class read. The sibling
`px8n_fs_write_at_arm_constructs_short_wrote_and_exact_no_progress` passed
1/1 and recorded five specialized Nat Match origins (19, 24, 29, 34, 37),
again no logged carried Nat-family entry. Structural observation of a
validated bounded Nat therefore already works, but those rows do not cover a
composed carried consumer. Independently, the targeted
`carried_nat_match_selects_zero_and_suc_immediates` row passed 1/1 on the
landed base for empty-suffix immediate Nat words; it is a consumer rig, not
a host-field producer witness.

The Architect must rule whether the composed carried sites receive a bounded
immediate structural observer or a static refusal. D0 provides no repair,
unignore, Nat parity result or full parity-suite claim.

Checks: 4 fired: `PrivateBufferSpan` field 2 and `PrivateTransferCount`
fields 0/1 are declared `BoundedNat`, and the current emitter supplies validated
`Lowered::BoundedNat`. 6 fired: the measured boundary and site membership
rule above distinguish 12 emissions from eight identities, three reached
host-field sites from one unselected host-field site, and four `Var` sites
with unresolved producer provenance. 8 fired: the old class-encoding return
failed the native frame validator before execution; the replacement probe
retained the original guard and trap result and only recorded routing/effects.
9 fired: px8n's specialized path was measured separately from the three
composed carried sites, rather than inferred from its green exit.
