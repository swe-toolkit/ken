---
id: RT-COMPOSED-RETURN-ATOMIC-CLOSEOUT
title: "Composed-return native repair, option (a)(i) WP3 of 3 (cuts the SOLE production candidate from WP1+WP2+WP3): complete the Ret-binder/capture/body edge if WP2's shared block parameter does not already supply it; add per-variant per-arrival application/result/Ret-input pairing with exactly-once closure; retire only superseded environment/seed-as-result claims; the candidate flips base-red ResourceBodyResult PatternMatchFailure to exact InvalidOffset on fs-read/write-at-offset, then Runtime QA + Architect on the exact SHA."
status: draft
owner: runtime
size: M
gate: runtime-qa+architect
tier: T1
depends_on: [RT-COMPOSED-RETURN-TAIL-FORWARD-EDGE]
blocks: []
github: null
origin: "Architect component design for the operator-funded composed-return native repair, option (a)(i) (PART 1/2 evt_381dzjykr4knn, PART 2/2 evt_5963far74b735, 2026-09-01). WP3 of the three-checkpoint ATOMIC merge unit: it cuts the SOLE production candidate from WP1+WP2+WP3. Flips ready when WP2's checkpoint is reached; the Steward releases it. This is the ONLY node of the three that gates and merges (Runtime QA + Architect on the exact SHA, then Steward M1-M4, then lieutenant). Semantic target: flip base-red ResourceBodyResult PatternMatchFailure -> exact InvalidOffset on the fs-read-at-offset / fs-write-at-offset witnesses. Bound base e6a6c5240."
---

> # WP3 of 3 — THE SEMANTIC LANDING. Cuts the SOLE production candidate from
> # WP1+WP2+WP3. DRAFT — reached after WP2's checkpoint.
> #
> # This is the ONLY mergeable object of the three. WP1 and WP2 are held
> # checkpoints; the first mergeable object is the FULL route-specific
> # result-to-Ret repair with BOTH exact products. A Direct-only candidate would
> # execute one route while Tail still discards its result; a Tail-only candidate
> # would leave carried Direct environment masquerading as result; a result-edge
> # candidate without exact products could merely move to another failure.
> #
> # Mechanism contract: Architect PART 1/2 `evt_381dzjykr4knn` + PART 2/2
> # `evt_5963far74b735`. Do not reopen the twelve-stop D0 chain.

## Deliverables (Architect PART 2/2)

- Complete the Ret-binder / capture / body edge ONLY if WP2's shared block
  parameter does not already supply it; derive read and write destinations
  independently from planner facts. If the shared Ret parameter does not naturally
  reach the exact capture/body read, repair ONLY the first planner-authorized
  Ret-binder edge in the same atomic candidate — NEVER write capture 0, never
  search by origin/value.
- Add per-variant, per-arrival application/result/Ret-input pairing and
  exactly-once closure.
- Retire ONLY superseded claims/tests describing environment or seed as result;
  keep general fallback paths used by other populations.
- Cut the sole production candidate from WP1+WP2+WP3.

## Acceptance criteria (Architect PART 2/2)

- **AC-EXACT-PRODUCTS.** Base controls show BOTH unchanged witnesses at
  `ResourceBodyResult` `PatternMatchFailure`; the candidate yields exact
  `InvalidOffset` for fs-read-at-offset and fs-write-at-offset, with
  interpreter-matching effect prefix/order.
- **AC-THREE-SUPPRESSIONS.** Three independent causal suppressions —
  continuation inheritance, route-specific application/production, and
  result-to-Ret binding. Direct and Tail have SEPARATE production suppressions.
  Each recreates the localized base-red observation, not an arbitrary error.
- **AC-AT-MOST-ONCE.** Prove at-most-once SEPARATELY for inheritance, Direct
  call, Tail existing call, Tail authority consumption, and Ret-input binding.
  Scalar totals are NOT evidence.
- **AC-PIN-DISCIPLINE.** Every pin states MEASURED/CLAIMED/GAP. Every negative has
  a positive reach control and a compile-preserving population-side mutation with
  provenance, exact test count, and byte-identical restoration.
  - MEASURED (Tail edge): the actual call-result SSA identity at the branch and
    the same identity at the Ret parameter/body input. CLAIMED: one governed
    result delivered once to its exact continuation. GAP: exact planner identity
    plus exclusive dynamic reach (move-only authority closes identity;
    per-arrival runtime pairing and drop/duplicate/substitution controls close
    reach/exclusivity).
  - MEASURED (Direct): the exact emitted continuation call and its returned
    Result. CLAIMED: application rather than environment reuse. GAP: the carried
    word is only captures, closed by the independently selected transport, ordered
    projections, and actual target call.
- **AC-AFFECTED-CLOSURE.** Targeted runs cover every Rust target and every
  `ken run` consumer loading a changed module, INCLUDING unchanged-file
  consumers. Never run `--workspace` locally (`COORDINATION §12`).
- **AC-CI-GATES.** The final exact SHA returns to Runtime QA and Architect; CI
  supplies the workspace / `--locked` / conformance gates.

## Gate + merge flow (this node only)

WP3 cuts the sole candidate. On it: fresh **Runtime QA + Architect** on the exact
SHA, then **Steward M1-M4** (verify gates + diff scope from objects + resolve the
Decision), then **ROUTED to the lieutenant** for CODE publish + full CI. On
landing, this node and the atomic merge unit close. Any uncovered outcome is a
HARD STOP to the Architect.
