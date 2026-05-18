# Wave: STORM Foundation

## Goal

Create STORM as the Knowledge Systems repo for weather systems, seasonal hazards,
disasters, preparedness tradeoffs, exposure records, and recovery windows.

## Thesis

Weather and disasters should create reusable pressure without forcing every
downstream repo to invent its own hazard grammar. STORM should validate
product-neutral weather/hazard records and hand off bounded consequences to the
right sibling repo.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | active | Create repo skeleton, Rust crates, seed fixture, docs, skills, and first tested contract. |
| 02 | Consumer probes | pending | Add BANISH/PORTO/CERES/FAUNA/FLORA/RITE/CANON fixture probes and panel-loop review. |
| 03 | Hazard expansion | pending | Generate 50-100 weather/hazard/disaster candidates and rubric gaps. |
| 04 | Publisher planning | pending | Plan CROP/PEBBLE/FLETCH/PROOF publishing after fixture shape stabilizes. |

## Success criteria

- README explains STORM and first validation commands.
- Product plan names consumers and non-goals.
- Rust core and CLI validate the seed fixture.
- Wave/pulse scaffolding and repo-local skills exist.
- TRACKER dependency intake records chain placement.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run -p storm-cli -- validate fixtures\seed-storm.json
git grep -n "STORM" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
