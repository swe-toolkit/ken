//! Module-owned `prove` declarations (spec 33 §3–4; 22 §1).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::{ElabEnv, ElabError, ElabResult, v2_extract};

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

    fn check(
        self,
        providers: &[(&str, &str)],
        client: &str,
    ) -> Result<(ElabEnv, Vec<ElabResult>), ElabError> {
        let root = tempfile::tempdir().expect("fixture root");
        for (name, source) in providers {
            fs::write(root.path().join(format!("{name}.ken")), source)
                .expect("write provider file");
        }
        let roots = [root.path().to_path_buf()];
        let mut env = ElabEnv::new().expect("base environment");
        let mut results = Vec::new();
        match self {
            Self::StrictRoots => {
                fs::write(root.path().join("Client.ken"), client).expect("write client file");
                env.elaborate_module_from_roots_strict(&roots, "Client")?;
            }
            Self::LegacyRoots => {
                fs::write(root.path().join("Client.ken"), client).expect("write client file");
                results = env.elaborate_module_from_roots_v1(&roots, "Client")?;
            }
            Self::DirectAfterRoots => {
                for (name, _) in providers {
                    results.extend(env.elaborate_module_from_roots_v1(&roots, name)?);
                }
                results.extend(env.elaborate_file_v1(client)?);
            }
            Self::DirectInline => {
                let mut source = String::new();
                for (name, body) in providers {
                    source.push_str(&format!("module {name} {{\n{body}\n}}\n"));
                }
                source.push_str(client);
                results = env.elaborate_file_v1(&source)?;
            }
        }
        Ok((env, results))
    }
}

fn prove_ids(results: &[ElabResult]) -> Vec<String> {
    results
        .iter()
        .flat_map(|result| v2_extract(result).obligations)
        .map(|triple| triple.id.0)
        .filter(|id| id.ends_with(".prove"))
        .collect()
}

const M: &str = "prove p : Top\npub data TM = MkTM\nexport MkTM";
const N: &str = "prove p : Top\npub data TN = MkTN\nexport MkTN";
const TWO_CLIENT: &str = "import M (TM)\nimport N (TN)\nconst k : Nat = Zero";

/// Promise class: durable invariant (33 §3; 22 §1). MEASURED: two loaded
/// modules admit same-spelling prove declarations with distinct checked keys,
/// no bare key, and qualified obligation ids where the API exposes results.
/// CLAIMED: each obligation has the identity of its owning module. THE GAP:
/// strict roots exposes only the environment; the other modes expose V1 ids.
#[test]
fn same_spelling_has_distinct_module_owners_in_all_modes() {
    for mode in Mode::ALL {
        let (env, results) = mode
            .check(&[("M", M), ("N", N)], TWO_CLIENT)
            .unwrap_or_else(|error| panic!("{mode:?} refused two owners: {error:?}"));
        assert_ne!(env.globals["M.p"], env.globals["N.p"], "{mode:?}");
        assert!(!env.globals.contains_key("p"), "{mode:?} bare p");
        assert!(env.globals.contains_key("M.TM"), "{mode:?} M loaded");
        assert!(env.globals.contains_key("N.TN"), "{mode:?} N loaded");
        if !matches!(mode, Mode::StrictRoots) {
            assert_eq!(prove_ids(&results), ["M.p.prove", "N.p.prove"], "{mode:?}");
        }
    }
}

const PRIVATE_PROVIDER: &str = "prove leak_p : Top\npub data Tag = MkTag\nexport MkTag";

/// Promise class: durable invariant (33 §3–4). MEASURED: an unimported prove
/// use refuses with each mode's surface error while importing an unrelated
/// public data constructor from the same unit succeeds. CLAIMED: a private
/// prove is not reachable through ambient globals. THE GAP: one private
/// spelling is exercised in each of the four distinct resolution routes.
#[test]
fn unimported_prove_refuses_in_each_mode() {
    for mode in Mode::ALL {
        let error = mode
            .check(
                &[("A", PRIVATE_PROVIDER)],
                "import A (Tag)\ntheorem use_p : Top = leak_p",
            )
            .err()
            .unwrap_or_else(|| panic!("{mode:?} leaked unimported prove"));
        assert!(
            match mode {
                Mode::StrictRoots =>
                    matches!(error, ElabError::UnboundName { ref name, .. } if name == "leak_p"),
                Mode::LegacyRoots | Mode::DirectAfterRoots | Mode::DirectInline =>
                    matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "leak_p"),
            },
            "{mode:?}: unexpected refusal {error:?}",
        );
        let (env, _) = mode
            .check(
                &[("A", PRIVATE_PROVIDER)],
                "import A (Tag, MkTag)\nconst c : Tag = MkTag",
            )
            .unwrap_or_else(|error| panic!("{mode:?} data control: {error:?}"));
        assert!(env.globals.contains_key("A.leak_p"), "{mode:?}");
        assert!(!env.globals.contains_key("leak_p"), "{mode:?}");
        assert!(env.globals.contains_key("A.Tag"), "{mode:?}");
    }
}

/// Promise class: durable invariant (33 §3). MEASURED: a theorem in each
/// module can use its locally declared prove, and both declarations keep
/// checked qualified keys and a qualified obligation id where available.
/// CLAIMED: qualification preserves local resolution, not just distinct
/// global names. THE GAP: the fixture exercises one local theorem per mode.
#[test]
fn in_module_prove_use_resolves_in_each_mode() {
    let provider = "prove in_p : Top\ntheorem use_in : Top = in_p\n\
        pub data Tag = MkTag\nexport MkTag";
    for mode in Mode::ALL {
        let (env, results) = mode
            .check(
                &[("A", provider)],
                "import A (Tag, MkTag)\nconst c : Tag = MkTag",
            )
            .unwrap_or_else(|error| panic!("{mode:?} in-module use: {error:?}"));
        assert!(env.globals.contains_key("A.in_p"), "{mode:?}");
        assert!(env.globals.contains_key("A.use_in"), "{mode:?}");
        assert!(!env.globals.contains_key("in_p"), "{mode:?}");
        assert!(!env.globals.contains_key("use_in"), "{mode:?}");
        if !matches!(mode, Mode::StrictRoots) {
            assert_eq!(prove_ids(&results), ["A.in_p.prove"], "{mode:?}");
        }
    }
}

/// Promise class: durable invariant (33 §3; 22 §1). MEASURED: top-level
/// prove keeps its key and obligation id, a same-spelling module prove
/// coexists, and the trusted-base identity set equals the flat control.
/// CLAIMED: modules change scope, not the kernel's trust boundary. THE GAP:
/// one Top obligation is compared with its module counterpart.
#[test]
fn flat_prove_identity_and_trust_set_are_unchanged() {
    let baseline: BTreeSet<_> = ElabEnv::new()
        .unwrap()
        .env
        .trusted_base()
        .into_iter()
        .collect();
    let mut flat = ElabEnv::new().unwrap();
    let flat_results = flat.elaborate_file_v1("prove q : Top").expect("flat prove");
    assert!(flat.globals.contains_key("q"));
    assert_eq!(prove_ids(&flat_results), ["q.prove"]);
    let flat_trust: BTreeSet<_> = flat.env.trusted_base().into_iter().collect();
    assert_eq!(flat_trust.difference(&baseline).count(), 1);

    let mut module = ElabEnv::new().unwrap();
    let results = module
        .elaborate_file_v1("module M { prove q : Top }")
        .expect("module prove");
    assert!(module.globals.contains_key("M.q"));
    assert!(!module.globals.contains_key("q"));
    assert_eq!(prove_ids(&results), ["M.q.prove"]);
    let module_trust: BTreeSet<_> = module.env.trusted_base().into_iter().collect();
    assert_eq!(module_trust, flat_trust);

    let mut both = ElabEnv::new().unwrap();
    let results = both
        .elaborate_file_v1("prove q : Top\nmodule M { prove q : Top }")
        .expect("flat and module prove coexist");
    assert_ne!(both.globals["q"], both.globals["M.q"]);
    assert_eq!(prove_ids(&results), ["q.prove", "M.q.prove"]);
}
