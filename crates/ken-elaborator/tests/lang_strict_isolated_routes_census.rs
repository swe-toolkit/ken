//! Transition sentinel for the Increment 1 isolated-source strict frontier.
//! Contract: docs/program/issues/LANG-STRICT-ISOLATED-ROUTES.md AC-1c.
//! This deliberately stops when Increment 2 adds homes or Increment 3 migrates
//! the population; those increments must replace this baseline with their
//! updated behavioral acceptance, not update these numbers as a snapshot.

use ken_elaborator::modules::isolated_catalog_root;
use ken_elaborator::{ElabEnv, ElabError};
use std::path::{Path, PathBuf};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.to_string_lossy().ends_with(".ken")
            || path.to_string_lossy().ends_with(".ken.md")
        {
            out.push(path);
        }
    }
}

/// Promise class: transition sentinel. The input is the behavior of Ken
/// sources in examples/, conformance/, and crates/*/tests, not the spelling,
/// placement, line counts, or prose of any repository file. The AC-0 measured
/// 91 files, 62 Legacy-clean, 12 Strict-clean, 50 Strict-UnboundName. Intended
/// new package homes and migrations will turn this sentinel red by design.
#[test]
fn strict_isolated_baseline_accepts_twelve_of_sixty_two_legacy_clean_files() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let mut files = Vec::new();
    for tree in ["examples", "conformance", "crates"] {
        collect(&repo.join(tree), &mut files);
    }
    files.retain(|path| {
        !path.components().any(|part| part.as_os_str() == "catalog")
            && (!path.components().any(|part| part.as_os_str() == "crates")
                || path.components().any(|part| part.as_os_str() == "tests"))
    });
    files.sort();
    let mut legacy_clean = 0;
    let mut strict_clean = 0;
    let mut strict_unbound = 0;
    let mut surprises = Vec::new();
    for path in &files {
        let source = std::fs::read_to_string(path).unwrap();
        let literate = path.to_string_lossy().ends_with(".ken.md");
        let mut legacy = ElabEnv::new().unwrap();
        let legacy_result = if literate {
            legacy.elaborate_ken_md_file_v1(&source)
        } else {
            legacy.elaborate_file_v1(&source)
        };
        if legacy_result.is_err() {
            continue;
        }
        legacy_clean += 1;
        let mut strict = ElabEnv::new().unwrap();
        let roots: Vec<_> = isolated_catalog_root(path).into_iter().collect();
        let result = if literate {
            strict.elaborate_isolated_ken_md_file_strict_v1(&roots, &source)
        } else {
            strict.elaborate_isolated_file_strict_v1(&roots, &source)
        };
        match result {
            Ok(_) => strict_clean += 1,
            Err(ElabError::UnboundName { .. }) => strict_unbound += 1,
            Err(error) => surprises.push((path.display().to_string(), error)),
        }
    }
    assert!(
        surprises.is_empty(),
        "strict non-UnboundName refusals: {surprises:?}"
    );
    assert_eq!(legacy_clean, strict_clean + strict_unbound);
    eprintln!("isolated strict frontier: files={} legacy_clean={legacy_clean} strict_clean={strict_clean} strict_unbound={strict_unbound}", files.len());
    assert_eq!(
        (files.len(), legacy_clean, strict_clean, strict_unbound),
        (91, 62, 12, 50)
    );
}
