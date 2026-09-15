---
splat_schema: 1
splat_id: 70219a9d-618c-52de-80dc-6918dcb1b6da
splat_kind: creature
splat_key: dnd2024:creatures/frost_giant
splat_name: Frost Giant
splat_origin:
  key: dnd2024:reference/creature/frost_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285, 286
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Neutral Evil
  movement_text: 40 ft.
  skills_text: Athletics +9, Perception +3
  immunities_text: Cold
  senses_text: Passive Perception 13
  languages_text: Giant
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 149 (13d12 + 65)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 21
  save_constitution:
    value: 8
  intelligence:
    value: 9
  save_intelligence:
    value: -1
  wisdom:
    value: 10
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 4
  armor_class:
    value: 15
  max_hp:
    value: 149
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/frost_giant
  implementation: dnd2024:creature_frost_giant
  parameters:
    features:
      frost_axe:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 9
        targets:
          count: 1
        hit:
        - op: damage
          average: 19
          amount:
            dice:
              count: 2
              sides: 12
            add: 6
          damage_type: slashing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: cold
        reach_feet: 10
      great_bow:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 9
        targets:
          count: 1
        hit:
        - op: damage
          average: 17
          amount:
            dice:
              count: 2
              sides: 10
            add: 6
          damage_type: piercing
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: cold
        - op: modify_speed
          add_feet: -10
          ends:
            event: turn.end
            participant: target
            occurrence: next
        range_feet: 150
        long_range_feet: 600
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: frost_axe
        - feature: great_bow
      war_cry:
        economy: bonus_action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        targets:
          choice:
          - self
          - one_creature_that_can_see_or_hear_self
        effects:
        - op: temporary_hit_points
          average: 16
          amount:
            dice:
              count: 2
              sides: 10
            add: 5
        - op: advantage
          roll: attack
          ends:
            event: turn.start
            participant: self
            occurrence: next
    skill_totals:
      athletics: 9
      perception: 3
    defenses:
      immunities:
        damage_types:
        - cold
    senses:
      passive_perception: 13
      special: []
    communication:
      speaks:
      - giant
      understands:
      - giant
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
- id: dnd2024:creature_frost_giant
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
  source_sha256: 87fedabb897d95d9bbbac61a6c4b3668ed38d514a20de8c5c20fd888f985a5ab
  source_path: builtin/srd-5.2.1/creature/frost_giant.md
  pages:
  - 285
  - 286
  engine_gaps: &id001
  - dnd2024:creature_frost_giant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 925ff7a4e129134046b2c0282836620fdf9ff13b12c8fd23dccffb9f2a7d2b72
splat_clauses:
- key: source_rules
  text: '# Frost Giant Huge Giant, Neutral Evil AC 15 Initiative +2 (12) HP 149 (13d12 + 65) Speed 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 9 -1 -1 Con 21 +5 +8 Int 9 -1 -1 Wis 10 +0 +3 Cha
    12 +1 +4 Skills Athletics +9, Perception +3 Immunities Cold Senses Passive Perception 13 Languages
    Giant CR 8 (XP 3,900; PB +3) ### Actions Multiattack. The giant makes two attacks, using Frost Axe
    or Great Bow in any combination. Frost Axe. Melee Attack Roll: +9, reach 10 ft. Hit: 19 (2d12 + 6)
    Slashing damage plus 9 (2d8) Cold damage. Great Bow. Ranged Attack Roll: +9, range 150/600 ft. Hit:
    17 (2d10 + 6) Piercing damage plus 7 (2d6) Cold damage, and the target’s Speed decreases by 10 feet
    until the end of its next turn. ### Bonus Actions War Cry (Recharge 5–6). The giant or one creature
    of its choice that can see or hear it gains 16 (2d10 + 5) Temporary Hit Points and has Advantage on
    attack rolls until the start of the giant’s next turn.'
  sha256: ca798f80aa1125630d4e4ac870287749ed7e61ad25baf3e900e7aedc43a5cca0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Frost Giant

<!-- SRD page 285 -->

Huge Giant, Neutral Evil\
AC 15 Initiative +2 (12)\
HP 149 (13d12 + 65)\
Speed 40 ft.\

<!-- SRD page 286 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 9 −1 −1 Con 21 +5 +8\
Int 9 −1 −1 Wis 10 +0 +3 Cha 12 +1 +4\
Skills Athletics +9, Perception +3\
Immunities Cold\
Senses Passive Perception 13\
Languages Giant\
CR 8 (XP 3,900; PB +3)\

### Actions

Multiattack. The giant makes two attacks, using Frost\
Axe or Great Bow in any combination.\
Frost Axe. Melee Attack Roll: +9, reach 10 ft. Hit: 19\
(2d12 + 6) Slashing damage plus 9 (2d8) Cold damage.\
Great Bow. Ranged Attack Roll: +9, range 150/600 ft.\
Hit: 17 (2d10 + 6) Piercing damage plus 7 (2d6) Cold\
damage, and the target’s Speed decreases by 10 feet\
until the end of its next turn.\

### Bonus Actions

War Cry (Recharge 5–6). The giant or one creature of\
its choice that can see or hear it gains 16 (2d10 + 5)\
Temporary Hit Points and has Advantage on attack rolls\
until the start of the giant’s next turn.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]], [[../../srd-5.2.1/pages/286|page 286]].
