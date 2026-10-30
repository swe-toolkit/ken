use super::*;
use std::io::Read;
use std::os::fd::AsRawFd;

#[test]
fn spawn_plan_passes_exact_argv_and_env_and_reports_exit() {
    let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    plan.push_argument(b"caller-zero".to_vec()).unwrap();
    plan.push_argument(b"-c".to_vec()).unwrap();
    plan.push_argument(
        b"printf '%s:%s:%s' \"$0\" \"${PX10_SELECTED-<unset>}\" \"${HOME-<unset>}\"".to_vec(),
    )
    .unwrap();
    plan.push_environment(b"PX10_SELECTED".to_vec(), b"chosen".to_vec())
        .unwrap();
    plan.set_stdio(1, ChildStdioV1::Pipe).unwrap();
    let mut child = plan.spawn().unwrap();
    let mut output = Vec::new();
    std::fs::File::from(child.take_pipe(1).unwrap())
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(output, b"caller-zero:chosen:<unset>");
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    assert!(child.take_pipe(1).is_err());
    child.settle(false).unwrap();
}

#[test]
fn spawn_plan_closes_unselected_parent_fds_and_restores_sigpipe() {
    let parent_fd = std::fs::File::open("/dev/null").unwrap();
    // SAFETY: fcntl duplicates to a new descriptor at least 100 without CLOEXEC.
    let inherited = unsafe { libc::fcntl(parent_fd.as_raw_fd(), libc::F_DUPFD, 100) };
    assert!(inherited >= 100);
    let mut plan = SpawnPlanV1::new(b"/bin/sh".to_vec()).unwrap();
    plan.push_argument(b"caller-zero".to_vec()).unwrap();
    plan.push_argument(b"-c".to_vec()).unwrap();
    plan.push_argument(b"printf 'ready\\n'; read input || :".to_vec())
        .unwrap();
    plan.set_stdio(0, ChildStdioV1::Pipe).unwrap();
    plan.set_stdio(1, ChildStdioV1::Pipe).unwrap();
    let mut child = plan.spawn().unwrap();
    let pid = child.pid();
    let mut out = std::fs::File::from(child.take_pipe(1).unwrap());
    let mut ready = [0; 6];
    out.read_exact(&mut ready).unwrap();
    assert_eq!(&ready, b"ready\n");
    let mut fds = std::fs::read_dir(format!("/proc/{pid}/fd"))
        .unwrap()
        .map(|fd| {
            fd.unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    fds.sort_unstable();
    assert_eq!(fds, [0, 1, 2]);
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).unwrap();
    let ignored = status
        .lines()
        .find(|line| line.starts_with("SigIgn:"))
        .unwrap();
    let bits = u64::from_str_radix(ignored.split_whitespace().nth(1).unwrap(), 16).unwrap();
    assert_eq!(bits & (1 << (libc::SIGPIPE - 1)), 0);
    // SAFETY: the inherited descriptor is uniquely owned here.
    unsafe { libc::close(inherited) };
    drop(child.take_pipe(0));
    assert_eq!(child.wait().unwrap(), ChildExitV1::Exited(0));
    child.settle(false).unwrap();
}

#[test]
fn spawn_plan_rejects_missing_program_and_malformed_inputs() {
    let mut missing = SpawnPlanV1::new(b"/path/does-not-exist-px10".to_vec()).unwrap();
    missing.push_argument(b"caller-zero".to_vec()).unwrap();
    assert_eq!(
        missing.spawn().unwrap_err().raw_os_error(),
        Some(libc::ENOENT)
    );
    let mut empty = SpawnPlanV1::new(b"/bin/true".to_vec()).unwrap();
    assert_eq!(
        empty.clone().spawn().unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty.push_argument(b"x\0y".to_vec()).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty
            .push_environment(b"BAD=KEY".to_vec(), vec![])
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
    assert_eq!(
        empty.set_stdio(3, ChildStdioV1::Null).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    empty.set_stdio(0, ChildStdioV1::Null).unwrap();
    assert_eq!(
        empty.set_stdio(0, ChildStdioV1::Pipe).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
}
