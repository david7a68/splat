---
splat_schema: 1
splat_id: 4a2827a5-4abb-5488-965b-d32d2333a195
splat_kind: creature
splat_key: dnd2024:creatures/giant_elk
splat_name: Giant Elk
splat_origin:
  key: dnd2024:reference/creature/giant_elk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 351
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: celestial
  alignment: Neutral Good
  movement_text: 60 ft.
  skills_text: Perception +4
  resistances_text: Necrotic, Radiant
  senses_text: Darkvision 90 ft.; Passive Perception 14
  languages_text: Celestial; understands Common, Elvish, and Sylvan but can’t speak them
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 42 (5d12 + 10)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 6
  dexterity:
    value: 18
  save_dexterity:
    value: 6
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 14
  max_hp:
    value: 42
  initiative:
    value: 6
  speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_elk
  implementation: dnd2024:creature_giant_elk
  parameters:
    features:
      ram:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 6
            add: 4
          damage_type: bludgeoning
        - op: damage
          average: 5
          amount:
            dice:
              count: 2
              sides: 4
            add: 0
          damage_type: radiant
        - op: if
          test:
            target_kind: creature
            target_size_max: huge
            immediately_preceding_movement:
              minimum_feet: 20
              direction: straight_toward_target
          then:
          - op: damage
            average: 5
            amount:
              dice:
                count: 2
                sides: 4
              add: 0
            damage_type: bludgeoning
          - op: condition
            condition: prone
        reach_feet: 10
    skill_totals:
      perception: 4
    defenses:
      resistances:
        damage_types:
        - necrotic
        - radiant
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 90
    communication:
      speaks:
      - celestial
      understands:
      - celestial
      - common
      - elvish
      - sylvan
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
- id: dnd2024:creature_giant_elk
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
  source_sha256: 352c3e2dd727ce63c9ed71527dbfec4522f5096942890af7b0626a36b0efa219
  source_path: builtin/srd-5.2.1/creature/giant_elk.md
  pages:
  - 351
  engine_gaps: &id001
  - dnd2024:creature_giant_elk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 4d3f80b66c0d56acf2b52e2704bb3e3112d2537a6a7b550b9fa525e6dcd7dbad
splat_clauses:
- key: source_rules
  text: '# Giant Elk Huge Celestial, Neutral Good AC 14 Initiative +6 (16) HP 42 (5d12 + 10) Speed 60
    ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +6 Dex 18 +4 +6 Con 14 +2 +2 Int 7 -2 -2 Wis 14 +2 +2 Cha
    10 +0 +0 Skills Perception +4 Resistances Necrotic, Radiant Senses Darkvision 90 ft.; Passive Perception
    14 Languages Celestial; understands Common, Elvish, and Sylvan but can’t speak them CR 2 (XP 450;
    PB +2) ### Actions Ram. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Bludgeoning damage plus
    5 (2d4) Radiant damage. If the target is a Huge or smaller creature and the elk moved 20+ feet straight
    toward it immediately before the hit, the target takes an extra 5 (2d4) Bludgeoning damage and has
    the Prone condition.'
  sha256: f0ac382ce8958d7d936ff353184fa8c7f5b807f0a2a8a9898d881859d4d5862f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Elk

<!-- SRD page 351 -->

Huge Celestial, Neutral Good\
AC 14 Initiative +6 (16)\
HP 42 (5d12 + 10)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +6 Dex 18 +4 +6 Con 14 +2 +2\
Int 7 −2 −2 Wis 14 +2 +2 Cha 10 +0 +0\
Skills Perception +4\
Resistances Necrotic, Radiant\
Senses Darkvision 90 ft.; Passive Perception 14\
Languages Celestial; understands Common, Elvish, and\
Sylvan but can’t speak them\
CR 2 (XP 450; PB +2)\

### Actions

Ram. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6\
+ 4) Bludgeoning damage plus 5 (2d4) Radiant damage.\
If the target is a Huge or smaller creature and the elk\
moved 20+ feet straight toward it immediately before\
the hit, the target takes an extra 5 (2d4) Bludgeoning\
damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/351|page 351]].
