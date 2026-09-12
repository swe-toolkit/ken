# Source input and dispatch

> **Availability:** partial. **Authority:** explanatory.

The `ken` command accepts a source path for `check` and `run`. The
[CLI dispatcher](../../../../crates/ken-cli/src/main.rs) reads the path and
creates an elaboration environment before selecting its input route. A path
ending in `.ken.md` uses the literate-source route; other paths use the ordinary
source route. Catalog-addressed paths instead enter the roots loader, which
loads the addressed module and then runs that entry's checked fences.

The command does not admit text merely by reading it. Its shared
[elaboration helper](../../../../crates/ken-cli/src/main.rs) returns only after
one of those routes has elaborated the input. An unreadable path, an
initialization failure, or an elaboration error makes the command report an
error rather than continue to execution.

`ken check` stops at that front-end boundary. `ken run` uses the same source
format selection before it asks the interpreter to evaluate an entry point. The
input route therefore does not itself establish that a program has an entry
point or that it can run; those are later concerns. For the front-end stages
that follow input selection, read [lexing](lexing.md).
