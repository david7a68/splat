//! Source-pinned conversion. A provider ID or display name alone never proves rule equivalence.
use super::{
    content::{named_key, reference_id},
    *,
};
use crate::content::character::{
    CastingBinding, Choice, ClassProgression, Contribution, EntryRef, Gap, Import, Sheet,
};
use std::sync::OnceLock;
mod campaign_options;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    version: u32,
    sources: BTreeMap<String, Conversion>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Conversion {
    name: String,
    source_sha256: String,
    builtin: Option<String>,
    parameters: Option<Value>,
    required_inputs: Vec<Gap>,
    mechanic: String,
}
fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        serde_json::from_str(include_str!("../../builtin/dndbeyond/conversions.json"))
            .expect("checked conversion registry")
    })
}
fn builtin() -> Result<&'static BTreeMap<String, Entry>, String> {
    static BUILTIN: OnceLock<Result<BTreeMap<String, Entry>, String>> = OnceLock::new();
    BUILTIN
        .get_or_init(|| {
            Ok(crate::content::definitions::entries()?
                .into_iter()
                .filter_map(|e| e.key.clone().map(|k| (k, e)))
                .collect())
        })
        .as_ref()
        .map_err(Clone::clone)
}
/// Resolve only a source-pinned match to its canonical shared definition.
pub(super) fn shared_reference(identity: &str, source: &Value) -> Result<Option<Uuid>, String> {
    let Some(spec) = registry()
        .sources
        .get(identity)
        .filter(|s| s.source_sha256 == fingerprint(source))
    else {
        return Ok(None);
    };
    let Some(key) = &spec.builtin else {
        return Ok(None);
    };
    let entry = builtin()?
        .get(key)
        .ok_or_else(|| format!("Missing converted rule {key}"))?;
    let meta = crate::content::definitions::conversion(entry)?.ok_or("Missing builtin coverage")?;
    if !meta.conversion_gaps.is_empty() {
        return Err(format!("Builtin rule still has conversion gaps: {key}"));
    }
    Ok(Some(entry.id))
}

/// Public source link for a shared rule, usable both in Splat and other Markdown readers.
pub(super) fn shared_source_link(id: Uuid) -> Result<Option<String>, String> {
    let Some(entry) = builtin()?.values().find(|entry| entry.id == id) else {
        return Ok(None);
    };
    let meta = crate::content::definitions::conversion(entry)?.ok_or("Missing builtin coverage")?;
    let page = meta
        .pages
        .first()
        .ok_or("Missing shared rule source page")?;
    Ok(Some(format!(
        "{}#page={page}",
        crate::content::builtin::SOURCE_URL
    )))
}

/// Paths emitted by the former copying importer that now resolve to the shared catalogue.
pub(super) fn retired_shared_paths(record: &EntitySnapshot) -> Result<Vec<String>, String> {
    let folder = format!("Definitions/{}", record.definition.splat_id);
    let mut paths = BTreeSet::new();
    for key in builtin()?.keys() {
        paths.insert(format!(
            "{folder}/Rules/{}.md",
            reference_id(record, "rule", key)
        ));
    }
    for item in &record.inventory {
        if shared_reference(
            &format!("item:{}", string(&item.definition["definitionKey"])),
            &item.definition,
        )?
        .is_some()
        {
            paths.insert(format!(
                "{folder}/{}.md",
                reference_id(record, "item", &item.definition_id.to_string())
            ));
        }
    }
    for spell in &record.character.spells {
        let source = source_id(&spell["definition"]["id"]);
        if shared_reference(&format!("spell:{source}"), &spell["definition"])?.is_some() {
            paths.insert(format!(
                "{folder}/{}.md",
                reference_id(record, "spell", &source)
            ));
        }
    }
    Ok(paths.into_iter().collect())
}

fn fingerprint(source: &Value) -> String {
    let keys = [
        "id",
        "entityTypeId",
        "definitionKey",
        "name",
        "description",
        "snippet",
        "sources",
        "isLegacy",
        "level",
        "damage",
        "damageType",
        "properties",
        "range",
        "longRange",
        "armorClass",
        "armorTypeId",
        "components",
        "duration",
        "activation",
    ];
    let selected: BTreeMap<_, _> = keys
        .into_iter()
        .map(|key| (key, source[key].clone()))
        .collect();
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&selected).expect("JSON source"))
    )
}
fn coverage(fields: &Value) -> Result<Import, String> {
    serde_json::from_value(fields["splat_import"].clone()).map_err(|e| e.to_string())
}
fn put_coverage(fields: &mut Value, mut report: Import) -> Result<(), String> {
    report.version = 2;
    report.engine_gaps.sort();
    report.engine_gaps.dedup();
    report.rule_fingerprint = Some(crate::content::character::rule_fingerprint(
        &serde_yaml_ng::to_value(&*fields)
            .map_err(|e| e.to_string())?
            .as_mapping()
            .ok_or("Invalid fields")?
            .clone(),
    )?);
    fields["splat_import"] = serde_json::to_value(report).map_err(|e| e.to_string())?;
    Ok(())
}
fn empty_report() -> Import {
    Import {
        version: 2,
        gaps: vec![],
        engine_gaps: vec![],
        required_inputs: vec![],
        rule_fingerprint: None,
    }
}
fn note(path: String, fields: Value, body: &str) -> Result<ContentNote, String> {
    Ok(ContentNote {
        relative_path: path,
        markdown: format!(
            "---\n{}---\n\n{body}\n",
            serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?
        ),
    })
}
struct Rules<'a> {
    record: &'a EntitySnapshot,
    notes: BTreeMap<Uuid, ContentNote>,
    identities: BTreeMap<String, Uuid>,
    shared_engine_gaps: BTreeSet<String>,
}
impl<'a> Rules<'a> {
    fn new(record: &'a EntitySnapshot) -> Self {
        Self {
            record,
            notes: BTreeMap::new(),
            identities: BTreeMap::new(),
            shared_engine_gaps: BTreeSet::new(),
        }
    }
    fn remap(&mut self, value: &mut Value) -> Result<(), String> {
        match value {
            Value::Object(map) => {
                if !map.is_empty()
                    && map
                        .keys()
                        .all(|k| ["id", "key", "link"].contains(&k.as_str()))
                {
                    let key = if let Some(k) = map.get("key").and_then(Value::as_str) {
                        Some(k.to_owned())
                    } else if let Some(id) = map.get("id").and_then(Value::as_str) {
                        builtin()?
                            .iter()
                            .find(|(_, e)| e.id.to_string() == id)
                            .map(|(k, _)| k.clone())
                    } else {
                        None
                    };
                    if let Some(key) = key {
                        *value = json!({"id":self.add_builtin(&key)?});
                        return Ok(());
                    }
                }
                for child in map.values_mut() {
                    self.remap(child)?;
                }
            }
            Value::Array(values) => {
                for child in values {
                    self.remap(child)?;
                }
            }
            _ => (),
        }
        Ok(())
    }
    fn add_builtin(&mut self, key: &str) -> Result<Uuid, String> {
        if let Some(id) = self.identities.get(key) {
            return Ok(*id);
        }
        let original = builtin()?
            .get(key)
            .ok_or_else(|| format!("Missing converted rule {key}"))?
            .clone();
        let meta = crate::content::definitions::conversion(&original)?
            .ok_or("Missing builtin coverage")?;
        if !meta.conversion_gaps.is_empty() {
            return Err(format!("Builtin rule still has conversion gaps: {key}"));
        }
        let id = original.id;
        self.identities.insert(key.into(), id);
        self.shared_engine_gaps.extend(meta.engine_gaps);
        // Keep the catalogue's identity and dependencies. Never publish an SRD copy.
        for dependency in crate::content::required_references(&original)? {
            let key = match dependency {
                crate::content::Reference::Key(key) => Some(key),
                crate::content::Reference::Id(id) => builtin()?
                    .iter()
                    .find(|(_, entry)| entry.id == id)
                    .map(|(key, _)| key.clone()),
            };
            if let Some(key) = key {
                self.add_builtin(&key)?;
            }
        }
        Ok(id)
    }
    fn apply(
        &mut self,
        identity: &str,
        source: &Value,
        fields: &mut Value,
    ) -> Result<bool, String> {
        let Some(spec) = registry()
            .sources
            .get(identity)
            .filter(|s| s.source_sha256 == fingerprint(source))
        else {
            return Ok(false);
        };
        let mut report = coverage(fields)?;
        if spec.builtin.is_some() {
            return Err("Shared source definitions must be referenced, not copied".into());
        }
        let mut parameters = spec
            .parameters
            .clone()
            .ok_or("Conversion requires typed parameters")?;
        self.remap(&mut parameters)?;
        let implementation = format!("dndbeyond:rules/{}", spec.mechanic);
        fields["splat_rule"] = json!({"mechanic":implementation,"implementation":implementation,"parameters":parameters});
        fields["splat_requires_mechanics"] = json!([implementation]);
        fields["splat_contracts"] = json!([{"id":implementation,"inputs":{"actor":{"type":"participant","required":true},"selection":{"type":"choice","required":false},"event":{"type":"event","required":false},"targets":{"type":"participant_list","required":false},"source_item":{"type":"item_instance","required":false}},"outputs":["grants","state_changes"],"events":["rule.invoked","rule.completed"],"capabilities":["declarative_rule_parameters","character_grants"]}]);
        report.engine_gaps.push(implementation);
        report.gaps.clear();
        report.required_inputs.extend(spec.required_inputs.clone());
        put_coverage(fields, report)?;
        Ok(true)
    }
    fn add_source(&mut self, identity: &str, source: &Value) -> Result<Option<Uuid>, String> {
        let Some(spec) = registry()
            .sources
            .get(identity)
            .filter(|s| s.source_sha256 == fingerprint(source))
        else {
            return Ok(None);
        };
        if let Some(id) = self.identities.get(identity) {
            return Ok(Some(*id));
        }
        if let Some(key) = &spec.builtin {
            let id = self.add_builtin(key)?;
            self.identities.insert(identity.into(), id);
            return Ok(Some(id));
        }
        let canonical = format!("dndbeyond:rules/{}", spec.mechanic);
        if let Some(id) = self.identities.get(&canonical).copied() {
            self.identities.insert(identity.into(), id);
            return Ok(Some(id));
        }
        let id = reference_id(self.record, "rule", &canonical);
        self.identities.insert(canonical, id);
        let mut fields = json!({"splat_schema":1,"splat_id":id,"splat_name":spec.name,"splat_kind":"rule","splat_import":empty_report()});
        self.apply(identity, source, &mut fields)?;
        self.identities.insert(identity.into(), id);
        self.notes.insert(
            id,
            note(
                format!(
                    "Definitions/{}/Rules/{id}.md",
                    self.record.definition.splat_id
                ),
                fields,
                &format!(
                    "# {}\n\nSource-pinned rule conversion. Engine implementation is separate.",
                    spec.name
                ),
            )?,
        );
        Ok(Some(id))
    }
}
fn feature_key(d: &Value) -> String {
    format!(
        "feature_{}",
        stable_id(&format!(
            "{}:{}",
            d["entityTypeId"],
            id(&d["id"]).unwrap_or_else(|| string(&d["name"]).into())
        ))
        .simple()
    )
}
fn source_id(v: &Value) -> String {
    v.as_str()
        .map(str::to_owned)
        .or_else(|| v.as_u64().map(|n| n.to_string()))
        .unwrap_or_default()
}

pub(super) fn complete(
    record: &EntitySnapshot,
    notes: &mut Vec<ContentNote>,
) -> Result<(), String> {
    if registry().version != 1 {
        return Err("Unknown conversion registry".into());
    }
    let mut rules = Rules::new(record);
    let mut creature = notes.pop().ok_or("Missing creature")?;
    for original in notes.iter_mut() {
        let entry = Entry::parse(&original.relative_path, &original.markdown)?;
        let mut fields = serde_json::to_value(&entry.fields).map_err(|e| e.to_string())?;
        let source = if entry.kind == "item" {
            record
                .inventory
                .iter()
                .find(|i| reference_id(record, "item", &i.definition_id.to_string()) == entry.id)
                .map(|i| {
                    (
                        format!("item:{}", string(&i.definition["definitionKey"])),
                        &i.definition,
                    )
                })
        } else {
            record.character.spells.iter().find_map(|s| {
                let d = &s["definition"];
                let identity = id(&d["id"])
                    .or_else(|| d["definitionKey"].as_str().map(str::to_owned))
                    .unwrap_or_else(|| string(&d["name"]).into());
                (reference_id(record, "spell", &identity) == entry.id)
                    .then(|| (format!("spell:{}", source_id(&d["id"])), d))
            })
        };
        if let Some((identity, source)) = source {
            rules.apply(&identity, source, &mut fields)?;
        }
        *original = note(original.relative_path.clone(), fields, &entry.body)?;
    }
    let entry = Entry::parse(&creature.relative_path, &creature.markdown)?;
    for reference in crate::content::required_references(&entry)? {
        let key = match reference {
            crate::content::Reference::Id(id) => builtin()?
                .iter()
                .find(|(_, e)| e.id == id)
                .map(|(k, _)| k.clone()),
            crate::content::Reference::Key(key) => builtin()?.contains_key(&key).then_some(key),
        };
        if let Some(key) = key {
            rules.add_builtin(&key)?;
        }
    }
    let mut fields = serde_json::to_value(&entry.fields).map_err(|e| e.to_string())?;
    let mut sheet: Sheet =
        serde_json::from_value(fields["splat_character"].clone()).map_err(|e| e.to_string())?;
    let mut report = coverage(&fields)?;
    let mut converted = BTreeSet::new();
    for feature in &record.character.features {
        let d = feature.get("definition").unwrap_or(feature);
        let key = feature_key(d);
        if let Some(id) = rules.add_source(&format!("feature:{}", source_id(&d["id"])), d)? {
            sheet.rules.insert(key.clone(), EntryRef { id });
            converted.insert(format!("splat_character/features/{key}"));
        }
    }
    for (group, values) in record.source.raw["actions"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.iter())
    {
        for action in list(values) {
            let key = named_key(&format!("action_{}", super::key(group)), action);
            if let Some(id) =
                rules.add_source(&format!("action:{}", source_id(&action["id"])), action)?
            {
                sheet.rules.insert(key.clone(), EntryRef { id });
                converted.insert(format!("splat_character/features/{key}"));
                // Correct 2024 Rage recovery: one on a Short Rest, all on a Long Rest.
                if (action["id"].as_str() == Some("9414000") || action["id"] == 9414000)
                    && let Some(pool) = sheet.features.get(&key).and_then(|f| f.resource.as_ref())
                {
                    fields["splat_resources"][pool]["recovery"] = json!([{"key":"short_rest","mechanic":"dnd2024:short_rest","restore":{"add":1}},{"key":"long_rest","mechanic":"dnd2024:long_rest","restore":"all"}]);
                }
            }
        }
    }
    for (key, feature) in &sheet.features {
        if let Some(pool) = &feature.resource
            && let Some(rule) = sheet.rules.get(key)
        {
            sheet.resource_bindings.insert(
                key.clone(),
                crate::content::character::ResourceBinding {
                    rule: rule.clone(),
                    pool: pool.clone(),
                },
            );
        }
    }
    collect_choices(record, &mut sheet, &mut report);
    campaign_options::complete(&mut rules, &mut sheet, &mut report)?;
    for (group, values) in record.source.raw["modifiers"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.iter())
    {
        for m in list(values) {
            sheet.contributions.push(Contribution {
                source_id: source_id(&m["id"]),
                component_id: source_id(&m["componentId"]),
                category: group.into(),
                kind: string(&m["type"]).into(),
                stat: string(&m["subType"]).into(),
                amount: m["value"].as_i64(),
                ability: m["statId"]
                    .as_u64()
                    .and_then(|i| i.checked_sub(1))
                    .and_then(|i| ABILITIES.get(i as usize))
                    .map(|s| s.to_string()),
                restriction: string(&m["restriction"]).into(),
            });
        }
    }
    // Supplied totals are a defined baseline. Source contributions are not added again.
    fields["splat_stat_policy"] = json!({"mode":"supplied_baseline","contributions":"splat_character.contributions","apply_contributions_again":false,"when_included_source_changes":"require_refreshed_provider_total_or_validated_rebuild","dynamic_effects":"apply_only_changes_not_included_in_baseline"});
    converted.insert("splat_stats".into());
    report
        .engine_gaps
        .push("splat:supplied_stat_baseline".into());
    for (key, attack) in &sheet.attacks {
        let matched = record
            .inventory
            .iter()
            .filter(|i| i.name == attack.name)
            .collect::<Vec<_>>();
        let mut refs = Vec::new();
        for item in &matched {
            let shared = shared_reference(
                &format!("item:{}", string(&item.definition["definitionKey"])),
                &item.definition,
            )?;
            let target = shared
                .unwrap_or_else(|| reference_id(record, "item", &item.definition_id.to_string()));
            let is_converted = shared.is_some()
                || notes.iter().any(|n| {
                    Entry::parse(&n.relative_path, &n.markdown)
                        .is_ok_and(|e| e.id == target && e.fields.contains_key("splat_rule"))
                });
            if is_converted {
                refs.push(json!({"id":target}));
            }
        }
        if attack.name == "Unarmed Strike" {
            refs.push(json!({"id":rules.add_builtin("dnd2024:glossary/unarmed_strike")?}));
        }
        if !refs.is_empty() {
            fields["splat_attack_bindings"][key] = json!({"rules":refs,"inventory_rows":matched.iter().map(|i|format!("item_{}",i.source_id)).collect::<Vec<_>>(),"supplied_attack_bonus":attack.attack_bonus,"select_one_item_instance":!matched.is_empty(),"requires_equipment_state":if matched.is_empty(){"none"}else{"wielded"},"mastery_from":"splat_character.rules","damage_ability_already_included":true});
            converted.insert(format!("splat_actions/{key}"));
        } else if let Some(spell) = sheet.spells.values().find(|s| s.name == attack.name) {
            let is_converted = builtin()?.values().any(|e| e.id == spell.entry.id)
                || notes.iter().any(|n| {
                    Entry::parse(&n.relative_path, &n.markdown).is_ok_and(|e| {
                        e.id == spell.entry.id && e.fields.contains_key("splat_rule")
                    })
                });
            if is_converted {
                fields["splat_attack_bindings"][key] = json!({"rules":[spell.entry],"cast_via":"splat_character.spells","standalone_attack_is_cast":false});
                converted.insert(format!("splat_character/attacks/{key}"));
            }
        }
    }
    bind_spells(record, &mut sheet, &fields);
    report.gaps.retain(|g| !converted.contains(&g.path));
    let mut kept = Vec::new();
    for g in report.gaps.drain(..) {
        if g.path.starts_with("splat_inventory_details/") && g.reason.starts_with("Source marks") {
            report.required_inputs.push(g);
        } else {
            kept.push(g);
        }
    }
    report.gaps = kept;
    for rule in notes.iter().chain(rules.notes.values()) {
        let entry = Entry::parse(&rule.relative_path, &rule.markdown)?;
        let cov: Import = serde_yaml_ng::from_value(entry.fields["splat_import"].clone())
            .map_err(|e| e.to_string())?;
        report.engine_gaps.extend(cov.engine_gaps);
        for mut gap in cov.gaps {
            gap.path = format!("dependencies/{}/{}", entry.id, gap.path);
            if !report.gaps.iter().any(|g| g.path == gap.path) {
                report.gaps.push(gap);
            }
        }
        for input in cov.required_inputs {
            if !report.required_inputs.iter().any(|i| i.path == input.path) {
                report.required_inputs.push(input);
            }
        }
    }
    report.engine_gaps.extend(rules.shared_engine_gaps);
    fields["splat_character"] = serde_json::to_value(sheet).map_err(|e| e.to_string())?;
    put_coverage(&mut fields, report)?;
    let mut body = entry
        .body
        .split("## Import gaps\n")
        .next()
        .unwrap_or(&entry.body)
        .to_owned();
    let cov = coverage(&fields)?;
    body.push_str("## Conversion status\n\n");
    body.push_str(&format!(
        "{} conversion gaps; {} required engine adapters. Engine implementation is separate.\n\n",
        cov.gaps.len(),
        cov.engine_gaps.len()
    ));
    for gap in cov.gaps {
        body.push_str(&format!("- {}\n", gap.reason));
    }
    if !cov.required_inputs.is_empty() {
        body.push_str("\n### Required inputs\n\n");
    }
    for input in cov.required_inputs {
        body.push_str(&format!("- {}: {}\n", input.path, input.reason));
    }
    creature = note(creature.relative_path, fields, &body)?;
    notes.extend(rules.notes.into_values());
    notes.push(creature);
    Ok(())
}
fn collect_choices(record: &EntitySnapshot, sheet: &mut Sheet, report: &mut Import) {
    let definitions = list(&record.source.raw["choices"]["choiceDefinitions"]);
    for (group, choices) in record.source.raw["choices"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.iter())
        .filter(|(k, _)| k.as_str() != "choiceDefinitions")
    {
        for c in list(choices) {
            let selected_id = &c["optionValue"];
            let definition_key = format!("{}-{}", c["componentTypeId"], c["type"]);
            let defined_options = definitions
                .iter()
                .filter(|d| d["id"] == definition_key)
                .flat_map(|d| list(&d["options"]));
            let mut selected = if selected_id.is_null() {
                None
            } else {
                list(&c["options"])
                    .iter()
                    .chain(defined_options)
                    .find(|o| o["id"] == *selected_id)
                    .and_then(|o| o["label"].as_str())
                    .map(str::to_owned)
            };
            if selected.is_none() && !selected_id.is_null() {
                selected = list(&record.source.raw["options"][group])
                    .iter()
                    .find(|o| {
                        o["componentId"] == c["componentId"]
                            && o["definition"]["id"] == *selected_id
                    })
                    .and_then(|o| o["definition"]["name"].as_str())
                    .map(str::to_owned);
            }
            if selected.is_none() && c["type"] == 7 {
                selected = list(&record.source.raw["classes"])
                    .iter()
                    .find(|class| {
                        class["subclassDefinition"]["id"] == *selected_id && !selected_id.is_null()
                    })
                    .and_then(|class| class["subclassDefinition"]["name"].as_str())
                    .map(str::to_owned);
            }
            let active = record
                .character
                .features
                .iter()
                .any(|f| f.get("definition").unwrap_or(f)["id"] == c["componentId"])
                || group == "background";
            let label = c["label"]
                .as_str()
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    record
                        .character
                        .features
                        .iter()
                        .map(|f| f.get("definition").unwrap_or(f))
                        .find(|f| f["id"] == c["componentId"])
                        .map(|f| string(&f["name"]).into())
                        .unwrap_or_else(|| "Future class choice".into())
                });
            if active
                && selected.is_none()
                && c["isOptional"] != true
                && !campaign_options::is_system_choice(c)
            {
                report.required_inputs.push(Gap {
                    path: format!(
                        "choices/{group}/{}/{}",
                        source_id(&c["componentId"]),
                        source_id(&c["id"])
                    ),
                    reason: format!(
                        "{label}: source choice is unset or its selected label is unavailable."
                    ),
                });
            }
            sheet.choices.push(Choice {
                source_id: source_id(&c["id"]),
                component_id: source_id(&c["componentId"]),
                category: group.clone(),
                label,
                selected_option_id: (!selected_id.is_null()).then(|| source_id(selected_id)),
                selected,
            });
        }
    }
}
fn bind_spells(record: &EntitySnapshot, sheet: &mut Sheet, fields: &Value) {
    for class in list(&record.source.raw["classes"]) {
        let d = &class["definition"];
        let rules = &d["spellRules"];
        let numbers = |key: &str| {
            list(&rules[key])
                .iter()
                .map(|v| v.as_u64().unwrap_or(0))
                .collect()
        };
        sheet.class_progression.insert(
            super::key(string(&d["name"])),
            ClassProgression {
                cantrips: numbers("levelCantripsKnownMaxes"),
                spells_known: numbers("levelSpellKnownMaxes"),
                spells_prepared: numbers("levelPreparedSpellMaxes"),
                spell_slots: list(&rules["levelSpellSlots"])
                    .iter()
                    .map(|row| list(row).iter().map(|v| v.as_u64().unwrap_or(0)).collect())
                    .collect(),
                casting_ability: ability(&d["spellCastingAbilityId"]),
                ritual_caster: rules["isRitualSpellCaster"] == true,
            },
        );
    }
    for (key, spell) in &mut sheet.spells {
        let raw = record
            .character
            .spells
            .iter()
            .find(|s| named_key("spell", s) == *key);
        let class = raw
            .and_then(|s| {
                list(&record.source.raw["classSpells"])
                    .iter()
                    .find(|c| list(&c["spells"]).iter().any(|cs| cs["id"] == s["id"]))
            })
            .and_then(|c| {
                list(&record.source.raw["classes"])
                    .iter()
                    .find(|class| class["id"] == c["characterClassId"])
            });
        if spell.ability.is_none() {
            spell.ability = class.and_then(|c| ability(&c["definition"]["spellCastingAbilityId"]));
        }
        let class_name = class.map(|c| string(&c["definition"]["name"]));
        let total = |key: &str| {
            class_name
                .and_then(|name| {
                    list(&record.character.casting[key])
                        .iter()
                        .find(|v| list(&v["sources"]).iter().any(|s| s == name))
                })
                .and_then(|v| v["value"].as_i64())
        };
        let level = raw
            .and_then(|s| s["definition"]["level"].as_u64())
            .unwrap_or(0);
        let modifier = spell
            .ability
            .as_ref()
            .and_then(|a| record.stat(&format!("{a}_modifier")));
        let bonus = record.stat("proficiency_bonus");
        // Use the exact class's supplied total; racial grants retain their separate ability.
        let attack = total("spellAttacks")
            .or_else(|| {
                record
                    .character
                    .attacks
                    .iter()
                    .find(|a| string(&a["name"]) == spell.name)
                    .and_then(|a| a["toHit"].as_i64())
            })
            .or_else(|| modifier.zip(bonus).map(|(a, b)| a + b));
        let save = spell
            .save_dc
            .or_else(|| total("saveDcs"))
            .or_else(|| modifier.zip(bonus).map(|(a, b)| 8 + a + b));
        let pools = fields["splat_resources"]
            .as_object()
            .into_iter()
            .flat_map(|p| p.iter())
            .filter(|(_, p)| {
                p["kind"] == "spell_slot" && p["level"].as_u64().is_some_and(|l| l >= level)
            })
            .map(|(k, _)| k.clone())
            .collect();
        spell.casting = Some(CastingBinding {
            attack_bonus: attack,
            save_dc: save,
            slot_pools: if spell.uses_spell_slot {
                pools
            } else {
                Vec::new()
            },
            at_will: level == 0 || raw.is_some_and(|s| s["baseLevelAtWill"] == true),
            ritual_only: raw.is_some_and(|s| s["castOnlyAsRitual"] == true),
            free_cast_pool: None,
        });
    }
}
fn ability(value: &Value) -> Option<String> {
    value
        .as_u64()
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| ABILITIES.get(i as usize))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_uses_existing_complete_rules_and_exact_source_fingerprints() {
        assert_eq!(registry().version, 1);
        for (identity, spec) in &registry().sources {
            assert_eq!(spec.source_sha256.len(), 64, "{identity}");
            assert_ne!(
                spec.builtin.is_some(),
                spec.parameters.is_some(),
                "{identity}"
            );
            if let Some(key) = &spec.builtin {
                let entry = &builtin().unwrap()[key];
                let meta = crate::content::definitions::conversion(entry)
                    .unwrap()
                    .unwrap();
                assert!(meta.conversion_gaps.is_empty(), "{key}");
            }
        }
    }
}

#[cfg(test)]
mod conversion_tests {
    use super::*;
    #[test]
    fn matching_items_spells_and_features_reference_shared_entries_without_copies() {
        let data: Value =
            serde_json::from_str(include_str!("rules/fixtures/shared-srd.json")).unwrap();
        let mut record = super::super::tests::sample();
        for item in &mut record.inventory {
            item.definition = data["item"].clone();
            item.name = "Dagger".into();
        }
        record.character.spells = vec![
            json!({"id":17,"definition":data["spell"],"prepared":true,"usesSpellSlot":false,"spellCastingAbilityId":4}),
        ];
        record.character.features = vec![data["feature"].clone()];
        let notes = record.content_notes().unwrap();
        assert_eq!(
            notes.len(),
            1,
            "Matching definitions belong only to the shared catalogue"
        );
        let entry = Entry::parse(&notes[0].relative_path, &notes[0].markdown).unwrap();
        let dagger = builtin().unwrap()["dnd2024:equipment/dagger"].id;
        assert!(
            entry
                .inventory
                .iter()
                .all(|i| matches!(i.item, crate::content::Reference::Id(id) if id == dagger))
        );
        let links: Vec<_> = pulldown_cmark::Parser::new(&entry.body)
            .filter_map(|event| match event {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link { dest_url, .. }) => {
                    Some(dest_url.into_string())
                }
                _ => None,
            })
            .collect();
        assert!(
            links
                .iter()
                .any(|url| url.starts_with(crate::content::builtin::SOURCE_URL)
                    && url.contains("#page="))
        );
        assert!(
            !links.iter().any(|url| url.starts_with("Definitions/")),
            "Shared spell links must not point to nonexistent private files"
        );
        assert_eq!(entry.inventory.len(), 2);
        assert_ne!(entry.inventory[0].key, entry.inventory[1].key);
        let sheet: Sheet =
            serde_yaml_ng::from_value(entry.fields["splat_character"].clone()).unwrap();
        assert_eq!(
            sheet.spells.values().next().unwrap().entry.id,
            builtin().unwrap()["dnd2024:spells/fire_bolt"].id
        );
        assert_eq!(
            sheet.rules.values().next().unwrap().id,
            builtin().unwrap()[data["feature_key"].as_str().unwrap()].id
        );
        assert!(
            !crate::content::validate(std::slice::from_ref(&entry))
                .errors
                .is_empty()
        );
        assert!(
            crate::content::validate_with_builtin(std::slice::from_ref(&entry))
                .unwrap()
                .errors
                .is_empty()
        );
        // Two characters share the same rules, but keep distinct creature and item instances.
        record.source.character_id = "999".into();
        record.definition.splat_id = Uuid::new_v4();
        let other = record.entry().unwrap();
        let other: Sheet =
            serde_yaml_ng::from_value(other.fields["splat_character"].clone()).unwrap();
        assert_eq!(
            other.spells.values().next().unwrap().entry.id,
            sheet.spells.values().next().unwrap().entry.id
        );
        // A changed provider definition must fall back to its own unconverted local entry.
        record.character.spells[0]["definition"]["description"] = json!("Different behavior");
        let changed = record.content_notes().unwrap();
        assert_eq!(changed.len(), 2);
        assert!(
            changed
                .iter()
                .any(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap().kind == "spell")
        );
    }
    #[test]
    fn shared_backpack_supports_imported_contents() {
        let data: Value =
            serde_json::from_str(include_str!("rules/fixtures/shared-srd.json")).unwrap();
        let mut record = super::super::tests::sample();
        record.inventory[0].definition = data["backpack"].clone();
        record.inventory[0].definition_id = stable_id("backpack");
        record.inventory[0].name = "Backpack".into();
        record.inventory[0].quantity = 1;
        record.inventory[1].definition = data["item"].clone();
        record.inventory[1].name = "Dagger".into();
        record.inventory[1].holder_source_id = Some(record.inventory[0].source_id.clone());
        let notes = record.content_notes().unwrap();
        assert_eq!(notes.len(), 1);
        let entry = record.entry().unwrap();
        let backpack = &builtin().unwrap()["dnd2024:equipment/backpack_2_gp"];
        assert!(
            matches!(entry.inventory[0].item, crate::content::Reference::Id(id) if id == backpack.id)
        );
        assert_eq!(entry.inventory[1].container.as_deref(), Some("item_7"));
        assert_eq!(
            backpack.fields["splat_item"]["containment"]["capacity"]["mass"]["value"].as_i64(),
            Some(30)
        );
    }
    fn small() -> Value {
        json!({"activation":null,"definitionKey":"racial-trait:8429473","description":"<p>You are Small.</p>","snippet":"You are Small.","entityTypeId":1960452172_u64,"id":8429473,"name":"Size","sources":[{"pageNumber":20,"sourceId":85,"sourceType":1}]})
    }
    #[test]
    fn exact_source_closes_gap_but_same_id_with_changed_rules_does_not() {
        let mut record = super::super::tests::sample();
        record.character.features = vec![json!({"definition":small()})];
        let notes = record.content_notes().unwrap();
        let entry = Entry::parse("character.md", &notes.last().unwrap().markdown).unwrap();
        let key = feature_key(&small());
        let import: Import =
            serde_yaml_ng::from_value(entry.fields["splat_import"].clone()).unwrap();
        assert!(
            !import
                .gaps
                .iter()
                .any(|g| g.path == format!("splat_character/features/{key}"))
        );
        assert!(entry.fields["splat_character"]["rules"].get(&key).is_some());
        assert!(
            import
                .engine_gaps
                .iter()
                .any(|k| k == "dndbeyond:rules/small")
        );
        record.character.features[0]["definition"]["description"] = json!("<p>You are Huge.</p>");
        let entry = record.entry().unwrap();
        let import: Import =
            serde_yaml_ng::from_value(entry.fields["splat_import"].clone()).unwrap();
        assert!(
            import
                .gaps
                .iter()
                .any(|g| g.path == format!("splat_character/features/{key}"))
        );
        assert!(entry.fields["splat_character"]["rules"].get(&key).is_none());
    }
    #[test]
    fn editing_converted_mechanics_invalidates_coverage() {
        let mut record = super::super::tests::sample();
        record.character.features = vec![json!({"definition":small()})];
        let notes = record.content_notes().unwrap();
        let rule = notes
            .iter()
            .map(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap())
            .find(|e| e.kind == "rule")
            .unwrap();
        let mut fields = rule.fields;
        fields["splat_rule"]["parameters"]["size"] = serde_yaml_ng::Value::from("huge");
        let text = format!("---\n{}---\n", serde_yaml_ng::to_string(&fields).unwrap());
        assert!(
            Entry::parse("edited.md", &text)
                .unwrap_err()
                .contains("coverage")
        );
    }
    #[test]
    fn repeated_rules_share_identity_but_still_check_source_drift() {
        let record = super::super::tests::sample();
        let mut rules = Rules::new(&record);
        let first = rules
            .add_source("feature:8429473", &small())
            .unwrap()
            .unwrap();
        assert_eq!(
            rules.add_source("feature:8429473", &small()).unwrap(),
            Some(first)
        );
        assert_eq!(rules.notes.len(), 1);
        let mut changed = small();
        changed["snippet"] = json!("You are Huge.");
        assert!(
            rules
                .add_source("feature:8429473", &changed)
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn coverage_includes_character_casting_and_resource_rules() {
        let record = super::super::tests::sample();
        let entry = record.entry().unwrap();
        for key in ["splat_character", "splat_resources", "splat_stats"] {
            let mut fields = entry.fields.clone();
            let original = crate::content::character::rule_fingerprint(&fields).unwrap();
            fields.remove(key);
            assert_ne!(
                original,
                crate::content::character::rule_fingerprint(&fields).unwrap(),
                "{key}"
            );
        }
    }
    #[test]
    fn legacy_and_revised_tinderboxes_have_different_action_costs() {
        let legacy = &registry().sources["item:2103445194:90"];
        assert_eq!(
            legacy.parameters.as_ref().unwrap()["light_exposed_fuel"]["economy"],
            "action"
        );
        let revised = &registry().sources["item:2103445194:434"];
        assert_eq!(
            revised.builtin.as_deref(),
            Some("dnd2024:equipment/tinderbox_5_sp")
        );
        let fields = &builtin().unwrap()[revised.builtin.as_ref().unwrap()].fields;
        assert!(
            serde_yaml_ng::to_string(&fields["splat_rule"])
                .unwrap()
                .contains("bonus_action")
        );
    }
    #[test]
    fn class_spells_bind_supplied_class_totals_and_slot_pools() {
        let mut record = super::super::tests::sample();
        record.source.raw["classes"][0]["definition"]["spellCastingAbilityId"] = json!(4);
        let spell = json!({"id":22,"definition":{"id":33,"name":"Synthetic spell","level":1},"usesSpellSlot":true});
        record.source.raw["classSpells"] = json!([{"characterClassId":8,"spells":[spell]}]);
        record.character.spells = vec![spell];
        record.character.casting = json!({"saveDcs":[{"sources":["Wizard"],"value":19}],"spellAttacks":[{"sources":["Wizard"],"value":11}]});
        let mut sheet = Sheet::default();
        sheet.spells.insert(
            "spell_22".into(),
            crate::content::character::KnownSpell {
                entry: EntryRef { id: Uuid::new_v4() },
                name: "Synthetic spell".into(),
                prepared: true,
                always_prepared: false,
                uses_spell_slot: true,
                ability: None,
                save_dc: None,
                cast_at_level: None,
                restriction: String::new(),
                casting: None,
            },
        );
        let fields = json!({"splat_resources":record.definition.splat_resources});
        bind_spells(&record, &mut sheet, &fields);
        let result = &sheet.spells["spell_22"];
        assert_eq!(result.ability.as_deref(), Some("intelligence"));
        let binding = result.casting.as_ref().unwrap();
        assert_eq!(binding.save_dc, Some(19));
        assert_eq!(binding.attack_bonus, Some(11));
        assert_eq!(binding.slot_pools, vec!["spell_slots_1"]);
        assert!(!binding.at_will);

        // An innate grant uses its own ability, even when the actor also has a class total.
        record.source.raw["classSpells"] = json!([]);
        sheet.spells.get_mut("spell_22").unwrap().ability = Some("charisma".into());
        bind_spells(&record, &mut sheet, &fields);
        let innate = sheet.spells["spell_22"].casting.as_ref().unwrap();
        let expected =
            record.stat("charisma_modifier").unwrap() + record.stat("proficiency_bonus").unwrap();
        assert_eq!(innate.save_dc, Some(8 + expected));
        assert_eq!(innate.attack_bonus, Some(expected));
    }
}
