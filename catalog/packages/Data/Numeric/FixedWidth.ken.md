# Checked fixed-width integer conversions

`Data.Numeric.FixedWidth` exposes the safe `Int`-to-fixed-width conversion
family. Each conversion returns `None` outside its target's range rather than
silently narrowing the value.

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
case explicitly. These conversions supply one uniform `Option`-returning
boundary for signed and unsigned 8-, 16-, 32-, and 64-bit integers.

## Definition

```ken
export intToInt8,
  intToInt16,
  intToInt32,
  intToInt64,
  intToUInt8,
  intToUInt16,
  intToUInt32,
  intToUInt64
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

All eight safe conversions share a single home. The unchecked low-level
narrowing primitives are not exported by this module; a caller uses the
`Option`-returning operations instead.

## References

- [Integer (computer science)](https://en.wikipedia.org/wiki/Integer_(computer_science))
  describes finite-range signed and unsigned integer representations. Ken's
  conversion contract is specified in `spec/10-kernel/18a-primitive-registry.md`
  §5.7 and its surface availability in `spec/30-surface/38-ffi-io.md` §1.12.

## Trust & derivation

The exports refer to the already-checked prelude's existing safe conversion
identities, not new implementations. This package introduces no postulate,
primitive, axiom, or trusted-base entry. The integer primitives on which the
checked definitions depend remain inherited trust, not proofs of their own
implementation.
