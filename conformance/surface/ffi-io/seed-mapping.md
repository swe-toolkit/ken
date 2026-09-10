# Memory-mapping conformance — seed cases (ABI-S6)

Format: `../../README.md`. These pin the normative Mapping-surface contract of
`spec/30-surface/38-ffi-io.md §1.9` (opaque runtime-owned regions and bounded
byte views; authority `docs/program/10-linux-abi-completion.md §4`). The cases
net the *discriminating* form of each property — each reds a plausible
non-conforming implementation — not the prose. The Mapping surface splits
across two runtime gates. The **anonymous** window-direct surface — the
`withMapping` bracket, `MappingHandle`/`MappingWindow`, the window-direct
`mapBytes`/`mapWrite`, the `Mapping` resource kind, and the 4 KiB granule
accounting — is **ABI-S6 / D5a**, discharged by the **D1** increment: cases 2
and 3 (4 KiB granule; opacity + bounds) are **RED-UNTIL-BUILT /
BLOCKED-ON-ABI-S6-D5a** and lift to green on the D1 candidate. **File-backed
copy-on-write** — a `FileBacked` mapping acquired via `MappingAcquireFile` — is
the **ABI-S6 / D5b** successor, so case 1 (MAP_PRIVATE file COW) is **(gated:
D5b)** and stays RED across the D5b interval: what blocks it is file-backed
*acquisition*, not the window-direct `mapWrite` it uses, which D1 lands.

The staged gate stays honest because a live sibling enforces the posture across
the whole interval: while case 1's file-backed COW flip is dormant, the D5a
opacity/bounds case and every case's native==interpreter parity assertion keep
opacity, bounds-not-clamp, and native/interp agreement enforced from the moment
D1 lands — a fully-gated axis with no live enforcer would leave the posture
unguarded until D5b; this one does not. The seed is staged now as the control
that makes §1.9's negatives testable, for D5a/D1 to build against and D5b to
extend.

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
- given: a file with known original bytes; a program that acquires a
  `FileBacked` mapping over it via `withMapping … ReadWrite`, `mapWrite`s
  different bytes into an in-range `MappingWindow`, then — after the bracket
  settles — reads the same file through the ordinary `§1.3` file API.
- expect: **RED — (gated: D5b)** — the post-write file read returns the
  **original** bytes, not the mapped write. The in-mapping read view observes the
  write (process-local visibility) while the file is unchanged.
- fixture: **BLOCKED-ON-ABI-S6-D5b** — `FileBacked` acquisition
  (`MappingAcquireFile`) is the copy-on-write successor capability, deferred to
  D5b. D1 lands the window-direct `mapWrite` this case uses, but not file-backed
  acquisition; the gate is the `FileBacked` mapping, not the write.
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
- expect: **RED-UNTIL-BUILT** — the charged sizes are exactly `4096`, `4096`, and
  `8192` (`ceil(n / 4096) * 4096`); a zero-length mapping is not admitted; the
  native and interpreted paths charge the identical values.
- fixture: **BLOCKED-ON-ABI-S6-D5a**.
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
- expect: **RED-UNTIL-BUILT** — no raw address, pointer, or page reference is
  observable in any Ken value or the resource token (only the opaque handle, the
  window descriptor, and the scalar extent); the in-range access copies bytes,
  the out-of-range window returns a fail-visible `ResourceError` rather than an
  unchecked or clamped access, and a wrong-kind token returns
  `ResourceKindMismatch` naming the `Mapping` identity (with same-kind controls
  succeeding).
- fixture: **BLOCKED-ON-ABI-S6-D5a**.
- control: an implementation that projects the raw region address into a Ken
  value, that forges a `MappingHandle` outside `withMapping`, or that reads an
  out-of-range window without a fail-visible error, reds; a triplicated or
  wrong-kind acceptance reds the `ResourceKindMismatch` net.
- why: opacity is what keeps raw pointers out of application Ken (`§4`, §6); an
  unchecked or address-exposing view defeats the substrate MMIO later relies on.
