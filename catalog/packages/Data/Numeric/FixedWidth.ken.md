# Checked fixed-width integer conversions

`Data.Numeric.FixedWidth` exposes safe narrowing from `Int` to bounded machine
integers and checked or saturating arithmetic for eight fixed widths. Each
fixed-width conversion returns `None` outside its range. The three ABI-scalar
conversions instead return `Err RangeError`, recording the target name, input
and manifested bounds. Neither shape silently narrows a value.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

An arbitrary-precision `Int` need not fit a particular machine-width integer.
A caller selecting a fixed-width representation must handle the out-of-range
case explicitly. The eight fixed-width conversions return `Option`; checked
arithmetic returns `Option` on overflow and saturating arithmetic clamps to
bounds. `USize`, `ISize`, and `CInt` are target-manifest scalars, so their
conversions use `Result RangeError` to report which manifest bound was missed.

## Definition

```ken
export intToInt8,
  intToInt16,
  intToInt32,
  intToInt64,
  intToUInt8,
  intToUInt16,
  intToUInt32,
  intToUInt64,
  checkedAddInt8,
  checkedSubInt8,
  checkedMulInt8,
  saturatingAddInt8,
  saturatingSubInt8,
  saturatingMulInt8,
  checkedAddInt16,
  checkedSubInt16,
  checkedMulInt16,
  saturatingAddInt16,
  saturatingSubInt16,
  saturatingMulInt16,
  checkedAddInt32,
  checkedSubInt32,
  checkedMulInt32,
  saturatingAddInt32,
  saturatingSubInt32,
  saturatingMulInt32,
  checkedAddInt64,
  checkedSubInt64,
  checkedMulInt64,
  saturatingAddInt64,
  saturatingSubInt64,
  saturatingMulInt64,
  checkedAddUInt8,
  checkedSubUInt8,
  checkedMulUInt8,
  saturatingAddUInt8,
  saturatingSubUInt8,
  saturatingMulUInt8,
  checkedAddUInt16,
  checkedSubUInt16,
  checkedMulUInt16,
  saturatingAddUInt16,
  saturatingSubUInt16,
  saturatingMulUInt16,
  checkedAddUInt32,
  checkedSubUInt32,
  checkedMulUInt32,
  saturatingAddUInt32,
  saturatingSubUInt32,
  saturatingMulUInt32,
  checkedAddUInt64,
  checkedSubUInt64,
  checkedMulUInt64,
  saturatingAddUInt64,
  saturatingSubUInt64,
  saturatingMulUInt64,
  RangeError,
  MkRangeError,
  intToUSize,
  intToISize,
  intToCInt
```

## Using it

An isolated client imports the desired conversion before using it. The
result must be handled as an `Option`; `Some` contains the narrowed value,
while `None` reports a value outside the selected range.

```ken ignore
import Data.Numeric.FixedWidth (intToUInt8)

fn checked_byte (n : Int) : Option UInt8 = intToUInt8 n
```

## Laws & proofs

This module states no new law or proof. The eight exported functions are
existing checked definitions over fixed-width integer types and their
conversion primitives; their range checks and `Option` results are part of
those definitions, not assumptions added by this package.

## Design notes

The eight fixed-width conversions and their 48 arithmetic operations share
one home with the three ABI-scalar conversions and their `RangeError` payload
and constructor. Checked and saturating operations have distinct result shapes:
overflow is `None` for checked arithmetic and a bound for saturation. The raw
`int_to_*_raw` narrowings remain private; callers use the safe operations.

## References

- [Integer (computer science)](https://en.wikipedia.org/wiki/Integer_(computer_science))
  describes finite-range signed and unsigned integer representations. Ken's
  conversion contract is specified in `spec/10-kernel/18a-primitive-registry.md`
  §5.7 and its surface availability in `spec/30-surface/38-ffi-io.md` §1.12.

## Trust & derivation

The exports refer to existing checked prelude identities, not new
implementations. This package introduces no postulate, primitive, axiom, or
trusted-base entry. Integer primitives and the ABI scalar conversion and
retract primitives remain inherited trust, not proofs of their implementation.
