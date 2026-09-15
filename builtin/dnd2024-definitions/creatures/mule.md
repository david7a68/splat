---
splat_schema: 1
splat_id: 40b2a9a6-ad8a-554f-8b8b-eb21dd929b2e
splat_kind: creature
splat_key: dnd2024:creatures/mule
splat_name: Mule
splat_origin:
  key: dnd2024:reference/creature/mule
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
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
    value: 10
  max_hp:
    value: 11
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mule
  implementation: dnd2024:creature_mule
  parameters:
    features:
      hooves:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 4
            add: 2
          damage_type: bludgeoning
        reach_feet: 5
      beast_of_burden:
        economy: passive
        carrying_capacity_size_increment: 1
    defenses: {}
    senses:
      passive_perception: 10
      special: []
    communication:
      speaks: []
      understands: []
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_mule
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events: []
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9ee802ba184d603449b7bc44c0708f7aa2296d041c67ba76a7b5c5b0cf9247f8
  source_path: builtin/srd-5.2.1/creature/mule.md
  pages:
  - 357
  engine_gaps: &id001
  - dnd2024:creature_mule
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f3a815dda7fb0f4947b8725bb899a12573f84adb2182e15a5b968c524fee3e77
splat_clauses:
- key: source_rules
  text: '# Mule Medium Beast, Unaligned AC 10 Initiative +0 (10) HP 11 (2d8 + 2) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 14 +2 +4 Dex 10 +0 +0 Con 13 +1 +1 Int 2 -4 -4 Wis 10 +0 +0 Cha 5 -3 -3 Senses
    Passive Perception 10 Languages None CR 1/8 (XP 25; PB +2) ### Traits Beast of Burden. The mule counts
    as one size larger for the purpose of determining its carrying capacity. ### Actions Hooves. Melee
    Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Bludgeoning damage.'
  sha256: 3fa9f39bbd64bded8e866bf41ee2a70f093f96c8667e7a50c4ad2252e885f30a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mule

<!-- SRD page 357 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 11 (2d8 + 2)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +4 Dex 10 +0 +0 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha  5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Traits

Beast of Burden. The mule counts as one size larger for\
the purpose of determining its carrying capacity.\

### Actions

Hooves. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4\
+ 2) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/357|page 357]].
