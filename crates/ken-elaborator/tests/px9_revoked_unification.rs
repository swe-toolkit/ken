//! PX9-INC2B's single revoked identity and preserved resource lifecycle surface.
//!
//! Source: `spec/30-surface/38-ffi-io.md` section 1.8 and the Architect's
//! corrected INC2 decomposition. The ResourceError inventory is a normative
//! compatibility vector; the canonical identity and zero-trust assertions are
//! durable invariants.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{GlobalId, Term};

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
    env.elaborate_file(
        r#"
const px9_canonical_resource_error : ResourceError = ResourceHostIO Revoked

theorem px9_canonical_revoked_is_permanent :
  Equal Transience (error_transience Revoked) Permanent = Proved
"#,
    )
    .expect("canonical ResourceHostIO Revoked must elaborate and classify Permanent");
    assert_eq!(
        env.env.trusted_base(),
        before,
        "using the canonical revoked identity must add no trust"
    );
}
