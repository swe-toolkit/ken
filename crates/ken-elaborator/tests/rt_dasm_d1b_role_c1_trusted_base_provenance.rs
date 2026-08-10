//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — the pre-source trusted-base
//! roster, and the admission it feeds.
//!
//! **The property.** Nothing inspectable in a finished package distinguishes a
//! prelude postulate from one the package's own source introduced with the same
//! shape and the same canonical identity. Only *when* it entered `Σ` separates
//! them. The roster is the only record of that instant, so native admission can
//! ask "was this trusted before the package could speak?" and get an answer.
//!
//! ⛔ Every assertion here is a **set relation**. No count is pinned: the
//! prelude's size is not a contract, and freezing it would make this file red on
//! any unrelated prelude change while proving nothing extra.

use std::collections::BTreeSet;

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerManifest, CompilerSource, CompilerTargetKind,
    TargetSelector,
};
use ken_elaborator::erasure::erase_checked_core_package_for_target;
use ken_elaborator::ElabEnv;
use ken_runtime::RuntimeProgram;

const PKG: &str = "c1_provenance_pkg";

/// The clean carrier: no user-introduced trust of any kind.
const CLEAN_SOURCE: &str = "const addTwoThree : Nat = Suc (Suc (Suc Zero))";

/// The same carrier plus exactly one user-introduced trusted-base entry.
///
/// ⛔ Deliberately **without** `ensures`: an `ensures` clause mints a second
/// trusted runtime-check postulate, which would make "exactly one new entry"
/// false and turn a singleton assertion into a vague one.
const USER_TRUST_SOURCE: &str =
    "foreign c1_user_trust : Int -> Int = \"c1_user_trust\" \"fixture\" pure\n\
     const addTwoThree : Nat = Suc (Suc (Suc Zero))";

fn erased(source: &str) -> RuntimeProgram {
    let out = compile_ken_package_sources(
        &CompilerManifest::new(PKG, Vec::new()),
        vec![CompilerSource::new("src/main.ken", source)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(SymbolNamespace::Module, vec![PKG.to_string()]),
            symbol: StableSymbol::declaration(PKG, &[], "addTwoThree"),
            kind: CompilerTargetKind::Executable,
        },
    )
    .expect("the carrier source emits a checked-core package");
    let closure = out.closures.first().expect("selected target closure");
    erase_checked_core_package_for_target(&out.package, closure.reachable_declarations.iter())
        .expect("the carrier package erases")
}

/// The trust targets a package actually claims.
fn tuple_targets(program: &RuntimeProgram) -> BTreeSet<String> {
    program
        .erased_core
        .metadata
        .assumption_trust_metadata
        .values()
        .map(|trust| trust.target.clone())
        .collect()
}

fn roster(program: &RuntimeProgram) -> BTreeSet<String> {
    program
        .erased_core
        .metadata
        .checked_core
        .native_trusted_base
        .as_ref()
        .expect("a driver-compiled package carries the pre-source roster")
        .targets
        .clone()
}

/// PRODUCER CONTROL — the roster is captured before source, and stays put.
///
/// This is the half that cannot be observed from a finished package: it watches
/// the live trusted base move while the immutable roster does not.
#[test]
fn c1_the_roster_is_closed_before_source_and_one_foreign_adds_exactly_one_live_entry() {
    let mut elab = ElabEnv::empty().expect("the elaborator environment builds");

    let captured = elab.prelude_env.native_trusted_base.clone();
    let live_before: BTreeSet<_> = elab.env.trusted_base().into_iter().collect();

    // Every compiler declaration precedes the capture, and none follows it: at
    // the moment `ElabEnv::empty()` returns, the captured roster IS the live
    // trusted base. A later compiler initializer that declared a trusted entry
    // would break this equality -- which is exactly the regression it guards.
    assert_eq!(
        captured, live_before,
        "the captured roster and the live trusted base disagree at the moment the environment \
         becomes usable; a compiler stage is declaring trusted entries after the capture"
    );
    assert!(
        !captured.is_empty(),
        "the roster is empty, so every set relation below would hold vacuously"
    );

    elab.elaborate_file(USER_TRUST_SOURCE)
        .expect("the foreign fixture elaborates through the normal surface path");

    let live_after: BTreeSet<_> = elab.env.trusted_base().into_iter().collect();

    // EXACTLY ONE new entry, and nothing removed.
    let added: Vec<_> = live_after.difference(&live_before).copied().collect();
    let removed: Vec<_> = live_before.difference(&live_after).copied().collect();
    assert_eq!(
        added.len(),
        1,
        "one `foreign` declaration must add exactly one trusted-base entry, got {added:?}"
    );
    assert!(
        removed.is_empty(),
        "elaborating a declaration removed trusted-base entries: {removed:?}"
    );

    // THE IMMUTABILITY HALF. Source elaboration moved the live set; the roster
    // must not have moved with it, or it would record the package's own claim
    // rather than what preceded it.
    assert_eq!(
        elab.prelude_env.native_trusted_base, captured,
        "elaborating package source mutated the captured roster; it is no longer a record of the \
         pre-source trusted base and cannot answer the provenance question"
    );
}

/// The clean carrier closes: roster and claimed targets are the same set.
#[test]
fn c1_a_clean_package_has_roster_equal_to_its_trust_targets() {
    let program = erased(CLEAN_SOURCE);
    let roster = roster(&program);
    let targets = tuple_targets(&program);

    assert_eq!(
        roster, targets,
        "a package with no user-introduced trust must claim exactly the pre-source roster; \
         claimed-not-in-roster={:?}, roster-not-claimed={:?}",
        targets.difference(&roster).collect::<Vec<_>>(),
        roster.difference(&targets).collect::<Vec<_>>()
    );
    assert!(
        !targets.is_empty(),
        "the clean carrier claims no trust at all, so the equality above is vacuous"
    );

    // And it is admitted.
    ken_runtime::native_program_admission(&program)
        .expect("a clean package-backed carrier is admitted to native compilation");
}

/// THE DISCRIMINATOR — one user `foreign` is roster-plus-one, and admission
/// refuses it **specifically** on the roster/tuple mismatch.
///
/// ⚠ Whole-pipeline rejection would NOT establish this. The general native
/// blockers refuse several things about such a package independently, so a test
/// that only observed "it was refused" would pass even if the roster check did
/// nothing. This exercises the trust closure **directly** and asserts the
/// diagnostic names the source-introduced target.
#[test]
fn c1_one_user_foreign_is_roster_plus_one_and_is_refused_on_the_roster_mismatch() {
    let clean = erased(CLEAN_SOURCE);
    let dirty = erased(USER_TRUST_SOURCE);

    let clean_targets = tuple_targets(&clean);
    let dirty_targets = tuple_targets(&dirty);
    let dirty_roster = roster(&dirty);

    // The roster is unchanged by the user's declaration -- it is a fact about
    // the prelude, not about this package's source.
    assert_eq!(
        roster(&clean),
        dirty_roster,
        "adding a user `foreign` changed the pre-source roster; the capture is not before source"
    );

    // Exactly one new claimed target, and it is the user's.
    let introduced: Vec<_> = dirty_targets.difference(&dirty_roster).cloned().collect();
    assert_eq!(
        introduced.len(),
        1,
        "one user `foreign` must add exactly one claimed trust target beyond the roster, got \
         {introduced:?}"
    );
    assert!(
        introduced[0].contains("c1_user_trust"),
        "the one target beyond the roster should be the user's foreign, got {}",
        introduced[0]
    );
    assert!(
        dirty_roster.difference(&dirty_targets).next().is_none(),
        "the user declaration should only ADD; the roster must remain a subset of the claims"
    );
    // The clean package is the positive control for the comparison above: the
    // difference is empty there, so a non-empty difference here is caused by
    // the added declaration and not by the carrier.
    assert!(
        clean_targets.difference(&roster(&clean)).next().is_none(),
        "the clean carrier already claims trust beyond its roster, so the discriminator above \
         cannot be attributed to the user declaration"
    );

    // THE REFUSAL, AND ITS REASON. Not merely "refused".
    let error = ken_runtime::native_program_admission(&dirty)
        .expect_err("a package claiming source-introduced trust must not be admitted");
    let rendered = error.to_string();
    assert!(
        rendered.contains("pre-source trusted-base roster")
            && rendered.contains("c1_user_trust"),
        "admission refused for the wrong reason -- it must name the roster mismatch and the \
         source-introduced target, got: {rendered}"
    );
}

/// A severed tuple partner refuses, and does so before the roster check can
/// mask it.
#[test]
fn c1_a_severed_tuple_partner_is_refused() {
    let mut program = erased(CLEAN_SOURCE);
    let victim = program
        .erased_core
        .metadata
        .assumption_trust_metadata
        .keys()
        .next()
        .expect("the carrier carries trust tuples")
        .clone();
    program.erased_core.metadata.assumptions.remove(&victim);

    let rendered = ken_runtime::native_program_admission(&program)
        .expect_err("a tuple whose assumption half was removed must not be admitted")
        .to_string();
    assert!(
        rendered.contains("not the same set"),
        "a severed tuple partner must be refused as an unpaired tuple, got: {rendered}"
    );
}

/// A noncanonical assumption identity refuses, even though the target is a
/// genuine roster member.
#[test]
fn c1_a_retargeted_assumption_identity_is_refused() {
    let mut program = erased(CLEAN_SOURCE);
    let (key, mut trust) = program
        .erased_core
        .metadata
        .assumption_trust_metadata
        .iter()
        .next()
        .map(|(key, trust)| (key.clone(), trust.clone()))
        .expect("the carrier carries trust tuples");
    // Point this assumption at a DIFFERENT roster member. Both symbols are
    // genuine and both are in the roster, so only the key/target relation is
    // wrong -- which is precisely what the canonical-projection check exists
    // to catch, and what a per-field check would miss.
    let other = program
        .erased_core
        .metadata
        .assumption_trust_metadata
        .values()
        .map(|t| t.target.clone())
        .find(|t| *t != trust.target)
        .expect("the carrier carries more than one trust target");
    trust.target = other;
    program
        .erased_core
        .metadata
        .assumption_trust_metadata
        .insert(key, trust);

    let rendered = ken_runtime::native_program_admission(&program)
        .expect_err("an assumption whose identity does not derive from its target is not admitted")
        .to_string();
    assert!(
        rendered.contains("canonical projection"),
        "a retargeted assumption must be refused on its identity, got: {rendered}"
    );
}

/// A missing roster refuses. This is the case the seed lane makes lawful
/// elsewhere and that must be fatal here.
#[test]
fn c1_a_package_without_a_roster_is_refused() {
    let mut program = erased(CLEAN_SOURCE);
    program.erased_core.metadata.checked_core.native_trusted_base = None;

    let rendered = ken_runtime::native_program_admission(&program)
        .expect_err("a package-backed program without a roster must not be admitted")
        .to_string();
    assert!(
        rendered.contains("requires the pre-source trusted-base roster"),
        "a missing roster must be refused as a missing roster, got: {rendered}"
    );
}
