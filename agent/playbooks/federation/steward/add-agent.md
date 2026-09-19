# Requesting a seat change

The Steward does not provision, remove, re-home, credential, configure, or
launch agents. Those acts modify workflow state (`moot.toml`, startup prompts,
credentials, or fleet topology) and are covered by the workflow-edit
prohibition in `../steward.md` section 3.

## Steward responsibility

The Steward identifies a product-capability gap and routes it to the operator.

## Product need

When the current roster cannot execute an operator-authorized lane, tell the
operator:

- the product lane and WP that need the seat.

## Tier and duration

State the capability tier using `MODELS.md` names and whether the need is
temporary or standing.

## Requested change

Name the current seat, if any, that is blocked or mis-tiered, and ask for the
smallest change that restores the product lane.

## Negative scope

Do not propose credentials, backend configuration, startup prompts, or a
`moot.toml` patch. Do not inspect another seat's secrets.

## Operator-designated provisioner

The operator chooses a non-Steward provisioner and supplies the applicable
infrastructure procedure. The Steward waits for a confirmation that the seat is
live, then assigns product work through the normal release path.

A new seat does not authorize a new lane. Lane count and objectives remain
operator-owned.