---
splat_schema: 1
splat_id: 7d967769-0c10-5986-91ca-3bb88fcfa044
splat_kind: creature
splat_key: dnd2024:creatures/minotaur_of_baphomet
splat_name: Minotaur of Baphomet
splat_origin:
  key: dnd2024:reference/creature/minotaur_of_baphomet
splat_sources:
- title: SRD 5.2.1
  locator: pp. 309
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 40 ft.
  skills_text: Perception +7, Survival +7
  senses_text: Darkvision 60 ft.; Passive Perception 17
  languages_text: Abyssal
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 85 (10d10 + 30)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 14
  max_hp:
    value: 85
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/minotaur_of_baphomet
  implementation: dnd2024:creature_minotaur_of_baphomet
  parameters:
    skill_totals:
      perception: 7
      survival: 7
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 17
    languages:
      speak: true
      known:
      - abyssal
    features:
      abyssal_glaive:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 12
              add: 4
            average: 10
          - type: necrotic
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            average: 10
      gore:
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
          - type: piercing
            amount:
              dice:
                count: 4
                sides: 6
              add: 4
            average: 18
        - op: conditional
          when:
            target_maximum_size: large
            actor_moved_straight_toward_target_ft_at_least: 10
            immediately_before_hit: true
            target_kind: creature
          effects:
          - op: damage
            components:
            - type: piercing
              amount:
                dice:
                  count: 3
                  sides: 6
                add: 0
              average: 10
          - op: condition.apply
            condition: prone
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
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
- id: dnd2024:creature_minotaur_of_baphomet
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
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 5e79c832e17a6c5cc0af3c6554c927c28f57381537ed8f8920a8df7980d2ae02
  source_path: builtin/srd-5.2.1/creature/minotaur_of_baphomet.md
  pages:
  - 309
  engine_gaps: &id001
  - dnd2024:creature_minotaur_of_baphomet
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: e780d46e6d116a27e52d608ff0d7890bb178c00aefabc67c671220d5f78d2a72
splat_clauses:
- key: source_rules
  text: '# Minotaur of Baphomet Large Monstrosity, Chaotic Evil AC 14 Initiative +0 (10) HP 85 (10d10
    + 30) Speed 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 11 +0 +0 Con 16 +3 +3 Int 6 -2 -2 Wis
    16 +3 +3 Cha 9 -1 -1 Skills Perception +7, Survival +7 Senses Darkvision 60 ft.; Passive Perception
    17 Languages Abyssal CR 3 (XP 700; PB +2) ### Actions Abyssal Glaive. Melee Attack Roll: +6, reach
    10 ft. Hit: 10 (1d12 + 4) Slashing damage plus 10 (3d6) Necrotic damage. Gore (Recharge 5–6). Melee
    Attack Roll: +6, reach 5 ft. Hit: 18 (4d6 + 4) Piercing damage. If the target is a Large or smaller
    creature and the minotaur moved 10+ feet straight toward it immediately before the hit, the target
    takes an extra 10 (3d6) Piercing damage and has the Prone condition.'
  sha256: 07e29b63c46a3c46866e3b1f00acb50b7ca2a034ed87d904cfcc077621ff1031
  status: contract_specified
splat_requires_mechanics: *id001
---

# Minotaur of Baphomet

<!-- SRD page 309 -->

Large Monstrosity, Chaotic Evil\
AC 14 Initiative +0 (10)\
HP 85 (10d10 + 30)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 11 +0 +0 Con 16 +3 +3\
Int 6 −2 −2 Wis 16 +3 +3 Cha 9 −1 −1\
Skills Perception +7, Survival +7\
Senses Darkvision 60 ft.; Passive Perception 17\
Languages Abyssal\
CR 3 (XP 700; PB +2)\

### Actions

Abyssal Glaive. Melee Attack Roll: +6, reach 10 ft.\
Hit: 10 (1d12 + 4) Slashing damage plus 10 (3d6) Ne-\
crotic damage.\
Gore (Recharge 5–6). Melee Attack Roll: +6, reach\
5 ft. Hit: 18 (4d6 + 4) Piercing damage. If the target is a\
Large or smaller creature and the minotaur moved 10+\
feet straight toward it immediately before the hit, the\
target takes an extra 10 (3d6) Piercing damage and has\
the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/309|page 309]].
