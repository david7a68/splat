---
splat_schema: 1
splat_id: a13cbe49-3f0c-5ea5-aef8-a35899999be3
splat_kind: creature
splat_key: dnd2024:creatures/bugbear_warrior
splat_name: Bugbear Warrior
splat_origin:
  key: dnd2024:reference/creature/bugbear_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Chaotic Evil
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  skills_text: Stealth +6, Survival +2
  gear_text: Hide Armor, Light Hammers (3)
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common, Goblin
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 33 (6d8 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 14
  max_hp:
    value: 33
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bugbear_warrior
  implementation: dnd2024:creature_bugbear_warrior
  parameters:
    features:
      grab:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 6
            add: 2
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: medium
          then:
          - op: condition
            condition: grappled
            escape_dc: 12
            source: self
        reach_feet: 10
      light_hammer:
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
          average: 9
          amount:
            dice:
              count: 3
              sides: 4
            add: 2
          damage_type: bludgeoning
        reach_feet: 10
        range_feet: 20
        long_range_feet: 60
        advantage_when:
          target_grappled_by: self
      abduct:
        economy: passive
        move_grappled_creature_extra_cost: 0
    skill_totals:
      stealth: 6
      survival: 2
    defenses: {}
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Hide Armor
      quantity: 1
    - item_name: Light Hammers
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
- id: dnd2024:creature_bugbear_warrior
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
  source_sha256: bc7b8cc5fc760206d8f7eba1d903d9b00faaff910715e041eadc9ef21fe790a9
  source_path: builtin/srd-5.2.1/creature/bugbear_warrior.md
  pages:
  - 272
  engine_gaps: &id001
  - dnd2024:creature_bugbear_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d745febb64f90b02929e6beded660e3beb767718de5b4bbf2d4c917c58c81efc
splat_clauses:
- key: source_rules
  text: '# Bugbear Warrior Medium Fey (Goblinoid), Chaotic Evil AC 14 Initiative +2 (12) HP 33 (6d8 +
    6) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2 +2 Con 13 +1 +1 Int 8 -1 -1 Wis
    11 +0 +0 Cha 9 -1 -1 Skills Stealth +6, Survival +2 Gear Hide Armor, Light Hammers (3) Senses Darkvision
    60 ft.; Passive Perception 10 Languages Common, Goblin CR 1 (XP 200; PB +2) ### Traits Abduct. The
    bugbear needn’t spend extra movement to move a creature it is grappling. ### Actions Grab. Melee Attack
    Roll: +4, reach 10 ft. Hit: 9 (2d6 + 2) Bludgeoning damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 12). Light Hammer. Melee or Ranged Attack Roll: +4 (with
    Advantage if the target is Grappled by the bugbear), reach 10 ft. or range 20/60 ft. Hit: 9 (3d4 +
    2) Bludgeoning damage.'
  sha256: ac61ea56a135d0bf083365c89fddb4c16ad20c4a702e63b4c4e489f47517922b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bugbear Warrior

<!-- SRD page 272 -->

Medium Fey (Goblinoid), Chaotic Evil\
AC 14 Initiative +2 (12)\
HP 33 (6d8 + 6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 13 +1 +1\
Int 8 −1 −1 Wis 11 +0 +0 Cha 9 −1 −1\
Skills Stealth +6, Survival +2\
Gear Hide Armor, Light Hammers (3)\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 1 (XP 200; PB +2)\

### Traits

Abduct. The bugbear needn’t spend extra movement to\
move a creature it is grappling.\

### Actions

Grab. Melee Attack Roll: +4, reach 10 ft. Hit: 9 (2d6\
+ 2) Bludgeoning damage. If the target is a Medium\
or smaller creature, it has the Grappled condition (es-\
cape DC 12).\
Light Hammer. Melee or Ranged Attack Roll: +4 (with\
Advantage if the target is Grappled by the bugbear),\
reach 10 ft. or range 20/60 ft. Hit: 9 (3d4 + 2) Blud-\
geoning damage.\

Original layout: [[../../srd-5.2.1/pages/272|page 272]].
