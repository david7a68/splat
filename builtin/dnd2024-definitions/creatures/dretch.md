---
splat_schema: 1
splat_id: ed70a67c-f9f5-5278-ac81-426da49fed85
splat_kind: creature
splat_key: dnd2024:creatures/dretch
splat_name: Dretch
splat_origin:
  key: dnd2024:reference/creature/dretch
splat_sources:
- title: SRD 5.2.1
  locator: pp. 281
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 20 ft.
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Abyssal; telepathy 60 ft. (works only with creatures that understand Abyssal)
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 18 (4d6 + 4)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 18
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/dretch
  implementation: dnd2024:creature_dretch
  parameters:
    features:
      rend:
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
          damage_type: slashing
        reach_feet: 5
      fetid_cloud:
        economy: action
        resource:
          capacity: 1
          recovery: long_rest
          cost: 1
        op: save
        ability: constitution
        dc: 11
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            size_feet: 10
            origin: self
        failure:
        - op: condition
          condition: poisoned
          ends:
            event: turn.end
            participant: target
            occurrence: next
          while_active:
            turn_economy_exclusive_choice:
            - action
            - bonus_action
            reactions_allowed: false
        success: []
    defenses:
      resistances:
        damage_types:
        - cold
        - fire
        - lightning
      immunities:
        damage_types:
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 9
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - abyssal
      understands:
      - abyssal
      telepathy:
        range_feet: 60
        target_understands: abyssal
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
- id: dnd2024:creature_dretch
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
  - turn.end
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
  source_sha256: 44ee146c8a6f938f133624eb4e34d91bafa9c076eb8c28201bed5f2fc72f2f16
  source_path: builtin/srd-5.2.1/creature/dretch.md
  pages:
  - 281
  engine_gaps: &id001
  - dnd2024:creature_dretch
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 1ebb66463305d74a5b37cffabe115584a4e8163abe768116a12c43336ee21e31
splat_clauses:
- key: source_rules
  text: '# Dretch Small Fiend (Demon), Chaotic Evil AC 11 Initiative +0 (10) HP 18 (4d6 + 4) Speed 20
    ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 11 +0 +0 Con 12 +1 +1 Int 5 -3 -3 Wis 8 -1 -1 Cha
    3 -4 -4 Resistances Cold, Fire, Lightning Immunities Poison; Poisoned Senses Darkvision 60 ft.; Passive
    Perception 9 Languages Abyssal; telepathy 60 ft. (works only with creatures that understand Abyssal)
    CR 1/4 (XP 50; PB +2) ### Actions Rend. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Slashing
    damage. Fetid Cloud (1/Day). Constitution Saving Throw: DC 11, each creature in a 10-foot Emanation
    originating from the dretch. Failure: The target has the Poisoned condition until the end of its next
    turn. While Poisoned, the creature can take either an action or a Bonus Action on its turn, not both,
    and it can’t take Reactions.'
  sha256: 913aaf00583a034f68c723c5d91af21896bdb32742a2d9d92fc94bccbde8ec61
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dretch

<!-- SRD page 281 -->

Small Fiend (Demon), Chaotic Evil\
AC 11 Initiative +0 (10)\
HP 18 (4d6 + 4)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 11 +0 +0 Con 12 +1 +1\
Int 5 −3 −3 Wis 8 −1 −1 Cha 3 −4 −4\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Abyssal; telepathy 60 ft. (works only with\
creatures that understand Abyssal)\
CR 1/4 (XP 50; PB +2)\

### Actions

Rend. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1)\
Slashing damage.\
Fetid Cloud (1/Day). Constitution Saving Throw: DC 11,\
each creature in a 10-foot Emanation originating from\
the dretch. Failure: The target has the Poisoned condi-\
tion until the end of its next turn. While Poisoned, the\
creature can take either an action or a Bonus Action on\
its turn, not both, and it can’t take Reactions.\

Original layout: [[../../srd-5.2.1/pages/281|page 281]].
