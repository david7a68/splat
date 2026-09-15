---
splat_schema: 1
splat_id: e76f4d97-c2ca-599e-a26e-7ded8834ee22
splat_kind: creature
splat_key: dnd2024:creatures/black_bear
splat_name: Black Bear
splat_origin:
  key: dnd2024:reference/creature/black_bear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft., Swim 30 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d8 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 14
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
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/black_bear
  implementation: dnd2024:creature_black_bear
  parameters:
    features:
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 6
            add: 2
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
- id: dnd2024:creature_black_bear
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
  source_sha256: 1d66d2ea0cb1546b99eb8212a9088422199dd0d0449c4860f183beb3669a6d69
  source_path: builtin/srd-5.2.1/creature/black_bear.md
  pages:
  - 345
  engine_gaps: &id001
  - dnd2024:creature_black_bear
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/black_bear
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d3824aaafd2b85993eddb4d13414e93db1590930efbd12e3376dbd36e0f4ce5f
splat_clauses:
- key: source_rules
  text: '# Black Bear Medium Beast, Unaligned AC 11 Initiative +1 (11) HP 19 (3d8 + 6) Speed 30 ft., Climb
    30 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2 Int 2 -4 -4
    Wis 12 +1 +1 Cha 7 -2 -2 Skills Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages
    None CR 1/2 (XP 100; PB +2) ### Actions Multiattack. The bear makes two Rend attacks. Rend. Melee
    Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage.'
  sha256: a0720d2cb50d42c2f0acf5d806965526b913527deb9de472d7c4c77a124cdac3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Black Bear

<!-- SRD page 345 -->

Medium Beast, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d8 + 6)\
Speed 30 ft., Climb 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +1 Con 14 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Multiattack. The bear makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].
