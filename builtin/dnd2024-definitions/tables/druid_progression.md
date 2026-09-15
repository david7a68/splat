---
splat_schema: 1
splat_id: fa5779ce-0dbe-5665-87a9-7c04f37f04f0
splat_kind: rule
splat_key: dnd2024:tables/druid_progression
splat_name: Druid Progression
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/druid_progression
  parameters:
    class: druid
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Spellcasting, Druidic, Primal Order
      wild_shape: 0
      cantrips: 2
      prepared_spells: 4
      spell_slots_1: 2
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_1_druidic
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_1_primal_order
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Wild Shape, Wild Companion
      wild_shape: 2
      cantrips: 2
      prepared_spells: 5
      spell_slots_1: 3
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_2_wild_shape
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_2_wild_companion
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Druid Subclass
      wild_shape: 2
      cantrips: 2
      prepared_spells: 6
      spell_slots_1: 4
      spell_slots_2: 2
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/druid_level_3_druid_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      wild_shape: 2
      cantrips: 3
      prepared_spells: 7
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Wild Resurgence
      wild_shape: 2
      cantrips: 3
      prepared_spells: 9
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 2
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_5_wild_resurgence
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      wild_shape: 3
      cantrips: 3
      prepared_spells: 10
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 0
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: Elemental Fury
      wild_shape: 3
      cantrips: 3
      prepared_spells: 11
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 1
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_7_elemental_fury
        at_class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      wild_shape: 3
      cantrips: 3
      prepared_spells: 12
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 2
      spell_slots_5: 0
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: —
      wild_shape: 3
      cantrips: 3
      prepared_spells: 14
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 1
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants: []
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      wild_shape: 3
      cantrips: 4
      prepared_spells: 15
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 0
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: —
      wild_shape: 3
      cantrips: 4
      prepared_spells: 16
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants: []
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      wild_shape: 3
      cantrips: 4
      prepared_spells: 16
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 0
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: —
      wild_shape: 3
      cantrips: 4
      prepared_spells: 17
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 0
      spell_slots_9: 0
      grants: []
    - level: 14
      proficiency_bonus: 5
      features_text: Subclass feature
      wild_shape: 3
      cantrips: 4
      prepared_spells: 17
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 0
      spell_slots_9: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Improved Elemental Fury
      wild_shape: 3
      cantrips: 4
      prepared_spells: 18
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 1
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_15_improved_elemental_fury
        at_class_level: 15
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      wild_shape: 3
      cantrips: 4
      prepared_spells: 18
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 1
      spell_slots_9: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: —
      wild_shape: 4
      cantrips: 4
      prepared_spells: 19
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 1
      spell_slots_9: 1
      grants: []
    - level: 18
      proficiency_bonus: 6
      features_text: Beast Spells
      wild_shape: 4
      cantrips: 4
      prepared_spells: 20
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 3
      spell_slots_6: 1
      spell_slots_7: 1
      spell_slots_8: 1
      spell_slots_9: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_18_beast_spells
        at_class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      wild_shape: 4
      cantrips: 4
      prepared_spells: 21
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 3
      spell_slots_6: 2
      spell_slots_7: 1
      spell_slots_8: 1
      spell_slots_9: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Archdruid
      wild_shape: 4
      cantrips: 4
      prepared_spells: 22
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 3
      spell_slots_6: 2
      spell_slots_7: 2
      spell_slots_8: 1
      spell_slots_9: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/druid_level_20_archdruid
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/druid
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
      cantrips:
        kind: selection_capacity
        selection: cantrips
        eligibility: owning_feature_contract
      prepared_spells:
        kind: selection_capacity
        selection: prepared_spells
        eligibility: owning_feature_contract
      proficiency_bonus:
        kind: bonus
        applies_to: proficiency
        use_class_row_only_if_single_class: true
      spell_slots_1:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 1
        recovery: long_rest
      spell_slots_2:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 2
        recovery: long_rest
      spell_slots_3:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 3
        recovery: long_rest
      spell_slots_4:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 4
        recovery: long_rest
      spell_slots_5:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 5
        recovery: long_rest
      spell_slots_6:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 6
        recovery: long_rest
      spell_slots_7:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 7
        recovery: long_rest
      spell_slots_8:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 8
        recovery: long_rest
      spell_slots_9:
        kind: resource_capacity
        resource: spell_slot
        spell_level: 9
        recovery: long_rest
      wild_shape:
        kind: resource_capacity
        resource: wild_shape
        recovery: owning_feature_contract
        level_change_refills: false
  implementation: dnd2024:class_progression_druid_progression
splat_contracts:
- id: dnd2024:class_progression_druid_progression
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
  - 41
  engine_gaps: &id001
  - dnd2024:class_progression_druid_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 862adda3394cb2dd26fffc5c164a94aa542fecfc3763b450a798068077daf2cf
splat_clauses:
- key: source_rules
  text: '# Druid Progression From SRD 5.2.1 page 41. Druid Features Proficiency Wild Prepared ——Spell
    Slots per Spell Level—— Level Bonus Class Features Shape Cantrips Spells 1 2 3 4 5 6 7 8 9 1 +2 Spellcasting,
    Druidic, — 2 4 2 — — — — — — — — Primal Order 2 +2 Wild Shape, 2 2 5 3 — — — — — — — — Wild Companion
    3 +2 Druid Subclass 2 2 6 4 2 — — — — — — — 4 +2 Ability Score Improvement 2 3 7 4 3 — — — — — — —
    5 +3 Wild Resurgence 2 3 9 4 3 2 — — — — — — 6 +3 Subclass feature 3 3 10 4 3 3 — — — — — — 7 +3 Elemental
    Fury 3 3 11 4 3 3 1 — — — — — 8 +3 Ability Score Improvement 3 3 12 4 3 3 2 — — — — — 9 +4 — 3 3 14
    4 3 3 3 1 — — — — 10 +4 Subclass feature 3 4 15 4 3 3 3 2 — — — — 11 +4 — 3 4 16 4 3 3 3 2 1 — — —
    12 +4 Ability Score Improvement 3 4 16 4 3 3 3 2 1 — — — 13 +5 — 3 4 17 4 3 3 3 2 1 1 — — 14 +5 Subclass
    feature 3 4 17 4 3 3 3 2 1 1 — — 15 +5 Improved Elemental Fury 3 4 18 4 3 3 3 2 1 1 1 — 16 +5 Ability
    Score Improvement 3 4 18 4 3 3 3 2 1 1 1 — 17 +6 — 4 4 19 4 3 3 3 2 1 1 1 1 18 +6 Beast Spells 4 4
    20 4 3 3 3 3 1 1 1 1 19 +6 Epic Boon 4 4 21 4 3 3 3 3 2 1 1 1 20 +6 Archdruid 4 4 22 4 3 3 3 3 2 2
    1 1 41 System Reference Document 5.2.1'
  sha256: 62ac40c4b51c06053a8aa162bbf0f248f019d40d6e6f07ad4abadddc05616123
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druid Progression

From SRD 5.2.1 page 41.

     Druid Features
             Proficiency                                   Wild                Prepared   ——Spell Slots per Spell Level——
     Level     Bonus     Class Features                    Shape    Cantrips    Spells    1   2   3   4   5    6   7   8       9
       1         +2         Spellcasting, Druidic,           —         2          4       2   — — — — — — — —
                            Primal Order
       2         +2         Wild Shape,                       2        2          5       3   — — — — — — — —
                            Wild Companion
       3         +2         Druid Subclass                    2        2          6       4   2   — — — — — — —
       4         +2         Ability Score Improvement         2        3          7       4   3   — — — — — — —
       5         +3         Wild Resurgence                   2        3          9       4   3   2   — — — — — —
       6         +3         Subclass feature                  3        3          10      4   3   3   — — — — — —
       7         +3         Elemental Fury                    3        3          11      4   3   3   1   — — — — —
       8         +3         Ability Score Improvement         3        3          12      4   3   3   2   — — — — —
       9         +4         —                                 3        3          14      4   3   3   3   1   — — — —
       10        +4         Subclass feature                  3        4          15      4   3   3   3   2   — — — —
       11        +4         —                                 3        4          16      4   3   3   3   2    1   — — —
       12        +4         Ability Score Improvement         3        4          16      4   3   3   3   2    1   — — —
       13        +5         —                                 3        4          17      4   3   3   3   2    1   1   — —
       14        +5         Subclass feature                  3        4          17      4   3   3   3   2    1   1   — —
       15        +5         Improved Elemental Fury           3        4          18      4   3   3   3   2    1   1   1   —
       16        +5         Ability Score Improvement         3        4          18      4   3   3   3   2    1   1   1   —
       17        +6         —                                 4        4          19      4   3   3   3   2    1   1   1       1
       18        +6         Beast Spells                      4        4          20      4   3   3   3   3    1   1   1       1
       19        +6         Epic Boon                         4        4          21      4   3   3   3   3    2   1   1       1
       20        +6         Archdruid                         4        4          22      4   3   3   3   3    2   2   1       1



41    System Reference Document 5.2.1
