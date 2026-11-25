# Directory reading uses a batch stream

Format: `../../README.md`.
Spec: ABI-S2 directory stream in `spec/30-surface/38-ffi-io.md`
(parallel Spec child).

Clean-room record: no reference-contact. No material under `local/refs` was
consulted; expected behavior is grounded in Architect AC-0
`evt_1d1tqakxsz753` and the ABI-S2 frame.

These cases cover `withDirectory` and `readDirectoryBatch`. They pin only the
public stream results and the specified open error. They do not assert syscall
order, raw directory order, or a particular number of batches beyond the
multi-batch distinction. This seed is unexecuted; no official conformance run,
CI, publication, or landing is claimed.

## Multi-batch enumeration returns every entry once

### surface/ffi-io/directory-stream-multi-batch-contents-and-sticky-end

- promise class: **durable invariant** — an unmodified directory is enumerated
  exactly once, with an explicit sticky end marker
- spec: ABI-S2 directory-stream contract in `spec/30-surface/38-ffi-io.md`
- given: a runtime fixture creates an unmodified directory beneath a real
  `Cap APartial` with `Enumerate` authority. It contains 400 regular files
  with distinct 200-byte ASCII names. A client opens the directory with
  `withDirectory`, calls `readDirectoryBatch` until the first
  `DirectoryProgress.DirectoryEnd`, then calls it once more. The consumer
  has fixture-specific `Nat` fuel 402: enough for one entry per batch, one
  end, and one sticky-end read. Exhausting fuel before that observation fails
  the case.
- measured: the sequence of public `DirectoryProgress` values and the
  `DirEntry` names returned by each `DirectoryProgress.DirectoryEntries`.
  The expected name multiset comes from the independent fixture inventory.
- claimed: the stream may split entries across batches, but for an unmodified
  directory it returns each entry exactly once and ends explicitly; the end
  result remains stable on later reads.
- the gap: a count alone can hide both a duplicate and an omission. The case
  compares the complete multiset of names with the independently created set.
- expect: at least two nonempty `DirectoryProgress.DirectoryEntries` values
  precede `DirectoryProgress.DirectoryEnd`; every data batch is nonempty.
  Across all batches, each of the 400 fixture names occurs exactly once and no
  other name occurs. The next `readDirectoryBatch` after the first end also
  returns `DirectoryProgress.DirectoryEnd`. No order among entries or batches
  is required.
- controls: the multi-batch fixture rejects an implementation that returns
  only an end marker or stops after one data batch. The next-read assertion
  distinguishes a sticky end from a transient empty result.
- why: the fixture size forces multiple public data batches; the exact count
  of batches is not a contract value. One-entry-per-directory or one-batch-only
  behavior would lose entries, while sorting or pinning a host's order would
  add a promise the contract leaves unspecified.

## Empty directories return the same end marker

### surface/ffi-io/empty-directory-returns-sticky-end

- promise class: **durable invariant** — an empty directory is represented by
  the explicit end variant, not an empty data batch
- spec: ABI-S2 directory-stream contract in `spec/30-surface/38-ffi-io.md`
- given: a separate empty directory beneath the same kind of real
  `Cap APartial` with `Enumerate` authority. The client opens it with
  `withDirectory` and calls `readDirectoryBatch` twice on the returned handle.
- measured: the two public `DirectoryProgress` results.
- claimed: an empty directory produces `DirectoryEnd` immediately, and
  `DirectoryEnd` is sticky.
- the gap: a single successful open does not distinguish a proper end marker
  from an empty `DirectoryEntries` value or a one-shot end.
- expect: both calls return `DirectoryProgress.DirectoryEnd`; neither returns
  `DirectoryProgress.DirectoryEntries` with an empty list.
- controls: the multi-batch case reaches nonempty `DirectoryEntries` values
  with the same operation; this case reaches the empty boundary.
- why: callers distinguish data from completion by the explicit variant, not
  by a zero-length batch.

## Directory acquisition preserves capability-root confinement

### surface/ffi-io/directory-acquisition-refuses-root-escape

- promise class: **durable invariant** — directory acquisition cannot escape
  the filesystem capability root
- spec: ABI-S2 directory acquisition in
  `spec/30-surface/38-ffi-io.md`; I-5 path-resolution and confinement pairs
  in `conformance/security/capabilities/seed-capabilities.md`
- given: one real runner-minted `Cap AFull` rooted at `dir1/sub`, an
  in-root directory at `dir1/sub/ok`, and an outside sibling directory at
  `dir1/outside`. Make two fresh `withDirectory` calls with the same cap and
  bracket body, changing only the path between `dir1/sub/ok` and
  `dir1/sub/../outside`. The body records whether it was entered.
- measured: each outer `withDirectory` result, body-entry observation,
  filesystem effect trace, and separately recorded resolver outcome. The
  resolver record does not count as the denied operation reaching the host.
- claimed: `FsOpenDirectory` applies the existing path-resolution rule at the
  capability root; resolving and reopening a traversal as an ordinary path
  must not admit an out-of-root directory.
- the gap: a valid in-root open alone cannot distinguish root-relative
  resolution from an implementation that accepts an escaped path. The paired
  operation and capability hold those inputs fixed.
- expect: the in-root directory opens and enters the body. The
  `dir1/sub/../outside` request is refused at the I-5 `ScopeEscape` boundary,
  before body entry and with an empty filesystem effect trace. Do not pin
  diagnostic wording or span beyond the refusal specified by I-5.
- controls: the in-root directory is the positive control under the same cap,
  operation, and body; only the target path changes.
- why: I-5 AC1–AC3 already distinguish beneath-root resolution from accepting
  an escaped path on other filesystem operations. This pair applies that same
  boundary to the new directory-acquisition operation.

## Opening a regular file refuses as not a directory

### surface/ffi-io/regular-file-opened-as-directory-refuses-not-directory

- promise class: **normative compatibility vector** — the directory-open error
  retains its operation tag and cause
- spec: ABI-S2 directory-stream contract in `spec/30-surface/38-ffi-io.md`
- given: under one directory-enumeration capability, place a regular file and
  a directory beneath the same permitted parent. In separate fresh calls,
  invoke `withDirectory` on each path with the same body.
- measured: the outer `withDirectory` result for each path.
- claimed: directory mode is checked at acquisition; a regular file is not
  accepted as a directory, and the error is attributed to the directory-open
  operation.
- the gap: a refusal for an insufficient capability, missing path, or later
  batch call would not establish the `NotDirectory` contract. The paired valid
  directory keeps the capability and parent constant.
- expect: opening the directory succeeds, so the empty or multi-batch stream
  controls above can be reached. Opening the regular file returns
  `FileError(FileOperation.OpReadDirectory, Some path, NotDirectory)` from
  `withDirectory` before its body is entered. Do not pin diagnostic wording or
  span.
- controls: the directory path is the positive control with the same
  capability, parent, and bracket body; only the terminal object differs.
- why: `FileOperation.OpReadDirectory` remains the open-error tag even though
  the old whole-list `read_directory` operation is retired.
