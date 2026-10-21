use ken_elaborator::{
    checked_core::{StableSymbol, SymbolNamespace},
    compiler_driver::{compile_ken_source, CompilerSource, CompilerTargetKind, TargetSelector},
    ElabEnv, ElabError, ObligationKind,
};
use ken_kernel::{Decl, KernelError};

const Q1: &str = "def AnyNat = { x : Nat | Top }\n\
    fn f (n : AnyNat) : Nat = match n { Zero |-> Zero ; Suc m |-> f m }";

/// Promise class: durable invariant (spec 17 §4, spec 39 §5.4).
/// MEASURED: the package producer emits a checked private recursive worker
/// and names it in recursion metadata. CLAIMED: public wrapper and private
/// carrier worker remain a closed checked source package. THE GAP: native
/// execution, example omission, and other recursive forms have separate pins.
#[test]
fn refined_parameter_package_contains_recursive_private_worker() {
    let mut env = ElabEnv::new().expect("prelude");
    let results = env.elaborate_file_v1(Q1).expect("q1 must elaborate");
    let result = results.last().expect("public f result");
    assert_eq!(result.auxiliary_ids.len(), 1);
    let worker = result.auxiliary_ids[0];
    assert_ne!(worker, result.def_id);
    assert!(!env.globals.values().any(|id| *id == worker));
    assert!(matches!(
        env.env.lookup(worker),
        Some(Decl::Transparent { .. })
    ));
    assert!(env.env.is_recursive_transparent(worker));
    assert!(!env.env.is_recursive_transparent(result.def_id));

    let package = compile_ken_source(
        "rpsr",
        CompilerSource::new("src/q1.ken", Q1),
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec!["rpsr".into()]),
            symbol: StableSymbol::declaration("rpsr", &[], "f"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("generic source package must include checked worker")
    .package;
    let semantic = &package.artifact.semantic;
    let wrapper = semantic
        .declarations
        .keys()
        .find(|symbol| format!("{symbol:?}").contains("f"))
        .expect("wrapper declaration");
    let private = semantic
        .declarations
        .keys()
        .find(|symbol| format!("{symbol:?}").contains("f#"))
        .expect("worker declaration");
    assert_ne!(wrapper, private);
    assert!(semantic
        .recursion_metadata
        .values()
        .any(|meta| meta.group_members.contains(private)));
    assert!(!semantic
        .recursion_metadata
        .values()
        .any(|meta| meta.group_members.contains(wrapper)));
}

fn one_solved_introduction(env: &mut ElabEnv, source: &str) {
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .expect("refined structural recursion accepts");
    assert_eq!(
        result.auxiliary_ids.len(),
        1,
        "expected one checked private worker"
    );
    assert_eq!(
        result.obligations.len(),
        1,
        "pre-SIGMA base has one introduction"
    );
    assert!(matches!(
        result.obligations[0].kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(!env.is_open_hole(result.obligations[0].hole_id));
    assert_eq!(
        env.env.trusted_base(),
        before,
        "checked worker must add no trust"
    );
    let worker = result.auxiliary_ids[0];
    assert!(!env.globals.values().any(|id| *id == worker));
    assert!(env.env.is_recursive_transparent(worker));
    assert!(!env.env.is_recursive_transparent(result.def_id));
}

/// Promise class: durable invariant (spec 17 §4, spec 39 §5.4).
/// MEASURED: five source shapes accepted at pre-SIGMA base b22a41649 remain
/// checked with one solved RefinementIntroduction each, and the proc follows
/// the same admission route. CLAIMED: carrier-typed workers restore structural
/// descent without erasing the caller's original refinement obligation.
/// THE GAP: package/native/example emission has separate boundary tests.
#[test]
fn refined_parameter_recursion_keeps_base_obligations_for_all_shapes() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("alias");
    env.elaborate_decl("fn leq_n (x : Nat) (y : Nat) : Bool = True")
        .expect("predicate");
    env.elaborate_decl("const ten : Nat = Zero").expect("bound");
    one_solved_introduction(
        &mut env,
        "fn named (n : AnyNat) : Nat = match n { Zero |-> Zero ; Suc m |-> named m }",
    );
    one_solved_introduction(
        &mut env,
        "fn inline (n : { x : Nat | IsTrue (leq_n x ten) }) : Nat = \
         match n { Zero |-> Zero ; Suc m |-> inline m }",
    );
    one_solved_introduction(
        &mut env,
        "fn length (xs : { l : List Int | Top }) : Nat = \
         match xs { Nil |-> Zero ; Cons x rest |-> Suc (length rest) }",
    );
    one_solved_introduction(
        &mut env,
        "fn accum (n : AnyNat) (acc : Nat) : Nat = \
         match n { Zero |-> acc ; Suc m |-> accum m (Suc acc) }",
    );
    one_solved_introduction(
        &mut env,
        "fn nested (n : AnyNat) : Nat = match n { Zero |-> Zero ; \
         Suc m |-> match m { Zero |-> Zero ; Suc p |-> nested p } }",
    );
    one_solved_introduction(
        &mut env,
        "proc process (n : AnyNat) : Nat visits [Console] = \
         match n { Zero |-> Zero ; Suc m |-> process m }",
    );
}

fn must_remain_nonterminating(env: &mut ElabEnv, source: &str) {
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("no structural decrease");
    assert!(
        matches!(error, ElabError::KernelRejected {
        error: KernelError::NotTerminating(ref msg), ..
    } if msg == "SCT: idempotent self-loop has no strictly-decreasing parameter"),
        "expected the unchanged K0014 SCT refusal, got {error:?}"
    );
}

/// Promise class: durable invariant (spec 17 §4).
/// MEASURED: both nondecreasing siblings still fail with exact K0014; a
/// refined side parameter beside an ordinary decreasing parameter selects
/// the original already-accepted body, not a synthesized worker.
/// CLAIMED: the adapter cannot turn DownEq/reconstruction into Down and leaves
/// already-accepted recursive programs byte-identical. THE GAP: the former is
/// measured at these exact two sibling inputs, not all possible group forms.
#[test]
fn nondecreasing_calls_stay_k0014_and_plain_descent_uses_old_path() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("alias");
    must_remain_nonterminating(
        &mut env,
        "fn same (n : AnyNat) : Nat = match n { Zero |-> Zero ; Suc m |-> same n }",
    );
    must_remain_nonterminating(
        &mut env,
        "fn reconstruct (n : AnyNat) : Nat = \
         match n { Zero |-> Zero ; Suc m |-> reconstruct (Suc m) }",
    );
    let unchanged = env
        .elaborate_decl_v1(
            "fn decrease (n : Nat) (side : AnyNat) : Nat = \
         match n { Zero |-> Zero ; Suc m |-> decrease m side }",
        )
        .expect("unaffected source must stay accepted");
    assert!(
        unchanged.auxiliary_ids.is_empty(),
        "old successful SCT path is unchanged"
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 39 §5.4).
/// MEASURED: the V1 contract and mutual-SCC admissions both produce private
/// carrier workers, and each recursive public identity stays outside the
/// worker group. CLAIMED: all three user-recursive admission routes share the
/// same kernel-checked adapter. THE GAP: this pins a two-member mixed group,
/// not all group sizes or combinations of refined argument positions.
#[test]
fn v1_contract_and_mutual_group_reach_refined_worker_adapter() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("alias");
    let v1 = env
        .elaborate_decl_v1(
            "fn needs (n : AnyNat) : Nat requires Top = \
         match n { Zero |-> Zero ; Suc m |-> needs m }",
        )
        .expect("V1 recursive contract accepts");
    assert_eq!(v1.auxiliary_ids.len(), 1);
    assert!(env.env.is_recursive_transparent(v1.auxiliary_ids[0]));
    assert!(!env.env.is_recursive_transparent(v1.def_id));

    let group = env
        .elaborate_file_v1(
            "fn even (n : AnyNat) : Bool = \
         match n { Zero |-> True ; Suc m |-> odd m }\n\
         fn odd (n : AnyNat) : Bool = \
         match n { Zero |-> False ; Suc m |-> even m }",
        )
        .expect("mutual group accepts");
    assert_eq!(group.len(), 2);
    for result in &group {
        assert_eq!(result.auxiliary_ids.len(), 1);
        assert!(env.env.is_recursive_transparent(result.auxiliary_ids[0]));
        assert!(!env.env.is_recursive_transparent(result.def_id));
        assert!(!env
            .globals
            .values()
            .any(|id| *id == result.auxiliary_ids[0]));
    }

    // An unrefined member of a mutually recursive group must itself have a
    // worker: otherwise its edge back into the refined worker is invisible
    // while it is still an opaque public placeholder.
    let mixed = env
        .elaborate_file_v1(
            "fn mixed_a (n : AnyNat) : Bool = \
         match n { Zero |-> True ; Suc m |-> mixed_b m }\n\
         fn mixed_b (n : Nat) : Bool = \
         match n { Zero |-> False ; Suc m |-> mixed_a m }",
        )
        .expect("mixed group admits through one worker SCC");
    assert_eq!(mixed.len(), 2);
    for result in &mixed {
        assert_eq!(result.auxiliary_ids.len(), 1);
        assert!(env.env.is_recursive_transparent(result.auxiliary_ids[0]));
        assert!(!env.env.is_recursive_transparent(result.def_id));
    }
}

/// Promise class: durable invariant (spec 33 §3.2.1, spec 39 §5.4).
/// MEASURED: a checked example fence with its own private recursive worker
/// checks, but neither the example wrapper nor its worker ships in the source
/// package. CLAIMED: the example exclusion follows the explicit worker owner
/// across the package boundary. THE GAP: this is a single-fence example; it
/// does not assert behavior of a separately named nested module export.
#[test]
fn recursive_refined_example_excludes_both_wrapper_and_worker() {
    let source = "# Checked package\n\n```ken\n\
        def AnyNat = { x : Nat | Top }\n\
        const base : Nat = Zero\n\
        ```\n\n```ken example\n\
        fn ex (n : AnyNat) : Nat = match n { Zero |-> Zero ; Suc m |-> ex m }\n\
        ```\n";
    let out = compile_ken_source(
        "rpsr-example",
        CompilerSource::new("examples/rpsr.ken.md", source),
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec!["rpsr-example".into()],
            ),
            symbol: StableSymbol::declaration("rpsr-example", &[], "base"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("example checks without shipping private declarations");
    let semantic = &out.package.artifact.semantic;
    assert!(semantic
        .declarations
        .contains_key(&StableSymbol::declaration("rpsr-example", &[], "base",)));
    let is_example_decl = |symbol: &StableSymbol| {
        symbol.namespace == SymbolNamespace::Declaration
            && symbol
                .components
                .last()
                .is_some_and(|name| name == "ex" || name.starts_with("ex#"))
    };
    assert!(
        !semantic.declarations.keys().any(is_example_decl),
        "neither checked example wrapper nor worker declaration may ship"
    );
    assert!(
        !semantic.symbols.iter().any(|symbol| {
            symbol.namespace == SymbolNamespace::Declaration
                && symbol
                    .components
                    .last()
                    .is_some_and(|name| name.starts_with("ex#"))
        }),
        "the example worker must not enter the source symbol set"
    );
    // The example's checked obligation may still name its public owner `ex`
    // as provenance; that reference is not an emitted declaration.
    assert!(!semantic
        .recursion_metadata
        .values()
        .any(|meta| { meta.group_members.iter().any(is_example_decl) }));
}
