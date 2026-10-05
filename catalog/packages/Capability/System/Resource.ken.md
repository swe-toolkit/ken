# System.Resource

`System.Resource` is the checked bracket over the runtime's generation-checked
filesystem resource table. Resource handles are ordinary copyable Ken values:
Ken does not make them affine. Liveness is runtime-enforced and Ward-checked.
An escaped copy is legal, but after its bracket settles every later use returns
`Closed`; insufficient rights return `RightNotHeld`. A live token whose acquiring
authority lineage has been revoked instead returns the canonical `Revoked`
identity through the resource error's host-I/O arm before backend access;
revocation never prevents settlement.
The checked acquisition equations `resource_with_acquire`,
`buffer_with_acquire`, `mapping_with_anonymous_acquire`, and
`mapping_with_file_acquire` expose the effect-tree ordering before the delayed
body. The bracket settles on normal return, returned error, and a controlled
runtime trap. Trap-primary/cleanup-secondary ordering is currently exercised
by a private caller-controlled runtime fixture; Ken has no checked-source
controlled-trap producer yet, so public checked-Ken reachability of that face
is deferred. The guarantee excludes external process destruction, abort, fatal
signal, and machine failure.

`withResource` is the sole public filesystem acquisition route. The checked
`resource_after_open_error` and `resource_after_open_success` equations show
that an acquisition error skips the body, whereas success binds the body to
`release_if_live`. The Buffer and Mapping brackets carry the same ordering
through `buffer_after_allocate_error`, `buffer_after_allocate_success`,
`mapping_after_allocate_error`, and `mapping_after_allocate_success`.
`resource_release_settles` identifies the release effect and its settlement
continuation. The four generic settlement equations
`resource_settle_body_ok_clean`, `resource_settle_body_ok_release_error`,
`resource_settle_body_error_clean`, and
`resource_settle_body_error_release_error` distinguish body success from body
failure, and an already-closed release from another release error. `release` is
deliberately non-idempotent; an early release invalidates every copy, while
the bracket's private finalizer treats the resulting `Closed` as already
settled rather than closing the OS resource twice.

```ken
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
