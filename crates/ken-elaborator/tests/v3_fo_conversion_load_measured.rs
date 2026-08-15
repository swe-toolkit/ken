//! `V3-FO-CONVERSION-LOAD-MEASURED` `D1`-`D4`: measure the elaborator's Rust
//! reference checker's cost running `check_cert(embed(Sigma, f), pi)` on
//! obligations that arise from compiling REAL Ken source programs -- never a
//! hand-built `IForm`/`Cert`, per this node's own stated hazard
//! (`RT-SYNTHESIZED-ENV-RECORD-OCCURRENCE` `D3`'s no-source-witness lesson).
//!
//! **Corpus construction** follows `v2_acceptance.rs`'s own established
//! pattern: `ElabEnv::declare_postulate_raw` pre-declares the abstract
//! sort/predicate vocabulary (exactly as its `decl_nat_pred` does), then a
//! REAL `fn ... ensures ...` declaration is elaborated through
//! `elaborate_decl_v1` and extracted through `v2_extract` -- the actual
//! V1/V2 pipeline, not a Rust-constructed `Term`. The obligation measured is
//! `goal_closed` exactly as V2 produced it.
//!
//! # `AC-2` CORRECTION -- this does NOT measure kernel conversion
//!
//! **Architect review, `evt_7cmys9wyp7k8c`, on `b52d160c8`; Steward scope
//! ruling amending `AC-2`, `evt_6m3q3tsvg09pz`.** The original framing
//! claimed this measured "kernel conversion". It does not, and the gap is
//! structural, not a measurement error:
//!
//! **What is timed is `ken_elaborator::fo_kripke::check_cert`
//! (`fo_kripke.rs:807`), a native Rust function recursing through a Rust
//! `check_tree` over Rust `Form`/`Cert` structs.** `23 §4.4`'s cost model
//! calls for kernel conversion evaluating a KEN-LEVEL `check_cert` -- per
//! `conformance/verify/prover/seed-prover.md:49-50`, *"`check_cert` = the
//! **Ken-level reflective Bool checker** over quoted formulas (`23 §4` route
//! (a)) -- an ordinary **kernel-checked function**, distinct from the kernel
//! API `check`"*. **That Ken-level artifact -- a `check_cert`/`embed`/
//! `Form`/`Cert` expressed as Ken terms the kernel itself reduces -- does
//! not exist anywhere in this tree** (`grep -rn check_cert library/
//! catalog/` is empty). The quantity `AC-2` originally named is **not
//! takeable at this SHA**, because the artifact it would be taken on has not
//! been authored. Route (a)'s implementation -- authoring that Ken-level
//! checker -- is filed as its own successor,
//! `V3-FO-KEN-LEVEL-CHECKER-AUTHORING`, and is explicitly NOT this node's
//! scope.
//!
//! **`AC-2` is amended, not abandoned.** What this file measures is now
//! stated as what it is: **the elaborator's Rust reference checker over the
//! same certificate trees a Ken-level checker would receive.** `cert_nodes`
//! (the structural size of the derivation) is the transferable part -- the
//! same certificate, the same tree shape, would be the input to the
//! Ken-level checker once one exists. The wall-clock numbers below are **a
//! lower bound of unknown tightness on route (a)'s real cost, and they say
//! NOTHING about termination under kernel reduction** -- the argued-but-not-
//! mechanized half of `18 §6` that `docs/design/fo-route-theorem-home.md`
//! §4 names as the actual open question. **These numbers must never be read
//! or cited as "the cost of route (a)".** `AC-2`'s solver exclusion is
//! unchanged: there is no solver anywhere on this path; `find_certificate`
//! is a deterministic Rust search over the slice's three rules, never a Z3
//! call.
//!
//! **Profile: debug**, matching both `ken-cargo test` locally and CI's
//! `cargo nextest run --workspace --locked` (`.github/workflows/ci.yml`).
//! Microsecond figures are debug-profile figures, not release; no release
//! run was taken.
//!
//! **`D1` -- the corpus, and whether it had to be written.** Zero existing
//! Ken source programs in this repository (`conformance/`, `library/`, or
//! elsewhere) currently produce a first-order obligation route FO can quote
//! -- `grep`ping the tree for `ensures`/`prove` clauses shaped as
//! implication/forall chains over an abstract predicate turned up none. **The
//! corpus below had to be authored**, all 13 programs. That scarcity is
//! itself the `D1` finding: real Ken programs producing this exact shape are
//! not merely rare in the wild today, none exist in this codebase's own
//! corpus at all -- no Ken program in this repository would exercise route
//! FO at all if it existed today. Each program is a real `.ken`-syntax `fn`
//! declaration with a genuine `ensures` clause; the abstract sort/predicate
//! vocabulary each program's `ensures` clause uses is pre-declared exactly
//! as `v2_acceptance.rs`'s own `decl_nat_pred` pre-declares `SomeProp`
//! before elaborating a real `fn ... ensures SomeProp result ...` -- the
//! FUNCTION whose body shapes the obligation is genuine surface syntax
//! elaborated by the real elaborator. Every row's exact source string is
//! printed by the corpus generators below and reproduced in the measured
//! table (`AC-1`: demonstrate the provenance, not merely assert it).
//!
//! **`D2`/`D3` -- the measurement and its scaling.** For each program: the
//! wall-clock spent in `check_cert(embed(problem.f), cert)` alone (the Rust
//! reference checker, per the `AC-2` correction above). Two independent axes
//! are varied: implication-chain DEPTH (certificate size / formula depth in
//! one dimension) and FORALL-nesting depth (a second, independent dimension
//! `23 §4.5`'s grammar also allows). **The measured distribution, taken
//! 2026-08-15, debug profile:**
//!
//! ```text
//! label            formula_depth   cert_nodes   wall_clock_us
//! imp_chain[1]                 1           10              56
//! imp_chain[2]                 2           13              76
//! imp_chain[4]                 4           19             148
//! imp_chain[8]                 8           31             265
//! imp_chain[16]                16          55             884
//! imp_chain[32]                32         103            2417
//! imp_chain[48]                48         151            4914
//! imp_chain[64]                64         199            9077
//! forall_chain[1]               1          10              61
//! forall_chain[2]               2          14              89
//! forall_chain[4]               4          22             143
//! forall_chain[8]               8          38             325
//! forall_chain[16]             16          70            1089
//! ```
//!
//! (This 13-row table, on the 256 MiB thread, `2026-08-15`, this SHA.)
//!
//! Both axes show consistent, roughly-quadratic growth in `cert_nodes` and
//! wall-clock alike -- doubling depth roughly triples-to-quadruples the
//! time. The report is the distribution, not an average; the worst observed
//! case is `imp_chain[64]` and is called out explicitly by the test itself.
//! A fresh run may print slightly different microsecond figures (machine
//! load, not the mechanism); this table is the durable record `D2`/`D3`
//! require, not merely captured `eprintln!` output invisible without
//! `--nocapture`.
//!
//! **`D4` -- termination, corrected.** The original candidate reported an
//! implication chain of depth 56 as "a genuine pathological case" -- a
//! stack overflow aborting the process. **That finding was wrong, and this
//! repository had already built the instrument that shows it.** Four other
//! test files in this exact directory (`cc3_parsing_cursor_decoder_
//! acceptance.rs`, `l3_strings_surface_acceptance.rs`,
//! `map_build_acceptance.rs`, `r3_c2_source_mixed_branch.rs`) already run
//! deep-recursion cases on an oversized 256 MiB test thread, precisely
//! because the DEFAULT 8 MiB test-thread stack hits "plain (non-algorithmic)
//! stack exhaustion" on ordinary deep recursion, independent of any
//! algorithmic defect (`l3_strings_surface_acceptance.rs:102-104`, verbatim
//! in that file's own doc comment). This test now uses that SAME
//! `run_with_big_stack` pattern.
//!
//! **Under the 256 MiB thread, depth 56 elaborates without incident, and so
//! does every depth tried up to 1024** (probed by hand: 56, 64, 128, 256,
//! 512, 1024, all `Ok`, all producing exactly one obligation; probe not
//! retained -- the corpus below is the durable evidence, extended to depth
//! 64 for CI margin past the original false boundary). **The depth-56 crash
//! was the harness's default-stack limit, not a property of the elaborator.**
//! Per-stage instrumentation in the earlier turn (added and removed by hand)
//! had already shown the overflow occurred inside `ElabEnv::
//! elaborate_decl_v1` itself, before `v2_extract`/`discover_and_quote_fo`/
//! `find_certificate`/`check_cert` ever ran -- that finding stands and is
//! now explained: ordinary recursive-descent parsing/elaboration of a
//! deeply right-nested `->` chain, under a stack too small for it, exactly
//! the class the four sibling test files already guard against.
//!
//! **The corrected `D4` result: no pathological case was found.** Every run
//! in the corpus (now including depth 64, past the original false boundary)
//! terminates and is accepted, on the same 256 MiB thread the shipped test
//! now uses -- which also gives the corpus itself CI margin it did not
//! previously have (`AC-5`; 48 was never bisected against 49..55 before, and
//! CI is different hardware from the authoring box).
//!
//! **The original doc's claim that an abort "would take the whole suite
//! down with it, silently, on every future run" was also wrong and is
//! retracted.** CI runs `cargo nextest run --workspace --locked`
//! (`.github/workflows/ci.yml`), which executes each test in its own
//! process; an abort there surfaces as one attributed test failure, not a
//! silent whole-suite loss. It would have taken the whole `ken-elaborator`
//! test binary down only under a local `cargo test` of that target. Both
//! points are moot now that no abort occurs at all in the shipped corpus.

use ken_elaborator::{
    fo_kripke::{check_cert, discover_and_quote_fo, embed, find_certificate},
    v2_extract, ElabEnv,
};
use ken_kernel::{Level, Term};
use std::time::{Duration, Instant};

/// Every acceptance run here uses a real deep-recursion depth (implication
/// chains up to 64, forall-nesting up to 16) -- an oversized default
/// test-thread stack avoids the plain (non-algorithmic) stack exhaustion
/// this hits under the default 8 MiB stack, exactly the class
/// `l3_strings_surface_acceptance.rs`'s own `run_with_big_stack` (its
/// identical body, reused here) already guards against in this same
/// directory (`D4` correction, module doc above).
fn run_with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(f)
        .expect("spawn big-stack test thread")
        .join()
        .expect("test thread panicked");
}

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

/// `N`-deep forall-nesting: the FUNCTION binds `N` object parameters
/// `x0:A .. x{N-1}:A`, each Pi-closed into `goal_closed` by V2 extraction
/// (`22 §2.2`, one `Forall` per parameter once quoted), and the `ensures`
/// clause itself is the tautology `P x{N-1} -> P x{N-1}` -- so the quoted
/// obligation is `forall x0 .. forall x{N-1}. (P x{N-1} -> P x{N-1})`. The
/// tautology (not a bare atom) is deliberate: a bare `forall x. P x` has no
/// assumption in scope for `Init` to close against, so `find_certificate`
/// would find nothing regardless of `N`, measuring nothing. Closing on the
/// innermost bound variable's own tautology isolates FORALL-RIGHT-chain
/// cost specifically, with a real certificate at every depth. Provenance:
/// `fn forall_chain_{N} (x0:A)..(x{N-1}:A) : A ensures P x{N-1} -> P x{N-1} = x0`.
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
    source: String,
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

    // `AC-2` correction: this times the Rust reference `check_cert` alone
    // (native recursion over `Form`/`Cert`), never kernel conversion --
    // see the module doc for why that quantity is not takeable at this SHA.
    let start = Instant::now();
    let accepted = check_cert(&target, &cert);
    let wall_clock = start.elapsed();

    assert!(accepted, "{label}: check_cert must accept this genuine certificate");

    Measurement {
        label: label.to_string(),
        source: source.to_string(),
        formula_depth,
        cert_node_count,
        wall_clock,
        outcome: "terminated, accepted",
    }
}

/// `D1`-`D4`: the full corpus, measured and reported. Runs on a 256 MiB
/// thread (`D4` correction, module doc) so the corpus's own margin past the
/// original false depth-56 boundary is real, not merely asserted.
#[test]
fn measure_kernel_conversion_load_on_real_source_programs() {
    run_with_big_stack(|| {
        let mut env = ElabEnv::new().expect("base env");
        declare_fo_vocabulary(&mut env, "A", "P");

        let mut measurements = Vec::new();

        // Axis 1: implication-chain depth, extended to 64 -- past the
        // original false depth-56 boundary (a default-8-MiB-thread
        // artifact, corrected above), now with real margin on this 256 MiB
        // thread.
        for depth in [1usize, 2, 4, 8, 16, 32, 48, 64] {
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
        eprintln!(
            "{:<20} {:>14} {:>16} {:>16} {:>20}",
            "label", "formula_depth", "cert_nodes", "wall_clock_us", "outcome"
        );
        for m in &measurements {
            eprintln!(
                "{:<20} {:>14} {:>16} {:>16} {:>20}  source: {}",
                m.label,
                m.formula_depth,
                m.cert_node_count,
                m.wall_clock.as_micros(),
                m.outcome,
                m.source
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
        eprintln!(
            "all {} runs terminated and were accepted; no pathological case observed \
             (D4 -- see module doc for the corrected depth-56 finding)",
            measurements.len()
        );
    });
}
