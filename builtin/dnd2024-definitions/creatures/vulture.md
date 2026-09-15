---
splat_schema: 1
splat_id: cb038f77-0dc2-56f4-a91d-e123d1acb532
splat_kind: creature
splat_key: dnd2024:creatures/vulture
splat_name: Vulture
splat_origin:
  key: dnd2024:reference/creature/vulture
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363, 364
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 50 ft.
  skills_text: Perception +3
  senses_text: Passive Perception 13
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 5 (1d8 + 1)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
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
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 10
  max_hp:
    value: 5
  initiative:
    value: 0
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/vulture
  implementation: dnd2024:creature_vulture
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
      beak:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 2
        targets:
          count: 1
        hit:
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: piercing
        reach_feet: 5
    skill_totals:
      perception: 3
    defenses: {}
    senses:
      passive_perception: 13
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
- id: dnd2024:creature_vulture
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
  source_sha256: 7258a5b926d1e4f5e3364b1ba92d46b263c400f6cabd65f1e5e509d65c0718c5
  source_path: builtin/srd-5.2.1/creature/vulture.md
  pages:
  - 363
  - 364
  engine_gaps: &id001
  - dnd2024:creature_vulture
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: bd7947e04393dc9b1ef6e71e6f439e5fbdd5d69c059065ec9ae1b25da0f2d9c5
splat_clauses:
- key: source_rules
  text: '# Vulture Medium Beast, Unaligned AC 10 Initiative +0 (10) HP 5 (1d8 + 1) Speed 10 ft., Fly 50
    ft. MOD SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 10 +0 +0 Con 13 +1 +1 Int 2 -4 -4 Wis 12 +1 +1 Cha
    4 -3 -3 Skills Perception +3 Senses Passive Perception 13 Languages None CR 0 (XP 10; PB +2) ### Traits
    Pack Tactics. The vulture has Advantage on an attack roll against a creature if at least one of the
    vulture’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.
    ### Actions Beak. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Piercing damage.'
  sha256: f05d292851cdc8589e7d8f045463b20ab84859cd2c755e461a47d561e087db4c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vulture

<!-- SRD page 363 -->

Medium Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 5 (1d8 + 1)\
Speed 10 ft., Fly 50 ft.\

<!-- SRD page 364 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 10 +0 +0 Con 13 +1 +1\
Int 2 −4 −4 Wis 12 +1 +1 Cha  4 −3 −3\
Skills Perception +3\
Senses Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Pack Tactics. The vulture has Advantage on an attack\
roll against a creature if at least one of the vulture’s al-\
lies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\

### Actions

Beak. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/363|page 363]], [[../../srd-5.2.1/pages/364|page 364]].
