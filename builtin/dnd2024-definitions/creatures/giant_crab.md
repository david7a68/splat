---
splat_schema: 1
splat_id: 3bbfcc51-aff7-5988-8d2d-bed44121df53
splat_kind: creature
splat_key: dnd2024:creatures/giant_crab
splat_name: Giant Crab
splat_origin:
  key: dnd2024:reference/creature/giant_crab
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Stealth +3
  senses_text: Blindsight 30 ft.; Passive Perception 9
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 13 (3d8)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 15
  max_hp:
    value: 13
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_crab
  implementation: dnd2024:creature_giant_crab
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 6
            add: 1
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
          then:
          - op: condition
            condition: grappled
            escape_dc: 11
            source: self
            appendages:
              kind: claw
              available: 2
              used: 1
        reach_feet: 5
    skill_totals:
      stealth: 3
    defenses: {}
    senses:
      passive_perception: 9
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
- id: dnd2024:creature_giant_crab
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
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 1777d84511f1c9a98ef310bcf3b42b87d0da065478fea7e1a2b14a95a4dd1a39
  source_path: builtin/srd-5.2.1/creature/giant_crab.md
  pages:
  - 350
  engine_gaps: &id001
  - dnd2024:creature_giant_crab
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: be8f219fc262ad10e173654f030df1c8aec4c3e27ecb4591b7065c113b8ff12e
splat_clauses:
- key: source_rules
  text: '# Giant Crab Medium Beast, Unaligned AC 15 Initiative +1 (11) HP 13 (3d8) Speed 30 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 13 +1 +1 Con11 +0 +0 Int 1 -5 -5 Wis 9 -1 -1 Cha
    3 -4 -4 Skills Stealth +3 Senses Blindsight 30 ft.; Passive Perception 9 Languages None CR 1/8 (XP
    25; PB +2) ### Traits Amphibious. The crab can breathe air and water. ### Actions Claw. Melee Attack
    Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Bludgeoning damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 11) from one of two claws.'
  sha256: f2ed0226ec02b01b868bd1fe5e0398e35129f0f12ae289bab59ba596c075ef8e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Crab

<!-- SRD page 350 -->

Medium Beast, Unaligned\
AC 15  Initiative +1 (11)\
HP 13 (3d8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 13 +1 +1 Con11 +0 +0\
Int 1 −5 −5 Wis 9 −1 −1 Cha 3 −4 −4\
Skills Stealth +3\
Senses Blindsight 30 ft.; Passive Perception 9\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Traits

Amphibious. The crab can breathe air and water.\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1)\
Bludgeoning damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 11) from one of two claws.\

Original layout: [[../../srd-5.2.1/pages/350|page 350]].
