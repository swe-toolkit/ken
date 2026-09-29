//! PX9-INC1 domain-general system errors and two-axis retry classification.
//!
//! Sources: PX9-C (`spec/30-surface/38-ffi-io.md` §1.8) and the
//! Architect's PX9-INC1 D0 reconciliation ruling. These controls are durable
//! invariants except for the exact constructor inventories, which are normative
//! compatibility vectors: adding a domain is deliberately additive and must
//! extend the relevant inventory and total classifier together.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{inductive::peel_app, inductive::peel_pi, Decl, GlobalId, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;

const SYSTEM_ERROR: &str = "Capability.System.Error";
const SYSTEM_ERROR_IMPORT: &str =
    "import Capability.System.Error (Transience, Transient, Permanent, \
    Idempotence, Idempotent, NonIdempotent, RetryGuidance, RetryAdvised, \
    RetryUnsafeNonIdempotent, DoNotRetryPermanent, Operation, FilesystemOp, \
    ResourceRef, FilesystemResource, SafeContext, NoSafeContext, RedactedSafeContext, \
    SystemError, MkSystemError, file_error_to_system, error_transience, \
    operation_idempotence, retry_guidance)";

fn system_error_env() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("prelude");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], SYSTEM_ERROR)
        .expect("System.Error must roots-load through the catalog");
    (env, owned)
}

fn system_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, SYSTEM_ERROR, name)
        .unwrap_or_else(|error| panic!("System.Error owner {name}: {error}"))
}

fn system_constructor(env: &ElabEnv, owned: &[GlobalId], family: &str, name: &str) -> GlobalId {
    let owner = system_id(env, owned, family);
    let qualified = format!("{SYSTEM_ERROR}.{name}");
    let id = env.globals[&qualified];
    let (inductive, _) = env.env.constructor(id).expect("System.Error constructor");
    assert_eq!(inductive.id, owner, "{qualified} must belong to {family}");
    id
}

fn system_inductive<'a>(
    env: &'a ElabEnv,
    owned: &[GlobalId],
    name: &str,
) -> &'a ken_kernel::InductiveDecl {
    env.env
        .inductive(system_id(env, owned, name))
        .unwrap_or_else(|| panic!("`{name}` must be an owned inductive"))
}

fn inductive<'a>(env: &'a ElabEnv, name: &str) -> &'a ken_kernel::InductiveDecl {
    env.env
        .inductive(env.globals[name])
        .unwrap_or_else(|| panic!("`{name}` must be an inductive"))
}

fn constructor_names(env: &ElabEnv, family: &str) -> Vec<String> {
    inductive(env, family)
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

fn system_constructor_names(env: &ElabEnv, owned: &[GlobalId], family: &str) -> Vec<String> {
    system_inductive(env, owned, family)
        .constructors
        .iter()
        .map(|constructor| {
            let name = env
                .globals
                .iter()
                .find_map(|(name, id)| {
                    (*id == constructor.id)
                        .then(|| name.strip_prefix("Capability.System.Error."))
                        .flatten()
                })
                .unwrap_or_else(|| panic!("constructor {:?} has no provider name", constructor.id));
            assert_eq!(
                system_constructor(env, owned, family, name),
                constructor.id,
                "{family}.{name} must resolve by checked constructor identity"
            );
            name.to_owned()
        })
        .collect()
}

fn former(id: GlobalId) -> Term {
    Term::indformer(id, vec![])
}

fn head_global(term: &Term) -> Option<GlobalId> {
    let (head, _) = peel_app(term);
    match head {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            Some(id)
        }
        _ => None,
    }
}

fn signature_heads(env: &ElabEnv, owned: &[GlobalId], name: &str) -> (Vec<GlobalId>, GlobalId) {
    let declaration = env
        .env
        .lookup(system_id(env, owned, name))
        .unwrap_or_else(|| panic!("`{name}` must be a declaration"));
    let ty = match declaration {
        Decl::Transparent { ty, .. } | Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => ty,
        Decl::Inductive(inductive) => &inductive.former_type,
    };
    let (domains, result) = peel_pi(ty);
    (
        domains
            .iter()
            .map(|domain| {
                head_global(domain)
                    .unwrap_or_else(|| panic!("`{name}` has a non-global domain head: {domain:?}"))
            })
            .collect(),
        head_global(&result)
            .unwrap_or_else(|| panic!("`{name}` has a non-global result head: {result:?}")),
    )
}

/// Promise class: durable absence and checked-package trust invariant.
/// MEASURED: the fresh prelude lacks every moved family and classifier, and
/// loading the real package adds no trusted ID. CLAIMED: System.Error owns
/// the classification surface without extending the trusted base. THE GAP:
/// this row does not assert runtime effects or host classification policy.
#[test]
fn fresh_prelude_omits_system_error_and_package_adds_no_trust() {
    let mut env = ElabEnv::new().expect("prelude");
    let moved = [
        "Transience",
        "Idempotence",
        "RetryGuidance",
        "Operation",
        "ResourceRef",
        "SafeContext",
        "SystemError",
        "file_error_to_system",
        "error_transience",
        "operation_idempotence",
        "retry_guidance",
    ];
    for name in moved.into_iter().chain([
        "Transient",
        "Permanent",
        "Idempotent",
        "NonIdempotent",
        "RetryAdvised",
        "RetryUnsafeNonIdempotent",
        "DoNotRetryPermanent",
        "FilesystemOp",
        "FilesystemResource",
        "NoSafeContext",
        "RedactedSafeContext",
        "MkSystemError",
    ]) {
        assert!(
            !env.globals.contains_key(name),
            "prelude must not register {name}"
        );
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], SYSTEM_ERROR)
        .expect("System.Error must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "System.Error must not extend trusted_base()");
    for name in moved {
        assert!(!before.contains(&system_id(&env, &owned, name)));
    }
    for name in [
        "retry_advised_only_for_transient_idempotent",
        "error_transience_revoked_permanent",
        "retry_guidance_transient_idempotent",
        "retry_guidance_transient_nonidempotent",
        "retry_guidance_permanent_idempotent",
        "retry_guidance_permanent_nonidempotent",
        "retry_guidance_idempotence_flip",
    ] {
        let id = system_id(&env, &owned, name);
        assert!(
            env.env.transparent_body(id).is_some(),
            "{name} must be a checked proof"
        );
        assert!(!after.contains(&id), "{name} must not add trust");
    }
}

#[test]
fn system_error_shape_wraps_filesystem_slots_and_preserves_ioerror_identity() {
    let (mut env, owned) = system_error_env();

    assert_eq!(
        system_constructor_names(&env, &owned, "Transience"),
        ["Transient", "Permanent"]
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "Idempotence"),
        ["Idempotent", "NonIdempotent"]
    );
    assert_eq!(
        constructor_names(&env, "FileOperation"),
        [
            "OpReadFile",
            "OpWriteFile",
            "OpAppendFile",
            "OpMetadata",
            "OpReadDirectory",
            "OpCreateDirectory",
            "OpRemoveFile",
            "OpRemoveDirectory",
            "OpRename",
            "OpChangeMode",
            "OpSeek",
            "OpSetLength",
            "OpSync",
            "OpGetInheritance",
            "OpSetInheritance",
            "OpDuplicate",
        ]
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "RetryGuidance"),
        [
            "RetryAdvised",
            "RetryUnsafeNonIdempotent",
            "DoNotRetryPermanent",
        ]
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "Operation"),
        ["FilesystemOp"]
    );
    assert_eq!(
        system_inductive(&env, &owned, "Operation").constructors[0].args,
        [former(env.globals["FileOperation"])]
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "ResourceRef"),
        ["FilesystemResource"]
    );
    assert_eq!(
        system_inductive(&env, &owned, "ResourceRef").constructors[0].args,
        [Term::app(
            former(env.globals["Option"]),
            Term::const_(env.globals["Bytes"], vec![]),
        )]
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "SafeContext"),
        ["NoSafeContext", "RedactedSafeContext"]
    );
    assert!(
        system_inductive(&env, &owned, "SafeContext")
            .constructors
            .iter()
            .all(|constructor| constructor.args.is_empty()),
        "SafeContext must remain structurally unable to carry authority-sensitive bytes"
    );
    assert_eq!(
        system_constructor_names(&env, &owned, "SystemError"),
        ["MkSystemError"]
    );
    assert_eq!(
        system_inductive(&env, &owned, "SystemError").constructors[0].args,
        [
            former(system_id(&env, &owned, "Operation")),
            former(system_id(&env, &owned, "ResourceRef")),
            former(env.globals["IOError"]),
            former(system_id(&env, &owned, "SafeContext")),
        ]
    );

    assert!(
        !env.globals.contains_key("ErrorIdentity"),
        "INC1 reuses IOError rather than minting a colliding identity family"
    );
    let revoked = env.globals["Revoked"];
    assert_eq!(
        env.env
            .constructor(revoked)
            .expect("Revoked constructor")
            .0
            .id,
        env.globals["IOError"],
        "the canonical Revoked name must still belong to IOError"
    );

    env.elaborate_file(&format!(
        "{SYSTEM_ERROR_IMPORT}\n{}",
        r#"
const px9_error : SystemError =
  MkSystemError
    (FilesystemOp OpReadFile)
    (FilesystemResource (None Bytes))
    Revoked
    RedactedSafeContext

fn px9_error_identity (error : SystemError) : IOError =
  match error {
    MkSystemError _operation _resource identity _context ↦ identity
  }

theorem px9_error_identity_is_canonical :
  Equal IOError (px9_error_identity px9_error) Revoked = Proved
"#
    ))
    .expect("the reconciled SystemError construction and identity projection elaborate");
}

#[test]
fn total_classifiers_compute_the_complete_ruled_matrices() {
    let (mut env, owned) = system_error_env();
    env.elaborate_file(
        &format!("{SYSTEM_ERROR_IMPORT}\n{}", r#"
theorem px9_not_found : Equal Transience (error_transience NotFound) Permanent = Proved
theorem px9_permission_denied : Equal Transience (error_transience PermissionDenied) Permanent = Proved
theorem px9_capability_denied : Equal Transience (error_transience CapabilityDenied) Permanent = Proved
theorem px9_broken_pipe : Equal Transience (error_transience BrokenPipe) Permanent = Proved
theorem px9_interrupted : Equal Transience (error_transience Interrupted) Transient = Proved
theorem px9_already_exists : Equal Transience (error_transience AlreadyExists) Permanent = Proved
theorem px9_invalid_input : Equal Transience (error_transience InvalidInput) Permanent = Proved
theorem px9_is_directory : Equal Transience (error_transience IsDirectory) Permanent = Proved
theorem px9_not_directory : Equal Transience (error_transience NotDirectory) Permanent = Proved
theorem px9_not_empty : Equal Transience (error_transience NotEmpty) Permanent = Proved
theorem px9_unsupported : Equal Transience (error_transience Unsupported) Permanent = Proved
theorem px9_revoked : Equal Transience (error_transience Revoked) Permanent = Proved
const px9_other_identity : IOError = Other 0
theorem px9_other : Equal Transience (error_transience px9_other_identity) Permanent = Proved

theorem px9_read_file : Equal Idempotence (operation_idempotence (FilesystemOp OpReadFile)) Idempotent = Proved
theorem px9_write_file : Equal Idempotence (operation_idempotence (FilesystemOp OpWriteFile)) NonIdempotent = Proved
theorem px9_append_file : Equal Idempotence (operation_idempotence (FilesystemOp OpAppendFile)) NonIdempotent = Proved
theorem px9_metadata : Equal Idempotence (operation_idempotence (FilesystemOp OpMetadata)) Idempotent = Proved
theorem px9_read_directory : Equal Idempotence (operation_idempotence (FilesystemOp OpReadDirectory)) Idempotent = Proved
theorem px9_create_directory : Equal Idempotence (operation_idempotence (FilesystemOp OpCreateDirectory)) NonIdempotent = Proved
theorem px9_remove_file : Equal Idempotence (operation_idempotence (FilesystemOp OpRemoveFile)) NonIdempotent = Proved
theorem px9_remove_directory : Equal Idempotence (operation_idempotence (FilesystemOp OpRemoveDirectory)) NonIdempotent = Proved
theorem px9_rename : Equal Idempotence (operation_idempotence (FilesystemOp OpRename)) NonIdempotent = Proved
theorem px9_change_mode : Equal Idempotence (operation_idempotence (FilesystemOp OpChangeMode)) Idempotent = Proved
theorem px9_seek : Equal Idempotence (operation_idempotence (FilesystemOp OpSeek)) NonIdempotent = Proved
theorem px9_set_length : Equal Idempotence (operation_idempotence (FilesystemOp OpSetLength)) Idempotent = Proved
theorem px9_sync : Equal Idempotence (operation_idempotence (FilesystemOp OpSync)) Idempotent = Proved
theorem px9_get_inheritance : Equal Idempotence (operation_idempotence (FilesystemOp OpGetInheritance)) Idempotent = Proved
theorem px9_set_inheritance : Equal Idempotence (operation_idempotence (FilesystemOp OpSetInheritance)) Idempotent = Proved
theorem px9_duplicate : Equal Idempotence (operation_idempotence (FilesystemOp OpDuplicate)) NonIdempotent = Proved

theorem px9_transient_idempotent : Equal RetryGuidance (retry_guidance Transient Idempotent) RetryAdvised = Proved
theorem px9_transient_nonidempotent : Equal RetryGuidance (retry_guidance Transient NonIdempotent) RetryUnsafeNonIdempotent = Proved
theorem px9_permanent_idempotent : Equal RetryGuidance (retry_guidance Permanent Idempotent) DoNotRetryPermanent = Proved
theorem px9_permanent_nonidempotent : Equal RetryGuidance (retry_guidance Permanent NonIdempotent) DoNotRetryPermanent = Proved
"#),
    )
    .expect("all 13 identity, 16 operation, and four retry matrix cells must compute");

    for law in [
        "retry_guidance_transient_idempotent",
        "retry_guidance_transient_nonidempotent",
        "retry_guidance_permanent_idempotent",
        "retry_guidance_permanent_nonidempotent",
        "error_transience_revoked_permanent",
        "retry_guidance_idempotence_flip",
    ] {
        assert!(
            env.env
                .transparent_body(system_id(&env, &owned, law))
                .is_some(),
            "`{law}` must be a checked transparent proof"
        );
    }
}

#[test]
fn sanctioned_retry_surface_consumes_both_classification_axes() {
    let (env, owned) = system_error_env();
    let retry_guidance = system_id(&env, &owned, "RetryGuidance");
    let bool_type = env.globals["Bool"];
    let system_error = system_id(&env, &owned, "SystemError");
    let io_error = env.globals["IOError"];

    let mut guidance_producers = BTreeSet::new();
    let mut error_only_bool_functions = BTreeSet::new();
    for (name, id) in &env.globals {
        let Some(declaration) = env.env.lookup(*id) else {
            continue;
        };
        let ty = match declaration {
            Decl::Transparent { ty, .. } | Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
                ty
            }
            Decl::Inductive(_) => continue,
        };
        let (domains, result) = peel_pi(ty);
        let result_head = head_global(&result);
        if result_head == Some(retry_guidance) {
            assert!(
                owned.contains(id),
                "retry guidance producer {name} must be owned"
            );
            guidance_producers.insert(*id);
        }
        if result_head == Some(bool_type)
            && domains
                .iter()
                .filter_map(head_global)
                .any(|domain| domain == system_error || domain == io_error)
        {
            error_only_bool_functions.insert(name.clone());
        }
    }

    assert_eq!(
        guidance_producers,
        BTreeSet::from([system_id(&env, &owned, "retry_guidance")]),
        "retry_guidance is the sole sanctioned producer of retry advice"
    );
    assert!(
        error_only_bool_functions.is_empty(),
        "the surface must not expose an error-only Bool retry classifier: {error_only_bool_functions:?}"
    );
    assert_eq!(
        signature_heads(&env, &owned, "retry_guidance"),
        (
            vec![
                system_id(&env, &owned, "Transience"),
                system_id(&env, &owned, "Idempotence")
            ],
            retry_guidance,
        )
    );
    assert_eq!(
        signature_heads(&env, &owned, "error_transience"),
        (vec![io_error], system_id(&env, &owned, "Transience"))
    );
    assert_eq!(
        signature_heads(&env, &owned, "operation_idempotence"),
        (
            vec![system_id(&env, &owned, "Operation")],
            system_id(&env, &owned, "Idempotence"),
        )
    );
}
