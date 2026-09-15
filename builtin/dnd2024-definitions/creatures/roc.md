---
splat_schema: 1
splat_id: 9eac7390-434c-519f-b483-0c6612df0bc3
splat_kind: creature
splat_key: dnd2024:creatures/roc
splat_name: Roc
splat_origin:
  key: dnd2024:reference/creature/roc
splat_sources:
- title: SRD 5.2.1
  locator: pp. 320
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Unaligned
  movement_text: 20 ft., Fly 120 ft.
  skills_text: Perception +8
  senses_text: Passive Perception 18
  languages_text: None
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 248 (16d20 + 80)
splat_stats:
  strength:
    value: 28
  save_strength:
    value: 9
  dexterity:
    value: 10
  save_dexterity:
    value: 4
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 4
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 15
  max_hp:
    value: 248
  initiative:
    value: 8
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 120
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/roc
  implementation: dnd2024:creature_roc
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: beak
          count: 2
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: talons
      beak:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 13
          reach_ft: 10
        hit:
          damage:
          - average: 28
            roll:
              count: 3
              sides: 12
              modifier: 9
            type: piercing
      talons:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 13
          reach_ft: 5
        hit:
          damage:
          - average: 23
            roll:
              count: 4
              sides: 6
              modifier: 9
            type: slashing
          grapple:
            condition: grappled
            source: self
            escape_dc: 19
            maximum_size: huge
            appendage_pool:
              count: 2
              type: talons
              cost: 2
            while_grappled:
              conditions:
              - restrained
      swoop:
        economy: bonus_action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: sequence
        requires:
          has_grappled_creature: true
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: fly_speed
            - 0.5
          provoke_opportunity_attacks: false
        - operation: drop_grappled_creature
    skill_totals:
      perception: 8
    senses:
      passive_perception: 18
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
- id: dnd2024:creature_roc
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
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: d2237f3693d91986578a587277882706321e98d4ed913f3e8bb4d3f6bfee7535
  source_path: builtin/srd-5.2.1/creature/roc.md
  pages:
  - 320
  engine_gaps: &id001
  - dnd2024:creature_roc
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 32e544421297969aefd8709a3e8a6c2fd303338302f286b1f155435d4bcb49f0
splat_clauses:
- key: source_rules
  text: '# Roc Gargantuan Monstrosity, Unaligned AC 15 Initiative +8 (18) HP 248 (16d20 + 80) Speed 20
    ft., Fly 120 ft. MOD SAVE MOD SAVE MOD SAVE Str 28 +9 +9 Dex 10 +0 +4 Con 20 +5 +5 Int 3 -4 -4 Wis
    10 +0 +4 Cha 9 -1 -1 Skills Perception +8 Senses Passive Perception 18 Languages None CR 11 (XP 7,200;
    PB +4) ### Actions Multiattack. The roc makes two Beak attacks. It can replace one attack with a Talons
    attack. Beak. Melee Attack Roll: +13, reach 10 ft. Hit: 28 (3d12 + 9) Piercing damage. Talons. Melee
    Attack Roll: +13, reach 5 ft. Hit: 23 (4d6 + 9) Slashing damage. If the target is a Huge or smaller
    creature, it has the Grappled condition (escape DC 19) from both talons, and it has the Restrained
    condition until the grapple ends. ### Bonus Actions Swoop (Recharge 5–6). If the roc has a creature
    Grappled, the roc flies up to half its Fly Speed without provoking Opportunity Attacks and drops that
    creature.'
  sha256: 9833813d27c5d25927e409089254173c8f33f5da501cde47b88a8a024ed844f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Roc

<!-- SRD page 320 -->

Gargantuan Monstrosity, Unaligned\
AC 15 Initiative +8 (18)\
HP 248 (16d20 + 80)\
Speed 20 ft., Fly 120 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 28 +9 +9 Dex 10 +0 +4 Con 20 +5 +5\
Int 3 −4 −4 Wis 10 +0 +4 Cha 9 −1 −1\
Skills Perception +8\
Senses Passive Perception 18\
Languages None\
CR 11 (XP 7,200; PB +4)\

### Actions

Multiattack. The roc makes two Beak attacks. It can\
replace one attack with a Talons attack.\
Beak. Melee Attack Roll: +13, reach 10 ft. Hit: 28 (3d12\
+ 9) Piercing damage.\
Talons. Melee Attack Roll: +13, reach 5 ft. Hit: 23 (4d6\
+ 9) Slashing damage. If the target is a Huge or smaller\
creature, it has the Grappled condition (escape DC 19)\
from both talons, and it has the Restrained condition\
until the grapple ends.\

### Bonus Actions

Swoop (Recharge 5–6). If the roc has a creature Grap-\
pled, the roc flies up to half its Fly Speed without pro-\
voking Opportunity Attacks and drops that creature.\

Original layout: [[../../srd-5.2.1/pages/320|page 320]].
