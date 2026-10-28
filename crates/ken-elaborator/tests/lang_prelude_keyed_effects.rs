//! Checked Unit/Coproduct/Instant floor admission (30 §4; 33 §3.3; 36 §1–2).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const EFFECT_TYPES: [&str; 3] = ["Unit", "Coproduct", "Instant"];
const MODES: [&str; 3] = ["strict-roots", "legacy-file", "legacy-roots"];

fn elaborate_into(env: &mut ElabEnv, source: &str, mode: &str) -> Result<(), ElabError> {
    match mode {
        "strict-roots" | "legacy-roots" => {
            let root = tempfile::tempdir().expect("effect floor fixture directory");
            fs::write(root.path().join("Entry.ken"), source).expect("effect floor fixture");
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

fn error(source: &str, mode: &str) -> ElabError {
    let mut env = ElabEnv::new().expect("checked prelude");
    elaborate_into(&mut env, source, mode).expect_err("source must be refused")
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

/// Promise class: normative compatibility vector. MEASURED: strict roots and
/// both shipping legacy routes resolve all three pre-source floor types and
/// four constructor paths to their registered IDs and checked parent types.
/// CLAIMED: the reserved floor exposes only these exact identities with no
/// new trust. THE GAP: this covers consumer resolution, not every native
/// reader; the runtime role and host tests cover those independent readers.
#[test]
fn effect_floor_names_resolve_to_captured_ids_in_strict_and_legacy_modes() {
    let base = ElabEnv::new().expect("checked prelude");
    let trust: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    for (name, expected) in [
        ("Unit", base.prelude_env.unit_id),
        ("Coproduct", base.prelude_env.coproduct_id),
        ("Instant", base.prelude_env.instant_id),
    ] {
        assert_eq!(base.globals[name], expected, "{name}: registered exact ID");
        assert!(!trust.contains(&expected), "{name}: rechecked, not trusted");
    }
    for (name, parent) in [
        ("MkUnit", base.prelude_env.unit_id),
        ("Coproduct.InL", base.prelude_env.coproduct_id),
        ("Coproduct.InR", base.prelude_env.coproduct_id),
        ("Instant.MkInstant", base.prelude_env.instant_id),
    ] {
        let id = base.globals[name];
        let (family, _) = base.env.constructor(id).expect("checked constructor");
        assert_eq!(family.id, parent, "{name}: checked parent");
        assert!(!trust.contains(&id), "{name}: rechecked, not trusted");
    }
    assert_eq!(base.globals["MkUnit"], base.prelude_env.mkunit_id);
    assert_eq!(base.globals["Coproduct.InL"], base.prelude_env.inl_id);
    assert_eq!(base.globals["Coproduct.InR"], base.prelude_env.inr_id);
    assert_eq!(
        base.globals["Instant.MkInstant"],
        base.prelude_env.mkinstant_id
    );
    assert_eq!(
        base.prelude_env.runtime_roles.unit,
        base.prelude_env.mkunit_id
    );
    assert_eq!(base.prelude_env.runtime_roles.in_l, base.prelude_env.inl_id);
    assert_eq!(base.prelude_env.runtime_roles.in_r, base.prelude_env.inr_id);
    assert_eq!(
        base.prelude_env.runtime_roles.mk_instant,
        base.prelude_env.mkinstant_id
    );

    let cases = [
        ("Unit", "const witness : Unit = MkUnit", "MkUnit"),
        (
            "Coproduct",
            "const witness : Coproduct Nat Nat = Coproduct.InL Nat Nat Zero",
            "Coproduct.InL",
        ),
        (
            "Coproduct",
            "const witness : Coproduct Nat Nat = Coproduct.InR Nat Nat Zero",
            "Coproduct.InR",
        ),
        (
            "Instant",
            "const witness : Instant = Instant.MkInstant 23",
            "Instant.MkInstant",
        ),
    ];
    for mode in MODES {
        for (type_name, source, constructor) in cases {
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
            assert!(
                contains_id(&ty, base.globals[type_name]),
                "{mode} {type_name}: exact type"
            );
            assert!(
                contains_id(&body, base.globals[constructor]),
                "{mode} {constructor}: exact ctor"
            );
            assert_eq!(env.globals[type_name], base.globals[type_name]);
            assert_eq!(env.globals[constructor], base.globals[constructor]);
            assert_eq!(
                env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
                trust
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: the prelude has no bare
/// coproduct or clock constructor keys, while source-local same-spelling
/// constructors remain legal in each route. CLAIMED: scoping does not reserve
/// those names or keep global aliases. THE GAP: this global-binding check is
/// not the source-refusal pin; the three independent rows below provide it.
#[test]
fn scoped_effect_constructors_have_no_bare_aliases() {
    let base = ElabEnv::new().expect("checked prelude");
    for name in ["InL", "InR", "MkInstant"] {
        assert!(!base.globals.contains_key(name), "{name}: no bare alias");
    }
    for mode in MODES {
        let local = elaborate("data Local = InL | InR | MkInstant", mode)
            .unwrap_or_else(|failure| panic!("{mode}: legal local constructor: {failure:?}"));
        for name in ["Coproduct.InL", "Coproduct.InR", "Instant.MkInstant"] {
            assert_eq!(
                local.globals[name], base.globals[name],
                "{mode} {name}: unchanged"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: this route independently
/// refuses each bare scoped constructor in expression position with its
/// mode-specific error. CLAIMED: no source route accepts a bare prelude alias.
/// THE GAP: the qualified positives are pinned independently in the exact-ID
/// row above; constructor patterns are tested by other consumers.
fn assert_bare_scoped_constructors_refuse(mode: &str) {
    for (name, source) in [
        (
            "InL",
            "const witness : Coproduct Nat Nat = InL Nat Nat Zero",
        ),
        (
            "InR",
            "const witness : Coproduct Nat Nat = InR Nat Nat Zero",
        ),
        ("MkInstant", "const witness : Instant = MkInstant 23"),
    ] {
        let failure = match elaborate(source, mode) {
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
}

#[test]
fn bare_scoped_effect_constructors_refuse_in_strict_roots() {
    assert_bare_scoped_constructors_refuse("strict-roots");
}

#[test]
fn bare_scoped_effect_constructors_refuse_in_legacy_roots() {
    assert_bare_scoped_constructors_refuse("legacy-roots");
}

#[test]
fn bare_scoped_effect_constructors_refuse_in_legacy_file() {
    assert_bare_scoped_constructors_refuse("legacy-file");
}

/// Promise class: durable invariant. MEASURED: same-spelling source type or
/// constructor declarations for every new floor type refuse at admission.
/// CLAIMED: no source allocation can substitute these compiler-owned type IDs.
/// THE GAP: this tests direct type/constructor heads, not every future syntax.
#[test]
fn effect_floor_type_collisions_refuse_before_source_allocation() {
    let base = ElabEnv::new().expect("checked prelude");
    for mode in MODES {
        for type_name in EFFECT_TYPES {
            for source in [
                format!("data {type_name} = LocalCtor"),
                format!("data LocalHolder = {type_name}"),
            ] {
                let mut env = ElabEnv::new().expect("checked prelude");
                let declarations_before = env.env.declarations().len();
                let next_id_before = env.env.next_global_id();
                let trust_before = env.env.trusted_base();
                let failure = elaborate_into(&mut env, &source, mode)
                    .expect_err("same-spelling floor declaration must refuse");
                assert!(
                    matches!(failure, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == type_name),
                    "{mode} {type_name}: {source}: {failure:?}"
                );
                assert_eq!(env.globals[type_name], base.globals[type_name]);
                assert!(!env.globals.contains_key("LocalCtor"));
                assert!(!env.globals.contains_key("LocalHolder"));
                assert!(!env.globals.contains_key("Entry.LocalCtor"));
                assert!(!env.globals.contains_key("Entry.LocalHolder"));
                assert_eq!(env.env.declarations().len(), declarations_before);
                assert_eq!(env.env.next_global_id(), next_id_before);
                assert_eq!(env.env.trusted_base(), trust_before);
            }
        }
    }
    for mode in MODES {
        let failure = error("data Local = MkUnit", mode);
        assert!(
            matches!(failure, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == "MkUnit"),
            "{mode} bare floor constructor: {failure:?}"
        );
    }
}
