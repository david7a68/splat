---
splat_schema: 1
splat_id: 92183bf8-0664-5742-938f-992b30280a12
splat_kind: creature
splat_key: dnd2024:creatures/giant_boar
splat_name: Giant Boar
splat_origin:
  key: dnd2024:reference/creature/giant_boar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 8
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 42 (5d10 + 15)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 5
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 42
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_boar
  implementation: dnd2024:creature_giant_boar
  parameters:
    features:
      gore:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 10
          amount:
            dice:
              count: 2
              sides: 6
            add: 3
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
            average: 7
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            damage_type: piercing
          - op: condition
            condition: prone
        reach_feet: 5
      bloodied_fury:
        economy: passive
        advantage:
          roll: melee_attack
          when: self_bloodied
    defenses: {}
    senses:
      passive_perception: 8
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
- id: dnd2024:creature_giant_boar
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
  source_sha256: 71a8cc74d9b47372012c5d3d9a2fc15624b198f45c7a9f2d94cefd455a012df4
  source_path: builtin/srd-5.2.1/creature/giant_boar.md
  pages:
  - 349
  engine_gaps: &id001
  - dnd2024:creature_giant_boar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 86d8d1839cef310b21e0906b3a50249259b456dc4b418b90d3304ea061c6869c
splat_clauses:
- key: source_rules
  text: '# Giant Boar Large Beast, Unaligned AC 13 Initiative +0 (10) HP 42 (5d10 + 15) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 17 +3 +5 Dex 10 +0 +0 Con 16 +3 +3 Int 2 -4 -4 Wis 7 -2 -2 Cha 5 -3 -3
    Senses Passive Perception 8 Languages None CR 2 (XP 450; PB +2) ### Traits Bloodied Fury. The boar
    has Advantage on melee attack rolls while it is Bloodied. ### Actions Gore. Melee Attack Roll: +5,
    reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the target is a Large or smaller creature and the
    boar moved 20+ feet straight toward it immediately before the hit, the target takes an extra 7 (2d6)
    Piercing damage and has the Prone condition.'
  sha256: 668f387639350fe46e5b070f4ec7099fc91d036f36b11c23835ce47b12822abd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Boar

<!-- SRD page 349 -->

Large Beast, Unaligned\
AC 13  Initiative +0 (10)\
HP 42 (5d10 + 15)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +5 Dex 10 +0 +0 Con 16 +3 +3\
Int 2 −4 −4 Wis 7 −2 −2 Cha 5 −3 −3\
Senses Passive Perception 8\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Bloodied Fury. The boar has Advantage on melee at-\
tack rolls while it is Bloodied.\

### Actions

Gore. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Piercing damage. If the target is a Large or smaller\
creature and the boar moved 20+ feet straight toward it\
immediately before the hit, the target takes an extra 7\
(2d6) Piercing damage and has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]].
