# Open intrinsic and package-surface ledger

> Status: **open inventory, not a floor-membership decision**. The normative
> fifteen-member prelude and its exact constructor/companion closure are
> `30-taxonomy §4`; resolution and protected names are `33 §3.3`. This ledger
> cannot admit a sixteenth member or change `trusted_base()`. Every addition to
> the closed floor needs a per-name operator decision and an atomic roster
> change. Its job is to keep candidate identities and outstanding evidence
> visible while L2 measures them and L3 moves derivable conveniences.

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

For each name below, the keying column names a **mechanism to check**, not a
finding that its identity reader is complete. L2's D0 must trace registration
through every reader; check whether the *source contract independently needs*
that exact identity (`30 §4`'s conjunctive internal-provision arm). The possible
outcomes are: witnessed and proposed to the operator for B; keyed but not
floor-witnessed and exposed only through an operator-approved,
identity-preserving explicit import; compiler-internal only, without source
resolution; or unkeyed, promised ordinary Ken in a package. An unkeyed,
unpromised registration may be removed only after D0 checks all unit classes.
A copied source declaration **cannot** replace a machinery-keyed `GlobalId`.
The proposed compiler-provided intrinsic module is still an operator question,
not an authorized delivery mechanism.

## Per-name candidates requiring a keying and reachability decision

Each table entry is one registered spelling (an attached `::` proof is a
separate identity). Rows sharing a mechanism still require individual D0
reads; a family label is not a witness for every member. `prelude.rs` and
`program_admission.rs` citations below refer to the exact base above.

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
| `Prod` | Product carrier in `ProcessInput`'s environment field; check host-runner ABI identity, not just its structural shape. |
| `MkProd` | `program_admission.rs` captures this constructor when building an input; verify its `Prod` parent and whether a source-visible product is required. |
| `eqChar` | `decimal_char.rs::set_eq_entry` captures the checked definition's ID for literal/comparator selection (`33 §6.2`); confirm every reader. |
| `leqChar` | Checked projection/comparison definition (`decimal_char.rs`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `charToInt` | Checked `Char → Int` projection (`18a §5.9.1`); **no identity reader yet confirmed**, so default to package if D0 finds none. |
| `IO` | Checked Console-ITree alias (`prelude.rs`); confirm whether the effect reifier selects the alias ID or only the underlying constructors. |
| `FS` | Checked Auth-indexed ITree alias (`prelude.rs`); confirm whether the reifier/runner selects its ID. |
| `Stdout` | Console `Stream` constructor used by source `write`/`print_line`; trace runtime stream selection and the public Console contract. |
| `Stderr` | Console `Stream` constructor; same reader/contract check independently. |
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
| `BufferSpan` | Constructor-private minted transfer span (`38 §1.7.1–2`); trace response-type and runtime span ID. |
| `BufferWindow` | Public immutable request descriptor (`38 §1.7.1`); check identity reader separately from bounds checking. |
| `MkBufferWindow` | `BufferWindow` constructor in request creation; check whether the runtime selects this exact ID. |
| `buffer_nat_add` | Checked span/count helper; trace prelude reverse dependencies and host-read identity, if any. |
| `buffer_span_budget` | Checked span budget proof helper (`38 §1.7.3`); test whether runtime reads its ID. |
| `buffer_span_length` | Checked span projection; test whether runtime reads its ID. |
| `TransferCount` | Positioned-transfer count carrier (`38 §1.7.2`); trace driver result identity. |
| `transfer_count_int` | Checked projection used by transfer laws; trace source and native readers. |
| `transfer_count_nat` | Checked projection; trace source and native readers. |
| `transfer_count_positive` | Checked positivity witness; trace source and native readers. |
| `transfer_count_positive_prop` | Checked positivity proposition; trace source and native readers. |
| `transfer_count_remaining` | Checked remaining-count helper; trace source and native readers. |
| `transfer_count_request_budget` | Checked request-budget helper; trace source and native readers. |
| `transfer_count_request_budget::bounded` | Attached proof; trace dependent proof clients and exact subject ID. |
| `write_all_all_success` | Checked all-success law (`38 §1.7.3`); test public proof need vs internal-only helper. |
| `write_all_all_success::all_success` | Attached proof of preceding law; test public proof need independently. |
| `write_all_call_bound` | Checked call-bound law; test public proof need. |
| `write_all_call_bound::termination` | Attached proof of preceding law; test public proof need independently. |
| `write_all_complete` | Checked complete-success law; test public proof need. |
| `write_all_complete::success_complete` | Attached proof of preceding law; test public proof need independently. |
| `write_all_exact_prefix_prop` | Checked exact-prefix proposition; test public proof need. |
| `write_all_exact_prefix_prop::exact_prefix` | Attached proof of preceding proposition; test public proof need independently. |
| `write_all_first_error` | Checked first-error law; test public proof need. |
| `write_all_first_error::first_error` | Attached proof of preceding law; test public proof need independently. |
| `ResourceError` | Runtime resource refusal family (`38 §1.7–1.9`); trace host/reifier identity and public error carrier. |
| `ResourceBodyResult` | Bracket body result carrier (`38 §1.7.1`); trace bracket host and source identity. |
| `ResourceBodyOk` | `ResourceBodyResult` constructor; trace host result discriminator. |
| `ResourceBodyErr` | `ResourceBodyResult` constructor; trace host result discriminator. |
| `ResourceBracketResult` | Settled bracket result carrier (`38 §1.7.1`); trace host result discriminator. |
| `ResourceBracketOk` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketBodyError` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketReleaseError` | `ResourceBracketResult` constructor; trace host result discriminator. |
| `ResourceBracketBodyAndReleaseError` | `ResourceBracketResult` constructor; trace host result discriminator. |

These names remain **outside** the current B. Capturing a name's ID proves
keying, not the separate clause that source must name it without an import.
A keyed public name not admitted to B cannot be recreated by copying a
catalog definition: an explicit, identity-preserving import needs an operator
ruling before implementation. An unkeyed checked helper can be imported from
an ordinary package. Compiler-only IDs need no source spelling. Each case is
per-name; in particular `leqChar` does not inherit `eqChar`'s registry key.

## Unused-by-catalog does not mean unpromised

The following is a **spec-promise audit**, not proof that every member of a
family is unkeyed. L2's D0 must still check producer-to-reader keying,
source-use outside the catalog census, and exact spelling. The normative
behavior remains required even for a name no current catalog root references.
Under `30 §4`, none of these ordinary convenience families acquires ambient
availability merely by being registered. The applicable disposition for a
**verified unkeyed** member is an explicit-import standard package; for a
member with a native identity reader, use the keyed route above instead.

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
full unit reach and ID keying, and each L3 move checks reverse dependencies
and exact identity before replacement. The operator decides each new floor
member and the outstanding compiler-provided intrinsic-module question;
package conveniences and compiler-internal names are not floor members by
word association.
