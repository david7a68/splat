---
splat_schema: 1
splat_id: d2158bed-a30b-5114-8de1-f38d011610c0
splat_kind: rule_group
splat_key: dnd2024:classes/monk
splat_name: Monk
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/monk
  implementation: dnd2024:classes_monk
  parameters:
    class: monk
    primary_abilities:
    - dexterity
    - wisdom
    hit_point_die:
      count_per_class_level: 1
      sides: 8
    first_class_traits:
      save_proficiencies:
      - strength
      - dexterity
      skill_proficiencies:
        count: 2
        choices:
        - acrobatics
        - athletics
        - history
        - insight
        - religion
        - stealth
      weapon_proficiencies:
        simple: true
        martial_required_property_any:
        - light
      armor_training: []
      tool_proficiency_choice:
        count: 1
        categories:
        - artisans_tools
        - musical_instrument
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/spear
            quantity: 1
          - definition:
              key: dnd2024:equipment/dagger
            quantity: 5
          - definition:
              key: dnd2024:equipment/explorer_s_pack_10_gp
            quantity: 1
          currency:
            gp: 11
          matching_proficient_tool: true
        currency:
          currency:
            gp: 50
    progression:
      key: dnd2024:tables/monk_progression
    feature_grants:
    - minimum_class_level: 10
      definition:
        key: dnd2024:class_features/monk_level_10_heightened_focus
    - minimum_class_level: 10
      definition:
        key: dnd2024:class_features/monk_level_10_self_restoration
    - minimum_class_level: 13
      definition:
        key: dnd2024:class_features/monk_level_13_deflect_energy
    - minimum_class_level: 14
      definition:
        key: dnd2024:class_features/monk_level_14_disciplined_survivor
    - minimum_class_level: 15
      definition:
        key: dnd2024:class_features/monk_level_15_perfect_focus
    - minimum_class_level: 18
      definition:
        key: dnd2024:class_features/monk_level_18_superior_defense
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/monk_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/monk_level_1_martial_arts
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/monk_level_1_unarmored_defense
    - minimum_class_level: 20
      definition:
        key: dnd2024:class_features/monk_level_20_body_and_mind
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/monk_level_2_monk_s_focus
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/monk_level_2_unarmored_movement
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/monk_level_2_uncanny_metabolism
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/monk_level_3_deflect_attacks
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/monk_level_3_monk_subclass
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/monk_level_4_ability_score_improvement
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/monk_level_4_slow_fall
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/monk_level_5_extra_attack
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/monk_level_5_stunning_strike
    - minimum_class_level: 6
      definition:
        key: dnd2024:class_features/monk_level_6_empowered_strikes
    - minimum_class_level: 7
      definition:
        key: dnd2024:class_features/monk_level_7_evasion
    - minimum_class_level: 9
      definition:
        key: dnd2024:class_features/monk_level_9_acrobatic_movement
splat_contracts:
- id: dnd2024:classes_monk
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
  - movement
  - equipment
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:classes_monk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8097132598197a53ea86d67bbd7d0f0e1307826ca2395c685529db7352714097
  line_span:
    first: 4088
    count: 22
splat_clauses:
- key: source_rules
  text: '# Monk Core Monk Traits Primary Ability Dexterity and Wisdom Hit Point Die D8 per Monk level
    Saving Throw Strength and Dexterity Proficiencies Skill Proficiencies Choose 2: Acrobatics, Athletics,
    History, Insight, Religion, or Stealth Weapon Proficiencies Simple weapons and Martial weapons that
    have the Light property Tool Proficiencies Choose one type of Artisan’s Tools or Musical Instrument
    (see “Equipment”) Armor Training None Starting Equipment Choose A or B: (A) Spear, 5 Daggers, Artisan’s
    Tools or Musical Instrument chosen for the tool proficiency above, Explorer’s Pack, and 11 GP; or
    (B) 50 GP'
  sha256: ccfb6a983198031f830c257b8251f67e82ca234870458d4184546603ec0b2cc2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Monk

<!-- SRD page 49 -->

Core Monk Traits\
Primary Ability Dexterity and Wisdom\
Hit Point Die D8 per Monk level\
Saving Throw Strength and Dexterity\
Proficiencies\
Skill Proficiencies Choose 2: Acrobatics, Athlet-\
ics, History, Insight, Religion,\
or Stealth\
Weapon Proficiencies Simple weapons and Martial\
weapons that have the Light\
property\
Tool Proficiencies Choose one type of Artisan’s\
Tools or Musical Instrument (see\
“Equipment”)\
Armor Training None\
Starting Equipment Choose A or B: (A) Spear, 5\
Daggers, Artisan’s Tools or\
Musical Instrument chosen\
for the tool proficiency above,\
Explorer’s Pack, and 11 GP; or\
(B) 50 GP\
