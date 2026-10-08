# Formal languages: deterministic automata

> Status: **DRAFT v0 (SPEC-FORMAL-LANGUAGES-DFA-CONTRACT).** Section 1 is
> normative for the `Algorithm.FormalLanguages.Dfa` package's record,
> operations, checked laws, finiteness boundary, and trust boundary. Sections
> 2–6 name later work but do not specify or deliver it. This chapter introduces
> no kernel, trust, or surface-syntax change.

A deterministic automaton describes a transition for every state and input
symbol and a Boolean acceptance test. Its state carrier need not be finite:
finite input words make `run` a terminating fold, but say nothing about how
many states the automaton can occupy.

## 1. Deterministic automata (`Algorithm.FormalLanguages.Dfa`)

### 1.1 Carrier and derivation

The package declares an ordinary Ken inductive record:

```ken
data Dfa q a = MkDfa (q → a → q) q (q → Bool)
```

Here `q : Type` is any state carrier and `a : Type` is any input alphabet.
The fields are, in order, a total transition function, a start state, and a
Boolean final-state test. This contract requires neither `DecEq q` nor
`DecEq a`: none of its operations compares states or input symbols. The
transition is a function, not a table, and the final-state test computes a
`Bool`, not a proposition or a proof. A table may represent an automaton in
a later executable, but it does not replace this public record.

`Dfa (Fin n) a` is the particular carrier choice `q = Fin n`. The `Dfa`
record itself supplies **no finiteness evidence**; it is finite only when a
separate certificate for `q` is supplied. In particular, constructing a
`Dfa q a` or terminating a call to `run` on a finite `List a` does not
establish decidable reachability or emptiness.

The derivation path is ordinary inductive data and structural recursion over
the prelude `List`, `Bool`, and compiler-origin `Pair` with its checked
`mk_pair`, `pair_fst`, and `pair_snd` companions (`README §1`). This package
uses `list_append` from `Data.Collections.Derived`, `bool_and` and `bool_or`
from `Core.Classes.LawfulClasses`, and `cong` from
`Core.Logic.Transport` for its proofs. `bool_not` is a shared, public
ordinary-Ken function of `Core.Classes.LawfulClasses`; the accompanying
catalog build promotes it there and removes the private duplicate in
`Data.Collections.Map`. This is a package-level dependency, **not** a new
primitive or prelude name.

### 1.2 Operations and computational meaning

The following is a public type synopsis; the semantic equations below fix
its behavior without prescribing the private layout of function bodies.
`Pair q r` is the prelude's non-dependent pair, not a finite-state encoding.

```ken
step : (q : Type) → (a : Type) → Dfa q a → q → a → q
start : (q : Type) → (a : Type) → Dfa q a → q
final : (q : Type) → (a : Type) → Dfa q a → q → Bool
run : (q : Type) → (a : Type) → Dfa q a → q → List a → q
accepts : (q : Type) → (a : Type) → Dfa q a → List a → Bool
complement : (q : Type) → (a : Type) → Dfa q a → Dfa q a
product : (q : Type) → (r : Type) → (a : Type) →
          (Bool → Bool → Bool) → Dfa q a → Dfa r a → Dfa (Pair q r) a
intersection : (q : Type) → (r : Type) → (a : Type) →
               Dfa q a → Dfa r a → Dfa (Pair q r) a
union : (q : Type) → (r : Type) → (a : Type) →
        Dfa q a → Dfa r a → Dfa (Pair q r) a
```

For `d = MkDfa transition initial accepting`, `step d s x` is
`transition s x`, `start d` is `initial`, and `final d s` is `accepting s`.
`run d s [] = s`, and `run d s (x :: xs) = run d (step d s x) xs`:
transitions process the input in order, starting at **the supplied `s`**.
`accepts d w = final d (run d (start d) w)`. Thus `run` is available from
any state; only `accepts` fixes the initial state.

`complement d` retains `d`'s transition and start state, and replaces its
final-state test with `λs. bool_not (final d s)`. For any
`combine : Bool → Bool → Bool`, `product combine d e` has state
`Pair q r`, start `(start d, start e)`, transition
`(s1, s2), x ↦ (step d s1 x, step e s2 x)`, and final-state test
`(s1, s2) ↦ combine (final d s1) (final e s2)`. Both machines read the
**same** symbol at each step. The two named abbreviations are exactly
`intersection d e = product bool_and d e` and
`union d e = product bool_or d e`; there is no second automaton
construction or alphabet-equality requirement.

### 1.3 General checked laws

The seven named laws below are required **proved theorems** of the package;
not postulates, tests, or propositions left as open obligations. The
signatures abbreviate explicit type arguments to the operations in §1.2
where the types determine them. `Equal A x y` is the checked equality
proposition at carrier `A` (`../10-kernel/15-identity.md`), not equality of
private implementation layouts. Each law holds for all parameters shown,
without an assumption of finiteness or decidable equality.

```ken
theorem run_append
  (q : Type) (a : Type) (d : Dfa q a) (s : q)
  (u : List a) (v : List a) :
  Equal q (run d s (list_append a u v)) (run d (run d s u) v)

theorem run_complement
  (q : Type) (a : Type) (d : Dfa q a) (s : q) (w : List a) :
  Equal q (run (complement d) s w) (run d s w)

theorem accepts_complement
  (q : Type) (a : Type) (d : Dfa q a) (w : List a) :
  Equal Bool (accepts (complement d) w) (bool_not (accepts d w))

theorem run_product
  (q : Type) (r : Type) (a : Type)
  (combine : Bool → Bool → Bool) (d : Dfa q a) (e : Dfa r a)
  (s1 : q) (s2 : r) (w : List a) :
  Equal (Pair q r)
    (run (product combine d e) (mk_pair q r s1 s2) w)
    (mk_pair q r (run d s1 w) (run e s2 w))

theorem accepts_product
  (q : Type) (r : Type) (a : Type)
  (combine : Bool → Bool → Bool) (d : Dfa q a) (e : Dfa r a)
  (w : List a) :
  Equal Bool (accepts (product combine d e) w)
    (combine (accepts d w) (accepts e w))

theorem accepts_intersection
  (q : Type) (r : Type) (a : Type)
  (d : Dfa q a) (e : Dfa r a) (w : List a) :
  Equal Bool (accepts (intersection d e) w)
    (bool_and (accepts d w) (accepts e w))

theorem accepts_union
  (q : Type) (r : Type) (a : Type)
  (d : Dfa q a) (e : Dfa r a) (w : List a) :
  Equal Bool (accepts (union d e) w)
    (bool_or (accepts d w) (accepts e w))
```

`run_append` follows structural induction on `u` for **any** initial state.
`run_complement` follows induction on `w`: complement preserves every
transition, although matching the record inside `run` means its result is
not generally definitionally equal to the original `run` on a neutral `d`.
`accepts_complement` transports that state equality through the negated
final-state test (for example, with the ordinary checked `cong`), not a bare
`Refl` assertion on the neutral computation.

`run_product` follows induction on `w` **from arbitrary `s1` and `s2`**, not
only from the two start states. The checked proof uses a separate private
helper `run_product_nil` for its pair-valued empty-word case; it states the
`run_product` equality at `w = []` and is not an eighth public law. This
avoids relying on the match arm to reduce a pair-typed goal to an
`Equal`-shaped goal before checking `Refl`. `accepts_product` transports the
state-pair equality
through the product's final-state test. The intersection and union laws are
instances of this single general law with `combine = bool_and` and
`combine = bool_or`, respectively. These proof obligations concern the
actual functions exported by the package: a second specification-only
product or a tested finite sample does not discharge them.

### 1.4 Trust and delivery boundary

`Algorithm.FormalLanguages.Dfa` is an optional, explicitly imported
standard package, not a built-in or a prelude extension (`README §1`). The
record and operations are ordinary checked Ken; all seven public laws must
have kernel-checked proof terms over those same definitions. Their
propositions are proof-irrelevant `Ω` inhabitants over the stated `Type`
carriers, not proof-relevant data smuggled into `Ω`. The package adds **no
`Axiom`, primitive, kernel declaration or reduction rule, or
`trusted_base()` entry**. It must not treat an unchecked test, a table
implementation, or a finiteness assumption as proof of any law.

This chapter fixes the contract first; it does not claim that the catalog
package or these proofs are landed. `CAT-FORMAL-LANGUAGES-DFA` must deliver
both the computational definitions and their seven checked law proofs before
the package is complete. A partial tested build is not a proved package.

## 2. Finite-state evidence (deferred)

A later contract will give a `Finite q` certificate consisting of an
enumeration and membership proof, with decidable emptiness and reachability;
`Fin n` then receives an instance. No such certificate, decision procedure,
or finite-state guarantee is part of §1.

## 3. Nondeterministic automata (deferred)

A later contract will add NFA and subset construction with a
language-equality law. Section 1 neither supplies nondeterminism nor
postulates the subset-construction law.

## 4. Regular expressions (deferred)

A later contract will add regex and a derivative matcher. `DecEq a` enters
there; it is not a prerequisite for §1's alphabet.

## 5. Equivalence and minimisation (deferred)

Equivalence and minimisation require the later finite-state evidence and
decidable state equality. Section 1 has no such decision procedure.

## 6. Lexer input bridge (deferred)

A later `Bytes`/`Cursor` runner bridge for `Capability.Parsing` lexers will
relate byte-runner results to `run` over the corresponding byte list. Section
1 runs over `List a` only and makes no byte/lexer claim.
