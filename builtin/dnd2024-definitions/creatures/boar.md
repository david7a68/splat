---
splat_schema: 1
splat_id: f8b9d24b-5839-5f42-8249-f828ca119c19
splat_kind: creature
splat_key: dnd2024:creatures/boar
splat_name: Boar
splat_origin:
  key: dnd2024:reference/creature/boar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 9
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 13 (2d8 + 4)
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
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 13
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/boar
  implementation: dnd2024:creature_boar
  parameters:
    features:
      gore:
        economy: action
        op: attack
        attack_modes:
        - melee
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
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
            immediately_preceding_movement:
              minimum_feet: 20
              direction: straight_toward_target
          then:
          - op: damage
            average: 3
            amount:
              dice:
                count: 1
                sides: 6
              add: 0
            damage_type: piercing
          - op: condition
            condition: prone
        reach_feet: 5
      bloodied_fury:
        economy: passive
        advantage:
          roll: attack
          when: self_bloodied
    defenses: {}
    senses:
      passive_perception: 9
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
- id: dnd2024:creature_boar
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
  source_sha256: 065ab616d8b018779560ae074d7760c351a9dff6d2f5af96e9300e9d2b46e01e
  source_path: builtin/srd-5.2.1/creature/boar.md
  pages:
  - 346
  engine_gaps: &id001
  - dnd2024:creature_boar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: eecd0a61f6253fc5e0a5b6dd5bb5fc2be689373c0ef8df35b6a65cc7a9b3218a
splat_clauses:
- key: source_rules
  text: '# Boar Medium Beast, Unaligned AC 11 Initiative +0 (10) HP 13 (2d8 + 4) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 13 +1 +1 Dex 11 +0 +0 Con 14 +2 +2 Int 2 -4 -4 Wis 9 -1 -1 Cha 5 -3 -3 Senses
    Passive Perception 9 Languages None CR 1/4 (XP 50; PB +2) ### Traits Bloodied Fury. While Bloodied,
    the boar has Advantage on attack rolls. ### Actions Gore. Melee Attack Roll: +3, reach 5 ft. Hit:
    4 (1d6 + 1) Piercing damage. If the target is a Medium or smaller creature and the boar moved 20+
    feet straight toward it immediately before the hit, the target takes an extra 3 (1d6) Piercing damage
    and has the Prone condition.'
  sha256: 913851858812ede8f293e93615279d5589aedefb232e6fbecbb235799c76d36e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Boar

<!-- SRD page 346 -->

Medium Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 13 (2d8 + 4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 11 +0 +0 Con 14 +2 +2\
Int 2 −4 −4 Wis 9 −1 −1 Cha 5 −3 −3\
Senses Passive Perception 9\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Bloodied Fury. While Bloodied, the boar has Advan-\
tage on attack rolls.\

### Actions

Gore. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1)\
Piercing damage. If the target is a Medium or smaller\
creature and the boar moved 20+ feet straight toward it\
immediately before the hit, the target takes an extra 3\
(1d6) Piercing damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/346|page 346]].
