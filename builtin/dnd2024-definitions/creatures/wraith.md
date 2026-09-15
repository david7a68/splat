---
splat_schema: 1
splat_id: 98271f17-1270-51f4-8b3c-fbcafb58085c
splat_kind: creature
splat_key: dnd2024:creatures/wraith
splat_name: Wraith
splat_origin:
  key: dnd2024:reference/creature/wraith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 342, 343
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral Evil
  size_options_text: Medium or Small
  movement_text: 5 ft., Fly 60 ft. (hover)
  resistances_text: Acid, Bludgeoning, Cold, Fire, Piercing, Slashing
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone,
    Restrained, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Common plus two other languages
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 67 (9d8 + 27)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 13
  max_hp:
    value: 67
  initiative:
    value: 3
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/wraith
  implementation: dnd2024:creature_wraith
  parameters:
    features:
      sunlight_sensitivity:
        economy: passive
        disadvantage:
          rolls:
          - attack
          - ability_check
          when: self_in_sunlight
      life_drain:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 21
          amount:
            dice:
              count: 4
              sides: 8
            add: 3
          damage_type: necrotic
        - op: if
          test:
            target_kind: creature
          then:
          - op: reduce_hit_point_maximum
            amount:
              event_value: damage_taken
        reach_feet: 5
      incorporeal_movement:
        economy: passive
        move_through:
        - creatures
        - objects
        movement_cost: difficult_terrain
        turn_end_inside_object:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 10
            add: 0
          damage_type: force
      create_specter:
        economy: action
        op: create_creature
        targets:
          kind: corpse
          former_creature_type: humanoid
          range_feet: 10
          dead_for_max:
            minutes: 1
        definition:
          key: dnd2024:creatures/specter
        destination_choice:
        - corpse_space
        - nearest_unoccupied_space
        controller: self
        controlled_capacity: 7
    defenses:
      resistances:
        damage_types:
        - acid
        - bludgeoning
        - cold
        - fire
        - piercing
        - slashing
      immunities:
        damage_types:
        - necrotic
        - poison
        conditions:
        - charmed
        - exhaustion
        - grappled
        - paralyzed
        - petrified
        - poisoned
        - prone
        - restrained
        - unconscious
    senses:
      passive_perception: 12
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 2
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
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
- id: dnd2024:creature_wraith
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
  - summoning
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: fc1a732b323860833ebf17daadf377f8fe3caea4bc4c21a3fe704e47d4303430
  source_path: builtin/srd-5.2.1/creature/wraith.md
  pages:
  - 342
  - 343
  engine_gaps: &id001
  - dnd2024:creature_wraith
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 19228aaefbac02fef77e99e09605a352f398f750d02930173c6b7068c498a6a4
splat_clauses:
- key: source_rules
  text: '# Wraith Medium or Small Undead, Neutral Evil AC 13 Initiative +3 (13) HP 67 (9d8 + 27) Speed
    5 ft., Fly 60 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 16 +3 +3 Con 16 +3 +3 Int 12
    +1 +1 Wis 14 +2 +2 Cha 15 +2 +2 Resistances Acid, Bludgeoning, Cold, Fire, Piercing, Slashing Immunities
    Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious Senses Darkvision 60 ft.; Passive Perception 12 Languages Common plus two other languages
    CR 5 (XP 1,800; PB +3) ### Traits Incorporeal Movement. The wraith can move through other creatures
    and objects as if they were Difficult Terrain. It takes 5 (1d10) Force damage if it ends its turn
    inside an object. Sunlight Sensitivity. While in sunlight, the wraith has Disadvantage on ability
    checks and attack rolls. ### Actions Life Drain. Melee Attack Roll: +6, reach 5 ft. Hit: 21 (4d8 +
    3) Necrotic damage. If the target is a creature, its Hit Point maximum decreases by an amount equal
    to the damage taken. Create Specter. The wraith targets a Humanoid corpse within 10 feet of itself
    that has been dead for no longer than 1 minute. The target’s spirit rises as a Specter in the space
    of its corpse or in the nearest unoccupied space. The specter is under the wraith’s control. The wraith
    can have no more than seven specters under its control at a time.'
  sha256: 3e978fc631b82085530eddd9dc94d573b84df737ba7fec5089be20cac655c590
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wraith

<!-- SRD page 342 -->

Medium or Small Undead, Neutral Evil\
AC 13 Initiative +3 (13)\
HP 67 (9d8 + 27)\
Speed 5 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 16 +3 +3 Con 16 +3 +3\
Int 12 +1 +1 Wis 14 +2 +2 Cha 15 +2 +2\
Resistances Acid, Bludgeoning, Cold, Fire, Piercing,\
Slashing\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Grappled, Paralyzed, Petrified, Poisoned, Prone,\
Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Common plus two other languages\
CR 5 (XP 1,800; PB +3)\

<!-- SRD page 343 -->


### Traits

Incorporeal Movement. The wraith can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\
Sunlight Sensitivity. While in sunlight, the wraith has\
Disadvantage on ability checks and attack rolls.\

### Actions

Life Drain. Melee Attack Roll: +6, reach 5 ft. Hit: 21\
(4d8 + 3) Necrotic damage. If the target is a creature, its\
Hit Point maximum decreases by an amount equal to\
the damage taken.\
Create Specter. The wraith targets a Humanoid corpse\
within 10 feet of itself that has been dead for no longer\
than 1 minute. The target’s spirit rises as a Specter in\
the space of its corpse or in the nearest unoccupied\
space. The specter is under the wraith’s control. The\
wraith can have no more than seven specters under its\
control at a time.\

Original layout: [[../../srd-5.2.1/pages/342|page 342]], [[../../srd-5.2.1/pages/343|page 343]].
