# System.IO

`System.IO` exposes explicitly positioned, single-transfer buffer I/O.
`readAt` and `writeAt` never maintain a hidden file cursor. `readAt` returns
`ReadEof` or a positive `ReadSome`; `writeAt` returns a positive `Wrote`, while
a zero host write is the distinct `NoProgress` error. `spanBytes` (`freeze`)
copies only the validated current span.

`writeAll` recurses over its initial `BufferSpan` budget. Its checked prelude
laws name the actual loop and its private response step, then establish entry,
stop, request, first-error, done, continue, and offset advance. The private
step cannot mint a public `BufferSpan`. This package proves the arithmetic
facts needed to use those laws for each `Wrote` response. Errors need no count
premise; every successful response supplies `write_all_count_fits` for the
span named by that request.

| Clause | Checked loop laws | Response arithmetic |
|---|---|---|
| Termination at most initial budget | `write_all_entry`, `_stop`, `_request`, `_continue` | `write_all_strict_decrease` |
| Exact prefix | `_continue`, `write_all_advance_start`, `write_all_exact_prefix_prop::exact_prefix` | `write_all_count_fits` |
| Success completeness | `_stop`, `_done` | `write_all_fuel_sufficient`, `write_all_complete_after_wrote`, `write_all_zero_budget` |
| First error unchanged | `_request`, `_first_error_step` | No response premise |
| All successful writes finish | `_done`, `_continue` | `write_all_fuel_sufficient`, `write_all_complete_after_wrote` |

The old observer-only helpers below remain checked utilities, but their
claims do not establish the loop's behavior. No law adds an axiom or trusted
entry. Exactly-once settlement and liveness remain delegated runtime
properties.

```ken
import Core.Logic.Transport (cong, sym, trans)

import Data.Numeric.Nat.Arithmetic (add)

pub fn write_all_count_fits (span : BufferSpan) (count : TransferCount) : Prop =
  Equal
    Nat
    (add (transfer_count_nat count) (transfer_count_remaining count))
    (buffer_span_budget span)

pub theorem write_all_strict_decrease
      (span : BufferSpan) (count : TransferCount) (fits : write_all_count_fits span count)
    : Equal Nat
        (add (Suc (transfer_count_remaining count)) (transfer_count_predecessor count))
        (buffer_span_budget span) =
  trans
    Nat
    (add (Suc (transfer_count_remaining count)) (transfer_count_predecessor count))
    (add (transfer_count_nat count) (transfer_count_remaining count))
    (buffer_span_budget span)
    (trans
      Nat
      (add (Suc (transfer_count_remaining count)) (transfer_count_predecessor count))
      (add (Suc (transfer_count_predecessor count)) (transfer_count_remaining count))
      (add (transfer_count_nat count) (transfer_count_remaining count))
      (write_all_successor_sum
        (transfer_count_predecessor count)
        (transfer_count_remaining count))
      (cong
        Nat
        Nat
        (Suc (transfer_count_predecessor count))
        (transfer_count_nat count)
        (λn. add n (transfer_count_remaining count))
        (sym
          Nat
          (transfer_count_nat count)
          (Suc (transfer_count_predecessor count))
          (transfer_count_nat_succ count))))
    fits

pub theorem write_all_fuel_sufficient
      (span : BufferSpan)
      (count : TransferCount)
      (rest : Nat)
      (slack : Nat)
      (fits : write_all_count_fits span count)
      (fuel : Equal Nat (add (buffer_span_budget span) slack) (Suc rest))
    : Equal Nat
        (add (transfer_count_remaining count) (add slack (transfer_count_predecessor count)))
        rest =
  let
    remaining = transfer_count_remaining count;
    predecessor = transfer_count_predecessor count;
    budget = buffer_span_budget span;
    step_budget : Equal Nat (add (Suc remaining) predecessor) budget =
      write_all_strict_decrease span count fits;
    total_fuel : Equal Nat (add (add (Suc remaining) predecessor) slack) (Suc rest) =
      trans
        Nat
        (add (add (Suc remaining) predecessor) slack)
        (add budget slack)
        (Suc rest)
        (cong
          Nat
          Nat
          (add (Suc remaining) predecessor)
          budget
          (λavailable. add available slack)
          step_budget)
        fuel;
    next_fuel : Equal Nat (Suc (add remaining (add slack predecessor))) (Suc rest) =
      trans
        Nat
        (Suc (add remaining (add slack predecessor)))
        (add (add (Suc remaining) predecessor) slack)
        (Suc rest)
        (sym
          Nat
          (add (add (Suc remaining) predecessor) slack)
          (Suc (add remaining (add slack predecessor)))
          (write_all_fuel_shape remaining predecessor slack))
        total_fuel
  in
    write_all_suc_cancel (add remaining (add slack predecessor)) rest next_fuel

pub theorem write_all_complete_after_wrote
      (span : BufferSpan)
      (count : TransferCount)
      (fits : write_all_count_fits span count)
      (done : Equal Nat (transfer_count_remaining count) Zero)
    : Equal Nat (transfer_count_nat count) (buffer_span_budget span) =
  trans
    Nat
    (transfer_count_nat count)
    (add (transfer_count_nat count) (transfer_count_remaining count))
    (buffer_span_budget span)
    (J
      (λremaining _.
        Equal Nat (transfer_count_nat count) (add (transfer_count_nat count) remaining))
      Refl
      (sym Nat (transfer_count_remaining count) Zero done))
    fits

pub theorem write_all_zero_budget
      (budget : Nat)
    : (slack : Nat) → Equal Nat (add budget slack) Zero → Equal Nat budget Zero =
  λslack.
    match slack {
      Zero ↦ λzero. zero;
      Suc more ↦ λzero. absurd zero
    }

theorem write_all_terminates (fuel : Nat) : Equal Nat (write_all_call_bound fuel) fuel =
  proof termination for write_all_call_bound fuel

theorem write_all_preserves_exact_prefix
      (span : BufferSpan) (count : TransferCount)
    : write_all_exact_prefix_prop span count =
  proof exact_prefix for write_all_exact_prefix_prop span count

theorem write_all_success_is_complete : Equal Bool (write_all_complete Zero) True =
  proof success_complete for write_all_complete

theorem write_all_preserves_first_error
      (error : ResourceError)
    : Equal
        (Result ResourceError Unit)
        (write_all_first_error error)
        (Err ResourceError Unit error) =
  proof first_error for write_all_first_error error

theorem write_all_all_success_holds
      (fuel : Nat)
    : Equal Bool (write_all_all_success fuel) True =
  proof all_success for write_all_all_success fuel

theorem write_all_fuel_shape
      (remaining : Nat) (predecessor : Nat) (slack : Nat)
    : Equal Nat
        (add (add (Suc remaining) predecessor) slack)
        (Suc (add remaining (add slack predecessor))) =
  trans
    Nat
    (add (add (Suc remaining) predecessor) slack)
    (add (Suc remaining) (add predecessor slack))
    (Suc (add remaining (add slack predecessor)))
    (sym
      Nat
      (add (Suc remaining) (add predecessor slack))
      (add (add (Suc remaining) predecessor) slack)
      ((proof assoc for add) (Suc remaining) predecessor slack))
    (trans
      Nat
      (add (Suc remaining) (add predecessor slack))
      (Suc (add remaining (add predecessor slack)))
      (Suc (add remaining (add slack predecessor)))
      ((proof suc_l for add) remaining (add predecessor slack))
      (cong
        Nat
        Nat
        (add predecessor slack)
        (add slack predecessor)
        (λother. Suc (add remaining other))
        ((proof comm for add) predecessor slack)))

theorem write_all_successor_sum
      (predecessor : Nat) (remaining : Nat)
    : Equal Nat (add (Suc remaining) predecessor) (add (Suc predecessor) remaining) =
  trans
    Nat
    (add (Suc remaining) predecessor)
    (Suc (add remaining predecessor))
    (add (Suc predecessor) remaining)
    ((proof suc_l for add) remaining predecessor)
    (trans
      Nat
      (Suc (add remaining predecessor))
      (Suc (add predecessor remaining))
      (add (Suc predecessor) remaining)
      (cong
        Nat
        Nat
        (add remaining predecessor)
        (add predecessor remaining)
        Suc
        ((proof comm for add) remaining predecessor))
      (sym
        Nat
        (add (Suc predecessor) remaining)
        (Suc (add predecessor remaining))
        ((proof suc_l for add) predecessor remaining)))

theorem write_all_suc_cancel
      (left : Nat) (right : Nat) (same_successor : Equal Nat (Suc left) (Suc right))
    : Equal Nat left right =
  cong Nat Nat (Suc left) (Suc right) write_all_nat_predecessor same_successor

fn write_all_nat_predecessor (n : Nat) : Nat =
  match n {
    Zero ↦ Zero;
    Suc earlier ↦ earlier
  }

fn write_all_call_bound (fuel : Nat) : Nat =
  match fuel {
    Zero ↦ Zero;
    Suc rest ↦ Suc (write_all_call_bound rest)
  }

proof termination for write_all_call_bound
      (fuel : Nat)
    : Equal Nat (write_all_call_bound fuel) fuel =
  match fuel {
    Zero ↦ Proved;
    Suc rest ↦
      cong
        Nat
        Nat
        (write_all_call_bound rest)
        rest
        Suc
        ((proof termination for write_all_call_bound) rest)
  }

fn write_all_complete (remaining : Nat) : Bool =
  match remaining {
    Zero ↦ True;
    Suc rest ↦ False
  }

proof success_complete for write_all_complete : Equal Bool (write_all_complete Zero) True =
  Proved

fn write_all_first_error (error : ResourceError) : Result ResourceError Unit =
  Err ResourceError Unit error

proof first_error for write_all_first_error
      (error : ResourceError)
    : Equal
        (Result ResourceError Unit)
        (write_all_first_error error)
        (Err ResourceError Unit error) =
  Refl

fn write_all_all_success (fuel : Nat) : Bool =
  match fuel {
    Zero ↦ True;
    Suc rest ↦ write_all_all_success rest
  }

proof all_success for write_all_all_success
      (fuel : Nat)
    : Equal Bool (write_all_all_success fuel) True =
  match fuel {
    Zero ↦ Proved;
    Suc rest ↦ (proof all_success for write_all_all_success) rest
  }
```

Fuel sufficiency requires the response premise. For budget one, fuel one,
slack zero, count predecessor zero, and count remaining two, the premise is
false (`1 + 2 ≠ 1`), the request reaches fuel zero with bytes left, and S2's
conclusion demands `2 = 0`. The closed false equation is rejected by `Proved`;
changing only remaining to zero makes the same terminal check. The generic
true twin below uses the premise-bearing theorem itself.

```ken reject
theorem write_all_reject_premise_free_fuel_at_exhaustion
    : Equal Nat (add (Suc (Suc Zero)) (add Zero Zero)) Zero =
  Proved
```

```ken example
theorem write_all_example_zero_remaining : Equal Nat (add Zero (add Zero Zero)) Zero = Proved

theorem write_all_example_fuel_with_premise
      (span : BufferSpan)
      (count : TransferCount)
      (rest : Nat)
      (slack : Nat)
      (fits : write_all_count_fits span count)
      (fuel : Equal Nat (add (buffer_span_budget span) slack) (Suc rest))
    : Equal Nat
        (add (transfer_count_remaining count) (add slack (transfer_count_predecessor count)))
        rest =
  write_all_fuel_sufficient span count rest slack fits fuel
```
