//! `LANG-TYPE-PROJECTION-SURFACE-FORM` — a projection form in the surface TYPE
//! grammar, so a parameter may be typed by a projection from an EARLIER
//! parameter in the same telescope (`33 §6.3`, `../50-stdlib/58b §1`).
//!
//! Every fixture here is a BEHAVIOUR on the elaborator: a source string in,
//! an accept or a specific rejection out. Nothing asserts a fact about the
//! repository's own text.
//!
//! **The class fixture is `58b §1`'s shape, not a convenient simplification.**
//! `Membership` declares `Query : Type` — a field whose type IS a universe,
//! with `member` naming it afterwards — and the field name is UPPERCASE. Both
//! facts are load-bearing: the case is what the expression-position projection
//! loop does not admit, and it is why this node needed a named form at all.

use ken_elaborator::{error::ElabError, ElabEnv};
use ken_kernel::{GlobalEnv, GlobalId};
use std::collections::BTreeSet;

/// `58b §1`'s two-field shape with the specified UPPERCASE query field.
const UPPER_FIELD_CLASS: &str = "class Sack A { Query : Type ; member : Query -> A -> Bool }";

/// The same shape with a lowercase field, so every claim below about the
/// uppercase case has a same-shape control that differs ONLY in the case.
const LOWER_FIELD_CLASS: &str = "class Bag A { query : Type ; member : query -> A -> Bool }";

fn env_with(class: &str) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude must elaborate");
    env.elaborate_decl(class)
        .expect("the class fixture must elaborate; every case below rests on it");
    env
}

fn elab(env: &mut ElabEnv, source: &str) -> Result<GlobalId, ElabError> {
    env.elaborate_decl(source)
}

/// The byte range of `needle` inside `source`, so a span assertion names the
/// fixture's own coordinates rather than a number copied out of a passing run.
fn span_of(source: &str, needle: &str) -> (usize, usize) {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture must contain `{needle}`"));
    (start, start + needle.len())
}

fn trusted_base_set(env: &GlobalEnv) -> BTreeSet<GlobalId> {
    env.trusted_base().into_iter().collect()
}

// ---------------------------------------------------------------- AC-1 ------

/// **AC-1.** All three consumer bindings from `SPEC-MEMBERSHIP-CLASS-CONTRACT`
/// (`58b §2`, `§4`, and `33 §6.3`) become writable.
///
/// **These three bindings are TWO parse positions, not three, and the test
/// says so rather than letting "three fixtures" read as three routes.**
/// `membership_member_at` and `member_holds` are both `ast::Binder::ty` inside
/// a `params: Vec<Binder>` telescope — they differ in the declaration keyword
/// and the result type, which is the ENCLOSURE. `same_members` is the
/// genuinely distinct position: an `Expr::EPi` domain inside a body. All three
/// are exercised because all three are the real consumer bindings; the note is
/// so nobody reads the count as coverage of three routes.
#[test]
fn ac1_the_three_consumer_bindings_are_writable() {
    // `membership_member_at (c) (d) (q : d.Query) (x) : Bool` — `33 §6.3`.
    // Parameter telescope, `Bool` result.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let member_at = "fn membership_member_at (c : Type) (d : Sack c) (q : d.Query) (x : c) : Bool \
                     = d.member q x";
    assert!(
        elab(&mut env, member_at).is_ok(),
        "the telescope binding must elaborate: {:?}",
        elab(&mut env_with(UPPER_FIELD_CLASS), member_at)
    );

    // `member_holds (c) (d) (q : d.Query) (x) : Ω` — `58b §2`.
    // Parameter telescope, `Omega` result. It is an ordinary definition, not a
    // `prop` declaration: `58b §2` gives it a defining equation (`:=`), and a
    // record field is a thing an instance SUPPLIES while this is defined once.
    // `IsTrue b := Equal Bool b True` (`51 §2`) is inlined so the fixture rests
    // on the prelude rather than on a catalog binding.
    // Same POSITION as the case above; a different result sort.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let member_holds = "fn member_holds (c : Type) (d : Sack c) (q : d.Query) (x : c) : Omega \
                        = Equal Bool (d.member q x) True";
    assert!(
        elab(&mut env, member_holds).is_ok(),
        "the Omega-result telescope binding must elaborate: {:?}",
        elab(&mut env_with(UPPER_FIELD_CLASS), member_holds)
    );

    // A `prop` DECLARATION reaches the projection through a third elaboration
    // path (`elaborate_prop_decl`), distinct from the `fn`/`const` path both
    // cases above take. This is a declaration-path axis, NOT a fourth parse
    // position — the annotation is still `ast::Binder::ty`.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let prop_shape = "prop MemberHolds (c : Type) (d : Sack c) (q : d.Query) (x : c) : Omega";
    assert!(
        elab(&mut env, prop_shape).is_ok(),
        "the prop declaration path must reach the projection too: {:?}",
        elab(&mut env_with(UPPER_FIELD_CLASS), prop_shape)
    );

    // `same_members c d x y := (q : d.Query) → Equal Bool …` — `58b §4`.
    // THE DISTINCT POSITION: an `Expr::EPi` domain in the body, not a
    // declaration parameter.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let same_members = "fn same_members (c : Type) (d : Sack c) (x : c) (y : c) : Omega \
                        = (q : d.Query) -> Equal Bool (d.member q x) (d.member q y)";
    assert!(
        elab(&mut env, same_members).is_ok(),
        "the Pi-domain-in-a-body binding must elaborate: {:?}",
        elab(&mut env_with(UPPER_FIELD_CLASS), same_members)
    );
}

/// A lowercase field name reaches the same three positions. This is the
/// control for the case axis: it differs from the fixtures above ONLY in the
/// field's case, so a regression that admitted one case and not the other
/// reddens exactly one of the two tests.
#[test]
fn ac1_a_lowercase_field_reaches_the_same_positions() {
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(elab(
        &mut env,
        "fn lower_at (c : Type) (d : Bag c) (q : d.query) (x : c) : Bool = d.member q x"
    )
    .is_ok());

    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(elab(
        &mut env,
        "prop LowerHolds (c : Type) (d : Bag c) (q : d.query) (x : c) : Omega"
    )
    .is_ok());
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(elab(
        &mut env,
        "fn lower_holds (c : Type) (d : Bag c) (q : d.query) (x : c) : Omega \
         = Equal Bool (d.member q x) True"
    )
    .is_ok());

    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(elab(
        &mut env,
        "fn lower_same (c : Type) (d : Bag c) (x : c) (y : c) : Omega \
         = (q : d.query) -> Equal Bool (d.member q x) (d.member q y)"
    )
    .is_ok());
}

// ---------------------------------------------------------------- AC-2 ------

/// **AC-2.** Both rejections fire, **and the span points at the PROJECTION**.
///
/// The span half is the part that can silently fail: a rejection carrying the
/// enclosing declaration's span satisfies "it rejects" and fails the
/// deliverable, and the two are indistinguishable from a bare `is_err`. Each
/// expected range is computed from the fixture's own text, so it is not a
/// number copied out of a passing run.
#[test]
fn ac2_both_rejections_fire_with_a_span_on_the_projection() {
    // 1. the projected object is not a record.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let not_a_record = "fn reject_not_record (b : Bool) (q : b.Query) : Bool = True";
    let (start, end) = span_of(not_a_record, "b.Query");
    match elab(&mut env, not_a_record) {
        Err(ElabError::TypeMismatch { span, reason }) => {
            assert!(
                reason.contains("not a named-field owner"),
                "must reject for the owner reason, not an unrelated one: {reason}"
            );
            assert_eq!(
                (span.start, span.end),
                (start, end),
                "the span must cover `b.Query`, not the declaration"
            );
        }
        other => panic!("a non-record base must be refused as a TypeMismatch: {other:?}"),
    }

    // 2. the field does not exist on the class.
    let mut env = env_with(UPPER_FIELD_CLASS);
    let absent = "fn reject_absent (c : Type) (d : Sack c) (q : d.Nope) : Bool = True";
    let (start, end) = span_of(absent, "d.Nope");
    match elab(&mut env, absent) {
        Err(ElabError::UnresolvedCon { name, span }) => {
            assert_eq!(name, "Nope", "the diagnostic must name the missing field");
            assert_eq!(
                (span.start, span.end),
                (start, end),
                "the span must cover `d.Nope`, not the declaration"
            );
        }
        other => panic!("an absent field must be refused as UnresolvedCon: {other:?}"),
    }
}

/// The two rejections above are only meaningful if the SAME fixture shape
/// accepts when the projection is well formed — otherwise both could be
/// refused by something upstream that never reaches the projection at all.
#[test]
fn ac2_positive_control_the_same_shape_accepts_a_valid_projection() {
    let mut env = env_with(UPPER_FIELD_CLASS);
    assert!(elab(
        &mut env,
        "fn accept_valid (c : Type) (d : Sack c) (q : d.Query) : Bool = True"
    )
    .is_ok());
}

// -------------------------------------------- the category divergence -------

/// **Neither grammar is a subset of the other after this node**, and every
/// cell of that table is pinned here because no containment rule summarises
/// it. A later "harmonization" that deletes a capability in one direction
/// while adding one in the other passes any weaker net.
///
/// ```text
///                  expression position   type position
///   d.query          admitted              admitted
///   d.Query          REJECTED              admitted
///   d.1 / d.2        admitted              REJECTED
/// ```
///
/// **The expression side's uppercase rejection is an UNBUILT HALF, not a
/// reserved spelling.** `parse_atom_expr_base`'s `Ident` arm returns
/// `Expr::EVar` and never calls `parse_dotted` — which is reachable only from
/// a `ConId` start — so in expression position `d.Query` is claimed by no
/// production at all. It is not ambiguous and not deferred to a competing
/// reading; no consumer on this node reached it. **Close it when one does**,
/// at which point this test's `upper`/expression row is what has to change.
#[test]
fn the_two_projection_grammars_are_not_subsets_of_each_other() {
    // lowercase: admitted in BOTH.
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(
        elab(
            &mut env,
            "fn e_lower (c : Type) (d : Bag c) : Type = d.query"
        )
        .is_ok(),
        "lowercase, expression position"
    );
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(
        elab(
            &mut env,
            "fn t_lower (c : Type) (d : Bag c) (q : d.query) : Bool = True"
        )
        .is_ok(),
        "lowercase, type position"
    );

    // uppercase: TYPE position only.
    let mut env = env_with(UPPER_FIELD_CLASS);
    assert!(
        elab(
            &mut env,
            "fn t_upper (c : Type) (d : Sack c) (q : d.Query) : Bool = True"
        )
        .is_ok(),
        "uppercase, type position"
    );
    let mut env = env_with(UPPER_FIELD_CLASS);
    assert!(
        matches!(
            elab(
                &mut env,
                "fn e_upper (c : Type) (d : Sack c) : Type = d.Query"
            ),
            Err(ElabError::ParseError { .. })
        ),
        "uppercase, expression position: unbuilt half, still a parse error"
    );

    // positional: EXPRESSION position only.
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(
        elab(&mut env, "fn e_pos (c : Type) (d : Bag c) : Type = d.1").is_ok(),
        "positional, expression position"
    );
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(
        elab(
            &mut env,
            "fn t_pos (c : Type) (d : Bag c) (q : d.1) : Bool = True"
        )
        .is_err(),
        "positional, type position"
    );
}

/// `(q : d.1)` reaches the new production the moment it consumes `Token::Dot`,
/// so the arm is authored rather than left to fall through. The generic
/// `expected RParen, found Dot` points at the paren; this points at the form.
#[test]
fn positional_projection_in_type_position_names_the_unsupported_form() {
    let mut env = env_with(LOWER_FIELD_CLASS);
    let source = "fn t_pos (c : Type) (d : Bag c) (q : d.1) : Bool = True";
    let (_, _) = span_of(source, ".1");
    match elab(&mut env, source) {
        Err(ElabError::ParseError { msg, span }) => {
            assert!(
                msg.contains("positional projection") && msg.contains("type position"),
                "the message must name the unsupported form, not the paren: {msg}"
            );
            let (start, end) = span_of(source, ".1");
            assert_eq!(
                (span.start, span.end),
                (start, end),
                "the span must cover `.1`"
            );
        }
        other => panic!("positional projection in type position must be a ParseError: {other:?}"),
    }
}

// ------------------------------------------------------- non-regression -----

/// The production attaches to a LOWERCASE head, and a `ConId` head still
/// reaches `parse_dotted` as a qualified name — so the two are decided at
/// different STAGES, which is the whole no-ambiguity argument. A regression
/// that routed `Bag.Query` into the projection production would turn this
/// resolution failure into a parse failure.
#[test]
fn a_conid_head_still_parses_as_a_qualified_name_not_a_projection() {
    let mut env = env_with(LOWER_FIELD_CLASS);
    match elab(&mut env, "fn qualified (q : Bag.Query) : Bool = True") {
        Err(ElabError::UnresolvedCon { name, .. }) => assert_eq!(
            name, "Bag.Query",
            "a ConId head must still be folded into one dotted name"
        ),
        other => panic!("`Bag.Query` must fail at RESOLUTION, not in the parser: {other:?}"),
    }
}

/// A lowercase head with no dot is still an ordinary type variable — the arm
/// returns before building a projection when the dot is absent or is not
/// followed by a field token.
#[test]
fn a_lowercase_head_without_a_dot_is_still_a_type_variable() {
    let mut env = env_with(LOWER_FIELD_CLASS);
    assert!(elab(&mut env, "fn plain (a : Type) (x : a) : Type = a").is_ok());
}

// ---------------------------------------------------------------- AC-4 ------

/// **AC-4.** No kernel change is in scope: elaborating every fixture above
/// leaves the trust base exactly as the prelude left it.
///
/// Asserted as SET EQUALITY against a second, independently constructed
/// environment rather than as a count — a count balances under a swap.
#[test]
fn ac4_the_projection_form_adds_nothing_to_the_trusted_base() {
    let baseline = ElabEnv::new().expect("prelude must elaborate");
    let before = trusted_base_set(&baseline.env);

    let mut env = env_with(UPPER_FIELD_CLASS);
    elab(
        &mut env,
        "fn membership_member_at (c : Type) (d : Sack c) (q : d.Query) (x : c) : Bool \
         = d.member q x",
    )
    .expect("fixture must elaborate");
    elab(
        &mut env,
        "fn member_holds (c : Type) (d : Sack c) (q : d.Query) (x : c) : Omega \
         = Equal Bool (d.member q x) True",
    )
    .expect("fixture must elaborate");
    elab(
        &mut env,
        "prop MemberHolds (c : Type) (d : Sack c) (q : d.Query) (x : c) : Omega",
    )
    .expect("fixture must elaborate");
    elab(
        &mut env,
        "fn same_members (c : Type) (d : Sack c) (x : c) (y : c) : Omega \
         = (q : d.Query) -> Equal Bool (d.member q x) (d.member q y)",
    )
    .expect("fixture must elaborate");

    let after = trusted_base_set(&env.env);
    assert_eq!(
        before, after,
        "a surface-to-kernel resolution gap must not move the trust base"
    );
}

// ------------------------------------------------ a forced third arm --------

/// The `data` elaborator converts an `RType` straight to a kernel `Term` with
/// no `ClassEnv` in hand, so exhaustiveness forced a third rejection there.
/// It is authored rather than left to a `todo!()` or a fabricated index, and
/// it is exercised rather than merely written: an arm with no reaching fixture
/// is the thing a reviewer cannot tell from a guess.
///
/// This is NOT one of `AC-2`'s two rejections — those are `infer_proj`'s, on
/// the ordinary declaration path. This one is a different production refusing
/// a shape ordinary syntax can reach.
#[test]
fn a_projection_in_a_data_declaration_is_refused_where_it_is_written() {
    let mut env = env_with(UPPER_FIELD_CLASS);
    let source = "data D (c : Type) (d : Sack c) (q : d.Query) : Type where { MkD : D c d q }";
    match elab(&mut env, source) {
        Err(ElabError::TypeMismatch { span, reason }) => {
            assert!(
                reason.contains("data declaration") && reason.contains("Query"),
                "the refusal must name the field and the production: {reason}"
            );
            let (start, end) = span_of(source, "d.Query");
            assert_eq!(
                (span.start, span.end),
                (start, end),
                "the span must cover `d.Query`, not the declaration"
            );
        }
        other => panic!("a projection in a data telescope must be refused, located: {other:?}"),
    }
}
