---
splat_schema: 1
splat_id: 376d504e-1cfd-5dbc-946b-39e90cb4af9b
splat_kind: creature
splat_key: dnd2024:creatures/giant_octopus
splat_name: Giant Octopus
splat_origin:
  key: dnd2024:reference/creature/giant_octopus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Swim 60 ft.
  skills_text: Perception +4, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 45 (7d10 + 7)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 45
  initiative:
    value: 1
  speed:
    value: 10
    unit: ft
  swim_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_octopus
  implementation: dnd2024:creature_giant_octopus
  parameters:
    features:
      water_breathing:
        economy: trait
        operation: passive_rule
        breathing:
        - water
        hold_breath_outside_water_seconds: 3600
      tentacles:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 10
        hit:
          damage:
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
            maximum_size: medium
            appendage_pool:
              count: 8
              type: tentacles
              cost: 8
            while_grappled:
              conditions:
              - restrained
      ink_cloud:
        economy: reaction
        resource:
          kind: daily
          uses: 1
          recovery: long_rest
        operation: sequence
        trigger: self.damage_taken
        requires:
          self_underwater: true
        effects:
        - operation: create_area
          area:
            shape: cube
            size_ft: 10
            center: self
          obscurement: heavy
          duration_seconds: 60
          ends_on:
          - strong_current
          - similar_dispersal_effect
        - operation: move
          maximum_ft:
            multiply:
            - stat: swim_speed
            - 1
    skill_totals:
      perception: 4
      stealth: 5
    senses:
      darkvision: 60
      passive_perception: 14
    languages:
      speaks: []
      understands: []
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_giant_octopus
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - damage.resolved
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 2e33b2d234b8e16bf875fcd935b2e25ef25d002ecbe1daab9e8f8a0ef967c482
  source_path: builtin/srd-5.2.1/creature/giant_octopus.md
  pages:
  - 352
  engine_gaps: &id001
  - dnd2024:creature_giant_octopus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: f340944fecbc7a03921c82acd3f403ed49506cac3c92ec56a3656ba83abb1e2b
splat_clauses:
- key: source_rules
  text: '# Giant Octopus Large Beast, Unaligned AC 11 Initiative +1 (11) HP 45 (7d10 + 7) Speed 10 ft.,
    Swim 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1 Int 5 -3 -3 Wis 10 +0
    +0 Cha 4 -3 -3 Skills Perception +4, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 14 Languages
    None CR 1 (XP 200; PB +2) ### Traits Water Breathing. The octopus can breathe only underwater. It
    can hold its breath for 1 hour outside water. ### Actions Tentacles. Melee Attack Roll: +5, reach
    10 ft. Hit: 10 (2d6 + 3) Bludgeoning damage. If the target is a Medium or smaller creature, it has
    the Grappled condition (escape DC 13) from all eight tentacles. While Grappled, the target has the
    Restrained condition. ### Reactions Ink Cloud (1/Day). Trigger: The octopus takes damage while underwater.
    Response: The octopus releases ink that fills a 10-foot Cube centered on itself, and the octopus moves
    up to its Swim Speed. The Cube is Heavily Obscured for 1 minute or until a strong current or similar
    effect disperses the ink.'
  sha256: d23891c357f790ccc2f5b03e49fef668f567387b3317bf68bb0baf0344f1632c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Octopus

<!-- SRD page 352 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 45 (7d10 + 7)\
Speed 10 ft., Swim 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
Int 5 −3 −3 Wis 10 +0 +0 Cha 4 −3 −3\
Skills Perception +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Water Breathing. The octopus can breathe only under-\
water. It can hold its breath for 1 hour outside water.\

### Actions

Tentacles. Melee Attack Roll: +5, reach 10 ft. Hit: 10\
(2d6 + 3) Bludgeoning damage. If the target is a Me-\
dium or smaller creature, it has the Grappled condition\
(escape DC 13) from all eight tentacles. While Grap-\
pled, the target has the Restrained condition.\

### Reactions

Ink Cloud (1/Day). Trigger: The octopus takes damage\
while underwater. Response: The octopus releases ink\
that fills a 10-foot Cube centered on itself, and the oc-\
topus moves up to its Swim Speed. The Cube is Heavily\
Obscured for 1 minute or until a strong current or simi-\
lar effect disperses the ink.\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].
