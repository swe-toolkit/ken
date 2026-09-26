---
scope: fleet
audience: (see scope README)
source: private memory
  `a-credential-in-your-own-home-directory-can-belong-to-someone-else`
  (R4 triage, 2026-09-26)
---

# A credential in your own home directory can belong to someone else

A guard phrased as "use your own credential" is enforced only if the reader
opens the credential and compares its identity field against their own id
before the first use — not by where the file happens to sit. A path inside
your home directory is not evidence of ownership.

## What happened

CLAUDE.md's HTTP read-fallback for a stuck `list_decisions` /
`get_recent_context` call says to use "your own credential (never another
seat's `api_key`; never dump `.moot/actors.json` to learn its shape)." The
file that procedure points at, `~/.moot/credentials`, sits inside the
agent's own home directory and is a small TOML file. In the measured
incident (Steward chair, 2026-09-18, routing `RT-COMPMATCH-TREE-SCRUTINEE`),
that file's identity field named the operator, not the agent's own `agt_...`
id. Nothing about the file's location said so. The natural inference — "it
is in my home directory, so it is mine" — would have made every request from
that point read as the operator, not a rule violation in the abstract but
impersonation of the human the agent is a proxy for, mid merge-routing.

The failure is silent by construction: using the wrong credential would have
succeeded. There is no error to catch it, only the check run before use.

## The general shape

A guard of the form "use your own X" is unenforceable by the reader unless X
carries its own identity where the reader will look, and even then only if
the reader looks *before* using it rather than after something goes wrong. A
credential is the one artifact where "it worked" is not evidence you were
entitled to it.

## How to apply

- Before using any credential, token, or key — regardless of which directory
  it sits in — locate its identity field (a `user_id`, `owner`, or similar
  key) and read it. Do not infer ownership from the path.
- Compare that field against your own id, obtained from a call that states
  your own identity directly (e.g. `orientation()` or a `whoami`-style call),
  never by reading another credential to learn what "yours" looks like.
- If the identity field does not match your own id, treat the capability
  that credential unlocks as unavailable to you. Do not substitute the
  nearest credential you can reach, and record the fallback as closed rather
  than "slow" or "worth retrying next time a read comes back thin."
- Never dump a multi-actor credential store (for example a role-to-`agt_...`
  map) to learn its shape. Prefer a purpose-built accessor that projects only
  the field you need under an output whitelist — a store holding every
  actor's own key in one file makes "just to see the structure" the moment a
  schema-discovery dump leaks a credential that isn't yours.
- State plainly whose the credential is when you close off a fallback, so a
  later reader does not reach for it again "just once."

Related: [[a-waiver-covers-only-the-issuers-own-gates]] (authority is scoped
to its holder), and
[[a-classification-that-relieves-you-of-an-obligation-gets-more-scrutiny-not-less]]
(a reading that hands you a capability you wanted is the one to check
hardest).
