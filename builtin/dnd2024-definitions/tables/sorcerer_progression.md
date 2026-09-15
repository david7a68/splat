---
splat_schema: 1
splat_id: a9d22feb-65dc-57b0-a7a3-bd4d58b61f25
splat_kind: rule
splat_key: dnd2024:tables/sorcerer_progression
splat_name: Sorcerer Progression
splat_origin:
  key: dnd2024:reference/pages/065
splat_sources:
- title: SRD 5.2.1
  locator: pp. 65
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/sorcerer_progression
  parameters:
    class: sorcerer
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Spellcasting, Innate Sorcery
      sorcery_points: 0
      cantrips: 4
      prepared_spells: 2
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
          key: dnd2024:class_features/sorcerer_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Font of Magic, Metamagic
      sorcery_points: 2
      cantrips: 4
      prepared_spells: 4
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
          key: dnd2024:class_features/sorcerer_level_2_font_of_magic
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/sorcerer_level_2_metamagic
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Sorcerer Subclass
      sorcery_points: 3
      cantrips: 4
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
          key: dnd2024:class_features/sorcerer_level_3_sorcerer_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      sorcery_points: 4
      cantrips: 5
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
          key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Sorcerous Restoration
      sorcery_points: 5
      cantrips: 5
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
          key: dnd2024:class_features/sorcerer_level_5_sorcerous_restoration
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      sorcery_points: 6
      cantrips: 5
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
      features_text: Sorcery Incarnate
      sorcery_points: 7
      cantrips: 5
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
          key: dnd2024:class_features/sorcerer_level_7_sorcery_incarnate
        at_class_level: 7
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      sorcery_points: 8
      cantrips: 5
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
          key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: —
      sorcery_points: 9
      cantrips: 5
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
      features_text: Metamagic
      sorcery_points: 10
      cantrips: 6
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
          key: dnd2024:class_features/sorcerer_level_2_metamagic
        at_class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: —
      sorcery_points: 11
      cantrips: 6
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
      sorcery_points: 12
      cantrips: 6
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
          key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: —
      sorcery_points: 13
      cantrips: 6
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
      sorcery_points: 14
      cantrips: 6
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
      features_text: —
      sorcery_points: 15
      cantrips: 6
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
      sorcery_points: 16
      cantrips: 6
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
          key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Metamagic
      sorcery_points: 17
      cantrips: 6
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
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/sorcerer_level_2_metamagic
        at_class_level: 17
    - level: 18
      proficiency_bonus: 6
      features_text: Subclass feature
      sorcery_points: 18
      cantrips: 6
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
      - operation: grant_selected_subclass_features
        class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      sorcery_points: 19
      cantrips: 6
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
          key: dnd2024:class_features/sorcerer_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Arcane Apotheosis
      sorcery_points: 20
      cantrips: 6
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
          key: dnd2024:class_features/sorcerer_level_20_arcane_apotheosis
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/sorcerer
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
      sorcery_points:
        kind: resource_capacity
        resource: sorcery_points
        recovery: owning_feature_contract
        level_change_refills: false
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
  implementation: dnd2024:class_progression_sorcerer_progression
splat_contracts:
- id: dnd2024:class_progression_sorcerer_progression
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
  - 65
  engine_gaps: &id001
  - dnd2024:class_progression_sorcerer_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 80a745ae411d5baa3bc4c0a542deabfff261f002554d412bef982d1a1cefbfb6
splat_clauses:
- key: source_rules
  text: '# Sorcerer Progression From SRD 5.2.1 page 65. Sorcerer Features Proficiency Sorcery Prepared
    ——Spell Slots per Spell Level—— Level Bonus Class Features Points Cantrips Spells 1 2 3 4 5 6 7 8
    9 1 +2 Spellcasting, — 4 2 2 — — — — — — — — Innate Sorcery 2 +2 Font of Magic, 2 4 4 3 — — — — —
    — — — Metamagic 3 +2 Sorcerer Subclass 3 4 6 4 2 — — — — — — — 4 +2 Ability Score 4 5 7 4 3 — — —
    — — — — Improvement 5 +3 Sorcerous 5 5 9 4 3 2 — — — — — — Restoration 6 +3 Subclass feature 6 5 10
    4 3 3 — — — — — — 7 +3 Sorcery Incarnate 7 5 11 4 3 3 1 — — — — — 8 +3 Ability Score 8 5 12 4 3 3
    2 — — — — — Improvement 9 +4 — 9 5 14 4 3 3 3 1 — — — — 10 +4 Metamagic 10 6 15 4 3 3 3 2 — — — —
    11 +4 — 11 6 16 4 3 3 3 2 1 — — — 12 +4 Ability Score 12 6 16 4 3 3 3 2 1 — — — Improvement 13 +5
    — 13 6 17 4 3 3 3 2 1 1 — — 14 +5 Subclass feature 14 6 17 4 3 3 3 2 1 1 — — 15 +5 — 15 6 18 4 3 3
    3 2 1 1 1 — 16 +5 Ability Score 16 6 18 4 3 3 3 2 1 1 1 — Improvement 17 +6 Metamagic 17 6 19 4 3
    3 3 2 1 1 1 1 18 +6 Subclass feature 18 6 20 4 3 3 3 3 1 1 1 1 19 +6 Epic Boon 19 6 21 4 3 3 3 3 2
    1 1 1 20 +6 Arcane Apotheosis 20 6 22 4 3 3 3 3 2 2 1 1 spells. You regain all expended slots when
    you finagainst the number of spells you can prepare with ish a Long Rest. this feature, but those
    spells otherwise count as Prepared Spells of Level 1+. You prepare the list Sorcerer spells for you.
    of level 1+ spells that are available for you to cast Changing Your Prepared Spells. Whenever you
    with this feature. To start, choose two level 1 Sorgain a Sorcerer level, you can replace one spell
    on cerer spells. Burning Hands and Detect Magic are your list with another Sorcerer spell for which
    you recommended. have spell slots. The number of spells on your list increases as Spellcasting Ability.
    Charisma is your spellcastyou gain Sorcerer levels, as shown in the Prepared ing ability for your
    Sorcerer spells. Spells column of the Sorcerer Features table. When- Spellcasting Focus. You can use
    an Arcane Focus ever that number increases, choose additional as a Spellcasting Focus for your Sorcerer
    spells. Sorcerer spells until the number of spells on your list matches the number in the Sorcerer
    Features Level 1: Innate Sorcery table. The chosen spells must be of a level for which An event in
    your past left an indelible mark on you, you have spell slots. For example, if you’re a level 3 infusing
    you with simmering magic. As a Bonus Sorcerer, your list of prepared spells can include six Action,
    you can unleash that magic for 1 minute, Sorcerer spells of level 1 or 2 in any combination. during
    which you gain the following benefits: If another Sorcerer feature gives you spells that you always
    have prepared, those spells don’t count 65 System Reference Document 5.2.1'
  sha256: 7e078c012e04d874c1e9480e7bd7420779b8ef7d1d60cd42a1d1db58ed12c2c8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sorcerer Progression

From SRD 5.2.1 page 65.

     Sorcerer Features
            Proficiency                          Sorcery                 Prepared     ——Spell Slots per Spell Level——
      Level Bonus       Class Features            Points      Cantrips    Spells     1    2   3   4   5    6   7   8   9
        1        +2       Spellcasting,             —            4           2       2   — — — — — — — —
                          Innate Sorcery
        2        +2       Font of Magic,            2            4           4       3   — — — — — — — —
                          Metamagic
        3        +2       Sorcerer Subclass         3            4           6       4    2   — — — — — — —
        4        +2       Ability Score             4            5           7       4    3   — — — — — — —
                          Improvement
        5        +3       Sorcerous                 5            5           9       4    3   2   — — — — — —
                          Restoration
        6        +3       Subclass feature          6            5           10      4    3   3   — — — — — —
        7        +3       Sorcery Incarnate         7            5           11      4    3   3   1   — — — — —
        8        +3       Ability Score             8            5           12      4    3   3   2   — — — — —
                          Improvement
        9        +4       —                         9            5           14      4    3   3   3   1   — — — —
       10        +4       Metamagic                 10           6           15      4    3   3   3   2   — — — —
       11        +4       —                         11           6           16      4    3   3   3   2    1   — — —
       12        +4       Ability Score             12           6           16      4    3   3   3   2    1   — — —
                          Improvement
       13        +5       —                         13           6           17      4    3   3   3   2    1   1   — —
       14        +5       Subclass feature          14           6           17      4    3   3   3   2    1   1   — —
       15        +5       —                         15           6           18      4    3   3   3   2    1   1   1   —
       16        +5       Ability Score             16           6           18      4    3   3   3   2    1   1   1   —
                          Improvement
       17        +6       Metamagic                 17           6           19      4    3   3   3   2    1   1   1   1
       18        +6       Subclass feature          18           6           20      4    3   3   3   3    1   1   1   1
       19        +6       Epic Boon                 19           6           21      4    3   3   3   3    2   1   1   1
       20        +6       Arcane Apotheosis         20           6           22      4    3   3   3   3    2   2   1   1



     spells. You regain all expended slots when you fin-             against the number of spells you can prepare with
     ish a Long Rest.                                                this feature, but those spells otherwise count as
       Prepared Spells of Level 1+. You prepare the list             Sorcerer spells for you.
     of level 1+ spells that are available for you to cast             Changing Your Prepared Spells. Whenever you
     with this feature. To start, choose two level 1 Sor-            gain a Sorcerer level, you can replace one spell on
     cerer spells. Burning Hands and Detect Magic are                your list with another Sorcerer spell for which you
     recommended.                                                    have spell slots.
       The number of spells on your list increases as                  Spellcasting Ability. Charisma is your spellcast-
     you gain Sorcerer levels, as shown in the Prepared              ing ability for your Sorcerer spells.
     Spells column of the Sorcerer Features table. When-               Spellcasting Focus. You can use an Arcane Focus
     ever that number increases, choose additional                   as a Spellcasting Focus for your Sorcerer spells.
     Sorcerer spells until the number of spells on your
     list matches the number in the Sorcerer Features                Level 1: Innate Sorcery
     table. The chosen spells must be of a level for which           An event in your past left an indelible mark on you,
     you have spell slots. For example, if you’re a level 3          infusing you with simmering magic. As a Bonus
     Sorcerer, your list of prepared spells can include six          Action, you can unleash that magic for 1 minute,
     Sorcerer spells of level 1 or 2 in any combination.             during which you gain the following benefits:
       If another Sorcerer feature gives you spells that
     you always have prepared, those spells don’t count


65   System Reference Document 5.2.1
