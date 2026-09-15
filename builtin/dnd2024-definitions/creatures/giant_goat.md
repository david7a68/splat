---
splat_schema: 1
splat_id: d51ed266-40a6-5815-a688-fc8ff8b17603
splat_kind: creature
splat_key: dnd2024:creatures/giant_goat
splat_name: Giant Goat
splat_origin:
  key: dnd2024:reference/creature/giant_goat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 351
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 30 ft.
  skills_text: Perception +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 5
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_goat
  implementation: dnd2024:creature_giant_goat
  parameters:
    skill_totals:
      perception: 3
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 13
    languages:
      speak: true
      known: []
    features:
      ram:
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
          - type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 6
              add: 3
            average: 6
        - op: conditional
          when:
            target_maximum_size: large
            actor_moved_straight_toward_target_ft_at_least: 20
            immediately_before_hit: true
            target_kind: creature
          effects:
          - op: damage
            components:
            - type: bludgeoning
              amount:
                dice:
                  count: 2
                  sides: 4
                add: 0
              average: 5
          - op: condition.apply
            condition: prone
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
- id: dnd2024:creature_giant_goat
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
  source_sha256: d5d1f9d7761e7b6da1486aa6ba2d8b298da306f616e701ab535e3daaec6ea695
  source_path: builtin/srd-5.2.1/creature/giant_goat.md
  pages:
  - 351
  engine_gaps: &id001
  - dnd2024:creature_giant_goat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 88bccba4089b90d18fa46359730d59e71c116378c2186998e4acd3c6b7fcb16a
splat_clauses:
- key: source_rules
  text: '# Giant Goat Large Beast, Unaligned AC 11 Initiative +1 (11) HP 19 (3d10 + 3) Speed 40 ft., Climb
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +5 Dex 13 +1 +1 Con 12 +1 +1 Int 3 -4 -4 Wis 12 +1 +1
    Cha 6 -2 -2 Skills Perception +3 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR
    1/2 (XP 100; PB +2) ### Actions Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning
    damage. If the target is a Large or smaller creature and the goat moved 20+ feet straight toward it
    immediately before the hit, the target takes an extra 5 (2d4) Bludgeoning damage and has the Prone
    condition.'
  sha256: 5a3fa3c1b5b9936095378b0d5a146ced2ba62fffa9e86fb8427fc80bf895d8b2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Goat

<!-- SRD page 351 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +5 Dex 13 +1 +1 Con 12 +1 +1\
Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Bludgeoning damage. If the target is a Large or smaller\
creature and the goat moved 20+ feet straight toward it\
immediately before the hit, the target takes an extra 5\
(2d4) Bludgeoning damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/351|page 351]].
