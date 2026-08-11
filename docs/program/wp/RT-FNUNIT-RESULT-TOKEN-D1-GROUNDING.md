# RT-FNUNIT-RESULT-TOKEN D1 — the scoping answer

Node: `docs/program/issues/RT-FNUNIT-RESULT-TOKEN.md`. Frame:
`RT-FNUNIT-RESULT-TOKEN.md`. Measurement-only turn: **no `crates/` change, no
fixture authored, no repair, and `D2` not started.**

Base and merge-base `origin/main` `a6438b76afcd717cd9c42f22a5d8d4036ba22b0a`,
re-derived. Every command targeted; no `--workspace`.

## 0. The headline

**The premise this deliverable was cut on is false, and correcting it makes the
question cheaper rather than more expensive.**

The frame (§3) and the issue node (§85) both record, from the Adversary's
census on `91435b89`, that `nc22` is *"the ONLY one of 21 `nc` fixtures carrying
a `Call` whose callee is a `Closure` or `LexicalClosure`"*, and conclude that
family width is **unestablishable** without authoring fixtures.

**There is a second instance, it is in production rather than test code, and it
is green on the functionized lane today.**

⇒ The failing class is **not** "a `Call` whose callee is a closure". That shape
already works on `FunctionizedUnits`. Whatever `nc22` hits, the callee is not
it.

## 1. Fixed inputs, re-pinned

At `a6438b76`, all three blobs are **identical to the Steward's `22fb3a61`
pins** — `artifact/api/tests.rs` has not moved again since `90ddcf1c`.

| path | pinned `22fb3a61` | measured `a6438b76` | |
|---|---|---|---|
| `cranelift_backend/surface.rs` | `99b9b507` | `99b9b507` | unchanged |
| `cranelift_backend/artifact/api/tests.rs` | `7ea92d79` | `7ea92d79` | unchanged |
| `cranelift_backend/compiled.rs` | `31e5c149` | `31e5c149` | unchanged |

## 2. The deliverable is live — reproduced, not inherited

`scripts/ken-cargo test -p ken-runtime --lib nc22_cranelift_agrees --
--ignored`:

```
left: BackendFailure { stage: NativeLoweringOrExecution,
                       reason: "native result token 265 is not in the result table" }
right: RuntimeIrNativeAgreement { stage: RuntimeIrNativeCompare }
```

at `artifact/api/tests.rs:188`. **`AC-1`'s "seen to fail before it passes"
precondition is now on the record at this base**, which it was not — the row has
been dark since the skip.

## 3. The second instance

`crates/ken-runtime/src/ir.rs:1121`, inside **`pub fn nc5_seed_examples()`** —
production code, not a test fixture:

```
name:               "closure-capture-application"
checked_core_shape: let y = 2 in (\x . add_int x y) 5
ir:                 Call { callee: Closure { captures: ["decl:fixture::Local::y"],
                                             params:   ["x"],
                                             body:     PrimitiveCall add_int(Var 0, Var 1) },
                           args: [Int 5] }
observation:        Returned(Int 7)
```

That is a `Call` whose callee is a `Closure`, carrying a capture. It is one of
exactly **five** `nc5` seed examples, and `nc5_seed_examples()` is described in
`control.rs:11898` as *"the gate, not a sample: the single production function
producing the seed corpus"*, selected by name from `values.rs`,
`constructors.rs`, `artifact/api/tests.rs` and `ken-interp`'s differentials.

### 3.1 It is on the functionized lane, and it is green

Two committed controls, both **measured passing** at `a6438b76`:

- **`d3_the_seed_corpus_fires_no_residual_at_all`** (`control.rs`) enumerates
  every `nc5` example through `enumerate_recursive_descent_residuals` and
  asserts the firing map is **empty in both directions**. Zero residuals is the
  lane statement: no member of the seed corpus is held on `RecursiveDescent`.
- **`program_runner_preflights_metadata_before_backend_lowering`**
  (`artifact/api/tests.rs`) runs the seed program and asserts **all five**
  reports carry `NativeFidelity::F1SeedObservationAgreement`.

⇒ **The corpus does NOT have "zero live coverage of this shape in either
direction".** It has green coverage in one direction, and that coverage is a
production-corpus gate rather than an incidental row.

### 3.2 The one qualification, stated because it bounds the claim above

The residual census reads `example.ir` directly, so it certainly sees the
`Call`. **I did not trace that call into emitted code**, so I have not
excluded that the closure application is folded before native lowering. If it
is, the green instance bounds the *lane selector* rather than the *emitter*,
and §4's axis still holds but its green cell weakens. Settling it means
observing the emitted unit for that example — which is `D2`'s instrument, and
`D2` is not started.

## 4. So what IS the axis — the two instances differ on their RETURN

| | `nc5` `closure-capture-application` | `nc22` |
|---|---|---|
| callee | `Closure`, one capture | `Closure`, no captures |
| call body | `PrimitiveCall add_int` | `Match` → `Record { ok: If, value: sub_int(mul_int) }` |
| **returns** | **`Int 7`** | **`Record { ok: Bool(true), value: Int(7) }`** |
| on `FunctionizedUnits` | **green** | **fails, token 265** |

`RT-WORKER-FIXTURE-DECODE` §1e keys decoder selection on the `Lowered` shape
(`lowering/mod.rs:18181-18216` for `Int`/`Bool`/`Table`). **An `Int` return and
a `Record` return therefore select different arms by construction.** That is the
axis, and it is the axis §1e is organised around — not the callee.

**This is a scoping finding, not a diagnosis.** Naming the arm `nc22` selects is
`D2` and is deliberately not attempted here; §1e's own warning is that the
message localizes nothing.

## 5. Corpus census, so the population is in the claim

`callee: Box::new(RuntimeExpr::{Closure,LexicalClosure})` across `crates/`:
**63 construction sites in 8 files.**

| file | sites |
|---|---|
| `lowering/core/tests/control.rs` | 28 |
| `lowering/core/tests/constructors.rs` | 12 |
| `planning/static_transition.rs` | 10 |
| `lowering/core/tests/effects.rs` | 9 |
| `object_linker_packaging.rs` | 1 |
| `platform_runtime_support.rs` | 1 |
| `ir.rs` | 1 |
| `artifact/api/tests.rs` | 1 |

The `artifact/api/tests.rs` count of **one** confirms the Adversary's census
*for that file* — `nc22` is alone there. The census understated the corpus
because the shape's other `nc` instance lives in `ir.rs` production code, where
a fixture sweep would not look.

### 5.1 A third instance worth naming, because `#7` deletes its lane

`object_linker_packaging.rs:3033`, in
`process_artifact_maps_exitcode_and_reports_terminal_traps` — **measured green
at this base** — builds a `Match` whose *scrutinee* is a `Call` with a
`LexicalClosure` callee, and executes it to a linked process artifact. Its own
comment reads:

> *"This producer Match is the retained RecursiveDescent sibling."*

⇒ A second **executing** instance of the shape, in a different composition
(closure-callee call in scrutinee position rather than tail position), currently
green **on the lane `RT-DESCENT-RETIRE` deletes**. It is not this node's row,
and I am not claiming it fails on the functionized lane — only that it is a
member of the population `#8` says would silently narrow, and no one has
enumerated it.

## 6. Scope disposition — the node is NOT mis-sized on the stated ground

The frame offers one basis for a re-cut: that answering `D1` *"requires
authoring fixtures that do not exist"*. **That basis does not hold.** A second
instance exists, is green, and differs from `nc22` on exactly the axis that
selects the decoder — so `D1` is answerable from the corpus, and is answered
above, without authoring anything.

**I am therefore not returning a mis-sizing**, and I am not absorbing one
either: what I am returning is that the premise recorded in **two Steward-owned
documents** is false, which is the Steward's to correct.

What remains genuinely uncovered is narrower than the frame feared, and it is
`AC-2`'s "closed or explicitly reported" rather than a re-cut:

- the failing cell is **one** return shape (`Record`), measured;
- the passing cell is **one** return shape (`Int`), measured, subject to §3.2;
- the `Bool`, bare-constructor and `Boundary` return cells crossed with a
  closure-callee call are **not** instantiated anywhere in the corpus.

⇒ **Sizing recommendation: `M` stands for `D2`–`D4` against `nc22`.** Whether
the three uncovered cells are authored here or reported under `AC-2` should
follow `D2`'s arm, because if the arm is shape-selected then covering them is
three small fixtures, and if it is `TrapOnly` or an unrecognized `Boundary` tag
then §7's hard stop is live and the question moves anyway.

## 7. Scope

No repair, no fixture, no `D2`. `nc22` untouched — not un-skipped, not
narrowed, not re-routed. `nc22_program_with_body` untouched, so
`a_package_backed_program_without_a_role_record_refuses_before_lowering` and
`the_synthetic_entrypoint_consumes_the_authority_it_is_given` are unperturbed
and unread. No decode-surface change.

Disk stayed between 8G and 12G free; no failure this turn was non-compile, so
no `df` triage was owed beyond the checks recorded.

Provenance: Ken-owned frames, tracker, this repository's own source and these
measurements only. No `local/refs/`, permissive, copyleft or excluded-prototype
contact.
