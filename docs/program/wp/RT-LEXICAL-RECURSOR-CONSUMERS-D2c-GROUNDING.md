# RT-LEXICAL-RECURSOR-CONSUMERS — `D2c` (R3 before-hole) grounding

**Evidence only: no repair, no control, `crates/` byte-identical.**

## 0. Provenance — two coordinates, derived at the moment of writing

| coordinate | exact | moves? |
|---|---|---|
| **measurement base** — the tree every figure was taken on | `07dc6593` | **no.** A commit on this branch |
| **candidate merge-base** — `git merge-base HEAD origin/main` | `39bc507b` | **yes**, whenever **either** ref moves |

⛔ The merge-base is **re-derived here**, not carried forward from an earlier
handback. A previous revision of this lane's record repeated a captured value
that went stale while the branch stood still, because `main` advanced onto a
commit the branch already contained.

> # A LAWFUL PRECEDENT EXISTS. THE MECHANISM IS NOT YET DETERMINED.
>
> The **retained lane compiles this exact fixture**, so `D2c` is a question
> about what `RecursiveDescent` does differently — a **comparison**, not an
> invention. That is the single most useful thing this grounding establishes,
> and it is measured rather than assumed.

## 1. Scope, and the row-5 split preserved

`D2c` is **R3 before-hole only**. Measured at the base, under B-only exclusion:

| row 5 compile | first refusal |
|---|---|
| **before-hole** — `D2c`'s subject | `ComputationalMatch` — *a computational recursor closure names an in-flight activation, not a transferable value* |
| after-hole — **outside** `D2c` | `StaticWorkerBinding` — *a `Var` in value position … has no value representation* |

⛔ The after-hole compile is **reported, not repaired**, and `D2c` must not
claim row 5 green: repairing before-hole leaves the other compile at its own
wall.

## 2. The seam, exactly

`transfer_into_carrier` is the sole production entry to the admissibility walk.
Instrumented there, the refusing transfer is:

```text
origin = StaticOriginId(23)   admissible = false
  Constructor ctor:fixture::PX8JHoleOutput::Node
    ComputationalRecursorClosure
```

**Caller chain, from the backtrace:**

```text
lower_computational_match_expr
  → lower_carried_computational_match
    → transfer_into_carrier(origin 23, Constructor Node[ComputationalRecursorClosure])
```

⇒ **First missing consumption/representation owner:
`lower_carried_computational_match`**, on the **carried computational-match**
route. It carries a case-body **result** into the carrier while that result
still contains an **un-consumed recursor closure**.

## 3. The contrast that isolates it

Four transfers on the same compile are **admissible**, and they differ in
exactly one way — none contains a recursor closure:

| origin | value | admissible |
|---|---|---|
| 18 | `Constructor Unit::MkUnit` | yes |
| 8 | `Constructor Result::Ok[Unit::MkUnit]` | yes |
| 26 | `Constructor PX8JHoleInput::Leaf` | yes |
| **23** | **`Constructor PX8JHoleOutput::Node[ComputationalRecursorClosure]`** | **no** |

⇒ The refusal is **not** about carrying a constructor, and not about this
producer. It is precisely the **un-consumed in-flight activation inside** the
carried result. The guard is correct: a recursor closure names an activation,
not a value.

**This is the same shape `D2a` and `D2b` each resolved** — a protocol value
reaching a value position because nothing consumed it at its owner. It is **not
the same root**: different owner, different route, different protocol value.

## 4. The lawful precedent — measured

**The retained lane compiles this fixture.** With no exclusion and production
authority, `px8j_capture_source_trace` returns `Ok`.

⇒ `RecursiveDescent` already handles a carried result containing this shape.
Whatever it does is a **landed, lawful mechanism**, so `D2c` is a question of
finding and reusing it rather than designing one.

⛔ **What this does NOT establish:** *which* mechanism, or that it transfers to
the functionized lane. The two lanes differ precisely in how bodies are emitted,
so the retained lane may consume the closure at a seam the functionized lane
does not have. **That is the next measurement, and it is not made here.**

## 5. Verdict

**A lawful mechanism is NOT yet determined**, and no repair is attempted.

What is determined: the exact seam, the first missing owner, the incoming route,
the isolating contrast, and the existence of a landed precedent on the retained
lane. The next step is a **differential** — trace the same fixture on the
retained lane to the point where the recursor closure is consumed, and ask
whether that consumption has a functionized counterpart.

## 6. Scope

No repair, no control, no enum change, no `R4`, no `D3`, no retirement or lane
deletion, no tracker `status:` change, and the approved `D2a` rider and `D2b`
mechanism are untouched. Probes were temporary and are removed —
`lowering/mod.rs` sha256 begins `36121e14`, matching the measurement base.
