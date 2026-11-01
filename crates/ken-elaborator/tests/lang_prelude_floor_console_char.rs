//! ConsoleStream/Char floor and exact-ID package-home boundaries (30 §4).
//!
//! Promise classes: normative compatibility vectors for the checked prelude
//! IDs and closed floor, and durable behavioral invariants for source binding.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const MODES: [&str; 3] = ["strict-roots", "legacy-roots", "legacy-file"];
const PACKAGED: [(&str, &str); 61] = [
    ("Capability.Console.Text", "IO"),
    ("Capability.Console.Text", "write"),
    ("Capability.Console.Text", "print_line"),
    ("Core.Classes.LawfulClasses", "charToInt"),
    ("Core.Classes.LawfulClasses", "leqChar"),
    ("Capability.Filesystem.Authority", "FS"),
    ("Capability.Filesystem.Authority", "read_bytes"),
    ("Capability.Filesystem.Authority", "write_file"),
    ("Capability.System.Buffer", "BufferWindow"),
    ("Capability.System.Buffer", "MkBufferWindow"),
    ("Capability.System.Buffer", "buffer_span_length"),
    ("Capability.System.Buffer", "buffer_span_budget"),
    ("Capability.System.Buffer", "transfer_count_int"),
    ("Capability.System.Buffer", "transfer_count_nat"),
    ("Capability.System.Buffer", "transfer_count_remaining"),
    ("Capability.System.Buffer", "transfer_count_positive"),
    ("Capability.System.Buffer", "transfer_count_positive_prop"),
    ("Capability.System.Buffer", "transfer_count_predecessor"),
    ("Capability.System.Buffer", "transfer_count_nat_succ"),
    ("Capability.System.Error", "FileOperation"),
    ("Capability.System.Error", "OpReadFile"),
    ("Capability.System.Error", "OpWriteFile"),
    ("Capability.System.Error", "OpAppendFile"),
    ("Capability.System.Error", "OpMetadata"),
    ("Capability.System.Error", "OpReadDirectory"),
    ("Capability.System.Error", "OpCreateDirectory"),
    ("Capability.System.Error", "OpRemoveFile"),
    ("Capability.System.Error", "OpRemoveDirectory"),
    ("Capability.System.Error", "OpRename"),
    ("Capability.System.Error", "OpChangeMode"),
    ("Capability.System.Error", "OpSeek"),
    ("Capability.System.Error", "OpSetLength"),
    ("Capability.System.Error", "OpSync"),
    ("Capability.System.Error", "OpGetInheritance"),
    ("Capability.System.Error", "OpSetInheritance"),
    ("Capability.System.Error", "OpDuplicate"),
    ("Capability.System.IO", "write_all_exact_prefix_prop"),
    (
        "Capability.System.IO",
        "write_all_exact_prefix_prop::exact_prefix",
    ),
    ("Capability.System.Resource", "ResourceBodyResult"),
    ("Capability.System.Resource", "ResourceBodyOk"),
    ("Capability.System.Resource", "ResourceBodyErr"),
    ("Capability.System.Resource", "ResourceBracketResult"),
    ("Capability.System.Resource", "ResourceBracketOk"),
    ("Capability.System.Resource", "ResourceBracketBodyError"),
    ("Capability.System.Resource", "ResourceBracketReleaseError"),
    (
        "Capability.System.Resource",
        "ResourceBracketBodyAndReleaseError",
    ),
    ("Capability.Process.Exit", "host_program"),
    ("Capability.Process.Exit", "host_program_then"),
    ("Capability.Process.Exit", "host_console"),
    ("Capability.System.Resource", "withResource"),
    ("Capability.System.Resource", "withBuffer"),
    ("Capability.System.Resource", "withMapping"),
    ("Capability.System.Resource", "release"),
    ("Capability.System.IO", "readAt"),
    ("Capability.System.IO", "spanBytes"),
    ("Capability.Filesystem.Authority", "readFile"),
    ("Capability.Filesystem.Authority", "writeFile"),
    ("Capability.System.Buffer", "BufferHandle"),
    ("Capability.Process.Exit", "host_fs"),
    ("Capability.Process.Exit", "host_pure"),
    ("Capability.Process.Exit", "host_bind"),
];

fn load(env: &mut ElabEnv, source: &str, mode: &str) -> Result<(), ElabError> {
    let root = tempfile::tempdir().expect("Console/Char source root");
    fs::write(root.path().join("Entry.ken"), source).expect("write client source");
    match mode {
        "strict-roots" => {
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")?;
        }
        "legacy-roots" => {
            env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")?;
        }
        "legacy-file" => {
            env.elaborate_file_v1(source)?;
        }
        _ => panic!("unknown source mode: {mode}"),
    }
    Ok(())
}

fn contains_id(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| contains_id(child, target)),
    }
}

fn checked_body(env: &ElabEnv, mode: &str, name: &str) -> Term {
    let qualified = if mode == "legacy-file" {
        name.to_string()
    } else {
        format!("Entry.{name}")
    };
    env.env
        .transparent_body(env.globals[&qualified])
        .unwrap_or_else(|| panic!("{qualified} has a checked body"))
        .1
        .clone()
}

fn write_home(root: &Path) {
    for (path, body) in [
        (
            "Capability/Console/Text.ken",
            "export IO, write, print_line",
        ),
        (
            "Core/Classes/LawfulClasses.ken",
            "export charToInt, leqChar",
        ),
    ] {
        let target = root.join(path);
        fs::create_dir_all(target.parent().expect("home parent")).expect("create package home");
        fs::write(target, body).expect("write exact-path package home");
    }
}

/// MEASURED: the two new floor entries name the old checked family/comparator
/// IDs; the carrier's three checked constructors have qualified keys and the
/// same parent as before, with no bare leaf alias or new trust. CLAIMED: the
/// precise floor closure is checked-ID-based. THE GAP: source resolution is
/// tested independently below, not inferred from `globals` key membership.
#[test]
fn console_stream_family_and_eq_char_keep_exact_checked_ids_without_trust() {
    let env = ElabEnv::new().expect("checked prelude");
    let floor = PRELUDE_FLOOR_NAMES.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        floor.len(),
        PRELUDE_FLOOR_NAMES.len(),
        "closed floor has no duplicate"
    );
    assert_eq!(
        floor.len(),
        37,
        "30 §4 floor count with five scoped families"
    );
    assert!(floor.contains("ConsoleStream") && floor.contains("eqChar"));
    assert!(!floor.contains("Stream"));
    for (_, name) in PACKAGED {
        assert!(!floor.contains(name), "{name}: package, never floor");
    }
    assert_eq!(PRELUDE_PACKAGE_HOMES.as_slice(), PACKAGED.as_slice());
    assert_eq!(env.globals["ConsoleStream"], env.prelude_env.stream_id);
    assert!(
        !env.globals.contains_key("Stream"),
        "no old source carrier alias"
    );
    let family = env
        .env
        .inductive(env.prelude_env.stream_id)
        .expect("checked carrier");
    assert_eq!(family.constructors.len(), 3);
    for (leaf, ctor) in ["Stdin", "Stdout", "Stderr"]
        .into_iter()
        .zip(&family.constructors)
    {
        assert_eq!(env.globals[&format!("ConsoleStream.{leaf}")], ctor.id);
        assert!(!env.globals.contains_key(leaf), "{leaf} is never bare");
        assert_eq!(
            env.env.constructor(ctor.id).expect("checked member").0.id,
            family.id
        );
    }
    assert!(env.env.lookup(env.globals["eqChar"]).is_some());
    assert!(!env.env.trusted_base().contains(&env.globals["eqChar"]));
}

/// MEASURED: all three source entry routes resolve admitted references to the
/// checked carrier, three qualified cases and eqChar IDs; wrong bare leaves
/// fail in each route. CLAIMED: floor
/// admission is complete, exact, and no bare constructor is reserved.
/// THE GAP: internal/package acceptance is the separate negative axis below.
#[test]
fn admitted_console_char_resolves_qualified_family_in_each_entry_mode() {
    for mode in MODES {
        let mut env = ElabEnv::new().expect("prelude");
        let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
        let carrier = env.globals["ConsoleStream"];
        let stdout = env.globals["ConsoleStream.Stdout"];
        let stdin = env.globals["ConsoleStream.Stdin"];
        let stderr = env.globals["ConsoleStream.Stderr"];
        let eq = env.globals["eqChar"];
        load(
            &mut env,
            "fn id (x : ConsoleStream) : ConsoleStream = x\n\
             const out : ConsoleStream = ConsoleStream.Stdout\n\
             const input : ConsoleStream = ConsoleStream.Stdin\n\
             const err : ConsoleStream = ConsoleStream.Stderr\n\
             const same : Bool = eqChar 65 65",
            mode,
        )
        .unwrap_or_else(|e| panic!("{mode} resolves exact floor family: {e:?}"));
        assert!(contains_id(&checked_body(&env, mode, "id"), carrier));
        for (name, id) in [
            ("out", stdout),
            ("input", stdin),
            ("err", stderr),
            ("same", eq),
        ] {
            assert!(
                contains_id(&checked_body(&env, mode, name), id),
                "{mode}: {name}"
            );
        }
        assert_eq!(before, env.env.trusted_base().into_iter().collect());
        for leaf in ["Stdin", "Stdout", "Stderr"] {
            let mut separate = ElabEnv::new().expect("prelude");
            let source = format!("const probe : ConsoleStream = {leaf}");
            let err = load(&mut separate, &source, mode).expect_err("bare leaf refused");
            assert!(format!("{err:?}").contains(leaf), "{mode}: {leaf}: {err:?}");
        }
    }
}

/// MEASURED: a source declaration of either newly admitted member is
/// refused in each entry mode; an unrelated one-token local remains legal.
/// CLAIMED: precisely the two checked names, not a spelling family, are
/// reserved. THE GAP: the checked use and constructor scope have an
/// independent positive row, so a broken parser cannot pass on rejection.
#[test]
fn admitted_console_char_refuse_same_name_declarations_in_each_entry_mode() {
    for mode in MODES {
        for (name, control) in [
            ("ConsoleStream", "LocalStream"),
            ("eqChar", "local_char_eq"),
        ] {
            let mut env = ElabEnv::new().expect("prelude");
            let source = format!("const {name} : Nat = Zero");
            let err = load(&mut env, &source, mode).expect_err("admitted shadow refused");
            assert!(format!("{err:?}").contains(name), "{mode}: {name}: {err:?}");
            let mut env = ElabEnv::new().expect("prelude");
            load(&mut env, &format!("const {control} : Nat = Zero"), mode)
                .unwrap_or_else(|e| panic!("{mode}: unrelated control must accept: {e:?}"));
        }
    }
}

/// MEASURED: strict and legacy source may declare the five packaged names,
/// the internal Stream, and each bare constructor spelling as unrelated fresh
/// local IDs, while the original checked definitions remain in the kernel.
/// CLAIMED: no over-reservation or name-based remapping of native roles.
/// THE GAP: package imports must additionally select their original IDs.
#[test]
fn package_internal_and_bare_leaf_names_are_not_reserved() {
    for mode in MODES {
        for name in [
            "IO",
            "write",
            "print_line",
            "charToInt",
            "leqChar",
            "Stream",
            "Stdin",
            "Stdout",
            "Stderr",
        ] {
            let base = ElabEnv::new().expect("prelude");
            let original = base.globals.get(name).copied();
            let mut env = ElabEnv::new().expect("prelude");
            let source = format!("const {name} : Nat = Zero");
            load(&mut env, &source, mode)
                .unwrap_or_else(|e| panic!("{mode}: {name} must be a legal local: {e:?}"));
            let local = if mode == "legacy-file" {
                name.to_string()
            } else {
                format!("Entry.{name}")
            };
            assert_ne!(
                Some(env.globals[&local]),
                original,
                "{name} never inherits original ID"
            );
        }
    }
}

/// MEASURED: a strict roots client accepts a same-name `ConsoleOp` declaration
/// and assigns it a fresh local ID, leaving the checked family untouched.
/// CLAIMED: the compiler-internal ConsoleOp is not admitted by family label.
/// THE GAP: a floor-addition mutation must reach this declaration and refuse
/// it here, not fail earlier during prelude initialization.
#[test]
fn compiler_internal_console_op_same_name_local_accepts_in_strict_roots() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let original = env.globals["ConsoleOp"];
    load(&mut env, "const ConsoleOp : Nat = Zero", "strict-roots")
        .expect("internal ConsoleOp leaves the same-name local available");
    assert_ne!(env.globals["Entry.ConsoleOp"], original);
    assert_eq!(env.globals["ConsoleOp"], original);
}

/// MEASURED: strict and legacy roots clients import all five exact-ID home
/// exports and use the ConsoleStream.Stdout family in `write`; their checked
/// bodies mention the pre-source IDs captured from a fresh environment.
/// CLAIMED: the home grant is an import edge, not a spelling alias or trust.
/// THE GAP: refusals without that edge and a mutable-global takeover are
/// separate tests; the real Console.Text home also has its own client suite.
fn check_home_imports_select_all_five(mode: &str) {
    let root = tempfile::tempdir().expect("roots");
    write_home(root.path());
    fs::write(
        root.path().join("Entry.ken"),
        "import Capability.Console.Text (IO, write, print_line)\n\
         import Core.Classes.LawfulClasses (charToInt, leqChar)\n\
         proc send (payload : Bytes) : IO (Result IOError Unit) visits [Console] =\n\
           write ConsoleStream.Stdout payload\n\
         proc line (message : String) : IO Unit visits [Console] =\n\
           print_line message\n\
         const char_code : Int = charToInt 65\n\
         const char_order : Bool = leqChar 65 66",
    )
    .expect("client");
    let mut env = ElabEnv::new().expect("prelude");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let expected = PACKAGED.map(|(_, name)| (name, env.globals[name]));
    let stdout = env.globals["ConsoleStream.Stdout"];
    match mode {
        "strict-roots" => {
            env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
                .expect("strict home and client load");
        }
        "legacy-roots" => {
            env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")
                .expect("legacy home and client load");
        }
        _ => panic!("unknown home import mode: {mode}"),
    }
    for (source, id) in [
        ("Entry.send", expected[1].1),
        ("Entry.line", expected[2].1),
        ("Entry.char_code", expected[3].1),
        ("Entry.char_order", expected[4].1),
    ] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[source])
            .expect("checked client body");
        assert!(
            contains_id(&body, id),
            "{source} must select the pre-source ID"
        );
    }
    assert!(contains_id(&checked_body(&env, mode, "send"), stdout));
    let (_, ty) = env
        .env
        .const_type(env.globals["Entry.send"])
        .expect("send type");
    assert!(contains_id(&ty, expected[0].1), "IO in checked result type");
    assert_eq!(before, env.env.trusted_base().into_iter().collect());
}

#[test]
fn strict_home_imports_select_all_five_pre_source_ids() {
    check_home_imports_select_all_five("strict-roots");
}

#[test]
fn legacy_home_imports_select_all_five_pre_source_ids() {
    check_home_imports_select_all_five("legacy-roots");
}

/// MEASURED: a strict client without the right import cannot select any of
/// the five package identities, and a non-home file cannot publish any of
/// them. CLAIMED: the home grant creates neither ambient fallback nor an
/// arbitrary publisher. THE GAP: the all-name imported client is the paired
/// positive, and the grant-removal mutation must redden that positive row.
#[test]
fn strict_unimported_and_non_home_publisher_refuse_each_package_identity() {
    for (name, source) in [
        ("IO", "fn no_io (x : IO Nat) : IO Nat = x"),
        ("write", "const no_write = write"),
        ("print_line", "const no_print = print_line"),
        ("charToInt", "const no_code : Int = charToInt 65"),
        ("leqChar", "const no_order : Bool = leqChar 65 66"),
    ] {
        let mut missing = ElabEnv::new().expect("prelude");
        let err = load(&mut missing, source, "strict-roots")
            .expect_err("package name is unavailable without import");
        assert!(
            matches!(
                err,
                ElabError::UnboundName { .. } | ElabError::UnresolvedCon { .. }
            ),
            "{name}: unexpected failure: {err:?}"
        );
        assert!(format!("{err:?}").contains(name), "{name}: {err:?}");
        let mut non_home = ElabEnv::new().expect("prelude");
        let err = load(&mut non_home, &format!("export {name}"), "strict-roots")
            .expect_err("non-home cannot publish a pre-source package identity");
        assert!(
            matches!(
                err,
                ElabError::UnboundName { .. } | ElabError::UnresolvedCon { .. }
            ),
            "{name}: unexpected export failure: {err:?}"
        );
        assert!(format!("{err:?}").contains(name), "{name}: {err:?}");
    }
}

/// MEASURED: a checked same-name local in the exact home cannot silently
/// override its granted compiler ID. CLAIMED: export/local ambiguity fails
/// closed rather than silently rewriting the package's checked identity.
/// THE GAP: the ordinary no-local home succeeds in the five-name client row.
#[test]
fn home_local_collision_with_granted_write_refuses_as_ambiguous() {
    let root = tempfile::tempdir().expect("roots");
    fs::create_dir_all(root.path().join("Capability/Console")).expect("home directory");
    fs::write(
        root.path().join("Capability/Console/Text.ken"),
        "export IO, write, print_line\nconst write : Nat = Zero",
    )
    .expect("home collision source");
    let mut env = ElabEnv::new().expect("prelude");
    let original = env.globals["write"];
    let err = env
        .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Capability.Console.Text")
        .expect_err("home's local may not replace a granted checked identity");
    assert!(
        matches!(err, ElabError::AmbiguousReference { .. }),
        "{err:?}"
    );
    assert_eq!(env.globals["write"], original);
}

/// MEASURED: after a legacy declaration takes over flat `globals["write"]`,
/// a roots-loaded client still imports the pre-source write identity. CLAIMED:
/// checked provider identity, not a mutable global name, controls selection.
/// THE GAP: strict import checks separately ensure no bare fallback exists.
#[test]
fn legacy_takeover_does_not_redirect_home_imported_write() {
    let root = tempfile::tempdir().expect("roots");
    write_home(root.path());
    fs::write(
        root.path().join("Entry.ken"),
        "import Capability.Console.Text (IO, write)\n\
         proc send (payload : Bytes) : IO (Result IOError Unit) visits [Console] =\n\
           write ConsoleStream.Stdout payload",
    )
    .expect("client");
    let mut env = ElabEnv::new().expect("prelude");
    let original = env.globals["write"];
    env.elaborate_file_v1("const write : Nat = Zero")
        .expect("legacy source may shadow a package identity");
    let shadow = env.globals["write"];
    assert_ne!(original, shadow);
    env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")
        .expect("legacy roots import remains exact-ID keyed");
    let body = checked_body(&env, "legacy-roots", "send");
    assert!(contains_id(&body, original));
    assert!(!contains_id(&body, shadow));
}
