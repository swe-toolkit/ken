---
name: a-kernel-guarded-encoder-is-non-trusted-iff-the-composite-is-kernel-rechecked-against-an-independent-obligation-and-the-apparatus-lands-inert-with-zero-verdict-path-callers
description: >-
  An untrusted encoder that builds candidate proof terms is non-trusted only if
  the composite is kernel-checked against an obligation the encoder did not
  choose, and the load-bearing test is a faithful-vs-wrong pair that both pass
  the Rust checker and separate only at the kernel check. A soundness-bearing
  apparatus can land inert (zero verdict-path callers, the boundary still
  returns Unknown); verify that, then attack the later wiring landing with the
  four-condition checklist (Proved only via the kernel check, independent
  obligation on the live route, separation at the kernel, bare callers
  fail-closed). V3-FO 2026-09-04.
metadata:
  type: feedback
---

# A kernel-guarded encoder is non-trusted iff the composite is kernel-re-checked against an independent obligation, and the apparatus can land INERT

**Measured 2026-09-04 on `V3-FO-ROUTE-CONSUMPTION-APPARATUS`
`9b89a7436bb48f5279ba176dc1f073727acdaa65` (PR #3310, base `af443017f`),
verdict `evt_5p2n8mnrsa600` (thread `thr_7vb4452cn7m1y`).** Flagged by the
Steward/lieutenant as the soundness-bearing FO landing: "verify the encoder
faithfulness is genuinely kernel-checked, not merely asserted." Component B is a
Rust ↔ catalog encoder that builds candidate proof terms from an untrusted
encoding; the design claim is that the kernel is the sole authority. Provenance
clean, all 4 blobs byte-identical to reviewed `3486d3fd2`, +892/-7 in
`crates/ken-elaborator/` only.

## The shape: a "kernel-guarded encoder" landing

An untrusted layer (a Rust encoder + a Rust checker `check_cert`) produces
candidate terms; a kernel check is supposed to be the only thing that can turn
those into a `Proved`. The failure mode to hunt is a path where the untrusted
layer's acceptance becomes the verdict — i.e. the encoder is effectively
trusted. Do NOT clear this on the commit's word ("encoder not trusted, kernel
re-checks"). Prove it structurally against the landed blob.

## Tell 1 — the encoder is genuinely NOT trusted (two structural facts)

**(a) The composite is kernel-re-checked against an INDEPENDENT obligation.**
The composite-builder (`kernel_checked_fo_composite`) assembles the catalog's own
kernel-checked theorems (`fok_checker_soundness ∘ fok_embedding_adequacy`)
applied to the encoder's untrusted terms, then its sole authority is
`ken_kernel::check(env, &Context::new(), &composite, phi_closed).ok()?`.
`phi_closed` is a PASSED-IN parameter — on the route it is `triple.goal_closed`
(classified independently, `classify(env, &triple.goal_closed)`), NOT derived
from the encoder. So the check is exactly `fok_denote(encode(problem)) ≡
phi_closed`, and a Rust encoding mistake can cause only `None`, never a proof of
a lookalike proposition. Confirm `phi_closed` is not computed from the encoder's
output anywhere on the path.

**(b) The discriminator lives at the KERNEL check, not the Rust checker.** The
load-bearing test is a faithful-vs-wrong pair on a SHARED input: same source
form `f`, same certificate `cert`, differing only in the independently
interpreted atom environment (faithful predicate vs a wrong one). The pair is
non-degenerate iff BOTH arms pass the Rust `check_cert` AND the catalog
checker-validity prefix (`infer` on `checker_soundness` applied to the encoded
target+cert), and they separate ONLY at the final kernel composite check
(faithful -> `is_some` and `check(composite, phi_closed)` succeeds; wrong ->
`is_none`). This is the proof that the untrusted layer accepts both and the
kernel is what separates them. **If the wrong arm were rejected at `check_cert`,
the encoder/Rust-checker WOULD be the authority** — the whole claim would be
vacuous. Verify the wrong arm explicitly `assert!(check_cert(...))` passes and
the prefix `infer` passes, before the kernel `is_none`.

Corroborating Attack-3 teeth to confirm on this shape: fail-closed on a partial
catalog install (missing a theorem OR a single encoder constructor -> `resolve`
returns `None`, no partial/lookalike encoding); carried `GlobalId`s survive
source-name removal (the prover never re-looks-up by name); and a zero-trust
snapshot equality asserted on BOTH the faithful-accept and the wrong-reject arms
(neither path smuggles a `trusted_base` declaration).

## Tell 2 — a soundness-bearing apparatus can land INERT (verify it did)

The subtle part: this mechanism landed WITHOUT emitting any `Proved`. It is a
prerequisite; a later deliverable consumes it. A soundness-bearing landing that
changes no verdict is the safest possible shape — but only if you confirm the
inertness, because the same diff one wiring-line later is where the soundness
actually bites. Three structural confirmations:

1. **The composite-builder has ZERO production callers.** `git grep
   kernel_checked_fo_composite <sha> -- 'src/*.rs' 'tests/*.rs'`: in `src/` only
   its definition and a doc comment; every real call site is the test file. The
   composite cannot leak a `Proved` because nothing on the verdict path calls it.
2. **The verdict boundary returns `Unknown`, never `Proved`, even on a genuine
   accepted certificate.** `attempt_fo_with_signature` — when `quote_fo +
   find_certificate + check_cert` all accept — still returns `Verdict::Unknown`,
   with an explicit in-code AC citation (here AC-5/AC-6: the two theorems "have
   no approved kernel-checked home yet"). Read the boundary function's actual
   returned verdict on the accepting branch; do not infer it from the route
   name.
3. **No new `Proved` and no new trust on the path.** `grep '^\+' … | grep
   Verdict::Proved` is empty; the encoder src files are ABSENT from the
   production `trusted_base|declare_postulate` grep (the hits are all
   pre-existing unrelated files); the `declare_postulate_raw`/`trusted_base`
   lines are all confined to the test file (route fixtures + the zero-delta
   asserts).

Also confirm the bare path is behavior-identical: the public entry
(`attempt_obligation`) delegates with `catalog = None`, and the old bare
discover fn (`discover_and_quote_fo`) is preserved and still called by the
new `_with_catalog` wrapper. The new catalog-carrying entry
(`attempt_obligation_with_catalog_handles`) has no production caller yet.

## The wiring landing — the reusable checklist

The inert apparatus was later wired by `V3-FO-ROUTE-PROVED-COMPOSITION` (D3's
"one-return flip"), landed squash `520a1d750d3c0acb7274d5fc1f9c6939606b4271`
(PR #3312, base `c6daee907`), verdict `evt_4rkfvzk9pqt1t`. NO OBJECTION.

The single production semantic change was in `attempt_fo_with_signature`: the
`check_cert`-accepted branch, which the apparatus landing left as an
unconditional `emit_unknown_hole_fo_withheld`, became
`match kernel_checked_fo_composite(env, sig, &problem, &cert, phi_closed) {
Some(cert) => Verdict::Proved { cert }, None => emit_unknown_hole_fo_withheld(..) }`.
All four attack conditions were confirmed structurally against the landed blob,
and this is the reusable checklist for **any inert-apparatus-then-wire pair**:

- **(i) `Proved` only via the kernel check.** `fo_kripke.rs` was byte-identical
  base->landed (so `kernel_checked_fo_composite` intact: `Some` only after
  `ken_kernel::check(.., phi_closed).ok()?`); `check_cert` guards the block
  (necessary) but is not sufficient — a non-checking composite is `None ->`
  withheld `Unknown`. Count the `Verdict::Proved` sites base-vs-landed: base 3,
  landed 4, so exactly ONE was added, and every one is kernel-backed
  (`attempt_with_cert`, `attempt_d`/`attempt_ipc` via `try_ipc_cert`, the new
  composite).
- **(ii) `phi_closed` is the independent obligation on the LIVE route.**
  `classify(env, &triple.goal_closed)`; then
  `Route::FO => attempt_fo(.., &triple.goal_closed, ..)`; it flows unchanged
  into the kernel check, never derived from the encoder.
- **(iii) faithful-vs-wrong separates at the kernel check, not `check_cert`.**
  The negative test passes `check_cert`, gets `is_none` on the composite, and
  the LIVE route returns `Unknown` with the WITHHELD hole label; the positive
  test asserts the `Proved` cert EQUALS the composite AND re-kernel-checks.
- **(iv) bare callers fail-closed.** `sig.catalog.as_ref()?` -> `None` ->
  withheld `Unknown` for any env lacking the FoKripke catalog.

The tell that the wiring is safe: the diff is one function's return value; the
three fall-throughs (quotation refused / no cert / `check_cert` false) still
land at `attempt_ipc` with the ORDINARY (not withheld) hole label — so the ONLY
paths that moved are `check_cert`-accepted + composite-kernel-checks (now
`Proved`). Deletions were the old unconditional-`Unknown` comment/return + the
apparatus test's stale "stays unknown" asserts, correctly co-evolved. TCB delta
zero (kernel tree + `FoKripke.ken` + `fo_kripke.rs` blobs all unchanged, no
`declare_postulate`/`trusted_base` added to production).

## Relation to siblings

Sibling of
[[a-mutation-fixture-narrowed-to-the-bool-checker-surface-excising-a-kernel-checked-theorem-is-a-re-pin-not-a-weakening]]
(same V3-FO subsystem; there the guarantee was carried by the kernel check + a
compensating structural pin, here by the kernel re-check of an encoder-built
composite against an independent obligation). Both are instances of the fleet
rule that a Rust-side check is never the soundness authority for a verified
language — the kernel is — so an adversary clears "encoder/checker is trusted?"
by locating where the verdict's `Proved` is actually minted and proving it is a
kernel `check` against an obligation the untrusted layer did not get to choose.
