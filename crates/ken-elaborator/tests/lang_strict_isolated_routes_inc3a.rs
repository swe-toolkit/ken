//! LANG-STRICT-ISOLATED-ROUTES Inc 3a vocabulary at the strict source entry.
//! Normative identities: 30 §4 and 38 §1.12; no check/run/build route switch.

use std::path::PathBuf;

use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{normalize, Context, Term};

fn catalog() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalog/packages")
        .canonicalize()
        .expect("catalog root")
}

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new()?;
    env.elaborate_isolated_file_strict_v1(&[catalog()], source)?;
    Ok(env)
}

/// Promise class: durable invariant. MEASURED: every new H name resolves at
/// its original checked ID through one explicitly named unit, adds no trust
/// beyond loading that unit, and refuses without import. CLAIMED: the extra homes do not
/// expand the ambient floor or mint lookalikes. THE GAP: source users with
/// missing catalog roots are measured separately; this pin exercises the
/// strict isolated loader with an explicit catalog root.
#[test]
fn ruled_h_names_import_at_the_original_checked_ids_only() {
    let bootstrap = ElabEnv::new().expect("pre-source checked identities");
    let names: [(&str, &[&str]); 10] = [
        (
            "Capability.Filesystem.Authority",
            &[
                "append_file",
                "change_mode",
                "create_directory",
                "read_directory",
                "remove_directory",
                "remove_file",
                "rename_file",
                "file_metadata",
            ],
        ),
        ("Capability.System.IO", &["writeAt", "writeAll"]),
        ("Capability.System.Buffer", &["freeze"]),
        ("Capability.System.Resource", &["resourceMetadata"]),
        ("Capability.Console.Text", &["read", "flush", "is_terminal"]),
        ("Core.Classes.LawfulClasses", &["intToChar"]),
        ("Core.Effect.Coproduct", &["inject_l", "inject_r"]),
        ("Capability.Time.WallClock", &["wall_now"]),
        ("Capability.Process.Exit", &["host_clock"]),
        (
            "Data.Numeric.FixedWidth",
            &[
                "intToInt8",
                "intToInt16",
                "intToInt32",
                "intToInt64",
                "intToUInt8",
                "intToUInt16",
                "intToUInt32",
                "intToUInt64",
            ],
        ),
    ];
    for (home, members) in names {
        // Importing LawfulClasses can bring its own pre-existing law axioms;
        // homing a checked conversion must not add trust beyond loading that
        // same module. Compare selected and unselected imports of the home.
        let loaded = strict(&format!("import {home}\n"))
            .unwrap_or_else(|err| panic!("loading {home} refused: {err:?}"));
        let home_trust = loaded.env.trusted_base();
        for name in members {
            let providers = PRELUDE_PACKAGE_HOMES
                .iter()
                .filter_map(|(owner, member)| (*member == *name).then_some(*owner))
                .collect::<Vec<_>>();
            assert_eq!(providers, [home], "{name} has its one explicit home");
            assert!(!PRELUDE_FLOOR_NAMES.contains(name), "{name} is not floor");
            let with_import = format!("import {home} ({name})\nexport {name}\n");
            let imported = strict(&with_import)
                .unwrap_or_else(|err| panic!("importing {home}.{name} refused: {err:?}"));
            assert_eq!(
                imported.globals[*name], bootstrap.globals[*name],
                "{name}: exact ID"
            );
            assert_eq!(
                imported.env.trusted_base(),
                home_trust,
                "{name}: no additional trust beyond its package load"
            );
            match strict(&format!("export {name}\n")) {
                Err(ElabError::UnboundName { name: missing, .. }) => assert_eq!(missing, *name),
                Err(err) => panic!("{name}: wrong no-import refusal: {err:?}"),
                Ok(_) => panic!("{name} available without import"),
            }
        }
    }
}

/// Promise class: durable invariant. MEASURED: both resolution modes bind the
/// new specific `itree_bind` spelling to the exact captured bind ID; Strict
/// reserves that spelling and `resp_coproduct` but refuses bare `bind` unless
/// a user defines it, while Legacy still accepts the original bare spelling.
/// CLAIMED: direct ITree sequencing is source-nameable without reserving an
/// unrelated package's `bind`. THE GAP: this checks checked-source binding and
/// identical trusted-base sets, not interpreter or native execution.
#[test]
fn generic_effect_floor_uses_captured_id_and_leaves_bare_bind_for_users() {
    let baseline = ElabEnv::new().expect("checked prelude");
    let bind = baseline.prelude_env.bind_id;
    assert_eq!(baseline.globals["itree_bind"], bind);
    assert_eq!(baseline.globals["bind"], bind, "Legacy binding survives");
    assert_eq!(
        baseline.globals["resp_coproduct"],
        baseline.prelude_env.resp_coproduct_id
    );
    for name in ["itree_bind", "resp_coproduct"] {
        assert!(PRELUDE_FLOOR_NAMES.contains(&name));
        assert!(PRELUDE_PACKAGE_HOMES
            .iter()
            .all(|(_, member)| *member != name));
    }
    assert!(!PRELUDE_FLOOR_NAMES.contains(&"bind"));
    let source = "fn bind_tree (tree : ITree ConsoleOp console_resp Unit)
                  : ITree ConsoleOp console_resp Unit =
                    itree_bind ConsoleOp console_resp Unit Unit tree
                      (λv. ITree.Ret ConsoleOp console_resp Unit v)
                  fn response (op : Coproduct (StateOp Nat) ConsoleOp) : Type =
                    resp_coproduct (StateOp Nat) ConsoleOp (resp_state Nat) console_resp op\n";
    let selected = strict(source).expect("specific ITree bind and response in Strict");
    assert_eq!(selected.globals["itree_bind"], bind);
    assert_eq!(selected.env.trusted_base(), baseline.env.trusted_base());
    let mut legacy = ElabEnv::new().expect("checked prelude");
    legacy
        .elaborate_file_v1(source)
        .expect("the specific ITree bind is also available in Legacy");
    let mut bare_legacy = ElabEnv::new().expect("checked prelude");
    bare_legacy
        .elaborate_file_v1(&source.replace("itree_bind", "bind"))
        .expect("Legacy bare bind retains the checked ID");
    match strict(&source.replace("itree_bind", "bind")) {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, "bind"),
        Err(error) => panic!("wrong bare bind refusal: {error:?}"),
        Ok(_) => panic!("bare bind leaked into the strict floor"),
    }
    let shadow = strict("fn bind (n : Nat) : Nat = n\nfn apply (n : Nat) : Nat = bind n\n")
        .expect("user-defined bind is not reserved");
    assert_ne!(
        shadow.globals["bind"], bind,
        "the user's bind is a new definition"
    );
    assert_eq!(
        shadow.globals["itree_bind"], bind,
        "the user cannot redirect the floor alias"
    );
}

/// Promise class: normative compatibility vector. MEASURED: seven admitted families
/// and every scoped case stay at the native-captured inductive IDs, a strict
/// source can name every case, bare source leaves refuse, and trust is fixed.
/// CLAIMED: source spelling changes without changing the native reply IDs.
/// THE GAP: this checks the captured identities' source bindings, not
/// interpreter/native execution of every operation.
#[test]
fn admitted_scoped_families_expose_only_qualified_checked_constructors() {
    let baseline = ElabEnv::new().expect("pre-source checked identities");
    let roles = &baseline.prelude_env.runtime_roles;
    for (path, native_id) in [
        ("ReadResult.Chunk", baseline.prelude_env.chunk_id),
        ("StateOp.Get", baseline.prelude_env.get_id),
        ("StateOp.Put", baseline.prelude_env.put_id),
        ("ConsoleOp.Read", roles.op_console_read),
        ("ConsoleOp.Write", roles.op_console_write),
        ("ConsoleOp.Flush", roles.op_console_flush),
        ("ConsoleOp.IsTerminal", roles.op_console_is_terminal),
        ("ReadResult.Eof", baseline.prelude_env.eof_id),
        ("FileKind.KFile", roles.file_kind_file),
        ("FileKind.KDirectory", roles.file_kind_directory),
        ("FileKind.KSymlink", roles.file_kind_symlink),
        ("FileKind.KOther", roles.file_kind_other),
        ("FileMetadata.MkFileMetadata", roles.file_metadata),
        ("DirEntry.MkDirEntry", roles.dir_entry),
        ("FileOperation.OpReadFile", roles.file_operation_read),
        ("FileOperation.OpWriteFile", roles.file_operation_write),
        ("FileOperation.OpAppendFile", roles.file_operation_append),
        ("FileOperation.OpMetadata", roles.file_operation_metadata),
        (
            "FileOperation.OpReadDirectory",
            roles.file_operation_read_directory,
        ),
        (
            "FileOperation.OpCreateDirectory",
            roles.file_operation_create_directory,
        ),
        (
            "FileOperation.OpRemoveFile",
            roles.file_operation_remove_file,
        ),
        (
            "FileOperation.OpRemoveDirectory",
            roles.file_operation_remove_directory,
        ),
        ("FileOperation.OpRename", roles.file_operation_rename),
        (
            "FileOperation.OpChangeMode",
            roles.file_operation_change_mode,
        ),
        ("FileOperation.OpSeek", roles.file_operation_seek),
        ("FileOperation.OpSetLength", roles.file_operation_set_length),
        ("FileOperation.OpSync", roles.file_operation_sync),
        (
            "FileOperation.OpGetInheritance",
            roles.file_operation_get_inheritance,
        ),
        (
            "FileOperation.OpSetInheritance",
            roles.file_operation_set_inheritance,
        ),
        ("FileOperation.OpDuplicate", roles.file_operation_duplicate),
    ] {
        assert_eq!(
            baseline.globals[path], native_id,
            "{path}: native captured ID"
        );
    }
    let mut source = String::new();
    for (family, members) in [
        ("ReadResult", vec!["Chunk", "Eof"]),
        ("StateOp", vec!["Get", "Put"]),
        ("ConsoleOp", vec!["Read", "Write", "Flush", "IsTerminal"]),
        (
            "FileKind",
            vec!["KFile", "KDirectory", "KSymlink", "KOther"],
        ),
        ("FileMetadata", vec!["MkFileMetadata"]),
        ("DirEntry", vec!["MkDirEntry"]),
        (
            "FileOperation",
            vec![
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
            ],
        ),
    ] {
        assert!(PRELUDE_FLOOR_NAMES.contains(&family), "{family} is floor");
        let id = baseline.globals[family];
        let checked = baseline.env.inductive(id).expect("checked family");
        assert_eq!(
            checked.constructors.len(),
            members.len(),
            "complete family {family}"
        );
        if family == "StateOp" {
            source.push_str("fn id_state_op (s : Type) (op : StateOp s) : StateOp s = op\n");
        } else {
            source.push_str(&format!("fn id_{family} (x : {family}) : {family} = x\n"));
        }
        for (index, (member, ctor)) in members.into_iter().zip(&checked.constructors).enumerate() {
            let path = format!("{family}.{member}");
            assert_eq!(baseline.globals[&path], ctor.id, "{path} checked ID");
            assert!(
                !baseline.globals.contains_key(member),
                "{member} is not bare"
            );
            assert_eq!(
                baseline
                    .env
                    .constructor(ctor.id)
                    .expect("checked ctor")
                    .0
                    .id,
                id
            );
            if matches!(family, "FileKind" | "FileOperation") {
                source.push_str(&format!(
                    "const case_{family}_{index} : {family} = {path}\n"
                ));
            }
        }
    }
    source.push_str(
        "const chunk : ReadResult = ReadResult.Chunk (bytes_encode \"x\")\n\
         const eof : ReadResult = ReadResult.Eof\n\
         const metadata : FileMetadata = FileMetadata.MkFileMetadata 1 FileKind.KFile\n\
         const entry : DirEntry = DirEntry.MkDirEntry (bytes_encode \"n\") FileKind.KFile\n\
         const get : StateOp Nat = StateOp.Get Nat\n\
         const put : StateOp Nat = StateOp.Put Nat Zero\n\
         fn response (op : StateOp Nat) : Type = resp_state Nat op\n\
         const console_read : ConsoleOp = ConsoleOp.Read ConsoleStream.Stdin 1\n\
         const console_write : ConsoleOp = ConsoleOp.Write ConsoleStream.Stdout (bytes_encode \"x\")\n\
         const console_flush : ConsoleOp = ConsoleOp.Flush ConsoleStream.Stdout\n\
         const terminal_check : ConsoleOp = ConsoleOp.IsTerminal ConsoleStream.Stdout\n\
         fn console_result (op : ConsoleOp) : Type = console_resp op\n",
    );
    assert_eq!(
        baseline.globals["StateOp"],
        baseline.prelude_env.state_op_id
    );
    assert_eq!(baseline.globals["ConsoleOp"], roles.console_family);
    assert_eq!(
        baseline.globals["resp_state"],
        baseline.prelude_env.resp_state_id
    );
    let selected = strict(&source).expect("all scoped family constructors in strict source");
    assert_eq!(selected.env.trusted_base(), baseline.env.trusted_base());
    for (family, leaf, expr) in [
        ("ReadResult", "Chunk", "Chunk (bytes_encode \"x\")"),
        ("FileKind", "KFile", "KFile"),
        (
            "FileMetadata",
            "MkFileMetadata",
            "MkFileMetadata 1 FileKind.KFile",
        ),
        (
            "DirEntry",
            "MkDirEntry",
            "MkDirEntry (bytes_encode \"n\") FileKind.KFile",
        ),
        ("FileOperation", "OpReadFile", "OpReadFile"),
        ("StateOp Nat", "Get", "Get Nat"),
        ("ConsoleOp", "Read", "Read ConsoleStream.Stdin 1"),
        (
            "ConsoleOp",
            "Write",
            "Write ConsoleStream.Stdout (bytes_encode \"x\")",
        ),
        ("ConsoleOp", "Flush", "Flush ConsoleStream.Stdout"),
        ("ConsoleOp", "IsTerminal", "IsTerminal ConsoleStream.Stdout"),
    ] {
        let bare = format!("const leak : {family} = {expr}\n");
        match strict(&bare) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, leaf),
            Err(err) => panic!("{leaf}: wrong bare refusal: {err:?}"),
            Ok(_) => panic!("{leaf} leaked as a bare constructor"),
        }
    }
}

/// Promise class: durable invariant. MEASURED: the imported `Not` is a
/// different checked identity with a definitionally equal body, without
/// changing trust. CLAIMED: the explicit import may replace the pre-source
/// `Not` for users of negation. THE GAP: equality is measured at a symbolic
/// proposition here; actual users still owe strict-entry source checks.
#[test]
fn imported_not_is_definitionally_equal_to_the_pre_source_negation() {
    let prelude = ElabEnv::new().expect("checked prelude");
    let imported = strict("import Core.Logic.Not (Not)\nexport Not\n")
        .expect("ordinary logic negation import");
    let old_id = prelude.globals["Not"];
    let new_id = imported.globals["Not"];
    assert_ne!(
        old_id, new_id,
        "the import is a distinct checked definition"
    );
    let variable = Term::const_(imported.globals["Top"], vec![]);
    let old = Term::app(Term::const_(old_id, vec![]), variable.clone());
    let new = Term::app(Term::const_(new_id, vec![]), variable);
    assert_eq!(
        normalize(&imported.env, &Context::new(), &old),
        normalize(&imported.env, &Context::new(), &new),
        "both checked negations have the same reduced body"
    );
    assert_eq!(imported.env.trusted_base(), prelude.env.trusted_base());
}

/// Promise class: durable invariant. MEASURED: the on-disk rewritten source
/// keeps the Legacy checker verdict and the same body accepts under strict
/// with its import but rejects without it. CLAIMED: a R rewrite does not
/// require opening an internal HostIO constructor to user code. THE GAP:
/// execution parity remains the original native/interpreter test's oracle;
/// this pin checks strict elaboration, not native execution.
#[test]
fn rewritten_nested_read_fixture_checks_at_strict_entry_with_ephemeral_imports() {
    let fixture = include_str!("../../ken-cli/tests/fixtures/rt_nested_read_at_b2.ken");
    let (header, body) = fixture.split_once('\n').expect("program header");
    assert!(
        header.starts_with("program capabilities "),
        "program header"
    );
    let imports = "import Capability.Process.Exit (host_bind, host_fs, host_pure)\n\
                   import Capability.Filesystem.Authority (readFile)\n";
    let with_imports = format!("{header}\n{imports}{body}");
    strict(&with_imports).expect("R fixture with its explicit package imports");
    let without_wrapper_imports =
        format!("{header}\nimport Capability.Filesystem.Authority (readFile)\n{body}");
    match strict(&without_wrapper_imports) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(
                ["host_bind", "host_fs", "host_pure"].contains(&name.as_str()),
                "{name}"
            )
        }
        Err(err) => panic!("wrong missing-wrapper refusal: {err:?}"),
        Ok(_) => panic!("R fixture accepted without package imports"),
    }
}

/// Promise class: transition sentinel, retired when 3b persists imports and
/// switches these source routes to Strict. MEASURED: the actual 33 rewritten
/// fixture files keep their Legacy acceptance or missing-name refusal under
/// Strict with their checked home imports; no hand-copied fixture is tested.
/// CLAIMED: the ruled exact-body HostIO R migrations are source-valid at the
/// future strict entry. THE GAP: current native/interpreter parity is pinned
/// by each fixture's owning test, not inferred from these elaboration verdicts.
#[test]
fn rewritten_hostio_fixtures_recheck_from_the_same_disk_sources_under_strict() {
    use std::collections::{BTreeMap, BTreeSet};

    use ken_elaborator::lexer::{Lexer, Token};

    const USERS: [&str; 33] = [
        "crates/ken-cli/tests/fixtures/abi_s5_block_on_stdin.ken",
        "crates/ken-cli/tests/fixtures/abi_s6_mapping_file_backed.ken",
        "crates/ken-cli/tests/fixtures/abi_s6_mapping_file_backed_cow.ken",
        "crates/ken-cli/tests/fixtures/d0_abi_mapping_source.ken",
        "crates/ken-cli/tests/fixtures/d0_branched_scrutinee_source.ken",
        "crates/ken-cli/tests/fixtures/d0_px8ta_nested_source.ken",
        "crates/ken-cli/tests/fixtures/d0_rt_parity_source.ken",
        "crates/ken-cli/tests/fixtures/px10_argv_env_pipe.ken",
        "crates/ken-cli/tests/fixtures/px10_child_wait_smoke.ken",
        "crates/ken-cli/tests/fixtures/px10_live_child.ken",
        "crates/ken-cli/tests/fixtures/px10_native_tested_pending_overlap.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_buffer.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_lookalike.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_no_bracket.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_resource.ken",
        "crates/ken-cli/tests/fixtures/rt_bracket_body_once_three.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_explicit.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_plain.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_refined.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_refnonrec.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_requires.ken",
        "crates/ken-cli/tests/fixtures/rt_native_erased_omega_call_requires_dropped.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_at_b2.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_at_more.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_captured_read.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_length_more.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_nested_control.ken",
        "crates/ken-cli/tests/fixtures/rt_nested_read_single_read_control.ken",
        "crates/ken-cli/tests/fixtures/rt_performing_underived_acmain.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_region_early_release.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_region_multileaf.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_transport_file_backed.ken",
        "crates/ken-elaborator/tests/fixtures/rt_bracket_transport_mapping.ken",
    ];
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root");
    for path in USERS {
        let source = std::fs::read_to_string(repo.join(path)).expect("actual R user source");
        let tokens = Lexer::lex(&source).expect("lex actual user source");
        let names = tokens
            .iter()
            .filter_map(|(token, _)| match token {
                Token::Ident(name) | Token::ConId(name) => Some(name.as_str()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let mut imports: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for &(home, name) in &PRELUDE_PACKAGE_HOMES {
            if names.contains(name) {
                imports.entry(home).or_default().insert(name);
            }
        }
        let (header, body) = source.split_once('\n').expect("fixture program header");
        assert!(
            header.starts_with("program capabilities "),
            "{path}: {header}"
        );
        let import_lines = imports
            .into_iter()
            .map(|(home, names)| {
                format!(
                    "import {home} ({})\n",
                    names.into_iter().collect::<Vec<_>>().join(", ")
                )
            })
            .collect::<String>();
        let with_imports = format!("{header}\n{import_lines}{body}");
        let mut legacy = ElabEnv::new().expect("prelude");
        let old = legacy.elaborate_file_v1(&source);
        let new = strict(&with_imports).map(|_| ());
        match (old, new) {
            (Ok(_), Ok(_)) => {}
            (
                Err(ElabError::UnresolvedCon { name: before, .. }),
                Err(
                    ElabError::UnboundName { name: after, .. }
                    | ElabError::UnresolvedCon { name: after, .. },
                ),
            ) if before == after => {}
            (old, new) => panic!("{path}: source verdict shifted: Legacy={old:?}, Strict={new:?}"),
        }
    }
}
