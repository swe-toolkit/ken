//! NFA path acceptance, determinization, checked language laws and emptiness.
//!
//! Contract: spec/50-stdlib/61-formal-languages.md §3; conformance seed:
//! conformance/stdlib/formal-languages/seed-nfa.md.
//! Promise class: durable semantic invariants; concrete words and state
//! predicates are independent fixed fixtures, not a mask-layout snapshot.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::Decl;

const NFA: &str = "Algorithm.FormalLanguages.Nfa";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn checked_bool(env: &ElabEnv, name: &str) -> bool {
    let id = env.globals[name];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{name} must have a checked transparent body");
    };
    match eval(&[], body, &env.env, &mut EvalStore::new()) {
        EvalVal::Ctor { id, args, .. } if id == env.numeric_env.bool_true_id && args.is_empty() => {
            true
        }
        EvalVal::Ctor { id, args, .. }
            if id == env.numeric_env.bool_false_id && args.is_empty() =>
        {
            false
        }
        other => panic!("{name} must compute to a Bool constructor, got {other:?}"),
    }
}

/// MEASURED: real provider roots-loads precede Nfa roots-loading, all public
/// operations/laws are Nfa-owned, generic clients independently restate the
/// three §3.3 law types, and the exact trusted-GlobalId set is unchanged.
/// CLAIMED: public NFA soundness, completeness and emptiness exclusion are
/// kernel-checked, with no new local assumptions. THE GAP: typed clients
/// establish the published propositions, not concrete execution; companion
/// fixtures independently observe the Boolean decisions.
#[test]
fn nfa_public_surface_generic_laws_and_trust_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.And",
        "Data.Finite.Finite",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must load: {error:?}"));
    }
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[root()], NFA)
        .expect("the fully proved Nfa package must roots-load");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(after, before, "Nfa must add no local trust");
    for name in [
        "Nfa",
        "nfa_step",
        "nfa_initial",
        "nfa_final",
        "path_accepts",
        "NfaAcceptance",
        "nfa_accepts",
        "subset_state",
        "determinize",
        "subset_finite",
        "nfa_is_empty",
        "determinize_sound",
        "determinize_complete",
        "nfa_is_empty_rejects",
    ] {
        let id = env.globals[&format!("{NFA}.{name}")];
        assert!(owned.contains(&id), "{name} must be Nfa-owned");
    }
    for (family, constructor) in [("Nfa", "MkNfa"), ("NfaAcceptance", "Accepted")] {
        let family = env.globals[&format!("{NFA}.{family}")];
        let constructor = env.globals[&format!("{NFA}.{constructor}")];
        assert!(
            env.env
                .inductive(family)
                .expect("the public carrier is checked inductive data")
                .constructors
                .iter()
                .any(|item| item.id == constructor),
            "the exported constructor must belong to its Nfa family"
        );
    }
    for law in [
        "determinize_sound",
        "determinize_complete",
        "nfa_is_empty_rejects",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[&format!("{NFA}.{law}")]),
                Some(Decl::Transparent { .. })
            ),
            "{law} must carry a checked transparent proof term"
        );
    }
    env.elaborate_file(
        "import Algorithm.FormalLanguages.Nfa
           (Nfa, MkNfa, nfa_step, nfa_initial, nfa_final, path_accepts,
            NfaAcceptance, Accepted, nfa_accepts, subset_state, determinize,
            subset_finite, nfa_is_empty, determinize_sound,
            determinize_complete, nfa_is_empty_rejects)
         import Algorithm.FormalLanguages.Dfa (Dfa, accepts)
         import Data.Finite.Finite (Finite)
         theorem client_sound
           (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a) :
           Equal Bool (accepts (subset_state q fq) a (determinize q a fq n) w) True
             → nfa_accepts q a n w =
           determinize_sound q a fq n w
         theorem client_complete
           (q : Type) (a : Type) (fq : Finite q) (n : Nfa q a) (w : List a) :
           nfa_accepts q a n w
             → Equal Bool (accepts (subset_state q fq) a (determinize q a fq n) w) True =
           determinize_complete q a fq n w
         theorem client_empty
           (q : Type) (a : Type) (fq : Finite q) (fa : Finite a)
           (n : Nfa q a) (w : List a) :
           Equal Bool (nfa_is_empty q a fq fa n) True
             → nfa_accepts q a n w → Bottom =
           nfa_is_empty_rejects q a fq fa n w",
    )
    .expect("all three universal laws must inhabit the independently stated client types");
    for private in [
        "subset_next",
        "mask",
        "mask_any",
        "mask_build",
        "holds",
        "mask_finite",
        "Reached",
        "MkReached",
        "run_sound",
        "sound_end",
        "sound_step",
        "sound_edge",
        "run_complete",
        "build_holds",
        "build_bit",
    ] {
        match env.elaborate_file(&format!("import {NFA} ({private})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{NFA}.{private}"));
            }
            Err(other) => panic!("{private} rejected for the wrong reason: {other:?}"),
            Ok(_) => panic!("{private} must remain private"),
        }
    }
}

/// MEASURED: checked Nfa/Finite/Dfa/Reachability definitions execute through
/// the real interpreter on an accepted word, a rejected word, and machines
/// differing only in finality or initiality; proof clients also exercise
/// endpoint and length-mismatch branches of path_accepts. CLAIMED: the
/// Boolean subset construction and finite emptiness match §3's contract.
/// THE GAP: fixed examples do not replace generic kernel-checked laws.
#[test]
fn forward_edge_word_reject_control_and_finality_pair_compute() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], NFA)
        .expect("Nfa and all transitive catalog providers must roots-load");
    env.elaborate_file(
        "import Algorithm.FormalLanguages.Nfa
           (Nfa, MkNfa, path_accepts, NfaAcceptance, Accepted, nfa_accepts,
            subset_state, determinize, nfa_is_empty)
         import Algorithm.FormalLanguages.Dfa (accepts)
         import Data.Finite.Finite (bool_finite)
         import Core.Logic.And (And, Both)

         fn directed_edge (s : Bool) (x : Bool) (t : Bool) : Bool =
           match s {
             False ↦ match x {
               False ↦ t;
               True ↦ False
             };
             True ↦ False
           }
         const good : Nfa Bool Bool =
           MkNfa Bool Bool directed_edge
             (λs. match s { False ↦ True; True ↦ False })
             (λs. s)
         const no_final : Nfa Bool Bool =
           MkNfa Bool Bool directed_edge
             (λs. match s { False ↦ True; True ↦ False })
             (λs. False)
         const no_initial : Nfa Bool Bool =
           MkNfa Bool Bool directed_edge (λs. False) (λs. s)
         const input_a : List Bool = Cons Bool False (Nil Bool)
         const input_b : List Bool = Cons Bool True (Nil Bool)
         const path_a : List Bool = Cons Bool True (Nil Bool)

         theorem accepted_path : path_accepts Bool Bool good False input_a path_a =
           trunc_intro
             (Both (Equal Bool True True) (Equal Bool True True) Proved Proved)
         theorem explicit_acceptance : nfa_accepts Bool Bool good input_a =
           trunc_intro (Accepted Bool Bool good input_a False path_a Proved accepted_path)
         theorem no_empty_path_from_nonfinal :
           path_accepts Bool Bool good False (Nil Bool) (Nil Bool) → Bottom =
           λbad. absurd bad
         theorem no_short_path : path_accepts Bool Bool good False input_a (Nil Bool) → Bottom =
           λbad. absurd bad
         theorem no_long_path : path_accepts Bool Bool good False (Nil Bool) path_a → Bottom =
           λbad. absurd bad

         const accepted_a : Bool =
           accepts (subset_state Bool bool_finite) Bool
             (determinize Bool Bool bool_finite good) input_a
         const rejected_b : Bool =
           accepts (subset_state Bool bool_finite) Bool
             (determinize Bool Bool bool_finite good) input_b
         const rejects_empty : Bool =
           accepts (subset_state Bool bool_finite) Bool
             (determinize Bool Bool bool_finite good) (Nil Bool)
         const nonempty_decision : Bool =
           nfa_is_empty Bool Bool bool_finite bool_finite good
         const empty_decision : Bool =
           nfa_is_empty Bool Bool bool_finite bool_finite no_final
         const no_initial_word : Bool =
           accepts (subset_state Bool bool_finite) Bool
             (determinize Bool Bool bool_finite no_initial) input_a
         const no_initial_decision : Bool =
           nfa_is_empty Bool Bool bool_finite bool_finite no_initial",
    )
    .expect("actual Nfa path, subset and finite-decision clients must check");
    assert!(checked_bool(&env, "accepted_a"));
    assert!(!checked_bool(&env, "rejected_b"));
    assert!(!checked_bool(&env, "rejects_empty"));
    assert!(!checked_bool(&env, "nonempty_decision"));
    assert!(checked_bool(&env, "empty_decision"));
    assert!(!checked_bool(&env, "no_initial_word"));
    assert!(checked_bool(&env, "no_initial_decision"));
}

/// MEASURED: the initial and final predicates are identical in this fixture;
/// its two-symbol forward path succeeds while reversing the first edge would
/// leave the subset empty. CLAIMED: the subset step follows source-to-target
/// edges, not their inverse. THE GAP: the two-state vector detects this one
/// direction independently of the final/initial predicate, while the public
/// theorem proves arbitrary paths.
#[test]
fn initial_equals_final_discriminates_forward_edge_direction() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], NFA)
        .expect("Nfa and providers must roots-load");
    env.elaborate_file(
        "import Algorithm.FormalLanguages.Nfa (Nfa, MkNfa, subset_state, determinize)
         import Algorithm.FormalLanguages.Dfa (accepts)
         import Data.Finite.Finite (bool_finite)
         fn edge (s : Bool) (x : Bool) (t : Bool) : Bool =
           match x {
             False ↦ match s {
               False ↦ t;
               True ↦ False
             };
             True ↦ match s {
               True ↦ match t { False ↦ True; True ↦ False };
               False ↦ False
             }
           }
         const forward : Nfa Bool Bool =
           MkNfa Bool Bool edge
             (λs. match s { False ↦ True; True ↦ False })
             (λs. match s { False ↦ True; True ↦ False })
         const word : List Bool =
           Cons Bool False (Cons Bool True (Nil Bool))
         const accepted_forward : Bool =
           accepts (subset_state Bool bool_finite) Bool
             (determinize Bool Bool bool_finite forward) word",
    )
    .expect("controlled forward-edge fixture must elaborate");
    assert!(checked_bool(&env, "accepted_forward"));
}
