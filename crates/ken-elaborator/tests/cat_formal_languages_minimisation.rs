//! DFA future-language equivalence and reached-state minimisation.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §5; ten durable-invariant
//! cases: conformance/stdlib/formal-languages/seed-minimisation.md.
//! Fixtures test Boolean results and equality of future/reached states, never
//! a particular representative or state count.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::Decl;

const MIN: &str = "Algorithm.FormalLanguages.Minimisation";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn loaded() -> (ElabEnv, Vec<ken_kernel::GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MIN)
        .expect("§5's fully checked package must load from real catalog roots");
    (env, owned)
}

/// MEASURED: a fresh roots loader publishes all thirteen names as transparent,
/// checked identities; a selective generic client applies the two framed laws;
/// exact trusted GlobalId sets agree across the entire dependency closure.
/// CLAIMED: the eight public proofs and five public operations are checked and
/// this package adds no local trust. THE GAP: the generic clients pin law
/// signatures and ownership; closed seed behavior is exercised separately.
/// Promise class: durable invariant (the thirteen names are the §5 contract).
#[test]
fn public_laws_generic_clients_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    // Baseline the exact imported provider closure first: its inherited
    // primitive-contract trust is not introduced by Minimisation.
    for provider in [
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
        "Data.Finite.Finite",
        "Data.Collections.Derived",
        "Data.Sums.Combinators",
        "Core.Logic.Or",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MIN)
        .expect("Minimisation and all declared providers must roots-load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "catalog dependency load must add no trust");
    for name in [
        "same_future",
        "equivalent",
        "equivalent_states",
        "canonical",
        "minimise",
        "equivalent_sound",
        "equivalent_complete",
        "equivalent_states_sound",
        "equivalent_states_complete",
        "canonical_same_future",
        "canonical_unique",
        "accepts_minimise",
        "minimise_reduced",
    ] {
        let id = env.globals[&format!("{MIN}.{name}")];
        assert!(owned.contains(&id), "{name} must be owned by Minimisation");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be an actual checked transparent declaration"
        );
    }
    env.elaborate_file(
        r#"import Algorithm.FormalLanguages.Minimisation
             (same_future, equivalent, equivalent_states, canonical, minimise,
              equivalent_sound, equivalent_complete, equivalent_states_sound,
              equivalent_states_complete, canonical_same_future,
              canonical_unique, accepts_minimise, minimise_reduced)
           import Algorithm.FormalLanguages.Dfa (Dfa, accepts, run, start)
           import Data.Finite.Finite (Finite)
           import Data.Collections.Derived (list_append)
           theorem client_equivalent_sound
             (q : Type) (r : Type) (a : Type)
             (fq : Finite q) (fr : Finite r) (fa : Finite a)
             (d : Dfa q a) (e : Dfa r a) :
             Equal Bool (equivalent q r a fq fr fa d e) True →
             (w : List a) → Equal Bool (accepts q a d w) (accepts r a e w) =
             equivalent_sound q r a fq fr fa d e
           theorem client_equivalent_complete
             (q : Type) (r : Type) (a : Type)
             (fq : Finite q) (fr : Finite r) (fa : Finite a)
             (d : Dfa q a) (e : Dfa r a) :
             ((w : List a) → Equal Bool (accepts q a d w) (accepts r a e w)) →
             Equal Bool (equivalent q r a fq fr fa d e) True =
             equivalent_complete q r a fq fr fa d e
           theorem client_equivalent_states_sound
             (q : Type) (r : Type) (a : Type)
             (fq : Finite q) (fr : Finite r) (fa : Finite a)
             (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) :
             Equal Bool (equivalent_states q r a fq fr fa d e s t) True →
             same_future q r a d e s t =
             equivalent_states_sound q r a fq fr fa d e s t
           theorem client_equivalent_states_complete
             (q : Type) (r : Type) (a : Type)
             (fq : Finite q) (fr : Finite r) (fa : Finite a)
             (d : Dfa q a) (e : Dfa r a) (s : q) (t : r) :
             same_future q r a d e s t →
             Equal Bool (equivalent_states q r a fq fr fa d e s t) True =
             equivalent_states_complete q r a fq fr fa d e s t
           theorem client_canonical_same_future
             (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
             (d : Dfa q a) (s : q) :
             same_future q q a d d (canonical q a fq fa d s) s =
             canonical_same_future q a fq fa d s
           theorem client_canonical_unique
             (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
             (d : Dfa q a) (s : q) (t : q) :
             same_future q q a d d s t →
             Equal q (canonical q a fq fa d s) (canonical q a fq fa d t) =
             canonical_unique q a fq fa d s t
           theorem client_accepts_minimise
             (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
             (d : Dfa q a) (w : List a) :
             Equal Bool (accepts q a (minimise q a fq fa d) w) (accepts q a d w) =
             accepts_minimise q a fq fa d w
           theorem client_minimise_reduced
             (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
             (d : Dfa q a) (u : List a) (v : List a) :
             ((w : List a) →
               Equal Bool
                 (accepts q a (minimise q a fq fa d) (list_append a u w))
                 (accepts q a (minimise q a fq fa d) (list_append a v w))) →
             Equal q
               (run q a (minimise q a fq fa d)
                 (start q a (minimise q a fq fa d)) u)
               (run q a (minimise q a fq fa d)
                 (start q a (minimise q a fq fa d)) v) =
             minimise_reduced q a fq fa d u v"#,
    )
    .expect("all eight general §5 law signatures must be usable by a separate client");
    for name in ["disagree", "pick", "choose", "run_minimise", "sound_at"] {
        match env.elaborate_file(&format!("import {MIN} ({name})")) {
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{MIN}.{name}"));
            }
            Err(other) => panic!("private {name} rejected for the wrong reason: {other:?}"),
            Ok(_) => panic!("private {name} must not be selectively importable"),
        }
    }
}

const FIXTURES: &str = r#"
import Algorithm.FormalLanguages.Minimisation
  (same_future, equivalent, equivalent_states, canonical, minimise,
   equivalent_sound, equivalent_complete, equivalent_states_sound,
   equivalent_states_complete, canonical_same_future, canonical_unique,
   accepts_minimise, minimise_reduced)
import Algorithm.FormalLanguages.Dfa (Dfa, MkDfa, final, run, start, accepts)
import Data.Finite.Finite (bool_finite, unit_finite)
import Data.Collections.Derived (list_append)
import Core.Classes.LawfulClasses (bool_not)

const accept_all : Dfa Unit Bool = MkDfa Unit Bool (λs. λx. s) MkUnit (λs. True)
const reject_all : Dfa Unit Bool = MkDfa Unit Bool (λs. λx. s) MkUnit (λs. False)
const twin : Dfa Bool Bool = MkDfa Bool Bool (λs. λx. bool_not s) True (λs. True)
const only_empty : Dfa Bool Bool = MkDfa Bool Bool (λs. λx. False) True (λs. s)
const z : List Bool = Nil Bool
const f : List Bool = Cons Bool False z
const ft : List Bool = Cons Bool False (Cons Bool True z)
const m_twin : Dfa Bool Bool = minimise Bool Bool bool_finite bool_finite twin
const m_empty : Dfa Bool Bool = minimise Bool Bool bool_finite bool_finite only_empty
"#;

fn fixture() -> ElabEnv {
    let (mut env, _) = loaded();
    env.elaborate_file(FIXTURES)
        .expect("closed seed fixtures must use the public package and finite certificates");
    env
}

fn truth(env: &mut ElabEnv, name: &str, expr: &str, expected: bool) {
    let value = if expected { "True" } else { "False" };
    env.elaborate_file(&format!(
        "theorem seed_{name} : Equal Bool ({expr}) {value} = Proved"
    ))
    .unwrap_or_else(|error| panic!("{name} must kernel-reduce to {value}: {error:?}"));
}

fn checked(env: &mut ElabEnv, code: &str) {
    env.elaborate_file(code)
        .unwrap_or_else(|error| panic!("positive seed proof must check: {error:?}\n{code}"));
}

/// MEASURED: on the two distinct carriers the finite product decision is
/// True, and the independently constant-final predicates inhabit both law
/// premises. CLAIMED: language equality is not equality of state shapes.
/// THE GAP: these concrete inputs supplement the universal checked laws.
#[test]
fn seed_equivalent_different_carriers() {
    let mut env = fixture();
    truth(
        &mut env,
        "different_carriers",
        "equivalent Bool Unit Bool bool_finite unit_finite bool_finite twin accept_all",
        true,
    );
    for (name, word) in [("nil", "z"), ("false", "f"), ("false_true", "ft")] {
        truth(
            &mut env,
            &format!("twin_{name}"),
            &format!("accepts Bool Bool twin {word}"),
            true,
        );
        truth(
            &mut env,
            &format!("all_{name}"),
            &format!("accepts Unit Bool accept_all {word}"),
            true,
        );
    }
    checked(
        &mut env,
        r#"
      theorem all_words_agree : (w : List Bool) →
        Equal Bool (accepts Bool Bool twin w) (accepts Unit Bool accept_all w) =
        λw. Proved
      theorem positive_complete :
        Equal Bool (equivalent Bool Unit Bool bool_finite unit_finite bool_finite twin accept_all) True =
        equivalent_complete Bool Unit Bool bool_finite unit_finite bool_finite twin accept_all all_words_agree
      theorem positive_sound : (w : List Bool) →
        Equal Bool (accepts Bool Bool twin w) (accepts Unit Bool accept_all w) =
        equivalent_sound Bool Unit Bool bool_finite unit_finite bool_finite twin accept_all positive_complete
    "#,
    );
}

/// MEASURED: the same start/transition and certificates yield True for a
/// self-pair, False after changing only the second final predicate. CLAIMED:
/// empty-word disagreement is detected. THE GAP: longer words are separate.
#[test]
fn seed_equivalent_empty_word_final_flip() {
    let mut env = fixture();
    truth(
        &mut env,
        "self_pair",
        "equivalent Unit Unit Bool unit_finite unit_finite bool_finite accept_all accept_all",
        true,
    );
    truth(
        &mut env,
        "opposite_final",
        "equivalent Unit Unit Bool unit_finite unit_finite bool_finite accept_all reject_all",
        false,
    );
    truth(
        &mut env,
        "accept_empty",
        "accepts Unit Bool accept_all z",
        true,
    );
    truth(
        &mut env,
        "reject_empty",
        "accepts Unit Bool reject_all z",
        false,
    );
}

/// MEASURED: equal at [] but unequal after [False] for the same two DFAs;
/// the product decision returns False. CLAIMED: it searches reachable
/// disagreement, not only its initial final test. THE GAP: no witness order.
#[test]
fn seed_equivalent_disagreement_after_transition() {
    let mut env = fixture();
    truth(&mut env, "only_nil", "accepts Bool Bool only_empty z", true);
    truth(&mut env, "all_nil", "accepts Unit Bool accept_all z", true);
    truth(
        &mut env,
        "only_false",
        "accepts Bool Bool only_empty f",
        false,
    );
    truth(
        &mut env,
        "all_false",
        "accepts Unit Bool accept_all f",
        true,
    );
    truth(
        &mut env,
        "transition_disagrees",
        "equivalent Bool Unit Bool bool_finite unit_finite bool_finite only_empty accept_all",
        false,
    );
}

/// MEASURED: different Bool states of one toggling automaton agree on every
/// suffix by their constant final test; both state-specific law premises live.
/// CLAIMED: decision compares futures rather than state identities.
/// THE GAP: not all automata are represented by this single fixture.
#[test]
fn seed_equivalent_twin_states() {
    let mut env = fixture();
    truth(
        &mut env,
        "twin_states",
        "equivalent_states Bool Bool Bool bool_finite bool_finite bool_finite twin twin True False",
        true,
    );
    checked(
        &mut env,
        r#"
      theorem distinct_twin_states : Equal Bool True False → Bottom = λh. absurd h
      theorem twin_futures : same_future Bool Bool Bool twin twin True False = λw. Proved
      theorem positive_states_complete :
        Equal Bool (equivalent_states Bool Bool Bool bool_finite bool_finite bool_finite twin twin True False) True =
        equivalent_states_complete Bool Bool Bool bool_finite bool_finite bool_finite twin twin True False twin_futures
      theorem positive_states_sound : same_future Bool Bool Bool twin twin True False =
        equivalent_states_sound Bool Bool Bool bool_finite bool_finite bool_finite twin twin True False positive_states_complete
    "#,
    );
}

/// MEASURED: from supplied True/False states of one machine, empty-word
/// results differ and the state decision is False. CLAIMED: supplied starts
/// are respected. THE GAP: this case does not inspect the witness list.
#[test]
fn seed_inequivalent_only_empty_states() {
    let mut env = fixture();
    truth(&mut env, "only_states", "equivalent_states Bool Bool Bool bool_finite bool_finite bool_finite only_empty only_empty True False", false);
    truth(
        &mut env,
        "state_true",
        "final Bool Bool only_empty (run Bool Bool only_empty True z)",
        true,
    );
    truth(
        &mut env,
        "state_false",
        "final Bool Bool only_empty (run Bool Bool only_empty False z)",
        false,
    );
    checked(
        &mut env,
        r#"
      theorem different_futures_not_same :
        same_future Bool Bool Bool only_empty only_empty True False → Bottom =
        λagree. absurd (agree z)
    "#,
    );
}

/// MEASURED: the original and minimised twin/only-empty machines agree on
/// three independent words each, including accepted and rejected results.
/// CLAIMED: language preservation. THE GAP: six observations do not replace
/// the general checked law, which is also applied on these concrete words.
#[test]
fn seed_minimise_preserves_two_languages() {
    let mut env = fixture();
    for (machine, minimised, values) in [
        ("twin", "m_twin", [true, true, true]),
        ("only_empty", "m_empty", [true, false, false]),
    ] {
        for (word, expected) in ["z", "f", "ft"].into_iter().zip(values) {
            truth(
                &mut env,
                &format!("{machine}_original_{word}"),
                &format!("accepts Bool Bool {machine} {word}"),
                expected,
            );
            truth(
                &mut env,
                &format!("{machine}_minimised_{word}"),
                &format!("accepts Bool Bool {minimised} {word}"),
                expected,
            );
            checked(&mut env, &format!(
                "theorem law_{machine}_{word} : Equal Bool (accepts Bool Bool {minimised} {word}) (accepts Bool Bool {machine} {word}) = accepts_minimise Bool Bool bool_finite bool_finite {machine} {word}"
            ));
        }
    }
}

/// MEASURED: distinct prefixes [] and [False] of twin have the same future
/// in the minimised machine; the positive premise of `minimise_reduced`
/// produces equality of the reached states, without choosing either value.
/// CLAIMED: redundant reachable futures collapse. THE GAP: this pair is a
/// seed instance, not a count of states or a proof for all prefixes.
#[test]
fn seed_minimise_reduced_twin_reached() {
    let mut env = fixture();
    truth(
        &mut env,
        "original_moves",
        "run Bool Bool twin True f",
        false,
    );
    checked(
        &mut env,
        r#"
      theorem twin_minimised_residual : (w : List Bool) →
        Equal Bool
          (accepts Bool Bool m_twin (list_append Bool z w))
          (accepts Bool Bool m_twin (list_append Bool f w)) = λw. Proved
      theorem twin_reduced :
        Equal Bool
          (run Bool Bool m_twin (start Bool Bool m_twin) z)
          (run Bool Bool m_twin (start Bool Bool m_twin) f) =
        minimise_reduced Bool Bool bool_finite bool_finite twin z f twin_minimised_residual
    "#,
    );
}

/// MEASURED: only-empty accepts [] and rejects [False] after minimisation;
/// the empty continuation refutes equal residuals. CLAIMED: different
/// reachable futures cannot be merged. THE GAP: no cardinality assertion.
#[test]
fn seed_minimise_does_not_collapse_different_futures() {
    let mut env = fixture();
    truth(
        &mut env,
        "different_nil",
        "accepts Bool Bool m_empty z",
        true,
    );
    truth(
        &mut env,
        "different_after_false",
        "accepts Bool Bool m_empty f",
        false,
    );
    checked(
        &mut env,
        r#"
      theorem unequal_residuals :
        ((w : List Bool) →
          Equal Bool
            (accepts Bool Bool m_empty (list_append Bool z w))
            (accepts Bool Bool m_empty (list_append Bool f w))) → Bottom =
        λagree. absurd (agree z)
    "#,
    );
}

/// MEASURED: the same-future law relates the chosen canonical state to the
/// only-empty input across all suffixes; closed suffix results include both
/// True and False without pinning the representative. CLAIMED: future
/// language preservation. THE GAP: the finite witnesses supplement the law.
#[test]
fn seed_canonical_same_future_only_empty() {
    let mut env = fixture();
    checked(
        &mut env,
        r#"
      theorem canonical_only_future :
        same_future Bool Bool Bool only_empty only_empty
          (canonical Bool Bool bool_finite bool_finite only_empty True) True =
        canonical_same_future Bool Bool bool_finite bool_finite only_empty True
    "#,
    );
    for (name, word, expected) in [
        ("nil", "z", true),
        ("false", "f", false),
        ("false_true", "ft", false),
    ] {
        truth(
            &mut env,
            &format!("only_future_{name}"),
            &format!("final Bool Bool only_empty (run Bool Bool only_empty True {word})"),
            expected,
        );
        checked(&mut env, &format!(
            "theorem canonical_agrees_{name} : Equal Bool (final Bool Bool only_empty (run Bool Bool only_empty (canonical Bool Bool bool_finite bool_finite only_empty True) {word})) (final Bool Bool only_empty (run Bool Bool only_empty True {word})) = canonical_only_future {word}"
        ));
    }
}

/// MEASURED: independent constant-final agreement on all words for two
/// distinct twin states inhabits `canonical_unique`'s premise; its result
/// equates the canonical outputs, not either concrete Bool representative.
/// CLAIMED: one canonical value per future language. THE GAP: a single pair
/// cannot prove the general law without the checked theorem.
#[test]
fn seed_canonical_unique_twin_states() {
    let mut env = fixture();
    checked(
        &mut env,
        r#"
      theorem twin_future_agreement :
        same_future Bool Bool Bool twin twin True False = λw. Proved
      theorem twin_canonical_equal :
        Equal Bool
          (canonical Bool Bool bool_finite bool_finite twin True)
          (canonical Bool Bool bool_finite bool_finite twin False) =
        canonical_unique Bool Bool bool_finite bool_finite twin True False twin_future_agreement
    "#,
    );
}
