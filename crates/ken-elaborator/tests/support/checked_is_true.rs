//! Select the checked LawfulClasses IsTrue in test-side Ken source clients.
//! The published conformance seed stays unchanged; only its extracted source
//! is re-keyed, preserving every other import and declaration.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl, ImportKind};

const MODULE: &str = "Core.Classes.LawfulClasses";
const ALIAS: &str = "CatalogIsTrue";
const IMPORT_MARKER: &str = "__I1B_IS_TRUE_IMPORT__";

fn identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn rename_identifier(source: &str) -> String {
    let mut result = source.to_owned();
    let starts = source
        .match_indices("IsTrue")
        .filter_map(|(start, _)| {
            let end = start + "IsTrue".len();
            let before = start
                .checked_sub(1)
                .and_then(|index| source.as_bytes().get(index));
            let after = source.as_bytes().get(end);
            (!before.is_some_and(|byte| identifier_byte(*byte))
                && !after.is_some_and(|byte| identifier_byte(*byte)))
            .then_some(start)
        })
        .collect::<Vec<_>>();
    for start in starts.into_iter().rev() {
        result.replace_range(start..start + "IsTrue".len(), ALIAS);
    }
    result
}

pub fn selected_source(source: &str) -> String {
    selected_source_inner(source, false)
}

pub fn selected_source_with_import(source: &str) -> String {
    selected_source_inner(source, true)
}

fn selected_source_inner(source: &str, add_missing_import: bool) -> String {
    let mut edits = Vec::new();
    for declaration in parse_decls(source).expect("Ken client source parses") {
        let Decl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        else {
            continue;
        };
        if module != MODULE
            || !items
                .iter()
                .any(|item| item.name == "IsTrue" && item.rename.is_none())
        {
            continue;
        }
        let items = items
            .into_iter()
            .map(|item| {
                if item.name == "IsTrue" && item.rename.is_none() {
                    IMPORT_MARKER.to_string()
                } else {
                    match item.rename {
                        Some(alias) => format!("{} as {alias}", item.name),
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
    client = rename_identifier(&client).replace(IMPORT_MARKER, "IsTrue as CatalogIsTrue");
    if !had_import && add_missing_import {
        format!("import {MODULE} (IsTrue as {ALIAS})\n{client}")
    } else {
        client
    }
}
