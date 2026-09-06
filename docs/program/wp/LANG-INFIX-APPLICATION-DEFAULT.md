# WP: LANG-INFIX-APPLICATION-DEFAULT

Steward frame to Team Language. The authoritative, shovel-ready design lives in
the issue node `docs/program/issues/LANG-INFIX-APPLICATION-DEFAULT.md` -- read
that AND this. This frame is a release pointer; the node holds the deliverables,
acceptance criteria, fixed inputs, and banned scope.

- Owner: Team Language
- Branch: `wp/LANG-INFIX-APPLICATION-DEFAULT`
- Size: S. Tier: T1 (a small parser cascade insertion, but the review turns on
  no-silent-reassociation of a working operator cascade, not on line count).
- Risk: low-to-moderate -- it inserts a precedence level into a landed,
  normative arithmetic cascade, which is exactly where a silent reassociation
  would hide (see the node's AC-3).
- Base / fixed inputs: current `origin/main` `d89c764f6`. Re-measure the node's
  perishable anchors (`parser.rs:1994 -> :2012 -> :2032`, `ast.rs:558`,
  `spec/30-surface/32-grammar.md:199/:373`) at your cut; a false anchor is a
  finding, not something to build around.

## Why this is the next language deliverable

The 2026-09-05 operator sequence is symbolic-operators -> match-patterns/
quick-wins -> kernel-hardening -> SEC1. Symbolic-operators is the first bucket;
its chain root `LANG-SYMBOLIC-OPERATOR-NAMES` is MERGED, so `<+>` can now be
defined and this node's premise is true. This is the next unbuilt link
(`LANG-INFIX-APPLICATION-DEFAULT` -> then `LANG-FIXITY-DECL-SURFACE`, which is
blocked behind it).

## Scope discipline (from the node -- do not exceed)

- Every operator is `infixl 9` here. No fixity declaration parsing, no fixity
  table -- that is `LANG-FIXITY-DECL-SURFACE`.
- Do not change the landed arithmetic ordering; do not widen `BinOp`. A user
  operator is an ordinary function applied infix, not a new built-in.
- Assert associativity/precedence STRUCTURALLY (parsed/elaborated shape), never
  by evaluated value -- a symmetric operator hides wrong associativity from a
  value assertion.

## Gate

Architect approach-review REQUIRED (soundness-adjacent parser/elaborator change)
+ Language QA + CI. CV N/A. Path to land: build -> fresh SHA -> Architect +
Language QA + CI -> resolved Decision + git_request to the Steward -> M1-M4 ->
lieutenant.

## Steward merge-time obligation (recorded so it is not dropped)

At THIS node's merge Decision, the Steward pings the Architect for the
`LANG-FIXITY-DECL-SURFACE` import-scoping ruling (`evt_33rw7w8xkdya2`): the
Architect committed to ruling it here, on a base where the parser's actual
table-consultation shape is visible. This is an obligation on the merge, not on
the ring.

## Contention

No path contention with the concurrent runtime FsMetadata respin: this touches
`ken-parser` + `ken-elaborator/src/elab.rs`, FsMetadata touches
`ken-verify`/`ken-runtime`/`ken-host` + elaborator `{compiler_driver, erasure,
prelude}` (disjoint files). Shares the machine-wide `ken-cargo` build lock with
the runtime lane -- runtime is lane-1 priority under the standing yield
directive; take the lock in the gaps, do not launch a long package closure while
a runtime targeted validation is in flight.
