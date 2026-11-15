#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::error::ElabError;
use ken_elaborator::ElabEnv;

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(
        &[catalog_or::catalog_root()],
        "Capability.Filesystem.Authority",
    )
    .expect("Capability.Filesystem.Authority must roots-load");
    env.execute_loaded_entry_checked_fences("Capability.Filesystem.Authority")
        .expect("Capability.Filesystem.Authority checked fences keep their contract");
    env
}

fn capability_program(header: &str) -> String {
    format!(
        "{header}\n\
         import Capability.Filesystem.Authority (FS, readFile)\n\
         proc main (caps : ProgramCaps APartial) (path : Bytes)\n\
           : FS APartial (Result FileError Bytes) visits [FS] =\n\
           match caps {{\n\
             ProgramCaps.MkProgramCaps cap ↦ readFile APartial cap path\n\
           }}\n"
    )
}

fn authority_caller(authority: &str) -> String {
    format!(
        "import Capability.Filesystem.Authority (FS, writeFile)\n\
         proc authority_caller (cap : Cap {authority}) (path : Bytes)\n\
           (policy : CreatePolicy) (contents : Bytes)\n\
           : FS AFull (Result FileError Unit) visits [FS] =\n\
           writeFile cap path policy contents\n"
    )
}

#[test]
fn capability_filesystem_authority_fragment_elaborates() {
    let _env = mk_env();
}

#[test]
fn filesystem_program_requires_the_declared_capability_and_accepts_its_twin() {
    let source_without_capability = capability_program("program");
    let mut rejected_env = mk_env();
    match rejected_env.elaborate_file(&source_without_capability) {
        Err(ElabError::MissingCapability { effect, .. }) => {
            assert_eq!(effect, "FS");
        }
        other => panic!("expected MissingCapability(FS), got {other:?}"),
    }

    let source_with_capability = capability_program("program capabilities FS APartial");
    let mut accepted_env = mk_env();
    accepted_env
        .elaborate_file(&source_with_capability)
        .expect("the identical program with its FS capability must elaborate");
}

#[test]
fn full_authority_index_rejects_partial_and_accepts_full() {
    let mut rejected_env = mk_env();
    let wrong = authority_caller("APartial");
    assert!(
        matches!(
            rejected_env.elaborate_file(&wrong),
            Err(ElabError::KernelRejected {
                error: ken_kernel::KernelError::TypeMismatch { .. },
                ..
            })
        ),
        "Cap APartial must not satisfy a Cap AFull parameter"
    );

    let mut accepted_env = mk_env();
    let correct = authority_caller("AFull");
    accepted_env
        .elaborate_file(&correct)
        .expect("Cap AFull must satisfy the Cap AFull parameter");
}
