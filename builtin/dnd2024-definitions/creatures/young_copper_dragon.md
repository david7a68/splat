---
splat_schema: 1
splat_id: 76207bbe-547f-5d70-94ce-cb0c8ab9bccc
splat_kind: creature
splat_key: dnd2024:creatures/young_copper_dragon
splat_name: Young Copper Dragon
splat_origin:
  key: dnd2024:reference/creature/young_copper_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 276
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Deception +5, Perception +7, Stealth +4
  immunities_text: Acid
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17
  languages_text: Common, Draconic
  cr_text: 7 (XP 2,900; PB +3)
  hit_points_text: 119 (14d10 + 42)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 12
  save_dexterity:
    value: 4
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 13
  save_wisdom:
    value: 4
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 17
  max_hp:
    value: 119
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
  mechanic: dnd2024:creatures/young_copper_dragon
  implementation: dnd2024:creature_young_copper_dragon
  parameters:
    skill_totals:
      deception: 5
      perception: 7
      stealth: 4
    damage_immunities:
    - acid
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 30
    - sense: darkvision
      range_ft: 120
    passive_perception: 17
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
        attack_bonus: 7
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
                sides: 10
              add: 4
            average: 15
      acid_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 40
            width_ft: 5
        failure:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 9
                sides: 8
              add: 0
            average: 40
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
      slowing_breath:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 30
        failure:
        - op: effect.apply
          duration:
            until: target.next_turn_end
          rules:
            reactions_allowed: false
            speed_multiplier: 0.5
            turn_economy_choice:
            - action
            - bonus_action
            both_allowed: false
        success: []
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 3
          feature: rend
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: slowing_breath
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
- id: dnd2024:creature_young_copper_dragon
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
  source_sha256: b5c128dee97d52dda6957ef5c58d63bedd097ee69dbec696ed8cab95a6a34eec
  source_path: builtin/srd-5.2.1/creature/young_copper_dragon.md
  pages:
  - 276
  engine_gaps: &id001
  - dnd2024:creature_young_copper_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 07bb96b2c7c40200847c85bf36856b839c7478b2b72b3ea291442252091f792f
splat_clauses:
- key: source_rules
  text: '# Young Copper Dragon Large Dragon (Metallic), Chaotic Good AC 17 Initiative +4 (14) HP 119 (14d10
    + 42) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 12 +1 +4
    Con 17 +3 +3 Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2 Skills Deception +5, Perception +7, Stealth +4
    Immunities Acid Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 Languages Common,
    Draconic CR 7 (XP 2,900; PB +3) ### Actions Multiattack. The dragon makes three Rend attacks. It can
    replace one attack with a use of Slowing Breath. Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 15
    (2d10 + 4) Slashing damage. Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 14, each creature
    in a 40-foot-long, 5-footwide Line. Failure: 40 (9d8) Acid damage. Success: Half damage. Slowing Breath.
    Constitution Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: The target can’t take
    Reactions; its Speed is halved; and it can take either an action or a Bonus Action on its turn, not
    both. This effect lasts until the end of its next turn.'
  sha256: 079b2e68a10918192c244ce6df0ebb9dd8af3bb2a601e924f3bda97efa60353f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Copper Dragon

<!-- SRD page 276 -->

Large Dragon (Metallic), Chaotic Good\
AC 17  Initiative +4 (14)\
HP 119 (14d10 + 42)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 12 +1 +4 Con 17 +3 +3\
Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2\
Skills Deception +5, Perception +7, Stealth +4\
Immunities Acid\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 17\
Languages Common, Draconic\
CR 7 (XP 2,900; PB +3)\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Slowing Breath.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 15 (2d10\
+ 4) Slashing damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 14, each creature in a 40-foot-long, 5-foot-\
wide Line. Failure: 40 (9d8) Acid damage. Success:\
Half damage.\
Slowing Breath. Constitution Saving Throw: DC 14,\
each creature in a 30-foot Cone. Failure: The target\
can’t take Reactions; its Speed is halved; and it can take\
either an action or a Bonus Action on its turn, not both.\
This effect lasts until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/276|page 276]].
