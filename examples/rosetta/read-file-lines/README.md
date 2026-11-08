# Read a file line by line

Read a text file and process it line by line.

Reference: <https://rosettacode.org/wiki/Read_a_file_line_by_line>

## Status

`main` declares `Cap APartial` on its own signature (the manifest
IS the signature); `ken run` reads that declaration, mints exactly that
authority (never full, never ambient), and binds it before running the
program. `read_bytes` carries the cap into a real `[FS]` effect
computation, gated by the runtime `authorizes` check
(`ken-interp/src/eval.rs`) before any syscall. Reads the checked-in
hermetic fixture `conformance/fs/fixtures/three-lines.txt`, splits it into
lines (`str::lines()` terminator semantics — a trailing `\n` yields no
trailing empty line), and prints each line via `[Console]` from *inside*
the program.

`main` composes its `[FS]` read and `[Console]` printing in one checked
`HostIO APartial` tree. `host_fs` lifts `read_bytes`, `host_console` lifts
`print_line`, and `host_bind` sequences their responses. The interpreter
handles the resulting effect tree without requiring source to name its
internal coproduct operation types. On a missing file or denied capability,
`main` prints the named error through Console and returns a nonzero
`ExitCode.Failure`; it does not report a false success.
