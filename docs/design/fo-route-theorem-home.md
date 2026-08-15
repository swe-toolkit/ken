# Route FO: the home of the certificate machinery and the two theorems

**Architect design ruling, 2026-08-15.** Settles the *artifact home* half of
`spec/20-verification/23-prover.md §4.4`, which assigns the placement to the
Architect and the operator jointly. **It does not settle the evaluator/TCB
posture**, which stays with the operator; §4 below says why that is the
right split and what evidence it should be decided against.

This ruling does not reopen `OQ-12`. Reflective route (a) is decided, on
intrinsic merits, and everything here assumes it.

## 1. The ruling

**`IForm`, `Form`, `Cert`, `Derivation` are ordinary Ken inductives, and
`check_cert`, `check_tree`, `embed`, `denote`, `classically_valid` are ordinary
Ken definitions, in Ken source in a library. `embedding_adequacy` and
`checker_soundness` are Ken theorems, proved and kernel-checked — not
postulated, and not implemented in Rust.**

No `declare_primitive`. No `declare_postulate`. No new kernel file.

## 2. Why this is confirmation, not choice

The spec has already fixed most of it, and reading it that way is cheaper than
re-deriving it:

- **`check_cert` is specified as Ken-level.** `23 §4.3`: *"a Ken-level total
  function over ordinary derived data, distinct from the kernel API `check` in
  `18 §4`."* A Rust implementation would contradict the chapter that defines it.
- **`classically_valid` needs no classical axiom.** It is defined
  proof-theoretically — `classically_valid q := Derives([] => [q])` with
  `Derives(s) : Omega := || Derivation(s) ||` — and `23 §4.3` states the point
  explicitly: *"It is proof data, not an assumed classical oracle."* The obvious
  worry, that a route named "classical" drags excluded middle into a logic where
  `16 §1.3` does not assume it, does not arise.
- **`Cert` is data the adapter must produce.** `23 §4.3`: *"A solver proof
  format has no authority: an adapter must produce this `Cert` or the outcome is
  `unknown`."* That is a Ken datatype by construction.

What remained genuinely open was the *consequence* — and that turns out to be
the strongest argument for the placement.

## 3. The trusted-base accounting: zero new entries

`18 §4.1`/`§5` fixes what `trusted_base()` contains:

> returns **exactly** the registered primitives + admitted postulates —
> excluding the prelude and **excluding definitions/inductives, which are
> re-checked rather than trusted.**

Under §1, every artifact route FO introduces is a definition or an inductive.
So:

| artifact | kind | enters `trusted_base()`? |
|---|---|---|
| `IForm`, `Form`, `Cert`, `Derivation` | inductives | no — re-checked |
| `check_cert`, `check_tree`, `embed`, `denote` | definitions | no — re-checked |
| `classically_valid` | definition over a truncation | no — re-checked |
| `embedding_adequacy`, `checker_soundness` | **proved** theorems | no — re-checked |

**Route FO adds nothing to the trusted base.** That is the whole cost answer to
the placement question, and it is a strong result: a solver-backed discharge
path that enlarges the TCB by zero entries.

Two consequences worth stating because they are what a Rust home would have
cost:

- **No collision with `18a`'s do-not-reopen guardrail.** `18a §(5)` pins the
  `prim_reduce` path as tier-b tested-not-trusted, with *"no `eq → Eq`
  reflection bridge and no evaluator dependency in the kernel"*, so that a
  curated-crate bug is *"a wrong value, never a false proof."* A Rust
  `check_cert` consulted by conversion would invert exactly that property: an
  implementation bug would become a **false proof**. The guardrail is
  spec-level and marked do-not-reopen; §1 stays clear of it rather than
  arguing with it.
- **No dependency on K3.** Kernel-executed reductions are a *separate, later*
  decision (`18 §5`: *"Promoting those operations into conversion would enlarge
  the kernel TCB and is the separate K3 decision"*). A primitive home would have
  made route FO wait on K3 or pre-empt it.

**Postulating the two theorems is not an alternative.** `23 §4.4` already
forbids the outcome — *"Until both theorems are kernel-checked in an approved
home, route FO cannot return `proved`"* — and admitting them would put two
load-bearing metatheorems into `trusted_base()`, which is the opposite of the
result above.

## 4. What this ruling does NOT settle, and why

The discharge shape in `23 §4.4` is

```
sound Sigma C rho f pi ok := embedding_adequacy Sigma C rho f
                               (checker_soundness (embed Sigma f) pi ok)
-- where ok : check_cert (embed Sigma f) pi = True
-- hence  sound Sigma C rho f pi (refl True) : denote C rho f
```

For `refl True` to typecheck, **kernel conversion must reduce
`check_cert (embed Sigma f) pi` to `True`** — the kernel has to actually
run the checker. That is what "discharge by computation" means, and `OQ-12`
bought it deliberately.

So the accounting in §3 is complete as an **entry count** and incomplete as a
**risk statement**:

> **The TCB gains no entries, but it gains a new load class.** Route FO's
> soundness comes to rest on conversion executing a deep recursive function
> correctly and terminating.

Per `18 §6`, subject reduction is **"Argued ... to be mechanized"** and
confluence / unique normal forms is **"Argued"**. Route FO would therefore lean
on the argued-not-mechanized part of the kernel, at a computation depth nothing
else in Ken currently exercises. Idiomatic Ken code does not ask conversion to
run a proof checker.

**This is not an argument against route (a)**, and it is not a reason to prefer
a Rust home — a Rust home is strictly worse on this axis, since it moves the
same computation *outside* the re-checked region. It is the honest statement of
what the reflective route costs, and it identifies the one thing worth
measuring.

### SETTLED 2026-08-15 BY THE OPERATOR. Build it and measure it.

> Nothing ventured, nothing gained. We will only know the cost if we build it
> and test it on real programs, so we should do that.

**The posture this note left open is decided: the new load class is accepted as
something to measure, not to pre-empt.** `18 §6`'s subject reduction and
confluence do **not** have to be mechanized before route FO is built and
exercised.

**Two things this does not clear.** `23 §4.4` still forbids `proved` until both
theorems are kernel-checked in an approved home — a precondition, not a
decision, and both are unproved and unstarted. And the cost remains **unmeasured**;
predicting a blowup is not measuring one.

**The measurement does not need either theorem.** `refl True` at
`check_cert (embed Sigma f) pi = True` forces exactly the conversion work in
question; `embedding_adequacy` and `checker_soundness` turn that computation
into a *discharge* rather than making it expensive. **So the number is
obtainable well before the metatheory is proved.** Framed as
`V3-FO-CONVERSION-LOAD-MEASURED`, which depends on
`V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` because nothing real reaches route FO
until then.

### The evidence this should be decided against

`OQ-12` records its own residual risk as *whether the adequacy +
checker-soundness metatheory mechanizes cleanly — a **feasibility** risk,
retired by a thin front-loaded (a) slice.* **`V3-FO-KRIPKE-SLICE` is that
slice.** So the posture decision does not have to be made in the abstract; it
can be made against the slice's evidence, which costs only sequencing.

One obstacle, already routed: route FO's accepted-certificate path and its
"nothing could establish this" path both converge on `emit_unknown_hole` with
the audit label `"prover unknown goal"`, so the two are indistinguishable in
`trusted_base()`. The count the posture decision most needs — how often route FO
would actually discharge a real obligation — is generated at that line and
discarded. `V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` carries the fix.

**When reading that count: a large withheld count is evidence that route FO
would discharge real work. It is NOT evidence that those obligations are closer
to proved.** The label records provenance, not strength; the postulate is
admitted either way.

## 5. Two masking gates

Route FO cannot return `proved` today for **two independent reasons**, and they
hide each other:

1. `attempt_fo` mints a fresh `FoSliceSignature` per call, so `quote_fo` refuses
   every externally-constructed obligation and the route falls through to the
   IPC fallback.
2. `23 §4.4` forbids `proved` until both theorems are kernel-checked in an
   approved home.

Gate 1 does all the visible work today; **gate 2 has never been under load.**
`V3-FO-OBLIGATION-SIGNATURE-DISCOVERY` removes gate 1 — at which point §4.4
becomes the only thing between an accepted certificate and `proved`, for the
first time, inside a node whose subject is signature matching. That is why its
`D0` must state the §4.4 interaction as part of the soundness obligation, and
its mutation must be aimed at the §4.4 gate rather than at quotation: a test
proving only that quotation started working leaves the last remaining gate
unmeasured.
