---
splat_schema: 1
splat_id: 30866ef2-a238-5b10-be76-9bf148aaf82f
splat_kind: creature
splat_key: dnd2024:creatures/ogre
splat_name: Ogre
splat_origin:
  key: dnd2024:reference/creature/ogre
splat_sources:
- title: SRD 5.2.1
  locator: pp. 312
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: giant
  alignment: Chaotic Evil
  movement_text: 40 ft.
  gear_text: Greatclub, Javelins (3)
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: Common, Giant
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 68 (8d10 + 24)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 68
  initiative:
    value: -1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ogre
  implementation: dnd2024:creature_ogre
  parameters:
    features:
      greatclub:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 13
          amount:
            dice:
              count: 2
              sides: 8
            add: 4
          damage_type: bludgeoning
        reach_feet: 5
      javelin:
        economy: action
        op: attack
        attack_modes:
        - melee
        - ranged
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
          damage_type: piercing
        reach_feet: 5
        range_feet: 30
        long_range_feet: 120
    defenses: {}
    senses:
      passive_perception: 8
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Greatclub
      quantity: 1
    - item_name: Javelins
      quantity: 3
    communication:
      speaks:
      - common
      - giant
      understands:
      - common
      - giant
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
- id: dnd2024:creature_ogre
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
  source_sha256: 85bb8b96173b4e3e6307f41baf371b00120260b30ac099b24ce1bcf7b7a8467b
  source_path: builtin/srd-5.2.1/creature/ogre.md
  pages:
  - 312
  engine_gaps: &id001
  - dnd2024:creature_ogre
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/ogre
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 0920678c2e2b0f9f690c163fc2fc4b97be1d8e8d5b8aae4ee6ee422a857d0703
splat_clauses:
- key: source_rules
  text: '# Ogre Large Giant, Chaotic Evil AC 11 Initiative -1 (9) HP 68 (8d10 + 24) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 19 +4 +4 Dex 8 -1 -1 Con 16 +3 +3 Int 5 -3 -3 Wis 7 -2 -2 Cha 7 -2 -2 Gear Greatclub,
    Javelins (3) Senses Darkvision 60 ft.; Passive Perception 8 Languages Common, Giant CR 2 (XP 450;
    PB +2) ### Actions Greatclub. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage.
    Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 30/120 ft. Hit: 11 (2d6 + 4) Piercing
    damage.'
  sha256: eb374e6f60b36f07082e8b3ebc24294e5c4cb4c350114b2c406d88a2f12b0a2d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ogre

<!-- SRD page 312 -->

Large Giant, Chaotic Evil\
AC 11 Initiative −1 (9)\
HP 68 (8d10 + 24)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
Int 5 −3 −3 Wis 7 −2 −2 Cha 7 −2 −2\
Gear Greatclub, Javelins (3)\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Common, Giant\
CR 2 (XP 450; PB +2)\

### Actions

Greatclub. Melee Attack Roll: +6, reach 5 ft. Hit: 13\
(2d8 + 4) Bludgeoning damage.\
Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or\
range 30/120 ft. Hit: 11 (2d6 + 4) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/312|page 312]].
