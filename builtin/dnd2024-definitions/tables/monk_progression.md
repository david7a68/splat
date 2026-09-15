---
splat_schema: 1
splat_id: 235f10c4-25ec-5b36-8da9-344befa0f4c4
splat_kind: rule
splat_key: dnd2024:tables/monk_progression
splat_name: Monk Progression
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:tables/monk_progression
  parameters:
    class: monk
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Martial Arts, Unarmored Defense
      martial_arts: 1d6
      focus_points: 0
      unarmored_movement: 0
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_1_martial_arts
        at_class_level: 1
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_1_unarmored_defense
        at_class_level: 1
      martial_arts_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 2
      proficiency_bonus: 2
      features_text: Monk’s Focus, Unarmored Movement, Uncanny Metabolism
      martial_arts: 1d6
      focus_points: 2
      unarmored_movement: 10
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_2_monk_s_focus
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_2_unarmored_movement
        at_class_level: 2
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_2_uncanny_metabolism
        at_class_level: 2
      martial_arts_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 3
      proficiency_bonus: 2
      features_text: Deflect Attacks, Monk Subclass
      martial_arts: 1d6
      focus_points: 3
      unarmored_movement: 10
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_3_deflect_attacks
        at_class_level: 3
      - operation: choose_subclass
        feature:
          key: dnd2024:class_features/monk_level_3_monk_subclass
        at_class_level: 3
      martial_arts_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement, Slow Fall
      martial_arts: 1d6
      focus_points: 4
      unarmored_movement: 10
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_4_ability_score_improvement
        at_class_level: 4
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_4_slow_fall
        at_class_level: 4
      martial_arts_dice:
        dice:
          count: 1
          sides: 6
        bonus: 0
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Stunning Strike
      martial_arts: 1d8
      focus_points: 5
      unarmored_movement: 10
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_5_extra_attack
        at_class_level: 5
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_5_stunning_strike
        at_class_level: 5
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 6
      proficiency_bonus: 3
      features_text: Empowered Strikes, Subclass feature
      martial_arts: 1d8
      focus_points: 6
      unarmored_movement: 15
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_6_empowered_strikes
        at_class_level: 6
      - operation: grant_selected_subclass_features
        class_level: 6
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 7
      proficiency_bonus: 3
      features_text: Evasion
      martial_arts: 1d8
      focus_points: 7
      unarmored_movement: 15
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_7_evasion
        at_class_level: 7
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      martial_arts: 1d8
      focus_points: 8
      unarmored_movement: 15
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_4_ability_score_improvement
        at_class_level: 8
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 9
      proficiency_bonus: 4
      features_text: Acrobatic Movement
      martial_arts: 1d8
      focus_points: 9
      unarmored_movement: 15
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_9_acrobatic_movement
        at_class_level: 9
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 10
      proficiency_bonus: 4
      features_text: Heightened Focus, Self-Restoration
      martial_arts: 1d8
      focus_points: 10
      unarmored_movement: 20
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_10_heightened_focus
        at_class_level: 10
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_10_self_restoration
        at_class_level: 10
      martial_arts_dice:
        dice:
          count: 1
          sides: 8
        bonus: 0
    - level: 11
      proficiency_bonus: 4
      features_text: Subclass feature
      martial_arts: 1d10
      focus_points: 11
      unarmored_movement: 20
      grants:
      - operation: grant_selected_subclass_features
        class_level: 11
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      martial_arts: 1d10
      focus_points: 12
      unarmored_movement: 20
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_4_ability_score_improvement
        at_class_level: 12
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 13
      proficiency_bonus: 5
      features_text: Deflect Energy
      martial_arts: 1d10
      focus_points: 13
      unarmored_movement: 20
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_13_deflect_energy
        at_class_level: 13
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 14
      proficiency_bonus: 5
      features_text: Disciplined Survivor
      martial_arts: 1d10
      focus_points: 14
      unarmored_movement: 25
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_14_disciplined_survivor
        at_class_level: 14
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 15
      proficiency_bonus: 5
      features_text: Perfect Focus
      martial_arts: 1d10
      focus_points: 15
      unarmored_movement: 25
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_15_perfect_focus
        at_class_level: 15
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      martial_arts: 1d10
      focus_points: 16
      unarmored_movement: 25
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_4_ability_score_improvement
        at_class_level: 16
      martial_arts_dice:
        dice:
          count: 1
          sides: 10
        bonus: 0
    - level: 17
      proficiency_bonus: 6
      features_text: Subclass feature
      martial_arts: 1d12
      focus_points: 17
      unarmored_movement: 25
      grants:
      - operation: grant_selected_subclass_features
        class_level: 17
      martial_arts_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 18
      proficiency_bonus: 6
      features_text: Superior Defense
      martial_arts: 1d12
      focus_points: 18
      unarmored_movement: 30
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_18_superior_defense
        at_class_level: 18
      martial_arts_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      martial_arts: 1d12
      focus_points: 19
      unarmored_movement: 30
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_19_epic_boon
        at_class_level: 19
      martial_arts_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    - level: 20
      proficiency_bonus: 6
      features_text: Body and Mind
      martial_arts: 1d12
      focus_points: 20
      unarmored_movement: 30
      grants:
      - operation: grant_feature
        feature:
          key: dnd2024:class_features/monk_level_20_body_and_mind
        at_class_level: 20
      martial_arts_dice:
        dice:
          count: 1
          sides: 12
        bonus: 0
    class_definition:
      key: dnd2024:classes/monk
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
      focus_points:
        kind: resource_capacity
        resource: focus_points
        recovery: owning_feature_contract
        level_change_refills: false
      martial_arts:
        kind: dice_expression
        applies_to: martial_arts
      proficiency_bonus:
        kind: bonus
        applies_to: proficiency
        use_class_row_only_if_single_class: true
      unarmored_movement:
        kind: speed_bonus
        unit: ft
        requires: unarmored_movement_feature_eligibility
  implementation: dnd2024:class_progression_monk_progression
splat_contracts:
- id: dnd2024:class_progression_monk_progression
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
  - 50
  engine_gaps: &id001
  - dnd2024:class_progression_monk_progression
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 541a87b4c254b575f6cc618b79f1a58be0a990da386b479c808d9d04d93ae193
splat_clauses:
- key: source_rules
  text: '# Monk Progression From SRD 5.2.1 page 50. Monk Features Proficiency Martial Focus Unarmored
    Level Bonus Class Features Arts Points Movement 1 +2 Martial Arts, Unarmored Defense 1d6 — — 2 +2
    Monk’s Focus, Unarmored Movement, 1d6 2 +10 ft. Uncanny Metabolism 3 +2 Deflect Attacks, Monk Subclass
    1d6 3 +10 ft. 4 +2 Ability Score Improvement, Slow Fall 1d6 4 +10 ft. 5 +3 Extra Attack, Stunning
    Strike 1d8 5 +10 ft. 6 +3 Empowered Strikes, Subclass feature 1d8 6 +15 ft. 7 +3 Evasion 1d8 7 +15
    ft. 8 +3 Ability Score Improvement 1d8 8 +15 ft. 9 +4 Acrobatic Movement 1d8 9 +15 ft. 10 +4 Heightened
    Focus, Self-Restoration 1d8 10 +20 ft. 11 +4 Subclass feature 1d10 11 +20 ft. 12 +4 Ability Score
    Improvement 1d10 12 +20 ft. 13 +5 Deflect Energy 1d10 13 +20 ft. 14 +5 Disciplined Survivor 1d10 14
    +25 ft. 15 +5 Perfect Focus 1d10 15 +25 ft. 16 +5 Ability Score Improvement 1d10 16 +25 ft. 17 +6
    Subclass feature 1d12 17 +25 ft. 18 +6 Superior Defense 1d12 18 +30 ft. 19 +6 Epic Boon 1d12 19 +30
    ft. 20 +6 Body and Mind 1d12 20 +30 ft. Level 1: Martial Arts Level 1: Unarmored Defense Your practice
    of martial arts gives you mastery of While you aren’t wearing armor or wielding a combat styles that
    use your Unarmed Strike and Shield, your base Armor Class equals 10 plus your Monk weapons, which
    are the following: Dexterity and Wisdom modifiers. • Simple Melee weapons Level 2: Monk’s Focus •
    Martial Melee weapons that have the Light Your focus and martial training allow you to harproperty
    ness a well of extraordinary energy within yourself. You gain the following benefits while you are
    un- This energy is represented by Focus Points. Your armed or wielding only Monk weapons and you ar-
    Monk level determines the number of points you en’t wearing armor or wielding a Shield. have, as shown
    in the Focus Points column of the Bonus Unarmed Strike. You can make an Un- Monk Features table. armed
    Strike as a Bonus Action. You can expend these points to enhance or fuel Martial Arts Die. You can
    roll 1d6 in place of the certain Monk features. You start knowing three normal damage of your Unarmed
    Strike or Monk such features: Flurry of Blows, Patient Defense, and weapons. This die changes as you
    gain Monk levels, Step of the Wind, each of which is detailed below. as shown in the Martial Arts
    column of the Monk When you expend a Focus Point, it is unavailable Features table. until you finish
    a Short or Long Rest, at the end of Dexterous Attacks. You can use your Dexterity which you regain
    all your expended points. modifier instead of your Strength modifier for the Some features that use
    Focus Points require your attack and damage rolls of your Unarmed Strikes target to make a saving
    throw. The save DC equals 8 and Monk weapons. In addition, when you use the plus your Wisdom modifier
    and Proficiency Bonus. Grapple or Shove option of your Unarmed Strike, Flurry of Blows. You can expend
    1 Focus Point to you can use your Dexterity modifier instead of your make two Unarmed Strikes as a
    Bonus Action. Strength modifier to determine the save DC. Patient Defense. You can take the Disengage
    action as a Bonus Action. Alternatively, you can 50 System Reference Document 5.2.1'
  sha256: c7101828b3629b4092a71431e970e46d4077d6686eb21b9812d1c3dfb9b948d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Monk Progression

From SRD 5.2.1 page 50.

     Monk Features
              Proficiency                                                       Martial       Focus     Unarmored
      Level     Bonus     Class Features                                         Arts         Points    Movement
        1         +2      Martial Arts, Unarmored Defense                         1d6          —             —
        2         +2      Monk’s Focus, Unarmored Movement,                       1d6           2          +10 ft.
                          Uncanny Metabolism
        3         +2      Deflect Attacks, Monk Subclass                          1d6           3          +10 ft.
        4         +2      Ability Score Improvement, Slow Fall                    1d6           4          +10 ft.
        5         +3      Extra Attack, Stunning Strike                           1d8           5          +10 ft.
        6         +3      Empowered Strikes, Subclass feature                     1d8           6          +15 ft.
        7         +3      Evasion                                                 1d8           7          +15 ft.
        8         +3      Ability Score Improvement                               1d8           8          +15 ft.
        9         +4      Acrobatic Movement                                      1d8           9          +15 ft.
       10         +4      Heightened Focus, Self-Restoration                      1d8          10          +20 ft.
       11         +4      Subclass feature                                       1d10          11          +20 ft.
       12         +4      Ability Score Improvement                              1d10          12          +20 ft.
       13         +5      Deflect Energy                                         1d10          13          +20 ft.
       14         +5      Disciplined Survivor                                   1d10          14          +25 ft.
       15         +5      Perfect Focus                                          1d10          15          +25 ft.
       16         +5      Ability Score Improvement                              1d10          16          +25 ft.
       17         +6      Subclass feature                                       1d12          17          +25 ft.
       18         +6      Superior Defense                                       1d12          18          +30 ft.
       19         +6      Epic Boon                                              1d12          19          +30 ft.
       20         +6      Body and Mind                                          1d12          20          +30 ft.


     Level 1: Martial Arts                                       Level 1: Unarmored Defense
     Your practice of martial arts gives you mastery of          While you aren’t wearing armor or wielding a
     combat styles that use your Unarmed Strike and              Shield, your base Armor Class equals 10 plus your
     Monk weapons, which are the following:                      Dexterity and Wisdom modifiers.
     • Simple Melee weapons                                      Level 2: Monk’s Focus
     • Martial Melee weapons that have the Light                 Your focus and martial training allow you to har-
       property                                                  ness a well of extraordinary energy within yourself.
     You gain the following benefits while you are un-           This energy is represented by Focus Points. Your
     armed or wielding only Monk weapons and you ar-             Monk level determines the number of points you
     en’t wearing armor or wielding a Shield.                    have, as shown in the Focus Points column of the
       Bonus Unarmed Strike. You can make an Un-                 Monk Features table.
     armed Strike as a Bonus Action.                               You can expend these points to enhance or fuel
       Martial Arts Die. You can roll 1d6 in place of the        certain Monk features. You start knowing three
     normal damage of your Unarmed Strike or Monk                such features: Flurry of Blows, Patient Defense, and
     weapons. This die changes as you gain Monk levels,          Step of the Wind, each of which is detailed below.
     as shown in the Martial Arts column of the Monk               When you expend a Focus Point, it is unavailable
     Features table.                                             until you finish a Short or Long Rest, at the end of
       Dexterous Attacks. You can use your Dexterity             which you regain all your expended points.
     modifier instead of your Strength modifier for the            Some features that use Focus Points require your
     attack and damage rolls of your Unarmed Strikes             target to make a saving throw. The save DC equals 8
     and Monk weapons. In addition, when you use the             plus your Wisdom modifier and Proficiency Bonus.
     Grapple or Shove option of your Unarmed Strike,               Flurry of Blows. You can expend 1 Focus Point to
     you can use your Dexterity modifier instead of your         make two Unarmed Strikes as a Bonus Action.
     Strength modifier to determine the save DC.                   Patient Defense. You can take the Disengage
                                                                 action as a Bonus Action. Alternatively, you can


50   System Reference Document 5.2.1
