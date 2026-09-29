//! PX9-INC2A's permanent filesystem-to-system error injection.
//!
//! Source: the Architect's corrected injection design after the INC2A D0
//! consumer census. These controls are durable invariants: later error domains
//! extend `Operation` and `ResourceRef` without changing `FileError` or this
//! injection.

use ken_elaborator::ElabEnv;
use ken_kernel::{inductive::peel_app, inductive::peel_pi, Decl, GlobalId, Term};

#[path = "support/catalog_or.rs"]
mod catalog_or;

const SYSTEM_ERROR: &str = "System.Error";
const SYSTEM_ERROR_IMPORT: &str = "import System.Error (SystemError, MkSystemError, \
    FilesystemOp, FilesystemResource, NoSafeContext, file_error_to_system)";

fn system_id(env: &ElabEnv, owned: &[GlobalId], name: &str) -> GlobalId {
    catalog_or::provider_owned_id(env, owned, SYSTEM_ERROR, name)
        .unwrap_or_else(|error| panic!("System.Error owner {name}: {error}"))
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

#[test]
fn bridge_is_total_transparent_and_preserves_every_file_error_field() {
    let mut env = ElabEnv::new().expect("prelude");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], SYSTEM_ERROR)
        .expect("System.Error must roots-load");

    let file_error = env.globals["FileError"];
    let system_error = system_id(&env, &owned, "SystemError");
    let bridge = system_id(&env, &owned, "file_error_to_system");
    let bridge_decl = env.env.lookup(bridge).expect("bridge declaration");
    let bridge_ty = match bridge_decl {
        Decl::Transparent { ty, .. } => ty,
        other => panic!("file_error_to_system must be transparent, got {other:?}"),
    };
    let (domains, result) = peel_pi(bridge_ty);
    assert_eq!(
        domains.len(),
        1,
        "the bridge must consume exactly FileError"
    );
    assert_eq!(head_global(&domains[0]), Some(file_error));
    assert_eq!(head_global(&result), Some(system_error));
    assert!(
        !env.env.trusted_base().contains(&bridge),
        "an ordinary total bridge must not enter the trusted base"
    );

    let file_error_decl = env
        .env
        .inductive(file_error)
        .expect("FileError must remain an inductive");
    assert_eq!(file_error_decl.constructors.len(), 1);
    assert_eq!(
        file_error_decl.constructors[0].args,
        [
            former(env.globals["FileOperation"]),
            Term::app(
                former(env.globals["Option"]),
                Term::const_(env.globals["Bytes"], vec![]),
            ),
            former(env.globals["IOError"]),
        ],
        "FileError remains the unchanged filesystem-domain carrier"
    );

    env.elaborate_file(&format!(
        "{SYSTEM_ERROR_IMPORT}\n{}",
        r#"
theorem px9_file_error_injection
  (operation : FileOperation)
  (resource : Option Bytes)
  (identity : IOError) :
  Equal SystemError
    (file_error_to_system (MkFileError operation resource identity))
    (MkSystemError
      (FilesystemOp operation)
      (FilesystemResource resource)
      identity
      NoSafeContext) = Refl
"#
    ))
    .expect("the bridge must compute for every FileError and preserve all fields");
}
