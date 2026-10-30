# Open intrinsic and package-surface ledger

> Status: **open evidence ledger, not a floor-membership witness**. The
> normative thirty-identity prelude and its exact constructor/companion
> closure are `30-taxonomy §4`; resolution and protected names are `33 §3.3`.
> This ledger alone cannot admit another member or change `trusted_base()`.
> Once D0 proves both §4 witness clauses per name, a keyed, source-required
> identity joins the reserved floor with an atomic roster/count change.
> The final list is informational for the operator, not a per-name approval
> gate.

## Reading the inventory without over-claiming

The Architect's inventory at `origin/main` `50966beb2` counted **503** entries
in `ElabEnv::new().globals`, **135** provisionally classified in the built-in
resolution set B, and **88** names reached in the roots-loaded catalog census
(83 of those outside B). Those numbers have different populations. The census
measures catalog roots and their imports, **not** isolated files, examples,
conformance, all fixtures, REPL sessions, or all compiler-registered names.
The inventory's source-literal-count column is a **lower bound**, not a
producer-to-reader proof of keying: a reader of an ID captured during prelude
registration need never repeat the name literal. Neither an absent literal nor
an unused catalog census name proves a global is safe to remove.

For unruled candidate names below, the keying column names a **native
reader to check**, not proof that the reader set is complete. The
admitted entrypoint-ABI, effect/clock, ITree and FS/resource rows instead
record ruled exact-ID keying and independent source need (`30 §4`'s
conjunction).
Both proven → reserved, always-present floor. Keyed but no independent source
need → compiler-internal, not source-resolvable. Unkeyed but promised →
explicit-import Ken package. Unkeyed and unpromised → remove only after the
full unit reach is measured. `Empty` has a measured native reader and no
independent source-name need: its disposition is internal-only. Its fixed
pre-source ID is captured and read by `elaborate_space_decl`; it is not an
unfixed spelling lookup. The source name is supplied by a catalog
package, not the prelude, with an identity distinct from the compiler's. A
copied source declaration **cannot** replace a machinery-keyed `GlobalId`;
there is **no intrinsic import module**. The unkeyed laws/helpers below are
not candidates for floor membership.

Apart from internal-only `Empty` and the admitted entrypoint-ABI,
effect/clock, ITree and six FS/resource families below, the candidate
names in this table are **observed spellings**, not names approved for
permanent reservation. Before a later slice admits a newly witnessed
name, Spec checks that its spelling closely encloses its meaning
(`30 §4`). For the admitted `IOError` family, `NotFound`, `Other` and
`Unsupported` gain specificity through `IOError.C` qualification, not
renaming or bare reservation. `write` and `Stdout` remain specificity
probes for later slices. `Instant` is admitted as a specific type spelling, while its
constructor is qualified `Instant.MkInstant`. `Success`/`Failure`
are admitted only as qualified `ExitCode.Success`/`ExitCode.Failure`,
not bare reservations. Spec owns public naming folds in `35`/`38`
and this ledger. Any necessary type/function rename lands atomically
with floor admission and catalog/fixture clients. Qualification `T.C`
is now normative in expressions and patterns for every data type
(`32 §4`, `33 §3.3`, `34 §1`).
It can make a generic constructor specific without reserving its bare
name; this FS/resource slice assigns no replacement spelling to any
type or function.

## Per-type constructor scope choices

The operator permits a **per-type** scoped-constructors property, not a
per-constructor switch. These choices govern B if a candidate type later
satisfies both §4 witnesses; a table entry is not floor membership. An
unscoped floor type brings its entire constructor family into bare B; a scoped
one contributes only its type name and uses qualified `T.C` paths. Every
constructor still has the same checked parent and `GlobalId`.

| Type | Scoped? | Specificity judgment |
|---|---|---|
| `ResourceKind` (current floor) | Yes | `ResourceKind.FsHandle`, `ResourceKind.Buffer`, `ResourceKind.Mapping`, `ResourceKind.SpawnPlan`, `ResourceKind.ChildProcess`, `ResourceKind.PipeRead`, and `ResourceKind.PipeWrite` are the seven public paths; none of their leaf names is reserved bare by this family. |
| `Auth`, `Bool`, `List`, `Nat`, `Option`, `Result`, `Utf8Error` (current floor) | No | The settled bare constructor vocabulary remains available; in particular `True`/`False`, `Zero`/`Suc`, `Nil`/`Cons`, `None`/`Some`, `Ok`/`Err` stay bare. |
| `IOError` (floor) | Yes | All thirteen causes use `IOError.C`; `NotFound`, `Other`, `Unsupported` and `Revoked` are not bare reservations. |
| `ExitCode` (floor) | Yes | `ExitCode.Success`/`ExitCode.Failure` state their exit-code meaning; the bare names are not reserved. |
| `ResourceError` (floor) | Yes | All twelve error causes use `ResourceError.C`; no bare error name is reserved. |
| `ResourceBodyResult`, `ResourceBracketResult` (package) | Yes if later admitted | Both are source-promised and unkeyed: they stay outside B; qualification here is not an admission. |
| `Instant` (current floor) | Yes | `Instant.MkInstant` names the exact checked clock constructor without reserving bare `MkInstant`. |
| `ITree` (current floor) | Yes | The whole checked `Ret`/`Vis` family is reachable as `ITree.Ret`/`ITree.Vis`; bare `Ret`/`Vis` are free for source declarations, not reserved or renamed. |
| `Stream` (candidate) | Yes if admitted | `Stdout` and `Stderr` need their carrier; source need and keying remain to be ruled. |
| `CreatePolicy` (floor) | Yes | The three cases use `CreatePolicy.C`, retaining their unchanged native artifact leaves. |
| `ProcessInput` (floor) | No | Its `MkProcessInput` constructor is bare and reserved with its type. |
| `Unit` (current floor) | No | `MkUnit` names its exact checked parent and remains bare and reserved. |
| `FileError` (floor) | No | `MkFileError` remains bare and reserved with its parent. |
| `BufferWindow` (package) | No if later admitted | `MkBufferWindow` names its carrier, but neither is reserved now; strict-mode same-name source declarations remain legal. |
| `ProgramCaps` (floor) | Yes | Its runner-minted `MkProgramCaps` does not reserve a bare spelling; constructor visibility remains unchanged. |
| `BufferSpan`, `TransferCount` (floor) | Yes | Exact minted constructor IDs retain private visibility; neither bare spelling nor a public qualified constructor is admitted. |
| `Coproduct` (current floor) | Yes | `InL`/`InR` are generic; only `Coproduct.InL`/`Coproduct.InR` resolve to the exact constructors. |
| `Prod` (floor) | No | `MkProd` names its carrier and is bare and reserved; retargeting the constructor-backed runtime representation to floor `Pair` is not part of this slice. |

The dedicated L2 qualified-constructor/scoped-property slice precedes L2-3
floor additions. It also migrates uses of the existing `ResourceKind` family
from bare to qualified form; the change must not strand catalog or fixture
clients. L2-3 then admits only D0-witnessed types with the property selected
above. A scoped type's constructor rows below identify exact parent and
possible native reader, **not** additional bare names to add to B.

## Reserved entrypoint names, other candidates, and internal identities

Each table entry is one named identity. Except for the constructor-private
`PrivateBufferSpan` and `PrivateTransferCount` (not exported through `globals`
but recorded on their types), these spellings occur in the 503-name global
inventory. `Empty` is recorded here as an **internal-only target with a measured
exact-ID reader**, not a proposed reservation: the native site uses its
captured pre-source ID and the independent source-name clause is false.
Rows sharing a reader still require individual D0 reads; a family label
is not a keying proof for every member. Reservation is
per **type and whole constructor family**, never a subset chosen by catalog
use. For a scoped type, only the type is reserved. Its constructor IDs remain
exact, and its public constructor paths are qualified. D0 traces each native
constructor producer and checks the parent type's independent source need.
The entrypoint-ABI and `Empty` rows are re-grounded on landed
`f3e71af9c`; the effect/clock rows are settled against the exact-ID
producer/reader and source requirements in the keyed-effects D0.
The six FS/resource families are admitted by the native constructor
producer/reader and parent-ID probe on `fd2f57557` (Architect AC-0),
paired with their independent `38` source need (Spec Leader AC-0).
Other inventory citations retain their stated census base. Only
marked witnessed entrypoint-ABI, effect/clock, ITree and FS/resource
rows below join the floor; no other row alone reserves a name.

| Name | Possible keying mechanism and independent source obligation |
|---|---|
| `Empty` (internal-only, not floor) | `PreludeEnv.empty_id` captures the compiler's exact pre-source identity (`prelude.rs:388`); `elaborate_space_decl` reads it (`elab.rs:20897`). **Source need:** none for that exact identity under `36 §4.1–4.2`; the source name belongs to `Core.Logic.EmptyDec` with a distinct checked identity, not to the prelude. |
| `Unit` (floor, unscoped) | `PreludeEnv.unit_id` captures the exact checked effect-response result identity; source must name it (`36 §1.1`, §2.1). |
| `MkUnit` (bare floor constructor) | `CanonicalRuntimeRoles::unit` selects the exact checked constructor whose parent is `Unit`; the unscoped family remains bare and reserved. |
| `ProcessInput` (floor, unscoped) | `program_admission.rs` builds the checked `main` parameter from its registered type (`33 §3.2.1`); source must name that exact identity. |
| `MkProcessInput` (bare floor constructor) | Runner input uses its exact ID; the whole unscoped `ProcessInput` constructor family is reserved. |
| `ExitCode` (floor, scoped) | `program_admission.rs` builds the checked `main` result from this exact type (`33 §3.2.1`). |
| `Success` (qualified floor constructor) | Host-runner status uses its exact `ExitCode` constructor ID; source path `ExitCode.Success`, not bare `Success`. |
| `Failure` (qualified floor constructor) | Host-runner status uses its exact `ExitCode` constructor ID; source path `ExitCode.Failure`, not bare `Failure`. |
| `ProgramCaps` (floor, scoped) | The `main` capability parameter uses this exact source-named type despite catalog-census absence (`33 §3.2.1`). |
| `MkProgramCaps` (scoped floor constructor) | Runner-minted exact `ProgramCaps` constructor; no bare reservation and no widening of constructor visibility. |
| `HostIO` (floor type-level function) | The checked `main` result uses its exact identity as `HostIO a ExitCode` (`33 §3.2.1`); it has no constructor family. |
| `Prod` (floor, unscoped) | `ProcessInput`'s environment field names `List (Prod Bytes Bytes)`; its exact constructor-backed runtime identity is source-required. Floor `Pair` cannot replace its representation in this slice. |
| `MkProd` (bare floor constructor) | Native environment-pair consumers select its exact ID; the whole unscoped `Prod` family is reserved. |
| `host_exit` (internal-only) | Native entrypoint machinery keys this exact ID, but `33 §3.2.1` does not require programs to name it. An example use does not confer source need; the example migrates in the same build. |
| `eqChar` | `decimal_char.rs::set_eq_entry` captures the checked definition's ID for literal/comparator selection (`33 §6.2`); confirm every reader. |
| `leqChar` | Checked projection/comparison definition (`decimal_char.rs`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `charToInt` | Checked `Char → Int` projection (`18a §5.9.1`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `IO` | Checked Console-ITree alias (`prelude.rs`); confirm whether the effect reifier selects the alias ID or only the underlying constructors. |
| `FS` (package, not floor) | Source-promised Auth-indexed ITree alias in `read_bytes`/`write_file` types (`38`); no native reader of this alias ID. The effect/capability label `FS` is a separate namespace. |
| `ITree` (floor, scoped) | `PreludeEnv.itree_id` captures the checked pre-source inductive ID; effect lowering and the host interpreter select that family by exact ID. The surface requires source to name the canonical type and its constructors (`36 §2.1`, §2.4). A same-shaped source family does not replace it. |
| `Ret` (qualified floor constructor) | `PreludeEnv.ret_id` and the runtime-role capture select its checked `ITree` constructor ID; source uses `ITree.Ret`, never a reserved bare `Ret`. The constructor itself is not renamed. |
| `Vis` (qualified floor constructor) | `PreludeEnv.vis_id` and the runtime-role capture select its checked `ITree` constructor ID; source uses `ITree.Vis`, never a reserved bare `Vis`. The constructor itself is not renamed. |
| `Coproduct` (floor, scoped) | `PreludeEnv.coproduct_id` identifies the checked effect-signature sum; native composition keys the exact injection parents and source needs the family (`36 §2.3`, §4.5.7). |
| `InL` (qualified floor constructor) | `CanonicalRuntimeRoles::in_l` selects the exact `Coproduct` constructor; only `Coproduct.InL` resolves, not bare `InL`. |
| `InR` (qualified floor constructor) | `CanonicalRuntimeRoles::in_r` selects the exact `Coproduct` constructor; only `Coproduct.InR` resolves, not bare `InR`. |
| `Stream` | Console stream carrier; trace `ConsoleOp` response/reifier identity and source use before reserving its complete constructor family. |
| `Stdin` | `Stream` constructor; trace the native stream selector and source requirement. |
| `Stdout` | `Stream` constructor used by source `write`/`print_line`; trace native stream selection. |
| `Stderr` | `Stream` constructor; trace the native selector independently. |
| `read_bytes` (package, not floor) | Source-promised checked FS wrapper; native code keys the emitted `ITree.Vis`/`ReadFile` identities, not this wrapper's ID. |
| `write` | Checked Console call wrapper (`prelude.rs`); same independent test. |
| `write_file` (package, not floor) | Source-promised checked FS wrapper; native code keys the emitted `ITree.Vis`/`WriteFile` identities, not this wrapper's ID. |
| `FileError` (floor, unscoped) | `38 §1.3.1` requires this public FS result carrier; native code mints the exact checked `MkFileError` constructor and thereby keys its parent. |
| `MkFileError` (bare floor constructor) | `CanonicalRuntimeRoles::file_error` selects the checked `FileError` constructor; source keeps bare `MkFileError`. |
| `IOError` (floor, scoped) | `38 §1.3.1` requires the cause carrier including `Revoked`; all thirteen checked cause IDs are produced by native code and fix their shared parent. Only `IOError.C` paths resolve. |
| `AlreadyExists` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_already_exists`; parent `IOError`; source `IOError.AlreadyExists`, not bare. |
| `BrokenPipe` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_broken_pipe`; parent `IOError`; source `IOError.BrokenPipe`, not bare. |
| `CapabilityDenied` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_capability_denied`; parent `IOError`; source `IOError.CapabilityDenied`, not bare. |
| `Interrupted` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_interrupted`; parent `IOError`; source `IOError.Interrupted`, not bare. |
| `InvalidInput` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_invalid_input`; parent `IOError`; source `IOError.InvalidInput`, not bare. |
| `IsDirectory` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_is_directory`; parent `IOError`; source `IOError.IsDirectory`, not bare. |
| `NotDirectory` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_not_directory`; parent `IOError`; source `IOError.NotDirectory`, not bare. |
| `NotEmpty` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_not_empty`; parent `IOError`; source `IOError.NotEmpty`, not bare. |
| `NotFound` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_not_found`; parent `IOError`; source `IOError.NotFound`, not bare. |
| `Other` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_other`; parent `IOError`; source `IOError.Other`, not bare. |
| `PermissionDenied` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_permission_denied`; parent `IOError`; source `IOError.PermissionDenied`, not bare. |
| `Revoked` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_revoked`; parent `IOError`; source `IOError.Revoked` names the single `38 §1.8` identity, not bare. |
| `Unsupported` (qualified floor constructor) | `CanonicalRuntimeRoles::io_error_unsupported`; parent `IOError`; source `IOError.Unsupported`, not bare. |
| `Instant` (floor, scoped) | Source names the canonical clock-response result (`36 §1.1`); the native clock response keys its checked `MkInstant` constructor and exact `Instant` parent. |
| `MkInstant` (qualified floor constructor) | `CanonicalRuntimeRoles::mk_instant` selects the checked `Instant` constructor; only `Instant.MkInstant` resolves, not bare `MkInstant`. |
| `CreatePolicy` (floor, scoped) | Source selects public `WriteFile` creation semantics (`38 §1.3.1`); the native interpreter reads its three exact constructor IDs. Source cases use `CreatePolicy.C`, with no type rename. |
| `CreateNew` (qualified floor constructor) | Exact-ID native policy reader; source `CreatePolicy.CreateNew`, not bare. The native suffix reader still sees the unchanged `CreatePolicy::CreateNew` artifact leaf. |
| `CreateOrTruncate` (qualified floor constructor) | Exact-ID native policy reader; source `CreatePolicy.CreateOrTruncate`, not bare; retain its native artifact leaf. |
| `CreateOrKeep` (qualified floor constructor) | Exact-ID native policy reader; source `CreatePolicy.CreateOrKeep`, not bare; retain its native artifact leaf. |
| `BufferSpan` (floor, scoped) | `38 §1.7.1–2` requires this exact bounded span across operations; native code mints `PrivateBufferSpan`, fixing its checked parent. |
| `PrivateBufferSpan` (private floor constructor) | Exact checked `BufferSpan` constructor minted by the host; its parent is admitted without changing constructor privacy or reserving a bare name. |
| `BufferWindow` (package, not floor) | Source-promised public request descriptor (`38 §1.7.1`); no native ID reader. Strict-mode same-name source declaration is accepted; legacy resolution is unchanged. |
| `MkBufferWindow` (package constructor) | Checked request descriptor constructor; neither it nor its unkeyed parent enters B. |
| `TransferCount` (floor, scoped) | `38 §1.7.2` exposes this exact progress carrier and its count view; native code mints `PrivateTransferCount`, fixing the checked parent. |
| `PrivateTransferCount` (private floor constructor) | Exact checked `TransferCount` constructor minted by the host; constructor privacy and absence of a bare reservation remain. |
| `ResourceError` (floor, scoped) | `38 §1.7–1.9` requires this public refusal family; native code produces all twelve exact checked constructor IDs, fixing their shared parent. Only `ResourceError.C` resolves. |
| `ResourceHostIO` (qualified floor constructor) | Native host-error wrapper ID; parent `ResourceError`; source `ResourceError.ResourceHostIO`, not bare. |
| `Closed` (qualified floor constructor) | Native closed-token refusal ID; parent `ResourceError`; source `ResourceError.Closed`, not bare. |
| `MalformedResource` (qualified floor constructor) | Native malformed-token refusal ID; parent `ResourceError`; source `ResourceError.MalformedResource`, not bare. |
| `RightNotHeld` (qualified floor constructor) | Native right-refusal ID; parent `ResourceError`; source `ResourceError.RightNotHeld`, not bare. |
| `ReleaseFailed` (qualified floor constructor) | Native release-error ID; parent `ResourceError`; source `ResourceError.ReleaseFailed`, not bare. |
| `ResourceKindMismatch` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_kind_mismatch`; parent `ResourceError`; source `ResourceError.ResourceKindMismatch`, not bare. |
| `BufferLimit` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_buffer_limit`; parent `ResourceError`; source `ResourceError.BufferLimit`, not bare. |
| `AllocationFailed` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_allocation_failed`; parent `ResourceError`; source `ResourceError.AllocationFailed`, not bare. |
| `InvalidOffset` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_invalid_offset`; parent `ResourceError`; source `ResourceError.InvalidOffset`, not bare. |
| `InvalidBounds` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_invalid_bounds`; parent `ResourceError`; source `ResourceError.InvalidBounds`, not bare. |
| `NoProgress` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_no_progress`; parent `ResourceError`; source `ResourceError.NoProgress`, not bare. |
| `MappingLimit` (qualified floor constructor) | `CanonicalRuntimeRoles::resource_mapping_limit`; parent `ResourceError`; source `ResourceError.MappingLimit`, not bare. |
| `ResourceBodyResult` (package, not floor) | Source-promised bracket body-result carrier (`38 §1.7.1`); no native reader or producer of its exact ID. |
| `ResourceBodyOk` (package constructor) | Checked `ResourceBodyResult` case, not a floor constructor or bare reservation. |
| `ResourceBodyErr` (package constructor) | Checked `ResourceBodyResult` case, not a floor constructor or bare reservation. |
| `ResourceBracketResult` (package, not floor) | Source-promised settled bracket-result carrier (`38 §1.7.1`); no native reader or producer of its exact ID. |
| `ResourceBracketOk` (package constructor) | Checked `ResourceBracketResult` case, not a floor constructor or bare reservation. |
| `ResourceBracketBodyError` (package constructor) | Checked `ResourceBracketResult` case, not a floor constructor or bare reservation. |
| `ResourceBracketReleaseError` (package constructor) | Checked `ResourceBracketResult` case, not a floor constructor or bare reservation. |
| `ResourceBracketBodyAndReleaseError` (package constructor) | Checked `ResourceBracketResult` case, not a floor constructor or bare reservation. |

The five entrypoint-ABI, three effect/clock, `ITree` and six FS/resource
type identities marked floor above join B alongside the original fifteen.
Unscoped constructor families reserve bare names; scoped families retain
exact IDs reachable only through `T.C` when public, without bare B entries.
Private constructors stay hidden even when their parent type is scoped.
Other rows remain **outside** B. Capturing an ID proves keying, not
independent source need. `Empty` is read by exact ID but remains internal-only;
`host_exit` is likewise keyed without independent source need. A future
`36` contract requiring source to name that exact space identity needs
separate Spec gates. If adopted, existing Q-B admits it
to the floor without per-name operator approval, with atomic roster/count
change and retirement of the catalog `Empty`; it adds nothing to today's floor.
Both witnesses admit a type to B; its whole constructor family follows the
scoped choice above. Without source need it is internal-only, and without
keying it is package material if promised.
For checked definitions the test remains per-name: `leqChar` does not inherit
`eqChar`'s registry key. `IOError`'s thirteen causes are one **scoped**
family, not thirteen bare B names. Architect's D0 names each cause's
native producer separately, without inferring all readers from one cause.
The twelve `ResourceError` constructors likewise retain their exact IDs
without bare reservations. `ITree` joins the type floor without adding bare
`Ret` or `Vis` to B;
source may bind either under ordinary non-B rules. The D0 resolved
`Prod`/`MkProd` as a reserved unscoped family. A future
representation change to kernel `Pair` would be a separate Runtime successor,
not a second source-visible ABI in this slice.

## Checked laws and helpers are not intrinsic candidates

No `write_all_*`, `transfer_count_*`, or `buffer_*` lemma/helper is keyed by
native machinery just because it reasons about a keyed carrier. The behavior
or proof that the spec promises moves to a checked package; an unneeded pure
helper's prelude registration is deleted. D0 still measures actual uses and
reverse dependencies before any such move or deletion.

| Name | Package or removal disposition |
|---|---|
| `buffer_nat_add` | Pure internal helper: package-private only if its proof needs it; otherwise delete. |
| `buffer_span_budget` | Checked span-budget lemma: place with the buffer laws if used (`38 §1.7.3`). |
| `buffer_span_length` | Public span-length projection: export from the buffer package (`38 §1.7.1`). |
| `transfer_count_int` | Checked transfer projection in the package (`38 §1.7.2`). |
| `transfer_count_nat` | Checked transfer projection in the package (`38 §1.7.2`). |
| `transfer_count_positive` | Checked positivity witness in the package (`38 §1.7.2–3`). |
| `transfer_count_positive_prop` | Checked positivity proposition in the package (`38 §1.7.2–3`). |
| `transfer_count_remaining` | Checked remaining-count helper: package-private if needed; otherwise delete. |
| `transfer_count_request_budget` | Checked budget helper: package-private if needed; otherwise delete. |
| `transfer_count_request_budget::bounded` | Attached boundedness proof: publish with its package subject if required. |
| `write_all_all_success` | Checked law in the capability package (`38 §1.7.3`). |
| `write_all_all_success::all_success` | Attached proof in that package. |
| `write_all_call_bound` | Checked law in the capability package. |
| `write_all_call_bound::termination` | Attached proof in that package. |
| `write_all_complete` | Checked law in the capability package. |
| `write_all_complete::success_complete` | Attached proof in that package. |
| `write_all_exact_prefix_prop` | Checked proposition in the capability package. |
| `write_all_exact_prefix_prop::exact_prefix` | Attached proof in that package. |
| `write_all_first_error` | Checked law in the capability package. |
| `write_all_first_error::first_error` | Attached proof in that package. |

## Unused-by-catalog does not mean unpromised

The following is a **spec-promise audit**, not proof that every member of a
family is unkeyed. L2's D0 must still check producer-to-reader keying,
source-use outside the catalog census, and exact spelling. The normative
behavior remains required even for a name no current catalog root references.
Under `30 §4`, none of these ordinary convenience families acquires ambient
availability merely by being registered. A **verified unkeyed** promised name
is an explicit-import package definition; a keyed identity joins the floor
only if source must name that exact identity, otherwise it remains internal.

| Family | Existing normative promise | Disposition once unkeyed is confirmed |
|---|---|---|
| `checked*` / `saturating*` | `35 §3.2` promises `Option`-on-overflow and clamp semantics as distinct explicit classes; `18a §5.3` derives them from bignum and safe casts. | Keep the named behavior as checked package definitions; never silently delete the class or make bare arithmetic wrap. |
| `intTo*` | `35 §5` promises explicit total/partial conversions; `18a §5.7` distinguishes safe `intToIntN` from native unchecked `int_to_*_raw`. | Export safe conversions from packages; raw casts retain only their audited primitive role, not an ambient safe alias. Missing non-width conversions remain declared gaps. |
| `decimal*` | `35 §2.3` and `18a §5.6.1` require exact decimal arithmetic and distinct value equality; `decimalPow10Unbounded` is the exceptional audited postulate. | Place ordinary derived arithmetic/comparison helpers in packages; do not silently equate value-equality with structural `Equal` or delete the deferred alignment boundary. |
| Retry/transience | `38 §1.8` requires separate error-transience and operation-idempotence inputs and one permanent `Revoked` identity. | Publish checked classification types and laws through an explicit package; a transience-only `retryable` is not a substitute. |
| Mapping/buffer | `38 §1.7` and `§1.9` require bounded, bracket-scoped buffer/mapping operations, view identities, and refusals. | Public checked brackets, descriptors, accessors and laws move into explicit-import packages; native resource operations/IDs stay behind the host boundary. |
| File/dir operations | `38 §1.3.1`, `§1.7` and `62 §4` require typed capability wrappers and file/dir effects, including static full-authority write and runtime read sufficiency. | Import checked operations from a package; moving a wrapper cannot drop its capability, error, or effect contract. |

`38 §1.7.1` and `§1.9` currently call portions of the buffer/mapping surface a
"public prelude API." That availability label conflicts with the now-closed
`30 §4` floor: it does **not** enroll those names in B. The API signatures and
behavior are retained; a follow-up normative wording reconciliation in `38`
is owed outside this S0's two-chapter edit scope. Likewise `35 §6.2`'s
"prelude propositions" label cannot authorize arbitrary numeric law names
without the §4 witness. These are explicit documentation seams, not silent
exceptions to strict resolution.

No removal or move is authorized by this ledger alone. L2 D0 measures the
full unit reach, each native ID reader, and independent source need; each L3
move checks reverse dependencies and identity before replacement. L2-3 then
adds every fully witnessed name to the floor, with the roster/count updated
atomically; its final list is shown to the operator for information. There is
no intrinsic module and no extra per-name operator approval gate. Package
conveniences and compiler-internal identities are not floor members by word
association.
