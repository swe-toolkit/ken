# Capability.System.Mapping

The checked mapping descriptors and byte transfers are available through one
explicit-import package. The resource bracket remains in
[System.Resource](Resource.ken.md).

## Contents

[Motivation](#motivation), [Definition](#definition), [Using it](#using-it),
[Laws & proofs](#laws--proofs), [Design notes](#design-notes),
[References](#references), [Trust & derivation](#trust--derivation).

## Motivation

A mapping has a bounded extent and a lifetime managed by `withMapping`.
This package names its checked source, window, opaque handle and transfer
operations without exposing the handle's private constructor.

## Definition

```ken
export MappingHandle,
  MappingWindow,
  MkMappingWindow,
  MappingSource,
  Anonymous,
  FileBacked,
  mapBytes,
  mapWrite
```

## Using it

Import `MappingSource` and either `Anonymous` or `FileBacked` to describe a
mapping source. Import `MappingWindow` and `MkMappingWindow` for a bounded read
request. `mapBytes` copies out bytes, while `mapWrite` accepts an offset and
payload. Acquire and settle the handle through `withMapping` from
[System.Resource](Resource.ken.md).

## Laws & proofs

This package adds no declarations or proofs. It exposes the existing checked
mapping identities; runtime liveness and bounds enforcement are not kernel
proofs introduced by an export.

## Design notes

`MappingHandle` has no public constructor. Importing its type cannot grant
access to `PrivateMappingHandle` or to its resource and extent fields. The
window and source constructors are public and are exported with their complete
families.

## References

[Ken's mapped-region contract](../../../../spec/30-surface/38-ffi-io.md)
sets the lifetime, protection, window and transfer behavior.

## Trust & derivation

Every export refers to an existing checked pre-source identity. This module
adds no postulate, primitive or trusted-base entry. Mapping effects retain the
runtime and capability assumptions of the underlying operations.
