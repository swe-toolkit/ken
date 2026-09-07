//! PX9-INC1 domain-general system errors and two-axis retry classification.
//!
//! Sources: PX9-C (`spec/40-effects/41-system-effects.md` §1.8) and the
//! Architect's PX9-INC1 D0 reconciliation ruling. These controls are durable
//! invariants except for the exact constructor inventories, which are normative
//! compatibility vectors: adding a domain is deliberately additive and must
//! extend the relevant inventory and total classifier together.

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{inductive::peel_app, inductive::peel_pi, Decl, GlobalId, Term};

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

fn signature_heads(env: &ElabEnv, name: &str) -> (Vec<GlobalId>, GlobalId) {
    let declaration = env
        .env
        .lookup(env.globals[name])
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

#[test]
fn system_error_shape_wraps_filesystem_slots_and_preserves_ioerror_identity() {
    let mut env = ElabEnv::new().expect("PX9-INC1 prelude");

    assert_eq!(
        constructor_names(&env, "Transience"),
        ["Transient", "Permanent"]
    );
    assert_eq!(
        constructor_names(&env, "Idempotence"),
        ["Idempotent", "NonIdempotent"]
    );
    assert_eq!(
        constructor_names(&env, "RetryGuidance"),
        [
            "RetryAdvised",
            "RetryUnsafeNonIdempotent",
            "DoNotRetryPermanent",
        ]
    );
    assert_eq!(constructor_names(&env, "Operation"), ["FilesystemOp"]);
    assert_eq!(
        inductive(&env, "Operation").constructors[0].args,
        [former(env.globals["FileOperation"])]
    );
    assert_eq!(
        constructor_names(&env, "ResourceRef"),
        ["FilesystemResource"]
    );
    assert_eq!(
        inductive(&env, "ResourceRef").constructors[0].args,
        [Term::app(
            former(env.globals["Option"]),
            Term::const_(env.globals["Bytes"], vec![]),
        )]
    );
    assert_eq!(
        constructor_names(&env, "SafeContext"),
        ["NoSafeContext", "RedactedSafeContext"]
    );
    assert!(
        inductive(&env, "SafeContext")
            .constructors
            .iter()
            .all(|constructor| constructor.args.is_empty()),
        "SafeContext must remain structurally unable to carry authority-sensitive bytes"
    );
    assert_eq!(constructor_names(&env, "SystemError"), ["MkSystemError"]);
    assert_eq!(
        inductive(&env, "SystemError").constructors[0].args,
        [
            former(env.globals["Operation"]),
            former(env.globals["ResourceRef"]),
            former(env.globals["IOError"]),
            former(env.globals["SafeContext"]),
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

    env.elaborate_file(
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
"#,
    )
    .expect("the reconciled SystemError construction and identity projection elaborate");
}

#[test]
fn total_classifiers_compute_the_complete_ruled_matrices() {
    let mut env = ElabEnv::new().expect("PX9-INC1 prelude");
    env.elaborate_file(
        r#"
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

theorem px9_transient_idempotent : Equal RetryGuidance (retry_guidance Transient Idempotent) RetryAdvised = Proved
theorem px9_transient_nonidempotent : Equal RetryGuidance (retry_guidance Transient NonIdempotent) RetryUnsafeNonIdempotent = Proved
theorem px9_permanent_idempotent : Equal RetryGuidance (retry_guidance Permanent Idempotent) DoNotRetryPermanent = Proved
theorem px9_permanent_nonidempotent : Equal RetryGuidance (retry_guidance Permanent NonIdempotent) DoNotRetryPermanent = Proved
"#,
    )
    .expect("all 13 identity, 10 operation, and four retry matrix cells must compute");

    for law in [
        "retry_guidance_transient_idempotent",
        "retry_guidance_transient_nonidempotent",
        "retry_guidance_permanent_idempotent",
        "retry_guidance_permanent_nonidempotent",
        "error_transience_revoked_permanent",
        "retry_guidance_idempotence_flip",
    ] {
        assert!(
            env.env.transparent_body(env.globals[law]).is_some(),
            "`{law}` must be a checked transparent proof"
        );
    }
}

#[test]
fn sanctioned_retry_surface_consumes_both_classification_axes() {
    let env = ElabEnv::new().expect("PX9-INC1 prelude");
    let retry_guidance = env.globals["RetryGuidance"];
    let bool_type = env.globals["Bool"];
    let system_error = env.globals["SystemError"];
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
            guidance_producers.insert(name.clone());
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
        BTreeSet::from(["retry_guidance".to_string()]),
        "retry_guidance is the sole sanctioned producer of retry advice"
    );
    assert!(
        error_only_bool_functions.is_empty(),
        "the surface must not expose an error-only Bool retry classifier: {error_only_bool_functions:?}"
    );
    assert_eq!(
        signature_heads(&env, "retry_guidance"),
        (
            vec![env.globals["Transience"], env.globals["Idempotence"]],
            retry_guidance,
        )
    );
    assert_eq!(
        signature_heads(&env, "error_transience"),
        (vec![io_error], env.globals["Transience"])
    );
    assert_eq!(
        signature_heads(&env, "operation_idempotence"),
        (vec![env.globals["Operation"]], env.globals["Idempotence"],)
    );
}
