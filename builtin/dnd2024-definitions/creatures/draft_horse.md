---
splat_schema: 1
splat_id: d56719fd-ca46-59a2-afb6-162639274fc9
splat_kind: creature
splat_key: dnd2024:creatures/draft_horse
splat_name: Draft Horse
splat_origin:
  key: dnd2024:reference/creature/draft_horse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347, 348
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 15 (2d10 + 4)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 10
  max_hp:
    value: 15
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/draft_horse
  implementation: dnd2024:creature_draft_horse
  parameters:
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      hooves:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
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
              add: 4
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
- id: dnd2024:creature_draft_horse
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
  source_sha256: 9f25316aef5cd9c54f6fed923c8490921794ce30e78a750c85bb029bbea0904b
  source_path: builtin/srd-5.2.1/creature/draft_horse.md
  pages:
  - 347
  - 348
  engine_gaps: &id001
  - dnd2024:creature_draft_horse
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/draft_horse
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a291c4064a7845d36ba25038573bf3e23b138a93a76610476568cd95cd19e46f
splat_clauses:
- key: source_rules
  text: '# Draft Horse Large Beast, Unaligned AC 10 Initiative +0 (10) HP 15 (2d10 + 4) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 10 +0 +0 Con 15 +2 +2 Int 2 -4 -4 Wis 11 +0 +0 Cha 7 -2 -2
    Senses Passive Perception 10 Languages None CR 1/4 (XP 50; PB +2) ### Actions Hooves. Melee Attack
    Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Bludgeoning damage.'
  sha256: e880f80bbd21ca651073eff691b7ff1382f9fb8ecad871170761416730359b18
  status: contract_specified
splat_requires_mechanics: *id001
---

# Draft Horse

<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 15 (2d10 + 4)\
Speed 40 ft.\

<!-- SRD page 348 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]], [[../../srd-5.2.1/pages/348|page 348]].
