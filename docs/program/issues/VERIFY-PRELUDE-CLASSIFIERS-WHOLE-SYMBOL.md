---
id: VERIFY-PRELUDE-CLASSIFIERS-WHOLE-SYMBOL
title: "ken-runtime classifies prelude constructors by a suffix of the stable symbol (ends_with(\"::Stdout\") and about 25 more), so a user constructor with the same leaf name in another family matches the prelude's classifier. Since VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY the prelude symbols are fixed by checked identity: compare whole prelude symbols"
status: active
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY]
blocks: []
github: null
origin: "Architect carry in D0 evt_x572bpfnsyj1 (VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY): the suffix classifiers can now compare whole prelude symbols, as their own WP. Check 10 (lookup by spelling). Steward-filed per COORDINATION section 2."
---

# Classify prelude roles by whole stable symbol

## Objective

A native classifier recognizes a prelude constructor or declaration only
by its whole stable symbol, so a user family's same-named constructor is
never taken for it.

## Settled inputs (Architect D0 `evt_x572bpfnsyj1`)

- **Population.** 26 distinct `ends_with("::…")` suffixes over 61 lines
  in `crates/*/src/*` on `874e9f7bb` (58 production, 3 in-src test; all in
  ken-runtime). The D0 counted 56 on its earlier base (Steward
  `evt_7qa20faq8a0zp`). 25 are constructor
  leaves. `::buffer_nat_to_int` is the only declaration. Example:
  `console_stream_tag` (ken-runtime lowering/mod.rs) tests
  `ends_with("::Stdout")`.
- **The enabler.** After `VERIFY-NATIVE-STABLE-SYMBOL-PRELUDE-IDENTITY`,
  prelude ids take reserved symbols from checked identity, and a
  colliding user declaration takes `owner#ordinal`. So the whole prelude
  symbol is a fixed value a classifier can compare against.
- **The hazard is unmeasured.** A user constructor `Stdout` in another
  family has a symbol that also ends in `::Stdout`. Whether any classifier
  then misclassifies a valid program is AC-0's to measure.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

AC-0 first. Then each classifier compares the whole prelude stable
symbol, from one shared source, not a suffix. The Architect rules the
shared source at review.

## Acceptance

- **AC-0 (measure).** On the landed base: the classifier population by
  mechanism, and a user family declaring a constructor with each prelude
  leaf name, with the native outcome per row.
- **AC-1.** Every row in AC-0 classifies only the prelude constructor. A
  user `Stdout` in another family is treated as an ordinary constructor,
  and the program runs with the interpreter's output.
- **AC-2 (control).** The prelude console, buffer and file rows, the
  `zz_unrelated` controls, and the hash and roster pins are unchanged.
- **AC-3 (mutation, QA).** Restoring one classifier to its suffix test
  reddens its AC-1 row.

## Stop conditions

- A change to the spec 46 stable-symbol contract, or any kernel or trust
  change.
- The shared source needs a new stable-symbol namespace.
