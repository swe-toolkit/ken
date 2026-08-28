---
id: V3-FO-KEN-LEVEL-CHECKER-AUTHORING
title: "Author the Ken-level check_cert, embed, Form and Cert so the conversion cost 23 section 4.4 names becomes measurable at all, and take that measurement"
status: merged
owner: language
size: L
gate: none
depends_on: [V3-FO-CONVERSION-LOAD-MEASURED]
blocks: [V3-FO-CHECKER-SOUNDNESS]
github: null
origin: "Steward scope call evt_6m3q3tsvg09pz, 2026-08-15, on Architect review evt_7cmys9wyp7k8c of V3-FO-CONVERSION-LOAD-MEASURED at b52d160c8. The absence of check_cert from library/ and catalog/ was verified by the Steward against the tree at origin/main 30ee4dbf1 before filing. Steward-filed per COORDINATION section 2."
---

## Why this exists: the predecessor's `AC-2` named a quantity nobody can take

`V3-FO-CONVERSION-LOAD-MEASURED` `AC-2` required *"the measurement is of kernel
conversion."* Its cost model was explicit — kernel conversion must evaluate
`embed Sigma f` and then run `check_cert` over the whole derivation tree, forced
by `refl True` at

```
ok : check_cert (embed Sigma f) pi = True
```

**That measurement is not takeable, because the artifact does not exist.**
**Re-verified at `origin/main` `6e3b58009`, 2026-08-16:**
`grep -rn check_cert library/ catalog/` **still returns zero.** The only
`check_cert` in the tree is `ken_elaborator::fo_kripke::check_cert`, a **native
Rust** function recursing through `check_tree` over Rust `Cert`/`Sequent`
structs. `embed` is likewise Rust-side.

`conformance/verify/prover/seed-prover.md:49-50` states the distinction
normatively:

> `check_cert` = the **Ken-level reflective Bool checker** over quoted formulas
> (`23 §4` route (a)) — **an ordinary kernel-checked function, distinct from the
> kernel API `check`**.

⇒ **The predecessor measured the Rust reference checker.** Its `AC-2` was
amended to say so and to report the gap as a result; **this node is where the
gap is closed.**

## The Steward's framing error, recorded because the shape keeps recurring

The predecessor's frame argued the number was obtainable now because the
equation *"requires NEITHER theorem"* — true, and about the **metatheory** axis.
**It was read as clearing the way generally.** Nobody checked whether the thing
being measured existed.

> **A warrant that reaches one axis, read as reaching another.** The same shape
> cost lane 1 twice in the same campaign: a limit stated on the **instrumented
> sites** was read as a limit on the **population**.

## What this node must not be read as authorizing

**`23 §4.4` forbids `proved` until `embedding_adequacy` and `checker_soundness`
are kernel-checked in an approved home.** Both are unproved, unstarted, and have
no node. **Authoring the checker does not touch that**, and a working
`check_cert` in Ken is **not** grounds for a `proved` verdict.

**This node builds and measures. It does not discharge.**

## Fixed inputs, measured at `origin/main` `6e3b58009`, 2026-08-16

**The coordinates in the original filing were taken at `30ee4dbf1` and have
moved. These are the current ones. Re-measure before you baseline; do not cite
this table's line numbers from memory once your branch is anchored.**

| artifact | `crates/ken-elaborator/src/fo_kripke.rs` |
|---|---|
| `pub enum IForm` | `:382` — **five constructors**: `Bottom`, `Atom(IVar)`, `Or`, `Imp`, `Forall` |
| `pub struct Sequent` | `:465` — `{ gamma: Vec<Form>, delta: Vec<Form> }` |
| `pub enum Rule` | slice subset, **three variants**: `Init { left, right }`, `ImpRight { right }`, `ForallRight { right, eigen }`. **The general `23 §4.3` `Rule` has ~20.** |
| `pub struct Cert` | `:489` — `{ conclusion: Sequent, rule: Rule, children: Vec<Cert> }` |
| `pub fn embed` | `:851` |
| `pub fn check_cert` | `:861`, recursing through `fn check_tree` `:866` |
| `pub fn find_certificate` | `:999` |
| `pub fn quote_fo` | `:575`; `pub fn discover_and_quote_fo` `:348` |

**`attempt_fo` has moved and changed since the DAG text describes it.** It is
`crates/ken-elaborator/src/prover.rs:550` — **not** `ken-verify`, and **not**
a bare `attempt_ipc` passthrough. It discovers, quotes, searches, checks, and
returns an honest `Unknown` through `emit_unknown_hole_fo_withheld` when a
certificate is genuinely accepted, falling back to `attempt_ipc` only when
discovery or search fails. **The `Unknown`-not-`Proved` fail-safe is
`attempt_fo_with_signature`'s documented contract; `AC-4` protects it.**

## The increment cut — this is an `L` and an `L` cannot be released whole

**Dispatch ONE increment per turn** (`§4b`, the one-hour turn). Each is a
releasable candidate on its own:

| increment | deliverables | why it stands alone |
|---|---|---|
| **1** | `D0` + `D1` | types plus `embed`; testable against the Rust `embed` on quoted inputs before any checker exists |
| **2** | `D2` | `check_cert` itself, the largest single piece |
| **3** | `D3` | the differential control, which needs 1 and 2 and nothing else |
| **4** | `D4` + `D5` | the measurement and its termination report, which need a working checker |

**A hard stop inside any increment is a good outcome and is reported as one.**
Do not carry an unfinished increment into the next turn to make it look whole.

> ### `Form` IS STRICTLY LARGER THAN `embed`'s IMAGE, AND
> ### `check_cert` IS TOTAL OVER IT
>
> **Read `fo_kripke.rs`'s own doc comment above `IVar` before authoring `D2`.**
> `Form`/`QTerm` are **untyped**, and `check_tree` performs **no sort
> validation**: a hand-built ill-sorted target — a world eigenparameter in an
> object slot — **closes and returns `true`**, because `Init` needs only
> syntactic `Form` equality.
>
> **The safety mechanism is at the CALLER.** `quote_iform` admits only an
> in-scope object `Var` of the declared sort, so every `IForm` it produces
> carries object-sort indices only; the malformed formulas live entirely in
> `Form`'s excess and **no `IForm` maps to them.**
>
> ⇒ **Two consequences, both load-bearing.** A faithful Ken `check_cert`
> **inherits this property**, and that is correct — it must match the
> reference, not improve on it. And **`D3` agreeing does not certify
> sort-safety**; it certifies agreement. Whether the checker should validate
> sorts itself is [[CORE-FO-CHECK-TREE-SORT-VALIDATION]]'s question and is
> banned here.

## Deliverables

**`D0` — the quoted-syntax types in Ken.** `Form` and `Cert` as Ken inductives
matching `23 §4.3`'s slice subset, with the Rust `IForm`/`Cert` as the
reference for shape only. **State explicitly which Rust constructors have no Ken
counterpart and why**, rather than silently narrowing.

**The counts make that statement checkable:** `IForm` has **five**
constructors, `Rule` **three** slice variants against the general `§4.3`
`Rule`'s ~20. **A Ken `Rule` with three variants is correct and is the slice;
one with five or twenty is out of scope** (`23 §4.5`, and Banned scope below).
Say which of the ~20 you are not carrying — naming the gap is the deliverable,
not closing it.

**`D1` — `embed` in Ken**, the quoted-formula-to-proposition map, matching what
the Rust `embed` computes on the same inputs.

**`D2` — `check_cert` in Ken** as an ordinary kernel-checked total function,
per `seed-prover.md:49-50`. **Not a primitive, not an axiom, not a kernel API
call.**

> ### THE FAILURE ENCODING IS RULED. DO NOT RE-OPEN IT.
>
> **Architect `evt_5fc6hsgcn9exq`**, ruling on a fork the Steward routed
> (`evt_2g2j03b29wbq9`). **The answer is neither arm that was offered.**
>
> > **Ken `check_cert` is a total `Bool` mirroring the Rust's guarded
> > structure.** No `Maybe`/`Either` verdict wrapper, and **no change to the
> > Rust reference.** Where Rust writes `delta.get(i)` returning `Option`, Ken
> > writes a lookup returning `Maybe FokForm` **internally**, and every
> > `Nothing` maps to `False` — exactly what `else { return false }` does. **The
> > `Maybe` is a local detail of the lookup, never the shape of the verdict.**
>
> **The premise that made this look hard was withdrawn by its own author.**
> `check_cert` does **not** inherit `w_forces`'s partiality — it was written to
> the opposite discipline and already is in the tree. **Steward-verified:**
> `check_tree` contains **zero** `unwrap`/`expect`/`panic!`/`unreachable!`, and
> its only two bare index writes (`expected_delta[*right]`, in `ImpRight` and
> `ForallRight`) sit on a `clone()` of the very `Vec` whose `.get(*right)`
> returned `Some` two lines above — **cannot be out of range.**
>
> ⇒ **`check_cert` is total in BEHAVIOUR, not merely `-> bool` in signature.
> Mirror its structure and totality comes for free.**
>
> **`w_forces` remains the single documented exception and stays as it is.**
> Making the Rust side total would be the barred kind of reference change and
> would **lose information** — that panic marks a caller bug, with
> `quote_iform` as the real safety mechanism, and a defaulted `Zero`
> substitutes a silently wrong answer for a loud one. **Document the domain; do
> not force agreement.**
>
> **`search` shares the guarded discipline** — Steward-checked, since the
> Architect flagged it as unaudited: no panics, and its `delta[j]` writes take
> `j` from `enumerate()`, in range by construction. Relevant to `D4`.

> ### TWO CONSTRUCTORS CAN BE SWAPPED SILENTLY, AND `D2` IS WHERE THEY ARE FIRST READ
>
> **Adversary hunt on increment 1, `evt_2z1b5v3k0d8yr`.** This is the specific
> form of Architect finding 3's *"well-formed and correctly-sized, not shown
> right."*
>
> | constructor | shape | swap detectable? |
> |---|---|---|
> | `FokMkSequent (List FokForm) (List FokForm)` | two same-typed | **NO** |
> | `FokInit Nat Nat` | two same-typed | **NO** |
> | `FokForallRight Nat FokQTerm` | different types | yes — will not type-check **(SUPERSEDED — see below)** |
> | `FokMkCert FokSequent FokRule (List FokCert)` | all distinct | yes |
> | `FokAccess` / `FokDomainA` / `FokForcingP` | two same-typed | yes — **`embed` constructs them**, so `D1` already compares them positionally |
>
> ⇒ **Exactly two constructors are BOTH same-typed-adjacent AND outside
> `embed`'s image.** A `gamma`/`delta` or `left`/`right` swap **elaborates,
> kernel-checks, passes both arity tests, and is invisible to the differential**
> — because the differential only ever sees what `embed` produces, and `embed`
> produces neither.
>
> ### SUPERSEDED FOR `FokForallRight` — its shape changed
>
> **The table row above is HISTORY.** `V3-FO-SORTED-EIGENPARAMETER-DERIVATION`
> replaced the eigenterm with a parameter INDEX, so the constructor is now
> **`FokForallRight Nat Nat` — two same-typed fields, and a `right`/`eigen` swap
> TYPE-CHECKS.** The "different types, will not type-check" consequence no longer
> holds, and the count above is a count under the old shape.
>
> **This is stale prose, NOT a missing control.** The order is pinned
> BEHAVIOURALLY instead: the existing differential lawful rows carry `right=0`
> with `eigen=2`, so a swap changes the derived verdict rather than passing
> silently. **Do not open a control-gap node from this row** — check the
> behavioural pin first, and only if it is absent is there work here.
>
> **The mechanism is the positional mirror of a named Rust struct.** Rust's
> `{ gamma, delta }` and `{ left, right }` make a swap a **compile error**;
> Ken's positional constructors make it a **silent** one. And `Init` closing on
> `gamma[left] == delta[right]` is symmetric-looking enough that a swapped
> checker **can still accept the positive control.**
>
> **Required in `D2`, cheapest form:** one differential row that constructs a
> `FokMkSequent` and a `FokInit` with **distinguishable** contents and decodes
> them back, **pinning field order before anything reads it.**

> ### DO NOT SPEND A PASS ON `cases()`'s MISSING NESTING ORDER — IT IS REDUNDANT
>
> **Adversary measured this so nobody repeats it.** `cases()` has
> `Forall`-in-`Forall` and `Imp`-in-`Forall` but **nothing with `Forall` inside
> `Imp`** — visibly the one absent composition. They added
> `Forall(Imp(Bottom, Forall(Atom(IVar 1))))`, and it passes.
>
> Then they tested whether it **discriminates**, by mutating the `Forall` arm's
> recursive world (`Suc Zero` → `Suc world`): **an existing case
> (`forall_nested_inner_ref`) catches that mutation and the new row does not** —
> under an `Imp` the world is already `0`, so `Suc world` and `Suc Zero`
> coincide there.
>
> ⇒ **The proposed row is strictly weaker on the world axis and equal on the env
> axis. The population is better than its enumeration looks.** The gap is
> visible from reading `cases()`; **the redundancy is not**, which is exactly why
> it is recorded here.

**`D3` — a differential control against the Rust checker.** For every
certificate the predecessor's corpus produced, the Ken `check_cert` and the Rust
`check_cert` must agree. **A disagreement is the most valuable result this node
can produce and is reported as one.**

> **The predecessor's corpus is the FLOOR of `D3`'s input, not its extent.** Two
> further populations are required, neither derivable from it: the **rule-shape
> near-miss pairs** and the **equality-field near-misses**, both specified below.
>
> ⚠ **Stated because this deliverable and `AC-3` were keyed to the predecessor's
> corpus ALONE**, so a candidate could run the differential over it, agree, and
> **build no pair at all** while satisfying both as written. The near-miss
> requirement lived only in the blockquotes. `AC-3` now enforces all three.

> ### `D1`'s BLIND SPOT WAS FORCED. `D3`'s WOULD BE A CHOICE.
>
> **Architect `evt_5fc6hsgcn9exq`.** `D1`'s `cases()` excluded ill-scoped inputs
> **because the Rust `w_forces` would panic on them** — the exclusion was not a
> judgment call, it was the only option.
>
> **No such pressure exists for `check_cert`.** The Rust returns `false` on
> every malformed certificate, so malformed inputs are **perfectly comparable**.
> ⇒ **If `D3` ends up blind at the rejection boundary, that is a decision
> someone made, and it must be defended rather than inherited from `D1`.**
>
> ### AGREEMENT ON `false` IS WEAK EVIDENCE BY DEFAULT
>
> **Two implementations can agree on `false` while rejecting for different
> reasons.** A corpus of malformed certificates can therefore report **full
> agreement while neither side ever reaches the arm under test** — a control
> that looks strongest exactly where it is emptiest.
>
> **The remedy is a NEAR-MISS PAIR per rejection arm:** one certificate the arm
> rejects, and a **minimally different** one it accepts. **The accepting half is
> the load-bearing one** — it proves the traversal reached the arm at all, which
> agreement on `false` never shows.
>
> **These seven are the FLOOR, not the ceiling** (Architect's list, carried
> verbatim in substance):
>
> | arm | rejecting case | accepting near-miss |
> |---|---|---|
> | `Init` | `left`/`right` in range, formulas differing **in one field of a shared constructor** — not merely unequal | in range and equal |
> | `Init` | `left` or `right` out of range | in range |
> | `ImpRight` | `delta[right]` present but not an `Imp` | an `Imp` |
> | `ImpRight` / `ForallRight` | zero children, and two children | exactly one |
> | `ForallRight` | target neither `ForallWorld` nor `ForallObj` | a quantifier |
> | `ForallRight` | eigenparameter already mentioned in the conclusion | fresh |
> | root | conclusion sequent unequal to `[] => [q]` | equal |
>
> ### FIRST TASK OF `D3`: TWO OF `D2`'s TEN REJECTION CASES ARE DOUBLY MALFORMED
>
> **Architect `evt_2ee9qfch79vgg`, on the merged `D2`. Non-blocking there,
> load-bearing here, because the RULE-SHAPE pairs above get BUILT from those ten
> cases.**
>
> **Only the rule-shape ones.** The equality-field pairs required by the block
> two below are **not** derivable from this population — see *"EQUALITY-FIELD
> NEAR-MISSES ARE A SECOND AXIS."* Repairing these ten is necessary and **not
> sufficient**, and this sentence said otherwise until the Adversary measured it.
>
> `imp_right_target_not_imp` and `forall_right_target_not_quantifier` each supply
> `(Nil FokCert)` — **zero children** — alongside the wrong-shaped target. Each
> is therefore rejected by **two independent guards**, so the row proves the
> certificate is rejected and **not that the named guard is what rejected it**.
>
> **The fix is one token each: give both a single well-formed child.** Do it
> before deriving any pair from them.
>
> ⇒ **A pair derived from a doubly-malformed base inherits the ambiguity**, and
> the failure is self-concealing: the arm that then looks best-covered is the one
> whose coverage is weakest. That is the same defect this section already warns
> about one level up — agreement on `false` proving nothing about which arm was
> reached — arriving through the base case instead of through the corpus.
>
> **The other eight isolate cleanly.** `init_nonempty_children` is the best of
> the set: its indices would otherwise pass, so the children guard is the sole
> difference. `imp_right_zero_children` and `imp_right_two_children` are already
> accidental near-miss pairs against the acceptance test, differing from the
> accepted `cert1` only in child count — the shape this section asks for, arrived
> at by accident. **Keep them and say so; do not rebuild them.**
>
> ### EQUALITY-FIELD NEAR-MISSES ARE A SECOND AXIS. THE SEVEN PAIRS MISS IT.
>
> **Adversary `evt_4zatk7s32e74k`, measured on the merged `D2` by mutation.
> Two arms of `fok_form_eq` were killed with all seven tests GREEN:**
>
> | mutation | suite |
> |---|---|
> | `FokForcingP b1 b2 ↦ fok_qterm_eq a1 b1` — drop the **object** slot | **7 passed** |
> | `FokImp b1 b2 ↦ fok_form_eq a1 b1` — drop the **consequent** | **7 passed** |
>
> ⚠ **This is the soundness direction at the one place it matters.** `FokInit`
> closes on `fok_form_eq g d`. Under the first mutation `Γ, Force_P w x ⊢ Δ,
> Force_P w y` **closes by `Init`** for any `x ≠ y` — an invalid sequent
> accepted. **The code as landed is correct; the controls cannot tell that it is.**
>
> **Why the `D2` population is blind, precisely:** the ten malformed cases
> exercise `False` by giving a rule the **wrong-shaped target**, and
> `accepts_genuine_derivations` exercises a **genuinely valid** certificate.
> Nothing exercises one that is **nearly valid in the equality dimension**.
>
> ⇒ ***The seven pairs above are RULE-SHAPE near-misses and this is not one.***
> An equality-field near-miss is not derivable from the malformed-case
> population, so `D3` as originally framed would **not** have closed it. **The
> Steward's row 1 said only "unequal formulas"**, which any cross-constructor
> pair satisfies — and `fok_form_eq` is fully enumerated 9x9 with every
> cross-constructor arm a literal `False`, so those are the arms that cannot be
> wrong. **Over-acceptance can only come from a same-constructor arm ignoring a
> field**, which is why row 1 is now qualified.
>
> **PREFER THE ORACLE OVER ONE ROW PER ARM.** A single row killing the
> `FokForcingP` mutation is cheap and insufficient. **The durable form is
> already `D3`'s shape:** have the differential compare `fok_form_eq a b`
> against **Rust's derived `PartialEq`** over pairs that include field-level
> near-misses. `fok_form_eq` is hand-authored exactly where Rust *derives*, so
> the derivation is a real oracle — the same relationship `shift` had to
> `mentions_var0` — and **it covers all eight multi-field arms at once instead
> of one row each.** An oracle built from the thing the hand-written code
> mirrors cannot drift from it.
>
> ### `D2`'s TOTALITY CLAIM: ONE HALF VERIFIED, ONE HALF WAS NEVER TRUE
>
> The `D2` handback stated that `fok_list_form_set_nth` and
> `fok_list_form_append_one` are *"total, documented no-op out of range,
> unreachable at every call site since each is gated by a prior `fok_nth_form`
> success at the same index."* **The Steward repeated that grouping in the merge
> notification without opening either function.**
>
> **`set_nth` holds** — Adversary-checked at both call sites rather than from the
> comment: `FokImpRight` under `match fok_nth_form delta right { Some (FokImp p
> q) ↦ … }`, and `FokForallRight` under the same gate with `right` threaded into
> `fok_check_forall_right`. Same index both times.
>
> **`append_one` has no out-of-range case at all.** It takes **no index**, and
> its `Nil` arm is `Cons v (Nil)` — a correct append. So *"gated at the same
> index"* is not merely unverified for it, it is **meaningless**, and a reader
> auditing *"the out-of-range no-ops"* would go looking for a second one that
> does not exist.
>
> ⇒ **The defect is that two functions were described by one sentence.** Neither
> the ring, the Architect, nor the Steward opened `append_one`, because the
> sentence read as a single verified claim. **Same shape as the census: a claim
> reproduced rather than derived.**
>
> ### A KNOWN VERDICT-EQUIVALENT DIVERGENCE — DO NOT REPORT IT AS A DISAGREEMENT
>
> **Ken checks `ForallRight` freshness BEFORE the exactly-one-child guard; Rust
> checks the child count first.** Both are total and side-effect-free, so the
> verdict is identical on every input.
>
> Recorded here because `AC-3` makes a disagreement the most valuable result this
> node can produce, which is exactly the reading under which a reviewer flags an
> ordering difference as one. **It is a deliberate divergence, reviewed and
> accepted** (Architect, same event, arm-by-arm against `fo_kripke.rs`).

**`D4` — the measurement `AC-2` originally named.** Wall-clock and, where
obtainable, reduction-step count that **kernel conversion** spends on
`check_cert (embed Sigma f) pi = True` via `refl True`. **Report the
distribution and the worst case, not an average**, and state the build profile.

> **Carry from the Architect's non-blocking review of
> `V3-FO-SEARCH-FUEL-STACK-AGREEMENT` (merged #2393).** That node's fuel/stack
> comment **does not state the fuel the
> probe used**, so the number it records cannot be re-derived and a
> re-measurement needs a fuel override to reproduce it. **State the fuel this
> node's measurement runs under, in the artifact itself** — a measurement whose
> budget is not written down is not re-takeable, which is the same defect one
> level up.

**`D5` — termination, reported honestly.** Whether conversion terminated on
every case. `docs/design/fo-route-theorem-home.md` §4 names this the load class
that matters, leaning on the argued-not-mechanized half of `18 §6`. **A
non-terminating or pathological case is a result, not something to work
around.**

> ### TWO THINGS `D3` LEFT LOADED FOR `D4`/`D5`. NEITHER IS A DEFECT TODAY.
>
> **Architect `evt_2jrv6bkbf541t`, on the approved `D3`. Both are inert now and
> both fail silently later, which is why they are written down rather than
> fixed.**
>
> **1. `placeholder_child` is itself a REJECTING certificate.** It is
> `(FokMkCert (FokMkSequent (Nil FokForm) (Nil FokForm)) (FokInit Zero Zero)
> (Nil FokCert))` — an `Init` indexing into two empty lists.
>
> It is **inert at all three of its current sites**, verified individually: each
> traversal rejects before recursing into children (wrong-shaped target at two,
> wrong child count at the third), so its own invalidity is never evaluated. **No
> current row is over-determined by it.**
>
> ⇒ **The moment it is used in a position the traversal actually reaches, that
> row is rejected by two independent mechanisms** and proves only that rejection
> happened. **That is exactly the `D2` defect this node already repaired, re-created
> by a helper named for convenience** — its name says where it goes, not what it
> is. Either give it a valid derivation or rename it
> `rejecting_child_never_reached`.
>
> **This is the THIRD appearance of the over-determination shape on this node.**
> Two doubly-malformed cases at `D2`, and now a helper that is one use-site away
> from the same thing. **Treat the shape as recurring rather than the instances
> as unlucky.**
>
> **2. The serializer is unprotected on TWO of its THREE axes. MEASURED, and it
> corrects this node's own earlier claim.**
>
> **Adversary `evt_10dxqtbsf4tcw` mutated all three axes rather than reasoning
> about them:**
>
> | mutation | result |
> |---|---|
> | `sequent_source`: `gamma`/`delta` swapped | **RED** — protected |
> | `rule_source`: `FokInit` `left`/`right` swapped | **9 passed — NOT protected** |
> | `qterm_source`: `FokQBound`/`FokQParameter` swapped | **9 passed — NOT protected** |
>
> ⚠ **This node previously stated that `Init { left: 1, right: 0 }` protects the
> `rule_source` axis because `left != right`. THAT IS FALSE.** The argument is
> plausible and the measurement refutes it: swapping the serializer's two `Init`
> indices passes every test. **It was offered here as measured when it was
> reasoned.** The `gamma`/`delta` protection is real and did red.
>
> ⚠ **The third axis is not a field order at all, and nobody named it.**
> `qterm_source` can emit `FokQParameter` where the Rust held `Bound` — a
> **semantic class confusion in the shared component**, not a transposition.
>
> **Why it survives is the part worth having.** A **uniform** relabeling is a
> structure-preserving bijection, and `fok_form_eq`/`fok_qterm_eq` are purely
> **structural** — so every equality is preserved and the Ken side checks a
> *different formula* to the same verdict. ⇒ **The one function that should catch
> it is `fok_sequent_mentions_parameter`, which is NOT structural** — it looks
> specifically for `Parameter`. **That it still passes means the corpus's
> freshness cases do not discriminate a bound variable from an eigenparameter.**
>
> ### SEVERITY IS CONTROL-VALIDITY, NOT SOUNDNESS — AND `D4` IS WHERE IT BITES
>
> `FoKripke.ken` is untouched and correct. **What is weakened is the
> differential's own claim:** *"Ken and Rust agree on these 18 cases"* becomes a
> statement about a **different** 18 cases if the serializer is wrong — and
> **`D4` would measure conversion cost on the wrong input for the same reason.**
>
> **TWO PINS CLOSE ALL THREE AXES, and both are cheaper than auditing the
> population for asymmetry. `D4` OWES THEM BEFORE IT MEASURES:**
>
> 1. **one `Init` case with `left == right`** on sequents where the two readings
>    disagree — kills the `left`/`right` swap **without relying on the corpus
>    staying asymmetric**;
> 2. **one `ForallRight` freshness case whose eigenparameter number collides
>    with a live `Bound` index** — a uniform `Bound`/`Parameter` relabel then
>    changes the freshness verdict, which is **the only place the distinction is
>    semantic**.
>
> ⇒ **Pin the property; do not preserve the accident.** The earlier remedy here
> was *"do not normalize the population to symmetric sequents"*, which asks every
> future increment to protect an invariant nothing states. **These two pins make
> the population's shape stop mattering.**
>
> ### THE SHAPE HAS NOW OCCURRED FOUR TIMES ON THIS NODE. DESIGN `D4` AGAINST IT.
>
> **One shape, four instances — a property that holds because of what the corpus
> happens to contain, with nothing making it hold:**
>
> | # | instance | how it was found |
> |---|---|---|
> | 1 | two doubly-malformed rejection cases rejected by two guards each | Architect, reading |
> | 2 | `fok_form_eq` arms droppable with the suite green | Adversary, mutation |
> | 3 | `placeholder_child` inert only because no traversal reaches it | Architect, reading |
> | 4 | serializer axes held by an asymmetric population | Adversary, mutation |
>
> **Note which column found which.** Instances 1 and 3 are visible by reading —
> a reviewer can see a case is over-determined. **Instances 2 and 4 were invisible
> to three readers and fell immediately to a mutation**, and in both the reasoned
> argument for safety was *wrong*, not merely unproven. **`Init { left: 1, right:
> 0 }` was reasoned safe by the Architect, carried by the Steward, and refuted by
> one swap.**
>
> ⇒ **For `D4`, treat "this control is discriminating" as a MEASUREMENT and never
> a reading.** The node's own history says a reading of a control's power is
> unreliable on exactly the cases where it matters most.

## Acceptance criteria

**`AC-1`.** `D4`'s measured interval contains **kernel conversion** and nothing
else. **Demonstrate it** — the predecessor's whole `AC-2` failure was a bracket
that contained no conversion at all, and it went unnoticed because the commit
message filed the fact under a different criterion where it read as a virtue.

**`AC-2`.** The Ken `check_cert` is **kernel-checked**, with no new primitive,
no trusted axiom, and no addition to `trusted_base()`.

**`AC-3`.** `D3` agrees with the Rust checker **on all three populations** — the
predecessor's full corpus, the rule-shape near-miss pairs, and the
equality-field near-misses — **or the disagreement is reported rather than
reconciled by changing either side to match.**

**The two near-miss populations must EXIST, and that is part of this AC.** They
are not derivable from the predecessor's corpus, so agreement on that corpus
alone discharges nothing.

> **Each rule-shape arm needs its ACCEPTING half**, which is what proves the
> traversal reached the arm; agreement on `false` never shows it.
>
> **The equality-field population needs a same-constructor pair differing in one
> field, per multi-field arm.** The `PartialEq` oracle satisfies this for all
> eight at once and is the preferred form. **A candidate that adds one row for
> the measured `FokForcingP` mutation and stops has closed the instance and left
> the class open** — seven other multi-field arms stay unprobed, and the two
> mutations that survived were found by testing, not by reading.

**`AC-4`.** No FO `Proved` verdict. `23 §4.4`'s reservation is untouched.

**`AC-5`.** Deep cases run on the oversized test-thread stack helper
(`run_with_big_stack`, in **five** `crates/` files at `6e3b58009`), **so a
harness stack limit is never reported as a mechanism property.** That confusion
is exactly what the predecessor's `D4` had to be corrected for.

**`AC-6`.** No-regression, in CI (`COORDINATION §12`).

## Banned scope

- **Proving `embedding_adequacy` or `checker_soundness`.** `checker_soundness`
  is now [[V3-FO-CHECKER-SOUNDNESS]], filed `ready` on this node's merge.
  `embedding_adequacy` remains unfiled — it needs `denote`, which does not
  exist, and it is that node's successor rather than this one's.
- **Emitting `proved` for FO**, on any basis.
- **Widening the slice** beyond `23 §4.5`.
- **Changing the Rust checker to make `D3` agree.** It is the reference.
- **Adding sort validation to the checker.** See the `Form`-excess block above;
  that is [[CORE-FO-CHECK-TREE-SORT-VALIDATION]]'s, and folding it in here would
  make `D3` disagree with the reference **by design**, destroying the control.

## Sequencing

**PROMOTED `draft` → `ready`, 2026-08-16. Both conditions the node set for
itself are met.**

**1. The two nodes it was held behind are merged.**
[[V3-FO-GUARD-SHIFT-DIFFERENTIAL]] (#2371) and
[[V3-FO-DISCOVERY-BOTTOM-OVERCOLLECT]] (#2375), both verified `status: merged`
at `6e3b58009`. Its `depends_on`, [[V3-FO-CONVERSION-LOAD-MEASURED]], is merged
too.

**2. The operator asked for the route (a) cost.** That was one of the two
triggers this node named, and it had **already fired when the node was filed** —
`docs/design/fo-route-theorem-home.md` §4, settled 2026-08-15:

> *"Nothing ventured, nothing gained. We will only know the cost if we build it
> and test it on real programs, so we should do that."*

**The same note names this node's exact subject as the obstacle:** *"the
definitions the theorems are about have not been authored either, which is what
makes the cost untakeable rather than merely untaken."* **That is `D0`-`D2`.**

> ### THE ZERO-POPULATION ARGUMENT DOES NOT BLOCK THIS, AND SAYING WHY MATTERS
>
> The original Sequencing reasoned: no Ken program produces an FO-quotable
> obligation, so a cost number for checking one **gates nothing**. **That is
> still true and it is still not a reason to hold the node.**
>
> The operator's settlement is *build it and measure it* precisely **because**
> the cost is unknown — *"predicting a blowup is not measuring one."* **A
> measurement commissioned to find out what something costs cannot be
> deferred for not yet having a caller**; the caller is what the measurement
> informs.
>
> **Gates-nothing is a statement about urgency. It was read as a statement
> about readiness.** The two came apart here, and lane 2 sat idle across the
> gap.

**This is lane 2 under the operator's 2026-08-15 two-lane directive** — the FO
Kripke embedding half. It does not contend with lane 1 (runtime, `RecursiveDescent`)
or with verify's [[V3-Z3-EMISSION-CONTROL]].

**Not a prerequisite for anything, and nothing waits on it.** A hard stop or a
pathological measurement is a complete result; see `D5`.

## Closed 2026-08-16. Two items ride past it, and neither is owed by this node.

**All six deliverables landed.** `D0`-`D1` `41b49d94a`, `D2` `261483836`,
`D3` `996ffbb93`, `D4`+`D5` PR #2421 — Architect approval `evt_63v4a4yyr4vte`,
Decision `dec_4kd13xyfwd5c6` resolved on exact `04af417d2`.

**`D4`/`D5` result:** kernel conversion of `fok_check_cert` is **four to five
orders of magnitude** more expensive than the Rust reference checker at matching
depths, with super-linear rather than the reference's roughly-quadratic growth.
The corpus is capped at `imp_chain` depth 8 and `forall_chain` depth 4 against
the predecessor's depth 64, and **that cap is reported as the `D5` reach result**
rather than worked around. Seven cases, all terminating and accepted, fixed
production fuel 200, 1 GiB thread.

`AC-4` holds: **no FO `Proved` verdict on any basis.** `23 §4.4`'s reservation
is untouched. The successor is [[V3-FO-CHECKER-SOUNDNESS]].

> ### `refl True` IS WRONG IN THE SPEC, AND `D4` IS WHERE IT WAS MEASURED
>
> This node's own framing, `23 §4.4:531`, and `fo-route-theorem-home.md:110` all
> write the discharge as `sound Sigma C rho f pi (refl True)`. **This kernel
> rejects `Equal Bool True True = Refl`** — *"Refl expects an Eq-shaped goal"* —
> because equality at an inductive type reduces observationally past the `Eq`
> shape before `Refl`'s check runs.
>
> **`Proved` is the correct term** (`prelude.rs:899`, `Proved : Top`), it is the
> prelude's own established idiom, and it forces the *identical* conversion:
> the reduction of `fok_check_cert (...)` happens inside `eq_at_inductive`'s
> `whnf` calls on the goal **type**, before either proof term is considered.
>
> ⇒ **The discharge shape is sound; the spec's spelling of it is not.** Recorded
> here and carried into the successor. Correcting `23 §4.4` is the enclave's.

**Rider 1 — `sequent_source`'s `gamma`/`delta` field order IS measured, and it
reds. Corrected 2026-08-16 on Adversary `evt_5mbtyzs2qrj2r`.**

**This node, the merge notification, and the Architect's resolution all recorded
it as UNMEASURED. That was wrong, and the measurement predated every one of
those statements** — Adversary `evt_10dxqtbsf4tcw` reported `gamma`/`delta` as
the one axis of three that **did** red, in the same report that refuted the
other two. Re-run at `04af417d2`, the swap reds **three** tests, including both
pins added for the other axes.

> ### THE WITHDRAWAL WAS RIGHT IN METHOD AND WRONG IN CONCLUSION
>
> The Architect's `D3` claim — eleven empty-`gamma` cases protect it — was
> discarded because it shared provenance with the `Init` claim the Adversary had
> just refuted. **The correct response to "one of two claims from this source was
> wrong" is to CHECK the other, not to drop both.** The check had already been
> run and reported; dropping it discarded a real measurement.
>
> ⇒ **Shared provenance is grounds for re-checking, never for concluding.**

⚠ **The axis is protected but not safely so.** The protection comes from two
pins built for **different** axes — over-determined by accident, which is the
**fifth** instance on this node of a property holding because of what the corpus
happens to contain. **A dedicated pin is still warranted. A re-measurement is
not.**

**Rider 2 — the durable `D5` report omits the BUILD PROFILE.** The file writes
the fuel down on the predecessor's *a measurement's parameters must be written
down to be re-takeable* argument, and the identical argument applies to profile,
which moves this workload more than fuel does. The routing message said
*"release/build profile"*; the artifact's only profile word is `"debug profile"`
at line 92, attached to hand-probes the file explicitly excludes from the
durable record. **The headline four-to-five-orders comparison against the
predecessor's 265 microseconds is unanchored at both ends if the profiles
differ.** The `D5` conclusion survives either way, which is why it did not block.

> **Sharpened by the Adversary, `evt_5mbtyzs2qrj2r`: the emitted report writes
> down TWO of the three re-takeability parameters and drops the third.** It
> prints the fuel (*"`find_certificate`'s fixed production constant, 200"*) and
> the stack (*"1 GiB test thread"*). **The same commit demonstrates the
> discipline twice and omits it once** — and omits the one that matters most,
> because fuel and stack move the number by a factor while debug-versus-release
> moves this workload by orders of magnitude, **and the headline is stated in
> orders of magnitude.** A reader who re-takes in release gets a number that
> cannot be compared to the one they were given.

**Rider 3 — the `D4` control is sound BY CONSTRUCTION, and the load-bearing half
is the baseline, not the forced side.** The forced declaration is
`Equal Bool (fok_check_cert (fok_embed f) pi) True = Proved`, and `Proved` is
`Top`-introduction, so it typechecks only if the kernel reduces the application
to WHNF. The baseline is `const ..._baseline : Bool = fok_check_cert (fok_embed
f) pi`, which typechecks from the signature **without reducing the body**.

⇒ **The timed interval is `elaborate_decl`, which includes parsing and
elaborating a source string that GROWS WITH DEPTH.** Forced time alone would
conflate conversion cost with input size. Both declarations embed the same
`f_src`/`pi_src`, so the reported difference cancels it. **That is what makes
the growth figures mean conversion**, and it is a structural argument rather
than a reading (Adversary `evt_5mbtyzs2qrj2r`).

> ### THE SHAPE THIS NODE PRODUCED FIVE TIMES, KEPT BECAUSE IT IS THE LESSON
>
> **A property that holds because of what the corpus happens to contain, with
> nothing making it hold.**
>
> | # | instance | found by |
> |---|---|---|
> | 1 | two rejection cases rejected by two guards each | reading |
> | 2 | `fok_form_eq` arms droppable, suite green | **mutation** |
> | 3 | `placeholder_child` inert only because unreached | reading |
> | 4 | serializer axes held by an asymmetric population | **mutation** |
> | 5 | `gamma`/`delta` protected only by pins aimed at the OTHER two axes | **mutation** |
>
> **The three found by mutation were invisible to every reader, and in each the
> reasoned safety argument was WRONG rather than merely unproven.** Instance 3
> was closed by naming rather than by inertness — `rejecting_child_never_reached`
> — which is the durable form. **A control's discriminating power is a
> measurement, never a reading.**
>
> ⚠ **Instance 5 is the one to study, because the repair for instance 4 CREATED
> it.** Two pins built for `rule_source` and `qterm_source` happen to also red on
> a `gamma`/`delta` swap. **A fix that lands protection it was not aiming at
> leaves that protection undeclared**, so the next edit to the serializer can
> keep both pins green and silently unprotect the third axis. **Over-determined
> is not the same as protected.**
