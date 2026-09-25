# Open intrinsic and package-surface ledger

> Status: **open evidence ledger, not a floor-membership witness**. The
> normative fifteen-member prelude and its exact constructor/companion closure
> are `30-taxonomy §4`; resolution and protected names are `33 §3.3`. This
> ledger cannot admit a sixteenth member or change `trusted_base()`. Once D0
> proves both §4 witness clauses per name, a keyed, source-required identity
> joins the reserved floor with an atomic roster/count change. The final list
> is informational for the operator, not a per-name approval gate.

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

For each possible reserved name below, the keying column names a **native
reader to check**, not proof that the reader set is complete. L2's D0 must
trace registration through every reader and independently establish whether
the *source contract needs that exact identity* (`30 §4`'s conjunction).
Both proven → reserved, always-present floor. Keyed but no independent source
need → compiler-internal, not source-resolvable. Unkeyed but promised →
explicit-import Ken package. Unkeyed and unpromised → remove only after the
full unit reach is measured. A copied source declaration **cannot** replace a
machinery-keyed `GlobalId`; there is **no intrinsic import module**. The
unkeyed laws/helpers below are not candidates for floor membership.

The candidate names in this table are **observed spellings**, not names
approved for permanent reservation. Before L2-3 admits any newly witnessed
name, Spec checks that its spelling closely encloses its meaning (`30 §4`);
`NotFound`, `Other`, `Unsupported`, `Success`, `Failure`, `write`, `Stdout`, and
`Instant` are operator-named specificity probes, not an exhaustive list or
preselected renames. Spec owns the public naming fold in `35`/`38` and this
ledger. Any necessary type/function rename lands atomically with floor
admission and catalog/fixture clients. Qualification `T.C` is now normative
in expressions and patterns for every data type (`32 §4`, `33 §3.3`, `34 §1`).
It can make a generic constructor specific without reserving its bare name;
this S0 assigns no replacement spelling to any type or function.

## Per-type constructor scope choices

The operator permits a **per-type** scoped-constructors property, not a
per-constructor switch. These choices govern B if a candidate type later
satisfies both §4 witnesses; a table entry is not floor membership. An
unscoped floor type brings its entire constructor family into bare B; a scoped
one contributes only its type name and uses qualified `T.C` paths. Every
constructor still has the same checked parent and `GlobalId`.

| Type | Scoped? | Specificity judgment |
|---|---|---|
| `ResourceKind` (current floor) | Yes | `Buffer`, `Mapping`, `FsHandle` are generic bare names; their `ResourceKind.C` paths state their role. |
| `Auth`, `Bool`, `List`, `Nat`, `Option`, `Result`, `Utf8Error` (current floor) | No | The settled bare constructor vocabulary remains available; in particular `True`/`False`, `Zero`/`Suc`, `Nil`/`Cons`, `None`/`Some`, `Ok`/`Err` stay bare. |
| `IOError` | Yes if admitted | Error causes such as `NotFound`, `Other`, and `Unsupported` need their type qualifier. |
| `ExitCode` | Yes if admitted | `Success`/`Failure` need their exit-code meaning at the use site. |
| `ResourceError`, `ResourceBodyResult`, `ResourceBracketResult` | Yes if admitted | A result/error qualifier prevents generic error and status names becoming bare reservations. |
| `Stream`, `Instant`, `CreatePolicy` | Yes if admitted | `Stdout`, `Stderr`, `MkInstant`, and create-policy cases are specific only under their carrier; a generic type name may still need a rename. |
| `Unit`, `ProcessInput`, `FileError`, `BufferWindow` | No if admitted | `MkUnit`, `MkProcessInput`, `MkFileError`, and `MkBufferWindow` name their carriers closely. |
| `ProgramCaps`, `BufferSpan`, `TransferCount` | Yes if admitted | Their minted constructor identities need no additional bare source spelling; private visibility stays intact. |
| `Coproduct` | Yes if admitted | `InL`/`InR` are generic; the effect-signature role is explicit as `Coproduct.C`. |
| `Prod` | No if admitted | `MkProd` names its carrier; admission versus retargeting the ABI to floor `Pair` remains D0's question. |

The dedicated L2 qualified-constructor/scoped-property slice precedes L2-3
floor additions. It also migrates uses of the existing `ResourceKind` family
from bare to qualified form; the change must not strand catalog or fixture
clients. L2-3 then admits only D0-witnessed types with the property selected
above. A scoped type's constructor rows below identify exact parent and
possible native reader, **not** additional bare names to add to B.

## Possible reserved names: proposed reader per name

Each table entry is one named identity. Except for the constructor-private
`PrivateBufferSpan` and `PrivateTransferCount` (not exported through `globals`
but recorded on their types), these spellings occur in the 503-name global
inventory. Rows sharing a reader still require individual D0 reads; a family
label is not a keying proof for every member. Reservation is per **type and
whole constructor family**, never a subset chosen by catalog use. For a scoped
type, only the type is reserved; its constructor IDs remain exact and its
public constructor paths are qualified. D0 traces each native constructor
producer and checks the parent type's independent source requirement.
`prelude.rs` and `program_admission.rs` citations refer to the exact base
above. No row alone reserves a name today.

| Name | Possible keying mechanism and independent source obligation |
|---|---|
| `Unit` | Effect-response result type used by the reifier (`38 §1.7`, `prelude.rs`); verify the exact type ID and source need. |
| `MkUnit` | `CanonicalRuntimeRoles::unit`, effect-response constructor; verify exact parent `Unit`. |
| `ProcessInput` | `program_admission.rs` checks the `main` parameter against its registered type (`33 §3.2.1`). |
| `MkProcessInput` | `program_admission.rs` builds runner input with the registered constructor; verify exact parent `ProcessInput`. |
| `ExitCode` | `program_admission.rs` checks the `main` result against this registered type (`33 §3.2.1`). |
| `Success` | Host-runner return/status identity (`program_admission.rs`, `ken-runtime/src/native_process_entrypoint.rs`); verify parent `ExitCode`. |
| `Failure` | Same host-runner status boundary; verify parent `ExitCode`. |
| `ProgramCaps` | The `main` capability parameter's exact type in `program_admission.rs`, **source-named in `33 §3.2.1` despite catalog-census absence**. |
| `MkProgramCaps` | Runner-minted constructor in `program_admission.rs`; check if only internal, with exact `ProgramCaps` parent. |
| `HostIO` | `main` result's exact wrapper in `program_admission.rs`, **source-named in `33 §3.2.1`**; verify whether this identity must be imported. |
| `Prod` | `ProcessInput` environment carrier: D0 decides whether its exact identity is source-required, or the ABI can be retargeted to canonical floor `Pair`. Reserve-or-retarget, not assumed floor. |
| `MkProd` | `program_admission.rs` captures this constructor; D0 pairs its `Prod` parent with the same reserve-or-retarget choice. No partial constructor admission. |
| `eqChar` | `decimal_char.rs::set_eq_entry` captures the checked definition's ID for literal/comparator selection (`33 §6.2`); confirm every reader. |
| `leqChar` | Checked projection/comparison definition (`decimal_char.rs`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `charToInt` | Checked `Char → Int` projection (`18a §5.9.1`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `IO` | Checked Console-ITree alias (`prelude.rs`); confirm whether the effect reifier selects the alias ID or only the underlying constructors. |
| `FS` | Checked Auth-indexed ITree alias (`prelude.rs`); confirm whether the reifier/runner selects its ID. |
| `Coproduct` | Checked effect-signature sum (`effects/state.rs`, `34 §1`); native composition roles are possible ID readers, independent source need unproven. |
| `InL` | `CanonicalRuntimeRoles::in_l` captures constructor for effect composition; verify parent `Coproduct`. |
| `InR` | `CanonicalRuntimeRoles::in_r` captures constructor for effect composition; verify parent `Coproduct`. |
| `Stream` | Console stream carrier; trace `ConsoleOp` response/reifier identity and source use before reserving its complete constructor family. |
| `Stdin` | `Stream` constructor; trace the native stream selector and source requirement. |
| `Stdout` | `Stream` constructor used by source `write`/`print_line`; trace native stream selection. |
| `Stderr` | `Stream` constructor; trace the native selector independently. |
| `read_bytes` | Checked FS call wrapper (`prelude.rs`); test whether the compiler keys on its ID or only on its emitted `Vis`/`ReadFile`. |
| `write` | Checked Console call wrapper (`prelude.rs`); same independent test. |
| `write_file` | Checked FS call wrapper (`prelude.rs`); same independent test. |
| `FileError` | FS result carrier (`38 §1.3.1`); trace runtime construction and source-required error identity. |
| `MkFileError` | `CanonicalRuntimeRoles::file_error` constructor; verify registered `FileError` parent. |
| `IOError` | FS/Console cause carrier (`38 §1.3.1`); trace the runtime error-identity carrier. |
| `AlreadyExists` | `CanonicalRuntimeRoles::io_error_already_exists`; parent `IOError`. |
| `BrokenPipe` | `CanonicalRuntimeRoles::io_error_broken_pipe`; parent `IOError`. |
| `CapabilityDenied` | `CanonicalRuntimeRoles::io_error_capability_denied`; parent `IOError`. |
| `Interrupted` | `CanonicalRuntimeRoles::io_error_interrupted`; parent `IOError`. |
| `InvalidInput` | `CanonicalRuntimeRoles::io_error_invalid_input`; parent `IOError`. |
| `IsDirectory` | `CanonicalRuntimeRoles::io_error_is_directory`; parent `IOError`. |
| `NotDirectory` | `CanonicalRuntimeRoles::io_error_not_directory`; parent `IOError`. |
| `NotEmpty` | `CanonicalRuntimeRoles::io_error_not_empty`; parent `IOError`. |
| `NotFound` | `CanonicalRuntimeRoles::io_error_not_found`; parent `IOError`. |
| `Other` | `CanonicalRuntimeRoles::io_error_other`; parent `IOError`. |
| `PermissionDenied` | `CanonicalRuntimeRoles::io_error_permission_denied`; parent `IOError`. |
| `Revoked` | `CanonicalRuntimeRoles::io_error_revoked`; reconcile the single semantic identity required by `38 §1.8`. |
| `Unsupported` | `CanonicalRuntimeRoles::io_error_unsupported`; parent `IOError`. |
| `Instant` | Clock response carrier (`prelude.rs::clock_resp`); trace native clock response's type ID. |
| `MkInstant` | `CanonicalRuntimeRoles::mk_instant` for clock response; verify exact `Instant` parent. |
| `CreatePolicy` | FS `WriteFile` request parameter (`prelude.rs::fs_resp`); test whether native operation needs this type's ID. |
| `CreateNew` | `CreatePolicy` constructor; trace native create-policy discriminator. |
| `CreateOrTruncate` | `CreatePolicy` constructor; trace native create-policy discriminator. |
| `CreateOrKeep` | `CreatePolicy` constructor; trace native create-policy discriminator. |
| `BufferSpan` | Constructor-private minted transfer span (`38 §1.7.1–2`); trace response-type and runtime span ID. |
| `PrivateBufferSpan` | Exact `BufferSpan` constructor: trace host minting; parent admission never widens constructor-private visibility. |
| `BufferWindow` | Public immutable request descriptor (`38 §1.7.1`); check identity reader separately from bounds checking. |
| `MkBufferWindow` | `BufferWindow` constructor in request creation; check whether the runtime selects this exact ID. |
| `TransferCount` | Positioned-transfer count carrier (`38 §1.7.2`); trace driver result identity. |
| `PrivateTransferCount` | Exact `TransferCount` constructor: trace native minting; retain constructor-private visibility. |
| `ResourceError` | Runtime resource refusal family (`38 §1.7–1.9`); trace host/reifier identity and public error carrier. |
| `ResourceHostIO` | `ResourceError` constructor: trace native host-error wrapper ID. |
| `Closed` | `ResourceError` constructor: trace native closed-token refusal ID. |
| `MalformedResource` | `ResourceError` constructor: trace native malformed-token refusal ID. |
| `RightNotHeld` | `ResourceError` constructor: trace native right-refusal ID. |
| `ReleaseFailed` | `ResourceError` constructor: trace native release-error ID. |
| `ResourceKindMismatch` | `CanonicalRuntimeRoles::resource_kind_mismatch`; exact parent `ResourceError`. |
| `BufferLimit` | `CanonicalRuntimeRoles::resource_buffer_limit`; exact parent `ResourceError`. |
| `AllocationFailed` | `CanonicalRuntimeRoles::resource_allocation_failed`; exact parent `ResourceError`. |
| `InvalidOffset` | `CanonicalRuntimeRoles::resource_invalid_offset`; exact parent `ResourceError`. |
| `InvalidBounds` | `CanonicalRuntimeRoles::resource_invalid_bounds`; exact parent `ResourceError`. |
| `NoProgress` | `CanonicalRuntimeRoles::resource_no_progress`; exact parent `ResourceError`. |
| `MappingLimit` | `CanonicalRuntimeRoles::resource_mapping_limit`; exact parent `ResourceError`. |
| `ResourceBodyResult` | Bracket body result carrier (`38 §1.7.1`); trace bracket host and source identity. |
| `ResourceBodyOk` | `ResourceBodyResult` constructor; trace host result discriminator. |
| `ResourceBodyErr` | `ResourceBodyResult` constructor; trace host result discriminator. |
| `ResourceBracketResult` | Settled bracket result carrier (`38 §1.7.1`); trace host result discriminator. |
| `ResourceBracketOk` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketBodyError` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketReleaseError` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketBodyAndReleaseError` | `ResourceBracketResult` constructor; trace host result discriminator. |

These candidates remain **outside** the current B. Capturing an ID proves
keying, not independent source need. Both witnesses admit a type to B; its
whole constructor family follows the scoped choice above. Without source need
it is internal-only, and without keying it is package material if promised.
For checked definitions the test remains per-name: `leqChar` does not inherit
`eqChar`'s registry key. If `IOError` qualifies, its thirteen causes are one
**scoped** family, not thirteen bare B names. D0 still names the native
producer of each exact cause ID, never inferring all readers from one cause.
`Prod`/`MkProd` remains an explicit D0 decision: reserve the exact type and
its unscoped constructor, or retarget the Program-I field to already-floor
`Pair`, without asserting either outcome here.

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
