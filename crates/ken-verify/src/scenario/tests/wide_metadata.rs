//! The handle body consumes the field after a carrier crossing. The path
//! reply is observed but deliberately does not supply the Int under test.

use super::*;
use rustix::fs::{statx, AtFlags, StatxFlags as F, StatxTimestamp};
use std::fs::{self, DirBuilder, File, FileTimes};
use std::os::unix::fs::DirBuilderExt;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const EDGE: i128 = 1 << 55;
const BILLION: i128 = 1_000_000_000;

struct TmpfsRoot(PathBuf);

impl TmpfsRoot {
    fn new(row: &str) -> Self {
        // Linux tmpfs retains signed and large timestamps and supports the
        // sparse 2^55-byte file without allocating any file data. The normal
        // test scratch filesystem need not support either. Assert the exact
        // per-case statx premise rather than falling back or skipping.
        let root = PathBuf::from(format!("/dev/shm/ken-wide-{}-{row}", std::process::id()));
        DirBuilder::new()
            .mode(0o700)
            .create(&root)
            .expect("create dedicated /dev/shm metadata root");
        Self(root)
    }
}

impl Drop for TmpfsRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Copy)]
enum Field {
    Accessed(Option<i128>),
    Modified,
    Changed,
    Born,
    Size(i128),
}

fn system_time(ns: i128) -> SystemTime {
    let magnitude = ns.unsigned_abs();
    let duration = Duration::new(
        (magnitude / BILLION as u128).try_into().unwrap(),
        (magnitude % BILLION as u128).try_into().unwrap(),
    );
    if ns < 0 {
        UNIX_EPOCH - duration
    } else {
        UNIX_EPOCH + duration
    }
}

fn raw_ns(time: StatxTimestamp) -> i128 {
    i128::from(time.tv_sec) * BILLION + i128::from(time.tv_nsec)
}

fn handle_check(row: &str, field: Field, expected_exit: i32) {
    let root = TmpfsRoot::new(row);
    let fd = File::create(root.0.join("version")).unwrap();
    match field {
        Field::Accessed(ns) => fd
            .set_times(FileTimes::new().set_accessed(ns.map_or_else(SystemTime::now, system_time)))
            .unwrap(),
        Field::Modified => fd
            .set_times(FileTimes::new().set_modified(system_time(EDGE)))
            .unwrap(),
        Field::Changed | Field::Born => (),
        Field::Size(size) => fd.set_len(size.try_into().unwrap()).unwrap(),
    }
    let st = statx(&fd, "", AtFlags::EMPTY_PATH, F::BASIC_STATS | F::BTIME).unwrap();
    let mask = F::from_bits_retain(st.stx_mask);
    let (field_name, flag, value, exact_fixture_value) = match field {
        Field::Accessed(ns) => ("accessed", F::ATIME, raw_ns(st.stx_atime), ns),
        Field::Modified => ("modified", F::MTIME, raw_ns(st.stx_mtime), Some(EDGE)),
        Field::Changed => ("changed", F::CTIME, raw_ns(st.stx_ctime), None),
        Field::Born => ("born", F::BTIME, raw_ns(st.stx_btime), None),
        Field::Size(size) => ("size", F::SIZE, i128::from(st.stx_size), Some(size)),
    };
    assert!(
        mask.contains(flag),
        "{row}: fixture statx mask {mask:?} lacks {flag:?}"
    );
    if let Some(exact) = exact_fixture_value {
        assert_eq!(value, exact, "{row}: independent raw statx premise");
    } else {
        assert!(
            value >= EDGE,
            "{row}: present-day statx time must be wide: {value}"
        );
    }

    // The controls have the same consumer and one known unequal expected
    // value. In particular a consumer that always answers True fails them.
    let oracle = if expected_exit == 32 { EDGE } else { value };
    let oracle = if oracle < 0 {
        format!("sub_int (0 : Int) ({} : Int)", -oracle)
    } else {
        format!("{oracle} : Int")
    };
    let check = if matches!(field, Field::Size(_)) {
        format!("eq_int size ({oracle})")
    } else {
        format!(
            "match {field_name} {{ None |-> False; Some instant |-> \
             match instant {{ Instant.MkInstant n |-> eq_int n ({oracle}) }} }}"
        )
    };
    let scenario = abi_s4_both_metadata_scenario(row, "True", &check);
    let run = execute_scenario_with_roots(&scenario, Some(SharedMetadataRoot::Existing(&root.0)))
        .unwrap_or_else(|error| panic!("{row}: handle consumer failed: {error:?}"));
    assert_eq!(
        run.interpreter.exit_status, expected_exit,
        "{row}: interpreter"
    );
    assert_eq!(run.native.exit_status, expected_exit, "{row}: native");
    run.compare_exact()
        .expect("same handle outcome and canonical observations");
}

macro_rules! handle_case {
    ($name:ident, $field:expr, $exit:literal) => {
        #[test]
        fn $name() {
            handle_check(stringify!($name), $field, $exit);
        }
    };
}

handle_case!(accessed_spill_positive, Field::Accessed(Some(EDGE)), 0);
handle_case!(accessed_spill_negative, Field::Accessed(Some(-EDGE - 1)), 0);
handle_case!(accessed_present, Field::Accessed(None), 0);
handle_case!(
    accessed_big,
    Field::Accessed(Some(15_000_000_000_000_000_000)),
    0
);
handle_case!(modified_spill, Field::Modified, 0);
handle_case!(changed_present, Field::Changed, 0);
handle_case!(born_present, Field::Born, 0);
handle_case!(size_spill, Field::Size(EDGE), 0);
handle_case!(accessed_immediate_max, Field::Accessed(Some(EDGE - 1)), 32);
handle_case!(accessed_immediate_min, Field::Accessed(Some(-EDGE)), 32);
handle_case!(size_immediate_max, Field::Size(EDGE - 1), 32);

#[test]
fn non_metadata_carried_arithmetic() {
    let mut scenario = abi_s4_both_metadata_scenario("carried-wide-arithmetic", "True", "True");
    scenario.entry.source = include_str!("wide_arithmetic.ken").to_owned();
    scenario.expected_fs.clear();
    scenario.process_input.arguments = vec![b"pin".to_vec()];
    let run = execute_scenario_with_roots(&scenario, None)
        .unwrap_or_else(|error| panic!("non-metadata carried arithmetic failed: {error:?}"));
    assert_eq!(run.interpreter.exit_status, 0, "interpreter arithmetic");
    assert_eq!(run.native.exit_status, 0, "native arithmetic");
    run.compare_exact().expect("carried arithmetic parity");
}
