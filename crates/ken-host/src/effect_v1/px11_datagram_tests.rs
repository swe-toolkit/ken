//! PX11-DATAGRAM-SOCKETS: the shared dispatcher over the real datagram leaf.
//! Spec 38 §1.13.1.

use super::*;
use crate::{RevocationDomain, SocketAddressV1};

/// Real datagram leaves, with a count of each leaf the dispatcher reached.
#[derive(Default)]
struct CountingDatagramBackend {
    binds: usize,
    sends: usize,
    receives: usize,
}

impl HostEffectBackendV1 for CountingDatagramBackend {
    fn console_write(&mut self, _: ConsoleStreamV1, _: &[u8]) -> Result<(), IoErrorIdentityV1> {
        Err(IoErrorIdentityV1::Unsupported)
    }
    fn console_flush(&mut self, _: ConsoleStreamV1) -> Result<(), IoErrorIdentityV1> {
        Err(IoErrorIdentityV1::Unsupported)
    }
    fn console_is_terminal(&mut self, _: ConsoleStreamV1) -> bool {
        false
    }
    fn fs_read_file(&mut self, _: &CapabilityGrantV1, _: &[u8]) -> Result<Vec<u8>, FileErrorCauseV1> {
        Err(FileErrorCauseV1::Io(IoErrorIdentityV1::Unsupported))
    }
    fn fs_write_file(&mut self, _: &CapabilityGrantV1, _: &[u8], _: CreatePolicyV1, _: &[u8])
        -> Result<(), FileErrorCauseV1> {
        Err(FileErrorCauseV1::Io(IoErrorIdentityV1::Unsupported))
    }
    fn net_datagram_bind(&mut self, root: &crate::FsHandle, address: &SocketAddressV1)
        -> Result<crate::DatagramSocketV1, IoErrorIdentityV1> {
        self.binds += 1;
        crate::net_v1::datagram_bind(root, address)
    }
    fn net_datagram_send_to(&mut self, socket: &crate::DatagramSocketV1, root: &crate::FsHandle,
        peer: &SocketAddressV1, bytes: &[u8]) -> Result<(), IoErrorIdentityV1> {
        self.sends += 1;
        crate::net_v1::datagram_send_to(socket, root, peer, bytes)
    }
    fn net_datagram_receive_from(&mut self, socket: &crate::DatagramSocketV1, bytes: &mut [u8])
        -> Result<crate::DatagramReceivedV1, IoErrorIdentityV1> {
        self.receives += 1;
        crate::net_v1::datagram_receive_from(socket, bytes)
            .map_err(|error| io_error_identity_v1(&error))
    }
}

const LOOPBACK4: SocketAddressV1 = SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 };

struct Datagrams {
    backend: CountingDatagramBackend,
    revocation: RevocationDomain,
    capabilities: CapabilityTableV1,
    loopback: CapabilityTokenV1,
    no_net: CapabilityTokenV1,
    resources: ResourceTableV1,
}

impl Datagrams {
    fn new() -> Self {
        let mut revocation = RevocationDomain::default();
        let mut capabilities = CapabilityTableV1::default();
        let loopback = capabilities.insert(CapabilityGrantV1::mint_root(
            CapabilityTraceIdentity("net:datagram-loopback".into()),
            crate::Cap::mint(crate::AUTH_FULL, "FS")
                .with_declared_net_grant(&[crate::NetClassV1::Loopback]),
            &mut revocation,
        ));
        let no_net = capabilities.insert(CapabilityGrantV1::mint_root(
            CapabilityTraceIdentity("net:datagram-none".into()),
            crate::Cap::mint(crate::AUTH_FULL, "FS"),
            &mut revocation,
        ));
        Self {
            backend: CountingDatagramBackend::default(),
            revocation,
            capabilities,
            loopback,
            no_net,
            resources: ResourceTableV1::default(),
        }
    }

    fn dispatch(
        &mut self,
        operation: HostOpV1,
        capability: Option<CapabilityTokenV1>,
        inputs: ResourceInputsV1,
        request: CanonicalRequestV1,
    ) -> HostDispatchReplyV1 {
        dispatch_host_op_v1(
            &mut self.backend,
            &self.capabilities,
            &self.revocation,
            &mut self.resources,
            operation,
            capability,
            inputs,
            &request,
        )
        .expect("well-shaped datagram request")
    }

    fn bind(&mut self, address: SocketAddressV1) -> ResourceTokenV1 {
        let reply = self.dispatch(
            HostOpV1::NetDatagramBind,
            Some(self.loopback),
            ResourceInputsV1::None,
            CanonicalRequestV1::NetDatagramBind { address },
        );
        assert!(
            matches!(reply.outcome, CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
                resource_kind: ResourceKindV1::SocketDatagram, ..
            })),
            "{:?}",
            reply.outcome
        );
        reply.resource_token.expect("bind mints a token")
    }

    fn local(&mut self, socket: ResourceTokenV1) -> SocketAddressV1 {
        match self.dispatch(
            HostOpV1::NetDatagramLocalAddress,
            None,
            ResourceInputsV1::Target(socket),
            CanonicalRequestV1::NetDatagramLocalAddress,
        )
        .outcome
        {
            CanonicalOutcomeV1::Success(CanonicalReplyV1::SocketAddress(address)) => address,
            other => panic!("expected an address, got {other:?}"),
        }
    }

    fn receive(&mut self, socket: ResourceTokenV1, buffer: ResourceTokenV1, length: u64)
        -> CanonicalOutcomeV1 {
        self.dispatch(
            HostOpV1::NetDatagramReceiveFrom,
            None,
            ResourceInputsV1::FileBuffer { file: socket, buffer },
            CanonicalRequestV1::NetDatagramReceiveFrom { buffer_start: 0, length },
        )
        .outcome
    }

    /// Writes `pattern` over the whole buffer and installs it as the
    /// initialized window, so a send's refusal can only come from the guard
    /// under test and never from an uninitialized span.
    fn fill(&mut self, buffer: ResourceTokenV1, pattern: impl Fn(usize) -> u8) {
        let ResourceSlotStateV1::Live { owner: ResourceOwnerV1::Buffer(region), .. } =
            &mut self.resources.slots[buffer.slot as usize].state
        else {
            panic!("fresh buffer has a live buffer owner")
        };
        let capacity = region.capacity();
        for (index, byte) in region.bytes.iter_mut().enumerate() {
            *byte = pattern(index);
        }
        region.install_window(0, capacity);
    }

    fn send(&mut self, socket: ResourceTokenV1, buffer: ResourceTokenV1, length: u64,
        peer: SocketAddressV1) -> CanonicalOutcomeV1 {
        self.dispatch(
            HostOpV1::NetDatagramSendTo,
            None,
            ResourceInputsV1::FileBufferSpan { file: socket, target_buffer: buffer, span_origin: buffer },
            CanonicalRequestV1::NetDatagramSendTo { buffer_start: 0, length, peer },
        )
        .outcome
    }
}

fn udp_address(address: &SocketAddressV1) -> std::net::SocketAddr {
    let SocketAddressV1::Inet4 { octets, port } = address else { panic!("IPv4 expected") };
    std::net::SocketAddr::from((*octets, *port))
}

fn received(outcome: CanonicalOutcomeV1) -> (u64, u64, bool, Option<SocketAddressV1>) {
    match outcome {
        CanonicalOutcomeV1::Success(CanonicalReplyV1::DatagramReceipt(DatagramReceiptV1::Received {
            span, transferred, truncated, sender,
        })) => {
            assert_eq!(span.start(), 0);
            assert_eq!(span.length(), transferred.get());
            (transferred.get(), transferred.effective_request(), truncated, sender)
        }
        other => panic!("expected a received datagram, got {other:?}"),
    }
}

/// Promise class: durable invariant. MEASURED: through the shared dispatcher,
/// a bound socket receives an external datagram whole with its exact sender,
/// sends the received span to a second bound socket as one datagram (Unit),
/// and the second socket's six-byte window reports TruncatedDatagram with
/// count 6 of effective 6 and the first socket's exact local address; a
/// zero-length datagram is EmptyDatagram with its sender. CLAIMED: the
/// receipt's fit, count and sender follow §1.13.1. THE GAP: IPv4 only; the
/// CLI rows cover IPv6 and Unix through checked Ken.
#[test]
fn datagram_dispatch_reports_receipts_fit_and_exact_sender() {
    let mut fixture = Datagrams::new();
    let first = fixture.bind(LOOPBACK4);
    let second = fixture.bind(LOOPBACK4);
    let first_address = fixture.local(first);
    let second_address = fixture.local(second);
    let (buffer, _) = fixture.resources.insert_buffer(16).unwrap();
    let external = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    let external_address = SocketAddressV1::Inet4 {
        octets: [127, 0, 0, 1],
        port: external.local_addr().unwrap().port(),
    };
    external.send_to(b"PX11ok!", udp_address(&first_address)).unwrap();
    assert_eq!(
        received(fixture.receive(first, buffer, 16)),
        (7, 16, false, Some(external_address.clone()))
    );
    assert_eq!(
        fixture.send(first, buffer, 7, second_address.clone()),
        CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
    );
    let (short, _) = fixture.resources.insert_buffer(6).unwrap();
    assert_eq!(received(fixture.receive(second, short, 16)), (6, 6, true, Some(first_address)));
    external.send_to(b"", udp_address(&second_address)).unwrap();
    assert_eq!(
        fixture.receive(second, short, 6),
        CanonicalOutcomeV1::Success(CanonicalReplyV1::DatagramReceipt(DatagramReceiptV1::Empty {
            sender: Some(external_address),
        }))
    );
}

/// Promise class: durable invariant. MEASURED: an out-of-class peer
/// (CapabilityDenied), a peer of another family (InvalidInput), an effective
/// span over 65507 bytes (InvalidInput), an empty span and a zero window
/// (InvalidBounds) each return their typed refusal while the backend's send
/// and receive leaves stay at zero calls, the 65508-byte refusal on a fully
/// initialized buffer; then a fully initialized 65507-byte span is sent as
/// Unit with exactly one send-leaf call, and an external socket receives
/// exactly those 65507 bytes from the socket's own address. CLAIMED: every
/// send refusal precedes any syscall, and 65507 is the largest accepted
/// payload, not merely the largest unrefused one. THE GAP: the leaf count is
/// the backend boundary, not a syscall trace; the bound is pinned for IPv4
/// loopback only.
#[test]
fn datagram_send_refusals_precede_the_backend() {
    let mut fixture = Datagrams::new();
    let socket = fixture.bind(LOOPBACK4);
    let peer = fixture.local(socket);
    let (buffer, _) = fixture.resources.insert_buffer(65_508).unwrap();
    let denied = |cause| CanonicalOutcomeV1::Error(SemanticErrorV1::Io(cause));
    assert_eq!(
        fixture.send(socket, buffer, 1, SocketAddressV1::Inet4 { octets: [192, 0, 2, 1], port: 9 }),
        denied(IoErrorIdentityV1::CapabilityDenied)
    );
    assert_eq!(
        fixture.send(socket, buffer, 1, SocketAddressV1::Inet6 {
            octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            port: 9,
        }),
        denied(IoErrorIdentityV1::InvalidInput)
    );
    assert_eq!(fixture.send(socket, buffer, 65_508, peer.clone()), denied(IoErrorIdentityV1::InvalidInput));
    assert_eq!(
        fixture.send(socket, buffer, 0, peer.clone()),
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::InvalidBounds))
    );
    assert_eq!(
        fixture.receive(socket, buffer, 0),
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::InvalidBounds))
    );
    assert_eq!((fixture.backend.sends, fixture.backend.receives), (0, 0));
    // With every byte initialized, one byte over the bound is still refused
    // by the payload bound alone, before the leaf.
    let pattern = |index: usize| (index % 251) as u8;
    fixture.fill(buffer, pattern);
    assert_eq!(fixture.send(socket, buffer, 65_508, peer.clone()), denied(IoErrorIdentityV1::InvalidInput));
    assert_eq!(fixture.backend.sends, 0);
    // At the bound the fully initialized span is accepted and sent whole.
    let external = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    external.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
    let external_address = SocketAddressV1::Inet4 {
        octets: [127, 0, 0, 1],
        port: external.local_addr().unwrap().port(),
    };
    assert_eq!(
        fixture.send(socket, buffer, 65_507, external_address),
        CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
    );
    assert_eq!(fixture.backend.sends, 1);
    let mut observed = vec![0u8; 65_536];
    let (count, from) = external.recv_from(&mut observed).expect("the maximum datagram arrives");
    assert_eq!(count, 65_507);
    assert_eq!(from, udp_address(&peer));
    assert!(observed[..count].iter().enumerate().all(|(index, byte)| *byte == pattern(index)));
}

/// Promise class: durable invariant. MEASURED: a grant without Net, and a
/// Loopback grant asked for an Inet address, refuse bind with
/// CapabilityDenied; a revoked grant refuses with Revoked; the backend bind
/// leaf runs zero times for all three. CLAIMED: bind is gated on the
/// originating grant before any socket exists. THE GAP: the CLI cannot reach
/// the no-Net arm, because program admission refuses a Net row without a
/// Net declaration before main runs.
#[test]
fn datagram_bind_refusals_precede_the_backend() {
    let mut fixture = Datagrams::new();
    let denied = |cause| CanonicalOutcomeV1::Error(SemanticErrorV1::Io(cause));
    let no_net = fixture.no_net;
    assert_eq!(
        fixture.dispatch(HostOpV1::NetDatagramBind, Some(no_net), ResourceInputsV1::None,
            CanonicalRequestV1::NetDatagramBind { address: LOOPBACK4 }).outcome,
        denied(IoErrorIdentityV1::CapabilityDenied)
    );
    let loopback = fixture.loopback;
    assert_eq!(
        fixture.dispatch(HostOpV1::NetDatagramBind, Some(loopback), ResourceInputsV1::None,
            CanonicalRequestV1::NetDatagramBind {
                address: SocketAddressV1::Inet4 { octets: [192, 0, 2, 1], port: 0 },
            }).outcome,
        denied(IoErrorIdentityV1::CapabilityDenied)
    );
    assert!(fixture.capabilities.resolve(loopback).unwrap().revoke(&mut fixture.revocation));
    assert_eq!(
        fixture.dispatch(HostOpV1::NetDatagramBind, Some(loopback), ResourceInputsV1::None,
            CanonicalRequestV1::NetDatagramBind { address: LOOPBACK4 }).outcome,
        denied(IoErrorIdentityV1::Revoked)
    );
    assert_eq!(fixture.backend.binds, 0);
}

/// Promise class: durable invariant. MEASURED: a send on a socket whose
/// originating grant was revoked after bind returns Revoked with zero send
/// calls; the stream local-address operation refuses a datagram token by
/// kind; the datagram local-address operation refuses a buffer token by kind;
/// and a released socket is Closed. CLAIMED: datagram resources keep grant
/// provenance and kind discipline. THE GAP: the stream/datagram mismatch is
/// also excluded statically in checked Ken.
#[test]
fn datagram_resources_keep_provenance_kind_and_release() {
    let mut fixture = Datagrams::new();
    let socket = fixture.bind(LOOPBACK4);
    let peer = fixture.local(socket);
    let (buffer, _) = fixture.resources.insert_buffer(4).unwrap();
    assert!(matches!(
        fixture.dispatch(HostOpV1::NetLocalAddress, None, ResourceInputsV1::Target(socket),
            CanonicalRequestV1::NetLocalAddress).outcome,
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::ResourceKindMismatch {
            actual: ResourceKindV1::SocketDatagram, ..
        }))
    ));
    assert_eq!(
        fixture.dispatch(HostOpV1::NetDatagramLocalAddress, None, ResourceInputsV1::Target(buffer),
            CanonicalRequestV1::NetDatagramLocalAddress).outcome,
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::ResourceKindMismatch {
            expected: ResourceKindV1::SocketDatagram,
            actual: ResourceKindV1::Buffer,
        }))
    );
    let loopback = fixture.loopback;
    assert!(fixture.capabilities.resolve(loopback).unwrap().revoke(&mut fixture.revocation));
    assert_eq!(
        fixture.send(socket, buffer, 1, peer),
        CanonicalOutcomeV1::Error(SemanticErrorV1::Io(IoErrorIdentityV1::Revoked))
    );
    assert_eq!(fixture.backend.sends, 0);
    let released = fixture.dispatch(HostOpV1::ResourceRelease, None, ResourceInputsV1::Target(socket),
        CanonicalRequestV1::ResourceRelease);
    assert!(matches!(released.outcome, CanonicalOutcomeV1::Success(_)), "{:?}", released.outcome);
    assert_eq!(
        fixture.dispatch(HostOpV1::NetDatagramLocalAddress, None, ResourceInputsV1::Target(socket),
            CanonicalRequestV1::NetDatagramLocalAddress).outcome,
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::Closed))
    );
}
