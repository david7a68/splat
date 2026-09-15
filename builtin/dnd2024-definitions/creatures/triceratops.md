---
splat_schema: 1
splat_id: 95a127ec-d2fb-5908-8987-dd8846ede461
splat_kind: creature
splat_key: dnd2024:creatures/triceratops
splat_name: Triceratops
splat_origin:
  key: dnd2024:reference/creature/triceratops
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 50 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 114 (12d12 + 36)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 114
  initiative:
    value: -1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/triceratops
  implementation: dnd2024:creature_triceratops
  parameters:
    senses: []
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      gore:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 9
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
                sides: 12
              add: 6
            average: 19
        - op: conditional
          when:
            target_maximum_size: huge
            actor_moved_straight_toward_target_ft_at_least: 20
            immediately_before_hit: true
          effects:
          - op: damage
            components:
            - type: piercing
              amount:
                dice:
                  count: 2
                  sides: 8
                add: 0
              average: 9
          - op: condition.apply
            condition: prone
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: gore
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
- id: dnd2024:creature_triceratops
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
  source_sha256: de4688602cee24889e59533aa0124c6511a2dfbe225f1571a842b4d45444b978
  source_path: builtin/srd-5.2.1/creature/triceratops.md
  pages:
  - 363
  engine_gaps: &id001
  - dnd2024:creature_triceratops
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a981d45aaf52610072ea1300e881981ed563d8d91662a1f29dc08a837afdc885
splat_clauses:
- key: source_rules
  text: '# Triceratops Huge Beast (Dinosaur), Unaligned AC 14 Initiative -1 (9) HP 114 (12d12 + 36) Speed
    50 ft. MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +6 Dex 9 -1 -1 Con 17 +3 +3 Int 2 -4 -4 Wis 11 +0 +0 Cha
    5 -3 -3 Senses Passive Perception 10 Languages None CR 5 (XP 1,800; PB +3) ### Actions Multiattack.
    The triceratops makes two Gore attacks. Gore. Melee Attack Roll: +9, reach 5 ft. Hit: 19 (2d12 + 6)
    Piercing damage. If the target is Huge or smaller and the triceratops moved 20+ feet straight toward
    it immediately before the hit, the target takes an extra 9 (2d8) Piercing damage and has the Prone
    condition.'
  sha256: eeb47c0225c48e9c8e5e62ecf9b88f4e8054ff95a269b3344912a41d4e92716a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Triceratops

<!-- SRD page 363 -->

Huge Beast (Dinosaur), Unaligned\
AC 14  Initiative −1 (9)\
HP 114 (12d12 + 36)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 17 +3 +3\
Int 2 −4 −4 Wis 11 +0 +0 Cha  5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The triceratops makes two Gore attacks.\
Gore. Melee Attack Roll: +9, reach 5 ft. Hit: 19 (2d12\
+ 6) Piercing damage. If the target is Huge or smaller\
and the triceratops moved 20+ feet straight toward it\
immediately before the hit, the target takes an extra 9\
(2d8) Piercing damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/363|page 363]].
