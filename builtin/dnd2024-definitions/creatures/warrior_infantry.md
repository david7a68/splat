---
splat_schema: 1
splat_id: 885c82de-0918-50a6-90e1-c501b866da58
splat_kind: creature
splat_key: dnd2024:creatures/warrior_infantry
splat_name: Warrior Infantry
splat_origin:
  key: dnd2024:reference/creature/warrior_infantry
splat_sources:
- title: SRD 5.2.1
  locator: pp. 336, 337
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Chain Shirt, Spear
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 9 (2d8)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 8
  save_intelligence:
    value: -1
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
    value: 9
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/warrior_infantry
  implementation: dnd2024:creature_warrior_infantry
  parameters:
    features:
      pack_tactics:
        economy: passive
        advantage:
          roll: attack
          target_kind: creature
          requires:
            ally_distance_to_target_max_feet: 5
            ally_incapacitated: false
      spear:
        economy: action
        op: attack
        attack_modes:
        - melee
        - ranged
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 6
            add: 1
          damage_type: piercing
        reach_feet: 5
        range_feet: 20
        long_range_feet: 60
    defenses: {}
    senses:
      passive_perception: 10
      special: []
    gear:
    - item_name: Chain Shirt
      quantity: 1
    - item_name: Spear
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
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
- id: dnd2024:creature_warrior_infantry
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
  source_sha256: d3db9e2dbf274335f883bfef75233bdaa63c8501c3cff7e97ca50eeac8b8c32e
  source_path: builtin/srd-5.2.1/creature/warrior_infantry.md
  pages:
  - 336
  - 337
  engine_gaps: &id001
  - dnd2024:creature_warrior_infantry
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d0f63fcb4f4646083b4736f210d181d71d8dfcde4d668de1fab23d9e92df71a3
splat_clauses:
- key: source_rules
  text: '# Warrior Infantry Medium or Small Humanoid, Neutral AC 13 Initiative +0 (10) HP 9 (2d8) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 11 +0 +0 Con 11 +0 +0 Int 8 -1 -1 Wis 11 +0 +0
    Cha 8 -1 -1 Gear Chain Shirt, Spear Senses Passive Perception 10 Languages Common CR 1/8 (XP 25; PB
    +2) ### Traits Pack Tactics. The warrior has Advantage on an attack roll against a creature if at
    least one of the warrior’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated
    condition. ### Actions Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or range 20/60 ft. Hit:
    4 (1d6 + 1) Piercing damage.'
  sha256: 86961d7ed034c408d746d116fe29388103730ceb8d3a34a867c594a2382db1ba
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warrior Infantry

<!-- SRD page 336 -->

Medium or Small Humanoid, Neutral\
AC 13 Initiative +0 (10)\
HP 9 (2d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 11 +0 +0 Con 11 +0 +0\
Int 8 −1 −1 Wis 11 +0 +0 Cha 8 −1 −1\
Gear Chain Shirt, Spear\
Senses Passive Perception 10\
Languages Common\
CR 1/8 (XP 25; PB +2)\

<!-- SRD page 337 -->


### Traits

Pack Tactics. The warrior has Advantage on an attack\
roll against a creature if at least one of the warrior’s al-\
lies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\

### Actions

Spear. Melee or Ranged Attack Roll: +3, reach 5 ft. or\
range 20/60 ft. Hit: 4 (1d6 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/336|page 336]], [[../../srd-5.2.1/pages/337|page 337]].
