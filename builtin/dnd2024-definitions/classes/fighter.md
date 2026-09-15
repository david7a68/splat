---
splat_schema: 1
splat_id: 6d3c3b76-326b-5f07-8bd3-3802868b70ca
splat_kind: rule_group
splat_key: dnd2024:classes/fighter
splat_name: Fighter
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule_group:
  category: class
  members:
  - key: dnd2024:class_features/fighter_level_11_two_extra_attacks
  - key: dnd2024:class_features/fighter_level_13_studied_attacks
  - key: dnd2024:class_features/fighter_level_19_epic_boon
  - key: dnd2024:class_features/fighter_level_1_fighting_style
  - key: dnd2024:class_features/fighter_level_1_second_wind
  - key: dnd2024:class_features/fighter_level_1_weapon_mastery
  - key: dnd2024:class_features/fighter_level_20_three_extra_attacks
  - key: dnd2024:class_features/fighter_level_2_action_surge
  - key: dnd2024:class_features/fighter_level_2_tactical_mind
  - key: dnd2024:class_features/fighter_level_3_fighter_subclass
  - key: dnd2024:class_features/fighter_level_4_ability_score_improvement
  - key: dnd2024:class_features/fighter_level_5_extra_attack
  - key: dnd2024:class_features/fighter_level_5_tactical_shift
  - key: dnd2024:class_features/fighter_level_9_indomitable
  - key: dnd2024:class_features/fighter_level_9_tactical_master
splat_rule:
  mechanic: dnd2024:classes/fighter
  implementation: dnd2024:classes_fighter
  parameters:
    class: fighter
    primary_abilities:
    - strength
    - dexterity
    hit_point_die:
      sides: 10
      per_class_level: 1
    saving_throw_proficiencies:
    - strength
    - constitution
    skill_proficiencies:
      count: 2
      options:
      - acrobatics
      - animal_handling
      - athletics
      - history
      - insight
      - intimidation
      - persuasion
      - perception
      - survival
    weapon_proficiencies:
    - simple
    - martial
    armor_training:
    - light
    - medium
    - heavy
    - shield
    tool_proficiencies: []
    starting_equipment:
      count: 1
      options:
      - id: a
        grants:
        - item_type: chain_mail
          quantity: 1
        - item_type: greatsword
          quantity: 1
        - item_type: flail
          quantity: 1
        - item_type: javelin
          quantity: 8
        - item_type: dungeoneer_s_pack
          quantity: 1
        - item_type: gp
          quantity: 4
      - id: b
        grants:
        - item_type: studded_leather_armor
          quantity: 1
        - item_type: scimitar
          quantity: 1
        - item_type: shortsword
          quantity: 1
        - item_type: longbow
          quantity: 1
        - item_type: arrow
          quantity: 20
        - item_type: quiver
          quantity: 1
        - item_type: dungeoneer_s_pack
          quantity: 1
        - item_type: gp
          quantity: 11
      - id: c
        grants:
        - item_type: gp
          quantity: 155
    progression:
      key: dnd2024:tables/fighter_progression
    feature_grants:
    - feature:
        key: dnd2024:class_features/fighter_level_11_two_extra_attacks
      minimum_class_level: 11
    - feature:
        key: dnd2024:class_features/fighter_level_13_studied_attacks
      minimum_class_level: 13
    - feature:
        key: dnd2024:class_features/fighter_level_19_epic_boon
      minimum_class_level: 19
    - feature:
        key: dnd2024:class_features/fighter_level_1_fighting_style
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/fighter_level_1_second_wind
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/fighter_level_1_weapon_mastery
      minimum_class_level: 1
    - feature:
        key: dnd2024:class_features/fighter_level_20_three_extra_attacks
      minimum_class_level: 20
    - feature:
        key: dnd2024:class_features/fighter_level_2_action_surge
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/fighter_level_2_tactical_mind
      minimum_class_level: 2
    - feature:
        key: dnd2024:class_features/fighter_level_3_fighter_subclass
      minimum_class_level: 3
    - feature:
        key: dnd2024:class_features/fighter_level_4_ability_score_improvement
      minimum_class_level: 4
    - feature:
        key: dnd2024:class_features/fighter_level_5_extra_attack
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/fighter_level_5_tactical_shift
      minimum_class_level: 5
    - feature:
        key: dnd2024:class_features/fighter_level_9_indomitable
      minimum_class_level: 9
    - feature:
        key: dnd2024:class_features/fighter_level_9_tactical_master
      minimum_class_level: 9
splat_contracts:
- id: dnd2024:classes_fighter
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
  - 47
  engine_gaps: &id001
  - dnd2024:classes_fighter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 70289da21754e287b5607fe3a7096323a2b8076621f9d76483bca1861f75fbd5
  line_span:
    first: 3874
    count: 44
splat_clauses:
- key: source_rules
  text: '# Fighter Core Fighter Traits Primary Ability Strength or Dexterity Hit Point Die D10 per Fighter
    level Saving Throw Strength and Constitution Proficiencies Skill Proficiencies Choose 2: Acrobatics,
    Animal Handling, Athletics, History, Insight, Intimidation, Persuasion, Perception, or Survival Weapon
    Proficiencies Simple and Martial weapons Armor Training Light, Medium, and Heavy armor and Shields
    Starting Equipment Choose A, B, or C: (A) Chain Mail, Greatsword, Flail, 8 Javelins, Dungeoneer’s
    Pack, and 4 GP; (B) Studded Leather Armor, Scimitar, Short sword, Longbow, 20 Arrows, Quiver, Dungeoneer’s
    Pack, and 11 GP; or (C) 155 GP Fighter Features Proficiency Level Bonus Class Features 1 +2 Fighting
    Style, Second Wind, Weapon Mastery 2 +2 Action Surge (one use), Tactical Mind 3 +2 Fighter Subclass
    4 +2 Ability Score Improvement 5 +3 Extra Attack, Tactical Shift 6 +3 Ability Score Improvement 7
    +3 Subclass feature 8 +3 Ability Score Improvement 9 +4 Indomitable (one use), Tactical Master 10
    +4 Subclass feature 11 +4 Two Extra Attacks 12 +4 Ability Score Improvement 13 +5 Indomitable (two
    uses), Studied Attacks 14 +5 Ability Score Improvement 15 +5 Subclass feature 16 +5 Ability Score
    Improvement 17 +6 Action Surge (two uses), Indomitable (three uses) 18 +6 Subclass feature 19 +6 Epic
    Boon 20 +6 Three Extra Attacks'
  sha256: 0e19f92480717f45a42d356b50eef72c820b023f6a130274914156d991de3fb8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fighter

<!-- SRD page 47 -->

Core Fighter Traits\
Primary Ability Strength or Dexterity\
Hit Point Die D10 per Fighter level\
Saving Throw Strength and Constitution\
Proficiencies\
Skill Proficiencies Choose 2: Acrobatics, Animal\
Handling, Athletics, History,\
Insight, Intimidation, Persua-\
sion, Perception, or Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light, Medium, and Heavy ar-\
mor and Shields\
Starting Equipment Choose A, B, or C: (A) Chain\
Mail, Greatsword, Flail, 8 Jav-\
elins, Dungeoneer’s Pack, and\
4 GP; (B) Studded Leather\
Armor, Scimitar, Short sword,\
Longbow, 20 Arrows, Quiver,\
Dungeoneer’s Pack, and 11\
GP; or (C) 155 GP\
Fighter Features\
Proficiency\
Level Bonus Class Features\
1 +2 Fighting Style, Second Wind, Weapon Mastery\
2 +2 Action Surge (one use), Tactical Mind\
3 +2 Fighter Subclass\
4 +2 Ability Score Improvement\
5 +3 Extra Attack, Tactical Shift\
6 +3 Ability Score Improvement\
7 +3 Subclass feature\
8 +3 Ability Score Improvement\
9 +4 Indomitable (one use), Tactical Master\
10 +4 Subclass feature\
11 +4 Two Extra Attacks\
12 +4 Ability Score Improvement\
13 +5 Indomitable (two uses), Studied Attacks\
14 +5 Ability Score Improvement\
15 +5 Subclass feature\
16 +5 Ability Score Improvement\
17 +6 Action Surge (two uses), Indomitable (three uses)\
18 +6 Subclass feature\
19 +6 Epic Boon\
20 +6 Three Extra Attacks\
