---
splat_schema: 1
splat_id: d59c56e9-b533-5939-92ec-dcddd24de7c7
splat_kind: creature
splat_key: dnd2024:creatures/lion
splat_name: Lion
splat_origin:
  key: dnd2024:reference/creature/lion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 356, 357
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +3, Stealth +4
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (4d10)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/lion
  implementation: dnd2024:creature_lion
  parameters:
    features:
      pack_tactics:
        economy: passive
        advantage:
          roll: attack
          target_kind: creature
          requires:
            ally_distance_to_target_max_feet: 5
            ally_incapacitated: false
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: slashing
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 2
        replacement:
          max_attacks: 1
          choices:
          - feature: roar
      running_leap:
        economy: passive
        long_jump_max_feet: 25
        running_start_min_feet: 10
      roar:
        economy: action
        op: save
        ability: wisdom
        dc: 11
        targets:
          kind: creature
          count: 1
          range_feet: 15
        failure:
        - op: condition
          condition: frightened
          ends:
            event: turn.start
            participant: self
            occurrence: next
        success: []
    skill_totals:
      perception: 3
      stealth: 4
    defenses: {}
    senses:
      passive_perception: 13
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
- id: dnd2024:creature_lion
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
  - turn.start
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
  source_sha256: 2cd74f5899c5e162b87f448446e80fb3a427be918b512e481321bd14645b4beb
  source_path: builtin/srd-5.2.1/creature/lion.md
  pages:
  - 356
  - 357
  engine_gaps: &id001
  - dnd2024:creature_lion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e23fc5bc045d584949ff9c777e00af2248584df7d2072812ccc774e3fdbaa29c
splat_clauses:
- key: source_rules
  text: '# Lion Large Beast, Unaligned AC 12 Initiative +2 (12) HP 22 (4d10) Speed 50 ft. MOD SAVE MOD
    SAVE MOD SAVE Str 17 +3 +3 Dex 15 +2 +2 Con 11 +0 +0 Int 3 -4 -4 Wis 12 +1 +1 Cha 8 -1 -1 Skills Perception
    +3, Stealth +4 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR 1 (XP 200; PB +2)
    ### Traits Pack Tactics. The lion has Advantage on an attack roll against a creature if at least one
    of the lion’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated
    condition. Running Leap. With a 10-foot running start, the lion can Long Jump up to 25 feet. ### Actions
    Multiattack. The lion makes two Rend attacks. It can replace one attack with a use of Roar. Rend.
    Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage. Roar. Wisdom Saving Throw: DC
    11, one creature within 15 feet. Failure: The target has the Frightened condition until the start
    of the lion’s next turn.'
  sha256: ff91d418eb51c0c9bf7999ffe364e50d0c3c775ef735e53c6b6ac30df33d9866
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lion

<!-- SRD page 356 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 22 (4d10)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 15 +2 +2 Con 11 +0 +0\
Int 3 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +3, Stealth +4\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1 (XP 200; PB +2)\

<!-- SRD page 357 -->


### Traits

Pack Tactics. The lion has Advantage on an attack roll\
against a creature if at least one of the lion’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\
Running Leap. With a 10-foot running start, the lion\
can Long Jump up to 25 feet.\

### Actions

Multiattack. The lion makes two Rend attacks. It can\
replace one attack with a use of Roar.\
Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Slashing damage.\
Roar. Wisdom Saving Throw: DC 11, one creature\
within 15 feet. Failure: The target has the Frightened\
condition until the start of the lion’s next turn.\

Original layout: [[../../srd-5.2.1/pages/356|page 356]], [[../../srd-5.2.1/pages/357|page 357]].
