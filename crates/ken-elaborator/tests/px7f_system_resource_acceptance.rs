//! PX7-F checked surface and `ResourceHostIO` collision discriminator.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{env::PrimReduction, Decl};

#[test]
fn system_resource_is_opaque_and_only_the_bracket_mints_it() {
    let mut env = ElabEnv::empty().expect("PX7-F prelude bootstrap");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], "Capability.System.Resource")
        .expect("System.Resource owner must roots-load");
    env.execute_loaded_entry_checked_fences("Capability.System.Resource")
        .expect("System.Resource checked fences keep their contract");

    let resource = env.globals["Resource"];
    assert!(matches!(
        env.env.lookup(resource),
        Some(Decl::Primitive {
            reduction: PrimReduction::OpaqueType,
            ..
        })
    ));
    for private in [
        "PrivateFsOpen",
        "PrivateFsHandleMetadata",
        "PrivateBufferAllocate",
        "PrivateResourceRelease",
        "PrivateBracketRelease",
        "PrivateResourceTraceIdentity",
        "private_resource_acquire",
        "private_with_resource_after_open",
        "private_with_buffer_after_allocate",
        "release_if_live",
    ] {
        assert!(
            !env.globals.contains_key(private),
            "private representation/protocol identity `{private}` escaped"
        );
    }
    for public in ["withResource", "withBuffer", "resourceMetadata", "release"] {
        let id = env.globals[public];
        assert!(
            env.env.transparent_body(id).is_some(),
            "`{public}` must be an ordinary checked term"
        );
    }
}

#[test]
fn resource_host_io_preserves_the_existing_host_io_identity() {
    let mut env = ElabEnv::empty().expect("PX7-F prelude bootstrap");
    let host_io = env.globals["HostIO"];
    let resource_host_io = env.globals["ResourceError.ResourceHostIO"];
    assert_ne!(host_io, resource_host_io);
    assert!(env.env.transparent_body(host_io).is_some());
    assert!(env.env.constructor(resource_host_io).is_some());

    // Promise class: durable invariant. MEASURED: constructor-position reuse
    // of a floor type refuses before allocating CollisionMutation, while both
    // checked HostIO identities remain unchanged. CLAIMED: the floor protects
    // the former even against an unscoped same-spelling source constructor.
    // THE GAP: this fixture exercises HostIO; the ABI floor test covers the
    // remaining four entrypoint type names in the same constructor position.
    let error = env
        .elaborate_decl("data CollisionMutation = HostIO IOError")
        .expect_err("a source constructor spelled like a floor type is refused");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "HostIO"),
        "{error:?}"
    );
    assert_eq!(env.globals["HostIO"], host_io);
    assert_eq!(env.globals["ResourceError.ResourceHostIO"], resource_host_io);
    assert!(!env.globals.contains_key("CollisionMutation"));
}
