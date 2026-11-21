//! Checked package-home client behavior (spec 33 §3.3).
//! Promise class: durable invariant. MEASURED: a source client imports and
//! uses an inductive parent, constructor and transparent checked definition;
//! bare use refuses and an import selects the checked pre-source identity.
//! CLAIMED: the home changes admission, not the checked identity.
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
fn family_and_transparent_home_require_import() {
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

    fs::write(
        root.path().join("Entry.ken"),
        format!(
        "import Capability.System.Buffer (BufferWindow, MkBufferWindow, transfer_count_nat)\n{bare}"
    ),
    )
    .expect("imported entry");
    let mut imported = ElabEnv::new().expect("pre-source environment");
    let parent = imported.globals["BufferWindow"];
    let constructor = imported.globals["MkBufferWindow"];
    let projection = imported.globals["transfer_count_nat"];
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
