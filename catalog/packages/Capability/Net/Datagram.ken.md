# Capability.Net.Datagram

This checked datagram-socket surface binds a socket to a typed IPv4, IPv6 or
absolute Unix address, sends one bounded datagram to a typed peer, and
receives one datagram together with its sender's address. It uses the same
`SocketAddress` identity, Net classes and FS-root rule as
`Capability.Net.Stream`. The existing `Cap a` holds the declared Net class;
`Net` is the operation row. Socket descriptors remain bracket-owned and
opaque.

## Definition

```ken
import Capability.Net.Stream (socket_ipv4, socket_ipv6, socket_unix_path)

export SocketAddress,
  socket_ipv4,
  socket_ipv6,
  socket_unix_path,
  DatagramFit,
  WholeDatagram,
  TruncatedDatagram,
  DatagramReceipt,
  EmptyDatagram,
  ReceivedDatagram,
  with_datagram_socket,
  send_datagram,
  receive_datagram,
  datagram_local_address
```

The checked prelude identities this package publishes have these shapes:

```text
data DatagramFit = WholeDatagram | TruncatedDatagram
data DatagramReceipt = EmptyDatagram (Option SocketAddress)
  | ReceivedDatagram BufferSpan TransferCount DatagramFit (Option SocketAddress)

proc with_datagram_socket (a : Auth) (e : Type) (r : Type)
    (cap : Cap a) (address : SocketAddress)
    (body : Resource ResourceKind.SocketDatagram
      -> HostIO a (ResourceBodyResult e r))
  : HostIO a (Result ResourceError (ResourceBracketResult e r)) visits [Net]
proc send_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
    (peer : SocketAddress) (buffer : BufferHandle) (span : BufferSpan)
  : HostIO a (Result ResourceError Unit) visits [Net]
proc receive_datagram (a : Auth) (socket : Resource ResourceKind.SocketDatagram)
    (buffer : BufferHandle) (window : BufferWindow)
  : HostIO a (Result ResourceError DatagramReceipt) visits [Net]
proc datagram_local_address (a : Auth)
    (socket : Resource ResourceKind.SocketDatagram)
  : HostIO a (Result ResourceError SocketAddress) visits [Net]
```

## Using it

Construct an address with `socket_ipv4`, `socket_ipv6` or `socket_unix_path`,
then bind it with `with_datagram_socket`. Bind is the only acquisition, so
every datagram socket is bound; the bracket settles it when the body returns.
A Unix bind must lie beneath the capability's FS root and also needs FS
`AFull`; a refused bind leaves no inode, and release does not unlink the
pathname. Use `datagram_local_address` on a socket bound to port zero to learn
the assigned port.

`send_datagram` sends the whole effective span as one datagram or fails, so
its result is `Unit`. The peer must be in the socket's Net class and of the
socket's bound family, and the span must hold 1 to 65507 bytes. An empty span
is `InvalidBounds`; a larger one, a family mismatch or an invalid peer is
`InvalidInput`; an out-of-class peer is `CapabilityDenied`. All of these are
refused before any syscall.

`receive_datagram` blocks for one datagram. It delivers the prefix that fits
the effective window as `ReceivedDatagram`, with `TruncatedDatagram` exactly
when the datagram was longer than the window; the kernel discards the excess.
A zero-length datagram is `EmptyDatagram`. A zero effective window is
`InvalidBounds` without a host call. The sender is `Some` address for an IPv4
sender, and for an IPv6 sender with no scope ID; it is `None` for a scoped
IPv6 sender and for an unnamed or abstract Unix sender.

## Laws & proofs

The receipt's fit is a typed field rather than an error, because a truncated
datagram was consumed and its prefix delivered. Socket I/O, authority,
close-on-exec and release are runtime behavior rather than kernel proofs
introduced by this package.

## Trust & derivation

This package adds no trusted primitive or raw socket descriptor. Its
operations reuse checked prelude identities carried on the existing FSOp
under the Net row label. The host validates each address again at the
syscall boundary, gates the bind on the originating Net grant, and admits
every send's peer against the grant the socket retains. The native lane
refuses these operations at build time until a parity node lands.
