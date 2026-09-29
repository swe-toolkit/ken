# System.Error

`retry_guidance` is the purpose of `System.Error`: retry advice comes only
from a transient error paired with an idempotent operation. The checked
[converse theorem](#classification-laws) proves the "only from" direction,
and the [revoked theorem](#classification-laws) proves that
`error_transience Revoked = Permanent`. The envelope currently wraps
filesystem errors without changing the canonical `IOError` identities.

## Contents

- [Motivation](#motivation)
- [Definition](#definition)
  - [Data families](#data-families)
  - [Classification laws](#classification-laws)
  - [Additional classifiers](#additional-classifiers)
- [Using it](#using-it)
- [Laws and proofs](#laws-and-proofs)
- [Design notes](#design-notes)
- [References](#references)
- [Trust and derivation](#trust-and-derivation)

## Motivation

A transient failure can still follow a non-idempotent operation. Retrying
solely because the error is transient can duplicate that operation. The
retry verdict therefore consumes both the error's transience and the
operation's idempotence. A revoked capability is permanently unavailable;
`Revoked` remains the same `IOError` constructor used at the host boundary.

## Definition

Only a transient error paired with an idempotent operation yields
`RetryAdvised`. The converse checks every constructor pair, including the
three combinations that must not advise retry. The second law holds the
existing `IOError.Revoked` identity fixed at `Permanent`. The checked
inductive declarations precede their uses in function signatures; the
retry rule and both laws are the entry's first operations.

### Data families

```ken
export Transience, Transient, Permanent

data Transience = Transient | Permanent

export Idempotence, Idempotent, NonIdempotent

data Idempotence = Idempotent | NonIdempotent

export RetryGuidance, RetryAdvised, RetryUnsafeNonIdempotent, DoNotRetryPermanent

data RetryGuidance = RetryAdvised | RetryUnsafeNonIdempotent | DoNotRetryPermanent

export Operation, FilesystemOp

data Operation = FilesystemOp FileOperation

export ResourceRef, FilesystemResource

data ResourceRef = FilesystemResource (Option Bytes)

export SafeContext, NoSafeContext, RedactedSafeContext

data SafeContext = NoSafeContext | RedactedSafeContext

export SystemError, MkSystemError

data SystemError = MkSystemError Operation ResourceRef IOError SafeContext
```

### Classification laws

The rule is defined before its proofs. The converse analyzes both axes
independently: the positive pair introduces the conjunction; each negative
pair eliminates the contradictory advice premise.

```ken
pub fn retry_guidance (transience : Transience) (idempotence : Idempotence) : RetryGuidance =
  match transience {
    Transient ↦
      match idempotence {
        Idempotent ↦ RetryAdvised;
        NonIdempotent ↦ RetryUnsafeNonIdempotent
      };
    Permanent ↦
      match idempotence {
        Idempotent ↦ DoNotRetryPermanent;
        NonIdempotent ↦ DoNotRetryPermanent
      }
  }

pub theorem retry_advised_only_for_transient_idempotent
      (transience : Transience) (idempotence : Idempotence)
    : Equal RetryGuidance (retry_guidance transience idempotence) RetryAdvised
      → And (Equal Transience transience Transient) (Equal Idempotence idempotence Idempotent) =
  match transience {
    Transient ↦
      match idempotence {
        Idempotent ↦
          λevidence.
            and_intro
              (Equal Transience Transient Transient)
              (Equal Idempotence Idempotent Idempotent)
              Proved
              Proved;
        NonIdempotent ↦ λevidence. absurd evidence
      };
    Permanent ↦
      match idempotence {
        Idempotent ↦ λevidence. absurd evidence;
        NonIdempotent ↦ λevidence. absurd evidence
      }
  }

pub theorem error_transience_revoked_permanent
    : Equal Transience (error_transience Revoked) Permanent =
  Proved
```

### Additional classifiers

The remaining definitions preserve every filesystem operation and the
canonical `IOError` identity without making a retry decision themselves.

```ken
pub fn error_transience (identity : IOError) : Transience =
  match identity {
    NotFound ↦ Permanent;
    PermissionDenied ↦ Permanent;
    CapabilityDenied ↦ Permanent;
    BrokenPipe ↦ Permanent;
    Interrupted ↦ Transient;
    AlreadyExists ↦ Permanent;
    InvalidInput ↦ Permanent;
    IsDirectory ↦ Permanent;
    NotDirectory ↦ Permanent;
    NotEmpty ↦ Permanent;
    Unsupported ↦ Permanent;
    Revoked ↦ Permanent;
    Other _ ↦ Permanent
  }

pub fn operation_idempotence (operation : Operation) : Idempotence =
  match operation {
    FilesystemOp file_operation ↦
      match file_operation {
        OpReadFile ↦ Idempotent;
        OpWriteFile ↦ NonIdempotent;
        OpAppendFile ↦ NonIdempotent;
        OpMetadata ↦ Idempotent;
        OpReadDirectory ↦ Idempotent;
        OpCreateDirectory ↦ NonIdempotent;
        OpRemoveFile ↦ NonIdempotent;
        OpRemoveDirectory ↦ NonIdempotent;
        OpRename ↦ NonIdempotent;
        OpChangeMode ↦ Idempotent;
        OpSeek ↦ NonIdempotent;
        OpSetLength ↦ Idempotent;
        OpSync ↦ Idempotent;
        OpGetInheritance ↦ Idempotent;
        OpSetInheritance ↦ Idempotent;
        OpDuplicate ↦ NonIdempotent
      }
  }

pub fn file_error_to_system (error : FileError) : SystemError =
  match error {
    MkFileError operation resource identity ↦
      MkSystemError
        (FilesystemOp operation)
        (FilesystemResource resource)
        identity
        NoSafeContext
  }
```

## Using it

Classify the error and operation separately, then combine them. This example
uses the prelude's `IOError.Revoked` constructor without minting another
revocation identity.

```ken example
const revoked_retry : RetryGuidance =
  retry_guidance (error_transience Revoked) (operation_idempotence (FilesystemOp OpReadFile))

theorem revoked_retry_is_permanent : Equal RetryGuidance revoked_retry DoNotRetryPermanent =
  Proved
```

## Laws and proofs

The four finite-case lemmas pin every retry verdict. The first two also show
that a shared transient error changes its verdict when operation idempotence
changes. The converse in the definition is stronger: no other constructor
pair can produce `RetryAdvised`. The revoked law in the definition proves the
second normative classification property directly.

```ken
pub theorem retry_guidance_transient_idempotent
    : Equal RetryGuidance (retry_guidance Transient Idempotent) RetryAdvised =
  Proved

pub theorem retry_guidance_transient_nonidempotent
    : Equal RetryGuidance (retry_guidance Transient NonIdempotent) RetryUnsafeNonIdempotent =
  Proved

pub theorem retry_guidance_permanent_idempotent
    : Equal RetryGuidance (retry_guidance Permanent Idempotent) DoNotRetryPermanent =
  Proved

pub theorem retry_guidance_permanent_nonidempotent
    : Equal RetryGuidance (retry_guidance Permanent NonIdempotent) DoNotRetryPermanent =
  Proved

pub theorem retry_guidance_idempotence_flip
    : Equal
        (Prod RetryGuidance RetryGuidance)
        (MkProd
          RetryGuidance
          RetryGuidance
          (retry_guidance Transient Idempotent)
          (retry_guidance Transient NonIdempotent))
        (MkProd RetryGuidance RetryGuidance RetryAdvised RetryUnsafeNonIdempotent) =
  Refl
```

## Design notes

`SystemError` preserves an operation, a resource reference, the canonical
`IOError` identity, and a safe context. `file_error_to_system` wraps each
`FileError` field without interpreting or replacing its cause. A safe context
contains no authority-sensitive bytes; the current constructors carry no
payload. Classification is pure checked Ken, not a host retry policy.

## References

- [Ken error-classification contract](../../../spec/30-surface/38-ffi-io.md#18-error-classification-honest-retry-and-the-single-revoked-identity-px9)
  — normative retry and revocation properties.
- [Filesystem error rendering](../Capability/Filesystem/Errors.ken.md) —
  separate presentation policy for the unchanged `FileError` and `IOError`.
- [Idempotence](https://en.wikipedia.org/wiki/Idempotence) — general
  orientation; the actual verdicts are the checked definitions above.

## Trust and derivation

All seven data families, four classifier/bridge functions and seven laws are
ordinary checked declarations. The package adds **zero** entries to
`trusted_base()`. It uses the prelude's existing `FileError`, `FileOperation`,
`IOError`, `Revoked`, `Equal`, `And` and `Proved` identities; it introduces no
postulate, primitive, foreign signature or new error identity. The seven
laws comprise the converse, the revoked classification, four case lemmas,
and the idempotence-flip law. The conformance seed under
`conformance/surface/ffi-io/` describes the negative behavioral cases;
roots-loaded package acceptance tests check the concrete proof identities,
constructor owners and zero-trust delta.
