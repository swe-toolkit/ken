---
name: a-trusted-primitive-axiom-must-be-read-against-every-backend-that-implements-the-primitive
description: When a merge adds a trusted postulate about an opaque primitive, the primitive has three implementations (interpreter, runtime-IR evaluator, Cranelift lowering), and the acceptance test usually exercises only the interpreter. Read the axiom against each one, and probe the native paths directly.
metadata:
  type: feedback
---

# A trusted primitive axiom must be read against every backend

**Measured 2026-09-25 on BYTES-CONCAT-AND-ENCODE-CONTRACTS D2.** Squash
`0a6aa59875e85d4f924a20e203ada2a48be59b29`, reported at `evt_cr9t8xh9hpb7`.

## The shape

D2 added four trusted postulates about `bytes_concat`, `bytes_encode` and
`bytes_decode`. The acceptance test checked each one only against the
interpreter's `prim_reduce`. The same primitives have two more
implementations:

- `ken-runtime/src/runtime_ir_evaluator.rs`
- `ken-runtime/src/cranelift_backend/lowering/core/primitive.rs`

Those two had drifted from the interpreter. Native `bytes_decode` does not
NFC-normalize. Native `list_char_to_string` treats each Char as a UTF-8 byte,
so the chars ['Ã', '©'] come out as "é".

The axioms still held on every backend, but only because the non-normalizing
paths agree with each other. A partial fix to one native path would falsify
F3 on native, and no test would go red.

## How to apply

1. For each trusted axiom about a primitive, grep the symbol across
   `ken-interp`, `runtime_ir_evaluator.rs` and the Cranelift `primitive.rs`.
   Read the axiom against each implementation.
2. Probe native behaviour directly: add a throwaway test in
   `cranelift_backend/lowering/core/primitive/tests.rs` using
   `run_example_with_seed_observation`. Print the observation, then revert.
   Run it with `scripts/ken-cargo test -p ken-runtime --lib <name>`; it takes
   about 3 minutes.
3. Report backend disagreement even when the axiom survives. Name the
   ordering hazard: which partial fix would make the axiom false.

Related: [[tested-not-trusted-posture-needs-reachability-precondition]] (the
backends are tested, not trusted, but a trusted axiom about a primitive is
only as true as the implementation it is read against), and
[[differential-oracle-is-blind-to-a-shared-premise]] (two backends agreeing
with each other, not with the axiom, is what hid this drift).
