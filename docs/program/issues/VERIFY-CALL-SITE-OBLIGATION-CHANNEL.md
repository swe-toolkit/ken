---
id: VERIFY-CALL-SITE-OBLIGATION-CHANNEL
title: "A call-site Requires hole raised in an instance field, a space cell or elaborate_expr is postulated into the trusted base but never reported as an obligation, because those contexts drop cx.obligations. Report every such hole or refuse the call, so the trusted-base delta always equals the reported holes"
status: merged
owner: verify
size: M
tier: T1
gate: architect
depends_on: [VERIFY-CALL-SITE-PRECONDITION-DISCHARGE]
blocks: [LANG-REFINED-PARAM-REQUIRES-DESUGAR]
github: null
origin: "Adversary M8 evt_29hvmst78v9dp F1 on 392578133 (VERIFY-CALL-SITE-PRECONDITION-DISCHARGE): before that merge these calls were kernel-rejected; the merge turned a refusal into an unreported postulate. Placed in the Verify ring after LANG-ACTIVE-PREMISE-RELOCATION-STACK-FRAME and ahead of LANG-REFINED-PARAM-REQUIRES-DESUGAR, whose instance-field consumer (Derived.ken.md:1927) takes this route. Steward-filed per COORDINATION section 2."
---

# Every call-site Requires hole is reported

## Objective

For every declaration form, the trusted-base delta of elaborating it equals
the set of holes it reports as obligations. A call-site `Requires` hole is
either reported or the call is refused; it is never postulated silently.

## Settled inputs (Adversary `evt_29hvmst78v9dp`, measured on `392578133`)

- **The rows.** Callee `fn d (x : Int) (y : Int) : Int requires
  Not (Equal Int x y) = x`. Each row gives the trusted-base delta, then the
  number of holes reported.
  - `instance Endo Int { apply = λx. d x x }`: Ok, delta 1, 0 reported. The
    postulate `Π x. Not (Equal Int x x)` is false. Control `apply = λx. x`:
    delta 0.
  - `space S { mut cell : Int = d 0 0 }`: Ok, delta 1, 0 reported. Control
    `= 0`: delta 0.
  - `elaborate_expr("e", "d 0 0")`: Ok, delta 1. That API has no obligation
    channel.
  - Control `fn g (m : Int) : Int = d m m`: delta 1, 1 reported.
- **The mechanism.** These contexts carry `with_preconditions`, so
  `precondition_proof` (`elab.rs:4507`) declares the hole and pushes it onto
  `cx.obligations`. The context is then dropped without taking them.
  - Instance: `elab_instance_decl` (`:14167`) builds its contexts, then
    returns `obligations: vec![]` (`:14442`).
  - Space cell: the context at `:14773`, then `vec![]` at `:14797`.
- **Unmeasured siblings.** Space operations (`:14882`) and the
  `resolve_instance_dictionary` argument contexts. The Adversary counted 10
  functions that never take `cx.obligations`.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

1. At the start, list every `ElabCtx` built `with_preconditions` and say
   whether its obligations reach the declaration's result (check 2). The
   population is every such context, not only the three measured rows.
2. Repair each context that drops them. Each one either moves its
   obligations into the result, or, where the result has no obligation
   channel, refuses a call that needs a premise proof. The Architect rules
   which, per declaration form, on the list from step 1.

## Acceptance

- **AC-1.** The instance and space-cell rows either report 1 obligation with
  delta 1, or are refused with delta 0. `elaborate_expr` does the same. The
  controls keep their deltas.
- **AC-2.** One test per repaired form compares `trusted_base()` before and
  after with the reported `hole_id`s, and they are equal. Restoring the
  dropped obligations at one repaired site turns its row red.
- **AC-3.** The CALL-SITE and opaque-hole suites stay green, and CI is green.

## Stop conditions

- A form that must refuse has a catalog consumer that would then fail to
  check. Stop to the Architect with the consumer.
- The repair needs a kernel change.

## Not this WP

A `requires` call in a type position (`elab_type`'s `RType::RApp` arm,
`elab.rs:1115`) is refused even with its premise in scope (Adversary F2). It
fails closed and stays out of this node.

A declaration that fails after `precondition_proof` has declared its hole
leaves an orphan hole postulate in a reusable `ElabEnv`: the REPL `Session`,
`modules::expand_and_elaborate`, and `load_unit` (Architect
`evt_7x9fznedwjygv`). It affects every form without pending admission. The
orphan over-reports in `trusted_base()` and is unreachable from source, so it
is queued, not framed. Re-raise it when a product path reuses a failed
session's environment, including `compiler_driver.rs:4251`.

The `Refused` gate covers only `precondition_proof`. The binary-operator
holes (`/` and `%` at `elab.rs:10782`, `+` NoOvf at `:10708`) are still
declared in `elaborate_expr`'s Refused context and dropped, so `7 / 0` at the
REPL adds one unreported `Nonzero 0` entry to the session's trusted base
(Adversary M8 `evt_4jcsdc2rhwp5q`). It over-reports and the term does not
reference the hole. It is repaired in
`LANG-REFINEMENT-INTRODUCTION-OBLIGATION`, whose shared obligation gate
covers these arms (Architect `evt_3h5n5b9y0wzf`).

## Closeout

Merged `684e935fb` from exact `d02bb62e2` (PR run 37214263011). Verify QA
`evt_5pbjhymmzte7e`, Architect `evt_7k1t37xnjtykx`, Decision
`dec_183jvr0chtq3x`. Every context that installs preconditions states a
`PremiseHoles` mode: `Reported` takes each hole into a reported result, and
`Refused` refuses an unsupplied premise before a hole is declared, with the
new `ElabError::PremiseWithoutObligationChannel`. The call-site discharge
suite pins the Reported and Refused rows and asserts each hole delta first.
`LANG-REFINED-PARAM-REQUIRES-DESUGAR` is unblocked. The orphan hole above
stays queued.
