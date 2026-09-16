//! `LANG-APPLICATION-ATOM-CONTRACT-CONFORMANCE` acceptance.
//!
//! Spec source: `spec/30-surface/32-grammar.md §3` — the application-atom
//! contract pin. Application (`:264`) and projection (`:270`) are both
//! left-recursive postfix productions over `expr` at the same level, so source
//! order alone decides the nesting.
//!
//! Every assertion here is on the **AST**, never on a re-printed string: a
//! printer that re-emits the same source cannot distinguish the two trees, and
//! that indistinguishability is the whole defect.
//!
//! Promise classes: the nesting of application against projection is a durable
//! invariant (it is what `§3` pins). `old`'s binding is a transition sentinel —
//! it is asserted UNCHANGED here so that a future ruling on `21 §6.4` has to
//! come and change it deliberately rather than inherit it from a postfix edit.

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl, ElabError, Expr};

/// The body expression of a single-declaration source.
fn body_of(src: &str) -> Expr {
    let decls = parse_decls(src).unwrap_or_else(|e| panic!("{src} must parse: {e:?}"));
    match decls.into_iter().next().expect("one declaration") {
        Decl::ViewDecl { body, .. } => body,
        other => panic!("expected a view declaration, got {other:?}"),
    }
}

fn err_of(src: &str) -> ElabError {
    parse_decls(src).expect_err("must reject")
}

fn var(e: &Expr, name: &str) -> bool {
    matches!(e, Expr::EVar(n, _) if n == name) || matches!(e, Expr::ECon(n, _) if n == name)
}

const KEEP_BOX_VALUE: &str = "const k : Nat = keep box.value";
const BOX_VALUE_KEEP: &str = "const k : Nat = box.value keep";
const INTERLEAVED: &str = "const k : Nat = f a.b c";
const GROUPED_HEAD: &str = "const k : Nat = (keep box).value";
const GROUPED_ARG: &str = "const k : Nat = keep (box.value)";

/// AC-PROJ-NESTS, assertion 1. The pin's own case: an ungrouped projection
/// after an application head nests the APPLICATION in its left `expr`.
#[test]
fn ac_proj_nests_argument_projection_rides_the_spine() {
    match body_of(KEEP_BOX_VALUE) {
        Expr::EProj(base, field, _) => {
            assert_eq!(field, "value");
            match *base {
                Expr::EApp(head, arg, _) => {
                    assert!(var(&head, "keep") && var(&arg, "box"), "base must be A(keep, box)");
                }
                other => panic!("projection base must be an application, got {other:?}"),
            }
        }
        other => panic!("`keep box.value` must be Proj(A(keep, box), value), got {other:?}"),
    }
}

/// AC-PROJ-NESTS, assertion 2. The explicitly grouped form produces the SAME
/// tree, which is what makes assertion 1 a statement about nesting rather than
/// about parentheses.
#[test]
fn ac_proj_nests_grouped_head_agrees_with_ungrouped() {
    let ungrouped = format!("{:?}", body_of(KEEP_BOX_VALUE));
    let grouped = format!("{:?}", body_of(GROUPED_HEAD));
    // Compared as whole trees. Spans differ (the grouped source has parens), so
    // compare the shape with spans elided rather than the raw Debug text.
    assert_eq!(
        strip_spans(&ungrouped),
        strip_spans(&grouped),
        "`keep box.value` and `(keep box).value` must be the same tree"
    );
}

/// AC-PROJ-NESTS, assertion 3. The other grouping keeps the OLD tree. Without
/// this a candidate that collapsed all three forms into one would satisfy the
/// first two while having removed the distinction rather than fixed it.
#[test]
fn ac_proj_nests_grouped_argument_keeps_the_other_tree() {
    match body_of(GROUPED_ARG) {
        Expr::EApp(head, arg, _) => {
            assert!(var(&head, "keep"));
            assert!(
                matches!(*arg, Expr::EProj(..)),
                "`keep (box.value)` must keep the projection INSIDE the argument"
            );
        }
        other => panic!("`keep (box.value)` must be A(keep, Proj(box, value)), got {other:?}"),
    }
    assert_ne!(
        strip_spans(&format!("{:?}", body_of(KEEP_BOX_VALUE))),
        strip_spans(&format!("{:?}", body_of(GROUPED_ARG))),
        "the two groupings must remain DISTINGUISHABLE"
    );
}

/// The head position was already correct before this node and must stay so.
/// This is the case a two-sequential-loops patch regresses from correct to
/// unparseable, so it is asserted rather than assumed.
#[test]
fn head_position_projection_is_unchanged() {
    match body_of(BOX_VALUE_KEEP) {
        Expr::EApp(head, arg, _) => {
            assert!(matches!(*head, Expr::EProj(..)), "head must be Proj(box, value)");
            assert!(var(&arg, "keep"));
        }
        other => panic!("`box.value keep` must be A(Proj(box, value), keep), got {other:?}"),
    }
}

/// The interleaving case. No single-alternation grammar reaches it, and a
/// suite carrying only the pin's own example cannot fail for it.
#[test]
fn interleaved_application_and_projection_follow_source_order() {
    match body_of(INTERLEAVED) {
        Expr::EApp(proj, c, _) => {
            assert!(var(&c, "c"));
            match *proj {
                Expr::EProj(inner, ref field, _) if field == "b" => {
                    assert!(
                        matches!(*inner, Expr::EApp(ref f, ref a, _) if var(f, "f") && var(a, "a")),
                        "`f a.b c` must nest A(f, a) inside the projection"
                    );
                }
                other => panic!("expected Proj(.., b), got {other:?}"),
            }
        }
        other => panic!("`f a.b c` must be A(Proj(A(f, a), b), c), got {other:?}"),
    }
}

/// Positional projection travels with the named form. Dropping the `.1`/`.2`
/// arm when lifting the suffix would leave positional projection on the atom
/// path with nothing failing in the named cases.
#[test]
fn positional_projection_rides_the_same_spine() {
    assert!(
        matches!(body_of("const k : Nat = p.1 q"), Expr::EApp(h, _, _) if matches!(*h, Expr::EPosProj(..))),
        "`p.1 q` must be A(PosProj(p, 1), q)"
    );
    assert!(
        matches!(body_of("const k : Nat = f a.1"), Expr::EPosProj(b, 1, _) if matches!(*b, Expr::EApp(..))),
        "`f a.1` must be PosProj(A(f, a), 1)"
    );
}

/// AC-IF-REJECTS. An ungrouped `if` after an application head rejects AT the
/// `if`, not merely somewhere.
#[test]
fn ac_if_rejects_ungrouped_if_as_an_application_argument() {
    let src = "const k : Nat = keep if c then a else b";
    let at = src.find("if").expect("fixture contains `if`");
    match err_of(src) {
        ElabError::ParseError { span, msg } => {
            assert_eq!(span.start, at, "rejection must be located at the `if`: {msg}");
        }
        other => panic!("expected a parse error, got {other:?}"),
    }
}

/// AC-IF-REJECTS controls. A candidate that makes `if` unparseable everywhere
/// satisfies the criterion above and breaks the language, so both surviving
/// positions are asserted.
#[test]
fn ac_if_rejects_controls_grouped_and_leading_if_still_parse() {
    assert!(
        matches!(body_of("const k : Nat = keep (if c then a else b)"), Expr::EApp(..)),
        "a GROUPED if is still a legal application argument"
    );
    assert!(
        matches!(body_of("const k : Nat = if c then a else b"), Expr::EIf { .. }),
        "a LEADING if is dispatched before the argument loop and is unaffected"
    );
}

/// AC-OLD-UNCHANGED. `old` is the third caller of `parse_atom_expr` and is not
/// an `application_atom`, so `§3`'s pin does not reach it. Whether `old x.f`
/// should be `Proj(Old(x), f)` is a separate `21 §6.4` question; this pins the
/// current binding so that question cannot be answered by accident here.
#[test]
fn ac_old_unchanged_keeps_its_projection_inside() {
    match body_of("const k : Nat = old x.f") {
        Expr::EOld(inner, _) => assert!(
            matches!(*inner, Expr::EProj(..)),
            "`old x.f` must remain Old(Proj(x, f))"
        ),
        other => panic!("expected EOld, got {other:?}"),
    }
}

/// Debug text with `Span { .. }` payloads elided, so two trees can be compared
/// for SHAPE when they necessarily differ in offsets.
fn strip_spans(debug: &str) -> String {
    let mut out = String::with_capacity(debug.len());
    let mut rest = debug;
    while let Some(i) = rest.find("Span { ") {
        out.push_str(&rest[..i]);
        out.push_str("Span");
        match rest[i..].find('}') {
            Some(j) => rest = &rest[i + j + 1..],
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    out
}
