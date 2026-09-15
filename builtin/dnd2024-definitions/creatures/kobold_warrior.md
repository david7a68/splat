---
splat_schema: 1
splat_id: 8ceed437-0cda-5a4c-8c01-dd70cebae23e
splat_kind: creature
splat_key: dnd2024:creatures/kobold_warrior
splat_name: Kobold Warrior
splat_origin:
  key: dnd2024:reference/creature/kobold_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 302
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: dragon
  alignment: Neutral
  movement_text: 30 ft.
  gear_text: Daggers (3)
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: Common, Draconic
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 7 (3d6 - 3)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 9
  save_constitution:
    value: -1
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 14
  max_hp:
    value: 7
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/kobold_warrior
  implementation: dnd2024:creature_kobold_warrior
  parameters:
    gear:
    - item: daggers
      count: 3
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 8
    languages:
      speak: true
      known:
      - common
      - draconic
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
      sunlight_sensitivity:
        economy: passive
        op: roll.disadvantage
        when:
          in_sunlight: true
        rolls:
        - ability_check
        - attack
      dagger:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 20
          long: 60
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 4
              add: 2
            average: 4
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
- id: dnd2024:creature_kobold_warrior
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
  source_sha256: f5a4e840a790ec86586304ec3141f6bd51e02a1da013274aa35e58ff7f8dafff
  source_path: builtin/srd-5.2.1/creature/kobold_warrior.md
  pages:
  - 302
  engine_gaps: &id001
  - dnd2024:creature_kobold_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 22b2ec62998339a3dcbaac4f194550a3c596f1a987e717f187e8e83e75f3f9b2
splat_clauses:
- key: source_rules
  text: '# Kobold Warrior Small Dragon, Neutral AC 14 Initiative +2 (12) HP 7 (3d6 - 3) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 15 +2 +2 Con 9 -1 -1 Int 8 -1 -1 Wis 7 -2 -2 Cha 8 -1 -1 Gear
    Daggers (3) Senses Darkvision 60 ft.; Passive Perception 8 Languages Common, Draconic CR 1/8 (XP 25;
    PB +2) ### Traits Pack Tactics. The kobold has Advantage on an attack roll against a creature if at
    least one of the kobold’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated
    condition. Sunlight Sensitivity. While in sunlight, the kobold has Disadvantage on ability checks
    and attack rolls. ### Actions Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or range 20/60
    ft. Hit: 4 (1d4 + 2) Piercing damage.'
  sha256: cb7c94a405487e267fbf4ff1588ae4765ae1cea6c1e2621e7aaa7a46b60a159f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Kobold Warrior

<!-- SRD page 302 -->

Small Dragon, Neutral\
AC 14 Initiative +2 (12)\
HP 7 (3d6 − 3)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 15 +2 +2 Con 9 −1 −1\
Int 8 −1 −1 Wis 7 −2 −2 Cha 8 −1 −1\
Gear Daggers (3)\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Common, Draconic\
CR 1/8 (XP 25; PB +2)\

### Traits

Pack Tactics. The kobold has Advantage on an attack\
roll against a creature if at least one of the kobold’s al-\
lies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\
Sunlight Sensitivity. While in sunlight, the kobold has\
Disadvantage on ability checks and attack rolls.\

### Actions

Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or\
range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/302|page 302]].
