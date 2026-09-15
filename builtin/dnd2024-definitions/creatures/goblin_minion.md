---
splat_schema: 1
splat_id: 299cbf5b-b33b-59e4-80a9-b38f344dc807
splat_kind: creature
splat_key: dnd2024:creatures/goblin_minion
splat_name: Goblin Minion
splat_origin:
  key: dnd2024:reference/creature/goblin_minion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 290
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: fey
  alignment: Chaotic Neutral
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  skills_text: Stealth +6
  gear_text: Daggers (3)
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Common, Goblin
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 7 (2d6)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 7
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/goblin_minion
  implementation: dnd2024:creature_goblin_minion
  parameters:
    features:
      dagger:
        economy: action
        op: attack
        attack_modes:
        - melee
        - ranged
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 4
            add: 2
          damage_type: piercing
        reach_feet: 5
        range_feet: 20
        long_range_feet: 60
      nimble_escape:
        economy: bonus_action
        op: take_action
        choice:
        - disengage
        - hide
    skill_totals:
      stealth: 6
    defenses: {}
    senses:
      passive_perception: 9
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Daggers
      quantity: 3
    communication:
      speaks:
      - common
      - goblin
      understands:
      - common
      - goblin
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
- id: dnd2024:creature_goblin_minion
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
  source_sha256: 09050f1fcf90768cfb18278cb6fab5274e3d6f68789066cca8d351e73c6ee4dc
  source_path: builtin/srd-5.2.1/creature/goblin_minion.md
  pages:
  - 290
  engine_gaps: &id001
  - dnd2024:creature_goblin_minion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 523c3670c663841f13cf3a30c8c2ac8c0afabd69975941a9675a69dbfb0914bc
splat_clauses:
- key: source_rules
  text: '# Goblin Minion Small Fey (Goblinoid), Chaotic Neutral AC 12 Initiative +2 (12) HP 7 (2d6) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 15 +2 +2 Con 10 +0 +0 Int 10 +0 +0 Wis 8 -1 -1 Cha
    8 -1 -1 Skills Stealth +6 Gear Daggers (3) Senses Darkvision 60 ft.; Passive Perception 9 Languages
    Common, Goblin CR 1/8 (XP 25; PB +2) ### Actions Dagger. Melee or Ranged Attack Roll: +4, reach 5
    ft. or range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage. ### Bonus Actions Nimble Escape. The goblin
    takes the Disengage or Hide action.'
  sha256: 8da9fbed626347b3ce132c73cc9f8bbedfac4cd36f21bd9c844a04356fd65eb2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goblin Minion

<!-- SRD page 290 -->

Small Fey (Goblinoid), Chaotic Neutral\
AC 12 Initiative +2 (12)\
HP 7 (2d6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 15 +2 +2 Con 10 +0 +0\
Int 10 +0 +0 Wis 8 −1 −1 Cha 8 −1 −1\
Skills Stealth +6\
Gear Daggers (3)\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Common, Goblin\
CR 1/8 (XP 25; PB +2)\

### Actions

Dagger. Melee or Ranged Attack Roll: +4, reach 5 ft. or\
range 20/60 ft. Hit: 4 (1d4 + 2) Piercing damage.\

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or\
Hide action.\

Original layout: [[../../srd-5.2.1/pages/290|page 290]].
