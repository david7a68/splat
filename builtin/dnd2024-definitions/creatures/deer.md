---
splat_schema: 1
splat_id: ea02841c-a983-5671-9474-5f651ca54665
splat_kind: creature
splat_key: dnd2024:creatures/deer
splat_name: Deer
splat_origin:
  key: dnd2024:reference/creature/deer
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +4
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 4 (1d8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
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
    value: 13
  max_hp:
    value: 4
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/deer
  implementation: dnd2024:creature_deer
  parameters:
    skill_totals:
      perception: 4
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 14
    languages:
      speak: true
      known: []
    features:
      agile:
        economy: passive
        op: opportunity_attack.suppress
        trigger: actor.leaves_enemy_reach
        movement_modes:
        - all
      ram:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 2
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
                sides: 4
              add: 0
            average: 2
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
- id: dnd2024:creature_deer
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
  - actor.leaves_enemy_reach
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
  source_sha256: 72967b5fd81ae18549dc67aa06326b821d18a8bc7fb13dc745552ebca67659b7
  source_path: builtin/srd-5.2.1/creature/deer.md
  pages:
  - 347
  engine_gaps: &id001
  - dnd2024:creature_deer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 582a1b1a4c9fadccb3e5aa4ff517fb1d1ef6f749ba00ef522e82bce11a9b1bbb
splat_clauses:
- key: source_rules
  text: '# Deer Medium Beast, Unaligned AC 13 Initiative +3 (13) HP 4 (1d8) Speed 50 ft. MOD SAVE MOD
    SAVE MOD SAVE Str 11 +0 +0 Dex 16 +3 +3 Con 11 +0 +0 Int 2 -4 -4 Wis 14 +2 +2 Cha 5 -3 -3 Skills Perception
    +4 Senses Darkvision 60 ft.; Passive Perception 14 Languages None CR 0 (XP 10; PB +2) ### Traits Agile.
    The deer doesn’t provoke an Opportunity Attack when it moves out of an enemy’s reach. ### Actions
    Ram. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Bludgeoning damage.'
  sha256: 81ba49f0181c3f5a754d980bded20b123a4f58aa08b87d2e93a2cbd93894cc3c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Deer

<!-- SRD page 347 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 4 (1d8)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 16 +3 +3 Con 11 +0 +0\
Int 2 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
Skills Perception +4\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Agile. The deer doesn’t provoke an Opportunity Attack\
when it moves out of an enemy’s reach.\

### Actions

Ram. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]].
