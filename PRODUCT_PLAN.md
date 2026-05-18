# STORM Product Plan

## Vision

STORM is the portfolio Knowledge System for weather systems, seasonal hazards,
disasters, preparedness tradeoffs, exposure records, and recovery windows.

It should let downstream systems model weather pressure without turning every
repo into a weather simulator. STORM records the hazard and handoff; PORTO owns
place/access effects, CERES owns production/storage/labor shocks, FAUNA and
FLORA own ecological consequences, RITE owns public obligation and legitimacy,
CANON owns stable ids, and BANISH owns game mechanics.

## Waves

| Wave | Goal | Validation |
|---|---|---|
| Foundation | Rust workspace, seed schema, validation CLI, docs, and intake. | `cargo fmt --check`; `cargo test --quiet`; `cargo run -p storm-cli -- validate fixtures\seed-storm.json` |
| Consumer probes | Add BANISH/PORTO/CERES/FAUNA/FLORA/RITE/CANON fixture probes. | Fixture validation plus panel-loop review. |
| Hazard expansion | Generate 50-100 weather/hazard/disaster candidates and rubric gaps. | Expansion doc plus rubric update only for repeated gaps. |
| Publisher planning | Plan CROP/PEBBLE/FLETCH/PROOF packaging after fixture shape stabilizes. | Dependency tracker update and generated artifact sketch. |

## Initial Rust contract

| Type | Purpose |
|---|---|
| `WeatherSystem` | Weather pattern with kind, seasonality, signals, and evidence. |
| `HazardEvent` | Exposed event created by a weather system, with consequence and downstream handoffs. |
| `PreparednessProfile` | Preparation that changes exposure or timing while creating tradeoffs. |
| `RecoveryWindow` | Timing/action window after a hazard, with downstream decision hook. |

## Dependency posture

| Dependency | Status | Reason |
|---|---|---|
| PORTO | Planned consumer/provider | Place, route, access, terrain, and exposure geography. |
| CERES | Planned consumer/provider | Crop, storage, labor, fuel, and recovery production impacts. |
| FAUNA/FLORA | Planned consumers/providers | Animal and plant impact/recovery evidence. |
| RITE | Planned consumer/provider | Disaster obligations, legitimacy, mourning, and restitution. |
| CANON | Planned artifact consumer/provider | Stable ids for weather systems, hazard events, and consequences. |
| CROP/PEBBLE/FLETCH/PROOF | Planned publisher layers | Publish validated STORM packs after fixture shape stabilizes. |
| RLINE/SLICE | Deferred extraction candidates | Extract shared graph/query/time primitives only after repeated consumer proof. |
| ROLES | Planned artifact | Add hazard/source-custody/downstream-readiness panels after schema stabilizes. |

## Non-goals

- No real-world emergency guidance, weather forecasting, or climate prediction.
- No game mechanics, map routing, economic valuation, ritual adjudication, or
  ecological simulation.
- No broad disaster engine before consumer fixtures prove repeated needs.
