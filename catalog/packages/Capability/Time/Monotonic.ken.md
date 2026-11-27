# Capability.Time.Monotonic

Read the process-local monotonic clock and wait for a deadline on that clock.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

Use monotonic time for elapsed durations and deadlines. Unlike civil time,
this clock does not run backwards when the host adjusts its wall clock.

## Definition

```ken
export monotonic_now, sleep_until
```

## Using it

`monotonic_now` returns a reading in nanoseconds since a process-wide
baseline. Readings are never negative and never decrease within a process.
Derive a deadline by reading the clock and adding a duration in nanoseconds.
Pass the deadline to `sleep_until`; it returns once the same clock reads at or
past the deadline. If the deadline has already been reached, it returns without
suspending. A deadline's nanosecond value must be in `0 <= d < 2^64`; an
out-of-domain deadline is refused.

## Laws & proofs

This package declares no laws. Ordering across readings is a host property,
not a theorem about Ken terms.

## Design notes

The epoch is unspecified. Neither a reading nor a deadline is meaningful
outside the process that produced it. `sleep_until` is uncancellable;
cancellation belongs to PX12.

## References

- [Ken FFI and I/O specification](../../../../spec/30-surface/38-ffi-io.md)
  describes the host-operation boundary.
- [Wall clock](WallClock.ken.md) is the alternative for civil timestamps.

## Trust & derivation

The exports keep the existing checked prelude identities. The package adds no
trusted declarations; the readings and deadline behavior depend on the host.
