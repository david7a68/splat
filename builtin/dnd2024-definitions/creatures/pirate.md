---
splat_schema: 1
splat_id: 5a3c78f9-2858-53fa-970d-95bf9c8d5bcb
splat_kind: creature
splat_key: dnd2024:creatures/pirate
splat_name: Pirate
splat_origin:
  key: dnd2024:reference/creature/pirate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 314
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Daggers (6), Leather Armor
  senses_text: Passive Perception 11
  languages_text: Common plus one other language
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 33 (6d8 + 6)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 16
  save_dexterity:
    value: 5
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 14
  save_charisma:
    value: 4
  armor_class:
    value: 14
  max_hp:
    value: 33
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pirate
  implementation: dnd2024:creature_pirate
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: dagger
          count: 2
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: enthralling_panache
      dagger:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
          range_ft: 20
          long_range_ft: 60
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: piercing
      enthralling_panache:
        economy: action
        operation: saving_throw
        ability: wisdom
        dc: 12
        targets:
          kind: creature
          count: 1
          range_ft: 30
          visible: true
        failure:
          conditions:
          - condition: charmed
            until: self.next_turn_start
        success:
          effects: []
    gear:
    - item: daggers
      quantity: 6
    - item: leather_armor
      quantity: 1
    senses:
      passive_perception: 11
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 1
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_pirate
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
  - turn.started
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 1100ff08d4d14f002eb298de128a4b6baead7961fb881425dfcb1fa873fadace
  source_path: builtin/srd-5.2.1/creature/pirate.md
  pages:
  - 314
  engine_gaps: &id001
  - dnd2024:creature_pirate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a30185bae883f448a64fd2eafe45f2446a9e597d085d4e5af85db0cfb5842865
splat_clauses:
- key: source_rules
  text: '# Pirate Medium or Small Humanoid, Neutral AC 14 Initiative +5 (15) HP 33 (6d8 + 6) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 16 +3 +5 Con 12 +1 +1 Int 8 -1 -1 Wis 12 +1 +1 Cha
    14 +2 +4 Gear Daggers (6), Leather Armor Senses Passive Perception 11 Languages Common plus one other
    language CR 1 (XP 200; PB +2) ### Actions Multiattack. The pirate makes two Dagger attacks. It can
    replace one attack with a use of Enthralling Panache. Dagger. Melee or Ranged Attack Roll: +5, reach
    5 ft. or range 20/60 ft. Hit: 5 (1d4 + 3) Piercing damage. Enthralling Panache. Wisdom Saving Throw:
    DC 12, one creature the pirate can see within 30 feet. Failure: The target has the Charmed condition
    until the start of the pirate’s next turn.'
  sha256: 168bd6dd48053eda72b0fab4499acabe6753c6bb1748f1b22d39c94da69e36fb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pirate

<!-- SRD page 314 -->

Medium or Small Humanoid, Neutral\
AC 14  Initiative +5 (15)\
HP 33 (6d8 + 6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +5 Con 12 +1 +1\
Int 8 −1 −1 Wis 12 +1 +1 Cha 14 +2 +4\
Gear Daggers (6), Leather Armor\
Senses Passive Perception 11\
Languages Common plus one other language\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The pirate makes two Dagger attacks.\
It can replace one attack with a use of Enthrall-\
ing Panache.\
Dagger. Melee or Ranged Attack Roll: +5, reach 5 ft. or\
range 20/60 ft. Hit: 5 (1d4 + 3) Piercing damage.\
Enthralling Panache. Wisdom Saving Throw: DC 12,\
one creature the pirate can see within 30 feet. Failure:\
The target has the Charmed condition until the start of\
the pirate’s next turn.\

Original layout: [[../../srd-5.2.1/pages/314|page 314]].
