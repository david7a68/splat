---
splat_schema: 1
splat_id: 5aa5397e-f6d8-5109-9a7f-8b1b2ce96fde
splat_kind: creature
splat_key: dnd2024:creatures/spider
splat_name: Spider
splat_origin:
  key: dnd2024:reference/creature/spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Climb 20 ft.
  skills_text: Stealth +4
  senses_text: Darkvision 30 ft.; Passive Perception 10
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 12
  max_hp:
    value: 1
  initiative:
    value: 2
  speed:
    value: 20
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/spider
  implementation: dnd2024:creature_spider
  parameters:
    features:
      spider_climb:
        economy: passive
        climb_ability_check_required: false
        surfaces:
        - difficult
        - ceiling
      web_walker:
        economy: passive
        ignore_movement_restrictions:
        - web
        knows_location:
          targets: creatures_touching_same_web
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
          average: 1
          amount:
            constant: 1
          damage_type: piercing
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: poison
        reach_feet: 5
    skill_totals:
      stealth: 4
    defenses: {}
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
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
- id: dnd2024:creature_spider
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
  source_sha256: 09afdf3bd3882ad38925c43a3516446c4b363a05ae9ffb2dc90ce57f903058dd
  source_path: builtin/srd-5.2.1/creature/spider.md
  pages:
  - 361
  engine_gaps: &id001
  - dnd2024:creature_spider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 0c846385a7f95c8a951b461a0c7e120bf9cbcb13cb98a00bfe9f7b18a407ff15
splat_clauses:
- key: source_rules
  text: '# Spider Tiny Beast, Unaligned AC 12 Initiative +2 (12) HP 1 (1d4 - 1) Speed 20 ft., Climb 20
    ft. MOD SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 14 +2 +2 Con 8 -1 -1 Int 1 -5 -5 Wis 10 +0 +0 Cha 2
    -4 -4 Skills Stealth +4 Senses Darkvision 30 ft.; Passive Perception 10 Languages None CR 0 (XP 10;
    PB +2) ### Traits Spider Climb. The spider can climb difficult surfaces, including along ceilings,
    without needing to make an ability check. Web Walker. The spider ignores movement restrictions caused
    by webs, and the spider knows the location of any other creature in contact with the same web. ###
    Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage plus 2 (1d4) Poison damage.'
  sha256: e49e5df584b2546279825fcaadaa5dcc589daf142848c8288601fc1f38e9c6d1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spider

<!-- SRD page 361 -->

Tiny Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 1 (1d4 − 1)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 14 +2 +2 Con 8 −1 −1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 2 −4 −4\
Skills Stealth +4\
Senses Darkvision 30 ft.; Passive Perception 10\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The spider ignores movement restrictions\
caused by webs, and the spider knows the location of\
any other creature in contact with the same web.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing\
damage plus 2 (1d4) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].
