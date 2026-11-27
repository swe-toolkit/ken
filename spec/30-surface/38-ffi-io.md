# Bytes, I/O, and the FFI

> Status: **§1.1–§1.6 impl-ready (L6); §1.7 contract-pinned (PX8-T); §2–§4
> impl-ready (L7); FS descriptor retirement contract-pinned
> (RT-FS-DESCRIPTOR-INHERITANCE-DUPLICATE)** — elaborated to team-ready rigor
> and normative for the *trust and effect discipline*.
> **§1.1–§1.6 (`Bytes` + binary I/O, L6)** are
> built against the **landed** kernel/runtime (`14 §5` primitives, `41 §3a`
> encoding, `36 §1` effect rows). **§1.7** pins the contract that PX8-R/PX8-F
> must implement; it does not claim that substrate or surface is landed.
> **§2–§4 (the
> `foreign` FFI + the trust boundary, L7)** are elaborated below: the `foreign`
> declaration + C-ABI marshalling (§2), the trust-boundary discipline
> (foreign-as-listed-postulate, `pure`-as-claim, runtime contracts at the edge,
> mandatory effects — §3), and capability gating (§4). `Bytes`, binary I/O, and
> the FFI are how a verified core meets the outside world; the **boundary
> discipline** matters more than the syntax.
>
> **L6 grounding (perishable-frame reconcile):** `Bytes` has the landed durable
> canonical kind tag `0x05` (`41 §3a`); its in-process representation is private
> and is distinct from the serialization/Merkle hash (`§1.5`). Effect tracking
> rides L5's row system (`36 §1`); the one kernel
> admission L5's effect *denotation* needed — W-style (Π-bound) recursive
> inductives for the `ITree` `Vis` node (`14 §8.4`) — **has landed in K1.5**
> (`check_no_pi_bound_recursive` retired, `crates/ken-kernel/src/inductive.rs`),
> so **L6 carries no kernel-staging block** and adds **no new kernel rule**.
>
> **L7 grounding (perishable-frame reconcile):** a `foreign` rides the
> **landed** postulate machinery — `declare_postulate` → opaque constant (`11
> §4`: "how axioms, **FFI signatures**, and abstract interfaces are
> represented") recorded in `trusted_base()` (`18 §4.2`/`§5`) — and the
> **landed** B1 export, which projects every `trusted_base_delta` postulate
> (`25 §3`) into its `P` (assumptions) field under the boundary-label case
> (`71 §2.1`). Marshalling reuses L6's `Bytes`↔`(ptr,len)` (`§1.1`, `41 §1`);
> effects ride the `36 §1.4`
> escape check; capability gating couples Sec2 (`62`). **L7 adds no new kernel
> rule** — `foreign` is an existing postulate, not a new former. Pin against the
> landed code, not this line.

## 1. `Bytes` and binary I/O

`Bytes` is an **immutable, finite byte sequence** — the substrate for binary
protocols, hashing, serialization, and (in L7) FFI buffers. At the world
boundary, `read_bytes` reads bytes from files and `print_line` writes text to
the console. Every such operation is **effect-tracked** (`36`): an I/O
operation that does not carry its effect row is a **type error**. Text is
**never** an implicit reinterpretation of bytes — a
`String` is obtained from `Bytes` only through an **explicit, named**
`bytes_encode`/`bytes_decode` step (no hidden charset). Over that substrate sits
a **lawful serialization round-trip** (`§1.5`), the verified-component target
for G6.

### 1.1 The `Bytes` primitive

`Bytes` is a **primitive type** in the `14 §5` sense — an **opaque type
constant** with **no kernel-level constructors or eliminator**; it adds **no new
kernel rule** (guardrail). Its inhabitants are **literals** (introduced by the
elaborator as opaque primitive values) and the results of the primitive
operations (`§1.2`). This is the same discipline as the L1 numeric primitives
(`35`): the kernel gains an opaque constant plus a small, **audited** set of
registered operation symbols, listed in the trusted-base ledger (`18 §5`).

- **Immutable and finite.** A `Bytes` value never changes after construction;
  there is **no mutating operation** in the surface (the AC1 structural
  property). "Updating" bytes (for example, `slice` or `concat`) returns a value
  without changing either input. Allocation, copying, and sharing are private
  runtime choices (`41 §2`).
- **Durable encoding and private runtime representation (landed `41`).** The
  canonical encoding (`41 §3a`) is the **kind tag `0x05`** followed by the raw
  byte sequence (length-determined). Two `Bytes` values with identical content
  encode identically and compare equal. Their in-process addressing, storage,
  and equality acceleration are private; they are distinct from the
  cryptographic/Merkle hash used for serialization (`§1.5`).
- **The `(ptr, len)` boundary.** A `Bytes` value's machine face is a
  `(pointer, length)` pair (`38 §2` marshalling, `41 §1`). L6 keeps this
  boundary clean and explicit so **L7's** `foreign` marshalling (`Bytes` ↔ a C
  buffer) rides it without rework — but L6 builds **no** `foreign` mechanism.

**Literals (`31 §3`).** Two surface forms denote `Bytes`:

| Form | Example | Meaning |
|---|---|---|
| **byte string** | `b"GET / HTTP/1.1\r\n"` | a byte sequence from a string-like body with escapes (each `\xNN`/ASCII char ⇒ one byte) |
| **hex** | `0x[deadbeef]` | the bytes spelled as hex nibble pairs (here `de`, `ad`, `be`, `ef`) |

The **bracketed** `0x[…]` is the `Bytes` form; the **un-bracketed** `0xFF` is an
**`Int`** literal (`31 §3`, `35`) — the two are different tokens with different
types and must not be conflated. A `b"…"` literal elaborates **directly** to the
`Bytes` primitive (AC1), not via `String` (no decode round-trip at the literal).

The `0x[…]` body is a **contiguous** run of an even number of ASCII hexadecimal
digits, lexed as consecutive nibble pairs; it admits **no internal whitespace or
separator**. `0x[de ad]` is a lexical error, not a spelling of `0x[dead]`. The
groupings that appear as reading aids — the nibble-pair gloss above, and any
spaced `0x[…]` in a doc comment — name the byte boundaries; they are never legal
source. (Readability grouping in Ken's literals is the underscore
digit separator, `31 §3`; if grouping in `0x[…]` is ever wanted it follows that
convention, not whitespace, and is a separate additive decision.)

### 1.2 Core `Bytes` operations (primitive / prelude)

The core operations are primitive operations with registered
`PrimReduction::Op` symbols (`14 §5`). The interpreter evaluates them over
`Bytes` values at runtime, so `bytes_length 0x[deadbeef]` produces `4` as a
value. The landed kernel does not execute these `Bytes` Ops during conversion:
`bytes_length 0x[deadbeef] ≡ 4 : Int` is **not** definitional and the equation
does not close by `Refl`, even on literal operands. The separately specified
registered `leq_int` literal rule and checked-literal `string_to_list_char`
view compute in kernel WHNF (`../10-kernel/16 §2.2`, `../10-kernel/17 §1`);
they do not promote any `Bytes` operation. Kernel conversion for these
`Bytes` Ops remains K3-deferred; these declarations add no reduction rule.

| Op | Type | Notes |
|---|---|---|
| `length` | `Bytes → Int` | byte count; total, non-negative |
| `bytes_at` | `Bytes → Int → Option UInt8` | indexed byte; **partial** (invalid index ⇒ `None`) |
| `bytes_slice` | `Bytes → Int(start) → Int(len) → Option Bytes` | `None` on an invalid span |
| `concat` | `Bytes → Bytes → Bytes` | total; `++` on `Bytes` |
| `empty` | `Bytes` | the zero-length value; `length empty ≡ 0` |

- **Partiality follows the `35 §3` / `43 §2` discipline.** Indexing and slicing
  are the **partial** operations (bounds), handled the same way `35` handles
  `div`/`mod` by zero. `bytes_at b i` returns `None` when `i` is negative or
  outside `b`. `bytes_slice b start len` returns `None` when `start` or `len`
  is negative, or when that span extends past `b`; its third argument is a
  **length**, not an end offset. Neither operation returns a neutral or a
  fabricated byte/empty slice for invalid bounds. For verified code, an
  **obligation-generating** total form over a refinement
  (`at_pf : (b : Bytes) → (i : Int) →
  { And (IsTrue (leq_int 0 i))
        (IsTrue (leq_int (i + 1) (length b))) } → UInt8`, `34 §5`)
  is proven in-range
  ⇒ total and safe; unproven ⇒ a marked partial point that degrades to a runtime
  check (`unknown`/panic), **never** a silent out-of-bounds read. The two faces
  are the §`35 §3` "checked is the runtime face of an undischarged obligation,"
  not a separate mode.
- **A byte is `UInt8`** (`35`); `length`/indices are `Int` (the default integer,
  arbitrary-precision, `35 §2`).
- **Non-definitional laws are propositions** (`14 §5`), not assumed:
  `Equal Int (length (concat a b)) (length a + length b)`,
  `concat`-associativity, `Equal Bytes (concat a empty) a`, etc. — proved in
  the prelude (`50-stdlib/`, `20-verification/`), not baked into the kernel.
- **Exact surface spellings.** The primitive names `bytes_at` and
  `bytes_slice`, their signatures, their total failure behavior, and their
  registered runtime semantics over values are fixed here. Spellings of the
  remaining prelude conveniences (`++` vs `concat`, for example) remain a
  **`31`/prelude naming** detail (oracle-tagged for the build team).

### 1.3 Effect-tracked I/O

Every I/O operation carries its **exact effect row** (`visits`, `36 §1`). The
two landed, ordinary surface witnesses used by this clause are:

| Landed operation | Mandatory row |
|---|---|
| `read_bytes` | `visits [FS]` |
| `print_line` | `visits [Console]` |

Their complete result and capability types are defined by the landed FS and
Console floors; this clause fixes the operation-to-row association. The later
PX11 stream- and datagram-socket producers (§1.13, §1.13.1) carry
`visits [Net]` and supply a real `Net` operation-binding witness. They do
not retroactively alter the L6 `FS` and `Console` witnesses.

- **The no-untracked-I/O guard is the `36 §1.4` escape check — L6 introduces no
  new gate.** Each I/O operation is a `perform_E` site (`36 §1.2`): it is the
  **one** place its label (`FS`/`Console`) enters the inferred row. A call to an
  I/O operation from a context whose declared row does **not** contain that label is
  an **EFFECT-ESCAPE static error** (`ρ_inf ⊄ ρ_decl`). Pure-by-default ⇒ a
  `fn` with **no** `visits` has `ρ_decl = ∅`, so **any** I/O call escapes ⇒
  **untracked I/O is a compile error** (AC2/AC3). The accepting case carries the
  effect in its row; the rejecting case drops it — a **verdict flip**, exercised
  with the **≥2 distinct effects** `FS` and `Console` (`36 §1.4`).
- **Every I/O act is a `Vis` node (`36 §3.1` contract).** The I/O operations are
  the tree's `perform` sites: a function's effects are recoverable from its
  **type** (the latent row `A →[ρ] B`), never hidden between nodes. This is the
  interface Sec/B (`60-`/`70-`) read labels and capabilities off of — L6 must
  not perforate it (no effectful I/O that bypasses a `Vis` node).
- **I/O operations are not kernel primitives.** Unlike the **pure** `Bytes` ops
  (`§1.2`, which carry registered interpreter dispatch), an I/O operation is an
  **effect-performing surface operation** whose
  denotation is an `ITree` `Vis` node (`36 §2`), **not** a kernel reduction — it
  computes no literal in the kernel and adds **nothing** to the TCB. The kernel
  admission this denotation needs (W-style recursive inductives) **landed in
  K1.5** (banner + `14 §8.4`).
- **Capabilities (`36 §3`) are the authority face of the same row.** An I/O
  operation may additionally take a capability token (`using c : Cap FS`) to
  express least authority; L6 specifies the **effect-row** obligation (the
  type-error-if-untracked guarantee) and leaves capability *threading* as the
  `36 §3` mechanism it already is — no new machinery here.

#### 1.3.1 Typed FS capability API

Programs use a typed capability API over the unchanged, authority-polymorphic
FS producers. The coarse-v1 signatures are:

```ken
readFile : (a : Auth) -> Cap a -> Bytes -> FS a (Result FileError Bytes)

writeFile : Cap AFull -> Bytes -> CreatePolicy -> Bytes
  -> FS AFull (Result FileError Unit)
```

The checked public `withResource` acquisition selects its mode with
the existing pre-source `ResourceOpenMode` family:

```ken
data ResourceOpenMode = ResourceRead | ResourceMetadata | ResourceWriteCreate CreatePolicy
```

The family is scoped: callers use `ResourceOpenMode.ResourceRead` for
reading, `ResourceOpenMode.ResourceMetadata` for metadata and
`ResourceOpenMode.ResourceWriteCreate policy` for a write with the
existing `CreatePolicy` value. None of these leaves is bare-reserved;
their exact checked IDs, not same-shaped local modes, drive the FS
acquisition. This does not change the `readFile` or `writeFile` types.

`readFile` wraps `read_bytes`; `writeFile` wraps `write_file`. The wrappers do
not re-type or replace those raw producers. A read-and-write program calls
`readFile fullCap path` directly: the read wrapper accepts the authority carried
by any `Cap a`, including `ANone`, `APartial`, and `AFull`. The driver checks
read sufficiency at operation time; insufficient authority returns the named
`CapabilityDenied` result before host I/O. A static lower bound on `a` would
require bounded authority quantification, which v1 does not provide; that is a
v2 option rather than an implied v1 guarantee.

The write boundary remains static: a program holding only `Cap APartial` cannot
apply `writeFile`, so the attempt is ill-typed before the driver runs. The
driver's `CapabilityDenied` result remains defense in depth for writes and is
the primary authority floor for reads. User code has no public capability
constructor, capability-producing wrapper, Ken-callable attenuation function,
or Ken-callable revocation function. In particular, `readFile` and `writeFile`
are bound while the raw management names `attenuate` and `revoke` both resolve
as `UnboundName`, and no `Cap` constructor or producer is present in the Ken
environment. The wrappers are ordinary checked Ken definitions and add neither
a kernel rule nor a trusted primitive.

`create_directory` creates exactly one directory, the final component of
`path`, beneath the capability's root. It does not create missing parents,
which refuse with `NotFound`, and an existing entry at `path` refuses with
`AlreadyExists`. A caller that needs a parent chain calls it once per prefix.

Raw attenuation and revocation are trusted runner/host management actions over
non-Ken-visible grant identities (`62 §4`), not source operations. Attenuation
records child lineage; revocation closes the selected identity and its
descendants. A program observes neither action directly. It observes revocation
only when a later existing capability-consuming operation returns a
revocation result in its type-local error family:

- a path/capability operation returns
  `MkFileError <operation> <path> Revoked`, where `Revoked` is an `IOError`
  cause beside and distinct from `CapabilityDenied`;
- an ordinary FS or Net resource-token operation currently returns
  `ResourceHostIO IOError.Revoked` when its originating grant is revoked
  before admission. The successor contract, after error unification,
  projects this same denial as the nullary `ResourceError.Revoked`, distinct
  from `Closed`, `MalformedResource`, `RightNotHeld`,
  `ResourceKindMismatch`, and `ResourceHostIO _`. This successor is not a
  constructor of the currently complete twelve-case `ResourceError` family.

**Process host operations remain distinct from the successor projection.**
Later plan, child and pipe operations, including `signal_child`, return
`ResourceHostIO IOError.Revoked` when the originating Process grant is
revoked before admission (§1.11.2–§1.11.4). This retains the same
`IOError.Revoked` cause through the Process classification of §1.11.3;
it does not introduce a new semantic denial identity or permit the
signal to reach the child. The two enclosing error families remain
distinct; no shared sum is introduced. Neither current nor successor
mapping substitutes `CapabilityDenied`,
`ResourceHostIO IOError.CapabilityDenied`, `Closed`, malformed
capability or resource, stale generation, `RightNotHeld`,
`ResourceKindMismatch`, or another host-I/O cause for revocation.
For delivered resource-token and Process-host cases, `ResourceHostIO`
**with the exact `IOError.Revoked` cause** is the intentional carrier,
not a collapse of that cause.

### 1.4 Text is explicit `bytes_encode` / `bytes_decode` — no hidden charset

A `String` is obtained from `Bytes` **only** through a named, visible boundary;
there is **no** implicit charset reinterpretation anywhere in the surface
(AC4).

```
bytes_encode : String → Bytes
bytes_decode : Bytes → Result Utf8Error String
```

- **`bytes_encode` is total and UTF-8 by contract.** It serializes a `String`
  (which is **NFC-normalized UTF-8** by construction, `41 §3a`) to its UTF-8
  bytes. The **charset is named in the operation**, not hidden: a non-UTF-8
  codec (if ever added) is a **different named function** (`encode_latin1`,
  …), never an implicit reinterpretation. There is no `Bytes`-to-`String`
  coercion, no "default charset," and no path that yields text from bytes
  without `bytes_decode`.
- **`bytes_decode` is total with explicit failure.** For valid UTF-8 it returns
  `Ok s`; invalid UTF-8 returns `Err e` for an `e : Utf8Error`. It never returns
  a neutral term or silently substitutes text. AC4's negative face: the
  **only** way to a `String` is this named, fail-visible step; an implicit or
  hidden-charset path is **rejected** (does not exist).

For the primitive implementation, the late
`Data.Binary.BytesPrimitiveContracts` module names the following four and only
four new trusted postulates (the first is the concatenation view in `37 §2.6`):

```
AllAscii s :=
  AllAsciiCodes (map Char Int charToInt (string_to_list_char s))
AsciiBytes bs :=
  AllAsciiCodes (map UInt8 Int uint8_to_int (bytes_to_list bs))
IsUtf8 bs := match bytes_decode bs {
  Err _ ↦ Bottom; Ok text ↦ Equal Bytes (bytes_encode text) bs
}

bytes_concat_list_view : (a b : Bytes) →
  Equal (List UInt8) (bytes_to_list (bytes_concat a b))
    (list_append UInt8 (bytes_to_list a) (bytes_to_list b))
bytes_encode_ascii_octets : (s : String) → AllAscii s →
  Equal (List Int)
    (map UInt8 Int uint8_to_int (bytes_to_list (bytes_encode s)))
    (map Char Int charToInt (string_to_list_char s))
bytes_decode_encode : (s : String) →
  Equal (Result Utf8Error String)
    (bytes_decode (bytes_encode s)) (Ok Utf8Error String s)
ascii_bytes_utf8 : (bs : Bytes) → AsciiBytes bs → IsUtf8 bs
```

`AllAsciiCodes` is a finite checked family: 128 ASCII tags cover precisely
`0..127`, a checked equality indexes each `AsciiCode`, and list witnesses
recur structurally. The
`AllAscii` and `AsciiBytes` aliases and the `IsUtf8` predicate are transparent
checked definitions below Parsing, not additional trusted facts. In
particular the latter is definitionally the existing Parsing predicate; no
`Source` contract change or import into Parsing is needed. K3's checked
literal-character view constructs `AllAscii` for a concrete ASCII literal.
The F2 equation relates the complete encoding of any witnessed ASCII string
to its character codes; it is not a six-token table or a per-character law.
F3 supplies an actual proof term for the decode-after-encode equality, not
merely a proposition with no inhabitant. F4' states decode/re-encode identity
for ASCII byte lists only; it does not assert the false reverse round trip for
arbitrary UTF-8 bytes. For a concatenated printer output, F1 and F2 give its
ASCII byte list, F3 covers its encoded leaves, and F4' gives the `IsUtf8`
witness once its structural `AsciiBytes` witness is checked. These are
implementation contracts of opaque primitives; none reduces in the kernel.
They load only after the checked `Derived.list_append` provider, rather than
adding a premature `bytes.rs` registration. The added `trusted_base()` ledger
is exactly `bytes_concat_list_view`, `bytes_encode_ascii_octets`,
`bytes_decode_encode`, and `ascii_bytes_utf8`.

### 1.5 The serialization round-trip law

Over the `bytes_encode`/`bytes_decode` boundary, the intended
**one-directional** round-trip law has byte input explicitly conditional on
having been produced by `bytes_encode`:

```
∀ (s : String). Equal (Result Utf8Error String)
  (bytes_decode (bytes_encode s)) (Ok Utf8Error String s)
```

The pre-existing `BytesRoundTripLaw : Ω₀` is an opaque oracle-tagged
proposition, not definitionally the displayed equality and not itself an
inhabitant of it. F3 (`§1.4`) provides the explicit quantified equality
witness. It is a trusted property of the primitive implementation, not an
unassisted Ken-source proof of the decoder algorithm or a proof of the opaque
`BytesRoundTripLaw` marker. The marker's existing L8 proof target is separate;
clients requiring this equality consume F3 directly.

- **Why it holds (and the direction matters).** `bytes_encode s` is the UTF-8
  bytes of `s`; `bytes_decode` parses valid UTF-8 (which `bytes_encode` always
  produces) and **re-constructs** a `String`, NFC-normalizing at construction
  (`41 §3a`). Because `s` is **already** NFC and NFC is **idempotent**, the
  reconstructed string equals `s` — so the law holds. F3 gives a checked
  proof inhabitant for this equation relative to the trusted primitive
  contract. A successful discharge of the separate opaque
  `BytesRoundTripLaw` marker using a postulate witness tests only the
  obligation mechanism, not this semantic proof.
- **The reverse is NOT a law — pin the silence so it is not over-claimed.**
  There is no unconditional inverse for arbitrary bytes: from
  `Equal (Result Utf8Error String) (bytes_decode b) (Ok Utf8Error String s)`,
  it does **not** follow in general that `Equal Bytes (bytes_encode s) b`.
  Invalid UTF-8 has no `String`, and even valid **non-NFC** bytes normalize on
  `String` construction (`41 §3a`), so
  `bytes_encode` after a successful `bytes_decode` is not the identity on every
  `Bytes`. Conformance must assert only the conditional
  `String → Bytes → Result Utf8Error String` direction above; a general
  `Bytes → String → Bytes` inverse is a **wrong** case (it would reject
  conforming implementations). This is a verdict/law-boundary silence resolved
  **at the source** so the conformance author does not fill it the other way.
- **L6 delivers the interface + the law; the generic derivation is L8.** The
  derivable `encode`/`decode` for **arbitrary** types (the `Serialize`-class
  machinery) is the stdlib follow-on (`L8`); L6 fixes the `Bytes` substrate, the
  `String` boundary, and the round-trip **law/interface** — not the generic
  derivation. The serialization/integrity hash referenced for verified transport
  (a cryptographic/Merkle hash, BLAKE3 recommendation, `41 §3b`) is **separate**
  from in-process addressing (`§1.1`) and is selected downstream, not by L6.

### 1.6 What L6 delivers + acceptance

Team Foundation delivers, in the surface + runtime + prelude: the **`Bytes`
primitive** (`§1.1`, `14 §5` opaque constant + `41 §3a` `0x05` encoding,
immutable, `b"…"`/`0x[…]` literals); the **core ops** (`§1.2`, registered
reductions, `35 §3` partiality); the **effect-tracked I/O surface** (`§1.3`,
each op `visits` its exact row, untracked = type error via the `36 §1.4` escape
check); the explicit **`bytes_encode`/`bytes_decode`** boundary (`§1.4`, no
hidden charset); and the **round-trip law** (`§1.5`, one-directional; the
later F3 primitive contract supplies its equality witness). **No new kernel
rule** (`§1.1`); **no `foreign`** (that is L7, `§2`–`§3`).

**Acceptance (AC1–AC5).**

- **AC1 — `Bytes` primitive + immutable.** A `b"…"`/`0x[…]` literal elaborates
  to the `Bytes` primitive (structural assertion on the elaborated value/type,
  not "compiles"); **no mutating op exists**.
- **AC2 — `[FS]` tracked.** `read_bytes` (`visits [FS]`) called from a context
  lacking `FS` is a **type error** (reject); the properly-rowed call **accepts**
  — a **verdict flip**.
- **AC3 — `[Console]` tracked.** `print_line` (`visits [Console]`) — the
  **same** flip on a **distinct**, real effect (the ≥2-effect discrimination of
  `36 §1.4`). The later PX11 `Net` witness (§1.13) is distinct from this
  historical L6 `Console` acceptance pair.
- **AC4 — no hidden charset.** Producing text from `Bytes` **requires** the
  named `bytes_decode`; an implicit/hidden-charset path is **rejected** (or
  absent), and invalid input produces `Err`.
- **AC5 — round-trip law.**
  `Equal (Result Utf8Error String)
  (bytes_decode (bytes_encode s)) (Ok Utf8Error String s)` has the quantified
  F3 proof inhabitant (`§1.4`), not merely a passing sample. The pre-existing opaque
  marker's discharge test proves only that an obligation can accept a
  certificate; it does not prove the decoder from Ken source. The reverse is
  **not** asserted (`§1.5`).

**Conformance:** `../../conformance/surface/bytes-io/` — AC1–AC5 with per-case
**verdict/structural flip** and the **cross-case sweep** (the effect-tracking
cases `FS`/`Console` agree as one metatheory class). The **QA gate**: the
effect-tracking cases route a **real** I/O signature through the **actual** `36
§1.4` escape check (a real untracked call → real reject), not a synthetic flag.
(The L7 `foreign`/trust-boundary cases live separately under
`../../conformance/surface/ffi-io/`, `§5`.)

### 1.7 Positioned buffer I/O and total `writeAll` (PX8-T)

PX8 adds a bounded mutable-buffer floor without making ordinary Ken values
mutable or affine. `Buffer` is a second runtime resource kind beside
`FsHandle`; its opaque, copyable `BufferHandle` is acquired only through the
public `withBuffer` bracket. Capacity is fixed, strictly positive, and
non-growing for the handle's lifetime. The runtime invalidates escaped copies
when the bracket settles, exactly as for file resources. Allocation consumes
no ambient capability right. It is admitted against one deterministic policy:

```text
BufferLimitsV1 {
  per_buffer_max_capacity: Int,
  invocation_max_live_capacity: Int,
}
```

Both maxima are positive. The first bounds every allocation; the second bounds
the sum of live buffer capacities in one invocation. The complete policy is
bound into the checked/native plan. Reading either limit from an environment
variable, silently growing a buffer, or placing buffers in the capability table
is non-conforming.

The closed Ken resource-kind inventory becomes
`ResourceKind.FsHandle | ResourceKind.Buffer` (`§1.9` adds
`ResourceKind.Mapping`). Supplying a
live token of the wrong kind produces the distinct fail-visible identity

```text
ResourceKindMismatch {
  expected: ResourceKindV1,
  actual: ResourceKindV1,
}
```

under its own surface constructor and canonical wire discriminator. It is not
`MalformedResource`: the token is valid and live, but belongs to the other
kind. A buffer passed to an Fs-handle-only operation reports
`{ expected: FsHandle, actual: Buffer }`; those fields are frozen
`ResourceKindV1` wire tags, not bare Ken constructor spellings. Ken source
uses `ResourceKind.FsHandle` and `ResourceKind.Buffer`. A file handle passed
to a buffer-only operation reports the reversed payload. Valid same-kind
controls succeed. These two rejecting directions and the two accepting
controls are one non-degenerate conformance unit.

#### 1.7.1 Buffer views and positioned single transfers

There is no hidden file cursor and no mutable buffer cursor. The runtime tracks
one initialized live window per buffer, while Ken observes only:

- the opaque `BufferHandle`;
- an immutable `BufferWindow` request descriptor;
- a constructor-private immutable `BufferSpan` for the exact current live
  subrange; and
- scalar projections, including span length.

The checked handle binds capacity to the exact acquisition, not to a caller's
request:

```ken
data BufferHandle = PrivateBufferHandle (Resource ResourceKind.Buffer) Int
```

`withBuffer capacity` is the sole producer of `BufferHandle`. It constructs the
handle only after successfully allocating a buffer of that capacity, then
passes the handle to the bracket body. `PrivateBufferHandle` and both field
projections are absent from the public name map. Checked user code can
therefore neither forge a resource/capacity pairing, replace the stored
capacity, project the raw `Resource ResourceKind.Buffer`, nor construct a
handle outside `withBuffer`. The handle may be copied as an ordinary checked
value, but every copy denotes that same acquisition and becomes invalid when
the bracket settles.

The public prelude API has these argument and result shapes:

```ken
proc withBuffer (a : Auth) (e : Type) (r : Type) (capacity : Int)
  (body : BufferHandle -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [FS]

proc readAt (a : Auth) (file : Resource ResourceKind.FsHandle) (fileOffset : Int)
  (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError ReadProgress) visits [FS]

proc writeAt (a : Auth) (file : Resource ResourceKind.FsHandle) (fileOffset : Int)
  (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError WriteProgress) visits [FS]

proc setLength (a : Auth) (file : Resource ResourceKind.FsHandle) (length : Int)
  : HostIO a (Result ResourceError Unit) visits [FS]

proc sync (a : Auth) (file : Resource ResourceKind.FsHandle)
  : HostIO a (Result ResourceError Unit) visits [FS]

proc spanBytes (a : Auth) (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Bytes) visits [FS]

proc freeze (a : Auth) (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Bytes) visits [FS]

proc writeAll (a : Auth) (file : Resource ResourceKind.FsHandle) (fileOffset : Int)
  (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Unit) visits [FS]
```

`BufferWindow = MkBufferWindow Int Int` remains the public raw request
descriptor: its fields are caller-chosen and carry no authority. The capacity
used to admit that request is always the constructor-private capacity stored in
the supplied `BufferHandle`, never a caller-supplied capacity or a value derived
from `BufferWindow`.

No pointer, borrowed slice, file descriptor, mutable reference, or backing
region crosses the boundary. `spanBytes` (also called `freeze`) validates a
current span and may copy that bounded span to immutable `Bytes`; it cannot
expose the backing region. A `BufferSpan` carries a constructor-private
structural `Nat` attempt budget equal to its byte length. User code can neither
forge the budget nor choose a different one.

Every `BufferSpan` has an exact buffer-acquisition binding. A successful
`readAt` binds the span it mints to the buffer acquisition used by that call. An
acquisition is one resource lifetime: closing a buffer and later acquiring
another starts a different acquisition, regardless of internal storage reuse.
User code can neither forge nor project the binding. A remainder span derived
by checked library code preserves the original binding.

`spanBytes`/`freeze` and `writeAt` accept a span only with the exact buffer
acquisition to which it is bound. A different acquisition returns
`InvalidBounds`, even when capacity, start, length, and live-window shape all
match. Existing host-width admission retains precedence, so a foreign-span
`writeAt` whose file offset is rejected there still returns `InvalidOffset`.
After host-width admission, span validity is decided before copying or exposing
bytes and before any backend write. Acquisition mismatch and numeric
live-window invalidity deliberately share `InvalidBounds`: both mean that the
span is not valid for the supplied buffer, and no relative ordering between
those two same-identity failures is public. If the acquisition matches, the
existing resource errors retain their identities; in particular, using the
span with that exact acquisition after it closes returns `Closed`.

At the public prelude boundary the two explicitly positioned transfers are:

```text
readAt file fileOffset bufferHandle window
writeAt file fileOffset bufferHandle span
```

For a positive admitted request, each wrapper projects the exact
`Resource ResourceKind.Buffer` from `bufferHandle` and invokes exactly one
constructor-private host operation. `spanBytes`/`freeze`, `writeAll`, and
bracket settlement use that same private projection; settlement releases the
exact acquisition stored in the handle. The host resource token, private host
operations, and wire ABI are unchanged. The host repeats its existing
resource-kind, liveness, rights, and range validation as defense in depth; the
checked handle does not replace those checks.

`fileOffset` is nonnegative, and target offset-plus-length arithmetic is checked
for overflow before host I/O. The host-width limit for `readAt` and `writeAt`
`fileOffset`, for `fileOffset` plus the effective transfer length, and for a
`setLength` length is the host's largest representable file offset: `2^63 − 1`
on the Linux reference host, whose `off_t` is signed 64-bit. A value above
this limit returns `InvalidOffset` at its existing check: before dispatch for
the file offset or `setLength` length, and before backend I/O for the sum.
A value at or below the limit passes the width check but does not guarantee
success; an I/O refusal by the backend (for example `EFBIG`) is reported as
`ResourceHostIO`. Neither operation mutates a file-description cursor.
Sequential reads, writes, copies, and seek-like state are derived code
that explicitly threads `fileOffset + transferred`. There is no seek
primitive or relative file I/O, and PX8 adds no vectored I/O.

`setLength` and `sync` are explicit imports from `Capability.System.IO`.
Both operate on a live `ResourceWriteCreate` handle (any creation policy),
require `WRITE`, and report host I/O failures as `ResourceHostIO` in
`ResourceError`. Other handle modes are refused with `RightNotHeld` before
backend access. `setLength` rejects a negative or host-width-unrepresentable
length with `InvalidOffset` before dispatch. It sets the extent to `length`,
preserving the retained prefix and zero-filling any extension.

sync asks the host to flush the file's data and metadata (fsync(2)) and
returns the host's outcome. Durability after machine failure is the kernel
and filesystem's contract and is not tested by Ken.

Every FsHandle descriptor is created with close-on-exec, and no Ken operation
clears that flag or duplicates a descriptor. Inheritance into a child is decided
only by the spawn plan (§1.11), which starts the child with exactly {0, 1, 2};
the plan's closefrom step and the per-descriptor flag are independent, as for
sockets (§1.13). The host ABI has no descriptor-inheritance query, inheritance
setter or duplication operation.

Derived `readAt` admits a raw `BufferWindow` in this exact order:

1. Validate that `fileOffset` is nonnegative and representable at the host
   width. Failure returns `InvalidOffset` before inspecting the window, so this
   identity has precedence over a simultaneous window fault or closed-endpoint
   request.
2. Project the acquisition capacity from `BufferHandle`. Reject a negative
   `start`, a negative `length`, or `start > capacity` as `InvalidBounds`.
3. Compute `effective = min(length, capacity - start)`. This subtraction is
   nonnegative because step 2 has established `start ≤ capacity`.
4. If and only if `effective = 0`, return `ReadProgress.ReadEof` from
   checked code without emitting a private read operation or visiting
   the host. Thus
   `start = capacity` is an admitted closed endpoint: at capacity `8`, the raw
   request `(start = 8, length = 4)` returns `ReadProgress.ReadEof`
   without a host visit, while `start = 9` returns `InvalidBounds`.
5. Otherwise invoke the positive-length private read operation with the exact
   resource stored in the handle and the range `(start, effective)`.
   Offset-plus-effective overflow returns `InvalidOffset` before host I/O.

Consequently a start in the closed range `0 ≤ start ≤ capacity` with a length
beyond the available tail is capped to that tail, producing ordinary short
progress when the effective range is positive. A zero raw length and an
in-range closed endpoint are the two ways to derive a zero-effective
`ReadProgress.ReadEof`; neither may invoke a positive-length
primitive. A caller-supplied capacity is never consulted.

A mutating read invalidates the prior live window before the host operation.
Positive success installs exactly the bytes actually read. EOF or error leaves
the live window empty, so a short read cannot make stale bytes current.

#### 1.7.2 The exact progress partition

Progress is represented by operation-specific closed sums:

```text
data ReadProgress
  = ReadSome BufferSpan TransferCount
  | ReadEof

data WriteProgress
  = Wrote TransferCount
```

These are two complete, scoped checked families: source matches
`ReadProgress.ReadSome`, `ReadProgress.ReadEof` and
`WriteProgress.Wrote` at their exact constructor identities. Bare
`ReadSome`, `ReadEof` and `Wrote` are not reserved by these families;
constructor declarations in the family above retain their original
checked IDs.

`TransferCount` is constructor-private, strictly positive, and bounded by the
effective request: the post-validation length after any in-range tail cap. For
`readAt`, this is the post-cap window length; for `writeAt`, it is the remaining
input-span length. For a count `n`, `0 < n ≤ effective request`, and its
remaining request budget is `effective request - n`; the raw caller-requested
length is not a progress bound after capping. Thus a tail-capped `readAt` with
raw request 8, effective request 4, and count and span length 4 has remaining
request budget 0 and total request budget `count + remaining = 4`. It has an
`Int` projection. `ReadProgress.ReadSome`'s `BufferSpan` and count are
minted together, and the span length equals the count by construction.
`Complete` and `Partial` are derived comparisons between the count and effective
request; they are not runtime constructors or statuses. The checked view of a
count carries its positivity and upper-bound witnesses; user code cannot detach
those witnesses from the value or construct a count that violates them. The
downstream proof therefore eliminates the checked result structure rather than
postulating a fact about an arbitrary `Int`.

For a positive effective request, the partition is exact:

| Host observation | Result |
|---|---|
| read returns zero | `ReadProgress.ReadEof` |
| read returns `0 < n ≤ effective request` | `ReadProgress.ReadSome span n`, including a short read |
| write returns `0 < n ≤ effective request` | `WriteProgress.Wrote n`, including a short write |
| write returns zero | error `NoProgress`, never `WriteProgress.Wrote` |

`NoProgress` is the surface identity for the write-zero condition (also called
`WriteZero` at a host boundary); it is not a successful progress value.

The following are errors, never progress: `Revoked`, `Closed`,
`MalformedResource`, `ResourceKindMismatch`, `RightNotHeld`, invalid
offset/window/bounds, buffer-limit or allocation failure,
unsupported/nonblocking posture, and host I/O failure. `Revoked` means an
otherwise well-formed, live, sufficiently-righted operation lost at `62 §4.2`'s
admission boundary; it is not an alias for a neighbouring error. `Interrupted`
is a named error unless a separately specified and tested backend retry policy
consumes it internally. `WouldBlock` belongs to PX12 and is not a PX8 progress
status. PX9 may refine error payloads but must not change this progress partition
or collapse the `Revoked` identity.

The private positive-transfer contracts re-exposed by the checked wrappers
include these propositions (`remaining` and `effective_request` denote the
`Int` budgets fixed above):

- **write positivity/bounds:** successful `writeAt` on positive remaining bytes
  yields `WriteProgress.Wrote n` with `And (IsTrue (leq_int 1 n))
  (IsTrue (leq_int n remaining))`;
- **read positivity/bounds:** successful `readAt` on a positive effective
  request yields either `ReadProgress.ReadEof` or
  `ReadProgress.ReadSome span n` with
  `And (IsTrue (leq_int 1 n)) (IsTrue (leq_int n effective_request))`
  and `Equal Int (length span) n`;
- **position and bounds:** every transfer uses the explicit nonnegative file
  offset and an overflow-checked effective range; and
- **tail capping:** a request that starts in range but extends beyond the buffer
  tail uses the capped effective range for every progress bound and remaining
  request budget, while an out-of-range start fails.

The positivity propositions are reasonable-from contracts of the opaque
runtime operation. They are fixed, audited primitive guarantees rather than a
fresh `Axiom` at each caller. In particular, the `writeAll` proof below consumes
write positivity; it does not assume progress from a runtime hope.

#### 1.7.3 Derived `writeAll` and its theorem

`writeAll` is transparent checked Ken code, not a host primitive. It takes a
positioned file handle, `BufferHandle`, and `BufferSpan`, derives structural
`Nat` fuel from the span's constructor-private attempt budget, and calls
`writeAt` until the span is empty or the first transfer error occurs. The fuel
is never caller-supplied.

Let `B` be the initial span's bytes, `L` its byte length, and
`n₁, …, nₖ` the counts returned by successful primitive calls in source order.
Let `N = n₁ + … + nₖ`. The required theorem is the conjunction of:

1. **termination:** `writeAll` performs at most `L` primitive calls and always
   returns;
2. **exact-prefix invariant:** after every successful-call prefix,
   `And (IsTrue (leq_int 0 N)) (IsTrue (leq_int N L))` holds, the file offset
   and span start have both advanced by `N`, the remaining length is `L - N`,
   and the bytes written are exactly `B[0..N)`;
3. **success completeness:** `writeAll` returns success only when
   `Equal Int N L`, so
   the entire input span has been written;
4. **first-error preservation:** if the next primitive call returns transfer
   error `e`, including `NoProgress`, `writeAll` returns that same first `e`
   after exactly the prefix `B[0..N)` and makes no claim about bytes after that
   prefix; and
5. **all-success corollary:** if every primitive call succeeds, `writeAll`
   returns success and writes all of `B`.

The executable helper matches the structural budget. Empty input succeeds
without a host call. In the `Suc fuel` branch it performs one positive-length
`writeAt`; `WriteProgress.Wrote n` advances offset and span by `n`,
decreases remaining bytes strictly, and recurses with `fuel`; an error
returns unchanged. Since every successful count is positive and at
most the remaining length, no more than the original byte length
successful steps are possible. Fuel exhaustion with bytes
remaining is excluded by the positivity lemma and is not a user-visible error.

PX8-F must supply real kernel-checked terms that consume the checked
positivity/bounds witnesses and prove strict decrease, fuel sufficiency,
success-implies-full-transfer, and error-prefix preservation. None may be an
`Axiom` or a postulated `writeAll` theorem. Ward owns the resource-lifetime
property of `71 §2.3`; it is not the termination proof.

The conformance route has four independently reaching observations:

1. all full writes return success with the whole span written;
2. at least one positive short write still reaches success with exact prefix
   accounting;
3. a write-zero reaches and returns `NoProgress`; and
4. a mid-stream transfer error returns that exact error after the exact written
   prefix.

Each seed must prove that its named branch was reached. A suite-green result or
an output shared by a different branch is not evidence for this contract.

### 1.8 Error classification: honest retry and the single `Revoked` identity (PX9)

The cross-domain error surface (`System.Error`,
`../../docs/program/10-linux-abi-completion.md §4`, PX9) carries two
**normative** classification properties. Each is stated as a relation a
conformance seed can discriminate, and PX9's kernel-checked laws (the foundation
deliverable) are their executable witness — not prose. Two relations name the
model: `error_transience` classifies an error identity as `Transient` or
`Permanent`; `retry_guidance` maps a transience and an operation's idempotence
to a retry verdict.

**(1) Transience does not imply retry-safety.** Transience is a property of the
**error identity**; idempotence is a property of the **operation**. They are
orthogonal, and a `RetryAdvised` verdict is obtainable **only** from the pair
`(Transient, Idempotent)`. In particular `retry_guidance Transient
NonIdempotent = RetryUnsafeNonIdempotent`, never `RetryAdvised`. The
load-bearing form is negative: **there is no spec-blessed
`retryable : error → Bool`.** A retry verdict that reads only the error and does
not consume the operation's idempotence is non-conformant — "the error is
transient" is not, by itself, permission to retry. The honest corollary, and
the reason the negative form matters: `error_transience Revoked = Permanent` —
a revoked authority is gone, so retry cannot help, and `Revoked` is never
`Transient`.

**(2) `Revoked` is a single identity.** The path-level
`IOError.Revoked` (§1.3.1) and the delivered resource-token and
Process-host `ResourceHostIO IOError.Revoked` (§1.3.1, §1.11) carry
**one** semantic denial: an otherwise well-formed, live,
sufficiently-righted operation that lost at the `../60-security/62 §4.2`
admission boundary. The host-boundary revoked of the progress/error
partition (§1.7.2) retains that cause. The successor
`ResourceError.Revoked` is a type-local projection of the same identity,
not a third current checked constructor or a different denial. A conforming
error surface exposes exactly **one** `Revoked` identity, classified
`Permanent`. The `ABI-REVOKE` unification preserves it rather than
collapsing it into `CapabilityDenied`, `Closed`, or an unrelated host-I/O
cause, which §1.7.2 already forbids. The Process-host projection stays
`ResourceHostIO IOError.Revoked` even when the ordinary resource-token
projection changes; it is not such a collapse.

These are behavioral contracts on the error surface, introducing no kernel
feature: PX9's type and laws are re-checked Ken (the foundation deliverable),
and this section is the normative source those laws witness. The conformance
seed (`../../conformance/surface/ffi-io/`) discriminates each property against
its non-conformant counterpart — a `retryable`-from-transience-alone
implementation, and a `Revoked = Transient` or triplicated-`Revoked` surface.

(3) `Unsupported` is host inability, probed once. `IOError.Unsupported`
means that this host cannot provide a facility the operation requires; it
is not an authority denial, an object-state error or an input error, and
`error_transience IOError.Unsupported = Permanent`. For each facility
below, the host decides availability at most once per process, on the
first operation that needs it, by a probe that performs none of the
operation's effects. While the probe has found its facility absent, each
listed operation returns `IOError.Unsupported` in its existing error
form before using the facility, with no fallback, whatever errno the
kernel or a policy such as seccomp gave the probe; a policy's `EPERM` is
not `PermissionDenied` here. If a probe fails without establishing absence,
the current operation stops at that facility check and reports the probe's
error through the operation's existing error mapping. No availability
verdict is cached, and the next operation that needs the facility probes
again. An operation whose facility
is present behaves as if no probe existed.

| Facility | Needed by |
|---|---|
| process descriptors (`pidfd_open`, `pidfd_send_signal`, `waitid(P_PIDFD)`) | `ProcessSpawn` |
| `statx` | `FsMetadata`, `FsHandleMetadata`; `FsReadDirectoryBatch` only for an entry whose type the read did not report |
| `/proc/self/fd`, describing this process | recursive `FsRemoveDirectory`; `NetListen`, `NetConnect`, `NetDatagramBind` and `NetDatagramSendTo` on a Unix address |
| close-on-exec stream sockets, one facility per address family | `NetListen` and `NetConnect` on an address of that family |
| `accept4` | `NetAccept` |
| close-on-exec datagram sockets, one facility per address family | `NetDatagramBind` on an address of that family |

Descriptor closure in a spawned child is performed by the C library's
spawn machinery, whose mechanism Ken does not observe, so it is not
probed. A policy that denies a host call outside this table reaches that
operation's ordinary error mapping.

### 1.9 Memory mappings as opaque runtime-owned regions and bounded views (ABI-S6)

A **mapping** — anonymous or file-backed — is a third runtime resource kind
beside `FsHandle` and `Buffer`, exposed to Ken **only** as an opaque
runtime-owned region and bounded byte views. The opacity is **absolute**: no raw
address, host pointer, or page reference ever reaches a Ken value or the resource
token. Every access is a **bounded-view copy** — bytes out of, or caller bytes
into, a bounds-checked window — never a mutable pointer into the region. Access
is **window-direct**: a mapping exposes no separate view token. `Buffer`'s
`BufferSpan` (`§1.7.1`) earns its keep only because `readAt` **caps** — the
actually transferred subrange can be smaller than the request
(`ReadProgress.ReadSome BufferSpan _`, `§1.7.2`) — so a minted token
must carry what was actually read.
A mapping does **not** cap: it bounds-checks each window against a fixed extent
and rejects an out-of-range one (below), so a live window's subrange **is** the
window and no span carries an invariant the window does not. This is the
mapping/lifetime/bounded-access substrate L2-8 MMIO later builds on, and it is
what keeps raw pointers out of application Ken
(`../../docs/program/10-linux-abi-completion.md §4`, §6).

At this ABI-S6 stage, the closed Ken resource-kind inventory is
`ResourceKind.FsHandle | ResourceKind.Buffer | ResourceKind.Mapping`;
§1.11 extends that inventory with four child-process kinds. A
wrong-kind live token reports `ResourceKindMismatch` (`§1.7`) with the `Mapping`
wire tag (Ken's `ResourceKind.Mapping` constructor) in the offending
position, under the same non-degenerate accept/reject discipline (a mapping
token to a buffer- or file-only operation, and the reverse, reject;
same-kind controls succeed).

**Acquisition and lifetime.** An opaque, constructor-private `MappingHandle` is
acquired **only** through the public `withMapping` bracket — its sole producer —
mirroring `withBuffer` (`§1.7.1`) and the `FsHandle` real-syscall-backed
acquire/release pattern (`§1.3.1`):

```ken
data MappingHandle = PrivateMappingHandle (Resource ResourceKind.Mapping) MappingExtent
```

`PrivateMappingHandle` and its field projections are absent from the public name
map: checked user code can neither forge a resource/extent pairing, project the
raw `Resource ResourceKind.Mapping`, nor construct a handle outside
`withMapping`. The handle may be copied as an ordinary checked value, but every
copy denotes the same acquisition and becomes invalid when the bracket
settles — the runtime invalidates escaped copies exactly as for file and buffer
resources. Lifetime is
bracket-scoped; a use after settle, or after revocation at the
`../60-security/62 §4.2` admission boundary, yields the single `Revoked`
identity, classified `Permanent` (`§1.8`): a revoked mapping is gone, and retry
cannot restore it.

**Source, offset, protection, and views.** `withMapping` takes the mapping
source — anonymous with a length, or a file `Resource ResourceKind.FsHandle`
with a length (the mapping starts at file offset 0) — and a requested
protection (`MappingProt.ReadOnly` or `MappingProt.ReadWrite`). Ken
observes only the opaque `MappingHandle`, an immutable `MappingWindow`
(offset, length) naming a read subrange, and the scalar extent. A read view
(`mapBytes`) copies a window's bytes out; a write view (`mapWrite offset bytes`)
copies the caller's `bytes` into the subrange `[offset, offset + len bytes)`. The
two are **asymmetric by design**: a read's extent is the requested length, so the
read window carries it; a write's extent is its **payload**, so `mapWrite` takes
only an `offset` and derives the range from `len bytes` — there is no
independent, separately-checked write-window length. Such a length would be
redundant (the payload already fixes the extent) and, under MAP_PRIVATE COW
(below), a write range wider than its payload is incoherent — nothing would fill
the excess. Both a read window and a
write range are bounds-checked against the fixed extent; an out-of-range one is a
fail-visible `ResourceError`, **never clamped** and never an unchecked access.
Liveness is re-checked at **each** access: a `mapBytes`/`mapWrite` on a mapping
used after the bracket
settles, or after revocation at the `../60-security/62 §4.2` admission boundary,
yields the single `Revoked` identity, `Permanent` (`§1.8`). Because the check
lives at the access op, there is no separate view token to invalidate and no
mint-then-consume gap. The public checked API has these shapes:

```ken
proc withMapping (a : Auth) (e : Type) (r : Type)
  (source : MappingSource) (prot : MappingProt)
  (body : MappingHandle -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [FS]

proc mapBytes (a : Auth) (mapping : MappingHandle) (window : MappingWindow)
  : HostIO a (Result ResourceError Bytes) visits [FS]

proc mapWrite (a : Auth) (mapping : MappingHandle) (offset : Int) (bytes : Bytes)
  : HostIO a (Result ResourceError Unit) visits [FS]
```

where the read window `MappingWindow = MkMappingWindow Int Int` (offset, length),
`MappingSource = Anonymous Int | FileBacked (Resource ResourceKind.FsHandle) Int`
(each a length; a `FileBacked` mapping starts at file offset 0), and
`MappingProt = ReadOnly | ReadWrite`. These are checked, scoped cases:
source selects only `MappingProt.ReadOnly` or `MappingProt.ReadWrite`,
not bare `ReadOnly` or `ReadWrite`. A `MappingProt.ReadOnly` mapping
refuses `mapWrite` with a fail-visible `ResourceError`.

**Mapping package home (Increment 2b).** `Capability.System.Mapping`
is the explicit-import home for exactly eight checked pre-source names:
`MappingHandle`, `MappingWindow`, `MkMappingWindow`, `MappingSource`,
`Anonymous`, `FileBacked`, `mapBytes` and `mapWrite`. The
`MappingWindow` and `MappingSource` exports include their complete
public constructor families; importing the abstract `MappingHandle`
type does **not** reveal `PrivateMappingHandle` or its fields. The
separate bracket operation `withMapping` remains homed at
`Capability.System.Resource`, and `MappingProt` remains a scoped
floor family (§30 §4). These homes reuse the checked identities
rather than making new types or widening the floor. Strict isolated
source imports the home to name its exports; the package does not
switch any CLI resolution route.

**The surface is 1:1 with the frozen wire — no validate-and-discard parameter.**
Every checked parameter of the mapping surface passes through to a frozen-wire
field; none is checked and then dropped. Census: `Anonymous`'s length and a
mapping's protection are the allocate op's fields; a `FileBacked` mapping's
resource and length are the file-acquire op's fields; the read window's offset
and length are the read op's fields; `mapWrite`'s offset and bytes are the write
op's fields. The file-acquire wire is **offset-less** (it maps from file offset
0), so `FileBacked` carries a length and **no offset**: a mapping at a non-zero
file offset is not expressible, and is deliberately not a checked-then-discarded
surface field — were one ever needed it is a wire extension, not a surface
parameter. This is why each op is a **single unconditional wire visit** that
returns its response directly, with **no in-body response transform**: a checked
parameter the wire drops would force exactly such a transform, which the native
lowering cannot carry. (The three ABI-S6 surface corrections — the read
view-token, the write-window length, and this file offset — are one defect, a
validate-and-discard parameter absent from the frozen wire, closed by this rule
rather than per symptom.)

**MAP_PRIVATE isolation — file mappings are copy-on-write.** A file-backed
mapping is **private**: writes through a write view are **process-local and never
reach the backing file**. This preserves the D4 denotation (it is not a new
choice here). Shared mappings whose writes are visible to the file or to other
processes (`MAP_SHARED`) are **out of scope**, and a conforming surface exposes
no such mode. The observable consequence a conformance seed can pin: a program
that `mapWrite`s through a file mapping and then reads the same file through the
ordinary `§1.3`/`§1.7` API observes the **original** file bytes, not its private
writes.

**Page-accounting — normative and host-independent.** The `Mapping` resource kind
is charged in a **fixed canonical granule of 4096 bytes (4 KiB)**: a mapping of
`n` bytes charges `ceil(n / 4096) * 4096` against the mapping accounting, and a
zero-length mapping is not admitted. This granule is **spec-normative and
host-independent** — it is **never** read from `sysconf(_SC_PAGESIZE)` or any
host page size, and the native and interpreted paths charge the **identical**
canonical value, so a program's mapping accounting is deterministic and portable
and a conformance seed can pin it exactly (a 1-byte mapping charges 4096; a
4097-byte mapping charges 8192). It is normative precisely because its purpose is
cross-host conformance determinism, which is unachievable if the granule tracks
the host; 4 KiB is chosen as the conventional minimum page size and the familiar
canonical value. It is a pure **accounting** quantity: the runtime still maps
with the host's real page size, but that host size is invisible to Ken. `Buffer`
accounting stays **byte-granular** (`§1.7`) — page-rounding is specific to the
`Mapping` kind.

A conformance seed (`../../conformance/surface/ffi-io/seed-mapping.md`)
discriminates the load-bearing properties against their non-conforming
counterparts: the copy-on-write isolation against a `MAP_SHARED`/write-through
surface, the 4 KiB page-rounding against a host-page-derived or byte-granular
rule, and the opacity/bounds discipline against an address-exposing or unchecked
view.

### 1.10 Terminal and signal disposition at the executable edge (ABI-S5)

The existing `proc is_terminal (stream : ConsoleStream) : IO Bool visits [Console]`
returns `True` exactly when that stream's descriptor is a terminal, and `False`
otherwise. It is total: its result is `Bool`, not `Result IOError Bool`. This
states the already-shipped Console operation, not a new operation or a new
capability.

The signal posture depends on the executable path. In `ken run`, the CLI parent
re-executes a worker child, and the Ken program runs in the worker. Rust's
startup ignores SIGPIPE before `main` in both parent and worker. Rust's
stack-overflow guard catches SIGSEGV and SIGBUS in both; the worker chains a
fault recorder over those actions. In the C-started native artifact, Ken's
entry posture ignores SIGPIPE, but does not catch SIGSEGV or SIGBUS. An internal
caught signal 33 in the threaded worker is glibc's reservation, not a
Ken-established disposition or public signal contract. The fatal re-raise
paths terminate the process; they leave no live edge state for a child.

Ignoring SIGPIPE allows a closed stdout to reach the existing
`IOError.BrokenPipe` result path, rather than terminating the program by
SIGPIPE. Both `ken run` and the C-started artifact have their own tested
closed-stdout path. Ken changes no signal mask on either path. With a clean
launch, all masks are empty; dispositions and masks established by the launcher
that Ken does not change pass through unchanged. In particular, an inherited
ignored SIGHUP or SIGINT and a blocked SIGUSR1 are not cleared by Ken.

A child process spawned from either live edge starts with the dispositions and
mask that edge inherited, except that every disposition Ken established is
returned to `SIG_DFL`. In the current inventory this requires an explicit
SIGPIPE reset on both paths: an ignored action survives `exec`. Caught
SIGSEGV/SIGBUS actions on the `ken run` path return to `SIG_DFL` automatically
on `exec`; the artifact has no caught actions. A glibc-internal caught signal
33 also resets on `exec` and is not a Ken setting. The child inherits its
signal mask unchanged: neither a blanket empty-mask reset nor a
`SETSIGMASK` spawn action follows from this contract. PX10 must explicitly
reset SIGPIPE rather than relying on a particular Rust runtime's spawn choice.

### 1.11 Bracketed child spawn plans and wait (PX10-SPAWN-PLAN-WAIT)

> **Section 1.11 status: Process and stream operations native.**
> The Architect's AC-0 on `40fa3de6c` measured the host descriptor and
> signal mechanisms for the first interpreter slice; the Spec Leader
> ruled the public contract. The checked interpreter implements the
> plan, spawn, pipe and wait operations. The landed
> `effect_abi_v1.catalog` marks all six Process operations and both
> `ResourceStreamRead` and `ResourceStreamWrite` native. The PX10 seed's
> `process-spawn-passes-argv-and-environment` client calls neither
> `ProcessSignal` nor `ResourceStreamWrite`; its Process and pipe path
> also uses native `BufferAllocate`, `BufferFreeze` and
> `ResourceRelease`, and invokes no unavailable clock, entropy or FS
> operation. A `Process` declaration alone no longer entails a
> typed native-build availability refusal. The named
> `rt_native_settlement_placement_key.rs::f_argv_env_pipe_matches_fresh_interpreter`
> row compares native and fresh-interpreter exit, terminal and effect
> observations on its own generated child-probe argv-env pipe input.
> That row uses a stated deployment profile raising Invocation Nodes
> from the starter's 64 to 80; at the starter profile,
> `r_nodes_starter_argv_refuses_on_invocation_nodes` measures the same
> program building and running natively, then ending in typed
> Invocation Nodes capacity exhaustion.
> Its fixture and argument list differ from the `px10-argv-env` seed,
> so this is bounded parity evidence for the same operation path, not
> execution of that exact seed or parity for all Process programs.
> This status correction claims no exact-seed native execution,
> official conformance-runner execution, new CI, publication or
> landing, or trust measurement. The first PX10 slice added no axiom
> or kernel rule; trust growth remains an operator stop.

#### 1.11.1 A pure plan and its closed initial descriptor targets

`SpawnPlan` is a public type of immutable, pure Ken values. Its
constructor is private: source cannot construct a plan by positional
fields or project a host plan resource from it. Its **public builders**
leave the representation open for later attachments without changing a
public constructor. The first builder
and setters have these checked shapes:

```ken
pub data ChildStdio = ChildPipe | ChildNull | ChildInherit ConsoleStream
export ChildPipe, ChildNull, ChildInherit
pub data ChildExit = Exited UInt8 | Signaled UInt8
export Exited, Signaled

pub fn spawn_plan (program : Bytes) (arguments : List Bytes) : SpawnPlan
pub fn with_environment
  (plan : SpawnPlan) (env : List (Prod Bytes Bytes)) : SpawnPlan
pub fn with_stdin (plan : SpawnPlan) (target : ChildStdio) : SpawnPlan
pub fn with_stdout (plan : SpawnPlan) (target : ChildStdio) : SpawnPlan
pub fn with_stderr (plan : SpawnPlan) (target : ChildStdio) : SpawnPlan
```

`ChildExit` is a scoped, exact checked family: source uses
`ChildExit.Exited` and `ChildExit.Signaled` with their `UInt8`
payloads, not bare `Exited` or `Signaled`. This does not make the
separate `ChildStdio` family a floor admission.

**Child-process package home (Increment 2b, extended in Increment 3a).**
`Capability.Process.Child` is the explicit-import home for twenty-eight
checked pre-source names: `ChildStdio`, `ChildPipe`, `ChildNull`,
`ChildInherit`, `SpawnPlan`, `spawn_plan`, `with_environment`,
`with_stdin`, `with_stdout`, `with_stderr`, `withChild`, `wait_child`,
`take_stdin`, `take_stdout`, `take_stderr`, `read_pipe`, `write_pipe`,
`close_pipe_write`, `close_pipe_read`, `ChildSignal`, `SignalHangup`,
`SignalInterrupt`, `SignalQuit`, `SignalKill`, `SignalUser1`,
`SignalUser2`, `SignalTerminate` and `signal_child`.
The first twelve were homed in Increment 2b; Increment 3a added
`SpawnPlan`, `with_stderr`, `take_stdin` and `take_stderr` at their
existing IDs. The nine signal names are the §1.11.4 addition.
The three pipe names reuse their existing prelude IDs, with no
change to §1.11.2's types, lowering or native availability.
The `ChildStdio` export carries its complete three-constructor public
family; `ChildSignal` carries the seven §1.11.4 constructors, and the
`SpawnPlan` export exposes the type without its private constructor.
`ChildExit` and its qualified cases remain in the scoped floor
(§30 §4), not in this package. Other child and pipe operations
described below retain their existing contract; this roster does not
give them an unruled home. Each homed name requires an explicit import
in Strict isolated source; this addition alters neither authority,
trust nor CLI resolution routes.

`spawn_plan` sets the environment to empty and all three child streams to
`ChildNull`. `with_environment` replaces, rather than adds to, the plan's
environment; each stdio builder sets its named target. A plan is inert until
an authorized spawn. The program is a path of bytes passed directly to the
OS, with **no PATH search**. The argument list is passed verbatim, including
its caller-supplied first entry as `argv[0]`; Ken does not synthesize one.
The environment is exactly the plan's entries, not an overlay on the
parent's environment. An empty argument list is invalid when spawning.
Program, argument, key and value bytes containing NUL are invalid; an
environment key is also invalid when empty or containing `=`.

`ChildStdio` has only the three displayed choices. `ChildInherit` names one
of the parent's `ConsoleStream.Stdin`, `ConsoleStream.Stdout`, or
`ConsoleStream.Stderr` values, not an arbitrary host descriptor.
`ChildNull` connects the corresponding child stream to the null device.
`ChildPipe` creates a fresh pipe for that stream: child stdin gives the
parent a **write** end, while child stdout or stderr gives the parent a
**read** end. Unselected streams stay null. The child inherits no other
parent descriptor, even a parent descriptor without close-on-exec. Its
observable descriptor set at start is exactly `{0, 1, 2}`. A descriptor
outside stdin/stdout/stderr or a duplicate descriptor entry in the
host-side plan is invalid. The immutable Ken builders do not expose an
arbitrary descriptor map or a raw descriptor constructor.

The checked prelude retains its internal `ChildInherit Stream`
declaration. Source-name registration presents that same checked stream
family as `ConsoleStream`, with public paths `ConsoleStream.Stdin`,
`ConsoleStream.Stdout` and `ConsoleStream.Stderr`; the source-visible
`ChildInherit ConsoleStream` spelling adds neither a second family nor a
`Stream` source alias. The plan's descriptor behavior is unchanged.

#### 1.11.2 Explicit Process grant and resource lifetime

Spawn, wait and pipe operations declare `visits [Process]`, not `[FS]`.
They obey the ordinary `36 §1.4` effect-row and escape checks. A `Process`
effect is not ambient spawn authority. The program explicitly declares
`program capabilities Process` as well as the existing
`program capabilities FS AFull`. Admission rejects a Process declaration
without that full FS declaration. Only a declared Process grant causes
the runner to mint the additional EXECUTE right on its FS capability;
`FS AFull` alone does not provide EXECUTE. An attenuated capability
without EXECUTE cannot spawn. The child runs outside Ken's capability
confinement, so this declaration acknowledges the OS authority the child
can exercise; the parent's FS root is **not** a child sandbox.

The public child and pipe operations have these exact checked signatures.
The `Cap a` passed to `withChild` is the existing FS capability; there is
no second Process-capability argument and no `Cap AFull` type. The runner
adds EXECUTE to that capability only under the declaration above. A missing
or attenuated EXECUTE right returns
`Err (ResourceHostIO IOError.CapabilityDenied)` before the child starts.

```ken
proc withChild (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (plan : SpawnPlan)
    (body : Resource ResourceKind.ChildProcess -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Process]

proc take_stdin (a : Auth) (child : Resource ResourceKind.ChildProcess)
  : HostIO a (Result ResourceError (Resource ResourceKind.PipeWrite)) visits [Process]
proc take_stdout (a : Auth) (child : Resource ResourceKind.ChildProcess)
  : HostIO a (Result ResourceError (Resource ResourceKind.PipeRead)) visits [Process]
proc take_stderr (a : Auth) (child : Resource ResourceKind.ChildProcess)
  : HostIO a (Result ResourceError (Resource ResourceKind.PipeRead)) visits [Process]

proc read_pipe (a : Auth) (pipe : Resource ResourceKind.PipeRead)
    (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError ReadProgress) visits [Process]
proc write_pipe (a : Auth) (pipe : Resource ResourceKind.PipeWrite)
    (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError WriteProgress) visits [Process]

proc close_pipe_write (a : Auth) (pipe : Resource ResourceKind.PipeWrite)
  : HostIO a (Result ResourceError Unit) visits [Process]
proc close_pipe_read (a : Auth) (pipe : Resource ResourceKind.PipeRead)
  : HostIO a (Result ResourceError Unit) visits [Process]

proc wait_child (a : Auth) (child : Resource ResourceKind.ChildProcess)
  : HostIO a (Result ResourceError ChildExit) visits [Process]
```

`withChild` is the sole public child-acquisition route. It uses the pure
plan to start one child and passes the live child resource to its body;
the private host plan resource never reaches source. Plan admission or
spawn failure skips the body, releases the private plan, and creates no
child. On success the result uses the existing `ResourceBracketResult`
four-way distinction between body success/error and release error.
Pipe endpoints are resources owned by the child bracket. The body
obtains ends only through the three fixed-direction `take_*`
operations: `take_stdin` yields `PipeWrite`, while `take_stdout` and
`take_stderr` yield `PipeRead`. Each selected `ChildPipe` end can be taken
**once**. Taking it again, or taking a `ChildNull` or `ChildInherit` end,
returns `Err (ResourceHostIO IOError.InvalidInput)`. No take accepts an
arbitrary descriptor index. The resource kinds are
`ResourceKind.SpawnPlan` for the host-side encoded plan,
`ResourceKind.ChildProcess` for the live child, `ResourceKind.PipeRead`
for a parent stdout/stderr end, and `ResourceKind.PipeWrite` for a parent
stdin end. Together with `ResourceKind.FsHandle`,
`ResourceKind.Buffer`, and `ResourceKind.Mapping`, these are the complete
seven-kind inventory of this slice. All seven constructors are scoped
`ResourceKind.C` paths; none is a bare reserved name. `read_pipe` uses
an existing `BufferHandle` and admitted `BufferWindow`, yielding
`ReadProgress.ReadSome` with a bounded span and count or
`ReadProgress.ReadEof`. `write_pipe` uses a `BufferSpan` from that
same buffer acquisition and returns `WriteProgress.Wrote` with the
actual count; a short write remains the caller's work. Neither
exposes a raw file descriptor or pointer. A pipe client also needs the
existing `withBuffer` bracket and its `[FS]` effect, so its combined
row is `[FS, Process]`. A write after the child closes its read end
returns `Err (ResourceHostIO IOError.BrokenPipe)` without terminating
the Ken edge by SIGPIPE.

`close_pipe_write` lets a child reading stdin see EOF before a wait;
`close_pipe_read` is the symmetric early close. Both are non-idempotent:
closing an end invalidates every copy, and a repeated close returns
`Closed`. The bracket still owns and settles an end closed early by the
body. `wait_child` blocks until the child exits or is killed by a signal,
returning `ChildExit.Exited code` or `ChildExit.Signaled signal`,
with the observed code or signal number as `UInt8`. Its successful
result is cached: another `wait_child` on that live child returns the
same `ChildExit`. Stopped and
continued states are not wait results. At bracket settlement, taken and
untaken pipe ends are released **before** the child, in reverse
acquisition order; an unwaited child is reaped. On terminal-error
finalization an unreaped child is killed and reaped, not left as a
zombie. Revoking the originating grant does **not** signal the child;
later plan, child and pipe host operations retain `IOError.Revoked` via
`ResourceHostIO`. After settlement, resource use returns `Closed`.

#### 1.11.3 Process errors and the inherited signal boundary

The existing `Capability.System.Error` surface extends its operation
classification with `ProcessOp ProcessOperation` and its resource
classification with `ProcessResource (Option Bytes)`. The public
`ProcessOperation` constructors and the catalog-side lift are:

```ken
export ProcessOperation, SpawnChild, TakePipe, ReadPipe, WritePipe, ClosePipe, WaitChild, SignalChild

data ProcessOperation = SpawnChild | TakePipe | ReadPipe | WritePipe | ClosePipe | WaitChild | SignalChild

pub fn process_error_to_system
    (operation : ProcessOperation) (program : Option Bytes) (cause : IOError)
  : SystemError =
  MkSystemError (ProcessOp operation) (ProcessResource program) cause NoSafeContext
```

When known, `ProcessResource` carries the program bytes; it does not
expose a raw pid or fd. The public Process operations return the existing
`ResourceError` family: a host-I/O failure uses `ResourceHostIO IOError`.
Only that cause is passed to `process_error_to_system` with the operation
identity. `Closed`, `ResourceKindMismatch`, and `InvalidBounds` remain
resource-discipline errors, not `SystemError` causes. There is no parallel
`SpawnError` family or a new `Revoked` identity. Causes use the already
checked `IOError` identities:

- `IOError.NotFound` for ENOENT;
- `IOError.PermissionDenied` for EACCES or EPERM;
- `IOError.InvalidInput` for a malformed plan: NUL in program,
  argument or environment bytes; an empty or `=`-containing key;
  duplicate/invalid descriptor entries; or empty argv at spawn;
- `IOError.CapabilityDenied` when EXECUTE is absent;
- `IOError.Revoked` when the originating grant was revoked;
- `IOError.BrokenPipe` when writing to a closed pipe; and
- `IOError.Other errno` for other failures, including ENOEXEC, E2BIG,
  ETXTBSY and ECHILD.

The child begins with SIGPIPE at `SIG_DFL`: the spawn path explicitly
resets Ken's ignored SIGPIPE, rather than relying on a particular Rust
runtime. Its signal mask is inherited unchanged from the **spawning
thread**, not cleared. Other inherited dispositions that Ken did not
establish remain inherited as in §1.10; caught actions reset on exec.
No post-fork Ken callback or Ken code runs before exec. The public
contract is the observed descriptor, signal, result and authority
boundary, not a requirement to expose the host's spawn machinery.

A SIGCHLD disposition of `SIG_IGN` or `SA_NOCLDWAIT`, whether set by
the launcher or foreign code in the process, can cause the OS to
auto-reap the child. Then `wait_child` can return ECHILD as
`Err (ResourceHostIO (IOError.Other 10))` instead of a `ChildExit`;
Ken does not reset SIGCHLD or promise a successful wait under that
disposition. This residual does not permit changing §1.10's
inherited-disposition rule.

A child that can be held is identified by a process descriptor acquired
immediately after spawn and retained through settlement; `wait_child`
and terminal-error kill use that identity, never the raw process number.
If pidfd support is unavailable, `withChild` returns
`Err (ResourceHostIO IOError.Unsupported)` before creating a child,
with no PID fallback. If descriptor acquisition reports that the OS
has already auto-reaped the child, spawn still succeeds: the body
runs, its pipes remain usable, `wait_child` returns
`Err (ResourceHostIO (IOError.Other 10))`, and terminal settlement
sends no signal. A narrow identity residual remains only under a
SIGCHLD `SIG_IGN` or `SA_NOCLDWAIT` at any time from `posix_spawn`
returning through descriptor acquisition or during mint-failure
cleanup, whether established by the launcher or by foreign code:
if the child exits and an unrelated process reuses its PID before
identity acquisition, Ken could hold the unrelated process instead.
This requires the kernel's cyclic PID allocator to wrap the entire
PID space within that interval; Ken does not reset SIGCHLD to
suppress it.

If descriptor acquisition fails for any reason other than an
already-reaped child (ESRCH), spawn returns
`Err (ResourceHostIO (IOError.Other errno))` and sends SIGKILL to the
new child by its process number. This mint-failure cleanup is the
product's one raw-number signal. Spawn then waits for the child unless
SIGCHLD auto-reaps, in which case the OS reaps it. The child may have
begun executing; effects it has already performed are not undone.
This is consistent with `SpawnChild` being `NonIdempotent`: retrying
spawn may duplicate those effects. Under the SIGCHLD auto-reap
precondition above, the raw-number signal shares descriptor
acquisition's PID-reuse residual: if the child has been reaped and
its number reused, the signal may reach an unrelated process.

Outside that residual, the AC-1 observations include an
argv/environment echo through a stdout
pipe followed by `ChildExit.Exited 0`, and a self-SIGTERM child yielding
`ChildExit.Signaled 15`, under the checked interpreter. AC-2 observes the live
child's three descriptors, default SIGPIPE and inherited mask while a
non-CLOEXEC parent descriptor is present. Deny-by-default descriptors
and SIGPIPE reset are independent obligations: removing either must
change its corresponding live-child observation. In this slice a native
build refuses a `Process`-declaring program before object emission, so
these are interpreter observations; native parity is an obligation of
the successor slice.

Native Process parity, cwd attachments, credentials, namespaces,
cgroups, rlimits, PATH search, process groups and non-blocking wait
are successor work, not implicit fields or behaviors of this first
slice.

#### 1.11.4 Signalling a held child (PX10 Increment 2)

`Capability.Process.Child` exports one closed, seven-constructor
`ChildSignal` family and the following effectful operation:

```ken
pub data ChildSignal = SignalHangup | SignalInterrupt | SignalQuit | SignalKill | SignalUser1 | SignalUser2 | SignalTerminate
export SignalHangup, SignalInterrupt, SignalQuit, SignalKill, SignalUser1, SignalUser2, SignalTerminate

proc signal_child (a : Auth) (child : Resource ResourceKind.ChildProcess) (sig : ChildSignal)
  : HostIO a (Result ResourceError Unit) visits [Process]
```

In constructor order, the signal numbers are **1, 2, 3, 9, 10, 12
and 15**, matching the numbers reported by `ChildExit.Signaled`.
The family does not include STOP, TSTP, TTIN, TTOU or CONT: a
stopped child is not a `wait_child` result, so a wait or normal
settlement could block indefinitely. Signal zero probes liveness
without a useful Ken PID meaning and is excluded. CHLD, PIPE,
ALRM, WINCH, synchronous-fault and real-time signals have no ruled
use and are excluded. Expanding this family requires a later ruling;
it does not require a change to `signal_child`'s type.

The **held `ChildProcess` resource**, not a process identifier or a
second capability argument, supplies authority. Only `withChild`
acquires one; that bracket already requires an explicitly declared
Process grant and the EXECUTE right. No Ken source path can supply a
raw PID. The host operation is `ProcessSignal` (ABI code `0x0606`),
with **Target** resource admission and **no capability requirement**
at the host leaf; the resource admission checks the originating
grant's revocation before delivery. A revoked grant returns
`Err (ResourceHostIO IOError.Revoked)` without sending a signal.
Its Process effect row remains explicit despite the resource-based
authority. Delivery uses the held process descriptor, never a
raw-PID fallback or a signal to an unrelated process.

For a live child, including one that has exited but has not yet
been waited, signalling returns `Ok Unit`. An already-exited child's
signal is discarded by the OS; `wait_child` still reports its actual
exit, and a race between the signal and exit is decided by that
wait result. A child whose successful wait result is cached instead
returns `Err (ResourceHostIO (IOError.Other 3))` (ESRCH), decided
from the cache without a host signalling syscall. An already
auto-reaped child (`ReapedAtSpawn`, or later kernel ESRCH) returns
the same `Other 3` error. Use after the bracket has settled returns
`Closed`, not ESRCH. None of these arms grants authority to signal
any other process.

`Capability.System.Error` classifies the new
`ProcessOperation.SignalChild` as `NonIdempotent` beside the
`Idempotent` `WaitChild`: delivering a signal again can be observed,
for example by a child counting `SignalUser1`. No new grant,
capability kind, postulate or kernel rule is introduced. The
interpreter and native parity of this operation belong to the
separately reviewed Runtime half; the CV-owned conformance seed
checks black-box signal-and-wait and cached-wait outcomes. Neither
that seed nor the Runtime candidate is validated by this Spec child.

### 1.12 Strict isolated I/O vocabulary (Increment 3a)

> **Increment 3a status: specified, not an implementation or route gate.**
> The exact-base AC-0 census on `f3c016f89` identified source-written
> names blocked at the Strict entry; the Architect's floor analysis and
> Steward admission `evt_36xwaa64spemr` classify the exact checked IDs.
> Their package tests, assembled-candidate review, official conformance,
> CI and landed trust are separate evidence, not established here.

The four added Child homes and the six names held since Increment 2 are
resolved as follows. H means an existing checked pre-source identity
published by an ordinary catalog unit; B means a scoped floor family,
**not** a package home. These are source-availability changes, not new
operations or a change to any existing type, capability, effect row or
native artifact leaf. The `read` and `wall_now` operations keep their
existing Console and Clock effect rows, and `host_clock` remains the
Clock-to-HostIO lift.

| Written name | Strict source path | Existing checked role |
|---|---|---|
| `SpawnPlan` | H: import `Capability.Process.Child (SpawnPlan)` | Public plan type; constructor remains private. |
| `with_stderr` | H: import `Capability.Process.Child (with_stderr)` | Pure setter alongside `with_stdin` and `with_stdout`. |
| `take_stdin` | H: import `Capability.Process.Child (take_stdin)` | Parent write end of child stdin. |
| `take_stderr` | H: import `Capability.Process.Child (take_stderr)` | Parent read end of child stderr. |
| `read` | H: import `Capability.Console.Text (read)` | `ConsoleStream → Int → IO (Result IOError ReadResult)`, `visits [Console]`. |
| `ReadResult` | B: `ReadResult` without import | Exact checked Console read result type. |
| `Chunk` | B: `ReadResult.Chunk` | Carries the bytes read; bare `Chunk` is not reserved. |
| `Eof` | B: `ReadResult.Eof` | End-of-input case; bare `Eof` is not reserved. |
| `wall_now` | H: import `Capability.Time.WallClock (wall_now)` | `ClockIO Instant` action, `visits [Clock]`. |
| `host_clock` | H: import `Capability.Process.Exit (host_clock)` | `(a : Auth) (r : Type) (action : ClockIO r) : HostIO a r visits [Clock]`. |

The `ReadResult` family is exactly `Chunk Bytes | Eof`; its public
constructors use only the scoped paths shown. The native Console-read
reply is built with those same checked constructor IDs, so an imported
lookalike family cannot replace it. `Capability.Console.Text` already
homes `write` and `IO`, `Capability.Time.WallClock` has the wall-clock
helpers, and `Capability.Process.Exit` already homes the other HostIO
lifts. An H home requires both the package-home mapping and its unit's
public export; a mapping row alone is not an importable interface.
For each H name a Strict isolated client with its import accepts and
the same client without the import refuses `UnboundName`. Floor names
need no import; their qualified public cases resolve at the original
constructor IDs, with a same-spelled source family unable to replace
the native response. The CLI's `check`, `run` and `native-build` route
switches belong to Increment 3b, not this vocabulary slice.

The existing exact checked FS result carriers `FileKind` (`KFile`,
`KDirectory`, `KSymlink`, `KOther`), `FileMetadata` and `DirEntry`
(`MkDirEntry Bytes FileKind`) also enter the scoped floor. Native
metadata and directory replies select their checked constructors;
source uses only `FileKind.KFile` and its three sibling qualified
cases, `FileMetadata.MkFileMetadata` and `DirEntry.MkDirEntry`.
`FileOperation` has ten exact checked path-operation cases and is named
by source in `Capability.System.Error` (§33 §3.3). It moved from that
package home to the scoped floor in Increment 3a. Its complete cases
are available as `FileOperation.OpReadFile`, `FileOperation.OpWriteFile`,
`FileOperation.OpAppendFile`, `FileOperation.OpMetadata`,
`FileOperation.OpReadDirectory`, `FileOperation.OpCreateDirectory`,
`FileOperation.OpRemoveFile`, `FileOperation.OpRemoveDirectory`,
`FileOperation.OpRename`, and `FileOperation.OpChangeMode`.
`FileOperation` names only path operations; handle operations report through
`ResourceError` and have no `FileOperation` case.
Only these five I/O family names enter the counted floor from this
paragraph; none of their constructor leaves joins bare protected B.
`StateOp.Get` and `StateOp.Put` are likewise scoped (§36 §4.2).
Native stable-symbol leaves and runtime IDs do not change for the cases
that remain, including all ten `FileOperation` cases. The six retired
leaves have no stable symbol or runtime ID.

For ABI-S4, `FileMetadata` grows in place, retaining its scoped-floor
family and the qualified `FileMetadata.MkFileMetadata` constructor.
The complete constructor shape, in field order, is:

```text
MkFileMetadata Int FileKind (Option Int) (Option Int) Int Int (Option Int)
  (Option Instant) (Option Instant) (Option Instant) (Option Instant)
```

Both `file_metadata` and `resourceMetadata` obtain all fields from one
Linux `statx` result for an already-opened file or handle. The fields are,
in order:

1. Size in bytes (`Int`), required from `STATX_SIZE`.
2. Kind (`FileKind`), required from `STATX_TYPE`.
3. Mode (`Option Int`): `stx_mode & 0o7777` when `STATX_MODE` is set,
   regardless of file kind.
4. Hard-link count (`Option Int`), reported by `STATX_NLINK`.
5. Device major number (`Int`).
6. Device minor number (`Int`).
7. Inode number (`Option Int`), reported by `STATX_INO`.
8. Modification time (`Option Instant`), reported by `STATX_MTIME`.
9. Status-change time (`Option Instant`), reported by `STATX_CTIME`.
10. Access time (`Option Instant`), reported by `STATX_ATIME`.
11. Birth time (`Option Instant`), reported by `STATX_BTIME`.

The device numbers are not optional: `statx` supplies them without
availability bits. For each optional field, `Some` carries the value
exactly when its corresponding bit is set in the returned mask; otherwise
it is `None`. No zero or value from another field substitutes for absence.
The mask itself and all other `statx` fields are outside this carrier.

Each reported time uses the existing `Instant` in the same unit and epoch
as `wall_now`: signed nanoseconds since the Unix epoch, computed exactly
as `tv_sec * 1_000_000_000 + tv_nsec`. If either required bit is missing,
or a reported time has `tv_nsec` outside `[0, 1_000_000_000)`, the operation
fails without returning a partial or fabricated record. `file_metadata`
returns a `FileError` with `FileOperation.OpMetadata`, `Some path`, and
`IOError.Unsupported`; `resourceMetadata` returns
`ResourceHostIO IOError.Unsupported`.

Seven checked FS wrappers (`append_file`, `change_mode`, `create_directory`,
`remove_directory`, `remove_file`, `rename_file` and `file_metadata`) are homed in
`Capability.Filesystem.Authority`; `writeAt` and `writeAll` in
`Capability.System.IO`; `freeze` in `Capability.System.Buffer`;
`resourceMetadata` in `Capability.System.Resource`; `flush` and
`is_terminal` in `Capability.Console.Text`; and `intToChar` beside
`charToInt` in `Core.Classes.LawfulClasses` (§30 §4). Each retains its
checked type, capability and effect contract. None is a new floor
member. `FileOperation` and the four other scoped families receive
no package-home row. Neither `Core.Logic.And` nor
`Core.Logic.Conjunction` becomes the distinct pre-source `And`
identity; Conjunction's exported `And` is its own checked definition
(§30 §4). `StateOp` and `resp_state` enter the floor via §36 §4.2,
not via a package home. Per-name Strict-entry pins and held non-exact
shapes are recorded in §30 §4; no host operation constructor or State
signature is homed by analogy.

### 1.13 Typed stream sockets (PX11 first slice)

> **PX11 status: specified; stream-socket native parity is PX11 Inc2.**
> The seven `Net` stream host operations are `NativeTested`: they
> execute on the checked interpreter and on the native lane, and a
> declared `Net` grant no longer refuses a native build. The four
> datagram operations of §1.13.1 are interpreter-only. No seed execution, official
> conformance, CI, publication, landing or trusted-base measurement is
> claimed by this Spec addition.

`SocketAddress` is a closed, opaque three-arm type: IPv4 carries four
`UInt8` octets and a `UInt16` port; IPv6 carries sixteen `UInt8`
octets and a `UInt16` port; Unix carries an **absolute pathname** as
`Bytes`. Checked smart constructors provide those three cases, while
the representation constructors remain private. The IPv6 scope ID and
flowinfo are zero in this slice; scoped link-local addresses are a
successor. A Unix path starts with `/`, contains no zero byte, and is
2–107 bytes long: Linux's 108-byte `sun_path` includes the terminating
zero. An invalid path returns `IOError.InvalidInput` without creating
a socket. The host validates a decoded address again before any socket
syscall. Neither a relative Unix path nor the abstract namespace is
admitted: abstract names lack this slice's filesystem-scoped authority
model. No arbitrary address-family number, raw `sockaddr` pointer,
implicit character decoding or truncated path enters Ken. The
abstract namespace needs its own authority class in a successor.

`SocketAddress` is a prelude-floor identity (`30-taxonomy §4`), so
`Capability.System.Error` names it without importing this home. The
`Capability.Net.Stream` home exposes the three smart constructors,
re-exports that same `SocketAddress` identity, and provides the socket
resource operations below and a closed shutdown-direction family.
`ResourceKind` gains the two scoped cases `ResourceKind.SocketListener`
and `ResourceKind.SocketStream`; with §1.13.1's
`ResourceKind.SocketDatagram` there are ten in all. The contract shapes are:

```ken
pub fn socket_ipv4 (octets : Vec UInt8 4) (port : UInt16) : SocketAddress
pub fn socket_ipv6 (octets : Vec UInt8 16) (port : UInt16) : SocketAddress
pub fn socket_unix_path (path : Bytes) : Result IOError SocketAddress

pub data SocketShutdown = ShutdownRead | ShutdownWrite | ShutdownBoth
export ShutdownRead, ShutdownWrite, ShutdownBoth
pub data SocketTarget = ListeningSocket (Resource ResourceKind.SocketListener)
  | ConnectedSocket (Resource ResourceKind.SocketStream)
export ListeningSocket, ConnectedSocket

proc with_socket_listener (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (address : SocketAddress)
    (body : Resource ResourceKind.SocketListener
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc with_socket_accept (a : Auth) (e : Type) (r : Type)
    (listener : Resource ResourceKind.SocketListener)
    (body : Resource ResourceKind.SocketStream
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc with_socket_connection (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (address : SocketAddress)
    (body : Resource ResourceKind.SocketStream
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc receive_socket (a : Auth) (stream : Resource ResourceKind.SocketStream)
    (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError ReadProgress) visits [Net]
proc send_socket (a : Auth) (stream : Resource ResourceKind.SocketStream)
    (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError WriteProgress) visits [Net]
proc shutdown_socket (a : Auth) (stream : Resource ResourceKind.SocketStream)
    (how : SocketShutdown)
  : HostIO a (Result ResourceError Unit) visits [Net]
proc socket_local_address (a : Auth) (target : SocketTarget)
  : HostIO a (Result ResourceError SocketAddress) visits [Net]
```

A program declares its network authority with the header form
`program capabilities Net <class>+`, where `<class>` is exactly one of
`Loopback`, `Inet` or `Unix`. The `+` is grammar metanotation requiring
at least one class, not a source token. Thus
`program capabilities Net Loopback` is a complete single-class form;
no other class member, including `AFull`, is admitted as a `Net` class.
The runner mints those declared classes onto the existing `Cap a`, not
a separate `Cap_Net` value. `Loopback` permits
IPv4 `127.0.0.0/8` and IPv6 `::1`; `Inet` covers all IPv4/IPv6
addresses and implies `Loopback`; `Unix` covers absolute Unix paths
beneath the FS root of the presented `Cap a`. Both listen and connect
use the address's class; `0.0.0.0` and `::` need `Inet`, not `Loopback`.
A `Unix` declaration also requires `FS AFull`; `Loopback` and `Inet`
do not. At `NetListen` and `NetConnect` acquisition, an address outside
the held class returns `ResourceHostIO IOError.CapabilityDenied`, and
a revoked grant returns `ResourceHostIO IOError.Revoked`, before any
socket syscall. The listener or stream retains the originating grant's
provenance.

For Unix acquisition, a path outside the presented capability's FS root,
a `..` component, or a symbolic link at any component returns
`ResourceHostIO IOError.CapabilityDenied` before any socket syscall.
With no explicit FS root, the execution-start cwd is the root; root `/`
admits paths reached without symlinks. A spelling that reaches the root
only through a symbolic link is outside it. An empty or final-dot socket
name and a listener's final component longer than 82 bytes return
`IOError.InvalidInput`; a missing parent returns `IOError.NotFound`,
and a non-directory parent returns `IOError.NotDirectory`. An unreadable
or deleted root returns `IOError.CapabilityDenied`.

On a revoked originating grant, `NetAccept`, `NetReceive`, `NetSend`,
`NetShutdown` and `NetLocalAddress` return the delivered FS/Net
resource-token carrier `ResourceHostIO IOError.Revoked` before any
guarded host leaf. They are not Process host operations (§62 §4.1).
After unification, their successor projection is
`ResourceError.Revoked`; it is not a current checked constructor or a
thirteenth member of the present `ResourceError` family. Bracket
settlement still uses `ResourceRelease`. The
`a : Auth` argument and `HostIO a` are the existing authority-indexed
carrier; Net-class
membership is checked on the decoded address at admission, not
inferred from a string or from the effect row alone.

All seven stream operations declare `visits [Net]`; the existing §36 §1.4
escape check rejects a caller without `Net` in its row. Acquiring a
listener or connected stream also takes `Cap a`, and acceptance is
confined to an already-held listener. A listener bound to IPv4 or IPv6
port zero reports its OS-assigned port through `socket_local_address`;
a Unix listener reports the validated address it was acquired with,
not the host-internal pinned descriptor spelling. For a stream accepted
from a Unix listener, `socket_local_address` returns that listener's
validated address. Peer-address readback is not part of this slice. Each
bracket settles
its acquired resource via the existing `ResourceRelease`, even when
the body returns an error; the bracket's result retains the existing
four-way body/release distinction. A Unix pathname created by bind is
**not** unlinked on release: deleting it is a separate FS act. Listener
and stream resources are opaque and expose no raw descriptor. The
backend uses blocking stream sockets, with no implicit `SO_REUSEADDR`
or other option. A connection failure closes the newly created socket
before returning the error.

`receive_socket` and `send_socket` have their own `Net` operation
identities rather than reusing the Process-labelled
`ResourceStreamRead`/`ResourceStreamWrite` codes. They reuse those
operations' buffer bounds, replies and progress partition (§1.7.2).
Receive with a zero effective bound or a host zero result yields
`ReadProgress.ReadEof`; positive progress returns exactly the live
span and count, never more than the admitted `BufferWindow`. Send
returns `WriteProgress.Wrote` with a positive count not exceeding the
admitted `BufferSpan`; a short send leaves the remainder to the
caller, while a zero send yields `NoProgress`. Sending on a closed
peer returns `ResourceHostIO IOError.BrokenPipe` without SIGPIPE
terminating Ken (`MSG_NOSIGNAL`). After `ShutdownWrite` and queued
bytes have been received, the peer sees `ReadEof`; `ShutdownRead` and
`ShutdownBoth` choose their corresponding directions. `withBuffer`
still carries `[FS]`, so a socket client acquiring its own buffer
needs the combined `[FS, Net]` row, even though the transfer itself
carries only `[Net]`.

Every listener, outbound stream and accepted stream descriptor is
created with close-on-exec (`SOCK_CLOEXEC` or `accept4` with that flag),
without a descriptor-inheritance window. An exec'd PX10 child sees
none of those sockets; PX10's independent closefrom step is not a
substitute for the close-on-exec property on each descriptor.

`Capability.System.Error` keeps its canonical `IOError` family and
extends the existing operation and resource sums, without a parallel
socket-error family:

```ken
pub data SocketOperation = ListenSocket | AcceptSocket | ConnectSocket
  | ReceiveSocket | SendSocket | ShutdownSocket | LocalSocketAddress
  | CloseSocket | BindSocket
pub data Operation = FilesystemOp FileOperation
  | ProcessOp ProcessOperation | SocketOp SocketOperation
pub data ResourceRef = FilesystemResource (Option Bytes)
  | ProcessResource (Option Bytes) | SocketResource (Option SocketAddress)

pub fn socket_error_to_system
    (operation : SocketOperation) (address : Option SocketAddress)
    (cause : IOError) : SystemError =
  MkSystemError (SocketOp operation) (SocketResource address)
    cause NoSafeContext
```

The operation identity distinguishes the seven stream operations, the
datagram bind of §1.13.1 (`BindSocket`, last in the sum) and resource
closure; `LocalSocketAddress` is idempotent, while the other
socket operations, including close, are non-idempotent for retry
classification. The address is carried when known, never a raw fd;
accepted peers need not disclose their remote address. Only a
`ResourceHostIO` cause lifts into this envelope. `Closed`,
`ResourceKindMismatch` and `InvalidBounds` remain resource errors.
Connecting to an absent Unix pathname returns `IOError.NotFound` and
lifts with `SocketOp ConnectSocket` and `SocketResource (Some address)`;
connect to an existing endpoint refusing the connection maps
`ECONNREFUSED` to `IOError.Other 111`. `EADDRINUSE`, `ECONNRESET` and
`ENOTCONN` map to `IOError.Other 98`, `Other 104` and `Other 107`.
No new `IOError` constructor or trust-bearing rule is introduced.

The seven stream host operations are `NetListen`, `NetAccept`,
`NetConnect`, `NetReceive`, `NetSend`, `NetShutdown` and
`NetLocalAddress`. Their host-family codes are `0x0701` through `0x0707`
in that order; the four datagram operations of §1.13.1 follow at
`0x0708` through `0x070B`, for eleven `Net` operations in all.
`ResourceKind.SocketListener` and `ResourceKind.SocketStream` are new
opaque resource kinds; resource release uses the existing operation.
All seven stream operations are `NativeTested` and execute on both the
interpreter and native lanes. A declared Net grant no longer triggers a
native-build refusal. Typed option families, an injected resolver and DNS
boundary, nonblocking/readiness and abstract Unix names remain subsequent work.
No new kernel rule or trusted postulate is authorized. Any required trust
growth is an operator stop.

#### 1.13.1 Typed datagram sockets (PX11-DATAGRAM-SOCKETS)

> **Status: specified; host and interpreter lanes only.** The four
> datagram operations are `RepresentedUnavailable`. The shared executor
> admits them through an explicit interpreter-only roster naming exactly
> these four; the native ABI keeps its own `NativeTested` gate. Native
> parity is a successor node. No seed execution, official conformance,
> CI, publication, landing or trusted-base measurement is claimed by
> this Spec addition.

A datagram socket binds to a typed `SocketAddress`, sends one bounded
datagram to a typed peer, and receives one datagram together with its
sender's address. It reuses §1.13's `SocketAddress` identity and smart
constructors, `Net` classes, FS-root rule and bracket release. The
`Capability.Net.Datagram` home re-exports `SocketAddress`, `socket_ipv4`,
`socket_ipv6` and `socket_unix_path`, and provides the operations below.
`ResourceKind` gains the opaque scoped case `ResourceKind.SocketDatagram`,
for ten kinds in all. The contract shapes are:

```ken
pub data DatagramFit = WholeDatagram | TruncatedDatagram
export WholeDatagram, TruncatedDatagram
pub data DatagramReceipt = EmptyDatagram (Option SocketAddress)
  | ReceivedDatagram BufferSpan TransferCount DatagramFit (Option SocketAddress)
export EmptyDatagram, ReceivedDatagram

proc with_datagram_socket (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (address : SocketAddress)
    (body : Resource ResourceKind.SocketDatagram
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc send_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
    (peer : SocketAddress) (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Unit) visits [Net]
proc receive_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
    (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError DatagramReceipt) visits [Net]
proc datagram_local_address (a : Auth)
    (socket : Resource ResourceKind.SocketDatagram)
  : HostIO a (Result ResourceError SocketAddress) visits [Net]
```

The four host operations are `NetDatagramBind`, `NetDatagramSendTo`,
`NetDatagramReceiveFrom` and `NetDatagramLocalAddress`, with host-family
codes `0x0708` through `0x070B` in that order. They are carried on the
existing FSOp carrier under the `Net` row label, as the stream operations
are. All four declare `visits [Net]`. Release is the existing
`ResourceRelease`. A separate local-address operation exists because an
IPv4 or IPv6 socket bound to port zero can learn its port only by readback,
and widening `SocketTarget` or `NetLocalAddress` would change a stream
operation.

**Bind.** `with_datagram_socket` is the only datagram acquisition, so every
`Resource ResourceKind.SocketDatagram` is bound; there is no datagram
connect and no unbound or unconnected receive state, and no typed error
for one is added. Bind takes `Cap a` and checks the address's `Net` class
exactly as `NetListen` does: `Loopback`, `Inet` and `Unix`, where a `Unix`
declaration also needs `FS AFull`. An address outside the held class is
`ResourceHostIO IOError.CapabilityDenied`, and a revoked grant is
`ResourceHostIO IOError.Revoked`, both before any socket syscall. A Unix
path is resolved by §1.13's rules with the same errors: outside the root,
a `..` component or a symbolic link at any component is `CapabilityDenied`;
an empty or final-dot name is `InvalidInput`; a missing parent is
`NotFound`; a non-directory parent is `NotDirectory`. A refused bind leaves
no inode. The socket is `SOCK_DGRAM | SOCK_CLOEXEC`, blocking, with no
option set; a failure after `socket()` closes it. The resource retains the
originating grant's provenance, and release does not unlink the pathname.

A datagram Unix bind uses the spelling
`/proc/<pid>/fd/<parent fd>/<leaf>`, with the pid as `/proc` names this
process, not `/proc/self/...`, so the name the kernel records is unique
among live processes. The leaf is at most 79 bytes: the prefix takes at
most 28 of the 107 `sun_path` bytes (six for `/proc/`, seven pid digits,
four for `/fd/`, ten descriptor digits and a slash). A longer leaf is
`IOError.InvalidInput`. The socket keeps its pinned parent directory open
until release, so the descriptor number inside a live socket's name cannot
be reused by another live bind in the same process.

**Send.** `send_datagram` takes no `Cap`; the peer is admitted against the
socket's retained grant. An IP peer outside the held class is
`CapabilityDenied`, and a revoked grant is `Revoked`. A Unix peer is
admitted by the stream connect rule: §1.13's root resolution and symlink
refusal, then the leaf is pinned `O_PATH | O_NOFOLLOW` and the datagram is
sent to that pinned descriptor's `/proc/self/fd` spelling, which is the
destination only and is never observable. A peer whose family differs
from the socket's bound family is `IOError.InvalidInput`. A datagram is
atomic: the whole effective span is sent as one datagram or an error is
returned, so the result is `Unit`, not `WriteProgress`. The payload
maximum is 65507 bytes for every family, the IPv4 UDP maximum applied
uniformly; a larger effective span is `IOError.InvalidInput`. An empty
effective span is `ResourceError.InvalidBounds` by the shared buffer
check; sending an empty datagram is outside this slice. Every refusal in
this paragraph precedes any syscall. The send uses `MSG_NOSIGNAL`; other
OS failures map through the host's canonical identity mapping to
`IOError.Other errno`, as for streams.

**Receive.** `receive_datagram` blocks for one datagram, received with
`recvfrom` and `MSG_TRUNC` so the host learns its true length. The
delivered bytes are the prefix that fits the effective window, returned
as `ReceivedDatagram` with the live span and `TransferCount` (delivered,
effective). Its fit is `TruncatedDatagram` exactly when the true length
exceeds the effective window, and `WholeDatagram` otherwise; the kernel
discards the excess. Truncation is a typed field of a successful receipt,
not an error, because the datagram was consumed and its prefix delivered.
A received zero-length datagram is `EmptyDatagram` with its sender. A zero
effective window is `ResourceError.InvalidBounds` without a host call; it
cannot be `ReadEof`, because a datagram socket has no end of stream.
Readiness and nonblocking receive are PX12.

**Local address.** `datagram_local_address` returns the OS-assigned port
for an IP bind to port zero, and for a Unix socket the validated address
it was bound with.

**Sender identity.** An IPv4 sender is `Some` of its address and port. An
IPv6 sender is `Some` with its flowinfo dropped, because flowinfo is not
part of the address identity; a sender with a nonzero scope ID is `None`,
because this slice's `SocketAddress` has no scope. For Unix, the host keeps
a per-process table from each live datagram socket's exact kernel-recorded
name to the validated `SocketAddress` it was bound with; an entry is added
at bind and removed at release. A received Unix sender name equal to a live
entry is reported as that validated address. Otherwise a sender name that
validates as a `SocketAddress` Unix path (absolute, 2–107 bytes, no zero
byte) is reported as that path. An unnamed sender, an abstract name or a
name that does not validate is `None`.

The residual: if the socket file of a live entry is unlinked by a separate
FS act and another process binds the same location through this process's
`/proc` spelling, its datagrams report this socket's validated address.
That address still names the location where the sender is bound.

**Kinds and errors.** A stream or listener resource passed where a
datagram resource is required is excluded statically by the kind index; at
the host it is the existing `ResourceKindMismatch`, and a released socket
is the existing `Closed`. `Capability.System.Error`'s `SocketOperation`
gains one constructor, `BindSocket`, last in both the data declaration and
the export list, and non-idempotent. Send, receive, local address and close
reuse `SendSocket`, `ReceiveSocket`, `LocalSocketAddress` (idempotent) and
`CloseSocket`. `SocketResource` carries the bound address for bind,
receive and local address, and the peer for send. No new `IOError`
constructor is added.

**Native refusal.** A native build of a program that uses a datagram
operation fails at build time with the existing refusal: `N0001`,
`LoweringCause::Fragment`, reason
`effect <owner>::FSOp.<decimal code> is a represented unavailable lane`,
where the code is one of 1800–1803. Which of the four is reached first is
incidental to source order. The four operations are absent from the native
host-effect consumer roster, the native ABI refuses them at run time, and
its request decoder has no arm for them; no datagram code is emitted. A
stream-only program still builds and runs natively. No new kernel rule or
trusted postulate is authorized.

### 1.14 Bracketed directory batches (ABI-S2)

> **ABI-S2 status: contract specified.** Architect AC-0 `evt_1d1tqakxsz753`
> and its amendment `evt_1gs7gppsm39mj` are based on `5bc385c1c`.
> The Architect reported the exact-base retirement sweep and found that
> public `withResource` forwards its mode to `PrivateFsOpen` and passes a
> raw `FsHandle` to its body. The amendment therefore replaces the proposed
> mode extension with a private acquisition and uses the existing host
> rights check as a second boundary. This is design evidence, not an
> assembled run, official conformance, CI or landing.

#### 1.14.1 Acquisition and isolation

`withDirectory` is an explicit import from `Capability.Filesystem.Authority`.
`readDirectoryBatch` is an explicit import from `Capability.System.IO` beside
`readAt`. Their checked signatures are:

```text
proc withDirectory (a : Auth) (e : Type) (r : Type) (cap : Cap a) (path : Bytes)
  (body : DirectoryHandle -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result FileError (ResourceBracketResult e r)) visits [FS]

proc readDirectoryBatch (a : Auth) (dir : DirectoryHandle)
  : HostIO a (Result ResourceError DirectoryProgress) visits [FS]
```

`DirectoryHandle` is a scoped floor type wrapping
`Resource ResourceKind.FsHandle`. Its `PrivateDirectoryHandle` constructor
and raw-resource projection are private under both bare and qualified paths.
`withDirectory` is the only source acquisition route. It uses the private
`PrivateFsOpenDirectory` checked operation with the capability and path,
not `withResource`. The after-open step is also private. Acquisition failure
skips the body; success passes the wrapped handle to the body and uses the
existing resource bracket settlement and release rules. Copies refer to the
same acquisition and do not survive settlement as live handles.

`ResourceOpenMode`, its three constructors and the ordinary `FsOpen` ABI
are unchanged. No public mode or other public selector opens a directory
through generic `withResource`. There is no new `ResourceKind`.

The distinct `FsOpenDirectory` host operation requires `Enumerate` at
`AUTH_PARTIAL`, and opens the directory under the existing path resolution
and confinement rules. Directory open errors carry
`FileOperation.OpReadDirectory` and `Some path`. In both the interpreter
and native lanes, opening a regular file this way returns
`MkFileError FileOperation.OpReadDirectory (Some path) IOError.NotDirectory`.
The tag comes from the operation, not an open-mode value.

The host grants exactly `ENUMERATE` to the directory `FsHandle`. Batch reads
require that right on the held handle; they take no new capability argument.
Independently of source typing, only `FsReadDirectoryBatch` and
`ResourceRelease` may accept `ENUMERATE` alone. The host refuses other
`FsHandle` operations on a directory handle with `ResourceError.RightNotHeld`.
A batch read on any other `FsHandle` refuses with that same error. The
host reports both the required and held rights:

| Operation and handle | Exact host error |
|---|---|
| `FsReadAt` on a directory handle holding only `ENUMERATE` | `RightNotHeld { required: READ, held: ENUMERATE }` |
| `FsReadDirectoryBatch` (the host operation behind `readDirectoryBatch`) on an ordinary `Read` handle holding only `READ` | `RightNotHeld { required: ENUMERATE, held: READ }` |

No other source operation can advance or reset the directory stream position.

#### 1.14.2 Batches and end

`DirectoryProgress` is a scoped floor family with precisely two public
cases: `DirectoryProgress.DirectoryEntries (List DirEntry)` and
`DirectoryProgress.DirectoryEnd`. `PrivateFsReadDirectoryBatch` is a private
operation on the wrapped raw handle. Its response is
`Result ResourceError DirectoryProgress`.

Both lanes use the same host primitive on the held descriptor. Each
`DirectoryEntries` batch contains the filtered entries from exactly one
kernel `getdents64` fill, of at most 32768 bytes of kernel records. The bound
is on the records in a fill, not the directory's size or an entry count.
The operation omits `.` and `..` and skips fills with no entries left after
filtering. Thus a batch has at least one entry. A zero-byte fill yields
`DirectoryEnd`, not an empty entries batch. End is sticky: once
`readDirectoryBatch` returns `DirectoryEnd`, every later admitted call on
that live handle returns `DirectoryEnd` without consulting the filesystem.
An error is not sticky: it does not mark the stream ended, and a later
admitted call may return entries, `DirectoryEnd`, or an error. Before End,
reading continues at the descriptor's position rather than reopening the
path.

Order is unspecified. If a traversal of an unmodified directory reaches
`DirectoryEnd` without any batch-call errors, its concatenated batches
contain every entry other than `.` and `..` exactly once. After an error,
a retry need not recover entries consumed before the failed call returned.
Entries created or removed during reading may or may not appear.

At the host boundary the batch reply discriminates entries from end
explicitly; a zero count in the entries case or any entries in the end case
is invalid. Native results retain `DIRECTORY_LIST` for the list inside
`DirectoryEntries`, reusing the existing `DirEntry` and `FileKind` carriers.
The end case is nullary. Host failures use the existing `ResourceError`
family, not an end marker.

The whole-list `read_directory` source operation, its `FSOp` constructor
`ReadDirectory`, and the host operation `FsReadDirectory` (`0x0305`) are
retired in this same change. Neither new host operation reuses `0x0305`.
There is no compatibility alias or bounded whole-list replacement.
`FileOperation.OpReadDirectory` remains as the directory-open error tag.

### 1.15 Monotonic clock, deadline sleep and kernel entropy

> **Status: contract-pinned (RT-CLOCK-OP-NATIVE-PROMOTION).** Architect
> AC-0 `evt_4zbmaff96r9wv` rules the contract on `b9c1853d6`;
> implementation and assembled evidence are separate.

In Strict source, import `monotonic_now` and `sleep_until` from
`Capability.Time.Monotonic`, `EntropyIO` and `random_bytes` from
`Capability.System.Entropy`, and `host_entropy` from
`Capability.Process.Exit`, which also homes `host_clock`. These names
are not ambient floor bindings. Clock operations visit `Clock`, and
`random_bytes` visits `Entropy`; neither effect grants FS authority.

`monotonic_now` returns a `MonotonicInstant` containing the `Int` count of
nanoseconds elapsed since a process-wide baseline. Readings are nonnegative
and do not decrease within a process. The epoch is otherwise unspecified:
a reading or `Deadline` has no meaning outside its originating process.
Programs should derive deadlines from readings in that process. These
are host properties, not Ken theorems about external readings.

The floor admits the exact checked types `MonotonicInstant` and `Deadline`.
`MonotonicInstant.MkMonotonicInstant` wraps an `Int`, and
`Deadline.MkDeadline` wraps a `MonotonicInstant`. Only the qualified
constructors resolve at those checked identities. The native monotonic
reply selects the exact checked instant constructor; native sleep reads
both exact checked constructors, not their spellings.

`sleep_until` takes a `Deadline`. Its nanosecond operand `d` must satisfy
`0 ≤ d < 2^64`. With an admitted operand, sleep returns `Unit` only once
that same clock reads at or past `d`. An already reached deadline returns
without suspending. Sleep is uncancellable; cancellation is left to PX12.
Outside the domain, both lanes refuse the effect with the same terminal
class and no `ClockSleepUntil` trace event. There is no successful return
for an out-of-domain deadline.

`random_bytes` takes an `Int` count `n` and returns `Result IOError Bytes`.
For `0 ≤ n ≤ MAX_ENTROPY_REQUEST_BYTES_V1` (1024 bytes), success returns
exactly `n` bytes from the kernel CSPRNG, including empty `Bytes` for
zero. A negative or over-bound count returns `Err` with
`IOError.InvalidInput`, rather than truncating or supplying bytes. For a
negative count, both lanes must record the same event or neither must
record an event. If the kernel source cannot supply the full admitted
count, the result is `Err` with `IOError.Unsupported` and no bytes. There is
no fallback, substitute userspace PRNG, or retry on another source.
Returned bytes are runtime observations, not a proof of unpredictability
or a broader cryptographic guarantee.

## 2. The FFI surface — the `foreign` declaration

A **`foreign`** declaration binds a Ken name to an external (C-ABI) symbol,
giving it a **Ken type** and a **declared effect row**. It is the one surface
form that crosses out of the checked language, so it is built to make that
crossing **typed, effect-rowed, and visible** — never an untyped escape hatch.

```
foreign c_sqrt : Float → Float
  = symbol "sqrt"  library "m"  pure

foreign os_write : Int32 → Bytes → Int  visits [FS]
  = symbol "write"  library "c"
```

### 2.1 The declaration form (grammar + elaboration entry)

The `foreign` form extends the V0/`33` declaration grammar:

```
foreign-decl ::= "foreign" ident ":" type effect-row? "=" foreign-body
foreign-body ::= "symbol" string ("library" string)? "pure"?
effect-row   ::= "visits" "[" label ("," label)* "]"      -- 36 §1
```

- A `foreign` decl carries four things: a Ken **type** `T`, a **declared effect
  row** `ρ` (`visits [...]`; `pure` is the surface spelling of the **empty row**
  `ρ = ∅`), a C **symbol** name, and an optional **library**.
- `pure` and a non-empty `visits ρ` are **mutually exclusive**: `pure ≡ visits
  []`. A decl with neither defaults to `pure` (empty row) — and is then subject
  to §3.4 (a world-touching symbol with no row is the named residual, not a
  silent pass).
- **Spelling deferral (`OQ-syntax`, defer-spelling-not-concept).** The exact
  surface tokens (`foreign`/`symbol`/`library`/`pure`/`visits`) are reserved
  keywords whose literal spelling the build team finalizes; what is **locked
  here** is the **structure** (type + row + symbol + library), the **type+row
  obligation**, and the **postulate wiring** (§2.3). Conformance pins the
  structure and `(oracle)`-tags the literal keyword (assert-at-locked-
  granularity).

### 2.2 C-ABI marshalling (reuses L6)

A call to a `foreign` marshals Ken values to/from their C-ABI representations
following the primitive lowering (`41 §1`):

| Ken type | C-ABI representation | Source |
|---|---|---|
| scalars `Int*`/`UInt*`/`Float*`/`Bool`/`Char` | the named machine type (`i64`/`f64`/`i1`/`u32`/…) | `41 §1` typed immediates |
| `Bytes` | **`(ptr, len)`** — a pointer + length pair | L6 `§1.1`, `41 §1` |
| handle / `section` | runtime-defined opaque handle | `41 §1`, `44` |

- `Bytes` ↔ `(ptr, len)` is the **L6 boundary** (`§1.1`) — L7 **rides it, does
  not re-derive** (guardrail). L6 keeps that boundary clean and explicit
  precisely so this marshalling needs no rework.
- Marshalling is a **runtime-lowering** concern (`41`), **not a kernel rule**:
  the kernel sees only the `foreign`'s assumed Ken type (§2.3); the (ptr,len) /
  scalar lowering happens at the call in the runtime, below the trusted
  boundary.
- The FFI is **general** — any C-ABI symbol — **not** a fixed allowlist of
  externals. Generality is safe because every use is an explicit, *listed*
  concession (§3).

### 2.3 Elaboration — a `foreign` is a postulate

A `foreign f : T visits ρ = symbol "…"` elaborates to a **postulate** of the
rowed Ken type. The defensive elaboration entry:

```
elabForeign(Σ, ⟨ foreign f : T visits ρ = symbol s library l [pure] ⟩):
  T'  := elabType(·, rowed(T, ρ))      -- the latent-row type A →[ρ] B (36 §1); pure ⇒ ρ=∅
  id  := declare_postulate(Σ, f, [], T')  -- required label = foreign declaration name
  recordForeign(id, symbol = s, library = l, row = ρ)   -- elaborator-side link/marshalling record
  bind f ↦ id
```

- The kernel admits `f` as an **opaque constant** `f : T'` (`11 §4`: an opaque
  constant blocks δ and "is how axioms, **FFI signatures**, and abstract
  interfaces are represented") — there is **no body to unfold**, and it is
  **recorded in `trusted_base()`** (`18 §4.2`, the `declare_postulate` contract:
  "`id` admitted opaque; recorded in the trusted base").
- **No new kernel rule** (guardrail). `declare_postulate` is the **existing**
  kernel API (`18 §4.1`); the type+row is ordinary `36 §1` machinery; the
  symbol/library/marshalling is an **elaborator-side** record consulted by the
  runtime, never by the kernel. L7 adds **zero** kernel formation rules (the
  level-discipline reconcile is therefore trivially a pass — no new universe or
  former appears).

## 3. The trust boundary (the load-bearing part)

FFI is where Ken's guarantees **stop**. The discipline marks that frontier
**honestly and structurally**: every foreign is assumed, *listed*, effect-rowed,
and capability-gated — and the one soundness gap the type system cannot
mechanically close is **named** (§3.4), never hidden.

### 3.1 Foreign-as-listed-postulate → the trusted base (the honesty headline)

This is the load-bearing AC2 mechanism — the honesty-about-the-boundary
principle made structural.

- A `foreign` postulate (§2.3) is in `trusted_base()` (`18 §5`).
- A verified **artifact's** `trusted_base_delta` (`25 §3`) is the set of
  `trusted_base()` postulates its checked content **transitively depends on** —
  its **dependency cone**. The B1 export (`71`) computes it from
  `trusted_base()` membership over the artifact's verified content.
- **Resolve the silence — "relies on" is by *use*, not *declaration*.** A
  `foreign` decl in scope is **not** itself a reliance; what lists `os_write` in
  an artifact's delta is a verified definition that **calls** `os_write`. A
  `proc` that calls it has `os_write` in its delta; a `proc` that does not is
  **absent** from it — even with the `foreign` decl visible in the same module.
  The honesty property ranges over **the foreign functions the verified content
  reaches**, exactly as `18 §5` reads `trusted_base()` ("the assumptions a given
  program *depends on*") and `71 §2.1` reads the boundary ("which foreign
  functions it *relies on*"). (Pinning this at the source so the conformance
  author nets reliance-by-call, not the vacuous decl-lists-itself.)
- The B1 export projects every `trusted_base_delta` postulate into the **`P`
  (assumptions)** field, tagged `tested`, under the **FFI boundary-label** case
  (`71 §2.1` names "FFI" among the boundary labels feeding `P`; invariant I2,
  `71 §3.1`). So **a verified artifact's exported contract lists exactly the
  foreign functions it relies on — visible, not hidden.**
- **AC2 flips on dependency:** relied-on ⇒ the foreign's postulate is a `P`
  entry; not-relied-on ⇒ absent (and the export hash changes, `71 §3.3`).
  Conformance routes a **real** `foreign` decl through the **real** B1 export
  and observes the foreign in `P` — never a synthetic trust-base literal. This
  is the **FFI instance** of B1's assumption-visibility (subsumes
  `export/removing-assume-shrinks-P`, `71 §3.1` I2 — subsume-don't-proliferate).

### 3.2 `pure` is a claim, not a check — projects to *trusted*, never `Q`

The kernel **cannot check C**. A `foreign`'s type — including a `pure`
(empty-row) claim of referential transparency — is **assumed**, never verified;
it is part of the trusted boundary, and is the AC3 mechanism.

- **The no-over-claim projection (`71` I1).** A `foreign`'s assumed guarantee is
  a **postulate** of its type, so its goal **is** a member of `trusted_base()`.
  By the structural discriminator (`21 §5.4`, `71 §2.1`: a claim is `Q` **iff**
  its certificate `check`s **and** its goal is **not** a `trusted_base()`
  postulate) it can **never** project to `Q`. A `pure foreign`'s guarantee lands
  in **`P`/trusted** (`tested`), **never** kernel-certified `Q`. **Under-claim
  is the safe direction**: filing the assumed claim as `Q` would over-claim a
  kernel certification that does not exist.
- This is the FFI instance of the **trusted-by-typing-is-not-`Q`** discipline
  (cf. Sec1's declassify edge, B1's I1): a guarantee resting on a **trusted
  meta-claim** ("the C symbol is referentially transparent") is **not
  kernel-proved**, so it projects to *trusted*, not `Q`.
- A **wrong `pure`** is a soundness bug **confined to that postulate — and it is
  *listed*** (§3.1): even an incorrect purity claim cannot masquerade as proved,
  and the assumption itself is visible in `P`.
- **Discriminating, not green-vs-green.** *Nothing* about a foreign is ever in
  `Q`, so "the foreign is absent from `Q`" alone is vacuous. The net is the
  **pair on the same artifact**: a genuinely kernel-proved Ken-side
  postcondition (an `fn` wrapping the foreign, whose `ensures` over the
  *marshalling* — not the C body — discharges) projects to **`Q`** **while** the
  foreign's assumed `pure`/type claim projects to **`P`** — the field tracks
  **kernel-provedness**, not the `pure` keyword. The bug (trust the `pure`
  annotation, bucket it as a guarantee) lands the assumption in `Q` → red.
  (Reuses B1 EX-A's proved↔assumed pair, here
  `kernel-proved`↔`foreign-assumed`.)

### 3.3 Boundary contracts → runtime-checked assertions

A `requires`/`ensures` on a `foreign` is a proposition over an **opaque**
postulate (the AC4 mechanism).

- Where it is **statically unprovable** (the kernel has no body to reason
  about), it lowers to the **`tested`** epistemic status (`21 §5.2`): a
  **runtime-checked, fail-fast assertion** at the call boundary, **plus** a
  visible `P`/`tested` entry in the assumption boundary. This is **the** place
  runtime contracts earn their keep (`21 §5.2` names exactly "boundaries, FFI,
  untrusted input"): an unverifiable boundary becomes a **fail-fast** one rather
  than a silently-assumed one.
- **Structural, not "accepts."** Conformance observes the **emitted runtime
  check** (the lowered assertion exists at the call site) + the `tested` `P`
  entry — never merely that the program compiles (the discriminating-output
  discipline). A `foreign` `ensures` that **is** statically discharged (proved
  over the Ken-side marshalling) needs no runtime check and may reach `Q`
  (§3.2); the **unprovable** one **must** emit the runtime check — the two faces
  flip on static provability.

### 3.4 Effects are mandatory — and the one residual the discipline cannot catch

The AC5 mechanism: a catchable flip plus the single named gap (the honest limit,
`64`).

- **The catchable flip (the net).** A world-touching `foreign` carries its
  effect row (§2.1). A caller that performs the foreign's effect **without**
  declaring the matching row in its own signature is an **EFFECT-ESCAPE** static
  error (`36 §1.4`: `ρ_inf ⊄ ρ_decl`) — the **same** escape check L6 I/O rides
  (`§1.3`), **not** a new gate. A `proc` calling `os_write` (`visits [FS]`)
  without `[FS]` in its declared row is **rejected**; with `[FS]`, it
  **accepts** — a **verdict flip** on a **real** foreign through the **real**
  escape check.
- **The residual (the honest limit).** A `foreign` declared **`pure`** (empty
  row) whose C symbol **actually performs I/O** is the **one** gap the type
  discipline **cannot mechanically catch**: the kernel sees an empty row, no
  caller is forced to declare an effect, and the real I/O is invisible to
  typing. This is a **reviewer-surfaced flag**, **not** a verdict the type
  system flips. It is **mitigated, not eliminated**, by §3.1 — the mis-declared
  foreign is still a *listed* postulate, so the wrong claim is at least
  **visible** in `P`. Name it as the residual; **never** silently treat a
  `pure`-but-effectful foreign as sound (the conformance case asserts it is
  *flagged*, not accepted as a netted verdict).

## 4. Capabilities and least authority (couples Sec2)

A `foreign` world-action requires **two independent concessions**, and dropping
**either** rejects (the AC6 composition with Sec1/Sec2):

1. its **effect row** (`36 §1.4`, §3.4) — *may this code perform this effect*;
   and
2. the **gating capability** — a `Cap_FS`/`Cap_Net`/… token passed at the call
   (`using c : Cap E`, `36 §3`; the authority face, Sec2 `62`) — *is this code
   authorized to perform it*.

- The capability gate is **Sec2's** discipline (`62`): no ambient authority,
  least by default, the cap a real Π value the call needs (`36 §2.5`). A
  `foreign os_write` call in a context holding **no** `Cap_FS` is rejected
  **even if** its `[FS]` row is declared (the authority is missing); a context
  with `Cap_FS` but **no** `[FS]` row is rejected by the escape check (§3.4).
  **Both** are required — the composition keeps the unverified surface of any
  program **explicit and minimal**.
- **The trusted boundary is a small, enumerated set** (guardrail): the verified
  core stays pure; every FFI use is a *listed* foreign postulate (§3.1) **plus**
  an explicit capability — never an ambient escape.

## 5. Honest limits — kernel-checked vs trusted vs the named residual

Where Ken's guarantee sits, per boundary fact — the honest accounting (`64`):

| Boundary fact | Status | Netted by |
|---|---|---|
| The foreign's **Ken type** (arg/result shape, effect row) | **assumed** (postulate, `11 §4`) | listed in `trusted_base_delta` → `P` (§3.1) |
| A **statically-proved** Ken-side contract over the marshalling | **`Q`** (kernel-certified) | kernel `check` (`18 §4.5`) |
| A **statically-unprovable** `requires`/`ensures` on the foreign | **`tested`** | runtime-checked assertion (§3.3) + `P` |
| The **`pure`** / referential-transparency claim | **trusted**, never `Q` (§3.2) | listed; never kernel-certified |
| The **effect row** on the foreign, at call sites | **enforced** | `36 §1.4` escape check (§3.4) — the flip |
| A **`pure`-but-effectful** foreign | **the residual** | reviewer flag only (§3.4) — listed, not caught |

The single honest gap is the last row: the discipline **names** it but cannot
mechanically close it. Everything else is either kernel-checked (`Q`),
runtime-checked (`tested`), or **listed-as-assumed** (`P`) — never hidden. The
goal of the whole chapter is that the unverified surface of any program is
**small, explicit, and enumerable** — not zero (FFI is necessary), but *honest*.

## 6. What WS-L must deliver here (L6, L7)

This WS-L accounting covers §1.1–§1.6 and §2–§5. The additive §1.7
contract is PX8-R/PX8-F work and is not represented here as an L6/L7 delivery.

`Bytes` + binary I/O (effect-tracked, encode/decode to `String`) — **elaborated
in §1.1–§1.6, the L6 deliverable**; lawful, derivable serialization with a
provable round-trip (the **law/interface in §1.5**; the generic derivation is
L8). And
(**L7**, §2–§5): a **general** `foreign` FFI with typed/effect-rowed bindings
and C-ABI marshalling (§2); the trust-boundary discipline — foreign-as-listed-
postulate (§3.1), `pure`-as-claim-not-`Q` (§3.2), runtime contracts at the edge
(§3.3), mandatory effects + the named residual (§3.4); and capability gating
(§4). **No new kernel rule** (§2.3).

**L7 acceptance (AC1–AC5; ties to G6** — ≥1 FFI call in a verified component,
with the trust base showing exactly what is assumed):

- **AC1 — `foreign` binds + marshals.** A `foreign` decl elaborates to a typed,
  effect-rowed postulate (§2.3); a call marshals `Bytes`↔`(ptr,len)` + scalars↔
  machine types (§2.2) — a **structural** assertion on the binding/marshalling,
  not "compiles".
- **AC2 — foreign-as-listed-postulate (honesty headline, discriminating).** An
  artifact that **relies on** `foreign f` (calls it) has `f`'s postulate in its
  `trusted_base_delta` → `P`; one that does **not** is **absent** (§3.1). Routed
  through the **real** B1 export; the verdict flips on **dependency**.
- **AC3 — `pure` projects to *trusted*, never `Q`.** A `pure foreign`'s
  guarantee is assumed → `P`/`tested`, **never** `Q` (§3.2); under-claim is the
  safe direction. A genuinely-proved Ken-side claim in the *same* artifact
  reaches `Q` — the field tracks kernel-provedness, not the `pure` keyword.
- **AC4 — boundary contracts runtime-checked.** A statically-unprovable
  `requires`/`ensures` on a `foreign` lowers to a **runtime-checked** fail-fast
  assertion + a `tested` `P` entry (§3.3) — observe the **emitted** check, not a
  silent assumption.
- **AC5 — effects mandatory (discriminating).** A world-touching `foreign` whose
  effect escapes the caller's declared row is **rejected** (`36 §1.4`); the
  properly-rowed call **accepts** — a verdict flip (§3.4). The **`pure`-but-
  effectful** case is the **named residual** (flagged, not silently accepted as
  sound).

Conformance:
- `../../conformance/surface/bytes-io/` — the L6 `Bytes`/I/O/encode-decode/
  round-trip cases (`§1.6`).
- `../../conformance/surface/ffi-io/` — (L7) AC1–AC5 with per-case verdict/
  structural flip, the **AC2 honesty pair** (relied-on listed / not-relied-on
  absent, through the real export) and the **AC3 pure-not-`Q` pair**, the **AC5
  effects-mandatory flip + the named residual**, the **capability+effect
  composition** (`§4`), and a **G6 serialization round-trip proof** in an
  FFI-using verified component (a `foreign` decl whose postulate shows up in
  `trusted_base_delta`). **QA gate:** AC2/AC3 route a **real** `foreign` through
  the **actual** export/trust-base machinery (postulate listed, never `Q`); no
  synthetic trust-base literal.
