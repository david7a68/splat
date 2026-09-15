---
splat_schema: 1
splat_id: 64fbb432-a4f4-54bd-8634-28cf69142ff6
splat_kind: rule
splat_key: dnd2024:tables/bard_progression
splat_name: Bard Progression
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/bard_progression
  parameters:
    class: bard
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Bardic Inspiration, Spellcasting
      bardic_die: d6
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
          key: dnd2024:class_features/bard_level_1_bardic_inspiration
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/bard_level_1_spellcasting
        at_class_level: 1
      bardic_die_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 2
      proficiency_bonus: 2
      features_text: Expertise, Jack of All Trades
      bardic_die: d6
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
          key: dnd2024:class_features/bard_level_2_expertise
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/bard_level_2_jack_of_all_trades
        at_class_level: 2
      bardic_die_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 3
      proficiency_bonus: 2
      features_text: Bard Subclass
      bardic_die: d6
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
          key: dnd2024:class_features/bard_level_3_bard_subclass
        at_class_level: 3
      bardic_die_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      bardic_die: d6
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
          key: dnd2024:class_features/bard_level_4_ability_score_improvement
        at_class_level: 4
      bardic_die_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 5
      proficiency_bonus: 3
      features_text: Font of Inspiration
      bardic_die: d8
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
          key: dnd2024:class_features/bard_level_5_font_of_inspiration
        at_class_level: 5
      bardic_die_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      bardic_die: d8
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
      bardic_die_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 7
      proficiency_bonus: 3
      features_text: Countercharm
      bardic_die: d8
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
          key: dnd2024:class_features/bard_level_7_countercharm
        at_class_level: 7
      bardic_die_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      bardic_die: d8
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
          key: dnd2024:class_features/bard_level_4_ability_score_improvement
        at_class_level: 8
      bardic_die_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 9
      proficiency_bonus: 4
      features_text: Expertise
      bardic_die: d8
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
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/bard_level_2_expertise
        at_class_level: 9
      bardic_die_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 10
      proficiency_bonus: 4
      features_text: Magical Secrets
      bardic_die: d10
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
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/bard_level_10_magical_secrets
        at_class_level: 10
      bardic_die_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 11
      proficiency_bonus: 4
      features_text: —
      bardic_die: d10
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
      bardic_die_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      bardic_die: d10
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
          key: dnd2024:class_features/bard_level_4_ability_score_improvement
        at_class_level: 12
      bardic_die_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 13
      proficiency_bonus: 5
      features_text: —
      bardic_die: d10
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
      bardic_die_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 14
      proficiency_bonus: 5
      features_text: Subclass feature
      bardic_die: d10
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
      bardic_die_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 15
      proficiency_bonus: 5
      features_text: —
      bardic_die: d12
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
      grants: []
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      bardic_die: d12
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
          key: dnd2024:class_features/bard_level_4_ability_score_improvement
        at_class_level: 16
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 17
      proficiency_bonus: 6
      features_text: —
      bardic_die: d12
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
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 18
      proficiency_bonus: 6
      features_text: Superior Inspiration
      bardic_die: d12
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
          key: dnd2024:class_features/bard_level_18_superior_inspiration
        at_class_level: 18
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      bardic_die: d12
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
          key: dnd2024:class_features/bard_level_19_epic_boon
        at_class_level: 19
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 20
      proficiency_bonus: 6
      features_text: Words of Creation
      bardic_die: d12
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
          key: dnd2024:class_features/bard_level_20_words_of_creation
        at_class_level: 20
      bardic_die_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    class_definition:
      key: dnd2024:classes/bard
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
      bardic_die:
        kind: dice_expression
        applies_to: bardic_die
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
  implementation: dnd2024:class_progression_bard_progression
splat_contracts:
- id: dnd2024:class_progression_bard_progression
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
  - 31
  engine_gaps: &id001
  - dnd2024:class_progression_bard_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 4c049acc2e2a14879186813a81793242f3dd65038279178204d5f8049eb76a47
splat_clauses:
- key: source_rules
  text: '# Bard Progression From SRD 5.2.1 page 31. Bard Features Proficiency Bardic Prepared ——Spell
    Slots per Spell Level—— Level Bonus Class Features Die Cantrips Spells 1 2 3 4 5 6 7 8 9 1 +2 Bardic
    Inspiration, D6 2 4 2 — — — — — — — — Spellcasting 2 +2 Expertise, D6 2 5 3 — — — — — — — — Jack of
    All Trades 3 +2 Bard Subclass D6 2 6 4 2 — — — — — — — 4 +2 Ability Score Improvement D6 3 7 4 3 —
    — — — — — — 5 +3 Font of Inspiration D8 3 9 4 3 2 — — — — — — 6 +3 Subclass feature D8 3 10 4 3 3
    — — — — — — 7 +3 Countercharm D8 3 11 4 3 3 1 — — — — — 8 +3 Ability Score Improvement D8 3 12 4 3
    3 2 — — — — — 9 +4 Expertise D8 3 14 4 3 3 3 1 — — — — 10 +4 Magical Secrets D10 4 15 4 3 3 3 2 —
    — — — 11 +4 — D10 4 16 4 3 3 3 2 1 — — — 12 +4 Ability Score Improvement D10 4 16 4 3 3 3 2 1 — —
    — 13 +5 — D10 4 17 4 3 3 3 2 1 1 — — 14 +5 Subclass feature D10 4 17 4 3 3 3 2 1 1 — — 15 +5 — D12
    4 18 4 3 3 3 2 1 1 1 — 16 +5 Ability Score Improvement D12 4 18 4 3 3 3 2 1 1 1 — 17 +6 — D12 4 19
    4 3 3 3 2 1 1 1 1 18 +6 Superior Inspiration D12 4 20 4 3 3 3 3 1 1 1 1 19 +6 Epic Boon D12 4 21 4
    3 3 3 3 2 1 1 1 20 +6 Words of Creation D12 4 22 4 3 3 3 3 2 2 1 1 31 System Reference Document 5.2.1'
  sha256: 9c15df9fcfd7b83bd98ae96eb89ceb2e603f8297085346ce5a45d9546c9d8206
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bard Progression

From SRD 5.2.1 page 31.

     Bard Features
             Proficiency                                   Bardic          Prepared      ——Spell Slots per Spell Level——
     Level     Bonus        Class Features                  Die   Cantrips  Spells      1 2 3 4 5 6 7 8 9
       1         +2         Bardic Inspiration,              D6      2          4       2   — — — — — — — —
                            Spellcasting
       2         +2         Expertise,                       D6      2          5       3   — — — — — — — —
                            Jack of All Trades
       3         +2         Bard Subclass                    D6      2          6       4   2   — — — — — — —
       4         +2         Ability Score Improvement        D6      3          7       4   3   — — — — — — —
       5         +3         Font of Inspiration              D8      3          9       4   3    2   — — — — — —
       6         +3         Subclass feature                 D8      3         10       4   3    3   — — — — — —
       7         +3         Countercharm                     D8      3         11       4   3    3   1   — — — — —
       8         +3         Ability Score Improvement        D8      3         12       4   3    3   2   — — — — —
       9         +4         Expertise                        D8      3         14       4   3    3   3    1   — — — —
      10         +4         Magical Secrets                 D10      4         15       4   3    3   3    2   — — — —
       11        +4         —                               D10      4         16       4   3    3   3    2   1   — — —
       12        +4         Ability Score Improvement       D10      4         16       4   3    3   3    2   1   — — —
       13        +5         —                               D10      4         17       4   3    3   3    2   1    1   — —
       14        +5         Subclass feature                D10      4         17       4   3    3   3    2   1    1   — —
       15        +5         —                               D12      4         18       4   3    3   3    2   1    1   1    —
       16        +5         Ability Score Improvement       D12      4         18       4   3    3   3    2   1    1   1    —
       17        +6         —                               D12      4         19       4   3    3   3    2   1    1   1       1
       18        +6         Superior Inspiration            D12      4         20       4   3    3   3    3   1    1   1       1
       19        +6         Epic Boon                       D12      4         21       4   3    3   3    3   2    1   1       1
      20         +6         Words of Creation               D12      4         22       4   3    3   3    3   2    2   1       1



31   System Reference Document 5.2.1
