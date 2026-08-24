//! `LANG-TRUNCATION-SURFACE-SYNTAX`, `D1`-`D3` -- propositional truncation
//! gets a surface spelling and an elaboration rule.
//!
//! `Term::Trunc`/`Term::TruncProj` are already kernel-typed (`16 §6`); no
//! surface syntax or elaboration rule reached them. This file establishes,
//! by actual `.ken` elaboration:
//!
//! - `D1` -- formation `‖A‖` / `||A||` (one token, both spellings) lexes,
//!   parses, and elaborates to `Term::Trunc`, with the exact same core term a
//!   direct Rust construction (`denote`-style) would build (`AC-1`, `AC-3`).
//! - `D2` -- introduction (`trunc_intro a`, checked-mode only, `AC-4`'s
//!   actionable-remedy diagnostic) and elimination (`elim_trunc P f t`,
//!   elaborating to the kernel's existing `QuotElim` over a `Trunc`
//!   scrutinee), with the Ω-only restriction independently reverified at the
//!   KERNEL level (`AC-4`).
//! - `D3` -- the exact caller shape `Derives(s) : Omega := ‖ FokDerivation s
//!   ‖` (`AC-5`), against a placeholder `FokDerivation` -- never the real
//!   rule set, `fok_derives`, or `fok_classically_valid`, which remain
//!   [[V3-FO-CHECKER-SOUNDNESS]]'s to author (banned scope).
//!
//! No kernel change (`AC-2`): every construction below goes through ordinary
//! `ken_kernel::infer`/`check` on ALREADY-EXISTING `Term` variants.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;
use ken_kernel::{Context, KernelError, Term};

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

fn mk_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base env construction failed");
    catalog_or::load_core_logic_or(&mut env);
    env
}

fn trust(env: &ElabEnv) -> BTreeSet<ken_kernel::GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

// ---------------------------------------------------------------------
// D1: formation.
// ---------------------------------------------------------------------

/// `‖A‖ : Ω_l` for `A : Type l` -- the Unicode spelling, in a real
/// declaration (`D1`'s deliverable: "a `.ken` file declares something of
/// type Omega using it").
#[test]
fn d1_unicode_formation_elaborates_and_kernel_checks() {
    let mut env = mk_env();
    let before = trust(&env);

    // `const`, not `theorem`: the DECLARED type here is the bare universe
    // `Omega` itself (classifying at `Type 1`), not a specific proposition
    // (which is what `theorem` requires its declared type to classify at) --
    // matches the established `fn fok_derives (...) : Omega = ...` shape
    // ([[V3-FO-CHECKER-SOUNDNESS]] `D0`'s own probe).
    env.elaborate_decl("const truncBoolUnicode : Omega = ‖ Bool ‖")
        .expect("‖Bool‖ : Omega must elaborate and kernel-check");

    assert_eq!(
        before,
        trust(&env),
        "AC-1: an ordinary formation must add no trusted_base entry"
    );
}

/// Same statement, ASCII `||A||` spelling -- the collision-safe digraph
/// (two adjacent `|`, a genuine lexer token, not an identifier-sugar route
/// that could shadow a user global).
#[test]
fn d1_ascii_formation_elaborates_and_kernel_checks() {
    let mut env = mk_env();
    let before = trust(&env);

    env.elaborate_decl("const truncBoolAscii : Omega = ||Bool||")
        .expect("||Bool|| : Omega must elaborate and kernel-check");

    assert_eq!(
        before,
        trust(&env),
        "AC-1: an ordinary formation must add no trusted_base entry"
    );
}

/// `‖A‖` may itself be TRUNCATED again (`‖‖A‖‖`), and may wrap a non-atomic
/// expression (an application, not just a bare `ConId`) -- confirms the
/// grammar production parses a full expression between the delimiters, not
/// merely a single atom.
#[test]
fn d1_formation_wraps_a_full_expression_and_nests() {
    let mut env = mk_env();
    env.elaborate_decl("data Box (a : Type) : Type where { MkBox : a -> Box a }")
        .expect("a one-field parametric box must elaborate");

    env.elaborate_decl("const truncBoxBool : Omega = ‖ Box Bool ‖")
        .expect("‖ Box Bool ‖ -- an application inside the delimiters -- must elaborate");

    env.elaborate_decl("const truncTruncBool : Omega = ‖ ‖ Bool ‖ ‖")
        .expect_err(
            "‖‖Bool‖‖ truncates a Ω-classified thing, not a Type -- must be REJECTED (Trunc's \
             own argument must be A : Type l, `16 §6`), not silently accepted; this control \
             would catch a formation rule that forgot to check the inner sort",
        );
}

// ---------------------------------------------------------------------
// AC-3: the surface form is the SAME core term a direct Rust construction
// builds, established by a comparison the mutation controls show is real
// (non-vacuous).
// ---------------------------------------------------------------------

#[test]
fn ac3_surface_formation_matches_direct_rust_construction() {
    let mut env = mk_env();
    let bool_id = env
        .globals
        .get("Bool")
        .copied()
        .expect("Bool must be a prelude global");
    let nat_id = env
        .globals
        .get("Nat")
        .copied()
        .expect("Nat must be a prelude global");

    let (surface_term, surface_ty) = env
        .elaborate_expr("ac3_probe", "‖ Bool ‖")
        .expect("‖Bool‖ must elaborate as a standalone expression");

    let direct = Term::Trunc(Box::new(Term::IndFormer {
        id: bool_id,
        level_args: vec![],
    }));
    assert_eq!(
        surface_term, direct,
        "the surface `‖Bool‖` must build the EXACT same core term a direct `denote`-style Rust \
         construction (`Term::Trunc(Term::IndFormer{{id: bool_id, ..}})`) would"
    );
    assert!(
        matches!(
            ken_kernel::whnf(&env.env, &Context::new(), &surface_ty),
            Term::Omega(_)
        ),
        "‖Bool‖'s own inferred type must whnf to an Omega sort"
    );

    // Evasion controls: the comparison above must be able to FAIL, not just
    // pass for any term (a discriminator that never reds proves nothing).
    let wrong_inner_type = Term::Trunc(Box::new(Term::IndFormer {
        id: nat_id,
        level_args: vec![],
    }));
    assert_ne!(
        surface_term, wrong_inner_type,
        "a formation rule that ignored its argument and always truncated the wrong type must \
         be caught by this comparison"
    );

    let wrong_constructor = Term::TruncProj(Box::new(Term::IndFormer {
        id: bool_id,
        level_args: vec![],
    }));
    assert_ne!(
        surface_term, wrong_constructor,
        "a formation rule that emitted the INTRODUCTION node (TruncProj) instead of the \
         FORMATION node (Trunc) must be caught by this comparison"
    );
}

// ---------------------------------------------------------------------
// D2: introduction (`trunc_intro a`).
// ---------------------------------------------------------------------

/// The positive case: `trunc_intro a` checks against a `‖A‖`-shaped goal.
///
/// This WP adds no type-annotation-position spelling for `‖A‖` (out of
/// scope -- `D1`-`D3` are expression-position only), so there is no
/// DECLARATION whose own `: T` slot can read `‖Bool‖` directly to force a
/// checked target that way. `elim_trunc`'s `f` argument supplies one
/// instead: with `P = ‖Nat‖`, `f`'s expected type is `Bool -> ‖Nat‖`, so
/// INSIDE `f`'s body the expected type is exactly `‖Nat‖` -- a genuine
/// checked position reaching `trunc_intro`'s own `check` arm (distinct from
/// `elim_trunc`'s `t`-position special case, which never calls it).
#[test]
fn d2_trunc_intro_checks_against_a_trunc_shaped_goal() {
    let mut env = mk_env();
    let before = trust(&env);

    env.elaborate_expr(
        "d2_intro_positive_probe",
        "elim_trunc (‖ Nat ‖) (\\x . trunc_intro Zero) (trunc_intro True)",
    )
    .expect("trunc_intro Zero must check against the goal ‖Nat‖ inside f's body");

    assert_eq!(
        before,
        trust(&env),
        "AC-1: introduction must add no trusted_base entry"
    );
}

/// `AC-4`, first half: `trunc_intro a` used where NO expected type is
/// available must name its remedy, not degrade to `unresolved identifier`.
#[test]
fn d2_trunc_intro_in_infer_position_names_its_remedy() {
    let mut env = mk_env();
    // A bare standalone expression is elaborated in INFER mode
    // (`elaborate_expr` -> `infer`, never `check`) -- there is no expected
    // type anywhere for `trunc_intro`'s argument's type `A` to come from.
    let err = env
        .elaborate_expr("ac4_probe", "trunc_intro True")
        .expect_err("trunc_intro in infer position must be rejected, not silently accepted");
    let msg = err.to_string();
    assert!(
        msg.contains("expected type") || msg.contains("ascription"),
        "the refusal must NAME its remedy (an expected type / ascription), not read as an \
         unrelated 'unresolved identifier' or bare parse error -- got: {msg}"
    );
    assert!(
        !msg.to_lowercase().contains("unresolved"),
        "must not degrade to an unrelated unresolved-identifier error -- got: {msg}"
    );
}

/// `trunc_intro a` checked against a goal that is NOT `‖A‖`-shaped must also
/// name its remedy, not produce a generic type-mismatch. `const`, not
/// `theorem`: `Bool` need not classify at Omega for a `const`'s declared
/// type, so this reaches `trunc_intro`'s own check arm instead of being
/// intercepted first by an unrelated declaration-keyword restriction.
#[test]
fn d2_trunc_intro_against_a_non_trunc_goal_names_its_remedy() {
    let mut env = mk_env();
    let err = env
        .elaborate_decl("const introWrongGoal : Bool = trunc_intro True")
        .expect_err("trunc_intro against a non-‖A‖ goal must be rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("‖A‖") || msg.contains("trunc_intro"),
        "the refusal must name trunc_intro's own expectation, not a generic mismatch -- \
         got: {msg}"
    );
}

/// `trunc_intro` is `RESERVED_SUGAR` (`resolve.rs`, same collision-hygiene
/// list as `absurd`, FR-2) -- a user-declared global named `trunc_intro`
/// must be a hard resolve-time error, not a silent, permanent shadow.
/// Mirrors `fr2_absurd_collision_hygiene.rs`'s own controls for `absurd`.
#[test]
fn d2_trunc_intro_is_reserved_sugar_declaring_it_is_a_hard_error() {
    let mut env = mk_env();
    let err = env
        .elaborate_decl(
            "fn trunc_intro (c : Type) (e : Bool) : c = match e { True |-> e ; False |-> e }",
        )
        .expect_err("'trunc_intro' must be rejected as a reserved-sugar collision");
    match err {
        ken_elaborator::ElabError::ParseError { msg, .. } => {
            assert!(
                msg.contains("trunc_intro") && msg.contains("reserved surface sugar"),
                "the error must name the collision, not a generic message -- got: {msg}"
            );
        }
        other => panic!("expected a ParseError (a real hard error), got {other:?}"),
    }
}

/// `elim_trunc` is NOT in `RESERVED_SUGAR` (same `J`/`Eq` precedent: it
/// intercepts only a 3-argument application, so a lower-arity declaration
/// of the same name coexists) -- confirms the exclusion is a real, checked
/// design decision, not an omission.
#[test]
fn d2_elim_trunc_lower_arity_is_not_a_collision() {
    let mut env = mk_env();
    env.elaborate_decl("fn elim_trunc (x : Bool) : Bool = x")
        .expect("a 1-arg 'elim_trunc' is not the arity-3 elim_trunc sugar; must elaborate");
}

/// `trunc_intro`'s argument is itself CHECKED against the recovered `A` --
/// a mismatched payload must be refused, not silently accepted. Same
/// `elim_trunc`-`f`-body technique as the positive case above, but with
/// `P = ‖Bool‖` and a `Nat` payload (`Zero`) instead of a `Bool` one.
#[test]
fn d2_trunc_intro_argument_is_checked_against_the_recovered_carrier() {
    let mut env = mk_env();
    let err = env
        .elaborate_expr(
            "d2_intro_payload_probe",
            "elim_trunc (‖ Bool ‖) (\\x . trunc_intro Zero) (trunc_intro True)",
        )
        .expect_err(
            "Zero : Nat, not Bool -- trunc_intro's payload must be checked against the \
             recovered carrier (Bool, from P = ‖Bool‖), not waved through",
        );
    let _ = err.to_string();
}

// ---------------------------------------------------------------------
// D2: elimination (`elim_trunc P f t`).
// ---------------------------------------------------------------------

/// The positive case, exercised end-to-end via reduction: `elim_trunc P f
/// |a| ⇝ f a` (`16 §6`'s computation rule). `P` must itself be Ω-classified
/// (`elim_trunc`'s first argument, `16 §6`'s `P : Omega`) -- `Nat` alone
/// will not do, so `P = ‖Nat‖` (truncation always lands in Omega regardless
/// of what it wraps), and `f`'s body is itself a `trunc_intro` -- so
/// `elim_trunc ‖ Nat ‖ (λx. trunc_intro Zero) (trunc_intro True)` must
/// reduce to `trunc_intro Zero`'s own core term.
#[test]
fn d2_elim_trunc_elaborates_and_computes_via_the_i_reduction() {
    let mut env = mk_env();
    let before = trust(&env);

    let (term, _ty) = env
        .elaborate_expr(
            "d2_elim_probe",
            "elim_trunc (‖ Nat ‖) (\\x . trunc_intro Zero) (trunc_intro True)",
        )
        .expect("elim_trunc P f (trunc_intro a) must elaborate");

    assert_eq!(
        before,
        trust(&env),
        "AC-1: elimination must add no trusted_base entry"
    );

    let zero_id = env
        .globals
        .get("Zero")
        .copied()
        .expect("Zero is a prelude global");
    let expected = Term::TruncProj(Box::new(Term::Constructor {
        id: zero_id,
        level_args: vec![],
    }));
    let reduced = ken_kernel::whnf(&env.env, &Context::new(), &term);
    assert_eq!(
        reduced, expected,
        "elim_trunc ‖Nat‖ (\\x. trunc_intro Zero) (trunc_intro True) must whnf-reduce to \
         `trunc_intro Zero`'s own core term via the i-reduction, exactly as `16 §6` specifies"
    );
}

/// `AC-4`, second half -- reverified at the KERNEL level, independent of
/// this WP's own elaborator-side gate: eliminating a `‖A‖` into `Type`
/// remains refused with the kernel's OWN existing error
/// (`check.rs::infer_quot_elim`'s `opt_rel` check), not merely intercepted
/// by a redundant elaborator-level convenience gate. A raw `Term::QuotElim`
/// is built by hand here -- bypassing `elim_trunc`'s own sugar entirely --
/// because this WP's elaboration rule structurally can never emit a
/// Type-targeted motive (see the companion elaborator-level control below),
/// so the only way to prove the KERNEL's restriction independently still
/// holds is to attempt it directly against the kernel.
#[test]
fn ac4_kernel_level_type_target_elimination_of_trunc_remains_refused() {
    let env = mk_env();
    let bool_id = env
        .globals
        .get("Bool")
        .copied()
        .expect("Bool must be a prelude global");
    let true_id = env
        .globals
        .get("True")
        .copied()
        .expect("True must be a prelude global");

    let bool_ty = Term::IndFormer {
        id: bool_id,
        level_args: vec![],
    };
    // `TruncProj` is checked-only (kernel `check.rs::infer`'s explicit
    // non-inferable list) and `infer_quot_elim`'s FIRST step is `infer` on
    // the scrutinee -- so, exactly like `motive` below, the scrutinee must
    // be `Ascript`-wrapped to be inferable at all.
    let scrut_ty = Term::Trunc(Box::new(bool_ty.clone()));
    let scrut = Term::Ascript(
        Box::new(Term::TruncProj(Box::new(Term::Constructor {
            id: true_id,
            level_args: vec![],
        }))),
        Box::new(scrut_ty.clone()),
    );
    // motive := (λ_:‖Bool‖. Bool) ascripted at (‖Bool‖) -> Type 0 -- a
    // TYPE target, the shape `16 §6` forbids for a Trunc scrutinee.
    let motive_ty = Term::pi(scrut_ty.clone(), Term::Type(ken_kernel::Level::zero()));
    let motive_lam = Term::lam(scrut_ty, bool_ty.clone()); // closed term, no weaken needed
    let motive = Term::Ascript(Box::new(motive_lam), Box::new(motive_ty));
    let method = Term::lam(bool_ty, Term::var(0)); // identity on Bool

    let elim = Term::QuotElim {
        motive: Box::new(motive),
        method: Box::new(method.clone()),
        respect: Box::new(method), // unreachable before the rejection fires
        scrut: Box::new(scrut),
    };

    match ken_kernel::infer(&env.env, &Context::new(), &elim) {
        Err(KernelError::BadEliminator(msg)) => {
            assert!(
                msg.contains("Type target requires a Quot"),
                "must be refused with the kernel's OWN existing message, not a paraphrase -- \
                 got: {msg}"
            );
        }
        other => panic!(
            "a Type-targeted QuotElim over a Trunc scrutinee must be refused by the kernel's \
             existing opt_rel check -- got {other:?}"
        ),
    }
}

/// Companion elaborator-level control: `elim_trunc`'s OWN surface sugar
/// structurally cannot even ATTEMPT a Type-target elimination, since it
/// requires its first argument `P` to already be Ω-classified before
/// building anything -- a `Type`-classified `P` (e.g. `Nat`) is refused at
/// the elaborator, before the kernel is ever asked. This is defense in
/// depth, not a substitute for `ac4_kernel_level_...` above.
#[test]
fn d2_elim_trunc_surface_sugar_refuses_a_type_classified_target() {
    let mut env = mk_env();
    let err = env
        .elaborate_expr(
            "elim_trunc_type_target_probe",
            "elim_trunc Nat (\\_ . Zero) (trunc_intro True)",
        )
        .expect_err(
            "elim_trunc's first argument (the target) must be Ω-classified, not Type-classified",
        );
    let msg = err.to_string();
    assert!(
        msg.to_lowercase().contains("proposition") || msg.contains('Ω') || msg.contains("Omega"),
        "the refusal should name the Ω-classification requirement -- got: {msg}"
    );
}

// ---------------------------------------------------------------------
// D3: the caller -- the exact shape V3-FO-CHECKER-SOUNDNESS D1b needs.
// ---------------------------------------------------------------------

/// `Derives(s) : Omega := ‖ FokDerivation s ‖` (`AC-5`) -- against a
/// PLACEHOLDER `FokDerivation`, never the real rule set. `FoKripke.ken` is
/// loaded read-only (`include_str!`); this test declares `FokDerivation`
/// and `Derives` only in its own transient environment, touching no
/// checked-in file and authoring none of [[V3-FO-CHECKER-SOUNDNESS]]'s
/// `D1b` (`fok_derives`/`fok_classically_valid`'s REAL definitions).
#[test]
fn ac5_derives_exact_shape_elaborates_and_kernel_checks() {
    let mut env = mk_env();
    env.elaborate_file(FOK_SOURCE)
        .expect("FoKripke.ken must still elaborate/kernel-check unmodified");

    let before = trust(&env);

    env.elaborate_decl(
        "data FokDerivation : FokSequent -> Type where { \
           FokDerivationPlaceholder : (s : FokSequent) -> FokDerivation s \
         }",
    )
    .expect("a placeholder FokDerivation indexed by FokSequent must elaborate");

    // `fn`, not `theorem`: the declared type is the bare universe `Omega`
    // (classifies at `Type 1`), matching `D0`'s own `fn fok_derives` probe
    // shape -- `fn` requires >=1 explicit value parameter, satisfied by `s`.
    env.elaborate_decl("fn Derives (s : FokSequent) : Omega = ‖ FokDerivation s ‖")
        .expect(
            "`Derives(s) : Omega := ‖ FokDerivation s ‖` -- the exact D0-blocking shape -- must \
             now elaborate and kernel-check (AC-5)",
        );

    assert_eq!(
        before,
        trust(&env),
        "AC-1: the placeholder FokDerivation + Derives sequence must add no trusted_base entry"
    );
}

// ---------------------------------------------------------------------
// AC-6: corpus-wide oracle survival, including the formatter round-trip.
//
// Enumerated by grepping for glob-based catalog collectors that touch
// formatting/canonicalization (not by naming the one remembered):
//   - `crates/ken-cli/tests/ken_fmt.rs::strict_frozen_corpus_gate_is_green`
//   - `crates/ken-elaborator/tests/kenfmt_b1_lossless.rs::
//      whole_catalog_and_every_parseable_ken_fence_round_trip_byte_exactly`
//   - `crates/ken-elaborator/tests/kenfmt_b3_layout.rs::
//      ac7_whole_catalog_is_parse_preserved_idempotent_and_width_bounded`
//   - `crates/ken-elaborator/tests/kenfmt_b4_splicing.rs` (catalog +
//      examples/rosetta + spec walk)
//   - `crates/ken-elaborator/tests/kenfmt_c_capstone.rs`
// All five were run (targeted, `-p ken-cli`/`-p ken-elaborator --test
// <name>`) and stay green under this candidate -- expected, since nothing
// in the corpus uses the new token yet (`FoKripke.ken` is a plain `.ken`
// file, in ZERO of these `.ken.md`-globbing gates, per the node doc's own
// warning). None of the five can exercise `‖`/`||` for that reason, so the
// token's OWN formatter behavior is pinned here instead.
// ---------------------------------------------------------------------

/// The ASCII digraph canonicalizes to the Unicode glyph, exactly like every
/// other dual-spelling token in this table (`Arrow`, `MapsTo`, ...) --
/// `format.rs::canonical_token_spelling`.
#[test]
fn ac6_ascii_truncbar_canonicalizes_to_the_unicode_glyph() {
    let canonicalized = ken_elaborator::format::canonical_unicode("const x : Omega = ||Bool||");
    assert_eq!(
        canonicalized, "const x : Omega = ‖Bool‖",
        "the ASCII digraph must canonicalize to the Unicode glyph on BOTH the open and close \
         occurrence (two independent TruncBar tokens in the stream)"
    );
}

/// Already-canonical Unicode source is an exact fixed point (idempotence --
/// the property every corpus gate above actually checks at scale).
#[test]
fn ac6_unicode_truncbar_source_is_a_canonicalization_fixed_point() {
    let src = "const x : Omega = ‖Bool‖";
    let canonicalized = ken_elaborator::format::canonical_unicode(src);
    assert_eq!(
        canonicalized, src,
        "already-canonical ‖Bool‖ source must canonicalize to itself"
    );
}

/// A parse-invalid fragment containing the new token still round-trips
/// byte-for-byte through the narrow `canonicalize_lexed_tokens` fallback
/// path (used for `` ```ken ignore ``/`` ```ken reject `` fence bodies,
/// `kenfmt_b1`) -- confirms the SAME token-kind table drives both
/// canonicalization entry points, not just the parse-successful one.
#[test]
fn ac6_lexed_token_fallback_canonicalizes_truncbar_in_an_unparseable_fragment() {
    // Deliberately incomplete (`const x : Omega = ||Bool||` with a
    // dangling extra token after it) -- exercises the non-parsing fallback
    // specifically, not the ordinary `parse_lossless` path.
    let fragment = "||Bool|| @@@";
    let canonicalized = ken_elaborator::format::canonicalize_lexed_tokens(fragment)
        .expect("the lexed-token fallback must not itself error on an unparseable fragment");
    assert_eq!(
        canonicalized, "‖Bool‖ @@@",
        "TruncBar must canonicalize even when the surrounding fragment does not parse"
    );
}
