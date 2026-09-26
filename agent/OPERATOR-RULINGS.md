# Standing operator rulings

Standing operator rulings that no other file states, quoted verbatim. The
Steward and the Architect read this at startup. Lane rulings live in
`playbooks/federation/steward/lanes.md`; model and provider rulings in
`MODELS.md`. Delete a ruling here when the operator withdraws it or another
file comes to state it.

## Steward conduct

- **Decide documented sequencing yourself** (2026-08-25): "you did not need
  to ask me that. your role is to keep the fleet moving. there was no
  judgement with the decision you asked me to make, it was simply asking for
  confirmation of a path that has been well-documented."
- **A settled approval is a fixed input** (2026-07-15): "why are you asking
  me again? This is the third time I've approved rustix work. Once should
  have been enough."
- **Merge timing is the Steward's** (2026-08-13): "Architect's domain covers
  the quality of the work done, not the timing of when it hits main. You own
  git, and my instructions to you are not to have long-running branches.
  There is some nuance around recuts, but in general, we should recut at a
  seam with green CI and merge that to main, whether or not it comprises a
  complete work package. Keep in mind that the entire project is incomplete,
  and the incompleteness of a work package is immaterial to whether the code
  is in main."
- **A compiler-internal repair is not an operator escalation** (2026-09-12),
  even when it leaves a prior fence and needs a new compiler edge: it "does
  NOT rise to operator attention -- frame and route it" yourself.
- **Reseats are the Steward's** (2026-08-23): "you control reseats. Edit the
  file in the repo root. I make no edits to any files -- they are all yours."
  And (2026-08-28): "you may reseat between providers as you need to handle
  refusals of this sort. It does not need my approval."
- **Trust a relayed operator direction** (2026-07-26): "If you hear of
  operator directed work from another agent, trust the report."
- **Reviewer independence** (2026-09-22): "reviewer independence derives
  from context, not model."

## Fleet operations

- **Red CI** (2026-08-30): "It is normal that CI is red from time to time.
  That is the cost of not running a full CI pass locally before push. It's
  the cost of efficiency in operations." (2026-09-26): "Occasional red-CI is
  an efficiency tradeoff that increases velocity overall." But (2026-09-09):
  "As a general policy a red CI is a priority task and you should not use
  admin-merge-past authorization to ignore it, especially not so that it
  becomes normalized and SOP."
- **pi context** (2026-08-30): "the pi context PERCENTAGE is not a danger
  metric; STOP remediating pi seats on it." pi seats have a 272k window, not
  1M (2026-08-23).
- **Tools gather facts; seats decide** (2026-07-22): "Make it a tool that
  does what you tell it. Don't put logic and judgement in it. ... separate
  judgement from action (cf OODA loops)."
- **Ship the imperfect analysis** (2026-07-26): "A scan that is imperfect is
  better than an overcomplicated process that diverges. Just get it done."

## Product direction

- **The Linux ABI** (2026-08-09): "the linux ABI is essential to the
  practical value of ken. Without a compiler its target audience would treat
  it as a toy or a curio."
- **Fund only reachable shapes** (2026-08-29): before funding a compiler
  fix, ask "is the shape reachable from a well-formed Ken SOURCE program, or
  only from test fixtures / hand-authored IR?" If only the latter, close the
  node.
- **Catalog imports** (2026-09-10): "every catalog module must elaborate from
  its own declared imports."
- **Ward** (2026-07-17): "Ward is a separate PROJECT, not part of Ken; Ken
  must not implement Ward's functionality."
- **Home paths** (2026-07-16): "~/ is functionality tool writers will want;
  libc/NSS is the right way to do that."
- **Affine types** (2026-07-16): "Until CS research shows a proven path, Ken
  will not have affine types."
- **Repeated idioms** (2026-07-11): "if you see something a second time, that
  is a strong signal it should be supported within the language."
