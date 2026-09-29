//! PX9-INC2B's single revoked identity and preserved resource lifecycle surface.
//!
//! Source: `spec/30-surface/38-ffi-io.md` section 1.8 and the Architect's
//! corrected INC2 decomposition. The ResourceError inventory is a normative
//! compatibility vector; the canonical identity and zero-trust assertions are
//! durable invariants.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{GlobalId, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;

const SYSTEM_ERROR: &str = "Capability.System.Error";

fn system_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, SYSTEM_ERROR, name)
        .unwrap_or_else(|error| panic!("System.Error owner {name}: {error}"))
}

fn former(id: GlobalId) -> Term {
    Term::indformer(id, vec![])
}

fn constructor_names(env: &ElabEnv, family: &str) -> Vec<String> {
    env.env
        .inductive(env.globals[family])
        .unwrap_or_else(|| panic!("`{family}` must be an inductive"))
        .constructors
        .iter()
        .map(|constructor| {
            env.globals
                .iter()
                .find_map(|(name, id)| (*id == constructor.id).then(|| name.clone()))
                .unwrap_or_else(|| panic!("constructor {:?} has no public name", constructor.id))
        })
        .collect()
}

#[test]
fn resource_lifecycle_keeps_exact_non_revoked_arms_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("PX9-INC2B prelude");
    let resource_error = env.globals["ResourceError"];
    let declaration = env
        .env
        .inductive(resource_error)
        .expect("ResourceError inductive");

    assert_eq!(
        constructor_names(&env, "ResourceError"),
        [
            "ResourceHostIO",
            "Closed",
            "MalformedResource",
            "RightNotHeld",
            "ReleaseFailed",
            "ResourceKindMismatch",
            "BufferLimit",
            "AllocationFailed",
            "InvalidOffset",
            "InvalidBounds",
            "NoProgress",
            "MappingLimit",
        ]
    );
    assert_eq!(
        declaration
            .constructors
            .iter()
            .map(|constructor| constructor.args.clone())
            .collect::<Vec<_>>(),
        [
            vec![former(env.globals["IOError"])],
            vec![],
            vec![],
            vec![
                Term::const_(env.globals["Int"], vec![]),
                Term::const_(env.globals["Int"], vec![]),
            ],
            vec![
                former(env.globals["ResourceKind"]),
                former(env.globals["ResourceTraceIdentity"]),
                former(env.globals["IOError"]),
            ],
            vec![
                former(env.globals["ResourceKind"]),
                former(env.globals["ResourceKind"]),
            ],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
        "only the retired resource-local revoked arm may leave the lifecycle sum"
    );
    assert!(
        !env.globals.contains_key("ResourceRevoked"),
        "the resource-local revoked constructor must be retired"
    );

    let trusted: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert!(
        declaration
            .constructors
            .iter()
            .all(|constructor| !trusted.contains(&constructor.id)),
        "ResourceError constructors must remain outside the trusted base"
    );

    let revoked = env.globals["Revoked"];
    assert_eq!(
        env.env
            .constructor(revoked)
            .expect("Revoked constructor")
            .0
            .id,
        env.globals["IOError"],
        "Revoked must have exactly the canonical IOError identity"
    );

    let before = env.env.trusted_base();
    env.elaborate_decl(
        "const px9_canonical_resource_error : ResourceError = ResourceHostIO Revoked",
    )
    .expect("canonical ResourceHostIO Revoked must elaborate without a package");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], SYSTEM_ERROR)
        .expect("System.Error must roots-load for the classifier proof");
    let proofs = env
        .elaborate_file(
            "import Capability.System.Error (Transience, error_transience, Permanent)\n\
             theorem px9_canonical_revoked_is_permanent :\n\
               Equal Transience (error_transience Revoked) Permanent = Proved",
        )
        .expect("the package classifier must prove canonical Revoked Permanent");
    let theorem = *proofs.last().expect("classifier theorem must be checked");
    let references = catalog_or::declaration_references(
        env.env.lookup(theorem).expect("checked classifier proof"),
    );
    assert!(references.contains(&system_id(&env, &owned, "Transience")));
    assert!(references.contains(&system_id(&env, &owned, "error_transience")));
    assert!(
        references.contains(&revoked),
        "IOError.Revoked identity must be retained"
    );
    assert_eq!(
        env.env.trusted_base(),
        before,
        "using the canonical revoked identity and checked package adds no trust"
    );
}
