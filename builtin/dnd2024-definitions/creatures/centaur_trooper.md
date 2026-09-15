---
splat_schema: 1
splat_id: b3e09ba9-787f-5cd5-9e07-4bd1e256147c
splat_kind: creature
splat_key: dnd2024:creatures/centaur_trooper
splat_name: Centaur Trooper
splat_origin:
  key: dnd2024:reference/creature/centaur_trooper
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fey
  alignment: Neutral Good
  movement_text: 50 ft.
  skills_text: Athletics +6, Perception +3
  gear_text: Breastplate, Longbow, Pike
  senses_text: Passive Perception 13
  languages_text: Elvish, Sylvan
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 9
  save_intelligence:
    value: -1
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 16
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/centaur_trooper
  implementation: dnd2024:creature_centaur_trooper
  parameters:
    skill_totals:
      athletics: 6
      perception: 3
    gear:
    - item: breastplate
      count: 1
    - item: longbow
      count: 1
    - item: pike
      count: 1
    senses: []
    passive_perception: 13
    languages:
      speak: true
      known:
      - elvish
      - sylvan
    features:
      pike:
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
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 10
              add: 4
            average: 9
      longbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 150
          long: 600
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 2
            average: 6
      trampling_charge:
        economy: bonus_action
        op: movement
        maximum:
          multiply:
          - 1
          - actor_stat: speed
        opportunity_attacks: false
        through_creature_maximum_size: medium
        on_enter_creature_space:
          maximum_per_creature: 1
          effect:
            op: saving_throw
            ability: strength
            dc: 14
            targets:
              kind: entered_creature
            failure:
            - op: damage
              components:
              - type: bludgeoning
                amount:
                  dice:
                    count: 1
                    sides: 6
                  add: 4
                average: 7
            - op: condition.apply
              condition: prone
            success: []
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
        attacks:
          count: 2
          choices:
          - pike
          - longbow
          combination: any
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
- id: dnd2024:creature_centaur_trooper
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
  source_sha256: 45d5e0df2ece1ed7d988d767fbc4108a718db8424c4b269b7435c8588b5dc2e0
  source_path: builtin/srd-5.2.1/creature/centaur_trooper.md
  pages:
  - 272
  engine_gaps: &id001
  - dnd2024:creature_centaur_trooper
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: aa24ece5d7ee68198b505e453a5f0034376384815065c619758c4f5faf4cf69e
splat_clauses:
- key: source_rules
  text: '# Centaur Trooper Large Fey, Neutral Good AC 16 Initiative +2 (12) HP 45 (6d10 + 12) Speed 50
    ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 14 +2 +2 Con 14 +2 +2 Int 9 -1 -1 Wis 13 +1 +1 Cha
    11 +0 +0 Skills Athletics +6, Perception +3 Gear Breastplate, Longbow, Pike Senses Passive Perception
    13 Languages Elvish, Sylvan CR 2 (XP 450; PB +2) ### Actions Multiattack. The centaur makes two attacks,
    using Pike or Longbow in any combination. Pike. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10 +
    4) Piercing damage. Longbow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage.
    ### Bonus Actions Trampling Charge (Recharge 5–6). The centaur moves up to its Speed without provoking
    Opportunity Attacks and can move through the spaces of Medium or smaller creatures. Each creature
    whose space the centaur enters is targeted once by the following effect. Strength Saving Throw: DC
    14. Failure: 7 (1d6 + 4) Bludgeoning damage, and the target has the Prone condition.'
  sha256: 86563130a5f670e6348428c036a285d8179ed2e16a4d3ef3c4aff41a80978a69
  status: contract_specified
splat_requires_mechanics: *id001
---

# Centaur Trooper

<!-- SRD page 272 -->

Large Fey, Neutral Good\
AC 16 Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 14 +2 +2\
Int 9 −1 −1 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Athletics +6, Perception +3\
Gear Breastplate, Longbow, Pike\
Senses Passive Perception 13\
Languages Elvish, Sylvan\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The centaur makes two attacks, using Pike\
or Longbow in any combination.\
Pike. Melee Attack Roll: +6, reach 10 ft. Hit: 9 (1d10\
+ 4) Piercing damage.\
Longbow. Ranged Attack Roll: +4, range 150/600 ft.\
Hit: 6 (1d8 + 2) Piercing damage.\

### Bonus Actions

Trampling Charge (Recharge 5–6). The centaur moves\
up to its Speed without provoking Opportunity Attacks\
and can move through the spaces of Medium or smaller\
creatures. Each creature whose space the centaur en-\
ters is targeted once by the following effect. Strength\
Saving Throw: DC 14. Failure: 7 (1d6 + 4) Bludgeoning\
damage, and the target has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/272|page 272]].
