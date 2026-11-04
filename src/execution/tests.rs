use super::*;
fn engine(faces: Vec<u32>) -> Engine {
    Engine::new(&builtin::entries().unwrap(), Random::controlled(faces)).unwrap()
}
fn request(actor: &str, definition: &str, action: &str, targets: &[&str]) -> Use {
    let mut r = Use::new(actor, definition, action, targets);
    for t in targets {
        r.facts.insert((*t).into(), TargetFacts::clear());
    }
    r
}
#[test]
fn official_guard_attack_commits_even_when_it_misses() {
    let mut e = engine(vec![1]);
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.start_turn("guard").unwrap();
    let r = request("guard", "dnd2024:creatures/guard", "spear", &["skeleton"]);
    e.use_action(r.clone()).unwrap();
    assert_eq!(e.actors["skeleton"].hp, 13);
    assert!(!e.actors["guard"].action_available);
    assert!(e.use_action(r).is_err());
}
#[test]
fn critical_rolls_extra_dice_without_doubling_flat_damage() {
    let mut e = engine(vec![20, 2, 3]);
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.start_turn("guard").unwrap();
    e.use_action(request(
        "guard",
        "dnd2024:creatures/guard",
        "spear",
        &["skeleton"],
    ))
    .unwrap();
    assert_eq!(e.actors["skeleton"].hp, 7);
}
#[test]
fn multiattack_has_independent_rolls_and_one_action_cost() {
    let mut e = engine(vec![12, 1, 14, 2]);
    e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.start_turn("bear").unwrap();
    e.use_action(request(
        "bear",
        "dnd2024:creatures/black_bear",
        "multiattack",
        &["skeleton"],
    ))
    .unwrap();
    assert_eq!(e.actors["skeleton"].hp, 6);
    assert_eq!(e.events.iter().filter(|e| e.kind == "commit").count(), 1);
    assert_eq!(e.events.iter().filter(|e| e.kind == "attack").count(), 2);
}
fn caster(e: &mut Engine) {
    // This is an explicitly authored test grant, not a claim that an SRD Guard casts spells.
    let mut d = e.definitions["dnd2024:creatures/guard"].clone();
    d.pools.insert(
        "slots".into(),
        Pool {
            name: "Level 2 Slots".into(),
            max: 2,
            initial: None,
            kind: Some("spell_slot".into()),
            level: Some(2),
        },
    );
    e.definitions.insert("proof:caster".into(), d);
    e.add_actor("caster", "proof:caster").unwrap();
    e.actors.get_mut("caster").unwrap().spells.extend([
        "dnd2024:spells/acid_splash".into(),
        "dnd2024:spells/cure_wounds".into(),
        "dnd2024:spells/inflict_wounds".into(),
        "dnd2024:spells/healing_word".into(),
    ]);
    e.actors.get_mut("caster").unwrap().spellcasting = Some(Spellcasting {
        actor_level: 1,
        modifier: 3,
        save_dc: 13,
    });
    e.start_turn("caster").unwrap();
}
#[test]
fn acid_splash_uses_one_damage_roll_and_independent_saves() {
    let mut e = engine(vec![2, 5, 18]);
    caster(&mut e);
    e.add_actor("a", "dnd2024:creatures/skeleton").unwrap();
    e.add_actor("b", "dnd2024:creatures/skeleton").unwrap();
    let mut r = request("caster", "dnd2024:spells/acid_splash", "cast", &["a", "b"]);
    r.bindings = Map::from([
        ("cantrip_tier".into(), 0.into()),
        ("save_dc".into(), 13.into()),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    r.areas.insert(
        "acid_splash_sphere".into(),
        AreaFacts {
            members: vec!["a".into(), "b".into()],
            origin_in_range: Some(true),
            origin_clear_path: Some(true),
        },
    );
    e.use_action(r).unwrap();
    assert_eq!(e.actors["a"].hp, 8);
    assert_eq!(e.actors["b"].hp, 13);
    assert_eq!(e.events.iter().filter(|e| e.kind == "roll").count(), 1);
}
#[test]
fn revised_cure_wounds_scales_and_caps_healing() {
    let mut e = engine(vec![2, 2, 2, 2]);
    caster(&mut e);
    e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
    e.actors.get_mut("bear").unwrap().hp = 5;
    let mut r = request("caster", "dnd2024:spells/cure_wounds", "cast", &["bear"]);
    r.bindings = Map::from([
        ("cast_level".into(), 2.into()),
        ("slot_pool".into(), Value::String("slots".into())),
        ("spellcasting_modifier".into(), 3.into()),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["bear"].hp, 16);
    assert_eq!(e.actors["caster"].pools["slots"], 1);
}
#[test]
fn potion_consumes_only_the_selected_instance_as_a_bonus_action() {
    let mut e = engine(vec![2, 3]);
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.give_item("guard", "a", "dnd2024:items/potion_of_healing", 2)
        .unwrap();
    e.give_item("guard", "b", "dnd2024:items/potion_of_healing", 2)
        .unwrap();
    e.actors.get_mut("guard").unwrap().hp = 2;
    e.start_turn("guard").unwrap();
    let mut r = request(
        "guard",
        "dnd2024:items/potion_of_healing",
        "drink",
        &["guard"],
    );
    r.source_item = Some("a".into());
    r.free_hands = Some(1);
    r.item_accessible = Some(true);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["guard"].hp, 9);
    assert_eq!(e.actors["guard"].items["a"].quantity, 1);
    assert_eq!(e.actors["guard"].items["b"].quantity, 2);
    assert!(e.actors["guard"].action_available);
    assert!(!e.actors["guard"].bonus_action_available);
}
#[test]
fn unresolved_facts_do_not_spend_resources_or_randomness() {
    let mut e = engine(vec![20, 2, 3]);
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.start_turn("guard").unwrap();
    let mut bad = request("guard", "dnd2024:creatures/guard", "spear", &["skeleton"]);
    bad.facts.clear();
    let before = serde_json::to_value(&e.events).unwrap();
    assert!(e.use_action(bad).is_err());
    assert_eq!(serde_json::to_value(&e.events).unwrap(), before);
    assert!(e.actors["guard"].action_available);
    e.use_action(request(
        "guard",
        "dnd2024:creatures/guard",
        "spear",
        &["skeleton"],
    ))
    .unwrap();
    assert_eq!(e.actors["skeleton"].hp, 7);
}
#[test]
fn fmt_f2_reader_identity_and_table_conversion_preserve_strict_execution() {
    let mut entry = builtin::entries()
        .unwrap()
        .into_iter()
        .find(|e| e.name == "Guard")
        .unwrap();
    let before = compile(&entry).unwrap();
    let text = format!(
        "---\n{}---\n{}",
        serde_yaml_ng::to_string(&*entry.fields).unwrap(),
        entry.body
    );
    let converted = crate::content::formats::convert("Guard.md", &text).unwrap();
    let converted = crate::content::Entry::parse("Guard.md", &converted).unwrap();
    let after = compile(&converted).unwrap();
    assert_eq!(
        serde_json::to_value(before.stats).unwrap(),
        serde_json::to_value(after.stats).unwrap()
    );
    assert_eq!(
        serde_json::to_value(before.creature).unwrap(),
        serde_json::to_value(after.creature).unwrap()
    );
    entry.fields.get_mut("splat_creature").unwrap()["unknown_mechanic"] = true.into();
    assert!(
        compile(&entry)
            .unwrap_err()
            .contains("unknown field `unknown_mechanic`")
    );
}

#[test]
fn compiler_rejects_unknown_mechanics_instead_of_ignoring_them() {
    let mut entry = builtin::entries()
        .unwrap()
        .into_iter()
        .find(|e| e.name == "Guard")
        .unwrap();
    entry
        .fields
        .insert("splat_effects".into(), Value::Mapping(Default::default()));
    assert!(
        compile(&entry)
            .unwrap_err()
            .contains("unsupported executable field")
    );
}

#[test]
fn long_range_gives_disadvantage_and_advantage_cancels_it() {
    let mut e = engine(vec![18, 2]);
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.start_turn("skeleton").unwrap();
    let mut r = request(
        "skeleton",
        "dnd2024:creatures/skeleton",
        "shortbow",
        &["guard"],
    );
    r.facts.get_mut("guard").unwrap().range = Some(RangeBand::Long);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["guard"].hp, 11);
    let mut e = engine(vec![18, 2]);
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.start_turn("skeleton").unwrap();
    let mut r = request(
        "skeleton",
        "dnd2024:creatures/skeleton",
        "shortbow",
        &["guard"],
    );
    let facts = r.facts.get_mut("guard").unwrap();
    facts.range = Some(RangeBand::Long);
    facts.attack_advantage = Some(true);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["guard"].hp, 6);
}
#[test]
fn one_spell_slot_limit_applies_across_action_and_bonus_action() {
    let mut e = engine(vec![1, 1, 1, 1]);
    caster(&mut e);
    e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
    e.actors.get_mut("bear").unwrap().hp = 1;
    let mut r = request("caster", "dnd2024:spells/cure_wounds", "cast", &["bear"]);
    r.bindings = Map::from([
        ("cast_level".into(), 2.into()),
        ("slot_pool".into(), Value::String("slots".into())),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    e.use_action(r.clone()).unwrap();
    r.definition = "dnd2024:spells/healing_word".into();
    let before = e.events.clone();
    assert!(e.use_action(r).unwrap_err().contains("one spell slot"));
    assert!(e.actors["caster"].bonus_action_available);
    assert_eq!(e.actors["caster"].pools["slots"], 1);
    assert_eq!(e.events, before);
}
#[test]
fn save_success_halves_and_rounds_down_without_natural_one_auto_failure() {
    let mut e = engine(vec![20, 3, 4, 4]);
    caster(&mut e);
    e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
    let mut r = request("caster", "dnd2024:spells/inflict_wounds", "cast", &["bear"]);
    r.bindings = Map::from([
        ("cast_level".into(), 2.into()),
        ("slot_pool".into(), Value::String("slots".into())),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["bear"].hp, 14);
}
#[test]
fn incorrect_scaling_binding_is_rejected() {
    let mut e = engine(vec![]);
    caster(&mut e);
    let mut r = request("caster", "dnd2024:spells/acid_splash", "cast", &[]);
    r.bindings.insert("cantrip_tier".into(), 3.into());
    assert!(e.use_action(r).unwrap_err().contains("conflicts"));
}
#[test]
fn duplicate_nested_step_and_unknown_nested_field_are_rejected() {
    let entry = builtin::entries()
        .unwrap()
        .into_iter()
        .find(|e| e.name == "Black Bear")
        .unwrap();
    let text = serde_yaml_ng::to_string(&entry.fields).unwrap();
    let bad = text.replace("key: attack", "key: multiattack");
    let bad = crate::content::Entry::parse("bad", &format!("---\n{bad}---\n")).unwrap();
    assert!(compile(&bad).is_err());
    let bad = text.replace(
        "operation: dnd2024:attack",
        "operation: dnd2024:attack\n        unknown_rule: true",
    );
    assert!(
        crate::content::Entry::parse("bad", &format!("---\n{bad}---\n"))
            .and_then(|e| compile(&e))
            .is_err()
    );
}
#[test]
fn temporary_hp_absorbs_damage_and_skeleton_vulnerability_doubles_bludgeoning() {
    let mut e = engine(vec![12, 2]);
    let d = e.definitions.get_mut("dnd2024:creatures/guard").unwrap();
    d.actions.get_mut("spear").unwrap().steps[0]
        .on_hit
        .as_mut()
        .unwrap()
        .damage[0]
        .kind = "bludgeoning".into();
    e.add_actor("guard", "dnd2024:creatures/guard").unwrap();
    e.add_actor("skeleton", "dnd2024:creatures/skeleton")
        .unwrap();
    e.actors.get_mut("skeleton").unwrap().temporary_hp = 4;
    e.start_turn("guard").unwrap();
    e.use_action(request(
        "guard",
        "dnd2024:creatures/guard",
        "spear",
        &["skeleton"],
    ))
    .unwrap();
    assert_eq!(e.actors["skeleton"].hp, 11);
    assert_eq!(e.actors["skeleton"].temporary_hp, 0);
}
#[test]
fn negative_spellcasting_modifier_and_healing_cap_are_applied() {
    let mut e = engine(vec![8, 8, 8, 8]);
    caster(&mut e);
    e.actors
        .get_mut("caster")
        .unwrap()
        .spellcasting
        .as_mut()
        .unwrap()
        .modifier = -1;
    e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
    e.actors.get_mut("bear").unwrap().hp = 2;
    let mut r = request("caster", "dnd2024:spells/cure_wounds", "cast", &["bear"]);
    r.bindings = Map::from([
        ("cast_level".into(), 2.into()),
        ("slot_pool".into(), Value::String("slots".into())),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    e.use_action(r).unwrap();
    assert_eq!(e.actors["bear"].hp, 19);
}
#[test]
fn all_required_mechanics_and_numeric_parameter_types_are_checked() {
    let entry = builtin::entries()
        .unwrap()
        .into_iter()
        .find(|e| e.name == "Guard")
        .unwrap();
    let mut bad = entry.clone();
    bad.fields.insert(
        "splat_requires_mechanics".into(),
        serde_yaml_ng::from_str("[dnd2024:unimplemented]").unwrap(),
    );
    assert!(
        compile(&bad)
            .unwrap_err()
            .contains("Unsupported required mechanic")
    );
    let bad = serde_yaml_ng::to_string(&entry.fields)
        .unwrap()
        .replace("modifier: 3", "modifier: {binding: pool}")
        + "splat_parameters: {pool: {type: actor_resource, required: true}}\n";
    let bad = crate::content::Entry::parse("bad", &format!("---\n{bad}---\n")).unwrap();
    assert!(compile(&bad).is_err());
}
#[test]
fn natural_one_can_succeed_on_a_save_and_natural_twenty_can_fail() {
    for (face, bonus, success) in [(1, 20, true), (20, -10, false)] {
        let mut e = engine(vec![face, 2, 2, 2]);
        caster(&mut e);
        e.definitions
            .get_mut("dnd2024:creatures/black_bear")
            .unwrap()
            .stats
            .insert(
                "save_constitution".into(),
                Stat {
                    value: bonus,
                    unit: None,
                    includes: vec![],
                },
            );
        e.add_actor("bear", "dnd2024:creatures/black_bear").unwrap();
        let mut r = request("caster", "dnd2024:spells/inflict_wounds", "cast", &["bear"]);
        r.bindings = Map::from([
            ("cast_level".into(), 2.into()),
            ("slot_pool".into(), Value::String("slots".into())),
        ]);
        r.can_speak = Some(true);
        r.free_hands = Some(1);
        e.use_action(r).unwrap();
        assert_eq!(
            e.events.iter().find(|e| e.kind == "save").unwrap().detail["success"],
            success
        );
    }
}
#[test]
fn area_range_is_measured_to_the_origin_and_members_cannot_be_omitted() {
    let mut e = engine(vec![2, 5]);
    caster(&mut e);
    e.add_actor("a", "dnd2024:creatures/skeleton").unwrap();
    let mut r = request("caster", "dnd2024:spells/acid_splash", "cast", &["a"]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    r.facts.get_mut("a").unwrap().range = Some(RangeBand::OutOfRange);
    r.areas.insert(
        "acid_splash_sphere".into(),
        AreaFacts {
            members: vec!["a".into(), "caster".into()],
            origin_in_range: Some(true),
            origin_clear_path: Some(true),
        },
    );
    assert!(
        e.use_action(r.clone())
            .unwrap_err()
            .contains("every affected creature")
    );
    r.areas.get_mut("acid_splash_sphere").unwrap().members = vec!["a".into()];
    e.use_action(r).unwrap();
    assert_eq!(e.actors["a"].hp, 8);
}
#[test]
fn every_embedded_profile_compiles_and_every_creature_action_runs() {
    let entries = builtin::entries().unwrap();
    let disk_count = crate::content::read_directory(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("builtin/dnd2024-mechanics"),
    )
    .unwrap()
    .0
    .len();
    assert_eq!(
        entries.len(),
        disk_count,
        "New files must be included by incremental builds"
    );
    assert_eq!(entries.len(), 32);
    for entry in entries.iter().filter(|e| e.kind == "creature") {
        let definition = compile(entry).unwrap();
        for action in definition.actions.keys() {
            let mut engine = Engine::new(&entries, Random::seeded(7)).unwrap();
            engine
                .add_actor("actor", entry.key.as_deref().unwrap())
                .unwrap();
            engine
                .add_actor("target", "dnd2024:creatures/awakened_tree")
                .unwrap();
            engine.start_turn("actor").unwrap();
            engine
                .use_action(request(
                    "actor",
                    entry.key.as_deref().unwrap(),
                    action,
                    &["target"],
                ))
                .unwrap_or_else(|error| panic!("{} / {action}: {error}", entry.name));
            assert!(!engine.actors["actor"].action_available);
        }
    }
}

#[test]
fn supplied_save_is_used_instead_of_ability_calculation() {
    let dragon = crate::content::definitions::entries()
        .unwrap()
        .iter()
        .find(|entry| entry.key.as_deref() == Some("dnd2024:creatures/young_white_dragon"))
        .unwrap();
    let stats: Map<Stat> =
        serde_yaml_ng::from_value(dragon.fields[Value::String("splat_stats".into())].clone())
            .unwrap();
    assert_eq!(stats["save_intelligence"].value, -2);
    assert_eq!(stats["wisdom"].value, 11);
    assert_eq!(stats["save_wisdom"].value, 3);

    // Exercise only the existing save operation, not the unimplemented dragon
    // actions. The test grant changes Inflict Wounds to a Wisdom save.
    let mut e = engine(vec![10, 2, 2, 2]);
    caster(&mut e);
    e.definitions
        .get_mut("dnd2024:creatures/black_bear")
        .unwrap()
        .stats
        .extend(stats);
    e.definitions
        .get_mut("dnd2024:spells/inflict_wounds")
        .unwrap()
        .actions
        .get_mut("cast")
        .unwrap()
        .steps[0]
        .ability = Some("wisdom".into());
    e.add_actor("target", "dnd2024:creatures/black_bear")
        .unwrap();
    let mut r = request(
        "caster",
        "dnd2024:spells/inflict_wounds",
        "cast",
        &["target"],
    );
    r.bindings = Map::from([
        ("cast_level".into(), 2.into()),
        ("slot_pool".into(), Value::String("slots".into())),
    ]);
    r.can_speak = Some(true);
    r.free_hands = Some(1);
    e.use_action(r).unwrap();
    let save = &e
        .events
        .iter()
        .find(|event| event.kind == "save")
        .unwrap()
        .detail;
    assert_eq!(save["natural"], 10);
    assert_eq!(save["total"], 13);
}
