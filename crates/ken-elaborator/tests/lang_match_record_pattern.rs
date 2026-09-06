//! `LANG-MATCH-RECORD-PATTERN`: open named-projection controls for `32 §4`
//! and `34 §3.1`.
//!
//! Promise class: durable invariants. These controls distinguish declaration-
//! keyed negative-record projection from data elimination, and pin openness,
//! dependent field order, label diagnostics, and honest redundancy causes.

use ken_elaborator::{error::ElabError, ArmDeadCause, ElabEnv};
use ken_kernel::{whnf, Context, GlobalId, Term};

fn elaborate(env: &mut ElabEnv, source: &str) -> GlobalId {
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("elaboration failed: {error}"))
}

fn body(env: &ElabEnv, id: GlobalId) -> Term {
    env.env
        .transparent_body(id)
        .expect("test declaration is transparent")
        .1
}

fn constructor(id: GlobalId, arguments: impl IntoIterator<Item = Term>) -> Term {
    arguments.into_iter().fold(
        Term::Constructor {
            id,
            level_args: Vec::new(),
        },
        Term::app,
    )
}

fn count_projection_nodes(term: &Term) -> (usize, usize) {
    let here = match term {
        Term::Proj1(_) => (1, 0),
        Term::Proj2(_) => (0, 1),
        _ => (0, 0),
    };
    term.children().into_iter().fold(here, |counts, child| {
        let child_counts = count_projection_nodes(child);
        (counts.0 + child_counts.0, counts.1 + child_counts.1)
    })
}

fn count_eliminators(term: &Term) -> usize {
    usize::from(matches!(term, Term::Elim { .. }))
        + term
            .children()
            .into_iter()
            .map(count_eliminators)
            .sum::<usize>()
}

fn normalize_pairs(env: &ElabEnv, term: &Term) -> Term {
    match whnf(&env.env, &Context::new(), term) {
        Term::Pair(first, second) => {
            Term::pair(normalize_pairs(env, &first), normalize_pairs(env, &second))
        }
        reduced => reduced,
    }
}

#[test]
fn top_record_uses_labels_punning_and_omission_without_elimination() {
    // MEASURED: reversed source labels reconstruct `(first, second)`, the
    // punned `second` is in scope, `third` is omitted, raw core contains both
    // projection constructors, and no Elim. CLAIMED: record matching is open,
    // label-keyed negative projection. THE GAP: projection inventory alone
    // cannot prove label selection, so the reversed unequal values are the
    // independent direction discriminator.
    let mut env = ElabEnv::new().expect("base environment");
    let trusted_before = env.env.trusted_base();
    elaborate(&mut env, "record LabelPair { first : Nat, second : Nat }");
    elaborate(
        &mut env,
        "const record_input : LabelPair = { \
         first = Zero, second = Suc Zero }",
    );
    let rebound = elaborate(
        &mut env,
        "const record_rebound : (x : Nat) × Nat = match record_input { \
         { second, first = left } |-> (left, second) }",
    );

    let raw = body(&env, rebound);
    let (proj1, proj2) = count_projection_nodes(&raw);
    assert!(proj1 >= 1, "record lowering must select a field head");
    assert!(
        proj2 >= 1,
        "a later named field must traverse the record tail"
    );
    assert_eq!(
        count_eliminators(&raw),
        0,
        "negative record matching must not emit elim_D"
    );
    let zero = constructor(env.globals["Zero"], []);
    let one = constructor(env.globals["Suc"], [zero.clone()]);
    assert_eq!(normalize_pairs(&env, &raw), Term::pair(zero, one));
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn dependent_later_field_checks_in_declaration_order_not_source_order() {
    // MEASURED: both source orders type `element : carrier` and reduce to the
    // same Nat. CLAIMED: record fields are typed in declaration order while
    // source order is semantically insignificant. THE GAP: one order alone is
    // green under a source-ordered implementation, so the reverse/forward pair
    // is required.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "record Dependent { carrier : Type, value : carrier, tag : Nat }",
    );
    elaborate(
        &mut env,
        "const dependent_record : Dependent = { \
         carrier = Nat, value = Suc Zero, tag = Zero }",
    );
    let reverse = elaborate(
        &mut env,
        "const dependent_reverse : Nat = match dependent_record { \
         { value = element, carrier = carrier } |-> \
         let checked : carrier = element in Zero }",
    );
    let forward = elaborate(
        &mut env,
        "const dependent_forward : Nat = match dependent_record { \
         { carrier = carrier, value = element } |-> \
         let checked : carrier = element in Zero }",
    );

    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(whnf(&env.env, &Context::new(), &body(&env, reverse)), zero);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, forward)),
        whnf(&env.env, &Context::new(), &body(&env, reverse))
    );
}

#[test]
fn omitted_fields_bind_nothing_and_do_not_shift_outer_variables() {
    // MEASURED: a one-field open pattern is exhaustive and an omitted field
    // does not capture or shift the outer `fallback`. CLAIMED: omitted fields
    // are implicit wildcards with no lexical binding.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "record OpenPair { first : Nat, second : Nat }");
    elaborate(
        &mut env,
        "fn retain_outer (fallback : Nat) (pair : OpenPair) : Nat = \
         match pair { { first = _ } |-> fallback }",
    );
    let selected = elaborate(
        &mut env,
        "const retained_outer : Nat = retain_outer (Suc Zero) \
         { first = Zero, second = Zero }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn component_coverage_respects_open_fields_and_names_the_real_gap() {
    // MEASURED: omission covers both second-field values on the True branch,
    // while the False branch reports the missing second-field False. CLAIMED:
    // coverage descends componentwise and treats omission as wildcard.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "record Flags { first : Bool, second : Bool }");
    elaborate(
        &mut env,
        "const flags : Flags = { first = True, second = False }",
    );
    match env.elaborate_decl(
        "const incomplete_record : Nat = match flags { \
         { first = True } |-> Zero ; \
         { first = False, second = True } |-> Zero }",
    ) {
        Err(ElabError::ExhaustivenessError { missing, .. }) => {
            assert_eq!(missing.constructor, "False");
            assert_eq!(missing.arity, 0);
        }
        other => panic!("expected the missing inner False component, got {other:?}"),
    }
}

#[test]
fn open_subset_subsumes_later_fuller_record_with_honest_cause() {
    // MEASURED: the earlier `{ first = a }` row reaches every leaf the fuller
    // row can reach. CLAIMED: open-record redundancy is Subsumed and never the
    // unrelated NoInhabitants fallback.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "record PairRecord { first : Nat, second : Nat }");
    elaborate(
        &mut env,
        "const pair_record : PairRecord = { first = Zero, second = Zero }",
    );
    match env.elaborate_decl(
        "const redundant_record : Nat = match pair_record { \
         { first = earlier } |-> earlier ; \
         { first = later, second = _ } |-> later }",
    ) {
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::Subsumed { rest, .. },
            ..
        }) => assert!(rest.is_empty()),
        Err(ElabError::ReachabilityError {
            cause: ArmDeadCause::NoInhabitants,
            ..
        }) => panic!("a redundant open record arm must not be NoInhabitants"),
        other => panic!("expected open-record subsumption, got {other:?}"),
    }
}

#[test]
fn duplicate_and_unknown_labels_have_distinct_surface_diagnostics() {
    // MEASURED: duplicate and foreign labels select different exact variants,
    // names, and label spans. CLAIMED: labels form a closed declaration-keyed
    // set, never a positional or extension bag. THE GAP: rejection alone could
    // fire for unrelated parsing, so each error identity and coordinate is pinned.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(&mut env, "record Labelled { known : Nat, other : Nat }");
    elaborate(
        &mut env,
        "const labelled : Labelled = { known = Zero, other = Zero }",
    );

    let duplicate_source = "const duplicate_label : Nat = match labelled { \
         { known = first, known = second } |-> first }";
    let duplicate_start = duplicate_source.rfind("known").expect("second label");
    match env.elaborate_decl(duplicate_source) {
        Err(ElabError::ParseError { msg, span }) => {
            assert_eq!(msg, "duplicate record-pattern field 'known'");
            assert_eq!(span.start, duplicate_start);
            assert_eq!(span.end, duplicate_start + "known".len());
        }
        other => panic!("expected duplicate-label diagnostic, got {other:?}"),
    }

    let unknown_source = "const unknown_label : Nat = match labelled { \
         { missing = value } |-> value }";
    let unknown_start = unknown_source.find("missing").expect("unknown label");
    match env.elaborate_decl(unknown_source) {
        Err(ElabError::UnresolvedCon { name, span }) => {
            assert_eq!(name, "Labelled.missing");
            assert_eq!(span.start, unknown_start);
            assert_eq!(span.end, unknown_start + "missing".len());
        }
        other => panic!("expected unknown-label diagnostic, got {other:?}"),
    }
}

#[test]
fn record_fields_compose_with_tuple_constructor_and_as_patterns() {
    // MEASURED: tuple projection inside a named field reaches the Suc branch,
    // and its alias returns the complete Suc occurrence. CLAIMED: record
    // projection consumes the landed occurrence carrier without a parallel
    // nested-pattern path.
    let mut env = ElabEnv::new().expect("base environment");
    elaborate(
        &mut env,
        "record Envelope { payload : (x : Nat) × Nat, enabled : Bool }",
    );
    elaborate(
        &mut env,
        "const envelope : Envelope = { \
         payload = (Suc Zero, Zero), enabled = True }",
    );
    let selected = elaborate(
        &mut env,
        "const envelope_selected : Nat = match envelope { \
         { payload = (Zero, other), enabled = True } |-> other ; \
         { payload = (Suc child as whole, other), enabled = True } |-> whole ; \
         { enabled = False } |-> Zero }",
    );
    let zero = constructor(env.globals["Zero"], []);
    assert_eq!(
        whnf(&env.env, &Context::new(), &body(&env, selected)),
        constructor(env.globals["Suc"], [zero])
    );
}

#[test]
fn module_rewriting_recurses_through_record_field_patterns() {
    // MEASURED: module-local Bool constructors resolve below Record and both
    // arms elaborate. CLAIMED: module rewriting recursively reaches field
    // patterns rather than treating a Record node as a leaf.
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file(
        "module RecordOwner { \
         data LocalFlag = LocalOn | LocalOff ; \
         record Local { flag : LocalFlag, value : Nat } ; \
         const local : Local = { flag = LocalOn, value = Suc Zero } ; \
         fn select (item : Local) : Nat = match item { \
           { flag = LocalOn, value } |-> value ; \
           { flag = LocalOff, value } |-> value \
         } ; \
         pub const result : Nat = select local \
         }",
    )
    .expect("module record pattern elaborates");
    elaborate(
        &mut env,
        "const module_record_result : Nat = RecordOwner.result",
    );
}

fn assert_top_level_refusal(pattern: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    let source = format!("const refused : Nat = match Zero {{ {pattern} |-> Zero }}");
    match env.elaborate_decl(&source) {
        Err(ElabError::Internal(msg)) => assert_eq!(
            msg,
            "non-constructor pattern in match (wildcard/var not yet supported at top level; \
             use constructor patterns)"
        ),
        other => panic!("top-level wildcard/variable must remain refused, got {other:?}"),
    }
}

#[test]
fn top_level_catchalls_remain_refused_and_empty_record_is_not_inferred() {
    // MEASURED: bare top catchalls retain their exact refusal, while `{}` has
    // the record-specific parse diagnostic. CLAIMED: accepting a positive
    // record matcher does not widen the deferred catchall or zero-field forms.
    assert_top_level_refusal("value");
    assert_top_level_refusal("_");

    let mut env = ElabEnv::new().expect("base environment");
    match env.elaborate_decl("const malformed : Nat = match Zero { {} |-> Zero }") {
        Err(ElabError::ParseError { msg, .. }) => {
            assert_eq!(msg, "record patterns require at least one field")
        }
        other => panic!("expected empty-record-pattern diagnostic, got {other:?}"),
    }
}
