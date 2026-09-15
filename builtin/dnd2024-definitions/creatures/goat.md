---
splat_schema: 1
splat_id: 9d6079d4-0c76-5ee6-8576-9cedbe54f5a6
splat_kind: creature
splat_key: dnd2024:creatures/goat
splat_name: Goat
splat_origin:
  key: dnd2024:reference/creature/goat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 30 ft.
  skills_text: Perception +2
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 4 (1d8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 2
  dexterity:
    value: 10
  save_dexterity:
    value: 0
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
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 10
  max_hp:
    value: 4
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/goat
  implementation: dnd2024:creature_goat
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
              constant: 1
            average: 1
          replacement_when:
            if:
              actor_moved_straight_toward_target_ft_at_least: 20
              immediately_before_hit: true
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
- id: dnd2024:creature_goat
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
  source_sha256: 7197f92960e499dd3a2411e9bb68f98536b20369a75cf5dec846a3477384099b
  source_path: builtin/srd-5.2.1/creature/goat.md
  pages:
  - 355
  engine_gaps: &id001
  - dnd2024:creature_goat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ae83ae3e74acb7c3d600dcd23fc3e8241eaa041a49584dbb24c0ce1ff658b1ae
splat_clauses:
- key: source_rules
  text: '# Goat Medium Beast, Unaligned AC 10 Initiative +0 (10) HP 4 (1d8) Speed 40 ft., Climb 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +2 Dex 10 +0 +0 Con 11 +0 +0 Int 2 -4 -4 Wis 10 +0 +0 Cha 5 -3
    -3 Skills Perception +2 Senses Darkvision 60 ft.; Passive Perception 12 Languages None CR 0 (XP 10;
    PB +2) ### Actions Ram. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Bludgeoning damage, or 2 (1d4) Bludgeoning
    damage if the goat moved 20+ feet straight toward the target immediately before the hit.'
  sha256: a680a1eaa9a19bcf0a1180ff1703419c9f313d31fccbf42eb3d22f1739bf9380
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goat

<!-- SRD page 355 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 4 (1d8)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +2 Dex 10 +0 +0 Con 11 +0 +0\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Perception +2\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Ram. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Blud-\
geoning damage, or 2 (1d4) Bludgeoning damage if the\
goat moved 20+ feet straight toward the target immedi-\
ately before the hit.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]].
