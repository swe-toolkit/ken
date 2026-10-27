//! LANG-RECORD-DECL-MODULE-IDENTITY acceptance (spec 33 §§2–4).
//! Promise class: durable invariants. Adding another module or record cannot
//! authorize an unselected name or change which selected owner supplies fields.

use std::fs;

use ken_elaborator::{ElabEnv, ElabError};

#[derive(Clone, Copy, Debug)]
enum Mode {
    StrictRoots,
    LegacyRoots,
    DirectAfterRoots,
    DirectInline,
}

impl Mode {
    const ALL: [Self; 4] = [
        Self::StrictRoots,
        Self::LegacyRoots,
        Self::DirectAfterRoots,
        Self::DirectInline,
    ];

    fn check(self, providers: &[(&str, &str)], client: &str) -> Result<ElabEnv, ElabError> {
        let root = tempfile::tempdir().expect("fixture root");
        for (name, source) in providers {
            fs::write(root.path().join(format!("{name}.ken")), source)
                .expect("write provider source");
        }
        let roots = [root.path().to_path_buf()];
        let mut env = ElabEnv::new().expect("base environment");
        let ids = match self {
            Self::StrictRoots | Self::LegacyRoots => {
                fs::write(root.path().join("Client.ken"), client).expect("write client source");
                if matches!(self, Self::StrictRoots) {
                    env.elaborate_module_from_roots_strict(&roots, "Client")?
                } else {
                    env.elaborate_module_from_roots(&roots, "Client")?
                }
            }
            Self::DirectAfterRoots => {
                for (name, _) in providers {
                    env.elaborate_module_from_roots(&roots, name)?;
                }
                env.elaborate_file(client)?
            }
            Self::DirectInline => {
                let mut source = String::new();
                for (name, body) in providers {
                    source.push_str(&format!("module {name} {{\n{body}\n}}\n"));
                }
                source.push_str(client);
                env.elaborate_file(&source)?
            }
        };
        assert!(!ids.is_empty(), "{self:?} must check the client");
        Ok(env)
    }
}

const A: &str = "pub record Packet { field : Nat }\npub data Tag = MkTag";
const B: &str = "pub record Packet { other : Bool }\npub data Tag2 = MkTag2";
const PRIVATE_A: &str = "record Packet { field : Nat }\npub data Tag = MkTag";

fn checked(mode: Mode, providers: &[(&str, &str)], client: &str) -> ElabEnv {
    mode.check(providers, client)
        .unwrap_or_else(|error| panic!("{mode:?} must check {client:?}: {error:?}"))
}

fn refused(mode: Mode, providers: &[(&str, &str)], client: &str) -> ElabError {
    match mode.check(providers, client) {
        Ok(_) => panic!("{mode:?} wrongly admitted {client:?}"),
        Err(error) => error,
    }
}

fn assert_unselected(error: ElabError, mode: Mode, spelling: &str) {
    match mode {
        Mode::StrictRoots => assert!(
            matches!(error, ElabError::UnboundName { ref name, .. } if name == spelling),
            "{mode:?} must refuse {spelling} at scope: {error:?}"
        ),
        Mode::LegacyRoots | Mode::DirectAfterRoots | Mode::DirectInline => assert!(
            matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == spelling),
            "{mode:?} must refuse {spelling} like unselected data: {error:?}"
        ),
    }
}

/// MEASURED: a selected public record elaborates a typed projection, literal,
/// and record pattern in each loader; the provider has a qualified checked ID.
/// CLAIMED: `pub record` exports an importable owner with its own fields.
/// THE GAP: two distinct owners with conflicting fields are tested separately.
#[test]
fn imported_public_record_supports_fields_literals_and_patterns_in_four_modes() {
    for mode in Mode::ALL {
        let env = checked(
            mode,
            &[("A", A)],
            "import A (Packet)\n\
             fn get (d : Packet) : Nat = d.field\n\
             fn make (n : Nat) : Packet = { field = n }\n\
             fn take (d : Packet) : Nat = match d { { field } |-> field }",
        );
        assert!(
            env.globals.contains_key("A.Packet"),
            "{mode:?} checked owner"
        );
        assert!(
            !env.globals.contains_key("Packet"),
            "{mode:?} no bare owner"
        );
    }
}

/// MEASURED: the same-shaped unselected record and data control both refuse,
/// with the precise error variant for each mode, even after A was loaded.
/// CLAIMED: loading a provider is not a scope selection, whether it is public
/// or private. THE GAP: importing the public owner is the positive control.
#[test]
fn unselected_public_and_private_records_refuse_like_unselected_data() {
    for mode in Mode::ALL {
        for provider in [A, PRIVATE_A] {
            let record = refused(
                mode,
                &[("A", provider)],
                "import A (Tag)\nfn probe (d : Packet) : Packet = d",
            );
            assert_unselected(record, mode, "Packet");
        }
        let tag = refused(
            mode,
            &[("A", A)],
            "import A (Packet as Selected)\nfn probe (d : Tag) : Tag = d",
        );
        assert_unselected(tag, mode, "Tag");
        checked(
            mode,
            &[("A", A)],
            "import A (Tag)\nfn selected (t : Tag) : Tag = t",
        );
    }
}

/// MEASURED: selecting a private record fails at the exported-name boundary.
/// CLAIMED: qualification does not turn privacy into publication.
/// THE GAP: the matching public import succeeds in the selected-owner test.
#[test]
fn importing_private_record_is_refused() {
    for mode in Mode::ALL {
        let error = refused(
            mode,
            &[("A", PRIVATE_A)],
            "import A (Packet)\nfn probe (d : Packet) : Packet = d",
        );
        assert!(
            matches!(error, ElabError::UnboundName { ref name, .. } if name == "A.Packet"),
            "{mode:?} private export refusal: {error:?}"
        );
    }
}

/// MEASURED: two independently checked Packet owners have different IDs;
/// selecting either determines its field type, and B cannot borrow A's field.
/// CLAIMED: record fields are owned by checked type identity, not spelling or
/// the last record loaded. THE GAP: all four loader modes select both owners.
#[test]
fn two_same_named_modules_select_distinct_record_fields() {
    for mode in Mode::ALL {
        for (selected, other, field, ty) in
            [("A", "B", "field", "Nat"), ("B", "A", "other", "Bool")]
        {
            let source = format!(
                "import {selected} (Packet)\nimport {other} (Tag{})\n\
                 fn get (d : Packet) : {ty} = d.{field}",
                if other == "A" { "" } else { "2" }
            );
            let env = checked(mode, &[("A", A), ("B", B)], &source);
            let a = env.globals["A.Packet"];
            let b = env.globals["B.Packet"];
            assert_ne!(a, b, "{mode:?} different record owners");
            assert!(!env.globals.contains_key("Packet"), "{mode:?} no bare key");
        }
        let error = refused(
            mode,
            &[("A", A), ("B", B)],
            "import B (Packet)\nimport A (Tag)\nfn wrong (d : Packet) : Nat = d.field",
        );
        assert!(
            matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "field"),
            "{mode:?} B must not inherit A.field: {error:?}"
        );
    }
}

/// MEASURED: a qualified module import selects the record, and the declaring
/// unit resolves its own record when elaborated as an ordered barrier.
/// CLAIMED: local and qualified paths both resolve the owner's checked ID.
/// THE GAP: early use must still refuse; tested in the barrier control below.
#[test]
fn qualified_import_and_local_use_select_the_record_owner() {
    for mode in Mode::ALL {
        checked(
            mode,
            &[("A", A)],
            "import A\nfn get (d : A.Packet) : Nat = d.field",
        );
        let env = checked(
            mode,
            &[("A", "record Packet { field : Nat }\nfn get (d : Packet) : Nat = d.field\nfn make (n : Nat) : Packet = { field = n }\npub data Tag = MkTag")],
            "import A (Tag)\nfn use_tag (t : Tag) : Tag = t",
        );
        assert!(env.globals.contains_key("A.Packet"), "{mode:?} local owner");
    }
}

/// MEASURED: an early reference to the record remains an ordered-barrier
/// refusal; no mode admits the formerly ambient owner. CLAIMED: records are
/// not hoisted into a signature-first segment. THE GAP: diagnostics for this
/// pre-existing failure are outside this WP and may improve independently.
#[test]
fn forward_record_reference_remains_refused() {
    for mode in Mode::ALL {
        let error = refused(
            mode,
            &[("A", "fn early (d : Packet) : Nat = d.field\nrecord Packet { field : Nat }\npub data Tag = MkTag")],
            "import A (Tag)\nfn use_tag (t : Tag) : Tag = t",
        );
        assert!(
            matches!(
                error,
                ElabError::TypeMismatch { .. }
                    | ElabError::UnresolvedCon { .. }
                    | ElabError::UnboundName { .. }
            ),
            "{mode:?} early record reference must fail during name/type resolution: {error:?}"
        );
    }
}
