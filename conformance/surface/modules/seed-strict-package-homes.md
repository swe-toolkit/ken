# Strict catalog package homes — seed cases

Format: `../../README.md`.
Spec: `30-taxonomy §4`; `33 §3.3`; `35 §5`; `36 §4.2`; `38 §1.12`.
Attached-proof selection also uses `33 §8.2`.

These cases are authored but unexecuted. No Ken check or official
conformance-runner result is claimed. No CI, publication, landing, or trust
measurement is claimed.

### surface/modules/catalog-package-homes-preserve-checked-identities

- promise class: **normative compatibility vector** — the listed pre-source
  identities and complete constructor sets are fixed by §3.3
- spec: `33 §3.3`; the IO attached proof additionally uses `33 §8.2`
- given: in a fresh pre-source environment, record the `GlobalId` for each
  listed identity before loading any catalog source. Load the four retained
  home units and a separate catalog-root consumer with these explicit imports.
  Run each client through `elaborate_module_from_roots_strict` and through the
  `elaborate_module_from_roots` and `elaborate_module_from_roots_v1`
  entrypoints that §3.3 assigns Legacy roots:

  ```ken
  import Capability.Filesystem.Authority (FS, read_bytes, write_file)
  import Capability.System.Buffer
    (BufferWindow, MkBufferWindow, buffer_span_budget, buffer_span_length,
      transfer_count_int, transfer_count_nat, transfer_count_positive,
      transfer_count_positive_prop, transfer_count_remaining,
      transfer_count_nat_succ, transfer_count_predecessor)
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
  `transfer_count_remaining`. The IO owner's attached-proof identity is
  checked through its subject in the IO-private case below.
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
- expect: the clients accept in both roots modes. Each retained home's
  pre-source ordinary export subset is exactly its listed ordinary names. The
  23 named import bindings equal their pre-source `GlobalId`s; the attached
  proof is checked through its subject in the IO-private case, not as a bare
  export. Together the checks cover all 24 rows. No pre-source identity is
  attributed to more than one home. Constructor sets are exactly
  `BufferWindow` with `MkBufferWindow`, `ResourceBodyResult` with
  `ResourceBodyOk` and `ResourceBodyErr`, and `ResourceBracketResult` with
  its four imported result constructors. Every constructor parent equals its
  pre-source family ID. The Buffer home publishes no constructor of the floor
  `BufferSpan` or `TransferCount` families.
- controls: `MkBufferWindow` and its closed one-arm match are the positive
  inductive-family control. The Resource matches reach their complete
  constructor sets rather than relying on type-name-only imports. The
  System.IO-to-System.Buffer IDs are compared with the Buffer home IDs.
  `FileOperation` is covered separately as a no-import scoped floor family.
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
  `Capability.System.Buffer`, `write_all_exact_prefix_prop` from
  `Capability.System.IO`, and `ResourceBodyResult` from
  `Capability.System.Resource`. The Buffer negative retains `BufferWindow`
  and omits only `MkBufferWindow`. Each other Strict negative omits only its
  named import; all four home roots and other dependencies remain loaded, and
  the source body stays fixed.

  ```ken
  theorem fs_type_probe
      (computation : FS AFull (Result FileError Bytes)) : Top = Proved

  const window_probe : BufferWindow = MkBufferWindow 1 2

  fn io_subject_probe (span : BufferSpan) (count : TransferCount)
      (evidence : write_all_exact_prefix_prop span count)
    : write_all_exact_prefix_prop span count = evidence

  fn resource_result_type_probe (e : Type) (a : Type)
      (result : ResourceBodyResult e a) : ResourceBodyResult e a = result
  ```

  For each probe, make a positive and a negative Strict-root client that
  differ only by its home import. Pair `FS` with Authority,
  `MkBufferWindow` with Buffer, `write_all_exact_prefix_prop` with IO, and
  `ResourceBodyResult` with Resource. The IO probe uses the subject in a
  proposition type; attached-proof visibility is tested separately.
  `FileOperation` is no longer a System.Error home and is tested separately.
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
- controls: the four import-restored Strict clients isolate each retained
  home edge. The complete import clients in the preceding case are accepted
  controls.
- why: Strict roots resolve only through explicit imports and the closed
  prelude; a name held only by a catalog home is not ambient.

### surface/modules/inc2b-mapping-child-homes-preserve-identities

- promise class: **normative compatibility vector** — the listed checked
  pre-source names, home edges, and complete public constructor families are
  fixed by the spec; a changed home or identity requires a contract change
- spec: `30-taxonomy §4`; `38 §1.9, §1.11`; `33 §3.3`
- given: in a fresh pre-source environment, snapshot the `GlobalId` for each
  of the 20 names below before loading catalog sources. Load both home units
  and a separate client through `elaborate_module_from_roots_strict` with
  these explicit imports:

  ```ken
  import Capability.System.Mapping
    (MappingHandle, MappingWindow, MkMappingWindow, MappingSource,
      Anonymous, FileBacked, mapBytes, mapWrite)
  import Capability.Process.Child
    (ChildStdio, ChildPipe, ChildNull, ChildInherit, spawn_plan,
      with_environment, with_stdin, with_stdout, withChild,
      wait_child, take_stdout, read_pipe)

  fn mapping_handle_control (mapping : MappingHandle) : MappingHandle =
    mapping

  const mapping_window_control : MappingWindow = MkMappingWindow 1 2

  fn mapping_window_cases (window : MappingWindow) : Int =
    match window {
      MkMappingWindow offset length ↦ offset
    }

  fn mapping_source_cases (source : MappingSource) : Bool =
    match source {
      Anonymous length ↦ True;
      FileBacked handle length ↦ False
    }

  fn child_stdio_cases (target : ChildStdio) : Bool =
    match target {
      ChildPipe ↦ True;
      ChildNull ↦ False;
      ChildInherit stream ↦ True
    }
  ```

  In a separate Strict-root client, import `MappingHandle` and try this
  private-constructor pattern:

  ```ken
  import Capability.System.Mapping (MappingHandle)
  fn private_mapping_handle_probe (mapping : MappingHandle) : Bool =
    match mapping {
      PrivateMappingHandle resource extent ↦ True
    }
  ```

  Keep the accepted `mapping_handle_control` as the positive control; do not
  pin a diagnostic for the private-constructor refusal. This case makes no
  claim about the deferred `SpawnPlan`, `with_stderr`, `take_stdin`, or
  `take_stderr` homes.

  The Mapping inventory is `MappingHandle`, `MappingWindow`,
  `MkMappingWindow`, `MappingSource`, `Anonymous`, `FileBacked`,
  `mapBytes`, and `mapWrite`. The Child inventory is `ChildStdio`,
  `ChildPipe`, `ChildNull`, `ChildInherit`, `spawn_plan`,
  `with_environment`, `with_stdin`, `with_stdout`, `withChild`,
  `wait_child`, `take_stdout`, and `read_pipe`.
- measured: each home's public pre-source ordinary export identities, the
  consumer's resolved import `GlobalId`s, and each constructor's parent
  identity. The snapshot precedes loading both homes; no numeric `GlobalId`
  is hardcoded.
- claimed: every listed name imports from its specified home with its
  original checked identity, and each public data family is complete.
- the gap: matching spellings can hide a copied or rebound identity; a type
  import alone can also hide an omitted public constructor.
- expect: the client accepts. Each home's public pre-source ordinary export
  subset is exactly its inventory above; all 20 resolved imports equal their
  independent pre-source snapshots, and no identity is homed twice. The
  constructor sets are exactly `MappingWindow` with `MkMappingWindow`,
  `MappingSource` with `Anonymous` and `FileBacked`, and `ChildStdio` with
  `ChildPipe`, `ChildNull`, and `ChildInherit`; each constructor's parent
  equals its snapshotted family identity. `MappingHandle` is available as an
  abstract type, but its private constructor and fields are not publicly
  nameable.
- controls: the `MappingWindow`, `MappingSource`, and `ChildStdio` matches
  omit catch-all arms, so an incomplete public family refuses. The
  `MappingHandle` identity-function control uses its public type; the separate
  private-constructor probe must refuse at surface visibility.
- why: the exact imported `GlobalId` and parent relations distinguish these
  existing checked homes from a same-spelling replacement or incomplete
  family.

### surface/modules/inc2b-strict-home-imports-are-required

- promise class: **durable provider-edge invariant** — each new home name
  requires its explicit Strict-root import rather than a flat-global fallback
- spec: `30-taxonomy §4`; `38 §1.9, §1.11`; `33 §3.3`
- given: for each owner, load the same dependency-closed roots and make two
  fresh Strict-root clients differing only in the selected constructor's
  import. The Mapping pair is:

  ```ken
  import Capability.System.Mapping (MappingWindow, MkMappingWindow)
  const window_probe : MappingWindow = MkMappingWindow 1 2
  ```

  The negative Mapping client retains the `MappingWindow` import but omits
  `MkMappingWindow`; its source body is unchanged. The Child pair is:

  ```ken
  import Capability.Process.Child (ChildStdio, ChildPipe)
  const child_stdio_probe : ChildStdio = ChildPipe
  ```

  The negative Child client retains the `ChildStdio` import but omits
  `ChildPipe`; its source body is unchanged. The owner module and all other
  dependencies remain loaded in each pair.
- measured: the exact Strict-root resolution outcome for each client and the
  name reported by a refusal.
- claimed: each new module home is necessary for its imported public names;
  another loaded unit or the implementation-global table cannot supply an
  omitted name.
- the gap: a negative that differs in another import or in its source body
  could refuse for the wrong reason. Here each positive differs from its
  negative by only one constructor import.
- expect: both imported clients accept. The Mapping negative refuses with
  `UnboundName` naming `MkMappingWindow`; the Child negative refuses with
  `UnboundName` naming `ChildPipe`. No other missing dependency or type error
  satisfies either negative.
- controls: each accepted client is the control for its paired omitted-import
  refusal. The preceding identity case imports and checks the complete 8-name
  and 12-name inventories under Strict roots.
- why: Strict roots resolve through explicit imports, built-in vocabulary,
  and the closed prelude, not a flat implementation-global fallback.

### surface/modules/legacy-package-home-bare-uses-remain-until-l2-4

- promise class: **transition sentinel** — the current Legacy bare-use
  compatibility boundary must be reviewed when the roots entrypoint flips
- spec: `33 §3.3` (Legacy resolution and the L2-4 strict-entry flip)
- given: run each of the four source probes from
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
  probes use `FS`, `MkBufferWindow`, `write_all_exact_prefix_prop`, and
  `ResourceBodyResult` respectively. `FileOperation` is now a floor identity,
  not a home, and its no-import behavior is pinned separately.
- expect: all four Legacy clients accept. The corresponding Strict clients
  without home imports reject with `UnboundName`, as specified in the prior
  case.
- controls: the imported Legacy clients in the identity case remain the
  exact-identity controls; the Strict missing-import clients distinguish the
  Legacy result from ambient availability.
- why: changing the existing bare-use result is incompatible until the
  `L2-4` strict-entry flip. That successor owns the transition and must review
  or replace this sentinel for the named roots entrypoints.

### surface/modules/system-io-attached-proof-remains-private-to-owner

- promise class: **durable invariant** — an exported subject does not expose
  a private attached proof to external clients
- spec: `33 §3.3`, `33 §4.1`, and `33 §8.2`
- given: load the actual `Capability.System.IO` home under Strict and Legacy
  roots. Its checked `write_all_preserves_exact_prefix` theorem selects
  `exact_prefix` through `write_all_exact_prefix_prop`; record that internal
  reference's resolved `GlobalId`. For an external client in each mode,
  import only `write_all_exact_prefix_prop` and include the accepted
  `io_subject_probe` from the preceding case. Append one claim per fresh
  copy, leaving the subject import and probe unchanged:

  ```ken
  theorem external_via_path
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
      write_all_exact_prefix_prop::exact_prefix span count
  ```

  ```ken
  theorem external_via_selector
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
      proof exact_prefix for write_all_exact_prefix_prop span count
  ```

  ```ken
  theorem external_bare_name
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count = exact_prefix span count
  ```
- measured: the IO unit's internal attached-proof identity, its public
  subject import, and each external reference's surface-resolution result.
- claimed: the checked proof is homed with its subject and usable inside IO,
  but subject import alone does not export the private proof.
- the gap: each external refusal must reach proof visibility, not fail because
  the subject or its import is missing. The accepted subject probe provides
  that control.
- expect: the IO unit resolves its internal selector to the pre-source proof
  `GlobalId` under `write_all_exact_prefix_prop`. Its public interface
  includes the subject but not the attached proof. The external subject import
  and `io_subject_probe` accept; both qualified external forms reject as
  surface visibility errors. The bare `exact_prefix` name is unresolved.
- controls: the IO unit's internal proof use is the positive owner control.
  The next case controls external selection when the proof is explicitly
  public.
- why: §8.2 makes attached proofs private by default; the home mapping does
  not widen the IO module's ordinary public interface.

### surface/modules/public-attached-proof-selector-requires-pub

- promise class: **durable invariant** — a public attached proof is selected
  through its subject, never as a bare name
- spec: `33 §4.1` and `33 §8.2`
- given: in two fresh roots fixtures, place the provider at path
  `PublicProofHome.ken` (its implicit module path is `PublicProofHome`) and
  each consumer in a separate roots-loaded unit. The provider contains these
  direct declarations, with no inline module wrapper. The private variant
  differs only by omitting `pub` before `proof identity_self`:

  ```ken
  pub fn identity (x : Int) : Int = x;
  pub proof identity_self for identity (x : Int)
    : Equal Int (identity x) x = Refl
  ```

  In each fixture, run separate external clients that import only `identity`:

  ```ken
  import PublicProofHome (identity)

  theorem public_via_path (x : Int)
    : Equal Int (identity x) x = identity::identity_self x
  ```

  ```ken
  import PublicProofHome (identity)

  theorem public_via_selector (x : Int)
    : Equal Int (identity x) x = proof identity_self for identity x
  ```

  The bare-name control imports the same subject and theorem type but uses
  `identity_self x` as its proof reference:

  ```ken
  import PublicProofHome (identity)

  theorem public_via_bare (x : Int)
    : Equal Int (identity x) x = identity_self x
  ```
- measured: the attached-proof `GlobalId` resolved through the exported
  subject, the surface result when only `pub` changes, and the bare-name
  resolution outcome.
- claimed: `pub proof` makes the attached proof available through its subject;
  it does not add `identity_self` to the ordinary namespace.
- the gap: the subject and claim remain identical between fixture variants;
  the `pub` marker is the only visibility change. The proof claim mentions
  `identity` applied and is well-formed.
- expect: both qualified forms accept and resolve the same attached-proof
  `GlobalId` in the public variant. Both external qualified forms refuse in
  the private variant at surface visibility. The bare `identity_self`
  reference refuses as unresolved even when `pub proof` is present.
- controls: the public subject import and qualified proof are accepted
  controls for the private-visibility refusal. The bare-name client differs
  only at the proof reference.
- why: §8.2 requires both an exported subject and `pub proof` for external
  attached-proof visibility; subject selection alone cannot cross privacy.

### surface/modules/inc3a-file-operation-is-a-scoped-floor-family

- promise class: **normative compatibility vector** — the existing checked
  identity and complete constructor family are fixed by the spec
- spec: `30-taxonomy §4`; `33 §3.3`; `38 §1.12`
- given: before loading catalog source, snapshot the pre-source `GlobalId` of
  `FileOperation` and each of its sixteen constructor identities. In a fresh
  Strict-roots client with no `Capability.System.Error` import, use the exact
  type and complete qualified family:

  ```ken
  fn file_operation_control (operation : FileOperation) : Bool =
    match operation {
      FileOperation.OpReadFile ↦ True;
      FileOperation.OpWriteFile ↦ False;
      FileOperation.OpAppendFile ↦ False;
      FileOperation.OpMetadata ↦ True;
      FileOperation.OpReadDirectory ↦ True;
      FileOperation.OpCreateDirectory ↦ False;
      FileOperation.OpRemoveFile ↦ False;
      FileOperation.OpRemoveDirectory ↦ False;
      FileOperation.OpRename ↦ False;
      FileOperation.OpChangeMode ↦ True;
      FileOperation.OpSeek ↦ False;
      FileOperation.OpSetLength ↦ True;
      FileOperation.OpSync ↦ True;
      FileOperation.OpGetInheritance ↦ True;
      FileOperation.OpSetInheritance ↦ True;
      FileOperation.OpDuplicate ↦ False
    }
  ```

  For each constructor `C` in that closed family, make a separate pair of
  Strict clients with the same body and no imports. The positive expression
  is `const operation : FileOperation = FileOperation.C`; its paired negative
  changes only the right-hand side to bare `C`. Also compare two source
  declaration controls: `data FileOperation = LocalFileOperation` and
  `data LocalOperation = LocalFileOperation`. The first attempts to replace a
  floor name; the second controls that the declaration form is otherwise
  accepted.

  Load `Capability.System.Error` as a dependency in both clients of one
  further import pair. The accepted client uses `FileOperation` without
  importing it. Its paired client adds only
  `import Capability.System.Error (FileOperation, OpReadFile)`.
- measured: resolution of the type and all constructor paths against the
  pre-source snapshots; acceptance or refusal of each paired source client;
  and the public export set of `Capability.System.Error` for this family.
- claimed: `FileOperation` is an always-present scoped floor family, not a
  package home. All sixteen qualified cases retain their original IDs and
  parent; none of the bare cases is reserved or resolved to that family.
  `Capability.System.Error` no longer exports or homes the family.
- the gap: matching source spellings do not establish canonical identity or
  complete family registration. An unrelated failure in the negative clients
  could mimic the scoped-name or retired-home refusal; each positive keeps the
  source body fixed and changes only the spelling/import under test.
- expect: the no-import exhaustive match accepts and every measured ID equals
  its pre-source snapshot. Each qualified `FileOperation.C` client accepts;
  its bare `C` pair refuses with `UnboundName` naming `C`. The replacement
  named `FileOperation` refuses, while the `LocalOperation` control accepts.
  The old System.Error import refuses at import resolution; its no-import
  control accepts. No constructor ID, parent, or native artifact leaf changes.
- controls: the complete match and each qualified-constructor client are
  positive controls for their paired bare-name refusals. The local-name
  declaration controls the replacement refusal. The no-import client is the
  positive control for the retired Error-home import.
- why: moving this identity from H to B must preserve the exact family while
  making its sixteenth case usable by qualification, without leaving an old
  package route or bare-constructor fallback.

### surface/modules/inc3a-added-scoped-floor-families

- promise class: **normative compatibility vector** — the five remaining
  scoped floor families and `resp_state` retain their exact identities and
  complete public paths
- spec: `30-taxonomy §4`; `33 §3.3`; `36 §4.2`; `38 §1.12`
- given: in a fresh environment, snapshot the pre-source `GlobalId` of
  `ReadResult`, `FileKind`, `FileMetadata`, `DirEntry`, `StateOp`,
  `resp_state`, and every public constructor below. Use a Strict-roots client
  with no imports for these names:

  ```ken
  fn read_result_cases (result : ReadResult) : Bool =
    match result {
      ReadResult.Chunk bytes ↦ True;
      ReadResult.Eof ↦ False
    }

  fn file_kind_cases (kind : FileKind) : Bool =
    match kind {
      FileKind.KFile ↦ True;
      FileKind.KDirectory ↦ True;
      FileKind.KSymlink ↦ False;
      FileKind.KOther ↦ False
    }

  const metadata_control : FileMetadata =
    FileMetadata.MkFileMetadata 0 FileKind.KFile

  const entry_control : DirEntry =
    DirEntry.MkDirEntry b"file" FileKind.KDirectory

  fn state_operation_cases (operation : StateOp Bool) : Bool =
    match operation {
      StateOp.Get ↦ True;
      StateOp.Put value ↦ value
    }

  fn get_response (response : resp_state Bool StateOp.Get) : Bool =
    response

  fn put_response (response : resp_state Bool (StateOp.Put True)) : Unit =
    response
  ```

  The complete public constructor sets are `ReadResult.Chunk` and
  `ReadResult.Eof`; `FileKind.KFile`, `KDirectory`, `KSymlink`, and `KOther`;
  `FileMetadata.MkFileMetadata`; `DirEntry.MkDirEntry`; and `StateOp.Get` and
  `StateOp.Put`. For each listed `T.C`, make a separate positive client using
  the qualified constructor and a paired client that changes only `T.C` to
  bare `C`, with no local binding or import for `C`.

  Make a separate source declaration replacement client for each of
  `ReadResult`, `FileKind`, `FileMetadata`, and `DirEntry`, using
  `data T = LocalT` with the target name substituted for `T`. Replace
  `StateOp` with `data StateOp (s : Type) = LocalStateOp`. Each is paired
  with the same declaration under a unique local type name, which must
  accept. For `resp_state`, attempt a same-typed local definition:

  ```ken
  fn resp_state (s : Type) (operation : StateOp s) : Type =
    match operation {
      StateOp.Get ↦ s;
      StateOp.Put value ↦ Unit
    }
  ```

  Pair it with the same definition named `local_resp_state`. A separate
  negative client uses `State Bool` as a type; its positive control uses
  `StateOp Bool` as the checked operation carrier.
- measured: each unimported reference's resolved `GlobalId`, each constructor
  parent, the `resp_state` result types at `Get` and `Put`, and each paired
  client's elaboration verdict.
- claimed: each new floor identity is available without import at its exact
  pre-source identity. Each inductive family is complete and scoped: public
  cases resolve as `T.C`, not bare `C`. `resp_state` is an operation with
  type `(s : Type) → StateOp s → Type`, reducing to `s` for `Get` and `Unit`
  for `Put`; it has no constructor family. The source uses `StateOp` as the
  carrier, not `State` as a second type.
- the gap: a successful constructor spelling could resolve to a same-shaped
  source declaration, and a no-import acceptance alone would not prove the
  response family has the specified dependent cases. The snapshot, complete
  matches, type-level responses, and same-name replacement pairs expose those
  alternatives.
- expect: all no-import clients accept and resolve the snapshotted IDs.
  `ReadResult.Chunk`/`Eof`, the four `FileKind` cases, both single-constructor
  families, and `StateOp.Get`/`Put` are complete; every listed bare `C` pair
  refuses with `UnboundName` naming `C`. The `Get` response has type `Bool`
  and the `Put` response has type `Unit`. Each same-name floor replacement
  refuses, while a same-shaped declaration under its unique local control
  name accepts. The `State Bool` type use refuses with `UnboundName`; the
  `StateOp Bool` carrier use accepts.
- controls: each qualified constructor client controls its bare-name pair;
  each no-import type/operation use controls its source-name replacement.
  The `get_response` and `put_response` declarations are separate controls for
  the two `resp_state` reductions.
- why: the names are source-required checked identities, not optional package
  declarations; their complete paths and parent identities cannot be replaced
  by lookalikes.

### surface/modules/inc3a-added-checked-homes-require-each-import

- promise class: **durable provider-edge invariant** — every new H name needs
  its own explicit Strict-root import and retains its checked identity
- spec: `30-taxonomy §4`; `33 §3.3–§4.3`; `38 §1.12`
- given: snapshot each listed name's pre-source `GlobalId` before loading any
  catalog source. For every row, create a separate Strict-roots client with
  the owner's public export loaded. The positive client contains
  `import Owner (name)` followed by `export name`. Its paired negative has
  the same source body and dependencies, but omits only that import. This
  export is a source use of the imported binding and preserves its canonical
  identity under §4.3. For example, the `host_clock` pair is:

  ```ken
  import Capability.Process.Exit (host_clock)
  export host_clock
  ```

  The paired client removes only the import and retains `export host_clock`.
  Instantiate this pair independently for every row below.

  | H name | Owner |
  |---|---|
  | `append_file` | `Capability.Filesystem.Authority` |
  | `change_mode` | `Capability.Filesystem.Authority` |
  | `create_directory` | `Capability.Filesystem.Authority` |
  | `file_metadata` | `Capability.Filesystem.Authority` |
  | `read_directory` | `Capability.Filesystem.Authority` |
  | `remove_directory` | `Capability.Filesystem.Authority` |
  | `remove_file` | `Capability.Filesystem.Authority` |
  | `rename_file` | `Capability.Filesystem.Authority` |
  | `writeAt` | `Capability.System.IO` |
  | `writeAll` | `Capability.System.IO` |
  | `freeze` | `Capability.System.Buffer` |
  | `resourceMetadata` | `Capability.System.Resource` |
  | `flush` | `Capability.Console.Text` |
  | `is_terminal` | `Capability.Console.Text` |
  | `intToChar` | `Core.Classes.LawfulClasses` |
  | `SpawnPlan` | `Capability.Process.Child` |
  | `with_stderr` | `Capability.Process.Child` |
  | `take_stdin` | `Capability.Process.Child` |
  | `take_stderr` | `Capability.Process.Child` |
  | `read` | `Capability.Console.Text` |
  | `wall_now` | `Capability.Time.WallClock` |
  | `host_clock` | `Capability.Process.Exit` |

- measured: each positive client's import and re-export resolution, the
  resulting canonical `GlobalId`, and each paired omission's exact name error.
  The omitted-import `export name` is the source-name reference under test.
- claimed: all twenty-two names have the specified public home and original
  pre-source identity; an unrelated loaded module or flat-global entry cannot
  satisfy a missing Strict import.
- the gap: an omitted-import rejection could arise from another missing
  dependency or an invalid source body. Separate clients differ only by the
  selected import, while the accepted re-export is the positive control.
- expect: every imported client accepts and its re-export equals the
  pre-source `GlobalId` snapshot. Every omission client rejects with
  `UnboundName` naming exactly its omitted H name. No name is added to the
  floor, and each home publishes the checked identity rather than a copy.
- controls: the import-restored client is the positive control for each
  omitted-import case; the independent pre-source snapshot controls identity.
- why: each row is a separate route from an ordinary package home. A home
  cannot be inferred from a sibling name or from implementation-global
  registration.

### surface/modules/inc3a-fixed-width-imports-and-safe-range-boundaries

- promise class: **durable invariant** — each named conversion is explicit,
  imported from its home, and succeeds exactly on its target range
- spec: `35 §5`; `30-taxonomy §4`; `33 §3.3`; `10-kernel/18a §5.7`
- given: snapshot the eight pre-source conversion `GlobalId`s before loading
  catalog source. For each row, make a separate Strict-roots client importing
  only that conversion from `Data.Numeric.FixedWidth`. Let `f`, `T`, `lo`,
  and `hi` be that row's function, target, and inclusive bounds. The client
  declares four `Option T` values by evaluating `f (lo - 1)`, `f lo`, `f hi`,
  and `f (hi + 1)`. Pair it with an otherwise identical client that omits
  only this conversion import. The range endpoints are contract literals:

  | Conversion | Target range | Expected results |
  |---|---:|---|
  | `intToInt8` | `-128..127` | `-129 → None Int8`; `-128 → Some Int8 (-128)`; `127 → Some Int8 127`; `128 → None Int8` |
  | `intToInt16` | `-32768..32767` | `-32769 → None Int16`; `-32768 → Some Int16 (-32768)`; `32767 → Some Int16 32767`; `32768 → None Int16` |
  | `intToInt32` | `-2147483648..2147483647` | `-2147483649 → None Int32`; `-2147483648 → Some Int32 (-2147483648)`; `2147483647 → Some Int32 2147483647`; `2147483648 → None Int32` |
  | `intToInt64` | `-9223372036854775808..9223372036854775807` | `-9223372036854775809 → None Int64`; `-9223372036854775808 → Some Int64 (-9223372036854775808)`; `9223372036854775807 → Some Int64 9223372036854775807`; `9223372036854775808 → None Int64` |
  | `intToUInt8` | `0..255` | `-1 → None UInt8`; `0 → Some UInt8 0`; `255 → Some UInt8 255`; `256 → None UInt8` |
  | `intToUInt16` | `0..65535` | `-1 → None UInt16`; `0 → Some UInt16 0`; `65535 → Some UInt16 65535`; `65536 → None UInt16` |
  | `intToUInt32` | `0..4294967295` | `-1 → None UInt32`; `0 → Some UInt32 0`; `4294967295 → Some UInt32 4294967295`; `4294967296 → None UInt32` |
  | `intToUInt64` | `0..18446744073709551615` | `-1 → None UInt64`; `0 → Some UInt64 0`; `18446744073709551615 → Some UInt64 18446744073709551615`; `18446744073709551616 → None UInt64` |

- measured: each client's resolved conversion `GlobalId` and the four
  returned `Option T` values for its target.
- claimed: each imported name resolves to its existing checked identity and
  returns `Some` exactly within its target range, `None` just outside. No
  native unchecked raw cast or implicit conversion supplies these results.
- the gap: a range check for one width cannot establish the other seven
  widths' limits or import edges. Each conversion has its own import/omission
  pair and its own lower/upper boundary pair.
- expect: all eight imported clients accept and match all four expected
  outputs in their row. Each omission client rejects with `UnboundName`
  naming exactly its conversion. In-range endpoints return `Some T value`;
  adjacent out-of-range values return `None T`.
- controls: each import-restored client controls its one-name omission.
  The two in-range values control that a conversion is not an always-`None`
  stub; the adjacent out-of-range values distinguish safe narrowing from a
  wrapping raw cast.
- why: all eight definitions share one home but have distinct checked IDs and
  target ranges; no representative width stands for the remaining seven.
