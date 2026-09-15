---
splat_schema: 1
splat_id: 4bf3d8ad-b9ff-5916-b2ee-391305fdf40e
splat_kind: creature
splat_key: dnd2024:creatures/gnoll_warrior
splat_name: Gnoll Warrior
splat_origin:
  key: dnd2024:reference/creature/gnoll_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 289
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Chaotic Evil
  movement_text: 30 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Gnoll
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 27 (6d8)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 27
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/gnoll_warrior
  implementation: dnd2024:creature_gnoll_warrior
  parameters:
    features:
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 6
            add: 2
          damage_type: piercing
        reach_feet: 5
      bone_bow:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 10
            add: 1
          damage_type: piercing
        range_feet: 150
        long_range_feet: 600
      rampage:
        economy: bonus_action
        resource:
          capacity: 1
          recovery: long_rest
          cost: 1
        trigger: self.damage_dealt
        requires:
          target_kind: creature
          target_bloodied_before_damage: true
        op: sequence
        steps:
        - op: move
          distance_max:
            multiply:
            - stat: speed
            - numerator: 1
              denominator: 2
        - op: use_feature
          feature: rend
          count: 1
    defenses: {}
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - gnoll
      understands:
      - gnoll
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
- id: dnd2024:creature_gnoll_warrior
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
  events:
  - self.damage_dealt
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
  source_sha256: acc5355296e3af7ea6a87111879462c6c043de58a82fafa1f62156b801992d7e
  source_path: builtin/srd-5.2.1/creature/gnoll_warrior.md
  pages:
  - 289
  engine_gaps: &id001
  - dnd2024:creature_gnoll_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 28eea225b8d428dd91c2a7bc04dd5157317efc36e11a9a3fd882e65c891c2d90
splat_clauses:
- key: source_rules
  text: '# Gnoll Warrior Medium Fiend, Chaotic Evil AC 15 Initiative +1 (11) HP 27 (6d8) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 12 +1 +1 Con 11 +0 +0 Int 6 -2 -2 Wis 10 +0 +0 Cha 7 -2
    -2 Senses Darkvision 60 ft.; Passive Perception 10 Languages Gnoll CR 1/2 (XP 100; PB +2) ### Actions
    Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage. Bone Bow. Ranged Attack
    Roll: +3, range 150/600 ft. Hit: 6 (1d10 + 1) Piercing damage. ### Bonus Actions Rampage (1/Day).
    Immediately after dealing damage to a creature that is already Bloodied, the gnoll moves up to half
    its Speed, and it makes one Rend attack.'
  sha256: 26fb4a869b55efa9c64dad392824671d70aca8fa4f560435a8440b7b58369145
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gnoll Warrior

<!-- SRD page 289 -->

Medium Fiend, Chaotic Evil\
AC 15 Initiative +1 (11)\
HP 27 (6d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 12 +1 +1 Con 11 +0 +0\
Int 6 −2 −2 Wis 10 +0 +0 Cha 7 −2 −2\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Gnoll\
CR 1/2 (XP 100; PB +2)\

### Actions

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage.\
Bone Bow. Ranged Attack Roll: +3, range 150/600 ft.\
Hit: 6 (1d10 + 1) Piercing damage.\

### Bonus Actions

Rampage (1/Day). Immediately after dealing damage to\
a creature that is already Bloodied, the gnoll moves up\
to half its Speed, and it makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/289|page 289]].
