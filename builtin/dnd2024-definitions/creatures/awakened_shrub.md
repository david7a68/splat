---
splat_schema: 1
splat_id: 5afec75c-7981-5ce1-b7f7-3469de75b409
splat_kind: creature
splat_key: dnd2024:creatures/awakened_shrub
splat_name: Awakened Shrub
splat_origin:
  key: dnd2024:reference/creature/awakened_shrub
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: plant
  alignment: Neutral
  movement_text: 20 ft.
  resistances_text: Piercing
  vulnerabilities_text: Fire
  senses_text: Passive Perception 10
  languages_text: Common plus one other language
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 10 (3d6)
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 9
  max_hp:
    value: 10
  initiative:
    value: -1
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/awakened_shrub
  implementation: dnd2024:creature_awakened_shrub
  parameters:
    features:
      rake:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 1
        targets:
          count: 1
        hit:
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: slashing
        reach_feet: 5
    defenses:
      vulnerabilities:
        damage_types:
        - fire
      resistances:
        damage_types:
        - piercing
    senses:
      passive_perception: 10
      special: []
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
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
- id: dnd2024:creature_awakened_shrub
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
  source_sha256: 3956e1517c8e49e920c9bdb208fef3dc036a51012b810615b71dc5e9a839ee98
  source_path: builtin/srd-5.2.1/creature/awakened_shrub.md
  pages:
  - 260
  engine_gaps: &id001
  - dnd2024:creature_awakened_shrub
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/awakened_shrub
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: bbefd11681e49fdc3dfc40518ef1505638ab53f9a910b884886f800d289a4a18
splat_clauses:
- key: source_rules
  text: '# Awakened Shrub Small Plant, Neutral AC 9 Initiative -1 (9) HP 10 (3d6) Speed 20 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 3 -4 -4 Dex 8 -1 -1 Con 11 +0 +0 Int 10 +0 +0 Wis 10 +0 +0 Cha 6 -2 -2 Vulnerabilities
    Fire Resistances Piercing Senses Passive Perception 10 Languages Common plus one other language CR
    0 (XP 10; PB +2) ### Actions Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slashing damage.'
  sha256: 93916d3718c2bcf2d2b3de6ffc19fcd8c6b522bff6d97aad41db477cfd8bb380
  status: contract_specified
splat_requires_mechanics: *id001
---

# Awakened Shrub

<!-- SRD page 260 -->

Small Plant, Neutral\
AC 9 Initiative −1 (9)\
HP 10 (3d6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 8 −1 −1 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 6 −2 −2\
Vulnerabilities Fire\
Resistances Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 0 (XP 10; PB +2)\

### Actions

Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].
