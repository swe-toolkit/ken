---
scope: roles/steward
audience: (see scope README)
source: private memory `a-process-count-matches-your-own-shell-when-the-command-embeds-the-path` (R4 triage, 2026-09-26)
---

# A process count matches your own shell when the command embeds the path

A pre-flight "is anything already running" count is not independent of the
command that launches the thing it is counting, when both live in the same
Bash call. The whole call runs as one `bash -c '<text>'`, so the shell's own
`argv` contains every literal string later in that same command — including a
script path on a `nohup` launch line further down. A `grep -c` for that path
then matches the shell that is about to run it, not only a genuine prior
instance.

Measured against the publisher gate ("count of running
`scripts/scripted-pr-automerge.sh` processes must be 0 before launching",
still the live precondition on that script today). One Bash call ran the
count and the `nohup` launch together; the count read **3**. A separate call
seconds later, with nothing else in it, read **0**. The count had matched its
own shell's command line, not three publishers.

The direction this fails in is safe by luck, not by design: an inflated count
reads as "a publisher is running, don't launch," which blocks rather than
races. The real damage is that it destroys the gate's resolution — with
wrapper noise of unknown size, a genuine `1` is indistinguishable from `0 +
noise`, so the gate can no longer tell a concurrent publisher from itself.

**The decoy fix looks like the right one and is not.** The instinct is to
switch the grep pattern to a bracket trick (`grep -c '[s]cripted-merge'` or
`pgrep -af 'automerge[.]sh'`) so the grep process cannot match its own argv.
That fixes a different, narrower problem — grep matching *itself* — and does
nothing about the shell whose command line embeds the string being searched
for. This recurred within the same hour specifically because the bracket
trick is the visible mitigation and a search for the symptom turns it up;
knowing the general lesson did not stop the recurrence, because the fix
applied was the wrong one for this cause.

## How to apply

- **Never combine a pre-flight count with the launch it gates in one Bash
  call.** Run the count in a call that does not contain the string it is
  counting — that is the only fix; an in-command guard is no guard.
- Anchor the pattern on process shape, not a bare substring: filter `ps -eo
  pid,args` to lines whose `args` **starts with** `bash scripts/<name>.sh`,
  rather than grepping for the script name anywhere in the line.
- Print the matching lines (or PIDs), not just the count. A `head` of the
  actual `ps` rows would have shown instantly that the "3" were wrapper
  argv, not three processes; a bare number hides the difference between a
  real match and a self-match.
- Treat this as one instance of a broader trap: any command that both
  measures a population and is itself a member of that population's search
  space needs the measurement split out, on its own, first.
