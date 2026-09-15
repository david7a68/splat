---
splat_schema: 1
splat_id: 921e78dc-60f0-595a-9332-a0261b8abd5a
splat_kind: creature
splat_key: dnd2024:creatures/manticore
splat_name: Manticore
splat_origin:
  key: dnd2024:reference/creature/manticore
splat_sources:
- title: SRD 5.2.1
  locator: pp. 306
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Lawful Evil
  movement_text: 30 ft., Fly 50 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 68 (8d10 + 24)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 14
  max_hp:
    value: 68
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/manticore
  implementation: dnd2024:creature_manticore
  parameters:
    features:
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: slashing
        reach_feet: 5
      tail_spike:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: piercing
        range_feet: 100
        long_range_feet: 200
      multiattack:
        economy: action
        op: repeat_choice
        count: 3
        choices:
        - feature: rend
        - feature: tail_spike
    defenses: {}
    senses:
      passive_perception: 11
      special:
      - sense: darkvision
        range_feet: 60
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
- id: dnd2024:creature_manticore
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
  source_sha256: 29b953b54b0215a2f4cc24eeb0d56e837c8a45e916793b398fef845bd636de79
  source_path: builtin/srd-5.2.1/creature/manticore.md
  pages:
  - 306
  engine_gaps: &id001
  - dnd2024:creature_manticore
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: b3cff8c97c69e4ab82c371ed9cfc76076804e57214126a68d5bcbcc7624b1e52
splat_clauses:
- key: source_rules
  text: '# Manticore Large Monstrosity, Lawful Evil AC 14 Initiative +3 (13) HP 68 (8d10 + 24) Speed 30
    ft., Fly 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 16 +3 +3 Con 17 +3 +3 Int 7 -2 -2 Wis
    12 +1 +1 Cha 8 -1 -1 Senses Darkvision 60 ft.; Passive Perception 11 Languages Common CR 3 (XP 700;
    PB +2) ### Actions Multiattack. The manticore makes three attacks, using Rend or Tail Spike in any
    combination. Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage. Tail Spike.
    Ranged Attack Roll: +5, range 100/200 ft. Hit: 7 (1d8 + 3) Piercing damage.'
  sha256: 712d665c84e91334d441f85c57811b3dc9acb29e0cab4ecdcfd3fde97fb4a2c2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Manticore

<!-- SRD page 306 -->

Large Monstrosity, Lawful Evil\
AC 14 Initiative +3 (13)\
HP 68 (8d10 + 24)\
Speed 30 ft., Fly 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 16 +3 +3 Con 17 +3 +3\
Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The manticore makes three attacks, using\
Rend or Tail Spike in any combination.\
Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Slashing damage.\
Tail Spike. Ranged Attack Roll: +5, range 100/200 ft.\
Hit: 7 (1d8 + 3) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/306|page 306]].
