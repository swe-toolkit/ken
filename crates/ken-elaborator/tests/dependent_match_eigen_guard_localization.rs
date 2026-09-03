use ken_elaborator::ElabEnv;

fn eigen_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_file(
        "data EigenSequent = EigenMkSequent (List Nat) (List Nat)\n\
         data EigenGuard : EigenSequent -> Type where {\n\
           EigenGuarded : (sequent : EigenSequent) -> EigenGuard sequent\n\
         }\n\
         data EigenDerivation : EigenSequent -> Type where {\n\
           EigenLeaf : (gamma : List Nat) -> (right : Nat) ->\n\
             EigenDerivation\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat)));\n\
           EigenStay : (gamma : List Nat) -> (right : Nat) ->\n\
             EigenDerivation\n\
               (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))) ->\n\
             EigenDerivation\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat)));\n\
           EigenFresh : (gamma : List Nat) -> (right : Nat) -> (eigen : Nat) ->\n\
             EigenGuard\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) ->\n\
             EigenDerivation\n\
               (EigenMkSequent gamma (Cons Nat eigen (Nil Nat))) ->\n\
             EigenDerivation\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat)))\n\
         }\n\
         fn eigen_extend (levels : List Nat) (eigen : Nat) : List Nat =\n\
           Cons Nat eigen levels\n\
         data EigenGammaHolds : (levels : List Nat) -> (gamma : List Nat) -> Type where {\n\
           EigenGammaHeld : (levels : List Nat) -> (gamma : List Nat) ->\n\
             EigenGammaHolds levels gamma\n\
         }\n\
         data EigenTargetHolds : (levels : List Nat) -> (goal : Nat) -> Type where {\n\
           EigenTargetHeld : (levels : List Nat) -> (goal : Nat) ->\n\
             EigenTargetHolds levels goal\n\
         }\n\
         fn eigen_gamma_goal (levels : List Nat) (gamma : List Nat) : Omega =\n\
           ‖ EigenGammaHolds levels gamma ‖\n\
         fn eigen_target_goal (levels : List Nat) (goal : Nat) : Omega =\n\
           ‖ EigenTargetHolds levels goal ‖\n\
         theorem eigen_gamma_extend (levels : List Nat) (eigen : Nat)\n\
           (gamma : List Nat) (held : eigen_gamma_goal levels gamma)\n\
           : eigen_gamma_goal (eigen_extend levels eigen) gamma =\n\
           elim_trunc (eigen_gamma_goal (eigen_extend levels eigen) gamma)\n\
             (λvalue. trunc_intro\n\
               (EigenGammaHeld (eigen_extend levels eigen) gamma)) held\n\
         theorem eigen_target_unextend (levels : List Nat) (eigen : Nat)\n\
           (goal : Nat) (held : eigen_target_goal (eigen_extend levels eigen) goal)\n\
           : eigen_target_goal levels goal =\n\
           elim_trunc (eigen_target_goal levels goal)\n\
             (λvalue. trunc_intro (EigenTargetHeld levels goal)) held\n\
         fn eigen_motive (levels : List Nat) (sequent : EigenSequent) : Omega =\n\
           match sequent {\n\
             EigenMkSequent gamma delta ↦ (goal : Nat) ->\n\
               Equal (List Nat) delta (Cons Nat goal (Nil Nat)) ->\n\
               eigen_gamma_goal levels gamma -> eigen_target_goal levels Zero\n\
           }\n\
         theorem eigen_leaf_case (levels : List Nat) (gamma : List Nat)\n\
           (right : Nat)\n\
           : eigen_motive levels\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) =\n\
           λgoal. λsame. λforced.\n\
             trunc_intro (EigenTargetHeld levels Zero)\n\
         theorem eigen_stay_case (levels : List Nat) (gamma : List Nat)\n\
           (right : Nat)\n\
           (ih : eigen_motive levels\n\
             (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))))\n\
           : eigen_motive levels\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) =\n\
           λgoal. λsame. λforced. ih Zero Refl forced\n\
         theorem eigen_context_only_case (levels : List Nat) (gamma : List Nat)\n\
           (right : Nat) (eigen : Nat)\n\
           (ih : eigen_motive (eigen_extend levels eigen)\n\
             (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))))\n\
           : eigen_motive levels\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) =\n\
           λgoal. λsame. λforced.\n\
             eigen_target_unextend levels eigen Zero\n\
               (ih Zero Refl\n\
                 (eigen_gamma_extend levels eigen gamma forced))\n\
         theorem eigen_guard_only_case (levels : List Nat) (gamma : List Nat)\n\
           (right : Nat) (eigen : Nat)\n\
           (guard : EigenGuard\n\
             (EigenMkSequent gamma (Cons Nat right (Nil Nat))))\n\
           (ih : eigen_motive levels\n\
             (EigenMkSequent gamma (Cons Nat eigen (Nil Nat))))\n\
           : eigen_motive levels\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) =\n\
           λgoal. λsame. λforced. ih eigen Refl forced\n\
         theorem eigen_fresh_case (levels : List Nat) (gamma : List Nat)\n\
           (right : Nat) (eigen : Nat)\n\
           (guard : EigenGuard\n\
             (EigenMkSequent gamma (Cons Nat right (Nil Nat))))\n\
           (ih : eigen_motive (eigen_extend levels eigen)\n\
             (EigenMkSequent gamma (Cons Nat eigen (Nil Nat))))\n\
           : eigen_motive levels\n\
               (EigenMkSequent gamma (Cons Nat right (Nil Nat))) =\n\
           λgoal. λsame. λforced.\n\
             eigen_target_unextend levels eigen Zero\n\
               (ih eigen Refl\n\
                 (eigen_gamma_extend levels eigen gamma forced))",
    )
    .expect("generic eigen fixture prefix");
    env
}

#[test]
fn nonextending_recursive_sibling_remains_elaborable() {
    // Promise class: durable invariant. MEASURED: a recursive arm without a
    // refined-index-dependent field elaborates with no new trust. CLAIMED: the
    // dual-view mechanism does not blanket-change ordinary recursive matches.
    // THE GAP: the guard-present sibling below independently reaches the new
    // view selection.
    let mut env = eigen_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_decl(
        "theorem eigen_stay_sound (levels : List Nat) (sequent : EigenSequent)\n\
           (derivation : EigenDerivation sequent) : eigen_motive levels sequent =\n\
           match derivation {\n\
             EigenLeaf gamma right ↦ eigen_leaf_case levels gamma right;\n\
             EigenStay gamma right child ↦\n\
               eigen_stay_case levels gamma right\n\
                 (eigen_stay_sound levels\n\
                   (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))) child);\n\
             EigenFresh gamma right eigen guard child ↦\n\
               eigen_leaf_case levels gamma right\n\
           }",
    )
    .expect("the non-extending recursive arm is the green-before sibling");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn context_extension_without_dependent_guard_remains_elaborable() {
    // Promise class: durable invariant. MEASURED: extension of the motive-
    // threaded context without a dependent field elaborates before and after
    // the fix. CLAIMED: context extension is not the repaired predicate. THE
    // GAP: the guard-only row varies the actual causal axis independently.
    let mut env = eigen_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_decl(
        "theorem eigen_context_only_sound (levels : List Nat) (sequent : EigenSequent)\n\
           (derivation : EigenDerivation sequent) : eigen_motive levels sequent =\n\
           match derivation {\n\
             EigenLeaf gamma right ↦ eigen_leaf_case levels gamma right;\n\
             EigenStay gamma right child ↦\n\
               eigen_context_only_case levels gamma right Zero\n\
                 (eigen_context_only_sound (eigen_extend levels Zero)\n\
                   (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))) child);\n\
             EigenFresh gamma right eigen guard child ↦\n\
               eigen_leaf_case levels gamma right\n\
           }",
    )
    .expect("context extension alone must stay the green-before control");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn dependent_guard_without_context_extension_uses_constructor_local_view() {
    // Promise class: durable invariant. MEASURED: a field whose declared type
    // mentions the refined whole record index can be checked at its original
    // constructor-local type when passed to a helper, with no new trust.
    // CLAIMED: checking mode preserves the local half of the dual view. THE
    // GAP: the field is an ordinary generic family field; FoKripke coverage is
    // a separate downstream acceptance row.
    let mut env = eigen_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_decl(
        "theorem eigen_guard_only_sound (levels : List Nat) (sequent : EigenSequent)\n\
           (derivation : EigenDerivation sequent) : eigen_motive levels sequent =\n\
           match derivation {\n\
             EigenLeaf gamma right ↦ eigen_leaf_case levels gamma right;\n\
             EigenStay gamma right child ↦\n\
               eigen_stay_case levels gamma right\n\
                 (eigen_guard_only_sound levels\n\
                   (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))) child);\n\
             EigenFresh gamma right eigen guard child ↦\n\
               eigen_guard_only_case levels gamma right eigen guard\n\
                 (eigen_guard_only_sound levels\n\
                   (EigenMkSequent gamma (Cons Nat eigen (Nil Nat))) child)\n\
           }",
    )
    .expect("the dependent guard must retain its constructor-local view");
    assert_eq!(env.env.trusted_base(), trusted_before);
}

#[test]
fn dependent_guard_with_context_extension_uses_constructor_local_view() {
    // Promise class: durable invariant. MEASURED: the same dependent-field
    // view selection remains valid when an independent context extension is
    // present. CLAIMED: the repair closes the dependent-field class without
    // coupling it to the falsified extension predicate. THE GAP: the guard-only
    // row above proves the dependent field is sufficient to exercise the fix.
    let mut env = eigen_env();
    let trusted_before = env.env.trusted_base();
    env.elaborate_decl(
        "theorem eigen_sound (levels : List Nat) (sequent : EigenSequent)\n\
           (derivation : EigenDerivation sequent) : eigen_motive levels sequent =\n\
           match derivation {\n\
             EigenLeaf gamma right ↦ eigen_leaf_case levels gamma right;\n\
             EigenStay gamma right child ↦\n\
               eigen_stay_case levels gamma right\n\
                 (eigen_sound levels\n\
                   (EigenMkSequent gamma (Cons Nat Zero (Nil Nat))) child);\n\
             EigenFresh gamma right eigen guard child ↦\n\
               eigen_fresh_case levels gamma right eigen guard\n\
                 (eigen_sound (eigen_extend levels eigen)\n\
                   (EigenMkSequent gamma (Cons Nat eigen (Nil Nat))) child)\n\
           }",
    )
    .expect("the dependent guard must retain its local view under extension");
    assert_eq!(env.env.trusted_base(), trusted_before);
}
