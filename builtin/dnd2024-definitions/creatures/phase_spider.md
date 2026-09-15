---
splat_schema: 1
splat_id: a44921eb-156c-59ad-88f3-8372efa9b185
splat_kind: creature
splat_key: dnd2024:creatures/phase_spider
splat_name: Phase Spider
splat_origin:
  key: dnd2024:reference/creature/phase_spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313, 314
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 45 (7d10 + 7)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 45
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/phase_spider
  implementation: dnd2024:creature_phase_spider
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
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 10
            add: 3
          damage_type: piercing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: poison
        - op: if
          test:
            attack_reduces_target_hp_to: 0
          then:
          - op: stabilize
          - op: condition
            condition: poisoned
            ends:
              hours: 1
            while_active:
              conditions:
              - paralyzed
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: bite
          count: 2
      ethereal_sight:
        economy: passive
        from_planes:
        - material
        - ethereal
        see_other_plane: true
        range_feet: 60
      ethereal_jaunt:
        economy: bonus_action
        op: teleport
        plane_pairs:
        - - material
          - ethereal
        - - ethereal
          - material
    skill_totals:
      stealth: 7
    defenses: {}
    senses:
      passive_perception: 10
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
- id: dnd2024:creature_phase_spider
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
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 8480bbdc00078e6a8bfeadbf6198ebb0ead7f7b56d908b58c9fb8506c54787e8
  source_path: builtin/srd-5.2.1/creature/phase_spider.md
  pages:
  - 313
  - 314
  engine_gaps: &id001
  - dnd2024:creature_phase_spider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 5f87feefe01340bbbf973d9714cea74655a2735271cab64e8aa8d54851d7aced
splat_clauses:
- key: source_rules
  text: '# Phase Spider Large Monstrosity, Unaligned AC 14 Initiative +3 (13) HP 45 (7d10 + 7) Speed 30
    ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 16 +3 +3 Con 12 +1 +1 Int 6 -2 -2 Wis
    10 +0 +0 Cha 6 -2 -2 Skills Stealth +7 Senses Darkvision 60 ft.; Passive Perception 10 Languages None
    CR 3 (XP 700; PB +2) ### Traits Ethereal Sight. The spider can see 60 feet into the Ethereal Plane
    while on the Material Plane and vice versa. Spider Climb. The spider can climb difficult surfaces,
    including along ceilings, without needing to make an ability check. Web Walker. The spider ignores
    movement restrictions caused by webs, and the spider knows the location of any other creature in contact
    with the same web. ### Actions Multiattack. The spider makes two Bite attacks. Bite. Melee Attack
    Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Piercing damage plus 9 (2d8) Poison damage. If this damage
    reduces the target to 0 Hit Points, the target becomes Stable, and it has the Poisoned condition for
    1 hour. While Poisoned, the target also has the Paralyzed condition. ### Bonus Actions Ethereal Jaunt.
    The spider teleports from the Material Plane to the Ethereal Plane or vice versa.'
  sha256: 990d5e7b83fe0d5cc251be4d821ea977f73a65d848b0c45c9e7e10d427c38dbc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Phase Spider

<!-- SRD page 313 -->

Large Monstrosity, Unaligned\
AC 14 Initiative +3 (13)\
HP 45 (7d10 + 7)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 314 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 16 +3 +3 Con 12 +1 +1\
Int 6 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Skills Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 3 (XP 700; PB +2)\

### Traits

Ethereal Sight. The spider can see 60 feet into the Ethe-\
real Plane while on the Material Plane and vice versa.\
Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The spider ignores movement restrictions\
caused by webs, and the spider knows the location of\
any other creature in contact with the same web.\

### Actions

Multiattack. The spider makes two Bite attacks.\
Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 +\
3) Piercing damage plus 9 (2d8) Poison damage. If this\
damage reduces the target to 0 Hit Points, the target\
becomes Stable, and it has the Poisoned condition for 1\
hour. While Poisoned, the target also has the Paralyzed\
condition.\

### Bonus Actions

Ethereal Jaunt. The spider teleports from the Material\
Plane to the Ethereal Plane or vice versa.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]], [[../../srd-5.2.1/pages/314|page 314]].
