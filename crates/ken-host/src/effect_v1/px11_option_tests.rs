//! PX11-SOCKET-OPTION-FAMILIES: the shared dispatcher over the real stream
//! option and reusable-listener leaves. Spec 38 §1.13.2.
//!
//! The one `unsafe` block is the independent kernel oracle: a direct
//! `getsockopt` on a live descriptor this test owns, read by the option's
//! own libc constants rather than through the leaf under test.

#![allow(unsafe_code)]

use std::os::fd::AsRawFd;

use super::*;
use crate::{RevocationDomain, SocketAddressV1, StreamOptionV1};

/// Real socket leaves, with a count of each option-family leaf the
/// dispatcher reached. `scripted_get`, when set, replaces the kernel's
/// `getsockopt` answer, so a host that echoes the last set value is
/// distinguishable from one that reads the kernel.
#[derive(Default)]
struct CountingOptionBackend {
    reusable_listens: usize,
    sets: usize,
    gets: usize,
    scripted_get: Option<bool>,
}

impl HostEffectBackendV1 for CountingOptionBackend {
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
    fn net_listen_reusable(&mut self, root: &crate::FsHandle, address: &SocketAddressV1)
        -> Result<std::os::fd::OwnedFd, IoErrorIdentityV1> {
        self.reusable_listens += 1;
        crate::net_v1::listen_reusable(root, address)
    }
    fn net_set_stream_option(&mut self, stream: &std::os::fd::OwnedFd, option: StreamOptionV1,
        enabled: bool) -> Result<(), IoErrorIdentityV1> {
        self.sets += 1;
        crate::net_v1::set_stream_option(stream, option, enabled)
            .map_err(|error| io_error_identity_v1(&error))
    }
    fn net_stream_option(&mut self, stream: &std::os::fd::OwnedFd, option: StreamOptionV1)
        -> Result<bool, IoErrorIdentityV1> {
        self.gets += 1;
        let kernel = crate::net_v1::stream_option(stream, option)
            .map_err(|error| io_error_identity_v1(&error))?;
        Ok(self.scripted_get.unwrap_or(kernel))
    }
}

const LOOPBACK4: SocketAddressV1 = SocketAddressV1::Inet4 { octets: [127, 0, 0, 1], port: 0 };
const LOOPBACK6: SocketAddressV1 = SocketAddressV1::Inet6 {
    octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    port: 0,
};
const OPTIONS: [StreamOptionV1; 2] = [StreamOptionV1::NoDelay, StreamOptionV1::KeepAlive];

fn io(cause: IoErrorIdentityV1) -> CanonicalOutcomeV1 {
    CanonicalOutcomeV1::Error(SemanticErrorV1::Io(cause))
}

struct Options {
    backend: CountingOptionBackend,
    revocation: RevocationDomain,
    capabilities: CapabilityTableV1,
    loopback: CapabilityTokenV1,
    unix: CapabilityTokenV1,
    resources: ResourceTableV1,
}

impl Options {
    fn new() -> Self {
        let mut revocation = RevocationDomain::default();
        let mut capabilities = CapabilityTableV1::default();
        let loopback = capabilities.insert(CapabilityGrantV1::mint_root(
            CapabilityTraceIdentity("net:option-loopback".into()),
            crate::Cap::mint(crate::AUTH_FULL, "FS")
                .with_declared_net_grant(&[crate::NetClassV1::Loopback]),
            &mut revocation,
        ));
        let unix = capabilities.insert(CapabilityGrantV1::mint_root(
            CapabilityTraceIdentity("net:option-unix".into()),
            crate::Cap::mint(crate::AUTH_FULL, "FS")
                .with_declared_net_grant(&[crate::NetClassV1::Unix]),
            &mut revocation,
        ));
        Self {
            backend: CountingOptionBackend::default(),
            revocation,
            capabilities,
            loopback,
            unix,
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
        .expect("well-shaped option request")
    }

    fn acquire(&mut self, operation: HostOpV1, request: CanonicalRequestV1, inputs: ResourceInputsV1,
        capability: Option<CapabilityTokenV1>, kind: ResourceKindV1) -> ResourceTokenV1 {
        let reply = self.dispatch(operation, capability, inputs, request);
        match reply.outcome {
            CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired { resource_kind, .. })
                if resource_kind == kind => {}
            other => panic!("{operation:?} expected {kind:?}, got {other:?}"),
        }
        reply.resource_token.expect("acquisition mints a token")
    }

    fn local(&mut self, target: ResourceTokenV1) -> SocketAddressV1 {
        match self.dispatch(HostOpV1::NetLocalAddress, None, ResourceInputsV1::Target(target),
            CanonicalRequestV1::NetLocalAddress).outcome
        {
            CanonicalOutcomeV1::Success(CanonicalReplyV1::SocketAddress(address)) => address,
            other => panic!("expected an address, got {other:?}"),
        }
    }

    /// An outbound stream and its accepted peer over a loopback listener.
    fn connected_pair(&mut self, listen_on: SocketAddressV1) -> (ResourceTokenV1, ResourceTokenV1) {
        let loopback = Some(self.loopback);
        let listener = self.acquire(HostOpV1::NetListen,
            CanonicalRequestV1::NetListen { address: listen_on },
            ResourceInputsV1::None, loopback, ResourceKindV1::SocketListener);
        let address = self.local(listener);
        let outbound = self.acquire(HostOpV1::NetConnect,
            CanonicalRequestV1::NetConnect { address },
            ResourceInputsV1::None, loopback, ResourceKindV1::SocketStream);
        let accepted = self.acquire(HostOpV1::NetAccept, CanonicalRequestV1::NetAccept,
            ResourceInputsV1::Target(listener), None, ResourceKindV1::SocketStream);
        (outbound, accepted)
    }

    fn set(&mut self, stream: ResourceTokenV1, option: u64, enabled: bool) -> CanonicalOutcomeV1 {
        self.dispatch(HostOpV1::NetStreamOptionSet, None, ResourceInputsV1::Target(stream),
            CanonicalRequestV1::NetStreamOptionSet { option, enabled }).outcome
    }

    fn get(&mut self, stream: ResourceTokenV1, option: u64) -> CanonicalOutcomeV1 {
        self.dispatch(HostOpV1::NetStreamOptionGet, None, ResourceInputsV1::Target(stream),
            CanonicalRequestV1::NetStreamOptionGet { option }).outcome
    }

    /// The live descriptor behind a socket token, for an oracle that reads
    /// the kernel without going through the dispatcher.
    fn raw_fd(&self, token: ResourceTokenV1) -> std::os::fd::RawFd {
        match &self.resources.slots[token.slot as usize].state {
            ResourceSlotStateV1::Live { owner: ResourceOwnerV1::SocketStream { fd, .. }, .. }
            | ResourceSlotStateV1::Live { owner: ResourceOwnerV1::SocketListener { fd, .. }, .. } =>
                fd.as_raw_fd(),
            _ => panic!("live socket expected"),
        }
    }
}

/// An independent kernel read, by the option's own constants.
fn kernel_option(fd: std::os::fd::RawFd, level: libc::c_int, name: libc::c_int) -> libc::c_int {
    let mut value: libc::c_int = -1;
    let mut length = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    let status = unsafe {
        libc::getsockopt(fd, level, name, &mut value as *mut libc::c_int as *mut libc::c_void,
            &mut length)
    };
    assert_eq!(status, 0, "independent getsockopt");
    value
}

fn kernel_constants(option: StreamOptionV1) -> (libc::c_int, libc::c_int) {
    match option {
        StreamOptionV1::NoDelay => (libc::IPPROTO_TCP, libc::TCP_NODELAY),
        StreamOptionV1::KeepAlive => (libc::SOL_SOCKET, libc::SO_KEEPALIVE),
    }
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2.
/// Promise class: durable invariant. MEASURED: through the shared dispatcher,
/// on an IPv4 outbound, IPv4 accepted, IPv6 outbound and IPv6 accepted
/// stream, each option reads False when fresh, then True after a True set
/// and False after a False set; after every set an independent `getsockopt`
/// on the resource's own descriptor, keyed by the option's libc constants,
/// agrees with the dispatcher's readback. CLAIMED: set reaches the kernel's
/// option for that family, and get reports it. THE GAP: the independent read
/// shares the descriptor, not the leaf; the effect of the option on traffic
/// (Nagle, probes) is the kernel's and is not measured here.
#[test]
fn stream_options_read_back_the_kernel_value_on_every_ip_stream() {
    let mut fixture = Options::new();
    let (outbound4, accepted4) = fixture.connected_pair(LOOPBACK4);
    let (outbound6, accepted6) = fixture.connected_pair(LOOPBACK6);
    for stream in [outbound4, accepted4, outbound6, accepted6] {
        for option in OPTIONS {
            let code = option.code();
            let (level, name) = kernel_constants(option);
            assert_eq!(fixture.get(stream, code), CanonicalOutcomeV1::Success(CanonicalReplyV1::Bool(false)),
                "{option:?} default");
            for enabled in [true, false] {
                assert_eq!(fixture.set(stream, code, enabled),
                    CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit), "{option:?} set {enabled}");
                assert_eq!(kernel_option(fixture.raw_fd(stream), level, name) != 0, enabled,
                    "{option:?} kernel after set {enabled}");
                assert_eq!(fixture.get(stream, code),
                    CanonicalOutcomeV1::Success(CanonicalReplyV1::Bool(enabled)), "{option:?} read {enabled}");
            }
        }
    }
    assert_eq!((fixture.backend.sets, fixture.backend.gets), (16, 24));
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2.
/// Promise class: durable invariant. MEASURED: with a scripted backend whose
/// `getsockopt` answer is the opposite of the value just set, the
/// dispatcher's readback is the backend's answer each time, for both options
/// and both set values. CLAIMED: `stream_option` reports the kernel's value
/// and never a host-cached copy of the last set. THE GAP: the scripted answer
/// replaces only the readback leaf; the set leaf is real.
#[test]
fn stream_option_readback_is_the_backend_answer_not_the_last_set() {
    let mut fixture = Options::new();
    let (stream, _) = fixture.connected_pair(LOOPBACK4);
    for option in OPTIONS {
        for enabled in [true, false] {
            assert_eq!(fixture.set(stream, option.code(), enabled),
                CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit));
            fixture.backend.scripted_get = Some(!enabled);
            assert_eq!(fixture.get(stream, option.code()),
                CanonicalOutcomeV1::Success(CanonicalReplyV1::Bool(!enabled)),
                "{option:?} after set {enabled}");
            fixture.backend.scripted_get = None;
        }
    }
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2.
/// Promise class: durable invariant. MEASURED: on a Unix stream (a real
/// connected AF_UNIX descriptor recorded with the Unix family), set and get
/// of each option return InvalidInput; on an IP stream, option code 2 and
/// `u64::MAX` return InvalidInput for set and get; after the stream's
/// originating grant is revoked, set and get return Revoked; and after
/// release they are Closed. The backend's set and get leaves stay at zero
/// calls throughout. CLAIMED: the fit check, the code check, revocation and
/// release all refuse before any syscall. THE GAP: the Unix stream is
/// inserted directly rather than acquired, so its family record is the one
/// this row states; the CLI row R4 acquires one through checked Ken.
#[test]
fn unix_streams_unknown_codes_revocation_and_release_refuse_before_any_leaf() {
    let mut fixture = Options::new();
    let (unix_fd, _peer) = std::os::unix::net::UnixStream::pair().expect("unix pair");
    let unix_provenance = fixture.capabilities.resolve(fixture.unix).unwrap().revocation_node;
    let (unix_stream, _) = fixture.resources.insert_socket(
        std::os::fd::OwnedFd::from(unix_fd),
        ResourceKindV1::SocketStream,
        unix_provenance,
        None,
        crate::net_v1::SocketFamilyV1::Unix,
    );
    for option in OPTIONS {
        assert_eq!(fixture.set(unix_stream, option.code(), true), io(IoErrorIdentityV1::InvalidInput),
            "{option:?} set on Unix");
        assert_eq!(fixture.get(unix_stream, option.code()), io(IoErrorIdentityV1::InvalidInput),
            "{option:?} get on Unix");
    }
    let (stream, _) = fixture.connected_pair(LOOPBACK4);
    for code in [2, u64::MAX] {
        assert_eq!(fixture.set(stream, code, true), io(IoErrorIdentityV1::InvalidInput), "set code {code}");
        assert_eq!(fixture.get(stream, code), io(IoErrorIdentityV1::InvalidInput), "get code {code}");
    }
    assert_eq!((fixture.backend.sets, fixture.backend.gets), (0, 0));
    let loopback = fixture.loopback;
    assert!(fixture.capabilities.resolve(loopback).unwrap().revoke(&mut fixture.revocation));
    assert_eq!(fixture.set(stream, 0, true), io(IoErrorIdentityV1::Revoked));
    assert_eq!(fixture.get(stream, 0), io(IoErrorIdentityV1::Revoked));
    assert_eq!((fixture.backend.sets, fixture.backend.gets), (0, 0));
    let released = fixture.dispatch(HostOpV1::ResourceRelease, None, ResourceInputsV1::Target(unix_stream),
        CanonicalRequestV1::ResourceRelease);
    assert!(matches!(released.outcome, CanonicalOutcomeV1::Success(_)), "{:?}", released.outcome);
    assert_eq!(fixture.get(unix_stream, 0),
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::Closed)));
    assert_eq!((fixture.backend.sets, fixture.backend.gets), (0, 0));
}

/// SOURCE: spec/30-surface/38-ffi-io.md §1.13.2.
/// Promise class: durable invariant. MEASURED: a reusable listen on a Unix
/// address under a Net Unix grant returns InvalidInput, under a
/// loopback-only grant returns CapabilityDenied (grant admission first),
/// and without a grant returns CapabilityDenied, all with zero reusable
/// listen calls; then a reusable IPv4 and IPv6 listen each reach the leaf
/// once and mint a SocketListener whose descriptor reads SO_REUSEADDR 1 by
/// an independent `getsockopt`, while a plain NetListen descriptor reads 0.
/// CLAIMED: the reusable acquisition is IP only, passes every NetListen
/// guard, and sets the option the plain acquisition does not. THE GAP: the
/// option's rebind effect over TIME_WAIT and its before-bind order are
/// measured by the CLI row R2, not here.
#[test]
fn reusable_listener_refuses_unix_and_sets_reuse_only_on_its_own_acquisition() {
    let mut fixture = Options::new();
    let unix_address = SocketAddressV1::Unix { path: b"/px11-option-reusable.sock".to_vec() };
    let reusable = |address: SocketAddressV1| CanonicalRequestV1::NetListenReusable { address };
    let (unix, loopback) = (fixture.unix, fixture.loopback);
    assert_eq!(fixture.dispatch(HostOpV1::NetListenReusable, Some(unix), ResourceInputsV1::None,
        reusable(unix_address.clone())).outcome, io(IoErrorIdentityV1::InvalidInput));
    assert_eq!(fixture.dispatch(HostOpV1::NetListenReusable, Some(loopback), ResourceInputsV1::None,
        reusable(unix_address.clone())).outcome, io(IoErrorIdentityV1::CapabilityDenied));
    assert_eq!(fixture.dispatch(HostOpV1::NetListenReusable, None, ResourceInputsV1::None,
        reusable(LOOPBACK4)).outcome, io(IoErrorIdentityV1::CapabilityDenied));
    assert_eq!(fixture.backend.reusable_listens, 0);
    for (count, address) in [(1, LOOPBACK4), (2, LOOPBACK6)] {
        let listener = fixture.acquire(HostOpV1::NetListenReusable, reusable(address.clone()),
            ResourceInputsV1::None, Some(loopback), ResourceKindV1::SocketListener);
        assert_eq!(fixture.backend.reusable_listens, count);
        assert_eq!(kernel_option(fixture.raw_fd(listener), libc::SOL_SOCKET, libc::SO_REUSEADDR), 1,
            "{address:?} reusable");
        let plain = fixture.acquire(HostOpV1::NetListen, CanonicalRequestV1::NetListen { address },
            ResourceInputsV1::None, Some(loopback), ResourceKindV1::SocketListener);
        assert_eq!(kernel_option(fixture.raw_fd(plain), libc::SOL_SOCKET, libc::SO_REUSEADDR), 0);
    }
}
