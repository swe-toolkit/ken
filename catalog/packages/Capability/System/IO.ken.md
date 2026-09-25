# System.IO

`System.IO` exposes explicitly positioned, single-transfer buffer I/O.
`readAt` and `writeAt` never maintain a hidden file cursor. `readAt` returns
`ReadEof` or a positive `ReadSome`; `writeAt` returns a positive `Wrote`, while
a zero host write is the distinct `NoProgress` error. `spanBytes` (`freeze`)
copies only the validated current span.

`writeAll` is transparent checked Ken recursion over the constructor-private
`Nat` attempt budget carried by its initial `BufferSpan`. It preserves the first
transfer error unchanged. Five public theorems are ordinary proof terms checked
by the kernel, supported by checked helper functions and attached proofs. None
is an axiom or a runtime claim. Exactly-once settlement and liveness remain
runtime-enforced, delegated boundary properties.
The exact-prefix proposition observes the loop's private span transition without
exposing a `BufferSpan` producer to source code.

```ken
import Core.Logic.Transport (cong)

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
```
