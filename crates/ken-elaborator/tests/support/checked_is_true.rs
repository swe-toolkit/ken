//! Select checked catalog identities in test-side Ken source clients.
//! The published conformance seed stays unchanged; only its extracted source
//! is re-keyed, preserving every other import and declaration.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl, ImportKind};

/// One checked identity a client selects by an explicit alias.
#[derive(Clone, Copy)]
struct Selection {
    module: &'static str,
    name: &'static str,
    alias: &'static str,
    marker: &'static str,
}

const IS_TRUE: Selection = Selection {
    module: "Core.Classes.LawfulClasses",
    name: "IsTrue",
    alias: "CatalogIsTrue",
    marker: "__I1B_IS_TRUE_IMPORT__",
};

const AND: Selection = Selection {
    module: "Core.Logic.Conjunction",
    name: "And",
    alias: "CatalogAnd",
    marker: "__I1B_AND_IMPORT__",
};

fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn rename_identifier(source: &str, selection: Selection) -> String {
    let name = selection.name;
    let mut result = source.to_owned();
    let starts = source
        .match_indices(name)
        .filter_map(|(start, _)| {
            let end = start + name.len();
            let before = start
                .checked_sub(1)
                .and_then(|index| source.as_bytes().get(index));
            assert_ne!(
                before,
                Some(&b'.'),
                "qualified {name} is not re-keyed by this helper"
            );
            let after = source.as_bytes().get(end);
            (!before.is_some_and(|byte| identifier_byte(*byte))
                && !after.is_some_and(|byte| identifier_byte(*byte)))
            .then_some(start)
        })
        .collect::<Vec<_>>();
    for start in starts.into_iter().rev() {
        result.replace_range(start..start + name.len(), selection.alias);
    }
    result
}

/// Whether `decl` binds the unqualified selected name other than through the
/// unrenamed selective import this helper rewrites.
fn binds_name(decl: &Decl, selection: Selection) -> bool {
    let name_is = |name: &str| name == selection.name;
    match decl {
        Decl::DataDecl { name, ctors, .. } => {
            name_is(name) || ctors.iter().any(|ctor| name_is(&ctor.name))
        }
        Decl::ExplicitDataDecl { name, ctors, .. } => {
            name_is(name) || ctors.iter().any(|ctor| name_is(ctor.name()))
        }
        Decl::ClassDecl { name, fields, .. } => {
            name_is(name) || fields.iter().any(|field| name_is(&field.name))
        }
        Decl::ViewDecl { name, .. }
        | Decl::LetDecl { name, .. }
        | Decl::TheoremDecl { name, .. }
        | Decl::AxiomDecl { name, .. }
        | Decl::TypeAlias { name, .. }
        | Decl::PropDecl { name, .. }
        | Decl::RecordDecl { name, .. }
        | Decl::LawDecl { name, .. }
        | Decl::ProveDecl { name, .. }
        | Decl::SpaceDecl { name, .. }
        | Decl::ForeignDecl { name, .. }
        | Decl::TemporalDecl { name, .. } => name_is(name),
        Decl::AttachedProofDecl { proof_name, .. } => name_is(proof_name),
        Decl::ModuleDecl { name, decls, .. } => {
            name_is(name) || decls.iter().any(|inner| binds_name(inner, selection))
        }
        Decl::Pub(inner) => binds_name(inner, selection),
        Decl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            ..
        } => items.iter().any(|item| {
            item.rename.as_deref() == Some(selection.name)
                || (name_is(&item.name) && item.rename.is_none() && module != selection.module)
        }),
        Decl::ImportDecl {
            kind: ImportKind::Qualified | ImportKind::Aliased(_),
            ..
        }
        | Decl::ExportDecl { .. }
        | Decl::BoundaryDecl { .. }
        | Decl::FixityDecl { .. }
        | Decl::InstanceDecl { .. }
        | Decl::DeriveDecl { .. } => false,
    }
}

pub fn selected_source(source: &str) -> String {
    select(source, IS_TRUE, false)
}

pub fn selected_source_with_import(source: &str) -> String {
    select(source, IS_TRUE, true)
}

/// The proposition-sorted checked And, imported when the client names it.
pub fn selected_and_source_with_import(source: &str) -> String {
    select(source, AND, true)
}

fn select(source: &str, selection: Selection, add_missing_import: bool) -> String {
    let Selection {
        module,
        name,
        alias,
        marker,
    } = selection;
    if !source.contains(name) {
        return source.to_owned();
    }
    let declarations = parse_decls(source).expect("Ken client source parses");
    assert!(
        !declarations
            .iter()
            .any(|declaration| binds_name(declaration, selection)),
        "client binds its own {name}"
    );
    let mut edits = Vec::new();
    for declaration in declarations {
        let Decl::ImportDecl {
            module: imported,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        else {
            continue;
        };
        if imported != module
            || !items
                .iter()
                .any(|item| item.name == name && item.rename.is_none())
        {
            continue;
        }
        let items = items
            .into_iter()
            .map(|item| {
                if item.name == name && item.rename.is_none() {
                    marker.to_string()
                } else {
                    match item.rename {
                        Some(rename) => format!("{} as {rename}", item.name),
                        None => item.name,
                    }
                }
            })
            .collect::<Vec<_>>();
        edits.push((
            span.start..span.end,
            format!("import {module} ({})", items.join(", ")),
        ));
    }
    let had_import = !edits.is_empty();
    let mut client = source.to_owned();
    for (span, replacement) in edits.into_iter().rev() {
        client.replace_range(span, &replacement);
    }
    client = rename_identifier(&client, selection).replace(marker, &format!("{name} as {alias}"));
    if !had_import && add_missing_import {
        format!("import {module} ({name} as {alias})\n{client}")
    } else {
        client
    }
}
