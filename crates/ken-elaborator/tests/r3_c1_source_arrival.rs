//! `RT-LEXICAL-R3` **C1-source-arrival** — real Ken source reaches a retained
//! `ComputationalMatch` in erased Runtime IR, through the ordinary front end.
//!
//! Architect `evt_10ayk8fbjsz74` released this control on the back of the
//! bounded producer probe it authorized at `evt_2gzjt1zqy402z`. The lawful
//! production source is the **nested-result association**:
//!
//! ```text
//! surface `recursive result for xs/ys`
//!   -> checked hidden recursive-result variables
//!   -> recursive binder range
//!   -> retained generated lifted-family ComputationalMatch
//!   -> erased Runtime IR
//! ```
//!
//! ## WHAT THIS CONTROL PROVES, AND — IN ITS OWN WORDS — WHAT IT DOES NOT
//!
//! It proves **checker-to-Runtime-IR ARRIVAL**: a checker-produced computational
//! match, carrying its recursive-position identity, survives production selection
//! and erasure and is handed to Runtime.
//!
//! ⛔ **It proves NOTHING about fusion-plan population, and NOTHING about arrival
//! at the `R3` composed seat.** Those are strictly further down the pipeline than
//! anything measured here, and this row does not observe them at all. The
//! measured `ih_slots = 0, ih_invocations = 0` below is the concrete reason the
//! stronger reading is unavailable: the classification arrives **without** either
//! marker, so nothing here shows a fusion candidate being populated.
//!
//! **This warning is not boilerplate — this node has already paid for a control
//! whose text outran its measurement.** Four universal claims were published from
//! a bounded eight-program sample and had to be withdrawn at four separate source
//! sites (`f5afd91f`) after one probe refuted them. A reader arriving here cold
//! must not be able to take arrival for end-to-end.
//!
//! ## Promise classes, per assertion group
//!
//! - **Durable invariant** — every relation asserted below: retention of
//!   `liftSize`, the computational classification, the recursive-position run,
//!   and the selector-produced occurrences lying inside the recursive binder
//!   range. Any extension that keeps the nested-result association keeps these
//!   green; losing the association reds them.
//! - **Transition sentinel** — `ih_slots == 0 && ih_invocations == 0` only. It is
//!   named for its boundary and it names the event that retires it: **when
//!   checked IH slot/invocation metadata begins to populate for this family, this
//!   row must go red so that C1's boundary statement above is re-derived rather
//!   than inherited.** Do not simply update the numbers — the whole point of the
//!   row is that the conclusion changes when they move.
//!
//! Nothing here arms anything. No classifier, checker, marker, plan, or Runtime
//! mechanism is touched; the control reads production through public API only.

use std::collections::BTreeSet;

use ken_elaborator::checked_core::{
    checked_core_body_view_for_selection, CheckedCoreBodyTerm, CheckedCoreBodyViewSelection,
    CheckedCoreMatchView, StableSymbol, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;

/// Unchanged in substance from `nc14_data_match_lowering.rs`, which already runs
/// it through elaboration, kernel checking, erasure and the interpreter. The
/// `recursive result for xs/ys` selectors in the `Join` branch are the producer
/// under test and must not be edited.
const NESTED_LIFT_NAT_THREE_SOURCE: &str = "data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a \
    }\n\
    data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)\n\
    fn liftAdd (x : Nat) (y : Nat) : Nat = match x { \
      Zero |-> y ; Suc x2 |-> Suc (liftAdd x2 y) \
    }\n\
    fn liftSize (r : LiftRose) : Nat = match r { \
      LiftLeaf |-> Suc Zero ; \
      LiftNode b |-> Suc (match b { \
        Empty |-> Zero ; \
        One x |-> liftSize x ; \
        Join xs ys |-> liftAdd (recursive result for xs) \
                              (recursive result for ys) \
      }) \
    }\n\
    const liftSizeResult : Nat = liftSize \
      (LiftNode (Join LiftRose \
        (One LiftRose LiftLeaf) \
        (One LiftRose (LiftNode (Empty LiftRose)))))";

/// The recursive-position run the `Join` branch of the generated lifted family
/// carries. **A normative compatibility vector**: it is the arity relation
/// between `Join`'s two source arguments and its two hidden recursive results,
/// so changing it is a change to the lift's shape, not a refactor.
const JOIN_RECURSIVE_POSITIONS: [usize; 2] = [2, 3];

/// The two occurrences the `recursive result for xs` / `... for ys` selectors
/// produce in the peeled branch body. Asserted as a relation — each must land
/// **inside** the recursive binder range — with the exact pair kept because the
/// ruling names it.
const SELECTOR_OCCURRENCES: [usize; 2] = [1, 0];

fn decl_symbol(package: &str, name: &str) -> StableSymbol {
    StableSymbol::declaration(package, &[], name)
}

/// Every `Variable` occurrence in a peeled branch body, renormalized to that
/// body's own depth-0 frame so an index can be compared against the branch's
/// recursive binder range.
fn collect_variables(term: &CheckedCoreBodyTerm, depth: usize, out: &mut Vec<usize>) {
    match term {
        CheckedCoreBodyTerm::Variable { de_bruijn_index } => {
            if *de_bruijn_index >= depth {
                out.push(*de_bruijn_index - depth);
            }
        }
        CheckedCoreBodyTerm::Lambda { body, .. } => collect_variables(body, depth + 1, out),
        CheckedCoreBodyTerm::Application { function, argument } => {
            collect_variables(function, depth, out);
            collect_variables(argument, depth, out);
        }
        CheckedCoreBodyTerm::Let { value, body, .. } => {
            collect_variables(value, depth, out);
            collect_variables(body, depth + 1, out);
        }
        CheckedCoreBodyTerm::Match(view) => {
            collect_variables(&view.scrutinee, depth, out);
            for branch in &view.branches {
                collect_variables(&branch.method, depth, out);
            }
        }
        CheckedCoreBodyTerm::PrimitiveApplication(view) => {
            for argument in &view.arguments {
                collect_variables(argument, depth, out);
            }
        }
        _ => {}
    }
}

fn find_matches<'a>(term: &'a CheckedCoreBodyTerm, out: &mut Vec<&'a CheckedCoreMatchView>) {
    match term {
        CheckedCoreBodyTerm::Match(view) => {
            out.push(view);
            find_matches(&view.scrutinee, out);
            for branch in &view.branches {
                find_matches(&branch.method, out);
            }
        }
        CheckedCoreBodyTerm::Lambda { body, .. } => find_matches(body, out),
        CheckedCoreBodyTerm::Application { function, argument } => {
            find_matches(function, out);
            find_matches(argument, out);
        }
        CheckedCoreBodyTerm::Let { value, body, .. } => {
            find_matches(value, out);
            find_matches(body, out);
        }
        CheckedCoreBodyTerm::PrimitiveApplication(view) => {
            for argument in &view.arguments {
                find_matches(argument, out);
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct RuntimeCensus {
    computational: usize,
    recursive_position_runs: Vec<Vec<usize>>,
    ih_slots: usize,
    ih_invocations: usize,
}

fn walk_runtime(expr: &ken_runtime::RuntimeExpr, c: &mut RuntimeCensus) {
    use ken_runtime::RuntimeExpr;
    match expr {
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            c.computational += 1;
            walk_runtime(scrutinee, c);
            for case in cases {
                if !case.recursive_positions.is_empty() {
                    c.recursive_position_runs
                        .push(case.recursive_positions.clone());
                }
                walk_runtime(&case.body, c);
            }
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            walk_runtime(scrutinee, c);
            for case in cases {
                walk_runtime(&case.body, c);
            }
        }
        RuntimeExpr::CheckedComputationalIHSlots { body, .. } => {
            c.ih_slots += 1;
            walk_runtime(body, c);
        }
        RuntimeExpr::CheckedComputationalIHInvocation { body, .. } => {
            c.ih_invocations += 1;
            walk_runtime(body, c);
        }
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::Closure { body, .. }
        | RuntimeExpr::LexicalClosure { body, .. }
        | RuntimeExpr::Project { record: body, .. } => walk_runtime(body, c),
        RuntimeExpr::Let { value, body } => {
            walk_runtime(value, c);
            walk_runtime(body, c);
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            walk_runtime(scrutinee, c);
            walk_runtime(then_expr, c);
            walk_runtime(else_expr, c);
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Construct { args, .. } => {
            for a in args {
                walk_runtime(a, c);
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, v) in fields {
                walk_runtime(v, c);
            }
        }
        RuntimeExpr::Call { callee, args } => {
            walk_runtime(callee, c);
            for a in args {
                walk_runtime(a, c);
            }
        }
        RuntimeExpr::Effect {
            capability, args, ..
        } => {
            if let Some(cap) = capability {
                walk_runtime(&cap.value, c);
            }
            for a in args {
                walk_runtime(a, c);
            }
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. }
        | RuntimeExpr::Trap(_) => {}
    }
}

/// One selection of `liftSize` as a production target, walked from checked body
/// view through to erased Runtime IR.
fn assert_nested_result_producer_arrives(package_name: &str, kind: CompilerTargetKind) {
    let label = format!("{kind:?}");
    let target = decl_symbol(package_name, "liftSize");
    let out = compile_ken_package_sources(
        &CompilerManifest::new(package_name, Vec::new()),
        vec![CompilerSource::new(
            "src/main.ken",
            NESTED_LIFT_NAT_THREE_SOURCE,
        )],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![package_name.to_string()],
            ),
            symbol: target.clone(),
            kind,
        },
    )
    .expect("the nested-result source compiles through the ordinary front end");
    let closure = out.closures.first().expect("selected target closure");

    // RELATION 1 — production selection RETAINS the producing declaration.
    // Selecting `liftSize` directly is what makes this retention rather than
    // incidental reachability through `liftSizeResult`.
    assert!(
        closure.reachable_declarations.contains(&target),
        "{label} selection of liftSize must retain liftSize itself, not reach it incidentally"
    );

    let selection = CheckedCoreBodyViewSelection {
        package_identity: closure.report.package_identity.clone(),
        package_core_semantic_hash: closure.report.package_core_semantic_hash,
        package_artifact_hash: closure.report.package_artifact_hash,
        target_symbol: closure.report.target_symbol.clone(),
        reachable_declarations: closure.reachable_declarations.clone(),
        external_symbols: closure.external_symbols.clone(),
        dependency_semantic_hashes: closure.report.dependency_semantic_hashes.clone(),
    };
    let view = checked_core_body_view_for_selection(&out.package, &selection)
        .expect("the selected closure yields a checked body view");
    let declaration = view
        .declarations
        .get(&target)
        .expect("liftSize has a checked body in its own closure");

    let mut matches = Vec::new();
    find_matches(&declaration.body, &mut matches);

    // RELATION 2 — exactly one of liftSize's checked matches carries a branch
    // with recursive positions. Identified STRUCTURALLY: the generated lifted
    // family's symbol is a generated name and pinning it would be a snapshot.
    let carrying: Vec<&&CheckedCoreMatchView> = matches
        .iter()
        .filter(|m| {
            m.branches
                .iter()
                .any(|b| !b.constructor.recursive_positions.is_empty())
        })
        .collect();
    assert_eq!(
        carrying.len(),
        1,
        "{label}: exactly one checked match in liftSize carries a recursive-position branch"
    );
    let carrying = carrying[0];

    // RELATION 3 — the classifier answers TRUE on that match.
    assert!(
        carrying.computational_recursive_hypotheses,
        "{label}: the lifted-family match is classified computational"
    );

    // RELATION 4 + 5 — the Join branch's recursive-position run, and the two
    // selector-produced occurrences lying INSIDE its recursive binder range.
    let join = carrying
        .branches
        .iter()
        .find(|b| !b.constructor.recursive_positions.is_empty())
        .expect("the carrying match has its recursive-position branch");
    assert_eq!(
        join.constructor.recursive_positions,
        JOIN_RECURSIVE_POSITIONS.to_vec(),
        "{label}: the Join branch carries its two hidden recursive-result positions"
    );

    let recursive_count = join.constructor.recursive_positions.len();
    let binders = join.constructor.argument_count + recursive_count;
    let mut body = &join.method;
    for position in 0..binders {
        let CheckedCoreBodyTerm::Lambda { body: next, .. } = body else {
            panic!("{label}: Join branch method peels {binders} binders (stuck at {position})");
        };
        body = next.as_ref();
    }
    let mut occurrences = Vec::new();
    collect_variables(body, 0, &mut occurrences);
    assert_eq!(
        occurrences,
        SELECTOR_OCCURRENCES.to_vec(),
        "{label}: the two `recursive result for` selectors produce exactly these occurrences"
    );
    for occurrence in &occurrences {
        assert!(
            *occurrence < recursive_count,
            "{label}: selector occurrence {occurrence} lies inside the recursive binder range \
             0..{recursive_count} -- this is the relation the classifier reads"
        );
    }

    // RELATION 6 — the same shape SURVIVES erasure into Runtime IR.
    let program =
        erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
            .expect("the selected closure erases");
    let retained: BTreeSet<String> = program
        .declarations
        .iter()
        .map(|d| d.symbol.clone())
        .collect();
    assert!(
        retained.contains(&target.to_string()),
        "{label}: liftSize survives erasure into Runtime IR"
    );
    let mut census = RuntimeCensus::default();
    for declaration in &program.declarations {
        if let ken_runtime::RuntimeDeclarationKind::Transparent { body } = &declaration.kind {
            walk_runtime(body, &mut census);
        }
    }
    assert!(
        census.computational > 0,
        "{label}: the erased Runtime IR retains at least one ComputationalMatch"
    );
    assert!(
        census
            .recursive_position_runs
            .contains(&JOIN_RECURSIVE_POSITIONS.to_vec()),
        "{label}: the retained ComputationalMatch carries the Join recursive-position run, \
         so it is the SAME match and not merely some computational match"
    );

    // THE BOUNDARY, asserted rather than only described. TRANSITION SENTINEL:
    // when checked IH slot/invocation metadata begins to populate for this
    // family, this goes red ON PURPOSE so that C1's "arrival only" statement is
    // re-derived. Do not just update the numbers.
    assert_eq!(
        (census.ih_slots, census.ih_invocations),
        (0, 0),
        "{label}: C1 measures ARRIVAL ONLY -- the classification reaches Runtime IR with no IH \
         slot or invocation marker. If this is red, marker population has changed and C1's \
         boundary (it proves nothing about fusion-plan population or R3 composed-seat arrival) \
         must be RE-DERIVED, not restated with new numbers."
    );
}

#[test]
fn c1_source_nested_result_reaches_runtime_ir_under_executable_selection() {
    assert_nested_result_producer_arrives("r3_c1_exec_pkg", CompilerTargetKind::Executable);
}

#[test]
fn c1_source_nested_result_reaches_runtime_ir_under_library_selection() {
    assert_nested_result_producer_arrives("r3_c1_lib_pkg", CompilerTargetKind::Library);
}
