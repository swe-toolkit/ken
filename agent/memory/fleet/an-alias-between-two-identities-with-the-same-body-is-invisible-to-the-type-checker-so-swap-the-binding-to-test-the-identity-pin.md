---
name: an-alias-between-two-identities-with-the-same-body-is-invisible-to-the-type-checker-so-swap-the-binding-to-test-the-identity-pin
description: When two GlobalIds share a definitional body, as base IsTrue and LawfulClasses IsTrue do, a proof typechecks under either binding. Only an identity-level pin can tell which one an alias bound. Test the pin by rebinding to the other identity in a scratch catalog root and counting mentions of each.
metadata:
  type: feedback
---

# Two identities with one body: swap the binding to test the pin

**Measured 2026-09-23 on CAT-PARSING-CURSOR-LAWS.** Squash
`1da3055e2f6a31b6a904c01dd8ab03542c1ded04`. The hunt was CLEAN, reported at
`evt_7sk2k03n47sy6`.

## The shape

The frame had Cursor write `import Data.Numeric.Nat.Order (IsTrue as
NatOrderIsTrue, ...)`, because base binds its own `IsTrue`
(`crates/ken-elaborator/src/decimal_char.rs`). Both bodies are `Equal Bool b
True`. **So elaboration succeeding proves nothing about which identity the alias
bound.** The variant that spells the uses bare `IsTrue`, which binds base's,
still elaborates in legacy roots mode.

The durable pin is an `assert_providers_consumed` entry for
`Core.Classes.LawfulClasses.IsTrue`. It asks whether some owned declaration
mentions that `GlobalId`. That could in principle be met by a mention the
elaborator inserted, rather than by the alias itself.

## How to test it

1. Copy `catalog/packages` into the scratchpad and edit the package there so it
   binds the other identity.
2. Write a scratch `tests/` file that loads the providers and the module from
   each root with `elaborate_module_from_roots`.
3. For each identity, count the owned declarations whose type or body mentions
   it. Get base's id from `ElabEnv::new().globals["IsTrue"]` before loading any
   provider.

Here the counts were: landed 8 LawfulClasses and 0 base; variant 0 and 8. So the
pin goes RED on a mis-binding and is real.

## Why it matters

Same-body duplicate identities make the type checker an **inert** witness. It is
the same rule as "a control defined by the same condition as the defect is
guaranteed to pass" (an earlier lesson, since retired): the check has to be one
the wrong binding would fail. It pairs with
[[removing-an-ambient-fallback-can-close-the-only-working-route-when-the-intended-route-was-already-broken]],
whose first section is the undeclared-qualified-reference finding. That
dotted-reference scan was clean here; the fallback it hunted was later removed
(LANG-QUALIFIED-ACCESS-REQUIRES-IMPORT), so a qualified reference with no import
edge now fails to resolve.
