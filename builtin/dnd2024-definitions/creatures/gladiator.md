---
splat_schema: 1
splat_id: af7b6e6c-088d-5f7c-be06-016ec7d3d773
splat_kind: creature
splat_key: dnd2024:creatures/gladiator
splat_name: Gladiator
splat_origin:
  key: dnd2024:reference/creature/gladiator
splat_sources:
- title: SRD 5.2.1
  locator: pp. 289
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Athletics +10, Performance +5
  gear_text: Shield, Spears (3), Studded Leather Armor
  senses_text: Passive Perception 11
  languages_text: Common
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 112 (15d8 + 45)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 7
  dexterity:
    value: 15
  save_dexterity:
    value: 5
  constitution:
    value: 16
  save_constitution:
    value: 6
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 16
  max_hp:
    value: 112
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/gladiator
  implementation: dnd2024:creature_gladiator
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: spear
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: shield_bash
      spear:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 5
          range_ft: 20
          long_range_ft: 60
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: piercing
      shield_bash:
        economy: action
        operation: saving_throw
        ability: strength
        dc: 15
        targets:
          kind: creature
          count: 1
          range_ft: 5
          visible: true
        failure:
          damage:
          - average: 9
            roll:
              count: 2
              sides: 4
              modifier: 4
            type: bludgeoning
          prone:
            maximum_size: medium
        success:
          effects: []
      parry:
        economy: reaction
        operation: modify_ac
        trigger: self.hit_by_melee_attack
        requires:
          holding_weapon: true
        bonus: 3
        scope: triggering_attack
        reevaluate_hit: true
    skill_totals:
      athletics: 10
      performance: 5
    gear:
    - item: shield
      quantity: 1
    - item: spears
      quantity: 3
    - item: studded_leather_armor
      quantity: 1
    senses:
      passive_perception: 11
    languages:
      speaks:
      - common
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
- id: dnd2024:creature_gladiator
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
  source_sha256: 09d53cf2f4c1f3f3fd410e9794c9187b02ebd54eed22cb78d34d2edce1f3052d
  source_path: builtin/srd-5.2.1/creature/gladiator.md
  pages:
  - 289
  engine_gaps: &id001
  - dnd2024:creature_gladiator
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 134b4e414afc99e0fa198b6f1428d7aed074d4abfa37427eecb6c889e4ba30e3
splat_clauses:
- key: source_rules
  text: '# Gladiator Medium or Small Humanoid, Neutral AC 16 Initiative +5 (15) HP 112 (15d8 + 45) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +7 Dex 15 +2 +5 Con 16 +3 +6 Int 10 +0 +0 Wis 12 +1 +4
    Cha 15 +2 +2 Skills Athletics +10, Performance +5 Gear Shield, Spears (3), Studded Leather Armor Senses
    Passive Perception 11 Languages Common CR 5 (XP 1,800; PB +3) ### Actions Multiattack. The gladiator
    makes three Spear attacks. It can replace one attack with a use of Shield Bash. Spear. Melee or Ranged
    Attack Roll: +7, reach 5 ft. or range 20/60 ft. Hit: 11 (2d6 + 4) Piercing damage. Shield Bash. Strength
    Saving Throw: DC 15, one creature within 5 feet that the gladiator can see. Failure: 9 (2d4 + 4) Bludgeoning
    damage. If the target is a Medium or smaller creature, it has the Prone condition. ### Reactions Parry.
    Trigger: The gladiator is hit by a melee attack roll while holding a weapon. Response: The gladiator
    adds 3 to its AC against that attack, possibly causing it to miss.'
  sha256: a98f0936d19f0e120100f133cd5844463e6d4c48f8ac2b4e6372d52585c48351
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gladiator

<!-- SRD page 289 -->

Medium or Small Humanoid, Neutral\
AC 16 Initiative +5 (15)\
HP 112 (15d8 + 45)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +7 Dex 15 +2 +5 Con 16 +3 +6\
Int 10 +0 +0 Wis 12 +1 +4 Cha 15 +2 +2\
Skills Athletics +10, Performance +5\
Gear Shield, Spears (3), Studded Leather Armor\
Senses Passive Perception 11\
Languages Common\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The gladiator makes three Spear attacks. It\
can replace one attack with a use of Shield Bash.\
Spear. Melee or Ranged Attack Roll: +7, reach 5 ft. or\
range 20/60 ft. Hit: 11 (2d6 + 4) Piercing damage.\
Shield Bash. Strength Saving Throw: DC 15, one crea-\
ture within 5 feet that the gladiator can see. Failure: 9\
(2d4 + 4) Bludgeoning damage. If the target is a Me-\
dium or smaller creature, it has the Prone condition.\

### Reactions

Parry. Trigger: The gladiator is hit by a melee attack\
roll while holding a weapon. Response: The gladiator\
adds 3 to its AC against that attack, possibly causing\
it to miss.\

Original layout: [[../../srd-5.2.1/pages/289|page 289]].
