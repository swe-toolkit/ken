use super::*;
use std::io::Read;

struct ProcessBackend;
impl HostEffectBackendV1 for ProcessBackend {
    fn console_write(&mut self, _: ConsoleStreamV1, _: &[u8]) -> Result<(), IoErrorIdentityV1> {
        Err(IoErrorIdentityV1::Unsupported)
    }
    fn console_flush(&mut self, _: ConsoleStreamV1) -> Result<(), IoErrorIdentityV1> {
        Err(IoErrorIdentityV1::Unsupported)
    }
    fn console_is_terminal(&mut self, _: ConsoleStreamV1) -> bool { false }
    fn fs_read_file(&mut self, _: &CapabilityGrantV1, _: &[u8]) -> Result<Vec<u8>, FileErrorCauseV1> {
        Err(FileErrorCauseV1::Io(IoErrorIdentityV1::Unsupported))
    }
    fn fs_write_file(&mut self, _: &CapabilityGrantV1, _: &[u8], _: CreatePolicyV1, _: &[u8])
        -> Result<(), FileErrorCauseV1> {
        Err(FileErrorCauseV1::Io(IoErrorIdentityV1::Unsupported))
    }
}

// Promise class: durable invariant. MEASURED: the shared production dispatcher
// rejects a full-FS token without EXECUTE and, with a declared grant, exposes
// stdout bytes and a typed cached exit via its real glibc backend. CLAIMED:
// Process declaration adds spawn authority without changing existing AFull.
// THE GAP: the public Ken prelude and native C path need independent end-to-end
// witnesses; this component test alone does not establish either one.
#[test]
fn process_dispatch_requires_declared_execute_and_releases_child_pipes_before_wait() {
    let mut backend = ProcessBackend;
    let mut revocation = crate::RevocationDomain::default();
    let mut capabilities = CapabilityTableV1::default();
    let plain = capabilities.insert(CapabilityGrantV1::mint_root(
        program_caps_fs_trace_identity_v1(),
        crate::Cap::mint(crate::AUTH_FULL, "FS"), &mut revocation,
    ));
    let declared = capabilities.insert(CapabilityGrantV1::mint_root(
        program_caps_fs_trace_identity_v1(),
        crate::Cap::mint(crate::AUTH_FULL, "FS").with_declared_process_grant(), &mut revocation,
    ));
    let mut resources = ResourceTableV1::default();
    let open = CanonicalRequestV1::ProcessPlanOpen { program: b"/bin/echo".to_vec() };
    macro_rules! dispatch {
        ($backend:expr, $resources:expr, $op:expr, $cap:expr, $input:expr, $request:expr) => {
            dispatch_host_op_v1($backend, &capabilities, &revocation, $resources,
                                $op, $cap, $input, $request).unwrap()
        };
    }
    let denied = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessPlanOpen,
        Some(plain), ResourceInputsV1::None, &open);
    assert_eq!(denied.outcome, CanonicalOutcomeV1::Error(SemanticErrorV1::Io(
        IoErrorIdentityV1::CapabilityDenied)));
    assert!(denied.resource_token.is_none());

    let accepted = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessPlanOpen,
        Some(declared), ResourceInputsV1::None, &open);
    assert!(matches!(accepted.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::ResourceAcquired { resource_kind: ResourceKindV1::SpawnPlan, .. })));
    let plan = accepted.resource_token.unwrap();
    for argument in [b"caller-zero".as_slice(), b"PX10-ALIVE".as_slice()] {
        let pushed = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessPlanPush,
            None, ResourceInputsV1::Target(plan),
            &CanonicalRequestV1::ProcessPlanPush {
                field: 0, first: argument.to_vec(), second: Vec::new(), scalar: 0,
            });
        assert_eq!(pushed.outcome, CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit));
    }
    let pushed = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessPlanPush,
        None, ResourceInputsV1::Target(plan),
        &CanonicalRequestV1::ProcessPlanPush {
            field: 2, first: Vec::new(), second: Vec::new(), scalar: 1,
        });
    assert_eq!(pushed.outcome, CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit));

    let child_reply = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessSpawn,
        None, ResourceInputsV1::Target(plan), &CanonicalRequestV1::ProcessSpawn);
    assert!(matches!(child_reply.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::ResourceAcquired { resource_kind: ResourceKindV1::ChildProcess, .. })));
    let child = child_reply.resource_token.unwrap();
    let consumed = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessSpawn,
        None, ResourceInputsV1::Target(plan), &CanonicalRequestV1::ProcessSpawn);
    assert_eq!(consumed.outcome, CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
        ResourceErrorV1::Closed)));

    let pipe_reply = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessTakePipe,
        None, ResourceInputsV1::Target(child),
        &CanonicalRequestV1::ProcessTakePipe { fd: 1 });
    assert!(matches!(pipe_reply.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::ResourceAcquired { resource_kind: ResourceKindV1::PipeRead, .. })));
    let pipe = pipe_reply.resource_token.unwrap();
    let again = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessTakePipe,
        None, ResourceInputsV1::Target(child),
        &CanonicalRequestV1::ProcessTakePipe { fd: 1 });
    assert_eq!(again.outcome, CanonicalOutcomeV1::Error(SemanticErrorV1::Io(
        IoErrorIdentityV1::InvalidInput)));
    let buffer = dispatch!(&mut backend, &mut resources, HostOpV1::BufferAllocate,
        None, ResourceInputsV1::None, &CanonicalRequestV1::BufferAllocate { capacity: 64 })
        .resource_token.unwrap();
    let read = dispatch!(&mut backend, &mut resources, HostOpV1::ResourceStreamRead,
        None, ResourceInputsV1::FileBuffer { file: pipe, buffer },
        &CanonicalRequestV1::ResourceStreamRead { buffer_start: 0, length: 64 });
    assert!(matches!(read.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::ReadProgress(ReadProgressV1::ReadSome { .. }))));
    let frozen = dispatch!(&mut backend, &mut resources, HostOpV1::BufferFreeze,
        None, ResourceInputsV1::BufferSpanTarget { target: buffer, span_origin: buffer },
        &CanonicalRequestV1::BufferFreeze { start: 0, length: 11 });
    assert_eq!(frozen.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::Bytes(b"PX10-ALIVE\n".to_vec())));
    for _ in 0..2 {
        let waited = dispatch!(&mut backend, &mut resources, HostOpV1::ProcessWait,
            None, ResourceInputsV1::Target(child), &CanonicalRequestV1::ProcessWait);
        assert_eq!(waited.outcome, CanonicalOutcomeV1::Success(
            CanonicalReplyV1::ChildExit(crate::ChildExitV1::Exited(0))));
    }
    let settled = dispatch!(&mut backend, &mut resources, HostOpV1::ResourceRelease,
        None, ResourceInputsV1::Target(child), &CanonicalRequestV1::ResourceRelease);
    assert!(matches!(settled.outcome, CanonicalOutcomeV1::Success(
        CanonicalReplyV1::ResourceSettlement(ResourceSettlementObservationV1 {
            resource_kind: ResourceKindV1::ChildProcess,
            outcome: ResourceSettlementOutcomeV1::Released, ..
        }))));
    let closed = dispatch!(&mut backend, &mut resources, HostOpV1::ResourceStreamRead,
        None, ResourceInputsV1::FileBuffer { file: pipe, buffer },
        &CanonicalRequestV1::ResourceStreamRead { buffer_start: 0, length: 1 });
    assert_eq!(closed.outcome, CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
        ResourceErrorV1::Closed)));
    assert!(resources.finalize_all_with(&mut backend).iter().all(|settled|
        settled.resource_kind == ResourceKindV1::Buffer));
}

/// Promise class: normative compatibility vector for the seven public Linux
/// signal numbers; durable invariant for Target admission and held identity.
/// MEASURED: each signal through ProcessSignal returns Unit and a matching
/// waited Signaled exit; 0 and 4 terminal-refuse while the held child lives.
/// CLAIMED: only the seven numbers can be sent to a child resource by this
/// operation. THE GAP: a raw PID send to a different process needs the decoy
/// mutation; that is not implied by a matching exit on this child.
#[test]
fn process_signal_dispatches_seven_and_refuses_invalid_numbers() {
    let mut backend = ProcessBackend;
    for signal in [1_u64, 2, 3, 9, 10, 12, 15] {
        let mut revocation = crate::RevocationDomain::default();
        let provenance = revocation.mint_root();
        let mut plan = crate::SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
        for arg in [
            b"sh".as_slice(),
            b"-c".as_slice(),
            b"ulimit -c 0; printf x; exec sleep 30".as_slice(),
        ] {
            plan.push_argument(arg.to_vec()).unwrap();
        }
        plan.set_stdio(1, crate::ChildStdioV1::Pipe).unwrap();
        let mut child = plan.spawn().unwrap();
        let mut ready = [0_u8; 1];
        std::fs::File::from(child.take_pipe(1).unwrap())
            .read_exact(&mut ready)
            .unwrap();
        assert_eq!(ready, *b"x");
        let mut resources = ResourceTableV1::default();
        let (token, _) = resources.insert_child(child, provenance);
        let capabilities = CapabilityTableV1::default();
        let request = CanonicalRequestV1::ProcessSignal { signal };
        let dispatched = dispatch_host_op_v1(
            &mut backend,
            &capabilities,
            &revocation,
            &mut resources,
            HostOpV1::ProcessSignal,
            None,
            ResourceInputsV1::Target(token),
            &request,
        )
        .unwrap();
        assert_eq!(
            dispatched.outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit)
        );
        let waited = dispatch_host_op_v1(
            &mut backend,
            &capabilities,
            &revocation,
            &mut resources,
            HostOpV1::ProcessWait,
            None,
            ResourceInputsV1::Target(token),
            &CanonicalRequestV1::ProcessWait,
        )
        .unwrap();
        assert_eq!(
            waited.outcome,
            CanonicalOutcomeV1::Success(CanonicalReplyV1::ChildExit(crate::ChildExitV1::Signaled(
                signal as u8
            )))
        );
        let cached = dispatch_host_op_v1(
            &mut backend,
            &capabilities,
            &revocation,
            &mut resources,
            HostOpV1::ProcessSignal,
            None,
            ResourceInputsV1::Target(token),
            &request,
        )
        .unwrap();
        assert_eq!(
            cached.outcome,
            CanonicalOutcomeV1::Error(SemanticErrorV1::Io(IoErrorIdentityV1::Other(libc::ESRCH)))
        );
        resources.finalize_all_with(&mut backend);
    }

    for invalid in [0_u64, 4] {
        let mut revocation = crate::RevocationDomain::default();
        let provenance = revocation.mint_root();
        let mut plan = crate::SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
        for arg in [
            b"sh".as_slice(),
            b"-c".as_slice(),
            b"printf x; exec sleep 30".as_slice(),
        ] {
            plan.push_argument(arg.to_vec()).unwrap();
        }
        plan.set_stdio(1, crate::ChildStdioV1::Pipe).unwrap();
        let mut child = plan.spawn().unwrap();
        let mut ready = [0];
        std::fs::File::from(child.take_pipe(1).unwrap())
            .read_exact(&mut ready)
            .unwrap();
        let mut resources = ResourceTableV1::default();
        let (token, _) = resources.insert_child(child, provenance);
        let capabilities = CapabilityTableV1::default();
        let denied = dispatch_host_op_v1(
            &mut backend,
            &capabilities,
            &revocation,
            &mut resources,
            HostOpV1::ProcessSignal,
            None,
            ResourceInputsV1::Target(token),
            &CanonicalRequestV1::ProcessSignal { signal: invalid },
        );
        assert!(matches!(
            denied,
            Err(TerminalErrorV1::MalformedHostAbiField)
        ));
        let (owner, _, _) = resources
            .typed_owner_mut(token, ResourceKindV1::ChildProcess)
            .unwrap();
        let ResourceOwnerV1::Child(child) = owner else {
            unreachable!()
        };
        assert_child_lives_before_cleanup(&mut child.process);
        child.process.send_signal(libc::SIGKILL).unwrap();
        assert_eq!(
            child.process.wait().unwrap(),
            crate::ChildExitV1::Signaled(9),
            "invalid signal {invalid} reached child before refusal"
        );
        resources.finalize_all_with(&mut backend);
    }
}

// Independent liveness observation before cleanup: an immediate SIGKILL can
// mask a forbidden prior delivery in the eventual wait result. The process
// still held by a pidfd; test observation uses that same stable identity.
fn assert_child_lives_before_cleanup(child: &mut crate::ChildProcessV1) {
    let exited = crate::process_v1::child_pidfd_exited_for_test(child, 250).unwrap();
    if exited {
        let _ = child.send_signal(libc::SIGKILL);
        let _ = child.wait();
    }
    assert!(!exited, "signal reached child before refusal");
}

/// Promise class: durable invariant. A revoked Target refuses signal delivery
/// before the host leaf, while settlement retires the child resource as Closed.
#[test]
fn process_signal_revoked_target_keeps_child_alive_and_settled_target_is_closed() {
    let mut backend = ProcessBackend;
    let mut revocation = crate::RevocationDomain::default();
    let provenance = revocation.mint_root();
    let mut plan = crate::SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    for arg in [
        b"sh".as_slice(),
        b"-c".as_slice(),
        b"printf x; exec sleep 30".as_slice(),
    ] {
        plan.push_argument(arg.to_vec()).unwrap();
    }
    plan.set_stdio(1, crate::ChildStdioV1::Pipe).unwrap();
    let mut child = plan.spawn().unwrap();
    let mut ready = [0];
    std::fs::File::from(child.take_pipe(1).unwrap())
        .read_exact(&mut ready)
        .unwrap();
    let mut resources = ResourceTableV1::default();
    let (token, _) = resources.insert_child(child, provenance);
    assert!(revocation.revoke(provenance));
    let capabilities = CapabilityTableV1::default();
    let signal = CanonicalRequestV1::ProcessSignal { signal: 15 };
    let denied = dispatch_host_op_v1(
        &mut backend,
        &capabilities,
        &revocation,
        &mut resources,
        HostOpV1::ProcessSignal,
        None,
        ResourceInputsV1::Target(token),
        &signal,
    )
    .unwrap();
    assert_eq!(
        denied.outcome,
        CanonicalOutcomeV1::Error(SemanticErrorV1::Io(IoErrorIdentityV1::Revoked))
    );
    let (owner, _, _) = resources
        .typed_owner_mut(token, ResourceKindV1::ChildProcess)
        .unwrap();
    let ResourceOwnerV1::Child(child) = owner else {
        unreachable!()
    };
    assert_child_lives_before_cleanup(&mut child.process);
    child.process.send_signal(libc::SIGKILL).unwrap();
    assert_eq!(
        child.process.wait().unwrap(),
        crate::ChildExitV1::Signaled(9),
        "revoked signal reached the live child"
    );
    resources.finalize_all_with(&mut backend);
    let closed = dispatch_host_op_v1(
        &mut backend,
        &capabilities,
        &revocation,
        &mut resources,
        HostOpV1::ProcessSignal,
        None,
        ResourceInputsV1::Target(token),
        &signal,
    )
    .unwrap();
    assert_eq!(
        closed.outcome,
        CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(ResourceErrorV1::Closed))
    );
}
