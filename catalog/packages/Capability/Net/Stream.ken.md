# Capability.Net.Stream

This checked stream-socket surface uses typed IPv4, IPv6 and absolute Unix
addresses. The existing `Cap a` holds the declared Net class; `Net` is the
operation row. Socket descriptors remain bracket-owned and opaque.

## Definition

```ken
import Data.Vector.Vector (Vec, VNil, VCons)

export SocketAddress,
  socket_unix_path,
  SocketShutdown,
  ShutdownRead,
  ShutdownWrite,
  ShutdownBoth,
  SocketTarget,
  ListeningSocket,
  ConnectedSocket,
  with_socket_listener,
  with_socket_accept,
  with_socket_connection,
  receive_socket,
  send_socket,
  shutdown_socket,
  socket_local_address

pub fn socket_ipv4
      (octets : Vec UInt8 (Suc (Suc (Suc (Suc Zero))))) (port : UInt16)
    : SocketAddress =
  match octets {
    VCons _ a tail_a ↦
      match tail_a {
        VCons _ b tail_b ↦
          match tail_b {
            VCons _ c tail_c ↦
              match tail_c {
                VCons _ d _ ↦ private_socket_ipv4_octets a b c d port
              }
          }
      }
  }

pub fn socket_ipv6
      (octets : Vec
        UInt8
        (Suc
          (Suc
            (Suc
              (Suc
                (Suc
                  (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))))))))))))
      (port : UInt16)
    : SocketAddress =
  match octets {
    VCons _ a ta ↦
      match ta {
        VCons _ b tb ↦
          match tb {
            VCons _ c tc ↦
              match tc {
                VCons _ d td ↦ socket_ipv6_after_four a b c d td port
              }
          }
      }
  }

fn socket_ipv6_after_four
      (a : UInt8)
      (b : UInt8)
      (c : UInt8)
      (d : UInt8)
      (rest : Vec
        UInt8
        (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))))))))
      (port : UInt16)
    : SocketAddress =
  match rest {
    VCons _ e te ↦
      match te {
        VCons _ f tf ↦
          match tf {
            VCons _ g tg ↦
              match tg {
                VCons _ h th ↦ socket_ipv6_after_eight a b c d e f g h th port
              }
          }
      }
  }

fn socket_ipv6_after_eight
      (a : UInt8)
      (b : UInt8)
      (c : UInt8)
      (d : UInt8)
      (e : UInt8)
      (f : UInt8)
      (g : UInt8)
      (h : UInt8)
      (rest : Vec UInt8 (Suc (Suc (Suc (Suc (Suc (Suc (Suc (Suc Zero)))))))))
      (port : UInt16)
    : SocketAddress =
  match rest {
    VCons _ i ti ↦
      match ti {
        VCons _ j tj ↦
          match tj {
            VCons _ k tk ↦
              match tk {
                VCons _ l tl ↦ socket_ipv6_after_twelve a b c d e f g h i j k l tl port
              }
          }
      }
  }

fn socket_ipv6_after_twelve
      (a : UInt8)
      (b : UInt8)
      (c : UInt8)
      (d : UInt8)
      (e : UInt8)
      (f : UInt8)
      (g : UInt8)
      (h : UInt8)
      (i : UInt8)
      (j : UInt8)
      (k : UInt8)
      (l : UInt8)
      (rest : Vec UInt8 (Suc (Suc (Suc (Suc Zero)))))
      (port : UInt16)
    : SocketAddress =
  match rest {
    VCons _ m tm ↦
      match tm {
        VCons _ n tn ↦
          match tn {
            VCons _ o to ↦
              match to {
                VCons _ p _ ↦ private_socket_ipv6_octets a b c d e f g h i j k l m n o p port
              }
          }
      }
  }
```

## Using it

Import `socket_ipv4`, `socket_ipv6`, or `socket_unix_path` to construct a
socket address. A Unix path constructor returns `InvalidInput` for an invalid
absolute path before a syscall. The two acquisition forms, including accept,
settle their resource when the body returns. Use `socket_local_address` on a
listener bound to port zero to obtain the assigned port before connecting.
`receive_socket` uses a bounded buffer window; `send_socket` consumes a
buffer span. `shutdown_socket` communicates an end-of-stream in the selected
direction. Acquiring a buffer with `withBuffer` additionally requires `[FS]`.

## Laws & proofs

The `Vec` index supplies the IPv4 and IPv6 octet counts, and the checked
builders are pure definitions. Socket I/O, authority, close-on-exec and release
are runtime behavior rather than kernel proofs introduced by this package.

## Trust & derivation

This package adds no trusted primitive or raw socket descriptor. Its
constructors and operations reuse checked prelude identities. The host
validates each address again at the syscall boundary and gates the two
acquisitions on the originating Net grant.
