---
splat_schema: 1
splat_id: 034f7f82-aba5-5784-b239-2f9bc92ae296
splat_kind: creature
splat_key: dnd2024:creatures/venomous_snake
splat_name: Venomous Snake
splat_origin:
  key: dnd2024:reference/creature/venomous_snake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  senses_text: Blindsight 10 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 5 (2d4)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 12
  max_hp:
    value: 5
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/venomous_snake
  implementation: dnd2024:creature_venomous_snake
  parameters:
    features:
      bite:
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
      passive_perception: 10
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
- id: dnd2024:creature_venomous_snake
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
  source_sha256: 6dd3f433d2d497054ee28cbd63a96f9edf6b82e040434f7b3089a81bcc9eb47e
  source_path: builtin/srd-5.2.1/creature/venomous_snake.md
  pages:
  - 363
  engine_gaps: &id001
  - dnd2024:creature_venomous_snake
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d15c0b77b4b7f72e766c5b8a2eb1a02a8870ba5c9b84d7253b1d4629852c9eab
splat_clauses:
- key: source_rules
  text: '# Venomous Snake Tiny Beast, Unaligned AC 12 Initiative +2 (12) HP 5 (2d4) Speed 30 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 15 +2 +2 Con11 +0 +0 Int 1 -5 -5 Wis 10 +0 +0 Cha
    3 -4 -4 Senses Blindsight 10 ft.; Passive Perception 10 Languages None CR 1/8 (XP 25; PB +2) ### Actions
    Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage plus 3 (1d6) Poison damage.'
  sha256: c17849360fe3b1e7fff00b64bc5f735d968974847ec3d83541a43ac7769b390a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Venomous Snake

<!-- SRD page 363 -->

Tiny Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 5 (2d4)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 15 +2 +2 Con11 +0 +0\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Blindsight 10 ft.; Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Piercing damage plus 3 (1d6) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/363|page 363]].
