---
id: V3-FO-ROUTE-PROVED-COMPOSITION
title: "D3 honest-reach: route FO's public entry returns Proved by composing the two 23 section 4.4 theorems (checker-soundness and embedding-adequacy) into a kernel-checked cert for the quoted obligation — the explicitly reserved verdict-flip"
status: merged
owner: language
size: M
gate: none
tier: T1
status_note: "active 2026-09-04 — RE-RELEASED after apparatus prerequisite landed 9b89a7436 (node closed b188f359d); depends_on satisfied, kicked language ring"
depends_on: [V3-FO-CHECKER-SOUNDNESS, V3-FO-EMBEDDING-ADEQUACY, V3-FO-ROUTE-CONSUMPTION-APPARATUS]
blocks: []
github: null
origin: "Steward, 2026-09-04, at origin/main 985cd0436. D3 is the honest-reach verdict-flip reserved by V3-FO-KRIPKE-SLICE AC-5/AC-6 and named in V3-FO-EMBEDDING-ADEQUACY's origin (the releasable remainder is D2+D3 only). The second of the two 23 section 4.4 theorems (embedding_adequacy) landed 5c705a4d7 (D2b), completing the two-theorem prerequisite; the first (checker-soundness) was already merged. Lane-2 next confirmed by language-leader evt_70js2ckw1kbb0 (Option 1: this is the reserved verdict-flip, within lane-2's objective, NOT an operator re-scope; V3-FO-OBLIGATION-SIGNATURE-DISCOVERY already removed public-route reachability gate 1, leaving only 23.4.4 theorem composition). Decomposition and the contingent gate=none ruling from Architect evt_3th40hnvytpzp. Steward-filed per COORDINATION section 2."
---

> # OPERATIVE (Steward, 2026-09-04) — RE-RELEASED. The apparatus prerequisite
> # [[V3-FO-ROUTE-CONSUMPTION-APPARATUS]] LANDED origin/main 9b89a7436 (node
> # closed b188f359d; blob-verified by the Steward — fo_kripke.rs 45fd1206 and
> # prover.rs 72fa3c75 on main are identical to candidate 3486d3fd). Component A
> # (theorem-handle threading via FoCatalogHandles) and Component B (kernel-
> # guarded Rust<->catalog encoder, route classifies on the INDEPENDENT
> # ObligationTriple.goal_closed, encoder not trusted) are now present, so the
> # composite checker_soundness ∘ embedding_adequacy is assemblable and
> # kernel-checkable at the accepted return. D3's one-return flip — the LAST
> # step — is therefore released to the language ring. gate=none is CONFIRMED,
> # not merely contingent: the apparatus landed with the independent-phi_closed
> # measurement satisfied (Architect evt_5apcqx4h6vgyf), zero trusted_base delta.
> #
> # COORDINATE NOTE (re-measure, do not trust the line number): the apparatus
> # landed prover.rs +33 ADDITIVE (Component-A handle threading), so line numbers
> # below shifted. The flip target is defined SEMANTICALLY, not by line: the
> # accepted-certificate branch of `attempt_fo_with_signature` (reached only when
> # quote_fo succeeds AND find_certificate returns a cert AND check_cert is true)
> # that currently returns `emit_unknown_hole_fo_withheld` — AC-C of the
> # apparatus PRESERVED that exact return. Re-measure its line at 9b89a7436.
> # Prior kickoff was evt_74231xgpa2asm; the implementer's hard-stop
> # (evt_kw6zgeshvnp3) is now resolved by the landed apparatus.
>
> # The flip decomposition, unchanged (Architect z2610 evt_3th40hnvytpzp):
> # it flips exactly ONE return and moves no other prover path. The honesty of
> # the flip and its zero-TCB-delta are the SAME property: the kernel-check of
> # the composed cert against `phi_closed`. gate=none is CONTINGENT on that
> # kernel-check being buildable — which is exactly what the apparatus node
> # makes true (Component B's kernel-guarded denotation equation).

## Objective

One verdict-flip. In `attempt_fo_with_signature` (`crates/ken-elaborator/src/
prover.rs:574`), the accepted-certificate branch — reached only when
`quote_fo` succeeds AND `find_certificate` returns a cert AND
`check_cert(&target, &cert)` is true — currently returns
`emit_unknown_hole_fo_withheld(env, phi_closed)` (line 597). D3 replaces that
one return with `Verdict::Proved { cert }`, where `cert` is the KERNEL-CHECKED
composition `checker_soundness` then `fok_embedding_adequacy` applied to the
quoted obligation and validated by the kernel as a proof of `phi_closed`.
Every other path stays verbatim. This is the honest reach 23 section 4.4
reserved: route FO may return `Proved` now that BOTH theorems are proved and
merged.

## Fixed inputs, measured at origin/main `985cd0436`

- `prover.rs:574` `attempt_fo_with_signature`; **line 597** the accepted-branch
  `return emit_unknown_hole_fo_withheld(env, phi_closed)` — the ONLY line D3
  changes. `prover.rs:800` `emit_unknown_hole_fo_withheld` — stays defined (it
  still serves legitimately-withheld paths).
- `prover.rs:550` `attempt_fo` (discovery entry); it and the direct-signature
  entry share `attempt_fo_with_signature`, so the flip is reached from both.
- `crates/ken-elaborator/src/fo_kripke.rs`: `quote_fo`, `find_certificate`,
  `embed`, `check_cert` — the accepted-branch predicates (inputs, unchanged).
- The two kernel-checked theorems in
  `catalog/packages/Tooling/Verification/FoKripke.ken`: checker-soundness and
  `fok_embedding_adequacy` (adequacy proof landed 5c705a4d7, byte-identical to
  the prior-approved candidate; z2550 confirmed adequacy adds NO trusted
  declaration). `fok_embedding_adequacy` consumes an arbitrary
  `fok_classically_valid` inhabitant — `elim_trunc` over the derivation,
  target-soundness + K(Sigma) + forward correspondence, checker-soundness-free.

## Deliverables

1. **The composed cert term.** `checker_soundness` supplies the
   `fok_classically_valid` inhabitant from the checker-accept `(q, pi)`;
   `fok_embedding_adequacy` discharges it to `fok_denote`; the assembled term,
   applied to the accepted `(q, pi)` and the quoted obligation, is a
   kernel-checked proof of `phi_closed`.
2. **The flip.** Line-597 return becomes `Verdict::Proved { cert }` when the
   composed cert kernel-checks as `phi_closed`; otherwise fall through to the
   honest Unknown/withheld (never a false `Proved`).
3. **Tests** — the non-degenerate pair (AC-1/AC-2), preservation (AC-5), and
   the TCB pin (AC-6).

## Acceptance criteria

- **AC-1 (LOAD-BEARING — honesty AND zero-TCB-delta are this one property).**
  The returned cert kernel-checks as a proof of `phi_closed`: the kernel
  type-checks the composed term against `phi_closed`, so `Proved` is backed by
  the KERNEL, not by `check_cert` acceptance. Control (positive): a genuinely
  accepted cert (`quote_fo` ok, `find_certificate` returns `pi`, `check_cert`
  true) yields `Verdict::Proved { cert }` with native verdict == the
  kernel-checked composition.
- **AC-2 (DISCRIMINATING NEGATIVE, same accepted-branch input).** A
  `check_cert`-accepted cert whose composition does NOT kernel-check for this
  obligation (quotation mismatch / wrong obligation) yields Unknown/withheld,
  NEVER `Proved`. `Proved` iff it kernel-checks; accept-but-not-kernel-checkable
  stays honest-Unknown.
- **AC-3 (QUOTATION CORRESPONDENCE, the load-bearing step).** Pin
  `quote_fo(problem) = q` AND `embed`/`denote(q) = phi_closed`; the composite is
  kernel-checked AGAINST `phi_closed`, not a lookalike. The AC is that
  kernel-check against `phi_closed`, NOT merely "quote succeeded" — if `q` does
  not faithfully quote `phi_closed` the composite fails to kernel-check and MUST
  fall to Unknown.
- **AC-4 (GUARD, structural — an AC, not prose).** The kernel-check on the
  composed cert is a NECESSARY precondition of returning `Proved`; `check_cert`
  acceptance is necessary but NOT sufficient. There is no path from
  `check_cert`-true to `Proved` that bypasses the kernel-check of the composite.
- **AC-5 (PRESERVATION).** Quotation-refused and no-certificate paths stay
  verbatim (fall through to `attempt_ipc`); `emit_unknown_hole_fo_withheld`
  stays defined and reachable for legitimately-withheld paths; no prover path
  other than the single line-597 return moves.
- **AC-6 (TCB).** `trusted_base()` unchanged. The two theorems are consumed as
  already-merged kernel-checked theorems (proved, not postulated), so the
  composition adds zero trusted authority.

## Gate (contingent = none)

**gate = NONE, buildable now within lane 2, CONTINGENT on AC-1 + AC-6 + AC-2
holding** (Architect evt_3th40hnvytpzp, item 5). The only path that would touch
the TCB is returning `Proved` on `check_cert`-trust WITHOUT a kernel-checked
composite — that pulls the FO checker's soundness into the TCB. The two
theorems exist precisely to let the prover PROVE (not trust) that a
checker-accept yields the object proposition.

**If the implementer finds it CANNOT produce a kernel-checked composed cert (only
`check_cert`-trust is available): STOP and flag.** That is the operator gate —
do NOT release the flip; the language-leader routes the finding to the Steward,
who queues it for the operator (away until ~13:00 UTC 2026-09-04). The Architect
expects the composite IS kernel-checkable (the D2b arc was built to make exactly
this composition kernel-checkable), so gate=none is the likely outcome — but it
is contingent, not unconditional.

## Banned scope

No kernel/`Elim` change. No new postulate or trusted declaration. No change to
`FoKripke.ken` theorem statements or proofs (they are inputs). No relaxation of
`check_cert`. No change to any prover path other than the single line-597
return.

## Review

Architect reviews candidate design + soundness (evt_3th40hnvytpzp): the
composite kernel-checks as `phi_closed`, the accept-but-not-kernel-checkable
negative stays Unknown, `trusted_base()` unchanged, no other path moved.
Language QA reviews the test controls (the non-degenerate pair discriminates).
CV N/A unless a spec/conformance surface is touched (none expected).

## Sequencing and contention

**Contention: LOW.** Touches `crates/ken-elaborator/src/prover.rs` (one line
plus a composition helper) and new tests;
`catalog/packages/Tooling/Verification/FoKripke.ken` is READ-ONLY (theorems as
inputs). No overlap with lane-1 runtime (`crates/ken-runtime`, `crates/ken-cli`)
or lane-3 foundation (`catalog/` value modules). Publish queue unaffected.

## Capability tier: T1

Soundness-bearing: it constructs a composed kernel-checked proof term and pins
that `Proved` is backed by the kernel. Reasoning-dense despite a small diff — a
mis-composed cert or a bypassed kernel-check is a false `Proved`, the one
outcome the whole node exists to exclude.
