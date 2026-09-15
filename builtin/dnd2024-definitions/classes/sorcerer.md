---
splat_schema: 1
splat_id: 09ee2ce0-6490-5787-a99e-ab16a9386f75
splat_kind: rule_group
splat_key: dnd2024:classes/sorcerer
splat_name: Sorcerer
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/sorcerer_level_19_epic_boon
  - key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
  - key: dnd2024:class_features/sorcerer_level_1_spellcasting
  - key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
  - key: dnd2024:class_features/sorcerer_level_2_font_of_magic
  - key: dnd2024:class_features/sorcerer_level_2_metamagic
  - key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
  - key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
  - key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
  - key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
splat_rule:
  mechanic: dnd2024:classes/sorcerer
  implementation: dnd2024:classes_sorcerer
  parameters:
    class: sorcerer
    primary_abilities:
    - charisma
    hit_point_die:
      sides: 6
      per_class_level: 1
    saving_throw_proficiencies:
    - constitution
    - charisma
    skill_proficiencies:
      count: 2
      options:
      - arcana
      - deception
      - insight
      - intimidation
      - persuasion
      - religion
    weapon_proficiencies:
    - simple
    armor_training: []
    tool_proficiencies: []
    starting_equipment:
      count: 1
      options:
      - id: a
        grants:
        - item_type: spear
          quantity: 1
        - item_type: dagger
          quantity: 2
        - item_type: arcane_focus_crystal
          quantity: 1
        - item_type: dungeoneer_s_pack
          quantity: 1
        - item_type: gp
          quantity: 28
      - id: b
        grants:
        - item_type: gp
          quantity: 50
    progression:
      key: dnd2024:tables/sorcerer_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/sorcerer_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/sorcerer_level_1_spellcasting
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/sorcerer_level_2_font_of_magic
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/sorcerer_level_2_metamagic
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
      minimum_class_level: 7
splat_contracts:
- id: dnd2024:classes_sorcerer
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
  - 64
  engine_gaps: &id001
  - dnd2024:classes_sorcerer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d83af8f55137870dea1b4d0d6698f0dd0df11453788e0b92675d3ac2bd4da05a
  line_span:
    first: 5402
    count: 15
splat_clauses:
- key: source_rules
  text: '# Sorcerer Core Sorcerer Traits Primary Ability Charisma Hit Point Die D6 per Sorcerer level
    Saving Throw Constitution and Charisma Proficiencies Skill Proficiencies Choose 2: Arcana, Deception,
    Insight, Intimidation, Persuasion, or Religion Weapon Proficiencies Simple weapons Armor Training
    None Starting Equipment Choose A or B: (A) Spear, 2 Daggers, Arcane Focus (crystal), Dungeoneer’s
    Pack, and 28 GP; or (B) 50 GP'
  sha256: 4ae300cb67a070d55383aa832efcfe556f73d6a8779b84e503189c1a6e828dfc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sorcerer

<!-- SRD page 64 -->

Core Sorcerer Traits\
Primary Ability Charisma\
Hit Point Die D6 per Sorcerer level\
Saving Throw Constitution and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Arcana, Deception,\
Insight, Intimidation, Persua-\
sion, or Religion\
Weapon Proficiencies Simple weapons\
Armor Training None\
Starting Equipment Choose A or B: (A) Spear,\
2 Daggers, Arcane Focus\
(crystal), Dungeoneer’s Pack,\
and 28 GP; or (B) 50 GP\
