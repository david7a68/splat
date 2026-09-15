---
splat_schema: 1
splat_id: 507bda38-bdc0-529f-83be-3632e295489e
splat_kind: rule_group
splat_key: dnd2024:classes/rogue
splat_name: Rogue
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/rogue
  implementation: dnd2024:classes_rogue
  parameters:
    class: rogue
    primary_abilities:
    - dexterity
    hit_point_die:
      count_per_class_level: 1
      sides: 8
    first_class_traits:
      save_proficiencies:
      - dexterity
      - intelligence
      skill_proficiencies:
        count: 4
        choices:
        - acrobatics
        - athletics
        - deception
        - insight
        - intimidation
        - investigation
        - perception
        - persuasion
        - sleight_of_hand
        - stealth
      weapon_proficiencies:
        simple: true
        martial_required_property_any:
        - finesse
        - light
      armor_training:
      - light
      tool_proficiencies:
      - key: dnd2024:equipment/other_tools_thieves_tools_25_gp
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/leather_armor
            quantity: 1
          - definition:
              key: dnd2024:equipment/dagger
            quantity: 2
          - definition:
              key: dnd2024:equipment/shortsword
            quantity: 1
          - definition:
              key: dnd2024:equipment/shortbow
            quantity: 1
          - definition:
              key: dnd2024:equipment/ammunition_varies
            quantity: 20
            variant: arrows
          - definition:
              key: dnd2024:equipment/quiver_1_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/other_tools_thieves_tools_25_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/burglar_s_pack_16_gp
            quantity: 1
          currency:
            gp: 8
        currency:
          currency:
            gp: 100
    progression:
      key: dnd2024:tables/rogue_progression
    feature_grants:
    - minimum_class_level: 11
      definition:
        key: dnd2024:class_features/rogue_level_11_improved_cunning_strike
    - minimum_class_level: 14
      definition:
        key: dnd2024:class_features/rogue_level_14_devious_strikes
    - minimum_class_level: 15
      definition:
        key: dnd2024:class_features/rogue_level_15_slippery_mind
    - minimum_class_level: 18
      definition:
        key: dnd2024:class_features/rogue_level_18_elusive
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/rogue_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/rogue_level_1_expertise
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/rogue_level_1_sneak_attack
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/rogue_level_1_thieves_cant
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/rogue_level_1_weapon_mastery
    - minimum_class_level: 20
      definition:
        key: dnd2024:class_features/rogue_level_20_stroke_of_luck
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/rogue_level_2_cunning_action
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/rogue_level_3_rogue_subclass
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/rogue_level_3_steady_aim
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/rogue_level_4_ability_score_improvement
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/rogue_level_5_cunning_strike
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/rogue_level_5_uncanny_dodge
    - minimum_class_level: 7
      definition:
        key: dnd2024:class_features/rogue_level_7_evasion
    - minimum_class_level: 7
      definition:
        key: dnd2024:class_features/rogue_level_7_reliable_talent
splat_contracts:
- id: dnd2024:classes_rogue
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - class_traits
  - equipment_choices
  - progression
  - feature_grants
  events:
  - character.class_selected
  - character.level_gained
  capabilities:
  - character_features
  - proficiencies
  - d20_tests
  - equipment
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:classes_rogue
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9d24e73e4dcac02673b62e4290360d8dae4930a75c235fcd3e221fb7afd7ad14
  line_span:
    first: 5127
    count: 21
splat_clauses:
- key: source_rules
  text: '# Rogue Core Rogue Traits Primary Ability Dexterity Hit Point Die D8 per Rogue level Saving Throw
    Dexterity and Intelligence Proficiencies Skill Proficiencies Choose 4: Acrobatics, Athletics, Deception,
    Insight, Intimidation, Investigation, Perception, Persuasion, Sleight of Hand, or Stealth Weapon Proficiencies
    Simple weapons and Martial weapons that have the Finesse or Light property Tool Proficiencies Thieves’
    Tools Armor Training Light armor Starting Equipment Choose A or B: (A) Leather Armor, 2 Daggers, Short
    sword, Shortbow, 20 Arrows, Quiver, Thieves’ Tools, Burglar’s Pack, and 8 GP; or (B) 100 GP'
  sha256: f28bbde5a83ca4bde9b8384a0ac5605eeb9510866e977bafb8bb4fafae6ce994
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rogue

<!-- SRD page 61 -->

Core Rogue Traits\
Primary Ability Dexterity\
Hit Point Die D8 per Rogue level\
Saving Throw Dexterity and Intelligence\
Proficiencies\
Skill Proficiencies Choose 4: Acrobatics, Ath-\
letics, Deception, Insight,\
Intimidation, Investigation,\
Perception, Persuasion, Sleight\
of Hand, or Stealth\
Weapon Proficiencies Simple weapons and Martial\
weapons that have the Finesse\
or Light property\
Tool Proficiencies Thieves’ Tools\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather Ar-\
mor, 2 Daggers, Short sword,\
Shortbow, 20 Arrows, Quiver,\
Thieves’ Tools, Burglar’s Pack,\
and 8 GP; or (B) 100 GP\
