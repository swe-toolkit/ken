# Formal languages: automata and regular expressions

> Status: **DRAFT v0 (SPEC-FORMAL-LANGUAGES-DFA-CONTRACT;
> SPEC-FORMAL-LANGUAGES-FINITE-REACHABILITY-CONTRACT;
> SPEC-FORMAL-LANGUAGES-NFA-CONTRACT;
> SPEC-FORMAL-LANGUAGES-REGEX-CONTRACT;
> SPEC-FORMAL-LANGUAGES-MINIMISATION-CONTRACT).** Sections 1–3 specify Dfa,
> finite reachability, and Nfa; section 4 specifies Regex; section 5
> specifies Dfa equivalence and canonical-representative minimisation.
> Section 6 remains deferred. A contract does not itself claim package
> delivery. No kernel, trust, or surface-syntax change is introduced.

A deterministic automaton describes a transition for every state and input
symbol and a Boolean acceptance test. Its state carrier need not be finite:
finite input words make `run` a terminating fold, but say nothing about how
many states the automaton can occupy. Nondeterministic automata (§3) use
Boolean edge tests but describe acceptance by a truncated path proposition;
a finite-state certificate permits an equivalent deterministic Boolean view.

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
only from the two start states. The checked proof may state its pair-valued
empty-word case as a separate private helper (for example,
`run_product_nil`); such a helper is not an eighth public law. This proof
layout avoids relying on the match arm to reduce a pair-typed goal to an
`Equal`-shaped goal before checking `Refl`, but the helper's existence is
not required. `accepts_product` transports the state-pair equality through
the product's final-state test. The intersection and union laws are
instances of this single general law with `combine = bool_and` and
`combine = bool_or`, respectively. These proof obligations concern the
actual functions exported by the package: a second specification-only
product or a tested finite sample does not discharge them.

### 1.4 Trust and delivery boundary

`Algorithm.FormalLanguages.Dfa` is an optional, explicitly imported
standard package, not a built-in or a prelude extension (`README §1`). Its
ordinary `data Dfa` declaration uses the existing checked inductive
machinery (`../10-kernel/14-inductive.md`,
`../10-kernel/18-judgments.md §4`). Its functions and theorems use existing
checked declaration machinery too. All seven
public laws must have kernel-checked proof terms over those same definitions.
Their propositions are proof-irrelevant `Ω` inhabitants over the stated
`Type` carriers, not proof-relevant data smuggled into `Ω`. The package adds
**no `Axiom`, primitive, new kernel declaration kind/form or reduction rule,
or `trusted_base()` entry**. It must not treat an unchecked test, a table
implementation, or a finiteness assumption as proof of any law.

This chapter fixes the contract first; it does not claim that the catalog
package or these proofs are landed. `CAT-FORMAL-LANGUAGES-DFA` must deliver
both the computational definitions and their seven checked law proofs before
the package is complete. A partial tested build is not a proved package.

## 2. Finite evidence and decidable reachability

Section 1 permits any state and alphabet types. This section specifies two
**optional, explicitly imported** packages for the case where both are
certified finite: `Data.Finite.Finite` supplies reusable evidence, and
`Algorithm.FormalLanguages.Reachability` supplies the decision. Neither
package changes the type of `Dfa` or requires state equality.

### 2.1 Certificate (`Data.Finite.Finite`)

`list_elem` belongs to the general `Data.Collections.Derived` package, not
to automata. Its element and list arguments are type-valued, but its result
is a proposition. The public definitions and certificate contract are:

```ken
pub fn list_elem (a : Type) (x : a) (xs : List a) : Omega =
  match xs {
    Nil ↦ Bottom;
    Cons y rest ↦ ‖ Or (Equal a x y) (list_elem a x rest) ‖
  }

pub data Finite (q : Type) : Type where {
  MkFinite : (listed : List q) →
             ((x : q) → list_elem q x listed) → Finite q
}

pub fn elements (q : Type) (fq : Finite q) : List q
pub theorem covers (q : Type) (fq : Finite q) :
  (x : q) → list_elem q x (elements q fq)
pub fn fin_finite (n : Nat) : Finite (Fin n)
pub fn pair_finite (q : Type) (r : Type) :
  Finite q → Finite r → Finite (Pair q r)
```

`Finite q` is a **record value**, not a class or an inferred instance. Its
`listed` field may include duplicates; its `covers` evidence applies to
**every** `x : q`. Empty carriers admit an empty list. `list_elem`'s `Nil`
case is `Bottom`; its `Cons` case truncates the proof-relevant `Or`, so
membership resides in `Omega`, not in an index-bearing `Type` family. For
`a, q : Type` at level zero, `Equal a x y : Omega` and the recursive
membership is in `Omega`; truncation returns `Omega`, while `Finite q` is
an ordinary `Type` record with a proof-irrelevant coverage field. Its
certificate does **not** construct `DecEq q`, prohibit duplicates, or
promise an efficiently searchable representation. Decidable equality of
states remains a separate prerequisite for reaching a **named** state by
comparing it with the current state, not for Dfa language equivalence or
minimisation (§5).

`elements q fq` projects the supplied list; `covers q fq x` projects its
checked coverage proof. `fin_finite n` constructs a value covering every
inhabitant of `Fin n`, including the empty case at `n = Zero`; it is **not**
an automatically resolved instance. `pair_finite q r fq fr` enumerates
pairs of entries from both lists and proves coverage from both certificates
using Pair's checked projections and η. This value can certify the state
carrier of §1's `product`, `intersection` and `union`. No condition of
unique entries, fixed enumeration order, or `DecEq` is added by either
constructor.

The public generic Ω-membership lemmas belong with `list_elem` in
`Data.Collections.Derived`. Their contracts support the certificate without
requiring decidable equality:

```ken
pub theorem list_elem_head (a : Type) (x : a) (rest : List a) :
  list_elem a x (Cons a x rest)
pub theorem list_elem_later (a : Type) (x : a) (y : a) (rest : List a) :
  list_elem a x rest → list_elem a x (Cons a y rest)
pub theorem list_elem_map
  (a : Type) (b : Type) (f : a → b) (x : a) (xs : List a) :
  list_elem a x xs → list_elem b (f x) (map a b f xs)
pub theorem list_elem_append_left
  (a : Type) (x : a) (xs : List a) (ys : List a) :
  list_elem a x xs → list_elem a x (list_append a xs ys)
pub theorem list_elem_append_right
  (a : Type) (x : a) (xs : List a) (ys : List a) :
  list_elem a x ys → list_elem a x (list_append a xs ys)
pub theorem list_elem_concat_map
  (a : Type) (b : Type) (f : a → List b)
  (y : b) (x : a) (xs : List a) :
  list_elem a x xs → list_elem b y (f x) →
  list_elem b y (concat_map a b f xs)
pub theorem list_elem_transport
  (a : Type) (x : a) (y : a) (same : Equal a x y) (xs : List a) :
  list_elem a x xs → list_elem a y xs
```

The certificate package also supplies an enumeration and its checked
coverage lemma for `Fin`; they are the construction behind `fin_finite`:

```ken
pub fn fin_elements (n : Nat) : List (Fin n)
theorem fin_elements_cover (n : Nat) (i : Fin n) :
  list_elem (Fin n) i (fin_elements n)
```

The certificate package imports those list tools,
`Fin`/`FZero`/`FSuc` from `Data.Vector.Vector`, and the existing `Or` and
checked equality transport. The certificate lives in `Data.Finite.Finite`,
**not** in `Data.Vector.Vector`, and Vector does not import it. None of
these proof helpers introduces a new trusted rule or a second notion of
membership.

### 2.2 Decision (`Algorithm.FormalLanguages.Reachability`)

The public operations have the following types. The certificate for the
alphabet is explicit: searching all next symbols needs coverage for `a`
as well as coverage for `q`. A target is a **Boolean predicate on states**,
and the initial state is supplied by the caller except in the two accepted-
language operations.

```ken
pub fn find_word
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (target : q → Bool) (s : q) : Option (List a)
pub fn reachable
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (target : q → Bool) (s : q) : Bool
pub fn accepted_word
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) : Option (List a)
pub fn is_empty
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) : Bool
```

`find_word fq fa d target s` returns `Some w` **only if** `w` takes `s` to
a target state. It returns `None` exactly if there is no finite word that
reaches a target state. It need not choose a shortest or unique witness.
The other three operations are fixed views of this result:

```ken
reachable q a fq fa d target s =
  is_some (List a) (find_word q a fq fa d target s)
accepted_word q a fq fa d =
  find_word q a fq fa d (final q a d) (start q a d)
is_empty q a fq fa d =
  bool_not (reachable q a fq fa d (final q a d) (start q a d))
```

Here `is_some` is the existing `Data.Sums.Combinators` function, and
`bool_not` is the public Boolean function promised by §1's Dfa package
build; it is not assumed landed on this Spec base. A
`True` emptiness result means no accepted word exists; a `False` result
permits extracting an accepted `Some w` by case analysis on
`accepted_word`, not by assuming a word from a bare Boolean. The decision
is about **reachability of a predicate**; reaching a named state by
comparison is not supplied without `DecEq q` (§5).

### 2.3 Checked laws and completeness boundary

These four public laws have kernel-checked proofs over the **exported**
functions, not postulates or tests. Their types quantify over any finite
certificates, automaton, predicate, starting state and word; the proof
terms introduce no special property of `q` or `a` beyond the certificates.
Every `Equal Bool` proposition and each `Equal (Option (List a))` premise
inhabits `Omega`; each implication is a proposition.

```ken
pub theorem find_word_sound
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (target : q → Bool) (s : q) (w : List a) :
  Equal (Option (List a)) (find_word q a fq fa d target s)
    (Some (List a) w) →
  Equal Bool (target (run q a d s w)) True

pub theorem find_word_complete
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (target : q → Bool) (s : q) (w : List a) :
  Equal Bool (target (run q a d s w)) True →
  Equal Bool (reachable q a fq fa d target s) True

pub theorem accepted_word_accepts
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (w : List a) :
  Equal (Option (List a)) (accepted_word q a fq fa d)
    (Some (List a) w) →
  Equal Bool (accepts q a d w) True

pub theorem is_empty_rejects
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (w : List a) :
  Equal Bool (is_empty q a fq fa d) True →
  Equal Bool (accepts q a d w) False
```

Completeness is **in this contract**: `find_word_complete` applies to
**every** witness word, however long. A valid proof route uses a finite
symbol list and fuel bounded by `length q (elements q fq)` for the primary
search. A fuel-limited reachability predicate is monotone with fuel.
Counting its true entries over the possibly duplicate-containing state
list never exceeds that list's length; whenever the predicate has not
stabilised, the count strictly grows. Coverage lifts agreement on list
entries to all states by eliminating Ω membership **only into a
proposition**. Consequently reachability stabilises by the list-length
bound and stays stable; a word of arbitrary length yields reachability at
its length and hence at the bound. Alphabet coverage ensures that each
symbol in that word is considered by the search. This argument requires
neither `DecEq q` nor distinct enumeration elements, and does **not**
promise a time or space bound. A later table or BFS implementation must
prove the same four laws, not introduce an axiom to replace completeness.

The decision package imports `Data.Finite.Finite`, §1's Dfa API,
`Data.Sums.Combinators.is_some`, `bool_or`/`leq_nat` and the §1-promoted
`bool_not` plus `proof trans for leq_nat` from
`Core.Classes.LawfulClasses`,
`leq_nat_weaken_right` from `Data.Numeric.Nat.Order`,
`Data.Collections.List.length`; `map`, `concat_map` and the `list_elem`
tools from `Data.Collections.Derived`; and checked equality transport.
Private search, reach, stability, counting
and auxiliary proofs do not expand the public contract. For a
human-readable catalog package, lead with `is_empty`/`accepted_word` and
their laws, then the general `find_word` laws, followed by search and its
counting proof. The necessary data declarations precede their uses.

Both packages are ordinary checked Ken: they may declare the `Finite`
record, definitions and theorem terms through existing inductive and
judgment machinery, but add **no `Axiom`, primitive, new kernel declaration
kind/form, reduction rule, or `trusted_base()` entry**. None is a built-in,
a prelude addition, or a surface-syntax change. This section specifies a
contract, not an assertion that either package or its proofs has landed.

## 3. Nondeterministic automata and subset construction

`Algorithm.FormalLanguages.Nfa` is an optional, explicitly imported catalog
package. Its nondeterministic semantics is independent of finiteness:
`Nfa q a` can describe any `q : Type` and `a : Type`. Finiteness enters
only when determinizing a particular `q`, and alphabet finiteness enters
only when deciding emptiness of the resulting Dfa (§2).

### 3.1 Carrier, path and acceptance

The public carrier has a **Boolean transition relation**, an initial-state
predicate, and a final-state predicate, in that order. There may be several
initial states or outgoing successors, including none. The three public
projections expose the fields without requiring either state or symbol
equality:

```ken
pub data Nfa q a = MkNfa (q → a → q → Bool) (q → Bool) (q → Bool)
export MkNfa

pub fn nfa_step (q : Type) (a : Type) (n : Nfa q a) :
  q → a → q → Bool
pub fn nfa_initial (q : Type) (a : Type) (n : Nfa q a) :
  q → Bool
pub fn nfa_final (q : Type) (a : Type) (n : Nfa q a) :
  q → Bool
```

For `n = MkNfa transition initial accepting`, the projections are
`nfa_step q a n s x t = transition s x t`,
`nfa_initial q a n s = initial s`, and
`nfa_final q a n s = accepting s`. `nfa_step s x t` tests an edge from
`s` **to** `t` on `x`; it is not the reversed relation. A successor-list
field would require a decision about whether `t` occurs among successors
of `s`, reintroducing the forbidden `DecEq q` requirement. A Boolean
relation gives the subset construction an executable edge test directly.

A path records the state **after each symbol**. For a word and path of
unequal lengths `path_accepts` returns `Bottom`. On the empty word and
empty path, the current state must be final. On matching nonempty lists,
the first path state must be an edge successor, and the remainder must
accept from that successor. This definition uses generic proof-relevant
conjunction from the **separate** `Core.Logic.And` package:

```ken
pub data And (left : Omega) (right : Omega) : Type where {
  Both : left → right → And left right
}
export Both
```

`And`/`Both` are generic public data alongside `Core.Logic.Or`, not
Nfa-private machinery. `Pair` cannot replace `And` here: a pair of two
Ω propositions is not a `Pair` of `Type` fields. The ordinary `Type`
family records evidence for both Ω propositions; truncation converts
that proof-relevant conjunction into an Ω proposition at each path step.
The public path contract is:

```ken
pub fn path_accepts
  (q : Type) (a : Type) (n : Nfa q a)
  (s : q) (w : List a) (path : List q) : Omega =
  match w {
    Nil ↦ match path {
      Nil ↦ Equal Bool (nfa_final q a n s) True;
      Cons t ts ↦ Bottom
    };
    Cons x rest ↦ match path {
      Nil ↦ Bottom;
      Cons t ts ↦ ‖ And
        (Equal Bool (nfa_step q a n s x t) True)
        (path_accepts q a n t rest ts) ‖
    }
  }

pub data NfaAcceptance
  (q : Type) (a : Type) (n : Nfa q a) (w : List a) : Type where {
  Accepted : (s : q) → (path : List q) →
    Equal Bool (nfa_initial q a n s) True →
    path_accepts q a n s w path → NfaAcceptance q a n w
}
export Accepted

pub fn nfa_accepts
  (q : Type) (a : Type) (n : Nfa q a) (w : List a) : Omega =
  ‖ NfaAcceptance q a n w ‖
```

`NfaAcceptance` retains the starting state and complete path as
`Type`-sorted evidence; `Accepted` must carry the initial-state and path
proofs. `nfa_accepts` truncates their existence to Ω. For level-zero
`q, a : Type`, `Equal Bool ... True : Omega`, `And` and
`NfaAcceptance : Type`, and their truncations are in `Omega`. The Π
implications in §3.3 also land in Ω by the predicative maximum of their
level-zero domains and Ω codomains. Only **proofs of propositions** are
proof-irrelevant; `NfaAcceptance` and `And` do not erase their witnesses
merely because their constructor fields include propositions. NFA
acceptance is not a Boolean decision on an arbitrary carrier. It requires
no `Finite q`, `Finite a`, `DecEq q` or `DecEq a`, and has no ε-transition.

### 3.2 Subset construction and finite evidence

Given explicit `fq : Finite q` (§2), the public `subset_state q fq` is a
mask with one Boolean bit per **position** of `elements q fq`. Its ordinary
level-zero `Type` carrier is recursively shaped as `Unit` for `Nil` and
`Pair Bool (mask q rest)` for `Cons`; it is not an indexed family or a
published `Vec`. Duplicate listed states yield duplicate positions and
do not invalidate coverage or force deduplication. The private `mask`
representation and its helpers are not an additional public data API.

```ken
pub fn subset_state (q : Type) (fq : Finite q) : Type
pub fn determinize
  (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) :
  Dfa (subset_state q fq) a
pub fn subset_finite (q : Type) (fq : Finite q) :
  Finite (subset_state q fq)
```

`subset_state q fq` computes to `mask q (elements q fq)` with that shape.
`determinize q a fq n` starts with bit `s` set when
`nfa_initial q a n s` is `True`. For each input `x`, its next-state bit
at position `t` is set exactly when some currently set position `s` has
`nfa_step q a n s x t = True`. Its final test is `True` exactly when some
set position `s` has `nfa_final q a n s = True`. The Boolean tests use
the supplied list and `Core.Classes.LawfulClasses.bool_or`/`bool_and`;
these operations do not inspect equality of states or symbols. `Finite q`
supplies both the bit positions and `covers q fq s` for **every** state in
an accepting path. It is required for `determinize`, but `Finite a` is
not: a given transition consumes one supplied symbol at a time.

Two generic, **public values** belong in `Data.Finite.Finite`, beside
`fin_finite` and `pair_finite`, rather than inside Nfa:

```ken
pub const unit_finite : Finite Unit
pub const bool_finite : Finite Bool
```

They certify the ordinary `Unit` and `Bool` carriers with checked
coverage. `subset_finite` recursively uses `unit_finite` for the empty
mask and `pair_finite` on `Bool` (with `bool_finite`) and the recursively
certified tail mask for a nonempty mask. It is an explicit **value**, not
a class instance or inference rule, and it does not produce `DecEq q`.
Because every mask position has a Boolean value, §2's decision applies
to the constructed Dfa, even if
`elements q fq` has duplicates.

### 3.3 Checked language equality and emptiness

These three **public proved theorem types** bind the exported definitions.
The first two are the language-equality law, stated in both directions
between the Dfa's Boolean acceptance and the Nfa's Ω acceptance; no new
`Iff` or judgment is needed. Their conclusions are Ω propositions, not
proof-relevant path-returning algorithms.

```ken
pub theorem determinize_sound
  (q : Type) (a : Type) (fq : Finite q)
  (n : Nfa q a) (w : List a) :
  Equal Bool
    (accepts (subset_state q fq) a (determinize q a fq n) w) True →
  nfa_accepts q a n w

pub theorem determinize_complete
  (q : Type) (a : Type) (fq : Finite q)
  (n : Nfa q a) (w : List a) :
  nfa_accepts q a n w →
  Equal Bool
    (accepts (subset_state q fq) a (determinize q a fq n) w) True

pub fn nfa_is_empty
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (n : Nfa q a) : Bool =
  is_empty (subset_state q fq) a (subset_finite q fq) fa
    (determinize q a fq n)

pub theorem nfa_is_empty_rejects
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (n : Nfa q a) (w : List a) :
  Equal Bool (nfa_is_empty q a fq fa n) True →
  nfa_accepts q a n w → Bottom
```

`determinize_sound` traces a set final-state bit back through each input
transition to an initial state and constructs the path; the result is
truncated into Ω. `determinize_complete` walks each accepting path
forward, using `covers` to keep the corresponding bit set. Both
arguments generalise the mask for induction over the word and use the
same exported `determinize`, `accepts`, `path_accepts` and `nfa_accepts`
as the contract. They do not assert that an arbitrary subset mask is a
list of distinct states. A tested sample, unproved lemma or second
specification-only automaton cannot discharge either direction.

`nfa_is_empty` reuses §2's `is_empty` with `subset_finite q fq` and
`fa : Finite a`. `True` means no word is accepted; the displayed
`nfa_is_empty_rejects` is checked using §2's `is_empty_rejects` and
`determinize_complete`. `False` need not return a public NFA witness in
this contract. This is decidable emptiness for **finite certified states
and alphabet**, not a Boolean NFA acceptance procedure for an arbitrary
carrier. No bound on time or space is promised.

The Nfa package imports §1's Dfa and §2's Finite/Reachability APIs,
`Core.Logic.And`/`Core.Logic.Or`, checked transport, Boolean operations
and `Data.Collections.Derived`'s Ω-membership tools. `mask`, `mask_any`,
`mask_build`, `holds`, `mask_finite`, `subset_next` and the induction
lemmas remain private; the package is free to reorganize these internals
while proving the same public laws. All required definitions and proof
terms are checked by Ken's **existing** inductive, recursive and judgment
machinery. No `Axiom`, postulate, primitive, foreign value, new kernel
declaration kind/form or reduction rule, prelude addition, surface-syntax
change, or `trusted_base()` entry is introduced.
The checked development in the Architect's D0 ruling establishes this
contract's feasibility, not that any §3 package or theorem has landed.

## 4. Regular expressions and derivative matching

`Algorithm.FormalLanguages.Regex` is an optional, explicitly imported
package. A regular expression denotes a language of finite `List a` words
without deciding equality of alphabet symbols. A separate matcher decides
membership when the caller supplies `DecEq a`; no finite alphabet, finite
state certificate, or state equality is required. The denotation is defined
independently of the matcher, so the checked laws compare two distinct
constructions rather than validating a computation against itself.

### 4.1 Carrier and independent Ω denotation

The public carrier and its six exported constructors are ordinary checked
inductive data. `Fail` denotes no word, `Eps` only the empty word, `Sym c`
only the one-symbol word containing `c`, `Alt` either language, `Cat` word
concatenation, and `Star` finite repetition, including zero repetitions.
`Fail` avoids reusing the `Empty` spelling already owned by
`Core.Logic.EmptyDec` for a different type (`../30-surface/30 §4`).
Forming `Regex a` imposes no `DecEq a`, `Finite a`, or ordering constraint.

```ken
pub data Regex a =
  Fail | Eps | Sym a | Alt (Regex a) (Regex a)
  | Cat (Regex a) (Regex a) | Star (Regex a)
export Fail, Eps, Sym, Alt, Cat, Star
```

Two public, `Type`-sorted evidence families make concatenation and finite
repetition inspectable before truncation. `Split` witnesses a particular
factorisation `w = u ++ v` with a proof for each side. `Pieces` witnesses a
finite list of words whose concatenation is `w`, with a proof that each word
satisfies `p`. Their constructors are exported because they occur in the
reduct of the public language predicate.

```ken
pub data Split
  (a : Type) (p : List a → Omega) (r : List a → Omega)
  (w : List a) : Type where {
  MkSplit : (u : List a) → (v : List a) →
    Equal (List a) (list_append a u v) w →
    p u → r v → Split a p r w
}
export MkSplit

pub data Pieces
  (a : Type) (p : List a → Omega) (w : List a) : Type where {
  MkPieces : (ws : List (List a)) →
    Equal (List a) (list_concat a ws) w →
    list_all (List a) p ws → Pieces a p w
}
export MkPieces

pub fn regex_lang (a : Type) (r : Regex a) : List a → Omega
```

`regex_lang a r w` is structurally defined by `r`; the following equations
fix its result. `Fail` gives `Bottom` on every word. `Eps` gives
`Equal (List a) w (Nil a)`. `Sym c` gives `Bottom` on `Nil`, and on
`Cons a x rest` gives
`‖ And (Equal a x c) (Equal (List a) rest (Nil a)) ‖`.
`Alt r1 r2` gives
`‖ Or (regex_lang a r1 w) (regex_lang a r2 w) ‖`;
`Cat r1 r2` gives
`‖ Split a (regex_lang a r1) (regex_lang a r2) w ‖`;
`Star r1` gives `‖ Pieces a (regex_lang a r1) w ‖`.
The `Nil` list of pieces witnesses the empty word for every `Star`, even
when `r1 = Fail`; empty pieces do not force a nonempty match.

The two reusable list operations in those evidence types belong in
`Data.Collections.Derived`, beside `list_append`, **not** inside Regex:

```ken
pub fn list_concat (a : Type) (ws : List (List a)) : List a
pub fn list_all (t : Type) (p : t → Omega) (xs : List t) : Omega
```

`list_concat a [] = []` and
`list_concat a (u :: us) = list_append a u (list_concat a us)`.
`list_all t p [] = Equal Bool True True`, and
`list_all t p (y :: ys) = ‖ And (p y) (list_all t p ys) ‖`.
Thus `list_all` is an Ω predicate, not a Boolean decision or a product
of propositions stored in `Pair`. For level-zero `a : Type`, the equalities
in `Split` and `Pieces` inhabit `Omega`; the families themselves are
ordinary `Type`. `Or` and `And` are proof-relevant `Type` families whose
truncations inhabit `Omega`. Only their truncated propositions, and proofs
of propositions, are proof-irrelevant; no witness or list of pieces is
smuggled into Ω. The `List a → Omega` predicates and the theorem
implications below land at the predicative maximum of their level-zero
domains and Ω codomains.

### 4.2 Nullability, derivative, and finite-word decision

The three public operations have these types and behavior:

```ken
pub fn nullable (a : Type) (r : Regex a) : Bool
pub fn deriv (a : Type) (d : DecEq a) (x : a) (r : Regex a) : Regex a
pub fn regex_matches
  (a : Type) (d : DecEq a) (r : Regex a) (w : List a) : Bool
```

`nullable` is `False` on `Fail` and `Sym`, `True` on `Eps` and `Star`,
`bool_or` of its operands on `Alt`, and `bool_and` of its operands on
`Cat`. It is a structural Boolean computation; it uses no alphabet
equality. Write `guard b r = r` when `b = True` and `Fail` otherwise;
this is a private abbreviation, not a public operation.

`deriv d x` computes the left quotient by symbol `x`. Its `Fail` and
`Eps` cases produce `Fail`. Its `Sym c` case produces
`guard (d.eq x c) Eps`. Its `Alt r1 r2` case is the alternation of
`deriv d x r1` and `deriv d x r2`. Its `Cat r1 r2` case is the alternation
of `Cat (deriv d x r1) r2` and
`guard (nullable r1) (deriv d x r2)`; the second alternative exists only
when the left expression accepts the empty word. Its `Star r1` case is
`Cat (deriv d x r1) (Star r1)`. These equations specify the language of
the returned expression; no simplification or canonical syntax is promised.

`regex_matches d r [] = nullable r` and
`regex_matches d r (x :: xs) = regex_matches d (deriv d x r) xs`.
The word's structural descent makes the matcher total on finite inputs;
this is not a claim that repeatedly generated derivative expressions have
a finite quotient or bounded representation. `DecEq a` is an explicit
**value**, not an inferred constraint: it is needed by `deriv`'s `Sym`
case and consequently by `regex_matches`, but neither the carrier,
`regex_lang`, nor `nullable` requires it. Symbol matching uses both
`d.sound` and `d.complete`; an `Eq a` Boolean equivalence alone does
not connect its result to Ken's `Equal a` proposition.

### 4.3 Six checked laws

The following six public laws are proved theorems over the **same exported**
definitions. `Equal Bool ... True` is a checked Ω proposition, not an
informal true result. For `a : Type` at level zero, each implication
below inhabits Ω. None is an `Axiom`, an unfilled obligation, a test result,
or a theorem about a second specification-only matcher.

```ken
pub theorem nullable_sound (a : Type) (r : Regex a) :
  Equal Bool (nullable a r) True → regex_lang a r (Nil a)

pub theorem nullable_complete (a : Type) (r : Regex a) :
  regex_lang a r (Nil a) → Equal Bool (nullable a r) True

pub theorem deriv_sound (a : Type) (d : DecEq a) (x : a) (r : Regex a) :
  (w : List a) →
  regex_lang a (deriv a d x r) w → regex_lang a r (Cons a x w)

pub theorem deriv_complete (a : Type) (d : DecEq a) (x : a) (r : Regex a) :
  (w : List a) →
  regex_lang a r (Cons a x w) → regex_lang a (deriv a d x r) w

pub theorem regex_matches_sound
  (a : Type) (d : DecEq a) (r : Regex a) (w : List a) :
  Equal Bool (regex_matches a d r w) True → regex_lang a r w

pub theorem regex_matches_complete
  (a : Type) (d : DecEq a) (r : Regex a) (w : List a) :
  regex_lang a r w → Equal Bool (regex_matches a d r w) True
```

`nullable_sound` and `nullable_complete` equate empty-word denotation with
nullability. `deriv_sound` and `deriv_complete` equate acceptance of
`x :: w` with acceptance of `w` by the derivative, in **both**
directions. `regex_matches_sound` and `regex_matches_complete` lift these
facts by induction on the supplied word. A law whose premise is false
vacuously proves nothing about a positive instance; acceptance examples
must instantiate these laws with inhabited antecedents.

A proof of `Cat` completeness must handle both splits where the left word
is empty and where it starts with the input symbol. A proof of `Star`
completeness must handle an empty piece list, leading empty pieces, and
a first nonempty piece without assuming any bound on repetitions. The
proofs may use private structural lemmas; no private lemma names or proof
body layout is part of this contract. The checked equality interface
supplies `cong`/`trans`; the Boolean calculations use the existing public
`bool_or`/`bool_and`. Five generic Bool-truth lemmas needed by the proof
belong once in `Core.Classes.LawfulClasses`, not as another copy in Regex.
Their D0-checked types, to be exposed there by the catalog build, are:

```ken
pub theorem or_left (b : Bool) (c : Bool) :
  Equal Bool b True → Equal Bool (bool_or b c) True
pub theorem or_right (b : Bool) (c : Bool) :
  Equal Bool c True → Equal Bool (bool_or b c) True
pub theorem or_cases (b : Bool) (c : Bool) (g : Omega) :
  Equal Bool (bool_or b c) True →
  (Equal Bool b True → g) → (Equal Bool c True → g) → g
pub theorem and_true (b : Bool) (c : Bool) :
  Equal Bool b True → Equal Bool c True →
  Equal Bool (bool_and b c) True
pub theorem and_cases (b : Bool) (c : Bool) (g : Omega) :
  Equal Bool (bool_and b c) True →
  (Equal Bool b True → Equal Bool c True → g) → g
```

The Regex catalog build therefore depends on the **shared**
`Data.Collections.Derived.list_concat`/`list_all` and these five
`Core.Classes.LawfulClasses` proof helpers. They are catalog delivery
items, not declarations already landed on this Spec base and not an
invitation to extend this Spec WP into `catalog/`.

### 4.4 Trust, scope, and later automata links

The Regex package uses existing `List`, `Bool`, `Equal`, truncation,
structural induction, `Core.Logic.Or`/`And`, the public `DecEq` dictionary,
and the two shared list operations. All six laws require kernel-checked
proof terms. The Architect's checked D0 development demonstrates
feasibility with rc=0 and no `Axiom`; it does not establish that §4's
package, its shared dependencies, or these laws have landed. This
contract introduces **no `Axiom`, postulate, primitive, new kernel form or
reduction rule, prelude name, surface-syntax change, or `trusted_base()`
entry**. A tested matcher without these checked proofs is a partial
increment, not a completed proved package.

No conversion to a Dfa or Nfa, automaton language-equality theorem,
finite-derivative-quotient proof, or decidable state-equality law is in
§4. The Nfa of §3 has no ε-transitions; an ε-free position construction
would need its own finite-position certificate and language proof. A Dfa
from derivatives would additionally need a finite quotient of derivative
states. Those contracts, if introduced, belong after this section. Section 5
specifies Dfa equivalence and minimisation **without** `DecEq q`; it does
not link Regex to an automaton. No automaton package is imported by Regex.

## 5. Equivalence and minimisation

`Algorithm.FormalLanguages.Minimisation` is an optional, explicitly
imported, ordinary Ken package over §§1–2. It compares the languages of
two Dfa machines, or the futures of two particular states, using explicit
finite certificates for both state carriers and the shared alphabet. It
also normalises a Dfa to canonical representatives **within its original
carrier `q`**. Neither operation compares states for equality, so neither
requires `DecEq q`. `Finite q` remains a duplicate-permitting coverage
certificate, not an inferred equality decision. This section declares
five public operations and **eight public checked laws**; proof helpers
and a particular canonical-representative choice remain private.

### 5.1 Equality of future languages and its decision

The public proposition `same_future` describes language equality from
arbitrary states, even without finite certificates. For all finite words,
the two machines' Boolean final tests agree after independently running
that **same** word from their specified states. The state carriers may
differ (`q` and `r`); the alphabet `a` must be shared.

```ken
pub fn same_future
  (q : Type) (r : Type) (a : Type)
  (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) : Omega =
  (w : List a) →
    Equal Bool (final q a d (run q a d s w))
               (final r a e (run r a e t w))

pub fn equivalent
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) : Bool

pub fn equivalent_states
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) : Bool
```

The two decisions use §1's `product` and §2's `reachable`. Define the
private Boolean discriminator `disagree b c = bool_not (bool_eq b c)`:
`disagree True True = False`, `disagree False False = False`, and each
mixed pair gives `True`. The public `bool_eq` and `bool_not` come from
`Core.Classes.LawfulClasses`. The product's paired state is final
exactly when the component final tests **differ**. With
`p = product q r a disagree d e` and
`fp = pair_finite q r fq fr`, the fixed decision is

```ken
equivalent_states q r a fq fr fa d e s t =
  bool_not (reachable (Pair q r) a fp fa p
            (final (Pair q r) a p) (mk_pair q r s t))
equivalent q r a fq fr fa d e =
  equivalent_states q r a fq fr fa d e
    (start q a d) (start r a e)
```

Thus `equivalent` is also the `is_empty` view of this disagreement
product at the product's start. A `True` result means that **no** word
reaches a disagreement; `False` means a disagreement word exists, but
the decision does not promise which one `find_word` returns. Both state
certificates compose via §2's `pair_finite`; the input certificate is
required to search all possible symbols. Neither certificate decides
`Equal q` or `Equal r`.

Four public theorems relate this Boolean decision to the independent
future-language proposition. The first two quantify over the machines'
start states, the last two over any specified states. All binder lists
are explicit; in particular, `same_future` needs no `Finite` value,
while deciding it does.

```ken
pub theorem equivalent_sound
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) :
  Equal Bool (equivalent q r a fq fr fa d e) True →
  (w : List a) → Equal Bool (accepts q a d w) (accepts r a e w)

pub theorem equivalent_complete
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) :
  ((w : List a) → Equal Bool (accepts q a d w) (accepts r a e w)) →
  Equal Bool (equivalent q r a fq fr fa d e) True

pub theorem equivalent_states_sound
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) :
  Equal Bool (equivalent_states q r a fq fr fa d e s t) True →
  same_future q r a d e s t

pub theorem equivalent_states_complete
  (q : Type) (r : Type) (a : Type)
  (fq : Finite q) (fr : Finite r) (fa : Finite a)
  (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) :
  same_future q r a d e s t →
  Equal Bool (equivalent_states q r a fq fr fa d e s t) True
```

The checked `run_product` law relates a word's product state to the
pair of component runs. For soundness, a reachable disagreement would
contradict the decision's `True` result via `find_word_complete`; when
the product's final test is `False`, the two finals agree. For
completeness, a returned `Some w` would supply a `True` disagreement
through `find_word_sound`, contradicting the all-words agreement. These
are proofs against the **exported** product and decision, not tests of
a second specification-only decision. There is no state comparison:
only the two Boolean final results are compared.

### 5.2 Canonical representatives in the same carrier

`canonical` maps any state to a representative with the same future
language. Equal future languages map to **equal states** under this
map. These two laws fix what is observable about canonicalisation;
no particular element or order of `elements q fq` is promised. The
checked D0 construction can choose the first equivalent element of
the supplied finite list, but that is one private implementation, not
an additional contract, public choice, or state-equality decision.

```ken
pub fn canonical
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (s : q) : q

pub theorem canonical_same_future
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (s : q) :
  same_future q q a d d (canonical q a fq fa d s) s

pub theorem canonical_unique
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (s : q) (t : q) :
  same_future q q a d d s t →
  Equal q (canonical q a fq fa d s) (canonical q a fq fa d t)

pub fn minimise
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) : Dfa q a
```

The `minimise` machine has exactly carrier `q`. Its initial state is
`canonical q a fq fa d (start q a d)`. Its transition from state `s`
on symbol `x` is `canonical q a fq fa d (step q a d s x)`; its final test
is the original `final q a d`. All reachable states from its start are
canonical fixed points; states outside that reachable subgraph remain
in `q` and need not be distinct or reachable. There is no fresh
quotient type, `Fin`-indexed table, state count, or `DecEq q` argument.

Two further checked public laws state the behavioral guarantee and its
zero-trust minimality form. `accepts_minimise` preserves the accepted
language on **every** input word. `minimise_reduced` compares only the
states reached by words `u` and `v` from the minimised start: if their
future accepted languages agree for **every suffix** `w`, the reached
states are equal in `q`. It neither asserts that `q` contains the fewest
possible elements nor equates arbitrary unreachable states. All four
minimisation-side laws are statements about this same exported
`canonical` and `minimise`.

```ken
pub theorem accepts_minimise
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (w : List a) :
  Equal Bool (accepts q a (minimise q a fq fa d) w)
             (accepts q a d w)

pub theorem minimise_reduced
  (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
  (d : Dfa q a) (u : List a) (v : List a) :
  ((w : List a) →
    Equal Bool
      (accepts q a (minimise q a fq fa d) (list_append a u w))
      (accepts q a (minimise q a fq fa d) (list_append a v w))) →
  Equal q
    (run q a (minimise q a fq fa d)
      (start q a (minimise q a fq fa d)) u)
    (run q a (minimise q a fq fa d)
      (start q a (minimise q a fq fa d)) v)
```

The `canonical_same_future` proof relates the result of canonicalising
to its input; `canonical_unique` uses the `Finite q` coverage proof and
§5.1's sound and complete decision, not a Boolean decision on `Equal q`.
`run_append` and induction on the supplied words relate the futures of
two reached states to suffix acceptance. The laws together establish
that canonicalising twice is idempotent, that minimised runs stay in
canonical states, and that equal reached futures collapse as
`minimise_reduced` states. These are consequences, not further named
public laws or complexity bounds.

### 5.3 Trust boundary and exclusions

For `q`, `r`, `a : Type` at level zero, `Pair q r` and `List a` are
ordinary `Type` carriers, whereas their checked `Equal Bool` and
`Equal q` propositions are in `Omega`. The dependent product over
`List a` of `Equal Bool` propositions — `same_future` — also lands
in `Omega` by the predicative maximum. The law implications and
proofs of these propositions do not expose a proof-relevant witness or
require an impredicative elimination. `Finite` certificates, Boolean
comparison, products, and checked induction are existing library/
kernel machinery; no extra axiom proves a semantic property of
`canonical` or of `minimise`.

All eight public laws must be kernel-checked proofs over the delivered
package; `Axiom`, an open obligation, or green tests alone cannot
complete it. The Architect's D0 development checked at rc=0 with no
`Axiom`/postulate and demonstrated the eight laws' feasibility, not
that this §5 package or its seed has landed. The contract introduces
**no new kernel declaration kind/form, primitive, reduction rule,
prelude identity, surface syntax, or `trusted_base()` entry**. Ordinary
declarations use the existing checked machinery. The package may use
private lemmas but does not export a ninth law.

A cardinality-minimal DFA against any equivalent machine is **not**
claimed: `Finite q` permits duplicate enumeration and alone supplies
no decidable equality or distinct-state count, and such a theorem needs
additional cardinality and pigeonhole machinery. A quotient on a new
`Fin` carrier, trimming unreachable states, named-state reachability
by `DecEq q`, complexity bounds or a partition-refinement algorithm,
and Nfa/Regex equivalence remain outside §5. An Nfa decision could
compose with §3's `determinize`; a Regex-to-automaton decision would
first need §4.4's separately proved finite derivative quotient. Neither
is silently provided by this package. A future implementation of a
faster algorithm must still prove the **same eight laws**.


## 6. Lexer input bridge (deferred)

A later `Bytes`/`Cursor` runner bridge for `Capability.Parsing` lexers will
relate byte-runner results to `run` over the corresponding byte list.
Sections 1–5 use `List a` only and make no byte/lexer claim.
