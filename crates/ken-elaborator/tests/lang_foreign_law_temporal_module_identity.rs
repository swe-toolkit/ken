//! Module-owned foreign, law and temporal identities (spec 33 §3–4; 32 §1).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::checked_core::StableSymbol;
use ken_elaborator::compiler_driver::{
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
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
                .expect("write provider file");
        }
        let roots = [root.path().to_path_buf()];
        let mut env = ElabEnv::new().expect("base environment");
        match self {
            Self::StrictRoots | Self::LegacyRoots => {
                fs::write(root.path().join("Client.ken"), client).expect("write client file");
                if matches!(self, Self::StrictRoots) {
                    env.elaborate_module_from_roots_strict(&roots, "Client")?;
                } else {
                    env.elaborate_module_from_roots(&roots, "Client")?;
                }
            }
            Self::DirectAfterRoots => {
                for (name, _) in providers {
                    env.elaborate_module_from_roots(&roots, name)?;
                }
                env.elaborate_file(client)?;
            }
            Self::DirectInline => {
                let mut source = String::new();
                for (name, body) in providers {
                    source.push_str(&format!("module {name} {{\n{body}\n}}\n"));
                }
                source.push_str(client);
                env.elaborate_file(&source)?;
            }
        }
        Ok(env)
    }
}

const PRIVATE_PROVIDER: &str = "foreign leak_f : Int = \"probe\" \"libc.so\" pure\n\
    law LeakLaw (x) { f : Top }\n\
    temporal leak_t { always True }\n\
    pub data Tag = MkTag\nexport MkTag";

/// Promise class: durable invariant (33 §3.3–4.1). MEASURED: unimported
/// foreign, law, law-field and temporal names refuse at the surface in each
/// mode, while an imported data identity from the same provider still works.
/// CLAIMED: none of the four private keys leaks through the global table.
/// THE GAP: the named modes reach their distinct loader and direct-API routes;
/// this fixture has one provider, and public foreign has a separate test.
#[test]
fn unimported_private_kinds_refuse_at_the_surface_in_all_modes() {
    for (label, key, client) in [
        (
            "foreign",
            "leak_f",
            "import A (Tag)\nconst use_f : Int = leak_f",
        ),
        (
            "law",
            "LeakLaw",
            "import A (Tag)\nconst use_l : Omega = LeakLaw",
        ),
        (
            "law-field",
            "LeakLaw_f",
            "import A (Tag)\ntheorem use_lf : Top = LeakLaw_f",
        ),
        (
            "temporal",
            "leak_t",
            "import A (Tag)\nconst use_t : Int = leak_t",
        ),
    ] {
        for mode in Mode::ALL {
            let error = mode
                .check(&[("A", PRIVATE_PROVIDER)], client)
                .err()
                .unwrap_or_else(|| panic!("{label} {mode:?} wrongly admitted"));
            assert!(
                match mode {
                    Mode::StrictRoots =>
                        matches!(error, ElabError::UnboundName { ref name, .. } if name == key),
                    Mode::LegacyRoots | Mode::DirectAfterRoots | Mode::DirectInline =>
                        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == key),
                },
                "{label} {mode:?}: expected surface refusal, got {error:?}",
            );
        }
    }
    for mode in Mode::ALL {
        let env = mode
            .check(
                &[("A", PRIVATE_PROVIDER)],
                "import A (Tag, MkTag)\nconst imported : Tag = MkTag",
            )
            .unwrap_or_else(|error| panic!("imported data control {mode:?}: {error:?}"));
        assert!(env.globals.contains_key("A.Tag"), "{mode:?}");
        assert!(env.globals.contains_key("A.MkTag"), "{mode:?}");
    }
}

/// Promise class: durable invariant (33 §3–4). MEASURED: within the defining
/// module the foreign and minted law field resolve, with module-owned globals
/// and no ambient spelling; the temporal id is present without an Internal
/// error. CLAIMED: qualification and prebinding preserve local use in every
/// mode. THE GAP: the imported data control reaches the same provider; the
/// private-leak test independently establishes the negative boundary.
#[test]
fn in_module_uses_keep_checked_qualified_keys_in_all_modes() {
    let provider = "foreign in_f : Int = \"probe\" \"libc.so\" pure\n\
        const use_in : Int = in_f\n\
        law InLaw (x) { g : Top }\n\
        theorem use_g : Top = InLaw_g\n\
        const use_law : Omega = InLaw\n\
        temporal in_t { always True }\n\
        pub data Tag = MkTag\nexport MkTag";
    for mode in Mode::ALL {
        let env = mode
            .check(
                &[("A", provider)],
                "import A (Tag, MkTag)\nconst c : Tag = MkTag",
            )
            .unwrap_or_else(|error| panic!("in-module uses {mode:?}: {error:?}"));
        for key in ["in_f", "InLaw", "InLaw_g", "in_t"] {
            assert!(
                env.globals.contains_key(&format!("A.{key}")),
                "{mode:?} {key}"
            );
            assert!(!env.globals.contains_key(key), "{mode:?} bare {key}");
        }
        let foreign = &env.foreign_env.bindings["A.in_f"];
        assert_eq!((&*foreign.symbol, &*foreign.library), ("probe", "libc.so"));
    }
}

/// Promise class: durable invariant (33 §3–4; 36 §1.6). MEASURED: each
/// foreign reference with an [FS] row refuses a pure fn via EffectEscapes;
/// imported pub foreign is usable by a proc visiting [FS] in every mode.
/// CLAIMED: name qualification cannot silently shed effect metadata.
/// THE GAP: the unrelated top-level control uses the same signature, while
/// the two scoped routes require different name-resolution paths.
#[test]
fn qualified_foreign_effect_rows_reject_pure_calls_in_all_modes() {
    let public_provider = "pub foreign pf : Int -> Int = \"write\" \"libc\" [FS]\n\
        pub data Tag = MkTag\nexport MkTag";
    let private_provider = "foreign wf : Int -> Int = \"write\" \"libc\" [FS]\n\
        fn bad (n : Int) : Int = wf n\n\
        pub data Tag = MkTag\nexport MkTag";
    for mode in Mode::ALL {
        for (label, provider, client) in [
            (
                "in-module",
                private_provider,
                "import A (Tag, MkTag)\nconst c : Tag = MkTag",
            ),
            (
                "imported-pub",
                public_provider,
                "import A (pf)\nfn bad (n : Int) : Int = pf n",
            ),
        ] {
            let error = mode
                .check(&[("A", provider)], client)
                .err()
                .unwrap_or_else(|| panic!("{mode:?} {label}: pure effect escaped"));
            assert!(
                matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason.contains("EffectEscapes") && reason.contains("FS")),
                "{mode:?} {label}: wrong refusal {error:?}",
            );
        }
        mode.check(
            &[("A", public_provider)],
            "import A (pf)\nproc ok (n : Int) : Int visits [FS] = pf n",
        )
        .unwrap_or_else(|error| panic!("{mode:?}: authorized proc refused: {error:?}"));
    }
    let mut env = ElabEnv::new().expect("prelude");
    let error = env
        .elaborate_file(
            "foreign tf : Int -> Int = \"write\" \"libc\" [FS]\n\
             fn bad (n : Int) : Int = tf n",
        )
        .expect_err("top-level pure call to FS foreign must refuse");
    assert!(
        matches!(error, ElabError::TypeMismatch { ref reason, .. } if reason.contains("EffectEscapes") && reason.contains("FS")),
        "top-level pure call: {error:?}",
    );
}

/// Promise class: durable invariant (33 §3, 38 §1). MEASURED: two distinct
/// checked foreign keys retain their identical source-native bindings and the
/// checked-core symbol table contains two distinct declaration symbols.
/// CLAIMED: qualified Ken identity does not rewrite the native C symbol or
/// library and does not collapse two declaration symbols. THE GAP: native
/// execution of a user foreign remains unavailable; this tests the producer,
/// not a native artifact calling the foreign.
#[test]
fn two_module_foreigns_keep_native_bindings_and_distinct_stable_symbols() {
    let source = "module M { foreign f : Int -> Int = \"write\" \"libc\" pure }\n\
                  module N { foreign f : Int -> Int = \"write\" \"libc\" pure }\n\
                  const k : Nat = Zero";
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_file(source)
        .expect("both same-spelling module-owned foreign declarations");
    assert_ne!(env.globals["M.f"], env.globals["N.f"]);
    assert!(!env.globals.contains_key("f"));
    for key in ["M.f", "N.f"] {
        let binding = &env.foreign_env.bindings[key];
        assert_eq!((&*binding.symbol, &*binding.library), ("write", "libc"));
    }
    assert!(!env.foreign_env.bindings.contains_key("f"));

    let output = compile_ken_package_sources(
        &CompilerManifest::new("flt_fixture", vec![]),
        vec![CompilerSource::new("src/entry.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                ken_elaborator::checked_core::SymbolNamespace::Module,
                vec!["flt_fixture".into()],
            ),
            symbol: StableSymbol::declaration("flt_fixture", &[], "k"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("checked-core producer must not collide the foreign stable symbols");
    let symbols = &output.package.artifact.semantic.symbols;
    for module in ["M", "N"] {
        assert!(
            symbols.contains(&StableSymbol::declaration("flt_fixture", &[module], "f")),
            "{module}.f must have its own declaration stable symbol"
        );
    }
}

/// Promise class: durable invariant (33 §3; 64 §2). MEASURED: the exact
/// trusted-base GlobalId inventory for the same three declaration kinds is
/// identical in a module and in its flat form. CLAIMED: qualification adds
/// no kernel trust and never hides a foreign or law postulate. THE GAP: the
/// fixture has one of each kind, with otherwise identical declaration order.
#[test]
fn module_qualification_preserves_the_trusted_base_identity_set() {
    let declarations = "foreign mf : Int = \"probe\" \"libc.so\" pure\n\
                        law ML (x) { h : Top }\n\
                        temporal mt { always True }";
    let baseline: BTreeSet<_> = ElabEnv::new()
        .expect("prelude")
        .env
        .trusted_base()
        .into_iter()
        .collect();
    let mut module_env = ElabEnv::new().expect("prelude");
    module_env
        .elaborate_file(&format!("module M {{ {declarations} }}"))
        .expect("module-owned declarations");
    let mut flat_env = ElabEnv::new().expect("prelude");
    flat_env
        .elaborate_file(declarations)
        .expect("equivalent flat declarations");
    let module_base: BTreeSet<_> = module_env.env.trusted_base().into_iter().collect();
    let flat_base: BTreeSet<_> = flat_env.env.trusted_base().into_iter().collect();
    assert!(
        !module_base
            .difference(&baseline)
            .collect::<Vec<_>>()
            .is_empty(),
        "foreign and law must still contribute their declared trust"
    );
    assert_eq!(module_base, flat_base, "module syntax must add no trust");
}
