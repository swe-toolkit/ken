# Priority-queue law completion: component design

This is the Architect's proof-strategy ruling for
`CAT-PRIORITY-QUEUE-LAWS`, grounded at
`939c1661d95d4a0c1865fd68b431936f168db9a9`. It authorizes proof authoring,
not a claim that the complete theorem suite already checks.

The computational provider blob is
`1eec578603cdef349af21d944ac174af5919705c`. Keep that queue and its five
operations. The package is an unfinished computational increment until these
general proofs land, per `docs/PRINCIPLES.md` principle 16.

## Boundary and proof surface

All queue predicates, measures, observation functions, and queue theorems remain
private in `Data.Collections.PriorityQueue`. Its six public names and their
types stay unchanged. Private checked theorems are intrinsic package proofs;
this ruling does not promise an importable client proof API. Exporting such an
API requires a separate surface ruling, not silently weakening the six-name
contract.

Do not introduce another queue, change `meld` into a measured or fueled
algorithm, or add a sorting/extraction fallback. The observation functions
below occur only in the proof development. No existing queue operation calls
them. They may traverse a queue to state its properties; that is not permission
to insert a traversal into the operational call graph.

Use ordinary data in `Type` and Boolean predicates reflected through the
existing `IsTrue`. No proof-relevant inductive in `Omega`, postulate, primitive,
new trust entry, hole, or kernel change is authorized. Preserve and disclose
inherited assumptions of the supplied `Ord`; zero new trust is not zero
inherited trust. Complexity proofs remain outside this tranche.

## Exact definitions

The following is mathematical notation, not unverified surface code. Suppress
only the displayed fixed parameters `k`, `v`, `d : Ord k`. Write
`leq = ord_leq_at k d`, `Q = PriorityQueue k v leq`, and `E = Pair k v`.
Every operation in a proposition denotes the actual provider definition.

For arbitrary `p : k -> v -> Bool`, define `bit_p (x,y)` as one when `p x y`
is true, zero otherwise. Define the private structural measure:

```
C_p Empty = 0
C_p (Node r x y l t) = bit_p(x,y) + (C_p l + C_p t)
size q = C_(lambda x y. True) q
```

Here `+` is the existing `Data.Numeric.Nat.Arithmetic.add`, which recurses on
its SECOND operand. Counts quantify over arbitrary predicates and payload
types, not just equality predicates, keys, or a particular dictionary.
The count and balancing lemmas can be stronger than the public laws: quantify
over arbitrary `leq`, without validity or order-law premises.

Define `root_bound b Empty = True` and
`root_bound b (Node r x y l t) = leq b x`. Define `all_above b` by structural
recursion, conjoining `leq b x` and the two recursive results at a node.

Define `valid` by structural recursion. Empty is true. A node requires ALL of:

- both children satisfy `valid`;
- `root_bound x l` and `root_bound x t`;
- `leq_nat (rank t) (rank l)`;
- cached `r` equals `Suc (rank t)`.

The last Boolean clause may use both directions of canonical `leq_nat`;
prove its equivalence to `Equal Nat r (Suc (rank t))` using the existing
canonical Nat antisymmetry proof. Do not use an arbitrary `Ord Nat` relation
for structural ranks. Set `Valid q := IsTrue (valid q)`.

This local-root formulation must be proved to imply the global descendant
condition. Also prove `Valid q` implies `rank q` equals the structurally defined
right-spine length. Thus the cache predicate is bound to real shape, not merely
to other unchecked caches. No logarithmic complexity theorem follows here.

## Required general propositions

All displayed equalities are checked Ken equality propositions at their stated
carrier, not host comparisons. Quantify every free queue, entry, predicate,
priority and payload. Do not add validity premises to the conservation laws.

1. `Valid (empty d)` and validity of every singleton used by `insert`.
2. `Valid a -> Valid b -> Valid (meld leq a b)` and its actual `merge d` wrapper.
3. `Valid q -> Valid (insert d x y q)`.
4. If `pop_min d q = Some (e,q')`, then `Valid q -> Valid q'`.
5. `C_p (empty d) = 0`.
6. `C_p (merge d a b) = C_p a + C_p b`.
7. `C_p (insert d x y q) = bit_p(x,y) + C_p q`.
8. If `pop_min d q = Some (e,q')`, then
   `C_p q = bit_p(e) + C_p q'`, for EVERY `p`.
9. For `q = Node r x y l t`, `Valid q -> IsTrue (all_above x q)`.
   The root is an actual occurrence, and it bounds every stored priority.
10. Prove the constructor equations for actual `find_min` and `pop_min`:
    empty returns `None`; a node's `find_min` returns its own `(x,y)`;
    its `pop_min` returns that same entry and the actual merge of its children.
    Transport proposition 9 to the returned `find_min`/`pop_min` entry.
    A fabricated lower bound is not a minimum occurrence.
11. `pop_min d q = None` iff `q` is `Empty`; equivalently, `size q = 0`.
    Together with proposition 8 at the constant-true predicate, every successful
    pop satisfies `size q = Suc (size q')`.

The success hypotheses have the exact expanded result type
`Option (Pair (Pair k v) Q)`. They refer to the actual returned entry AND
remainder. They are not an assumption that the desired count equation holds.
No `Eq v`, `DecEq v`, or payload-order premise is permitted.

## Induction and reusable lemmas

Prove the worker facts first, then derive the public wrapper statements:

1. Structural count and `all_above` equations on the actual carrier.
2. `make_node_count`: its count is the root contribution plus both child counts,
   independent of the cached-rank comparison. The swapped branch needs addition
   commutativity, not a premise that the children have equal counts.
3. `make_node_valid`: valid children, a root bound for each, and canonical Nat
   order imply validity after either balancing branch. Prove the corresponding
   preservation of `all_above b` for any external bound `b`.
4. `meld_count` and preservation of `all_above b` by meld. Recurse on the SAME
   two-tree descent as the worker: `(first_right, second)` or
   `(first, second_right)`. Each edge strictly decreases one structural operand
   and leaves the other unchanged. No runtime size computation or fuel is needed.
5. `root_bound b q` plus `Valid q` implies `all_above b q`: structural induction
   and comparator transitivity. Use this to provide the strengthened lower-bound
   premises needed by `meld_valid`. On a false priority comparison, obtain the
   reverse true comparison from `d.total`, not by assuming negation reverses any
   arbitrary Boolean relation.
6. Combine these facts for `meld_valid`, then `empty`, singleton, insert and pop.
   Use reflexivity when including the root itself in global minimality.

The priority proof uses reflexivity, transitivity and totality from `d`.
It does not need to compare payloads, identify equal dictionaries, or impose
stability among ties. Priority antisymmetry is present in `Ord` but need not be
used by these proofs. Canonical Nat antisymmetry is a separate metadata lemma.

Reuse `Core.Logic.Transport.cong`, `sym`, and `trans`, and the existing
`bool_and` introduction/projection/algebra proofs. Do not redevelop arithmetic
or Boolean algebra locally. The following shared proof-only spillover is part
of the design:

- Make the EXISTING `add::zero_l`, `add::suc_l`, `add::assoc`, and `add::comm`
  proofs public in `Data.Numeric.Nat.Arithmetic`; keep their bodies unchanged.
- Add the ordinary derived `bool_cases` theorem shown below alongside the
  existing Boolean proof tools in `Core.Classes.LawfulClasses`.
- Expose the EXISTING `bool_or::left_false_elim` proof there, without changing
  its body. A public `leq_nat::total` proof can wrap the existing private
  `total_leq_nat` witness using `bool_or::eq_true_of_or`; do not synthesize an
  unrelated Nat dictionary or duplicate its structural totality proof.

These changes expose/reuse proofs, not new computational queue operations.
Foundation owns assigning this spillover and its affected-domain verification.

## Repeated pop: finite observations, not a new queue engine

Use the private, structurally Nat-recursive `observe_pops d n q` in the checked
appendix. It returns `(entries, remainder)` after at most `n` ACTUAL calls to
`pop_min`, stopping on `None`. At zero it returns `([],q)`. It is an observation
indexed by arbitrary finite length, not an operational fuel parameter or a
replacement implementation of extraction. None of the five public operations
may depend on it, `size`, a validity checker, or any new proof observer.

Define `Nondecreasing entries` by a reflected Boolean list predicate: each
entry's priority is `leq` every later entry's priority, and recurse on the tail.
Reuse the actual exported `Derived.length` for length; do not create another
length implementation. Let `LC_p` count predicate matches in the entry list.
Prove, for EVERY `n`, where `(es,r) = observe_pops d n q`:

```
Valid q -> Valid r
Valid q -> Nondecreasing es
C_p q = LC_p es + C_p r                 for every p
length es = min n (size q)             canonical Nat min
```

Also prove `size q = 0` iff `q` is `Empty`. Specialization at `n = size q`
therefore proves that the actual repeated-pop observation empties the queue,
returns exactly `size q` entries, conserves every `count_by p`, and is
nondecreasing under the SAME order. This is the total-drain theorem, not merely
"if a completed trace exists, then it is sorted."

The proof induction is on `n`, strengthened by validity, remaining counts, and
preservation of any external `all_above` bound through the observed prefix.
At a successful step, use the one-pop count equation and minimum theorem; at
`None`, use the proved empty characterization. To prove the length equation,
use the successful-pop size equation and canonical Nat arithmetic. This avoids
needing to assert that `pop_min` returns a syntactic subtree, which it does not.

The observer and full-drain specialization are PRIVATE proof observations.
They authorize neither a new public drain API nor a measured/fueled meld.

## Admission evidence and honest residuals

Architect independently ran one focused `ken-elaborator` test at the stated cut.
The same provider admits arbitrary-predicate `count_by`, the GENERAL
`make_node_count` proof in the appendix, and the exact `observe_pops` definition.
The final shared-helper placement also passes, with zero additional trust from
the queue probe over its loaded dependency closure. This is kernel admission,
not native execution and not admission of the yet-unwritten full theorem suite.

Two concrete findings determine the technique:

- Existing `add::comm` is private. Its selective consumer failed with exact
  `UnboundName Data.Numeric.Nat.Arithmetic.add::comm`. Making that existing
  proof public admits the use; do not replace it with a local commutativity proof.
- A direct match on the computed rank comparison followed by `Refl` failed
  conversion in the false branch. The explicit Boolean motive below checks.
  It describes the same actual balancing expression, and the final equality's
  left side still names actual `make_node`; it is not a theorem about a clone.

With the earlier local placement of the same Boolean helper, replacing the
actual balancing worker's false branch by a node containing `right` twice
caused `KernelRejected(TypeMismatch)`, while leaving the law and its explicit
motive untouched. Restoration passed. The later shared-helper placement was
separately rechecked green. Evidence is preserved in the Architect's
`local/pq-laws-design/` directory; the design itself is the durable handoff.

The complete meld, validity and drain proofs remain authoring work. If their
actual dependent motives, structural recursion, transport, or resource behavior
hit a new expressibility wall, return the exact smallest source and diagnostic.
Do not add an axiom, weaken the quantifiers, substitute finite tests, silently
change the queue, or treat this strategy as evidence that the missing proofs
already passed. A checker defect belongs at its layer and requires a new ruling.

## Acceptance and semantic review

Closure requires every general proposition above to be a real checked definition
in the provider closure. Review the exact theorem types and bodies, including
their references to actual operations. A copied specification implementation,
a constant-true validity predicate, or stronger unexplained premises is not
closure. Include checked nontrivial validity witnesses and separate malformed
cache, balance and heap-order refutations so that validity is not vacuous.

During verification, hold theorem statements/proofs fixed and make representative
compile-preserving defects in the actual worker: lose or duplicate a child,
miscompute cached rank, select the larger priority, or return the wrong remainder.
The relevant GENERAL proof must stop checking. These controls audit the proof's
binding; finite tests do not discharge the law. Retain the existing independent
public privacy, same-order typing, persistence and structural-cost evidence.
No source-text assertion is a substitute for checking the emitted proof terms.

Reconcile the package, seed and lifecycle claims so proof completion is not
reported before the complete suite checks. Workspace/locked/conformance gates
run in CI, never as local workspace tests. Complexity remains explicitly separate.

## Checked technique appendix

The snippets below are temporary admission probes, not a production patch.
The shared Boolean theorem is ordinary elimination, with no new kernel former:

```ken
pub theorem bool_cases (b : Bool) (motive : Bool → Prop)
      (yes : motive True) (no : motive False) : motive b =
  match b { True ↦ yes; False ↦ no }
```

The following count/balancing probe was appended inside the actual queue provider.
It additionally requires the existing `add::comm` proof to be public.

```ken
import Data.Numeric.Nat.Arithmetic (add)
import Core.Logic.Transport (cong)
import Core.Classes.LawfulClasses (bool_cases)

fn contribution (b : Bool) : Nat = match b { True ↦ Suc Zero; False ↦ Zero }

fn count_by (k : Type) (v : Type) (leq : k → k → Bool)
      (p : k → v → Bool) (q : PriorityQueue k v leq) : Nat =
  match q {
    Empty ↦ Zero;
    Node cached priority payload left right ↦
      add (contribution (p priority payload))
        (add (count_by k v leq p left) (count_by k v leq p right))
  }

theorem make_node_count (k : Type) (v : Type) (leq : k → k → Bool)
      (p : k → v → Bool) (priority : k) (payload : v)
      (left : PriorityQueue k v leq) (right : PriorityQueue k v leq)
    : Equal Nat
      (count_by k v leq p (make_node k v leq priority payload left right))
      (add (contribution (p priority payload))
        (add (count_by k v leq p left) (count_by k v leq p right))) =
  bool_cases (leq_nat (rank k v leq left) (rank k v leq right))
    (λb. Equal Nat
      (count_by k v leq p
        (match b {
          True ↦ Node k v leq (Suc (rank k v leq left)) priority payload right left;
          False ↦ Node k v leq (Suc (rank k v leq right)) priority payload left right
        }))
      (add (contribution (p priority payload))
        (add (count_by k v leq p left) (count_by k v leq p right))))
    (cong Nat Nat
      (add (count_by k v leq p right) (count_by k v leq p left))
      (add (count_by k v leq p left) (count_by k v leq p right))
      (λn. add (contribution (p priority payload)) n)
      ((proof comm for add) (count_by k v leq p right) (count_by k v leq p left)))
    Refl
```

The exact admitted private observation follows. Its Nat index is not added to
any queue operation.

```ken
fn observe_pops (k : Type) (v : Type) (d : Ord k) (n : Nat)
      (q : PriorityQueue k v (ord_leq_at k d))
    : Pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) =
  match n {
    Zero ↦ mk_pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d))
      (Nil (Pair k v)) q;
    Suc remaining ↦
      match pop_min k v d q {
        None ↦ mk_pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d))
          (Nil (Pair k v)) q;
        Some step ↦
          let entry = pair_fst (Pair k v) (PriorityQueue k v (ord_leq_at k d)) step;
              next = pair_snd (Pair k v) (PriorityQueue k v (ord_leq_at k d)) step;
              observed = observe_pops k v d remaining next
          in mk_pair (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d))
            (Cons (Pair k v) entry
              (pair_fst (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) observed))
            (pair_snd (List (Pair k v)) (PriorityQueue k v (ord_leq_at k d)) observed)
      }
  }
```
