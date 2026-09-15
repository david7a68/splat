---
splat_schema: 1
splat_id: 4716d703-8690-5db4-80ac-cd5400ee9f4b
splat_kind: creature
splat_key: dnd2024:creatures/seahorse
splat_name: Seahorse
splat_origin:
  key: dnd2024:reference/creature/seahorse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 20 ft.
  skills_text: Perception +2, Stealth +5
  senses_text: Passive Perception 12
  languages_text: None
  cr_text: 0 (XP 0; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 1
  save_strength:
    value: -5
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 12
  max_hp:
    value: 1
  initiative:
    value: 1
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/seahorse
  implementation: dnd2024:creature_seahorse
  parameters:
    features:
      water_breathing:
        economy: passive
        breathing_media:
        - water
      bubble_dash:
        economy: action
        requires:
          underwater: true
        op: move
        distance_max:
          multiply:
          - stat: swim_speed
          - 1
        provokes_opportunity_attacks: false
    skill_totals:
      perception: 2
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 12
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
- id: dnd2024:creature_seahorse
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
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: bda7248c4e6788d52ad13a90576f4be72a907484897f5cf28383c9f0d9a45702
  source_path: builtin/srd-5.2.1/creature/seahorse.md
  pages:
  - 361
  engine_gaps: &id001
  - dnd2024:creature_seahorse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: cf8ecccf3c26f2ac344620379dd8f0ea2a5f17cdfcc92e8759ba90bee5cb64cd
splat_clauses:
- key: source_rules
  text: '# Seahorse Tiny Beast, Unaligned AC 12 Initiative +1 (11) HP 1 (1d4 - 1) Speed 5 ft., Swim 20
    ft. MOD SAVE MOD SAVE MOD SAVE Str 1 -5 -5 Dex 12 +1 +1 Con 8 -1 -1 Int 1 -5 -5 Wis 10 +0 +0 Cha 2
    -4 -4 Skills Perception +2, Stealth +5 Senses Passive Perception 12 Languages None CR 0 (XP 0; PB
    +2) ### Traits Water Breathing. The seahorse can breathe only underwater. ### Actions Bubble Dash.
    While underwater, the seahorse moves up to its Swim Speed without provoking Opportunity Attacks.'
  sha256: d0d27d9ebae66818a5fb3ff80fa52f0971698358d3e617061f23f50d941bd30d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Seahorse

<!-- SRD page 361 -->

Tiny Beast, Unaligned\
AC 12  Initiative +1 (11)\
HP 1 (1d4 − 1)\
Speed 5 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 12 +1 +1 Con 8 −1 −1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 2 −4 −4\
Skills Perception +2, Stealth +5\
Senses Passive Perception 12\
Languages None\
CR 0 (XP 0; PB +2)\

### Traits

Water Breathing. The seahorse can breathe only\
underwater.\

### Actions

Bubble Dash. While underwater, the seahorse moves\
up to its Swim Speed without provoking Opportu-\
nity Attacks.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].
