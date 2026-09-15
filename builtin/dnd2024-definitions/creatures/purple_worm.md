---
splat_schema: 1
splat_id: b0392038-ffb8-5c6d-9fe9-99bc4eda61ee
splat_kind: creature
splat_key: dnd2024:creatures/purple_worm
splat_name: Purple Worm
splat_origin:
  key: dnd2024:reference/creature/purple_worm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316, 317
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Unaligned
  movement_text: 50 ft., Burrow 50 ft.
  senses_text: Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 15 (XP 13,000; PB +5)
  hit_points_text: 247 (15d20 + 90)
splat_stats:
  strength:
    value: 28
  save_strength:
    value: 9
  dexterity:
    value: 7
  save_dexterity:
    value: -2
  constitution:
    value: 22
  save_constitution:
    value: 11
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: 4
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 18
  max_hp:
    value: 247
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
  burrow_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/purple_worm
  implementation: dnd2024:creature_purple_worm
  parameters:
    senses:
    - sense: blindsight
      range_ft: 30
    - sense: tremorsense
      range_ft: 60
    passive_perception: 9
    languages:
      speak: true
      known: []
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 3
                sides: 8
              add: 9
            average: 22
        - op: grapple
          escape_dc: 19
          when:
            target_kind: creature
            target_maximum_size: large
          while_grappled:
          - op: condition.apply
            condition: restrained
      tail_stinger:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 6
              add: 9
            average: 16
          - type: poison
            amount:
              dice:
                count: 10
                sides: 6
              add: 0
            average: 35
      tunneler:
        economy: passive
        op: burrow.solid_rock
        speed_multiplier: 0.5
        leaves_tunnel_diameter_ft: 10
      swallow:
        economy: bonus_action
        op: saving_throw
        ability: strength
        dc: 19
        targets:
          kind: creature
          count: 1
          maximum_size: large
          grappled_by: actor
        failure:
        - op: swallow
          end_grapple: true
          conditions:
          - blinded
          - restrained
          total_cover_against: outside_actor
          periodic_damage:
            event: actor.turn_start
            damage:
            - type: acid
              amount:
                dice:
                  count: 5
                  sides: 6
                add: 0
              average: 17
        success: []
        swallowed_capacity: 3
        regurgitation:
          trigger: turn.end
          ability: constitution
          dc: 21
          failure:
            targets: all_swallowed_creatures
            exit_within_ft: 5
            condition: prone
          when:
            damage_to_actor_from_one_swallowed_creature_this_turn_at_least: 30
        actor_death:
          remove_condition: restrained
          escape_movement_cost_ft: 20
          exit_condition: prone
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 1
          feature: bite
        - count: 1
          feature: tail_stinger
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
- id: dnd2024:creature_purple_worm
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
  - that_turn.end
  - turn.end
  capabilities:
  - action_economy
  - attack_resolution
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: cb7b19b783001d1e37f5c6effa6f2f9af306c843905bbeacf614c56e9d366cc2
  source_path: builtin/srd-5.2.1/creature/purple_worm.md
  pages:
  - 316
  - 317
  engine_gaps: &id001
  - dnd2024:creature_purple_worm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 3183cd5bc850ba1a62709798c76f64fe1d56505c16cd8a9fd661778d85d9edf5
splat_clauses:
- key: source_rules
  text: '# Purple Worm Gargantuan Monstrosity, Unaligned AC 18 Initiative +3 (13) HP 247 (15d20 + 90)
    Speed 50 ft., Burrow 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 28 +9 +9 Dex 7 -2 -2 Con 22 +6 +11 Int
    1 -5 -5 Wis 8 -1 +4 Cha 4 -3 -3 Senses Blindsight 30 ft., Tremorsense 60 ft.; Passive Perception 9
    Languages None CR 15 (XP 13,000; PB +5) ### Traits Tunneler. The worm can burrow through solid rock
    at half its Burrow Speed and leaves a 10-foot-diameter tunnel in its wake. ### Actions Multiattack.
    The worm makes one Bite attack and one Tail Stinger attack. Bite. Melee Attack Roll: +14, reach 10
    ft. Hit: 22 (3d8 + 9) Piercing damage. If the target is a Large or smaller creature, it has the Grappled
    condition (escape DC 19), and it has the Restrained condition until the grapple ends. Tail Stinger.
    Melee Attack Roll: +14, reach 10 ft. Hit: 16 (2d6 + 9) Piercing damage plus 35 (10d6) Poison damage.
    ### Bonus Actions Swallow. Strength Saving Throw: DC 19, one Large or smaller creature Grappled by
    the worm (it can have up to three creatures swallowed at a time). Failure: The target is swallowed
    by the worm, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained
    conditions, has Total Cover against attacks and other effects outside the worm, and takes 17 (5d6)
    Acid damage at the start of each of the worm’s turns. If the worm takes 30 damage or more on a single
    turn from a creature inside it, the worm must succeed on a DC 21 Constitution saving throw at the
    end of that turn or regurgitate all swallowed creatures, each of which falls in a space within 5 feet
    of the worm and has the Prone condition. If the worm dies, any swallowed creature no longer has the
    Restrained condition and can escape from the corpse using 20 feet of movement, exiting Prone.'
  sha256: f20fc4d688d675825590278168f4e720cbd60fc45424b54605d49d7cb4e4e414
  status: contract_specified
splat_requires_mechanics: *id001
---

# Purple Worm

<!-- SRD page 316 -->

Gargantuan Monstrosity, Unaligned\
AC 18 Initiative +3 (13)\
HP 247 (15d20 + 90)\
Speed 50 ft., Burrow 50 ft.\

<!-- SRD page 317 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 28 +9 +9 Dex 7 −2 −2 Con 22 +6 +11\
Int 1 −5 −5 Wis 8 −1 +4 Cha 4 −3 −3\
Senses Blindsight 30 ft., Tremorsense 60 ft.;\
Passive Perception 9\
Languages None\
CR 15 (XP 13,000; PB +5)\

### Traits

Tunneler. The worm can burrow through solid rock at\
half its Burrow Speed and leaves a 10-foot-diameter\
tunnel in its wake.\

### Actions

Multiattack. The worm makes one Bite attack and one\
Tail Stinger attack.\
Bite. Melee Attack Roll: +14, reach 10 ft. Hit: 22\
(3d8 + 9) Piercing damage. If the target is a Large or\
smaller creature, it has the Grappled condition (escape\
DC 19), and it has the Restrained condition until the\
grapple ends.\
Tail Stinger. Melee Attack Roll: +14, reach 10 ft. Hit:\
16 (2d6 + 9) Piercing damage plus 35 (10d6) Poi-\
son damage.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or\
smaller creature Grappled by the worm (it can have up\
to three creatures swallowed at a time). Failure: The tar-\
get is swallowed by the worm, and the Grappled con-\
dition ends. A swallowed creature has the Blinded and\
Restrained conditions, has Total Cover against attacks\
and other effects outside the worm, and takes 17 (5d6)\
Acid damage at the start of each of the worm’s turns.\
If the worm takes 30 damage or more on a single turn\
from a creature inside it, the worm must succeed on a\
DC 21 Constitution saving throw at the end of that turn\
or regurgitate all swallowed creatures, each of which\
falls in a space within 5 feet of the worm and has the\
Prone condition. If the worm dies, any swallowed crea-\
ture no longer has the Restrained condition and can\
escape from the corpse using 20 feet of movement,\
exiting Prone.\

Original layout: [[../../srd-5.2.1/pages/316|page 316]], [[../../srd-5.2.1/pages/317|page 317]].
