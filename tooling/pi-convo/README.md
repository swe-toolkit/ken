# pi-convo fleet enforcement patch

`fleet-enforcement.patch` is applied to the installed `@mootup/pi-convo`
0.1.0 under `~/.pi/agent/npm/node_modules/@mootup/pi-convo` on this machine.
The operator upstreams it to the plugin's source. It does two things:

- **`get_transcript` is off by default.** Its `limit` does not bound the
  response, and one oversized payload drops the seat's convo transport. Set
  `PI_CONVO_ENABLE_GET_TRANSCRIPT=1` to register it.
- **A project command policy for the bash tool.** Before each bash call the
  plugin runs `PI_CONVO_BASH_POLICY`, or else `scripts/hooks/bash-policy`
  under the session's cwd, with the command on stdin. Exit 2 blocks the call
  and shows the policy's stdout as the reason. A missing or failing policy
  allows the call.

Claude Code seats get the same policy from `.claude/settings.json`: a
`PreToolUse` hook (`scripts/hooks/claude-bash-policy.sh`) and a permission
deny on `mcp__convo__get_transcript`. pi seats load the patched plugin at
their next start.
