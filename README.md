# STORM

Weather systems, seasonal hazards, disasters, preparedness, and recovery-window
knowledge for portfolio systems.

STORM is a Knowledge Systems repo. It does not own maps, economies, ecology,
ritual legitimacy, or game mechanics. It owns product-neutral records for
weather systems, hazard events, warning signals, exposure, preparedness tradeoffs,
downstream handoffs, and recovery windows.

## Product thesis

Weather and disasters become shallow when they are only random events or damage
modifiers. STORM provides portable records that let downstream systems ask: what
weather pattern is active, what signals announce it, who or what is exposed, what
preparedness changes the outcome, when recovery is possible, and which sibling
repo owns the resulting map, economy, ecology, ritual, or game decision?

## First consumers

| Consumer | Use |
|---|---|
| PORTO | Route closures, terrain access, floodplains, coastlines, visibility, and hazard geography. |
| CERES | Crop loss, storage damage, fuel shock, labor interruption, production downtime, and recovery cost. |
| FAUNA | Animal displacement, disease-vector habitat, shelter stress, migration disruption, and livestock exposure. |
| FLORA | Drought, frost, flood, fire weather, regrowth, blight windows, windthrow, and soil recovery. |
| RITE | Public obligations, disaster rites, legitimacy of evacuation, mourning, restitution, and recovery ceremonies. |
| BANISH | Playable settlement-pressure scenarios without STORM owning game rules. |
| CANON | Stable ids for named weather systems, hazard events, and continuity consequences. |

## Initial scope

1. Define product-neutral Rust primitives for weather systems, hazard events,
   warning signals, affected downstream systems, preparedness profiles, recovery
   windows, evidence references, and validation findings.
2. Provide a CLI that validates STORM fixture records.
3. Establish the first wave/pulse process and research skill.
4. Document boundaries with PORTO, CERES, FAUNA, FLORA, RITE, BANISH, CANON,
   LUCIA, MAXIM, CROP, PEBBLE, FLETCH, PROOF, RLINE, SLICE, and ROLES.
5. Defer forecasting models, climate simulation, emergency advice, and runtime
   disaster mechanics until repeated consumers prove stable requirements.

## Non-goals

- STORM does not provide real-world emergency, weather, or climate advice.
- STORM does not replace PORTO geography, CERES economics, FAUNA/FLORA ecology,
  RITE legitimacy, LUCIA interpretation, CANON identity, or BANISH mechanics.
- STORM does not become a forecasting engine or climate model in the foundation
  wave.
- STORM does not force downstream repos to link Rust crates before artifact
  contracts are stable.

## Crates

| Crate | Role |
|---|---|
| `storm-core` | Weather, hazard, preparedness, recovery, evidence, and validation primitives. |
| `storm-cli` | Validation CLI for STORM fixture records. |

## First validation

STORM uses [`QUALITY_RUBRIC.md`](QUALITY_RUBRIC.md) to score hazard clarity,
signal/exposure evidence, preparedness tradeoffs, recovery windows, downstream
handoff, and boundary discipline before an artifact is promoted beyond a draft
seed.

```powershell
cargo fmt --check
cargo test --quiet
cargo run -p storm-cli -- validate fixtures\seed-storm.json
git grep -n "STORM" -- README.md PRODUCT_PLAN.md context\waves\PHASES.md
```
