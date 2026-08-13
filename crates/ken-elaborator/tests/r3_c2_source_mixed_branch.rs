//! `RT-LEXICAL-R3` **C2-source** — the mixed nested/W-style candidate, and the
//! boundary walk Architect `evt_10ayk8fbjsz74` authorized over it.
//!
//! **This is an AUTHORIZED ATTEMPT, not a claim that every gate passes.** The
//! ruling is explicit that a stop with the exact boundary named is a complete
//! result. Gates 1-3 and gate 4a are positive and asserted below. Gate 4b is
//! still unreached.
//!
//! ## The candidate
//!
//! The same nested generated-lift family as `C1`, extended — not replaced, and
//! **not** a return to ordinary direct recursion — with a W-style constructor:
//!
//! ```text
//! Fork : (Bool -> Bag a) -> Bag a
//! ```
//!
//! One lifted-family match then carries both required branches:
//!
//! - **`Join`** — the classifier witness. Its `recursive result for xs/ys`
//!   selectors reference the recursive binder range, which is what earns the
//!   *whole* match its computational classification (the predicate is existential
//!   over branches — Architect `evt_7mg1x1vqe7qph`).
//! - **`Fork`** — the branch under test. It applies its selected function field
//!   directly (`k True`) and **never selects or invokes that position's hidden
//!   recursive result**.
//!
//! Motive stays Type-classified; the Omega spelling is deliberately not used, so
//! nothing here conflates the producer question with proof erasure.
//!
//! ## The walk
//!
//! | gate | what it asks | result |
//! |---|---|---|
//! | 1 | checked mixed match exists, classified computational via the selector branch | **POSITIVE**, asserted |
//! | 2 | `Fork` branch has a recursive position but no recursive-range reference | **POSITIVE**, asserted |
//! | 3 | erased Runtime IR retains that match and `Fork`'s recursive-position identity | **POSITIVE**, asserted |
//! | 4a | compiler-derived IH slots/calls and validated oriented plan | **POSITIVE**, asserted |
//! | 4b | existing Runtime static-fusion planner population | not reached |
//! | 5 | `R3` composed-seat arrival for the `Fork` branch | not reached |
//! | 6 | shared transport, `ConstructorChild` resolution, consume-once, green close | not reached |
//!
//! ## GATE 4a — THE RET-PAYLOAD RELATION
//!
//! The compiler-owned pre-object preparation seam now makes the plan-bearing
//! path readable without entering object emission. The C2 source below extends
//! the original fixture with a real `main` whose `ProcessInput` selects the
//! value passed to `liftSize`; it cannot fold to `[main]`.
//!
//! Normalization places the mixed computational match inside the checked HostIO
//! `Ret` payload. That payload now uses the existing plan-aware value lowering,
//! so all three compiler-derived slot templates are consumed exactly once and
//! preparation returns the immutable planned Runtime program. No caller-authored
//! plan or marker was introduced, and no classifier, checker, marker,
//! enumeration, fusion candidate, Runtime representation, ledger, or
//! closure-boundary mechanism was changed. Gate 4b and gates 5-6 remain
//! unreached; production remains unarmed.
//!
//! ## Promise class
//!
//! **Durable invariant** for gates 1-4a: they assert relations — the existence of
//! a classifier-earning selector branch, and a sibling branch that carries a
//! recursive position while referencing none of it — which any extension
//! preserving the mixed shape keeps green. Gate 4a additionally requires the
//! production preparation to consume the compiler-derived slot population and
//! return successfully.
//!
//! No classifier, checker, marker, oriented plan, planner census, fusion
//! candidate, ledger, or Runtime mechanism is touched. Production stays unarmed.

use ken_elaborator::checked_core::{
    checked_core_body_view_for_selection, CheckedCoreBodyTerm, CheckedCoreBodyViewSelection,
    CheckedCoreMatchBranchView, CheckedCoreMatchView, StableSymbol, SymbolNamespace,
};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, compile_native_program_sources, prepare_native_program_sources,
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The `C1` family extended with the W-style `Fork`. The `Join` branch's
/// selectors and the `Fork` branch's direct `k True` application are the two
/// structural facts the ruling fixes; they must not be edited.
const C2_MIXED_SOURCE: &str = "program capabilities FS APartial\n\
    data Bag (a : Type) : Type where { \
      Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a ; \
      Fork : (Bool -> Bag a) -> Bag a \
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
                              (recursive result for ys) ; \
        Fork k |-> match k True { \
          Empty |-> Zero ; \
          One _ |-> Suc Zero ; \
          Join _ _ |-> Suc (Suc Zero) ; \
          Fork _ |-> Suc (Suc (Suc Zero)) \
        } \
      }) \
    }\n\
    const liftSizeResult : Nat = liftSize \
      (LiftNode (Join LiftRose \
        (One LiftRose LiftLeaf) \
        (One LiftRose (LiftNode (Empty LiftRose)))))\n\
    fn inputRose (input : ProcessInput) : LiftRose = match input { \
      MkProcessInput arguments _environment _cwd |-> match arguments { \
        Nil |-> LiftLeaf ; \
        Cons _ rest |-> match rest { \
          Nil |-> LiftLeaf ; \
          Cons _ _ |-> LiftNode (Join LiftRose \
            (One LiftRose LiftLeaf) \
            (Fork LiftRose (\\b. match b { \
              False |-> Empty LiftRose ; True |-> One LiftRose LiftLeaf \
            }))) \
        } \
      } \
    }\n\
    fn sizeExit (n : Nat) : ExitCode = match n { \
      Zero |-> Success ; Suc _ |-> Failure 7 \
    }\n\
    fn main (input : ProcessInput) (_caps : ProgramCaps APartial) \
      : HostIO APartial ExitCode = \
      host_exit APartial (sizeExit (liftSize (inputRose input)))";

fn decl_symbol(package: &str, name: &str) -> StableSymbol {
    StableSymbol::declaration(package, &[], name)
}

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

/// The occurrences a branch's peeled method makes into its own recursive binder
/// range. Empty means the branch carries a recursive position it never consults —
/// which is exactly the `Fork` relation gate 2 asks for.
fn recursive_range_occurrences(branch: &CheckedCoreMatchBranchView) -> Vec<usize> {
    let recursive_count = branch.constructor.recursive_positions.len();
    let binders = branch.constructor.argument_count + recursive_count;
    let mut body = &branch.method;
    for _ in 0..binders {
        let CheckedCoreBodyTerm::Lambda { body: next, .. } = body else {
            panic!(
                "branch {} must peel {binders} binders",
                branch.constructor.symbol
            );
        };
        body = next.as_ref();
    }
    let mut occurrences = Vec::new();
    collect_variables(body, 0, &mut occurrences);
    occurrences
        .into_iter()
        .filter(|index| *index < recursive_count)
        .collect()
}

fn runtime_recursive_runs(expr: &ken_runtime::RuntimeExpr, out: &mut Vec<Vec<usize>>) {
    use ken_runtime::RuntimeExpr as R;
    match expr {
        R::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            runtime_recursive_runs(scrutinee, out);
            for case in cases {
                if !case.recursive_positions.is_empty() {
                    out.push(case.recursive_positions.clone());
                }
                runtime_recursive_runs(&case.body, out);
            }
        }
        R::Match {
            scrutinee, cases, ..
        } => {
            runtime_recursive_runs(scrutinee, out);
            for case in cases {
                runtime_recursive_runs(&case.body, out);
            }
        }
        R::CheckedComputationalIHSlots { body, .. }
        | R::CheckedComputationalIHInvocation { body, .. }
        | R::CheckedJoinSite { body, .. }
        | R::CheckedSubcontinuationFrame { body, .. }
        | R::CheckedRecursiveInvocation { body, .. }
        | R::Closure { body, .. }
        | R::LexicalClosure { body, .. }
        | R::Project { record: body, .. } => runtime_recursive_runs(body, out),
        R::Let { value, body } => {
            runtime_recursive_runs(value, out);
            runtime_recursive_runs(body, out);
        }
        R::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            runtime_recursive_runs(scrutinee, out);
            runtime_recursive_runs(then_expr, out);
            runtime_recursive_runs(else_expr, out);
        }
        R::PrimitiveCall { args, .. } | R::Construct { args, .. } => {
            for a in args {
                runtime_recursive_runs(a, out);
            }
        }
        R::Record { fields } => {
            for (_, v) in fields {
                runtime_recursive_runs(v, out);
            }
        }
        R::Call { callee, args } => {
            runtime_recursive_runs(callee, out);
            for a in args {
                runtime_recursive_runs(a, out);
            }
        }
        R::Effect {
            capability, args, ..
        } => {
            if let Some(k) = capability {
                runtime_recursive_runs(&k.value, out);
            }
            for a in args {
                runtime_recursive_runs(a, out);
            }
        }
        _ => {}
    }
}

#[test]
fn c2_source_mixed_branch_walk_gates_one_to_three() {
    let package_name = "r3_c2_mixed_pkg";
    let target = decl_symbol(package_name, "liftSize");

    // ---- GATE 1a: the fixed mixed shape ELABORATES through the ordinary front
    // end. A stop condition of the ruling is to report the exact surface/type
    // boundary if it does not; it does.
    let out = compile_ken_package_sources(
        &CompilerManifest::new(package_name, Vec::new()),
        vec![CompilerSource::new("src/main.ken", C2_MIXED_SOURCE)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![package_name.to_string()],
            ),
            symbol: target.clone(),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("GATE 1a: the mixed nested/W-style candidate elaborates");
    let closure = out.closures.first().expect("selected target closure");
    assert!(
        closure.reachable_declarations.contains(&target),
        "GATE 1b: production selection retains liftSize"
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
        .expect("liftSize has a checked body");

    let mut matches = Vec::new();
    find_matches(&declaration.body, &mut matches);

    // The lifted-family match is the one carrying MORE THAN ONE recursive-position
    // branch: the generated lift gives both Join and Fork hidden results, whereas
    // the source `Bag` match carries them without the lift's binder layout.
    // Identified structurally -- the family's symbol is generated.
    let mixed = matches
        .iter()
        .find(|m| {
            m.branches
                .iter()
                .filter(|b| !b.constructor.recursive_positions.is_empty())
                .count()
                == 2
                && m.branches.iter().any(|b| {
                    b.constructor.recursive_positions.len() == 2
                        && !recursive_range_occurrences(b).is_empty()
                })
        })
        .expect("GATE 1c: a lifted-family match carries both recursive-position branches");

    // ---- GATE 1d: motive is computationally classified.
    assert!(
        mixed.computational_recursive_hypotheses,
        "GATE 1d: the mixed lifted-family match's motive is computationally classified"
    );

    // ---- GATE 1e: the SELECTOR branch earns the whole match's classification.
    let selector = mixed
        .branches
        .iter()
        .find(|b| b.constructor.recursive_positions.len() == 2)
        .expect("the mixed match has its two-recursive-position selector branch");
    let selector_occurrences = recursive_range_occurrences(selector);
    assert_eq!(
        selector.constructor.recursive_positions,
        vec![2, 3],
        "GATE 1e: the Join branch carries its two hidden recursive-result positions"
    );
    assert_eq!(
        selector_occurrences.len(),
        2,
        "GATE 1e: both `recursive result for` selectors reference the recursive binder range, \
         which is what earns the WHOLE match its computational classification"
    );

    // ---- GATE 2: the Fork branch HAS a recursive position and references NONE
    // of it. This is the C2 relation itself.
    let fork = mixed
        .branches
        .iter()
        .find(|b| b.constructor.recursive_positions.len() == 1)
        .expect("GATE 2: the mixed match has the W-style Fork branch");
    assert_eq!(
        fork.constructor.recursive_positions,
        vec![1],
        "GATE 2: Fork carries exactly one hidden recursive-result position"
    );
    assert!(
        recursive_range_occurrences(fork).is_empty(),
        "GATE 2: the Fork branch applies its selected field directly and NEVER references its \
         own hidden recursive result -- this is the selected-argument-only branch under test, \
         lawfully sitting inside a match another branch classified"
    );

    // ---- GATE 3: the shape SURVIVES erasure, including Fork's identity.
    let program =
        erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
            .expect("GATE 3: the mixed closure erases");
    let mut runs = Vec::new();
    for d in &program.declarations {
        if let ken_runtime::RuntimeDeclarationKind::Transparent { body } = &d.kind {
            runtime_recursive_runs(body, &mut runs);
        }
    }
    assert!(
        runs.contains(&vec![2, 3]),
        "GATE 3: the erased ComputationalMatch retains the selector branch's recursive run"
    );
    assert!(
        runs.contains(&vec![1]),
        "GATE 3: the erased ComputationalMatch ALSO retains the Fork branch's recursive-position \
         identity -- the selected-argument branch survives to Runtime IR as itself"
    );

    // Gate 4a is asserted by the production preparation control below.
}

/// Durable invariant for the ordered gate-4a relation.
///
/// MEASURED: this real-source C2 witness reaches production preparation, whose
/// total-consumption validator accepts all compiler-derived slot templates.
///
/// CLAIMED: a computational match normalized into a checked HostIO `Ret` payload
/// retains the same slot population through plan-aware value lowering.
///
/// THE GAP: successful preparation alone could be a zero-slot false green. The
/// decoded production plan below therefore asserts the exact three templates,
/// their marker binding, and their path through the selected `Ret` payload.
#[test]
fn c2_gate_4a_preparation_consumes_every_compiler_derived_slot() {
    let package_name = "r3_c2_mixed_native_pkg";
    let preparation = prepare_native_program_sources(
        package_name,
        vec![CompilerSource::new("src/main.ken", C2_MIXED_SOURCE)],
    )
    .expect("input-dependent C2 reaches an immutable pre-object preparation");

    let plan_key = StableSymbol::new(
        SymbolNamespace::Metadata,
        vec![
            package_name.to_string(),
            "OrientedSubcontinuationPlanV1".to_string(),
        ],
    )
    .to_string();
    let plan_bytes = preparation
        .runtime_program()
        .erased_core
        .metadata
        .checked_core
        .metadata
        .get(&plan_key)
        .expect("preparation carries its oriented plan metadata");
    let plan = ken_runtime::OrientedSubcontinuationPlanV1::decode(plan_bytes)
        .expect("production emits a valid oriented plan");
    let mut supplied_and_consumed = plan
        .computational_ih_slots
        .iter()
        .map(|slot| {
            (
                slot.slot_template_id,
                slot.checked_match_ordinal,
                slot.recursive_position,
                slot.method_binder_ordinal,
            )
        })
        .collect::<Vec<_>>();
    supplied_and_consumed.sort_unstable();
    assert_eq!(
        supplied_and_consumed,
        vec![(0, 0, 2, 0), (1, 0, 3, 1), (2, 0, 1, 0)],
        "successful total validation must expose the exact three compiler-derived slot templates"
    );
    assert!(
        plan.computational_ih_slots
            .iter()
            .all(|slot| slot.runtime_marker_locations.len() == 1),
        "every supplied slot template must bind exactly one consumed Runtime marker"
    );
    assert!(
        plan.computational_ih_slots
            .iter()
            .all(|slot| slot.checked_occurrence_path.get(1..3) == Some(&[4, 3])),
        "every slot occurrence must descend through Ret's measured arity-four payload at index three"
    );

    assert!(
        preparation.executable_closure().len() > 1,
        "the input-dependent C2 witness must not fold to [main]"
    );
}

const R3_4B_ARTIFACT_OUTPUT: &str = "KEN_R3_4B_ARTIFACT_OUTPUT";
const R3_4B_IDENTITY_SOURCE: &str = r#"program capabilities FS APartial
fn unused_sibling (_input : ProcessInput) : ExitCode = Success
fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode = host_exit APartial Success
"#;

/// Nested-compilation worker for the artifact-identity control below.
///
/// The ordinary test process has no output path and returns immediately. The
/// driver runs this exact test twice from two independently compiled test
/// binaries. In the feature-on binary it opens the existing D2f observation
/// scope but deliberately does not inspect the rows: this increment proves
/// only that observation leaves an emitted native object unchanged. The
/// feature makes the C2 observation reachable to this crate, but this
/// increment deliberately does not measure that witness or any planner result.
#[test]
fn r3_4b_feature_artifact_worker() {
    let Some(output_dir) = std::env::var_os(R3_4B_ARTIFACT_OUTPUT).map(PathBuf::from) else {
        return;
    };
    let compile = || {
        compile_native_program_sources(
            "r3_4b_feature_identity_pkg",
            vec![CompilerSource::new(
                "src/main.ken",
                R3_4B_IDENTITY_SOURCE,
            )],
            &output_dir,
        )
    };

    #[cfg(feature = "r3-4b-observation")]
    let output = {
        let observation = ken_runtime::d2f_gate_observation_scope();
        let output = compile();
        drop(observation);
        output
    };
    #[cfg(not(feature = "r3-4b-observation"))]
    let output = compile();

    output.expect("the identity-control source emits its native object artifact");
    assert!(
        output_dir.join("ken-entrypoint.o").is_file(),
        "the worker must leave the emitted native object at the packaging path"
    );
}

fn r3_4b_compile_artifact(
    target_dir: &Path,
    output_dir: &Path,
    observation_feature: bool,
) -> Vec<u8> {
    std::fs::create_dir_all(target_dir).expect("per-configuration Cargo target directory");
    std::fs::create_dir_all(output_dir).expect("per-configuration native output directory");

    // Use Cargo directly inside the outer `scripts/ken-cargo` invocation. The
    // wrapper's machine-wide lock is already held; recursively taking it would
    // deadlock. The distinct private target directories provide isolation.
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("test")
        .arg("--manifest-path")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .arg("--test")
        .arg("r3_c2_source_mixed_branch")
        .arg("--no-default-features")
        .arg("--target-dir")
        .arg(target_dir)
        .env(R3_4B_ARTIFACT_OUTPUT, output_dir);
    if observation_feature {
        command.arg("--features").arg("r3-4b-observation");
    }
    command
        .arg("r3_4b_feature_artifact_worker")
        .arg("--")
        .arg("--exact")
        .arg("--nocapture");

    let output = command.output().expect("nested Cargo compilation runs");
    assert!(
        output.status.success(),
        "nested {} compilation failed:\nstdout:\n{}\nstderr:\n{}",
        if observation_feature {
            "feature-on"
        } else {
            "feature-off"
        },
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    std::fs::read(output_dir.join("ken-entrypoint.o"))
        .expect("the nested compilation leaves the actual native object artifact")
}

/// The default-off observation feature is inert at an emitted native object.
///
/// These are two Cargo compilations, not two executions of one test binary.
/// Each has its own target directory, and each worker emits the actual native
/// object from the same fixed source into its own output directory.
#[test]
fn r3_4b_observation_feature_is_native_artifact_identical() {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "r3-4b-observation-feature-{}",
        std::process::id()
    ));
    let off_target = root.join("feature-off-target");
    let on_target = root.join("feature-on-target");
    let off_output = root.join("feature-off-artifact");
    let on_output = root.join("feature-on-artifact");

    let off_bytes = r3_4b_compile_artifact(&off_target, &off_output, false);
    let on_bytes = r3_4b_compile_artifact(&on_target, &on_output, true);

    assert!(
        !off_bytes.is_empty() && !on_bytes.is_empty(),
        "the identity relation must compare two emitted objects, not empty buffers"
    );
    assert_eq!(
        off_bytes, on_bytes,
        "the feature-off and feature-on native object artifacts must be byte-identical"
    );
}
