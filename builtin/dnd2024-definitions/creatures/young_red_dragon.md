---
splat_schema: 1
splat_id: 968ba029-f6ed-5e58-94a3-afa191c0535e
splat_kind: creature
splat_key: dnd2024:creatures/young_red_dragon
splat_name: Young Red Dragon
splat_origin:
  key: dnd2024:reference/creature/young_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 318
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Perception +8, Stealth +4
  immunities_text: Fire
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18
  languages_text: Common, Draconic
  cr_text: 10 (XP 5,900; PB +4)
  hit_points_text: 178 (17d10 + 85)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 10
  save_dexterity:
    value: 4
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 4
  charisma:
    value: 19
  save_charisma:
    value: 4
  armor_class:
    value: 18
  max_hp:
    value: 178
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_red_dragon
  implementation: dnd2024:creature_young_red_dragon
  parameters:
    skill_totals:
      perception: 8
      stealth: 4
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 30
    - sense: darkvision
      range_ft: 120
    passive_perception: 18
    languages:
      speak: true
      known:
      - common
      - draconic
    features:
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 10
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
                count: 2
                sides: 6
              add: 6
            average: 13
          - type: fire
            amount:
              dice:
                count: 1
                sides: 6
              add: 0
            average: 3
      fire_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 17
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 30
        failure:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 16
                sides: 6
              add: 0
            average: 56
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
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
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 3
          feature: rend
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
- id: dnd2024:creature_young_red_dragon
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
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 1cc635c18a5dfe3762d65a8167746dc6bca8c6898cec616cca9b6c992628727e
  source_path: builtin/srd-5.2.1/creature/young_red_dragon.md
  pages:
  - 318
  engine_gaps: &id001
  - dnd2024:creature_young_red_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 2cb5cc9a6662c7220c7fa30db35ed3b35fa40157301a4b7bca00c17a572ce1ca
splat_clauses:
- key: source_rules
  text: '# Young Red Dragon Large Dragon (Chromatic), Chaotic Evil AC 18 Initiative +4 (14) HP 178 (17d10
    + 85) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 10 +0 +4
    Con 21 +5 +5 Int 14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4 Skills Perception +8, Stealth +4 Immunities Fire
    Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18 Languages Common, Draconic CR
    10 (XP 5,900; PB +4) ### Actions Multiattack. The dragon makes three Rend attacks. Rend. Melee Attack
    Roll: +10, reach 10 ft. Hit: 13 (2d6 + 6) Slashing damage plus 3 (1d6) Fire damage. Fire Breath (Recharge
    5–6). Dexterity Saving Throw: DC 17, each creature in a 30-foot Cone. Failure: 56 (16d6) Fire damage.
    Success: Half damage.'
  sha256: 6e62ef9e75c666e5824abe27dc3f21bf5286bf3a6218a8cfee763ebc5ea44796
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Red Dragon

<!-- SRD page 318 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 18 Initiative +4 (14)\
HP 178 (17d10 + 85)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 10 +0 +4 Con 21 +5 +5\
Int 14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4\
Skills Perception +8, Stealth +4\
Immunities Fire\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 18\
Languages Common, Draconic\
CR 10 (XP 5,900; PB +4)\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 13 (2d6\
+ 6) Slashing damage plus 3 (1d6) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 17, each creature in a 30-foot Cone. Failure: 56\
(16d6) Fire damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/318|page 318]].
