//! Total private wire codec for one completed linked-artifact effect trace.

use crate::{
    CanonicalOutcomeV1, CanonicalReplyV1, CanonicalRequestV1, CapabilityDeniedV1,
    CapabilityTraceIdentity, ConsoleStreamV1, CreatePolicyV1, DirEntryV1, EffectEvent,
    FdInheritancePolicyV1, FileErrorCauseV1, FileErrorIdentityV1, FileMetadataV1,
    FsCapabilityOperationV1, FsNodeKindV1, FsOpenModeV1, FsSeekFromV1, FsSyncModeV1, HostOpV1,
    IoErrorIdentityV1, MappingProtectionV1, ResourceBindingRole, ResourceErrorV1, ResourceKindV1,
    ResourceSettlementObservationV1, ResourceSettlementOutcomeV1, ResourceTraceIdentityV1,
    SemanticErrorV1, TerminalExitClass,
};

const MAGIC: &[u8; 8] = b"KETRACE3";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkedEffectTrace {
    pub plan_hash: u64,
    pub target_abi_hash: [u8; 32],
    pub host_effect_abi_hash: [u8; 32],
    pub terminal_value: i64,
    pub terminal_error: Option<crate::TerminalErrorV1>,
    pub effect_trace: Vec<EffectEvent>,
    pub terminal_exit: TerminalExitClass,
    /// Both regions' live counts when the generated entry returned, or `None`
    /// when no activation ran (a startup or pre-launch terminal).
    pub region_end: Option<ActivationRegionEndV1>,
}

/// One boundary region's live counts, read from its published header. Every
/// emitted store to a count adds a non-negative amount, so a count read at the
/// end of an activation is also that activation's high-water mark.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegionCountsV1 {
    pub nodes: u64,
    pub words: u64,
    pub data_bytes: u64,
    pub native_int_limbs: u64,
}

/// The invocation and persistent regions' counts at activation end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActivationRegionEndV1 {
    pub invocation: RegionCountsV1,
    pub persistent: RegionCountsV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectTraceWireError;

fn put_role(out: &mut Vec<u8>, role: ResourceBindingRole) {
    put_u8(
        out,
        match role {
            ResourceBindingRole::File => 0,
            ResourceBindingRole::Buffer => 1,
            ResourceBindingRole::Target => 2,
        },
    );
}

fn put_terminal_exit(out: &mut Vec<u8>, class: TerminalExitClass) {
    put_u8(
        out,
        match class {
            TerminalExitClass::NormalReturn => 0,
            TerminalExitClass::ReturnedError => 1,
            TerminalExitClass::ControlledTrap => 2,
        },
    );
}

fn put_u8(out: &mut Vec<u8>, value: u8) {
    out.push(value);
}

fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_i64(out: &mut Vec<u8>, value: i64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_bytes(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), EffectTraceWireError> {
    put_u64(
        out,
        u64::try_from(bytes.len()).map_err(|_| EffectTraceWireError)?,
    );
    out.extend_from_slice(bytes);
    Ok(())
}

fn put_string(out: &mut Vec<u8>, value: &str) -> Result<(), EffectTraceWireError> {
    put_bytes(out, value.as_bytes())
}

fn stream_tag(stream: ConsoleStreamV1) -> u8 {
    match stream {
        ConsoleStreamV1::Stdin => 0,
        ConsoleStreamV1::Stdout => 1,
        ConsoleStreamV1::Stderr => 2,
    }
}

fn inheritance_policy_tag(policy: FdInheritancePolicyV1) -> u8 {
    match policy {
        FdInheritancePolicyV1::Inherit => 0,
        FdInheritancePolicyV1::CloseOnExec => 1,
    }
}

fn put_mapping_protection(out: &mut Vec<u8>, protection: MappingProtectionV1) {
    match protection {
        MappingProtectionV1::ReadOnly => put_u8(out, 0),
        MappingProtectionV1::Writable => put_u8(out, 1),
    }
}

fn put_request(
    out: &mut Vec<u8>,
    request: &CanonicalRequestV1,
) -> Result<(), EffectTraceWireError> {
    match request {
        CanonicalRequestV1::ConsoleRead { stream, limit } => {
            put_u8(out, 0);
            put_u8(out, stream_tag(*stream));
            put_u64(out, *limit);
        }
        CanonicalRequestV1::ConsoleWrite { stream, bytes } => {
            put_u8(out, 1);
            put_u8(out, stream_tag(*stream));
            put_bytes(out, bytes)?;
        }
        CanonicalRequestV1::ConsoleFlush { stream } => {
            put_u8(out, 2);
            put_u8(out, stream_tag(*stream));
        }
        CanonicalRequestV1::ConsoleIsTerminal { stream } => {
            put_u8(out, 3);
            put_u8(out, stream_tag(*stream));
        }
        CanonicalRequestV1::ClockWallNow => put_u8(out, 4),
        CanonicalRequestV1::ClockMonotonicNow => put_u8(out, 22),
        CanonicalRequestV1::ClockSleepUntil { deadline } => {
            put_u8(out, 23);
            put_u64(out, *deadline);
        }
        CanonicalRequestV1::EntropyRandomBytes { count } => {
            put_u8(out, 24);
            put_u64(out, *count);
        }
        CanonicalRequestV1::FsReadFile { path } => {
            put_u8(out, 5);
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsWriteFile {
            path,
            create_policy,
            bytes,
        } => {
            put_u8(out, 6);
            put_bytes(out, path)?;
            put_u8(
                out,
                match create_policy {
                    CreatePolicyV1::CreateNew => 0,
                    CreatePolicyV1::CreateOrTruncate => 1,
                    CreatePolicyV1::CreateOrKeep => 2,
                },
            );
            put_bytes(out, bytes)?;
        }
        CanonicalRequestV1::FsAppendFile { path, bytes } => {
            put_u8(out, 7);
            put_bytes(out, path)?;
            put_bytes(out, bytes)?;
        }
        CanonicalRequestV1::FsMetadata { path } => {
            put_u8(out, 8);
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsOpenDirectory { path } => {
            put_u8(out, 50);
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsReadDirectoryBatch => put_u8(out, 51),
        CanonicalRequestV1::FsCreateDirectory { path } => {
            put_u8(out, 10);
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsRemoveFile { path } => {
            put_u8(out, 11);
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsRemoveDirectory { recursive, path } => {
            put_u8(out, 12);
            put_u8(out, u8::from(*recursive));
            put_bytes(out, path)?;
        }
        CanonicalRequestV1::FsRename {
            source,
            destination,
        } => {
            put_u8(out, 13);
            put_bytes(out, source)?;
            put_bytes(out, destination)?;
        }
        CanonicalRequestV1::FsChangeMode { path, mode } => {
            put_u8(out, 14);
            put_bytes(out, path)?;
            put_u16(out, *mode);
        }
        CanonicalRequestV1::FsOpen { path, mode } => {
            put_u8(out, 15);
            put_bytes(out, path)?;
            put_u8(
                out,
                match mode {
                    FsOpenModeV1::Read => 0,
                    FsOpenModeV1::Metadata => 1,
                    FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateNew) => 2,
                    FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateOrTruncate) => 3,
                    FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateOrKeep) => 4,
                },
            );
        }
        CanonicalRequestV1::FsHandleMetadata => put_u8(out, 16),
        CanonicalRequestV1::ResourceRelease => put_u8(out, 17),
        CanonicalRequestV1::FsReadAt {
            file_offset,
            buffer_start,
            length,
        } => {
            put_u8(out, 18);
            put_u64(out, *file_offset);
            put_u64(out, *buffer_start);
            put_u64(out, *length);
        }
        CanonicalRequestV1::FsWriteAt {
            file_offset,
            buffer_start,
            length,
        } => {
            put_u8(out, 19);
            put_u64(out, *file_offset);
            put_u64(out, *buffer_start);
            put_u64(out, *length);
        }
        CanonicalRequestV1::FsSeek { from } => {
            put_u8(out, 25);
            match from {
                FsSeekFromV1::Start(offset) => {
                    put_u8(out, 0);
                    put_u64(out, *offset);
                }
                FsSeekFromV1::Current(offset) => {
                    put_u8(out, 1);
                    put_i64(out, *offset);
                }
                FsSeekFromV1::End(offset) => {
                    put_u8(out, 2);
                    put_i64(out, *offset);
                }
            }
        }
        CanonicalRequestV1::FsSetLength { length } => {
            put_u8(out, 26);
            put_u64(out, *length);
        }
        CanonicalRequestV1::FsSync { mode } => {
            put_u8(out, 27);
            put_u8(
                out,
                match mode {
                    FsSyncModeV1::SyncFull => 0,
                    FsSyncModeV1::SyncData => 1,
                },
            );
        }
        CanonicalRequestV1::FsGetInheritance => put_u8(out, 28),
        CanonicalRequestV1::FsSetInheritance { policy } => {
            put_u8(out, 29);
            put_u8(out, inheritance_policy_tag(*policy));
        }
        CanonicalRequestV1::FsDuplicate { policy } => {
            put_u8(out, 30);
            put_u8(out, inheritance_policy_tag(*policy));
        }
        CanonicalRequestV1::BufferAllocate { capacity } => {
            put_u8(out, 20);
            put_u64(out, *capacity);
        }
        CanonicalRequestV1::BufferFreeze { start, length } => {
            put_u8(out, 21);
            put_u64(out, *start);
            put_u64(out, *length);
        }
        CanonicalRequestV1::MappingAllocate { length, protection } => {
            put_u8(out, 31);
            put_u64(out, *length);
            put_mapping_protection(out, *protection);
        }
        CanonicalRequestV1::MappingReadView { start, length } => {
            put_u8(out, 32);
            put_u64(out, *start);
            put_u64(out, *length);
        }
        CanonicalRequestV1::MappingWriteView { start, bytes } => {
            put_u8(out, 33);
            put_u64(out, *start);
            put_bytes(out, bytes)?;
        }
        CanonicalRequestV1::MappingAcquireFile { length, protection } => {
            put_u8(out, 34);
            put_u64(out, *length);
            put_mapping_protection(out, *protection);
        }
        CanonicalRequestV1::ResourceStreamRead { buffer_start, length } => {
            put_u8(out, 35); put_u64(out, *buffer_start); put_u64(out, *length);
        }
        CanonicalRequestV1::ResourceStreamWrite { buffer_start, length } => {
            put_u8(out, 36); put_u64(out, *buffer_start); put_u64(out, *length);
        }
        CanonicalRequestV1::ProcessPlanOpen { program } => { put_u8(out, 37); put_bytes(out, program)?; }
        CanonicalRequestV1::ProcessPlanPush { field, first, second, scalar } => {
            put_u8(out, 38); put_u64(out, *field); put_bytes(out, first)?;
            put_bytes(out, second)?; put_u64(out, *scalar);
        }
        CanonicalRequestV1::ProcessSpawn => put_u8(out, 39),
        CanonicalRequestV1::ProcessTakePipe { fd } => { put_u8(out, 40); put_u64(out, *fd); }
        CanonicalRequestV1::ProcessWait => put_u8(out, 41),
        CanonicalRequestV1::ProcessSignal { signal } => { put_u8(out, 42); put_u64(out, *signal); },
        CanonicalRequestV1::NetListen { address } => { put_u8(out, 43); put_socket_address(out, address)?; }
        CanonicalRequestV1::NetAccept => put_u8(out, 44),
        CanonicalRequestV1::NetConnect { address } => { put_u8(out, 45); put_socket_address(out, address)?; }
        CanonicalRequestV1::NetReceive { buffer_start, length } => { put_u8(out, 46); put_u64(out, *buffer_start); put_u64(out, *length); }
        CanonicalRequestV1::NetSend { buffer_start, length } => { put_u8(out, 47); put_u64(out, *buffer_start); put_u64(out, *length); }
        CanonicalRequestV1::NetShutdown { how } => { put_u8(out, 48); put_u64(out, *how); }
        CanonicalRequestV1::NetLocalAddress => put_u8(out, 49),
    }
    Ok(())
}

fn put_socket_address(out: &mut Vec<u8>, address: &crate::SocketAddressV1) -> Result<(), EffectTraceWireError> {
    address.validate().map_err(|_| EffectTraceWireError)?;
    match address {
        crate::SocketAddressV1::Inet4 { octets, port } => {
            put_u8(out, 0);
            out.extend_from_slice(octets);
            put_u16(out, *port);
        }
        crate::SocketAddressV1::Inet6 { octets, port } => {
            put_u8(out, 1);
            out.extend_from_slice(octets);
            put_u16(out, *port);
        }
        crate::SocketAddressV1::Unix { path } => {
            put_u8(out, 2);
            put_bytes(out, path)?;
        }
    }
    Ok(())
}

fn put_node_kind(out: &mut Vec<u8>, kind: FsNodeKindV1) {
    put_u8(
        out,
        match kind {
            FsNodeKindV1::File => 0,
            FsNodeKindV1::Directory => 1,
            FsNodeKindV1::Symlink => 2,
            FsNodeKindV1::Other => 3,
        },
    );
}

fn put_option<T>(out: &mut Vec<u8>, value: Option<T>, put: impl FnOnce(&mut Vec<u8>, T)) {
    put_u8(out, u8::from(value.is_some()));
    if let Some(value) = value {
        put(out, value);
    }
}

fn put_reply(out: &mut Vec<u8>, reply: &CanonicalReplyV1) -> Result<(), EffectTraceWireError> {
    match reply {
        CanonicalReplyV1::Unit => put_u8(out, 0),
        CanonicalReplyV1::Bool(value) => {
            put_u8(out, 1);
            put_u8(out, u8::from(*value));
        }
        CanonicalReplyV1::Bytes(bytes) => {
            put_u8(out, 2);
            put_bytes(out, bytes)?;
        }
        CanonicalReplyV1::ReadChunk(bytes) => {
            put_u8(out, 3);
            put_bytes(out, bytes)?;
        }
        CanonicalReplyV1::ReadEof => put_u8(out, 4),
        CanonicalReplyV1::Instant(bytes) => {
            put_u8(out, 5);
            put_bytes(out, bytes)?;
        }
        CanonicalReplyV1::MonotonicInstant(bytes) => {
            put_u8(out, 12);
            put_bytes(out, bytes)?;
        }
        CanonicalReplyV1::FileMetadata(metadata) => {
            put_u8(out, 6);
            put_u64(out, metadata.size);
            put_node_kind(out, metadata.kind);
            if metadata.mode.is_some_and(|mode| mode > 0o7777) {
                return Err(EffectTraceWireError);
            }
            put_option(out, metadata.mode, put_u64);
            put_option(out, metadata.links, put_u64);
            put_u64(out, metadata.device_major);
            put_u64(out, metadata.device_minor);
            put_option(out, metadata.inode, put_u64);
            for time in [
                metadata.modified,
                metadata.changed,
                metadata.accessed,
                metadata.born,
            ] {
                put_option(out, time, |out, value| out.extend(value.to_be_bytes()));
            }
        }
        CanonicalReplyV1::DirectoryEnd => put_u8(out, 17),
        CanonicalReplyV1::DirectoryEntries(entries) => {
            if entries.is_empty() {
                return Err(EffectTraceWireError);
            }
            put_u8(out, 7);
            put_u64(
                out,
                u64::try_from(entries.len()).map_err(|_| EffectTraceWireError)?,
            );
            for entry in entries {
                put_bytes(out, &entry.name)?;
                put_node_kind(out, entry.kind);
            }
        }
        CanonicalReplyV1::ResourceAcquired {
            schema_version,
            resource_kind,
            identity,
        } => {
            put_u8(out, 8);
            put_u16(out, *schema_version);
            put_resource_kind(out, *resource_kind);
            put_u64(out, identity.0);
        }
        CanonicalReplyV1::ResourceSettlement(settlement) => {
            put_u8(out, 9);
            put_settlement(out, settlement);
        }
        CanonicalReplyV1::ReadProgress(progress) => {
            put_u8(out, 10);
            match progress {
                crate::ReadProgressV1::ReadSome { span, transferred } => {
                    put_u8(out, 0);
                    put_u64(out, span.start());
                    put_u64(out, span.length());
                    put_u64(out, transferred.get());
                    put_u64(out, transferred.effective_request());
                }
                crate::ReadProgressV1::ReadEof => put_u8(out, 1),
            }
        }
        CanonicalReplyV1::WriteProgress(crate::WriteProgressV1::Wrote(transferred)) => {
            put_u8(out, 11);
            put_u64(out, transferred.get());
            put_u64(out, transferred.effective_request());
        }
        CanonicalReplyV1::SocketAddress(address) => {
            put_u8(out, 16);
            put_socket_address(out, address)?;
        }
        CanonicalReplyV1::ChildExit(exit) => {
            put_u8(out, 15);
            match exit {
                crate::ChildExitV1::Exited(code) => { put_u8(out, 0); put_u8(out, *code); }
                crate::ChildExitV1::Signaled(signal) => { put_u8(out, 1); put_u8(out, *signal); }
            }
        }
        CanonicalReplyV1::FilePosition(position) => {
            put_u8(out, 13);
            put_u64(out, *position);
        }
        CanonicalReplyV1::FdInheritancePolicy(policy) => {
            put_u8(out, 14);
            put_u8(out, inheritance_policy_tag(*policy));
        }
    }
    Ok(())
}

fn put_resource_kind(out: &mut Vec<u8>, kind: ResourceKindV1) {
    match kind {
        ResourceKindV1::FsHandle => put_u8(out, 0),
        ResourceKindV1::Buffer => put_u8(out, 1),
        ResourceKindV1::Mapping => put_u8(out, 2),
        ResourceKindV1::SpawnPlan => put_u8(out, 3),
        ResourceKindV1::ChildProcess => put_u8(out, 4),
        ResourceKindV1::PipeRead => put_u8(out, 5),
        ResourceKindV1::PipeWrite => put_u8(out, 6),
        ResourceKindV1::SocketListener => put_u8(out, 7),
        ResourceKindV1::SocketStream => put_u8(out, 8),
    }
}

fn put_settlement(out: &mut Vec<u8>, settlement: &ResourceSettlementObservationV1) {
    put_u16(out, settlement.schema_version);
    put_resource_kind(out, settlement.resource_kind);
    put_u64(out, settlement.identity.0);
    match settlement.outcome {
        ResourceSettlementOutcomeV1::Released => put_u8(out, 0),
        ResourceSettlementOutcomeV1::ReleaseFailed(io) => {
            put_u8(out, 1);
            put_io_error(out, io);
        }
    }
}

fn put_io_error(out: &mut Vec<u8>, error: IoErrorIdentityV1) {
    let (tag, raw) = match error {
        IoErrorIdentityV1::NotFound => (0, None),
        IoErrorIdentityV1::PermissionDenied => (1, None),
        IoErrorIdentityV1::CapabilityDenied => (12, None),
        IoErrorIdentityV1::BrokenPipe => (2, None),
        IoErrorIdentityV1::Interrupted => (3, None),
        IoErrorIdentityV1::AlreadyExists => (4, None),
        IoErrorIdentityV1::InvalidInput => (5, None),
        IoErrorIdentityV1::IsDirectory => (6, None),
        IoErrorIdentityV1::NotDirectory => (7, None),
        IoErrorIdentityV1::NotEmpty => (8, None),
        IoErrorIdentityV1::Unsupported => (9, None),
        IoErrorIdentityV1::Other(raw) => (10, Some(raw)),
        IoErrorIdentityV1::Revoked => (11, None),
    };
    put_u8(out, tag);
    if let Some(raw) = raw {
        put_i32(out, raw);
    }
}

fn fs_operation_tag(operation: FsCapabilityOperationV1) -> u8 {
    match operation {
        FsCapabilityOperationV1::Read => 0,
        FsCapabilityOperationV1::Write => 1,
        FsCapabilityOperationV1::Append => 2,
        FsCapabilityOperationV1::Metadata => 3,
        FsCapabilityOperationV1::Enumerate => 4,
        FsCapabilityOperationV1::CreateDirectory => 5,
        FsCapabilityOperationV1::RemoveFile => 6,
        FsCapabilityOperationV1::RemoveDirectory => 7,
        FsCapabilityOperationV1::RenameSource => 8,
        FsCapabilityOperationV1::RenameDestination => 9,
        FsCapabilityOperationV1::ChangeMode => 10,
        FsCapabilityOperationV1::Seek => 11,
        FsCapabilityOperationV1::SetLength => 12,
        FsCapabilityOperationV1::Sync => 13,
        FsCapabilityOperationV1::GetInheritance => 14,
        FsCapabilityOperationV1::SetInheritance => 15,
        FsCapabilityOperationV1::Duplicate => 16,
        FsCapabilityOperationV1::Spawn => 17,
    }
}

fn put_denial(out: &mut Vec<u8>, denial: &CapabilityDeniedV1) {
    match denial {
        CapabilityDeniedV1::RightNotHeld {
            operation,
            held_rights,
        } => {
            put_u8(out, 0);
            put_u8(out, fs_operation_tag(*operation));
            put_u8(out, *held_rights);
        }
        CapabilityDeniedV1::AuthorityInsufficient => put_u8(out, 1),
        CapabilityDeniedV1::ScopeEscape => put_u8(out, 2),
        CapabilityDeniedV1::SymlinkDenied => put_u8(out, 3),
        CapabilityDeniedV1::MalformedCapability => put_u8(out, 4),
    }
}

fn put_cause(out: &mut Vec<u8>, cause: &FileErrorCauseV1) {
    match cause {
        FileErrorCauseV1::Io(error) => {
            put_u8(out, 0);
            put_io_error(out, *error);
        }
        FileErrorCauseV1::Capability(error) => {
            put_u8(out, 1);
            put_denial(out, error);
        }
    }
}

fn put_error(out: &mut Vec<u8>, error: &SemanticErrorV1) -> Result<(), EffectTraceWireError> {
    match error {
        SemanticErrorV1::Io(error) => {
            put_u8(out, 0);
            put_io_error(out, *error);
        }
        SemanticErrorV1::File(error) => {
            put_u8(out, 1);
            put_u16(out, error.operation as u16);
            put_bytes(out, &error.relative_path)?;
            put_cause(out, &error.cause);
        }
        SemanticErrorV1::Capability(error) => {
            put_u8(out, 2);
            put_denial(out, error);
        }
        SemanticErrorV1::Resource(error) => {
            put_u8(out, 3);
            match error {
                ResourceErrorV1::Closed => put_u8(out, 0),
                ResourceErrorV1::MalformedResource => put_u8(out, 1),
                ResourceErrorV1::RightNotHeld { required, held } => {
                    put_u8(out, 2);
                    put_u8(out, *required);
                    put_u8(out, *held);
                }
                ResourceErrorV1::ReleaseFailed {
                    schema_version,
                    resource_kind,
                    identity,
                    io,
                } => {
                    put_u8(out, 3);
                    put_u16(out, *schema_version);
                    put_resource_kind(out, *resource_kind);
                    put_u64(out, identity.0);
                    put_io_error(out, *io);
                }
                ResourceErrorV1::ResourceKindMismatch { expected, actual } => {
                    put_u8(out, 4);
                    put_resource_kind(out, *expected);
                    put_resource_kind(out, *actual);
                }
                ResourceErrorV1::BufferLimit => put_u8(out, 5),
                ResourceErrorV1::InvalidOffset => put_u8(out, 6),
                ResourceErrorV1::InvalidBounds => put_u8(out, 7),
                ResourceErrorV1::NoProgress => put_u8(out, 8),
                ResourceErrorV1::AllocationFailed => put_u8(out, 9),
                ResourceErrorV1::MappingLimit => put_u8(out, 10),
            }
        }
    }
    Ok(())
}

fn capacity_scope_resource_is_valid(
    scope: crate::CapacityScopeV1,
    resource: crate::CapacityResourceV1,
) -> bool {
    use crate::{CapacityResourceV1 as Resource, CapacityScopeV1 as Scope};
    match (scope, resource) {
        (Scope::Runtime, Resource::InvocationEpochs)
        | (Scope::Invocation, Resource::EventGenerations | Resource::LivePendingSlots)
        | (
            Scope::Invocation | Scope::Persistent,
            Resource::Nodes | Resource::Words | Resource::DataBytes | Resource::NativeIntLimbs,
        ) => true,
        _ => false,
    }
}

pub fn encode_linked_effect_trace(
    trace: &LinkedEffectTrace,
) -> Result<Vec<u8>, EffectTraceWireError> {
    if trace.terminal_exit
        != crate::terminal_exit_class(trace.terminal_value, trace.terminal_error.as_ref())
    {
        return Err(EffectTraceWireError);
    }
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    put_u64(&mut out, trace.plan_hash);
    out.extend_from_slice(&trace.target_abi_hash);
    out.extend_from_slice(&trace.host_effect_abi_hash);
    put_i64(&mut out, trace.terminal_value);
    match &trace.terminal_error {
        None => put_u8(&mut out, 0),
        Some(crate::TerminalErrorV1::RootExecutionDenied) => put_u8(&mut out, 1),
        Some(crate::TerminalErrorV1::SelectedCallIntegrity(fault)) => {
            if trace.terminal_value != crate::SELECTED_CALL_INTEGRITY_STATUS_V1 {
                return Err(EffectTraceWireError);
            }
            put_u8(&mut out, 4);
            put_u8(
                &mut out,
                match fault {
                    crate::SelectedCallIntegrityFaultV1::WrongActivation => 0,
                    crate::SelectedCallIntegrityFaultV1::InvalidSlot => 1,
                    crate::SelectedCallIntegrityFaultV1::StaleGeneration => 2,
                    crate::SelectedCallIntegrityFaultV1::Spent => 3,
                    crate::SelectedCallIntegrityFaultV1::WrongTarget => 4,
                },
            );
        }
        Some(crate::TerminalErrorV1::CapacityExhausted(failure)) => {
            if !capacity_scope_resource_is_valid(failure.scope, failure.resource)
                || trace.terminal_value != crate::CAPACITY_EXHAUSTED_STATUS_V1
                || failure.requested <= failure.limit
            {
                return Err(EffectTraceWireError);
            }
            put_u8(&mut out, 3);
            put_u8(
                &mut out,
                match failure.scope {
                    crate::CapacityScopeV1::Runtime => 0,
                    crate::CapacityScopeV1::Invocation => 1,
                    crate::CapacityScopeV1::Persistent => 2,
                },
            );
            put_u8(
                &mut out,
                match failure.resource {
                    crate::CapacityResourceV1::InvocationEpochs => 0,
                    crate::CapacityResourceV1::EventGenerations => 1,
                    crate::CapacityResourceV1::LivePendingSlots => 2,
                    crate::CapacityResourceV1::Nodes => 3,
                    crate::CapacityResourceV1::Words => 4,
                    crate::CapacityResourceV1::DataBytes => 5,
                    crate::CapacityResourceV1::NativeIntLimbs => 6,
                },
            );
            out.extend_from_slice(&failure.limit.to_le_bytes());
            out.extend_from_slice(&failure.requested.to_le_bytes());
        }
        Some(crate::TerminalErrorV1::HomeRootResolutionFailed(failure)) => {
            put_u8(&mut out, 2);
            match failure {
                crate::HomeRootResolutionFailureV1::NoAccountRecord => put_u8(&mut out, 0),
                crate::HomeRootResolutionFailureV1::AccountRecordTooLarge => put_u8(&mut out, 1),
                crate::HomeRootResolutionFailureV1::AccountLookup(error) => {
                    put_u8(&mut out, 2);
                    put_io_error(&mut out, *error);
                }
                crate::HomeRootResolutionFailureV1::InvalidAccountRecord => put_u8(&mut out, 3),
                crate::HomeRootResolutionFailureV1::RootOpen(error) => {
                    put_u8(&mut out, 4);
                    put_io_error(&mut out, *error);
                }
                crate::HomeRootResolutionFailureV1::ScopeEscape => put_u8(&mut out, 5),
                crate::HomeRootResolutionFailureV1::SymlinkDenied => put_u8(&mut out, 6),
            }
        }
        Some(_) => return Err(EffectTraceWireError),
    }
    put_terminal_exit(&mut out, trace.terminal_exit);
    put_u64(
        &mut out,
        u64::try_from(trace.effect_trace.len()).map_err(|_| EffectTraceWireError)?,
    );
    for event in &trace.effect_trace {
        put_u64(&mut out, event.sequence);
        put_u16(&mut out, event.operation as u16);
        match &event.capability {
            None => put_u8(&mut out, 0),
            Some(identity) => {
                put_u8(&mut out, 1);
                put_string(&mut out, &identity.0)?;
            }
        }
        put_u64(
            &mut out,
            u64::try_from(event.resource_bindings.len()).map_err(|_| EffectTraceWireError)?,
        );
        for (role, identity) in &event.resource_bindings {
            put_role(&mut out, *role);
            put_u64(&mut out, identity.0);
        }
        put_request(&mut out, &event.request)?;
        match &event.outcome {
            CanonicalOutcomeV1::Success(reply) => {
                put_u8(&mut out, 0);
                put_reply(&mut out, reply)?;
            }
            CanonicalOutcomeV1::Error(error) => {
                put_u8(&mut out, 1);
                put_error(&mut out, error)?;
            }
        }
    }
    match trace.region_end {
        None => put_u8(&mut out, 0),
        Some(end) => {
            put_u8(&mut out, 1);
            for counts in [end.invocation, end.persistent] {
                put_u64(&mut out, counts.nodes);
                put_u64(&mut out, counts.words);
                put_u64(&mut out, counts.data_bytes);
                put_u64(&mut out, counts.native_int_limbs);
            }
        }
    }
    Ok(out)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> Cursor<'a> {
    fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], EffectTraceWireError> {
        let end = self.position.checked_add(len).ok_or(EffectTraceWireError)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(EffectTraceWireError)?;
        self.position = end;
        Ok(value)
    }
    fn u8(&mut self) -> Result<u8, EffectTraceWireError> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, EffectTraceWireError> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Result<u64, EffectTraceWireError> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn i64(&mut self) -> Result<i64, EffectTraceWireError> {
        Ok(i64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32, EffectTraceWireError> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn bytes(&mut self) -> Result<Vec<u8>, EffectTraceWireError> {
        let len = usize::try_from(self.u64()?).map_err(|_| EffectTraceWireError)?;
        Ok(self.take(len)?.to_vec())
    }
    fn string(&mut self) -> Result<String, EffectTraceWireError> {
        String::from_utf8(self.bytes()?).map_err(|_| EffectTraceWireError)
    }
}

fn get_inheritance_policy(
    cursor: &mut Cursor<'_>,
) -> Result<FdInheritancePolicyV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(FdInheritancePolicyV1::Inherit),
        1 => Ok(FdInheritancePolicyV1::CloseOnExec),
        _ => Err(EffectTraceWireError),
    }
}

fn get_stream(cursor: &mut Cursor<'_>) -> Result<ConsoleStreamV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(ConsoleStreamV1::Stdin),
        1 => Ok(ConsoleStreamV1::Stdout),
        2 => Ok(ConsoleStreamV1::Stderr),
        _ => Err(EffectTraceWireError),
    }
}
fn get_role(cursor: &mut Cursor<'_>) -> Result<ResourceBindingRole, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(ResourceBindingRole::File),
        1 => Ok(ResourceBindingRole::Buffer),
        2 => Ok(ResourceBindingRole::Target),
        _ => Err(EffectTraceWireError),
    }
}
fn get_terminal_exit(cursor: &mut Cursor<'_>) -> Result<TerminalExitClass, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(TerminalExitClass::NormalReturn),
        1 => Ok(TerminalExitClass::ReturnedError),
        2 => Ok(TerminalExitClass::ControlledTrap),
        _ => Err(EffectTraceWireError),
    }
}
fn get_bool(cursor: &mut Cursor<'_>) -> Result<bool, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(EffectTraceWireError),
    }
}
fn get_mapping_protection(
    cursor: &mut Cursor<'_>,
) -> Result<MappingProtectionV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(MappingProtectionV1::ReadOnly),
        1 => Ok(MappingProtectionV1::Writable),
        _ => Err(EffectTraceWireError),
    }
}
fn get_node_kind(cursor: &mut Cursor<'_>) -> Result<FsNodeKindV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(FsNodeKindV1::File),
        1 => Ok(FsNodeKindV1::Directory),
        2 => Ok(FsNodeKindV1::Symlink),
        3 => Ok(FsNodeKindV1::Other),
        _ => Err(EffectTraceWireError),
    }
}
fn get_request(cursor: &mut Cursor<'_>) -> Result<CanonicalRequestV1, EffectTraceWireError> {
    Ok(match cursor.u8()? {
        0 => CanonicalRequestV1::ConsoleRead {
            stream: get_stream(cursor)?,
            limit: cursor.u64()?,
        },
        1 => CanonicalRequestV1::ConsoleWrite {
            stream: get_stream(cursor)?,
            bytes: cursor.bytes()?,
        },
        2 => CanonicalRequestV1::ConsoleFlush {
            stream: get_stream(cursor)?,
        },
        3 => CanonicalRequestV1::ConsoleIsTerminal {
            stream: get_stream(cursor)?,
        },
        4 => CanonicalRequestV1::ClockWallNow,
        22 => CanonicalRequestV1::ClockMonotonicNow,
        23 => CanonicalRequestV1::ClockSleepUntil {
            deadline: cursor.u64()?,
        },
        24 => CanonicalRequestV1::EntropyRandomBytes {
            count: cursor.u64()?,
        },
        5 => CanonicalRequestV1::FsReadFile {
            path: cursor.bytes()?,
        },
        6 => {
            let path = cursor.bytes()?;
            let create_policy = match cursor.u8()? {
                0 => CreatePolicyV1::CreateNew,
                1 => CreatePolicyV1::CreateOrTruncate,
                2 => CreatePolicyV1::CreateOrKeep,
                _ => return Err(EffectTraceWireError),
            };
            CanonicalRequestV1::FsWriteFile {
                path,
                create_policy,
                bytes: cursor.bytes()?,
            }
        }
        7 => CanonicalRequestV1::FsAppendFile {
            path: cursor.bytes()?,
            bytes: cursor.bytes()?,
        },
        8 => CanonicalRequestV1::FsMetadata {
            path: cursor.bytes()?,
        },
        // Request tag 9 retired with the whole-directory operation.
        50 => CanonicalRequestV1::FsOpenDirectory { path: cursor.bytes()? },
        51 => CanonicalRequestV1::FsReadDirectoryBatch,
        10 => CanonicalRequestV1::FsCreateDirectory {
            path: cursor.bytes()?,
        },
        11 => CanonicalRequestV1::FsRemoveFile {
            path: cursor.bytes()?,
        },
        12 => CanonicalRequestV1::FsRemoveDirectory {
            recursive: get_bool(cursor)?,
            path: cursor.bytes()?,
        },
        13 => CanonicalRequestV1::FsRename {
            source: cursor.bytes()?,
            destination: cursor.bytes()?,
        },
        14 => CanonicalRequestV1::FsChangeMode {
            path: cursor.bytes()?,
            mode: cursor.u16()?,
        },
        15 => CanonicalRequestV1::FsOpen {
            path: cursor.bytes()?,
            mode: match cursor.u8()? {
                0 => FsOpenModeV1::Read,
                1 => FsOpenModeV1::Metadata,
                2 => FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateNew),
                3 => FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateOrTruncate),
                4 => FsOpenModeV1::WriteCreate(CreatePolicyV1::CreateOrKeep),
                _ => return Err(EffectTraceWireError),
            },
        },
        16 => CanonicalRequestV1::FsHandleMetadata,
        17 => CanonicalRequestV1::ResourceRelease,
        18 => CanonicalRequestV1::FsReadAt {
            file_offset: cursor.u64()?,
            buffer_start: cursor.u64()?,
            length: cursor.u64()?,
        },
        19 => CanonicalRequestV1::FsWriteAt {
            file_offset: cursor.u64()?,
            buffer_start: cursor.u64()?,
            length: cursor.u64()?,
        },
        25 => CanonicalRequestV1::FsSeek {
            from: match cursor.u8()? {
                0 => FsSeekFromV1::Start(cursor.u64()?),
                1 => FsSeekFromV1::Current(cursor.i64()?),
                2 => FsSeekFromV1::End(cursor.i64()?),
                _ => return Err(EffectTraceWireError),
            },
        },
        26 => CanonicalRequestV1::FsSetLength {
            length: cursor.u64()?,
        },
        27 => CanonicalRequestV1::FsSync {
            mode: match cursor.u8()? {
                0 => FsSyncModeV1::SyncFull,
                1 => FsSyncModeV1::SyncData,
                _ => return Err(EffectTraceWireError),
            },
        },
        28 => CanonicalRequestV1::FsGetInheritance,
        29 => CanonicalRequestV1::FsSetInheritance {
            policy: get_inheritance_policy(cursor)?,
        },
        30 => CanonicalRequestV1::FsDuplicate {
            policy: get_inheritance_policy(cursor)?,
        },
        20 => CanonicalRequestV1::BufferAllocate {
            capacity: cursor.u64()?,
        },
        21 => CanonicalRequestV1::BufferFreeze {
            start: cursor.u64()?,
            length: cursor.u64()?,
        },
        31 => CanonicalRequestV1::MappingAllocate {
            length: cursor.u64()?,
            protection: get_mapping_protection(cursor)?,
        },
        32 => CanonicalRequestV1::MappingReadView {
            start: cursor.u64()?,
            length: cursor.u64()?,
        },
        33 => CanonicalRequestV1::MappingWriteView {
            start: cursor.u64()?,
            bytes: cursor.bytes()?,
        },
        34 => CanonicalRequestV1::MappingAcquireFile {
            length: cursor.u64()?,
            protection: get_mapping_protection(cursor)?,
        },
        35 => CanonicalRequestV1::ResourceStreamRead { buffer_start: cursor.u64()?, length: cursor.u64()? },
        36 => CanonicalRequestV1::ResourceStreamWrite { buffer_start: cursor.u64()?, length: cursor.u64()? },
        37 => CanonicalRequestV1::ProcessPlanOpen { program: cursor.bytes()? },
        38 => CanonicalRequestV1::ProcessPlanPush { field: cursor.u64()?, first: cursor.bytes()?, second: cursor.bytes()?, scalar: cursor.u64()? },
        39 => CanonicalRequestV1::ProcessSpawn,
        40 => CanonicalRequestV1::ProcessTakePipe { fd: cursor.u64()? },
        41 => CanonicalRequestV1::ProcessWait,
        42 => CanonicalRequestV1::ProcessSignal { signal: cursor.u64()? },
        43 => CanonicalRequestV1::NetListen { address: get_socket_address(cursor)? },
        44 => CanonicalRequestV1::NetAccept,
        45 => CanonicalRequestV1::NetConnect { address: get_socket_address(cursor)? },
        46 => CanonicalRequestV1::NetReceive { buffer_start: cursor.u64()?, length: cursor.u64()? },
        47 => CanonicalRequestV1::NetSend { buffer_start: cursor.u64()?, length: cursor.u64()? },
        48 => CanonicalRequestV1::NetShutdown { how: cursor.u64()? },
        49 => CanonicalRequestV1::NetLocalAddress,
        _ => return Err(EffectTraceWireError),
    })
}

fn get_socket_address(cursor: &mut Cursor<'_>) -> Result<crate::SocketAddressV1, EffectTraceWireError> {
    let address = match cursor.u8()? {
        0 => crate::SocketAddressV1::Inet4 {
            octets: cursor.take(4)?.try_into().map_err(|_| EffectTraceWireError)?,
            port: cursor.u16()?,
        },
        1 => crate::SocketAddressV1::Inet6 {
            octets: cursor.take(16)?.try_into().map_err(|_| EffectTraceWireError)?,
            port: cursor.u16()?,
        },
        2 => crate::SocketAddressV1::Unix { path: cursor.bytes()? },
        _ => return Err(EffectTraceWireError),
    };
    address.validate().map_err(|_| EffectTraceWireError)?;
    Ok(address)
}

/// Fail-closed correlation the reply decoder cannot perform on its own: it
/// sees only a cursor, so `TransferCountV1::new` can prove `0 < count <=
/// effective_request` but nothing bounds `effective_request` against the
/// caller's own raw request length. A crafted trace with a valid count/
/// effective pair but an inflated `effective_request` would otherwise
/// decode. Reject mismatched request/reply shapes outright rather than
/// inferring a budget from a request the reply doesn't correspond to.
fn validate_transfer_request_bound(
    request: &CanonicalRequestV1,
    outcome: &CanonicalOutcomeV1,
) -> Result<(), EffectTraceWireError> {
    match outcome.transfer_request_bound() {
        None => Ok(()),
        Some(crate::TransferRequestBoundV1::ReadAt(transferred)) => match request {
            CanonicalRequestV1::FsReadAt { length, .. }
            | CanonicalRequestV1::ResourceStreamRead { length, .. }
            | CanonicalRequestV1::NetReceive { length, .. }
                if transferred.effective_request() <= *length =>
            {
                Ok(())
            }
            _ => Err(EffectTraceWireError),
        },
        Some(crate::TransferRequestBoundV1::WriteAt(transferred)) => match request {
            CanonicalRequestV1::FsWriteAt { length, .. }
            | CanonicalRequestV1::ResourceStreamWrite { length, .. }
            | CanonicalRequestV1::NetSend { length, .. }
                if transferred.effective_request() <= *length =>
            {
                Ok(())
            }
            _ => Err(EffectTraceWireError),
        },
    }
}

fn get_option<'a, T>(
    cursor: &mut Cursor<'a>,
    get: impl FnOnce(&mut Cursor<'a>) -> Result<T, EffectTraceWireError>,
) -> Result<Option<T>, EffectTraceWireError> {
    if get_bool(cursor)? {
        Ok(Some(get(cursor)?))
    } else {
        Ok(None)
    }
}

fn get_metadata_time(cursor: &mut Cursor<'_>) -> Result<i128, EffectTraceWireError> {
    Ok(i128::from_be_bytes(
        cursor
            .take(16)?
            .try_into()
            .map_err(|_| EffectTraceWireError)?,
    ))
}

fn get_reply(cursor: &mut Cursor<'_>) -> Result<CanonicalReplyV1, EffectTraceWireError> {
    Ok(match cursor.u8()? {
        0 => CanonicalReplyV1::Unit,
        1 => CanonicalReplyV1::Bool(get_bool(cursor)?),
        2 => CanonicalReplyV1::Bytes(cursor.bytes()?),
        3 => CanonicalReplyV1::ReadChunk(cursor.bytes()?),
        4 => CanonicalReplyV1::ReadEof,
        5 => CanonicalReplyV1::Instant(cursor.bytes()?),
        12 => CanonicalReplyV1::MonotonicInstant(cursor.bytes()?),
        13 => CanonicalReplyV1::FilePosition(cursor.u64()?),
        14 => CanonicalReplyV1::FdInheritancePolicy(get_inheritance_policy(cursor)?),
        16 => CanonicalReplyV1::SocketAddress(get_socket_address(cursor)?),
        15 => CanonicalReplyV1::ChildExit(match cursor.u8()? {
            0 => crate::ChildExitV1::Exited(cursor.u8()?),
            1 => crate::ChildExitV1::Signaled(cursor.u8()?),
            _ => return Err(EffectTraceWireError),
        }),
        6 => {
            let size = cursor.u64()?;
            let kind = get_node_kind(cursor)?;
            let mode = get_option(cursor, Cursor::u64)?;
            if mode.is_some_and(|mode| mode > 0o7777) {
                return Err(EffectTraceWireError);
            }
            CanonicalReplyV1::FileMetadata(FileMetadataV1 {
                size,
                kind,
                mode,
                links: get_option(cursor, Cursor::u64)?,
                device_major: cursor.u64()?,
                device_minor: cursor.u64()?,
                inode: get_option(cursor, Cursor::u64)?,
                modified: get_option(cursor, get_metadata_time)?,
                changed: get_option(cursor, get_metadata_time)?,
                accessed: get_option(cursor, get_metadata_time)?,
                born: get_option(cursor, get_metadata_time)?,
            })
        }
        17 => CanonicalReplyV1::DirectoryEnd,
        7 => {
            let count = usize::try_from(cursor.u64()?).map_err(|_| EffectTraceWireError)?;
            if count == 0 || count > cursor.remaining() / 9 {
                return Err(EffectTraceWireError);
            }
            let mut entries = Vec::with_capacity(count);
            for _ in 0..count {
                entries.push(DirEntryV1 {
                    name: cursor.bytes()?,
                    kind: get_node_kind(cursor)?,
                });
            }
            CanonicalReplyV1::DirectoryEntries(entries)
        }
        8 => CanonicalReplyV1::ResourceAcquired {
            schema_version: cursor.u16()?,
            resource_kind: get_resource_kind(cursor)?,
            identity: ResourceTraceIdentityV1(cursor.u64()?),
        },
        9 => CanonicalReplyV1::ResourceSettlement(get_settlement(cursor)?),
        10 => {
            let progress = match cursor.u8()? {
                0 => {
                    let start = cursor.u64()?;
                    let length = cursor.u64()?;
                    let transferred = cursor.u64()?;
                    let effective_request = cursor.u64()?;
                    if length != transferred {
                        return Err(EffectTraceWireError);
                    }
                    let transferred = crate::TransferCountV1::new(transferred, effective_request)
                        .ok_or(EffectTraceWireError)?;
                    crate::ReadProgressV1::ReadSome {
                        span: crate::BufferSpanV1 { start, length },
                        transferred,
                    }
                }
                1 => crate::ReadProgressV1::ReadEof,
                _ => return Err(EffectTraceWireError),
            };
            CanonicalReplyV1::ReadProgress(progress)
        }
        11 => {
            let count = cursor.u64()?;
            let effective_request = cursor.u64()?;
            let transferred = crate::TransferCountV1::new(count, effective_request)
                .ok_or(EffectTraceWireError)?;
            CanonicalReplyV1::WriteProgress(crate::WriteProgressV1::Wrote(transferred))
        }
        _ => return Err(EffectTraceWireError),
    })
}

fn get_resource_kind(cursor: &mut Cursor<'_>) -> Result<ResourceKindV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(ResourceKindV1::FsHandle),
        1 => Ok(ResourceKindV1::Buffer),
        2 => Ok(ResourceKindV1::Mapping),
        3 => Ok(ResourceKindV1::SpawnPlan),
        4 => Ok(ResourceKindV1::ChildProcess),
        5 => Ok(ResourceKindV1::PipeRead),
        6 => Ok(ResourceKindV1::PipeWrite),
        7 => Ok(ResourceKindV1::SocketListener),
        8 => Ok(ResourceKindV1::SocketStream),
        _ => Err(EffectTraceWireError),
    }
}

fn get_settlement(
    cursor: &mut Cursor<'_>,
) -> Result<ResourceSettlementObservationV1, EffectTraceWireError> {
    let schema_version = cursor.u16()?;
    let resource_kind = get_resource_kind(cursor)?;
    let identity = ResourceTraceIdentityV1(cursor.u64()?);
    let outcome = match cursor.u8()? {
        0 => ResourceSettlementOutcomeV1::Released,
        1 => ResourceSettlementOutcomeV1::ReleaseFailed(get_io_error(cursor)?),
        _ => return Err(EffectTraceWireError),
    };
    Ok(ResourceSettlementObservationV1 {
        schema_version,
        resource_kind,
        identity,
        outcome,
    })
}
fn get_io_error(cursor: &mut Cursor<'_>) -> Result<IoErrorIdentityV1, EffectTraceWireError> {
    Ok(match cursor.u8()? {
        0 => IoErrorIdentityV1::NotFound,
        1 => IoErrorIdentityV1::PermissionDenied,
        2 => IoErrorIdentityV1::BrokenPipe,
        3 => IoErrorIdentityV1::Interrupted,
        4 => IoErrorIdentityV1::AlreadyExists,
        5 => IoErrorIdentityV1::InvalidInput,
        6 => IoErrorIdentityV1::IsDirectory,
        7 => IoErrorIdentityV1::NotDirectory,
        8 => IoErrorIdentityV1::NotEmpty,
        9 => IoErrorIdentityV1::Unsupported,
        10 => IoErrorIdentityV1::Other(cursor.i32()?),
        11 => IoErrorIdentityV1::Revoked,
        12 => IoErrorIdentityV1::CapabilityDenied,
        _ => return Err(EffectTraceWireError),
    })
}
fn get_fs_operation(
    cursor: &mut Cursor<'_>,
) -> Result<FsCapabilityOperationV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(FsCapabilityOperationV1::Read),
        1 => Ok(FsCapabilityOperationV1::Write),
        2 => Ok(FsCapabilityOperationV1::Append),
        3 => Ok(FsCapabilityOperationV1::Metadata),
        4 => Ok(FsCapabilityOperationV1::Enumerate),
        5 => Ok(FsCapabilityOperationV1::CreateDirectory),
        6 => Ok(FsCapabilityOperationV1::RemoveFile),
        7 => Ok(FsCapabilityOperationV1::RemoveDirectory),
        8 => Ok(FsCapabilityOperationV1::RenameSource),
        9 => Ok(FsCapabilityOperationV1::RenameDestination),
        10 => Ok(FsCapabilityOperationV1::ChangeMode),
        11 => Ok(FsCapabilityOperationV1::Seek),
        12 => Ok(FsCapabilityOperationV1::SetLength),
        13 => Ok(FsCapabilityOperationV1::Sync),
        14 => Ok(FsCapabilityOperationV1::GetInheritance),
        15 => Ok(FsCapabilityOperationV1::SetInheritance),
        16 => Ok(FsCapabilityOperationV1::Duplicate),
        _ => Err(EffectTraceWireError),
    }
}
fn get_denial(cursor: &mut Cursor<'_>) -> Result<CapabilityDeniedV1, EffectTraceWireError> {
    Ok(match cursor.u8()? {
        0 => CapabilityDeniedV1::RightNotHeld {
            operation: get_fs_operation(cursor)?,
            held_rights: cursor.u8()?,
        },
        1 => CapabilityDeniedV1::AuthorityInsufficient,
        2 => CapabilityDeniedV1::ScopeEscape,
        3 => CapabilityDeniedV1::SymlinkDenied,
        4 => CapabilityDeniedV1::MalformedCapability,
        _ => return Err(EffectTraceWireError),
    })
}
fn get_cause(cursor: &mut Cursor<'_>) -> Result<FileErrorCauseV1, EffectTraceWireError> {
    match cursor.u8()? {
        0 => Ok(FileErrorCauseV1::Io(get_io_error(cursor)?)),
        1 => Ok(FileErrorCauseV1::Capability(get_denial(cursor)?)),
        _ => Err(EffectTraceWireError),
    }
}
fn get_error(cursor: &mut Cursor<'_>) -> Result<SemanticErrorV1, EffectTraceWireError> {
    Ok(match cursor.u8()? {
        0 => SemanticErrorV1::Io(get_io_error(cursor)?),
        1 => SemanticErrorV1::File(FileErrorIdentityV1 {
            operation: HostOpV1::try_from(cursor.u16()?).map_err(|_| EffectTraceWireError)?,
            relative_path: cursor.bytes()?,
            cause: get_cause(cursor)?,
        }),
        2 => SemanticErrorV1::Capability(get_denial(cursor)?),
        3 => SemanticErrorV1::Resource(match cursor.u8()? {
            0 => ResourceErrorV1::Closed,
            1 => ResourceErrorV1::MalformedResource,
            2 => ResourceErrorV1::RightNotHeld {
                required: cursor.u8()?,
                held: cursor.u8()?,
            },
            3 => ResourceErrorV1::ReleaseFailed {
                schema_version: cursor.u16()?,
                resource_kind: get_resource_kind(cursor)?,
                identity: ResourceTraceIdentityV1(cursor.u64()?),
                io: get_io_error(cursor)?,
            },
            4 => ResourceErrorV1::ResourceKindMismatch {
                expected: get_resource_kind(cursor)?,
                actual: get_resource_kind(cursor)?,
            },
            5 => ResourceErrorV1::BufferLimit,
            6 => ResourceErrorV1::InvalidOffset,
            7 => ResourceErrorV1::InvalidBounds,
            8 => ResourceErrorV1::NoProgress,
            9 => ResourceErrorV1::AllocationFailed,
            10 => ResourceErrorV1::MappingLimit,
            _ => return Err(EffectTraceWireError),
        }),
        _ => return Err(EffectTraceWireError),
    })
}

pub fn decode_linked_effect_trace(bytes: &[u8]) -> Result<LinkedEffectTrace, EffectTraceWireError> {
    let mut cursor = Cursor { bytes, position: 0 };
    if cursor.take(MAGIC.len())? != MAGIC {
        return Err(EffectTraceWireError);
    }
    let plan_hash = cursor.u64()?;
    let target_abi_hash = cursor.take(32)?.try_into().unwrap();
    let host_effect_abi_hash = cursor.take(32)?.try_into().unwrap();
    let terminal_value = cursor.i64()?;
    let terminal_error = match cursor.u8()? {
        0 => None,
        1 => Some(crate::TerminalErrorV1::RootExecutionDenied),
        4 => {
            if terminal_value != crate::SELECTED_CALL_INTEGRITY_STATUS_V1 {
                return Err(EffectTraceWireError);
            }
            Some(crate::TerminalErrorV1::SelectedCallIntegrity(
                match cursor.u8()? {
                    0 => crate::SelectedCallIntegrityFaultV1::WrongActivation,
                    1 => crate::SelectedCallIntegrityFaultV1::InvalidSlot,
                    2 => crate::SelectedCallIntegrityFaultV1::StaleGeneration,
                    3 => crate::SelectedCallIntegrityFaultV1::Spent,
                    4 => crate::SelectedCallIntegrityFaultV1::WrongTarget,
                    _ => return Err(EffectTraceWireError),
                },
            ))
        }
        3 => {
            let scope = match cursor.u8()? {
                0 => crate::CapacityScopeV1::Runtime,
                1 => crate::CapacityScopeV1::Invocation,
                2 => crate::CapacityScopeV1::Persistent,
                _ => return Err(EffectTraceWireError),
            };
            let resource = match cursor.u8()? {
                0 => crate::CapacityResourceV1::InvocationEpochs,
                1 => crate::CapacityResourceV1::EventGenerations,
                2 => crate::CapacityResourceV1::LivePendingSlots,
                3 => crate::CapacityResourceV1::Nodes,
                4 => crate::CapacityResourceV1::Words,
                5 => crate::CapacityResourceV1::DataBytes,
                6 => crate::CapacityResourceV1::NativeIntLimbs,
                _ => return Err(EffectTraceWireError),
            };
            if !capacity_scope_resource_is_valid(scope, resource) {
                return Err(EffectTraceWireError);
            }
            let limit = u128::from_le_bytes(cursor.take(16)?.try_into().unwrap());
            let requested = u128::from_le_bytes(cursor.take(16)?.try_into().unwrap());
            if terminal_value != crate::CAPACITY_EXHAUSTED_STATUS_V1 || requested <= limit {
                return Err(EffectTraceWireError);
            }
            Some(crate::TerminalErrorV1::CapacityExhausted(
                crate::CapacityExhaustedV1 {
                    scope,
                    resource,
                    limit,
                    requested,
                },
            ))
        }
        2 => Some(crate::TerminalErrorV1::HomeRootResolutionFailed(
            match cursor.u8()? {
                0 => crate::HomeRootResolutionFailureV1::NoAccountRecord,
                1 => crate::HomeRootResolutionFailureV1::AccountRecordTooLarge,
                2 => crate::HomeRootResolutionFailureV1::AccountLookup(get_io_error(&mut cursor)?),
                3 => crate::HomeRootResolutionFailureV1::InvalidAccountRecord,
                4 => crate::HomeRootResolutionFailureV1::RootOpen(get_io_error(&mut cursor)?),
                5 => crate::HomeRootResolutionFailureV1::ScopeEscape,
                6 => crate::HomeRootResolutionFailureV1::SymlinkDenied,
                _ => return Err(EffectTraceWireError),
            },
        )),
        _ => return Err(EffectTraceWireError),
    };
    let terminal_exit = get_terminal_exit(&mut cursor)?;
    let count = usize::try_from(cursor.u64()?).map_err(|_| EffectTraceWireError)?;
    // Minimum event: sequence + operation + capability tag + binding count +
    // request tag + outcome tag + reply/error tag.
    if count > cursor.remaining() / 22 {
        return Err(EffectTraceWireError);
    }
    let mut effect_trace = Vec::with_capacity(count);
    for _ in 0..count {
        let sequence = cursor.u64()?;
        let operation = HostOpV1::try_from(cursor.u16()?).map_err(|_| EffectTraceWireError)?;
        let capability = match cursor.u8()? {
            0 => None,
            1 => Some(CapabilityTraceIdentity(cursor.string()?)),
            _ => return Err(EffectTraceWireError),
        };
        let binding_count = usize::try_from(cursor.u64()?).map_err(|_| EffectTraceWireError)?;
        if binding_count > cursor.remaining() / 9 {
            return Err(EffectTraceWireError);
        }
        let mut resource_bindings = Vec::with_capacity(binding_count);
        for _ in 0..binding_count {
            resource_bindings.push((
                get_role(&mut cursor)?,
                ResourceTraceIdentityV1(cursor.u64()?),
            ));
        }
        let request = get_request(&mut cursor)?;
        let outcome = match cursor.u8()? {
            0 => CanonicalOutcomeV1::Success(get_reply(&mut cursor)?),
            1 => CanonicalOutcomeV1::Error(get_error(&mut cursor)?),
            _ => return Err(EffectTraceWireError),
        };
        validate_transfer_request_bound(&request, &outcome)?;
        effect_trace.push(EffectEvent {
            sequence,
            operation,
            capability,
            resource_bindings,
            request,
            outcome,
        });
    }
    let region_end = match cursor.u8()? {
        0 => None,
        1 => {
            let mut counts = || -> Result<RegionCountsV1, EffectTraceWireError> {
                Ok(RegionCountsV1 {
                    nodes: cursor.u64()?,
                    words: cursor.u64()?,
                    data_bytes: cursor.u64()?,
                    native_int_limbs: cursor.u64()?,
                })
            };
            let invocation = counts()?;
            let persistent = counts()?;
            Some(ActivationRegionEndV1 { invocation, persistent })
        }
        _ => return Err(EffectTraceWireError),
    };
    if cursor.position != bytes.len()
        || terminal_exit != crate::terminal_exit_class(terminal_value, terminal_error.as_ref())
    {
        return Err(EffectTraceWireError);
    }
    Ok(LinkedEffectTrace {
        plan_hash,
        target_abi_hash,
        host_effect_abi_hash,
        terminal_value,
        terminal_error,
        effect_trace,
        terminal_exit,
        region_end,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// MEASURED: absent time consumes only its tag; Some(0) adds 16 payload
    /// bytes and malformed tags do not decode. CLAIMED: wire absence has no
    /// numeric representation. THE GAP: both execution lanes must honor it.
    #[test]
    fn abi_s4_metadata_wire_distinguishes_absence_and_zero() {
        let mut row = FileMetadataV1::without_optional_fields(5, FsNodeKindV1::File, 71, 72);
        row.mode = Some(0o755);
        row.modified = Some(-1);
        let reply = CanonicalReplyV1::FileMetadata(row.clone());
        let mut absent = Vec::new();
        put_reply(&mut absent, &reply).unwrap();
        let mut cursor = Cursor {
            bytes: &absent,
            position: 0,
        };
        assert_eq!(get_reply(&mut cursor).unwrap(), reply);
        assert_eq!(cursor.remaining(), 0);
        assert_eq!(absent.last(), Some(&0));

        row.born = Some(0);
        let mut zero = Vec::new();
        let reply = CanonicalReplyV1::FileMetadata(row);
        put_reply(&mut zero, &reply).unwrap();
        let mut cursor = Cursor {
            bytes: &zero,
            position: 0,
        };
        assert_eq!(get_reply(&mut cursor).unwrap(), reply);
        assert_eq!(cursor.remaining(), 0);
        assert_eq!(&zero[..absent.len() - 1], &absent[..absent.len() - 1]);
        assert_eq!(
            &zero[absent.len() - 1..],
            &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        let mut malformed = absent.clone();
        *malformed.last_mut().unwrap() = 2;
        assert_eq!(
            get_reply(&mut Cursor {
                bytes: &malformed,
                position: 0
            }),
            Err(EffectTraceWireError)
        );
        malformed = absent.clone();
        malformed[9] = 0xff; // unknown FileKind
        assert_eq!(
            get_reply(&mut Cursor {
                bytes: &malformed,
                position: 0
            }),
            Err(EffectTraceWireError)
        );
        malformed = absent;
        malformed[11..19].copy_from_slice(&0o10000_u64.to_le_bytes());
        assert_eq!(
            get_reply(&mut Cursor {
                bytes: &malformed,
                position: 0
            }),
            Err(EffectTraceWireError)
        );
    }

    /// Promise: normative compatibility vector for the reserved typed terminal
    /// tag and durable invariant for unknown/malformed refusal. The positive
    /// image exercises the same decoder as the negative one-byte mutations.
    #[test]
    fn capacity_terminal_round_trips_and_rejects_forged_wire_tags() {
        let failure = crate::CapacityExhaustedV1 {
            scope: crate::CapacityScopeV1::Runtime,
            resource: crate::CapacityResourceV1::InvocationEpochs,
            limit: 0,
            requested: 1,
        };
        let trace = LinkedEffectTrace {
            plan_hash: 11,
            target_abi_hash: crate::TARGET_ABI_MANIFEST_HASH,
            host_effect_abi_hash: crate::HOST_EFFECT_ABI_V1_HASH,
            terminal_value: crate::CAPACITY_EXHAUSTED_STATUS_V1,
            terminal_error: Some(crate::TerminalErrorV1::CapacityExhausted(failure)),
            effect_trace: Vec::new(),
            terminal_exit: crate::TerminalExitClass::ControlledTrap,
            region_end: None,
        };
        let encoded = encode_linked_effect_trace(&trace).expect("typed capacity encodes");
        assert_eq!(decode_linked_effect_trace(&encoded).unwrap(), trace);
        let tag_at = MAGIC.len() + 8 + 32 + 32 + 8;
        for (offset, replacement) in [(tag_at, 0xff), (tag_at + 1, 1), (tag_at + 2, 1)] {
            let mut malformed = encoded.clone();
            malformed[offset] = replacement;
            assert_eq!(
                decode_linked_effect_trace(&malformed),
                Err(EffectTraceWireError)
            );
        }
        let mut truncated = encoded.clone();
        truncated.truncate(tag_at + 3 + 16);
        assert_eq!(
            decode_linked_effect_trace(&truncated),
            Err(EffectTraceWireError)
        );
        let mut wrong_status = encoded.clone();
        let status_at = MAGIC.len() + 8 + 32 + 32;
        wrong_status[status_at..status_at + 8].copy_from_slice(&(-8i64).to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&wrong_status),
            Err(EffectTraceWireError)
        );
        let mut inconsistent = trace;
        inconsistent.terminal_value = -8;
        assert_eq!(
            encode_linked_effect_trace(&inconsistent),
            Err(EffectTraceWireError)
        );
        inconsistent.terminal_value = crate::CAPACITY_EXHAUSTED_STATUS_V1;
        inconsistent.terminal_error = Some(crate::TerminalErrorV1::CapacityExhausted(
            crate::CapacityExhaustedV1 {
                requested: 0,
                ..failure
            },
        ));
        assert_eq!(
            encode_linked_effect_trace(&inconsistent),
            Err(EffectTraceWireError)
        );
    }

    /// Normative wire vector for the selected-call integrity terminal; the
    /// discriminator is a full typed round-trip plus negative tag/status/cause.
    #[test]
    fn selected_call_integrity_tag_round_trips_and_fails_closed() {
        let trace = LinkedEffectTrace {
            plan_hash: 12,
            target_abi_hash: crate::TARGET_ABI_MANIFEST_HASH,
            host_effect_abi_hash: crate::HOST_EFFECT_ABI_V1_HASH,
            terminal_value: crate::SELECTED_CALL_INTEGRITY_STATUS_V1,
            terminal_error: Some(crate::TerminalErrorV1::SelectedCallIntegrity(
                crate::SelectedCallIntegrityFaultV1::WrongTarget,
            )),
            effect_trace: Vec::new(),
            terminal_exit: crate::TerminalExitClass::ControlledTrap,
            region_end: None,
        };
        let encoded = encode_linked_effect_trace(&trace).expect("integrity wire");
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(trace.clone()));
        let tag_at = MAGIC.len() + 8 + 32 + 32 + 8;
        let mut malformed = encoded.clone();
        malformed[tag_at] = 5;
        assert_eq!(
            decode_linked_effect_trace(&malformed),
            Err(EffectTraceWireError)
        );
        malformed[tag_at] = 4;
        malformed[tag_at + 1] = 5;
        assert_eq!(
            decode_linked_effect_trace(&malformed),
            Err(EffectTraceWireError)
        );
        let mut wrong_status = encoded;
        let status_at = MAGIC.len() + 8 + 32 + 32;
        wrong_status[status_at..status_at + 8].copy_from_slice(&(-1i64).to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&wrong_status),
            Err(EffectTraceWireError)
        );
        let mut inconsistent = trace;
        inconsistent.terminal_value = -1;
        assert_eq!(
            encode_linked_effect_trace(&inconsistent),
            Err(EffectTraceWireError)
        );
    }

    /// ABI-S3 AC-3b. No cancellation surface exists on the sleep operation --
    /// no field, token, or status, INCLUDING an unused or reserved one (D2;
    /// PX12 owns cancellation).
    ///
    /// MEASURED: (a) the complete wire image of a ClockSleepUntil request is
    /// exactly one tag byte plus one little-endian u64, with every byte
    /// attributed; (b) the probed C record holds exactly one u64; and (c) the
    /// variant destructures with an exhaustive struct pattern binding exactly
    /// one field.
    /// CLAIMED: the request carries a deadline and nothing else.
    /// THE GAP: (a) alone would miss a field that exists but is not encoded,
    /// and (b) alone would miss one carried outside the C record. (c) closes
    /// the reserved/unused case, because a struct pattern with no `..` fails
    /// to COMPILE if any field is added. The three together leave no shape for
    /// a cancellation surface to occupy.
    ///
    /// ⚠ This is a NEGATIVE claim, so every part carries a positive control:
    /// the probe is shown to find the field that IS present, otherwise "found
    /// no cancellation surface" would be indistinguishable from a probe that
    /// inspects nothing.
    #[test]
    fn ac3b_the_sleep_request_carries_a_deadline_and_no_cancellation_surface() {
        const DEADLINE: u64 = 0x0102_0304_0506_0708;

        let encode = |deadline: u64| {
            let mut out = Vec::new();
            put_request(&mut out, &CanonicalRequestV1::ClockSleepUntil { deadline })
                .expect("the sleep request encodes");
            out
        };

        // POSITIVE CONTROL 1 -- the probe finds the field that IS present: the
        // deadline appears in the image, and changing it changes the image.
        let image = encode(DEADLINE);
        assert_eq!(image[0], 23, "sleep carries its wire tag");
        assert_eq!(
            &image[1..9],
            &DEADLINE.to_le_bytes(),
            "the deadline is present in the wire image"
        );
        assert_ne!(image, encode(0), "the image tracks the deadline");

        // Every byte is now attributed to the tag or the deadline, so an
        // encoded cancellation token or status has nowhere to sit.
        assert_eq!(
            image.len(),
            1 + 8,
            "the sleep request is exactly a tag and a deadline; any additional \
             encoded cancellation field would lengthen this image"
        );

        // POSITIVE CONTROL 2 -- the layout probe reads real records, shown by a
        // record that is genuinely larger.
        let fact = |name: &str| {
            crate::HOST_EFFECT_ABI_V1_FACTS
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| *value)
                .unwrap_or_else(|| panic!("probed layout fact {name}"))
        };
        assert_eq!(fact("OFFSET_ClockDeadlineRequestV1_deadline"), 0);
        assert!(
            fact("SIZE_HostReplyV1") > fact("SIZE_ClockDeadlineRequestV1"),
            "the layout probe distinguishes record sizes"
        );
        assert_eq!(
            fact("SIZE_ClockDeadlineRequestV1"),
            8,
            "the deadline record is exactly one u64; a cancellation token or \
             status field would enlarge it"
        );

        // POSITIVE CONTROL 3 + the reserved-field closure. This pattern has no
        // `..`, so it binds the fields that exist -- proving it inspects the
        // variant -- and adding ANY field, including an unused or reserved
        // one, makes this fail to compile.
        let CanonicalRequestV1::ClockSleepUntil { deadline } = encode_roundtrip(&image) else {
            panic!("the sleep request decodes to its own variant");
        };
        assert_eq!(deadline, DEADLINE);
    }

    fn encode_roundtrip(bytes: &[u8]) -> CanonicalRequestV1 {
        let mut cursor = Cursor { bytes, position: 0 };
        get_request(&mut cursor).expect("decodes")
    }

    /// ABI-S3 AC-1. Encode -> decode returns the identical canonical request
    /// for every operation added by this WP.
    ///
    /// The existing arity assert is explicitly NOT this test: a catalog closed
    /// at 25 says nothing about whether a request survives a round trip. The
    /// deadline and count are given distinct non-zero values so a codec that
    /// dropped a payload, or crossed the two u64 fields, cannot pass.
    #[test]
    fn ac1_every_new_request_survives_an_encode_decode_round_trip() {
        let requests = [
            CanonicalRequestV1::ClockMonotonicNow,
            CanonicalRequestV1::ClockSleepUntil {
                deadline: 0x0123_4567_89ab_cdef,
            },
            CanonicalRequestV1::EntropyRandomBytes { count: 977 },
        ];

        for request in &requests {
            let mut encoded = Vec::new();
            put_request(&mut encoded, request).expect("new requests encode");
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            let decoded = get_request(&mut cursor).expect("new requests decode");
            assert_eq!(&decoded, request);
        }

        // Non-vacuity: the three encodings are mutually distinct, so the
        // round trip above is not passing because every request encodes to
        // the same bytes.
        let encodings = requests
            .iter()
            .map(|request| {
                let mut out = Vec::new();
                put_request(&mut out, request).expect("encodes");
                out
            })
            .collect::<Vec<_>>();
        for (index, left) in encodings.iter().enumerate() {
            for right in &encodings[index + 1..] {
                assert_ne!(left, right);
            }
        }

        // And the wall clock, whose request carries no payload, does not
        // collide with the new monotonic read -- D1 on the wire.
        let mut wall = Vec::new();
        put_request(&mut wall, &CanonicalRequestV1::ClockWallNow).expect("encodes");
        assert_ne!(wall, encodings[0]);
    }

    /// Promise class: normative compatibility vector (AC-0 retirement).
    /// MEASURED: trace request tag 10 emits and consumes only the path bytes,
    /// and both old Bool encodings refuse. CLAIMED: tag 10 has no flag.
    /// THE GAP: this isolates the request codec, not raw ABI dispatch.
    #[test]
    fn fs_create_directory_tag_ten_has_only_a_path() {
        let expected = [10, 3, 0, 0, 0, 0, 0, 0, 0, b'a', b'/', b'b'];
        let request = CanonicalRequestV1::FsCreateDirectory {
            path: b"a/b".to_vec(),
        };
        let mut encoded = Vec::new();
        put_request(&mut encoded, &request).unwrap();
        assert_eq!(encoded, expected);

        let mut cursor = Cursor { bytes: &expected, position: 0 };
        assert_eq!(get_request(&mut cursor).unwrap(), request);
        assert_eq!(cursor.position, expected.len());
        for old_bool in [0, 1] {
            let mut old = expected.to_vec();
            old.insert(1, old_bool);
            let mut cursor = Cursor { bytes: &old, position: 0 };
            assert!(get_request(&mut cursor).is_err());
        }
    }

    /// Promise class: normative compatibility vector. Canonical ProcessSignal
    /// uses append-only tag 42 with a full u64 signal field, including unknown
    /// numbers for terminal dispatch validation rather than wire truncation.
    #[test]
    fn process_signal_request_round_trips_exact_append_only_tag() {
        for signal in [0_u64, 1, 3, 4, 9, 15, u64::MAX] {
            let request = CanonicalRequestV1::ProcessSignal { signal };
            let mut bytes = Vec::new();
            put_request(&mut bytes, &request).unwrap();
            assert_eq!(bytes[0], 42);
            assert_eq!(&bytes[1..], signal.to_le_bytes());
            let mut cursor = Cursor { bytes: &bytes, position: 0 };
            assert_eq!(get_request(&mut cursor).unwrap(), request);
            assert_eq!(cursor.position, bytes.len());
        }
    }

    /// Promise class: normative compatibility vector. MEASURED: each of the
    /// three trace-wire address family bytes equals its separately authored
    /// catalog binding. CLAIMED: trace and native ABI agree on family tags.
    /// THE GAP: this does not test native decoding or address byte lengths;
    /// those have independent native request known-answer tests.
    #[test]
    fn px11_trace_socket_families_equal_catalog_tags() {
        let cases = [
            (
                crate::SocketAddressV1::Inet4 {
                    octets: [127, 0, 0, 1],
                    port: 37,
                },
                "socket_family.inet4",
                0_u64,
            ),
            (
                crate::SocketAddressV1::Inet6 {
                    octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    port: 37,
                },
                "socket_family.inet6",
                1,
            ),
            (
                crate::SocketAddressV1::Unix {
                    path: b"/tmp/px11-family".to_vec(),
                },
                "socket_family.unix",
                2,
            ),
        ];
        for (address, name, expected) in cases {
            let tag = crate::HOST_EFFECT_ABI_V1_BINDINGS
                .iter()
                .find_map(|(kind, binding, value)| {
                    (*kind == "tag" && *binding == name).then_some(*value)
                })
                .unwrap_or_else(|| panic!("missing catalog tag {name}"));
            assert_eq!(tag, expected, "catalog {name}");
            let mut wire = Vec::new();
            put_socket_address(&mut wire, &address).unwrap();
            assert_eq!(u64::from(wire[0]), tag, "trace family {name}");
        }
    }

    /// Promise class: normative compatibility vector. MEASURED: seven Net
    /// request tags and two address families have independent known-answer
    /// bytes, decode consumes the whole request, and an invalid Unix path
    /// refuses instead of round-tripping. CLAIMED: the trace wire preserves
    /// typed socket requests and rejects invalid decoded addresses. THE GAP:
    /// the dispatcher independently verifies operation/request compatibility.
    #[test]
    fn px11_socket_wire_request_and_reply_tags_are_typed_and_fail_closed() {
        let ipv4 = crate::SocketAddressV1::Inet4 {
            octets: [127, 0, 0, 1],
            port: 0x0123,
        };
        let unix = crate::SocketAddressV1::Unix {
            path: b"/tmp/s".to_vec(),
        };
        let cases = [
            (CanonicalRequestV1::NetListen { address: ipv4.clone() }, 43),
            (CanonicalRequestV1::NetAccept, 44),
            (CanonicalRequestV1::NetConnect { address: unix.clone() }, 45),
            (CanonicalRequestV1::NetReceive { buffer_start: 3, length: 7 }, 46),
            (CanonicalRequestV1::NetSend { buffer_start: 5, length: 9 }, 47),
            (CanonicalRequestV1::NetShutdown { how: 2 }, 48),
            (CanonicalRequestV1::NetLocalAddress, 49),
        ];
        for (request, tag) in cases {
            let mut bytes = Vec::new();
            put_request(&mut bytes, &request).unwrap();
            assert_eq!(bytes[0], tag, "{request:?} tag");
            let mut cursor = Cursor { bytes: &bytes, position: 0 };
            assert_eq!(get_request(&mut cursor), Ok(request));
            assert_eq!(cursor.position, bytes.len());
        }
        let mut ipv4_wire = Vec::new();
        put_request(&mut ipv4_wire, &CanonicalRequestV1::NetListen { address: ipv4 }).unwrap();
        assert_eq!(ipv4_wire, [43, 0, 127, 0, 0, 1, 0x23, 0x01]);
        let mut unix_wire = Vec::new();
        put_request(&mut unix_wire, &CanonicalRequestV1::NetConnect { address: unix }).unwrap();
        assert_eq!(&unix_wire[..10], &[45, 2, 6, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&unix_wire[10..], b"/tmp/s");
        unix_wire[10] = b'x';
        let mut cursor = Cursor { bytes: &unix_wire, position: 0 };
        assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
        let reply = CanonicalReplyV1::SocketAddress(crate::SocketAddressV1::Inet6 {
            octets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            port: 0x1234,
        });
        let mut bytes = Vec::new();
        put_reply(&mut bytes, &reply).unwrap();
        assert_eq!(&bytes[..2], &[16, 1]);
        assert_eq!(&bytes[2..18], &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(&bytes[18..], &[0x34, 0x12]);
        let mut cursor = Cursor { bytes: &bytes, position: 0 };
        assert_eq!(get_reply(&mut cursor), Ok(reply));
        assert_eq!(cursor.position, bytes.len());
    }

    /// Promise class: normative compatibility vector. MEASURED: all three D3
    /// Mapping requests round-trip with distinct append-only request tags,
    /// ReadOnly/Writable encode as 0/1, and an unknown protection byte is
    /// rejected. CLAIMED: the represented Mapping trace wire is typed and
    /// fail-closed without widening the protection alphabet. THE GAP: opaque
    /// resource tokens live in EffectEvent resource bindings rather than this
    /// canonical request payload and are covered by dispatch tests.
    #[test]
    fn abi_s6_d3_mapping_requests_round_trip_and_protection_fails_closed() {
        let requests = [
            CanonicalRequestV1::MappingAllocate {
                length: 0x0123_4567_89ab_cdef,
                protection: MappingProtectionV1::ReadOnly,
            },
            CanonicalRequestV1::MappingAllocate {
                length: 0xfedc_ba98_7654_3210,
                protection: MappingProtectionV1::Writable,
            },
            CanonicalRequestV1::MappingReadView {
                start: 17,
                length: 31,
            },
            CanonicalRequestV1::MappingWriteView {
                start: 23,
                bytes: b"mapping".to_vec(),
            },
            CanonicalRequestV1::MappingAcquireFile {
                length: 64,
                protection: MappingProtectionV1::Writable,
            },
        ];
        let mut encodings = Vec::new();
        for request in &requests {
            let mut encoded = Vec::new();
            put_request(&mut encoded, request).expect("mapping request encodes");
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor).unwrap(), *request);
            assert_eq!(cursor.position, encoded.len());
            encodings.push(encoded);
        }
        assert_eq!(
            encodings.iter().map(|bytes| bytes[0]).collect::<Vec<_>>(),
            vec![31, 31, 32, 33, 34]
        );
        assert_eq!(encodings[0].last(), Some(&0));
        assert_eq!(encodings[1].last(), Some(&1));
        assert_ne!(encodings[0], encodings[1]);
        assert_ne!(encodings[2], encodings[3]);

        let mut unknown = vec![31];
        put_u64(&mut unknown, 8);
        put_u8(&mut unknown, 2);
        let mut cursor = Cursor {
            bytes: &unknown,
            position: 0,
        };
        assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
    }

    /// Promise class: normative compatibility vector. MEASURED: all three
    /// signed/unsigned seek-origin forms, set-length, file-position replies,
    /// and the two PX9 capability-operation identities round-trip with exact
    /// distinct tags; an unknown seek-origin tag and a truncated payload are
    /// rejected. CLAIMED: ABI-S1's descriptor wire shape
    /// is typed and fail-closed rather than an untagged offset. THE GAP: native
    /// execution remains deliberately unavailable in this partial.
    #[test]
    fn abi_s1_descriptor_requests_and_position_reply_round_trip_typed_wire() {
        let requests = [
            CanonicalRequestV1::FsSeek {
                from: FsSeekFromV1::Start(0x0123_4567_89ab_cdef),
            },
            CanonicalRequestV1::FsSeek {
                from: FsSeekFromV1::Current(-41),
            },
            CanonicalRequestV1::FsSeek {
                from: FsSeekFromV1::End(-73),
            },
            CanonicalRequestV1::FsSetLength {
                length: 0xfedc_ba98_7654_3210,
            },
        ];
        let mut encodings = Vec::new();
        for request in &requests {
            let mut encoded = Vec::new();
            put_request(&mut encoded, request).expect("descriptor request encodes");
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor).unwrap(), *request);
            assert_eq!(cursor.position, encoded.len());
            encodings.push(encoded);
        }
        assert_eq!(
            encodings.iter().map(|bytes| bytes[0]).collect::<Vec<_>>(),
            vec![25, 25, 25, 26]
        );
        assert_eq!(
            encodings
                .iter()
                .take(3)
                .map(|bytes| bytes[1])
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        for (index, left) in encodings.iter().enumerate() {
            for right in &encodings[index + 1..] {
                assert_ne!(left, right);
            }
        }

        let mut reply = Vec::new();
        put_reply(
            &mut reply,
            &CanonicalReplyV1::FilePosition(0x1020_3040_5060_7080),
        )
        .unwrap();
        assert_eq!(reply[0], 13);
        let mut cursor = Cursor {
            bytes: &reply,
            position: 0,
        };
        assert_eq!(
            get_reply(&mut cursor).unwrap(),
            CanonicalReplyV1::FilePosition(0x1020_3040_5060_7080)
        );
        assert_eq!(cursor.position, reply.len());

        for (operation, tag) in [
            (FsCapabilityOperationV1::Seek, 11),
            (FsCapabilityOperationV1::SetLength, 12),
        ] {
            let denial = CapabilityDeniedV1::RightNotHeld {
                operation,
                held_rights: 0x55,
            };
            let mut encoded = Vec::new();
            put_denial(&mut encoded, &denial);
            assert_eq!(encoded, [0, tag, 0x55]);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_denial(&mut cursor).unwrap(), denial);
            assert_eq!(cursor.position, encoded.len());
        }

        let malformed_origin = [25, 3];
        let mut cursor = Cursor {
            bytes: &malformed_origin,
            position: 0,
        };
        assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
        let truncated_set_length = [26, 1, 2, 3];
        let mut cursor = Cursor {
            bytes: &truncated_set_length,
            position: 0,
        };
        assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
    }

    /// Promise class: normative compatibility vector. MEASURED: full and data
    /// sync use distinct typed mode tags, round-trip without conflation, the
    /// pathless `FsSync` file-error identity survives, and unknown or absent
    /// modes are rejected. CLAIMED: ABI-S1 D3 preserves the
    /// fsync/fdatasync distinction at the canonical wire boundary. THE GAP:
    /// native execution remains deliberately unavailable in this partial.
    #[test]
    fn abi_s1_sync_modes_round_trip_distinct_and_fail_closed() {
        for (mode, bytes) in [
            (FsSyncModeV1::SyncFull, [27, 0]),
            (FsSyncModeV1::SyncData, [27, 1]),
        ] {
            let request = CanonicalRequestV1::FsSync { mode };
            let mut encoded = Vec::new();
            put_request(&mut encoded, &request).expect("sync request encodes");
            assert_eq!(encoded, bytes);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor).unwrap(), request);
            assert_eq!(cursor.position, encoded.len());
        }

        let denial = CapabilityDeniedV1::RightNotHeld {
            operation: FsCapabilityOperationV1::Sync,
            held_rights: 0x55,
        };
        let mut encoded = Vec::new();
        put_denial(&mut encoded, &denial);
        assert_eq!(encoded, [0, 13, 0x55]);
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_denial(&mut cursor).unwrap(), denial);
        assert_eq!(cursor.position, encoded.len());

        let file_error = SemanticErrorV1::File(FileErrorIdentityV1 {
            operation: HostOpV1::FsSync,
            relative_path: Vec::new(),
            cause: FileErrorCauseV1::Io(IoErrorIdentityV1::Other(77)),
        });
        let mut encoded = Vec::new();
        put_error(&mut encoded, &file_error).expect("sync file error encodes");
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_error(&mut cursor).unwrap(), file_error);
        assert_eq!(cursor.position, encoded.len());

        for malformed in [&[27, 2][..], &[27][..]] {
            let mut cursor = Cursor {
                bytes: malformed,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
        }
    }

    /// Promise class: normative compatibility vector. MEASURED: get and set
    /// have distinct request identities; both inheritance policies preserve
    /// their frozen tags through set requests and get replies; rights and PX9
    /// operation identities remain profile-specific; and unknown or truncated
    /// policy values are refused. CLAIMED: ABI-S1 D4 never conflates direction
    /// or defaults an unrecognized policy to inheritance. THE GAP: OS behavior
    /// and resource-right enforcement are exercised by the dispatcher test.
    #[test]
    fn abi_s1_inheritance_policy_wire_is_typed_distinct_and_fail_closed() {
        assert_eq!(FdInheritancePolicyV1::Inherit as u64, 0);
        assert_eq!(FdInheritancePolicyV1::CloseOnExec as u64, 1);

        let mut get = Vec::new();
        put_request(&mut get, &CanonicalRequestV1::FsGetInheritance).unwrap();
        assert_eq!(get, [28]);
        let mut cursor = Cursor {
            bytes: &get,
            position: 0,
        };
        assert_eq!(
            get_request(&mut cursor).unwrap(),
            CanonicalRequestV1::FsGetInheritance
        );
        assert_eq!(cursor.position, get.len());

        for (policy, request_bytes, reply_bytes) in [
            (FdInheritancePolicyV1::Inherit, [29, 0], [14, 0]),
            (FdInheritancePolicyV1::CloseOnExec, [29, 1], [14, 1]),
        ] {
            let request = CanonicalRequestV1::FsSetInheritance { policy };
            let mut encoded = Vec::new();
            put_request(&mut encoded, &request).unwrap();
            assert_eq!(encoded, request_bytes);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor).unwrap(), request);
            assert_eq!(cursor.position, encoded.len());

            let reply = CanonicalReplyV1::FdInheritancePolicy(policy);
            let mut encoded = Vec::new();
            put_reply(&mut encoded, &reply).unwrap();
            assert_eq!(encoded, reply_bytes);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_reply(&mut cursor).unwrap(), reply);
            assert_eq!(cursor.position, encoded.len());
        }

        for (operation, tag) in [
            (FsCapabilityOperationV1::GetInheritance, 14),
            (FsCapabilityOperationV1::SetInheritance, 15),
        ] {
            let denial = CapabilityDeniedV1::RightNotHeld {
                operation,
                held_rights: 0x55,
            };
            let mut encoded = Vec::new();
            put_denial(&mut encoded, &denial);
            assert_eq!(encoded, [0, tag, 0x55]);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_denial(&mut cursor).unwrap(), denial);
            assert_eq!(cursor.position, encoded.len());
        }

        for operation in [HostOpV1::FsGetInheritance, HostOpV1::FsSetInheritance] {
            let file_error = SemanticErrorV1::File(FileErrorIdentityV1 {
                operation,
                relative_path: Vec::new(),
                cause: FileErrorCauseV1::Io(IoErrorIdentityV1::Other(78)),
            });
            let mut encoded = Vec::new();
            put_error(&mut encoded, &file_error).unwrap();
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_error(&mut cursor).unwrap(), file_error);
            assert_eq!(cursor.position, encoded.len());
        }

        for malformed in [&[29, 2][..], &[29][..]] {
            let mut cursor = Cursor {
                bytes: malformed,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
        }
        for malformed in [&[14, 2][..], &[14][..]] {
            let mut cursor = Cursor {
                bytes: malformed,
                position: 0,
            };
            assert_eq!(get_reply(&mut cursor), Err(EffectTraceWireError));
        }
    }

    /// Promise class: normative compatibility vector. MEASURED: Mapping's
    /// failure and mismatch payloads remain exact; all seven resource-kind
    /// byte tags agree with their generated ABI bindings, and unassigned tags
    /// refuse. CLAIMED: ABI-S6 D1 and PX10 extend the resource-kind wire
    /// append-only without conflating identities. THE GAP: this is codec
    /// evidence; resource release behavior is pinned in `effect_v1`.
    #[test]
    fn abi_s6_d1_mapping_resource_kind_wire_is_distinct_and_fail_closed() {
        let failure = SemanticErrorV1::Resource(ResourceErrorV1::ReleaseFailed {
            schema_version: crate::RESOURCE_OBSERVATION_SCHEMA_VERSION_V1,
            resource_kind: ResourceKindV1::Mapping,
            identity: ResourceTraceIdentityV1(0x0102_0304_0506_0708),
            io: IoErrorIdentityV1::Unsupported,
        });
        let mut encoded = Vec::new();
        put_error(&mut encoded, &failure).unwrap();
        assert_eq!(encoded[0..5], [3, 3, 1, 0, 2]);
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_error(&mut cursor).unwrap(), failure);
        assert_eq!(cursor.position, encoded.len());

        let mismatch = SemanticErrorV1::Resource(ResourceErrorV1::ResourceKindMismatch {
            expected: ResourceKindV1::Mapping,
            actual: ResourceKindV1::Buffer,
        });
        encoded.clear();
        put_error(&mut encoded, &mismatch).unwrap();
        assert_eq!(encoded, [3, 4, 2, 1]);
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_error(&mut cursor).unwrap(), mismatch);
        assert_eq!(cursor.position, encoded.len());

        // Independent byte vectors and build-generated catalog bindings pin
        // the tag, so swapping encoder and decoder together cannot pass.
        let kinds = [
            (ResourceKindV1::FsHandle, "resource_kind.FsHandle", 0),
            (ResourceKindV1::Buffer, "resource_kind.Buffer", 1),
            (ResourceKindV1::Mapping, "resource_kind.Mapping", 2),
            (ResourceKindV1::SpawnPlan, "resource_kind.SpawnPlan", 3),
            (
                ResourceKindV1::ChildProcess,
                "resource_kind.ChildProcess",
                4,
            ),
            (ResourceKindV1::PipeRead, "resource_kind.PipeRead", 5),
            (ResourceKindV1::PipeWrite, "resource_kind.PipeWrite", 6),
            (ResourceKindV1::SocketListener, "resource_kind.SocketListener", 7),
            (ResourceKindV1::SocketStream, "resource_kind.SocketStream", 8),
        ];
        for (kind, name, tag) in kinds {
            match kind {
                ResourceKindV1::FsHandle
                | ResourceKindV1::Buffer
                | ResourceKindV1::Mapping
                | ResourceKindV1::SpawnPlan
                | ResourceKindV1::ChildProcess
                | ResourceKindV1::PipeRead
                | ResourceKindV1::PipeWrite
                | ResourceKindV1::SocketListener
                | ResourceKindV1::SocketStream => {}
            }
            let catalog_tag = crate::HOST_EFFECT_ABI_V1_BINDINGS
                .iter()
                .find_map(|(binding_kind, binding_name, value)| {
                    (*binding_kind == "tag" && *binding_name == name).then_some(*value)
                })
                .unwrap_or_else(|| panic!("missing generated ABI binding {name}"));
            assert_eq!(catalog_tag, u64::from(tag), "catalog {name}");
            let mut encoded = Vec::new();
            put_resource_kind(&mut encoded, kind);
            assert_eq!(encoded, [tag], "encoder {kind:?}");
            let mut cursor = Cursor {
                bytes: &[tag],
                position: 0,
            };
            assert_eq!(get_resource_kind(&mut cursor), Ok(kind), "decoder {kind:?}");
            assert_eq!(cursor.position, 1);
        }
        let mut assigned_tags = kinds.map(|(_, _, tag)| tag).to_vec();
        assigned_tags.sort_unstable();
        assigned_tags.dedup();
        assert_eq!(assigned_tags.len(), kinds.len(), "distinct wire tags");
        let first_unassigned = assigned_tags[assigned_tags.len() - 1] + 1;
        for invalid in [first_unassigned, u8::MAX] {
            let mut cursor = Cursor {
                bytes: &[invalid],
                position: 0,
            };
            assert_eq!(get_resource_kind(&mut cursor), Err(EffectTraceWireError));
        }
    }

    /// Promise class: normative compatibility vector. MEASURED: both frozen
    /// policies round-trip through the distinct duplicate request tag, the
    /// operation keeps its independent PX9 and right identities, and unknown
    /// or truncated policy payloads fail closed. CLAIMED: ABI-S1 D5 reuses the
    /// D4 policy without defaulting or tag conflation. THE GAP: descriptor and
    /// resource-table behavior are exercised by the dispatcher discriminator.
    #[test]
    fn abi_s1_duplicate_wire_is_typed_distinct_and_fail_closed() {
        for (policy, bytes) in [
            (FdInheritancePolicyV1::Inherit, [30, 0]),
            (FdInheritancePolicyV1::CloseOnExec, [30, 1]),
        ] {
            let request = CanonicalRequestV1::FsDuplicate { policy };
            let mut encoded = Vec::new();
            put_request(&mut encoded, &request).unwrap();
            assert_eq!(encoded, bytes);
            let mut cursor = Cursor {
                bytes: &encoded,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor).unwrap(), request);
            assert_eq!(cursor.position, encoded.len());
        }

        let denial = CapabilityDeniedV1::RightNotHeld {
            operation: FsCapabilityOperationV1::Duplicate,
            held_rights: 0x55,
        };
        let mut encoded = Vec::new();
        put_denial(&mut encoded, &denial);
        assert_eq!(encoded, [0, 16, 0x55]);
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_denial(&mut cursor).unwrap(), denial);
        assert_eq!(cursor.position, encoded.len());

        let file_error = SemanticErrorV1::File(FileErrorIdentityV1 {
            operation: HostOpV1::FsDuplicate,
            relative_path: Vec::new(),
            cause: FileErrorCauseV1::Io(IoErrorIdentityV1::Other(79)),
        });
        let mut encoded = Vec::new();
        put_error(&mut encoded, &file_error).unwrap();
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_error(&mut cursor).unwrap(), file_error);
        assert_eq!(cursor.position, encoded.len());

        let acquired = CanonicalReplyV1::ResourceAcquired {
            schema_version: 1,
            resource_kind: ResourceKindV1::FsHandle,
            identity: ResourceTraceIdentityV1(31),
        };
        let mut encoded = Vec::new();
        put_reply(&mut encoded, &acquired).unwrap();
        assert_eq!(
            encoded[0], 8,
            "duplicate reuses ResourceAcquired reply wire"
        );
        let mut cursor = Cursor {
            bytes: &encoded,
            position: 0,
        };
        assert_eq!(get_reply(&mut cursor).unwrap(), acquired);
        assert_eq!(cursor.position, encoded.len());

        for malformed in [&[30, 2][..], &[30][..]] {
            let mut cursor = Cursor {
                bytes: malformed,
                position: 0,
            };
            assert_eq!(get_request(&mut cursor), Err(EffectTraceWireError));
        }
    }

    fn representative_trace() -> LinkedEffectTrace {
        LinkedEffectTrace {
            plan_hash: 7,
            target_abi_hash: [3; 32],
            host_effect_abi_hash: [5; 32],
            terminal_value: 61,
            terminal_error: None,
            terminal_exit: TerminalExitClass::ReturnedError,
            region_end: None,
            effect_trace: vec![
                EffectEvent {
                    sequence: 0,
                    operation: HostOpV1::ConsoleWrite,
                    capability: None,
                    resource_bindings: Vec::new(),
                    request: CanonicalRequestV1::ConsoleWrite {
                        stream: ConsoleStreamV1::Stdout,
                        bytes: vec![0, 0xff],
                    },
                    outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Io(
                        IoErrorIdentityV1::BrokenPipe,
                    )),
                },
                EffectEvent {
                    sequence: 1,
                    operation: HostOpV1::FsReadFile,
                    capability: Some(CapabilityTraceIdentity("declared:FS".into())),
                    resource_bindings: Vec::new(),
                    request: CanonicalRequestV1::FsReadFile {
                        path: vec![b'f', 0xff],
                    },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(vec![1, 2])),
                },
                EffectEvent {
                    sequence: 2,
                    operation: HostOpV1::FsChangeMode,
                    capability: Some(CapabilityTraceIdentity("declared:FS".into())),
                    resource_bindings: Vec::new(),
                    request: CanonicalRequestV1::FsChangeMode {
                        path: vec![b'm', 0xfe],
                        mode: 0o640,
                    },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit),
                },
                EffectEvent {
                    sequence: 3,
                    operation: HostOpV1::FsOpen,
                    capability: Some(CapabilityTraceIdentity("declared:FS".into())),
                    resource_bindings: vec![(
                        ResourceBindingRole::Target,
                        ResourceTraceIdentityV1(1),
                    )],
                    request: CanonicalRequestV1::FsOpen {
                        path: vec![b'r', 0xff],
                        mode: FsOpenModeV1::Metadata,
                    },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
                        schema_version: 1,
                        resource_kind: ResourceKindV1::FsHandle,
                        identity: ResourceTraceIdentityV1(1),
                    }),
                },
                EffectEvent {
                    sequence: 4,
                    operation: HostOpV1::ResourceRelease,
                    capability: None,
                    resource_bindings: vec![(
                        ResourceBindingRole::Target,
                        ResourceTraceIdentityV1(1),
                    )],
                    request: CanonicalRequestV1::ResourceRelease,
                    outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
                        ResourceErrorV1::ReleaseFailed {
                            schema_version: 1,
                            resource_kind: ResourceKindV1::FsHandle,
                            identity: ResourceTraceIdentityV1(1),
                            io: IoErrorIdentityV1::Other(5),
                        },
                    )),
                },
                EffectEvent {
                    sequence: 5,
                    operation: HostOpV1::BufferAllocate,
                    capability: None,
                    resource_bindings: vec![(
                        ResourceBindingRole::Target,
                        ResourceTraceIdentityV1(2),
                    )],
                    request: CanonicalRequestV1::BufferAllocate { capacity: 8 },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::ResourceAcquired {
                        schema_version: 1,
                        resource_kind: ResourceKindV1::Buffer,
                        identity: ResourceTraceIdentityV1(2),
                    }),
                },
                EffectEvent {
                    sequence: 6,
                    operation: HostOpV1::FsReadAt,
                    capability: None,
                    resource_bindings: vec![
                        (ResourceBindingRole::File, ResourceTraceIdentityV1(1)),
                        (ResourceBindingRole::Buffer, ResourceTraceIdentityV1(2)),
                    ],
                    request: CanonicalRequestV1::FsReadAt {
                        file_offset: 9,
                        buffer_start: 1,
                        length: 4,
                    },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::ReadProgress(
                        crate::ReadProgressV1::ReadSome {
                            span: crate::BufferSpanV1 {
                                start: 1,
                                length: 2,
                            },
                            transferred: crate::TransferCountV1::new(2, 3)
                                .expect("2 <= effective 3"),
                        },
                    )),
                },
                EffectEvent {
                    sequence: 7,
                    operation: HostOpV1::FsWriteAt,
                    capability: None,
                    resource_bindings: Vec::new(),
                    request: CanonicalRequestV1::FsWriteAt {
                        file_offset: u64::MAX,
                        buffer_start: 0,
                        length: 1,
                    },
                    outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
                        ResourceErrorV1::ResourceKindMismatch {
                            expected: ResourceKindV1::FsHandle,
                            actual: ResourceKindV1::Buffer,
                        },
                    )),
                },
                EffectEvent {
                    sequence: 8,
                    operation: HostOpV1::BufferFreeze,
                    capability: None,
                    resource_bindings: vec![(
                        ResourceBindingRole::Target,
                        ResourceTraceIdentityV1(2),
                    )],
                    request: CanonicalRequestV1::BufferFreeze {
                        start: 1,
                        length: 2,
                    },
                    outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(vec![0xff, 0])),
                },
            ],
        }
    }

    #[test]
    fn linked_trace_codec_roundtrips_identity_bytes_and_outcomes() {
        let expected = representative_trace();
        let encoded = encode_linked_effect_trace(&expected).unwrap();
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(expected));
    }

    /// Promise class: normative compatibility vector. MEASURED: the sole
    /// effect-trace codec round-trips the canonical Revoked I/O identity inside
    /// a file error. CLAIMED: observation transport cannot collapse revocation
    /// into a capability or neighboring I/O identity. THE GAP: this is codec
    /// evidence; the dispatcher and both reifiers have independent controls.
    #[test]
    fn linked_trace_codec_preserves_file_origin_canonical_revoked_identity() {
        let mut expected = representative_trace();
        expected.effect_trace.push(EffectEvent {
            sequence: 9,
            operation: HostOpV1::FsReadFile,
            capability: Some(CapabilityTraceIdentity("revoked".to_string())),
            resource_bindings: Vec::new(),
            request: CanonicalRequestV1::FsReadFile {
                path: b"shared".to_vec(),
            },
            outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::File(FileErrorIdentityV1 {
                operation: HostOpV1::FsReadFile,
                relative_path: b"shared".to_vec(),
                cause: FileErrorCauseV1::Io(IoErrorIdentityV1::Revoked),
            })),
        });

        let encoded = encode_linked_effect_trace(&expected).unwrap();
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(expected));
    }

    /// Promise class: normative compatibility vector. MEASURED: the sole
    /// effect-trace codec round-trips the same canonical Revoked I/O identity
    /// from a resource operation. CLAIMED: observation transport unifies both
    /// origins without collapsing to a lifetime or neighboring I/O identity.
    /// THE GAP: the dispatcher and checked reifiers independently pin production.
    #[test]
    fn linked_trace_codec_preserves_resource_origin_canonical_revoked_identity() {
        let mut expected = representative_trace();
        expected.effect_trace.push(EffectEvent {
            sequence: 9,
            operation: HostOpV1::FsHandleMetadata,
            capability: None,
            resource_bindings: Vec::new(),
            request: CanonicalRequestV1::FsHandleMetadata,
            outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Io(IoErrorIdentityV1::Revoked)),
        });

        let encoded = encode_linked_effect_trace(&expected).unwrap();
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(expected));
    }

    #[test]
    fn linked_trace_codec_preserves_allocation_failure_identity() {
        let mut expected = representative_trace();
        expected.effect_trace.push(EffectEvent {
            sequence: 9,
            operation: HostOpV1::BufferAllocate,
            capability: None,
            resource_bindings: Vec::new(),
            request: CanonicalRequestV1::BufferAllocate { capacity: u64::MAX },
            outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
                ResourceErrorV1::AllocationFailed,
            )),
        });

        let encoded = encode_linked_effect_trace(&expected).unwrap();
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(expected));
    }

    /// Promise class: normative compatibility vector. MEASURED: the sole trace
    /// codec round-trips file-backed Mapping acquisition with request tag 34 and
    /// the append-only MappingLimit error identity. CLAIMED: D4's request and
    /// governor refusal remain distinct from anonymous allocation and
    /// AllocationFailed on the wire. THE GAP: raw C layout and dispatch behavior
    /// are pinned independently.
    #[test]
    fn abi_s6_d4_file_acquire_and_mapping_limit_round_trip_typed_wire() {
        let mut expected = representative_trace();
        expected.effect_trace.push(EffectEvent {
            sequence: 9,
            operation: HostOpV1::MappingAcquireFile,
            capability: None,
            resource_bindings: Vec::new(),
            request: CanonicalRequestV1::MappingAcquireFile {
                length: 1024 * 1024 + 1,
                protection: MappingProtectionV1::ReadOnly,
            },
            outcome: CanonicalOutcomeV1::Error(SemanticErrorV1::Resource(
                ResourceErrorV1::MappingLimit,
            )),
        });

        let encoded = encode_linked_effect_trace(&expected).unwrap();
        assert_eq!(decode_linked_effect_trace(&encoded), Ok(expected));
    }

    #[test]
    fn home_failure_wire_distinguishes_lookup_from_root_open_and_keeps_identity() {
        let mut lookup = representative_trace();
        lookup.terminal_error = Some(crate::TerminalErrorV1::HomeRootResolutionFailed(
            crate::HomeRootResolutionFailureV1::AccountLookup(IoErrorIdentityV1::Other(5)),
        ));
        lookup.terminal_exit = TerminalExitClass::ControlledTrap;
        let mut root_open = representative_trace();
        root_open.terminal_error = Some(crate::TerminalErrorV1::HomeRootResolutionFailed(
            crate::HomeRootResolutionFailureV1::RootOpen(IoErrorIdentityV1::Other(5)),
        ));
        root_open.terminal_exit = TerminalExitClass::ControlledTrap;

        let lookup_wire = encode_linked_effect_trace(&lookup).unwrap();
        let root_open_wire = encode_linked_effect_trace(&root_open).unwrap();
        assert_ne!(lookup_wire, root_open_wire);
        assert_eq!(decode_linked_effect_trace(&lookup_wire), Ok(lookup));
        assert_eq!(decode_linked_effect_trace(&root_open_wire), Ok(root_open));
    }

    /// Promise: normative compatibility vector for the KETRACE3 optional
    /// region-end suffix; durable invariant for malformed-tag and stale-version
    /// refusal. Distinct fields pin both region order and each count's order.
    #[test]
    fn region_end_wire_round_trips_both_tags_and_refuses_unknown_and_old_magic() {
        let absent = representative_trace();
        let encoded_absent = encode_linked_effect_trace(&absent).expect("encode absent record");
        assert_eq!(encoded_absent.last(), Some(&0));
        assert_eq!(decode_linked_effect_trace(&encoded_absent), Ok(absent.clone()));

        let end = ActivationRegionEndV1 {
            invocation: RegionCountsV1 {
                nodes: 11,
                words: 22,
                data_bytes: 33,
                native_int_limbs: 44,
            },
            persistent: RegionCountsV1 {
                nodes: 55,
                words: 66,
                data_bytes: 77,
                native_int_limbs: 88,
            },
        };
        let present = LinkedEffectTrace {
            region_end: Some(end),
            ..absent
        };
        let encoded_present = encode_linked_effect_trace(&present).expect("encode present record");
        let suffix_at = encoded_absent.len() - 1;
        assert_eq!(&encoded_present[..suffix_at], &encoded_absent[..suffix_at]);
        assert_eq!(encoded_present[suffix_at], 1);
        assert_eq!(encoded_present.len(), encoded_absent.len() + 8 * 8);
        assert_eq!(decode_linked_effect_trace(&encoded_present), Ok(present));

        let mut bad_tag = encoded_absent.clone();
        bad_tag[suffix_at] = 2;
        assert_eq!(decode_linked_effect_trace(&bad_tag), Err(EffectTraceWireError));
        let mut stale = encoded_absent;
        stale[..8].copy_from_slice(b"KETRACE2");
        assert_eq!(decode_linked_effect_trace(&stale), Err(EffectTraceWireError));
    }

    #[test]
    fn linked_trace_decoder_rejects_magic_truncation_trailing_and_count_drift() {
        let encoded = encode_linked_effect_trace(&representative_trace()).unwrap();

        let mut wrong_magic = encoded.clone();
        wrong_magic[0] ^= 1;
        assert_eq!(
            decode_linked_effect_trace(&wrong_magic),
            Err(EffectTraceWireError)
        );
        assert_eq!(
            decode_linked_effect_trace(&encoded[..encoded.len() - 1]),
            Err(EffectTraceWireError)
        );
        let mut trailing = encoded.clone();
        trailing.push(0);
        assert_eq!(
            decode_linked_effect_trace(&trailing),
            Err(EffectTraceWireError)
        );
        let mut impossible_count = encoded;
        impossible_count[88..96].copy_from_slice(&u64::MAX.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&impossible_count),
            Err(EffectTraceWireError)
        );
    }

    #[test]
    fn sole_codec_rejects_terminal_role_length_and_class_drift() {
        let trace = LinkedEffectTrace {
            plan_hash: 7,
            target_abi_hash: [3; 32],
            host_effect_abi_hash: [5; 32],
            terminal_value: 0,
            terminal_error: None,
            effect_trace: vec![EffectEvent {
                sequence: 0,
                operation: HostOpV1::ResourceRelease,
                capability: None,
                resource_bindings: vec![(ResourceBindingRole::Target, ResourceTraceIdentityV1(19))],
                request: CanonicalRequestV1::ResourceRelease,
                outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::Unit),
            }],
            terminal_exit: TerminalExitClass::NormalReturn,
            region_end: None,
        };
        let encoded = encode_linked_effect_trace(&trace).unwrap();

        let mut wrong_terminal_tag = encoded.clone();
        wrong_terminal_tag[89] = 3;
        assert_eq!(
            decode_linked_effect_trace(&wrong_terminal_tag),
            Err(EffectTraceWireError)
        );

        let mut wrong_role = encoded.clone();
        wrong_role[117] = 3;
        assert_eq!(
            decode_linked_effect_trace(&wrong_role),
            Err(EffectTraceWireError)
        );

        let mut impossible_bindings = encoded.clone();
        impossible_bindings[109..117].copy_from_slice(&u64::MAX.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&impossible_bindings),
            Err(EffectTraceWireError)
        );

        let mut mismatched_class = trace;
        mismatched_class.terminal_exit = TerminalExitClass::ReturnedError;
        assert_eq!(
            encode_linked_effect_trace(&mismatched_class),
            Err(EffectTraceWireError)
        );
    }

    fn single_read_some_trace(request_length: u64) -> LinkedEffectTrace {
        LinkedEffectTrace {
            plan_hash: 7,
            target_abi_hash: [3; 32],
            host_effect_abi_hash: [5; 32],
            terminal_value: 0,
            terminal_error: None,
            effect_trace: vec![EffectEvent {
                sequence: 0,
                operation: HostOpV1::FsReadAt,
                capability: None,
                resource_bindings: Vec::new(),
                request: CanonicalRequestV1::FsReadAt {
                    file_offset: 0,
                    buffer_start: 0,
                    length: request_length,
                },
                outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::ReadProgress(
                    crate::ReadProgressV1::ReadSome {
                        span: crate::BufferSpanV1 {
                            start: 0,
                            length: 4,
                        },
                        transferred: crate::TransferCountV1::new(4, 4).expect("4 <= effective 4"),
                    },
                )),
            }],
            terminal_exit: TerminalExitClass::NormalReturn,
            region_end: None,
        }
    }

    /// The gap the Architect found in `dec_3fz801fangyp`'s first review: the
    /// reply decoder alone can only prove `0 < count <= effective_request`
    /// (`TransferCountV1::new`'s own invariant, exercised by the first two
    /// mutations below); it cannot bound `effective_request` against the
    /// caller's own raw request length without `validate_transfer_request_
    /// bound` correlating the decoded request. `effective_request` is the
    /// last 8 bytes of this single-event trace's `ReadSome` payload,
    /// immediately before the KETRACE3 region-end tag, so mutating those
    /// bytes exercises exactly that field.
    #[test]
    fn wire_decoder_rejects_read_some_effective_request_zero_below_count_and_above_raw() {
        let encoded = encode_linked_effect_trace(&single_read_some_trace(8)).unwrap();
        assert!(
            decode_linked_effect_trace(&encoded).is_ok(),
            "baseline must be valid"
        );
        let tail = encoded.len() - 1 - 8;

        let mut effective_zero = encoded.clone();
        effective_zero[tail..tail + 8].copy_from_slice(&0u64.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&effective_zero),
            Err(EffectTraceWireError),
            "effective_request == 0"
        );

        let mut effective_below_count = encoded.clone();
        effective_below_count[tail..tail + 8].copy_from_slice(&3u64.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&effective_below_count),
            Err(EffectTraceWireError),
            "effective_request(3) < count(4)"
        );

        let mut effective_above_raw = encoded;
        effective_above_raw[tail..tail + 8].copy_from_slice(&9u64.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&effective_above_raw),
            Err(EffectTraceWireError),
            "effective_request(9) > raw request.length(8) — the fold-in fix"
        );
    }

    /// Boundary constraint 4 (`dec_1m6xdwjp2ttyn`): applies to both
    /// `ReadSome` and `Wrote`. Confirms the `effective_request > raw`
    /// correlation fix is symmetric, not `ReadSome`-only.
    #[test]
    fn wire_decoder_rejects_wrote_effective_request_above_raw_request() {
        let trace = LinkedEffectTrace {
            plan_hash: 7,
            target_abi_hash: [3; 32],
            host_effect_abi_hash: [5; 32],
            terminal_value: 0,
            terminal_error: None,
            effect_trace: vec![EffectEvent {
                sequence: 0,
                operation: HostOpV1::FsWriteAt,
                capability: None,
                resource_bindings: Vec::new(),
                request: CanonicalRequestV1::FsWriteAt {
                    file_offset: 0,
                    buffer_start: 0,
                    length: 8,
                },
                outcome: CanonicalOutcomeV1::Success(CanonicalReplyV1::WriteProgress(
                    crate::WriteProgressV1::Wrote(
                        crate::TransferCountV1::new(4, 4).expect("4 <= effective 4"),
                    ),
                )),
            }],
            terminal_exit: TerminalExitClass::NormalReturn,
            region_end: None,
        };
        let encoded = encode_linked_effect_trace(&trace).unwrap();
        assert!(
            decode_linked_effect_trace(&encoded).is_ok(),
            "baseline must be valid"
        );

        let mut effective_above_raw = encoded;
        let tail = effective_above_raw.len() - 1 - 8;
        effective_above_raw[tail..tail + 8].copy_from_slice(&9u64.to_le_bytes());
        assert_eq!(
            decode_linked_effect_trace(&effective_above_raw),
            Err(EffectTraceWireError),
            "effective_request(9) > raw request.length(8)"
        );
    }
}
