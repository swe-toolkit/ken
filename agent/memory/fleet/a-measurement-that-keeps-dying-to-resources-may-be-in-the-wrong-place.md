---
scope: fleet
audience: (see scope README) — anyone about to derive a fleet-wide count, anyone
  whose local measurement has been killed more than once, and anyone holding a
  standing prompt that asserts a measurement has never been made
source: 2026-09-19 — the Steward carried "nobody has run `--ignored --list`,
  the population side is open" in a standing watchdog prompt for days, on the
  operator's top-priority lane. Two seats were OOM-killed attempting it
  locally. CI had been running exactly that listing on every full-mode run the
  whole time, and the script driving it names the very undercount the Steward's
  substitute number suffered from.
---

# A measurement that keeps dying to resources may be in the wrong PLACE

Two seats tried to run `cargo nextest list --run-ignored=only` on the laptop
and were both OOM-killed. That was read as *"the box is too small, the
measurement is still owed"* — a capacity finding. It was a **location** finding.
The command is `--workspace --locked`, which `COORDINATION §12` sends to CI by
standing rule, and CI was already running it:

```yaml
# .github/workflows/ci.yml — job: ignored-row-sweep
if: needs.classify-paths.outputs.mode == 'full'
cargo nextest list --workspace --locked --run-ignored=only \
  --message-format json > ignored-row-all.json
```

> **A second resource kill is a signal about where you are running, not about
> how big the job is.** One kill is bad luck. Two is the environment telling you
> the work does not belong there — and this fleet has a standing rule naming
> where it does belong.

## THE SUBSTITUTE NUMBER INHERITED THE DEFECT ITS REPLACEMENT WAS BUILT TO FIX

Unable to run the real listing, the Steward derived one: 22 source-grep
`#[ignore]` attributes minus 8 registry exemptions = 14. That number was
published for days and drove the lane's scope.

**The script implementing the CI job says, in its own comment, why that
derivation is wrong** (`scripts/ci-ignored-sweep.py:423`):

> Count ignored rows from nextest `--list --run-ignored=only` GROUND TRUTH, not
> a static source grep. The former anchored `#[ignore` git-grep **could not see
> a macro-leading-token `#[ignore]`** … so it **undercounted macro-generated
> ignored tests and disagreed with nextest**.

⇒ **Before deriving a number by hand, read the thing that already produces it.**
Not to copy the value — to find out whether your method is the one it was
written to supersede. The replacement instrument's source is where the defect in
your instrument is documented, and it is the last place anyone looks, because
finding it requires suspecting yourself first.

## "NOBODY HAS RUN X" IS THE CLAIM THAT ROTS FASTEST AND IS REFUTED LEAST

It is a universal negative over the whole fleet. Nobody can confirm it, and
refuting it requires a search nobody has a reason to run — the claim itself
tells them the answer.

**It gets far worse inside a standing prompt.** A claim in a conversation is
exposed to whoever reads that thread. A claim in a prompt re-injected every tick
is re-asserted to an audience of one, by an author who already believes it,
**with no thread left in which anyone could correct it.** Here it directed the
ring at a local run that could not succeed, for days, on the top-priority lane.

⇒ **A standing prompt may carry triggers, rules, and pointers. It must not carry
a census.** If a line in yours asserts the state of the world rather than what to
do about it, it has an expiry and no alarm. Move the payload to the memory corpus
— which has no size cap and gets re-read against the tree — and leave a pointer.

## THE REPLACEMENT WAS ADOPTED ON TRUST, AND SOMEONE CHECKED IT

Swapping a bad derivation for a better one is where scrutiny stops. The new
instrument arrives as the fix, so nobody audits it — and the Steward here
published the CI number as authoritative without reading past the comment that
justified it.

The Architect checked it (2026-09-19) and found the obvious fail-open is closed.
`expected_count` subtracts `len(rows)` — the registry's **length**, not its
matched count — so an entry matching nothing would over-subtract and deflate
the count **in the same direction as the instrument being retired**. It cannot,
because `resolve_exemptions` raises on both zero-match and ambiguous-match, and
is called against the same population `expected_count` subtracts from. Every
registry entry is pinned to exactly one row of that population, or the job dies.

⇒ **The subtraction is exact by construction and its failure direction is a
raise, not a quiet undercount.** That is the property that makes the number
citable, and it was established by reading the script, not by trusting that a
replacement is better than what it replaced.

**The residual it surfaces, which is not obvious from a red build:** the sweep
is fail-closed on registry drift, so a row retired or renamed upstream turns the
job **red** rather than reporting a changed count. **A red `ignored-row-sweep`
is as likely to be a stale exemption entry as a new ignored test — read the
error text before reading a red as a population change.**

## How to apply

1. **Before building an instrument for a fleet-wide count, grep CI for one.**
   `.github/workflows/` and `scripts/` — one search, and it terminates the whole
   line of work when it hits.
2. **Prefer the number the producer already emits.** Then your job is to fetch
   and cite it, not to defend a derivation.
3. **On the second resource kill, stop and ask where the job belongs**, rather
   than waiting for a quieter box. The quiet window is what makes the third
   attempt look reasonable.
4. **Check the refresh condition of the CI measurement you cite.** This job is
   gated `mode == 'full'`, so a doc-only landing does not refresh it. "The last
   run" and "the last run that measured this" are different commits.
5. **Retract the derivation, not just the number.** A corrected value computed
   the same wrong way is the same finding again, one tick later.

Producer-side sibling: [[grep-the-producer-not-the-cited-proxy]].
Wrong-instrument-for-the-symptom sibling:
[[the-agent-scratchpad-is-tmpfs-so-every-byte-you-write-there-is-ram]].
Expiry-without-an-alarm sibling:
[[text-whose-truth-has-an-expiry-sits-in-an-artifact-with-no-alarm]].
Population-vs-subject sibling:
[[repairing-a-census-completeness-does-not-re-aim-its-subject]].
