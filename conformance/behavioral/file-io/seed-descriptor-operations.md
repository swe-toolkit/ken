# Filesystem descriptor operations — seed cases (DESCRIPTOR)

Format: `../../README.md`. These cases cover the black-box behavior added by
`spec/30-surface/38-ffi-io.md §1.7.1`: `setLength` and `sync` on a live
filesystem resource. They use the recording-host fixture form already used by
`behavioral/buffer-io/seed-buffer-io.md`; the assertions observe the program's
public result and the fixture's file contents or host request, not Rust
internals. Each `withResource` body returns its operation result(s) as data;
its acquisition and settlement succeed. The `expect` values are the inner
operation results, not the outer bracket result.

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
