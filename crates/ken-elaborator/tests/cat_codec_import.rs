//! CAT-MIGRATE-TIER-C-DATA-VALUE Codec import controls.
//!
//! Promise class: durable invariants. Codec owns its exact checked definition
//! family, publishes no catalog surface, imports only Transport's canonical
//! `cong`, and adds no trust. A future public Codec API is an intentional
//! interface change and must update this inventory rather than bypass it.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{parser, Decl as SurfaceDecl, ElabEnv, ElabError, ExportForm, ImportKind};
use ken_kernel::{Decl, GlobalId, Term};

const CODEC: &str = "Data.Text.Codec";
const TRANSPORT: &str = "Core.Logic.Transport";
const CODEC_KEN_MD: &str = include_str!("../../../catalog/packages/Data/Text/Codec.ken.md");

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load(module: &str) -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must isolated-roots-load: {error:?}"));
    (env, owned)
}

fn expected_owned_names() -> BTreeSet<String> {
    [
        "ascii_view",
        "ascii_view_none",
        "ascii_view_some",
        "byte_is_ascii",
        "classify_ascii_result",
        "codec_roundtrip_anchor",
        "decode_utf8",
        "decode_utf8::definition",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn collect_references(term: &Term, references: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
            references.insert(*id);
        }
        Term::Elim { fam, .. } => {
            references.insert(*fam);
        }
        _ => {}
    }
    for child in term.children() {
        collect_references(child, references);
    }
}

fn declaration_references(declaration: &Decl) -> BTreeSet<GlobalId> {
    let mut references = BTreeSet::new();
    match declaration {
        Decl::Transparent { ty, body, .. } => {
            collect_references(ty, &mut references);
            collect_references(body, &mut references);
        }
        Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. } => {
            collect_references(ty, &mut references);
        }
        Decl::Inductive(inductive) => {
            for term in &inductive.params {
                collect_references(term, &mut references);
            }
            for term in &inductive.indices {
                collect_references(term, &mut references);
            }
            collect_references(&inductive.former_type, &mut references);
            for constructor in &inductive.constructors {
                for term in &constructor.args {
                    collect_references(term, &mut references);
                }
                for term in &constructor.target_indices {
                    collect_references(term, &mut references);
                }
                collect_references(&constructor.type_, &mut references);
            }
        }
    }
    references
}

#[derive(Debug)]
struct PackageShape {
    providers: BTreeSet<(String, String)>,
    public_declarations: BTreeSet<String>,
    exports: BTreeSet<String>,
}

fn package_shape() -> PackageShape {
    let extracted = ken_elaborator::literate::extract_ken_md(CODEC_KEN_MD)
        .expect("Codec literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Codec extracted source must parse");
    let mut providers = BTreeSet::new();
    let mut public_declarations = BTreeSet::new();
    let mut exports = BTreeSet::new();

    for declaration in &declarations {
        if declaration.is_pub() {
            public_declarations.insert(declaration.unwrap_pub().name().to_owned());
        }
        match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl { module, kind, .. } => match kind {
                ImportKind::Selective(items) => {
                    providers.extend(items.iter().map(|item| (module.clone(), item.name.clone())));
                }
                ImportKind::Qualified | ImportKind::Aliased(_) => {
                    providers.insert((module.clone(), "*".to_owned()));
                }
            },
            SurfaceDecl::ExportDecl { form, .. } => {
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                exports.extend(
                    items
                        .iter()
                        .map(|item| item.rename.clone().unwrap_or_else(|| item.name.clone())),
                );
            }
            _ => {}
        }
    }

    PackageShape {
        providers,
        public_declarations,
        exports,
    }
}

/// MEASURED: ordinary roots loading yields exactly the eight named Codec
/// identities, every checked fence executes, and the trusted base is unchanged.
/// CLAIMED: Codec is standalone, owns exactly its checked definition family,
/// and adds no trust. THE GAP: none; loader results are the complete entry-unit
/// definition population and checked fences are executed separately.
#[test]
fn codec_owned_inventory_is_exact_and_standalone_with_zero_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    let before_trust = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned_results = env
        .elaborate_module_from_roots(&[catalog_root()], CODEC)
        .expect("Codec must isolated-roots-load");
    let after_trust = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let prefix = format!("{CODEC}.");
    let owned_names = env
        .globals
        .keys()
        .filter_map(|name| name.strip_prefix(&prefix).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        owned_names,
        expected_owned_names(),
        "Codec owned declaration inventory changed"
    );
    let owned_ids = env
        .globals
        .iter()
        .filter_map(|(name, id)| name.starts_with(&prefix).then_some(*id))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        owned_results.into_iter().collect::<BTreeSet<_>>(),
        owned_ids,
        "every Codec loader result must belong to its qualified identity population"
    );
    assert_eq!(after_trust, before_trust, "Codec must add zero trust");
    env.execute_loaded_entry_checked_fences(CODEC)
        .expect("Codec Definition and every checked fence must elaborate");
}

/// MEASURED: every Codec definition's checked type and body refer only to the
/// exact compiler floor plus Transport's canonical `cong`; both equality-law
/// consumers retain that same `GlobalId`. The semantic import inventory names
/// only that provider item. CLAIMED: Codec's entire direct catalog dependency
/// is `Core.Logic.Transport.cong`. THE GAP: none; the entry-unit identity set is
/// closed by the exact owned-inventory test above.
#[test]
fn codec_direct_provider_is_exact_canonical_transport_cong() {
    let base = ElabEnv::new().expect("base environment");
    let base_ids = base.globals.values().copied().collect::<BTreeSet<_>>();
    let (via_codec, owned_results) = load(CODEC);
    let (direct_transport, _) = load(TRANSPORT);
    let qualified_cong = format!("{TRANSPORT}.cong");
    let via_cong = via_codec.globals[&qualified_cong];
    let direct_cong = direct_transport.globals[&qualified_cong];
    assert_eq!(
        via_cong, direct_cong,
        "Codec must retain Transport's canonical cong identity"
    );

    let owned_ids = owned_results.into_iter().collect::<BTreeSet<_>>();
    let mut external = BTreeSet::new();
    for id in &owned_ids {
        let declaration = via_codec
            .env
            .lookup(*id)
            .unwrap_or_else(|| panic!("Codec identity {id:?} must have a declaration"));
        external.extend(declaration_references(declaration));
    }
    for id in &owned_ids {
        external.remove(id);
    }
    let non_floor = external
        .difference(&base_ids)
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        non_floor,
        BTreeSet::from([via_cong]),
        "Codec's only direct non-floor identity must be Transport.cong"
    );

    for consumer in ["ascii_view_none", "ascii_view_some"] {
        let id = via_codec.globals[&format!("{CODEC}.{consumer}")];
        let references = declaration_references(
            via_codec
                .env
                .lookup(id)
                .unwrap_or_else(|| panic!("{consumer} must have a checked declaration")),
        );
        assert!(
            references.contains(&via_cong),
            "{consumer} must consume Transport's canonical cong identity"
        );
    }

    assert_eq!(
        package_shape().providers,
        BTreeSet::from([(TRANSPORT.to_owned(), "cong".to_owned())]),
        "Codec must select exactly Core.Logic.Transport.cong"
    );
}

/// MEASURED: the semantic interface inventory contains no direct publication
/// or re-export, and selective-import queries reject every direct Codec
/// surface by its exact qualified name. CLAIMED: Codec publishes no catalog
/// surface. THE GAP: attached proofs are not selective-import spellings; their
/// visibility is covered by the semantic public-declaration inventory and by
/// their private owning subjects.
#[test]
fn codec_loader_visible_inventory_is_empty() {
    let shape = package_shape();
    assert_eq!(
        shape.public_declarations,
        BTreeSet::new(),
        "Codec must not directly publish a declaration"
    );
    assert_eq!(
        shape.exports,
        BTreeSet::new(),
        "Codec must not re-export a declaration"
    );

    let (mut env, _) = load(CODEC);
    let direct_surfaces = expected_owned_names()
        .into_iter()
        .filter(|surface| !surface.contains("::"))
        .collect::<BTreeSet<_>>();
    for (index, surface) in direct_surfaces.iter().enumerate() {
        let source = format!("import {CODEC} ({surface} as codec_private_{index})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{CODEC}.{surface}"));
            }
            Err(other) => {
                panic!("Codec import of {surface} failed for the wrong reason: {other:?}")
            }
            Ok(_) => panic!("Codec unexpectedly published {surface}"),
        }
    }
}

/// MEASURED: removing the sole selective import from the extracted checked
/// definition makes elaboration reach the original exact unresolved `cong`
/// boundary. CLAIMED: the Transport import is necessary rather than decorative.
/// THE GAP: none; the positive standalone test above loads the unchanged source,
/// while this paired negative changes only the import edge.
#[test]
fn codec_transport_import_withdrawal_restores_unresolved_cong() {
    let extracted = ken_elaborator::literate::extract_ken_md(CODEC_KEN_MD)
        .expect("Codec literate source must extract");
    let declarations =
        parser::parse_decls(&extracted.source).expect("Codec extracted source must parse");
    let import_edges = declarations
        .iter()
        .filter_map(|declaration| match declaration.unwrap_pub() {
            SurfaceDecl::ImportDecl {
                kind: ImportKind::Selective(items),
                span,
                ..
            } if items.len() == 1 => Some((
                span.start..span.end,
                items[0]
                    .rename
                    .clone()
                    .unwrap_or_else(|| items[0].name.clone()),
            )),
            SurfaceDecl::ImportDecl { .. } => {
                panic!("Codec's sole import must remain a singleton selection")
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        import_edges.len(),
        1,
        "the withdrawal fixture requires one semantic import edge"
    );
    let mut without_import = extracted.source;
    without_import.replace_range(import_edges[0].0.clone(), "");
    let mut env = ElabEnv::new().expect("base environment");
    match env.elaborate_file(&without_import) {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(name, import_edges[0].1),
        Err(other) => panic!("Codec import withdrawal failed for the wrong reason: {other:?}"),
        Ok(_) => panic!("Codec unexpectedly elaborated without its Transport import"),
    }
}
