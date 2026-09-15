---
splat_schema: 1
splat_id: 4c6a3cd1-4c23-5774-9f46-77be07f5e3b3
splat_kind: rule
splat_key: dnd2024:tables/cleric_progression
splat_name: Cleric Progression
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/cleric_progression
  parameters:
    class: cleric
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Spellcasting, Divine Order
      channel_divinity: 0
      cantrips: 3
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
          key: dnd2024:class_features/cleric_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/cleric_level_1_divine_order
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Channel Divinity
      channel_divinity: 2
      cantrips: 3
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
          key: dnd2024:class_features/cleric_level_2_channel_divinity
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Cleric Subclass
      channel_divinity: 2
      cantrips: 3
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
          key: dnd2024:class_features/cleric_level_3_cleric_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      channel_divinity: 2
      cantrips: 4
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
          key: dnd2024:class_features/cleric_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Sear Undead
      channel_divinity: 2
      cantrips: 4
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
          key: dnd2024:class_features/cleric_level_5_sear_undead
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      channel_divinity: 3
      cantrips: 4
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
      features_text: Blessed Strikes
      channel_divinity: 3
      cantrips: 4
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
          key: dnd2024:class_features/cleric_level_7_blessed_strikes
        at_class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      channel_divinity: 3
      cantrips: 4
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
          key: dnd2024:class_features/cleric_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: —
      channel_divinity: 3
      cantrips: 4
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
      features_text: Divine Intervention
      channel_divinity: 3
      cantrips: 5
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
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/cleric_level_10_divine_intervention
        at_class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: —
      channel_divinity: 3
      cantrips: 5
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
      channel_divinity: 3
      cantrips: 5
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
          key: dnd2024:class_features/cleric_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: —
      channel_divinity: 3
      cantrips: 5
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
      features_text: Improved Blessed Strikes
      channel_divinity: 3
      cantrips: 5
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
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
        at_class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: —
      channel_divinity: 3
      cantrips: 5
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
      grants: []
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      channel_divinity: 3
      cantrips: 5
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
          key: dnd2024:class_features/cleric_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Subclass feature
      channel_divinity: 3
      cantrips: 5
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
      grants:
      - operation: grant_selected_subclass_features
        class_level: 17
    - level: 18
      proficiency_bonus: 6
      features_text: —
      channel_divinity: 4
      cantrips: 5
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
      grants: []
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      channel_divinity: 4
      cantrips: 5
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
          key: dnd2024:class_features/cleric_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Greater Divine Intervention
      channel_divinity: 4
      cantrips: 5
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
          key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/cleric
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
      channel_divinity:
        kind: resource_capacity
        resource: channel_divinity
        recovery: owning_feature_contract
        level_change_refills: false
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
  implementation: dnd2024:class_progression_cleric_progression
splat_contracts:
- id: dnd2024:class_progression_cleric_progression
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
  - 36
  engine_gaps: &id001
  - dnd2024:class_progression_cleric_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 156c069f92879c32638651d69172f6000a474622b767eb1db54c1f8bdd1e647c
splat_clauses:
- key: source_rules
  text: '# Cleric Progression From SRD 5.2.1 page 36. Cleric Features Proficiency Channel Prepared ——Spell
    Slots per Spell Level—— Level Bonus Class Features Divinity Cantrips Spells 1 2 3 4 5 6 7 8 9 1 +2
    Spellcasting, Divine Order — 3 4 2 — — — — — — — — 2 +2 Channel Divinity 2 3 5 3 — — — — — — — — 3
    +2 Cleric Subclass 2 3 6 4 2 — — — — — — — 4 +2 Ability Score Improvement 2 4 7 4 3 — — — — — — —
    5 +3 Sear Undead 2 4 9 4 3 2 — — — — — — 6 +3 Subclass feature 3 4 10 4 3 3 — — — — — — 7 +3 Blessed
    Strikes 3 4 11 4 3 3 1 — — — — — 8 +3 Ability Score Improvement 3 4 12 4 3 3 2 — — — — — 9 +4 — 3
    4 14 4 3 3 3 1 — — — — 10 +4 Divine Intervention 3 5 15 4 3 3 3 2 — — — — 11 +4 — 3 5 16 4 3 3 3 2
    1 — — — 12 +4 Ability Score Improvement 3 5 16 4 3 3 3 2 1 — — — 13 +5 — 3 5 17 4 3 3 3 2 1 1 — —
    14 +5 Improved Blessed Strikes 3 5 17 4 3 3 3 2 1 1 — — 15 +5 — 3 5 18 4 3 3 3 2 1 1 1 — 16 +5 Ability
    Score Improvement 3 5 18 4 3 3 3 2 1 1 1 — 17 +6 Subclass feature 3 5 19 4 3 3 3 2 1 1 1 1 18 +6 —
    4 5 20 4 3 3 3 3 1 1 1 1 19 +6 Epic Boon 4 5 21 4 3 3 3 3 2 1 1 1 20 +6 Greater Divine Intervention
    4 5 22 4 3 3 3 3 2 2 1 1 36 System Reference Document 5.2.1'
  sha256: 98092add89a265ebd776a9e053cbc7bd988a5a43286e28697842ba1bfd08418d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cleric Progression

From SRD 5.2.1 page 36.

     Cleric Features
              Proficiency                                     Channel               Prepared   ——Spell Slots per Spell Level——
      Level     Bonus     Class Features                      Divinity   Cantrips    Spells    1   2   3   4    5   6    7     8   9
       1         +2           Spellcasting, Divine Order          —         3          4       2   — — — — — — — —
       2          +2          Channel Divinity                    2         3          5       3   — — — — — — — —
       3          +2          Cleric Subclass                     2         3          6       4   2   — — — — — — —
       4          +2          Ability Score Improvement           2         4          7       4   3   — — — — — — —
       5          +3          Sear Undead                         2         4          9       4   3   2   — — — — — —
       6          +3          Subclass feature                    3         4          10      4   3   3   — — — — — —
       7          +3          Blessed Strikes                     3         4          11      4   3   3   1    — — — — —
       8          +3          Ability Score Improvement           3         4          12      4   3   3   2    — — — — —
       9         +4           —                                   3         4          14      4   3   3   3    1   — — — —
       10        +4           Divine Intervention                 3         5          15      4   3   3   3    2   — — — —
       11        +4           —                                   3         5          16      4   3   3   3    2   1   — — —
       12         +4          Ability Score Improvement           3         5          16      4   3   3   3    2   1   — — —
       13         +5          —                                   3         5          17      4   3   3   3    2   1    1   — —
       14         +5          Improved Blessed Strikes            3         5          17      4   3   3   3    2   1    1   — —
       15         +5          —                                   3         5          18      4   3   3   3    2   1    1     1   —
       16         +5          Ability Score Improvement           3         5          18      4   3   3   3    2   1    1     1   —
       17         +6          Subclass feature                    3         5          19      4   3   3   3    2   1    1     1   1
       18         +6          —                                   4         5          20      4   3   3   3    3   1    1     1   1
       19         +6          Epic Boon                           4         5          21      4   3   3    3   3   2    1     1   1
       20         +6          Greater Divine Intervention         4         5          22      4   3   3    3   3   2    2     1   1



36     System Reference Document 5.2.1
