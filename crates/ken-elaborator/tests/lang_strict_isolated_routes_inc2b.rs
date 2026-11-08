//! Explicit Mapping and Child homes for Strict isolated files.
//! Contract: LANG-STRICT-ISOLATED-ROUTES AC-2b-a–c; 33 §3.3 and 38 §1.9, §1.11.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::literate::extract_ken_md;
use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};

const MAPPING: [&str; 8] = [
    "MappingHandle",
    "MappingWindow",
    "MkMappingWindow",
    "MappingSource",
    "Anonymous",
    "FileBacked",
    "mapBytes",
    "mapWrite",
];
const CHILD: [&str; 12] = [
    "ChildStdio",
    "ChildPipe",
    "ChildNull",
    "ChildInherit",
    "spawn_plan",
    "with_environment",
    "with_stdin",
    "with_stdout",
    "withChild",
    "wait_child",
    "take_stdout",
    "read_pipe",
];

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

/// Promise class: durable invariant. The exact checked identities have one
/// explicit owner; importing every member of each public family succeeds,
/// while the same isolated source without the import refuses by UnboundName.
#[test]
fn mapping_and_child_checked_homes_require_explicit_imports() {
    for (home, names) in [
        ("Capability.System.Mapping", MAPPING.as_slice()),
        ("Capability.Process.Child", CHILD.as_slice()),
    ] {
        let bootstrap = ElabEnv::new().expect("checked prelude");
        for name in names {
            let providers = PRELUDE_PACKAGE_HOMES
                .iter()
                .filter_map(|(owner, member)| (*member == *name).then_some(*owner))
                .collect::<Vec<_>>();
            assert_eq!(providers, [home], "{name} has exactly one ruled home");
            assert!(!PRELUDE_FLOOR_NAMES.contains(name), "{name} is not floor");
        }
        let source = format!(
            "import {home} ({})\nexport {}\n",
            names.join(", "),
            names.join(", ")
        );
        let selected =
            strict(&source).unwrap_or_else(|err| panic!("{home} complete import refused: {err:?}"));
        assert_eq!(
            selected.env.trusted_base(),
            bootstrap.env.trusted_base(),
            "{home} must preserve the exact trusted identity set"
        );
        for name in names {
            assert_eq!(
                selected.globals[*name], bootstrap.globals[*name],
                "{home}.{name} must retain the checked pre-source identity"
            );
            let body = format!("export {name}\n");
            strict(&format!("import {home} ({})\n{body}", names.join(", ")))
                .unwrap_or_else(|err| panic!("{home}.{name}: import/export refused: {err:?}"));
            match strict(&body) {
                Err(ElabError::UnboundName { name: missing, .. }) => {
                    assert_eq!(missing, *name, "{home}: removing only the import")
                }
                Err(other) => panic!("{home}.{name}: wrong no-import export: {other:?}"),
                Ok(_) => panic!("{home}.{name}: export accepted without import"),
            }
            let unimported = format!("const probe : Nat = {name}\n");
            match strict(&unimported) {
                Err(ElabError::UnboundName { name: missing, .. }) => {
                    assert_eq!(missing, *name, "{home}: no ambient fallback")
                }
                Err(other) => panic!("{home}.{name}: wrong refusal {other:?}"),
                Ok(_) => panic!("{home}.{name} available without import"),
            }
        }
    }
}

/// Promise class: durable invariant. The type-only MappingHandle home does
/// not publish its private constructor or grant a new capability to forge a
/// mapping from an unrelated resource and extent.
#[test]
fn mapping_handle_home_does_not_lease_its_private_constructor() {
    let bootstrap = ElabEnv::new().expect("checked prelude");
    let parent = bootstrap.globals["MappingHandle"];
    let private = bootstrap.env.inductive(parent).unwrap().constructors[0].id;
    let imported = strict(
        "import Capability.System.Mapping (MappingHandle)\n\
         fn keep (mapping : MappingHandle) : MappingHandle = mapping\n",
    )
    .expect("abstract mapping type imports");
    assert_eq!(imported.globals["MappingHandle"], parent);
    assert!(!imported.globals.values().any(|id| *id == private));
    match strict(
        "import Capability.System.Mapping (MappingHandle)\n\
         fn forge (r : Resource ResourceKind.Mapping) : MappingHandle =\n\
           MappingHandle.PrivateMappingHandle r (MkMappingExtent 8)\n",
    ) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(name.contains("PrivateMappingHandle"), "{name}")
        }
        Err(other) => panic!("private mapping constructor: wrong refusal {other:?}"),
        Ok(_) => panic!("private mapping constructor leaked"),
    }
}

/// Promise class: durable invariant. MEASURED: each name imports and exports
/// at its checked ID, rejects without import, and leaves trust unchanged;
/// the private constructor rejects beside the checked resource-kind case.
/// CLAIMED: four child identities have only their explicit package home,
/// without constructor leakage. THE GAP: these checks exercise the real strict
/// loader, not just home-table membership; CLI route switches are separate.
#[test]
fn later_child_homes_require_import_without_leaking_the_plan_constructor() {
    const LATER_CHILD: [&str; 4] = ["SpawnPlan", "with_stderr", "take_stdin", "take_stderr"];
    let bootstrap = ElabEnv::new().expect("checked prelude");
    for name in LATER_CHILD {
        let imported = format!("import Capability.Process.Child ({name})\nexport {name}\n");
        let env =
            strict(&imported).unwrap_or_else(|error| panic!("imported {name} refused: {error:?}"));
        assert_eq!(
            env.globals[name], bootstrap.globals[name],
            "{name} checked identity"
        );
        assert_eq!(
            env.env.trusted_base(),
            bootstrap.env.trusted_base(),
            "{name} trust"
        );
        assert_eq!(
            PRELUDE_PACKAGE_HOMES
                .iter()
                .filter(|(_, member)| *member == name)
                .map(|(home, _)| *home)
                .collect::<Vec<_>>(),
            ["Capability.Process.Child"],
            "{name} has one home"
        );
        match strict(&format!("export {name}\n")) {
            Err(ElabError::UnboundName { name: missing, .. }) => assert_eq!(missing, name),
            Err(error) => panic!("{name} without import: wrong refusal {error:?}"),
            Ok(_) => panic!("{name} without import unexpectedly accepted"),
        }
    }
    let probe = "import Capability.Process.Child (SpawnPlan)\n\
                 const kind_of : ResourceKind = ResourceKind.SpawnPlan\n\
                 fn id_plan (plan : SpawnPlan) : SpawnPlan = plan\n";
    strict(probe).expect("imported plan type and independent scoped resource kind");
    let forge = "import Capability.Process.Child (SpawnPlan, ChildNull)\n\
                 fn forge (exe : Bytes) : SpawnPlan =\n\
                   SpawnPlan.PrivateSpawnPlan exe (Nil Bytes)\n\
                     (Nil (Prod Bytes Bytes)) ChildNull ChildNull ChildNull\n";
    match strict(forge) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, "SpawnPlan.PrivateSpawnPlan");
        }
        Err(error) => panic!("private plan constructor: wrong refusal {error:?}"),
        Ok(_) => panic!("private plan constructor unexpectedly accepted"),
    }
}

fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("enumerate isolated sources") {
        let path = entry.expect("source entry").path();
        if path.is_dir() {
            collect_sources(&path, out);
        } else if path.to_string_lossy().ends_with(".ken")
            || path.to_string_lossy().ends_with(".ken.md")
        {
            out.push(path);
        }
    }
}

fn has_remainder_token(source: &str) -> bool {
    let Ok(tokens) = Lexer::lex(source) else {
        return false;
    };
    tokens.into_iter().any(|(token, _)| match token {
        Token::Ident(name) | Token::ConId(name) => {
            MAPPING.contains(&name.as_str()) || CHILD.contains(&name.as_str())
        }
        _ => false,
    })
}

fn with_home_imports(source: &str, code: &str, literate: bool) -> String {
    let used = Lexer::lex(code)
        .expect("Legacy-clean source must lex")
        .into_iter()
        .filter_map(|(token, _)| match token {
            Token::Ident(name) | Token::ConId(name) => Some(name),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let mut homes: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (home, name) in PRELUDE_PACKAGE_HOMES {
        if used.contains(name) {
            homes.entry(home).or_default().push(name);
        }
    }
    // Supply both new homes completely, even when only one occurs in a file.
    homes.insert("Capability.System.Mapping", MAPPING.to_vec());
    homes.insert("Capability.Process.Child", CHILD.to_vec());
    let imports = homes
        .into_iter()
        .map(|(home, names)| format!("import {home} ({})\n", names.join(", ")))
        .collect::<String>();
    if literate {
        return format!("```ken\n{imports}```\n{source}");
    }
    if source.starts_with("program capabilities ") {
        let (header, rest) = source.split_once('\n').expect("capability manifest line");
        format!("{header}\n{imports}{rest}")
    } else {
        format!("{imports}{source}")
    }
}

/// Promise class: transition sentinel for the 3a vocabulary / 3b route split.
/// Seven isolated files using D0's remainder vocabulary still check under
/// Legacy and now check under Strict with ephemeral checked-home imports.
/// Increment 3a rewrote their compiler-internal FSOp/bind use but did not
/// switch their real entry routes or persist imports in unrooted sources.
/// Lexical selection ignores comments, literals, and literate prose; the
/// assertions concern checker outcomes, not source-text occurrence counts.
#[test]
fn seven_legacy_clean_remainder_sources_have_measured_strict_frontier() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root");
    let mut files = Vec::new();
    for tree in ["examples", "conformance", "crates"] {
        collect_sources(&repo.join(tree), &mut files);
    }
    files.retain(|path| {
        !path.components().any(|part| part.as_os_str() == "catalog")
            && (!path.components().any(|part| part.as_os_str() == "crates")
                || path.components().any(|part| part.as_os_str() == "tests"))
    });
    files.sort();
    let mut remainder_sources = 0;
    let mut legacy_clean = 0;
    let mut strict_clean = 0;
    let mut strict_unbound = 0;
    for path in files {
        let source = std::fs::read_to_string(&path).expect("read isolated source");
        let literate = path.to_string_lossy().ends_with(".ken.md");
        let code = if literate {
            extract_ken_md(&source)
                .expect("extract literate source")
                .source
        } else {
            source.clone()
        };
        if !has_remainder_token(&code) {
            continue;
        }
        remainder_sources += 1;
        let mut legacy = ElabEnv::new().expect("checked prelude");
        let checked = if literate {
            legacy.elaborate_ken_md_file_v1(&source)
        } else {
            legacy.elaborate_file_v1(&source)
        };
        if checked.is_err() {
            continue;
        }
        legacy_clean += 1;
        let imported = with_home_imports(&source, &code, literate);
        let mut strict = ElabEnv::new().expect("checked prelude");
        let result = if literate {
            strict.elaborate_isolated_ken_md_file_strict_v1(&[catalog()], &imported)
        } else {
            strict.elaborate_isolated_file_strict_v1(&[catalog()], &imported)
        };
        match result {
            Ok(_) => strict_clean += 1,
            Err(ElabError::UnboundName { name, .. }) => {
                assert!(
                    !PRELUDE_PACKAGE_HOMES
                        .iter()
                        .any(|(_, member)| *member == name),
                    "{}: available home member `{name}` is still unbound",
                    path.display()
                );
                assert!(
                    matches!(name.as_str(), "FSOp" | "bind"),
                    "{}: unexpected Strict blocker `{name}`",
                    path.display()
                );
                eprintln!("{}: still held by `{name}`", path.display());
                strict_unbound += 1;
            }
            Err(error) => panic!("{}: unexpected strict error {error:?}", path.display()),
        }
    }
    eprintln!(
        "isolated remainder sources={remainder_sources} legacy_clean={legacy_clean} \
         strict_clean={strict_clean} strict_unbound={strict_unbound}"
    );
    assert_eq!(legacy_clean, 7);
    assert_eq!((strict_clean, strict_unbound), (7, 0));
}
