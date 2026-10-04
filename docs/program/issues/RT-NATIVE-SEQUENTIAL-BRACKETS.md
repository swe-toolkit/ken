---
id: RT-NATIVE-SEQUENTIAL-BRACKETS
title: "Native execution of two sequential resource brackets is distinguishable: a two-bracket program whose brackets get different responses exits with a code that encodes both outcomes, natively and in the interpreter, so a swapped response pairing changes the observation"
status: active
owner: runtime
size: S
gate: architect
tier: T1
depends_on: [RT-IGNORED-ROWS-NEXT-GROUP, RT-NATIVE-CONTINUATION-ENV-CARRIAGE]
blocks: []
github: null
origin: "Architect ruling 2026-09-28 (evt_1ys064x0c11be) on RT-IGNORED-ROWS-NEXT-GROUP's A1 blocker (runtime-implementer evt_5nmkjfcnemamh): native execution of two sequential brackets is the successor, framed from the verbatim refusal and the one-bracket control. Steward-filed per COORDINATION section 2."
---

# Native sequential resource brackets

## Recut (Architect `evt_4x84ykwjtmbsh`)

The side-slot repair is superseded. The continuation's environment travels
in the value, and `RT-NATIVE-CONTINUATION-ENV-CARRIAGE` builds that. This
WP is that node's first consumer: the two-bracket witness is its AC-1. The
retained evidence is D0b, D0c, D1b, T2, the scoped-stack unit rows and the
research advisory `evt_7rqnbfnw80fe`. WIP `a6a07bfd2` is not a candidate.

## Objective

A native program can use two resource brackets one after the other, and its
observation shows that each bracket received its own response.

## AC-0 result (Architect `evt_edpt4e5rpsy0`, on `45a15f913`)

- **The refusal is cleared by landed work.** ENV-CARRIAGE I-2, inside
  `2edc10ac9`, cleared the BoundaryCarrier refusal. The witness
  `rt_ignored_two_buffer_witness.ken` is byte-identical between
  `958121efa` and main. On `958121efa` its test pinned the refusal verbatim,
  and on main it runs with parity (runtime-implementer `evt_2yk1jehzrzh8s`).
  AC-2's revert control therefore holds by history. The one-bracket control
  `one_bracket_retains_native_parity` is green on main.
- **The arity guard stays.** `reject_carried_residual_arguments` fires only
  when `recursive_unit_body` is `None`, the eliminated-hypothesis case.
- **The current witness cannot tell the brackets apart** (check 8). Both
  bodies are the same `buffer_body`, the exit is always `Success`, and the
  releases are compared as a sorted set. A run that hands each Vis the other
  bracket's response gives the observation the test accepts.
- **The `rt_escape` rows are not this WP's.** `ESCAPE_FILE_THEN_READAT`
  (`:660`) stops first at "Match: dynamic arms must produce scalar Int or
  Bool values", and `ESCAPE_BUFFER_THEN_READAT` (`:691`) at the
  generated-entry typed-consumer-projection invariant. The Steward places
  them in L1.

## Deliverable

Test-only, in `crates/ken-cli/tests/`, with no production edit: a new
fixture, `rt_two_bracket_distinguishable.ken` or similar, beside the old one,
which stays unchanged.

- The first bracket is `withBuffer AFull Unit Unit (1 : Int) err_body`, whose
  body returns `ResourceBodyErr Unit Unit MkUnit`. The second is
  `withBuffer AFull Unit Unit (2 : Int) ok_body`, returning
  `ResourceBodyOk Unit Unit MkUnit`.
- Inside `\second`, match `first` and then `second`, each arm a direct
  `host_exit AFull (...)`, the shape at `px8f_buffer_native.rs:121-127`.
  First `ResourceBracketBodyError` with second `ResourceBracketOk` exits
  `Failure 21`; the swap exits `Failure 12`; anything else `Failure 99`.

## Acceptance

- **D0 (probe, then report).** Native build and run, and the interpreter, on
  main: exit, stdout and stderr. `first` is now live across the second
  bracket, a new capture, so a native refusal is possible. A refusal is a D0
  result, reported verbatim, and the Architect rules the site. The candidate
  is written only after the Architect accepts the D0.
- **AC-1.** A test on the new fixture asserts:
  - native exit equals the interpreter's, and both are 21;
  - the non-release traces are equal;
  - the releases are equal as a set, with two distinct members (sizes 1
    and 2). Chronology belongs to `RT-BRACKET-RELEASE-ORDER-PARITY`; do not
    assert order.
- **AC-2 (mutation, QA, scratch, restored byte-identically).** In the
  more-than-one-candidate selection in
  `planning/static_transition/responses.rs`, force first-by-origin and then
  last-by-origin. Record each outcome: a typed refusal, a changed exit (12
  or 99) or a changed trace all discriminate. The only failing outcome is
  exit 21 with equal traces.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- Any production edit, or relaxing the arity check for a value that really
  is an eliminated hypothesis: an Architect stop.
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40` or
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY`.

## Hard-stop inventory (§1b)

§1a count: 2 (Architect `evt_2htrfrdpq3wy`, `evt_6r92kzemmaxps`).

1. constructed context frame keyed on planner coordinates (continuation,
   position, body) — keyed on static coordinates, but one function
   constructs the same coordinates more than once with different operands.
2. constructed context frame scoped to the writer's lowering call extent —
   keyed on static lowering call nesting, but the consumer runs after that
   call returns.

Both entries are one predicate: a first-class, escaping continuation's
environment was associated with its consumer at compile time instead of
being carried by the value (`evt_4x84ykwjtmbsh`).
