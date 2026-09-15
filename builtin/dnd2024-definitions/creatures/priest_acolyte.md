---
splat_schema: 1
splat_id: 7d5560c6-5f2f-5d25-90b2-f22462ec433f
splat_kind: creature
splat_key: dnd2024:creatures/priest_acolyte
splat_name: Priest Acolyte
splat_origin:
  key: dnd2024:reference/creature/priest_acolyte
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  tags_text: (Cleric)
  movement_text: 30 ft.
  skills_text: Medicine +4, Religion +2
  gear_text: Chain Shirt, Holy Symbol, Mace
  senses_text: Passive Perception 12
  languages_text: Common
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 11
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/priest_acolyte
  implementation: dnd2024:creature_priest_acolyte
  parameters:
    skill_totals:
      medicine: 4
      religion: 2
    gear:
    - item: chain_shirt
      count: 1
    - item: holy_symbol
      count: 1
    - item: mace
      count: 1
    senses: []
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
    features:
      mace:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            average: 5
          - type: radiant
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
      radiant_flame:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 60
          long: 60
        hit:
        - op: damage
          components:
          - type: radiant
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: wisdom
          waived_components: []
          spells:
          - spell:
              key: dnd2024:spells/light
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/thaumaturgy
            uses:
              unlimited: true
      divine_aid:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: wisdom
          waived_components: []
          save_dc: 12
          ability_modifier: 2
          derived_dc:
            base: 8
            ability_modifier: 2
            proficiency_bonus: 2
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/bless
        - op: spell.cast
          spell:
            key: dnd2024:spells/healing_word
        - op: spell.cast
          spell:
            key: dnd2024:spells/sanctuary
        uses:
          count: 1
          recovery: long_rest
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_priest_acolyte
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 73876d2aa5226fdc599402ce90424de870561c86f9e98fd41806b61086440929
  source_path: builtin/srd-5.2.1/creature/priest_acolyte.md
  pages:
  - 316
  engine_gaps: &id001
  - dnd2024:creature_priest_acolyte
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 3b0c4b7a6c29e14004e62e4e6c5b51b415da6cfc90ff96cf1372178287445365
splat_clauses:
- key: source_rules
  text: '# Priest Acolyte Medium or Small Humanoid (Cleric), Neutral AC 13 Initiative +0 (10) HP 11 (2d8
    + 2) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 10 +0 +0 Con 12 +1 +1 Int 10 +0 +0 Wis
    14 +2 +2 Cha 11 +0 +0 Skills Medicine +4, Religion +2 Gear Chain Shirt, Holy Symbol, Mace Senses Passive
    Perception 12 Languages Common CR 1/4 (XP 50; PB +2) ### Actions Mace. Melee Attack Roll: +4, reach
    5 ft. Hit: 5 (1d6 + 2) Bludgeoning damage plus 2 (1d4) Radiant damage. Radiant Flame. Ranged Attack
    Roll: +4, range 60 ft. Hit: 7 (2d6) Radiant damage. Spellcasting. The priest casts one of the following
    spells, using Wisdom as the spellcasting ability: At Will: Light, Thaumaturgy ### Bonus Actions Divine
    Aid (1/Day). The priest casts Bless, Healing Word, or Sanctuary, using the same spellcasting ability
    as Spellcasting.'
  sha256: 5360d0149033a1e3dbe88980c6a28902bb405e0acd235a4f425472f958c4956e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Priest Acolyte

<!-- SRD page 316 -->

Medium or Small Humanoid (Cleric), Neutral\
AC 13 Initiative +0 (10)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 10 +0 +0 Con 12 +1 +1\
Int 10 +0 +0 Wis 14 +2 +2 Cha 11 +0 +0\
Skills Medicine +4, Religion +2\
Gear Chain Shirt, Holy Symbol, Mace\
Senses Passive Perception 12\
Languages Common\
CR 1/4 (XP 50; PB +2)\

### Actions

Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Bludgeoning damage plus 2 (1d4) Radiant damage.\
Radiant Flame. Ranged Attack Roll: +4, range 60 ft.\
Hit: 7 (2d6) Radiant damage.\
Spellcasting. The priest casts one of the following\
spells, using Wisdom as the spellcasting ability:\
At Will: Light, Thaumaturgy\

### Bonus Actions

Divine Aid (1/Day). The priest casts Bless, Healing\
Word, or Sanctuary, using the same spellcasting ability\
as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/316|page 316]].
