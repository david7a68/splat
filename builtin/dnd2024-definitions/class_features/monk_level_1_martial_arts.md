---
splat_schema: 1
splat_id: 96942747-6e7d-5727-af60-b822a8017e8e
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_1_martial_arts
splat_name: 'Level 1: Martial Arts'
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_1_martial_arts
  parameters:
    monk_weapon_categories:
    - category: simple
      range: melee
    - category: martial
      range: melee
      requires_property: light
    benefit_requirements:
      armor_worn: false
      shield_wielded: false
      weapons_wielded: none_or_only_monk_weapons
    bonus_unarmed_strike:
      action: bonus_action
      count: 1
    damage_die_substitution:
      optional: true
      attacks:
      - unarmed_strike
      - monk_weapon
      dice_by_monk_level:
        '1':
          dice:
            count: 1
            sides: 6
        '5':
          dice:
            count: 1
            sides: 8
        '11':
          dice:
            count: 1
            sides: 10
        '17':
          dice:
            count: 1
            sides: 12
    dexterity_substitution:
      optional: true
      replaces: strength
      for:
      - unarmed_strike_attack
      - unarmed_strike_damage
      - monk_weapon_attack
      - monk_weapon_damage
      - unarmed_grapple_save_dc
      - unarmed_shove_save_dc
  implementation: dnd2024:class_features_monk_level_1_martial_arts
splat_contracts:
- id: dnd2024:class_features_monk_level_1_martial_arts
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  events:
  - attack.pending
  - action.bonus
  capabilities:
  - attacks
  - ability_substitution
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 50
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_1_martial_arts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a902461b9cdc3a55f375ba039ba25303846bcb47d029325524fd2f816a26357c
  parent_name: Monk
  line_span:
    first: 4148
    count: 46
splat_clauses:
- key: source_rules
  text: '# Level 1: Martial Arts Your practice of martial arts gives you mastery of combat styles that
    use your Unarmed Strike and Monk weapons, which are the following: • Simple Melee weapons • Martial
    Melee weapons that have the Light property You gain the following benefits while you are unarmed or
    wielding only Monk weapons and you aren’t wearing armor or wielding a Shield. Bonus Unarmed Strike.
    You can make an Unarmed Strike as a Bonus Action. Martial Arts Die. You can roll 1d6 in place of the
    normal damage of your Unarmed Strike or Monk weapons. This die changes as you gain Monk levels, as
    shown in the Martial Arts column of the Monk Features table. Dexterous Attacks. You can use your Dexterity
    modifier instead of your Strength modifier for the attack and damage rolls of your Unarmed Strikes
    and Monk weapons. In addition, when you use the Grapple or Shove option of your Unarmed Strike, you
    can use your Dexterity modifier instead of your Strength modifier to determine the save DC. Martial
    Focus Unarmored Arts Points Movement 1d6 — — 1d6 2 +10 ft. 1d6 3 +10 ft. 1d6 4 +10 ft. 1d8 5 +10 ft.
    1d8 6 +15 ft. 1d8 7 +15 ft. 1d8 8 +15 ft. 1d8 9 +15 ft. 1d8 10 +20 ft. 1d10 11 +20 ft. 1d10 12 +20
    ft. 1d10 13 +20 ft. 1d10 14 +25 ft. 1d10 15 +25 ft. 1d10 16 +25 ft. 1d12 17 +25 ft. 1d12 18 +30 ft.
    1d12 19 +30 ft. 1d12 20 +30 ft.'
  sha256: d20fb39216c3f0e4e494f4e8404f5e99e6ca2bdfc3782e4afa37d54261b2e931
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Martial Arts

<!-- SRD page 50 -->

Your practice of martial arts gives you mastery of\
combat styles that use your Unarmed Strike and\
Monk weapons, which are the following:\
•  Simple Melee weapons\
•  Martial Melee weapons that have the Light\
property\
You gain the following benefits while you are un-\
armed or wielding only Monk weapons and you ar-\
en’t wearing armor or wielding a Shield.\
Bonus Unarmed Strike. You can make an Un-\
armed Strike as a Bonus Action.\
Martial Arts Die. You can roll 1d6 in place of the\
normal damage of your Unarmed Strike or Monk\
weapons. This die changes as you gain Monk levels,\
as shown in the Martial Arts column of the Monk\
Features table.\
Dexterous Attacks. You can use your Dexterity\
modifier instead of your Strength modifier for the\
attack and damage rolls of your Unarmed Strikes\
and Monk weapons. In addition, when you use the\
Grapple or Shove option of your Unarmed Strike,\
you can use your Dexterity modifier instead of your\
Strength modifier to determine the save DC.\
Martial Focus Unarmored\
Arts Points Movement\
1d6 — —\
1d6 2 +10 ft.\
1d6 3 +10 ft.\
1d6 4 +10 ft.\
1d8 5 +10 ft.\
1d8 6 +15 ft.\
1d8 7 +15 ft.\
1d8 8 +15 ft.\
1d8 9 +15 ft.\
1d8 10 +20 ft.\
1d10 11 +20 ft.\
1d10 12 +20 ft.\
1d10 13 +20 ft.\
1d10 14 +25 ft.\
1d10 15 +25 ft.\
1d10 16 +25 ft.\
1d12 17 +25 ft.\
1d12 18 +30 ft.\
1d12 19 +30 ft.\
1d12 20 +30 ft.\
