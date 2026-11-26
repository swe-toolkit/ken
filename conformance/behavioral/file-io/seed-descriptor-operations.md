# Filesystem descriptor operations — seed cases (DESCRIPTOR)

Format: `../../README.md`. These cases cover `spec/30-surface/38-ffi-io.md`
§1.7.1: `setLength` and `sync` on a live filesystem resource, plus the
close-on-exec flag on each acquired `FsHandle`. The operation cases use the
recording-host fixture form already used by
`behavioral/buffer-io/seed-buffer-io.md`; the descriptor case observes
host-boundary flags and child descriptors. Assertions use public results or
fixture state, not Rust internals. Each operation-case `withResource` body
returns its result(s) as data; acquisition and settlement succeed. Their
`expect` values are the inner results, not the outer bracket result.

These cases are authored but have not been run by the official conformance
runner. No CI, publication, or landing result is claimed.

## FS-A. Truncation and extension preserve the specified bytes

### file-io/set-length-truncates-and-zero-extends

- spec: `38 §1.3.1`, `§1.7.1`
- given: seed an existing host file with the bytes `ABCDEFGH`. Run a checked
  client with an explicit `visits [FS]` row, the `Capability.System.IO`
  `setLength` import, and valid FS authority. Acquire the file through
  `withResource` in
  `ResourceOpenMode.ResourceWriteCreate CreatePolicy.CreateOrKeep`. On that
  live handle, call `setLength` with `5`, then with `8`.
- measured: both public `Result ResourceError Unit` values and the final bytes
  observed from the host fixture after the bracket settles.
- claimed: shrinking preserves the retained prefix; extending zero-fills the
  newly exposed suffix.
- the gap: the fixture must execute the actual checked `HostIO` client against
  its recording filesystem and expose the resulting file bytes. It must not
  substitute a hand-fed `setLength` result.
- expect: both calls return `Ok Unit`. The final file is exactly the five-byte
  prefix `ABCDE` followed by three zero bytes. This distinguishes truncation
  from a no-op and zero-extension from restoring the discarded `FGH` bytes.
- why: `setLength` must implement the locked extent change, not merely accept
  the request or change a reported length.

## FS-B. Invalid lengths are refused before host access

### file-io/set-length-invalid-length-refuses-before-host

- spec: `38 §1.3.1`, `§1.7.1`
- given: seed each fresh host file with bytes `ABCDEFGH`. Acquire a writable
  handle through `withResource` in
  `ResourceOpenMode.ResourceWriteCreate CreatePolicy.CreateOrKeep`. Each
  checked client imports `setLength` from `Capability.System.IO`, declares
  `visits [FS]`, and receives valid FS authority. The invalid clients differ
  from a valid control only in the `setLength` argument: `-1` or the positive
  integer `18446744073709551616` on a 64-bit host. The positive control uses
  `4` on the same initial bytes and handle mode.
- measured: the exact public result, any host set-length request, and the
  fixture file contents after the call.
- claimed: a negative or host-width-unrepresentable length returns
  `InvalidOffset` before dispatch.
- the gap: the result alone cannot distinguish an early refusal from a host
  call that returns the same error; the recorder must observe the actual host
  boundary, and the file state is checked independently.
- expect: both invalid arms return
  `Err (ResourceError.InvalidOffset)`, leave the original bytes unchanged,
  and produce no host set-length request. The control returns `Ok Unit`,
  changes the file to four bytes, and reaches the host fixture.
- why: the accepted control proves the handle and recorder are live. The
  exact error plus zero host requests distinguishes pre-dispatch range
  admission from a host-side rejection or a wrapped length.

## FS-C. A read-only handle cannot change the extent

### file-io/set-length-requires-write-right

- spec: `38 §1.3.1`, `§1.7.1`
- given: seed each host file with bytes `ABCDEFGH` and provide the same valid
  FS authority. Compare fresh checked clients with `visits [FS]` that import
  `setLength` from `Capability.System.IO` and acquire through `withResource`.
  Both invoke `setLength` on the acquired resource with length `4`; only the
  bracket mode differs: `ResourceOpenMode.ResourceRead` versus
  `ResourceOpenMode.ResourceWriteCreate CreatePolicy.CreateOrKeep`.
- measured: the exact public result, host set-length requests, and final file
  contents.
- claimed: a non-writable handle is refused with `RightNotHeld` before
  backend access.
- the gap: `RightNotHeld` could otherwise be returned after a host operation;
  both the request recorder and unchanged file are part of the observation.
- expect: the read-mode arm returns
  `Err (ResourceError.RightNotHeld)`, makes no host set-length request, and
  leaves the file unchanged. The writable control returns `Ok Unit`, reaches
  the host fixture, and leaves a four-byte file.
- why: only the handle mode changes. The writable control rules out an
  always-refuse implementation, while the no-request observation rules out a
  late rights check.

## FS-D. Sync requests a flush and returns the host outcome

### file-io/sync-returns-host-outcome

- spec: `38 §1.3.1`, `§1.7.1`
- given: seed a host file with bytes `DATA`. Acquire a live writable handle
  through `withResource` in
  `ResourceOpenMode.ResourceWriteCreate CreatePolicy.CreateOrKeep`. Run
  otherwise-identical checked clients with `visits [FS]`, valid FS authority,
  and an explicit `sync` import from `Capability.System.IO`. Configure the
  recording host's sync response as `Ok Unit` in one arm and
  `Err IOError.Unsupported` in the other.
- measured: whether the host fixture observes a sync/flush request and the
  `Result ResourceError Unit` returned to the client.
- claimed: `sync` requests a host flush of file data and metadata, then returns
  that outcome through the existing resource error type.
- the gap: an operation result alone cannot distinguish a real host request
  from a no-op. The host request must be observed separately from the result.
- expect: both arms reach the host fixture. The success arm returns `Ok Unit`;
  the failure arm returns
  `Err (ResourceError.ResourceHostIO (IOError.Unsupported))`.
- named gap: durability after machine failure is not tested or claimed here;
  §38 leaves it to the host kernel and filesystem.
- why: the success and failure outcomes make the host-result mapping visible.
  Neither outcome is evidence of persistence across a machine failure.

## FS-E. Every live FsHandle is close-on-exec

### surface/ffi-io/fs-handle-fds-are-close-on-exec

- promise class: durable invariant.
- spec: `38 §1.3.1`, `§1.7.1`, `§1.11.1`.
- given: under one valid FS authority rooted at a unique temporary directory,
  acquire and keep live one `FsHandle` for every constructor of the public
  `ResourceOpenMode` family, using distinct fixture paths and mode-appropriate
  arguments and policies. Also keep live the private directory handle used by
  the `withResource` path. Nest those scopes around a checked PX10 spawn of
  `px10-held`. The body reads its readiness record, leaves stdin open, and waits
  while the host fixture observes the parent. Release the child only after
  observation.
- measured: confirm one successful live acquisition per public mode and the
  child's readiness before inspecting descriptors. Map the parent's
  `/proc/<parent>/fd` links to each distinct fixture path and the private
  directory. Read `fcntl(F_GETFD)` on every parent descriptor in the parent
  process; key the `FsHandle` flag assertions to their fixture paths and track
  the private directory descriptor separately. Read the child's
  `/proc/<pid>/fd` links while `px10-held` is blocked on stdin.
- claimed: every acquired `FsHandle` has `FD_CLOEXEC`. The child control has
  exactly `{0, 1, 2}` and no acquired fixture file or directory inode.
- the gap: the child descriptor set alone cannot prove the per-handle flag;
  PX10 closefrom can produce `{0, 1, 2}` even when `FD_CLOEXEC` is missing.
  The host fixture must assert the flag separately for every `FsHandle` mode,
  keyed to its distinct fixture path, rather than infer it from the child.
- mutation: in a separate host mutation, remove `| OFlags::CLOEXEC` from the
  `linux::open_resource_at` open flags. This changes the `FsHandle` acquisition
  path while leaving the child closefrom path intact.
- expect: without mutation, every `FsHandle` flag includes `FD_CLOEXEC` and
  the child has only `{0, 1, 2}`. Under mutation, every `FsHandle` flag
  assertion fails; the child descriptor assertion may remain green because
  closefrom is independent.
- why: direct parent-side flag observations distinguish the host obligation
  from the separately tested spawn-plan closefrom behavior.
