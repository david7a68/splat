---
splat_schema: 1
splat_id: 0f221b09-8d98-5661-9896-385e53f3f1e6
splat_kind: creature
splat_key: dnd2024:creatures/worg
splat_name: Worg
splat_origin:
  key: dnd2024:reference/creature/worg
splat_sources:
- title: SRD 5.2.1
  locator: pp. 342
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fey
  alignment: Neutral Evil
  movement_text: 50 ft.
  skills_text: Perception +4
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Goblin, Worg
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 26 (4d10 + 4)
splat_stats:
  strength:
    value: 16
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
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 26
  initiative:
    value: 1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/worg
  implementation: dnd2024:creature_worg
  parameters:
    features:
      bite:
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
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: piercing
          next_attack_against_target:
            advantage: true
            expires: self.next_turn_start
            uses: 1
    skill_totals:
      perception: 4
    senses:
      darkvision: 60
      passive_perception: 14
    languages:
      speaks:
      - goblin
      - worg
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
- id: dnd2024:creature_worg
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
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 014d26dd75282bc08da6381276a020a94c9039f76361708ca008f3ce7b25bea8
  source_path: builtin/srd-5.2.1/creature/worg.md
  pages:
  - 342
  engine_gaps: &id001
  - dnd2024:creature_worg
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 1bca2b0ee825ac5225e25ded9dfec58e2f7a538c8af3be471dd4904813a0666c
splat_clauses:
- key: source_rules
  text: '# Worg Large Fey, Neutral Evil AC 13 Initiative +1 (11) HP 26 (4d10 + 4) Speed 50 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 16 +3 +3 Dex 13 +1 +1 Con 13 +1 +1 Int 7 -2 -2 Wis 11 +0 +0 Cha 8 -1 -1 Skills
    Perception +4 Senses Darkvision 60 ft.; Passive Perception 14 Languages Goblin, Worg CR 1/2 (XP 100;
    PB +2) ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage, and
    the next attack roll made against the target before the start of the worg’s next turn has Advantage.'
  sha256: daca7fb3881abbda01c779e095b9658b6ddebe0ce608c43f1d867ad2920856e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Worg

<!-- SRD page 342 -->

Large Fey, Neutral Evil\
AC 13 Initiative +1 (11)\
HP 26 (4d10 + 4)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
Int 7 −2 −2 Wis 11 +0 +0 Cha 8 −1 −1\
Skills Perception +4\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Goblin, Worg\
CR 1/2 (XP 100; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage, and the next attack roll made against\
the target before the start of the worg’s next turn has\
Advantage.\

Original layout: [[../../srd-5.2.1/pages/342|page 342]].
