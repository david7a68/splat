---
splat_schema: 1
splat_id: 4c98848f-6ea9-5194-b756-113d2f989412
splat_kind: creature
splat_key: dnd2024:creatures/hobgoblin_captain
splat_name: Hobgoblin Captain
splat_origin:
  key: dnd2024:reference/creature/hobgoblin_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 299
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Lawful Evil
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  gear_text: Greatsword, Half Plate Armor, Longbow
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common, Goblin
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 58 (9d8 + 18)
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
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 58
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hobgoblin_captain
  implementation: dnd2024:creature_hobgoblin_captain
  parameters:
    features:
      greatsword:
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
          damage_type: slashing
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 5
      longbow:
        economy: action
        op: attack
        attack_modes:
        - ranged
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
        - op: damage
          average: 5
          amount:
            dice:
              count: 2
              sides: 4
            add: 0
          damage_type: poison
        range_feet: 150
        long_range_feet: 600
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: greatsword
        - feature: longbow
      aura_of_authority:
        economy: passive
        requires:
          self_incapacitated: false
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 10
            origin: self
          relation:
          - self
          - ally
        advantage:
          rolls:
          - attack
          - saving_throw
    defenses: {}
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Greatsword
      quantity: 1
    - item_name: Half Plate Armor
      quantity: 1
    - item_name: Longbow
      quantity: 1
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
- id: dnd2024:creature_hobgoblin_captain
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
  source_sha256: 0b26335479979eef84a1eaeeb0797614f42c89054d76e662e5ddbb611eefc14a
  source_path: builtin/srd-5.2.1/creature/hobgoblin_captain.md
  pages:
  - 299
  engine_gaps: &id001
  - dnd2024:creature_hobgoblin_captain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e0ac54896e04252222054ee551fe67aac6e68ddb2b61e053311bfc0c05944213
splat_clauses:
- key: source_rules
  text: '# Hobgoblin Captain Medium Fey (Goblinoid), Lawful Evil AC 17 Initiative +4 (14) HP 58 (9d8 +
    18) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2 Int 12 +1 +1 Wis
    10 +0 +0 Cha 13 +1 +1 Gear Greatsword, Half Plate Armor, Longbow Senses Darkvision 60 ft.; Passive
    Perception 10 Languages Common, Goblin CR 3 (XP 700; PB +2) ### Traits Aura of Authority. While in
    a 10-foot Emanation originating from the hobgoblin, the hobgoblin and its allies have Advantage on
    attack rolls and saving throws, provided the hobgoblin doesn’t have the Incapacitated condition. ###
    Actions Multiattack. The hobgoblin makes two attacks, using Greatsword or Longbow in any combination.
    Greatsword. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Slashing damage plus 3 (1d6) Poison
    damage. Longbow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage plus 5
    (2d4) Poison damage.'
  sha256: c6b186af5b1e043cf54dec2b435e1855b341ee1882be90fc4dc46cb16740beac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hobgoblin Captain

<!-- SRD page 299 -->

Medium Fey (Goblinoid), Lawful Evil\
AC 17 Initiative +4 (14)\
HP 58 (9d8 + 18)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2\
Int 12 +1 +1 Wis 10 +0 +0 Cha 13 +1 +1\
Gear Greatsword, Half Plate Armor, Longbow\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common, Goblin\
CR 3 (XP 700; PB +2)\

### Traits

Aura of Authority. While in a 10-foot Emanation origi-\
nating from the hobgoblin, the hobgoblin and its allies\
have Advantage on attack rolls and saving throws,\
provided the hobgoblin doesn’t have the Incapacitated\
condition.\

### Actions

Multiattack. The hobgoblin makes two attacks, using\
Greatsword or Longbow in any combination.\
Greatsword. Melee Attack Roll: +4, reach 5 ft. Hit: 9\
(2d6 + 2) Slashing damage plus 3 (1d6) Poison damage.\
Longbow. Ranged Attack Roll: +4, range 150/600\
ft. Hit: 6 (1d8 + 2) Piercing damage plus 5 (2d4) Poi-\
son damage.\

Original layout: [[../../srd-5.2.1/pages/299|page 299]].
