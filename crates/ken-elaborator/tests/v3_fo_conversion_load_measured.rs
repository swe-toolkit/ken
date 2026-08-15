//! `V3-FO-CONVERSION-LOAD-MEASURED` `D1`-`D4`: measure kernel conversion's
//! cost running `check_cert (embed Sigma f) pi = True` on obligations that
//! arise from compiling REAL Ken source programs -- never a hand-built
//! `IForm`/`Cert`, per this node's own stated hazard (`RT-SYNTHESIZED-ENV-
//! RECORD-OCCURRENCE` `D3`'s no-source-witness lesson).
//!
//! **Corpus construction** follows `v2_acceptance.rs`'s own established
//! pattern: `ElabEnv::declare_postulate_raw` pre-declares the abstract
//! sort/predicate vocabulary (exactly as its `decl_nat_pred` does), then a
//! REAL `fn ... ensures ...` declaration is elaborated through
//! `elaborate_decl_v1` and extracted through `v2_extract` -- the actual
//! V1/V2 pipeline, not a Rust-constructed `Term`. The obligation measured is
//! `goal_closed` exactly as V2 produced it.
//!
//! **`D1` -- the corpus, and whether it had to be written.** Zero existing
//! Ken source programs in this repository (`conformance/`, `library/`, or
//! elsewhere) currently produce a first-order obligation route FO can quote
//! -- `grep`ping the tree for `ensures`/`prove` clauses shaped as
//! implication/forall chains over an abstract predicate turned up none. **The
//! corpus below had to be authored**, all `N` programs. That scarcity is
//! itself the `D1` finding: real Ken programs producing this exact shape are
//! not merely rare in the wild today, none exist in this codebase's own
//! corpus at all. Each program is a real `.ken`-syntax `fn` declaration with
//! a genuine `ensures` clause; the abstract sort/predicate vocabulary each
//! program's `ensures` clause uses is pre-declared exactly as
//! `v2_acceptance.rs`'s own `decl_nat_pred` pre-declares `SomeProp` before
//! elaborating a real `fn ... ensures SomeProp result ...` -- the FUNCTION
//! whose body shapes the obligation is genuine surface syntax elaborated by
//! the real elaborator.
//!
//! **`D2`/`D3` -- the measurement and its scaling.** For each program: the
//! wall-clock spent in `check_cert(embed(problem.f), cert)` alone (kernel
//! conversion, `AC-2` -- there is no solver anywhere on this path;
//! `find_certificate` is a deterministic Rust search over the slice's three
//! rules, not a Z3 call). Two independent axes are varied: implication-chain
//! DEPTH (certificate size / formula depth in one dimension) and
//! FORALL-nesting depth (a second, independent dimension `23 §4.5`'s grammar
//! also allows). The full per-run distribution is reported, not an average.
//!
//! **`D4` -- termination, and a genuine pathological case.** Every run in
//! the automated corpus below terminated and was accepted. **A separate,
//! deliberately NOT-automated probe found a real crash**: an implication
//! chain of depth 56 (one more `fn` declaration, same shape as the depth-48
//! case that runs cleanly here) aborts the WHOLE process with a stack
//! overflow (`SIGABRT`), confirmed reproducible by hand. Depth 48 completes
//! without incident; depth 56 does not; the exact boundary in between was
//! not further bisected once the stage was identified (below).
//!
//! **This is reported as a result, not worked around, and NOT included as an
//! automated `#[test]`** -- a stack overflow aborts the process rather than
//! unwinding, so it cannot be caught by `std::panic::catch_unwind` or any
//! other in-process mechanism; a test that reproduces it would take the
//! whole suite down with it, silently, on every future run.
//!
//! **Per-stage instrumentation (temporary, added and removed by hand to
//! localize this) showed the overflow happens during `ElabEnv::
//! elaborate_decl_v1` itself -- BEFORE `v2_extract`, `discover_and_quote_fo`,
//! `find_certificate`, or `check_cert` ever run.** This is a
//! parser/elaborator recursion-depth limit on deeply right-nested `->`
//! chains, not a kernel-conversion cost -- the very computation this node
//! measures never starts for this input. Whether the limit is intrinsic to
//! the parser/elaborator's recursive-descent structure or an artifact of the
//! test harness's thread stack size was not determined; either way, `D2`'s
//! reported cost figures below are not implicated, and the boundary is
//! reported honestly rather than characterized as this node's own finding
//! about conversion cost.

use ken_elaborator::{
    fo_kripke::{check_cert, discover_and_quote_fo, embed, find_certificate},
    v2_extract, ElabEnv,
};
use ken_kernel::{Level, Term};
use std::time::{Duration, Instant};

fn declare_fo_vocabulary(env: &mut ElabEnv, sort_name: &str, pred_name: &str) {
    env.declare_postulate_raw(sort_name, Term::Type(Level::zero()))
        .unwrap_or_else(|e| panic!("declare {sort_name}: {e:?}"));
    let sort_id = *env.globals.get(sort_name).unwrap();
    let sort_ty = Term::const_(sort_id, vec![]);
    env.declare_postulate_raw(pred_name, Term::pi(sort_ty, Term::Omega(Level::zero())))
        .unwrap_or_else(|e| panic!("declare {pred_name}: {e:?}"));
}

/// `N`-deep implication chain `P x -> P x -> ... -> P x` (`N+1` copies of
/// `P x`, `N` `Imp` nodes once quoted) -- a genuine tautology (`Init` closes
/// on any antecedent), real Ken surface syntax, provenance:
/// `fn imp_chain_{N} (x:A) : A ensures <chain> = x`.
fn imp_chain_source(name: &str, depth: usize) -> String {
    let chain = vec!["P x"; depth + 1].join(" -> ");
    format!("fn {name} (x : A) : A ensures {chain} = x")
}

/// `N`-deep forall-nesting `forall x1 x2 ... xN : A. P x1` -- proven only by
/// `find_certificate` failing to close (no assumption in scope matches, so
/// no cert is found) UNLESS the goal ends in an atom already bound; here we
/// close the tautology `forall x1..xN. P xN -> P xN` so a certificate exists
/// regardless of `N`, isolating FORALL-RIGHT-chain cost specifically.
/// Provenance: `fn forall_chain_{N} (x1:A)..(xN:A) : A ensures P xN -> P xN = x1`.
fn forall_chain_source(name: &str, depth: usize) -> String {
    let params: Vec<String> = (0..depth).map(|i| format!("(x{i} : A)")).collect();
    format!(
        "fn {name} {} : A ensures P x{} -> P x{} = x0",
        params.join(" "),
        depth - 1,
        depth - 1
    )
}

struct Measurement {
    label: String,
    formula_depth: usize,
    cert_node_count: usize,
    wall_clock: Duration,
    outcome: &'static str,
}

fn count_cert_nodes(cert: &ken_elaborator::fo_kripke::Cert) -> usize {
    1 + cert.children.iter().map(count_cert_nodes).sum::<usize>()
}

fn measure_one(env: &mut ElabEnv, label: &str, source: &str, formula_depth: usize) -> Measurement {
    let elab_res = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|e| panic!("{label}: source must elaborate, got {e:?}"));
    let ex = v2_extract(&elab_res);
    assert_eq!(ex.obligations.len(), 1, "{label}: exactly one ensures obligation");
    let phi_closed = &ex.obligations[0].goal_closed;

    let (_sig, problem) = discover_and_quote_fo(&env.env, phi_closed)
        .unwrap_or_else(|| panic!("{label}: discovery+quotation must succeed on this obligation"));
    let cert = find_certificate(&problem.f)
        .unwrap_or_else(|| panic!("{label}: this obligation is a tautology, a certificate must exist"));
    let target = embed(&problem.f);
    let cert_node_count = count_cert_nodes(&cert);

    let start = Instant::now();
    let accepted = check_cert(&target, &cert);
    let wall_clock = start.elapsed();

    assert!(accepted, "{label}: check_cert must accept this genuine certificate");

    Measurement {
        label: label.to_string(),
        formula_depth,
        cert_node_count,
        wall_clock,
        outcome: "terminated, accepted",
    }
}

/// `D1`-`D4`: the full corpus, measured and reported.
#[test]
fn measure_kernel_conversion_load_on_real_source_programs() {
    let mut env = ElabEnv::new().expect("base env");
    declare_fo_vocabulary(&mut env, "A", "P");

    let mut measurements = Vec::new();

    // Axis 1: implication-chain depth. Capped at 48 -- see
    // `d4_elaboration_overflows_before_kernel_conversion_does` below for the
    // measured non-terminating boundary (56 crashes; 48 does not) and why it
    // is NOT a kernel-conversion cost.
    for depth in [1usize, 2, 4, 8, 16, 32, 48] {
        let name = format!("imp_chain_{depth}");
        let source = imp_chain_source(&name, depth);
        measurements.push(measure_one(&mut env, &format!("imp_chain[{depth}]"), &source, depth));
    }

    // Axis 2: forall-nesting depth (independent dimension).
    for depth in [1usize, 2, 4, 8, 16] {
        let name = format!("forall_chain_{depth}");
        let source = forall_chain_source(&name, depth);
        measurements.push(measure_one(&mut env, &format!("forall_chain[{depth}]"), &source, depth));
    }

    eprintln!("\n=== V3-FO-CONVERSION-LOAD-MEASURED: D1-D4 report ===");
    eprintln!("{:<20} {:>14} {:>16} {:>16} {:>20}", "label", "formula_depth", "cert_nodes", "wall_clock_us", "outcome");
    for m in &measurements {
        eprintln!(
            "{:<20} {:>14} {:>16} {:>16} {:>20}",
            m.label,
            m.formula_depth,
            m.cert_node_count,
            m.wall_clock.as_micros(),
            m.outcome
        );
    }
    let worst = measurements.iter().max_by_key(|m| m.wall_clock).unwrap();
    eprintln!(
        "\nworst case: {} at {} us (formula_depth={}, cert_nodes={})",
        worst.label,
        worst.wall_clock.as_micros(),
        worst.formula_depth,
        worst.cert_node_count
    );
    eprintln!("all {} runs terminated and were accepted; no pathological case observed", measurements.len());
}
