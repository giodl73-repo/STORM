# Pulse 04: FLETCH registry seed

## Goal

Publish the first STORM-owned FLETCH registry for stable weather/hazard fixtures,
rubrics, scenario maps, and game-needs assets.

## Change

- Added `.fletch\registries\storm-foundation-assets.json`.
- Registered the seed fixture, quality rubric, scenario expansion document, and
  game-needs rubric as repo-owned local file assets.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run -p storm-cli -- validate fixtures\seed-storm.json
fletch registry validate --file .fletch\registries\storm-foundation-assets.json
```
