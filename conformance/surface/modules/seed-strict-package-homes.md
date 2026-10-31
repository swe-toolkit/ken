# Strict catalog package homes — seed cases

Format: `../../README.md`.
Spec: `spec/30-surface/33-declarations.md §3.3`. Attached-proof selection
also uses §8.2.

These cases are authored but unexecuted. No Ken check or official
conformance-runner result is claimed. No CI, publication, landing, or trust
measurement is claimed.

### surface/modules/catalog-package-homes-preserve-checked-identities

- promise class: **normative compatibility vector** — the listed pre-source
  identities and complete constructor sets are fixed by §3.3
- spec: `33 §3.3`; the IO attached proof additionally uses `33 §8.2`
- given: in a fresh pre-source environment, record the `GlobalId` for each
  listed identity before loading any catalog source. Load the five home units
  and a separate catalog-root consumer with these explicit imports. Run each
  client through `elaborate_module_from_roots_strict` and through the
  `elaborate_module_from_roots` and `elaborate_module_from_roots_v1`
  entrypoints that §3.3 assigns Legacy roots:

  ```ken
  import Capability.Filesystem.Authority (FS, read_bytes, write_file)
  import Capability.System.Buffer
    (BufferWindow, MkBufferWindow, buffer_span_budget, buffer_span_length,
      transfer_count_int, transfer_count_nat, transfer_count_positive,
      transfer_count_positive_prop, transfer_count_remaining,
      transfer_count_nat_succ, transfer_count_predecessor)
  import Capability.System.Error
    (FileOperation, OpReadFile, OpWriteFile, OpAppendFile, OpMetadata,
      OpReadDirectory, OpCreateDirectory, OpRemoveFile, OpRemoveDirectory,
      OpRename, OpChangeMode, OpSeek, OpSetLength, OpSync, OpGetInheritance,
      OpSetInheritance, OpDuplicate)
  import Capability.System.IO (write_all_exact_prefix_prop)
  import Capability.System.Resource
    (ResourceBodyResult, ResourceBodyOk, ResourceBodyErr,
      ResourceBracketResult, ResourceBracketOk, ResourceBracketBodyError,
      ResourceBracketReleaseError, ResourceBracketBodyAndReleaseError)

  const buffer_window_control : BufferWindow = MkBufferWindow 1 2

  fn window_start (window : BufferWindow) : Int =
    match window {
      MkBufferWindow start length ↦ start
    }

  fn file_operation_control (operation : FileOperation) : Bool =
    match operation {
      OpReadFile ↦ True;
      OpWriteFile ↦ False;
      OpAppendFile ↦ False;
      OpMetadata ↦ True;
      OpReadDirectory ↦ True;
      OpCreateDirectory ↦ False;
      OpRemoveFile ↦ False;
      OpRemoveDirectory ↦ False;
      OpRename ↦ False;
      OpChangeMode ↦ True;
      OpSeek ↦ False;
      OpSetLength ↦ True;
      OpSync ↦ True;
      OpGetInheritance ↦ True;
      OpSetInheritance ↦ True;
      OpDuplicate ↦ False
    }

  fn body_result_control (e : Type) (a : Type)
      (result : ResourceBodyResult e a) : Bool =
    match result {
      ResourceBodyOk value ↦ True;
      ResourceBodyErr error ↦ False
    }

  fn bracket_result_control (e : Type) (a : Type)
      (result : ResourceBracketResult e a) : Bool =
    match result {
      ResourceBracketOk value ↦ True;
      ResourceBracketBodyError error ↦ False;
      ResourceBracketReleaseError error ↦ False;
      ResourceBracketBodyAndReleaseError body_error release_error ↦ False
    }
  ```

  The System.IO source unit's five imports from System.Buffer are also
  observed: `buffer_span_budget`, `transfer_count_nat`,
  `transfer_count_nat_succ`, `transfer_count_predecessor`, and
  `transfer_count_remaining`. Its attached proof is checked in the final case.
- measured: each home's public pre-source ordinary export identities, the
  consumer's resolved import `GlobalId`s, and constructor-parent identities.
  The snapshot is captured before loading any home; no numeric `GlobalId` is
  hardcoded.
- claimed: each explicit import preserves its exact pre-source identity in
  both roots modes, and each inductive family exposes its complete checked
  constructor set under the original parent identity.
- the gap: spelling equality is not identity equality. The oracle compares
  each resolved `GlobalId` with the independent pre-source snapshot and
  records each constructor's parent.
- expect: the clients accept in both roots modes. Each home's pre-source
  ordinary export subset is exactly its listed ordinary names. The 40 named
  import bindings equal their pre-source `GlobalId`s; the attached proof is
  checked through its subject in the final case, not as a bare export. Together
  the checks cover all 41 rows. No pre-source identity is attributed to more
  than one home. Constructor sets are exactly
  `BufferWindow` with `MkBufferWindow`, `FileOperation` with the sixteen
  imported `Op*` constructors, `ResourceBodyResult` with `ResourceBodyOk` and
  `ResourceBodyErr`, and `ResourceBracketResult` with its four imported result
  constructors. Every constructor parent equals its pre-source family ID. The
  Buffer home publishes no constructor of the floor `BufferSpan` or
  `TransferCount` families.
- controls: `MkBufferWindow` and its closed one-arm match are the positive
  inductive-family control. The other matches reach the complete FileOperation
  and Resource constructor sets rather than relying on type-name-only imports.
  The System.IO-to-System.Buffer IDs are compared with the Buffer home IDs.
- why: a spelling-only lookup, replacement identity, wrong-family
  constructor, omitted constructor, or duplicated home changes the resolved
  identity or family-parent relation.

### surface/modules/strict-package-home-imports-are-required

- promise class: **durable provider-edge invariant** — a Strict client cannot
  borrow a home name from the flat implementation-global table
- spec: `33 §3.3`
- given: in fresh catalog-root clients, use these source probes, each in its
  own input. The positive imports are `FS` from
  `Capability.Filesystem.Authority`, `BufferWindow` and `MkBufferWindow` from
  `Capability.System.Buffer`, `FileOperation` from `Capability.System.Error`,
  `write_all_exact_prefix_prop` from `Capability.System.IO`, and
  `ResourceBodyResult` from `Capability.System.Resource`. The Buffer negative
  retains `BufferWindow` and omits only `MkBufferWindow`. Each other Strict
  negative omits only its named import; all five provider roots and other
  dependencies remain loaded, and the source body stays fixed.

  ```ken
  theorem fs_type_probe
      (computation : FS AFull (Result FileError Bytes)) : Top = Proved

  const window_probe : BufferWindow = MkBufferWindow 1 2

  fn file_operation_type_probe
      (operation : FileOperation) : FileOperation = operation

  theorem io_subject_probe
        (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
      write_all_exact_prefix_prop::exact_prefix span count

  fn resource_result_type_probe (e : Type) (a : Type)
      (result : ResourceBodyResult e a) : ResourceBodyResult e a = result
  ```

  For each probe, make a positive and a negative Strict-root client that
  differ only by its home import. Pair `FS` with Authority, `MkBufferWindow`
  with Buffer, `FileOperation` with Error, `write_all_exact_prefix_prop` with
  IO, and `ResourceBodyResult` with Resource. The IO probe uses the
  subject-qualified proof from the final case.
- measured: the exact resolution outcome of each fresh client, with all other
  provider roots and dependencies present.
- claimed: each specified home edge is necessary in Strict roots; an
  unimported home name cannot be satisfied by a flat-global fallback.
- the gap: a negative result alone could come from another missing import or
  type error. The paired client adds only the correct home import; its
  acceptance is the positive control.
- expect: each Strict client with the selected home import omitted rejects
  with `UnboundName` naming exactly its target binding. Restoring only that
  home import accepts the same client.
- controls: the five import-restored Strict clients isolate each home edge.
  The complete import clients in the preceding case are accepted controls.
- why: Strict roots resolve only through explicit imports and the closed
  prelude; a name held only by a catalog home is not ambient.

### surface/modules/legacy-package-home-bare-uses-remain-until-l2-4

- promise class: **transition sentinel** — the current Legacy bare-use
  compatibility boundary must be reviewed when the roots entrypoint flips
- spec: `33 §3.3` (Legacy resolution and the L2-4 strict-entry flip)
- given: run each of the five source probes from
  `strict-package-home-imports-are-required` in a fresh catalog-root client
  with its home import removed and no same-name local declaration. Use the
  `elaborate_module_from_roots` and `elaborate_module_from_roots_v1`
  entrypoints that §3.3 currently assigns Legacy roots, with all provider
  roots loaded.
- measured: acceptance or the exact surface resolution failure for each
  bare-use client; no identity comparison is made on this Legacy path.
- claimed: the home migration preserves the existing representative bare
  uses in Legacy roots without promising their identities under all load
  orders.
- the gap: a passing input must actually use the representative name. The
  probes use `FS`, `MkBufferWindow`, `FileOperation`,
  `write_all_exact_prefix_prop`, and `ResourceBodyResult` respectively.
- expect: all five Legacy clients accept. The corresponding Strict clients
  without home imports reject with `UnboundName`, as specified in the prior
  case.
- controls: the imported Legacy clients in the identity case remain the
  exact-identity controls; the Strict missing-import clients distinguish the
  Legacy result from ambient availability.
- why: changing the existing bare-use result is incompatible until the
  `L2-4` strict-entry flip. That successor owns the transition and must review
  or replace this sentinel for the named roots entrypoints.

### surface/modules/system-io-attached-proof-is-selected-through-subject

- promise class: **durable invariant** — an imported subject exposes its
  attached proof only through the subject-qualified path
- spec: `33 §3.3` and `33 §8.2`
- given: in fresh catalog-root clients, import only
  `write_all_exact_prefix_prop` from `Capability.System.IO` and run under
  Strict and Legacy roots. With
  `span : BufferSpan` and `count : TransferCount`, the accepted client proves
  the imported proposition using both canonical and selector forms:

  ```ken
  import Capability.System.IO (write_all_exact_prefix_prop)

  theorem exact_prefix_via_path
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
      write_all_exact_prefix_prop::exact_prefix span count

  theorem exact_prefix_via_selector
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
      proof exact_prefix for write_all_exact_prefix_prop span count
  ```

  The Strict- and Legacy-root negative clients copy
  `exact_prefix_via_path` and change only its right-hand reference to the bare
  `exact_prefix span count`; the subject import and theorem type stay fixed.
- measured: the subject and attached-proof `GlobalId`s selected by each
  qualified reference, and the exact error for the bare reference.
- claimed: `exact_prefix` is the checked attached proof of the imported
  subject, is selected through that subject, and is not a bare binding.
- the gap: the negative must reach attached-proof name resolution, not fail
  because the subject or its import is missing. Its unchanged subject import
  and theorem type provide that control.
- expect: both qualified forms accept in both roots modes and resolve the
  same pre-source `write_all_exact_prefix_prop::exact_prefix` identity under
  the original subject ID. The IO subject and attached proof share the IO
  home, and the proof has no second home. The bare form rejects with
  `UnboundName(exact_prefix)` in both modes.
- controls: the imported subject and both qualified references are positive
  controls for the isolated bare-name refusal.
- why: attached-proof identity is namespaced by its subject; importing the
  subject does not put `exact_prefix` in the ordinary bare-name scope.
