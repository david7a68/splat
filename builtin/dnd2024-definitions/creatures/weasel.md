---
splat_schema: 1
splat_id: 5c49d536-cb5a-59ea-821a-e5df052847eb
splat_kind: creature
splat_key: dnd2024:creatures/weasel
splat_name: Weasel
splat_origin:
  key: dnd2024:reference/creature/weasel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 364
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Acrobatics +5, Perception +3, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 1
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/weasel
  implementation: dnd2024:creature_weasel
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: piercing
        reach_feet: 5
    skill_totals:
      acrobatics: 5
      perception: 3
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 13
      special:
      - sense: darkvision
        range_feet: 60
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
- id: dnd2024:creature_weasel
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
  source_sha256: fed661cf68b9c291185670a824152dee1e7576c1296ac2b2f2177e01a18d443d
  source_path: builtin/srd-5.2.1/creature/weasel.md
  pages:
  - 364
  engine_gaps: &id001
  - dnd2024:creature_weasel
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/weasel
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 53244459f8946160b2adb3914ea51004aed3c190e0b6c208eea0c2b8f8e8923e
splat_clauses:
- key: source_rules
  text: '# Weasel Tiny Beast, Unaligned AC 13 Initiative +3 (13) HP 1 (1d4 - 1) Speed 30 ft., Climb 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 3 -4 -4 Dex 16 +3 +3 Con 8 -1 -1 Int 2 -4 -4 Wis 12 +1 +1 Cha 3
    -4 -4 Skills Acrobatics +5, Perception +3, Stealth +5 Senses Darkvision 60 ft.; Passive Perception
    13 Languages None CR 0 (XP 10; PB +2) ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 1
    Piercing damage.'
  sha256: fdcd6d5fb76e3f0a189c907a3fee209d6dba98218e1cb49cb9f3a4996a15d8ee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weasel

<!-- SRD page 364 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 16 +3 +3 Con 8 −1 −1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 3 −4 −4\
Skills Acrobatics +5, Perception +3, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/364|page 364]].
