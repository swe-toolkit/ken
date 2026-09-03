//! `LANG-RECORD-INDEX-SIGMA-CLOSURE` acceptance.
//!
//! Generated dependent-match equality premises stay one-per-declared-index.
//! When a record index makes one such equality observationally reduce to a
//! `Sigma`, refinement consumes its projected `Eq` leaves atomically. `Top`
//! contributes no leaf, and any other evidence shape rejects the whole plan.

use ken_elaborator::{ElabEnv, ElabError};

#[test]
fn first_sigma_component_retypes_an_existing_field() {
    // Promise class: durable invariant.
    // MEASURED: a constructor field at `SigmaCell local` is passed where
    // `SigmaCell outer` is required while the branch goal itself is constant.
    // CLAIMED: field refinement consumes the first projected equality leaf.
    // THE GAP: the constant Bool goal prevents branch-goal refinement from
    // rescuing this fixture; removing the install consumer must re-red it.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data SigmaIx = SigmaMkIx Nat Bool")
        .expect("SigmaIx");
    env.elaborate_decl(
        "data SigmaCell : Nat -> Type where { \
           SigmaMkCell : (index : Nat) -> SigmaCell index \
         }",
    )
    .expect("SigmaCell");
    env.elaborate_decl(
        "data SigmaDerivation : SigmaIx -> Type where { \
           SigmaDeriv : (index : Nat) -> SigmaCell index \
             -> SigmaDerivation (SigmaMkIx index True) \
         }",
    )
    .expect("SigmaDerivation");
    env.elaborate_decl("fn use_sigma_cell (index : Nat) (cell : SigmaCell index) : Bool = True")
        .expect("use_sigma_cell");

    env.elaborate_decl(
        "fn sigma_install (index : Nat) \
           (derivation : SigmaDerivation (SigmaMkIx index True)) : Bool = \
         match derivation { \
           SigmaDeriv local cell ↦ use_sigma_cell index cell \
         }",
    )
    .expect("the first Sigma projection must retype the existing field");
}

#[test]
fn one_field_refinement_chains_both_sigma_components() {
    // Promise class: durable invariant.
    // MEASURED: one constructor field depends simultaneously on both varying
    // record components and reaches a consumer fixed at both outer values.
    // CLAIMED: install refinement folds every projected leaf through the prior
    // leaf's transported value and type. THE GAP: the first-component fixture
    // cannot detect overwriting or skipping the second component; this one can.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data ChainIx = ChainMkIx Nat Nat")
        .expect("ChainIx");
    env.elaborate_decl(
        "data ChainCell : Nat -> Nat -> Type where { \
           ChainMkCell : (first : Nat) -> (second : Nat) \
             -> ChainCell first second \
         }",
    )
    .expect("ChainCell");
    env.elaborate_decl(
        "data ChainDerivation : ChainIx -> Type where { \
           ChainDeriv : (first : Nat) -> (second : Nat) \
             -> ChainCell first second \
             -> ChainDerivation (ChainMkIx first second) \
         }",
    )
    .expect("ChainDerivation");
    env.elaborate_decl(
        "fn use_chain_cell (first : Nat) (second : Nat) \
           (cell : ChainCell first second) : Bool = True",
    )
    .expect("use_chain_cell");

    env.elaborate_decl(
        "fn sigma_chain (first : Nat) (second : Nat) \
           (derivation : ChainDerivation (ChainMkIx first second)) : Bool = \
         match derivation { \
           ChainDeriv local_first local_second cell ↦ \
             use_chain_cell first second cell \
         }",
    )
    .expect("both projected leaves must refine one field in sequence");
}

#[test]
fn second_sigma_component_refines_a_fresh_branch_goal() {
    // Promise class: durable invariant.
    // MEASURED: a fresh `SigmaOut local` is constructed against the caller's
    // `SigmaOut outer` goal, with the varying Nat in Sigma's second child.
    // CLAIMED: branch-goal refinement consumes the second projected Eq leaf.
    // THE GAP: there is no existing indexed field to retype, so removing the
    // goal consumer must independently re-red this fixture.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data SigmaIx = SigmaMkIx Bool Nat")
        .expect("SigmaIx");
    env.elaborate_decl(
        "data SigmaOut : Nat -> Type where { \
           SigmaMkOut : (index : Nat) -> SigmaOut index \
         }",
    )
    .expect("SigmaOut");
    env.elaborate_decl(
        "data SigmaDerivation : SigmaIx -> Type where { \
           SigmaDeriv : (index : Nat) \
             -> SigmaDerivation (SigmaMkIx True index) \
         }",
    )
    .expect("SigmaDerivation");

    env.elaborate_decl(
        "fn sigma_goal (index : Nat) \
           (derivation : SigmaDerivation (SigmaMkIx True index)) \
           : SigmaOut index = \
         match derivation { \
           SigmaDeriv local ↦ SigmaMkOut local \
         }",
    )
    .expect("the second Sigma projection must refine the fresh branch goal");
}

#[test]
fn recursive_record_field_uses_projected_evidence_not_whole_record_j() {
    // Promise class: durable invariant.
    // MEASURED: the recursive field type contains the whole constructor record
    // and is passed to a consumer fixed at the caller's outer record.
    // CLAIMED: refinement projects leaves before any J transport.
    // THE GAP: passing the unprojected premise to J produces the pinned
    // `BadEliminator` failure; the private unit test separately pins recursive-
    // IH Sigma discharge so neither property can rescue the other.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data SigmaIx = SigmaMkIx Nat Bool")
        .expect("SigmaIx");
    env.elaborate_decl(
        "data SigmaTree : SigmaIx -> Type where { \
           SigmaLeaf : (index : Nat) -> SigmaTree (SigmaMkIx index True); \
           SigmaStep : (index : Nat) -> SigmaTree (SigmaMkIx index True) \
             -> SigmaTree (SigmaMkIx index True) \
         }",
    )
    .expect("SigmaTree");

    env.elaborate_decl(
        "fn use_sigma_tree (index : Nat) \
           (tree : SigmaTree (SigmaMkIx index True)) : Bool = True",
    )
    .expect("use_sigma_tree");
    env.elaborate_decl(
        "fn sigma_child (index : Nat) \
           (tree : SigmaTree (SigmaMkIx index True)) : Bool = \
         match tree { \
           SigmaLeaf local ↦ True; \
           SigmaStep local child ↦ use_sigma_tree index child \
         }",
    )
    .expect("the recursive record field must never send whole Sigma evidence to J");
}

#[test]
fn unsupported_pi_beneath_sigma_rejects_the_entire_plan() {
    // Promise class: durable invariant.
    // MEASURED: equality of an Omega-valued record component reduces to
    // implication (`Pi`) evidence beneath the record Sigma.
    // CLAIMED: the refinement vocabulary is exactly Eq/Sigma/Top and rejects
    // unsupported evidence atomically rather than applying an earlier Nat leaf.
    // THE GAP: the exact walker diagnostic proves this is the consumer-side
    // refusal, while the Bool-record positives prove supported Sigma still runs.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "data SigmaPropIx : Type 1 where { \
           SigmaPropMkIx : Nat -> Omega -> SigmaPropIx \
         }",
    )
    .expect("SigmaPropIx");
    env.elaborate_decl(
        "data SigmaPropDerivation : SigmaPropIx -> Type 1 where { \
           SigmaPropDeriv : (index : Nat) -> (proposition : Omega) \
             -> SigmaPropDerivation (SigmaPropMkIx index proposition) \
         }",
    )
    .expect("SigmaPropDerivation");

    let error = env
        .elaborate_decl(
            "fn sigma_prop_reject (index : Nat) (proposition : Omega) \
               (derivation : SigmaPropDerivation \
                 (SigmaPropMkIx index proposition)) : Bool = \
             match derivation { SigmaPropDeriv local local_prop ↦ True }",
        )
        .expect_err("Pi evidence beneath Sigma must stay outside refinement vocabulary");

    assert!(
        matches!(error, ElabError::Internal(ref reason)
        if reason.starts_with(
            "index refinement: unsupported generated equality evidence shape"
        )),
        "unsupported nested evidence must hit the atomic walker refusal, got {error:?}"
    );
}

#[test]
fn hidden_result_refinement_handles_a_reducible_record_scrutinee() {
    // Promise class: durable invariant.
    // MEASURED: an indexless record is matched as a reducible constructor
    // expression inside a recursive definition, activating the hidden
    // whole-scrutinee equation whose endpoints both expose that constructor.
    // CLAIMED: the reachable hidden-result path shares component projection and
    // transports the recursive result without handing Sigma evidence to J.
    // THE GAP: a neutral record variable leaves the raw Eq stuck and would not
    // exercise this branch; the explicit constructor scrutinee is essential.
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl("data HiddenIx = HiddenMkIx Nat Bool")
        .expect("HiddenIx");
    env.elaborate_decl(
        "data HiddenOut : HiddenIx -> Type where { \
           HiddenMkOut : (index : Nat) -> (flag : Bool) \
             -> HiddenOut (HiddenMkIx index flag) \
         }",
    )
    .expect("HiddenOut");
    env.elaborate_decl(
        "data HiddenCell : Nat -> Type where { \
           HiddenMkCell : (index : Nat) -> HiddenCell index \
         }",
    )
    .expect("HiddenCell");

    env.elaborate_file(
        "fn hidden_record_a (fuel : Nat) (index : Nat) (flag : Bool) \
           (cell : HiddenCell index) : HiddenOut (HiddenMkIx index flag) = \
         match fuel { \
           Zero ↦ HiddenMkOut index flag; \
           Suc smaller ↦ match (HiddenMkIx index flag) { \
             HiddenMkIx local local_flag ↦ \
               hidden_record_b smaller local local_flag cell \
           } \
         }\n\
         fn hidden_record_b (fuel : Nat) (index : Nat) (flag : Bool) \
           (cell : HiddenCell index) : HiddenOut (HiddenMkIx index flag) = \
         match fuel { \
           Zero ↦ HiddenMkOut index flag; \
           Suc smaller ↦ match (HiddenMkIx index flag) { \
             HiddenMkIx local local_flag ↦ \
               hidden_record_a smaller local local_flag cell \
           } \
         }",
    )
    .expect("reachable hidden result refinement must project record equality evidence");
}
