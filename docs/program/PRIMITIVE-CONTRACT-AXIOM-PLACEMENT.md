# Primitive-contract axioms — where they live

Architect, 2026-09-14. Rules the design question the Steward routed at
`evt_7xy06gw6r1jgf` (thread `thr_11dcwfhxcsbdw`), arising from the operator's
PRINCIPLES #17. Grounded against `664691b83a45431ea81a0eca87e24c6af942ab28`.

PRINCIPLES #17 settled *what* these axioms are: an `Axiom` stating a tested
property of a TCB export is the proper expression of that property, not an
incomplete proof. It left the home open. This rules the home.

## The question was posed over a mixed inventory

The routing describes the status quo as *"these primitive-contract axioms are
declared as catalog-package `axiom`s, scattered from the primitives they
characterize."* Measured, that is true of **one** of the four items. The four
are four different mechanisms, and two of them are already correct.

| # | item | what it actually is | verdict |
|---|---|---|---|
| 1 | `string_to_list_char_retraction` | named catalog `axiom`, consumed by `pub theorem string_to_list_char_injective` | already correct, do not move |
| 2 | `Ord Int` `refl`/`antisym`/`trans`/`total` | four **anonymous** term-level `Axiom`s in instance fields | the one real defect |
| 3 | `Eq Int` / `DecEq Int` | not axioms at the use site — **proved** from kernel-issued `int_eq_sound`/`int_eq_complete` | already the target state |
| 4 | `BytesRoundTripLaw` | a law the **caller** supplies; `Codec` consumes it | not ours to issue |

Item 3 is the proposed target state, already landed and working. The question
"should these move to the prelude?" is therefore partly answered by the corpus:
one of them already did, and it is the healthiest entry in the inventory.

## The kernel has one object and one admission path

Every form in that table produces the **same kernel declaration**. There is no
representational difference to trade off:

- `declare_postulate` (`check.rs:1195`) `classify`s the type in the empty
  context and commits a `Decl::Opaque`. It **fails closed**: an ill-formed
  statement is refused, not trusted.
- A named catalog `axiom` is surface sugar — `ast.rs:297` spells `AxiomDecl`
  as *"`theorem name : T = Axiom`"*.
- A term-level `Axiom` elaborates through the same call
  (`elab.rs:1424-1436`), with the goal type as the statement.
- `declare_deceq_certificate` builds the statement in Rust from the
  registrant's own ids and routes through `declare_postulate` too.

**`trusted_base()` counts all of them identically** (`env.rs:568`). It filters
`Decl::Opaque` and non-`Literal` `Decl::Primitive`, minus `is_prelude`.

> **`is_prelude` is a three-id allowlist, not a prelude exemption.**
> `env.rs:347` returns true for exactly `top_id`, `bottom_id`, `tt_id` — the
> `Ω` vocabulary. **Anything the prelude declares beyond those three is in
> `trusted_base()` like anything else.** The name invites the opposite reading
> and a placement argument resting on it would be false.

## (2) TCB impact — the Steward's read is confirmed, with two corrections

The Steward's read was *"zero: it relocates the same trust assumption, it does
not grow the trusted base."* **Confirmed**, and now measured rather than
assumed. Two corrections to the reasoning, both of which matter later:

**It is zero because the two forms are the same `Decl::Opaque`, not because
prelude declarations are exempt.** See the box above. Relocation is
trust-neutral on the logical axis for that reason and no other.

**It is zero per declaration, not per program.** Prelude issuance is
unconditional: every program's environment gains the assumption whether or not
it uses the primitive. A catalog axiom arrives only with an `import`, and that
import edge is visible evidence **in the program's own text** that the program
leans on the assumption. Relocation therefore trades a per-program audit signal
for manifest completeness. That is a real cost, it is the only real cost, and
the criterion below decides when it is worth paying.

Labels do not soften this. **AX-2 is explicit that an opaque label is readable
audit metadata and never a kernel input** (`ax2_named_postulate_inertness.rs`:
*"labels do not participate in identity"*), and `elab.rs:315` notes they "may
legitimately repeat."

## (1) The home, and the criterion that selects it

There is no single home, and the axis the question was posed on is not the axis
that decides. **The deciding question is not where the axiom sits — it is
whose vocabulary the statement is written in.**

> **An axiom over a TCB export is kernel-issued, co-located with the
> registration, if and only if its statement is expressible in kernel
> vocabulary plus the ids the registration already holds. Otherwise it is a
> named catalog `axiom`. In neither case may it be an anonymous term-level
> `Axiom`.**

The inventory demonstrates both arms, in one file, already landed:

- **`DecEq Int` — kernel-issuable, and issued.** The certificate's statement
  uses only `Term::Eq`, the primitive type, the equality op, and the `Bool`
  type and `true` constructor — all passed in as ids
  (`declare_deceq_certificate`, `check.rs`). Nothing catalog-level appears.
  It is general and opt-in per primitive; the doc comment states `prim_ty` is
  *"the first registrant of this mechanism, not a special case of it."*
- **`Ord Int` — NOT kernel-issuable.** Its law statements are written in
  **catalog vocabulary**: `refl : (x : a) → IsTrue (leq x x)` and
  `total : … IsTrue (bool_or (leq x y) (leq y x))`
  (`LawfulClasses.ken.md:119-125`). `IsTrue` and `bool_or` are catalog
  definitions. Constructing these statements kernel-side would require
  pulling catalog vocabulary into the kernel — **a genuine TCB expansion, and
  exactly the thing this cleanup must not do.**

That is why the criterion is vocabulary and not proximity. Proximity would
say "both are about a primitive, co-locate both"; vocabulary says one can move
without touching the trust root and the other cannot.

**Item 1 falls out the same way and stays put.** The retraction quantifies over
`List Char` and states an `Equal` over a catalog datatype. Catalog vocabulary
⇒ named catalog `axiom` ⇒ it is already in its correct home. Relocating it
would pay the per-program audit-signal cost for no gain, and I am refusing
that half of the proposal explicitly.

**Item 4 is outside the criterion entirely.** `BytesRoundTripLaw` is
**consumed, not constructed** — `theorem codec_roundtrip_anchor (p :
BytesRoundTripLaw) : BytesRoundTripLaw = p`. The law is supplied by the
caller across the FFI boundary; Ken issues nothing and `Codec` correctly
declares its own trust delta zero. **A caller-supplied law has no issuance
home to rule on**, and folding it into a common home would misrepresent who
is trusting whom.

## (2b) The one real defect — and it is not a placement defect

`Ord Int`'s four laws are **correct in kind**. `leq_int` is a `reg_prim`
primitive, kernel-`Neutral`, so its order laws are not provable over Ken
structure; PRINCIPLES #17 says an axiom is proper there, and #16 does not
reach them. Nothing about that needs changing.

What is wrong is that they are **anonymous**:

```ken
instance Ord Int {
  leq = int_leq;
  refl = Axiom;
  antisym = Axiom;
  trans = Axiom;
  total = Axiom
}
```

Each `Axiom` mints a real `trusted_base()` entry named by `cx.owner_label`, a
provenance string that may repeat and does not participate in identity. So the
manifest gains **four entries that cannot be told apart, cited, or reused.**

**PRINCIPLES #17 asks the set to "read as a complete manifest of what Ken
trusts about its own implementation."** A manifest whose entries cannot be
named individually fails precisely where it would be audited. Two further
consequences, both practical:

- **It cannot be discharged incrementally.** A named axiom can later be
  replaced by a theorem of the same name with no change at any consumer. Four
  anonymous field `Axiom`s must each be found and rewritten in place.
- **It cannot be cited.** Nothing else can depend on `Ord Int`'s transitivity
  as a named fact, so any downstream proof needing it re-mints its own.

**The contrast is three lines up in the same file.** `Eq Int`'s laws are fully
*proved*, from the **named** kernel-issued `int_eq_sound` / `int_eq_complete`.
Same primitive type, adjacent instances, two different disciplines: `Eq`/`DecEq`
rest on two named trusted entries, `Ord` on four anonymous ones.

**Note what this means for the original question.** Relocating these four to the
prelude would **not fix them** — a prelude-issued anonymous postulate is just as
unnameable. The defect is namability, and it is independent of the home. This is
the part of the routing I am answering differently from how it was asked.

## (3) The mechanism — it exists, and it is not new

Yes, the prelude can inject a Ken-level axiom co-located with `reg_prim`, and
the pattern is landed rather than hypothetical. `declare_deceq_certificate` is
the template, in four steps:

1. build the statement `Term` from the registrant's own ids;
2. admit it via `declare_postulate` — which `classify`s first, so an
   incoherent registrant is **refused, not trusted**;
3. record the resulting ids in a registry keyed by the primitive
   (`DecEqCert`, `env.rs:293`);
4. surface those ids as ordinary Ken globals, where they are consumed as
   ordinary proof terms.

The fail-closed property in step 2 is load-bearing and already present — the
doc comment names the refusal cases (an `eq_op` not shaped
`prim_ty → prim_ty → bool_ty`). **Any new certificate must inherit it; a
registration path that admits its statement without `classify` is not this
mechanism.**

**This mechanism is not the answer for `Ord`**, for the vocabulary reason
above. It is recorded here because the routing asked whether it exists, and
because the next genuinely kernel-vocabulary contract should use it rather
than inventing a second path.

## (4) Disposition — a bounded WP now, much smaller and elsewhere

**Frame it now.** It is small, the precedent is one file away, and the manifest
argument is the operator's own.

**Scope — `Ord Int` only:**

- Give the four laws **names**, as catalog `axiom` declarations in
  `LawfulClasses.ken.md`, consumed by the `Ord Int` instance fields. The shape
  to copy is `string_to_list_char_retraction`: a named `axiom`, not `pub`,
  consumed by the public-facing declaration.
- **Do not** route them through a kernel certificate — `IsTrue`/`bool_or` are
  catalog vocabulary and the kernel must not acquire them.
- **`trusted_base()` cardinality must not change.** Four anonymous entries
  become four named entries. A diff that changes the count means the statements
  changed, which is not authorized here.

**Explicitly out of scope, each for a stated reason:**

- `string_to_list_char_retraction` — already in its correct home (item 1).
- `Eq Int`/`DecEq Int` — already the target state (item 3).
- `BytesRoundTripLaw` — caller-supplied, no issuance home (item 4).
- `string_ord_leq`'s laws — `string_ord_leq` is a Ken-level `fn`
  (`LawfulClasses.ken.md:2376`), so its laws are **proved**
  (`pub proof … for string_ord_leq`) and are PRINCIPLES #16 territory, not
  #17. They are not primitive contracts and must not be converted to axioms.

## Controls

1. **The four named axioms state what the anonymous ones stated.** Each new
   `axiom`'s type is the `Ord` class field type instantiated at `Int`,
   compared against the class declaration — not against prose.
2. **`trusted_base()` cardinality is unchanged**, measured before and after.
   A reduction would mean an assumption was silently dropped; an increase
   would mean one was added.
3. **Negative control on the kernel boundary:** `IsTrue` and `bool_or` gain
   no kernel-side construction. Grep the kernel crate for both names; the
   expected count is zero and it is zero today.
4. **The names are reachable.** Each new axiom is cited by its instance field;
   an axiom nothing cites is a manifest entry with no consumer and means the
   wiring did not land.

Control 3 is the one that can actually fail in the damaging direction. An
implementer reaching for symmetry with `DecEq` would try to build `Ord`'s
statements kernel-side, and that is the failure this design exists to prevent.

## Not authorized

No change to `declare_postulate`, `declare_primitive`, `trusted_base()`, or
`is_prelude`. No new kernel vocabulary. No relocation of item 1. No change to
the `Ord` or `Eq` class declarations. No change to any statement's content —
this names four assumptions that are already being made; it does not add,
remove, or restate one.
