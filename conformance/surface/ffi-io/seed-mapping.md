# Memory-mapping conformance — seed cases (ABI-S6)

Format: `../../README.md`. These pin the normative Mapping-surface contract of
`spec/30-surface/38-ffi-io.md §1.9` (opaque runtime-owned regions and bounded
byte views; authority `docs/program/10-linux-abi-completion.md §4`). The
cases pin the *discriminating* form of each property: each describes an
observation a plausible non-conforming implementation gets wrong.
Anonymous window-direct access and fixed-granule accounting (cases 2 and 3)
have named D5a witnesses on main. The D5b candidate supplies a named
file-backed copy-on-write witness for case 1, with `ResourceRead` source
admission; case 1 still needs independent QA's measured write-through
mutation before claiming its discriminator is proven. These references
identify executable tests, not official conformance-runner execution or
an approval of the assembled candidate. The source mapping is private;
window-direct `mapWrite` alone does not establish that writes stay off the
backing file.

## Reading disciplines

**The net is a discriminating pair, never a lone positive.** "A mapping can be
read" or "a mapping charges some size" passes green-vs-green against a
write-through or host-page-size implementation. Each property below is stated as
the outcome that a specific non-conforming implementation gets **wrong**, and
each case carries that implementation.

**Copy-out, never a pointer.** As with `Buffer` (`../../surface/ffi-io/` L7 and
`38 §1.7.1`), a mapping is observed only through bounded-view copies; a case that
inspected a raw address would be testing a surface §1.9 forbids.

### surface/ffi-io/mapping-file-writes-are-copy-on-write (MAP_PRIVATE)

- promise class: **normative property** — file mappings are private
  (copy-on-write); process-local writes never reach the file
- spec: `38 §1.9` (MAP_PRIVATE isolation); `38 §1.3`
- given: a file with known original bytes opened with `ResourceRead`; a
  program that acquires a `FileBacked` mapping over that source via
  `withMapping … ReadWrite`, `mapWrite`s different bytes at an in-range
  `offset` (write range `[offset, offset+len bytes)`), then — after the
  bracket settles — reads the same file through the ordinary `§1.3`
  file API. `ResourceRead` supplies source READ authority, not a
  WRITE|READ mode; private mapping writes use the acquired mapping's
  own `ReadWrite` protection, not file-write authority.
- expect: the post-write file read returns the **original** bytes, not
  the mapped write. The in-mapping read view observes the write
  (process-local visibility) while the file is unchanged.
- fixture: `crates/ken-cli/tests/fixtures/abi_s6_mapping_file_backed_cow.ken`.
  The exact witness is:

  ```text
  crates/ken-cli/tests/abi_s6_mapping_file_backed_cow_native.rs::file_backed_mapping_write_is_private_on_both_engines
  ```

  The Runtime implementer's checkpoint reports 1/1 named test passing,
  with assertions for both engines; independent QA and the
  disk-changing write-through mutation remain outstanding. This is
  candidate-local evidence only.
- control: a `MAP_SHARED`/write-through implementation reds — it propagates the
  mapped write to the file, so the post-write file read returns the mapped bytes
  instead of the original. A lone "the write is visible in the mapping" positive
  passes such an implementation; the file-read arm is what refutes it. The pair
  is one non-degenerate unit: same program, one arm reads the mapping (write
  visible), one reads the file (write absent).
- why: privacy is the whole denotation (`38 §1.9` preserves D4); a write-through
  surface silently corrupts the backing file, the exact hazard the contract bars.

### surface/ffi-io/mapping-charges-fixed-4kib-granule (page-accounting)

- promise class: **normative property** — page-accounting is a fixed 4 KiB
  canonical granule, host-independent
- spec: `38 §1.9` (page-accounting);
  `../../../docs/program/10-linux-abi-completion.md §4`
- given: mappings acquired at 1 byte, 4096 bytes, and 4097 bytes, each observed
  through the `Mapping` resource-kind accounting.
- expect: the charged sizes are exactly `4096`, `4096`, and `8192`
  (`ceil(n / 4096) * 4096`); a zero-length mapping is not admitted;
  the native and interpreted paths charge the identical values.
- fixture: the landed `crates/ken-host/src/effect_v1.rs` test
  `abi_s6_d5a_mapping_accounting_is_fixed_4kib_and_buffer_is_byte_granular`
  pins exact charges at the resource table. The landed test
  `fixed_accounting_boundary_lengths_reach_both_engines` in
  `crates/ken-cli/tests/abi_s6_mapping_surface_native.rs` checks the
  source-constructible 1- and 4097-byte acquisitions and engine parity.
  The trace does not expose a live-capacity counter; exact charges are
  not inferred from parity alone.
- control: a `sysconf(_SC_PAGESIZE)`-derived or byte-granular implementation reds
  — on a 16 KiB-page host a host-derived rule charges `16384` for the 1-byte
  mapping, and a byte-granular rule charges `1`/`4096`/`4097`; both differ from
  the canonical `4096`/`4096`/`8192`. The case is host-independent by
  construction: the expected values do not vary with the runtime's real page
  size, so the same seed passes on every host **only** for an implementation
  that ignores the host page size in accounting.
- why: the granule is normative precisely so accounting is deterministic and
  portable; a host-tracking rule makes a program's resource accounting
  non-portable, which the fixed granule exists to prevent.

### surface/ffi-io/mapping-is-opaque-and-bounds-checked (opacity + bounds)

- promise class: **normative property** — absolute opacity; bounded, checked
  access
- spec: `38 §1.9` (opacity, views, `ResourceKindMismatch`); `38 §1.7.1`
- given: an acquired `MappingHandle`; a `mapBytes` on an in-range
  `MappingWindow` and one on a window past the mapping extent; and a live
  `Mapping` token supplied to a buffer-only operation (and the reverse).
- expect: no raw address, pointer, or page reference is observable in
  any Ken value or the resource token (only the opaque handle, the window
  descriptor, and the scalar extent); the in-range access copies bytes,
  the out-of-range window returns a fail-visible `ResourceError` rather
  than an unchecked or clamped access, and a wrong-kind token returns
  `ResourceKindMismatch` naming the `Mapping` identity (with same-kind
  controls succeeding).
- fixture: in `crates/ken-cli/tests/abi_s6_mapping_surface_native.rs`,
  `window_direct_map_bytes_executes_natively_and_matches_the_interpreter`
  and `out_of_range_window_is_invalid_bounds_not_clamped` cover in-range
  copy, out-of-range refusal, and engine parity. The landed host tests
  `abi_s6_d1_mapping_is_opaque_lineage_bound_and_terminally_unmapped`
  and `abi_s6_d2_mapping_views_share_bounds_and_enforce_origin_rights_and_lifetime`
  in `crates/ken-host/src/effect_v1.rs` cover opaque resource tokens,
  Mapping-as-Buffer and Buffer-as-Mapping mismatch, and same-kind
  admission. They do not themselves prove that every source value
  excludes a pointer.
- control: an implementation that projects the raw region address into a Ken
  value, that forges a `MappingHandle` outside `withMapping`, or that reads an
  out-of-range window without a fail-visible error, reds; a triplicated or
  wrong-kind acceptance reds the `ResourceKindMismatch` net.
- why: opacity is what keeps raw pointers out of application Ken (`§4`, §6); an
  unchecked or address-exposing view defeats the substrate MMIO later relies on.
