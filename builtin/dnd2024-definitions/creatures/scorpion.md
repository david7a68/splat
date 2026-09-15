---
splat_schema: 1
splat_id: 108d8349-a58e-56b2-99d6-524205e33c86
splat_kind: creature
splat_key: dnd2024:creatures/scorpion
splat_name: Scorpion
splat_origin:
  key: dnd2024:reference/creature/scorpion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 10 ft.
  senses_text: Blindsight 10 ft.; Passive Perception 9
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 1
  initiative:
    value: 0
  speed:
    value: 10
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/scorpion
  implementation: dnd2024:creature_scorpion
  parameters:
    features:
      sting:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 2
        targets:
          count: 1
        hit:
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: piercing
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 5
    defenses: {}
    senses:
      passive_perception: 9
      special:
      - sense: blindsight
        range_feet: 10
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
- id: dnd2024:creature_scorpion
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
  source_sha256: ed0c55453657a66bb0f75c544b1c700ec60f55fa4a073c5a8e4acb3afceb0801
  source_path: builtin/srd-5.2.1/creature/scorpion.md
  pages:
  - 360
  engine_gaps: &id001
  - dnd2024:creature_scorpion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 40106de46f0302784223699f590335c4b3dcf21f4bc4fde94e5af4c7a2c801e0
splat_clauses:
- key: source_rules
  text: '# Scorpion Tiny Beast, Unaligned AC 11 Initiative +0 (10) HP 1 (1d4 - 1) Speed 10 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 2 -4 -4 Dex 11 +0 +0 Con 8 -1 -1 Int 1 -5 -5 Wis 8 -1 -1 Cha 2 -4 -4 Senses
    Blindsight 10 ft.; Passive Perception 9 Languages None CR 0 (XP 10; PB +2) ### Actions Sting. Melee
    Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage plus 3 (1d6) Poison damage.'
  sha256: a0984b659952478a7bb81db866f1595ca7eeafc71b5db54e1313188236a1cdeb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scorpion

<!-- SRD page 360 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 1 (1d4 − 1)\
Speed 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 11 +0 +0 Con 8 −1 −1\
Int 1 −5 −5 Wis 8 −1 −1 Cha 2 −4 −4\
Senses Blindsight 10 ft.; Passive Perception 9\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Sting. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing\
damage plus 3 (1d6) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/360|page 360]].
