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

/// Produce one physical diagnostic line even when an input path or message
/// contains a line terminator. The free-text message is not a stable ABI.
pub fn render_line(code: Code, message: impl fmt::Display) -> String {
    let message = message
        .to_string()
        .replace('\r', "\\r")
        .replace('\n', "\\n");
    format!("error[{code}]: {message}")
}

/// One append-only ledger: code, meaning label, active/retired status, and
/// owning enum variant. Retired rows reserve their code, label, and last variant.
/// Owning crates check active rows against exhaustive variant matches.
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
    pub variant: &'static str,
}

fn parse_ledger(input: &'static str) -> Vec<Row> {
    input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<_> = line.split('\t').collect();
            let [code, label, status, variant] = fields.as_slice() else {
                panic!("ledger row needs four tab-separated fields: {line:?}");
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
                variant,
            }
        })
        .collect()
}

fn validate_ledger(rows: &[Row]) {
    let mut codes = BTreeSet::new();
    let mut labels = BTreeSet::new();
    let mut active_variants = BTreeSet::new();
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
        let allowed_owner = match domain {
            'E' => matches!(owner, "elab" | "compiler" | "native_build"),
            'K' => owner == "kernel",
            'X' => matches!(owner, "run" | "run_io"),
            'N' => owner == "native",
            'H' => owner == "host",
            'C' => owner == "cli",
            _ => unreachable!("domain shape checked above"),
        };
        assert!(
            allowed_owner,
            "domain {domain} does not own label {}",
            row.label
        );
        let Some((enum_name, variant_name)) = row.variant.split_once("::") else {
            panic!("variant must be Enum::Variant: {}", row.variant);
        };
        assert!(
            [enum_name, variant_name].into_iter().all(|part| {
                !part.is_empty()
                    && part
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            }) && !variant_name.contains("::"),
            "variant must be Enum::Variant: {}",
            row.variant
        );
        if row.status == Status::Active {
            assert!(
                active_variants.insert(row.variant),
                "active variant reused: {}",
                row.variant
            );
        }
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
pub fn assert_table_matches_ledger(owned_prefixes: &[&str], table: &[(Code, &str, &str)]) {
    assert_table_matches_rows(&ledger(), owned_prefixes, table);
}

fn assert_table_matches_rows(rows: &[Row], owned_prefixes: &[&str], table: &[(Code, &str, &str)]) {
    let owns = |label: &str| {
        owned_prefixes
            .iter()
            .any(|prefix| label.starts_with(prefix))
    };
    let mut table_entries = BTreeSet::new();
    for (code, label, variant) in table {
        assert!(
            table_entries.insert((code.as_str(), *label, *variant)),
            "duplicate table entry: {code} {label} {variant}"
        );
        assert!(
            owns(label),
            "table entry {code} {label} is outside its owner's prefixes"
        );
        assert!(
            rows.iter().any(|row| {
                row.status == Status::Active
                    && row.code == code.as_str()
                    && row.label == *label
                    && row.variant == *variant
            }),
            "table entry {code} {label} {variant} is not an active ledger row"
        );
    }
    for row in rows
        .iter()
        .filter(|row| row.status == Status::Active && owns(row.label))
    {
        assert!(
            table_entries.contains(&(row.code, row.label, row.variant)),
            "ledger row {} {} {} has no producing table entry",
            row.code,
            row.label,
            row.variant
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
        const $table: &[($crate::Code, &str, &str)] = &[
            $( ($crate::Code::new($code), $label,
                concat!(stringify!($ty), "::", stringify!($variant))), )*
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
        const $table: &[($crate::Code, &str, &str)] = &[
            $( ($crate::Code::new($code), $label,
                concat!(stringify!($path), "::", stringify!($variant))), )*
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
        assert_eq!(
            render_line(Code::new("C0003"), "bad\nsubcommand\rname"),
            "error[C0003]: bad\\nsubcommand\\rname"
        );
    }

    #[test]
    fn registry_rejects_independent_malformations() {
        let good = "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.unbound_name\tretired\tElabError::UnboundName\n";
        let verify = |source| validate_ledger(&parse_ledger(source));
        verify(good);
        // Each control changes only one axis relative to a valid ledger.
        for bad in [
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0001\telab.unbound_name\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.parse_error\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0003\telab.unbound_name\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.unbound_name\tobsolete\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.unbound_name\tretired\n",
            "E0001\telab.parse_error\tactive\textra\tElabError::ParseError\nE0002\telab.unbound_name\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nK00A1\telab.unbound_name\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\t\tretired\tElabError::UnboundName\n",
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.unbound_name\tactive\tElabError::ParseError\n",
            "K0001\telab.parse_error\tactive\tKernelError::ParseError\n",
            "E0001\telab.parse_error\tactive\tParseError\n",
        ] {
            assert!(
                std::panic::catch_unwind(|| verify(bad)).is_err(),
                "malformed ledger was accepted: {bad:?}"
            );
        }
    }

    #[test]
    fn owner_table_rejects_missing_unregistered_and_retired_entries() {
        let rows = parse_ledger(
            "E0001\telab.parse_error\tactive\tElabError::ParseError\nE0002\telab.old_error\tretired\tElabError::OldError\n",
        );
        validate_ledger(&rows);
        let check =
            |table: &[(Code, &str, &str)]| assert_table_matches_rows(&rows, &["elab."], table);
        let valid = [(
            Code::new("E0001"),
            "elab.parse_error",
            "ElabError::ParseError",
        )];
        check(&valid);
        for bad in [
            vec![],
            vec![(Code::new("E9999"), "elab.missing", "ElabError::Missing")],
            vec![
                valid[0],
                (Code::new("E0002"), "elab.old_error", "ElabError::OldError"),
            ],
            vec![(
                Code::new("E0001"),
                "elab.parse_error",
                "ElabError::UnboundName",
            )],
            vec![valid[0], valid[0]],
        ] {
            assert!(
                std::panic::catch_unwind(|| check(&bad)).is_err(),
                "invalid table was accepted: {bad:?}"
            );
        }
        // An in-lockstep pair swap of two generated arms cannot pass the ledger.
        let two = parse_ledger(
            "K0001\tkernel.var_out_of_scope\tactive\tKernelError::VarOutOfScope\nK0002\tkernel.type_mismatch\tactive\tKernelError::TypeMismatch\n",
        );
        validate_ledger(&two);
        let swapped = [
            (
                Code::new("K0002"),
                "kernel.type_mismatch",
                "KernelError::VarOutOfScope",
            ),
            (
                Code::new("K0001"),
                "kernel.var_out_of_scope",
                "KernelError::TypeMismatch",
            ),
        ];
        assert!(
            std::panic::catch_unwind(|| assert_table_matches_rows(&two, &["kernel."], &swapped))
                .is_err(),
            "swapped variants must not inherit each other's registered code"
        );
        // The public entry point reads and validates the production ledger.
        assert!(ledger().iter().any(|row| row.code == "E0001"));
    }
}
