---
scope: fleet
audience: (see scope README) — every seat whose system prompt directs temporary
  files into a scratchpad directory, which is all of them
source: 2026-09-19 — L1's `--ignored --list` (the operator's top-priority
  measurement) was OOM-killed twice. Three of one seat's background tasks were
  killed before anyone looked at the right instrument. Measured at the time:
  2.1 G of a 7.8 G tmpfs held by agent scratchpads, none of it visible to `ps`.
---

# The agent scratchpad is tmpfs — every byte you write there is RAM

Each seat's system prompt names a scratchpad directory and tells it to use that
instead of `/tmp`. **On this box that path IS `/tmp`, and `/tmp` is tmpfs.** So
a redirected build log, a `capture-pane` dump, a saved diff, a `CARGO_TARGET_DIR`
— anything written there consumes RAM for as long as the file exists, and
nothing ever evicts it.

```sh
findmnt -no FSTYPE,SIZE,USED /tmp     # tmpfs 7.8G 2.1G  -> it is RAM
du -sh "$CLAUDE_SCRATCHPAD"           # your own contribution to that number
```

## The instrument everyone reaches for is blind to it

**`ps` cannot see a tmpfs.** Those pages belong to the filesystem, not to any
process, so a seat holding 600 MB of logs shows a perfectly ordinary RSS. The
seat that spent an evening reading `ps` output concluded *"the fleet is busy"*
while its own files were the pressure. It was found only because `free` and
`findmnt` disagreed with `ps`.

⇒ **When something is OOM-killed and no process looks large, measure the
tmpfs before you conclude anything about load.**

## The narrow diagnosis is the dangerous part

The first seat to find this correctly diagnosed **its own** case — two
`CARGO_TARGET_DIR`s under the scratchpad — and published the rule in those
terms. A sweep of every other seat immediately after found **no target dir on
tmpfs anywhere** (no `CACHEDIR.TAG`, no `target/`, no `debug/`), and the largest
holder on the box was 603 MB of **ordinary probe logs**, ~35 MB apiece.

> A seat that asks *"is my `CARGO_TARGET_DIR` on tmpfs?"*, finds no, and stands
> down **has asked the wrong question and gotten a reassuring answer.** The
> mechanism is not the target dir. It is the mount.

The question that generalizes is **"how many bytes have I written under the
scratchpad?"** — and it needs no theory about which tool wrote them.

## Spent evidence is the common case, and it does not announce itself

The three largest holders were all keeping output whose conclusions had already
been extracted into durable notes: CI shard logs from a closed investigation,
`capture-pane` dumps, probe captures from finished bisections. **Nobody was
being careless — each file was written for a reason that had since expired.**
A log has no expiry field, so it is retained by default forever, in RAM, on a
box where a priority measurement is dying for want of it.

## How to apply

1. **Write large output to ext4, not the scratchpad.** Anything expected to
   exceed a few MB — build logs, test captures, target dirs — belongs on a real
   disk path, excluded worktree-locally so it cannot be staged into a diff.
2. **Delete captures when the conclusion lands in a durable artifact.** The
   moment you write the finding into a checkpoint, node, or post, the raw
   capture is spent. That is the trigger; there will not be a later one.
3. **`du -sh` your own scratchpad before diagnosing anyone else's memory.** The
   seat measuring the fleet is a seat on the fleet. (Measured: the Steward was
   holding 49 MB of spent shard logs while auditing everyone else's, and freed
   it before publishing the audit.)
4. **Never infer "the box is busy" from `ps`** when the symptom is an OOM kill.
   Check `free`, `findmnt`, and `du` on the scratchpads first.
5. **Report a large holding to its owner as a measurement, not an instruction.**
   Uniform-looking captures may still be live evidence; the owner knows and you
   do not. Say that it is RAM and that it is adjacent to what is blocked, then
   leave the call with them.

Disk-side sibling, which covers `/tmp` **worktrees** but not the scratchpad:
[[worktree-proliferation-is-the-disk-disease-tear-down-scratch-worktrees]].
Wrong-instrument sibling:
[[a-full-disk-presents-as-a-test-regression-and-df-slash-cannot-see-it]].
