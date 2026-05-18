use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StormDocument {
    pub document_id: String,
    #[serde(default)]
    pub weather_systems: Vec<WeatherSystem>,
    #[serde(default)]
    pub hazard_events: Vec<HazardEvent>,
    #[serde(default)]
    pub preparedness_profiles: Vec<PreparednessProfile>,
    #[serde(default)]
    pub recovery_windows: Vec<RecoveryWindow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeatherSystem {
    pub system_id: String,
    pub label: String,
    pub kind: WeatherKind,
    pub seasonality: String,
    #[serde(default)]
    pub signals: Vec<WeatherSignal>,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherKind {
    Heatwave,
    ColdSnap,
    Drought,
    HeavyRain,
    Flood,
    Windstorm,
    Snowstorm,
    Hail,
    Lightning,
    Fog,
    CoastalStorm,
    WildfireWeather,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeatherSignal {
    pub signal_id: String,
    pub description: String,
    #[serde(default)]
    pub lead_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HazardEvent {
    pub event_id: String,
    pub label: String,
    pub weather_system: String,
    pub hazard_kind: HazardKind,
    pub exposure: String,
    pub consequence: String,
    #[serde(default)]
    pub affected_systems: Vec<AffectedSystem>,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HazardKind {
    CropLoss,
    RouteClosure,
    ShelterStress,
    DiseaseRisk,
    FireRisk,
    FloodDamage,
    FuelShock,
    WaterShortage,
    VisibilityLoss,
    InfrastructureDamage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedSystem {
    pub repo: DownstreamRepo,
    pub handoff: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownstreamRepo {
    Porto,
    Ceres,
    Fauna,
    Flora,
    Rite,
    Banish,
    Canon,
    Lucia,
    Tracker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparednessProfile {
    pub profile_id: String,
    pub label: String,
    pub target_hazard: String,
    pub preparation: String,
    pub tradeoff: String,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryWindow {
    pub window_id: String,
    pub hazard_event: String,
    pub timing: String,
    pub recovery_action: String,
    pub downstream_decision: String,
    #[serde(default)]
    pub evidence_refs: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source_id: String,
    pub path: String,
    #[serde(default)]
    pub anchor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub code: String,
    pub location: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    Error,
    Warning,
}

pub fn validate_document(document: &StormDocument) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    require_non_empty(
        &mut findings,
        "document_id",
        "document.document_id",
        &document.document_id,
    );

    let mut system_ids = HashSet::new();
    for system in &document.weather_systems {
        require_non_empty(
            &mut findings,
            "system_id",
            "weather_system.system_id",
            &system.system_id,
        );
        require_non_empty(
            &mut findings,
            "system_label",
            &system.system_id,
            &system.label,
        );
        require_non_empty(
            &mut findings,
            "seasonality",
            &system.system_id,
            &system.seasonality,
        );
        if !system_ids.insert(system.system_id.as_str()) {
            findings.push(error(
                "duplicate_system_id",
                &system.system_id,
                "weather system id appears more than once",
            ));
        }
        if system.signals.is_empty() {
            findings.push(warning(
                "system_without_signal",
                &system.system_id,
                "weather system has no signal",
            ));
        }
        if system.evidence_refs.is_empty() {
            findings.push(warning(
                "system_without_evidence",
                &system.system_id,
                "weather system has no evidence reference",
            ));
        }
        for signal in &system.signals {
            require_non_empty(
                &mut findings,
                "signal_id",
                &system.system_id,
                &signal.signal_id,
            );
            require_non_empty(
                &mut findings,
                "signal_description",
                &signal.signal_id,
                &signal.description,
            );
        }
    }

    let mut event_ids = HashSet::new();
    for event in &document.hazard_events {
        require_non_empty(
            &mut findings,
            "event_id",
            "hazard_event.event_id",
            &event.event_id,
        );
        require_non_empty(&mut findings, "event_label", &event.event_id, &event.label);
        if !event_ids.insert(event.event_id.as_str()) {
            findings.push(error(
                "duplicate_event_id",
                &event.event_id,
                "hazard event id appears more than once",
            ));
        }
        if !system_ids.contains(event.weather_system.as_str()) {
            findings.push(error(
                "unknown_weather_system",
                &event.event_id,
                &format!(
                    "hazard event references unknown weather system `{}`",
                    event.weather_system
                ),
            ));
        }
        require_non_empty(&mut findings, "exposure", &event.event_id, &event.exposure);
        require_non_empty(
            &mut findings,
            "consequence",
            &event.event_id,
            &event.consequence,
        );
        if event.affected_systems.is_empty() {
            findings.push(warning(
                "event_without_handoff",
                &event.event_id,
                "hazard event has no downstream system handoff",
            ));
        }
        for affected in &event.affected_systems {
            require_non_empty(
                &mut findings,
                "affected_handoff",
                &event.event_id,
                &affected.handoff,
            );
        }
    }

    let mut profile_ids = HashSet::new();
    for profile in &document.preparedness_profiles {
        require_non_empty(
            &mut findings,
            "profile_id",
            "preparedness_profile.profile_id",
            &profile.profile_id,
        );
        if !profile_ids.insert(profile.profile_id.as_str()) {
            findings.push(error(
                "duplicate_profile_id",
                &profile.profile_id,
                "preparedness profile id appears more than once",
            ));
        }
        if !event_ids.contains(profile.target_hazard.as_str()) {
            findings.push(error(
                "unknown_target_hazard",
                &profile.profile_id,
                &format!(
                    "preparedness profile references unknown hazard `{}`",
                    profile.target_hazard
                ),
            ));
        }
        require_non_empty(
            &mut findings,
            "preparation",
            &profile.profile_id,
            &profile.preparation,
        );
        require_non_empty(
            &mut findings,
            "tradeoff",
            &profile.profile_id,
            &profile.tradeoff,
        );
    }

    let mut window_ids = HashSet::new();
    for window in &document.recovery_windows {
        require_non_empty(
            &mut findings,
            "window_id",
            "recovery_window.window_id",
            &window.window_id,
        );
        if !window_ids.insert(window.window_id.as_str()) {
            findings.push(error(
                "duplicate_window_id",
                &window.window_id,
                "recovery window id appears more than once",
            ));
        }
        if !event_ids.contains(window.hazard_event.as_str()) {
            findings.push(error(
                "unknown_window_hazard",
                &window.window_id,
                &format!(
                    "recovery window references unknown hazard `{}`",
                    window.hazard_event
                ),
            ));
        }
        require_non_empty(
            &mut findings,
            "window_timing",
            &window.window_id,
            &window.timing,
        );
        require_non_empty(
            &mut findings,
            "recovery_action",
            &window.window_id,
            &window.recovery_action,
        );
        require_non_empty(
            &mut findings,
            "downstream_decision",
            &window.window_id,
            &window.downstream_decision,
        );
    }

    findings
}

pub fn has_errors(findings: &[ValidationFinding]) -> bool {
    findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error)
}

fn require_non_empty(
    findings: &mut Vec<ValidationFinding>,
    code: &str,
    location: &str,
    value: &str,
) {
    if value.trim().is_empty() {
        findings.push(error(code, location, "required field is empty"));
    }
}

fn error(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Error,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

fn warning(code: &str, location: &str, message: &str) -> ValidationFinding {
    ValidationFinding {
        severity: FindingSeverity::Warning,
        code: code.to_string(),
        location: location.to_string(),
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_storm_document_has_no_errors() {
        let document = seed_document();
        assert!(!has_errors(&validate_document(&document)));
    }

    #[test]
    fn hazard_events_must_reference_weather_systems() {
        let mut document = seed_document();
        document.hazard_events[0].weather_system = "weather:missing".to_string();
        let findings = validate_document(&document);
        assert!(findings
            .iter()
            .any(|finding| finding.code == "unknown_weather_system"));
        assert!(has_errors(&findings));
    }

    #[test]
    fn recovery_windows_must_reference_hazards() {
        let mut document = seed_document();
        document.recovery_windows[0].hazard_event = "hazard:missing".to_string();
        let findings = validate_document(&document);
        assert!(findings
            .iter()
            .any(|finding| finding.code == "unknown_window_hazard"));
        assert!(has_errors(&findings));
    }

    fn seed_document() -> StormDocument {
        StormDocument {
            document_id: "storm:test".to_string(),
            weather_systems: vec![WeatherSystem {
                system_id: "weather:late-autumn-raintrain".to_string(),
                label: "Late autumn raintrain".to_string(),
                kind: WeatherKind::HeavyRain,
                seasonality: "late autumn".to_string(),
                signals: vec![WeatherSignal {
                    signal_id: "signal:three-day-warm-rain".to_string(),
                    description: "Warm rain persists for three days before river rise.".to_string(),
                    lead_time: Some("3 days".to_string()),
                }],
                evidence_refs: vec![evidence("weather:late-autumn-raintrain")],
            }],
            hazard_events: vec![HazardEvent {
                event_id: "hazard:mill-race-flood".to_string(),
                label: "Mill race flood".to_string(),
                weather_system: "weather:late-autumn-raintrain".to_string(),
                hazard_kind: HazardKind::FloodDamage,
                exposure: "Low mill path, grain store, and footbridge are exposed.".to_string(),
                consequence: "Flood timing threatens grain, route access, and repair labor."
                    .to_string(),
                affected_systems: vec![
                    affected(DownstreamRepo::Porto, "Route closure and access evidence."),
                    affected(DownstreamRepo::Ceres, "Storage and labor shock evidence."),
                    affected(DownstreamRepo::Banish, "Playable flood-response pressure."),
                ],
                evidence_refs: vec![evidence("hazard:mill-race-flood")],
            }],
            preparedness_profiles: vec![PreparednessProfile {
                profile_id: "preparedness:sandbag-mill-path".to_string(),
                label: "Sandbag mill path".to_string(),
                target_hazard: "hazard:mill-race-flood".to_string(),
                preparation: "Stage barriers and move grain before river crest.".to_string(),
                tradeoff: "Uses dry labor hours that CERES might need for harvest.".to_string(),
                evidence_refs: vec![evidence("preparedness:sandbag-mill-path")],
            }],
            recovery_windows: vec![RecoveryWindow {
                window_id: "recovery:bridge-before-freeze".to_string(),
                hazard_event: "hazard:mill-race-flood".to_string(),
                timing: "after water falls and before first freeze".to_string(),
                recovery_action: "Repair footbridge and dry grain store.".to_string(),
                downstream_decision:
                    "BANISH can choose route repair, grain salvage, or winter shelter labor."
                        .to_string(),
                evidence_refs: vec![evidence("recovery:bridge-before-freeze")],
            }],
        }
    }

    fn affected(repo: DownstreamRepo, handoff: &str) -> AffectedSystem {
        AffectedSystem {
            repo,
            handoff: handoff.to_string(),
        }
    }

    fn evidence(anchor: &str) -> EvidenceRef {
        EvidenceRef {
            source_id: "seed".to_string(),
            path: "fixtures\\seed-storm.json".to_string(),
            anchor: Some(anchor.to_string()),
        }
    }
}
