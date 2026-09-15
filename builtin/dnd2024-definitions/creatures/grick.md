---
splat_schema: 1
splat_id: a12c9816-5379-5649-92c8-960c7dbf2989
splat_kind: creature
splat_key: dnd2024:creatures/grick
splat_name: Grick
splat_origin:
  key: dnd2024:reference/creature/grick
splat_sources:
- title: SRD 5.2.1
  locator: pp. 295
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: aberration
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Stealth +4
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 54 (12d8)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 54
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/grick
  implementation: dnd2024:creature_grick
  parameters:
    skill_totals:
      stealth: 4
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 12
    languages:
      speak: true
      known: []
    features:
      beak:
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
                sides: 6
              add: 2
            average: 9
      tentacles:
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
                sides: 10
              add: 2
            average: 7
        - op: grapple
          escape_dc: 12
          when:
            target_kind: creature
            target_maximum_size: medium
          limb_slots: 4
          slots_per_target: 4
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 1
          feature: beak
        - count: 1
          feature: tentacles
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
- id: dnd2024:creature_grick
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
  source_sha256: c3fea9187cdc936f6440ef46d271a4964ace3d79b234c1b53485701bd5f80657
  source_path: builtin/srd-5.2.1/creature/grick.md
  pages:
  - 295
  engine_gaps: &id001
  - dnd2024:creature_grick
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4a49ae3e689fff36da1bab5a822c2ae7d46be8759c62dd34c1ceef0eafaf15e0
splat_clauses:
- key: source_rules
  text: '# Grick Medium Aberration, Unaligned AC 14 Initiative +2 (12) HP 54 (12d8) Speed 30 ft., Climb
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 14 +2 +2 Con 11 +0 +0 Int 3 -4 -4 Wis 14 +2 +2
    Cha 5 -3 -3 Skills Stealth +4 Senses Darkvision 60 ft.; Passive Perception 12 Languages None CR 2
    (XP 450; PB +2) ### Actions Multiattack. The grick makes one Beak attack and one Tentacles attack.
    Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Piercing damage. Tentacles. Melee Attack
    Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 12) from all four tentacles.'
  sha256: 69624188a1d14bebc004d4df019ea3e434f274e48474e65e4e9168b851da217d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grick

<!-- SRD page 295 -->

Medium Aberration, Unaligned\
AC 14 Initiative +2 (12)\
HP 54 (12d8)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 14 +2 +2 Con 11 +0 +0\
Int 3 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
Skills Stealth +4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The grick makes one Beak attack and one\
Tentacles attack.\
Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2)\
Piercing damage.\
Tentacles. Melee Attack Roll: +4, reach 5 ft. Hit: 7\
(1d10 + 2) Slashing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 12) from all four tentacles.\

Original layout: [[../../srd-5.2.1/pages/295|page 295]].
