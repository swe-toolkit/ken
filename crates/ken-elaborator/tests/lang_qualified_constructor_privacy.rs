//! Private prelude identities must not be resurrected by qualified selection.

use std::mem::discriminant;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

#[derive(Clone, Copy, Debug)]
enum Entry {
    Decl,
    File,
}

fn elaborate(entry: Entry, source: &str) -> Result<(), ElabError> {
    let mut env = ElabEnv::new().expect("checked prelude and hidden roster");
    match entry {
        Entry::Decl => env.elaborate_decl(source).map(|_| ()),
        Entry::File => env.elaborate_file(source).map(|_| ()),
    }
}

fn assert_missing(entry: Entry, source: &str, missing: &str) -> ElabError {
    let err = elaborate(entry, source).expect_err("a hidden constructor must be absent");
    assert!(
        matches!(&err, ElabError::UnresolvedCon { name, .. } if name == missing),
        "{entry:?} `{source}`: wrong refusal for {missing}: {err:?}"
    );
    err
}

/// Promise class: regression witness. Every arm reached an actual hidden
/// constructor through T.C on the landed base, except the two zero-parameter
/// arms whose base forms were malformed; these use valid const declarations.
/// MEASURED: both ambient entry points refuse each expression and pattern
/// specifically at its checked constructor name. CLAIMED: neither resource
/// handles nor their span/count/trace representations can be forged or opened
/// through a qualified prelude family. THE GAP: imported IDs and future source
/// aliases need their own visibility checks; the public constructor suite is
/// run independently.
macro_rules! private_probe {
    ($file:ident, $decl:ident, $source:expr, $name:expr) => {
        #[test]
        fn $file() {
            assert_missing(Entry::File, $source, $name);
        }
        #[test]
        fn $decl() {
            assert_missing(Entry::Decl, $source, $name);
        }
    };
}

private_probe!(
    forge_buffer_file,
    forge_buffer_decl,
    "fn forge_buffer (r : Resource ResourceKind.Buffer) : BufferHandle = BufferHandle.PrivateBufferHandle r (5 : Int)",
    "BufferHandle.PrivateBufferHandle"
);
private_probe!(
    buffer_capacity_file,
    buffer_capacity_decl,
    "fn buffer_capacity (b : BufferHandle) : Int = match b { BufferHandle.PrivateBufferHandle r c |-> c }",
    "BufferHandle.PrivateBufferHandle"
);
private_probe!(
    buffer_resource_file,
    buffer_resource_decl,
    "fn buffer_resource (b : BufferHandle) : Resource ResourceKind.Buffer = match b { BufferHandle.PrivateBufferHandle r c |-> r }",
    "BufferHandle.PrivateBufferHandle"
);
private_probe!(
    forge_mapping_file,
    forge_mapping_decl,
    "fn forge_mapping (r : Resource ResourceKind.Mapping) (x : MappingExtent) : MappingHandle = MappingHandle.PrivateMappingHandle r x",
    "MappingHandle.PrivateMappingHandle"
);
private_probe!(
    mapping_resource_file,
    mapping_resource_decl,
    "fn mapping_resource (m : MappingHandle) : Resource ResourceKind.Mapping = match m { MappingHandle.PrivateMappingHandle r x |-> r }",
    "MappingHandle.PrivateMappingHandle"
);
private_probe!(
    forge_span_file,
    forge_span_decl,
    "fn forge_span (r : Resource ResourceKind.Buffer) : BufferSpan = BufferSpan.PrivateBufferSpan r (0 : Int) Zero",
    "BufferSpan.PrivateBufferSpan"
);
private_probe!(
    forge_count_file,
    forge_count_decl,
    "const forge_count : TransferCount = TransferCount.PrivateTransferCount Zero Zero",
    "TransferCount.PrivateTransferCount"
);
private_probe!(
    forge_trace_file,
    forge_trace_decl,
    "const forge_trace : ResourceTraceIdentity = ResourceTraceIdentity.PrivateResourceTraceIdentity (1 : Int) (2 : Int)",
    "ResourceTraceIdentity.PrivateResourceTraceIdentity"
);

/// Promise class: opacity, not only refusal. An unavailable constructor and
/// an invented member of the same family have the same error VARIANT in
/// expression, pattern, and qualified type-argument positions. The public
/// type and resource-kind controls keep each source shape meaningful.
#[test]
fn hidden_and_nonexistent_leaves_have_the_same_failure_class() {
    let pairs = [
        (
            "fn forge (r : Resource ResourceKind.Buffer) : BufferHandle = BufferHandle.PrivateBufferHandle r (5 : Int)",
            "fn forge (r : Resource ResourceKind.Buffer) : BufferHandle = BufferHandle.NoSuchLeaf r (5 : Int)",
        ),
        (
            "fn peek (b : BufferHandle) : Int = match b { BufferHandle.PrivateBufferHandle r c |-> c }",
            "fn peek (b : BufferHandle) : Int = match b { BufferHandle.NoSuchLeaf r c |-> c }",
        ),
        (
            "fn index (r : Resource BufferHandle.PrivateBufferHandle) : Nat = Zero",
            "fn index (r : Resource BufferHandle.NoSuchLeaf) : Nat = Zero",
        ),
    ];
    for entry in [Entry::Decl, Entry::File] {
        for (hidden, invented) in pairs {
            let hidden_err = assert_missing(entry, hidden, "BufferHandle.PrivateBufferHandle");
            let invented_err = assert_missing(entry, invented, "BufferHandle.NoSuchLeaf");
            assert_eq!(
                discriminant(&hidden_err),
                discriminant(&invented_err),
                "{entry:?}: revealing diagnostic for `{hidden}`: {hidden_err:?} vs {invented_err:?}"
            );
        }
        assert_missing(
            entry,
            "fn bare (r : Resource ResourceKind.Buffer) : BufferHandle = PrivateBufferHandle r (5 : Int)",
            "PrivateBufferHandle",
        );
    }
}

/// Promise class: non-regression. The public qualified resource kind is still
/// an atomic type argument and the public Bool constructor still resolves.
#[test]
fn public_qualified_constructors_preserve_checked_identity() {
    let mut env = ElabEnv::new().expect("checked private prelude definitions");
    let trust = env.env.trusted_base();
    env.elaborate_file(
        "fn keep (r : Resource ResourceKind.Buffer) : Resource ResourceKind.Buffer = r\n\
         const yes : Bool = Bool.True",
    )
    .expect("public qualified constructors survive private-name hiding");
    let (_, checked) = env
        .env
        .const_type(env.globals["keep"])
        .expect("checked Resource binder");
    let resource = Term::app(
        Term::const_(env.globals["Resource"], vec![]),
        Term::constructor(env.prelude_env.runtime_roles.resource_kind_buffer, vec![]),
    );
    assert_eq!(checked, Term::pi(resource.clone(), resource));
    assert!(matches!(
        env.env.transparent_body(env.globals["yes"]),
        Some((_, Term::Constructor { id, .. })) if id == env.globals["True"]
    ));
    assert_eq!(env.env.trusted_base(), trust);
}

/// Promise class: identity rather than the five demonstrated spellings. The
/// prelude's manual FSOp constructor IDs and all five data constructor IDs
/// survive in the kernel but no spelling in the ambient map retains them.
#[test]
fn hidden_roster_sweeps_all_kernel_constructor_identities() {
    let env = ElabEnv::new().expect("checked prelude");
    let p = &env.prelude_env;
    let mapping = env
        .env
        .inductive(env.globals["MappingHandle"])
        .expect("checked MappingHandle");
    assert_eq!(mapping.constructors.len(), 1);
    let private: [GlobalId; 16] = [
        p.private_fs_open_id,
        p.private_fs_handle_metadata_id,
        p.private_resource_release_id,
        p.private_buffer_allocate_id,
        p.private_fs_read_at_id,
        p.private_fs_write_at_id,
        p.private_buffer_freeze_id,
        p.private_mapping_allocate_id,
        p.private_mapping_read_view_id,
        p.private_mapping_write_view_id,
        p.private_mapping_acquire_file_id,
        p.private_buffer_handle_id,
        p.private_buffer_span_id,
        mapping.constructors[0].id,
        p.private_transfer_count_id,
        p.private_resource_trace_identity_id,
    ];
    for id in private {
        assert!(
            env.env.constructor(id).is_some(),
            "driver ID {id:?} was not a real constructor"
        );
        assert!(
            !env.globals.values().any(|visible| *visible == id),
            "private driver constructor {id:?} leaked under a globals spelling"
        );
    }
}
