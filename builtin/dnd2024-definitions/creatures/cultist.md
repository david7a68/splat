---
splat_schema: 1
splat_id: 90ed0908-d71a-533a-b9e2-616435f0b171
splat_kind: creature
splat_key: dnd2024:creatures/cultist
splat_name: Cultist
splat_origin:
  key: dnd2024:reference/creature/cultist
splat_sources:
- title: SRD 5.2.1
  locator: pp. 278
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Deception +2, Religion +2
  gear_text: Leather Armor, Sickle
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 9 (2d8)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 9
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/cultist
  implementation: dnd2024:creature_cultist
  parameters:
    features:
      ritual_sickle:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 4
            add: 1
          damage_type: slashing
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: necrotic
        reach_feet: 5
    skill_totals:
      deception: 2
      religion: 2
    defenses: {}
    senses:
      passive_perception: 10
      special: []
    gear:
    - item_name: Leather Armor
      quantity: 1
    - item_name: Sickle
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
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
- id: dnd2024:creature_cultist
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
  source_sha256: d50eda5c83b8d526bad6e218811a1030585b0006ab77cf21d6e2713caeca7443
  source_path: builtin/srd-5.2.1/creature/cultist.md
  pages:
  - 278
  engine_gaps: &id001
  - dnd2024:creature_cultist
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 3299c0ce171a7a13d85da53bd7360ddf70397f5077ff8fe72c9bd2fba90fd900
splat_clauses:
- key: source_rules
  text: '# Cultist Medium or Small Humanoid, Neutral AC 12 Initiative +1 (11) HP 9 (2d8) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 12 +1 +1 Con 10 +0 +0 Int 10 +0 +0 Wis 11 +0 +2 Cha 10
    +0 +0 Skills Deception +2, Religion +2 Gear Leather Armor, Sickle Senses Passive Perception 10 Languages
    Common CR 1/8 (XP 25; PB +2) ### Actions Ritual Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 3
    (1d4 + 1) Slashing damage plus 1 Necrotic damage.'
  sha256: 4f57b3032a52bee929f938ad0d7ce69bab79fad655ecf2b9520d4a4d6cc1ee90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cultist

<!-- SRD page 278 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 9 (2d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 10 +0 +0\
Int 10 +0 +0 Wis 11 +0 +2 Cha 10 +0 +0\
Skills Deception +2, Religion +2\
Gear Leather Armor, Sickle\
Senses Passive Perception 10\
Languages Common\
CR 1/8 (XP 25; PB +2)\

### Actions

Ritual Sickle. Melee Attack Roll: +3, reach 5 ft. Hit: 3\
(1d4 + 1) Slashing damage plus 1 Necrotic damage.\

Original layout: [[../../srd-5.2.1/pages/278|page 278]].
