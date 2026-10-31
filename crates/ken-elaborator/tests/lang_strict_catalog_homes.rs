//! Checked package-home client behavior (spec 33 §3.3).
//! Promise class: durable invariant. MEASURED: a source client imports and
//! uses an inductive parent, constructor and transparent checked definition;
//! strict bare use refuses and Legacy bare use preserves its old meaning.
//! CLAIMED: the home changes only Strict admission, not identity or Legacy.
//! THE GAP: the separate table and real-catalog pins test full family closure.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};
use std::fs;

fn contains(term: &Term, id: GlobalId) -> bool {
    match term {
        Term::Const { id: found, .. }
        | Term::IndFormer { id: found, .. }
        | Term::Constructor { id: found, .. }
            if *found == id =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == id => true,
        _ => term.children().into_iter().any(|child| contains(child, id)),
    }
}

#[test]
fn family_and_transparent_home_require_import_only_in_strict_roots() {
    let root = tempfile::tempdir().expect("fixture root");
    let home = root.path().join("Capability/System/Buffer.ken");
    fs::create_dir_all(home.parent().expect("parent")).expect("directory");
    fs::write(
        home,
        "export BufferWindow, MkBufferWindow, transfer_count_nat\n",
    )
    .expect("home");
    let bare =
        "fn window (start : Int) (length : Int) : BufferWindow = MkBufferWindow start length\n\
                fn count (n : TransferCount) : Nat = transfer_count_nat n\n";
    fs::write(root.path().join("Entry.ken"), bare).expect("bare entry");
    let mut strict = ElabEnv::new().expect("pre-source environment");
    assert!(matches!(strict.elaborate_module_from_roots_strict(
        &[root.path().to_path_buf()], "Entry"),
        Err(ElabError::UnboundName { name, .. }) if name == "BufferWindow"));

    let mut legacy = ElabEnv::new().expect("pre-source environment");
    let parent = legacy.globals["BufferWindow"];
    let constructor = legacy.globals["MkBufferWindow"];
    let projection = legacy.globals["transfer_count_nat"];
    legacy
        .elaborate_module_from_roots(&[root.path().to_path_buf()], "Entry")
        .expect("legacy bare meanings unchanged");
    let (_, legacy_type) = legacy
        .env
        .const_type(legacy.globals["Entry.window"])
        .expect("window signature");
    assert!(contains(&legacy_type, parent));
    let (_, legacy_body) = legacy
        .env
        .transparent_body(legacy.globals["Entry.window"])
        .expect("window body");
    assert!(contains(&legacy_body, constructor));
    let (_, legacy_count) = legacy
        .env
        .transparent_body(legacy.globals["Entry.count"])
        .expect("count body");
    assert!(contains(&legacy_count, projection));

    fs::write(
        root.path().join("Entry.ken"),
        format!(
        "import Capability.System.Buffer (BufferWindow, MkBufferWindow, transfer_count_nat)\n{bare}"
    ),
    )
    .expect("imported entry");
    let mut imported = ElabEnv::new().expect("pre-source environment");
    imported
        .elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
        .expect("strict imported checked identities");
    let (_, imported_type) = imported
        .env
        .const_type(imported.globals["Entry.window"])
        .expect("window signature");
    assert!(contains(&imported_type, parent));
    let (_, imported_body) = imported
        .env
        .transparent_body(imported.globals["Entry.window"])
        .expect("window body");
    assert!(contains(&imported_body, constructor));
    let (_, imported_count) = imported
        .env
        .transparent_body(imported.globals["Entry.count"])
        .expect("count body");
    assert!(contains(&imported_count, projection));
}
