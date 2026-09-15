---
splat_schema: 1
splat_id: 642cf218-adec-5322-a5c7-2c40ca2cee34
splat_kind: creature
splat_key: dnd2024:creatures/reef_shark
splat_name: Reef Shark
splat_origin:
  key: dnd2024:reference/creature/reef_shark
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 30 ft.
  skills_text: Perception +2
  senses_text: Blindsight 30 ft.; Passive Perception 12
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (4d8 + 4)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/reef_shark
  implementation: dnd2024:creature_reef_shark
  parameters:
    skill_totals:
      perception: 2
    senses:
    - sense: blindsight
      range_ft: 30
    passive_perception: 12
    languages:
      speak: true
      known: []
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
      water_breathing:
        economy: passive
        op: breathing.allow
        media:
        - water
        exclusive: true
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 4
              add: 2
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
- id: dnd2024:creature_reef_shark
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
  source_sha256: 5cbf0d1a3eb0590c1b181807657a88db5de39a1557a9f24bac61dee798586281
  source_path: builtin/srd-5.2.1/creature/reef_shark.md
  pages:
  - 360
  engine_gaps: &id001
  - dnd2024:creature_reef_shark
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 98bd265cf8d45259c33e6fde4b3a1e17ce7d023e6972f84077ef39826c342592
splat_clauses:
- key: source_rules
  text: '# Reef Shark Medium Beast, Unaligned AC 12 Initiative +2 (12) HP 22 (4d8 + 4) Speed 5 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1 Int 1 -5 -5 Wis 10 +0 +0
    Cha 4 -3 -3 Skills Perception +2 Senses Blindsight 30 ft.; Passive Perception 12 Languages None CR
    1/2 (XP 100; PB +2) ### Traits Pack Tactics. The shark has Advantage on an attack roll against a creature
    if at least one of the shark’s allies is within 5 feet of the creature and the ally doesn’t have the
    Incapacitated condition. Water Breathing. The shark can breathe only underwater. ### Actions Bite.
    Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2) Piercing damage.'
  sha256: 53c95f92b37a20bdffcb82126776bd5664515257eb71b28648dcbf30b36eb26a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reef Shark

<!-- SRD page 360 -->

Medium Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 22 (4d8 + 4)\
Speed 5 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +2 Dex 15 +2 +2 Con 13 +1 +1\
Int 1 −5 −5 Wis 10 +0 +0 Cha  4 −3 −3\
Skills Perception +2\
Senses Blindsight 30 ft.; Passive Perception 12\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Pack Tactics. The shark has Advantage on an attack roll\
against a creature if at least one of the shark’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\
Water Breathing. The shark can breathe only underwater.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/360|page 360]].
