//! Deterministic child executable for the checked PX10 interpreter tests.
//! The executable is a fixture, never a Ken process operation.

#[cfg(target_os = "linux")]
fn main() {
    use std::io::{Read, Write};
    use std::os::unix::ffi::OsStrExt;

    let arguments = std::env::args_os().collect::<Vec<_>>();
    match arguments.get(1).map(|arg| arg.as_bytes()) {
        Some(b"argv-env") => {
            let mut output = Vec::new();
            output.extend_from_slice(b"argv0=");
            output.extend_from_slice(arguments[0].as_bytes());
            output.extend_from_slice(b"\nargv1=");
            output.extend_from_slice(arguments.get(2).expect("payload argument").as_bytes());
            for (label, key) in [
                (b"selected=".as_slice(), "PX10_SELECTED"),
                (b"replaced=".as_slice(), "PX10_REPLACED"),
                (b"parent-only=".as_slice(), "PX10_PARENT_ONLY"),
            ] {
                output.extend_from_slice(b"\n");
                output.extend_from_slice(label);
                match std::env::var_os(key) {
                    Some(value) => output.extend_from_slice(value.as_bytes()),
                    None => output.extend_from_slice(b"<unset>"),
                }
            }
            output.push(b'\n');
            std::io::stdout()
                .write_all(&output)
                .expect("write fixture record");
        }
        Some(b"sigterm") => {
            // SAFETY: raise targets this process; the default SIGTERM action
            // terminates it. The fallback exit indicates a broken fixture.
            unsafe { libc::raise(libc::SIGTERM) };
            std::process::exit(96);
        }
        Some(b"held") => {
            let path = std::env::var_os("PX10_READY_PATH").expect("ready path");
            std::fs::write(path, std::process::id().to_string()).expect("publish pid");
            std::io::stdout()
                .write_all(b"ready\n")
                .expect("write ready marker");
            std::io::stdout().flush().expect("flush ready marker");
            std::io::stdin()
                .read_to_end(&mut Vec::new())
                .expect("await stdin EOF");
        }
        _ => std::process::exit(95),
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {}
