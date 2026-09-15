---
splat_schema: 1
splat_id: 15457a07-7534-57dd-8d2d-e338cc32d8bc
splat_kind: creature
splat_key: dnd2024:creatures/giant_centipede
splat_name: Giant Centipede
splat_origin:
  key: dnd2024:reference/creature/giant_centipede
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349, 350
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  senses_text: Blindsight 30 ft.; Passive Perception 8
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 9 (2d6 + 2)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 14
  max_hp:
    value: 9
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
  mechanic: dnd2024:creatures/giant_centipede
  implementation: dnd2024:creature_giant_centipede
  parameters:
    senses:
    - sense: blindsight
      range_ft: 30
    passive_perception: 8
    languages:
      speak: true
      known: []
    features:
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
                count: 1
                sides: 4
              add: 2
            average: 4
        - op: condition.apply
          condition: poisoned
          duration:
            until: actor.next_turn_start
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
- id: dnd2024:creature_giant_centipede
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
  source_sha256: 3b822dd4f34e0f993ca895063f9bd27bfe5f1cfa0073cf7c3db416875833f9d1
  source_path: builtin/srd-5.2.1/creature/giant_centipede.md
  pages:
  - 349
  - 350
  engine_gaps: &id001
  - dnd2024:creature_giant_centipede
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 0b378374972d00db6e19ccfbfc039d95c3dfd60862cd0d604697f7d00eb2006c
splat_clauses:
- key: source_rules
  text: '# Giant Centipede Small Beast, Unaligned AC 14 Initiative +2 (12) HP 9 (2d6 + 2) Speed 30 ft.,
    Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex14 +2 +2 Con 12 +1 +1 Int 1 -5 -5 Wis7 -2 -2
    Cha 3 -4 -4 Senses Blindsight 30 ft.; Passive Perception 8 Languages None CR 1/4 (XP 50; PB +2) ###
    Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, and the target
    has the Poisoned condition until the start of the centipede’s next turn.'
  sha256: f6690062cccd1173d185daf655a25e67980f4e4f9e999ce39b73fcf896e89950
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Centipede

<!-- SRD page 349 -->

Small Beast, Unaligned\
AC 14  Initiative +2 (12)\
HP 9 (2d6 + 2)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 350 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex14 +2 +2 Con 12 +1 +1\
Int 1 −5 −5 Wis7 −2 −2 Cha 3 −4 −4\
Senses Blindsight 30 ft.; Passive Perception 8\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Piercing damage, and the target has the Poisoned con-\
dition until the start of the centipede’s next turn.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]], [[../../srd-5.2.1/pages/350|page 350]].
