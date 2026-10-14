//! Stable diagnostic identities shared by the untrusted front ends and runtime.
//! A code names one cause; its accompanying message may provide changing context.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(&'static str);

impl Code {
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// One append-only ledger: code, meaning label, and active/retired status.
/// A retired row reserves its code and label forever. Tests in owning crates
/// check their active rows against the tables accompanying exhaustive matches.
pub const LEDGER: &str = include_str!("../codes.tsv");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Active,
    Retired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub code: &'static str,
    pub label: &'static str,
    pub status: Status,
}

fn parse_ledger(input: &'static str) -> Vec<Row> {
    input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<_> = line.split('\t').collect();
            let [code, label, status] = fields.as_slice() else {
                panic!("ledger row needs three tab-separated fields: {line:?}");
            };
            let status = match *status {
                "active" => Status::Active,
                "retired" => Status::Retired,
                other => panic!("ledger status must be active or retired: {other:?}"),
            };
            Row {
                code,
                label,
                status,
            }
        })
        .collect()
}

fn validate_ledger(rows: &[Row]) {
    let mut codes = BTreeSet::new();
    let mut labels = BTreeSet::new();
    let mut next = BTreeMap::<char, u32>::new();
    for row in rows {
        assert!(codes.insert(row.code), "code reused: {}", row.code);
        assert!(labels.insert(row.label), "label reused: {}", row.label);
        let bytes = row.code.as_bytes();
        assert!(
            bytes.len() == 5
                && matches!(bytes[0], b'C' | b'E' | b'H' | b'K' | b'N' | b'X')
                && bytes[1..].iter().all(u8::is_ascii_digit),
            "code shape is one domain letter and four digits: {}",
            row.code
        );
        let (owner, meaning) = row.label.split_once('.').unwrap_or(("", ""));
        assert!(
            !owner.is_empty()
                && !meaning.is_empty()
                && row.label.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'_'
                        || byte == b'.'
                })
                && meaning.split('.').all(|part| !part.is_empty()),
            "label must be a nonempty owner.meaning slug: {}",
            row.label
        );
        let domain = bytes[0] as char;
        let number = row.code[1..].parse::<u32>().expect("four ASCII digits");
        let expected = next.entry(domain).or_insert(1);
        assert_eq!(
            number, *expected,
            "codes in {domain} are dense and in order: {}",
            row.code
        );
        *expected += 1;
    }
}

pub fn ledger() -> Vec<Row> {
    let rows = parse_ledger(LEDGER);
    validate_ledger(&rows);
    rows
}

/// Every active row owned by these prefixes has exactly one table entry, and
/// every table entry is an active ledger row under one of those prefixes.
pub fn assert_table_matches_ledger(owned_prefixes: &[&str], table: &[(Code, &str)]) {
    assert_table_matches_rows(&ledger(), owned_prefixes, table);
}

fn assert_table_matches_rows(rows: &[Row], owned_prefixes: &[&str], table: &[(Code, &str)]) {
    let owns = |label: &str| {
        owned_prefixes
            .iter()
            .any(|prefix| label.starts_with(prefix))
    };
    let mut table_entries = BTreeSet::new();
    for (code, label) in table {
        assert!(
            table_entries.insert((code.as_str(), *label)),
            "duplicate table entry: {code} {label}"
        );
        assert!(
            owns(label),
            "table entry {code} {label} is outside its owner's prefixes"
        );
        assert!(
            rows.iter().any(|row| {
                row.status == Status::Active && row.code == code.as_str() && row.label == *label
            }),
            "table entry {code} {label} is not an active ledger row"
        );
    }
    for row in rows
        .iter()
        .filter(|row| row.status == Status::Active && owns(row.label))
    {
        assert!(
            table_entries.contains(&(row.code, row.label)),
            "ledger row {} {} has no producing table entry",
            row.code,
            row.label
        );
    }
}

/// Define an exhaustive error match and its checked table from the same
/// entries. The registry test then covers the codes actually returned by the
/// match; a separately hand-maintained table could agree with the ledger
/// while the production match silently returned a different code.
#[macro_export]
macro_rules! define_error_codes {
    (@pattern $path:ident :: $variant:ident []) => { $path::$variant };
    (@pattern $path:ident :: $variant:ident ($($fields:tt)*)) => {
        $path::$variant($($fields)*)
    };
    (@pattern $path:ident :: $variant:ident {$($fields:tt)*}) => {
        $path::$variant { $($fields)* }
    };
    (
        impl $ty:ty;
        table $table:ident;
        $( $path:ident :: $variant:ident $shape:tt => ($code:literal, $label:literal), )*
        ;
        $( $wrapped_path:ident :: $wrapped_variant:ident $wrapped_shape:tt => $delegate:expr, )*
    ) => {
        impl $ty {
            pub fn code(&self) -> $crate::Code {
                match self {
                    $( $crate::define_error_codes!(@pattern $path::$variant $shape) => $crate::Code::new($code), )*
                    $( $crate::define_error_codes!(@pattern $wrapped_path::$wrapped_variant $wrapped_shape) => $delegate, )*
                }
            }
        }
        #[cfg(test)]
        const $table: &[($crate::Code, &str)] = &[
            $( ($crate::Code::new($code), $label), )*
        ];
    };
    (
        fn $name:ident($error:ident: &$ty:ty);
        table $table:ident;
        $( $path:ident :: $variant:ident $shape:tt => ($code:literal, $label:literal), )*
    ) => {
        pub fn $name($error: &$ty) -> $crate::Code {
            match $error {
                $( $crate::define_error_codes!(@pattern $path::$variant $shape) => $crate::Code::new($code), )*
            }
        }
        #[cfg(test)]
        const $table: &[($crate::Code, &str)] = &[
            $( ($crate::Code::new($code), $label), )*
        ];
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_well_formed_unique_and_dense() {
        assert!(
            !ledger().is_empty(),
            "the registry must have producing owners"
        );
        assert_eq!(Code::new("E0001").to_string(), "E0001");
        assert_eq!(Code::new("E0001").as_str(), "E0001");
    }

    #[test]
    fn registry_rejects_independent_malformations() {
        let good = "E0001\telab.parse_error\tactive\nE0002\telab.unbound_name\tretired\n";
        let verify = |source| validate_ledger(&parse_ledger(source));
        verify(good);
        // Each control changes only one axis relative to a valid ledger.
        for bad in [
            "E0001\telab.parse_error\tactive\nE0001\telab.unbound_name\tretired\n",
            "E0001\telab.parse_error\tactive\nE0002\telab.parse_error\tretired\n",
            "E0001\telab.parse_error\tactive\nE0003\telab.unbound_name\tretired\n",
            "E0001\telab.parse_error\tactive\nE0002\telab.unbound_name\tobsolete\n",
            "E0001\telab.parse_error\tactive\nE0002\telab.unbound_name\n",
            "E0001\telab.parse_error\tactive\textra\nE0002\telab.unbound_name\tretired\n",
            "E0001\telab.parse_error\tactive\nK00A1\telab.unbound_name\tretired\n",
            "E0001\telab.parse_error\tactive\nE0002\t\tretired\n",
        ] {
            assert!(
                std::panic::catch_unwind(|| verify(bad)).is_err(),
                "malformed ledger was accepted: {bad:?}"
            );
        }
    }

    #[test]
    fn owner_table_rejects_missing_unregistered_and_retired_entries() {
        let rows =
            parse_ledger("E0001\telab.parse_error\tactive\nE0002\telab.old_error\tretired\n");
        validate_ledger(&rows);
        let check = |table: &[(Code, &str)]| assert_table_matches_rows(&rows, &["elab."], table);
        let valid = [(Code::new("E0001"), "elab.parse_error")];
        check(&valid);
        for bad in [
            vec![],
            vec![(Code::new("E9999"), "elab.missing")],
            vec![valid[0], (Code::new("E0002"), "elab.old_error")],
            vec![valid[0], valid[0]],
        ] {
            assert!(
                std::panic::catch_unwind(|| check(&bad)).is_err(),
                "invalid table was accepted: {bad:?}"
            );
        }
        // The public entry point reads and validates the production ledger.
        assert!(ledger().iter().any(|row| row.code == "E0001"));
    }
}
