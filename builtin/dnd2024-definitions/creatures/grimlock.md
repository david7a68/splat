---
splat_schema: 1
splat_id: 217f9689-bdcd-5867-9c0c-ed4a5b5c3313
splat_kind: creature
splat_key: dnd2024:creatures/grimlock
splat_name: Grimlock
splat_origin:
  key: dnd2024:reference/creature/grimlock
splat_sources:
- title: SRD 5.2.1
  locator: pp. 296
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: aberration
  alignment: Neutral Evil
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Athletics +5, Perception +3, Stealth +5
  senses_text: Blindsight 30 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 9
  save_intelligence:
    value: -1
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 11
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/grimlock
  implementation: dnd2024:creature_grimlock
  parameters:
    features:
      bone_cudgel:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: bludgeoning
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: psychic
        reach_feet: 5
    skill_totals:
      athletics: 5
      perception: 3
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 13
      special:
      - sense: blindsight
        range_feet: 30
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
- id: dnd2024:creature_grimlock
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
  source_sha256: f4d662872c3d17293b252ffc6f8f8c16ec7d07b2b7eee723eb58c2794a18b3dc
  source_path: builtin/srd-5.2.1/creature/grimlock.md
  pages:
  - 296
  engine_gaps: &id001
  - dnd2024:creature_grimlock
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e7326837a239928191ade517e789fb530e2ee902ffc44eafff916e37b3c81ada
splat_clauses:
- key: source_rules
  text: '# Grimlock Medium Aberration, Neutral Evil AC 11 Initiative +1 (11) HP 11 (2d8 + 2) Speed 30
    ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 12 +1 +1 Con 12 +1 +1 Int 9 -1 -1 Wis
    8 -1 -1 Cha 6 -2 -2 Skills Athletics +5, Perception +3, Stealth +5 Senses Blindsight 30 ft.; Passive
    Perception 13 Languages None CR 1/4 (XP 50; PB +2) ### Actions Bone Cudgel. Melee Attack Roll: +5,
    reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage plus 2 (1d4) Psychic damage.'
  sha256: 6f424c6d53d45b0dd26c9ccb70646c73936361a10dd9a034b488ef09a564699a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grimlock

<!-- SRD page 296 -->

Medium Aberration, Neutral Evil\
AC 11 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 12 +1 +1 Con 12 +1 +1\
Int 9 −1 −1 Wis 8 −1 −1 Cha 6 −2 −2\
Skills Athletics +5, Perception +3, Stealth +5\
Senses Blindsight 30 ft.; Passive Perception 13\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bone Cudgel. Melee Attack Roll: +5, reach 5 ft. Hit:\
6 (1d6 + 3) Bludgeoning damage plus 2 (1d4) Psy-\
chic damage.\

Original layout: [[../../srd-5.2.1/pages/296|page 296]].
