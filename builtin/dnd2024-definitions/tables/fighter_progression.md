---
splat_schema: 1
splat_id: a8941e4b-da5a-5490-aed3-a271872d1611
splat_kind: rule
splat_key: dnd2024:tables/fighter_progression
splat_name: Fighter Progression
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/fighter_progression
  parameters:
    class: fighter
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Fighting Style, Second Wind, Weapon Mastery
      second_wind: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_1_fighting_style
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_1_second_wind
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_1_weapon_mastery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Action Surge (one use), Tactical Mind
      second_wind: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_2_action_surge
        at_class_level: 2
        maximum_uses: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_2_tactical_mind
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Fighter Subclass
      second_wind: 2
      weapon_mastery: 3
      grants:
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/fighter_level_3_fighter_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Tactical Shift
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_5_extra_attack
        at_class_level: 5
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_5_tactical_shift
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: Subclass feature
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_selected_subclass_features
        class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: Indomitable (one use), Tactical Master
      second_wind: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_9_indomitable
        at_class_level: 9
        maximum_uses: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_9_tactical_master
        at_class_level: 9
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_selected_subclass_features
        class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: Two Extra Attacks
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_11_two_extra_attacks
        at_class_level: 11
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: Indomitable (two uses), Studied Attacks
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_9_indomitable
        at_class_level: 13
        maximum_uses: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_13_studied_attacks
        at_class_level: 13
    - level: 14
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 5
      grants:
      - operation: grant_selected_subclass_features
        class_level: 15
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 6
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Action Surge (two uses), Indomitable (three uses)
      second_wind: 4
      weapon_mastery: 6
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_2_action_surge
        at_class_level: 17
        maximum_uses: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_9_indomitable
        at_class_level: 17
        maximum_uses: 3
    - level: 18
      proficiency_bonus: 6
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 6
      grants:
      - operation: grant_selected_subclass_features
        class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      second_wind: 4
      weapon_mastery: 6
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Three Extra Attacks
      second_wind: 4
      weapon_mastery: 6
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/fighter_level_20_three_extra_attacks
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/fighter
    lookup:
      input: class_level
      minimum: 1
      maximum: 20
      match: exact
      numeric_columns: current_capacity_or_value
      feature_grants: all_crossed_levels
      repeat_application: idempotent_per_class_level_and_feature
    multiclass:
      proficiency_bonus_source: total_character_level
      spell_slot_override_rule:
        key: dnd2024:rules/multiclassing_spellcasting
      pact_magic_pool_separate: true
    column_semantics:
      proficiency_bonus:
        kind: bonus
        applies_to: proficiency
        use_class_row_only_if_single_class: true
      second_wind:
        kind: resource_capacity
        resource: second_wind
        recovery: owning_feature_contract
        level_change_refills: false
      weapon_mastery:
        kind: selection_capacity
        selection: weapon_mastery
        eligibility: owning_feature_contract
  implementation: dnd2024:class_progression_fighter_progression
splat_contracts:
- id: dnd2024:class_progression_fighter_progression
  inputs:
    actor:
      type: participant
      required: true
    class_level:
      type: integer
      required: true
    previous_class_level:
      required: false
      type: integer
    selected_subclass:
      required: false
      type: definition
    choices:
      required: false
      type: definition_list
  outputs:
  - feature_grants
  - selection_capacities
  - resource_capacities
  - class_values
  events:
  - character.class_level_changed
  capabilities:
  - class_progression
  - feature_grants
  - resource_capacities
  - character_choices
splat_conversion:
  status: specified
  category: table
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 47
  engine_gaps: &id001
  - dnd2024:class_progression_fighter_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 854606242259ecd464f98697dc02151d17f4264b7025a9f1e976c6b4ae63e3ca
splat_clauses:
- key: source_rules
  text: '# Fighter Progression From SRD 5.2.1 page 47. Fighter Features Proficiency Second Weapon Level
    Bonus Class Features Wind Mastery 1 +2 Fighting Style, Second Wind, Weapon Mastery 2 3 2 +2 Action
    Surge (one use), Tactical Mind 2 3 3 +2 Fighter Subclass 2 3 4 +2 Ability Score Improvement 3 4 5
    +3 Extra Attack, Tactical Shift 3 4 6 +3 Ability Score Improvement 3 4 7 +3 Subclass feature 3 4 8
    +3 Ability Score Improvement 3 4 9 +4 Indomitable (one use), Tactical Master 3 4 10 +4 Subclass feature
    4 5 11 +4 Two Extra Attacks 4 5 12 +4 Ability Score Improvement 4 5 13 +5 Indomitable (two uses),
    Studied Attacks 4 5 14 +5 Ability Score Improvement 4 5 15 +5 Subclass feature 4 5 16 +5 Ability Score
    Improvement 4 6 17 +6 Action Surge (two uses), Indomitable (three uses) 4 6 18 +6 Subclass feature
    4 6 19 +6 Epic Boon 4 6 20 +6 Three Extra Attacks 4 6 47 System Reference Document 5.2.1'
  sha256: 765e1c0fc46470ae3fec75614bcfb46ed868eaf0fb81ea27282797840e06bb1d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fighter Progression

From SRD 5.2.1 page 47.

     Fighter Features
                  Proficiency                                                                     Second        Weapon
       Level        Bonus          Class Features                                                  Wind         Mastery
         1             +2         Fighting Style, Second Wind, Weapon Mastery                        2              3
         2             +2         Action Surge (one use), Tactical Mind                              2              3
         3             +2         Fighter Subclass                                                   2              3
         4             +2         Ability Score Improvement                                          3              4
         5             +3         Extra Attack, Tactical Shift                                       3              4
         6             +3         Ability Score Improvement                                          3              4
         7             +3         Subclass feature                                                   3              4
         8             +3         Ability Score Improvement                                          3              4
         9             +4         Indomitable (one use), Tactical Master                             3              4
         10            +4         Subclass feature                                                   4              5
         11            +4         Two Extra Attacks                                                  4              5
         12            +4         Ability Score Improvement                                          4              5
         13            +5         Indomitable (two uses), Studied Attacks                            4              5
         14            +5         Ability Score Improvement                                          4              5
         15            +5         Subclass feature                                                   4              5
         16            +5         Ability Score Improvement                                          4              6
         17            +6         Action Surge (two uses), Indomitable (three uses)                  4              6
         18            +6         Subclass feature                                                   4              6
         19            +6         Epic Boon                                                          4              6
         20            +6         Three Extra Attacks                                                4              6



47   System Reference Document 5.2.1
