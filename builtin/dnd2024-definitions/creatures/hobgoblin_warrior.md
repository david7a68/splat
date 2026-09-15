---
splat_schema: 1
splat_id: 7144c6ca-61be-5129-83eb-ffc371f30da6
splat_kind: creature
splat_key: dnd2024:creatures/hobgoblin_warrior
splat_name: Hobgoblin Warrior
splat_origin:
  key: dnd2024:reference/creature/hobgoblin_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298, 299
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Lawful Evil
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  gear_text: Half Plate Armor, Longbow, Longsword, Shield
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common, Goblin
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 18
  max_hp:
    value: 11
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hobgoblin_warrior
  implementation: dnd2024:creature_hobgoblin_warrior
  parameters:
    gear:
    - item: half_plate_armor
      count: 1
    - item: longbow
      count: 1
    - item: longsword
      count: 1
    - item: shield
      count: 1
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known:
      - common
      - goblin
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
      longsword:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 3
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
                count: 2
                sides: 10
              add: 1
            average: 12
      longbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 150
          long: 600
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 1
            average: 5
          - type: poison
            amount:
              dice:
                count: 3
                sides: 4
              add: 0
            average: 7
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
- id: dnd2024:creature_hobgoblin_warrior
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
  source_sha256: a912d11ca9dca9e6115e17e10392f6c789fb05bd1ee0c1b6005702988df4b6c9
  source_path: builtin/srd-5.2.1/creature/hobgoblin_warrior.md
  pages:
  - 298
  - 299
  engine_gaps: &id001
  - dnd2024:creature_hobgoblin_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a782bae8860f1b8dcedf09b7d9b4c841081e0d07677bb36cf41a97303ad48fff
splat_clauses:
- key: source_rules
  text: '# Hobgoblin Warrior Medium Fey (Goblinoid), Lawful Evil AC 18 Initiative +3 (13) HP 11 (2d8 +
    2) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1 Int 10 +0 +0 Wis
    10 +0 +0 Cha 9 -1 -1 Gear Half Plate Armor, Longbow, Longsword, Shield Senses Darkvision 60 ft.; Passive
    Perception 10 Languages Common, Goblin CR 1/2 (XP 100; PB +2) ### Traits Pack Tactics. The hobgoblin
    has Advantage on an attack roll against a creature if at least one of the hobgoblin’s allies is within
    5 feet of the creature and the ally doesn’t have the Incapacitated condition. ### Actions Longsword.
    Melee Attack Roll: +3, reach 5 ft. Hit: 12 (2d10 + 1) Slashing damage. Longbow. Ranged Attack Roll:
    +3, range 150/600 ft. Hit: 5 (1d8 + 1) Piercing damage plus 7 (3d4) Poison damage.'
  sha256: 475ac0b3d7b62b5b400977c1f8ceb0d45133c96179146acef622a241ec9a1670
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hobgoblin Warrior

<!-- SRD page 298 -->

Medium Fey (Goblinoid), Lawful Evil\
AC 18 Initiative +3 (13)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 10 +0 +0 Cha 9 −1 −1\
Gear Half Plate Armor, Longbow, Longsword, Shield\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 1/2 (XP 100; PB +2)\

<!-- SRD page 299 -->


### Traits

Pack Tactics. The hobgoblin has Advantage on an at-\
tack roll against a creature if at least one of the hobgob-\
lin’s allies is within 5 feet of the creature and the ally\
doesn’t have the Incapacitated condition.\

### Actions

Longsword. Melee Attack Roll: +3, reach 5 ft. Hit: 12\
(2d10 + 1) Slashing damage.\
Longbow. Ranged Attack Roll: +3, range 150/600 ft.\
Hit: 5 (1d8 + 1) Piercing damage plus 7 (3d4) Poi-\
son damage.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]], [[../../srd-5.2.1/pages/299|page 299]].
