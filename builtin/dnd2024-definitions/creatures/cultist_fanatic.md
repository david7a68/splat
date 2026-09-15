---
splat_schema: 1
splat_id: 49c2f231-18d1-5fb3-bc11-02824f17e924
splat_kind: creature
splat_key: dnd2024:creatures/cultist_fanatic
splat_name: Cultist Fanatic
splat_origin:
  key: dnd2024:reference/creature/cultist_fanatic
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Deception +3, Persuasion +3, Religion +2
  gear_text: Holy Symbol, Leather Armor
  senses_text: Passive Perception 12
  languages_text: Common
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 44 (8d8 + 8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 14
  save_dexterity:
    value: 2
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
    value: 4
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 13
  max_hp:
    value: 44
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/cultist_fanatic
  implementation: dnd2024:creature_cultist_fanatic
  parameters:
    skill_totals:
      deception: 3
      persuasion: 3
      religion: 2
    gear:
    - item: holy_symbol
      count: 1
    - item: leather_armor
      count: 1
    senses: []
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
    features:
      pact_blade:
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
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 8
              add: 2
            average: 6
          - type: necrotic
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
          - spell:
              key: dnd2024:spells/command
            uses:
              count: 2
              recovery: long_rest
          - spell:
              key: dnd2024:spells/hold_person
            uses:
              count: 1
              recovery: long_rest
          save_dc: 12
          attack_bonus: 4
      spiritual_weapon:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: same_as_spellcasting
          waived_components: []
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/spiritual_weapon
        uses:
          count: 2
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
- id: dnd2024:creature_cultist_fanatic
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
  source_sha256: b58ce8c70af081d65f938d79d6804d5056f7f3f03442ec156b225e39e47e050c
  source_path: builtin/srd-5.2.1/creature/cultist_fanatic.md
  pages:
  - 278
  engine_gaps: &id001
  - dnd2024:creature_cultist_fanatic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ee04c06c8c3886bdfca7d257a272ff8d1836b0f3ab676b3de0338c7e3e650e02
splat_clauses:
- key: source_rules
  text: '# Cultist Fanatic Medium or Small Humanoid, Neutral AC 13 Initiative +2 (12) HP 44 (8d8 + 8)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1 Int 10 +0 +0 Wis 14
    +2 +4 Cha 13 +1 +1 Skills Deception +3, Persuasion +3, Religion +2 Gear Holy Symbol, Leather Armor
    Senses Passive Perception 12 Languages Common CR 2 (XP 450; PB +2) ### Actions Pact Blade. Melee Attack
    Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage plus 7 (2d6) Necrotic damage. Spellcasting.
    The cultist casts one of the following spells, using Wisdom as the spellcasting ability (spell save
    DC 12, +4 to hit with spell attacks): At Will: Light, Thaumaturgy 2/Day: Command 1/Day: Hold Person
    ### Bonus Actions Spiritual Weapon (2/Day). The cultist casts the Spiritual Weapon spell, using the
    same spellcasting ability as Spellcasting.'
  sha256: 8082b4fdd4ca37969f569279ea67888ab43f1f142101149bcb3da7cbe52074dc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cultist Fanatic

<!-- SRD page 278 -->

Medium or Small Humanoid, Neutral\
AC 13 Initiative +2 (12)\
HP 44 (8d8 + 8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
Int 10 +0 +0 Wis 14 +2 +4 Cha 13 +1 +1\
Skills Deception +3, Persuasion +3, Religion +2\
Gear Holy Symbol, Leather Armor\
Senses Passive Perception 12\
Languages Common\
CR 2 (XP 450; PB +2)\

### Actions

Pact Blade. Melee Attack Roll: +4, reach 5 ft.\
Hit: 6 (1d8 + 2) Slashing damage plus 7 (2d6) Ne-\
crotic damage.\
Spellcasting. The cultist casts one of the following\
spells, using Wisdom as the spellcasting ability (spell\
save DC 12, +4 to hit with spell attacks):\
At Will: Light, Thaumaturgy\
2/Day: Command\
1/Day: Hold Person\

### Bonus Actions

Spiritual Weapon (2/Day). The cultist casts the Spiritual\
Weapon spell, using the same spellcasting ability as\
Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/278|page 278]].
