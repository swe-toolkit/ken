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
use ken_kernel::{Decl, GlobalEnv, GlobalId, Term};
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

// ---------------------------------------------------------------- AC-3 ------

/// How a global is realised, in the one dimension the escape hatch would have
/// to move it along.
///
/// `33 §6.1` requires every standard meaning to be an **ordinary top-level
/// binding**, and `39 §6.9`'s completion policy keys on its `GlobalId`. An
/// elaborator builtin has no such binding: it is registered as a kernel
/// `Decl::Primitive` (or postulated as `Decl::Opaque`), which is precisely
/// what has no body to unfold and nothing for the policy to key on.
#[derive(Debug, PartialEq, Eq)]
enum Realisation {
    /// `Decl::Transparent` — a checked definition with a body. This is what
    /// "ordinary top-level binding" means at the kernel.
    OrdinaryTopLevelBinding,
    /// Registered rather than defined: a builtin's shape.
    PrimitiveOrPostulate,
    Inductive,
    Missing,
}

fn realisation_of(env: &GlobalEnv, id: GlobalId) -> Realisation {
    match env.lookup(id) {
        Some(Decl::Transparent { .. }) => Realisation::OrdinaryTopLevelBinding,
        Some(Decl::Primitive { .. }) | Some(Decl::Opaque { .. }) => {
            Realisation::PrimitiveOrPostulate
        }
        Some(Decl::Inductive(_)) => Realisation::Inductive,
        None => Realisation::Missing,
    }
}

/// Does this term mention a kernel projection anywhere?
fn mentions_projection(term: &Term) -> bool {
    match term {
        Term::Proj1(_) | Term::Proj2(_) => true,
        Term::Pi(domain, codomain) | Term::Sigma(domain, codomain) => {
            mentions_projection(domain) || mentions_projection(codomain)
        }
        Term::Lam(domain, body) => mentions_projection(domain) || mentions_projection(body),
        Term::App(f, a) => mentions_projection(f) || mentions_projection(a),
        Term::Ascript(value, ty) => mentions_projection(value) || mentions_projection(ty),
        _ => false,
    }
}

/// **AC-3 — THE BUILTIN ESCAPE HATCH IS CLOSED, and this is the criterion the
/// Architect said they cared about most.**
///
/// The hatch is to special-case the standard meaning in the elaborator instead
/// of supplying the missing surface form. That would make every dependent test
/// pass — **the observable behaviour is identical** — while falsifying the
/// ordinary-top-level-binding premise `39 §6.9`'s completion policy rests on
/// (`33 §6.1`, `SPEC-STANDARD-INFIX-BINDING §2f`). No test on the dependent
/// node would catch it.
///
/// **So this is a measurement on HOW the bindings resolve, in the positive
/// direction, not a grep for a builtin-shaped name coming back empty.** A
/// criterion satisfied by *"I did not do that"* is satisfied by an implementer
/// who did it without noticing.
///
/// Three arms, and the third is what makes the first two mean anything:
///
/// 1. each consumer binding realises as an **ordinary top-level binding**;
/// 2. **the named positive control** — an ordinary `fn` with no projection
///    anywhere in it — realises identically, so the predicate reports
///    "ordinary" for something already known to be ordinary;
/// 3. **the discriminator** — every one of `trusted_base()`'s members realises
///    as `PrimitiveOrPostulate`. Without this the predicate could be constant
///    and arms 1 and 2 would pass in a world where the hatch was taken.
///
/// The trusted-base arm keys on the **`Decl` shape**, never on trusted-base
/// membership, so it is a claim about the environment rather than a
/// restatement of how the set was built.
#[test]
fn ac3_the_consumer_bindings_resolve_as_ordinary_top_level_bindings_not_builtins() {
    let mut env = env_with(UPPER_FIELD_CLASS);

    // --- arm 3 first: the predicate can tell a builtin from a binding. -------
    let trusted = env.env.trusted_base();
    assert!(
        !trusted.is_empty(),
        "the discriminator is vacuous if the prelude declares no primitives"
    );
    let builtin_shaped = trusted.len();
    for id in &trusted {
        assert_eq!(
            realisation_of(&env.env, *id),
            Realisation::PrimitiveOrPostulate,
            "trusted-base member {id:?} must NOT look like an ordinary binding, \
             or this predicate cannot detect the escape hatch"
        );
    }

    // --- arm 2: the named positive control. ---------------------------------
    let control = elab(&mut env, "fn ordinary_control (x : Bool) : Bool = x")
        .expect("the control binding must elaborate");
    assert_eq!(
        realisation_of(&env.env, control),
        Realisation::OrdinaryTopLevelBinding,
        "the predicate must report an ordinary binding as ordinary"
    );
    assert!(
        !trusted.contains(&control),
        "an ordinary binding is not in the trust base"
    );
    let control_ty = match env.env.lookup(control) {
        Some(Decl::Transparent { ty, .. }) => ty.clone(),
        other => panic!("control must be transparent: {other:?}"),
    };
    assert!(
        !mentions_projection(&control_ty),
        "the control must contain no projection, or it cannot contrast"
    );

    // --- arm 1: every consumer binding, each on the same footing. -----------
    let bindings = [
        (
            "membership_member_at",
            "fn membership_member_at (c : Type) (d : Sack c) (q : d.Query) (x : c) : Bool \
             = d.member q x",
        ),
        (
            "member_holds",
            "fn member_holds (c : Type) (d : Sack c) (q : d.Query) (x : c) : Omega \
             = Equal Bool (d.member q x) True",
        ),
        (
            "same_members",
            "fn same_members (c : Type) (d : Sack c) (x : c) (y : c) : Omega \
             = (q : d.Query) -> Equal Bool (d.member q x) (d.member q y)",
        ),
    ];
    for (name, source) in bindings {
        let id =
            elab(&mut env, source).unwrap_or_else(|e| panic!("`{name}` must elaborate: {e:?}"));
        assert_eq!(
            realisation_of(&env.env, id),
            Realisation::OrdinaryTopLevelBinding,
            "`{name}` must be an ordinary top-level binding, the same shape as the control"
        );
        assert!(
            !env.env.trusted_base().contains(&id),
            "`{name}` must not have entered the trust base"
        );
    }

    // The trust base did not grow while three projection-typed bindings were
    // added: no builtin was registered along the way.
    assert_eq!(
        env.env.trusted_base().len(),
        builtin_shaped,
        "elaborating the consumer bindings must register nothing new as a builtin"
    );

    // And the resolution really went THROUGH a projection: the first binding's
    // elaborated type mentions a kernel projection, which the control's does
    // not. A special-cased builtin would have no reason to carry one.
    let member_at = elab(
        &mut env,
        "fn projection_reached (c : Type) (d : Sack c) (q : d.Query) : Bool = True",
    )
    .expect("must elaborate");
    let ty = match env.env.lookup(member_at) {
        Some(Decl::Transparent { ty, .. }) => ty.clone(),
        other => panic!("must be transparent: {other:?}"),
    };
    assert!(
        mentions_projection(&ty),
        "the binding's own type must carry the kernel projection the surface form resolved to"
    );
}

// ------------------------------------------- the chained fold path ----------

/// `d.inner.Query` — a second segment, which folds the first back into an
/// `Expr::EProj` before the outer `Type::TProj` is built. That fold is a
/// distinct branch in the production and nothing else here reaches it.
///
/// It is not speculative generality: a class field holding another dictionary
/// is an ordinary shape (`Applicative`'s `functor : Functor f` is one), so the
/// chain is reachable from the class vocabulary that already exists.
#[test]
fn a_chained_projection_folds_left_and_resolves_through_both_fields() {
    let mut env = ElabEnv::new().expect("prelude must elaborate");
    env.elaborate_decl("class Inner A { Query : Type ; k : A }")
        .expect("inner class must elaborate");
    env.elaborate_decl("class Outer A { inner : Inner A ; j : A }")
        .expect("outer class must elaborate");
    assert!(
        elab(
            &mut env,
            "fn chained (c : Type) (d : Outer c) (q : d.inner.Query) : Bool = True"
        )
        .is_ok(),
        "a two-segment projection must resolve through both fields"
    );
}

/// The chain's OUTER field is still looked up, and still refused where it is
/// written — so the fold does not lose the span or silently accept.
#[test]
fn a_chained_projection_with_an_absent_outer_field_is_refused_at_the_chain() {
    let mut env = ElabEnv::new().expect("prelude must elaborate");
    env.elaborate_decl("class Inner A { Query : Type ; k : A }")
        .expect("inner class must elaborate");
    env.elaborate_decl("class Outer A { inner : Inner A ; j : A }")
        .expect("outer class must elaborate");
    let source = "fn chained_bad (c : Type) (d : Outer c) (q : d.inner.Nope) : Bool = True";
    match elab(&mut env, source) {
        Err(ElabError::UnresolvedCon { name, span }) => {
            assert_eq!(name, "Nope");
            let (start, end) = span_of(source, "d.inner.Nope");
            assert_eq!(
                (span.start, span.end),
                (start, end),
                "the span must cover the whole chain, head through final field"
            );
        }
        other => panic!("an absent field on a chained projection must be refused: {other:?}"),
    }
}

// -------------------------------------- the declined third rejection --------

/// `(q : d.member)` — a field whose VALUE is not a type, written where a type
/// is expected. `elab_type`'s projection arm deliberately mints no rejection of
/// its own for this and leaves it to the kernel, on the ground that `AC-2`
/// names two rejections and a third arm with no reaching fixture would be
/// unexercised.
///
/// **That reasoning covers the arm I did not add and says nothing about the
/// fallback I chose instead — so the fallback is measured here rather than
/// asserted.** `d.member` is reachable from ordinary syntax, by the same
/// standard that earned the `data` declaration its own located diagnostic.
///
/// The measurement, and the part worth knowing: the kernel **does** refuse, as
/// a sort mismatch — but its span is the **enclosing declaration's**, not the
/// projection's, because a kernel term carries no surface coordinate. That is a
/// worse diagnostic than `AC-2`'s two, and it is recorded rather than left for
/// someone to discover. No soundness exposure: the refusal is real either way.
#[test]
fn a_non_type_field_in_type_position_is_refused_by_the_kernel_not_located() {
    let mut env = env_with(UPPER_FIELD_CLASS);
    let source = "fn non_type_field (c : Type) (d : Sack c) (q : d.member) : Bool = True";
    match elab(&mut env, source) {
        Err(ElabError::KernelRejected { span, .. }) => {
            let (projection_start, projection_end) = span_of(source, "d.member");
            assert_ne!(
                (span.start, span.end),
                (projection_start, projection_end),
                "if this ever becomes located, the comment above is stale -- update it"
            );
            assert_eq!(
                (span.start, span.end),
                (0, source.len()),
                "the kernel's refusal carries the declaration's span, not the projection's"
            );
        }
        other => panic!(
            "a field whose value is not a type must still be refused, by the kernel \
             if not by the elaborator: {other:?}"
        ),
    }
}

/// The control for the case above: the SAME class, the SAME shape, projecting
/// the field that IS a type. Without it, "the kernel refuses `d.member`" is
/// consistent with the kernel refusing every projection in type position.
#[test]
fn a_non_type_field_control_the_type_valued_field_on_the_same_class_accepts() {
    let mut env = env_with(UPPER_FIELD_CLASS);
    assert!(
        elab(
            &mut env,
            "fn type_field (c : Type) (d : Sack c) (q : d.Query) : Bool = True"
        )
        .is_ok(),
        "the type-valued field on the same class must still be accepted"
    );
}
