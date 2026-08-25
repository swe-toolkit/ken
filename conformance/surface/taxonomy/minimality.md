# Surface minimality — the derivation-path table (ES1)

Format: `../../README.md`. This is the **minimality proof** of the everyday
surface (`docs/program/everyday-surface-program.md`, `cfe5172`): the
load-bearing artifact certifying the invariant

> **The surface built-in set ≡ the surface `trusted_base()` delta.** A
> prelude/standard entry with a Ken-**derivation witness** lands as a
> re-checked `definition` (out of `trusted_base()`); only a genuine
> **irreducibility witness** stays a `postulate`/primitive.

so "built-in vs package" **is** "audited trust-root vs re-checkable Ken" — the
minimality proof is simultaneously a **TCB-hygiene** proof, the surface analog
of Sec4's TB-Sound + TB-Complete (`64 §1.1`/`§1.2`). The table has **two
halves**, each a **direction** of the discriminating check (a one-directional
table proves nothing):

- **Completeness (§C):** every prelude/package feature → a **real Ken
  derivation path** from the built-ins. A feature with **no** path is a
  **hidden built-in** (the TB-Complete omission — an assumption that hid).
- **Irredundancy (§A/§D):** every built-in → its **irreducibility witness**
  (why it *can't* be Ken-defined). A built-in **with** a path is **bloat** (a
  TB-Sound phantom — a needless `trusted_base()` entry).

**Method — real, not asserted.** Each verdict carries a witness grounded
against landed code (`prelude.rs`/`numbers.rs`/`bytes.rs` @ `b97ca5c`, the
kernel). The full *elaboration* witness (each demoted def kernel-checks in the
stated sort + `trusted_base()` shrinks by exactly the named ids) is **ES2's
build-verification** (a real `trusted_base()`-delta assertion — my Sec4 lane);
`ken-cli` is REPL-only and Ω-data/truncation may not be surfaced yet, so here
the witness is the **grounded sort-analysis**, which is what the taxonomy line
needs.

## A. The built-in set (irreducibility witnesses)

The surface TCB — each is **irreducible** (no Ken derivation path); a path
here would be bloat. None found (all four are genuine).

| Built-in | Witness (why not Ken-definable) |
|---|---|
| **Primitive types + literals** — `Int`/`Int8..64`/`UInt8..64`/`Decimal`/`Float`/`Float32` (`numbers.rs reg_ty`), `String`/`Bytes` (`bytes.rs`) + literal syntax | parser-produced opaque type constants (`declare_primitive` OpaqueType, `14 §5`); nothing is more primitive. `Char` is the checked refinement in §B, not a primitive type. |
| **Audited primitive ops** (`14 §5`) — `reg_binop` (`A→A→A` arith), `reg_cmpop` (`A→A→Bool`), the `String`/`Bytes` prims (`append`/`slice`/`byteLength`/`String↔List Char`) | bottom out in the kernel's audited `PrimReduction::Op` on literals; not expressible as pure Ken (they *are* the machine semantics). |
| **The effect / FFI boundary** — `foreign` + the base `IO`/effect primitive (`[Console]`/`[FS]`; `print_line` foreign) | I/O is not pure Ken — the effect boundary is where the world enters. |
| **Base elaborator syntax** — λ/app/`let`/`match`/annotation/`data`/`view`/`instance`, refinement types, the **operator-infix + fixity** affordance, `if`-sugar, minimal `module`/`import` | the language forms themselves; the parser/elaborator realizes them. (Note: `if` *desugars* to `match`, and operator *semantics* is package — but the **syntactic affordance** to write them is base. Syntax built-in; semantics derivable.) |

## B. The prelude set (closed signature + bootstrap inventories — AC2)

**Membership rule (normative, checkable):** the prelude is the closed union of
(1) Ken-defined types named by built-in primitive signatures and (2)
compiler-bootstrap identities that the surface contract requires source to name
and that source cannot recreate with the same `GlobalId` (`30-taxonomy §4`).
The signature arm is derived from the executable population: traverse the type
of every `Decl::Primitive`, then retain references whose declaration is an
ordinary checked inductive or transparent definition. The bootstrap arm requires
an exact-identity, constructor-parentage, and no-allocation witness. This is the
surface analog of the kernel's closed `is_prelude = {Top, Bottom, tt}` (`64 §1`),
not a catch-all.

| Prelude type | Membership witness | Derivation / identity witness |
|---|---|---|
| **`Auth`** | signature: `Cap : Auth → Type` | ordinary checked `data Auth = ANone \| APartial \| AFull`; the opaque primitive former needs this exact parameter identity. |
| **`Bool`** | signature: comparison result `A → A → Bool` | ordinary checked `data Bool = True \| False` — derivable, but signature-named ⇒ prelude (F1). |
| **`Char`** | signature: `String ↔ List Char` | checked transparent scalar type (`35 §2.4`); the constructor-free signature member. |
| **`List`** | signature: `String ↔ List Char` and `Bytes ↔ List UInt8` | ordinary checked `data List`; signature-named at two independent primitive surfaces. |
| **`Option`** | signature: `bytes_at` and `bytes_slice` results | ordinary checked `data Option a = None \| Some a`; strict source must name and eliminate the exact result family. |
| **`ResourceKind`** | signature: `Resource : ResourceKind → Type` | ordinary checked `data ResourceKind = FsHandle \| Buffer`; the opaque primitive former needs this exact parameter identity. |
| **`Result`** | signature: `bytes_decode : Bytes → Result Utf8Error String` | ordinary checked `data Result e a = Err e \| Ok a`; strict source must eliminate the exact result family. |
| **`Utf8Error`** | signature: the error argument of `bytes_decode`'s result | ordinary checked `data Utf8Error = InvalidUtf8`; no replacement family can inhabit the primitive result. |
| **`Nat`** | bootstrap identity: source must reach the compiler-installed family used as Ken's canonical natural/index carrier | ordinary checked `data Nat = Zero \| Suc Nat`; strict-floor installation reuses those exact ids and allocates none. |
| **`Ω` (Omega)** | kernel syntax: no-overflow propositions land in `Ω₀` | **kernel-provided** (the strict-prop universe, `16 §1`) — a kernel built-in referenced directly, not a surface prelude binding. |

The signature arm therefore closes to exactly **`{Auth, Bool, Char, List,
Option, ResourceKind, Result, Utf8Error}`**; the bootstrap arm adds exactly
**`Nat`**. For every inductive member, the floor admits constructors only by
matching their kernel-recorded parent `GlobalId`; `Char` has no constructor arm.
Every listed family/definition and constructor is outside `trusted_base()`.

### surface/taxonomy/prelude-signature-inventory-is-executable-and-closed

- promise class: **normative compatibility vector** — these exact identities are
  the current public primitive-signature and bootstrap contract
- spec: `30-taxonomy §4`; `33 §3.3`; `39 §2.0`
- given: in a fresh `ElabEnv`, walk the type term of **every**
  `Decl::Primitive`, collecting each referenced `GlobalId` whose declaration is
  `Decl::Inductive` or checked `Decl::Transparent`. Independently snapshot the
  nine expected type ids, every constructor id and recorded parent,
  `declarations().len()`, `next_global_id()`, and `trusted_base()`. Do not select
  declarations by helper name, source file, or a hand-picked primitive list.
- expect: the checked dependency set is exactly `{Auth, Bool, Char, List,
  Option, ResourceKind, Result, Utf8Error}`. Adding bootstrap `Nat` equals the
  exact floor type set. The constructor set is exactly the constructors recorded
  under the seven inductive signature members plus `Nat`; no same-spelling
  constructor with another parent qualifies. None of the type or constructor ids
  appears in `trusted_base()`, and installing the floor changes neither
  declaration count nor allocator position.
- controls: prove both equality directions with compile-preserving mutations.
  For under-inclusion, install a checked `Extra` type and a real test-only
  primitive whose signature names `Extra`, leaving the configured floor
  unchanged; the derived signature set grows and the assertion must red. For
  over-inclusion, add pre-installed checked `Prod` only to the configured floor,
  without adding a primitive/bootstrap witness; the configured set grows and
  the same assertion must red.
- why: producer traversal closes the population by construction. A selected
  `reg_*` grep omits primitives registered through another helper; a spelling
  list cannot distinguish a constructor attached to a lookalike family.
- **MEASURED:** at base `06c62313af62`, the producer traversal returns the exact
  eight-id set above; admitting their union with `Nat` preserves all ids,
  declaration/allocator accounting, and `trusted_base()`. A pre-installed
  non-member such as `Prod` remains unbound in strict roots. **CLAIMED:** the
  executable inventory is the whole prelude floor. **THE GAP:** conformance must
  derive the signature set from all primitive declarations, compare it with the
  explicit configured floor, and fail on a difference in either direction.
  Production resolution must not auto-admit an unreviewed newly observed name.

This case and the strict source-reaching cases in
`../modules/seed-modules.md` are **RED UNTIL
`LANG-MOD-NAT-FLOOR-REALIZATION`**.

**AC2 bloat finding — `OrdResult`.** `data OrdResult = Lt | Eq | Gt`
(`prelude.rs`) sits in the elaborator prelude, but no primitive signature names
it and it has no independent bootstrap-identity witness. By the membership rule
it is **not prelude**. Its origin is a workaround for an opaque,
non-matchable `Bool`; F1's ordinary `data Bool` removes that need. **Ruled
(`30-taxonomy §6`, `7fa08cd`):** remove `OrdResult` as bloat; the `Ord` package's
`Ordering` is the 3-way `compare` result. This is the AC2 discriminator firing
in the bloat direction.

## C. The standard-package set (completeness — derivation paths)

Every package feature has a **real Ken derivation path** from the built-ins.
No hidden built-in found — **but** each path names the built-in *floor* it
needs (remove that floor and the feature *becomes* a hidden built-in — the
load-bearing observation).

| Package feature | Derivation path from built-ins | Built-in floor |
|---|---|---|
| **operators** (`+ - * % == < >`) | `Ord`/`Eq` **class methods** (Lc, landed — `lawful_classes.ken`) back `== < >`; `+ - * %` bind directly to the audited prim ops (`int_add` etc. via `reg_binop`) + operator-infix syntax — **`class Num`/`instance Num Int` are specified-but-not-built** (named forward obligation, a future `class Num` WP), so `+`/`*` are not yet class-abstracted; user types get `== < >` by writing `Eq`/`Ord` instances | the audited prim op (`reg_binop`/`reg_cmpop`) + operator-infix syntax (base) + Lc |
| **`show`/formatting** (`Int.show`, …) | `Int` `div`/`mod` prims → digit `Char`s (literals) → `List Char` → **`list_char_to_string`** (landed) → concat via **`append`** (landed) | `div`/`mod` prims, Char literals, `list_char_to_string`, `append` (**all landed** `bytes.rs`/`numbers.rs`) |
| **collection combinators** (`map`/`filter`/`fold`/`range`) | total structural recursion over `List`/`elim_List` (L2/L3); `range` = fuel-bounded unfold (`37 §5`, no coinduction) | `data List` + `elim_List` (L2), recursion + SCT |
| **lawful classes** (`Monoid`/`Functor`/`Monad`/`Foldable`) | `class`/`instance` records (Lc, landed) carrying law propositions | Lc (`33 §5`, landed) + Ω (laws) |
| **string manipulation** (`split`/`join`/`pad`/`toUpper`) | over `String↔List Char` (landed conversions) + `append` + the combinators | `String↔List Char` + `append` + combinators |

**Completeness verdict: PASS** — every package feature is derivable; the
built-in *floor* (the audited String/Int prims + the `String↔List Char`
conversions + Lc) is exactly what makes the surface generable. Had `append` or
`list_char_to_string` **not** been landed, `show`/string-manipulation would be
**hidden built-ins** (no path) — that is the check that matters, and it passes
on the landed set.

## D. Irredundancy findings — the prelude postulates (the ★ TCB-hygiene half)

The entries audited in `prelude.rs` (Architect-approved
`evt_5bedyc3zyhr`), with their current fate after the landed follow-ons.
Entries marked as retired below are no longer live trust-root surfaces.
Verdicts + Ω-sort witnesses:

| Entry | Form | Verdict | Witness / action |
|---|---|---|---|
| **`Equal : Π(A). A→A→Ω`** | `declare_postulate` | **REDUNDANT — shadows a *computing* primitive** | the kernel provides native **`Eq A a b : Ω`** (computes, with `refl`/`J` — `16 §2`, `term.rs`). The postulate forfeits `Eq`'s computation + `J`-elim. **Action: delete, reference `Eq`** (not "define"). |
| **`And : Ω→Ω→Ω`** | `declare_postulate` | **DERIVABLE** | `data And (A B:Ω):Ω := conj (a:A)(b:B)` → **Ω** via both-keyed `sort_sigma` (Σ of two Ω → Ω); or `16 §1.3` derived connectives. |
| **`isSorted : Π(A). List A→Ω`** | `declare_postulate` | **DERIVABLE (★ soundness)** | Ω-recursion `isSorted (x::y::r)= And (x≤y)(isSorted (y::r))`. **Needs a Prop-valued `≤ : A→A→Ω`** — if `Ord` exposes only `Bool` `leq`, add `Le`/`IsTrue (leq a b):Ω` (else it's `Type`, a relevance leak). |
| **`Perm : Π(A). List A→List A→Ω`** | `declare_postulate` | **DERIVABLE (★ soundness)** | **Ω-sort fork:** the inductive relation (`refl\|swap\|trans\|cons`) is proof-**relevant** (`Type`) ⇒ needs **truncation** `∥·∥` to be an Ω predicate; count-equality (`Π x. Eq Nat (count x xs)(count x ys)`) is **natively Ω** but DecEq-dependent. Either is derivable; spec picks the form. |
| **opaque `Bool`** | `declare_primitive` (Opaque) | **DERIVABLE (F1)** | `data Bool = True\|False` (Type) — removes the opaque primitive **and** the `OrdResult` branch-workaround (§B). |
| **the former `Map`/`Set` placeholder** | historical `declare_postulate`, then `declare_primitive`, now retired | **DERIVABLE — proved package (landed `52-map`)** | The shipped carrier is ordinary inductive `Tree k v` over lawful `Ord k`; its operations and shipped laws are re-checked definitions, `Set` derives from the same carrier, and the one deferred permutation law is not postulated (`52 §1`/`§4.4`/`§7c`). It is out of `trusted_base()`. Extensional behavior and durable round-trip do not require a particular in-process representation: copying, sharing, or interning the value is private, and internal bytes are not observable (`41 §3a`/`§4`). |
| **`reg_novf` — the no-overflow PREDICATE** (`Fits`/`inBounds : Int → Ω`) | `declare_postulate` (`numbers.rs:190`) | **★ NEW (seed missed) — DERIVABLE (ruled §6)** | the reusable decidable fixed-width **bound predicate** (`(a+b)` within `[MIN_w, MAX_w]` over arbitrary-precision `Int`) → a **definition, out of `trusted_base()`** (`OQ-1a`). Same Prop-`≤`/`IsTrue` bridge as `isSorted`. My signature-grep surfaced it; §6 ruled it a definition. |
| **the L1 per-op no-silent-wrap OBLIGATION-HOLES** (`declare_postulate` goal per fixed-width op) | `declare_postulate` | **★ NOT bloat — a LIVE OBLIGATION (stays)** | the per-operation "no silent wrap" **proof obligation** awaiting prover discharge (`35 §3`, `43 §2`, [[soundness-AC-static-vs-runtime-face]]) — **legitimate trusted-until-discharged**, the overflow soundness net. Making *this* a "definition" would be circular or **eliminate the net**. It **stays** in `trusted_base()` as a live obligation (item-3, but the *good* kind). **The distinction the invariant turns on:** a *derivable* postulate is bloat; a *live proof-obligation* is not — the predicate demotes, the obligation-hole stays. If `reg_novf` does double duty, ES2 **splits** them (predicate→def / obligation→hole). Architect pre-flag `evt_57r42rsx3jx3w`. |
| **numeric + string literals** (`elab.rs:460`/`:503`, `elab_str_lit`) | `declare_postulate` (**per literal**) | **★ HIGHEST-VOLUME hygiene — TERM (ruled §3)** | each literal *value* (`42`, `"…"`, in `num_values`) is a **per-program `trusted_base()` postulate** — an *assumed* value for a *computed* constant. Ruling: a **primitive-constant TERM** (the *type* is item-2, listed once; the value is a real core term), **out of `trusted_base()`** — not a per-literal entry at all. Verified `elab.rs:460` (Architect VAL1 catch `evt_488kj79z0wqd7`). |

**Ω-sort discipline (the relevance-leak check, Architect `evt_5bedyc3zyhr`):**
every predicate demoted to a def must land in **Ω** (proof-irrelevant), not
`Type` — a `Type`-valued "prop" leaks content into the refinement carrier.
`And` ✓ (both- keyed Σ→Ω); `isSorted`/`Perm` per the forks above; `Bool` is
correctly `Type` (matchable data, not a prop).

## E. Trust-class accounting (AC4) — the `trusted_base()` delta

Both `Decl::Opaque` (item-3, **assumed axiom**) and `Decl::Primitive` (item-2,
**audited**) surface in `trusted_base()` (the `matches!(Opaque | Primitive)`
filter, `64 §1.2`, my Sec4 ground) — so the *category* is a
trust-level-honesty distinction, not a listed-or-not one:

The invariant turns on a distinction the accounting must keep sharp:
**`trusted_base()` should contain only genuine irreducibles + audited
primitives + *live proof-obligations*.** A **derivable** postulate is bloat
(demote); a **live obligation** (awaiting discharge) is legitimate (stays).
Two current fates, after the later `Map`/`Set` retirement:

- **Leave `trusted_base()` entirely** — the *derivable* / *shadowing* entries,
  now re-checked `Decl::Transparent` defs (or a kernel reference / a term):
  `Equal` (→ kernel `Eq`), `And`, `isSorted`, `Perm`, `Bool`, the `reg_novf`
  **predicate** (→ decidable-bound def), and **every literal** (→
  primitive-constant terms). The former `Map`/`Set` placeholder also leaves:
  landed `52-map` supersedes it with ordinary inductive `Tree k v`, re-checked
  operations and proofs, and `Set` derived from that carrier. The
  **assumed-axiom bloat** goes to zero, and the later Map retirement removes
  those two audited primitive entries as well.
- **Stay listed as *live obligations*** (item-3, the legitimate kind): the L1
  per-op **no-silent-wrap obligation-holes** — trusted-until-discharged, the
  overflow soundness net (`soundness-AC-static-vs-runtime-face`). **Not** bloat;
  removing them would eliminate the net.

**Net:** the surface **assumed-axiom bloat** in `trusted_base()` goes to
**zero**; among the entries in this table, only the **live overflow
obligations** legitimately remain trusted. The broader surface still contains
the genuine irreducibles and audited primitives in §A. The invariant holds on
the real set: **no built-in has a derivation path** (§A, no bloat) and **no
package/prelude feature lacks one** (§B/§C, no hidden built-in). `Map`/`Set`
are the landed derived-package case: their extensional behavior is independent
of whether a conforming runtime copies, shares, or privately interns their
ordinary data representation.

## Coverage map (AC → sections)

- **AC1** (invariant normative + minimal set exact; both directions): §A
  (irreducibility, no bloat) + §C (completeness, no hidden built-in) — the
  table exercises **bloat** (§B `OrdResult`, §D `Equal`/`And`/…) **and**
  hidden-built-in (§C, none found; the floor named).
- **AC2** (prelude closed by the two-arm rule): §B — executable traversal of
  every primitive type closes the eight-member signature set, the
  exact-identity/no-allocation `Nat` bootstrap witness adds the ninth member,
  and the `OrdResult` bloat finding proves the opposite direction (ruled remove;
  `Ordering`→package, §6).
- **AC3** (load-bearing predicates specified as definitions): §D — `And`/
  `isSorted`/`Perm` with defining equations + Ω-sort witnesses; the
  verified-`sort` refinement (`37 §6`) unfolds them (green-vs-green against a
  postulate otherwise).
- **AC4** (trust-class rulings exact): §E — the item-2/item-3 line per entry,
  the `trusted_base()` delta; `Equal` delete-for-`Eq`; the former `Map`/`Set`
  placeholder retired in favor of the proved package, with no replacement
  trust-root entry.

## Build-forward (current verification gate)

This artifact began as **spec + conformance only** (no crate). Its current
build-forward gate includes the later landed `52-map` supersession. The
conformance gate is the **elaboration witness** — producer-grepped, not
asserted:
1. Each demoted predicate (`And`/`isSorted`/`Perm`) **kernel-checks as a
   `Decl::Transparent` def in the stated Ω sort** (the relevance-leak check).
2. The **assumed-axiom** entries **leave** `trusted_base()` — `Equal`, `And`,
   `isSorted`, `Perm`, `Bool`, the `reg_novf` **predicate**, and the per-literal
   postulates (→ terms) — a real `trusted_base()`-delta assertion; no entry
   hides, none over-removed.
3. The former `Map`/`Set` prelude placeholders are **absent** from
   `trusted_base()` and are not `Decl::Primitive`/`Decl::Opaque`. The shipped
   package carrier and operations are re-checked inductive data/definitions;
   its observed values do not reveal whether the runtime copies, shares, or
   privately interns them.
4. **★ The live overflow obligation-holes still appear** (item-3, the
   legitimate kind) — a successor must **not** sweep them away with the
   predicate; the split (predicate→def / obligation→hole) is the load-bearing
   check, or the overflow net is lost.
A green build that hand-inserts the def or asserts "it type-checks" without
the `trusted_base()` delta (both the removals **and** the retained obligations)
is green-vs-green (`conformance-hand-feeds-the-deliverable`).
