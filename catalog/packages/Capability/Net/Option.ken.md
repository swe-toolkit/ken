# Capability.Net.Option

This checked surface holds the typed socket option families. A program sets
and reads `NoDelay` or `KeepAlive` on an IPv4 or IPv6 stream, and acquires a
listener that reuses its address with `with_reusable_socket_listener`. There
is no integer `(level, optname)` form. Clients import `SocketAddress` and its
smart constructors from `Capability.Net.Stream`; this package re-exports
nothing.

## Definition

```ken
export StreamOption,
  NoDelay,
  KeepAlive,
  with_reusable_socket_listener,
  set_stream_option,
  stream_option
```

The checked prelude identities this package publishes have these shapes:

```text
data StreamOption = NoDelay | KeepAlive

proc with_reusable_socket_listener (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (address : SocketAddress)
    (body : Resource ResourceKind.SocketListener
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc set_stream_option (a : Auth) (stream : Resource ResourceKind.SocketStream)
    (option : StreamOption) (enabled : Bool)
  : HostIO a (Result ResourceError Unit) visits [Net]
proc stream_option (a : Auth) (stream : Resource ResourceKind.SocketStream)
    (option : StreamOption)
  : HostIO a (Result ResourceError Bool) visits [Net]
```

## Using it

`NoDelay` disables Nagle's algorithm, for interactive request and response
latency. `KeepAlive` enables keepalive probes, for detecting a dead network
peer on an idle connection. Both apply to an IPv4 or IPv6 stream, outbound or
accepted. `set_stream_option` sets the option to `enabled`, and
`stream_option` reads the kernel's current value; it is never a cached copy
of the last set. A fresh stream reads `False` for both.

Either option on a Unix stream is `ResourceHostIO IOError.InvalidInput`
before any syscall. A Unix peer's death is reported without probes, so
keepalive on a Unix stream would claim probing that does not exist. Option
calls on a listener or a datagram socket do not typecheck.

Address reuse is a property of how a listener is bound, so it is an
acquisition rather than a settable option. `with_reusable_socket_listener` is
`with_socket_listener` with address reuse set before the bind; the address
class, revocation, bracket, release, close-on-exec and backlog are the same.
A reusable listener rebinds a port held by a closed connection only when that
connection was accepted from a reusable listener, and it never rebinds a live
listener's port; address in use stays `IOError.Other 98`. A Unix address is
`ResourceHostIO IOError.InvalidInput` after the Net grant is admitted and
before any syscall, so no inode is created. There is no readback: whether a
listener reuses its address is fixed by which acquisition the program called.

## Laws & proofs

None. Option state, its effect on traffic, and address reuse are runtime
behavior rather than kernel proofs introduced by this package.

## Trust & derivation

This package adds no trusted primitive, raw descriptor or option escape. Its
operations reuse checked prelude identities carried on the existing FSOp
under the Net row label. The host maps each option to one fixed kernel option,
refuses an option code above 1 before any syscall, and reads a stream's
family from the record made when it was acquired. The native lane refuses
these operations at build time until a parity node lands.
