use std::collections::BTreeSet;

use ken_elaborator::{parser, Decl, ElabEnv, ElabError, ExportForm};

use super::catalog_or;

struct PublicationQuery {
    surface: String,
    source: String,
    unpublished_names: BTreeSet<String>,
}

struct ModulePublicationQueries {
    dependency_imports: String,
    direct: Vec<PublicationQuery>,
    attached: Vec<PublicationQuery>,
}

fn rename_identifier(source: &str, from: &str, to: &str) -> String {
    let mut renamed = String::with_capacity(source.len());
    let mut token = String::new();
    let flush = |token: &mut String, renamed: &mut String| {
        if token == from {
            renamed.push_str(to);
        } else {
            renamed.push_str(token);
        }
        token.clear();
    };
    for character in source.chars() {
        if character.is_alphanumeric() || character == '_' {
            token.push(character);
        } else {
            flush(&mut token, &mut renamed);
            renamed.push(character);
        }
    }
    flush(&mut token, &mut renamed);
    renamed
}

fn direct_query(module: &str, label: &str, surface: &str, index: usize) -> PublicationQuery {
    let alias = format!("cat_tier_d_{label}_export_{index}");
    PublicationQuery {
        surface: surface.to_owned(),
        source: format!("import {module} ({surface} as {alias})"),
        unpublished_names: BTreeSet::from([format!("{module}.{surface}")]),
    }
}

fn publication_queries(source: &str, module: &str, label: &str) -> ModulePublicationQueries {
    let extracted = ken_elaborator::literate::extract_ken_md(source)
        .unwrap_or_else(|error| panic!("{module} literate source must extract: {error:?}"));
    let declarations = parser::parse_decls(&extracted.source)
        .unwrap_or_else(|error| panic!("{module} extracted source must parse: {error:?}"));
    let dependency_imports = declarations
        .iter()
        .filter(|declaration| matches!(declaration.unwrap_pub(), Decl::ImportDecl { .. }))
        .map(|declaration| {
            extracted.source[declaration.span().start..declaration.span().end].to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut direct = Vec::new();
    let mut attached = Vec::new();

    for declaration in &declarations {
        match declaration.unwrap_pub() {
            Decl::ViewDecl { .. }
            | Decl::LetDecl { .. }
            | Decl::PropDecl { .. }
            | Decl::TheoremDecl { .. }
            | Decl::AxiomDecl { .. }
            | Decl::TypeAlias { .. }
            | Decl::ClassDecl { .. } => direct.push(direct_query(
                module,
                label,
                declaration.unwrap_pub().name(),
                direct.len(),
            )),
            Decl::DataDecl { name, ctors, .. } => {
                direct.push(direct_query(module, label, name, direct.len()));
                for constructor in ctors {
                    direct.push(direct_query(module, label, &constructor.name, direct.len()));
                }
            }
            Decl::ExplicitDataDecl { name, ctors, .. } => {
                direct.push(direct_query(module, label, name, direct.len()));
                for constructor in ctors {
                    direct.push(direct_query(
                        module,
                        label,
                        constructor.name(),
                        direct.len(),
                    ));
                }
            }
            Decl::AttachedProofDecl {
                proof_name,
                subject,
                params,
                theorem,
                body,
                ..
            } => {
                let binders = params
                    .iter()
                    .map(|binder| &extracted.source[binder.span.start..binder.span.end])
                    .collect::<Vec<_>>()
                    .join(" ");
                let arguments = params
                    .iter()
                    .flat_map(|binder| binder.names.iter())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");
                let theorem_and_separator =
                    &extracted.source[theorem.span().start..body.span().start];
                let separator = theorem_and_separator
                    .rfind('=')
                    .expect("attached proof signature must end at its body separator");
                let theorem = theorem_and_separator[..separator].trim_end();
                let index = attached.len();
                let probe = format!("cat_tier_d_{label}_proof_probe_{index}");
                let alias = format!("cat_tier_d_{label}_subject_{index}");
                let binders = rename_identifier(&binders, subject, &alias);
                let theorem = rename_identifier(theorem, subject, &alias);
                let surface = format!("{subject}::{proof_name}");
                attached.push(PublicationQuery {
                    surface: surface.clone(),
                    source: format!(
                        "import {module} ({subject} as {alias})\n\
                         theorem {probe} {binders} : {theorem} = \
                         {alias}::{proof_name} {arguments}"
                    ),
                    unpublished_names: BTreeSet::from([
                        format!("{module}.{subject}"),
                        format!("{module}.{surface}"),
                    ]),
                });
            }
            Decl::ExportDecl { form, .. } => {
                let items = match form {
                    ExportForm::Facade { items, .. } | ExportForm::InScope { items } => items,
                };
                for item in items {
                    let surface = item.rename.as_deref().unwrap_or(&item.name);
                    direct.push(direct_query(module, label, surface, direct.len()));
                }
            }
            Decl::BoundaryDecl { .. }
            | Decl::FixityDecl { .. }
            | Decl::SpaceDecl { .. }
            | Decl::ProveDecl { .. }
            | Decl::LawDecl { .. }
            | Decl::ForeignDecl { .. }
            | Decl::TemporalDecl { .. }
            | Decl::RecordDecl { .. }
            | Decl::InstanceDecl { .. }
            | Decl::DeriveDecl { .. }
            | Decl::ModuleDecl { .. }
            | Decl::ImportDecl { .. } => {}
            Decl::Pub(_) => panic!("unwrap_pub must remove the visibility wrapper"),
        }
    }

    ModulePublicationQueries {
        dependency_imports,
        direct,
        attached,
    }
}

pub fn published_module_surfaces(source: &str, module: &str, label: &str) -> BTreeSet<String> {
    let queries = publication_queries(source, module, label);
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_or::catalog_root()], module)
        .unwrap_or_else(|error| panic!("{module} must roots-load: {error:?}"));
    if !queries.dependency_imports.is_empty() {
        env.elaborate_file(&queries.dependency_imports)
            .unwrap_or_else(|error| panic!("{module} dependency imports: {error:?}"));
    }
    let mut probe = |query: &PublicationQuery| match env.elaborate_file(&query.source) {
        Ok(_) => true,
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(
                query.unpublished_names.contains(&name),
                "publication query for {} failed at unrelated name `{name}`",
                query.surface
            );
            false
        }
        Err(other) => panic!(
            "publication query for {} failed unexpectedly: {other:?}\n{}",
            query.surface, query.source
        ),
    };

    let direct = queries
        .direct
        .into_iter()
        .filter(|query| probe(query))
        .map(|query| query.surface)
        .collect::<BTreeSet<_>>();
    let direct_imports = direct.iter().cloned().collect::<Vec<_>>();
    let attached = queries
        .attached
        .into_iter()
        .enumerate()
        .filter_map(|(query_index, mut query)| {
            let subject = query
                .surface
                .split_once("::")
                .expect("attached surface has a subject")
                .0;
            let (subject_import, declaration) = query
                .source
                .split_once('\n')
                .expect("attached query has an import and theorem");
            let mut declaration = declaration.to_owned();
            let mut imports = Vec::new();
            for (index, dependency) in direct_imports
                .iter()
                .filter(|surface| surface.as_str() != subject)
                .enumerate()
            {
                let alias = format!("cat_tier_d_{label}_dependency_{query_index}_{index}");
                let renamed = rename_identifier(&declaration, dependency, &alias);
                if renamed != declaration {
                    imports.push(format!("import {module} ({dependency} as {alias})"));
                    declaration = renamed;
                }
            }
            imports.push(subject_import.to_owned());
            imports.push(declaration);
            query.source = imports.join("\n");
            probe(&query).then_some(query.surface)
        })
        .collect::<BTreeSet<_>>();

    direct.union(&attached).cloned().collect()
}
