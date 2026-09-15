---
splat_schema: 1
splat_id: 7220231a-57f4-5b70-8d4f-2e4bd68482ab
splat_kind: creature
splat_key: dnd2024:creatures/pteranodon
splat_name: Pteranodon
splat_origin:
  key: dnd2024:reference/creature/pteranodon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 10 ft., Fly 60 ft.
  skills_text: Perception +1
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 13 (3d8)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
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
    value: 13
  max_hp:
    value: 13
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pteranodon
  implementation: dnd2024:creature_pteranodon
  parameters:
    features:
      flyby:
        economy: passive
        opportunity_attack_immunity:
          trigger: fly_out_of_enemy_reach
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 8
            add: 2
          damage_type: piercing
        reach_feet: 5
    skill_totals:
      perception: 1
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
- id: dnd2024:creature_pteranodon
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
  events:
  - fly_out_of_enemy_reach
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
  source_sha256: 9a10b3b5860b605970ad5b3d0a6547fd15726c3717735cb6d53349937e4c8623
  source_path: builtin/srd-5.2.1/creature/pteranodon.md
  pages:
  - 359
  engine_gaps: &id001
  - dnd2024:creature_pteranodon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: bcd95f7dcfdebdc5d2f4e06671d3307c335fe2a4a6d1af4e0cc24ab517c2d4b2
splat_clauses:
- key: source_rules
  text: '# Pteranodon Medium Beast (Dinosaur), Unaligned AC 13 Initiative +2 (12) HP 13 (3d8) Speed 10
    ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 15 +2 +2 Con 10 +0 +0 Int 2 -4 -4 Wis
    9 -1 -1 Cha 5 -3 -3 Skills Perception +1 Senses Passive Perception 11 Languages None CR 1/4 (XP 50;
    PB +2) ### Traits Flyby. The pteranodon doesn’t provoke an Opportunity Attack when it flies out of
    an enemy’s reach. ### Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Piercing damage.'
  sha256: dc0af30f3f2dc7691d8342870826fe024357e575e8145646e1243ca0935c5679
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pteranodon

<!-- SRD page 359 -->

Medium Beast (Dinosaur), Unaligned\
AC 13  Initiative +2 (12)\
HP 13 (3d8)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 15 +2 +2 Con 10 +0 +0\
Int 2 −4 −4 Wis 9 −1 −1 Cha  5 −3 −3\
Skills Perception +1\
Senses Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Flyby. The pteranodon doesn’t provoke an Opportunity\
Attack when it flies out of an enemy’s reach.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/359|page 359]].
