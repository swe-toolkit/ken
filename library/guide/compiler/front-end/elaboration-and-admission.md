# Elaboration and kernel admission

> **Availability:** partial. **Authority:** explanatory.

The elaboration environment in the [elaborator entry
module](../../../../crates/ken-elaborator/src/lib.rs) parses a file, expands its
module structure, and elaborates the resulting declarations in order. Later
declarations can therefore refer to declarations already elaborated in that
file. The module path uses the [elaboration
implementation](../../../../crates/ken-elaborator/src/elab.rs) to turn resolved
forms into kernel-core terms.

Elaboration prepares a term for checking; it is not the authority that makes a
source claim valid. The environment’s certificate-discharge path calls the
kernel checker, and a failed check leaves the certificate undischarged. The
kernel remains the authority for type checking and proof validity.

The [compiler driver](../../../../crates/ken-elaborator/src/compiler_driver.rs)
can collect admitted declarations into a `CheckedCorePackage`. That package is
a boundary for later consumers, not evidence that native lowering or an
executable artifact has occurred. Read [the chapter landing](../front-end.md)
for the full front-end route, then continue to [the kernel boundary](../kernel.md).

The language’s elaboration contract is defined by the [elaboration
specification](../../../../spec/30-surface/39-elaboration.md). This page
explains the implementation route that applies that contract and its admission
boundary.
