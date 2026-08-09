//! `RT-MATCH-RECURSOR-CONSUMERS` `AC-1`, frame section 4a -- the CROSS-CRATE census.
//!
//! The in-crate census merged at `28edeb00` is a partial: both of its recorders
//! are `#[cfg(test)]` in `ken-runtime`, so they reach only that crate's own unit
//! tests. This suite is the other side -- a real `ken-cli` native compilation,
//! observed through the feature-gated recorder 4a authorizes.
//!
//! **Observation only.** The recorder cannot remove a residual, set an
//! exclusion, choose an authority, or alter any planner/ABI value; the parity
//! row below is what holds that to account rather than asserting it.

/// Verbatim from `px7m_hostresult_computational_match.rs`, so this suite drives
/// a program already known to elaborate and reach linked native lowering. A
/// census whose fixture is novel measures the fixture first.
const CENSUS_PROGRAM: &str = r#"program capabilities FS APartial
proc two_step (label : String) : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit Unit
    (host_console APartial Unit (print_line label))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

proc after_write (written : Result IOError Unit)
  : HostIO APartial Unit visits [Console] =
  match written {
    Err _ |-> two_step "unexpected-error" ;
    Ok unit |-> match unit { MkUnit |-> two_step "ok-payload" }
  }

proc inner : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "probe:")))
    after_write

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit ExitCode inner (\_. host_exit APartial Success)
"#;

fn output_dir(case: &str) -> std::path::PathBuf {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(case);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn build(case: &str) -> bool {
    let dir = output_dir(case);
    ken_cli::build_native_program(
        CENSUS_PROGRAM,
        ken_cli::SourceFormat::Ken,
        "mrc_4a_census",
        &dir,
    )
    .is_ok()
}

/// **The three controls section 4a requires, plus the census itself.**
#[test]
fn mrc_4a_cross_crate_census_and_its_controls() {
    // CONTROL 3 -- parity. The same compile outside any scope, then inside one.
    // If these disagree the recorder changed a result and everything else here
    // is void, so it is asserted first.
    let outside = build("mrc_4a_outside");

    let (inside, rows) =
        ken_runtime::with_match_recursor_census(|| build("mrc_4a_inside"));

    assert_eq!(
        inside, outside,
        "recorder-on and recorder-off must agree on the compile result; a difference means the \
         census is activation, not observation"
    );

    // The scope must have SEEN something. A silently empty census and a
    // genuinely empty population are the two readings this must never conflate.
    assert!(
        !rows.is_empty(),
        "the scope captured no compilation at all -- the cross-crate export path is broken, or \
         the compile ran on another thread. Either way this is a BROKEN INSTRUMENT, not an empty \
         population"
    );

    // CONTROL 1 -- captured exactly once. Row identity is three-part, and the
    // ordinals must be dense from zero: a colliding key would DEDUPLICATE, so
    // the population would read smaller and cleaner than it is.
    let mut keys: Vec<(u32, &str, u64)> = rows
        .iter()
        .map(|row| (row.run, row.thread.as_str(), row.ordinal))
        .collect();
    let before = keys.len();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(
        keys.len(),
        before,
        "every row must carry a distinct (run, thread, ordinal) key; a duplicate means the census \
         is deduplicating compilations"
    );
    let ordinals: Vec<u64> = rows.iter().map(|row| row.ordinal).collect();
    assert_eq!(
        ordinals,
        (0..rows.len() as u64).collect::<Vec<_>>(),
        "ordinals must be dense and in order, or a compilation went unrecorded"
    );

    // The equation `entry = selector-arrival + pre-selector-return`, preserved
    // by construction: every entry is a row, and these two partition them.
    let arrived = rows.iter().filter(|row| row.reached_selector).count();
    let returned_early = rows.iter().filter(|row| !row.reached_selector).count();
    assert_eq!(
        arrived + returned_early,
        rows.len(),
        "entry must equal selector-arrival plus pre-selector-return"
    );
    for row in rows.iter().filter(|row| row.reached_selector) {
        assert!(
            row.validator_admitted,
            "a row that reached the selector must have been admitted by the transport validator"
        );
        assert!(
            row.authority.is_some(),
            "a row that reached the selector must carry production's own authority"
        );
    }
    for row in rows.iter().filter(|row| !row.reached_selector) {
        assert!(
            row.authority.is_none(),
            "a row that never reached the selector cannot carry an authority"
        );
    }

    // CONTROL 2 -- a known non-member is still a non-member. This program is
    // not in the A population, and the census must say so rather than reporting
    // nothing at all.
    let firing: Vec<&ken_runtime::MatchRecursorCensusRow> = rows
        .iter()
        .filter(|row| {
            row.residuals
                .iter()
                .any(|residual| residual == "MatchScrutineeRecursor")
        })
        .collect();

    // The census result, printed so the handback quotes measurement rather than
    // an assertion's absence.
    println!("MRC-4A-ROWS\t{}", rows.len());
    println!("MRC-4A-ARRIVED\t{arrived}");
    println!("MRC-4A-PRESELECTOR-RETURN\t{returned_early}");
    println!("MRC-4A-FIRING\t{}", firing.len());
    for row in &rows {
        println!(
            "MRC-4A-ROW\trun={}\tthread={}\tordinal={}\tresiduals={:?}\tadmitted={}\tselector={}\tauthority={:?}",
            row.run, row.thread, row.ordinal, row.residuals,
            row.validator_admitted, row.reached_selector, row.authority
        );
    }

    assert!(
        firing.is_empty(),
        "this fixture is a known NON-MEMBER; a MatchScrutineeRecursor row here is a real AC-1 \
         population member and must be returned through the D1/hard-stop path, not asserted away: \
         {firing:?}"
    );
}
