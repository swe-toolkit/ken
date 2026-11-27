# Area adds across an x split

Format: `../../README.md`.
Spec: `spec/50-stdlib/67-geometry.md` §24.
Architect AC-0: `evt_79khh3r4s38df`.

This seed covers the public `area_split_x` law. Its accepted client is
open in `hx`, with both order premises supplied as hypotheses. The
closed values are controls for the boundary and for the refused twin;
they do not replace the generic law application.

## Accepted open application

The split is at `m = 2`. The left box runs from x-coordinate 1 through
2, the right starts at 3, and `hx` remains open. Both comparisons are
hypotheses in the accepted theorem, which applies `area_split_x`.

```ken
import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue, leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox (Point, MkPoint, MkBox, area, area_split_x)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const six : Nat = Suc (Suc four)

theorem left_column_area : Equal Nat (area (MkBox (MkPoint one Zero) (MkPoint two one))) four = Proved
theorem right_column_area : Equal Nat (area (MkBox (MkPoint three Zero) (MkPoint four one))) four = Proved
theorem whole_area : Equal Nat (area (MkBox (MkPoint one Zero) (MkPoint four one))) (add four four) = Proved
theorem empty_left_column : Equal Nat (area (MkBox (MkPoint three Zero) (MkPoint two one))) Zero = Proved

theorem split_open_in_hx (hx : Nat)
    : CatalogIsTrue (leq_nat one three)
      → CatalogIsTrue (leq_nat two hx)
      → Equal Nat
          (add (area (MkBox (MkPoint one Zero) (MkPoint two one)))
               (area (MkBox (MkPoint three Zero) (MkPoint hx one))))
          (area (MkBox (MkPoint one Zero) (MkPoint hx one))) =
  λhl. λhh. area_split_x one Zero two hx one hl hh

theorem dropped_low_premise_false_at_three :
  Equal Nat
    (add (area (MkBox (MkPoint four Zero) (MkPoint two one)))
         (area (MkBox (MkPoint three Zero) (MkPoint three one))))
    two = Proved
theorem dropped_low_whole_is_zero :
  Equal Nat (area (MkBox (MkPoint four Zero) (MkPoint three one))) Zero = Proved
theorem dropped_low_premise_is_false : Equal Bool (leq_nat four three) False = Proved
```

### `stdlib/geometry/area-split-x-open-in-hx`

- spec: `spec/50-stdlib/67-geometry.md` §24.
- promise class: durable invariant.
- given: an open `hx`, the left area through coordinate 2, the right
  area beginning at 3, and both order premises as hypotheses.
- expect: applying `area_split_x` yields the whole box's area.
- measured: Architect AC-0's scratch D0 checked this exact open client
  with the law application.
- claimed: under both stated comparisons, the adjacent x spans add to
  the whole span, with the shared y span unchanged.
- gap: this client does not establish the generic law body.
- controls: closed checks give each partial area as four, the whole
  area as `add four four`, and the empty left column as zero.
- why: `hx` stays open and the accepted proof explicitly applies the
  public law rather than closing by computation.

## The open client does not close by computation

For each trial, start from a fresh copy of the accepted block's imports,
constants and closed controls. Append the same `split_open_in_hx`
theorem type, changing only its body to the indicated compute-only
closing. Keep `hx` open in both trials.

```ken
theorem split_open_in_hx (hx : Nat)
    : CatalogIsTrue (leq_nat one three)
      → CatalogIsTrue (leq_nat two hx)
      → Equal Nat
          (add (area (MkBox (MkPoint one Zero) (MkPoint two one)))
               (area (MkBox (MkPoint three Zero) (MkPoint hx one))))
          (area (MkBox (MkPoint one Zero) (MkPoint hx one))) =
  λhl. λhh. Proved
```

### `stdlib/geometry/area-split-x-open-hx-proved-refusal`

- spec: `spec/50-stdlib/67-geometry.md` §24.
- promise class: durable invariant.
- given: the fresh accepted prefix and the same theorem type, closed
  with `λhl. λhh. Proved`.
- expect: refusal with `KernelRejected(TypeMismatch)` at the theorem.
- measured: Architect AC-0 observed this refusal in scratch D0.
- claimed: this open client is proved by applying the public law;
  computation-only closings do not discharge the goal.
- gap: this does not characterize all proofs of the open equality.
- why: only the proof body changes; the accepted input and open `hx`
  stay fixed.

```ken
theorem split_open_in_hx (hx : Nat)
    : CatalogIsTrue (leq_nat one three)
      → CatalogIsTrue (leq_nat two hx)
      → Equal Nat
          (add (area (MkBox (MkPoint one Zero) (MkPoint two one)))
               (area (MkBox (MkPoint three Zero) (MkPoint hx one))))
          (area (MkBox (MkPoint one Zero) (MkPoint hx one))) =
  λhl. λhh. Refl
```

### `stdlib/geometry/area-split-x-open-hx-refl-refusal`

- spec: `spec/50-stdlib/67-geometry.md` §24.
- promise class: durable invariant.
- given: a fresh accepted prefix and the same theorem type, closed
  with `λhl. λhh. Refl`.
- expect: elaborator `TypeMismatch` because the goal sides are not
  convertible.
- measured: Architect AC-0 observed this refusal in scratch D0.
- claimed: the open area equality is not reflexive by conversion.
- gap: no finer source span or diagnostic wording is pinned.
- why: with `hx` open, the area and addition terms remain neutral;
  changing only the terminal cannot make the two sides convertible.

## Dropping the low-bound premise is false

Start from a fresh copy of the accepted block above and append only the
twin below. It drops `leq_nat lx (Suc m)` while retaining the premise
`leq_nat m hx`, and claims the same area-sum equation.

```ken
theorem refused_split_without_low_premise (hx : Nat)
    : CatalogIsTrue (leq_nat two hx)
      → Equal Nat
          (add (area (MkBox (MkPoint four Zero) (MkPoint two one)))
               (area (MkBox (MkPoint three Zero) (MkPoint hx one))))
          (area (MkBox (MkPoint four Zero) (MkPoint hx one))) =
  λhh. area_split_x four Zero two hx one Proved hh
```

### `stdlib/geometry/area-split-x-low-premise-dropped-refusal`

- spec: `spec/50-stdlib/67-geometry.md` §24.
- promise class: durable invariant.
- given: the fresh accepted prefix followed by the same-sum claim with
  only `leq_nat four three` omitted.
- expect: `KernelRejected(TypeMismatch)` at declaration granularity.
- measured: Architect AC-0 observed this exact refusal in scratch D0;
  its span covers the declaration, not a finer term.
- claimed: `leq_nat lx (Suc m)` is required for this application of
  `area_split_x`.
- gap: the refusal alone would not show the claim is false rather than
  merely missing a proof.
- controls: at `hx = 3`, the retained order premise holds; the closed
  controls show the sum is two, the whole area is zero, and the dropped
  low premise is false.
- why: the accepted prefix supplies a concrete counterexample to the
  same-sum claim when only the low-bound premise is removed.

## Evidence boundary

Architect AC-0 `evt_79khh3r4s38df` reports that its scratch D0 test ran
the accepted block and refused twin exactly as written. It also measured
the compute-only closings in fresh copies of the same prefix. The test
`scripts/ken-cargo test -p ken-elaborator --test arch_d0_geo_split`
returned rc 0 with 3/3. The scratch trusted `GlobalId` identity set
was unchanged (114 → 114). These are Architect D0 findings; CV ran no
tests. This authored seed has not been executed by the official
conformance runner. No assembled AC-T, CI, publication, landing, or
measured landed-trust result is claimed.
