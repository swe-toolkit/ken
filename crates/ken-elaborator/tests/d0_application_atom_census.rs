//! D0 census for `LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE`.
//!
//! Walks the parsed catalog for application arguments that the `32 §3`
//! application-atom contract pin forbids ungrouped. The census is structural:
//! it reads the parser's own AST, so it discriminates `d.leq`-as-projection
//! from `Module.name`-as-qualified-path (the latter is joined at parse time and
//! never becomes an `EProj` — `ast.rs` `EProj` doc), which no source-level
//! pattern can do.
//!
//! Closed by construction against new `Expr` and `Type` VARIANTS: every match
//! below is exhaustive with no `_` arm, so adding a variant is a compile error
//! rather than a silently skipped subtree. Residual, stated: `Decl` arms bind
//! the expression-bearing fields and use `..` for the rest, so a new
//! expression-bearing FIELD on an existing `Decl` variant would not error.

use ken_elaborator::literate::extract_ken_md;
use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    ConstructorSignatureArg, Decl, ExplicitDataCtor, Expr, LetBinding, SpaceCell,
    SpaceOperation, Type,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Row {
    Projection,
    If,
    OperatorName,
    Arrow,
    LambdaLetMatch,
}

#[derive(Debug, Clone)]
struct Site {
    row: Row,
    offset: usize,
    text: String,
}

fn is_symbolic(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|c| !c.is_alphanumeric() && c != '_')
}

fn classify_arg(arg: &Expr) -> Option<(Row, usize)> {
    match arg {
        Expr::EProj(_, _, s) | Expr::EPosProj(_, _, s) => Some((Row::Projection, s.start)),
        Expr::EIf { span, .. } => Some((Row::If, span.start)),
        Expr::EArrow(_, _, s) => Some((Row::Arrow, s.start)),
        Expr::ELam(_, _, s) | Expr::ELet(_, _, s) => Some((Row::LambdaLetMatch, s.start)),
        Expr::EMatch { span, .. } => Some((Row::LambdaLetMatch, span.start)),
        Expr::EVar(name, s) if is_symbolic(name) => Some((Row::OperatorName, s.start)),
        _ => None,
    }
}

fn walk_expr(e: &Expr, out: &mut Vec<(Row, usize)>) {
    if let Expr::EApp(head, arg, _) = e {
        // A PARENTHESIZED argument's span starts at its `(`; an ungrouped one
        // starts at its own first token. Measured on the parser, not assumed:
        //   keep box.value    EProj span 21..30, base EVar 21..24  -> ungrouped
        //   keep (box.value)  EProj span 21..32, base EVar 22..25  -> grouped
        // Grouped arguments are LEGAL under the pin and are not migration
        // sites; without this the arrow row alone reports 450 false positives.
        if let Some(hit) = classify_arg(arg) {
            out.push(hit);
        }
        walk_expr(head, out);
        walk_expr(arg, out);
        return;
    }
    match e {
        Expr::EVar(..)
        | Expr::ECon(..)
        | Expr::EUniv(..)
        | Expr::ENumLit(..)
        | Expr::EStr(..)
        | Expr::ECharLit(..)
        | Expr::EByteStr(..)
        | Expr::EAttachedProofRef { .. }
        | Expr::ERecursiveResult { .. } => {}
        Expr::EApp(..) => unreachable!("handled above"),
        Expr::ELam(_, b, _) | Expr::EOld(b, _) | Expr::ETrunc(b, _) => walk_expr(b, out),
        Expr::EProj(b, _, _) | Expr::EPosProj(b, _, _) => walk_expr(b, out),
        Expr::EBecomes(a, b, _) | Expr::EArrow(a, b, _) => {
            walk_expr(a, out);
            walk_expr(b, out);
        }
        Expr::EBinOp(_, a, b, _) => {
            walk_expr(a, out);
            walk_expr(b, out);
        }
        Expr::EAsc(b, t, _) => {
            walk_expr(b, out);
            walk_type(t, out);
        }
        Expr::EPi(_, t, b, _) => {
            walk_type(t, out);
            walk_expr(b, out);
        }
        Expr::ELet(bindings, body, _) => {
            for LetBinding {
                annotation, value, ..
            } in bindings
            {
                if let Some(t) = annotation {
                    walk_type(t, out);
                }
                walk_expr(value, out);
            }
            walk_expr(body, out);
        }
        Expr::EInfixSpine { operands, .. } => {
            for operand in operands {
                if let Expr::EVar(name, s) = operand {
                    if is_symbolic(name) {
                        out.push((Row::OperatorName, s.start));
                    }
                }
                walk_expr(operand, out);
            }
        }
        Expr::EMatch { scrut, arms, .. } => {
            walk_expr(scrut, out);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    walk_expr(g, out);
                }
                walk_expr(&arm.body, out);
            }
        }
        Expr::EIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            walk_expr(condition, out);
            walk_expr(then_branch, out);
            walk_expr(else_branch, out);
        }
        Expr::EPair(items, _) => {
            for i in items {
                walk_expr(i, out);
            }
        }
        Expr::ERecord { base, fields, .. } => {
            if let Some(b) = base {
                walk_expr(b, out);
            }
            for f in fields {
                walk_expr(&f.value, out);
            }
        }
    }
}

fn walk_type(t: &Type, out: &mut Vec<(Row, usize)>) {
    match t {
        Type::TUniv(..) | Type::TCon(..) | Type::TVar(..) => {}
        Type::TTrunc(a, _) => walk_type(a, out),
        Type::TPi(_, a, b, _) | Type::TSigma(_, a, b, _) | Type::TArr(a, b, _) => {
            walk_type(a, out);
            walk_type(b, out);
        }
        Type::TApp(a, b, _) => {
            walk_type(a, out);
            walk_type(b, out);
        }
        Type::TEffectArr(a, _, b, _) => {
            walk_type(a, out);
            walk_type(b, out);
        }
        Type::TRefine(_, a, e, _) => {
            walk_type(a, out);
            walk_expr(e, out);
        }
    }
}

fn walk_decl(d: &Decl, out: &mut Vec<(Row, usize)>) {
    match d {
        Decl::BoundaryDecl { .. }
        | Decl::FixityDecl { .. }
        | Decl::DeriveDecl { .. }
        | Decl::TemporalDecl { .. }
        | Decl::ExportDecl { .. }
        | Decl::ImportDecl { .. } => {}
        Decl::Pub(inner) => walk_decl(inner, out),
        Decl::ModuleDecl { decls, .. } => {
            for inner in decls {
                walk_decl(inner, out);
            }
        }
        Decl::ViewDecl {
            params,
            ret_ty,
            requires,
            ensures,
            body,
            ..
        } => {
            for b in params { walk_type(&b.ty, out); }
            if let Some(t) = ret_ty {
                walk_type(t, out);
            }
            for e in requires.iter().chain(ensures) {
                walk_expr(e, out);
            }
            walk_expr(body, out);
        }
        Decl::SpaceDecl {
            cells, operations, ..
        } => {
            for SpaceCell { ty, init, .. } in cells {
                walk_type(ty, out);
                walk_expr(init, out);
            }
            for SpaceOperation {
                params,
                ret_ty,
                requires,
                ensures,
                body,
                ..
            } in operations
            {
                for b in params { walk_type(&b.ty, out); }
                walk_type(ret_ty, out);
                for e in requires.iter().chain(ensures) {
                    walk_expr(e, out);
                }
                walk_expr(body, out);
            }
        }
        Decl::LetDecl { ty, val, .. } => {
            if let Some(t) = ty {
                walk_type(t, out);
            }
            walk_expr(val, out);
        }
        Decl::ProveDecl { prop, .. } => walk_expr(prop, out),
        Decl::PropDecl {
            params,
            ret_ty,
            intros,
            ..
        } => {
            for b in params { walk_type(&b.ty, out); }
            walk_type(ret_ty, out);
            for i in intros {
                walk_type(&i.ty, out);
            }
        }
        Decl::TheoremDecl {
            params,
            theorem,
            body,
            ..
        }
        | Decl::AttachedProofDecl {
            params,
            theorem,
            body,
            ..
        } => {
            for b in params { walk_type(&b.ty, out); }
            walk_type(theorem, out);
            walk_expr(body, out);
        }
        Decl::AxiomDecl { theorem, .. } => walk_type(theorem, out),
        Decl::LawDecl { fields, .. } | Decl::InstanceDecl { fields, .. } => {
            for (_, e) in fields {
                walk_expr(e, out);
            }
        }
        Decl::DataDecl { ctors, .. } => {
            for c in ctors {
                for a in &c.args {
                    walk_type(a, out);
                }
            }
        }
        Decl::ExplicitDataDecl {
            params,
            family,
            ctors,
            ..
        } => {
            for b in params { walk_type(&b.ty, out); }
            walk_type(family, out);
            for c in ctors {
                match c {
                    ExplicitDataCtor::Simple(inner) => {
                        for a in &inner.args {
                            walk_type(a, out);
                        }
                    }
                    ExplicitDataCtor::Signature { signature, .. } => {
                        for a in &signature.args {
                            match a {
                                ConstructorSignatureArg::Explicit(b)
                                | ConstructorSignatureArg::Implicit(b) => walk_type(&b.ty, out),
                                ConstructorSignatureArg::Anonymous(e) => walk_expr(e, out),
                            }
                        }
                        walk_expr(&signature.result, out);
                    }
                }
            }
        }
        Decl::TypeAlias { ty, .. } | Decl::ForeignDecl { ty, .. } => walk_type(ty, out),
        Decl::RecordDecl { fields, .. } => {
            for (_, t) in fields {
                walk_type(t, out);
            }
        }
        Decl::ClassDecl {
            param_kind, fields, ..
        } => {
            if let Some(t) = param_kind {
                walk_type(t, out);
            }
            for f in fields {
                walk_type(&f.ty, out);
            }
        }
    }
}

fn collect_files(root: &Path, acc: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect_files(&p, acc);
        } else {
            let n = p.to_string_lossy();
            if n.ends_with(".ken.md") || n.ends_with(".ken") {
                acc.push(p);
            }
        }
    }
}

fn line_of(src: &str, offset: usize) -> usize {
    src.as_bytes()[..offset.min(src.len())]
        .iter()
        .filter(|&&b| b == b'\n')
        .count()
        + 1
}

/// Blank everything outside `range`, preserving byte offsets. Must operate on
/// BYTES: `b as char` re-encodes any byte above 0x7F as its Latin-1 codepoint
/// and corrupts every multi-byte UTF-8 sequence in the kept region, which
/// surfaced as 23 spurious `NonAsciiIdentifierCharacter` parse failures --
/// i.e. as silent UNDER-reporting of the census.
fn blank_outside(src: &str, range: &std::ops::Range<usize>) -> String {
    let kept: Vec<u8> = src
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            if b == b'\n' || range.contains(&i) {
                b
            } else {
                b' '
            }
        })
        .collect();
    String::from_utf8(kept).expect("range boundaries are line-aligned, so UTF-8 stays intact")
}

#[test]
fn d0_application_atom_census() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf();

    let mut files = Vec::new();
    for root in ["catalog", "library", "conformance"] {
        collect_files(&repo.join(root), &mut files);
    }
    files.sort();

    let mut ledger: BTreeMap<String, Vec<Site>> = BTreeMap::new();
    let mut parse_failures: Vec<(String, String)> = Vec::new();
    let mut scanned = 0usize;

    for path in &files {
        let rel = path
            .strip_prefix(&repo)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        let Ok(src) = std::fs::read_to_string(path) else {
            parse_failures.push((rel, "unreadable".into()));
            continue;
        };

        let mut units: Vec<(String, String)> = Vec::new();
        if rel.ends_with(".ken.md") {
            match extract_ken_md(&src) {
                Ok(ex) => {
                    units.push(("compiled".into(), ex.source.clone()));
                    for r in &ex.example_ranges {
                        units.push(("example".into(), blank_outside(&src, r)));
                    }
                    for r in &ex.reject_ranges {
                        units.push(("reject".into(), blank_outside(&src, r)));
                    }
                }
                Err(e) => {
                    parse_failures.push((rel.clone(), format!("extract: {e:?}")));
                    continue;
                }
            }
        } else {
            units.push(("source".into(), src.clone()));
        }

        scanned += 1;
        for (kind, unit) in units {
            match parse_decls(&unit) {
                Ok(decls) => {
                    let mut hits = Vec::new();
                    for d in &decls {
                        walk_decl(d, &mut hits);
                    }
                    for (row, offset) in hits {
                        if src.as_bytes().get(offset) == Some(&b'(') {
                            continue; // grouped: legal under the pin
                        }
                        let line = line_of(&src, offset);
                        ledger.entry(rel.clone()).or_default().push(Site {
                            row,
                            offset,
                            text: format!("{rel}:{line} [{kind}]"),
                        });
                    }
                }
                Err(e) => parse_failures.push((format!("{rel} [{kind}]"), format!("{e:?}"))),
            }
        }
    }

    let mut by_row: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for sites in ledger.values() {
        for s in sites {
            by_row
                .entry(format!("{:?}", s.row))
                .or_default()
                .push(s.text.clone());
        }
    }

    println!("\n===== D0 LEDGER =====");
    println!("files discovered: {}  scanned: {}", files.len(), scanned);
    // Print EVERY row, including empty ones. A row absent from the output
    // reads as "not looked at"; an explicit 0 beside a passing positive
    // control is a measurement.
    for empty in ["Projection", "If", "OperatorName", "Arrow", "LambdaLetMatch"] {
        by_row.entry(empty.to_string()).or_default();
    }
    for (row, mut sites) in by_row {
        sites.sort();
        sites.dedup();
        println!("\n-- row {row}: {} site(s)", sites.len());
        for s in &sites {
            println!("   {s}");
        }
    }
    println!("\n-- parse/extract failures: {}", parse_failures.len());
    for (f, e) in &parse_failures {
        println!("   {f}: {}", e.chars().take(110).collect::<String>());
    }
    println!("===== END D0 LEDGER =====\n");
}

/// How to read each ROW's zero. A zero means different things per row, and
/// without this they are indistinguishable:
///
///   Projection / If   the divergent shape IS producible, so the instrument
///                     fires on a synthetic case and a catalog zero is
///                     "none found".
///   Arrow / lambda / let / match / bare operator
///                     the divergent shape is NOT producible at this base --
///                     the source either already parses into the conforming
///                     tree or is a parse error -- so a zero is "no such
///                     source can exist", which is stronger than "none found".
#[test]
fn d0_row_reachability_controls() {
    // Rows whose divergent shape exists: the instrument must fire.
    for (row, src) in [
        (Row::Projection, "const k : Nat = keep box.value"),
        (Row::If, "const k : Nat = keep if c then a else b"),
    ] {
        let decls = parse_decls(src).expect("control source parses");
        let mut hits = Vec::new();
        for d in &decls {
            walk_decl(d, &mut hits);
        }
        let fired = hits
            .iter()
            .filter(|(r, off)| *r == row && src.as_bytes().get(*off) != Some(&b'('))
            .count();
        assert!(fired >= 1, "instrument blind to {row:?}: {src}");
        println!("CONTROL {row:?}: producible, instrument fires ({fired}) -> catalog 0 would mean NONE FOUND");
    }

    // Rows whose divergent shape is not producible. Each is asserted for the
    // REASON it is unreachable, not merely that the walk returned nothing.
    let arrow = parse_decls("const k : Nat = keep a -> b").expect("arrow parses");
    assert!(
        format!("{arrow:?}").contains("EArrow(EApp("),
        "arrow must already nest the application in its left expr"
    );
    println!("CONTROL Arrow: `keep a -> b` parses as EArrow(EApp(..)) -- already the pin's shape");

    for src in [
        "const k : Nat = keep \\ x . x",
        "const k : Nat = keep let z = a in z",
        "const k : Nat = keep match x { A |-> b }",
    ] {
        assert!(parse_decls(src).is_err(), "must reject ungrouped: {src}");
    }
    println!("CONTROL LambdaLetMatch: all three reject ungrouped after an application head");

    let infix = parse_decls("const k : Nat = map \u{2264} xs").expect("infix parses");
    assert!(
        format!("{infix:?}").contains("EInfixSpine"),
        "an operator between atoms is infix, never a bare argument"
    );
    assert!(parse_decls("const k : Nat = keep \u{2264}").is_err());
    let grouped = parse_decls("const k : Nat = keep (\u{2264}) xs").expect("grouped parses");
    assert!(format!("{grouped:?}").contains("EApp(EApp("));
    println!("CONTROL OperatorName: between atoms it is INFIX, trailing it is a parse error, grouped it is legal -> the divergent shape is unreachable");
}
