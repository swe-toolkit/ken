//! Checked ITree floor admission and its scoped constructor family
//! (30-taxonomy §4; 33 §3.3; 34 §1; 36 §2.1).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const MODES: [&str; 3] = ["strict-roots", "legacy-roots", "legacy-file"];
const RET_SOURCE: &str = "fn responses (op : Nat) : Type = Unit\n\
     const witness : ITree Nat responses Nat = \
     ITree.Ret Nat responses Nat Zero";
const VIS_SOURCE: &str = "fn responses (op : Nat) : Type = Unit\n\
     const witness : ITree Nat responses Nat = \
     ITree.Vis Nat responses Nat Zero \
     (\\response. ITree.Ret Nat responses Nat Zero)";

fn elaborate_into(env: &mut ElabEnv, source: &str, mode: &str) -> Result<(), ElabError> {
    match mode {
        "strict-roots" | "legacy-roots" => {
            let root = tempfile::tempdir().expect("ITree floor fixture directory");
            fs::write(root.path().join("Entry.ken"), source).expect("ITree floor fixture");
            if mode == "strict-roots" {
                env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")?;
            } else {
                env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")?;
            }
        }
        "legacy-file" => {
            env.elaborate_file_v1(source)?;
        }
        other => panic!("unknown mode: {other}"),
    }
    Ok(())
}

fn elaborate(source: &str, mode: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new().expect("checked prelude");
    elaborate_into(&mut env, source, mode)?;
    Ok(env)
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

/// Promise class: normative compatibility vector. MEASURED: every named route
/// resolves the type and both qualified constructors to the exact pre-source
/// checked IDs and parents, with an identical trusted-base identity set.
/// CLAIMED: ITree is the reserved checked floor carrier, and qualification
/// exposes its unchanged family without adding trust. THE GAP: a sourced
/// collision and unqualified-name refusal have independent rows below.
#[test]
fn itree_floor_resolves_checked_type_and_qualified_constructors() {
    let base = ElabEnv::new().expect("checked prelude");
    let trust: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    let parent = base.prelude_env.itree_id;
    assert_eq!(base.globals["ITree"], parent);
    assert!(!trust.contains(&parent), "ITree is kernel-rechecked");
    for (name, id) in [
        ("ITree.Ret", base.prelude_env.ret_id),
        ("ITree.Vis", base.prelude_env.vis_id),
    ] {
        assert_eq!(base.globals[name], id, "{name}: exact registered ID");
        let (family, _) = base.env.constructor(id).expect("checked constructor");
        assert_eq!(family.id, parent, "{name}: original checked parent");
        assert!(!trust.contains(&id), "{name}: kernel-rechecked");
    }
    for mode in MODES {
        for (source, constructor) in [(RET_SOURCE, "ITree.Ret"), (VIS_SOURCE, "ITree.Vis")] {
            let env = elaborate(source, mode)
                .unwrap_or_else(|failure| panic!("{mode} {constructor}: {failure:?}"));
            let witness = if mode == "legacy-file" {
                "witness"
            } else {
                "Entry.witness"
            };
            let id = env.globals[witness];
            let (_, ty) = env.env.const_type(id).expect("checked witness type");
            let (_, body) = env.env.transparent_body(id).expect("checked witness body");
            assert!(contains_id(&ty, parent), "{mode}: exact ITree type");
            assert!(
                contains_id(&body, base.globals[constructor]),
                "{mode}: exact {constructor} node"
            );
            assert_eq!(env.globals["ITree"], parent);
            assert_eq!(env.globals[constructor], base.globals[constructor]);
            assert_eq!(
                env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
                trust,
                "{mode}: no added trust"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: one unqualified prelude
/// constructor fails with this route's exact resolution error, and a legal
/// same-spelling local succeeds in the independent local-binding test.
/// CLAIMED: bare Ret and Vis are not prelude aliases. THE GAP: the qualified
/// checked-ID positive is exercised in its own row above, not before refusal.
fn assert_bare_refuses(mode: &str, name: &str, qualified_source: &str) {
    let source = qualified_source.replacen(&format!("ITree.{name}"), name, 1);
    assert_ne!(
        source, qualified_source,
        "negative fixture changed its token"
    );
    let failure = match elaborate(&source, mode) {
        Ok(_) => panic!("{mode} bare {name}: unexpectedly accepted"),
        Err(failure) => failure,
    };
    if mode == "strict-roots" {
        assert!(
            matches!(failure, ElabError::UnboundName { name: ref rejected, .. } if rejected == name),
            "{mode} bare {name}: {failure:?}"
        );
    } else {
        assert!(
            matches!(failure, ElabError::UnresolvedCon { name: ref rejected, .. } if rejected == name),
            "{mode} bare {name}: {failure:?}"
        );
    }
}

#[test]
fn bare_ret_refuses_in_strict_roots() {
    assert_bare_refuses("strict-roots", "Ret", RET_SOURCE);
}

#[test]
fn bare_vis_refuses_in_strict_roots() {
    assert_bare_refuses("strict-roots", "Vis", VIS_SOURCE);
}

#[test]
fn bare_ret_refuses_in_legacy_roots() {
    assert_bare_refuses("legacy-roots", "Ret", RET_SOURCE);
}

#[test]
fn bare_vis_refuses_in_legacy_roots() {
    assert_bare_refuses("legacy-roots", "Vis", VIS_SOURCE);
}

#[test]
fn bare_ret_refuses_in_legacy_file() {
    assert_bare_refuses("legacy-file", "Ret", RET_SOURCE);
}

#[test]
fn bare_vis_refuses_in_legacy_file() {
    assert_bare_refuses("legacy-file", "Vis", VIS_SOURCE);
}

/// Promise class: durable invariant. MEASURED: no bare global alias survives,
/// while source declarations of both bare names elaborate in each route.
/// CLAIMED: scoping frees Ret and Vis rather than reserving their spellings.
/// THE GAP: source refusal without a local is independently tested above.
#[test]
fn bare_names_are_free_for_local_constructors_without_aliases() {
    let base = ElabEnv::new().expect("checked prelude");
    for name in ["Ret", "Vis"] {
        assert!(!base.globals.contains_key(name), "bare {name}: no alias");
    }
    for mode in MODES {
        let local = elaborate("data Local = Ret | Vis", mode)
            .unwrap_or_else(|failure| panic!("{mode}: local constructors: {failure:?}"));
        for name in ["ITree.Ret", "ITree.Vis"] {
            assert_eq!(local.globals[name], base.globals[name], "{mode}: {name}");
        }
    }
}

/// Promise class: durable invariant. MEASURED: a same-name source type or
/// constructor is refused before it allocates or replaces the checked ID.
/// CLAIMED: ITree is a reserved type, not a source-shadowable lookalike.
/// THE GAP: this direct head-collision test does not enumerate every syntax.
#[test]
fn itree_type_collision_refuses_before_source_allocation() {
    let base = ElabEnv::new().expect("checked prelude");
    for mode in MODES {
        for source in ["data ITree = LocalCtor", "data LocalHolder = ITree"] {
            let mut env = ElabEnv::new().expect("checked prelude");
            let declarations_before = env.env.declarations().len();
            let next_id_before = env.env.next_global_id();
            let trust_before = env.env.trusted_base();
            let failure = elaborate_into(&mut env, source, mode)
                .expect_err("same-spelling floor declaration must refuse");
            assert!(
                matches!(failure, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == "ITree"),
                "{mode} {source}: {failure:?}"
            );
            assert_eq!(env.globals["ITree"], base.globals["ITree"]);
            assert_eq!(env.env.declarations().len(), declarations_before);
            assert_eq!(env.env.next_global_id(), next_id_before);
            assert_eq!(env.env.trusted_base(), trust_before);
        }
    }
}
