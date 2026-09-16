# WP frame — `SPEC-STANDARD-INFIX-BINDING`

**Owner:** Spec enclave · **Size:** S · **Risk:** low (normative surface, no new
trust root) · **Tier:** T1 · **Gate:** none · **Deps:**
[[SPEC-RESERVED-INFIX-NAMES]] — **merged**

**Measured at:** `origin/main` `13d3313c42e8af7782cd4e4c5fe765c333e79d8b`.
This is a **RECORD** of where the fixed inputs below were taken, not a base a
candidate must be cut from. Cut from `main` as you find it; re-measure §2's
inventory at the cut, because it is a census of live code.

**Origin:** Steward frames 2026-09-16, executing this node's own release
condition — *"released after `SPEC-RESERVED-INFIX-NAMES` lands"* — which is met.
Design authority is the Architect's final-A decomposition `evt_784ge2nq65dfy`.

> # WHY THIS IS THE LANE-2 CRITICAL PATH, AND IT WAS NOT VISIBLE FROM ANY SEAT
>
> Lane 2 has **one** `ready` node and it is in flight
> ([[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]]). Every other language node is
> `draft`. Its successor A1 ([[LANG-STANDARD-INFIX-CALL-COMPLETION]]) is held on
> **this** node, and the node after that
> ([[LANG-MEMBERSHIP-OPERATOR-SURFACE]]) is held on A1. So when the WP in flight
> lands, **the language ring goes idle unless this contract has moved** — and
> the spec enclave reports idle with nothing owed, because a `draft` node whose
> release condition silently became true does not announce itself.
>
> The roster's lane-2 objective (`LANG-ACTIVE-PREMISE-KERNEL-VIEW`) is
> **merged**, and so is its named successor. The citation decayed; the lane did
> not.

---

## 1. Objective

Fix the **standard meanings** of `∧ ∨ ≤ ≥ ≠` and the **use-site
call-completion contract** that A1 implements, grounded in
`spec/30-surface/33-declarations.md §5.4` and `spec/30-surface/39-elaboration.md`.

This node writes a contract. It ships no elaborator code, admits no new name,
and adds nothing to the TCB.

## 2. Fixed inputs — measured, not asserted

### 2a. The anchors exist and say what the node claims

    spec/30-surface/33-declarations.md:567-664   §5.4 constraint -> implicit instance argument
    spec/30-surface/39-elaboration.md            present
    spec/30-surface/31-lexical.md:141-153        A0's admitted spellings

`33 §5.4` is the mechanism this contract binds to: a `where C A` constraint
elaborates to an implicit `Π` over the class record; at a **use site** the
elaborator inserts a metavariable and **discharges it by instance search**
(`39 §6`). The dictionary is named `d<v>` deterministically. **The completion
machinery is specified; what is missing is the standard bindings that use it.**

`31-lexical.md:141-142` is load-bearing for §3a: the admitted spellings are
`≤`/`<=`, `≥`/`>=`, `≠`/`/=`, `∧`/`/\`, `∨`/`\/`, and glyph-only `∈`, and
**each paired spelling is ONE token.** A0 admitted names, not meanings.

### 2b. The `≠` carrier inventory — a CLOSED census, and it is the whole risk

`≠` is specified as the negation of *the exact comparator the existing
`NumericEnv` `==` path selects*. That path is `NumericEnv::classify_eq`, and its
table has **exactly five writes in the whole workspace**:

    carrier   key id           comparator     registered at             when
    Int       int_id           eq_int         numbers.rs:558            always
    Float     float_id         eq_float       numbers.rs:559            always
    Float32   float32_id       eq_float32     numbers.rs:560            always
    Decimal   decimalpair_id   decimal_eq     decimal_char.rs:211       only if register_decimal_char runs
    Char      char_id          eq_char        decimal_char.rs:268       only if register_decimal_char runs

Census instrument, and it closes because the field is private to `numbers.rs`
and its only mutator is `pub(crate) set_eq_entry`:

```sh
git grep -n 'set_eq_entry\|eq_table.insert' -- crates/ | grep -v '///'
```

**Nat, Bool and String are absent.** That is the measurement, not a sample.

### 2c. TWO facts a contract written from the node's prose alone would get wrong

**Decimal is keyed on `decimalpair_id`, NOT `decimal_id`.** `Decimal :=
DecimalPair` is a transparent alias (`18a §5.6.1`), so `whnf` delivers
`IndFormer{id: decimalpair_id}` and a lookup keyed on `decimal_id` is **never
performed**. A normative sentence naming "Decimal" without naming its registered
representation describes a row that cannot be reached. (`numbers.rs:191-197`
documents exactly this for `classify_add`; `classify_eq` shares it.)

**There are TWO refusals, and they are not the same refusal.**

    x ≠ y  at Nat / Bool / String   head IS Const/IndFormer; NO ROW    table miss
    x ≠ y  at a bound type variable head is NEITHER                    structural

`classify_eq` matches only `Term::Const` and `Term::IndFormer`; every other head
— a variable, an application, a `Π` — returns `None` **before any table lookup
happens**. One sentence saying "unsupported carrier" collapses these, and the
collapse is not cosmetic: the first is a gap a future row closes, the second
is a statement about what `≠` **is** (a carrier-directed comparator, never
universal disequality, never `DecEq`). See AC-TWO-REFUSALS-DISTINCT.

### 2d. One asymmetry, named so the contract does not accidentally freeze it

`classify_eq` has one generic caller (`elab.rs:9412`) and four direct
per-carrier calls (`elab.rs:14987`, `:15027`, `:15043`, `:15082`) for Int,
Float, Float32 and **Char**. **Decimal is absent from the direct set** and is
reached only through the generic path. This is an observation about today's
elaborator, **not** something the contract should ratify — specify the carrier
set, not the call sites.

### 2e. `Core.Operators` does not exist

`catalog/packages/Core/` holds `Classes/` and `Logic/` only. The home is
**proposed** by this contract and **created** by A1. Do not write it as extant.

## 3. Deliverables

### 3a. Standard bindings — ordinary checked functions, reached by ordinary import

- `∧` = `bool_and`, `∨` = `bool_or`. Results `Bool`. **No invented
  short-circuit**: single, left-to-right operand evaluation.
- `≤` / `≥` = `Ord` through an **actual dictionary** (`33 §5.4`'s implicit
  instance argument), not a built-in. **`≥` reverses already-evaluated VALUES
  inside its wrapper**, never by reversing operand ASTs — so left-to-right
  evaluation order is observable and unchanged.
- `≠` = negation of the comparator selected by the existing `NumericEnv` `==`
  path, on **the same carriers**, with **the same refusals**. It is **not**
  universal disequality and **not** `DecEq`.
- No automatic `Ω` connective, no equality refinement, no proof-witness
  conversion. Legacy `==` unchanged.

### 3b. The completion policy

The common call elaborator completes the omitted prefix for these bindings:
infer the operand carrier, resolve the required dictionary via the existing
class resolver **or** select the existing equality comparator, then check the
**fully saturated ordinary application** in the kernel.

**The policy binds to the defining `GlobalId` + checked telescope, NEVER to the
occurrence's glyph text.** Three consequences the text must state, because each
is a way an implementation gets it wrong:

    a renamed standard binding          still completes       (keyed on GlobalId)
    an unrelated local `≤`              stays ordinary        (different GlobalId)
    `ord_leq_at Nat d`                  stays a valid partial application
                                        (do NOT reinterpret every 2-arg call
                                         as a 4-arg helper)

### 3c. Standard fixities — of the bindings, not the tokens

    ∧  infixr 3
    ∨  infixr 2
    ≤ ≥ ≠  infix 4

### 3d. The `≠` carrier inventory, normatively, with both refusal classes

§2b's five rows, Decimal named by its **registered representation**, and §2c's
two refusals given distinct normative treatment.

### 3e. Paired conformance rows

Covering 3a-3d. Rows that gate on a capability A1 has not shipped carry a
`RED-UNTIL-LANG-STANDARD-INFIX-CALL-COMPLETION` tag — **and that node exists**,
so the tag is a gate rather than a dangling name.

## 4. Acceptance

Each criterion names the control that can **fail** it. A criterion whose control
cannot fail is not on this list.

**AC-CARRIER-INVENTORY-EXACT.** The normative carrier set is exactly §2b's five,
and no more. **Control:** re-run §2b's census at the candidate; the spec's list
and the census agree by name in both directions. A spec listing four or six
fails. *(Derived, not counted — if a sixth carrier is registered before this
lands, the census moves and the spec moves with it.)*

**AC-DECIMAL-NAMED-BY-REGISTRATION.** The Decimal row names the representation
actually keyed. **Control:** a reader following the spec's Decimal row reaches
`decimalpair_id`. A row naming only `decimal_id` fails, and it fails silently in
prose — so the control is a citation to `numbers.rs`'s alias note, not a reading
of the sentence.

**AC-TWO-REFUSALS-DISTINCT.** The table-miss refusal and the structural refusal
are separately stated. **Control:** the text yields *different* answers for
`x ≠ y` at `Nat` and at a bound type variable, and says which is a closable gap.
A single "unsupported carrier" clause fails.

**AC-COMPLETION-KEYED-ON-GLOBALID.** **Control:** the three cases in §3b each
resolve as written, and the text is what decides them — not a worked example
appended beside it.

**AC-GEQ-REVERSES-VALUES.** **Control:** the text forbids operand-AST reversal
explicitly and states left-to-right evaluation for `≥`. A text that only says
"`≥` is `≤` flipped" fails, because that is the implementation this AC exists
to exclude.

**AC-NO-NEW-TRUST-ROOT.** No new `Eq`/`DecEq` instance, no Float-equality law,
no TCB entry (Architect, `evt_784ge2nq65dfy`). **Control:** the candidate's diff
adds **zero** rows to the TCB inventory — a `-0`/`+0` measurement on that file,
not an assurance.

**AC-NO-SHIPPED-CODE.** Spec + conformance only. **Control:**
`git diff <base> HEAD -- crates/` is **empty**.

## 5. Not this node

- The elaborator implementation — [[LANG-STANDARD-INFIX-CALL-COMPLETION]] (A1).
- Name/fixity-target admission — [[SPEC-RESERVED-INFIX-NAMES]], merged.
- Membership's binding and class — [[SPEC-MEMBERSHIP-CLASS-CONTRACT]] and
  [[LANG-MEMBERSHIP-OPERATOR-SURFACE]], the parallel B track.
- Creating `Core.Operators`. This contract proposes the home; A1 builds it.
- Adding a carrier to the `==` table. Widening the inventory is a separate,
  soundness-bearing decision and this node **records** the inventory.

## 6. Contention

`spec/30-surface/` — principally `33-declarations.md`, `39-elaboration.md`, and
the conformance surface. No `crates/` paths, so **no cross-lane contention**
with either build lane.

Both reserved-infix predecessors are `merged` and the spec enclave holds no live
candidate. Check live node status at `origin/main` — not for a branch ref, which
on this repo is wrong about landing far more often than it is right — before
releasing.

## 7. Related

- [[LANG-STANDARD-INFIX-CALL-COMPLETION]] — A1, held on this node.
- [[SPEC-RESERVED-INFIX-NAMES]] — A0's spec prerequisite, merged.
- [[LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE]] — the lane-2 WP in flight; it
  touches `32 §3`'s application-atom pin, which this node does not.
