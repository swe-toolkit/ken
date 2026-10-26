//! LANG-CLASS-IDENTITY-BY-CHECKED-ID acceptance (spec 33 §§3.2–3.3, 5.3–5.5).
//! Promise class: durable invariants. The independently loaded class owners,
//! checked dictionary types and where bodies are the observable identities.

use std::fs;
use std::path::Path;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

#[derive(Clone, Copy, Debug)]
enum Mode {
    Strict,
    Legacy,
    Direct,
}

impl Mode {
    const ALL: [Self; 3] = [Self::Strict, Self::Legacy, Self::Direct];

    fn is_direct(self) -> bool {
        matches!(self, Self::Direct)
    }

    fn checked_key(self, client: &str, name: &str) -> String {
        if self.is_direct() {
            name.to_owned()
        } else {
            format!("{client}.{name}")
        }
    }

    fn check(
        self,
        env: &mut ElabEnv,
        root: &Path,
        client: &str,
        source: &str,
    ) -> Result<(), ElabError> {
        let ids = if self.is_direct() {
            env.elaborate_file(source)?
        } else {
            fs::write(root.join(format!("{client}.ken")), source).expect("write client fixture");
            match self {
                Self::Strict => {
                    env.elaborate_module_from_roots_strict(&[root.to_path_buf()], client)?
                }
                Self::Legacy => env.elaborate_module_from_roots(&[root.to_path_buf()], client)?,
                Self::Direct => unreachable!(),
            }
        };
        assert!(!ids.is_empty(), "{self:?}/{client} must check declarations");
        Ok(())
    }
}

fn fixture(mode: Mode, a: &str, b: &str) -> (tempfile::TempDir, ElabEnv) {
    let root = tempfile::tempdir().expect("fixture root");
    fs::write(root.path().join("A.ken"), a).expect("write A");
    fs::write(root.path().join("B.ken"), b).expect("write B");
    let mut env = ElabEnv::new().expect("base environment");
    for (provider, source) in [("A", a), ("B", b)] {
        let wrapped = if mode.is_direct() {
            format!("module {provider} {{\n{source}\n}}")
        } else {
            source.to_owned()
        };
        mode.check(&mut env, root.path(), provider, &wrapped)
            .unwrap_or_else(|error| panic!("{mode:?} preload {provider}: {error:?}"));
    }
    (root, env)
}

const A: &str = "pub class View a { get : a -> a }\npub data Tag = MkTag";
const B: &str = "pub class View a { get : a -> a }";

fn mentions(term: &Term, id: GlobalId) -> bool {
    match term {
        Term::Const { id: found, .. }
        | Term::IndFormer { id: found, .. }
        | Term::Constructor { id: found, .. }
            if *found == id =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == id => true,
        _ => term.children().into_iter().any(|child| mentions(child, id)),
    }
}

fn checked_type_mentions(env: &ElabEnv, id: GlobalId, owner: GlobalId) {
    let (_, ty) = env.env.const_type(id).expect("checked type");
    assert!(
        mentions(&ty, owner),
        "checked type {ty:?} must mention {owner:?}"
    );
}

fn checked_body_mentions(env: &ElabEnv, id: GlobalId, dictionary: GlobalId) {
    let (_, body) = env.env.transparent_body(id).expect("checked body");
    assert!(
        mentions(&body, dictionary),
        "checked body {body:?} must mention {dictionary:?}"
    );
}

fn assert_unselected(error: ElabError, mode: Mode, spelling: &str) {
    match mode {
        Mode::Strict => assert!(
            matches!(error, ElabError::UnboundName { ref name, .. } if name == spelling),
            "{mode:?} unselected {spelling} must refuse at scope: {error:?}"
        ),
        Mode::Legacy | Mode::Direct => assert!(
            matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == spelling),
            "{mode:?} unselected {spelling} must refuse like data: {error:?}"
        ),
    }
}

/// MEASURED: each loader/API selects the imported class's checked ID in a
/// dictionary type, RCon type and where body, despite both loaded providers.
/// CLAIMED: same-spelled class declarations do not share a mutable identity.
/// THE GAP: parameterized instance heads are covered by the existing registry
/// tests; this pins an ordinary locally owned data head and both selections.
#[test]
fn imported_view_selects_its_checked_owner_in_all_three_modes() {
    for mode in Mode::ALL {
        for (provider, client) in [("A", "C"), ("B", "Cprime")] {
            let (root, mut env) = fixture(mode, A, B);
            let a = env.globals["A.View"];
            let b = env.globals["B.View"];
            assert_ne!(
                a, b,
                "{mode:?} distinct class declarations need distinct IDs"
            );
            assert!(
                !env.globals.contains_key("View"),
                "module classes cannot mint a bare global"
            );
            let source = format!(
                "import {provider} (View)\n\
                 data Local = MkLocal\n\
                 instance View Local {{ get = λx. x }}\n\
                 fn type_probe (d : View Local) : View Local = d\n\
                 fn constraint_probe (x : Local) : Local where View Local = d.get x"
            );
            mode.check(&mut env, root.path(), client, &source)
                .unwrap_or_else(|error| panic!("{mode:?}/{client}: {error:?}"));
            let owner = if provider == "A" { a } else { b };
            let dictionary_key = format!(
                "{provider}.View_instance_{}",
                mode.checked_key(client, "Local")
            );
            let dictionary = env.globals[&dictionary_key];
            checked_type_mentions(&env, dictionary, owner);
            checked_type_mentions(
                &env,
                env.globals[&mode.checked_key(client, "type_probe")],
                owner,
            );
            checked_body_mentions(
                &env,
                env.globals[&mode.checked_key(client, "constraint_probe")],
                dictionary,
            );
        }
    }
}

/// MEASURED: selected class rows check, while unselected same-shape class
/// type/where rows refuse with the same error variant as unselected data.
/// CLAIMED: a previously loaded provider is not a scope selection.
/// THE GAP: direct API aliasing of an unrelated prelude import is tested in
/// the real LawfulClasses control; this fixture has no ambient name conflict.
#[test]
fn unselected_class_type_and_where_refuse_like_unselected_data() {
    for mode in Mode::ALL {
        let (root, mut env) = fixture(mode, A, B);
        mode.check(
            &mut env,
            root.path(),
            "SelectedType",
            "import A (View as AView)\nfn selected (d : AView Nat) : AView Nat = d",
        )
        .expect("selected class type checks");
        let (root, mut env) = fixture(mode, A, B);
        mode.check(&mut env, root.path(), "SelectedWhere",
            "import A (View as AView)\ndata Local = MkLocal\ninstance AView Local { get = λx. x }\nconst selected : Local where AView Local = d.get MkLocal")
            .expect("selected class constraint checks");
        for (client, source, spelling) in [
            ("UnselectedType", "import A (View as AView)\nfn unselected (d : View Nat) : View Nat = d", "View"),
            ("UnselectedWhere", "import A (View as AView)\ndata Local = MkLocal\ninstance AView Local { get = λx. x }\nconst unselected : Local where View Local = d.get MkLocal", "View"),
            ("UnselectedData", "import A (View as AView)\nfn unselected (d : Tag) : Tag = d", "Tag"),
        ] {
            let (root, mut env) = fixture(mode, A, B);
            let error = mode.check(&mut env, root.path(), client, source)
                .expect_err("unselected class/data must refuse");
            assert_unselected(error, mode, spelling);
        }
    }
}

/// MEASURED: a qualified import and two aliased same-spelled providers in a
/// single client retain distinct class/dictionary IDs and selected where uses.
/// CLAIMED: canonical keying separates synthesized dictionary identities.
/// THE GAP: direct API's exported aliases are tested through an explicit
/// module wrapper for providers; its consumer is deliberately top-level.
#[test]
fn qualified_import_and_two_aliases_select_distinct_dictionaries() {
    for mode in Mode::ALL {
        let (root, mut env) = fixture(mode, A, B);
        mode.check(
            &mut env,
            root.path(),
            "Qualified",
            "import A\nfn qualified (d : A.View Nat) : A.View Nat = d",
        )
        .expect("qualified A.View under import A checks");
        checked_type_mentions(
            &env,
            env.globals[&mode.checked_key("Qualified", "qualified")],
            env.globals["A.View"],
        );

        let (root, mut env) = fixture(mode, A, B);
        let client = "Both";
        let source = "import A (View)\nimport B (View as BView)\n\
            data Local = MkLocal\n\
            instance View Local { get = λx. x }\n\
            instance BView Local { get = λx. x }\n\
            fn a_type (d : View Local) : View Local = d\n\
            fn b_type (d : BView Local) : BView Local = d\n\
            const a_selected : Local where View Local = d.get MkLocal\n\
            const b_selected : Local where BView Local = d.get MkLocal\n\
            const a_dictionary : View Local = View_instance_Local\n\
            const b_dictionary : BView Local = BView_instance_Local";
        mode.check(&mut env, root.path(), client, source)
            .unwrap_or_else(|error| panic!("{mode:?} same-client aliased providers: {error:?}"));
        let local = if mode.is_direct() {
            "Local"
        } else {
            "Both.Local"
        };
        let a_dict = env.globals[&format!("A.View_instance_{local}")];
        let b_dict = env.globals[&format!("B.View_instance_{local}")];
        assert_ne!(
            a_dict, b_dict,
            "two checked class IDs must mint distinct dictionaries"
        );
        checked_type_mentions(&env, a_dict, env.globals["A.View"]);
        checked_type_mentions(&env, b_dict, env.globals["B.View"]);
        checked_type_mentions(
            &env,
            env.globals[&mode.checked_key(client, "a_type")],
            env.globals["A.View"],
        );
        checked_type_mentions(
            &env,
            env.globals[&mode.checked_key(client, "b_type")],
            env.globals["B.View"],
        );
        checked_body_mentions(
            &env,
            env.globals[&mode.checked_key(client, "a_selected")],
            a_dict,
        );
        checked_body_mentions(
            &env,
            env.globals[&mode.checked_key(client, "b_selected")],
            b_dict,
        );
        checked_body_mentions(
            &env,
            env.globals[&mode.checked_key(client, "a_dictionary")],
            a_dict,
        );
        checked_body_mentions(
            &env,
            env.globals[&mode.checked_key(client, "b_dictionary")],
            b_dict,
        );
        let (_, a_body) = env
            .env
            .transparent_body(env.globals[&mode.checked_key(client, "a_selected")])
            .unwrap();
        let (_, b_body) = env
            .env
            .transparent_body(env.globals[&mode.checked_key(client, "b_selected")])
            .unwrap();
        assert!(
            !mentions(&a_body, b_dict) && !mentions(&b_body, a_dict),
            "each where clause selects only its imported class's dictionary"
        );
    }
}

/// Promise class: durable invariant (spec 33 §3.3).
/// MEASURED: a module-local class is prebound with its scope's other local
/// declarations, so it collides with a same-spelled imported class, and it is
/// usable as a type after its declaration in every mode.
/// CLAIMED: a local class takes part in scope collision exactly as local data
/// does. MEASURED separately below: before its ordered barrier, a class
/// reference is refused even if another class holds its canonical key.
#[test]
fn local_class_is_prebound_for_collision_and_type_use() {
    for mode in [Mode::Strict, Mode::Legacy] {
        let (root, mut env) = fixture(mode, A, B);
        let error = mode
            .check(
                &mut env,
                root.path(),
                "LocalCollides",
                "import A (View)\nclass View a { get : a -> a }",
            )
            .expect_err("a local class must collide with a same-spelled import");
        assert!(
            matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "View"),
            "{mode:?}: {error:?}"
        );
    }
    for mode in Mode::ALL {
        let (root, mut env) = fixture(mode, A, B);
        mode.check(
            &mut env,
            root.path(),
            "LocalTypeUse",
            "class View a { get : a -> a }\nfn f (d : View Nat) : View Nat = d",
        )
        .expect("a local class is usable as a type after its declaration");
    }
}

const FILE_CLASS: &str =
    "pub class View a { get : a -> a }\ninstance View Nat { get = \\x. x }";

fn class_session(warm: bool) -> (tempfile::TempDir, ElabEnv) {
    let root = tempfile::tempdir().expect("class fixture root");
    let mut env = ElabEnv::new().expect("base environment");
    if warm {
        fs::write(root.path().join("A.ken"), FILE_CLASS).expect("write A provider");
        env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "A")
            .expect("check old A provider");
    }
    (root, env)
}

/// Promise class: durable invariant (spec 33 §§1, 3.3).
/// MEASURED: type, where, own-field, superclass and early-instance references
/// to an unadmitted local class all refuse with its canonical name, whether
/// another unit already loaded that name or this is a fresh session.
/// CLAIMED: a pending local class cannot borrow an older checked global ID.
/// THE GAP: nested inline-module classes are not exercised here.
#[test]
fn forward_local_class_refuses_stale_warm_identity() {
    for (row, source) in [
        (
            "F",
            "module A { fn early (d : View Nat) : View Nat = d\npub class View a { other : Nat } }",
        ),
        (
            "W",
            "module A { const early : Nat where View Nat = d.get Zero\npub class View a { other : Nat } }",
        ),
        (
            "SELF",
            "module A { pub class View a { other : View a -> Nat } }",
        ),
        (
            "SUPER-FWD",
            "module A { pub class Sub a { base : View a }\npub class View a { other : Nat } }",
        ),
        (
            "INST-BEFORE",
            "module A { instance View Nat { other = Zero }\npub class View a { other : Nat } }",
        ),
    ] {
        for warm in [false, true] {
            let (_root, mut env) = class_session(warm);
            let error = env.elaborate_file(source)
                .expect_err("class reference before its barrier must refuse");
            assert!(
                matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "A.View"),
                "{row}/warm={warm}: expected canonical unresolved class, got {error:?}"
            );
        }
    }
}

/// Promise class: durable invariant (spec 33 §§1, 3.3).
/// MEASURED: a reference after the ordered barrier uses its new checked class
/// ID, not an older class at the same key; ordinary data forward use remains
/// accepted, both after a previous provider load and in a fresh session.
/// CLAIMED: the class guard is confined to pending classes, not all forward
/// names or already checked classes. THE GAP: imported class IDs are tested
/// by the existing selection and alias tests above rather than here.
#[test]
fn after_class_barrier_and_data_forward_keep_checked_identity() {
    let after = "module A { pub class View a { other : Nat }\nfn late (d : View Nat) : View Nat = d }";
    let data = "module A { fn early (t : T) : T = t\npub data T = MkT }";
    for warm in [false, true] {
        let (_root, mut env) = class_session(warm);
        let old_owner = env.globals.get("A.View").copied();
        env.elaborate_file(after)
            .unwrap_or_else(|error| panic!("AFTER/warm={warm}: {error:?}"));
        let new_owner = env.globals["A.View"];
        if let Some(old_owner) = old_owner {
            assert_ne!(old_owner, new_owner, "the local class must have a new ID");
        }
        checked_type_mentions(&env, env.globals["A.late"], new_owner);

        let (_root, mut env) = class_session(warm);
        env.elaborate_file(data)
            .unwrap_or_else(|error| panic!("DATA-FWD/warm={warm}: {error:?}"));
        checked_type_mentions(&env, env.globals["A.early"], env.globals["A.T"]);
    }
}
