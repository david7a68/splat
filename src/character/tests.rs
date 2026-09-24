use super::*;
pub(super) fn sample() -> EntitySnapshot {
    let abilities = ["str", "dex", "con", "int", "wis", "cha"]
        .iter()
        .map(|name| json!({"name":name,"score":14,"modifier":2,"save":4}))
        .collect::<Vec<_>>();
    let skills=["Athletics","Acrobatics","Sleight of Hand","Stealth","Arcana","History","Investigation","Nature","Religion","Animal Handling","Insight","Medicine","Perception","Survival","Deception","Intimidation","Performance","Persuasion"].iter().map(|name|json!({"name":name,"modifier":5,"isProficient":true,"isExpert":false,"isHalfProficient":false,"isAdvantage":true,"isDisadvantage":false})).collect::<Vec<_>>();
    EntitySnapshot::from_beyond(json!({"id":123,"name":"Example","campaign":{"id":456},"race":{"sizeId":3,"creatureTypeId":11},"classes":[{"id":8,"level":1,"hitDiceUsed":0,"definition":{"name":"Wizard","spellRules":{"levelSpellSlots":[[],[2]]}}}],"spellSlots":[{"level":1,"used":1}],"inventory":[{"id":7,"quantity":2,"equipped":true,"isAttuned":true,"definition":{"id":8,"entityTypeId":9,"name":"Wand"}},{"id":10,"quantity":1,"equipped":false,"definition":{"id":8,"entityTypeId":9,"name":"Wand"}}],"notes":{"otherNotes":"preserved"},"actions":{"race":[{"id":10,"name":"Ability uses","limitedUse":{"maxUses":1,"statModifierUsesId":3,"operator":1,"useProficiencyBonus":true,"proficiencyBonusOperator":1,"numberUsed":2}}]}}),json!({"characterId":123,"name":"Example","campaign":{"id":456},"level":1,"armorClass":19,"proficiencyBonus":2,"initiativeBonus":7,"passivePerception":15,"passiveInsight":12,"passiveInvestigation":13,"abilities":abilities,"skills":skills,"hitPointInfo":{"maximum":25,"current":17,"temp":4},"deathSaveInfo":{"successCount":0,"failCount":0},"speeds":[{"name":"Walking","distance":40}],"race":{"name":"Goblin"},"classes":[],"attacks":[{"name":"Wand","toHit":7}],"castingInfo":{"saveDcs":[{"value":16,"sources":["Wizard"]}]},"inspiration":true}),"456","123",10,11).unwrap()
}
#[test]
fn supplied_totals_are_not_recomputed_and_source_data_is_lossless() {
    let s = sample();
    assert_eq!(s.stat("armor_class"), Some(19));
    assert_eq!(s.stat("initiative"), Some(7));
    assert_eq!(s.stat("strength_save"), Some(4));
    assert_eq!(s.stat("skill_stealth"), Some(5));
    assert_eq!(s.stat("speed_walking"), Some(40));
    assert_eq!(s.initial.hp, 17);
    assert_eq!(s.initial.temporary_hp, 4);
    assert_eq!(s.initial.resources["spell_slots_1"], 1);
    assert_eq!(s.definition.splat_resources["feature_10"].max, 5);
    assert_eq!(s.initial.resources["feature_10"], 3);
    assert_eq!(s.source.raw["notes"]["otherNotes"], "preserved");
    assert!(s.character.skills[0].advantage);
}
#[test]
fn inventory_instances_and_creature_identity_remain_stable() {
    let a = sample();
    let mut raw = a.source.raw.clone();
    raw["name"] = json!("Renamed");
    let mut computed = a.source.computed.clone();
    computed["name"] = json!("Renamed");
    let b = EntitySnapshot::from_beyond(raw, computed, "456", "123", 12, 13).unwrap();
    assert_eq!(a.definition.splat_id, b.definition.splat_id);
    assert_eq!(a.inventory[0].id, b.inventory[0].id);
    assert_ne!(a.inventory[0].id, a.inventory[1].id);
    assert_eq!(a.inventory[0].definition_id, a.inventory[1].definition_id);
    assert_eq!(a.inventory[0].attuned_to, Some(a.definition.splat_id));
}
#[test]
fn entry_uses_creature_schema_without_claiming_execution() {
    let s = sample();
    let entry = s.entry().unwrap();
    assert_eq!(entry.kind, "creature");
    assert_eq!(entry.id, s.definition.splat_id);
    assert!(crate::execution::model::compile(&entry).is_err());
}
#[test]
fn cache_roundtrip_and_roster_scope_preserve_latest_records() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = Cache::at(tmp.path().into());
    let local = Uuid::new_v4();
    let original = sample();
    cache.save(local, &original).unwrap();
    cache.save_roster(local, "456", &["123".into()]).unwrap();
    let records = cache.load_all(local, "456").unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].source.content_hash, original.source.content_hash);
    assert_eq!(records[0].source.raw, original.source.raw);
    assert_eq!(records[0].source.computed, original.source.computed);
    assert_eq!(records[0].initial.hp, 17);
    assert!(cache.load_all(Uuid::new_v4(), "456").unwrap().is_empty());
    cache.save_roster(local, "456", &[]).unwrap();
    assert!(cache.load_all(local, "456").unwrap().is_empty());
    assert!(cache.path(local, "../secret", "123").is_err());
}
#[test]
fn failed_validation_does_not_replace_cache() {
    let tmp = tempfile::tempdir().unwrap();
    let cache = Cache::at(tmp.path().into());
    let local = Uuid::new_v4();
    let mut s = sample();
    let path = cache.save(local, &s).unwrap();
    let old = std::fs::read(&path).unwrap();
    s.initial.hp = 999;
    assert!(cache.save(local, &s).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), old);
    std::fs::write(&path, b"future schema").unwrap();
    assert!(cache.save(local, &sample()).is_err());
    assert_eq!(std::fs::read(path).unwrap(), b"future schema");
}
#[test]
fn missing_totals_or_wrong_membership_are_errors() {
    let s = sample();
    let mut c = s.source.computed.clone();
    c["armorClass"] = Value::Null;
    assert!(EntitySnapshot::from_beyond(s.source.raw.clone(), c, "456", "123", 10, 11).is_err());
    assert!(
        EntitySnapshot::from_beyond(
            s.source.raw.clone(),
            s.source.computed.clone(),
            "999",
            "123",
            10,
            11
        )
        .is_err()
    );
}

#[test]
fn campaign_copy_is_complete_parseable_and_keeps_personal_notes() {
    let dir = tempfile::tempdir().unwrap();
    let mut record = sample();
    let path = record.save_campaign_copy(dir.path()).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let entry = Entry::parse(path.to_str().unwrap(), &text).unwrap();
    assert_eq!(entry.id, record.definition.splat_id);
    assert_eq!(entry.kind, "creature");
    assert!(!text.contains("\"raw\""));
    assert!(text.contains("Wand"));
    std::fs::write(&path, format!("{text}\nMy own campaign notes.\n")).unwrap();
    record.definition.splat_name = "Renamed".into();
    record.initial.temporary_hp = 8;
    assert_eq!(record.save_campaign_copy(dir.path()).unwrap(), path);
    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(updated.contains("temporary HP: 8"));
    assert!(updated.ends_with("My own campaign notes.\n"));
    let edited = updated.replace("temporary HP: 8", "temporary HP: 9");
    std::fs::write(&path, &edited).unwrap();
    assert!(record.save_campaign_copy(dir.path()).is_err());
    assert_eq!(std::fs::read_to_string(path).unwrap(), edited);
}

#[test]
fn campaign_bundle_uses_content_inventory_and_normalized_sheet() {
    let record = sample();
    let dir = tempfile::tempdir().unwrap();
    record.save_campaign_copy(dir.path()).unwrap();
    let (entries, errors) = crate::content::read_directory(dir.path()).unwrap();
    assert!(errors.is_empty(), "{errors:?}");
    let report = crate::content::validate(&entries);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    let creature = entries.iter().find(|e| e.kind == "creature").unwrap();
    assert_eq!(creature.inventory.len(), 2);
    assert_ne!(creature.inventory[0].key, creature.inventory[1].key);
    assert!(creature.fields.contains_key("splat_character"));
    assert!(creature.fields.contains_key("splat_import"));
    assert_eq!(
        creature.fields["splat_initial"]["resources"]["spell_slots_1"].as_i64(),
        Some(1)
    );
    assert!(crate::execution::model::compile(creature).is_err());
}

#[test]
fn campaign_copy_does_not_overwrite_unmanaged_notes() {
    let dir = tempfile::tempdir().unwrap();
    let record = sample();
    let path = record.campaign_path(dir.path()).unwrap();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "My creature note").unwrap();
    assert!(record.save_campaign_copy(dir.path()).is_err());
    assert_eq!(std::fs::read_to_string(path).unwrap(), "My creature note");
}

#[test]
fn imported_attack_totals_and_duplicate_profiles_survive_refresh() {
    let mut record = sample();
    let attack = json!({"name":"Unarmed Strike","toHit":4,"damage":{"type":"Bludgeoning","value":{"diceString":"1d4","fixedValue":-1}},"range":{"label":"Reach","value":5},"saveInfo":null});
    record.character.attacks = vec![attack.clone(), attack];
    let first = record.entry().unwrap();
    let actions = first.fields["splat_actions"].as_mapping().unwrap();
    assert_eq!(actions.len(), 2);
    let keys = actions.keys().cloned().collect::<Vec<_>>();
    assert_eq!(
        actions[&keys[0]]["rolls"]["damage"]["expression"].as_str(),
        Some("1d4-1")
    );
    record.character.attacks[0]["toHit"] = json!(6);
    let refreshed = record.entry().unwrap();
    assert_eq!(
        refreshed.fields["splat_actions"]
            .as_mapping()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        keys
    );
    assert_eq!(
        refreshed.fields["splat_actions"][&keys[0]]["rolls"]["attack"]["modifier"].as_i64(),
        Some(6)
    );
}

#[test]
fn spells_are_referenced_content_without_free_attack_actions() {
    let mut record = sample();
    record.character.spells = vec![
        json!({"id":17,"prepared":true,"usesSpellSlot":true,"spellCastingAbilityId":4,"definition":{"id":18,"name":"Test spell","level":1,"school":"Evocation","description":"<p>Spell text.</p>","components":[1,2],"range":{"rangeValue":60,"origin":"Ranged"},"duration":{"durationType":"Instantaneous"}}}),
    ];
    record.character.attacks = vec![
        json!({"name":"Test spell","toHit":7,"damage":{"type":"Fire","value":5},"range":{"value":60,"origin":"Ranged"}}),
    ];
    let notes = record.content_notes().unwrap();
    let entries = notes
        .iter()
        .map(|n| Entry::parse(&n.relative_path, &n.markdown).unwrap())
        .collect::<Vec<_>>();
    let creature = entries.last().unwrap();
    assert!(
        creature.fields["splat_actions"]
            .as_mapping()
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        creature.fields["splat_character"]["attacks"]
            .as_mapping()
            .unwrap()
            .len(),
        1
    );
    let spell = creature.fields["splat_character"]["spells"]
        .as_mapping()
        .unwrap()
        .values()
        .next()
        .unwrap();
    assert_eq!(spell["prepared"].as_bool(), Some(true));
    assert_eq!(spell["uses_spell_slot"].as_bool(), Some(true));
    assert_eq!(spell["ability"].as_str(), Some("intelligence"));
    assert!(
        entries
            .iter()
            .any(|e| e.kind == "spell" && e.name == "Test spell")
    );
    assert!(crate::content::validate(&entries).errors.is_empty());
    let missing = entries
        .iter()
        .filter(|e| e.kind != "spell")
        .cloned()
        .collect::<Vec<_>>();
    assert!(!crate::content::validate(&missing).errors.is_empty());
}

#[test]
fn containers_are_structured_and_dependency_edits_block_the_whole_refresh() {
    let mut record = sample();
    record.inventory[0].quantity = 1;
    record.inventory[1].holder_source_id = Some(record.inventory[0].source_id.clone());
    let dir = tempfile::tempdir().unwrap();
    let creature_path = record.save_campaign_copy(dir.path()).unwrap();
    let before = std::fs::read_to_string(&creature_path).unwrap();
    let entry = Entry::parse("character.md", &before).unwrap();
    assert_eq!(entry.inventory[1].container.as_deref(), Some("item_7"));
    assert_eq!(entry.inventory[1].equipment, "stored");
    let dependency = record.content_notes().unwrap().remove(0);
    let path = dir
        .path()
        .join("Characters/D&D Beyond")
        .join(dependency.relative_path);
    std::fs::write(&path, "A local edit").unwrap();
    record.definition.splat_name = "Changed".into();
    assert!(record.save_campaign_copy(dir.path()).is_err());
    assert_eq!(std::fs::read_to_string(creature_path).unwrap(), before);
    assert_eq!(std::fs::read_to_string(path).unwrap(), "A local edit");
}

#[test]
fn malformed_character_extensions_are_rejected() {
    let entry = sample().entry().unwrap();
    let mut fields = entry.fields.clone();
    fields["splat_character"]["features"] =
        serde_yaml_ng::from_str("{bad: {name: Broken}}").unwrap();
    let text = format!("---\n{}---\n", serde_yaml_ng::to_string(&fields).unwrap());
    assert!(Entry::parse("bad.md", &text).is_err());
    let mut fields = entry.fields;
    fields["splat_import"]["version"] = serde_yaml_ng::Value::from(99);
    let text = format!("---\n{}---\n", serde_yaml_ng::to_string(&fields).unwrap());
    assert!(Entry::parse("bad.md", &text).is_err());
}

#[test]
fn custom_campaign_storage_survives_moving_the_folder() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("original");
    std::fs::create_dir(&root).unwrap();
    let local = Uuid::new_v4();
    std::fs::write(root.join("Campaign.md"), format!("---\nsplat_schema: 1\nsplat_id: {local}\nsplat_kind: campaign\nsplat_name: Test\nsplat_namespace: test_campaign\nsplat_storage:\n  characters: Party/Sheets\n  character_cache: Data/Cache\n---\n")).unwrap();
    let record = sample();
    let cache = Cache::for_campaign(&root).unwrap();
    cache.save(local, &record).unwrap();
    cache.save_roster(local, "456", &["123".into()]).unwrap();
    let path = record.save_campaign_copy(&root).unwrap();
    assert!(path.starts_with(root.join("Party/Sheets")));
    assert!(!root.join("Characters").exists());
    let moved = temp.path().join("moved");
    std::fs::rename(&root, &moved).unwrap();
    let restored = Cache::for_campaign(&moved)
        .unwrap()
        .load_all(local, "456")
        .unwrap();
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].definition.splat_id, record.definition.splat_id);
    assert!(record.campaign_path(&moved).unwrap().exists());
}
