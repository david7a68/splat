---
splat_schema: 1
splat_id: 419b96b5-267e-51fa-83d8-2337a760cc3b
splat_kind: creature
splat_key: dnd2024:creatures/giant_weasel
splat_name: Giant Weasel
splat_origin:
  key: dnd2024:reference/creature/giant_weasel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 30 ft.
  skills_text: Acrobatics +5, Perception +3, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 9 (2d8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 9
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_weasel
  implementation: dnd2024:creature_giant_weasel
  parameters:
    skill_totals:
      acrobatics: 5
      perception: 3
      stealth: 5
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 13
    languages:
      speak: true
      known: []
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
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
                count: 1
                sides: 4
              add: 3
            average: 5
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
- id: dnd2024:creature_giant_weasel
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
  source_sha256: 681bd64afe9baad45bee00746ca908113ee109c3672cf554a68f546197f6bac0
  source_path: builtin/srd-5.2.1/creature/giant_weasel.md
  pages:
  - 355
  engine_gaps: &id001
  - dnd2024:creature_giant_weasel
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/giant_weasel
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ed6b2e1c08cfa5a22e8cde99ffb200f55f8396f56dd5ca871d401f02f0e6e75f
splat_clauses:
- key: source_rules
  text: '# Giant Weasel Medium Beast, Unaligned AC 13 Initiative +3 (13) HP 9 (2d8) Speed 40 ft., Climb
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 17 +3 +3 Con 10 +0 +0 Int 4 -3 -3 Wis 12 +1 +1
    Cha 5 -3 -3 Skills Acrobatics +5, Perception +3, Stealth +5 Senses Darkvision 60 ft.; Passive Perception
    13 Languages None CR 1/8 (XP 25; PB +2) ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit:
    5 (1d4 + 3) Piercing damage.'
  sha256: c763fe71846f795bad8a279c058653aabe16b6050ec4ddc4ac961616b3478495
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Weasel

<!-- SRD page 355 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 9 (2d8)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 17 +3 +3 Con 10 +0 +0\
Int 4 −3 −3 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Acrobatics +5, Perception +3, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]].
