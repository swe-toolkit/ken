---
name: a-catalog-laws-statement-is-pinned-only-by-use-so-weaken-a-private-one-and-unpack-a-public-one
description: Elaboration checks a law's body against its stated type; nothing checks the stated type. A private law's test (name resolves, Transparent, no trust) stays green when the statement is weakened, so probe by weakening it. A public law is pinned by client use, but only if a client unpacks it, not merely builds it. A falsifier that reddens first at a private helper is settled by showing the public statement is false under the mutation.
metadata:
  type: feedback
---

# A catalog law's statement is pinned only by use: weaken a private one, unpack a public one

A law's proof is checked against its stated type. What pins the **statement**
is use: a client that relies on the proposition fails if it changes. A private
law has no client. A public law has one only if the client actually consumes
the proposition.

## Private: weaken the statement under the same name

**Measured 2026-09-24 on CAT-MAP-DOM-MEMBER-LAW.** Squash
`1a4495376f3d5b7a9ca218a05d56ff1180079598`. One LOW test-gap finding at
`evt_35kdyheqftzve` (thread `thr_5mvh6wqb240kg`).

The private law `dom_preserves_membership` had a test asserting that the name
resolves, the declaration is `Decl::Transparent`, and `trusted_base_delta` is
empty. Replacing the statement with `Equal Bool (member ...) (member ...) =
Refl` kept all three green, although the test's doc claimed no concrete
example "stands in for the general result". The Deque PopBack law had no such
gap: it was public, and its closeout test applied it on inhabited deques.

For every new law, ask whether it is public (pinned by client use) or private
(pinned only by source review). For a private law, run:

1. **Break the subject.** Should go red: the law is non-vacuous.
2. **Publish the law.** The export sentinel should go red.
3. **Weaken the statement under the same name.** If green, the test pins the
   name, not the law. Only this mutation finds the gap.
4. **Before filing, weaken the statement and also break the subject.** If
   another checked declaration still rejects every broken subject (usually a
   public law over the same branch), a defect cannot hide behind the gap:
   state it, do not file it.

CAT-VALIDATION-AP-ERROR-LAW (squash
`ac0f42f0b8f9787e1ee6d1ce151ef4b692b28ab8`, `evt_1retee69yqq6w`): mutation 3
stayed green, but five two-error-branch mutants (a flip, dropping either error,
duplicating one) were all rejected at load by the pre-existing public
`validation_ap_cmp`. Map had no public law over `dom`, so its gap was filed.
File a load-bearing gap LOW: the law is correct, the gap is in what the test can
detect, and the remedy (for example a structural pin on the kernel `ty`) is
the owner's.

## Public: a client must unpack it, not only build it

**Measured 2026-09-23 on CAT-PARSING-DECODER-PRESERVATION.** Squash
`7008b6bb673b78af93afc98575606b5a04012800`. CLEAN, `evt_47t8re80kwnf4`.

The public `DecoderPreserves` unfolds to a **private** match predicate
(`DecoderResultPreserved`). The consumer fixture only **built**
`DecoderPreserves` terms by composing the laws. A real client, such as the held
`CAT-PARSING-LAWS`, must **unpack** one: from a proof plus
`Equal (d cur) (Decoded v next)`, get `good_cursor next`. A client cannot name
a private body, so this could have been unusable.

Probe with a scratch strict client and three unpacking routes: coerce
`h cur g` into a client-authored match of the same shape; J with the public
predicate on a constant decoder as the motive; J with the client's own match as
the motive. Add one control that must be rejected. All three were accepted,
because the kernel unfolds private definitions during conversion; the control
was rejected with TypeMismatch.

## A falsifier that stops at the first error

The same PR disclosed that its AC-3 mutation reddened first at a **private**
equation, so the public law was "not independently observed". Do not build a
mutation that gets past the private equation. Ask whether the **public
statement** is false under the mutation. If it is stated about the public
combinator and the mutated behaviour gives a counterexample, no proof can exist
and the law is not vacuous. Name the counterexample (here zero progress from a
stalled `many`, with `pure` as the fallback), and cite a closed fixture that
builds it if one exists.

Related: "a catalog proof suite over a recursive scan is self covering via
elaboration and its only silent weakening is a vacuous ordering predicate" (an
earlier lesson, since retired) (self-coverage covers the body, not the
statement),
[[a-zero-trust-delta-control-must-assert-an-exact-after-minus-before-inventory-not-a-superset-and-by-identity-not-by-name]]
(the dual: pins the statement, loses the trust pin), "a control defined by the
same condition as the defect is guaranteed to pass" (an earlier lesson, since
retired),
[[an-alias-between-two-identities-with-the-same-body-is-invisible-to-the-type-checker-so-swap-the-binding-to-test-the-identity-pin]].
