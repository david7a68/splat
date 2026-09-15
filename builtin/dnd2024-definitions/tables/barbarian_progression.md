---
splat_schema: 1
splat_id: 947560e8-cc55-5932-85a3-4cb7a9603d9b
splat_kind: rule
splat_key: dnd2024:tables/barbarian_progression
splat_name: Barbarian Progression
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/barbarian_progression
  parameters:
    class: barbarian
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Rage, Unarmored Defense, Weapon Mastery
      rages: 2
      rage_damage: 2
      weapon_mastery: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_1_rage
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_1_unarmored_defense
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_1_weapon_mastery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Danger Sense, Reckless Attack
      rages: 2
      rage_damage: 2
      weapon_mastery: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_2_danger_sense
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_2_reckless_attack
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Barbarian Subclass, Primal Knowledge
      rages: 3
      rage_damage: 2
      weapon_mastery: 2
      grants:
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
        at_class_level: 3
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_3_primal_knowledge
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      rages: 3
      rage_damage: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Fast Movement
      rages: 3
      rage_damage: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_5_extra_attack
        at_class_level: 5
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_5_fast_movement
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
      grants:
      - operation: grant_selected_subclass_features
        class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: Feral Instinct, Instinctive Pounce
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_7_feral_instinct
        at_class_level: 7
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
        at_class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: Brutal Strike
      rages: 4
      rage_damage: 3
      weapon_mastery: 3
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_9_brutal_strike
        at_class_level: 9
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      rages: 4
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_selected_subclass_features
        class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: Relentless Rage
      rages: 4
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_11_relentless_rage
        at_class_level: 11
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: Improved Brutal Strike
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
        at_class_level: 13
    - level: 14
      proficiency_bonus: 5
      features_text: Subclass feature
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_selected_subclass_features
        class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Persistent Rage
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_15_persistent_rage
        at_class_level: 15
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      rages: 5
      rage_damage: 4
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Improved Brutal Strike
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
        at_class_level: 17
    - level: 18
      proficiency_bonus: 6
      features_text: Indomitable Might
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_18_indomitable_might
        at_class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Primal Champion
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/barbarian_level_20_primal_champion
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/barbarian
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
      rage_damage:
        kind: damage_bonus
        requires: rage_feature_eligibility
      rages:
        kind: resource_capacity
        resource: rages
        recovery: owning_feature_contract
        level_change_refills: false
      weapon_mastery:
        kind: selection_capacity
        selection: weapon_mastery
        eligibility: owning_feature_contract
  implementation: dnd2024:class_progression_barbarian_progression
splat_contracts:
- id: dnd2024:class_progression_barbarian_progression
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
  - 28
  engine_gaps: &id001
  - dnd2024:class_progression_barbarian_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f1c67836986e4a06470b4783985047639fe284fc265dd3ee70c8919de2de2ff1
splat_clauses:
- key: source_rules
  text: '# Barbarian Progression From SRD 5.2.1 page 28. Barbarian Features Proficiency Rage Weapon Level
    Bonus Class Features Rages Damage Mastery 1 +2 Rage, Unarmored Defense, Weapon Mastery 2 +2 2 2 +2
    Danger Sense, Reckless Attack 2 +2 2 3 +2 Barbarian Subclass, Primal Knowledge 3 +2 2 4 +2 Ability
    Score Improvement 3 +2 3 5 +3 Extra Attack, Fast Movement 3 +2 3 6 +3 Subclass feature 4 +2 3 7 +3
    Feral Instinct, Instinctive Pounce 4 +2 3 8 +3 Ability Score Improvement 4 +2 3 9 +4 Brutal Strike
    4 +3 3 10 +4 Subclass feature 4 +3 4 11 +4 Relentless Rage 4 +3 4 12 +4 Ability Score Improvement
    5 +3 4 13 +5 Improved Brutal Strike 5 +3 4 14 +5 Subclass feature 5 +3 4 15 +5 Persistent Rage 5 +3
    4 16 +5 Ability Score Improvement 5 +4 4 17 +6 Improved Brutal Strike 6 +4 4 18 +6 Indomitable Might
    6 +4 4 19 +6 Epic Boon 6 +4 4 20 +6 Primal Champion 6 +4 4 28 System Reference Document 5.2.1'
  sha256: e8971fa4d787440d771018abff19b2018903ec4be504788a54d8420bd1654145
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barbarian Progression

From SRD 5.2.1 page 28.

     Barbarian Features
                Proficiency                                                                          Rage       Weapon
       Level      Bonus        Class Features                                            Rages      Damage      Mastery
         1           +2        Rage, Unarmored Defense, Weapon Mastery                     2          +2           2
         2           +2        Danger Sense, Reckless Attack                               2          +2           2
         3           +2        Barbarian Subclass, Primal Knowledge                        3          +2           2
         4           +2        Ability Score Improvement                                   3          +2           3
         5           +3        Extra Attack, Fast Movement                                 3          +2           3
         6           +3        Subclass feature                                            4          +2           3
         7           +3        Feral Instinct, Instinctive Pounce                          4          +2           3
         8           +3        Ability Score Improvement                                   4          +2           3
         9           +4        Brutal Strike                                               4          +3           3
        10           +4        Subclass feature                                            4          +3           4
        11           +4        Relentless Rage                                             4          +3           4
        12           +4        Ability Score Improvement                                   5          +3           4
        13           +5        Improved Brutal Strike                                      5          +3           4
        14           +5        Subclass feature                                            5          +3           4
        15           +5        Persistent Rage                                             5          +3           4
        16           +5        Ability Score Improvement                                   5          +4           4
        17           +6        Improved Brutal Strike                                      6          +4           4
        18           +6        Indomitable Might                                           6          +4           4
        19           +6        Epic Boon                                                   6          +4           4
        20           +6        Primal Champion                                             6          +4           4



28   System Reference Document 5.2.1
