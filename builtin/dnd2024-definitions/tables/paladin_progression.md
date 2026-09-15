---
splat_schema: 1
splat_id: 865c298e-3797-559d-8907-54fc43e851a1
splat_kind: rule
splat_key: dnd2024:tables/paladin_progression
splat_name: Paladin Progression
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/paladin_progression
  parameters:
    class: paladin
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Lay On Hands, Spellcasting, Weapon Mastery
      channel_divinity: 0
      prepared_spells: 2
      spell_slots_1: 2
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_1_lay_on_hands
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_1_weapon_mastery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Fighting Style, Paladin’s Smite
      channel_divinity: 0
      prepared_spells: 3
      spell_slots_1: 2
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_2_fighting_style
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_2_paladin_s_smite
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Channel Divinity, Paladin Subclass
      channel_divinity: 2
      prepared_spells: 4
      spell_slots_1: 3
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_3_channel_divinity
        at_class_level: 3
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/paladin_level_3_paladin_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      channel_divinity: 2
      prepared_spells: 5
      spell_slots_1: 3
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Faithful Steed
      channel_divinity: 2
      prepared_spells: 6
      spell_slots_1: 4
      spell_slots_2: 2
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_5_extra_attack
        at_class_level: 5
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_5_faithful_steed
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Aura of Protection
      channel_divinity: 2
      prepared_spells: 6
      spell_slots_1: 4
      spell_slots_2: 2
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_6_aura_of_protection
        at_class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: Subclass feature
      channel_divinity: 2
      prepared_spells: 7
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      channel_divinity: 2
      prepared_spells: 7
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: Abjure Foes
      channel_divinity: 2
      prepared_spells: 9
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 2
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_9_abjure_foes
        at_class_level: 9
    - level: 10
      proficiency_bonus: 4
      features_text: Aura of Courage
      channel_divinity: 2
      prepared_spells: 9
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 2
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_10_aura_of_courage
        at_class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: Radiant Strikes
      channel_divinity: 3
      prepared_spells: 10
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_11_radiant_strikes
        at_class_level: 11
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      channel_divinity: 3
      prepared_spells: 10
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: —
      channel_divinity: 3
      prepared_spells: 11
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 1
      spell_slots_5: 0
      grants: []
    - level: 14
      proficiency_bonus: 5
      features_text: Restoring Touch
      channel_divinity: 3
      prepared_spells: 11
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 1
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_14_restoring_touch
        at_class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Subclass feature
      channel_divinity: 3
      prepared_spells: 12
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 2
      spell_slots_5: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 15
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      channel_divinity: 3
      prepared_spells: 12
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 2
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: —
      channel_divinity: 3
      prepared_spells: 14
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 1
      grants: []
    - level: 18
      proficiency_bonus: 6
      features_text: Aura Expansion
      channel_divinity: 3
      prepared_spells: 14
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_18_aura_expansion
        at_class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      channel_divinity: 3
      prepared_spells: 15
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/paladin_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Subclass feature
      channel_divinity: 3
      prepared_spells: 15
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      grants:
      - operation: grant_selected_subclass_features
        class_level: 20
    class_definition:
      key: dnd2024:classes/paladin
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
  implementation: dnd2024:class_progression_paladin_progression
splat_contracts:
- id: dnd2024:class_progression_paladin_progression
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
  - 53
  engine_gaps: &id001
  - dnd2024:class_progression_paladin_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 144fe61aeec82979426caafb34a00af4eb2bd6ccdb4838b4dae3f3300f52bafd
splat_clauses:
- key: source_rules
  text: '# Paladin Progression From SRD 5.2.1 page 53. Paladin Features Proficiency Channel Prepared —Spell
    Slots per Spell Level— Level Bonus Class Features Divinity Spells 1 2 3 4 5 1 +2 Lay On Hands, Spellcasting,
    — 2 2 — — — — Weapon Mastery 2 +2 Fighting Style, Paladin’s Smite — 3 2 — — — — 3 +2 Channel Divinity,
    Paladin Subclass 2 4 3 — — — — 4 +2 Ability Score Improvement 2 5 3 — — — — 5 +3 Extra Attack, Faithful
    Steed 2 6 4 2 — — — 6 +3 Aura of Protection 2 6 4 2 — — — 7 +3 Subclass feature 2 7 4 3 — — — 8 +3
    Ability Score Improvement 2 7 4 3 — — — 9 +4 Abjure Foes 2 9 4 3 2 — — 10 +4 Aura of Courage 2 9 4
    3 2 — — 11 +4 Radiant Strikes 3 10 4 3 3 — — 12 +4 Ability Score Improvement 3 10 4 3 3 — — 13 +5
    — 3 11 4 3 3 1 — 14 +5 Restoring Touch 3 11 4 3 3 1 — 15 +5 Subclass feature 3 12 4 3 3 2 — 16 +5
    Ability Score Improvement 3 12 4 3 3 2 — 17 +6 — 3 14 4 3 3 3 1 18 +6 Aura Expansion 3 14 4 3 3 3
    1 19 +6 Epic Boon 3 15 4 3 3 3 2 20 +6 Subclass feature 3 15 4 3 3 3 2 53 System Reference Document
    5.2.1'
  sha256: 5de22c48f98c23967571cc8e826626ec4572db01d10f097b348bea69e59ca6e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paladin Progression

From SRD 5.2.1 page 53.

     Paladin Features
              Proficiency                                        Channel    Prepared    —Spell Slots per Spell Level—
      Level     Bonus       Class Features                       Divinity    Spells     1      2      3      4       5
        1         +2        Lay On Hands, Spellcasting,            —           2        2     —      —      —        —
                            Weapon Mastery
        2         +2        Fighting Style, Paladin’s Smite        —           3        2     —      —      —        —
        3         +2        Channel Divinity, Paladin Subclass      2          4        3     —      —      —        —
        4         +2        Ability Score Improvement               2          5        3     —      —      —        —
        5         +3        Extra Attack, Faithful Steed            2          6        4      2     —      —        —
        6         +3        Aura of Protection                      2          6        4      2     —      —        —
        7         +3        Subclass feature                        2          7        4      3     —      —        —
        8         +3        Ability Score Improvement               2          7        4      3     —      —        —
        9         +4        Abjure Foes                             2          9        4      3      2     —        —
       10         +4        Aura of Courage                         2          9        4      3      2     —        —
       11         +4        Radiant Strikes                         3          10       4      3      3     —        —
       12         +4        Ability Score Improvement               3          10       4      3      3     —        —
       13         +5        —                                       3          11       4      3      3      1       —
       14         +5        Restoring Touch                         3          11       4      3      3      1       —
       15         +5        Subclass feature                        3          12       4      3      3      2       —
       16         +5        Ability Score Improvement               3          12       4      3      3      2       —
       17         +6        —                                       3          14       4      3      3      3       1
       18         +6        Aura Expansion                          3          14       4      3      3      3       1
       19         +6        Epic Boon                               3          15       4      3      3      3       2
       20         +6        Subclass feature                        3          15       4      3      3      3       2



53   System Reference Document 5.2.1
