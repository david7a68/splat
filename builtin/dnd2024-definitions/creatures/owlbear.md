---
splat_schema: 1
splat_id: 72cc0187-32ce-54cd-b408-09dc5fb4205c
splat_kind: creature
splat_key: dnd2024:creatures/owlbear
splat_name: Owlbear
splat_origin:
  key: dnd2024:reference/creature/owlbear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 59 (7d10 + 21)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 59
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/owlbear
  implementation: dnd2024:creature_owlbear
  parameters:
    features:
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 14
          amount:
            dice:
              count: 2
              sides: 8
            add: 5
          damage_type: slashing
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 2
    skill_totals:
      perception: 5
    defenses: {}
    senses:
      passive_perception: 15
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
- id: dnd2024:creature_owlbear
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
  source_sha256: 9e6bc93dfded1047b1aa5481f158a805a65a73a9e46850ac606188f725b11bc7
  source_path: builtin/srd-5.2.1/creature/owlbear.md
  pages:
  - 313
  engine_gaps: &id001
  - dnd2024:creature_owlbear
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/owlbear
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 37a1be79bd7681402385b4a778e123061298e464d7b0cc2625c8131443025805
splat_clauses:
- key: source_rules
  text: '# Owlbear Large Monstrosity, Unaligned AC 13 Initiative +1 (11) HP 59 (7d10 + 21) Speed 40 ft.,
    Climb 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 12 +1 +1 Con 17 +3 +3 Int 3 -4 -4 Wis 12
    +1 +1 Cha 7 -2 -2 Skills Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages None
    CR 3 (XP 700; PB +2) ### Actions Multiattack. The owlbear makes two Rend attacks. Rend. Melee Attack
    Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage.'
  sha256: 4e41cdca42852bcbaaa891f7ec39cf018cb9a01b542ebb2653f06376c7e178d2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Owlbear

<!-- SRD page 313 -->

Large Monstrosity, Unaligned\
AC 13 Initiative +1 (11)\
HP 59 (7d10 + 21)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 17 +3 +3\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The owlbear makes two Rend attacks.\
Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8\
+ 5) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].
