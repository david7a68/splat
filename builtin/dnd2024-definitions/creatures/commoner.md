---
splat_schema: 1
splat_id: 64bcc909-76f0-5047-bcbb-66ce3c75d0f8
splat_kind: creature
splat_key: dnd2024:creatures/commoner
splat_name: Commoner
splat_origin:
  key: dnd2024:reference/creature/commoner
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Club
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 4 (1d8)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 10
  max_hp:
    value: 4
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/commoner
  implementation: dnd2024:creature_commoner
  parameters:
    features:
      club:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 2
        targets:
          count: 1
        hit:
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: bludgeoning
        reach_feet: 5
      training:
        economy: passive
        skill_choice:
          chooser: gm
          count: 1
          from: skills
        grant_skill_proficiency: true
        advantage:
          roll: ability_check
          skill: chosen_skill
    defenses: {}
    senses:
      passive_perception: 10
      special: []
    gear:
    - item_name: Club
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
- id: dnd2024:creature_commoner
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
  source_sha256: 01cc0f628cdf1564d4ecc1afd61a0cd2e25eec4bcfc28f82cbaf6c634e2e810a
  source_path: builtin/srd-5.2.1/creature/commoner.md
  pages:
  - 275
  engine_gaps: &id001
  - dnd2024:creature_commoner
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 8e874add3dac22117baaa70fea151bca504429d349dcb5b26d9a51b410d71125
splat_clauses:
- key: source_rules
  text: '# Commoner Medium or Small Humanoid, Neutral AC 10 Initiative +0 (10) HP 4 (1d8) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 10 +0 +0 Con 10 +0 +0 Int 10 +0 +0 Wis 10 +0 +0 Cha 10
    +0 +0 Gear Club Senses Passive Perception 10 Languages Common CR 0 (XP 10; PB +2) ### Traits Training.
    The commoner has proficiency in one skill of the GM’s choice and has Advantage whenever it makes an
    ability check using that skill. ### Actions Club. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4)
    Bludgeoning damage.'
  sha256: c6741f70de0b852d486372e808da8a1778068158d17e2349569ae02c05922131
  status: contract_specified
splat_requires_mechanics: *id001
---

# Commoner

<!-- SRD page 275 -->

Medium or Small Humanoid, Neutral\
AC 10 Initiative +0 (10)\
HP 4 (1d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 10 +0 +0 Con 10 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
Gear Club\
Senses Passive Perception 10\
Languages Common\
CR 0 (XP 10; PB +2)\

### Traits

Training. The commoner has proficiency in one skill of\
the GM’s choice and has Advantage whenever it makes\
an ability check using that skill.\

### Actions

Club. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/275|page 275]].
