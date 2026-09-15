---
splat_schema: 1
splat_id: 5df7adb8-5218-5c6f-b1ff-7232ef3527bc
splat_kind: creature
splat_key: dnd2024:creatures/tiger
splat_name: Tiger
splat_origin:
  key: dnd2024:reference/creature/tiger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 363
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +3, Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 30 (4d10 + 8)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 30
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/tiger
  implementation: dnd2024:creature_tiger
  parameters:
    features:
      rend:
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
          damage_type: slashing
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        reach_feet: 5
      nimble_escape:
        economy: bonus_action
        op: take_action
        choice:
        - disengage
        - hide
    skill_totals:
      perception: 3
      stealth: 7
    defenses: {}
    senses:
      passive_perception: 13
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
- id: dnd2024:creature_tiger
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
  source_sha256: cc7300c09b9ac3f50666c026ff5b9e1cdf5047c7b6bf577e318ce7827ca862e7
  source_path: builtin/srd-5.2.1/creature/tiger.md
  pages:
  - 363
  engine_gaps: &id001
  - dnd2024:creature_tiger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 60bc94a75c0d900b37738cfcc427ae593efa1f52dc9b32f3e42775e53357ce3f
splat_clauses:
- key: source_rules
  text: '# Tiger Large Beast, Unaligned AC 13 Initiative +3 (13) HP 30 (4d10 + 8) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 17 +3 +3 Dex 16 +3 +3 Con 14 +2 +2 Int 3 -4 -4 Wis 12 +1 +1 Cha 8 -1 -1 Skills
    Perception +3, Stealth +7 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR 1 (XP
    200; PB +2) ### Actions Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage.
    If the target is a Large or smaller creature, it has the Prone condition. ### Bonus Actions Nimble
    Escape. The tiger takes the Disengage or Hide action.'
  sha256: 92f10b2e4174bf7540c70494d73bb757242c640d18143f56193190d5cb881243
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tiger

<!-- SRD page 363 -->

Large Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 30 (4d10 + 8)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 16 +3 +3 Con 14 +2 +2\
Int 3 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +3, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage. If the target is a Large or smaller\
creature, it has the Prone condition.\

### Bonus Actions

Nimble Escape. The tiger takes the Disengage or\
Hide action.\

Original layout: [[../../srd-5.2.1/pages/363|page 363]].
