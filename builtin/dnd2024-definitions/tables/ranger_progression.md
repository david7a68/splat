---
splat_schema: 1
splat_id: cd5a5303-9e81-5ecc-b785-cf57fa501e3b
splat_kind: rule
splat_key: dnd2024:tables/ranger_progression
splat_name: Ranger Progression
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/ranger_progression
  parameters:
    class: ranger
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Spellcasting, Favored Enemy, Weapon Mastery
      favored_enemy: 2
      prepared_spells: 2
      spell_slots_1: 2
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_1_spellcasting
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_1_favored_enemy
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_1_weapon_mastery
        at_class_level: 1
    - level: 2
      proficiency_bonus: 2
      features_text: Deft Explorer, Fighting Style
      favored_enemy: 2
      prepared_spells: 3
      spell_slots_1: 2
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_2_deft_explorer
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_2_fighting_style
        at_class_level: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Ranger Subclass
      favored_enemy: 2
      prepared_spells: 4
      spell_slots_1: 3
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/ranger_level_3_ranger_subclass
        at_class_level: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      favored_enemy: 2
      prepared_spells: 5
      spell_slots_1: 3
      spell_slots_2: 0
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_4_ability_score_improvement
        at_class_level: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack
      favored_enemy: 3
      prepared_spells: 6
      spell_slots_1: 4
      spell_slots_2: 2
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_5_extra_attack
        at_class_level: 5
    - level: 6
      proficiency_bonus: 3
      features_text: Roving
      favored_enemy: 3
      prepared_spells: 6
      spell_slots_1: 4
      spell_slots_2: 2
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_6_roving
        at_class_level: 6
    - level: 7
      proficiency_bonus: 3
      features_text: Subclass feature
      favored_enemy: 3
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
      favored_enemy: 3
      prepared_spells: 7
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 0
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_4_ability_score_improvement
        at_class_level: 8
    - level: 9
      proficiency_bonus: 4
      features_text: Expertise
      favored_enemy: 4
      prepared_spells: 9
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 2
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_9_expertise
        at_class_level: 9
    - level: 10
      proficiency_bonus: 4
      features_text: Tireless
      favored_enemy: 4
      prepared_spells: 9
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 2
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_10_tireless
        at_class_level: 10
    - level: 11
      proficiency_bonus: 4
      features_text: Subclass feature
      favored_enemy: 4
      prepared_spells: 10
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_selected_subclass_features
        class_level: 11
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      favored_enemy: 4
      prepared_spells: 10
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 0
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_4_ability_score_improvement
        at_class_level: 12
    - level: 13
      proficiency_bonus: 5
      features_text: Relentless Hunter
      favored_enemy: 5
      prepared_spells: 11
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 1
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_13_relentless_hunter
        at_class_level: 13
    - level: 14
      proficiency_bonus: 5
      features_text: Nature’s Veil
      favored_enemy: 5
      prepared_spells: 11
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 1
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_14_nature_s_veil
        at_class_level: 14
    - level: 15
      proficiency_bonus: 5
      features_text: Subclass feature
      favored_enemy: 5
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
      favored_enemy: 5
      prepared_spells: 12
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 2
      spell_slots_5: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_4_ability_score_improvement
        at_class_level: 16
    - level: 17
      proficiency_bonus: 6
      features_text: Precise Hunter
      favored_enemy: 6
      prepared_spells: 14
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_17_precise_hunter
        at_class_level: 17
    - level: 18
      proficiency_bonus: 6
      features_text: Feral Senses
      favored_enemy: 6
      prepared_spells: 14
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 1
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_18_feral_senses
        at_class_level: 18
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      favored_enemy: 6
      prepared_spells: 15
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_19_epic_boon
        at_class_level: 19
    - level: 20
      proficiency_bonus: 6
      features_text: Foe Slayer
      favored_enemy: 6
      prepared_spells: 15
      spell_slots_1: 4
      spell_slots_2: 3
      spell_slots_3: 3
      spell_slots_4: 3
      spell_slots_5: 2
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/ranger_level_20_foe_slayer
        at_class_level: 20
    class_definition:
      key: dnd2024:classes/ranger
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
      favored_enemy:
        kind: resource_capacity
        resource: favored_enemy
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
  implementation: dnd2024:class_progression_ranger_progression
splat_contracts:
- id: dnd2024:class_progression_ranger_progression
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
  - 58
  engine_gaps: &id001
  - dnd2024:class_progression_ranger_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 821efecfa9a2f5be5825a25898020857c2f733ad12abf96343d4e6c45524834a
splat_clauses:
- key: source_rules
  text: '# Ranger Progression From SRD 5.2.1 page 58. Ranger Features Proficiency Favored Prepared —Spell
    Slots per Spell Level— Level Bonus Class Features Enemy Spells 1 2 3 4 5 1 +2 Spellcasting, Favored
    Enemy, 2 2 2 — — — — Weapon Mastery 2 +2 Deft Explorer, Fighting Style 2 3 2 — — — — 3 +2 Ranger Subclass
    2 4 3 — — — — 4 +2 Ability Score Improvement 2 5 3 — — — — 5 +3 Extra Attack 3 6 4 2 — — — 6 +3 Roving
    3 6 4 2 — — — 7 +3 Subclass feature 3 7 4 3 — — — 8 +3 Ability Score Improvement 3 7 4 3 — — — 9 +4
    Expertise 4 9 4 3 2 — — 10 +4 Tireless 4 9 4 3 2 — — 11 +4 Subclass feature 4 10 4 3 3 — — 12 +4 Ability
    Score Improvement 4 10 4 3 3 — — 13 +5 Relentless Hunter 5 11 4 3 3 1 — 14 +5 Nature’s Veil 5 11 4
    3 3 1 — 15 +5 Subclass feature 5 12 4 3 3 2 — 16 +5 Ability Score Improvement 5 12 4 3 3 2 — 17 +6
    Precise Hunter 6 14 4 3 3 3 1 18 +6 Feral Senses 6 14 4 3 3 3 1 19 +6 Epic Boon 6 15 4 3 3 3 2 20
    +6 Foe Slayer 6 15 4 3 3 3 2 Prepared Spells of Level 1+. You prepare the Changing Your Prepared Spells.
    Whenever you list of level 1+ spells that are available for you to finish a Long Rest, you can replace
    one spell on your cast with this feature. To start, choose two level 1 list with another Ranger spell
    for which you have Ranger spells. Cure Wounds and Ensnaring Strike are spell slots. recommended. Spellcasting
    Ability. Wisdom is your spellcasting The number of spells on your list increases as you ability for
    your Ranger spells. gain Ranger levels, as shown in the Prepared Spells Spellcasting Focus. You can
    use a Druidic Focus column of the Ranger Features table. Whenever that as a Spellcasting Focus for
    your Ranger spells. number increases, choose additional Ranger spells until the number of spells on
    your list matches the Level 1: Favored Enemy number in the Ranger Features table. The chosen You always
    have the Hunter’s Mark spell prepared. spells must be of a level for which you have spell You can
    cast it twice without expending a spell slot, slots. For example, if you’re a level 5 Ranger, your
    and you regain all expended uses of this ability list of prepared spells can include six Ranger spells
    when you finish a Long Rest. of level 1 or 2 in any combination. The number of times you can cast
    the spell with- If another Ranger feature gives you spells that out a spell slot increases when you
    reach certain you always have prepared, those spells don’t count Ranger levels, as shown in the Favored
    Enemy colagainst the number of spells you can prepare with umn of the Ranger Features table. this
    feature, but those spells otherwise count as Level 1: Weapon Mastery Ranger spells for you. Your training
    with weapons allows you to use the mastery properties of two kinds of weapons of your 58 System Reference
    Document 5.2.1'
  sha256: 0a208df3f0a891aa7fb01b088bb87ded8ab762004c892dcd50afd56462cacdef
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ranger Progression

From SRD 5.2.1 page 58.

     Ranger Features
              Proficiency                                    Favored   Prepared    —Spell Slots per Spell Level—
      Level     Bonus       Class Features                    Enemy     Spells     1      2      3      4      5
        1         +2        Spellcasting, Favored Enemy,       2          2        2     —      —      —      —
                            Weapon Mastery
        2         +2        Deft Explorer, Fighting Style      2          3        2     —      —      —      —
        3         +2        Ranger Subclass                     2         4        3     —      —      —      —
        4         +2        Ability Score Improvement           2         5        3     —      —      —      —
        5         +3        Extra Attack                        3         6        4      2     —      —      —
        6         +3        Roving                              3         6        4      2     —      —      —
        7         +3        Subclass feature                    3         7        4      3     —      —      —
        8         +3        Ability Score Improvement           3         7        4      3     —      —      —
        9         +4        Expertise                           4         9        4      3      2     —      —
       10         +4        Tireless                            4         9        4      3      2     —      —
       11         +4        Subclass feature                    4         10       4      3      3     —      —
       12         +4        Ability Score Improvement           4         10       4      3      3     —      —
       13         +5        Relentless Hunter                   5         11       4      3      3      1     —
       14         +5        Nature’s Veil                       5         11       4      3      3      1     —
       15         +5        Subclass feature                    5         12       4      3      3      2     —
       16         +5        Ability Score Improvement           5         12       4      3      3      2     —
       17         +6        Precise Hunter                      6         14       4      3      3      3      1
       18         +6        Feral Senses                        6         14       4      3      3      3      1
       19         +6        Epic Boon                           6         15       4      3      3      3      2
       20         +6        Foe Slayer                          6         15       4      3      3      3      2


       Prepared Spells of Level 1+. You prepare the            Changing Your Prepared Spells. Whenever you
     list of level 1+ spells that are available for you to   finish a Long Rest, you can replace one spell on your
     cast with this feature. To start, choose two level 1    list with another Ranger spell for which you have
     Ranger spells. Cure Wounds and Ensnaring Strike are     spell slots.
     recommended.                                              Spellcasting Ability. Wisdom is your spellcasting
       The number of spells on your list increases as you    ability for your Ranger spells.
     gain Ranger levels, as shown in the Prepared Spells       Spellcasting Focus. You can use a Druidic Focus
     column of the Ranger Features table. Whenever that      as a Spellcasting Focus for your Ranger spells.
     number increases, choose additional Ranger spells
     until the number of spells on your list matches the     Level 1: Favored Enemy
     number in the Ranger Features table. The chosen         You always have the Hunter’s Mark spell prepared.
     spells must be of a level for which you have spell      You can cast it twice without expending a spell slot,
     slots. For example, if you’re a level 5 Ranger, your    and you regain all expended uses of this ability
     list of prepared spells can include six Ranger spells   when you finish a Long Rest.
     of level 1 or 2 in any combination.                       The number of times you can cast the spell with-
       If another Ranger feature gives you spells that       out a spell slot increases when you reach certain
     you always have prepared, those spells don’t count      Ranger levels, as shown in the Favored Enemy col-
     against the number of spells you can prepare with       umn of the Ranger Features table.
     this feature, but those spells otherwise count as       Level 1: Weapon Mastery
     Ranger spells for you.                                  Your training with weapons allows you to use the
                                                             mastery properties of two kinds of weapons of your




58   System Reference Document 5.2.1
