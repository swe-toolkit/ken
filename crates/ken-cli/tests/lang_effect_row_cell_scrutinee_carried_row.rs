//! Spec 36 §§1.1–1.4, 2.4; CELL-SCRUTINEE Architect AC-0 and AC-0b.
//! Promise class: durable behavioral invariant. MEASURED: effect-carrying
//! space cells (including nonzero cell indices), checked scrutinee types and
//! space-operation parameters release their callback row before a match or
//! let can discard it; pure/tree controls remain executable. CLAIMED: every
//! reachable cell-carried callback is
//! accounted for once, and a non-tree action is refused at check time. THE GAP:
//! operator forms without a constructible row-bearing result are reviewed
//! structurally; a declaration's registered body footprint over-approximates
//! its results (Pick Off is charged Box's row), and an arrow-typed lambda
//! binder whose arrow is written inline, not through an alias, is a sibling, not
//! guarded here. LET-BOUND's named binary already pins q02, q11 and q15.
//! Inline sources avoid adding files to the strict-isolated .ken census.

use std::process::{Command, Output};

const PRE: &str = r#"program capabilities FS APartial
proc action (x : Unit) : HostIO APartial Unit visits [Console] =
  host_bind APartial (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write ConsoleStream.Stdout (bytes_encode "a")))
    (\_. host_pure APartial Unit MkUnit)
fn quiet (x : Unit) : HostIO APartial Unit = host_pure APartial Unit MkUnit
data Box = MkBox (Unit ->[Console] HostIO APartial Unit)
data PBox = MkPBox (Unit -> HostIO APartial Unit)
data Flag = On | Off
record Slot { bx : Box }
def Callback = Unit ->[Console] HostIO APartial Unit
data Bag (a : Type) : Type where { Empty : Bag a ; One : a -> Bag a ; Join : Bag a -> Bag a -> Bag a }
data Rose = Leaf | Node (Bag Rose)
"#;
const TWO: &str = "data Two = TA (Unit ->[Console] HostIO APartial Unit) | TB (Unit ->[Console] HostIO APartial Unit)\n";
const MAIN_B: &str = r#"proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "b")
"#;
const CELL: &str = "mut held : Box = MkBox action";
const TWO_CELLS: &str = "mut idle : PBox = MkPBox quiet\n  mut held : Box = MkBox action";
const FIRE: &str = "proc fire () : Unit visits [Holder, Console] =";
const USE_CALLBACK: &str = "MkBox f ↦ let ignored = f MkUnit in MkUnit";
const ROSE: &str = "data Rose = Leaf | Node (Bag Rose)\n";
const PICK: &str = "fn Pick (f : Flag) : Type = match f { On ↦ Box ; Off ↦ PBox }\n";

const ALIAS: &str = "const AliasType : Type 1 = Type 0
fn PickAlias (f : Flag) : AliasType = match f { On ↦ Box ; Off ↦ PBox }
const AliasBox : AliasType = Box
const AliasPBox : AliasType = PBox
";
const LET_ALIAS: &str = "const AliasType : Type 1 = Type 0
let LetAliasBox : AliasType = Box
let LetAliasPBox : AliasType = PBox
";

/// Declares the type-valued fn Pick after the shared prelude.
fn with_pick(source: String) -> String {
    source.replacen(ROSE, &format!("{ROSE}{PICK}"), 1)
}

/// Declares type-valued declarations whose result type is an alias of a
/// universe, so no written codomain is `Type`.
fn with_alias(source: String) -> String {
    source.replacen(ROSE, &format!("{ROSE}{ALIAS}"), 1)
}

/// Declares top-level lets with an aliased universe result type, keeping the
/// type spelling and parameter read identical to the const controls.
fn with_let_alias(source: String) -> String {
    source.replacen(ROSE, &format!("{ROSE}{LET_ALIAS}"), 1)
}

fn space(cell: &str, signature: &str, body: &str, two: bool) -> String {
    format!(
        "{PRE}{}space Holder {{\n  {cell}\n  {signature}\n{body}\n}}\n{MAIN_B}",
        if two { TWO } else { "" }
    )
}

fn top_pick(recursive: bool) -> String {
    let mut source = PRE.to_owned();
    source.push_str(if recursive {
        r#"proc pick (r : Rose) : Box visits [Console] = match r {
  Leaf ↦ MkBox action ;
  Node b ↦ match b {
    Empty ↦ MkBox action ;
    One x ↦ pick x ;
    Join xs ys ↦ match (recursive result for xs) { MkBox f ↦ let ignored = f MkUnit in MkBox f } } }
"#
    } else {
        r#"proc pick (r : Rose) : Box visits [Console] = match r {
  Leaf ↦ MkBox action ;
  Node b ↦ match b {
    Empty ↦ MkBox action ;
    One x ↦ pick x ;
    Join xs ys ↦ match MkBox action { MkBox f ↦ let ignored = f MkUnit in MkBox f } } }
"#
    });
    source.push_str(MAIN_B);
    source
}

fn source_q17_annotated_cell() -> String {
    space(
        CELL,
        FIRE,
        "    let b : Box = held in\n    match b { MkBox f ↦ let ignored = f MkUnit in MkUnit }",
        false,
    )
}

fn source_q18_direct_constructor() -> String {
    let mut source = PRE.to_owned();
    source.push_str(
        r#"proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match MkBox action { MkBox f ↦ let ignored = f MkUnit in
    host_program APartial (print_line "b") }
"#,
    );
    source
}

#[derive(Clone, Copy)]
enum Observation {
    RefuseNonTree,
    RefuseFalsePurity,
    RefuseInclusion,
    RefuseTreeScrutinee,
    PrintsB,
    PrintsAB,
}

fn cli(name: &str, source: &str) -> Output {
    let dir = std::env::temp_dir().join(format!("ken-cell-row-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("fixture directory");
    let path = dir.join(format!("{name}.ken"));
    std::fs::write(&path, source).expect("write inline Ken program");
    let output = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&path)
        .output()
        .expect("run Ken CLI");
    let _ = std::fs::remove_file(path);
    output
}

fn assert_observation(name: &str, source: &str, expected: Observation) {
    let output = cli(name, source);
    let stderr = String::from_utf8_lossy(&output.stderr);
    match expected {
        Observation::RefuseNonTree | Observation::RefuseFalsePurity => {
            assert_eq!(output.status.code(), Some(1), "{name}: {stderr}");
            assert!(output.stdout.is_empty(), "{name}: rejected program ran");
            assert!(stderr.starts_with("error[E0038]"), "{name}: {stderr}");
            let reason = match expected {
                Observation::RefuseNonTree => "cannot be sequenced (36 §2.4)",
                Observation::RefuseFalsePurity => "false purity or effect escape",
                _ => unreachable!(),
            };
            assert!(stderr.contains(reason), "{name}: {stderr}");
        }
        Observation::RefuseInclusion => {
            assert_eq!(output.status.code(), Some(1), "{name}: {stderr}");
            assert!(output.stdout.is_empty(), "{name}: rejected program ran");
            assert!(stderr.starts_with("error[E0087]"), "{name}: {stderr}");
            assert!(
                stderr.contains("UnsupportedEffectInclusion"),
                "{name}: {stderr}"
            );
        }
        Observation::RefuseTreeScrutinee => {
            assert_eq!(output.status.code(), Some(1), "{name}: {stderr}");
            assert!(output.stdout.is_empty(), "{name}: rejected program ran");
            assert!(stderr.starts_with("error[E0038]"), "{name}: {stderr}");
            assert!(stderr.contains("TypeMismatch"), "{name}: {stderr}");
            assert!(
                stderr.contains("constructor 'MkBox' is not a constructor of type 'ITree'"),
                "{name}: {stderr}"
            );
        }
        Observation::PrintsB | Observation::PrintsAB => {
            assert_eq!(output.status.code(), Some(0), "{name}: {stderr}");
            let bytes = if matches!(expected, Observation::PrintsAB) {
                b"ab\n".as_slice()
            } else {
                b"b\n".as_slice()
            };
            assert_eq!(output.stdout, bytes, "{name}: missing or doubled action");
        }
    }
}

macro_rules! pin {
    ($test:ident, $source:expr, $result:ident) => {
        #[test]
        fn $test() {
            assert_observation(stringify!($test), &$source, Observation::$result);
        }
    };
}

pin!(
    c01_direct_cell,
    space(
        CELL,
        FIRE,
        &format!("    match held {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(c02_pair_let, space(CELL, FIRE, "    let p = (held, MkUnit) in\n    match p { (MkBox f, _) ↦ let ignored = f MkUnit in MkUnit }", false), RefuseNonTree);
pin!(
    c03_pair_direct,
    space(
        CELL,
        FIRE,
        "    match (held, MkUnit) { (MkBox f, _) ↦ let ignored = f MkUnit in MkUnit }",
        false
    ),
    RefuseNonTree
);
pin!(
    c04_if,
    space(
        CELL,
        FIRE,
        &format!("    match (if True then held else held) {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c05_match,
    space(
        CELL,
        FIRE,
        &format!("    match (match On {{ On ↦ held ; Off ↦ held }}) {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c06_nested_let,
    space(
        CELL,
        FIRE,
        &format!("    match (let c = held in c) {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c07_record_projection,
    space(
        CELL,
        FIRE,
        &format!("    match ({{ bx = held }} : Slot).bx {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c08_application,
    space(
        CELL,
        FIRE,
        &format!(r"    match (((\x. x) : Box -> Box) held) {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c09_arm_extracted_callback,
    space(
        CELL,
        FIRE,
        "    let k = match held { MkBox f ↦ f } in\n    let ignored = k MkUnit in MkUnit",
        false
    ),
    RefuseNonTree
);
// Pair on the same two-cell declaration: selection must use the RCell index,
// not the first cell's row. The c09 path isolates the pre-read of the callback
// extracted from the second cell; the pure first cell is the same-form control.
pin!(
    c09_second_cell_arm_extracted_callback,
    space(
        TWO_CELLS,
        FIRE,
        "    let k = match held { MkBox f ↦ f } in\n    let ignored = k MkUnit in MkUnit",
        false
    ),
    RefuseNonTree
);
pin!(
    k14_first_cell_pure_before_carried_second,
    space(
        TWO_CELLS,
        FIRE,
        "    let k = match idle { MkPBox f ↦ f } in\n    let ignored = k MkUnit in MkUnit",
        false
    ),
    PrintsB
);
pin!(
    c10_lambda_binder_cell,
    space(
        CELL,
        FIRE,
        &format!(r"    ((\b. match b {{ {USE_CALLBACK} }}) : Box -> Unit) held"),
        false
    ),
    RefuseNonTree
);
pin!(
    c11_position_projection,
    space(
        CELL,
        FIRE,
        &format!("    match (held, MkUnit).1 {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    c12_pair_let_projection,
    space(
        CELL,
        FIRE,
        &format!("    let p = (held, MkUnit) in\n    match p.1 {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);

pin!(
    k01_pure_cell,
    space(
        "mut held : PBox = MkPBox quiet",
        FIRE,
        "    match held { MkPBox f ↦ let ignored = f MkUnit in MkUnit }",
        false
    ),
    PrintsB
);
pin!(
    k02_constructor_pair,
    {
        let mut s = PRE.to_owned();
        s.push_str("proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)\n  : HostIO APartial ExitCode visits [Console] =\n  let p = (MkBox action, MkUnit) in\n  match p { (MkBox f, _) ↦ let ignored = f MkUnit in\n    host_program APartial (print_line \"b\") }\n");
        s
    },
    PrintsAB
);
pin!(
    k10_lambda_constructor,
    space(
        CELL,
        FIRE,
        &format!(r"    ((\b. match b {{ {USE_CALLBACK} }}) : Box -> Unit) (MkBox action)"),
        false
    ),
    RefuseNonTree
);
pin!(
    k12_pure_parameter,
    space(
        CELL,
        "proc fire (n : Nat) : Unit visits [Holder] =",
        "    let m = n in MkUnit",
        false
    ),
    PrintsB
);
pin!(
    k13_pure_or_pattern,
    space(
        CELL,
        "proc fire (b : Two) : Unit visits [Holder, Console] =",
        "    match b { TA f | TB f ↦ MkUnit }",
        true
    ),
    PrintsB
);

pin!(
    p0_unused_cell_read,
    space(
        CELL,
        "proc peek () : Unit visits [Holder] =",
        "    let b = held in MkUnit",
        false
    ),
    RefuseFalsePurity
);
pin!(
    p1_unused_parameter_read,
    space(
        CELL,
        "proc peek (b : Box) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    ),
    RefuseFalsePurity
);
pin!(
    p2_space_parameter_match,
    space(
        CELL,
        "proc peek (b : Box) : Unit visits [Holder, Console] =",
        &format!("    match b {{ {USE_CALLBACK} }}"),
        false
    ),
    RefuseNonTree
);
pin!(
    p3_top_parameter_match,
    {
        let mut s = PRE.to_owned();
        s.push_str(&format!(
            "proc g (b : Box) : Unit visits [Console] = match b {{ {USE_CALLBACK} }}\n"
        ));
        s.push_str(MAIN_B);
        s
    },
    RefuseInclusion
);
pin!(
    p4_space_arrow_parameter,
    space(
        CELL,
        "proc fire (k : Callback) : Unit visits [Holder, Console] =",
        "    let ignored = k MkUnit in MkUnit",
        false
    ),
    RefuseNonTree
);

// Recursive results now sequence through the let/scrutinee slots (36 §2.4).
// Their arm still invokes a HostIO callback inside an IO tree, so both row
// shapes refuse the unsupported inclusion (E0087), as r1k does.
pin!(r1_recursive_result, top_pick(true), RefuseInclusion);
pin!(
    r1f_recursive_result_fs_row,
    {
        let source = top_pick(true);
        let fs = source.replacen(
            "visits [Console] = match r",
            "visits [FS, Console] = match r",
            1,
        );
        assert_ne!(fs, source, "the recursive pick's row moved");
        fs
    },
    RefuseInclusion
);
pin!(r1k_recursive_constructor, top_pick(false), RefuseInclusion);
pin!(
    t1_tree_cell,
    space(
        "mut act : HostIO APartial Unit = action MkUnit",
        "proc peek () : Unit visits [Holder] =",
        "    let t = act in MkUnit",
        false
    ),
    PrintsB
);

pin!(
    s1_or_pattern_parameter,
    space(
        CELL,
        "proc fire (b : Two) : Unit visits [Holder, Console] =",
        "    match b { TA f | TB f ↦ let ignored = f MkUnit in MkUnit }",
        true
    ),
    RefuseNonTree
);
pin!(
    s2_or_pattern_lambda,
    space(
        CELL,
        FIRE,
        r"    ((\b. match b { TA f | TB f ↦ let ignored = f MkUnit in MkUnit }) : Two -> Unit) (TA action)",
        true
    ),
    RefuseNonTree
);
pin!(
    s3_inferred_match_parameter,
    space(
        CELL,
        "proc fire (b : Box) : Unit visits [Holder, Console] =",
        &format!("    let r = match b {{ {USE_CALLBACK} }} in r"),
        true
    ),
    RefuseNonTree
);
pin!(
    s4_inferred_match_lambda,
    space(
        CELL,
        FIRE,
        &format!(
            r"    ((\b. let r = match b {{ {USE_CALLBACK} }} in r) : Box -> Unit) (MkBox action)"
        ),
        true
    ),
    RefuseNonTree
);
pin!(
    x1_inferred_match_unused,
    space(
        CELL,
        FIRE,
        &format!(
            r"    ((\b. let r = match b {{ {USE_CALLBACK} }} in MkUnit) : Box -> Unit) (MkBox action)"
        ),
        true
    ),
    RefuseNonTree
);

// The alias Callback is registered with its arrow's latent row; reducing
// it to the kernel Pi would erase that row.
pin!(
    k11_lambda_arrow_alias,
    space(
        CELL,
        FIRE,
        r"    ((\k. let ignored = k MkUnit in MkUnit) : Callback -> Unit) action",
        false
    ),
    RefuseNonTree
);
// A binder typed by a type-valued fn and then matched is refused through
// the scrutinee join on the dispatched type, with or without Pick's
// registration.
pin!(
    c13_lambda_binder_type_function,
    with_pick(space(
        CELL,
        FIRE,
        &format!(r"    ((\b. match b {{ {USE_CALLBACK} }}) : Pick On -> Unit) held"),
        false
    )),
    RefuseNonTree
);
pin!(
    c14_let_type_function,
    with_pick(space(
        CELL,
        FIRE,
        &format!("    let c : Pick On = MkBox action in\n    match c {{ {USE_CALLBACK} }}"),
        false
    )),
    RefuseNonTree
);

// No match reaches these two: only Pick's registered footprint, read
// without reducing `Pick On`, charges Console to a proc that does not visit
// it. The PBox twin is the same form over a type with no row.
pin!(
    c15_parameter_let_type_function,
    with_pick(space(
        CELL,
        "proc peek (b : Pick On) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    RefuseFalsePurity
);
pin!(
    c16_lambda_argument_type_function,
    with_pick(space(
        CELL,
        "proc peek (b : Pick On) : Unit visits [Holder] =",
        r"    ((\x. MkUnit) : Pick On -> Unit) b",
        false
    )),
    RefuseFalsePurity
);
// The same reads through declarations whose result type is the alias
// AliasType, not a written universe. Registration does not classify the
// codomain, so these charge Console like c15; the PBox twin has no row.
pin!(
    c17_alias_codomain_type_function,
    with_alias(space(
        CELL,
        "proc peek (b : PickAlias On) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    RefuseFalsePurity
);
pin!(
    c18_alias_codomain_const,
    with_alias(space(
        CELL,
        "proc peek (b : AliasBox) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    RefuseFalsePurity
);
pin!(
    k16_alias_codomain_pure_const,
    with_alias(space(
        CELL,
        "proc peek (b : AliasPBox) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    PrintsB
);
// A top-level let can also name a carried type. The row-bearing read must
// charge Console even without a match; the same-form pure let must print b.
// Removing only RDeclKind::Let from registration distinguishes this pair.
pin!(
    c19_alias_codomain_top_level_let,
    with_let_alias(space(
        CELL,
        "proc peek (b : LetAliasBox) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    RefuseFalsePurity
);
pin!(
    k17_alias_codomain_pure_top_level_let,
    with_let_alias(space(
        CELL,
        "proc peek (b : LetAliasPBox) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    )),
    PrintsB
);
pin!(
    k15_pure_parameter_let,
    space(
        CELL,
        "proc peek (b : PBox) : Unit visits [Holder] =",
        "    let c = b in MkUnit",
        false
    ),
    PrintsB
);

pin!(
    q17_annotated_cell,
    source_q17_annotated_cell(),
    RefuseNonTree
);
pin!(
    q18_direct_constructor,
    source_q18_direct_constructor(),
    PrintsAB
);
