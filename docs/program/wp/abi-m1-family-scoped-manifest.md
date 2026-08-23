# WP — ABI-M1: manifest v2, family-scoped, versioned, generated from family schemas

Runtime priority-1 lane (Linux ABI completion program). One WP, one branch
`wp/ABI-M1-family-scoped-manifest`, one PR. Owner: runtime. Size: L. Gate: none.
Depends on: ABI-R3 (merged). Blocks: ABI-M2, ABI-S4, PX10, PX11.

Source: `docs/program/10-linux-abi-completion.md` Track M (`:135-142`), the §5
dependency graph (`ABI_R3 -> ABI_M1 -> {ABI_M2, ABI_S4, PX10, PX11}`), and the
campaign's AC discipline (`:316-317`).

Fixed inputs measured at `origin/main a12f74158`.

## Objective

Convert the flat, single-family `TargetAbi` manifest into a family-scoped,
versioned manifest generated from per-family schemas — applying the same
structure-derived discipline ABI-R3 applied to the operation enum, now to the
ABI-fact manifest. A new family, a new fact, or a new facility-ABI-version must
be a build break, never a silent omission. Native-target only.

Concretely, the manifest gains: per-family grouping of constants and record
layouts, an expanded target-identity family (arch, pointer width, endianness, C
scalar widths and alignments), a facility-ABI-version per enabled family, and a
canonical projection hash per family that composes into the whole-manifest hash.

## Fixed inputs (SETTLED, at `origin/main a12f74158`)

1. The v1 manifest type + fail-closed assertion:
   `crates/ken-host/src/lib.rs` — `AbiFact { name, value: u64 }` (`:53-57`),
   `TargetAbi` (`:59-69`), `assert_target_abi_identity` fail-closed on
   backend-unavailable or hash mismatch (`:92-100`).
2. The generator: `crates/ken-host/build.rs` — `canonical_manifest` (`:434`),
   `linux_raw_facts` (`:346-385`), the C-header cross-check `run_probe` over
   `abi_probe.c`, fail-closed (`:402-432`), `TARGET_ABI` /
   `TARGET_ABI_MANIFEST_HASH` emit (`:474-475`), `SCHEMA_VERSION = 1` (`:9`),
   canonical serialization + SHA-256 (`:84`, `:258`), non-Linux/cross target
   records unavailable backend + no facts (`:70-77`).
3. The ABI-R3 derive precedent to mirror: `crates/ken-host/src/effect_v1.rs` —
   `HostOpV1::next_in_inventory` exhaustive match, no wildcard, omission is
   `error[E0004]` (`:100-130`); `COUNT` / `ALL` derived by walking the chain
   (`:63-96`); `availability` / `is_ambient` exhaustive per-op (`:138-200`).
4. The generated catalog + partition: `crates/ken-host/effect_abi_v1.catalog`;
   `build.rs` parse (`parse_effect_catalog` `:110+`, `HOST_EFFECT_ABI_V1_CATALOG`
   / `HOST_EFFECT_ABI_V1_HASH` `:238`, `:286`); the ken-verify partition test
   `imported_catalog_partition_is_exact_and_closed` in
   `crates/ken-verify/src/catalog.rs`.
5. Current fact families in `linux_raw_facts` (`build.rs:346-385`), all in one
   flat list under one whole-manifest hash, no per-family grouping / version /
   projection hash: target-identity/scalar (`POINTER_WIDTH`, `C_INT_WIDTH`),
   `OFlags`, `AtFlags`, `Mode`, `*at` syscall numbers, errno.
6. The native artifact binds the manifest hash:
   `crates/ken-runtime/src/target_abi.rs`.
7. Program scope: `10-linux-abi-completion.md` — "manifest v2, native-target
   only" (`:135-142`), the v2 definition "family-scoped and generated, not
   cross-target" (`:90-93`), the AC discipline "state the property, its closure
   axes, and a loud failure on the unhandled case" (`:316-317`), tracked !=
   releasable (`:357-361`).

STALE TEXT — do NOT trust (verified against the tree, they contradict it):
- `docs/program/wp/ABI-R3-derived-operation-inventory.md:12-15` still says
  "shovel-ready / blocked depends_on [PX8]". ABI-R3 is MERGED; the issue node
  and git history (`8abe427f3`, `2a1a17104`, edge dropped `5d6da33ce`) are
  authoritative.
- `docs/program/wp/px3-abi-scalars.md:3-4,31-34` claims the manifest "carries no
  width fact yet". Width facts (`POINTER_WIDTH`, `C_INT_WIDTH`) have landed
  (`build.rs:349-350`).

## Design judgment (front-loaded)

Pinned by this frame:

- DISCIPLINE. Family schemas drive generation. The set of families, and the
  facts / record layouts per family, are derived from a single structure (a
  schema table / enum) so that adding a family or a fact without threading it is
  a compile error — the ABI-R3 `next_in_inventory` pattern applied to the
  manifest. No handwritten growing list survives.
- REUSE, do not re-cut. Extend PX2's `TargetAbi` and the `build.rs` generator in
  place. Keep `run_probe` / `abi_probe.c` and its fail-closed cross-check. Bump
  `SCHEMA_VERSION` 1 -> 2. This is subsumption of the v1 manifest, not a parallel
  generator.
- HASHING. Each enabled family projects its own canonical hash; the whole-
  manifest hash composes the per-family projections. Target identity becomes its
  own target-identity family, expanded to arch / pointer width / endianness / C
  scalar widths + alignments.
- SOURCED-FROM-SOURCE. Every fact a header-projected family carries must be
  cross-checked by the C-header probe (fail-closed). Do not invent an ABI fact
  the probe cannot confirm.

Routed, NOT decided here:

- OPEN DESIGN FORK -> Architect (D0 return-condition). The concrete family-schema
  REPRESENTATION — how families and their facts / layouts are declared so
  generation stays structure-derived and a new one is a build break — is a
  component-design call. D0 probes whether it is cleanly expressible extending
  `TargetAbi`; if it hits a representability wall, D0 returns to the Architect
  with a symptom inventory rather than inventing a representation (the FO-D0 /
  M6-Case-C pattern). Named reviewer: Architect, consistent with ABI-R3.
- FOUNDATION PARTICIPATION (named per the issue's §2 note). Foundation is
  consulted for the family taxonomy (which families exist and their fact / layout
  membership) and for the ken-verify catalog-partition impact. Foundation is a
  reviewer, not a blocking dependency: the buildable work is runtime-crate-only
  (`ken-host` + `ken-verify`), so it does NOT depend on Foundation's module/
  import block. Surfacing ABI facts into `library/` (Ken level) is OUT of this
  WP (deferred; "native-target only").

## Deliverables (staged; each targets a releasable increment or a hard stop)

- D0 — representation / buildability probe. The family-schema structure
  elaborates extending `TargetAbi`; a family or fact omission is a build error;
  a whole-manifest hash composed from per-family projection hashes is
  expressible. Commit a probe test. No change to the emitted v1 fact values yet.
  RETURN-FORK to the Architect if the representation is not cleanly expressible.
- D1 — manifest v2. Refactor `linux_raw_facts` into per-family schemas; expand
  the target-identity family (arch, endianness, scalar widths + alignments); add
  a facility-ABI-version per family; add per-family projection hashes composing
  into the manifest hash; `SCHEMA_VERSION` 1 -> 2. Keep the C-header cross-check
  fail-closed. Update the native-artifact hash binding
  (`ken-runtime/src/target_abi.rs`).
- D2 — closure ACs + cross-check. The build-break, per-family-hash, fail-closed
  cross-check, and ken-verify partition controls below, made to pass / extended.
- D3 — honest reach (only if D1/D2 leave a bounded residual; state it, do not
  pad).

## Acceptance criteria (property + closure axis + loud failure)

- AC-1. Adding a family or a fact to the schema without threading it through the
  generator is a COMPILE error (E0004-class), not a silent omission — mirrors
  ABI-R3. Control: a probe that omits a family fails to build.
- AC-2. Every enabled family projects a canonical hash; mutating one family's
  fact flips exactly that family's projection hash and the composed manifest
  hash, and no other family's projection hash. Control: mutate one family fact,
  observe only that projection + the top hash change.
- AC-3. The C-header cross-check stays fail-closed for every header-projected
  family; a header/manifest disagreement aborts the build. Control: perturb the
  probe, build aborts.
- AC-4. `SCHEMA_VERSION == 2`; the native artifact binds the v2 manifest hash; a
  hash mismatch fails closed at `assert_target_abi_identity`. Control: a
  stale-hash test reds.
- AC-5. The ken-verify catalog partition
  (`imported_catalog_partition_is_exact_and_closed`) stays exact and closed over
  the v2 manifest. Control: the existing partition test passes / is extended.
- AC-6. Non-Linux / cross targets still record an unavailable backend and emit
  no facts — no cross-target generation is added. Control: the existing
  cross-target guard.
- AC-7 (designed return). If the family-schema representation is not cleanly
  expressible extending `TargetAbi`, D0 returns to the Architect with the
  symptom inventory rather than inventing a representation. This is a valid
  terminal outcome, not a failure.

TCB: zero trusted_base / TCB delta. This is runtime host-manifest machinery — no
spec, conformance, checker, reference, or kernel/SCT path is edited. Confirm no
trust path in review.

## Contention

Touches `crates/ken-host` (`build.rs`, `lib.rs`, `effect_abi_v1.catalog`;
`effect_v1.rs` is a read-only precedent, not edited), `crates/ken-verify/src/
catalog.rs`, and `crates/ken-runtime/src/target_abi.rs` (the hash binding).

- Off the M6 / PX8 critical path: M6 touches `ken-runtime` lowering / aggregates,
  a different area; and M6 is enclave-blocked and idle now. Low contention.
- Lane-2 language edits `ken-elaborator` / lexer / parser — different crates.
- NOT contention-free with any other `ken-host` toucher; the runtime ring has no
  parallel `ken-host` work in flight, so coordinate a merge window only if that
  changes.

## Capability tier (§4h)

T1-leaning: structure-derived-schema design plus the soundness of per-family
hashing, built on two strong landed precedents (ABI-R3 derive discipline, PX2
generator). Runtime-implementer (Opus5) is T1-fit; neither over- nor
under-provisioned.

## Reviewers

Architect (soundness / interface + D0 return-fork target), runtime-QA
(differential manifest + closure controls), Foundation (family-taxonomy
consult).

## Do-not guards

- Do NOT re-cut the manifest generator or the C-header cross-check; extend in
  place.
- Do NOT add cross-target generation, signed / content-addressed manifests, or
  CI native-builder matrices (`10-...md:141-142`).
- Do NOT surface ABI facts into `library/` in this WP (deferred; keeps the WP
  clear of Foundation's module/import block).
- Do NOT hand-write a growing fact list; generation must be structure-derived.
- Do NOT invent an ABI fact the C-header probe cannot cross-check for a
  header-projected family (fail-closed, sourced-from-source).

## Sequencing (Steward-owned)

Release now to the idle runtime ring — M6 is enclave-blocked, and this is off the
M6 / PX8 critical path, so it keeps the ring productive without waiting on the
enclave. When the M6 denotation ruling lands, the runtime-leader sequences the
switch (finish the current ABI-M1 increment or hard-stop first). After ABI-M1
merges, ABI-M2, ABI-S4, and the PX10 / PX11 exit domains unblock on the M side.
