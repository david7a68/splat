---
splat_schema: 1
splat_id: 4a5382bb-7c82-5ff1-a3e9-3fb1e0b4dc35
splat_kind: rule
splat_key: dnd2024:tables/wizard_progression
splat_name: Wizard Progression
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/wizard_progression
  parameters:
    class: wizard
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Spellcasting, Ritual Adept, Arcane Recovery
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
          key: dnd2024:class_features/wizard_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/wizard_level_1_ritual_adept
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/wizard_level_1_arcane_recovery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Scholar
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
          key: dnd2024:class_features/wizard_level_2_scholar
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Wizard Subclass
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
          key: dnd2024:class_features/wizard_level_3_wizard_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
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
          key: dnd2024:class_features/wizard_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Memorize Spell
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
          key: dnd2024:class_features/wizard_level_5_memorize_spell
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
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
      features_text: —
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
      grants: []
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
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
          key: dnd2024:class_features/wizard_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: —
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
      features_text: Subclass feature
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
      - operation: grant_selected_subclass_features
        class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: —
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
          key: dnd2024:class_features/wizard_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: —
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
      features_text: Subclass feature
      cantrips: 5
      prepared_spells: 18
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
      features_text: —
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
      spell_slots_9: 0
      grants: []
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      cantrips: 5
      prepared_spells: 21
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
          key: dnd2024:class_features/wizard_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: —
      cantrips: 5
      prepared_spells: 22
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
      features_text: Spell Mastery
      cantrips: 5
      prepared_spells: 23
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
          key: dnd2024:class_features/wizard_level_18_spell_mastery
        at_class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      cantrips: 5
      prepared_spells: 24
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
          key: dnd2024:class_features/wizard_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Signature Spells
      cantrips: 5
      prepared_spells: 25
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
          key: dnd2024:class_features/wizard_level_20_signature_spells
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/wizard
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
  implementation: dnd2024:class_progression_wizard_progression
splat_contracts:
- id: dnd2024:class_progression_wizard_progression
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
  - 77
  engine_gaps: &id001
  - dnd2024:class_progression_wizard_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c6f534fcaacd82bfdbb971df25c42bcf71a654839f27ef2b358f678e1133d7a2
splat_clauses:
- key: source_rules
  text: '# Wizard Progression From SRD 5.2.1 page 77. Wizard Features Proficiency Prepared ——Spell Slots
    per Spell Level—— Level Bonus Class Features Cantrips Spells 1 2 3 4 5 6 7 8 9 1 +2 Spellcasting,
    Ritual Adept, 3 4 2 — — — — — — — — Arcane Recovery 2 +2 Scholar 3 5 3 — — — — — — — — 3 +2 Wizard
    Subclass 3 6 4 2 — — — — — — — 4 +2 Ability Score Improvement 4 7 4 3 — — — — — — — 5 +3 Memorize
    Spell 4 9 4 3 2 — — — — — — 6 +3 Subclass feature 4 10 4 3 3 — — — — — — 7 +3 — 4 11 4 3 3 1 — — —
    — — 8 +3 Ability Score Improvement 4 12 4 3 3 2 — — — — — 9 +4 — 4 14 4 3 3 3 1 — — — — 10 +4 Subclass
    feature 5 15 4 3 3 3 2 — — — — 11 +4 — 5 16 4 3 3 3 2 1 — — — 12 +4 Ability Score Improvement 5 16
    4 3 3 3 2 1 — — — 13 +5 — 5 17 4 3 3 3 2 1 1 — — 14 +5 Subclass feature 5 18 4 3 3 3 2 1 1 — — 15
    +5 — 5 19 4 3 3 3 2 1 1 1 — 16 +5 Ability Score Improvement 5 21 4 3 3 3 2 1 1 1 — 17 +6 — 5 22 4
    3 3 3 2 1 1 1 1 18 +6 Spell Mastery 5 23 4 3 3 3 3 1 1 1 1 19 +6 Epic Boon 5 24 4 3 3 3 3 2 1 1 1
    20 +6 Signature Spells 5 25 4 3 3 3 3 2 2 1 1 77 System Reference Document 5.2.1'
  sha256: f14d1c5f6462437df9b0d48a782d6d926f539e74048c35e906321860c882489e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wizard Progression

From SRD 5.2.1 page 77.

     Wizard Features
            Proficiency                                                      Prepared     ——Spell Slots per Spell Level——
      Level Bonus       Class Features                           Cantrips     Spells     1    2   3   4    5   6    7   8    9
        1         +2       Spellcasting, Ritual Adept,              3            4       2   — — — — — — — —
                           Arcane Recovery
        2         +2       Scholar                                  3            5       3   — — — — — — — —
        3         +2       Wizard Subclass                          3            6       4    2   — — — — — — —
        4         +2       Ability Score Improvement                4            7       4    3   — — — — — — —
        5         +3       Memorize Spell                           4            9       4    3   2   — — — — — —
        6         +3       Subclass feature                         4           10       4    3   3   — — — — — —
        7         +3       —                                        4           11       4    3   3    1   — — — — —
        8         +3       Ability Score Improvement                4           12       4    3   3    2   — — — — —
        9         +4       —                                        4           14       4    3   3    3   1   — — — —
       10         +4       Subclass feature                         5           15       4    3   3    3   2   — — — —
       11         +4       —                                        5           16       4    3   3    3   2   1    — — —
       12         +4       Ability Score Improvement                5           16       4    3   3    3   2   1    — — —
       13         +5       —                                        5           17       4    3   3    3   2   1    1   — —
       14         +5       Subclass feature                         5           18       4    3   3    3   2   1    1   — —
       15         +5       —                                        5           19       4    3   3    3   2   1    1   1   —
       16         +5       Ability Score Improvement                5           21       4    3   3    3   2   1    1   1   —
       17         +6       —                                        5           22       4    3   3    3   2    1   1   1    1
       18         +6       Spell Mastery                            5           23       4    3   3    3   3    1   1   1    1
       19         +6       Epic Boon                                5           24       4    3   3    3   3    2   1   1    1
       20         +6       Signature Spells                         5           25       4    3   3    3   3    2   2   1    1



77   System Reference Document 5.2.1
