# System.Resource

`System.Resource` is the checked bracket over the runtime's generation-checked
filesystem resource table. Resource handles are ordinary copyable Ken values:
Ken does not make them affine. Liveness is runtime-enforced and Ward-checked.
An escaped copy is legal, but after its bracket settles every later use
returns `ResourceError.Closed`; insufficient rights return `RightNotHeld`. A
live token whose acquiring authority lineage has been revoked instead
returns the canonical `Revoked` identity through the resource error's
host-I/O arm before backend access; revocation never prevents settlement.
The bracket settles on normal return, returned error, and a controlled
runtime trap. Trap-primary/cleanup-secondary ordering is currently exercised
by a private caller-controlled runtime fixture; Ken has no checked-source
controlled-trap producer yet, so public checked-Ken reachability of that face
is deferred. The guarantee excludes external process destruction, abort,
fatal signal, and machine failure.

`withResource` is the sole public filesystem acquisition route. The checked
prelude acquisition, after-open, release, and settlement equations describe
the bracket's internal steps, including the Buffer and Mapping brackets.
The resource-law and write-all tests verify these as checked implementation
equations. Their statements name private helpers such as `release_if_live`
and `resource_settle_result_for`, so a Strict client cannot state them as
package laws. An acquisition error skips the body; success runs the body
and settles the resource. A body error and a release error remain distinct.
`release` is deliberately non-idempotent; an early release invalidates every
copy, while the bracket's private finalizer treats the resulting
`ResourceError.Closed` as already settled rather than closing the OS resource
twice.

```ken
export resourceMetadata,
  ResourceBodyResult,
  ResourceBodyOk,
  ResourceBodyErr,
  ResourceBracketResult,
  ResourceBracketOk,
  ResourceBracketBodyError,
  ResourceBracketReleaseError,
  ResourceBracketBodyAndReleaseError,
  withResource,
  withBuffer,
  withMapping,
  release

fn resource_body_success (e : Type) (a : Type) (value : a) : ResourceBodyResult e a =
  ResourceBodyOk e a value

fn resource_body_failure (e : Type) (a : Type) (error : e) : ResourceBodyResult e a =
  ResourceBodyErr e a error

fn resource_bracket_succeeded
      (e : Type) (a : Type) (outcome : ResourceBracketResult e a)
    : Bool =
  match outcome {
    ResourceBracketOk value ↦ True;
    ResourceBracketBodyError error ↦ False;
    ResourceBracketReleaseError error ↦ False;
    ResourceBracketBodyAndReleaseError body_error release_error ↦ False
  }
```

## Using it

Use `withResource` to acquire and settle a filesystem resource. A copy kept
past the bracket's settlement cannot revive the handle: a later use reports
`ResourceError.Closed`. That qualified constructor spelling distinguishes the
error from an unqualified name, which is not available in a Strict client.
