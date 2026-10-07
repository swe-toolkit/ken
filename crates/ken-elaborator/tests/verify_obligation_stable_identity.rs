//! Stable obligation and hole identity at the checked-core package boundary.
//! Source-order controls vary only the allocation history, never the two files.

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    CompilerManifest, CompilerSource, CompilerTargetKind, TargetSelector,
    compile_ken_package_sources,
};
use ken_elaborator::{ElabEnv, v2_extract};

const PKG: &str = "verify_stable_identity_ac0";
const TARGET: &str = r#"program capabilities FS APartial
const ac0_need : String requires Equal Int 0 0 = "ac0-run"
const ac0_use : String = ac0_need
proc main
      (_input : ProcessInput) (_caps : ProgramCaps APartial)
    : HostIO APartial ExitCode
    visits [Console] =
  host_program APartial (print_line ac0_use)
"#;
const UNRELATED: &str = "const unrelated_stable_shift : Nat = Suc (Suc Zero)\n";
const HOLE: &str = "ac0_use#0";
const OBLIGATION: &str = "ac0_use.requires.0";

fn target() -> CompilerSource {
    CompilerSource::new("src/main.ken", TARGET)
}

fn extra() -> CompilerSource {
    CompilerSource::new("src/extra.ken", UNRELATED)
}

fn decl(name: &str) -> StableSymbol {
    StableSymbol::declaration(PKG, &[], name)
}

fn compile(sources: Vec<CompilerSource>) -> CheckedCorePackage {
    compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        sources,
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: decl("main"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("source admits and emits a checked package")
    .package
}

fn assert_open_requires(package: &CheckedCorePackage) {
    let semantic = &package.artifact.semantic;
    let obligation = StableSymbol::obligation(OBLIGATION);
    let hole = decl(HOLE);
    assert!(semantic.obligations.contains_key(&obligation));
    assert!(semantic.symbols.contains(&hole));
    assert!(semantic.trusted_base_delta.contains_key(&hole));
    assert_eq!(
        semantic.obligation_metadata[&obligation].origin,
        decl("ac0_use")
    );
}

/// Promise class: durable invariant. MEASURED: real package semantic inputs,
/// obligation id and hole symbol for one fixed two-file declaration set in
/// both elaboration orders. CLAIMED: producer-local allocation cannot affect
/// package meaning. THE GAP: a different corpus legitimately has a different
/// package hash; the alone-vs-two-file row below compares only target entries.
#[test]
fn reordered_files_keep_package_semantics_and_hole_identity() {
    let first = compile(vec![extra(), target()]);
    let second = compile(vec![target(), extra()]);
    assert_open_requires(&first);
    assert_open_requires(&second);
    assert_eq!(first.artifact.semantic, second.artifact.semantic);
    assert_eq!(first.core_semantic_hash, second.core_semantic_hash);
}

/// Promise class: durable invariant. MEASURED: same target source alone and
/// after an unrelated source has byte-identical target declarations, obligation
/// entry, hole symbol and trust entry. CLAIMED: inserting a different checked
/// declaration before this file does not change its own semantic identity.
/// THE GAP: this does not compare whole-package hashes, which include the
/// extra declaration by contract.
#[test]
fn unrelated_predecessor_keeps_target_declaration_entries() {
    let alone = compile(vec![target()]);
    let after = compile(vec![extra(), target()]);
    assert_open_requires(&alone);
    assert_open_requires(&after);
    let a = &alone.artifact.semantic;
    let b = &after.artifact.semantic;
    for name in ["ac0_need", "ac0_use", "main"] {
        let symbol = decl(name);
        assert_eq!(a.declarations.get(&symbol), b.declarations.get(&symbol));
    }
    let obligation = StableSymbol::obligation(OBLIGATION);
    let hole = decl(HOLE);
    assert_eq!(
        a.obligations.get(&obligation),
        b.obligations.get(&obligation)
    );
    assert_eq!(
        a.obligation_metadata.get(&obligation),
        b.obligation_metadata.get(&obligation)
    );
    assert_eq!(
        a.trusted_base_delta.get(&hole),
        b.trusted_base_delta.get(&hole)
    );
    assert!(a.symbols.contains(&hole) && b.symbols.contains(&hole));
}

/// Promise class: durable invariant. MEASURED: two open call sites within one
/// declaration yield two distinct obligation IDs and two stable hole symbols.
/// CLAIMED: owner-local ordinals distinguish holes without a session counter.
/// THE GAP: the test fixes one declaration shape, not every possible site.
#[test]
fn two_open_calls_in_one_owner_have_distinct_stable_ids() {
    let source = "const need : String requires Equal Int 0 0 = \"run\"\n\
                  const use_twice : String = let first : String = need in need\n";
    let mut env = ElabEnv::new().expect("prelude");
    let results = env.elaborate_file_v1(source).expect("both calls elaborate");
    let owner = results
        .iter()
        .find(|result| result.name == "use_twice")
        .unwrap();
    let obligations = v2_extract(owner).obligations;
    assert_eq!(obligations.len(), 2);
    assert_ne!(obligations[0].id, obligations[1].id);
    assert_ne!(obligations[0].hole_id, obligations[1].hole_id);
    assert_eq!(obligations[0].id.0, "use_twice.requires.0");
    assert_eq!(obligations[1].id.0, "use_twice.requires.1");

    let package = compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        vec![CompilerSource::new("src/two.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: decl("use_twice"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("both holes reach the package")
    .package;
    let semantic = &package.artifact.semantic;
    for (index, id) in ["use_twice.requires.0", "use_twice.requires.1"]
        .into_iter()
        .enumerate()
    {
        assert!(
            semantic
                .obligations
                .contains_key(&StableSymbol::obligation(id))
        );
        assert!(
            semantic
                .trusted_base_delta
                .contains_key(&decl(&format!("use_twice#{index}")))
        );
    }
}

/// Promise class: durable invariant. MEASURED: two members of a real recursive
/// SCC each allocate an unnamed string literal under their OWN qualified
/// declaration name. CLAIMED: shared SCC staging cannot collapse unrelated
/// member ownership into a single group/session key. THE GAP: this reaches
/// both body passes but not every possible multi-phase type signature.
#[test]
fn mutual_members_own_their_literal_allocations_separately() {
    let source = "fn isEven (n : Nat) : String = match n { Zero |-> \"even\" ; Suc m |-> isOdd m }\n\
                  fn isOdd (n : Nat) : String = match n { Zero |-> \"odd\" ; Suc m |-> isEven m }\n";
    let package = compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        vec![CompilerSource::new("src/mutual.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: decl("isEven"),
            kind: CompilerTargetKind::NonRuntime,
        },
    )
    .expect("mutually recursive literal-bearing declarations elaborate")
    .package;
    let symbols = &package.artifact.semantic.symbols;
    assert!(symbols.contains(&decl("isEven#0")));
    assert!(symbols.contains(&decl("isOdd#0")));
}

/// Promise class: durable invariant. MEASURED: the same obligation-free source
/// compiled twice carries the same hash and no obligations. CLAIMED: identity
/// stabilization cannot inject obligations into unrelated files. THE GAP:
/// this controls determinism, not the legacy numeric hash pinned elsewhere.
#[test]
fn obligation_free_source_preserves_its_semantic_hash() {
    let source = || CompilerSource::new("src/plain.ken", "const main : Bool = True\n");
    let first = compile(vec![source()]);
    let second = compile(vec![source()]);
    assert!(first.artifact.semantic.obligations.is_empty());
    assert_eq!(first.core_semantic_hash, second.core_semantic_hash);
}
