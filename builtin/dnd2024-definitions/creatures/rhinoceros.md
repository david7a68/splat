---
splat_schema: 1
splat_id: 385242bd-9fd7-5a19-8d90-6affd94ceecc
splat_kind: creature
splat_key: dnd2024:creatures/rhinoceros
splat_name: Rhinoceros
splat_origin:
  key: dnd2024:reference/creature/rhinoceros
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 45
  initiative:
    value: -1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/rhinoceros
  implementation: dnd2024:creature_rhinoceros
  parameters:
    features:
      gore:
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
          damage_type: piercing
        - op: if
          test:
            target_kind: creature
            target_size_max: large
            immediately_preceding_movement:
              minimum_feet: 20
              direction: straight_toward_target
          then:
          - op: damage
            average: 9
            amount:
              dice:
                count: 2
                sides: 8
              add: 0
            damage_type: piercing
          - op: condition
            condition: prone
        reach_feet: 5
    defenses: {}
    senses:
      passive_perception: 11
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
- id: dnd2024:creature_rhinoceros
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
  source_sha256: 0a1533eb2f10558551447c1f5f8fe32ccfd5f0735435ee486c1fa2daa373045d
  source_path: builtin/srd-5.2.1/creature/rhinoceros.md
  pages:
  - 360
  engine_gaps: &id001
  - dnd2024:creature_rhinoceros
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 3bfff7e149f385388718ed2b8fee66368d995a8b66c7eef4e1d8abccd5a54aa9
splat_clauses:
- key: source_rules
  text: '# Rhinoceros Large Beast, Unaligned AC 13 Initiative -1 (9) HP 45 (6d10 + 12) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 8 -1 -1 Con 15 +2 +2 Int 2 -4 -4 Wis 12 +1 +1 Cha 6 -2 -2
    Senses Passive Perception 11 Languages None CR 2 (XP 450; PB +2) ### Actions Gore. Melee Attack Roll:
    +7, reach 5 ft. Hit: 14 (2d8 + 5) Piercing damage. If target is a Large or smaller creature and the
    rhinoceros moved 20+ feet straight toward it immediately before the hit, the target takes an extra
    9 (2d8) Piercing damage and has the Prone condition.'
  sha256: 43592313bde3201d41a28ceb99f12a49c52a8ea52223b638cdd51c17a0a41507
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rhinoceros

<!-- SRD page 360 -->

Large Beast, Unaligned\
AC 13  Initiative −1 (9)\
HP 45 (6d10 + 12)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 8 −1 −1 Con 15 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Passive Perception 11\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Gore. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 +\
5) Piercing damage. If target is a Large or smaller crea-\
ture and the rhinoceros moved 20+ feet straight toward\
it immediately before the hit, the target takes an extra 9\
(2d8) Piercing damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/360|page 360]].
