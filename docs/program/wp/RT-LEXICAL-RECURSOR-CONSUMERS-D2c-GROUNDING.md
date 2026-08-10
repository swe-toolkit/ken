# RT-LEXICAL-RECURSOR-CONSUMERS — `D2c` (R3 before-hole) grounding

**No production repair.** The candidate's only `crates/` change is the `D2a`
test-only `NonZeroUsize` durability rider in
`lowering/core/tests/control.rs`. `D2c` did not author that rider and does not
modify its mechanism; its sole interaction with it is the comment-only
decorative-glyph removal recorded in §6. `D2c` itself is evidence only: this
record and the differential below add **no further control** and no production
code.

## 0. Provenance — two coordinates, derived at the moment of writing

| coordinate | exact | moves? |
|---|---|---|
| **measurement base** — the tree every figure was taken on | `07dc6593` | **no.** A commit on this branch |
| **candidate merge-base** — `git merge-base HEAD origin/main` | `39bc507b` | **yes**, whenever **either** ref moves |

The merge-base is **re-derived here**, not carried forward from an earlier
handback. A previous revision of this lane's record repeated a captured value
that went stale while the branch stood still, because `main` advanced onto a
commit the branch already contained.

> # NO REUSABLE PRECEDENT. THE MECHANISM IS NOT DETERMINED.
>
> The retained lane compiles this fixture — and the differential in §4 shows it
> does so by making **zero carrier transfers**, not by consuming the recursor
> closure somewhere else. **There is no retained seam to copy.**
>
> An earlier revision of this banner read *"a lawful precedent exists … a
> comparison, not an invention."* That was inferred from the retained lane's
> `Ok` before the differential was taken, and it is **withdrawn**: outcome
> equivalence is not mechanism equivalence, which is the inference the ruling
> explicitly forbids.

## 1. Scope, and the row-5 split preserved

`D2c` is **R3 before-hole only**. Measured at the base, under B-only exclusion:

| row 5 compile | first refusal |
|---|---|
| **before-hole** — `D2c`'s subject | `ComputationalMatch` — *a computational recursor closure names an in-flight activation, not a transferable value* |
| after-hole — **outside** `D2c` | `StaticWorkerBinding` — *a `Var` in value position … has no value representation* |

The after-hole compile is **reported, not repaired**, and `D2c` must not
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

## 4. The retained lane compiles it — and the differential says why

**The retained lane compiles this fixture.** With no exclusion and production
authority, `px8j_capture_source_trace` returns `Ok`.

**I first recorded that as "a landed, lawful mechanism to find and reuse."
The differential shows that reading is WRONG, and this section supersedes it.**

### 4.1 The differential, at the same seam, on the same fixture

Instrumented at `transfer_into_carrier` — the sole production entry — across
both lanes on the identical expression:

| lane | carrier transfers | outcome |
|---|---|---|
| **retained (`RecursiveDescent`)** | **ZERO** | `Ok` |
| functionized, B-only exclusion | **six**, the sixth inadmissible | refused |

The functionized transfers, in order: `BorrowedNativeValue`, `CapabilityToken`,
`Constructor Unit::MkUnit`, `Constructor Result::Ok[Unit]`,
`Constructor PX8JHoleInput::Leaf`, and then
`Constructor PX8JHoleOutput::Node[ComputationalRecursorClosure]` — inadmissible.

### 4.2 The smallest differential explanation

**The retained lane does not consume the recursor closure at some other seam. It
never carries the value at all.** Zero transfers means the carried
computational-match route is not taken there, so the boundary the functionized
lane meets is never reached.

⇒ **There is NO retained consumption seam to copy.** The retained lane does not
answer the question; it never asks it. An "exact lawful counterpart already
available on the functionized path" does **not** exist, and inferring one from
the retained lane's `Ok` would have been exactly the equivalence-from-outcome
the ruling forbids — which is the inference my §4 originally invited.

### 4.3 The first non-transferable boundary

`lower_carried_computational_match` requires the case-body **result** to cross
the carrier boundary. On this fixture that result is
`Constructor Node[ComputationalRecursorClosure]`, and a recursor closure names
an in-flight activation rather than a value.

The boundary is therefore **structural to the functionized route**, not a
missing consumption that the retained lane performs elsewhere.

## 5. Verdict

**A lawful mechanism is NOT determined, and the differential closes the reuse
route rather than opening it.**

Determined: the exact seam, the first missing owner
(`lower_carried_computational_match`), the incoming carried computational-match
route, the isolating contrast against four admissible sibling transfers, and now
the measured fact that the retained lane carries **nothing** on this fixture.

⇒ Any `D2c` repair is a **new mechanism on the functionized lane** — either
consuming the recursor closure at its owner before the enclosing constructor is
built, or not carrying that result — not the adoption of a landed one. **Both
are design decisions and neither is authorized here**, so this checkpoint stops
at the boundary rather than choosing.

## 6. Scope

No repair, no new control, no enum change, no `R4`, no `D3`, no retirement or
lane deletion, and no tracker `status:` change. The approved `D2a` rider and
the `D2b` mechanism are unchanged in mechanism: the icon strip removed one
decorative glyph from the rider's comment and altered nothing else in it.
Probes were temporary and are removed —
`lowering/mod.rs` sha256 begins `36121e14`, matching the measurement base.
