---
splat_schema: 1
splat_id: 5d350a08-a081-5ac9-b116-d8e9d941d9fc
splat_kind: creature
splat_key: dnd2024:creatures/giant_rat
splat_name: Giant Rat
splat_origin:
  key: dnd2024:reference/creature/giant_rat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +2
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 7 (2d6)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
  dexterity:
    value: 16
  save_dexterity:
    value: 5
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 7
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_rat
  implementation: dnd2024:creature_giant_rat
  parameters:
    skill_totals:
      perception: 2
    senses:
    - sense: darkvision
      range_ft: 60
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
- id: dnd2024:creature_giant_rat
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
  source_sha256: 056b16ea46f71adddd4908b48fb77a5367dd620415d6581aaa4c2d1052a5a09c
  source_path: builtin/srd-5.2.1/creature/giant_rat.md
  pages:
  - 353
  engine_gaps: &id001
  - dnd2024:creature_giant_rat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 9243bb78d1c2c7434c04cfc1daa9d499b776dea85fa755060ce342ce5536cad4
splat_clauses:
- key: source_rules
  text: '# Giant Rat Small Beast, Unaligned AC 13 Initiative +3 (13) HP 7 (2d6) Speed 30 ft., Climb 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 16 +3 +5 Con 11 +0 +0 Int 2 -4 -4 Wis 10 +0 +0 Cha
    4 -3 -3 Skills Perception +2 Senses Darkvision 60 ft.; Passive Perception 12 Languages None CR 1/8
    (XP 25; PB +2) ### Traits Pack Tactics. The rat has Advantage on an attack roll against a creature
    if at least one of the rat’s allies is within 5 feet of the creature and the ally doesn’t have the
    Incapacitated condition. ### Actions Bite. Melee Attack Roll: +5, reach 5 feet. Hit: 5 (1d4 + 3) Piercing
    damage.'
  sha256: f87170bc20c02ac11ecd24a7b82f5d8482fbe1e7ad722f37a26905b5246dd4e1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Rat

<!-- SRD page 353 -->

Small Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 7 (2d6)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 16 +3 +5 Con 11 +0 +0\
Int 2 −4 −4 Wis 10 +0 +0 Cha 4 −3 −3\
Skills Perception +2\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Traits

Pack Tactics. The rat has Advantage on an attack roll\
against a creature if at least one of the rat’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 feet. Hit: 5 (1d4\
+ 3) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/353|page 353]].
