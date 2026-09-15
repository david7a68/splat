---
splat_schema: 1
splat_id: aed521b4-3fb7-58f7-a35e-bc8ab5848eef
splat_kind: rule_group
splat_key: dnd2024:classes/ranger
splat_name: Ranger
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
splat_rule:
  mechanic: dnd2024:classes/ranger
  implementation: dnd2024:classes_ranger
  parameters:
    class: ranger
    primary_abilities:
    - dexterity
    - wisdom
    hit_point_die:
      count_per_class_level: 1
      sides: 10
    first_class_traits:
      save_proficiencies:
      - strength
      - dexterity
      skill_proficiencies:
        count: 3
        choices:
        - animal_handling
        - athletics
        - insight
        - investigation
        - nature
        - perception
        - stealth
        - survival
      weapon_proficiencies:
        simple: true
        martial: true
      armor_training:
      - light
      - medium
      - shield
    starting_equipment:
      choose: 1
      options:
        equipment:
          items:
          - definition:
              key: dnd2024:equipment/studded_leather_armor
            quantity: 1
          - definition:
              key: dnd2024:equipment/scimitar
            quantity: 1
          - definition:
              key: dnd2024:equipment/shortsword
            quantity: 1
          - definition:
              key: dnd2024:equipment/longbow
            quantity: 1
          - definition:
              key: dnd2024:equipment/ammunition_varies
            quantity: 20
            variant: arrows
          - definition:
              key: dnd2024:equipment/quiver_1_gp
            quantity: 1
          - definition:
              key: dnd2024:equipment/druidic_focus_varies
            quantity: 1
            variant: sprig_of_mistletoe
          - definition:
              key: dnd2024:equipment/explorer_s_pack_10_gp
            quantity: 1
          currency:
            gp: 7
        currency:
          currency:
            gp: 150
    progression:
      key: dnd2024:tables/ranger_progression
    feature_grants:
    - minimum_class_level: 10
      definition:
        key: dnd2024:class_features/ranger_level_10_tireless
    - minimum_class_level: 13
      definition:
        key: dnd2024:class_features/ranger_level_13_relentless_hunter
    - minimum_class_level: 14
      definition:
        key: dnd2024:class_features/ranger_level_14_nature_s_veil
    - minimum_class_level: 17
      definition:
        key: dnd2024:class_features/ranger_level_17_precise_hunter
    - minimum_class_level: 18
      definition:
        key: dnd2024:class_features/ranger_level_18_feral_senses
    - minimum_class_level: 19
      definition:
        key: dnd2024:class_features/ranger_level_19_epic_boon
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/ranger_level_1_favored_enemy
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/ranger_level_1_spellcasting
    - minimum_class_level: 1
      definition:
        key: dnd2024:class_features/ranger_level_1_weapon_mastery
    - minimum_class_level: 20
      definition:
        key: dnd2024:class_features/ranger_level_20_foe_slayer
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/ranger_level_2_deft_explorer
    - minimum_class_level: 2
      definition:
        key: dnd2024:class_features/ranger_level_2_fighting_style
    - minimum_class_level: 3
      definition:
        key: dnd2024:class_features/ranger_level_3_ranger_subclass
    - minimum_class_level: 4
      definition:
        key: dnd2024:class_features/ranger_level_4_ability_score_improvement
    - minimum_class_level: 5
      definition:
        key: dnd2024:class_features/ranger_level_5_extra_attack
    - minimum_class_level: 6
      definition:
        key: dnd2024:class_features/ranger_level_6_roving
    - minimum_class_level: 9
      definition:
        key: dnd2024:class_features/ranger_level_9_expertise
splat_contracts:
- id: dnd2024:classes_ranger
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
  - spellcasting
  - proficiencies
  - d20_tests
  - equipment
  - senses
splat_conversion:
  status: specified
  category: class
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 57
  engine_gaps: &id001
  - dnd2024:classes_ranger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 1676b93e234b956e57ffb3a29d33c382090f3448b81a01a196304ac529f97344
  line_span:
    first: 4791
    count: 19
splat_clauses:
- key: source_rules
  text: '# Ranger Core Ranger Traits Primary Ability Dexterity and Wisdom Hit Point Die D10 per Ranger
    level Saving Throw Strength and Dexterity Proficiencies Skill Proficiencies Choose 3: Animal Handling,
    Athletics, Insight, Investigation, Nature, Perception, Stealth, or Survival Weapon Proficiencies Simple
    and Martial weapons Armor Training Light and Medium armor and Shields Starting Equipment Choose A
    or B: (A) Studded Leather Armor, Scimitar, Shortsword, Longbow, 20 Arrows, Quiver, Druidic Focus (sprig
    of mistletoe), Explorer’s Pack, and 7 GP; or (B) 150 GP'
  sha256: 89d9df57f46d801a9a20899a8a774a11d24702bc5ee2e1a928b54c6d66bcc28f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ranger

<!-- SRD page 57 -->

Core Ranger Traits\
Primary Ability Dexterity and Wisdom\
Hit Point Die D10 per Ranger level\
Saving Throw Strength and Dexterity\
Proficiencies\
Skill Proficiencies Choose 3: Animal Handling,\
Athletics, Insight, Investigation,\
Nature, Perception, Stealth, or\
Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Studded\
Leather Armor, Scimitar,\
Shortsword, Longbow, 20 Ar-\
rows, Quiver, Druidic Focus\
(sprig of mistletoe), Explorer’s\
Pack, and 7 GP; or (B) 150 GP\
