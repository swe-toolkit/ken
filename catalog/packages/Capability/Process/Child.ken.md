# Capability.Process.Child

The checked child-process plan, selected pipe operations and wait result are
available through an explicit import.

## Contents

[Motivation](#motivation), [Definition](#definition), [Using it](#using-it),
[Laws & proofs](#laws--proofs), [Design notes](#design-notes),
[References](#references), [Trust & derivation](#trust--derivation).

## Motivation

A child plan is a pure value until `withChild` acquires a process resource.
The selected public builders and pipe operations share one package home.
`ChildExit` remains an always-present scoped checked family, not an export
from this module.

## Definition

```ken
export ChildStdio,
  ChildPipe,
  ChildNull,
  ChildInherit,
  spawn_plan,
  with_environment,
  with_stdin,
  with_stdout,
  withChild,
  wait_child,
  take_stdout,
  read_pipe
```

## Using it

Import `spawn_plan` to build a plan and `ChildStdio` with its public cases to
select a stream. `withChild` acquires the child in a resource bracket;
`take_stdout` obtains its readable pipe, and `read_pipe` transfers from that
pipe. The program still needs the Process and full FS grants required by the
[checked process contract](../../../../spec/30-surface/38-ffi-io.md).

## Laws & proofs

This package adds no declarations or proofs. The exported plan builders are
checked definitions; runtime process, pipe and lifetime behavior is not a
kernel proof introduced by this export.

## Design notes

`ChildStdio` is exported with all three public cases. The private positional
plan constructor stays unavailable. `ChildExit` is already a scoped floor
family, so waiting does not give it a second package identity.

## References

[Ken's child-process contract](../../../../spec/30-surface/38-ffi-io.md)
sets the plan, capability and pipe-operation behavior.

## Trust & derivation

The exports reuse exact checked pre-source identities and add no postulate,
primitive or trusted-base entry. Executing a child retains the existing
runtime authority boundary and its explicit Process grant.
