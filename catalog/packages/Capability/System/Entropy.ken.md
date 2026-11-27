# Capability.System.Entropy

Request a bounded byte sequence from the host's kernel entropy source.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
- [Using it](#using-it)
- [Laws & proofs](#laws--proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust & derivation](#trust--derivation)

## Motivation

`EntropyIO` names the checked action type without exposing the operation or
response representation. `random_bytes` asks the host for a byte sequence of
an explicit length.

## Definition

```ken
export EntropyIO, random_bytes
```

## Using it

For a count from 0 through 1024 inclusive, a successful `random_bytes` reply
contains exactly the requested number of bytes. A negative count or a count
above 1024 gives `Err InvalidInput`. If the source is unavailable or cannot
supply the full request, the reply is `Err Unsupported` with no bytes.
Use `host_entropy` from `Capability.Process.Exit` to lift the action into
`HostIO`.

## Laws & proofs

This package declares no laws and makes no security claim beyond the bytes
observed in a successful reply.

## Design notes

The host uses only the kernel's entropy source. It does not substitute a
fallback source or return a partial buffer on failure.

## References

- [Ken FFI and I/O specification](../../../../spec/30-surface/38-ffi-io.md)
  describes the host-operation boundary.
- [Process exit and HostIO helpers](../Process/Exit.ken.md) provides the
  `host_entropy` lift.

## Trust & derivation

The exports keep the existing checked prelude identities. The package adds no
trusted declarations; the byte source and reply behavior depend on the host.
