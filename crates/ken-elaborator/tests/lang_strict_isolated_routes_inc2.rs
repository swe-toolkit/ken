//! Increment 2 homes and floor identities on Strict isolated entries.
//! Spec: 30 §4, 33 §3.2.1, 36 §4.5 and 38; frame AC-2a–e.

use std::path::PathBuf;

use ken_elaborator::modules::PRELUDE_PACKAGE_HOMES;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

fn catalog() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn term_uses(term: &Term, target: ken_kernel::GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        _ => term
            .children()
            .into_iter()
            .any(|child| term_uses(child, target)),
    }
}

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new()?;
    env.elaborate_isolated_file_strict_v1(&[catalog()], source)?;
    Ok(env)
}

/// Promise class: durable invariant. Every newly homed checked identity can
/// be selected from its one provider; no bare unimported spelling is Strict.
/// Five explicit provider rows are checked against the production inventory;
/// every member has its own omission/refusal control.
#[test]
fn all_new_home_identities_select_only_their_own_provider() {
    // Compatibility vector from Increment 2's named home contract, not a
    // slice whose offsets shift when an unrelated package acquires a home.
    let promised: [(&str, &[&str]); 5] = [
        (
            "Capability.Process.Exit",
            &[
                "host_program",
                "host_program_then",
                "host_console",
                "host_fs",
                "host_pure",
                "host_bind",
            ],
        ),
        (
            "Capability.System.Resource",
            &["withResource", "withBuffer", "withMapping", "release"],
        ),
        ("Capability.System.IO", &["readAt", "spanBytes"]),
        (
            "Capability.Filesystem.Authority",
            &["readFile", "writeFile"],
        ),
        ("Capability.System.Buffer", &["BufferHandle"]),
    ];
    for (home, names) in promised {
        for name in names {
            let providers = PRELUDE_PACKAGE_HOMES
                .iter()
                .filter_map(|(owner, member)| (*member == *name).then_some(*owner))
                .collect::<Vec<_>>();
            assert_eq!(providers, [home], "{name} has one promised provider");
        }
        let import = format!(
            "import {home} ({})\nexport {}\n",
            names.join(", "),
            names.join(", ")
        );
        strict(&import).unwrap_or_else(|err| panic!("all {home} exports select: {err:?}"));
        for name in names {
            let missing = format!("const probe : Nat = {name}\n");
            match strict(&missing) {
                Err(ElabError::UnboundName { name: refused, .. }) => {
                    assert_eq!(&refused, name, "{home}: no ambient fallback")
                }
                Err(other) => panic!("{home}.{name}: wrong refusal: {other:?}"),
                Ok(_) => panic!("{home}.{name}: bare name accepted without import"),
            }
        }
    }
}

/// Promise class: durable invariant. The exact checked HostIO trio is
/// imported from Exit, not a source-local or ambient alias. The strict
/// program refers to both the lift and the sequencing definition, while a
/// matched root lacking the Exit import refuses on host_bind alone. `ken run`
/// observes both actions in the separate CLI binary.
#[test]
fn strict_hostio_composition_uses_checked_exit_import_without_new_trust() {
    let imports = "import Capability.Console.Text (print_line)\n\
                   import Capability.Process.Exit (host_bind, host_pure, host_console)\n";
    let body = "proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n\
                : HostIO APartial ExitCode visits [Console] =\n\
                  host_bind APartial Unit ExitCode\n\
                    (host_console APartial Unit (print_line \"one\"))\n\
                    (\\u. host_bind APartial Unit ExitCode\n\
                      (host_console APartial Unit (print_line \"two\"))\n\
                      (\\v. host_pure APartial ExitCode ExitCode.Success))\n";
    let mut env = ElabEnv::new().unwrap();
    let baseline = env.env.trusted_base();
    let host_bind = env.globals["host_bind"];
    let host_pure = env.globals["host_pure"];
    assert!(matches!(
        env.env.lookup(host_bind),
        Some(Decl::Transparent { .. })
    ));
    assert!(matches!(
        env.env.lookup(host_pure),
        Some(Decl::Transparent { .. })
    ));
    let source = format!("program capabilities FS APartial\n{imports}{body}");
    env.elaborate_isolated_file_strict_v1(&[catalog()], &source)
        .expect("strict imported HostIO sequencing");
    let (_, main) = env.env.transparent_body(env.globals["main"]).unwrap();
    assert!(term_uses(&main, host_bind));
    assert!(term_uses(&main, host_pure));
    assert_eq!(
        env.env.trusted_base(),
        baseline,
        "HostIO trio adds no trust"
    );
    let absent = format!(
        "program capabilities FS APartial\n\
        import Capability.Console.Text (print_line)\n{body}"
    );
    match strict(&absent) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "host_bind"),
        Err(other) => panic!("without Exit import wrong refusal: {other:?}"),
        Ok(_) => panic!("HostIO sequencing leaked into Strict ambient scope"),
    }
}

/// Promise class: durable invariant. The abstract BufferHandle type is
/// imported by exact identity, but its private constructor remains absent.
/// BufferWindow is a public complete family, not an abstract-home escape.
#[test]
fn buffer_handle_imports_as_type_without_a_constructor_lease() {
    let source = "import Capability.System.Buffer (BufferHandle)\n\
                  fn keep (handle : BufferHandle) : BufferHandle = handle\n";
    let mut env = ElabEnv::new().unwrap();
    let parent = env.globals["BufferHandle"];
    let private_ctor = env.env.inductive(parent).unwrap().constructors[0].id;
    let before = env.env.trusted_base();
    env.elaborate_isolated_file_strict_v1(&[catalog()], source)
        .expect("abstract family type imports");
    assert_eq!(env.globals["BufferHandle"], parent);
    assert_eq!(env.env.trusted_base(), before);
    assert!(!env.globals.values().any(|id| *id == private_ctor));
    match strict(
        "import Capability.System.Buffer (BufferHandle)\n\
                  fn forge (r : Resource ResourceKind.Buffer) : BufferHandle =\n\
                    BufferHandle.PrivateBufferHandle r 16\n",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(name.contains("PrivateBufferHandle"), "{name}")
        }
        Err(other) => panic!("private constructor: wrong refusal {other:?}"),
        Ok(_) => panic!("private constructor became source-visible"),
    }
}

/// Promise class: durable invariant. Strict users can name all five checked
/// parent identities and only the qualified constructor paths. A source `get`
/// and `put` remains locally nameable, distinct from State's specific floor.
#[test]
fn scoped_family_and_state_bindings_are_checked_and_non_generic() {
    let env = strict(
        "fn read_result (p : ReadProgress) : Bool =\n\
           match p { ReadProgress.ReadEof ↦ True; ReadProgress.ReadSome _ _ ↦ False }\n\
         fn write_result (p : WriteProgress) : Bool =\n\
           match p { WriteProgress.Wrote _ ↦ True }\n\
         fn open_mode (p : ResourceOpenMode) : Bool =\n\
           match p { ResourceOpenMode.ResourceRead ↦ True;\n\
                     ResourceOpenMode.ResourceMetadata ↦ False;\n\
                     ResourceOpenMode.ResourceWriteCreate _ ↦ False }\n\
         fn mapping (p : MappingProt) : Bool =\n\
           match p { MappingProt.ReadOnly ↦ True; MappingProt.ReadWrite ↦ False }\n\
         fn child (p : ChildExit) : Bool =\n\
           match p { ChildExit.Exited _ ↦ True; ChildExit.Signaled _ ↦ False }\n\
         fn get (x : Nat) : Nat = x\n\
         fn put (x : Nat) : Nat = get x\n\
         fn resp (u : Unit) : Type = Unit\n\
         const state_read : ITree Unit resp (Pair Nat Nat) =\n\
           run_state Nat Unit resp Nat Zero (state_get Nat Unit resp MkUnit)\n\
         const state_write : ITree Unit resp (Pair Unit Nat) =\n\
           run_state Nat Unit resp Unit Zero (state_put Nat Unit resp Zero)\n",
    )
    .expect("five scoped families and three State floor names");
    for name in ["state_get", "state_put", "run_state"] {
        let p = ElabEnv::new().unwrap();
        let expected = p.globals[name];
        assert_eq!(env.globals[name], expected);
        assert!(env.env.lookup(expected).is_some());
    }
    assert_ne!(env.globals["get"], env.globals["state_get"]);
    assert_ne!(env.globals["put"], env.globals["state_put"]);
    for (family, leaf) in [
        ("ReadProgress", "ReadEof"),
        ("WriteProgress", "Wrote"),
        ("ResourceOpenMode", "ResourceRead"),
        ("MappingProt", "ReadOnly"),
        ("ChildExit", "Exited"),
    ] {
        let parent = env.globals[family];
        let ctor = env.globals[&format!("{family}.{leaf}")];
        assert_eq!(env.env.constructor(ctor).unwrap().0.id, parent);
        assert!(!env.globals.contains_key(leaf), "{leaf} must not be bare");
        strict(&format!("const {leaf} : Nat = Zero\n"))
            .unwrap_or_else(|err| panic!("unrelated local {leaf} refused: {err:?}"));
    }
}
