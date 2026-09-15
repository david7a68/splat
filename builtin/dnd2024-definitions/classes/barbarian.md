---
splat_schema: 1
splat_id: 87bf41fd-c017-5c9e-aeb8-4307c9489216
splat_kind: rule_group
splat_key: dnd2024:classes/barbarian
splat_name: Barbarian
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/barbarian_level_11_relentless_rage
  - key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
  - key: dnd2024:class_features/barbarian_level_15_persistent_rage
  - key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
  - key: dnd2024:class_features/barbarian_level_18_indomitable_might
  - key: dnd2024:class_features/barbarian_level_19_epic_boon
  - key: dnd2024:class_features/barbarian_level_1_rage
  - key: dnd2024:class_features/barbarian_level_1_unarmored_defense
  - key: dnd2024:class_features/barbarian_level_1_weapon_mastery
  - key: dnd2024:class_features/barbarian_level_20_primal_champion
  - key: dnd2024:class_features/barbarian_level_2_danger_sense
  - key: dnd2024:class_features/barbarian_level_2_reckless_attack
  - key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
  - key: dnd2024:class_features/barbarian_level_3_primal_knowledge
  - key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
  - key: dnd2024:class_features/barbarian_level_5_extra_attack
  - key: dnd2024:class_features/barbarian_level_5_fast_movement
  - key: dnd2024:class_features/barbarian_level_7_feral_instinct
  - key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
  - key: dnd2024:class_features/barbarian_level_9_brutal_strike
splat_rule:
  mechanic: dnd2024:classes/barbarian
  implementation: dnd2024:classes_barbarian
  parameters:
    class: barbarian
    primary_abilities:
    - strength
    hit_point_die:
      sides: 12
      per_class_level: 1
    saving_throw_proficiencies:
    - strength
    - constitution
    skill_proficiencies:
      count: 2
      options:
      - animal_handling
      - athletics
      - intimidation
      - nature
      - perception
      - survival
    weapon_proficiencies:
    - simple
    - martial
    armor_training:
    - light
    - medium
    - shield
    tool_proficiencies: []
    starting_equipment:
      count: 1
      options:
      - id: a
        grants:
        - item_type: greataxe
          quantity: 1
        - item_type: handaxe
          quantity: 4
        - item_type: explorer_s_pack
          quantity: 1
        - item_type: gp
          quantity: 15
      - id: b
        grants:
        - item_type: gp
          quantity: 75
    progression:
      key: dnd2024:tables/barbarian_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/barbarian_level_11_relentless_rage
      minimum_class_level: 11
    - feature:
        key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
      minimum_class_level: 13
    - feature:
        key: dnd2024:class_features/barbarian_level_15_persistent_rage
      minimum_class_level: 15
    - feature:
        key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
      minimum_class_level: 17
    - feature:
        key: dnd2024:class_features/barbarian_level_18_indomitable_might
      minimum_class_level: 18
    - feature:
        key: dnd2024:class_features/barbarian_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/barbarian_level_1_rage
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/barbarian_level_1_unarmored_defense
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/barbarian_level_1_weapon_mastery
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/barbarian_level_20_primal_champion
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/barbarian_level_2_danger_sense
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/barbarian_level_2_reckless_attack
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/barbarian_level_3_primal_knowledge
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/barbarian_level_5_extra_attack
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/barbarian_level_5_fast_movement
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/barbarian_level_7_feral_instinct
      minimum_class_level: 7
    - feature:
        key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
      minimum_class_level: 7
    - feature:
        key: dnd2024:class_features/barbarian_level_9_brutal_strike
      minimum_class_level: 9
splat_contracts:
- id: dnd2024:classes_barbarian
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
  - 28
  engine_gaps: &id001
  - dnd2024:classes_barbarian
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 10e5f3941bf31d22223e836c9aff7cc5f73c6b61c1cdee175390ab468bc71c8c
  line_span:
    first: 2207
    count: 15
splat_clauses:
- key: source_rules
  text: '# Barbarian Core Barbarian Traits Primary Ability Strength Hit Point Die D12 per Barbarian level
    Saving Throw Strength and Constitution Proficiencies Skill Proficiencies Choose 2: Animal Handling,
    Athletics, Intimidation, Nature, Perception, or Survival Weapon Proficiencies Simple and Martial weapons
    Armor Training Light and Medium armor and Shields Starting Equipment Choose A or B: (A) Greataxe,
    4 Handaxes, Explorer’s Pack, and 15 GP; or (B) 75 GP'
  sha256: fc2fc743d5c0646d953f7870a69560af410f2216a1cad799fde0b3e1574bc583
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barbarian

<!-- SRD page 28 -->

Core Barbarian Traits\
Primary Ability Strength\
Hit Point Die D12 per Barbarian level\
Saving Throw Strength and Constitution\
Proficiencies\
Skill Proficiencies Choose 2: Animal Handling,\
Athletics, Intimidation, Na-\
ture, Perception, or Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Greataxe,\
4 Handaxes, Explorer’s Pack,\
and 15 GP; or (B) 75 GP\
