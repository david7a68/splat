---
splat_schema: 1
splat_id: d0494afe-effc-54d5-8b6b-4d9ef95ba0df
splat_kind: creature
splat_key: dnd2024:creatures/satyr
splat_name: Satyr
splat_origin:
  key: dnd2024:reference/creature/satyr
splat_sources:
- title: SRD 5.2.1
  locator: pp. 321, 322
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Chaotic Neutral
  movement_text: 40 ft.
  skills_text: Perception +2, Performance +6, Stealth +5
  senses_text: Passive Perception 12
  languages_text: Common, Elvish, Sylvan
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 31 (7d8)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 13
  max_hp:
    value: 31
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/satyr
  implementation: dnd2024:creature_satyr
  parameters:
    features:
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      hooves:
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
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: bludgeoning
          push:
            maximum_ft: 10
            direction: straight_away_from_self
            maximum_size: medium
      mockery:
        economy: action
        operation: saving_throw
        ability: wisdom
        dc: 12
        targets:
          kind: creature
          count: 1
          range_ft: 90
          visible: true
        failure:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 6
              modifier: 2
            type: psychic
        success:
          effects: []
    skill_totals:
      perception: 2
      performance: 6
      stealth: 5
    senses:
      passive_perception: 12
    languages:
      speaks:
      - common
      - elvish
      - sylvan
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
- id: dnd2024:creature_satyr
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 47fd98532cc419dee7f5f1c422a0d12c1e24e1b7d65759a52921afe096e425cb
  source_path: builtin/srd-5.2.1/creature/satyr.md
  pages:
  - 321
  - 322
  engine_gaps: &id001
  - dnd2024:creature_satyr
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 7b3952f6bf2917a0b96f74a0e465dc94da5150a7569daf33458f6a6d975b9efa
splat_clauses:
- key: source_rules
  text: '# Satyr Medium Fey, Chaotic Neutral AC 13 Initiative +3 (13) HP 31 (7d8) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 12 +1 +1 Dex 16 +3 +3 Con 11 +0 +0 Int 12 +1 +1 Wis 10 +0 +0 Cha 14 +2 +2 Skills
    Perception +2, Performance +6, Stealth +5 Senses Passive Perception 12 Languages Common, Elvish, Sylvan
    CR 1/2 (XP 100; PB +2) ### Traits Magic Resistance. The satyr has Advantage on saving throws against
    spells and other magical effects. ### Actions Hooves. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4
    + 3) Bludgeoning damage. If the target is a Medium or smaller creature, the satyr pushes the target
    up to 10 feet straight away from itself. Mockery. Wisdom Saving Throw: DC 12, one creature the satyr
    can see within 90 feet. Failure: 5 (1d6 + 2) Psychic damage.'
  sha256: dba0202350735ace1363be3d40888970004ac6149f65cabb0fc4df89da1d0167
  status: contract_specified
splat_requires_mechanics: *id001
---

# Satyr

<!-- SRD page 321 -->

Medium Fey, Chaotic Neutral\
AC 13 Initiative +3 (13)\
HP 31 (7d8)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 16 +3 +3 Con 11 +0 +0\
Int 12 +1 +1 Wis 10 +0 +0 Cha 14 +2 +2\
Skills Perception +2, Performance +6, Stealth +5\
Senses Passive Perception 12\
Languages Common, Elvish, Sylvan\
CR 1/2 (XP 100; PB +2)\

<!-- SRD page 322 -->


### Traits

Magic Resistance. The satyr has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Hooves. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4\
+ 3) Bludgeoning damage. If the target is a Medium or\
smaller creature, the satyr pushes the target up to 10\
feet straight away from itself.\
Mockery. Wisdom Saving Throw: DC 12, one creature\
the satyr can see within 90 feet. Failure: 5 (1d6 + 2)\
Psychic damage.\

Original layout: [[../../srd-5.2.1/pages/321|page 321]], [[../../srd-5.2.1/pages/322|page 322]].
