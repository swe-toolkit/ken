---
id: RT-CONTINUATION-CALL-TOKEN-ONCE
title: "Check the continuation call-token exactly-once obligation with RT-FRAME-MARKER-ONCE's per-key state lattice instead of a flat per-Function claim ledger, so claims in mutually exclusive successors no longer collide; census the other flat exactly-once ledgers first"
status: ready
owner: runtime
size: M
gate: architect
tier: T1
depends_on: [RT-FRAME-MARKER-ONCE]
blocks: []
github: null
origin: "Architect RT-FRAME-MARKER-ONCE sentinel ruling evt_6cyyg6xz8csab (T-i branch), on the runtime-implementer's measurement evt_qv8t1jrprwfm. Steward scope call evt_77f647ed6x03g: FRAME-MARKER-ONCE lands narrow; the call token is this successor, which extends the landed engine rather than re-deriving it. Steward-filed per COORDINATION section 2."
---

# Continuation call token checked once by the lattice

## Objective

The continuation call-token obligation is checked by the per-key state
lattice that `RT-FRAME-MARKER-ONCE` lands, one engine over exactly-once
obligation kinds. The double-bind sentinel then runs natively.

## Settled inputs (measured on `RT-FRAME-MARKER-ONCE` WIP `b5a82424f`)

- **The refusal.** With the frame-marker refusal cleared, the double-bind
  sentinel `checked_double_bind_admits_then_refuses_at_frame_marker` first
  refuses at object emission: "a continuation call token was claimed twice,
  first by Predeclared(PredeclaredFunctionId(5))".
- **Class (T-i), a flat-ledger false positive** (`evt_qv8t1jrprwfm`).
  - In `u2:44`, the first claim (`block54`) and the duplicate (`block247`)
    have the same full identity, including `recursive_position=1`.
  - They sit in exclusive successors of `block7` (`brif v23` to `block9` and
    `block10`): the duplicate is reachable from entry, avoiding the first
    claim, and neither reaches the other.
  - It is the same defect `RT-FRAME-MARKER-ONCE` closes, in a sibling ledger.
- **The interpreter** gives `captured\ncaptured\n`, exit 0, with
  `IsTerminal`, `Write`, `Write`.
- **Other flat exactly-once ledgers** named by the Architect, with no
  witnesses measured: `mod.rs:9594` (affine splice capability) and
  `mod.rs:13441` (root answer authority).

## Deliverable

Continuation call-token claims are events of the landed lattice engine, not
entries in a flat per-Function set. Any other flat ledger the census shows is
reached by an exclusive-arm witness joins the same engine, as the Architect
rules at AC-0.

## Acceptance

- **AC-0 (census, then ruling; no build).**
  - List every flat exactly-once ledger in the lowering, with its key, its
    insert and check sites, and any currently ignored or refused test that
    reaches it.
  - Map the call token onto the lattice's events (which site is an enter,
    which a receipt).
  - The Architect rules the mapping and which ledgers join, with a
    control-by-rule table, before any edit.
- **AC-1.** The double-bind sentinel reaches native parity with the
  interpreter: stdout, exit code and effect sequence. If a later independent
  refusal appears instead, report it verbatim; this WP then lands on its unit
  controls and the refusal is placed.
- **AC-2 (control).**
  - Restoring the flat call-token ledger returns the sentinel to the
    "claimed twice" refusal.
  - A same-path double claim still refuses, by exactly one lattice rule.
  - Every currently green runtime suite stays green.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- A ledger whose obligation is not exactly-once per key (for example
  at-most-once, or keyed per path) is a stop to the Architect with its site.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.
