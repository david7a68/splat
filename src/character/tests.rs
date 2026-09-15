use super::*;
fn sample() -> EntitySnapshot {
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
    assert!(text.contains("otherNotes"));
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
fn campaign_copy_does_not_overwrite_unmanaged_notes() {
    let dir = tempfile::tempdir().unwrap();
    let record = sample();
    let path = record.campaign_path(dir.path());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "My creature note").unwrap();
    assert!(record.save_campaign_copy(dir.path()).is_err());
    assert_eq!(std::fs::read_to_string(path).unwrap(), "My creature note");
}
