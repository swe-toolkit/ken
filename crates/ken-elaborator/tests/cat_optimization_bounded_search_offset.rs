//! Bounded search split at an offset (Spec 72 §2.8).
//!
//! The closed clients are package fixtures, not conformance-seed execution.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const PROVIDERS: [&str; 5] = [
    "Core.Logic.Conjunction",
    "Core.Logic.Transport",
    "Core.Classes.LawfulClasses",
    "Data.Numeric.Nat.Order",
    "Data.Numeric.Nat.Arithmetic",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch roots-loads");
    env
}

fn contains_const(term: &Term, id: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id: reference, .. } if *reference == id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN_CLIENTS: &str = r#"
import Algorithm.Optimization.BoundedSearch (find_below, find_below_offset, find_below_offset_none)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Numeric.Nat.Arithmetic (add)

theorem open_offset
      (p : Nat → Bool) (q : Nat → Bool) (offset : Nat) (width : Nat) (n : Nat)
    : ((m : Nat) → Equal Bool (lt_nat m width) True → Equal Bool (q m) (p (add offset m)))
      → Equal (Option Nat) (find_below p offset) (None Nat)
      → Equal (Option Nat) (find_below q width) (Some Nat n)
      → Equal (Option Nat) (find_below p (add offset width)) (Some Nat (add offset n)) =
  find_below_offset p q offset width n

theorem open_offset_none
      (p : Nat → Bool) (q : Nat → Bool) (offset : Nat) (width : Nat)
    : ((m : Nat) → Equal Bool (lt_nat m width) True → Equal Bool (q m) (p (add offset m)))
      → Equal (Option Nat) (find_below p offset) (None Nat)
      → Equal (Option Nat) (find_below q width) (None Nat)
      → Equal (Option Nat) (find_below p (add offset width)) (None Nat) =
  find_below_offset_none p q offset width
"#;

/// MEASURED: exact trust identities, ownership and transparency, and open
/// clients of both complete public types. This measures trust added by the
/// package, not trust already in its providers.
#[test]
fn offset_laws_are_owned_transparent_and_add_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in PROVIDERS {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("both offset proofs kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "BoundedSearch trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "no trusted GlobalId is added or removed");

    env.elaborate_file(OPEN_CLIENTS)
        .expect("open clients check both exact public types");
    let shared = env.globals[&format!("{PACKAGE}.offset_search")];
    assert!(owned.contains(&shared));
    for (law, client) in [
        ("find_below_offset", "open_offset"),
        ("find_below_offset_none", "open_offset_none"),
    ] {
        let id = env.globals[&format!("{PACKAGE}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to BoundedSearch");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        let (_, proof) = env.env.transparent_body(id).expect("checked law body");
        assert!(contains_const(&proof, shared), "{law} uses offset_search");
        let (_, proof) = env
            .env
            .transparent_body(env.globals[client])
            .expect("checked open client body");
        assert!(contains_const(&proof, id), "{client} uses the exact {law}");
    }
}

#[test]
fn offset_helpers_refuse_selective_imports() {
    let mut env = loaded_package();
    for helper in [
        "offset_outcome",
        "offset_search",
        "offset_next",
        "offset_found_at",
    ] {
        match env.elaborate_file(&format!("import {PACKAGE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{helper}"));
            }
            other => panic!("{helper} must stay private: {other:?}"),
        }
    }
}

const ACCEPTED: &str = r#"
import Algorithm.Optimization.BoundedSearch (find_below, find_below_offset, find_below_offset_none)
import Data.Numeric.Nat.Order (lt_nat)
import Data.Numeric.Nat.Arithmetic (add)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six
const eight : Nat = Suc seven

fn hits_at_five_or_seven (m : Nat) : Bool =
  match m {
    Zero ↦ False;
    Suc a ↦
      match a {
        Zero ↦ False;
        Suc b ↦
          match b {
            Zero ↦ False;
            Suc c ↦
              match c {
                Zero ↦ False;
                Suc d ↦
                  match d {
                    Zero ↦ False;
                    Suc e ↦
                      match e {
                        Zero ↦ True;
                        Suc f ↦
                          match f {
                            Zero ↦ False;
                            Suc g ↦
                              match g {
                                Zero ↦ True;
                                Suc h ↦ False
                              }
                          }
                      }
                  }
              }
          }
      }
  }

fn after_three (m : Nat) : Bool = hits_at_five_or_seven (add three m)

theorem shift_three (width : Nat) (m : Nat)
    : Equal Bool (lt_nat m width) True
      → Equal Bool (after_three m) (hits_at_five_or_seven (add three m)) =
  λwithin. Refl

theorem first_part_three
    : Equal (Option Nat) (find_below hits_at_five_or_seven three) (None Nat) = Proved

theorem after_three_within_four
    : Equal (Option Nat) (find_below after_three four) (Some Nat two) = Proved

theorem below_seven_by_law
    : Equal (Option Nat) (find_below hits_at_five_or_seven seven) (Some Nat five) =
  find_below_offset hits_at_five_or_seven after_three three four two
    (shift_three four) first_part_three after_three_within_four

theorem below_seven_direct
    : Equal (Option Nat) (find_below hits_at_five_or_seven seven) (Some Nat five) = Proved

theorem after_three_within_two
    : Equal (Option Nat) (find_below after_three two) (None Nat) = Proved

theorem below_five_by_law
    : Equal (Option Nat) (find_below hits_at_five_or_seven five) (None Nat) =
  find_below_offset_none hits_at_five_or_seven after_three three two
    (shift_three two) first_part_three after_three_within_two

theorem below_five_direct
    : Equal (Option Nat) (find_below hits_at_five_or_seven five) (None Nat) = Proved
"#;

const EARLIER_HIT_CONTROLS: &str = r#"
fn after_six (m : Nat) : Bool = hits_at_five_or_seven (add six m)

theorem shift_six (m : Nat)
    : Equal Bool (lt_nat m two) True
      → Equal Bool (after_six m) (hits_at_five_or_seven (add six m)) =
  λwithin. Refl

theorem after_six_within_two
    : Equal (Option Nat) (find_below after_six two) (Some Nat one) = Proved

theorem below_six_direct
    : Equal (Option Nat) (find_below hits_at_five_or_seven six) (Some Nat five) = Proved

theorem below_eight_direct
    : Equal (Option Nat) (find_below hits_at_five_or_seven eight) (Some Nat five) = Proved
"#;

const FALSE_PREMISE_AND_APPLICATION: &str = r#"
theorem first_part_six : Equal (Option Nat) (find_below hits_at_five_or_seven six) (None Nat) = Proved

theorem below_eight_by_law
    : Equal (Option Nat) (find_below hits_at_five_or_seven eight) (Some Nat seven) =
  find_below_offset hits_at_five_or_seven after_six six two one
    shift_six first_part_six after_six_within_two
"#;

const FALSE_CONCLUSION: &str = r#"
theorem below_eight_wrong_witness
    : Equal (Option Nat) (find_below hits_at_five_or_seven eight) (Some Nat seven) = Proved
"#;

fn accepted_offset_clients() -> ElabEnv {
    let mut env = loaded_package();
    env.elaborate_file(ACCEPTED)
        .expect("Some and None law applications and independent same-form results check");
    env
}

fn earlier_hit_clients() -> ElabEnv {
    let mut env = accepted_offset_clients();
    env.elaborate_file(EARLIER_HIT_CONTROLS)
        .expect("shift and later hit check; direct searches at six and eight both find five");
    env
}

fn assert_refused_at_theorem(env: &mut ElabEnv, source: &str, theorem: &str) {
    let parsed = parse_decls(source).expect("refused client parses");
    let Some(SourceDecl::TheoremDecl {
        name, span: claim, ..
    }) = parsed.first()
    else {
        panic!("refused client must start with the isolated claim");
    };
    assert_eq!(name, theorem);
    let proved = claim.start
        + source[claim.start..claim.end]
            .find("Proved")
            .expect("claim body");
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim.start <= span.start
            && span.start <= proved
            && proved + "Proved".len() <= span.end
            && span.end <= claim.end =>
        {
            eprintln!("{theorem}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must kernel-refuse {theorem} in {claim:?}: {other:?}"),
    }
}

#[test]
fn empty_first_part_allows_shifted_hit_or_no_hit() {
    accepted_offset_clients();
}

#[test]
fn earlier_hit_refuses_at_the_first_part_none_premise() {
    let mut env = earlier_hit_clients();
    assert_refused_at_theorem(&mut env, FALSE_PREMISE_AND_APPLICATION, "first_part_six");
}

#[test]
fn earlier_hit_cannot_be_replaced_by_the_shifted_hit() {
    let mut env = earlier_hit_clients();
    assert_refused_at_theorem(&mut env, FALSE_CONCLUSION, "below_eight_wrong_witness");
}
