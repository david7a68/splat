---
splat_schema: 1
splat_id: cb346188-c152-5a56-8d74-da37fa5a3a29
splat_kind: creature
splat_key: dnd2024:creatures/young_bronze_dragon
splat_name: Young Bronze Dragon
splat_origin:
  key: dnd2024:reference/creature/young_bronze_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 270
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Insight +4, Perception +7, Stealth +3
  immunities_text: Lightning
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17
  languages_text: Common, Draconic
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 142 (15d10 + 60)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 10
  save_dexterity:
    value: 3
  constitution:
    value: 19
  save_constitution:
    value: 4
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 13
  save_wisdom:
    value: 4
  charisma:
    value: 17
  save_charisma:
    value: 3
  armor_class:
    value: 17
  max_hp:
    value: 142
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_bronze_dragon
  implementation: dnd2024:creature_young_bronze_dragon
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 16
          amount:
            dice:
              count: 2
              sides: 10
            add: 5
          damage_type: slashing
        reach_feet: 10
      lightning_breath:
        economy: action
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
        op: save
        ability: dexterity
        dc: 15
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 60
            origin: self
            width_feet: 5
        failure:
        - op: damage
          average: 49
          amount:
            dice:
              count: 9
              sides: 10
            add: 0
          damage_type: lightning
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 3
        replacement:
          max_attacks: 1
          choices:
          - feature: repulsion_breath
      repulsion_breath:
        economy: action
        op: save
        ability: strength
        dc: 15
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: push
          distance_max_feet: 40
          direction: straight_away_from_self
        - op: condition
          condition: prone
        success: []
    skill_totals:
      insight: 4
      perception: 7
      stealth: 3
    defenses:
      immunities:
        damage_types:
        - lightning
    senses:
      passive_perception: 17
      special:
      - sense: blindsight
        range_feet: 30
      - sense: darkvision
        range_feet: 120
    communication:
      speaks:
      - common
      - draconic
      understands:
      - common
      - draconic
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
- id: dnd2024:creature_young_bronze_dragon
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
  source_sha256: b6d2dec4279d5abe8b1230954f39eb839b6a8fbd7ba8a4d3cc1adb07fd92bc96
  source_path: builtin/srd-5.2.1/creature/young_bronze_dragon.md
  pages:
  - 270
  engine_gaps: &id001
  - dnd2024:creature_young_bronze_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 2514c0349cdf7dfb0ee7ea23a6969d68fc54daaf226654384fa1d26042060291
splat_clauses:
- key: source_rules
  text: '# Young Bronze Dragon Large Dragon (Metallic), Lawful Good AC 17 Initiative +3 (13) HP 142 (15d10
    + 60) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 10 +0 +3 Con
    19 +4 +4 Int 14 +2 +2 Wis 13 +1 +4 Cha 17 +3 +3 Skills Insight +4, Perception +7, Stealth +3 Immunities
    Lightning Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 Languages Common, Draconic
    CR 8 (XP 3,900; PB +3) ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack.
    The dragon makes three Rend attacks. It can replace one attack with a use of Repulsion Breath. Rend.
    Melee Attack Roll: +8, reach 10 ft. Hit: 16 (2d10 + 5) Slashing damage. Lightning Breath (Recharge
    5–6). Dexterity Saving Throw: DC 15, each creature in a 60-foot-long, 5-footwide Line. Failure: 49
    (9d10) Lightning damage. Success: Half damage. Repulsion Breath. Strength Saving Throw: DC 15, each
    creature in a 30-foot Cone. Failure: The target is pushed up to 40 feet straight away from the dragon
    and has the Prone condition.'
  sha256: a314431894742e996a93dbbff75679a16f616fd9754b96b31d9abbcbf3cd01e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Bronze Dragon

<!-- SRD page 270 -->

Large Dragon (Metallic), Lawful Good\
AC 17 Initiative +3 (13)\
HP 142 (15d10 + 60)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 10 +0 +3 Con 19 +4 +4\
Int 14 +2 +2 Wis 13 +1 +4 Cha 17 +3 +3\
Skills Insight +4, Perception +7, Stealth +3\
Immunities Lightning\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 17\
Languages Common, Draconic\
CR 8 (XP 3,900; PB +3)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Repulsion Breath.\
Rend. Melee Attack Roll: +8, reach 10 ft. Hit: 16 (2d10\
+ 5) Slashing damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 15, each creature in a 60-foot-long, 5-foot-\
wide Line. Failure: 49 (9d10) Lightning damage. Suc-\
cess: Half damage.\
Repulsion Breath. Strength Saving Throw: DC 15, each\
creature in a 30-foot Cone. Failure: The target is pushed\
up to 40 feet straight away from the dragon and has the\
Prone condition.\

Original layout: [[../../srd-5.2.1/pages/270|page 270]].
