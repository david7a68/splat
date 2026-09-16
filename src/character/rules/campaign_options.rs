//! Explicit optional grants. Empty provider selections grant nothing; missing data stays unknown.
use super::*;
use crate::content::character::{CampaignOptionSelection, CampaignOptionState};

const SYSTEMS: [(u64, &str); 3] = [
    (2048517, "dark_bargain"),
    (2331813, "character_threads"),
    (2331889, "runestones"),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionRegistry {
    version: u32,
    options: BTreeMap<String, OptionRule>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionRule {
    system: String,
    name: String,
    family: String,
    stage: u64,
    source_sha256: String,
    parameters: Value,
    required_inputs: Vec<Gap>,
    #[serde(default)]
    conversion_gaps: Vec<Gap>,
}
fn catalogue() -> &'static OptionRegistry {
    static OPTIONS: OnceLock<OptionRegistry> = OnceLock::new();
    OPTIONS.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../builtin/dndbeyond/campaign-options.json"
        ))
        .expect("checked campaign option catalogue")
    })
}
fn option_fingerprint(source: &Value) -> String {
    let values: BTreeMap<_, _> = ["id", "label", "description", "sourceId"]
        .into_iter()
        .map(|key| (key, source[key].clone()))
        .collect();
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&values).expect("JSON option"))
    )
}
pub(super) fn is_system_choice(choice: &Value) -> bool {
    choice["componentTypeId"] == 1088085227_u64
        && SYSTEMS.iter().any(|(id, _)| choice["componentId"] == *id)
}
fn add_option(rules: &mut Rules<'_>, option_id: &str, spec: &OptionRule) -> Result<Uuid, String> {
    let identity = format!("campaign_option:{option_id}");
    if let Some(id) = rules.identities.get(&identity) {
        return Ok(*id);
    }
    let id = reference_id(rules.record, "rule", &identity);
    let implementation = format!("dndbeyond:campaign_options/{}", spec.system);
    let mut parameters = spec.parameters.clone();
    rules.remap(&mut parameters)?;
    let mut fields = json!({
        "splat_schema": 1, "splat_id": id, "splat_kind": "rule", "splat_name": spec.name,
        "splat_rule": {"mechanic": implementation, "implementation": implementation, "parameters": parameters},
        "splat_requires_mechanics": [implementation],
        "splat_contracts": [{"id": implementation,
            "inputs": {"actor": {"type": "participant", "required": true}, "selection": {"type": "choice", "required": true}, "event": {"type": "event", "required": false}, "targets": {"type": "participant_list", "required": false}},
            "outputs": ["grants", "state_changes"], "events": ["rule.invoked", "rule.completed"],
            "capabilities": ["declarative_rule_parameters", "character_grants"]}]
    });
    let mut report = empty_report();
    report.gaps = spec.conversion_gaps.clone();
    report.engine_gaps.push(implementation);
    report.required_inputs = spec.required_inputs.clone();
    put_coverage(&mut fields, report)?;
    rules.notes.insert(id, note(
        format!("Definitions/{}/Rules/{id}.md", rules.record.definition.splat_id), fields,
        &format!("# {}\n\nSource option `{option_id}`; family `{}`, stage {}. Granted only while selected. Supplied character totals already include provider modifiers. Simulation support is separate.", spec.name, spec.family, spec.stage),
    )?);
    rules.identities.insert(identity, id);
    Ok(id)
}

pub(super) fn complete(
    rules: &mut Rules<'_>,
    sheet: &mut Sheet,
    report: &mut Import,
) -> Result<(), String> {
    if catalogue().version != 1 {
        return Err("Unknown campaign option catalogue".into());
    }
    let record = rules.record;
    for (feature_id, system) in SYSTEMS {
        let feature = record
            .character
            .features
            .iter()
            .map(|f| f.get("definition").unwrap_or(f))
            .find(|f| f["id"] == feature_id);
        let Some(feature) = feature else {
            sheet
                .campaign_options
                .insert(system.into(), CampaignOptionState::Inactive);
            continue;
        };
        let choices: Vec<_> = record.source.raw["choices"]
            .as_object()
            .into_iter()
            .flat_map(|groups| groups.iter())
            .filter(|(key, _)| key.as_str() != "choiceDefinitions")
            .flat_map(|(_, choices)| list(choices))
            .filter(|c| is_system_choice(c) && c["componentId"] == feature_id)
            .collect();
        if choices.is_empty()
            || choices.iter().any(|c| c.get("optionValue").is_none())
            || !sheet.rules.contains_key(&feature_key(feature))
        {
            sheet
                .campaign_options
                .insert(system.into(), CampaignOptionState::Unknown);
            report.required_inputs.push(Gap { path: format!("campaign_options/{system}"), reason: "Source selection data or its parent rule is unavailable; no optional benefit is granted.".into() });
            continue;
        }
        let mut selections = Vec::new();
        let mut families = BTreeSet::new();
        for choice in choices {
            let selected = &choice["optionValue"];
            if selected.is_null() {
                continue;
            }
            let option_id = source_id(selected);
            let scope = format!("{}-{}", choice["componentTypeId"], choice["type"]);
            let source = list(&choice["options"])
                .iter()
                .chain(
                    list(&record.source.raw["choices"]["choiceDefinitions"])
                        .iter()
                        .filter(|d| d["id"] == scope)
                        .flat_map(|d| list(&d["options"])),
                )
                .find(|o| o["id"] == *selected && o.get("description").is_some());
            let spec = catalogue()
                .options
                .get(&option_id)
                .filter(|s| s.system == system)
                .filter(|s| source.is_some_and(|o| option_fingerprint(o) == s.source_sha256))
                .filter(|_| list(&choice["optionIds"]).contains(selected));
            let rule = if let Some(spec) = spec {
                if !families.insert(&spec.family) {
                    report.gaps.push(Gap { path: format!("campaign_options/{system}/{option_id}"), reason: "Multiple stages of the same optional grant are selected; resolve the source selections before granting effects.".into() });
                    None
                } else {
                    Some(EntryRef {
                        id: add_option(rules, &option_id, spec)?,
                    })
                }
            } else {
                report.gaps.push(Gap { path: format!("campaign_options/{system}/{option_id}"), reason: "Selected option is unknown, changed, or not allowed by its source choice; its rules need conversion.".into() });
                None
            };
            selections.push(CampaignOptionSelection {
                choice_id: source_id(&choice["id"]),
                option_id: option_id.clone(),
                name: source
                    .and_then(|o| o["label"].as_str())
                    .unwrap_or(&option_id)
                    .into(),
                rule,
            });
        }
        sheet.campaign_options.insert(
            system.into(),
            if selections.is_empty() {
                CampaignOptionState::Unselected
            } else {
                CampaignOptionState::Active { selections }
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        serde_json::from_str(include_str!("fixtures/campaign-options.json")).unwrap()
    }
    fn sample(feature: u64, option: Option<u64>) -> EntitySnapshot {
        let mut record = crate::character::tests::sample();
        let fixture = fixture();
        record.character.features = list(&fixture["features"])
            .iter()
            .filter(|f| f["id"] == feature)
            .cloned()
            .collect();
        let ids: Vec<_> = catalogue()
            .options
            .iter()
            .filter(|(_, s)| s.system == SYSTEMS.iter().find(|(id, _)| *id == feature).unwrap().1)
            .map(|(id, _)| id.parse::<u64>().unwrap())
            .collect();
        record.source.raw["choices"] = json!({
            "feat": [{"id":"test-choice", "componentId":feature, "componentTypeId":1088085227_u64, "type":3, "optionIds":ids, "optionValue":option, "options":[]}],
            "choiceDefinitions":[{"id":"1088085227-3", "options":fixture["options"]}]
        });
        record
    }
    fn sheet(record: &EntitySnapshot) -> (Sheet, Import) {
        let entry = record.entry().unwrap();
        (
            serde_yaml_ng::from_value(entry.fields["splat_character"].clone()).unwrap(),
            serde_yaml_ng::from_value(entry.fields["splat_import"].clone()).unwrap(),
        )
    }
    #[test]
    fn absent_unselected_and_unknown_are_distinct_and_grant_nothing() {
        let (s, _) = sheet(&crate::character::tests::sample());
        assert!(matches!(
            s.campaign_options["dark_bargain"],
            CampaignOptionState::Inactive
        ));
        let mut record = sample(2048517, None);
        let (s, report) = sheet(&record);
        assert!(matches!(
            s.campaign_options["dark_bargain"],
            CampaignOptionState::Unselected
        ));
        assert!(
            !report
                .required_inputs
                .iter()
                .any(|g| g.path.starts_with("campaign_options/")
                    || g.reason.contains("Dark Bargain"))
        );
        record.source.raw["choices"]["feat"] = json!([]);
        let (s, report) = sheet(&record);
        assert!(matches!(
            s.campaign_options["dark_bargain"],
            CampaignOptionState::Unknown
        ));
        assert!(
            report
                .required_inputs
                .iter()
                .any(|g| g.path == "campaign_options/dark_bargain")
        );
    }
    #[test]
    fn active_bargain_imports_benefit_and_cost_and_deselect_removes_grant() {
        let mut record = sample(2048517, Some(5098965));
        let (s, report) = sheet(&record);
        let CampaignOptionState::Active { selections } = &s.campaign_options["dark_bargain"] else {
            panic!("expected active")
        };
        let id = selections[0].rule.as_ref().unwrap().id;
        let rule = record
            .content_notes()
            .unwrap()
            .into_iter()
            .map(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap())
            .find(|e| e.id == id)
            .unwrap();
        let params = &rule.fields["splat_rule"]["parameters"];
        assert_eq!(params["ancient_might"]["strength"]["add"].as_i64(), Some(2));
        assert_eq!(
            params["primeval_strength"]["duration_minutes"].as_i64(),
            Some(1)
        );
        assert_eq!(params["pyres_price"]["until"].as_str(), Some("long_rest"));
        assert!(
            !report
                .gaps
                .iter()
                .any(|g| g.path.starts_with("campaign_options/"))
        );
        record.source.raw["choices"]["feat"][0]["optionValue"] = Value::Null;
        let (s, _) = sheet(&record);
        assert!(matches!(
            s.campaign_options["dark_bargain"],
            CampaignOptionState::Unselected
        ));
        assert!(
            !record
                .content_notes()
                .unwrap()
                .iter()
                .any(|n| n.markdown.contains(&id.to_string()))
        );
    }
    #[test]
    fn all_three_systems_use_scoped_selected_definitions() {
        for (feature, option, system) in [
            (2048517, 5098965, "dark_bargain"),
            (2331813, 5807813, "character_threads"),
            (2331889, 5807896, "runestones"),
        ] {
            let mut record = sample(feature, Some(option));
            // A short selected-option row is not the full source definition.
            record.source.raw["choices"]["feat"][0]["options"] =
                json!([{"id":option,"label":"selected"}]);
            let (s, _) = sheet(&record);
            let CampaignOptionState::Active { selections } = &s.campaign_options[system] else {
                panic!("expected active")
            };
            assert!(selections[0].rule.is_some());
            record.source.raw["choices"]["feat"][0]
                .as_object_mut()
                .unwrap()
                .remove("optionValue");
            let (s, _) = sheet(&record);
            assert!(matches!(
                s.campaign_options[system],
                CampaignOptionState::Unknown
            ));
        }
    }
    #[test]
    fn source_drift_and_disallowed_choice_never_receive_verified_rules() {
        for tamper in ["description", "optionIds"] {
            let mut record = sample(2331889, Some(5807896));
            if tamper == "description" {
                let opts = record.source.raw["choices"]["choiceDefinitions"][0]["options"]
                    .as_array_mut()
                    .unwrap();
                opts.iter_mut().find(|o| o["id"] == 5807896).unwrap()["description"] =
                    json!("Changed rules");
            } else {
                record.source.raw["choices"]["feat"][0]["optionIds"] = json!([]);
            }
            let (s, report) = sheet(&record);
            let CampaignOptionState::Active { selections } = &s.campaign_options["runestones"]
            else {
                panic!("selection lost")
            };
            assert!(selections[0].rule.is_none());
            assert!(
                report
                    .gaps
                    .iter()
                    .any(|g| g.path == "campaign_options/runestones/5807896")
            );
        }
    }
    #[test]
    fn all_option_definitions_and_transitive_references_validate() {
        assert_eq!(catalogue().options.len(), 95);
        let record = crate::character::tests::sample();
        let mut rules = Rules::new(&record);
        for (id, spec) in &catalogue().options {
            assert_eq!(spec.source_sha256.len(), 64);
            add_option(&mut rules, id, spec).unwrap();
        }
        let entries: Vec<_> = rules
            .notes
            .values()
            .map(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap())
            .collect();
        let report = crate::content::validate(&entries);
        assert!(report.errors.is_empty(), "{:?}", report.errors);
    }
    #[test]
    fn selected_reward_with_unresolved_spell_reports_gap_on_character() {
        let record = sample(2331813, Some(5807810));
        let (s, report) = sheet(&record);
        assert!(matches!(
            s.campaign_options["character_threads"],
            CampaignOptionState::Active { .. }
        ));
        assert!(report.gaps.iter().any(|g| g.path.ends_with("/wrath_spell")));
        assert!(
            report
                .required_inputs
                .iter()
                .any(|g| g.reason.contains("Wrathful Smite version"))
        );
        let mut inactive = record;
        inactive.source.raw["choices"]["feat"][0]["optionValue"] = Value::Null;
        let (_, report) = sheet(&inactive);
        assert!(!report.gaps.iter().any(|g| g.path.ends_with("/wrath_spell")));
        assert!(
            !report
                .required_inputs
                .iter()
                .any(|g| g.reason.contains("Wrathful Smite version"))
        );
    }
    #[test]
    fn active_schema_rejects_missing_selections_and_checks_rule_references() {
        let record = sample(2331889, Some(5807896));
        let entry = record.entry().unwrap();
        let mut fields = serde_json::to_value(entry.fields).unwrap();
        fields["splat_character"]["campaign_options"]["runestones"]["selections"] = json!([]);
        put_coverage(
            &mut fields,
            coverage(&serde_json::to_value(record.entry().unwrap().fields).unwrap()).unwrap(),
        )
        .unwrap();
        let n = note("invalid.md".into(), fields, "").unwrap();
        assert!(
            Entry::parse(&n.relative_path, &n.markdown)
                .unwrap_err()
                .contains("selections")
        );
        let mut notes = record.content_notes().unwrap();
        let (s, _) = sheet(&record);
        let CampaignOptionState::Active { selections } = &s.campaign_options["runestones"] else {
            panic!("expected active")
        };
        let id = selections[0].rule.as_ref().unwrap().id;
        notes.retain(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap().id != id);
        let entries: Vec<_> = notes
            .iter()
            .map(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap())
            .collect();
        assert!(!crate::content::validate(&entries).errors.is_empty());
    }
}
