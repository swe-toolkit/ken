//! CAT-ORD-NAT-CANONICAL-OWNER acceptance controls.
//!
//! Promise classes: the identity, ownership, trust, and behavior controls are
//! durable invariants. Pair floor realization advances the former Pair boundary
//! to its freshly measured next Strict frontier while ownership controls remain
//! on the compatibility loader. These controls use parsed module identities,
//! loader-produced `GlobalId`s, and the class registry. They do not treat
//! repository prose or frozen numeric ids as an oracle.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{literate, parser, Decl, ElabEnv, ElabError, ExportForm};
use ken_kernel::{GlobalId, Term};

const ORDER: &str = "Data.Numeric.Nat.Order";
const LAWFUL: &str = "Core.Classes.LawfulClasses";
const COMPARE: &str = "Core.Logic.Compare";

fn catalog_root() -> PathBuf {
    std::env::var_os("CAT_ORD_CATALOG_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("catalog/packages")
        })
}

fn parse_source(path: &Path) -> Vec<Decl> {
    let source = fs::read_to_string(path).expect("catalog source must be readable UTF-8");
    if path.to_string_lossy().ends_with(".ken.md") {
        let extracted = literate::extract_ken_md(&source).expect("literate extraction");
        literate::validate_ken_md_fences(&extracted).expect("valid literate fences");
        parser::parse_decls(&extracted.source).expect("catalog Ken must parse")
    } else {
        parser::parse_decls(&source).expect("catalog Ken must parse")
    }
}

fn canonical_module(root: &Path, path: &Path) -> String {
    let relative = path
        .strip_prefix(root)
        .expect("discovered source must be below the selected catalog root");
    let rendered = relative.to_string_lossy();
    let without_suffix = rendered
        .strip_suffix(".ken.md")
        .or_else(|| rendered.strip_suffix(".ken"))
        .expect("catalog source has a Ken suffix");
    without_suffix.replace(std::path::MAIN_SEPARATOR, ".")
}

fn collect_imports_and_public_targets(
    module: &str,
    decls: &[Decl],
    imports: &mut BTreeSet<String>,
    public_targets: &mut BTreeSet<String>,
    instance_classes: &mut BTreeSet<String>,
) {
    for decl in decls {
        match decl {
            Decl::Pub(inner) => {
                let name = inner.name();
                if !name.is_empty() {
                    public_targets.insert(format!("{module}.{name}"));
                }
            }
            Decl::ImportDecl {
                module: provider, ..
            } => {
                imports.insert(provider.clone());
            }
            Decl::ExportDecl {
                form:
                    ExportForm::Facade {
                        module: provider,
                        items,
                    },
                ..
            } => {
                imports.insert(provider.clone());
                public_targets.extend(items.iter().map(|item| format!("{provider}.{}", item.name)));
            }
            Decl::InstanceDecl { class_name, .. } => {
                instance_classes.insert(class_name.clone());
            }
            Decl::ModuleDecl { decls, .. } => collect_imports_and_public_targets(
                module,
                decls,
                imports,
                public_targets,
                instance_classes,
            ),
            _ => {}
        }
    }
}

#[derive(Debug)]
struct ParsedUnit {
    imports: BTreeSet<String>,
    public_targets: BTreeSet<String>,
    instance_classes: BTreeSet<String>,
}

fn discover_units(root: &Path) -> BTreeMap<String, ParsedUnit> {
    fn walk(root: &Path, dir: &Path, units: &mut BTreeMap<String, ParsedUnit>) {
        for entry in fs::read_dir(dir).expect("read catalog directory") {
            let path = entry.expect("read catalog entry").path();
            if path.is_dir() {
                walk(root, &path, units);
                continue;
            }
            if !path.to_string_lossy().ends_with(".ken")
                && !path.to_string_lossy().ends_with(".ken.md")
            {
                continue;
            }
            let module = canonical_module(root, &path);
            let decls = parse_source(&path);
            let mut imports = BTreeSet::new();
            let mut public_targets = BTreeSet::new();
            let mut instance_classes = BTreeSet::new();
            collect_imports_and_public_targets(
                &module,
                &decls,
                &mut imports,
                &mut public_targets,
                &mut instance_classes,
            );
            assert!(
                units
                    .insert(
                        module,
                        ParsedUnit {
                            imports,
                            public_targets,
                            instance_classes,
                        },
                    )
                    .is_none(),
                "canonical module identities must be unique"
            );
        }
    }

    let mut units = BTreeMap::new();
    walk(root, root, &mut units);
    units
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
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
            .any(|child| term_mentions(child, target)),
    }
}

fn load_order() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], ORDER)
        .unwrap_or_else(|error| {
            panic!(
                "ordinary compatibility roots must build the Order facade; this is not Strict evidence: {error}"
            )
        });
    env
}

fn canonical_ord_nat(env: &ElabEnv) -> (GlobalId, GlobalId, GlobalId) {
    // Class globals retain their canonical class-registry spelling while the
    // loader records defined-at ownership separately in module/package
    // provenance. Compare identities, never numeric allocation positions.
    let ord = env.globals["Ord"];
    let nat = env.globals["Nat"];
    let mut matches = env
        .class_env
        .instances
        .iter()
        .filter(|((class, head), info)| {
            env.globals.get(class) == Some(&ord)
                && env.globals.get(head) == Some(&nat)
                && info.class_name == *class
        });
    let (_, instance) = matches
        .next()
        .expect("the loader registry must contain Ord at the exact native Nat identity");
    assert!(
        matches.next().is_none(),
        "the loader registry must contain exactly one Ord entry at the exact native Nat identity"
    );
    (ord, nat, instance.instance_id)
}

/// MEASURED: parsed module identities give LawfulClasses -> Compare directly;
/// Order reaches the unique LawfulClasses Ord provider, the catalog publishes no
/// competing Pair identity, and Strict Compare advances past Pair to the next
/// typed unavailable name. CLAIMED: Pair floor realization removes only the
/// Pair boundary without projecting catalog closure. THE GAP: the diagnostic
/// corroborates the next frontier; membership comes from the closed inventory.
#[test]
fn pair_floor_advances_the_structural_compare_boundary() {
    let root = catalog_root();
    let units = discover_units(&root);
    let ord_providers: BTreeSet<_> = units
        .values()
        .flat_map(|unit| {
            unit.public_targets
                .iter()
                .filter(|target| target.rsplit('.').next() == Some("Ord"))
                .cloned()
        })
        .collect();
    assert_eq!(
        ord_providers,
        BTreeSet::from([format!("{LAWFUL}.Ord")]),
        "the parsed provider inventory must give Ord one canonical identity even when a facade republishes it"
    );
    assert!(
        units[ORDER].imports.contains(LAWFUL)
            || units[ORDER].instance_classes.contains("Ord"),
        "Order must reach its unique LawfulClasses Ord provider explicitly or through its parsed instance subject"
    );
    assert!(units[LAWFUL].imports.contains(COMPARE));

    let pair_providers: BTreeSet<_> = units
        .values()
        .flat_map(|unit| {
            unit.public_targets
                .iter()
                .filter(|target| target.rsplit('.').next() == Some("Pair"))
                .cloned()
        })
        .collect();
    assert!(
        pair_providers.is_empty(),
        "the floor realization must not create a catalog Pair provider: {pair_providers:?}"
    );
    assert!(PRELUDE_FLOOR_NAMES.contains(&"Pair"));

    let mut strict = ElabEnv::new().expect("base environment");
    let canonical_pair = strict.globals["Pair"];
    let error = strict
        .elaborate_module_from_roots_strict(&[root], COMPARE)
        .expect_err("Compare retains later non-floor dependencies");
    assert_eq!(strict.globals["Pair"], canonical_pair);
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. } if name == "Equal"),
        "Pair must no longer be the first Strict boundary, got {error:?}"
    );
}

/// MEASURED: ordinary roots elaborate both real units. CLAIMED: Pair floor
/// realization changes no ownership or behavior on the compatibility surface.
/// THE GAP: this is not Strict closure evidence.
#[test]
fn compatibility_roots_still_build_lawful_classes_and_order() {
    for module in [LAWFUL, ORDER] {
        let mut env = ElabEnv::new().expect("base environment");
        env.elaborate_module_from_roots(&[catalog_root()], module)
            .unwrap_or_else(|error| panic!("{module} compatibility load failed: {error}"));
    }
}

/// MEASURED: the instance registry contains one entry keyed by the canonical
/// class and native Nat identities; its package/module provenance is the class
/// provider. CLAIMED: LawfulClasses is the defined-at owner and no Order or
/// second dictionary registration exists. THE GAP: this remains compatibility
/// evidence until the later catalog re-entry closes its independent frontier.
#[test]
fn registry_has_one_lawful_classes_owned_ord_nat_dictionary() {
    let env = load_order();
    let (ord, _nat, dictionary) = canonical_ord_nat(&env);
    let class = env
        .class_env
        .class("Ord")
        .expect("canonical Ord class metadata");
    assert_eq!(class.projection.type_id, ord);

    let info = env
        .class_env
        .instances
        .values()
        .find(|info| info.instance_id == dictionary)
        .expect("canonical dictionary registry entry");
    assert_eq!(info.defining_package, LAWFUL);
    assert_eq!(info.module_id, class.module_id);
    assert_eq!(
        env.class_env.global_modules.get(&dictionary),
        Some(&class.module_id),
        "the dictionary GlobalId must be defined in the class owner's module"
    );
}

/// MEASURED: an Order-only selective import resolves the relation to the
/// provider GlobalId and implicit dispatch records the same registry identity
/// and defining package. CLAIMED: the facade carries identities without an
/// alias or dictionary redeclaration. THE GAP: attached-proof identity is
/// pinned independently below.
#[test]
fn order_facade_carries_relation_and_dictionary_identities() {
    let mut env = load_order();
    let provider_leq = env.globals[&format!("{LAWFUL}.leq_nat")];
    let (_, _, dictionary) = canonical_ord_nat(&env);

    env.elaborate_file(
        "import Data.Numeric.Nat.Order (Ord, leq_nat)\n\
         fn cat_ord_facade_probe (x : Nat) (y : Nat) : Bool where Ord Nat =\n\
           leq_nat x y\n\
         theorem cat_ord_behavior_probe\n\
           : Equal Bool (cat_ord_facade_probe Zero (Suc Zero)) True = Proved",
    )
    .expect("Order-only consumer must resolve the facade and its carried dictionary");

    let probe = env.globals["cat_ord_facade_probe"];
    let (_, body) = env
        .env
        .transparent_body(probe)
        .expect("consumer probe must remain transparent");
    assert!(
        term_mentions(&body, provider_leq),
        "the Order public path must resolve to the LawfulClasses leq_nat GlobalId"
    );
    let resolution = env
        .class_env
        .resolution_provenance
        .iter()
        .rev()
        .find(|resolution| resolution.class_name == "Ord" && resolution.head_type == "Nat")
        .expect("Order-only consumer must record Ord Nat resolution provenance");
    assert_eq!(resolution.instance_id, dictionary);
    assert_eq!(resolution.defining_package, LAWFUL);
}

/// MEASURED: direct provider lookup yields one attached-proof GlobalId, while
/// the Order module has no competing attached identity. CLAIMED: the bridge is
/// owned by the canonical `bool_or` provider. THE GAP: a same-shaped proof is
/// not identity evidence; only the loader's resolved registry is used here.
#[test]
fn bool_or_bridge_has_only_the_provider_identity() {
    let env = load_order();
    let provider = format!("{LAWFUL}.bool_or::eq_true_of_or");
    let forbidden = format!("{ORDER}.bool_or::eq_true_of_or");
    let bridge = env.globals[&provider];
    assert!(env.env.transparent_body(bridge).is_some());
    assert!(
        !env.globals.contains_key(&forbidden),
        "Order must not mint a competing bridge identity"
    );
}

/// MEASURED: the ownership closure adds only the pre-existing audited Ord Int
/// postulates, while the Nat relation, bridge, and dictionary are transparent
/// and executable on a concrete non-degenerate case. CLAIMED: moving ownership
/// changes neither TCB posture nor behavior. THE GAP: equality of source bodies
/// is not claimed; behavior and kernel artifact kinds are the oracle.
#[test]
fn ownership_move_preserves_trust_posture_and_behavior() {
    let mut env = ElabEnv::new().expect("base environment");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], ORDER)
        .expect("compatibility roots build Order");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let added_names: BTreeSet<_> = after
        .difference(&before)
        .map(|id| match env.env.lookup(*id) {
            Some(ken_kernel::Decl::Opaque { name, .. }) => name.as_str(),
            other => panic!("new trusted entry must retain opaque provenance: {other:?}"),
        })
        .collect();
    assert_eq!(
        added_names,
        BTreeSet::from([
            "Ord.Int.antisym",
            "Ord.Int.refl",
            "Ord.Int.total",
            "Ord.Int.trans",
        ])
    );

    let leq = env.globals[&format!("{LAWFUL}.leq_nat")];
    let bridge = env.globals[&format!("{LAWFUL}.bool_or::eq_true_of_or")];
    let (_, _, dictionary) = canonical_ord_nat(&env);
    for id in [leq, bridge, dictionary] {
        assert!(
            env.env.transparent_body(id).is_some(),
            "the migrated Nat component must stay kernel-checked and transparent"
        );
        assert!(
            !after.contains(&id),
            "the migrated Nat component adds no trust"
        );
    }

    env.elaborate_file(
        "import Data.Numeric.Nat.Order (leq_nat)\n\
         theorem cat_ord_lt_behavior\n\
           : Equal Bool (leq_nat Zero (Suc Zero)) True = Proved\n\
         theorem cat_ord_gt_behavior\n\
           : Equal Bool (leq_nat (Suc Zero) Zero) False = Proved\n\
         fn cat_ord_dictionary_leq (x : Nat) (y : Nat) : Bool =\n\
           (Ord_instance_Nat).leq x y\n\
         theorem cat_ord_dictionary_behavior\n\
           : Equal Bool (cat_ord_dictionary_leq Zero (Suc Zero)) True = Proved",
    )
    .expect("the relation must preserve opposite concrete behavior");
}
