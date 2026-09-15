---
splat_schema: 1
splat_id: c69dc034-79a5-5200-8bc8-1c29a6586988
splat_kind: creature
splat_key: dnd2024:creatures/tough
splat_name: Tough
splat_origin:
  key: dnd2024:reference/creature/tough
splat_sources:
- title: SRD 5.2.1
  locator: pp. 332
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Heavy Crossbow, Leather Armor, Mace
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 32 (5d8 + 10)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 32
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/tough
  implementation: dnd2024:creature_tough
  parameters:
    gear:
    - item: heavy_crossbow
      count: 1
    - item: leather_armor
      count: 1
    - item: mace
      count: 1
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known:
      - common
    features:
      pack_tactics:
        economy: passive
        op: roll.advantage
        rolls:
        - attack
        target_kind: creature
        when:
          ally_within_target_ft: 5
          ally_excluded_conditions:
          - incapacitated
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
      heavy_crossbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 100
          long: 400
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              add: 1
            average: 6
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
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
- id: dnd2024:creature_tough
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9d829914ffb3371f2bbff6228013c3d3bb0d714e35adaf0981354417bba1e845
  source_path: builtin/srd-5.2.1/creature/tough.md
  pages:
  - 332
  engine_gaps: &id001
  - dnd2024:creature_tough
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: bc270700f24c26fb85dcc1435ee96c54157e9bb654e1f0582638c1759677affd
splat_clauses:
- key: source_rules
  text: '# Tough Medium or Small Humanoid, Neutral AC 12 Initiative +1 (11) HP 32 (5d8 + 10) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2 Int 10 +0 +0 Wis 10 +0 +0 Cha
    11 +0 +0 Gear Heavy Crossbow, Leather Armor, Mace Senses Passive Perception 10 Languages Common CR
    1/2 (XP 100; PB +2) ### Traits Pack Tactics. The tough has Advantage on an attack roll against a creature
    if at least one of the tough’s allies is within 5 feet of the creature and the ally doesn’t have the
    Incapacitated condition. ### Actions Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Bludgeoning
    damage. Heavy Crossbow. Ranged Attack Roll: +3, range 100/400 ft. Hit: 6 (1d10 + 1) Piercing damage.'
  sha256: 5f4ab58fe589652a49c5f35236457f40baaac689b172ee78251c56a050cb03c1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tough

<!-- SRD page 332 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 32 (5d8 + 10)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2\
Int 10 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
Gear Heavy Crossbow, Leather Armor, Mace\
Senses Passive Perception 10\
Languages Common\
CR 1/2 (XP 100; PB +2)\

### Traits

Pack Tactics. The tough has Advantage on an attack\
roll against a creature if at least one of the tough’s allies\
is within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Mace. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Bludgeoning damage.\
Heavy Crossbow. Ranged Attack Roll: +3, range\
100/400 ft. Hit: 6 (1d10 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/332|page 332]].
