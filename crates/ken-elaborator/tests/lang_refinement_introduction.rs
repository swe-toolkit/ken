use ken_elaborator::{v2_extract, ElabEnv, ProvKind};
use ken_kernel::{Level, Term};

fn arbitrary_predicate(env: &mut ElabEnv) {
    let int = Term::const_(env.globals["Int"], vec![]);
    env.declare_postulate_raw("P", Term::pi(int, Term::omega(Level::Zero)))
        .expect("predicate postulate for this fixture");
}

#[test]
fn named_introduction_emits_only_on_new_values() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }")
        .expect("alias");
    let introduction = env
        .elaborate_decl_v1("const five : Pos = 5")
        .expect("intro");
    assert_eq!(introduction.obligations.len(), 1);
    assert!(env.is_open_hole(introduction.obligations[0].hole_id));
    let extracted = v2_extract(&introduction);
    assert_eq!(extracted.obligations.len(), 1);
    assert!(matches!(
        extracted.obligations[0].provenance.kind,
        ProvKind::RefinementIntroduction
    ));
    let same = env
        .elaborate_decl_v1("fn same (p : Pos) : Pos = p")
        .expect("reuse");
    assert!(
        same.obligations.is_empty(),
        "a value retaining the same alias is not new"
    );
    let forget = env
        .elaborate_decl_v1("fn forget (p : Pos) : Int = p")
        .expect("forget");
    assert!(
        forget.obligations.is_empty(),
        "forgetting to the carrier is free"
    );
    let open = env
        .elaborate_decl_v1("fn to_pos (n : Int) : Pos = n")
        .expect("open intro");
    assert_eq!(open.obligations.len(), 1);
    assert!(env.is_open_hole(open.obligations[0].hole_id));
}

#[test]
fn char_scalar_boundary_is_an_obligation_not_a_rejection() {
    let mut env = ElabEnv::new().expect("prelude");
    let valid = env
        .elaborate_decl_v1("const valid : Char = 55295")
        .expect("valid scalar");
    assert_eq!(valid.obligations.len(), 1);
    assert!(
        !env.is_open_hole(valid.obligations[0].hole_id),
        "valid goal: {:?}; whnf: {:?}",
        valid.obligations[0].goal_closed,
        ken_kernel::whnf(
            &env.env,
            &ken_kernel::Context::new(),
            &valid.obligations[0].goal_closed
        )
    );
    let invalid = env
        .elaborate_decl_v1("const invalid : Char = 55296")
        .expect("open invalid scalar");
    assert_eq!(invalid.obligations.len(), 1);
    assert!(env.is_open_hole(invalid.obligations[0].hole_id));
}

#[test]
fn matched_scalar_introduction_carries_and_uses_its_branch_equation() {
    let mut env = ElabEnv::new().expect("prelude");
    let branch = env.elaborate_decl_v1(
        "fn checked_scalar (n : Int) : Option Char = match (inRangeBool n) { True |-> Some Char n ; False |-> None Char }"
    ).expect("checked branch");
    assert_eq!(
        branch.obligations.len(),
        1,
        "the Some branch introduces Char once"
    );
    let goal = &branch.obligations[0].goal_closed;
    let Term::Pi(_, under_n) = goal else {
        panic!("missing n binder: {goal:?}")
    };
    let Term::Pi(equation, _) = under_n.as_ref() else {
        panic!("missing case equation: {goal:?}")
    };
    assert!(
        matches!(equation.as_ref(), Term::Eq(..)),
        "the branch binder must be equality: {goal:?}"
    );
    assert!(
        !env.is_open_hole(branch.obligations[0].hole_id),
        "branch equation must close the scalar goal: {goal:?}"
    );
    let wrong = env.elaborate_decl_v1(
        "fn wrong_scalar (n : Int) : Option Char = match (inRangeBool n) { True |-> None Char ; False |-> Some Char n }",
    ).expect("the false arm still has a visible scalar obligation");
    assert_eq!(wrong.obligations.len(), 1);
    assert!(
        env.is_open_hole(wrong.obligations[0].hole_id),
        "a False-branch path equation does not prove isScalar n"
    );
}

#[test]
fn literal_annotations_are_site_local_and_parameter_predicates_reach_calls() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    let literal = env
        .elaborate_decl_v1("const literal : Int = (4 : { x : Int | P x })")
        .expect("literal");
    assert_eq!(literal.obligations.len(), 1);
    let extracted = v2_extract(&literal);
    assert_eq!(extracted.obligations.len(), 1);
    assert!(matches!(
        extracted.obligations[0].provenance.kind,
        ProvKind::RefinementIntroduction
    ));
    let carrier = env
        .elaborate_decl_v1("const carrier : Int = 4")
        .expect("carrier");
    assert!(
        carrier.obligations.is_empty(),
        "a literal must not taint its carrier globally"
    );
    let parameter = env
        .elaborate_decl_v1("fn take (x : { y : Int | P y }) : Int = x")
        .expect("callee");
    assert!(
        parameter.obligations.is_empty(),
        "a parameter is not an introduced value"
    );
    let caller = env
        .elaborate_decl_v1("const caller : Int = take 4")
        .expect("caller");
    assert_eq!(caller.obligations.len(), 1);
    assert!(env.is_open_hole(caller.obligations[0].hole_id));
}

#[test]
fn introductions_cover_literal_and_named_returns_lets_and_match_arms() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }")
        .expect("named refinement");
    for (source, expected) in [
        ("const literal_result : { x : Int | P x } = 5", 1),
        ("fn named_result (n : Int) : Pos = n", 1),
        ("const via_let : Int = let x : Pos = 5 in x", 1),
        ("const literal_let : Int = let x : { x : Int | P x } = 5 in x", 1),
        ("const named_match : Pos = match True { True |-> 5 ; False |-> 6 }", 2),
        ("const let_match : Int = let y : Pos = match True { True |-> 5 ; False |-> 6 } in y", 2),
        // Current literal site seed: §22 §2.2 per-leaf realization is a
        // successor, not a conformance claim of this row.
        ("const literal_let_match : Int = let y : { x : Int | P x } = match True { True |-> 5 ; False |-> 6 } in y", 1),
        ("const plain_carrier : Int = 5", 0),
    ] {
        let result = env.elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(result.obligations.len(), expected, "{source}");
    }
}

#[test]
fn named_return_over_a_match_is_realized_per_leaf() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(
        "data Glyph : Type where { MkGlyph : (code : Int) -> (glyph : Char) -> Glyph }",
    )
    .expect("glyph record");
    let reuse = env
        .elaborate_decl_v1(
            "fn glyph_of (g : Glyph) : Char = match g { MkGlyph code glyph |-> glyph }",
        )
        .expect("field re-use");
    assert!(
        reuse.obligations.is_empty(),
        "an arm returning a Char-typed field is a re-use"
    );
    let leaves = env
        .elaborate_decl_v1("fn pick (b : Bool) : Char = match b { True |-> 48 ; False |-> 57 }")
        .expect("closed leaves");
    assert_eq!(
        leaves.obligations.len(),
        2,
        "one obligation per leaf, none over the whole match"
    );
    assert!(
        leaves
            .obligations
            .iter()
            .all(|o| !env.is_open_hole(o.hole_id)),
        "each closed leaf discharges"
    );
    let open = env
        .elaborate_decl_v1(
            "fn widen (b : Bool) (n : Int) : Char = match b { True |-> 48 ; False |-> n }",
        )
        .expect("open leaf");
    assert_eq!(open.obligations.len(), 2);
    assert_eq!(
        open.obligations
            .iter()
            .filter(|o| env.is_open_hole(o.hole_id))
            .count(),
        1,
        "only the unconstrained leaf stays open"
    );
}

#[test]
fn constructor_fields_and_direct_refined_arguments_emit_once() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }")
        .expect("named refinement");
    env.elaborate_decl("data BoxPos = MkBoxPos Pos")
        .expect("constructor");
    env.elaborate_decl("data BoxLit = MkBoxLit { value : { x : Int | P x } }")
        .expect("literal refined constructor field");
    env.elaborate_decl("data BoxPlain = MkBoxPlain Int")
        .expect("plain constructor");
    for source in [
        "const named_field : BoxPos = MkBoxPos 5",
        "const literal_field : BoxLit = MkBoxLit 5",
    ] {
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"));
        assert_eq!(result.obligations.len(), 1, "{source}");
    }
    let plain = env
        .elaborate_decl_v1("const plain_field : BoxPlain = MkBoxPlain 5")
        .expect("plain carrier field");
    assert!(plain.obligations.is_empty());
    let param = env
        .elaborate_decl_v1(
            "fn divide (n : Int) (d : { z : Int | Not (Equal Int z 0) }) : Int = n / d",
        )
        .expect("refined parameter");
    assert_eq!(
        param.obligations.len(),
        1,
        "the callee retains its PartialPrim hole"
    );
    let caller = env
        .elaborate_decl_v1("fn caller (u : Int) : Int = divide 1 0")
        .expect("direct caller");
    assert_eq!(
        caller.obligations.len(),
        1,
        "caller owes one refinement, not a callee assumption"
    );
    assert!(env.is_open_hole(caller.obligations[0].hole_id));
}

#[test]
fn named_record_literal_field_introduces_its_site_predicate() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("record RefRecord { value : { x : Int | P x }, plain : Int }")
        .expect("record with one literal refined field");
    let row = env
        .elaborate_decl_v1("const row : RefRecord = { value = 5, plain = 6 }")
        .expect("record introduction");
    assert_eq!(row.obligations.len(), 1);
    assert!(env.is_open_hole(row.obligations[0].hole_id));
    env.elaborate_decl("record DepRecord { left : Int, right : { y : Int | Equal Int y left } }")
        .expect("dependent record field");
    let equal = env
        .elaborate_decl_v1("const equal : DepRecord = { left = 5, right = 5 }")
        .expect("equal dependent field");
    assert_eq!(equal.obligations.len(), 1);
    assert!(!env.is_open_hole(equal.obligations[0].hole_id));
    let unequal = env
        .elaborate_decl_v1("const unequal : DepRecord = { left = 5, right = 6 }")
        .expect("unequal dependent field");
    assert_eq!(unequal.obligations.len(), 1);
    assert!(env.is_open_hole(unequal.obligations[0].hole_id));
}

#[test]
fn dependent_literal_constructor_field_uses_its_checked_prefix_arguments() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(
        "data DepBox : Type where { MkDepBox : (x : Int) -> (r : { y : Int | Equal Int y x }) -> DepBox }",
    ).expect("dependent literal field");
    let good = env
        .elaborate_decl_v1("const good : DepBox = MkDepBox 5 5")
        .expect("equal second field");
    assert_eq!(good.obligations.len(), 1);
    assert!(!env.is_open_hole(good.obligations[0].hole_id));
    let bad = env
        .elaborate_decl_v1("const bad : DepBox = MkDepBox 5 6")
        .expect("mismatched second field remains visible");
    assert_eq!(bad.obligations.len(), 1);
    assert!(env.is_open_hole(bad.obligations[0].hole_id));
}

#[test]
fn distinct_refinement_aliases_do_not_share_an_int_carrier_key() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }")
        .expect("positive refinement");
    env.elaborate_decl("def Neg = { x : Int | Not (P x) }")
        .expect("negative refinement");
    let negative = env
        .elaborate_decl_v1("const negative : Neg = 5")
        .expect("Neg introduction");
    assert_eq!(negative.obligations.len(), 1);
    let conversion = env
        .elaborate_decl_v1("const converted : Pos = negative")
        .expect("a different alias still requires introduction");
    assert_eq!(conversion.obligations.len(), 1);
    assert!(env.is_open_hole(conversion.obligations[0].hole_id));
}

#[test]
fn mutually_recursive_source_introductions_keep_alias_identity() {
    let mut env = ElabEnv::new().expect("prelude");
    arbitrary_predicate(&mut env);
    env.elaborate_decl("def Pos = { x : Int | P x }")
        .expect("alias");
    let results = env
        .elaborate_file_v1(
            "fn left (n : Nat) : Pos = match n { Zero |-> 5 ; Suc m |-> right m }\n\
         fn right (n : Nat) : Pos = match n { Zero |-> 6 ; Suc m |-> left m }",
        )
        .expect("mutual source group");
    assert_eq!(results.len(), 2);
    for result in &results {
        assert_eq!(
            result.obligations.len(),
            1,
            "each base case introduces Pos once"
        );
        assert!(env.is_open_hole(result.obligations[0].hole_id));
    }
}

#[test]
fn ill_typed_constructor_refinement_rejects_without_partial_registration() {
    let mut env = ElabEnv::new().expect("prelude");
    let original_globals = env.globals.clone();
    let original_env = env.env.clone();
    assert!(env
        .elaborate_decl("data BadRefinement = MkBadRefinement { value : { x : Int | 3 } }",)
        .is_err());
    assert!(
        env.globals == original_globals,
        "rejected data retained a source name"
    );
    assert!(
        env.env == original_env,
        "rejected data retained a kernel declaration"
    );
    env.elaborate_decl("data BadRefinement = MkBadRefinement Int")
        .expect("a failed predicate must not hold either name");
    let original_globals = env.globals.clone();
    let original_env = env.env.clone();
    assert!(env
        .elaborate_decl("record BadRecordRefinement { value : { x : Int | 3 } }",)
        .is_err());
    assert!(
        env.globals == original_globals,
        "rejected record retained a source name"
    );
    assert!(
        env.env == original_env,
        "rejected record retained a kernel declaration"
    );
    env.elaborate_decl("record BadRecordRefinement { value : Int }")
        .expect("a failed record predicate must not hold its name");
}

#[test]
fn guide_posint_closed_introduction_and_reuse() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def PosInt = { n : Int | Equal Bool (leq_int 0 n) True }")
        .expect("guide alias");
    let five = env
        .elaborate_decl_v1("const five : PosInt = 5")
        .expect("guide introduction");
    assert_eq!(five.obligations.len(), 1);
    assert!(!env.is_open_hole(five.obligations[0].hole_id));
    let add = env
        .elaborate_decl_v1("fn add_to_pos_int (n : Int) (p : PosInt) : Int = add_int n p")
        .expect("guide consumer");
    assert!(add.obligations.is_empty());
    let ten = env
        .elaborate_decl_v1("const ten : Int = add_to_pos_int five five")
        .expect("guide re-use and forgetful conversion");
    assert!(ten.obligations.is_empty());
}
