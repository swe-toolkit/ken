//! Strict local B-name introduction is refused at the source binder, while
//! the same source retains Legacy resolution. These are behavioral pins, not
//! assertions about the repository's text or about a frozen B inventory.

use ken_elaborator::error::{ElabError, Span};
use ken_elaborator::ElabEnv;
use std::fs;

// Promise class: durable invariant (spec 33, no binding of a built-in name).
// MEASURED: each Strict source returns the named binder error at the own or
// introducing AST span and that same source checks under Legacy.
// CLAIMED: every represented binder form reaches the Strict introduction gate
// without changing Legacy. THE GAP: new binder forms need their own row; AC-3
// removes the gate to show these existing rows were checking its behavior.
macro_rules! binder_row {
    ($test_name:ident, $source:expr, $binder:expr, $start:expr, $end:expr) => {
        binder_row!($test_name, $source, $binder, $start, $end, strict_roots);
    };
    ($test_name:ident, $source:expr, $binder:expr, $start:expr, $end:expr, $route:ident) => {
        #[test]
        fn $test_name() {
            let source = $source;
            let mut legacy = ElabEnv::new().expect("checked prelude");
            legacy
                .elaborate_file_v1(source)
                .unwrap_or_else(|error| panic!("Legacy source changed: {error:?}"));
            let error = match $route(source) {
                Err(error) => error,
                Ok(_) => panic!("Strict must refuse the built-in binder"),
            };
            assert_eq!(error.code().as_str(), "E0089");
            assert!(matches!(&error,
                ElabError::BuiltinNameBinding { name, span }
                if name == $binder && *span == Span::new($start, $end)),
                "wrong Strict binder refusal: {error:?}");
        }
    };
}

binder_row!(
    view_parameter,
    "fn f (pair_fst : Bool) : Bool = pair_fst",
    "pair_fst",
    5,
    22
);
binder_row!(
    lambda_first_name,
    "const f : Bool -> Bool = λpair_fst. pair_fst",
    "pair_fst",
    25,
    45
);
binder_row!(
    lambda_second_name,
    "const f : Bool -> Bool -> Bool = λx pair_fst. pair_fst",
    "pair_fst",
    33,
    55
);
binder_row!(
    sequential_let,
    "const f : Bool = let pair_fst = True in pair_fst",
    "pair_fst",
    21,
    29
);
binder_row!(
    expression_pi,
    "const F : Type = (pair_fst : Bool) -> Bool",
    "pair_fst",
    17,
    42
);
binder_row!(
    expression_sigma,
    "const F : Type = (pair_fst : Bool) × Bool",
    "pair_fst",
    17,
    42
);
binder_row!(
    type_pi,
    "fn f : (pair_fst : Bool) -> Bool = λx. x",
    "pair_fst",
    7,
    32
);
binder_row!(
    type_sigma,
    "const f : (pair_fst : Bool) × Bool = (True, True)",
    "pair_fst",
    10,
    35
);
binder_row!(
    type_refinement,
    "fn f (x : {pair_fst : Nat | Top}) : Nat = x",
    "pair_fst",
    10,
    32
);
binder_row!(
    constructor_pattern_variable,
    "data Box = MkBox Bool\nfn f (b : Box) : Bool = match b { MkBox pair_fst |-> pair_fst }",
    "pair_fst",
    62,
    70
);
binder_row!(
    pattern_as_alias,
    "fn f (b : Bool) : Bool = match b { True as pair_fst |-> pair_fst ; False |-> False }",
    "pair_fst",
    35,
    51
);
binder_row!(
    or_pattern_as_alias,
    "fn f (b : Bool) : Bool = match b { (True | False) as pair_fst |-> pair_fst }",
    "pair_fst",
    35,
    61
);
binder_row!(or_pattern_variable,
    "data Box = MkBox Bool\nfn f (b : Box) : Bool = match b { MkBox (pair_fst | pair_fst) |-> pair_fst }",
    "pair_fst", 63, 71);
binder_row!(
    match_equation,
    "fn f (b : Bool) : Bool = match b eqn: pair_fst { True |-> True ; False |-> False }",
    "pair_fst",
    25,
    82
);
binder_row!(
    simple_data_parameter,
    "data Box pair_fst = MkBox pair_fst",
    "pair_fst",
    0,
    34
);
binder_row!(
    explicit_data_parameter,
    "data Box (pair_fst : Type) : Type where { MkBox : pair_fst -> Box pair_fst }",
    "pair_fst",
    9,
    26
);
binder_row!(
    explicit_constructor_parameter,
    "data Holder : Type where { MkHolder : (pair_fst : Bool) -> Holder }",
    "pair_fst",
    38,
    55
);
binder_row!(
    explicit_family_index,
    "data Flag : (pair_fst : Nat) -> Type where { MkFlag : Flag Zero }",
    "pair_fst",
    12,
    36
);
binder_row!(
    proposition_parameter,
    "prop P (pair_fst : Nat) : Omega where { intro : P pair_fst }",
    "pair_fst",
    7,
    23
);
binder_row!(
    theorem_parameter,
    "theorem taut (pair_fst : Bool) : Top = Proved",
    "pair_fst",
    13,
    30
);
binder_row!(
    record_field,
    "record Rec { pair_fst : Bool }",
    "pair_fst",
    0,
    30
);
binder_row!(
    class_method,
    "class LocalClass a { pair_fst : a -> a }",
    "pair_fst",
    0,
    40
);
binder_row!(
    class_parameter,
    "class LocalClass pair_fst { keep : pair_fst -> pair_fst }",
    "pair_fst",
    0,
    57
);
binder_row!(
    law_parameter,
    "law LocalLaw (Proved) { keep : Top }",
    "Proved",
    0,
    36
);
binder_row!(where_dictionary,
    "class Flag a { tag : Bool }\ninstance Flag Nat { tag = True }\nconst witness : Bool where (pair_fst : Flag Nat) = pair_fst.tag",
    "pair_fst", 61, 124);
binder_row!(instance_dictionary,
    "class Flag a { tag : Bool }\nclass Holder a { selected : Bool }\ninstance Flag Nat { tag = True }\ninstance Holder Nat where (pair_fst : Flag Nat) { selected = pair_fst.tag }",
    "pair_fst", 96, 171);
// A file-backed Strict roots unit refuses a space before resolution as nested
// placement. Isolated Strict is the valid space route to this same Scope gate.
binder_row!(
    space_cell,
    "space LocalSpace { mut pair_fst : Int = 0 }",
    "pair_fst",
    19,
    41,
    strict_isolated
);
binder_row!(space_operation_parameter,
    "space LocalSpace { mut n : Int = 0 proc get (pair_fst : Int) : Int visits [LocalSpace] = pair_fst }",
    "pair_fst", 44, 60, strict_isolated);

#[test]
fn non_builtin_local_shadows_source_declared_name_under_strict() {
    let source = "const x : Bool = False\nfn f (x : Bool) : Bool = x";
    ElabEnv::new()
        .unwrap()
        .elaborate_isolated_file_strict_v1(&[], source)
        .expect("non-B lexical shadowing remains available");
}

#[test]
fn qualified_floor_use_is_not_a_binding() {
    ElabEnv::new()
        .unwrap()
        .elaborate_isolated_file_strict_v1(&[], "fn f (x : Bool) : Bool = Bool.True")
        .expect("qualified constructor use does not introduce a B name");
}

fn strict_roots(source: &str) -> Result<ElabEnv, ElabError> {
    let root = tempfile::tempdir().expect("Strict root");
    fs::write(root.path().join("Entry.ken"), source).expect("write source");
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")?;
    Ok(env)
}

fn strict_isolated(source: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_isolated_file_strict_v1(&[], source)?;
    Ok(env)
}

// Promise class: durable invariant (spec 33 §3.3, module-scope B refusal).
// MEASURED: each mode refuses the law's own B name Pair with E0058, while
// its derived Pair_fst is outside B. CLAIMED: the new LawDerivedNames arm
// retains the prior own-name refusal. THE GAP: a derived-B-only test cannot
// see a removed own-name check; removing it must redden both named rows.
fn assert_law_own_builtin_name(error: &ElabError, source: &str, local_origin: &str) {
    assert_eq!(error.code().as_str(), "E0058", "{error:?}");
    assert!(
        matches!(error,
        ElabError::AmbiguousReference { name, sources, span }
        if name == "Pair"
            && sources.iter().map(String::as_str).eq(["<prelude>.Pair", local_origin])
            && *span == Span::new(0, source.len())),
        "law's own B name must refuse before derived name: {error:?}"
    );
}

#[test]
fn law_own_builtin_name_refuses_before_non_builtin_derived_name_strict() {
    let source = "law Pair (x) { fst : Top }";
    let error = match strict_roots(source) {
        Err(error) => error,
        Ok(_) => panic!("Strict own-name B introduction must refuse"),
    };
    assert_law_own_builtin_name(&error, source, "Entry.Pair");
}

#[test]
fn law_own_builtin_name_retains_legacy_e0058() {
    let source = "law Pair (x) { fst : Top }";
    let error = ElabEnv::new()
        .unwrap()
        .elaborate_file_v1(source)
        .expect_err("Legacy own-name B introduction must keep its E0058 refusal");
    assert_law_own_builtin_name(&error, source, "Pair");
}

// Promise class: durable invariant. The refusing law would bind the exact
// prelude pair_fst through its derived identity if the module prebind guard
// ignored derived names; the outside-B law checks through the same route.
#[test]
fn law_derived_builtin_name_refuses_at_module_prebind() {
    let source = "law pair (x) { fst : Top }";
    let error = match strict_roots(source) {
        Err(error) => error,
        Ok(_) => panic!("law-derived pair_fst must be refused"),
    };
    assert_eq!(error.code().as_str(), "E0058");
    assert!(
        matches!(&error,
        ElabError::AmbiguousReference { name, sources, span }
        if name == "pair_fst"
            && sources.iter().map(String::as_str).eq(["<prelude>.pair_fst", "Entry.pair_fst"])
            && *span == Span::new(0, source.len())),
        "must use existing E0058 module-scope refusal, got {error:?}"
    );
}

#[test]
fn law_derived_outside_builtin_names_remains_accepted() {
    strict_roots("law local_pair (x) { fst : Top }").expect("non-B derived law identity is lawful");
}

#[test]
fn legacy_law_derived_collision_keeps_its_prior_error() {
    let error = ElabEnv::new()
        .unwrap()
        .elaborate_file_v1("law pair (x) { fst : Top }")
        .expect_err("Legacy collision still refuses through its identity check");
    assert!(
        matches!(error,
        ElabError::DeclarationIdentityCollision { identity, .. }
        if identity == "pair_fst"),
        "Strict E0058 must not change Legacy collision diagnostics"
    );
}
