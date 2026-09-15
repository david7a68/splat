---
splat_schema: 1
splat_id: 99562e22-b8dc-5d70-be60-dbc025fba62f
splat_kind: rule_group
splat_key: dnd2024:classes/bard
splat_name: Bard
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/bard_level_10_magical_secrets
  - key: dnd2024:class_features/bard_level_18_superior_inspiration
  - key: dnd2024:class_features/bard_level_19_epic_boon
  - key: dnd2024:class_features/bard_level_1_bardic_inspiration
  - key: dnd2024:class_features/bard_level_1_spellcasting
  - key: dnd2024:class_features/bard_level_20_words_of_creation
  - key: dnd2024:class_features/bard_level_2_expertise
  - key: dnd2024:class_features/bard_level_2_jack_of_all_trades
  - key: dnd2024:class_features/bard_level_3_bard_subclass
  - key: dnd2024:class_features/bard_level_4_ability_score_improvement
  - key: dnd2024:class_features/bard_level_5_font_of_inspiration
  - key: dnd2024:class_features/bard_level_7_countercharm
splat_rule:
  mechanic: dnd2024:classes/bard
  implementation: dnd2024:classes_bard
  parameters:
    class: bard
    primary_abilities:
    - charisma
    hit_point_die:
      sides: 8
      per_class_level: 1
    saving_throw_proficiencies:
    - dexterity
    - charisma
    skill_proficiencies:
      count: 3
      options: any_skill
    weapon_proficiencies:
    - simple
    armor_training:
    - light
    tool_proficiencies:
    - category: musical_instrument
      count: 3
    starting_equipment:
      count: 1
      options:
      - id: a
        grants:
        - item_type: leather_armor
          quantity: 1
        - item_type: dagger
          quantity: 2
        - item_type: musical_instrument_choice
          quantity: 1
        - item_type: entertainer_s_pack
          quantity: 1
        - item_type: gp
          quantity: 19
      - id: b
        grants:
        - item_type: gp
          quantity: 90
    progression:
      key: dnd2024:tables/bard_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/bard_level_10_magical_secrets
      minimum_class_level: 10
    - feature:
        key: dnd2024:class_features/bard_level_18_superior_inspiration
      minimum_class_level: 18
    - feature:
        key: dnd2024:class_features/bard_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/bard_level_1_bardic_inspiration
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/bard_level_1_spellcasting
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/bard_level_20_words_of_creation
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/bard_level_2_expertise
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/bard_level_2_jack_of_all_trades
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/bard_level_3_bard_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/bard_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/bard_level_5_font_of_inspiration
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/bard_level_7_countercharm
      minimum_class_level: 7
splat_contracts:
- id: dnd2024:classes_bard
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
  - 31
  engine_gaps: &id001
  - dnd2024:classes_bard
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 5d78891af25cbc0cb2f2d0cdb7bf527f88cc3f1fca2d0cd3e10fb19cfbbfa3c9
  line_span:
    first: 2492
    count: 17
splat_clauses:
- key: source_rules
  text: '# Bard Core Bard Traits Primary Ability Charisma Hit Point Die D8 per Bard level Saving Throw
    Dexterity and Charisma Proficiencies Skill Proficiencies Choose any 3 skills (see “Playing the Game”)
    Weapon Proficiencies Simple weapons Tool Proficiencies Choose 3 Musical Instruments (see “Equipment”)
    Armor Training Light armor Starting Equipment Choose A or B: (A) Leather Armor, 2 Daggers, Musical
    Instrument of your choice, Entertainer’s Pack, and 19 GP; or (B) 90 GP'
  sha256: c77d14336bcdd5af6dee8c1f930d3e8e9448c766e50006d7b574a867855f9250
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bard

<!-- SRD page 31 -->

Core Bard Traits\
Primary Ability Charisma\
Hit Point Die D8 per Bard level\
Saving Throw Dexterity and Charisma\
Proficiencies\
Skill Proficiencies Choose any 3 skills (see “Playing\
the Game”)\
Weapon Proficiencies Simple weapons\
Tool Proficiencies Choose 3 Musical Instruments\
(see “Equipment”)\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather\
Armor, 2 Daggers, Musical\
Instrument of your choice, En-\
tertainer’s Pack, and 19 GP; or\
(B) 90 GP\
