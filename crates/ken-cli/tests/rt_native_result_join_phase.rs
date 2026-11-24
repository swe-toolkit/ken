#![cfg(target_os = "linux")]

use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

// Spec 42 §5, 45 §4. Promise class: durable native/interpreter parity.
// MEASURED: checked source builds and either lane returns the known status
// with empty program streams for the same argv and filesystem inputs.
// CLAIMED: merging a producer under a Result consumer preserves its selected
// exit, including when the producer's own result is a carried payload.
// THE GAP: the ignored Ok payload is not observed, and these cases cover
// only the measured producer routes. Per-site reverts distinguish those
// routes; this test does not pin compiler representations or diagnostics.
struct Fixture {
    root: tempfile::TempDir,
    source: PathBuf,
    build: PathBuf,
}

impl Fixture {
    fn new(label: &str, source: &str) -> Self {
        let root = tempfile::tempdir().expect("isolated program root");
        let catalog = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog");
        std::os::unix::fs::symlink(catalog, root.path().join("catalog")).unwrap();
        let path = root.path().join(format!("{label}.ken"));
        std::fs::write(&path, source).unwrap();
        let profile = root.path().join("profile.json");
        std::fs::write(
            &profile,
            include_str!("fixtures/rt_native_result_join_phase/profile.json"),
        )
        .unwrap();
        let build = root.path().join("build");
        let output = Command::new(env!("CARGO_BIN_EXE_ken"))
            .args(["native-build"])
            .arg(&path)
            .arg(&build)
            .arg(&profile)
            .current_dir(root.path())
            .env_remove("RUST_MIN_STACK")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{label}: native-build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Self {
            root,
            source: path,
            build,
        }
    }

    fn interpreted(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_ken"))
            .arg("run")
            .arg(&self.source)
            .arg("--")
            .args(args)
            .current_dir(self.root.path())
            .env_clear()
            .output()
            .unwrap()
    }

    fn assert_argv0(&self, label: &str, argv0: &str, expected: i32) {
        let native = Command::new(self.build.join("ken-starter"))
            .arg0(argv0)
            .current_dir(self.root.path())
            .env_clear()
            .output()
            .unwrap();
        assert_outcomes(label, self.interpreted(&[argv0]), native, expected);
    }

    fn assert_no_args(&self) {
        // Linux can insert an empty argv[0] for execve with an empty vector.
        // To exercise Nil rather than Cons "", call the emitted C starter
        // with argc=0 and link the unchanged native object and support archive.
        let wrapper = self.build.join("empty-argv.c");
        std::fs::write(
            &wrapper,
            "extern int ken_test_process_main(int, char **, char **);\n\
             int main(void) { char *empty[] = {0};\n\
               return ken_test_process_main(0, empty, empty); }\n",
        )
        .unwrap();
        let native = self.run_starter_with_args_wrapper("empty-argv", &wrapper, &[]);
        assert_outcomes("no args", self.interpreted(&[]), native, 74);
    }

    fn run_starter_with_args_wrapper(
        &self,
        label: &str,
        wrapper: &Path,
        linker_options: &[&str],
    ) -> Output {
        let shim = self.build.join(format!("{label}-starter.o"));
        run_cc(
            Command::new("cc")
                .args(["-Dmain=ken_test_process_main", "-c"])
                .arg(self.build.join("ken-entrypoint-main.c"))
                .arg("-o")
                .arg(&shim),
        );
        let archive = std::env::var_os("KEN_RUNTIME_SUPPORT_STATICLIB")
            .expect("the test runner supplies the runtime support archive");
        let executable = self.build.join(label);
        run_cc(
            Command::new("cc")
                .arg("-Wl,--gc-sections")
                .args(linker_options)
                .arg(self.build.join("ken-entrypoint.o"))
                .arg(&shim)
                .arg(&wrapper)
                .arg(archive)
                .args(["-ldl", "-lpthread", "-lm", "-o"])
                .arg(&executable),
        );
        Command::new(executable)
            .current_dir(self.root.path())
            .env_clear()
            .output()
            .unwrap()
    }
}

fn run_cc(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_outcomes(label: &str, interpreted: Output, native: Output, expected: i32) {
    assert_eq!(
        interpreted.status.code(),
        Some(expected),
        "{label}: {interpreted:?}"
    );
    assert_eq!(
        native.status.code(),
        interpreted.status.code(),
        "{label}: {native:?}"
    );
    assert!(
        interpreted.stdout.is_empty() && interpreted.stderr.is_empty(),
        "{label}: {interpreted:?}"
    );
    assert_eq!(native.stdout, interpreted.stdout, "{label}: stdout parity");
    assert_eq!(native.stderr, interpreted.stderr, "{label}: stderr parity");
}

macro_rules! argv_cases {
    ($test:ident, $fixture:literal, $($input:expr => $exit:literal),+ $(,)?) => {
        #[test]
        fn $test() {
            let fixture = Fixture::new($fixture, include_str!(concat!(
                "fixtures/rt_native_result_join_phase/", $fixture, ".ken"
            )));
            $(fixture.assert_argv0(&format!("argv[0]={:?}", $input), $input, $exit);)+
        }
    };
}

argv_cases!(borrowed_nested_bool, "q9", "ab" => 0, "x" => 75, &"a".repeat(108) => 75);
argv_cases!(unused_helper_removed, "jqa", "ab" => 0, "x" => 75);
argv_cases!(single_bool, "jqb", "ab" => 0, "x" => 75);
argv_cases!(ok_in_outer_arm, "jqc", "ab" => 0, "x" => 75);
argv_cases!(borrowed_option, "q8", "/" => 0, "x" => 75, "" => 75);
argv_cases!(borrowed_socket_body_without_nul_walk, "q6", "/k" => 0, "ab" => 75, "/" => 75, &format!("/{}", "a".repeat(107)) => 75);
argv_cases!(static_socket_body_without_nul_walk, "q7", "/k" => 0, "ab" => 75);
argv_cases!(carried_bool_with_borrowed_payload, "q1", "ab" => 0, "" => 75);
argv_cases!(carried_bool_with_static_payload, "q2", "ab" => 0, "" => 75);
argv_cases!(static_payload_under_nested_bool, "jqd", "ab" => 0, "x" => 75);

#[test]
fn dynamic_file_error_cause() {
    let fixture = Fixture::new(
        "d2-dynamic",
        include_str!("fixtures/rt_native_result_join_phase/d2-dynamic.ken"),
    );
    let path = fixture.root.path().join("d2-missing.bin");
    fixture.assert_argv0("missing file", "x", 0);
    std::fs::write(&path, b"file").unwrap();
    fixture.assert_argv0("regular file", "x", 75);
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    // Interpreter observed IsDirectory on the assessed Linux host. The
    // property here is the non-NotFound arm, not its particular cause tag.
    fixture.assert_argv0("directory", "x", 75);
    fixture.assert_no_args();
}

// Spec 38 §1.13, 42 §5, 45 §4. Promise class: durable parity invariant.
// MEASURED: both lanes see the same path bytes and produce the expected exit.
// CLAIMED: the public socket address constructor performs its path validation
// after native lowering, including the interior-NUL check on borrowed Bytes.
// THE GAP: this observes validation, not the returned address or socket I/O.
#[test]
fn socket_unix_path_matches_interpreter() {
    let source = include_str!("fixtures/rt_native_result_join_phase/socket_unix_path.ken");
    let fixture = Fixture::new("socket-unix-path", source);
    fixture.assert_argv0("absolute path", "/tmp/k.sock", 0);
    fixture.assert_argv0("relative path", "k.sock", 75);
    fixture.assert_argv0(
        "108-byte absolute path",
        &format!("/{}", "a".repeat(107)),
        75,
    );

    // execve cannot pass interior NULs. Use the same byte slice directly in
    // the interpreter and in the generated native starter's argv packer.
    // The C wrapper supplies the length for this one buffer only. It keeps
    // the native object's borrowed argv path and the Ken fixture unchanged.
    let path = b"/tmp/a\0b";
    let roots = [Path::new(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")];
    let mut host = ken_interp::PosixHost::new_at(fixture.root.path());
    let interpreted = ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &roots,
        &[path.to_vec()],
        &[],
        fixture.root.path().as_os_str().as_encoded_bytes(),
        &mut host,
    )
    .expect("interpreter accepts a length-delimited Bytes input");
    assert_eq!(interpreted.exit_status, 75, "interior NUL");
    assert!(interpreted.stdout.is_empty() && interpreted.stderr.is_empty());

    let initializer = path
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let wrapper = fixture.build.join("nul-argv.c");
    std::fs::write(
        &wrapper,
        format!(
            r#"#include <stdbool.h>
#include <stddef.h>
extern int ken_test_process_main(int, char **, char **);
extern size_t __real_strlen(const char *);
static char path[] = {{ {initializer}, 0 }};
static bool supplied_path_length;
size_t __wrap_strlen(const char *value) {{
    if (value != path) return __real_strlen(value);
    supplied_path_length = true;
    return sizeof(path) - 1;
}}
int main(void) {{
    char *args[] = {{ path, NULL }};
    char *env[] = {{ NULL }};
    int status = ken_test_process_main(1, args, env);
    return supplied_path_length ? status : 76;
}}
"#
        ),
    )
    .unwrap();
    let native =
        fixture.run_starter_with_args_wrapper("nul-argv", &wrapper, &["-Wl,--wrap=strlen"]);
    assert_eq!(
        native.status.code(),
        Some(interpreted.exit_status),
        "interior NUL: {native:?}"
    );
    assert_eq!(
        native.stdout, interpreted.stdout,
        "interior NUL: stdout parity"
    );
    assert_eq!(
        native.stderr, interpreted.stderr,
        "interior NUL: stderr parity"
    );
}
