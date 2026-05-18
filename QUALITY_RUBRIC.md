# STORM Quality Rubric

Score each promoted STORM artifact 0-4 on each axis. A fixture is ready for
downstream prototype use only when every axis is at least 3.

| Axis | 0 | 2 | 4 |
|---|---|---|---|
| Hazard clarity | Weather is generic mood. | Hazard type is named. | Weather system, hazard kind, exposure, and consequence are explicit. |
| Signal and timing | Event arrives without warning or season. | Season or warning is named. | Signals, lead time, seasonality, and recovery timing are usable downstream. |
| Preparedness tradeoff | No preparation is possible. | Preparation is named. | Preparation changes exposure while creating a visible labor, resource, route, or legitimacy tradeoff. |
| Recovery window | Disaster ends as a flat penalty. | Recovery action is named. | Recovery timing, action, and downstream decision are explicit. |
| Downstream handoff | STORM owns every consequence. | One consumer is named. | PORTO, CERES, FAUNA, FLORA, RITE, BANISH, CANON, or LUCIA handoffs are explicit and bounded. |
| Evidence readiness | Claims are asserted without source or fixture. | Seed evidence exists. | Evidence refs, ids, and fixture records can be validated and packed downstream. |

## Foundation gate

A STORM seed passes the first gate when it proves a weather system can create a
hazard event with exposure, preparedness tradeoff, recovery window, and bounded
handoffs to sibling repos without becoming a forecasting engine or game mechanic.

Validation:

```powershell
cargo run -p storm-cli -- validate fixtures\seed-storm.json
```
