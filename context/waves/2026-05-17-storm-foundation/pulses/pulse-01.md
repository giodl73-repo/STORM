# Pulse 01: Workspace foundation

## Goal

Create the STORM repository foundation and first tested weather/hazard contract.

## Changes

- Add Rust workspace with `storm-core` and `storm-cli`.
- Add seed fixture proving weather system, hazard, preparedness, recovery, and
  downstream handoff records.
- Add README, product plan, quality rubric, wave docs, and repo skills.
- Add TRACKER dependency intake and portfolio records.

## Validation

- `cargo fmt --check`
- `cargo test --quiet`
- `cargo run -p storm-cli -- validate fixtures\seed-storm.json`
- `git grep -n "STORM" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md`
- `git diff --check`

## Status

Active.
