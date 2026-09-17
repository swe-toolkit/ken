---
id: RT-HOST-RESPONSE-ROUTE-KEY-COLLISION
title: "Decide whether the static-transition planner's `two host response cases claim one operation constructor` invariant is too strong: its route map is keyed on `case.constructor` ALONE while the value it stores carries `operation` as a field, so two cases sharing a constructor across different operations collide. Four ignored rows fail with this one byte-identical message under TWO different labels claiming two different mechanisms -- the runtime says one defect, the labels say two."
status: ready
owner: runtime
size: S
gate: none
tier: T1
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-17, second repair node sized from the RT-IGNORED-FAILING-ROWS-INVENTORY ledger (488acf24e) on operator directive 2026-09-15 'The other tests should be fixed.' S2 is the cluster where the ledger's central finding is sharpest: four rows, one byte-identical planner message, two mutually inconsistent labels, and the labels disagree with each other exactly where the runtime agrees. Steward-filed per COORDINATION section 2."
---

# The defect, and the hypothesis that is NOT yet a diagnosis

Four ignored rows fail with one byte-identical signature, measured from runs at
`c041de7c3` by `RT-IGNORED-FAILING-ROWS-INVENTORY` (ledger `488acf24e`, `S2`,
4 of 15 rows):

    Cranelift backend failure: native static transition planner invariant
    failed; please report this compiler bug: two host response cases claim
    one operation constructor

**The invariant, at `origin/main` `fe649049f`:**

    crates/ken-runtime/src/cranelift_backend/planning/
      static_transition/responses.rs:1281

    let route = HostResponseRoute {
        operation: *operation,          <- the VALUE carries the operation
        effect_origin,
        producer_call_origin,
        response_origin,
    };
    if routes.insert(case.constructor.clone(), route).is_some() {
        return Err(planner_error(
            "two host response cases claim one operation constructor",
        ));
    }

`routes` is a `BTreeMap<RuntimeSymbol, HostResponseRoute>` **keyed on
`case.constructor` alone.**

> **THE KEY OMITS WHAT THE VALUE CARRIES.** Two cases that share a constructor
> but belong to **different operations** collide, and the invariant reports a
> duplicate that is not one.

**This is a hypothesis with a cheap falsifier, NOT a diagnosis, and it must not
be written into the repair as though it were settled.** Nobody has run these
four rows against it. See the frame's §3 for the one measurement that decides
it, and note that the opposite outcome — **the invariant is right and the four
tests assert something false** — closes this node just as well.

# The rows, keyed by FILE and LINE because the names collide

**At `c041de7c3`:**

    crates/ken-cli/tests/rt_escape_second_resource_native.rs:654
      escaped_resource_used_by_fanning_host_op_matches_interpreter
    crates/ken-cli/tests/rt_escape_second_resource_native.rs:714
      nat_fanout_escaped_resource_matches_interpreter
    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs:150
      nested_ok_payload_reaches_both_real_executors
    crates/ken-cli/tests/px7n_nested_computational_eliminator.rs:171
      nested_err_payload_reaches_both_real_executors

**`nested_ok_payload_reaches_both_real_executors` and
`nested_err_payload_reaches_both_real_executors` ALSO EXIST in
`crates/ken-cli/tests/px7o_heterogeneous_eliminator_frames.rs` (`:128`,
`:133`).** Those are **not** this node's rows. **Never resolve one of these by
symbol name** — a name-keyed instrument cannot tell the two files apart, and
two of the four rows have a same-named twin one file over.

# The finding that made this the second cut

**Two labels, two claimed mechanisms, one runtime signature:**

    RT-CLOSURE-BOUNDARY-LANE  merged  "a runtime-local closure has no durable
                                       lane across the boundary"          2 rows
    RT-FRAME-MARKER-ONCE      draft   "the checked Runtime frame marker is
                                       consumed more than once"           2 rows

**The ledger records `label agrees? NO` for all four.** Neither label mentions
a response constructor, an operation, or a planner route.

⇒ **The labels disagree with each other exactly where the runtime agrees
byte-for-byte.** That is positive evidence that the runtime partition is the
reliable one here and the label partition is not — the ledger's §4 finding,
in its sharpest instance.

**Both labels also read `fails at base 21fd46dc`**, a base neither row has been
re-measured against since. **A label carrying a stale base is a claim about a
tree nobody is standing in.**

# Related

- [[RT-IGNORED-FAILING-ROWS-INVENTORY]] — the ledger that measured `S2`, cited
  at `488acf24e`. **Provenance, not a dependency**: every input above was
  re-derived from the tree except the observed signature, which is a
  measurement at a fixed SHA.
- [[RT-CARRIED-RESIDUAL-IH-ARITY]] — the ledger's `S1`, the other node this
  sizing cuts. **Different subsystem (lowering, not planning) and a different
  mechanism. Do not fold.**
- The ledger's `S6` and `S8` are single rows carrying **different**
  planner-invariant messages from the same subsystem. **They may be absorbed
  here once this node measures its cause — do not assume it, and do not pull
  them in speculatively.**
