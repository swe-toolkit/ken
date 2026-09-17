//! Census instrument for `LANG-BARE-OPERATOR-ATOM-REJECTION` (frame ask 2).
//!
//! NOT A COMMITTED TEST. This is a one-shot MEASUREMENT, recovered from the
//! retired `d0_application_atom_census.rs` at
//! `b17ab14bd91cc10caaa89af91d5806b4e768d5b6` and RE-AIMED. It is a test of
//! repository text, which the operator rule forbids as a committed oracle; it
//! exists to answer "is there a migration tail?" once, and is deleted after the
//! number is stated.
//!
//! THE PREDICATE IS NOT THE ONE IT INHERITED. The D0 `OperatorName` row asked
//! about an operator in ARGUMENT position (`keep <+>`), and correctly found
//! that shape unreachable -- the parser already rejects it. This row asks about
//! an operator that is an ungrouped VALUE: a symbolic `EVar` that is
//! (a) not grouped, and (b) not in application-HEAD position. That shape IS
//! reachable at this base -- `const k : Nat = <+>` parses to a bare `EVar` --
//! so a catalog zero here means NONE FOUND, not "no such source can exist".
//!
//! Soundness is one-directional by design: a false negative is fatal (it hides
//! a migration tail and mis-sizes the node), a false positive merely costs a
//! read. The predicate is therefore an over-approximation, and the negatives
//! table below is the anti-degeneracy guard -- without it, `|_| true` is
//! trivially "sound".
//!
//! Closed by construction against new `Expr`/`Type` VARIANTS: every match is
//! exhaustive with no `_` arm. Residual, stated: `Decl` arms bind the
//! expression-bearing fields and use `..`, so a new expression-bearing FIELD on
//! an existing `Decl` variant would not error.

use ken_elaborator::lexer::{Lexer, Token};
use ken_elaborator::literate::extract_ken_md;
use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    ConstructorSignatureArg, Decl, ExplicitDataCtor, Expr, LetBinding, SpaceCell, SpaceOperation,
    Type,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Site {
    offset: usize,
    text: String,
}

thread_local! {
    /// Every symbolic `EVar` the walk sees, head-position and grouped included.
    /// Without this, a zero ledger and an instrument that never reached any
    /// catalog expression print the same number.
    static BASE_RATE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Every `EVar` of any kind -- the outermost base rate.
    static ALL_VARS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Every DISTINCT `EVar` name, so the zero carries a denominator and the
    /// residue can be inspected instead of asserted.
    static NAMES: std::cell::RefCell<std::collections::BTreeSet<String>> =
        std::cell::RefCell::new(std::collections::BTreeSet::new());
}

/// An `operator_name` is whatever the LEXER calls one -- not whatever looks
/// symbolic to me. The parser's own `canonical_operator_name` is private, so
/// this replicates its arms exactly against `Lexer::lex`, which is the producer
/// that decides whether `parser.rs:2965`'s arm fires at all.
///
/// The heuristic this replaces ("first char is not alphanumeric or `_`") is a
/// claim about SPELLING, and a false negative here is the fatal direction: it
/// would hide a migration site and mis-size the node. Asking the lexer removes
/// the guess.
fn is_operator_name(name: &str) -> bool {
    let Ok(toks) = Lexer::lex(name) else {
        return false;
    };
    let mut real = toks
        .iter()
        .map(|(t, _)| t)
        .filter(|t| !matches!(t, Token::Eof));
    let first = real.next();
    if real.next().is_some() {
        return false; // more than one token: not a single operator_name
    }
    matches!(
        first,
        Some(
            Token::Operator(_)
                | Token::Le
                | Token::Ge
                | Token::Ne
                | Token::And
                | Token::Or
                | Token::Member
        )
    )
}

/// `in_head` is the whole re-aiming. An `operator_name` is LEGAL as an
/// `operator_prefix` head with at least one following atom, so the head side of
/// every `EApp` is exempt and every other position is a candidate.
///
/// The grouping half of the predicate is NOT decided here: a grouped `(<+>)`
/// and a bare `<+>` produce the SAME node and their spans start at the SAME
/// offset, so the AST cannot discriminate them. The discriminator is the source
/// BYTE at `span.start`, applied at the call site against the original text.
fn walk_expr(e: &Expr, in_head: bool, out: &mut Vec<usize>) {
    if let Expr::EApp(head, arg, _) = e {
        walk_expr(head, true, out);
        walk_expr(arg, false, out);
        return;
    }
    if let Expr::EVar(name, s) = e {
        ALL_VARS.with(|c| c.set(c.get() + 1));
        NAMES.with(|n| {
            n.borrow_mut().insert(name.clone());
        });
        if is_operator_name(name) {
            BASE_RATE.with(|c| c.set(c.get() + 1));
            if !in_head {
                out.push(s.start);
            }
        }
        return;
    }
    match e {
        Expr::EVar(..) => unreachable!("handled above"),
        Expr::ECon(..)
        | Expr::EUniv(..)
        | Expr::ENumLit(..)
        | Expr::EStr(..)
        | Expr::ECharLit(..)
        | Expr::EByteStr(..)
        | Expr::EAttachedProofRef { .. }
        | Expr::ERecursiveResult { .. } => {}
        Expr::EApp(..) => unreachable!("handled above"),
        Expr::ELam(_, b, _) | Expr::EOld(b, _) | Expr::ETrunc(b, _) => walk_expr(b, false, out),
        Expr::EProj(b, _, _) | Expr::EPosProj(b, _, _) => walk_expr(b, false, out),
        Expr::EBecomes(a, b, _) | Expr::EArrow(a, b, _) => {
            walk_expr(a, false, out);
            walk_expr(b, false, out);
        }
        Expr::EBinOp(_, a, b, _) => {
            walk_expr(a, false, out);
            walk_expr(b, false, out);
        }
        Expr::EAsc(b, t, _) => {
            walk_expr(b, false, out);
            walk_type(t, out);
        }
        Expr::EPi(_, t, b, _) => {
            walk_type(t, out);
            walk_expr(b, false, out);
        }
        Expr::ELet(bindings, body, _) => {
            for LetBinding {
                annotation, value, ..
            } in bindings
            {
                if let Some(t) = annotation {
                    walk_type(t, out);
                }
                walk_expr(value, false, out);
            }
            walk_expr(body, false, out);
        }
        // An infix spine's OPERATOR is not an operand (measured: `x <+> y`
        // yields `EInfixSpine` with operands `[x, y]`), so the untouched infix
        // path at `parser.rs:2254` contributes nothing here. A symbolic
        // OPERAND, however, is a bare operator used as a value -- this row.
        Expr::EInfixSpine { operands, .. } => {
            for operand in operands {
                walk_expr(operand, false, out);
            }
        }
        Expr::EMatch { scrut, arms, .. } => {
            walk_expr(scrut, false, out);
            for arm in arms {
                if let Some(g) = &arm.guard {
                    walk_expr(g, false, out);
                }
                walk_expr(&arm.body, false, out);
            }
        }
        Expr::EIf {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            walk_expr(condition, false, out);
            walk_expr(then_branch, false, out);
            walk_expr(else_branch, false, out);
        }
        Expr::EPair(items, _) => {
            for i in items {
                walk_expr(i, false, out);
            }
        }
        Expr::ERecord { base, fields, .. } => {
            if let Some(b) = base {
                walk_expr(b, false, out);
            }
            for f in fields {
                walk_expr(&f.value, false, out);
            }
        }
    }
}

fn walk_type(t: &Type, out: &mut Vec<usize>) {
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
            walk_expr(e, false, out);
        }
    }
}

fn walk_decl(d: &Decl, out: &mut Vec<usize>) {
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
            for b in params {
                walk_type(&b.ty, out);
            }
            if let Some(t) = ret_ty {
                walk_type(t, out);
            }
            for e in requires.iter().chain(ensures) {
                walk_expr(e, false, out);
            }
            walk_expr(body, false, out);
        }
        Decl::SpaceDecl {
            cells, operations, ..
        } => {
            for SpaceCell { ty, init, .. } in cells {
                walk_type(ty, out);
                walk_expr(init, false, out);
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
                for b in params {
                    walk_type(&b.ty, out);
                }
                walk_type(ret_ty, out);
                for e in requires.iter().chain(ensures) {
                    walk_expr(e, false, out);
                }
                walk_expr(body, false, out);
            }
        }
        Decl::LetDecl { ty, val, .. } => {
            if let Some(t) = ty {
                walk_type(t, out);
            }
            walk_expr(val, false, out);
        }
        Decl::ProveDecl { prop, .. } => walk_expr(prop, false, out),
        Decl::PropDecl {
            params,
            ret_ty,
            intros,
            ..
        } => {
            for b in params {
                walk_type(&b.ty, out);
            }
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
            for b in params {
                walk_type(&b.ty, out);
            }
            walk_type(theorem, out);
            walk_expr(body, false, out);
        }
        Decl::AxiomDecl { theorem, .. } => walk_type(theorem, out),
        Decl::LawDecl { fields, .. } | Decl::InstanceDecl { fields, .. } => {
            for (_, e) in fields {
                walk_expr(e, false, out);
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
            for b in params {
                walk_type(&b.ty, out);
            }
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
                                ConstructorSignatureArg::Anonymous(e) => walk_expr(e, false, out),
                            }
                        }
                        walk_expr(&signature.result, false, out);
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
fn bare_operator_atom_census() {
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
    let mut grouped_skipped = 0usize;

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
                    for offset in hits {
                        // The grouped/bare discriminator. `(<+>)` and `<+>`
                        // produce the SAME node with the SAME span start, so
                        // this byte is the only thing that separates the legal
                        // form from the divergent one.
                        if src.as_bytes().get(offset) == Some(&b'(') {
                            grouped_skipped += 1;
                            continue;
                        }
                        let line = line_of(&src, offset);
                        ledger.entry(rel.clone()).or_default().push(Site {
                            offset,
                            text: format!("{rel}:{line} [{kind}]"),
                        });
                    }
                }
                Err(e) => parse_failures.push((format!("{rel} [{kind}]"), format!("{e:?}"))),
            }
        }
    }

    // Dedup on (file, BYTE OFFSET), never on the rendered `file:line`: two
    // distinct sites on one line collapse under a line key, which under-counted
    // the projection ledger 24 -> 23 on the predecessor node.
    let mut sites: Vec<(String, usize, String)> = Vec::new();
    for (file, ss) in &ledger {
        for s in ss {
            sites.push((file.clone(), s.offset, s.text.clone()));
        }
    }
    sites.sort();
    sites.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    println!("\n===== BARE-OPERATOR LEDGER =====");
    println!("files discovered: {}  scanned: {}", files.len(), scanned);
    println!("grouped occurrences skipped (legal, preserved): {grouped_skipped}");
    println!(
        "BASE RATE  EVar total: {}   symbolic EVar total (head+non-head): {}",
        ALL_VARS.with(|c| c.get()),
        BASE_RATE.with(|c| c.get())
    );
    NAMES.with(|n| {
        let names = n.borrow();
        let ops: Vec<&String> = names.iter().filter(|x| is_operator_name(x)).collect();
        let odd: Vec<&String> = names
            .iter()
            .filter(|x| {
                x.chars()
                    .next()
                    .is_some_and(|c| !c.is_alphanumeric() && c != '_')
            })
            .collect();
        println!("DENOMINATOR distinct EVar names: {}", names.len());
        println!(
            "  ... that the LEXER calls an operator_name: {} {ops:?}",
            ops.len()
        );
        println!(
            "  ... that merely LOOK symbolic (old heuristic): {} {odd:?}",
            odd.len()
        );
    });
    println!(
        "\n-- bare ungrouped non-head operator_name: {} site(s)",
        sites.len()
    );
    for (_, off, text) in &sites {
        println!("   {text}  @byte {off}");
    }
    println!("\n-- parse/extract failures: {}", parse_failures.len());
    for (f, e) in &parse_failures {
        println!("   {f}: {}", e.chars().take(140).collect::<String>());
    }
    println!("===== END BARE-OPERATOR LEDGER =====\n");
}

/// The anti-degeneracy guard. A census whose predicate is `|_| true` is
/// trivially free of false negatives, so "sound in the fatal direction" is
/// worth nothing without a NEGATIVES table: each row below is a form the
/// parser accepts today and that this node must PRESERVE, and each must NOT
/// fire. The positive row is what makes a catalog zero mean "none found"
/// rather than "the instrument cannot see anything".
#[test]
fn census_reachability_and_negative_controls() {
    fn fire(src: &str) -> usize {
        let decls = parse_decls(src).expect("control source parses");
        let mut hits = Vec::new();
        for d in &decls {
            walk_decl(d, &mut hits);
        }
        hits.iter()
            .filter(|off| src.as_bytes().get(**off) != Some(&b'('))
            .count()
    }

    // POSITIVE: the divergent shape is PRODUCIBLE at this base. This is the
    // difference from the D0 row this instrument was recovered from, whose
    // argument-position shape the parser already rejected.
    //
    // These are NOT all top-level bodies. A positive control that only ever
    // fires on `LetDecl.val` proves the KEY and says nothing about REACH, and a
    // catalog zero would then be indistinguishable from a walker that never
    // descends. Each row below puts the operator in a different NESTED
    // expression position, so a zero on the catalog is a fact about the
    // catalog.
    for (position, src) in [
        ("decl body", "const k : Nat = <+>"),
        ("decl body, glyph", "const k : Nat = \u{2264}"),
        ("let binding value", "const k : Nat = let z = <+> in z"),
        ("lambda body", "const k : Nat = \\ x . <+>"),
        ("if branch", "const k : Nat = if c then <+> else g"),
        ("match arm body", "const k : Nat = match x { A |-> <+> }"),
        ("record field", "const k : Nat = { a = <+> }"),
        ("under a grouped argument", "const k : Nat = f (\\ x . <+>)"),
    ] {
        let n = fire(src);
        assert!(n >= 1, "instrument blind at {position}: {src}");
        println!("POSITIVE  fires({n})  {position}: {src}");
    }

    // Measured, not assumed: an operator TRAILING an application head is
    // already a parse error at this base, so it is not part of this row and
    // cannot be a positive control. Recording it keeps the next reader from
    // re-deriving it as I did.
    assert!(
        parse_decls("const k : Nat = f (g \u{2264})").is_err(),
        "a trailing operator is expected to already reject"
    );
    println!("NOT THIS ROW  `f (g \u{2264})` already rejects -- argument position is closed");

    // NEGATIVES: every form this node must leave working.
    for (why, src) in [
        ("grouped operand is legal", "const k : Nat = (<+>)"),
        ("grouped glyph is legal", "const k : Nat = (\u{2264})"),
        ("prefix head with one atom", "const k : Nat = <+> Zero"),
        ("prefix head with two atoms", "const k : Nat = <+> Zero One"),
        ("glyph prefix head", "const k : Nat = \u{2264} Zero One"),
        (
            "infix spine: operator is not an operand",
            "const k : Nat = x <+> y",
        ),
        ("infix glyph spine", "const k : Nat = x \u{2264} y"),
        ("grouped, then applied", "const k : Nat = (<+>) a b"),
        ("ordinary application untouched", "const k : Nat = f a b"),
    ] {
        let n = fire(src);
        assert_eq!(n, 0, "false positive ({why}): {src}");
        println!("NEGATIVE  fires(0)   {why}: {src}");
    }
}
