---
splat_schema: 1
splat_id: efbd366f-30e0-509f-b4e7-8ced200babb5
splat_kind: rule_group
splat_key: dnd2024:classes/druid
splat_name: Druid
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/druid_level_15_improved_elemental_fury
  - key: dnd2024:class_features/druid_level_18_beast_spells
  - key: dnd2024:class_features/druid_level_19_epic_boon
  - key: dnd2024:class_features/druid_level_1_druidic
  - key: dnd2024:class_features/druid_level_1_primal_order
  - key: dnd2024:class_features/druid_level_1_spellcasting
  - key: dnd2024:class_features/druid_level_20_archdruid
  - key: dnd2024:class_features/druid_level_2_wild_companion
  - key: dnd2024:class_features/druid_level_2_wild_shape
  - key: dnd2024:class_features/druid_level_3_druid_subclass
  - key: dnd2024:class_features/druid_level_4_ability_score_improvement
  - key: dnd2024:class_features/druid_level_5_wild_resurgence
  - key: dnd2024:class_features/druid_level_7_elemental_fury
splat_rule:
  mechanic: dnd2024:classes/druid
  implementation: dnd2024:classes_druid
  parameters:
    class: druid
    primary_abilities:
    - wisdom
    hit_point_die:
      sides: 8
      per_class_level: 1
    saving_throw_proficiencies:
    - intelligence
    - wisdom
    skill_proficiencies:
      count: 2
      options:
      - animal_handling
      - arcana
      - insight
      - medicine
      - nature
      - perception
      - religion
      - survival
    weapon_proficiencies:
    - simple
    armor_training:
    - light
    - shield
    tool_proficiencies:
    - herbalism_kit
    starting_equipment:
      count: 1
      options:
      - id: a
        grants:
        - item_type: leather_armor
          quantity: 1
        - item_type: shield
          quantity: 1
        - item_type: sickle
          quantity: 1
        - item_type: druidic_focus_quarterstaff
          quantity: 1
        - item_type: explorer_s_pack
          quantity: 1
        - item_type: herbalism_kit
          quantity: 1
        - item_type: gp
          quantity: 9
      - id: b
        grants:
        - item_type: gp
          quantity: 50
    progression:
      key: dnd2024:tables/druid_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/druid_level_15_improved_elemental_fury
      minimum_class_level: 15
    - feature:
        key: dnd2024:class_features/druid_level_18_beast_spells
      minimum_class_level: 18
    - feature:
        key: dnd2024:class_features/druid_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/druid_level_1_druidic
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/druid_level_1_primal_order
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/druid_level_1_spellcasting
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/druid_level_20_archdruid
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/druid_level_2_wild_companion
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/druid_level_2_wild_shape
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/druid_level_3_druid_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/druid_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/druid_level_5_wild_resurgence
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/druid_level_7_elemental_fury
      minimum_class_level: 7
splat_contracts:
- id: dnd2024:classes_druid
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - proficiencies
  - inventory_changes
  events:
  - character.class_selected
  - character.level_gained
  capabilities:
  - character_creation
  - class_progression
  - inventory
  - proficiencies
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 41
  engine_gaps: &id001
  - dnd2024:classes_druid
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c8475a4ac4ac91ef50294f082a93143a08d7a35b81b9eca6514b21ca45ae70a8
  line_span:
    first: 3340
    count: 43
splat_clauses:
- key: source_rules
  text: '# Druid Core Druid Traits Primary Ability Wisdom Hit Point Die D8 per Druid level Saving Throw
    Intelligence and Wisdom Proficiencies Skill Proficiencies Choose 2: Animal Handling, Arcana, Insight,
    Medicine, Nature, Perception, Religion, or Survival Weapon Proficiencies Simple weapons Tool Proficiencies
    Herbalism Kit Armor Training Light armor and Shields Starting Equipment Choose A or B: (A) Leather
    Armor, Shield, Sickle, Druidic Focus (Quarterstaff), Explorer’s Pack, Herbalism Kit, and 9 GP; or
    (B) 50 GP Druid Features Proficiency Wild Level Bonus Class Features Shape 1 +2 Spellcasting, Druidic,
    — Primal Order 2 +2 Wild Shape, 2 Wild Companion 3 +2 Druid Subclass 2 4 +2 Ability Score Improvement
    2 5 +3 Wild Resurgence 2 6 +3 Subclass feature 3 7 +3 Elemental Fury 3 8 +3 Ability Score Improvement
    3 9 +4 — 3 10 +4 Subclass feature 3 11 +4 — 3 12 +4 Ability Score Improvement 3 13 +5 — 3 14 +5 Subclass
    feature 3 15 +5 Improved Elemental Fury 3 16 +5 Ability Score Improvement 3 17 +6 — 4 18 +6 Beast
    Spells 4 19 +6 Epic Boon 4 20 +6 Archdruid 4'
  sha256: 34a949e51def5e07a41ff050b7df8cabd2d16a266af7b3b95959606f3e7605b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druid

<!-- SRD page 41 -->

Core Druid Traits\
Primary Ability Wisdom\
Hit Point Die D8 per Druid level\
Saving Throw Intelligence and Wisdom\
Proficiencies\
Skill Proficiencies Choose 2: Animal Handling,\
Arcana, Insight, Medicine, Na-\
ture, Perception, Religion, or\
Survival\
Weapon Proficiencies Simple weapons\
Tool Proficiencies Herbalism Kit\
Armor Training Light armor and Shields\
Starting Equipment Choose A or B: (A) Leather Ar-\
mor, Shield, Sickle, Druidic Fo-\
cus (Quarterstaff), Explorer’s\
Pack, Herbalism Kit, and 9 GP;\
or (B) 50 GP\
Druid Features\
Proficiency Wild\
Level Bonus Class Features Shape\
1 +2 Spellcasting, Druidic, —\
Primal Order\
2 +2 Wild Shape, 2\
Wild Companion\
3 +2 Druid Subclass 2\
4 +2 Ability Score Improvement 2\
5 +3 Wild Resurgence 2\
6 +3 Subclass feature 3\
7 +3 Elemental Fury 3\
8 +3 Ability Score Improvement 3\
9 +4 — 3\
10 +4 Subclass feature 3\
11 +4 — 3\
12 +4 Ability Score Improvement 3\
13 +5 — 3\
14 +5 Subclass feature 3\
15 +5 Improved Elemental Fury 3\
16 +5 Ability Score Improvement 3\
17 +6 — 4\
18 +6 Beast Spells 4\
19 +6 Epic Boon 4\
20 +6 Archdruid 4\
