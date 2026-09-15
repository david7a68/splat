---
splat_schema: 1
splat_id: bdda2fb9-a1ef-5e7b-a9bf-b25a82d74042
splat_kind: rule_group
splat_key: dnd2024:classes/cleric
splat_name: Cleric
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/cleric_level_10_divine_intervention
  - key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
  - key: dnd2024:class_features/cleric_level_19_epic_boon
  - key: dnd2024:class_features/cleric_level_1_divine_order
  - key: dnd2024:class_features/cleric_level_1_spellcasting
  - key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
  - key: dnd2024:class_features/cleric_level_2_channel_divinity
  - key: dnd2024:class_features/cleric_level_3_cleric_subclass
  - key: dnd2024:class_features/cleric_level_4_ability_score_improvement
  - key: dnd2024:class_features/cleric_level_5_sear_undead
  - key: dnd2024:class_features/cleric_level_7_blessed_strikes
splat_rule:
  mechanic: dnd2024:classes/cleric
  implementation: dnd2024:classes_cleric
  parameters:
    class: cleric
    primary_abilities:
    - wisdom
    hit_point_die:
      sides: 8
      per_class_level: 1
    saving_throw_proficiencies:
    - wisdom
    - charisma
    skill_proficiencies:
      count: 2
      options:
      - history
      - insight
      - medicine
      - persuasion
      - religion
    weapon_proficiencies:
    - simple
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
        - item_type: chain_shirt
          quantity: 1
        - item_type: shield
          quantity: 1
        - item_type: mace
          quantity: 1
        - item_type: holy_symbol
          quantity: 1
        - item_type: priest_s_pack
          quantity: 1
        - item_type: gp
          quantity: 7
      - id: b
        grants:
        - item_type: gp
          quantity: 110
    progression:
      key: dnd2024:tables/cleric_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/cleric_level_10_divine_intervention
      minimum_class_level: 10
    - feature:
        key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
      minimum_class_level: 14
    - feature:
        key: dnd2024:class_features/cleric_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/cleric_level_1_divine_order
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/cleric_level_1_spellcasting
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/cleric_level_2_channel_divinity
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/cleric_level_3_cleric_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/cleric_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/cleric_level_5_sear_undead
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/cleric_level_7_blessed_strikes
      minimum_class_level: 7
splat_contracts:
- id: dnd2024:classes_cleric
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
  - 36
  engine_gaps: &id001
  - dnd2024:classes_cleric
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 1312a06a6bdc4e7a53fe2f7df4da25015a2fd6ccabbcc2fc4e9075c49fff5a6e
  line_span:
    first: 2913
    count: 16
splat_clauses:
- key: source_rules
  text: '# Cleric Core Cleric Traits Primary Ability Wisdom Hit Point Die D8 per Cleric level Saving Throw
    Wisdom and Charisma Proficiencies Skill Proficiencies Choose 2: History, Insight, Medicine, Persuasion,
    or Religion Weapon Proficiencies Simple weapons Armor Training Light and Medium armor and Shields
    Starting Equipment Choose A or B: (A) Chain Shirt, Shield, Mace, Holy Symbol, Priest’s Pack, and 7
    GP; or (B) 110 GP'
  sha256: 7670e17aeaedc25b7b5b2843f415c93476729fe776145e24d95bcaff1c42577c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cleric

<!-- SRD page 36 -->

Core Cleric Traits\
Primary Ability Wisdom\
Hit Point Die D8 per Cleric level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: History, Insight,\
Medicine, Persuasion, or\
Religion\
Weapon Proficiencies Simple weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Chain Shirt,\
Shield, Mace, Holy Symbol,\
Priest’s Pack, and 7 GP; or (B)\
110 GP\
