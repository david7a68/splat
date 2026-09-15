---
splat_schema: 1
splat_id: 08ef1af3-e4b0-5309-a741-163066ff99df
splat_kind: creature
splat_key: dnd2024:creatures/darkmantle
splat_name: Darkmantle
splat_origin:
  key: dnd2024:reference/creature/darkmantle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278, 279
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: aberration
  alignment: Unaligned
  movement_text: 10 ft., Fly 30 ft.
  skills_text: Stealth +3
  senses_text: Blindsight 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (5d6 + 5)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
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
    value: 11
  max_hp:
    value: 22
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/darkmantle
  implementation: dnd2024:creature_darkmantle
  parameters:
    features:
      crush:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 6
            roll:
              count: 1
              sides: 6
              modifier: 3
            type: bludgeoning
          attach:
            target: hit_target
            covered_if:
              maximum_size: medium
              attack_had_advantage: true
            while_covered:
              conditions:
              - blinded
              suffocating: true
            while_attached:
              attack_targets:
              - attached_target
              attack_advantage: true
              speed: 0
              speed_bonuses: false
              moves_with_target: true
            detach_self:
              economy: movement
              cost_ft: 5
              on_own_turn: true
            detach_target:
              economy: action
              check: strength_athletics
              dc: 13
              actor: attached_target
      darkness_aura:
        economy: action
        resource:
          kind: daily
          uses: 1
          recovery: long_rest
        operation: create_area
        area:
          shape: emanation
          size_ft: 15
          origin: self
        magical_darkness: true
        concentration: true
        duration_seconds: 600
        darkvision_penetrates: false
        light_illuminates: false
    skill_totals:
      stealth: 3
    senses:
      blindsight: 60
      passive_perception: 10
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
- id: dnd2024:creature_darkmantle
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
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 09388b33cedeeba676c6b8924203eab6ea0b4dba0b9b30c2df37487a13b643d9
  source_path: builtin/srd-5.2.1/creature/darkmantle.md
  pages:
  - 278
  - 279
  engine_gaps: &id001
  - dnd2024:creature_darkmantle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: cb1714577408a53a47237644023abe728af2836972c20d7eab7e4ac91f9ba70e
splat_clauses:
- key: source_rules
  text: '# Darkmantle Small Aberration, Unaligned AC 11 Initiative +3 (13) HP 22 (5d6 + 5) Speed 10 ft.,
    Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 12 +1 +1 Con 13 +1 +1 Int 2 -4 -4 Wis 10 +0
    +0 Cha 5 -3 -3 Skills Stealth +3 Senses Blindsight 60 ft.; Passive Perception 10 Languages None CR
    1/2 (XP 100; PB +2) ### Actions Crush. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning
    damage, and the darkmantle attaches to the target. If the target is a Medium or smaller creature and
    the darkmantle had Advantage on the attack roll, it covers the target, which has the Blinded condition
    and is suffocating while the darkmantle is attached in this way. While attached to a target, the darkmantle
    can attack only the target but has Advantage on its attack rolls. Its Speed becomes 0, it can’t benefit
    from any bonus to its Speed, and it moves with the target. A creature can take an action to try to
    detach the darkmantle from itself, doing so with a successful DC 13 Strength (Athletics) check. On
    its turn, the darkmantle can detach itself by using 5 feet of movement. Darkness Aura (1/Day). Magical
    Darkness fills a 15-foot Emanation originating from the darkmantle. This effect lasts while the darkmantle
    maintains Concentration on it, up to 10 minutes. Darkvision can’t penetrate this area, and no light
    can illuminate it.'
  sha256: aa3ea7614a0e1f1953500f72db598517859865f29aea7d7c30f16df48a08afe4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Darkmantle

<!-- SRD page 278 -->

Small Aberration, Unaligned\
AC 11 Initiative +3 (13)\
HP 22 (5d6 + 5)\
Speed 10 ft., Fly 30 ft.\

<!-- SRD page 279 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 12 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Stealth +3\
Senses Blindsight 60 ft.; Passive Perception 10\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Crush. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Bludgeoning damage, and the darkmantle attaches\
to the target. If the target is a Medium or smaller crea-\
ture and the darkmantle had Advantage on the attack\
roll, it covers the target, which has the Blinded condi-\
tion and is suffocating while the darkmantle is attached\
in this way.\
While attached to a target, the darkmantle can attack\
only the target but has Advantage on its attack rolls. Its\
Speed becomes 0, it can’t benefit from any bonus to its\
Speed, and it moves with the target.\
A creature can take an action to try to detach the\
darkmantle from itself, doing so with a successful DC\
13 Strength (Athletics) check. On its turn, the darkman-\
tle can detach itself by using 5 feet of movement.\
Darkness Aura (1/Day). Magical Darkness fills a 15-foot\
Emanation originating from the darkmantle. This effect\
lasts while the darkmantle maintains Concentration\
on it, up to 10 minutes. Darkvision can’t penetrate this\
area, and no light can illuminate it.\

Original layout: [[../../srd-5.2.1/pages/278|page 278]], [[../../srd-5.2.1/pages/279|page 279]].
