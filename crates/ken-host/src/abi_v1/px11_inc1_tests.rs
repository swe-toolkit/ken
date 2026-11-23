use super::*;
use crate::{ResourceInputsV1, ResourceTokenV1, SocketAddressV1};

fn fact(name: &str) -> u64 {
    crate::HOST_EFFECT_ABI_V1_FACTS
        .iter()
        .find_map(|(key, value)| (*key == name).then_some(*value))
        .unwrap_or_else(|| panic!("missing C layout fact {name}"))
}

/// Promise class: normative compatibility vector. MEASURED: C-probed layout
/// agrees with each Rust record, including the nested address, and the two
/// stream record pairs have equal generated offsets, sizes and alignments.
/// CLAIMED: both ABIs can use the shared stream emitter in Inc2. THE GAP:
/// emitter lowering itself remains unavailable until Inc2's parity tests.
#[test]
fn px11_native_net_layout_matches_c_and_stream_offsets_pairwise() {
    macro_rules! record {
        ($ty:ident; $($field:ident),+ $(,)?) => {
            assert_eq!(fact(concat!("SIZE_", stringify!($ty))), std::mem::size_of::<$ty>() as u64);
            assert_eq!(fact(concat!("ALIGN_", stringify!($ty))), std::mem::align_of::<$ty>() as u64);
            $(assert_eq!(fact(concat!("OFFSET_", stringify!($ty), "_", stringify!($field))),
                std::mem::offset_of!($ty, $field) as u64);)+
        };
    }
    record!(SocketAddressRecordV1; family, bytes, port);
    record!(NetListenRequestV1; capability, address);
    record!(NetAcceptRequestV1; listener);
    record!(NetConnectRequestV1; capability, address);
    record!(NetReceiveRequestV1; stream, buffer, buffer_start, length);
    record!(NetSendRequestV1; stream, buffer, buffer_start, length, span_origin);
    record!(NetShutdownRequestV1; stream, how);
    record!(NetLocalAddressRequestV1; target);
    for field in ["stream", "buffer", "buffer_start", "length"] {
        assert_eq!(
            fact(&format!("OFFSET_NetReceiveRequestV1_{field}")),
            fact(&format!("OFFSET_ResourceStreamReadRequestV1_{field}")),
            "read field {field}",
        );
        assert_eq!(
            fact(&format!("OFFSET_NetSendRequestV1_{field}")),
            fact(&format!("OFFSET_ResourceStreamWriteRequestV1_{field}")),
            "write field {field}",
        );
    }
    assert_eq!(
        fact("OFFSET_NetSendRequestV1_span_origin"),
        fact("OFFSET_ResourceStreamWriteRequestV1_span_origin"),
    );
    for (left, right) in [
        ("NetReceiveRequestV1", "ResourceStreamReadRequestV1"),
        ("NetSendRequestV1", "ResourceStreamWriteRequestV1"),
    ] {
        for prefix in ["SIZE", "ALIGN"] {
            assert_eq!(
                fact(&format!("{prefix}_{left}")),
                fact(&format!("{prefix}_{right}"))
            );
        }
    }
}

unsafe fn decode<T>(
    op: HostOpV1,
    record: &T,
) -> Result<
    (
        Option<CapabilityTokenV1>,
        ResourceInputsV1,
        CanonicalRequestV1,
    ),
    i64,
> {
    unsafe { decode_native_request_v1(op, (record as *const T).cast(), std::mem::size_of::<T>()) }
}

fn token(word: u64) -> ResourceTokenV1 {
    ResourceTokenV1::from_erased_identity(word)
}

/// Promise class: normative compatibility vector. MEASURED: each of seven
/// distinct C-shaped records yields the typed canonical request and exact
/// resource/capability operands, and malformed addresses fail with -1.
/// CLAIMED: Inc1's native decoder is complete and structural only. THE GAP:
/// these calls bypass the unchanged RepresentedUnavailable dispatch gate;
/// operational native parity is an Inc2 acceptance criterion.
#[test]
fn px11_native_net_request_decode_known_answers_and_malformed_addresses() {
    let ipv4 = [127, 0, 0, 1];
    let ipv6 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let unix = b"/tmp/px11-native";
    let address = |family, bytes: &[u8], port| SocketAddressRecordV1 {
        family,
        bytes: SliceV1 {
            data: bytes.as_ptr(),
            len: bytes.len(),
        },
        port,
    };
    let cap = 0x0000_0002_0000_0003;
    let listener = 0x0000_0004_0000_0005;
    let stream = 0x0000_0006_0000_0007;
    let buffer = 0x0000_0008_0000_0009;
    let span_origin = 0x0000_000a_0000_000b;
    let cases = [
        (
            unsafe {
                decode(
                    HostOpV1::NetListen,
                    &NetListenRequestV1 {
                        capability: cap,
                        address: address(0, &ipv4, 4242),
                    },
                )
            }
            .unwrap(),
            (
                Some(CapabilityTokenV1::from_erased_identity(cap)),
                ResourceInputsV1::None,
                CanonicalRequestV1::NetListen {
                    address: SocketAddressV1::Inet4 {
                        octets: ipv4,
                        port: 4242,
                    },
                },
            ),
        ),
        (
            unsafe { decode(HostOpV1::NetAccept, &NetAcceptRequestV1 { listener }) }.unwrap(),
            (
                None,
                ResourceInputsV1::Target(token(listener)),
                CanonicalRequestV1::NetAccept,
            ),
        ),
        (
            unsafe {
                decode(
                    HostOpV1::NetConnect,
                    &NetConnectRequestV1 {
                        capability: cap,
                        address: address(2, unix, 0),
                    },
                )
            }
            .unwrap(),
            (
                Some(CapabilityTokenV1::from_erased_identity(cap)),
                ResourceInputsV1::None,
                CanonicalRequestV1::NetConnect {
                    address: SocketAddressV1::Unix {
                        path: unix.to_vec(),
                    },
                },
            ),
        ),
        (
            unsafe {
                decode(
                    HostOpV1::NetReceive,
                    &NetReceiveRequestV1 {
                        stream,
                        buffer,
                        buffer_start: 21,
                        length: 34,
                    },
                )
            }
            .unwrap(),
            (
                None,
                ResourceInputsV1::FileBuffer {
                    file: token(stream),
                    buffer: token(buffer),
                },
                CanonicalRequestV1::NetReceive {
                    buffer_start: 21,
                    length: 34,
                },
            ),
        ),
        (
            unsafe {
                decode(
                    HostOpV1::NetSend,
                    &NetSendRequestV1 {
                        stream,
                        buffer,
                        buffer_start: 55,
                        length: 89,
                        span_origin,
                    },
                )
            }
            .unwrap(),
            (
                None,
                ResourceInputsV1::FileBufferSpan {
                    file: token(stream),
                    target_buffer: token(buffer),
                    span_origin: token(span_origin),
                },
                CanonicalRequestV1::NetSend {
                    buffer_start: 55,
                    length: 89,
                },
            ),
        ),
        (
            unsafe {
                decode(
                    HostOpV1::NetShutdown,
                    &NetShutdownRequestV1 { stream, how: 99 },
                )
            }
            .unwrap(),
            (
                None,
                ResourceInputsV1::Target(token(stream)),
                CanonicalRequestV1::NetShutdown { how: 99 },
            ),
        ),
        (
            unsafe {
                decode(
                    HostOpV1::NetLocalAddress,
                    &NetLocalAddressRequestV1 { target: listener },
                )
            }
            .unwrap(),
            (
                None,
                ResourceInputsV1::Target(token(listener)),
                CanonicalRequestV1::NetLocalAddress,
            ),
        ),
    ];
    for (actual, expected) in cases {
        assert_eq!(actual, expected);
    }
    assert_eq!(
        unsafe {
            decode(
                HostOpV1::NetConnect,
                &NetConnectRequestV1 {
                    capability: cap,
                    address: address(1, &ipv6, 65535),
                },
            )
        }
        .unwrap()
        .2,
        CanonicalRequestV1::NetConnect {
            address: SocketAddressV1::Inet6 {
                octets: ipv6,
                port: 65535
            }
        },
    );
    for (family, bytes, port) in [
        (3, &ipv4[..], 0),
        (0, &ipv4[..3], 0),
        (0, &ipv4[..], 65536),
        (1, &ipv6[..15], 0),
        (1, &ipv6[..], 65536),
        (2, &unix[..], 1),
    ] {
        assert_eq!(
            unsafe {
                decode(
                    HostOpV1::NetListen,
                    &NetListenRequestV1 {
                        capability: cap,
                        address: address(family, bytes, port),
                    },
                )
            }
            .err(),
            Some(-1),
            "listen family={family} len={} port={port}",
            bytes.len(),
        );
    }
    let null_bytes = SocketAddressRecordV1 {
        family: 0,
        bytes: SliceV1 {
            data: std::ptr::null(),
            len: 4,
        },
        port: 0,
    };
    assert_eq!(
        unsafe {
            decode(
                HostOpV1::NetConnect,
                &NetConnectRequestV1 {
                    capability: cap,
                    address: null_bytes,
                },
            )
        }
        .err(),
        Some(-1)
    );
    // The decoder deliberately does not validate a Unix path; the leaf owns it.
    assert_eq!(
        unsafe {
            decode(
                HostOpV1::NetListen,
                &NetListenRequestV1 {
                    capability: cap,
                    address: address(2, b"relative", 0),
                },
            )
        }
        .unwrap()
        .2,
        CanonicalRequestV1::NetListen {
            address: SocketAddressV1::Unix {
                path: b"relative".to_vec()
            }
        }
    );
    for op in [
        HostOpV1::NetListen,
        HostOpV1::NetAccept,
        HostOpV1::NetConnect,
        HostOpV1::NetReceive,
        HostOpV1::NetSend,
        HostOpV1::NetShutdown,
        HostOpV1::NetLocalAddress,
    ] {
        assert_eq!(
            op.availability(),
            crate::HostOpAvailabilityV1::RepresentedUnavailable
        );
        assert!(
            require_native_operation_v1(op).is_err(),
            "{op:?} remains unavailable"
        );
    }
}

/// Promise class: normative compatibility vector. MEASURED: reply tag,
/// family and port detail, and arena-owned bytes on all three address forms.
/// CLAIMED: the Inc1 native reply can carry a SocketAddress without silently
/// encoding an IOError.NotFound. THE GAP: Inc2 owns the generated reply decoder.
#[test]
fn px11_native_socket_address_reply_known_answers() {
    let mut context = initialize_process_context(
        RootPath::new(std::env::current_dir().unwrap()).unwrap(),
        FsRootSpec::default(),
        AUTH_FULL,
        47,
        false,
        crate::NetScopeV1::default(),
        &[],
        establish_process_posture_v1(Ok(EffectiveUidSnapshotV1::scripted(1000)), true),
        Some(EffectiveUidSnapshotV1::scripted(1000)),
    )
    .unwrap();
    let cases = [
        (
            SocketAddressV1::Inet4 {
                octets: [127, 0, 0, 1],
                port: 4242,
            },
            0 | (4242 << 8),
            vec![127, 0, 0, 1],
        ),
        (
            SocketAddressV1::Inet6 {
                octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                port: 65535,
            },
            1 | (65535 << 8),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ),
        (
            SocketAddressV1::Unix {
                path: b"/tmp/px11-native".to_vec(),
            },
            2,
            b"/tmp/px11-native".to_vec(),
        ),
    ];
    let binding = crate::HOST_EFFECT_ABI_V1_BINDINGS
        .iter()
        .find_map(|(kind, name, value)| {
            (*kind == "tag" && *name == "reply.socket_address").then_some(*value)
        })
        .expect("socket address reply tag in generated catalog bindings");
    assert_eq!(binding, 10);
    for (address, detail, expected_bytes) in cases {
        let mut reply = HostReplyV1 {
            tag: 0,
            detail: 0,
            bytes: SliceV1 {
                data: std::ptr::null(),
                len: 0,
            },
            resource_error: ResourceErrorReplyV1::default(),
            effective_request: 0,
            metadata: FileMetadataReplyV1::default(),
        };
        set_reply(
            &mut reply,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::SocketAddress(address)),
            &mut context,
        );
        assert_eq!(reply.tag, binding);
        assert_eq!(reply.detail, detail);
        assert_eq!(reply.effective_request, 0);
        assert_eq!(reply.resource_error, ResourceErrorReplyV1::default());
        assert_eq!(
            unsafe { std::slice::from_raw_parts(reply.bytes.data, reply.bytes.len) },
            expected_bytes
        );
        assert_eq!(
            context.response_arena.last().unwrap().as_ref(),
            expected_bytes
        );
    }
    drop(context);
}

/// Promise class: durable invariant. MEASURED: the real init rejects the
/// out-of-range and Unix/non-Full inputs before minting, and its sole root
/// grant transports scopes 0, 1 and 4 with distinct admission results.
/// CLAIMED: native init neither invents nor drops a checked Net grant.
/// THE GAP: native Net dispatch is still gated until Inc2.
#[test]
fn px11_native_init_scope_validation_and_root_grant() {
    let cwd = b".";
    let ipv4 = SocketAddressV1::Inet4 {
        octets: [127, 0, 0, 1],
        port: 7,
    };
    let unix = SocketAddressV1::Unix {
        path: b"/tmp/px11-native".to_vec(),
    };
    for (scope, authority, expected_status) in
        [(8, 2, -1), (4, 1, -1), (4, 2, 0), (1, 2, 0), (0, 2, 0)]
    {
        let mut result = HostInitResultV1 {
            context: std::ptr::null_mut(),
            capability: 0,
            plan_hash: 0,
        };
        let status = with_scripted_effective_uid_v1(1000, || unsafe {
            ken_host_invocation_v1_init(
                cwd.as_ptr(),
                cwd.len(),
                1,
                std::ptr::null(),
                0,
                authority,
                47,
                0,
                0,
                scope,
                1,
                crate::TARGET_ABI_MANIFEST_HASH.as_ptr(),
                crate::HOST_EFFECT_ABI_V1_HASH.as_ptr(),
                std::ptr::null(),
                0,
                &mut result,
            )
        });
        assert_eq!(
            status, expected_status,
            "scope={scope} authority={authority}"
        );
        if status != 0 {
            assert!(result.context.is_null());
            assert_eq!(result.capability, 0);
            assert_eq!(result.plan_hash, 0);
            continue;
        }
        let context = unsafe { &*result.context.cast::<ProcessContext>() };
        assert_eq!(
            result.capability,
            1_u64 << 32,
            "the sole root token has slot zero"
        );
        assert!(
            context
                .capabilities
                .resolve(CapabilityTokenV1::from_erased_identity((1_u64 << 32) | 1))
                .is_err(),
            "no second capability slot was minted"
        );
        let grant = context.capabilities.resolve(context.capability).unwrap();
        assert_eq!(result.capability, context.capability.erased_identity());
        assert_eq!(grant.capability.net_scope().bits(), scope as u8);
        assert_eq!(grant.capability.net_scope().admits(&ipv4), scope == 1);
        assert_eq!(grant.capability.net_scope().admits(&unix), scope == 4);
        unsafe { ken_host_invocation_v1_destroy(result.context) };
    }
    assert_eq!(crate::NetScopeV1::from_bits_v1(7).unwrap().bits(), 7);
    assert_eq!(crate::NetScopeV1::from_bits_v1(8), None);
}

/// Promise class: durable invariant. MEASURED: native init's root grant
/// denies the sibling path with no inode, and admits the path under jail
/// through native decode and real host dispatch. CLAIMED: native scope
/// transport preserves the declared FS root rather than using the cwd.
/// THE GAP: inode absence does not count syscalls; pre-syscall ordering
/// is shared with the interpreter and rests on the shared Unix leaf.
#[test]
fn px11_native_init_dispatch_confines_unix_listen_to_declared_root() {
    use std::os::unix::fs::{DirBuilderExt, FileTypeExt};
    use std::path::{Path, PathBuf};

    struct TestDir(PathBuf);
    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let dir = std::env::temp_dir().join(format!("ken-inc1-root-{}", std::process::id()));
    std::fs::DirBuilder::new().mode(0o700).create(&dir).unwrap();
    let dir = TestDir(dir);
    let cwd = std::fs::canonicalize(&dir.0).unwrap();
    let jail = cwd.join("jail");
    std::fs::create_dir(&jail).unwrap();
    let uid = EffectiveUidSnapshotV1::scripted(1000);
    let mut context = initialize_process_context_with_lookup(
        RootPath::new(&cwd).unwrap(),
        FsRootSpec::ExecutionStartCwd(b"jail".to_vec()),
        AUTH_FULL,
        1,
        false,
        crate::NetScopeV1::from_bits_v1(4).unwrap(),
        &[],
        establish_process_posture_v1(Ok(uid), false),
        Some(uid),
        &crate::account_db_v1::LibcAccountHomeLookupV1,
    )
    .unwrap_or_else(|_| panic!("native init should bind the declared rooted Net scope"));
    let mut listen = |path: &Path| {
        let bytes = path.as_os_str().as_encoded_bytes();
        let record = NetListenRequestV1 {
            capability: context.capability.erased_identity(),
            address: SocketAddressRecordV1 {
                family: 2,
                bytes: SliceV1 {
                    data: bytes.as_ptr(),
                    len: bytes.len(),
                },
                port: 0,
            },
        };
        let op = HostOpV1::NetListen;
        // Evidence steps below the still-closed native availability gate;
        // the context, decoder, dispatcher and socket backend are production.
        let (capability, resource, request) = unsafe { decode(op, &record) }.unwrap();
        dispatch_host_op_v1(
            &mut context.host,
            &context.capabilities,
            &context.revocation,
            &mut context.resources,
            op,
            capability,
            resource,
            &request,
        )
        .unwrap()
    };
    let outside = cwd.join("outside.sock");
    let outside_reply = listen(&outside);
    assert_eq!(
        outside_reply.outcome,
        CanonicalOutcomeV1::Error(crate::SemanticErrorV1::Io(
            IoErrorIdentityV1::CapabilityDenied
        )),
        "native Unix listen outside the declared root must return CapabilityDenied",
    );
    assert!(outside_reply.resource_token.is_none());
    assert_eq!(
        std::fs::symlink_metadata(&outside).unwrap_err().kind(),
        std::io::ErrorKind::NotFound,
    );

    let inside = jail.join("inside.sock");
    let inside_reply = listen(&inside);
    assert!(
        matches!(
            inside_reply.outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
                resource_kind: crate::ResourceKindV1::SocketListener,
                ..
            })
        ),
        "in-root Unix listen did not acquire a listener: {:?}",
        inside_reply.outcome
    );
    assert!(inside_reply.resource_token.is_some());
    assert!(
        std::fs::symlink_metadata(&inside)
            .unwrap()
            .file_type()
            .is_socket()
    );
}
